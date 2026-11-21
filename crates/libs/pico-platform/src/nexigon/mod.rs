//! Connection of a Pico W to Nexigon Hub, enabled by the `nexigon` feature.
//!
//! [`run`] takes the [`Config`] of the device, the [`Peripherals`] that the
//! connection needs, and an [`Application`]. It joins a Wi-Fi network,
//! synchronizes the clock with SNTP, and connects to Nexigon Hub over TLS with
//! the device certificate. It learns the device ID with `actor_GetActor`,
//! describes the commands of the device, and checks for updates. Every 30
//! seconds, it publishes a heartbeat event and the event of the application.
//!
//! A firmware creates its [`Config`] with
//! [`nexigon_config!`](crate::nexigon_config). The module uses
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
use embassy_rp::peripherals::FLASH;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::Timer;
use log::error;
use log::info;
use log::warn;
pub use sidex_embedded;
use sidex_embedded::Encode;
use sidex_embedded::EncodeError;
use sidex_embedded::json::Writer;
use static_cell::StaticCell;

use self::api::actor::Actor;
use self::api::actor::GetActorAction;
use self::api::actor::GetActorOutput;
pub use self::config::Config;
use self::firmware::Firmware;
pub use self::network::WifiPeripherals;

/// Interval between heartbeats.
const EVENT_INTERVAL: Duration = Duration::from_secs(30);

/// Interval between update checks.
const UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Delay before connecting again after a failure.
const RETRY_DELAY: Duration = Duration::from_secs(10);

/// Peripherals that the connection to Nexigon Hub needs.
pub struct Peripherals {
    /// Flash, which holds the firmware slots for updates.
    pub flash: Peri<'static, FLASH>,
    /// Peripherals of the Wi-Fi chip.
    pub wifi: WifiPeripherals,
}

/// Application on a device, which publishes an event of its own with every
/// heartbeat.
pub trait Application {
    /// Body of the events of the application.
    type Event: Encode;

    /// Category of the events of the application.
    const EVENT_CATEGORY: &'static str;

    /// Returns the event to publish with the next heartbeat, if any.
    fn next_event(&mut self) -> impl Future<Output = Option<Self::Event>>;
}

/// Event of an application that publishes only heartbeats.
pub enum NoEvent {}

impl Encode for NoEvent {
    fn encode(&self, _writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        match *self {}
    }
}

/// Application that publishes only heartbeats.
impl Application for () {
    type Event = NoEvent;

    const EVENT_CATEGORY: &'static str = "none";

    async fn next_event(&mut self) -> Option<NoEvent> {
        None
    }
}

/// Connects to Nexigon Hub, publishes events, runs commands, and installs
/// updates, reconnecting after failures.
pub async fn run<A: Application>(
    spawner: Spawner,
    config: &'static Config,
    peripherals: Peripherals,
    application: A,
) -> ! {
    let firmware = Firmware::new(peripherals.flash, config.release_key);
    if firmware.awaits_commit() {
        info!("running a new firmware, which commits itself once it reaches Nexigon Hub");
    }
    spawner.spawn(firmware::commit_deadline_task().unwrap());

    let (stack, control) = network::start(spawner, peripherals.wifi, config).await;

    static TLS_BUFFERS: StaticCell<tls::Buffers> = StaticCell::new();
    static DOWNLOAD_BUFFERS: StaticCell<tls::Buffers> = StaticCell::new();
    static HUB_BUFFERS: StaticCell<hub::Buffers> = StaticCell::new();
    let tls_buffers = TLS_BUFFERS.init(tls::Buffers::new());
    let hub_buffers = HUB_BUFFERS.init(hub::Buffers::new());
    let mut runtime = Runtime {
        config,
        stack,
        control,
        firmware,
        commands: commands::Device::new(),
        download_buffers: DOWNLOAD_BUFFERS.init(tls::Buffers::new()),
        application,
    };
    loop {
        network::ensure_up(stack, &mut runtime.control, config).await;
        while clock::unix_seconds().is_none() {
            match clock::sync(stack).await {
                Ok(()) => info!("synchronized clock: {:?}", clock::now()),
                Err(error) => {
                    warn!("failed to synchronize clock: {error:?}");
                    Timer::after(RETRY_DELAY).await;
                }
            }
        }

        info!("connecting to Nexigon Hub at {}", config.hub_host);
        let started = Instant::now();
        let connection = match tls::connect(stack, tls_buffers, config.hub_host, config).await {
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
        let seed = RoscRng.next_u32();
        let error = match hub::Session::open(connection, hub_buffers, seed, config).await {
            Ok(mut session) => {
                info!("connected in {} ms", started.elapsed().as_millis());
                match runtime.serve(&mut session).await {
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

/// State of the device that outlives sessions with Nexigon Hub.
struct Runtime<A> {
    config: &'static Config,
    stack: Stack<'static>,
    /// Control of the Wi-Fi chip, which also drives the LED.
    control: cyw43::Control<'static>,
    firmware: Firmware,
    commands: commands::Device,
    download_buffers: &'static mut tls::Buffers,
    application: A,
}

impl<A: Application> Runtime<A> {
    /// Serves a session: checks for updates every [`UPDATE_CHECK_INTERVAL`],
    /// publishes events every [`EVENT_INTERVAL`], and runs commands in
    /// between.
    async fn serve(
        &mut self,
        session: &mut hub::Session<'_, tls::Connection<'_>>,
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
                let outcome = ota::check(
                    session,
                    &device_id,
                    &mut self.firmware,
                    self.stack,
                    self.download_buffers,
                    self.config,
                )
                .await?;
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
                commands::run(session, &mut self.commands, &mut self.control).await?;
                if self.commands.take_update_check_request() {
                    next_update_check = Instant::now();
                }
            } else if Instant::now() >= next_heartbeat {
                let event = self.application.next_event().await;
                let extra = event
                    .as_ref()
                    .map(|event| (A::EVENT_CATEGORY, event as &dyn Encode));
                events::publish(session, &device_id, self.config.firmware_version, extra).await?;
                if extra.is_some() {
                    info!("published heartbeat and {} event", A::EVENT_CATEGORY);
                } else {
                    info!("published heartbeat");
                }
                next_heartbeat += EVENT_INTERVAL;
            }
        }
    }
}
