//! Example firmware for the Raspberry Pi Pico W that stays connected to
//! Nexigon Hub, publishes a heartbeat event and the readings of a DHT11
//! sensor every 30 seconds, runs commands, and installs updates.
//!
//! The firmware joins a Wi-Fi network, synchronizes its clock with SNTP, and
//! connects to Nexigon Hub over TLS with its device certificate. It learns its
//! device ID with `actor_GetActor`, describes its commands, checks for updates,
//! and then publishes events with `devices_PublishEvents`. Logs go to the USB
//! serial port.

#![no_std]
#![no_main]

mod clock;
mod commands;
mod config;
mod dht11;
mod download;
mod firmware;
mod hub;
mod multiplex;
mod ota;
mod tls;
mod websocket;

/// Types of the device API of Nexigon Hub, generated with Sidex.
mod nexigon_api {
    include!("generated/nexigon_api.rs");
}

use core::convert::Infallible;
use core::fmt::Write as _;

use cyw43::JoinOptions;
use cyw43::aligned_bytes;
use cyw43_pio::DEFAULT_CLOCK_DIVIDER;
use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_net::StackResources;
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::dma;
use embassy_rp::gpio::Level;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::DMA_CH0;
use embassy_rp::peripherals::PIO0;
use embassy_rp::peripherals::USB;
use embassy_rp::pio;
use embassy_rp::pio::Pio;
use embassy_rp::usb;
use embassy_rp::watchdog::Watchdog;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::Timer;
use log::error;
use log::info;
use log::warn;
use panic_reset as _;
use sidex_embedded::Encode;
use sidex_embedded::EncodeError;
use sidex_embedded::json::Writer;
use sidex_embedded::types::Json;
use sidex_embedded::types::Map;
use sidex_embedded::types::Seq;
use sidex_embedded::types::Str;
use sidex_embedded::types::Timestamp;
use static_cell::StaticCell;

use crate::dht11::Dht11;
use crate::firmware::Firmware;

use crate::nexigon_api::actor::Actor;
use crate::nexigon_api::actor::GetActorAction;
use crate::nexigon_api::actor::GetActorOutput;
use crate::nexigon_api::devices::DeviceEvent;
use crate::nexigon_api::devices::DeviceEventId;
use crate::nexigon_api::devices::DeviceEventSeverity;
use crate::nexigon_api::devices::DeviceId;
use crate::nexigon_api::devices::PublishDeviceEventsAction;
use crate::nexigon_api::outputs::Empty;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
});

/// Interval between heartbeat and environment events.
const EVENT_INTERVAL: Duration = Duration::from_secs(30);

/// Interval between update checks.
const UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Delay before retrying after a failure.
const RETRY_DELAY: Duration = Duration::from_secs(10);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // The bootloader has started the watchdog, which must be fed from now on.
    spawner.spawn(firmware::watchdog_task(Watchdog::new(p.WATCHDOG)).unwrap());
    let mut firmware = Firmware::new(p.FLASH);
    // The Joy-IT explorer board connects its DHT11 sensor to GP0.
    let mut sensor = Dht11::new(p.PIN_0);
    spawner.spawn(logger_task(usb::Driver::new(p.USB, Irqs)).unwrap());
    // Give the host time to open the USB serial port, so that it receives
    // the first messages.
    Timer::after_secs(3).await;
    info!("starting firmware {}", config::FIRMWARE_VERSION);
    if firmware.awaits_commit() {
        info!("running a new firmware, which commits itself once it reaches Nexigon Hub");
    }

    let wifi_firmware = aligned_bytes!("../firmware/cyw43/43439A0.bin");
    let clm = aligned_bytes!("../firmware/cyw43/43439A0_clm.bin");
    let nvram = aligned_bytes!("../firmware/cyw43/nvram_rp2040.bin");

    let power = Output::new(p.PIN_23, Level::Low);
    let chip_select = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        chip_select,
        p.PIN_24,
        p.PIN_29,
        dma::Channel::new(p.DMA_CH0, Irqs),
    );

    static WIFI_STATE: StaticCell<cyw43::State> = StaticCell::new();
    let (net_device, mut control, runner) = cyw43::new(
        WIFI_STATE.init(cyw43::State::new()),
        power,
        spi,
        wifi_firmware,
        nvram,
    )
    .await;
    spawner.spawn(wifi_task(runner).unwrap());
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    static NET_RESOURCES: StaticCell<StackResources<4>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        embassy_net::Config::dhcpv4(Default::default()),
        NET_RESOURCES.init(StackResources::new()),
        RoscRng.next_u64(),
    );
    spawner.spawn(net_task(runner).unwrap());

    info!("joining Wi-Fi network {}", config::WIFI_SSID);
    while let Err(error) = control
        .join(
            config::WIFI_SSID,
            JoinOptions::new(config::WIFI_PASSWORD.as_bytes()),
        )
        .await
    {
        warn!("failed to join Wi-Fi network: {error:?}");
        Timer::after(RETRY_DELAY).await;
    }
    stack.wait_config_up().await;
    info!(
        "network is up: {:?}",
        stack.config_v4().map(|config| config.address)
    );

    run(stack, &mut control, &mut firmware, &mut sensor).await
}

/// Connects to Nexigon Hub, publishes events, runs commands, and installs
/// updates, reconnecting after failures.
async fn run(
    stack: Stack<'static>,
    control: &mut cyw43::Control<'static>,
    firmware: &mut Firmware,
    sensor: &mut Dht11<'static>,
) -> ! {
    static TLS_BUFFERS: StaticCell<tls::Buffers> = StaticCell::new();
    static DOWNLOAD_BUFFERS: StaticCell<tls::Buffers> = StaticCell::new();
    static HUB_BUFFERS: StaticCell<hub::Buffers> = StaticCell::new();
    let tls_buffers = TLS_BUFFERS.init(tls::Buffers::new());
    let download_buffers = DOWNLOAD_BUFFERS.init(tls::Buffers::new());
    let hub_buffers = HUB_BUFFERS.init(hub::Buffers::new());
    let mut device = commands::Device::new(control);
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
                    firmware,
                    sensor,
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
                crate::firmware::reset();
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
            publish_events(session, &device_id, reading).await?;
            match reading {
                Some(reading) => info!(
                    "published heartbeat and {:.1} °C, {:.0} % humidity",
                    reading.temperature_celsius, reading.humidity_percent
                ),
                None => info!("published heartbeat"),
            }
            next_heartbeat += EVENT_INTERVAL;
        }
    }
}

/// Publishes a heartbeat event and, if there is a reading of the sensor, an
/// environment event.
async fn publish_events(
    session: &mut hub::Session<'_, tls::Connection<'_>>,
    device_id: &str,
    reading: Option<dht11::Reading>,
) -> Result<(), hub::Error> {
    let now = clock::now().expect("the clock is synchronized before connecting");
    let heartbeat_id = event_id(now);
    let heartbeat = Heartbeat {
        uptime_seconds: Instant::now().as_secs(),
    };
    let environment_id = event_id(now);
    let environment = reading.map(Environment);
    let events: heapless::Vec<DeviceEvent<'_>, 2> =
        core::iter::once(info_event(&heartbeat_id, "heartbeat", &heartbeat, now))
            .chain(
                environment.as_ref().map(|environment| {
                    info_event(&environment_id, "environment", environment, now)
                }),
            )
            .collect();
    let action = PublishDeviceEventsAction {
        device_id: DeviceId(Str::new(device_id)),
        events: Seq::new(&events),
    };
    session
        .invoke::<Empty>("devices_PublishEvents", &action)
        .await?;
    Ok(())
}

/// Creates an event of severity info.
fn info_event<'a>(
    event_id: &'a str,
    category: &'a str,
    body: &'a dyn Encode,
    now: Timestamp,
) -> DeviceEvent<'a> {
    DeviceEvent {
        event_id: DeviceEventId(Str::new(event_id)),
        category: Some(Str::new(category)),
        severity: DeviceEventSeverity::Info,
        body: Json::from_value(body),
        attributes: Map::new(&[]),
        emitted_at: now,
        received_at: None,
    }
}

/// Generates the ID of an event emitted at `now`, for instance,
/// `d_ev_20261120-1435-4Kd9xQm2`.
fn event_id(now: Timestamp) -> heapless::String<32> {
    const BASE58: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let mut time = heapless::String::<32>::new();
    write!(time, "{now}").expect("timestamps fit into 32 bytes");
    let mut id = heapless::String::new();
    write!(
        id,
        "d_ev_{}{}{}-{}{}-",
        &time[0..4],
        &time[5..7],
        &time[8..10],
        &time[11..13],
        &time[14..16]
    )
    .expect("event IDs fit into 32 bytes");
    while id.len() < 27 {
        // Rejection sampling keeps the digits uniformly distributed.
        let digit = usize::from(RoscRng::next_u8() & 63);
        if let Some(digit) = BASE58.get(digit) {
            id.push(char::from(*digit))
                .expect("event IDs fit into 32 bytes");
        }
    }
    id
}

/// Body of heartbeat events.
struct Heartbeat {
    uptime_seconds: u64,
}

impl Encode for Heartbeat {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        let mut object = writer.object()?;
        self.uptime_seconds.encode(object.field("uptimeSeconds")?)?;
        Str::new(config::FIRMWARE_VERSION).encode(object.field("firmwareVersion")?)?;
        object.end()
    }
}

/// Body of environment events.
struct Environment(dht11::Reading);

impl Encode for Environment {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        let mut object = writer.object()?;
        self.0
            .temperature_celsius
            .encode(object.field("temperatureCelsius")?)?;
        self.0
            .humidity_percent
            .encode(object.field("humidityPercent")?)?;
        object.end()
    }
}

#[embassy_executor::task]
async fn logger_task(driver: usb::Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}
