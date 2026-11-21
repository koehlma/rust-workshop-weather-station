//! Connection to Nexigon Hub, enabled by the `nexigon` feature.
//!
//! [`run`] takes the [`Peripherals`] that the connection needs and the sensor.
//! It joins a Wi-Fi network, synchronizes the clock with SNTP, and connects to
//! Nexigon Hub over TLS with the device certificate. It learns the device ID
//! with `actor_GetActor`, describes the commands of the device, and checks for
//! updates. Every 30 seconds, it publishes a heartbeat event and the reading
//! of the sensor as an environment event.
//!
//! Outside of this module, nothing depends on Nexigon. The module uses
//! [`crate::watchdog`] to roll back updates that do not commit themselves.

mod clock;
mod commands;
mod config;
mod download;
mod events;
mod firmware;
mod hub;
mod multiplex;
mod network;
mod ota;
mod tls;
mod websocket;

/// Types of the device API of Nexigon Hub, generated with Sidex.
mod api {
    include!("generated/nexigon_api.rs");
}

use core::convert::Infallible;
use core::fmt::Write as _;

use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_rp::Peri;
use embassy_rp::clocks::RoscRng;
use embassy_rp::peripherals::DMA_CH0;
use embassy_rp::peripherals::FLASH;
use embassy_rp::peripherals::PIN_23;
use embassy_rp::peripherals::PIN_24;
use embassy_rp::peripherals::PIN_25;
use embassy_rp::peripherals::PIN_29;
use embassy_rp::peripherals::PIO0;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::Timer;
use log::error;
use log::info;
use log::warn;
use static_cell::StaticCell;

use self::api::actor::Actor;
use self::api::actor::GetActorAction;
use self::api::actor::GetActorOutput;
use self::firmware::Firmware;
use crate::dht11::Dht11;

/// Interval between heartbeat and environment events.
const EVENT_INTERVAL: Duration = Duration::from_secs(30);

/// Interval between update checks.
const UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Delay before connecting again after a failure.
const RETRY_DELAY: Duration = Duration::from_secs(10);

/// Peripherals that the connection to Nexigon Hub needs.
pub struct Peripherals {
    /// Flash, which holds the firmware slots for updates.
    pub flash: Peri<'static, FLASH>,
    /// Power of the Wi-Fi chip.
    pub wifi_power: Peri<'static, PIN_23>,
    /// Data line of the Wi-Fi chip.
    pub wifi_data: Peri<'static, PIN_24>,
    /// Chip select of the Wi-Fi chip.
    pub wifi_chip_select: Peri<'static, PIN_25>,
    /// Clock of the Wi-Fi chip.
    pub wifi_clock: Peri<'static, PIN_29>,
    /// PIO block that drives the bus of the Wi-Fi chip.
    pub wifi_pio: Peri<'static, PIO0>,
    /// DMA channel of the bus of the Wi-Fi chip.
    pub wifi_dma: Peri<'static, DMA_CH0>,
}

/// Connects to Nexigon Hub, publishes events, runs commands, and installs
/// updates, reconnecting after failures.
pub async fn run(spawner: Spawner, peripherals: Peripherals, mut sensor: Dht11<'static>) -> ! {
    let mut firmware = Firmware::new(peripherals.flash);
    if firmware.awaits_commit() {
        info!("running a new firmware, which commits itself once it reaches Nexigon Hub");
    }
    spawner.spawn(firmware::commit_deadline_task().unwrap());

    let (stack, mut control) = network::start(
        spawner,
        peripherals.wifi_power,
        peripherals.wifi_data,
        peripherals.wifi_chip_select,
        peripherals.wifi_clock,
        peripherals.wifi_pio,
        peripherals.wifi_dma,
    )
    .await;

    static TLS_BUFFERS: StaticCell<tls::Buffers> = StaticCell::new();
    static DOWNLOAD_BUFFERS: StaticCell<tls::Buffers> = StaticCell::new();
    static HUB_BUFFERS: StaticCell<hub::Buffers> = StaticCell::new();
    let tls_buffers = TLS_BUFFERS.init(tls::Buffers::new());
    let download_buffers = DOWNLOAD_BUFFERS.init(tls::Buffers::new());
    let hub_buffers = HUB_BUFFERS.init(hub::Buffers::new());
    let mut device = commands::Device::new(&mut control);
    loop {
        while clock::unix_seconds().is_none() {
            match clock::sync(stack).await {
                Ok(()) => info!("synchronized clock: {:?}", clock::now()),
                Err(error) => {
                    warn!("failed to synchronize clock: {error:?}");
                    Timer::after(RETRY_DELAY).await;
                }
            }
        }

        info!("connecting to Nexigon Hub at {}", config::HUB_HOST);
        let started = Instant::now();
        let connection = match tls::connect(stack, tls_buffers, config::HUB_HOST).await {
            Ok(connection) => connection,
            Err(error) => {
                warn!(
                    "failed to connect to Nexigon Hub after {} ms: {error}",
                    started.elapsed().as_millis()
                );
                Timer::after(RETRY_DELAY).await;
                continue;
            }
        };
        let error = match hub::Session::open(connection, hub_buffers, RoscRng.next_u32()).await {
            Ok(mut session) => {
                info!("connected in {} ms", started.elapsed().as_millis());
                let served = serve(
                    &mut session,
                    &mut device,
                    &mut firmware,
                    &mut sensor,
                    stack,
                    download_buffers,
                )
                .await;
                match served {
                    Ok(never) => match never {},
                    Err(error) => error,
                }
            }
            Err(error) => error,
        };
        warn!(
            "connection to Nexigon Hub failed after {} ms: {error}",
            started.elapsed().as_millis()
        );
        Timer::after(RETRY_DELAY).await;
    }
}

/// Serves a session: checks for updates every [`UPDATE_CHECK_INTERVAL`],
/// publishes events every [`EVENT_INTERVAL`], and runs commands in between.
async fn serve(
    session: &mut hub::Session<'_, tls::Connection<'_>>,
    device: &mut commands::Device<'_>,
    firmware: &mut Firmware,
    sensor: &mut Dht11<'_>,
    stack: Stack<'_>,
    download_buffers: &mut tls::Buffers,
) -> Result<Infallible, hub::Error> {
    let GetActorOutput { actor } = session.invoke("actor_GetActor", &GetActorAction {}).await?;
    let Actor::Device(actor) = actor else {
        error!("authenticated as another kind of actor: {actor:?}");
        return Err(hub::Error::Protocol("not authenticated as a device"));
    };
    let mut device_id = heapless::String::<32>::new();
    write!(device_id, "{}", actor.device_id.0).expect("device IDs have 24 bytes");
    info!("authenticated as device {device_id}");

    commands::publish_manifest(session, &device_id).await?;
    info!("published commands");

    let mut next_heartbeat = Instant::now();
    let mut next_update_check = Instant::now();
    loop {
        if Instant::now() >= next_update_check {
            next_update_check = Instant::now() + UPDATE_CHECK_INTERVAL;
            let outcome =
                ota::check(session, &device_id, firmware, stack, download_buffers).await?;
            if let ota::Outcome::Activate = outcome {
                // Give the logger time to send the last messages.
                Timer::after_millis(500).await;
                firmware::reset();
            }
        }
        if session
            .wait_for_command(next_heartbeat.min(next_update_check))
            .await?
        {
            commands::run(session, device).await?;
            if device.take_update_check_request() {
                next_update_check = Instant::now();
            }
        } else if Instant::now() >= next_heartbeat {
            let reading = match sensor.read().await {
                Ok(reading) => Some(reading),
                Err(error) => {
                    warn!("failed to read the DHT11 sensor: {error}");
                    None
                }
            };
            events::publish(session, &device_id, reading).await?;
            match reading {
                Some(reading) => info!("published heartbeat and {reading}"),
                None => info!("published heartbeat"),
            }
            next_heartbeat += EVENT_INTERVAL;
        }
    }
}
