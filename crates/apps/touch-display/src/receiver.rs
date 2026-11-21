//! Reception, storage, and display of the measurements of the weather
//! station.
//!
//! [`task`] receives measurements as UDP datagrams on [`PORT`]. A datagram
//! holds the body of an environment event of the weather station, the time
//! of the measurement in `measuredAt`, and a `sequence` number, which the
//! weather station increments with every datagram since its start. The task
//! accepts measurements only from the IP address that [`set_sensor_address`]
//! sets, and rejects repeated sequence numbers. It adds every accepted
//! measurement to the [`Storage`] and the [`History`].
//!
//! The [`Display`] rotates through pages: the dashboard with a plot of the
//! history of each [`Metric`] for [`PLOT_DURATION`], and then an animated
//! [`Scene`] of the [`Weather`] for [`SCENE_DURATION`]. On the dashboard, a
//! tap on a tile plots its metric, a tap on the plot shows the next page, and
//! a tap on a chip plots its [`Interval`], whose history the task then reads
//! from the storage. A tap on the scene returns to the dashboard. After a
//! tap, the page stays for [`HOLD_AFTER_TAP`].
//!
//! A measurement without `measuredAt` gets the time of its reception. While
//! the clock of the touch display is not synchronized either, up to
//! [`UNTIMED_CAPACITY`] such measurements wait in memory.

use core::cell::Cell;
use core::fmt::Write as _;
use core::net::Ipv4Addr;

use embassy_futures::select::Either3;
use embassy_futures::select::select3;
use embassy_futures::yield_now;
use embassy_net::IpAddress;
use embassy_net::IpEndpoint;
use embassy_net::Stack;
use embassy_net::udp::PacketMetadata;
use embassy_net::udp::UdpSocket;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::Timer;
use embedded_graphics::Drawable;
use heapless::Deque;
use heapless::String;
use log::info;
use log::warn;
use pico_platform::nexigon;
use pico_platform::nexigon::sidex_embedded::Decode;
use pico_platform::nexigon::sidex_embedded::DecodeError;
use pico_platform::nexigon::sidex_embedded::json::Reader;
use pico_platform::nexigon::sidex_embedded::types::Json;
use pico_platform::nexigon::sidex_embedded::types::Timestamp;
use portable_atomic::AtomicU32;
use portable_atomic::Ordering;

use crate::display::Display;
use crate::format::Record;
use crate::history::BUCKETS;
use crate::history::History;
use crate::history::Interval;
use crate::history::Metric;
use crate::scene::Scene;
use crate::screen;
use crate::screen::Frame;
use crate::screen::Screen;
use crate::screen::Status;
use crate::screen::Target;
use crate::storage::DAY_SECONDS;
use crate::storage::Storage;
use crate::touch;
use crate::weather::PressureTrend;
use crate::weather::TREND_SECONDS;
use crate::weather::Weather;

/// UDP port on which the display receives measurements.
pub const PORT: u16 = 4242;

/// Maximum number of measurements that wait for the clock.
const UNTIMED_CAPACITY: usize = 16;

/// Time for which the dashboard plots a metric before the next page.
const PLOT_DURATION: Duration = Duration::from_secs(10);

/// Time for which the scene shows before the next page.
const SCENE_DURATION: Duration = Duration::from_secs(20);

/// Time between two frames of the scene, at most 10 frames per second.
const FRAME_INTERVAL: Duration = Duration::from_millis(100);

/// Shortest pause after a frame of the scene, which drawing blocks, so that
/// the network and the watchdog keep running when frames take longer than
/// [`FRAME_INTERVAL`].
const FRAME_PAUSE: Duration = Duration::from_millis(20);

/// Time for which a page stays after a tap.
const HOLD_AFTER_TAP: Duration = Duration::from_secs(60);

/// Age after which measurements count as outdated, four missed measurements.
const STALE_AFTER: Duration = Duration::from_secs(150);

/// IP address of the weather station, if configured.
static SENSOR_ADDRESS: Mutex<CriticalSectionRawMutex, Cell<Option<Ipv4Addr>>> =
    Mutex::new(Cell::new(None));

/// Number of datagrams rejected since the start.
static REJECTED: AtomicU32 = AtomicU32::new(0);

/// Number of datagrams lost since the start, according to the gaps in their
/// sequence numbers.
static LOST: AtomicU32 = AtomicU32::new(0);

/// Counts of the datagrams since the start.
#[derive(Debug, Clone, Copy)]
pub struct Counts {
    /// Datagrams rejected because they came from another address, repeated a
    /// sequence number, or did not hold a measurement.
    pub rejected: u32,
    /// Datagrams lost, according to the gaps in their sequence numbers.
    pub lost: u32,
}

/// Returns the counts of the datagrams since the start.
pub fn counts() -> Counts {
    Counts {
        rejected: REJECTED.load(Ordering::Relaxed),
        lost: LOST.load(Ordering::Relaxed),
    }
}

/// Accepts measurements only from `address`, or from no address if it is
/// `None`.
pub fn set_sensor_address(address: Option<Ipv4Addr>) {
    let previous = SENSOR_ADDRESS.lock(|sensor| sensor.replace(address));
    if address != previous {
        match address {
            Some(address) => info!("accepting measurements from {address}"),
            None => info!("no weather station configured"),
        }
    }
}

/// Receives measurements, stores them, and shows them.
#[embassy_executor::task]
pub async fn task(stack: Stack<'static>, mut display: Display, mut storage: Storage) -> ! {
    let mut rx_meta = [PacketMetadata::EMPTY; 4];
    let mut rx_buffer = [0; 1024];
    let mut tx_meta = [PacketMetadata::EMPTY; 1];
    let mut tx_buffer = [0; 16];
    let mut socket = UdpSocket::new(
        stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );
    socket.bind(PORT).expect("the socket is not bound yet");
    info!("receiving measurements on UDP port {PORT}");
    let mut datagram = [0; 256];
    let mut receiver = Receiver {
        last_sequence: None,
        untimed: Deque::new(),
        latest: None,
        rejected: None,
        history: History::new(Interval::Day),
        trend: PressureTrend::new(),
        loaded: false,
        page: Page::Plot(Metric::Temperature),
        scene_started: Instant::now(),
        hold_until: Instant::now(),
    };
    let mut next_page = Instant::now() + receiver.page.duration();
    loop {
        receiver.load(&mut storage, &mut display).await;
        let mut deadline = next_page;
        match receiver.page {
            Page::Plot(_) => receiver.show(&mut display, &storage, false).await,
            Page::Scene => {
                let started = Instant::now();
                receiver.show_scene(&mut display).await;
                let drawn = Instant::now();
                // Log the first frame of every showing.
                if started - receiver.scene_started < FRAME_INTERVAL {
                    info!(
                        "drew a frame of the scene in {} ms",
                        (drawn - started).as_millis()
                    );
                }
                deadline = deadline.min((started + FRAME_INTERVAL).max(drawn + FRAME_PAUSE));
            }
        }
        if let Some(checkpoint) = storage.checkpoint_deadline() {
            deadline = deadline.min(checkpoint);
        }
        let event = select3(
            socket.recv_from(&mut datagram),
            Timer::at(deadline),
            touch::TAPS.receive(),
        )
        .await;
        match event {
            Either3::First(Ok((length, metadata))) => {
                match receiver.accept(metadata.endpoint, &datagram[..length]) {
                    Some(measurement) => receiver.add(&mut storage, measurement),
                    None => REJECTED.add(1, Ordering::Relaxed),
                }
            }
            Either3::First(Err(error)) => {
                warn!("failed to receive a datagram: {error:?}");
                REJECTED.add(1, Ordering::Relaxed);
            }
            Either3::Second(()) => {
                let now = Instant::now();
                if storage
                    .checkpoint_deadline()
                    .is_some_and(|checkpoint| checkpoint <= now)
                {
                    storage.checkpoint();
                }
                if next_page <= now {
                    if receiver.hold_until <= now {
                        receiver.turn_to(receiver.page.next());
                    }
                    next_page = now + receiver.page.duration();
                }
            }
            Either3::Third(point) => {
                let target = screen::target_at(point);
                info!("tap at ({}, {}) selects {target:?}", point.x, point.y);
                receiver.select(target);
            }
        }
    }
}

/// State of the reception and the display.
struct Receiver {
    /// Sequence number of the latest accepted datagram.
    last_sequence: Option<u32>,
    /// Measurements that wait for the clock, with the time of their
    /// reception.
    untimed: Deque<(Instant, Measurement), UNTIMED_CAPACITY>,
    /// Latest accepted measurement with the time of its reception.
    latest: Option<(Instant, Measurement)>,
    /// Latest datagram from another address than the weather station, with
    /// the time of its reception.
    rejected: Option<(Instant, IpAddress)>,
    /// Measurements of the interval that the plot shows.
    history: History,
    /// Air pressure of the last hours.
    trend: PressureTrend,
    /// Indicates whether the history and the trend hold the measurements of
    /// the storage.
    loaded: bool,
    /// Page that the display shows.
    page: Page,
    /// Time at which the scene started to show.
    scene_started: Instant,
    /// Time until which the page stays.
    hold_until: Instant,
}

/// Page of the display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    /// Dashboard with a plot of a metric.
    Plot(Metric),
    /// Animated scene of the weather.
    Scene,
}

impl Page {
    /// Returns the page that follows this one.
    fn next(self) -> Self {
        match self {
            Self::Plot(Metric::Pressure) => Self::Scene,
            Self::Plot(metric) => Self::Plot(metric.next()),
            Self::Scene => Self::Plot(Metric::Temperature),
        }
    }

    /// Returns the time for which the page shows before the next one.
    fn duration(self) -> Duration {
        match self {
            Self::Plot(_) => PLOT_DURATION,
            Self::Scene => SCENE_DURATION,
        }
    }
}

impl Receiver {
    /// Returns the measurement of `datagram`, or `None` if it is not from the
    /// weather station, repeats the sequence number of the previous datagram,
    /// or holds no measurement.
    fn accept(&mut self, sender: IpEndpoint, datagram: &[u8]) -> Option<Measurement> {
        let Some(address) = SENSOR_ADDRESS.lock(Cell::get) else {
            warn!("rejected a datagram from {sender}: no weather station configured");
            return None;
        };
        if sender.addr != IpAddress::Ipv4(address) {
            warn!("rejected a datagram from {sender}: expected {address}");
            self.rejected = Some((Instant::now(), sender.addr));
            return None;
        }
        let Some(measurement) = core::str::from_utf8(datagram)
            .ok()
            .and_then(|text| Json::parse(text).ok())
            .and_then(|json| json.decode::<Measurement>().ok())
        else {
            warn!("rejected a datagram from {sender}: no measurement");
            return None;
        };
        if let Some(sequence) = measurement.sequence {
            match self.last_sequence {
                Some(last) if sequence == last => {
                    warn!("rejected a repeated datagram {sequence} from {sender}");
                    return None;
                }
                // A smaller sequence number means that the weather station
                // has restarted.
                Some(last) if sequence > last.saturating_add(1) => {
                    let lost = sequence - last - 1;
                    warn!("lost {lost} datagrams before datagram {sequence}");
                    LOST.add(lost, Ordering::Relaxed);
                }
                _ => {}
            }
            self.last_sequence = Some(sequence);
        }
        Some(measurement)
    }

    /// Adds `measurement` to `storage`, after the measurements that wait for
    /// the clock.
    fn add(&mut self, storage: &mut Storage, measurement: Measurement) {
        self.latest = Some((Instant::now(), measurement));
        let time = measurement
            .measured_at
            .and_then(|time| u32::try_from(time.unix_seconds()).ok());
        match nexigon::unix_seconds() {
            Some(now) => {
                while let Some((received, waiting)) = self.untimed.pop_front() {
                    let age = received.elapsed().as_secs();
                    self.store(storage, &waiting.record(now.saturating_sub(age) as u32));
                }
                self.store(storage, &measurement.record(time.unwrap_or(now as u32)));
            }
            None => match time {
                Some(time) => self.store(storage, &measurement.record(time)),
                None => {
                    if self.untimed.is_full() {
                        warn!("dropped a measurement without time");
                        self.untimed.pop_front();
                    }
                    self.untimed
                        .push_back((Instant::now(), measurement))
                        .expect("the queue is not full after dropping the oldest measurement");
                }
            },
        }
    }

    /// Adds `record` to `storage` and, once loaded, to the history and the
    /// trend.
    fn store(&mut self, storage: &mut Storage, record: &Record) {
        storage.push(record);
        if self.loaded {
            self.history.add(record);
            self.trend.add(record);
        }
    }

    /// Applies a tap on `target`.
    fn select(&mut self, target: Target) {
        let page = match (self.page, target) {
            (Page::Scene, _) => self.page.next(),
            (Page::Plot(_), Target::Metric(metric)) => Page::Plot(metric),
            (Page::Plot(_), Target::Plot) => self.page.next(),
            (Page::Plot(_), Target::Interval(interval)) => {
                if interval != self.history.interval() {
                    self.history.reset(interval);
                    self.loaded = false;
                }
                self.page
            }
        };
        self.turn_to(page);
        self.hold_until = Instant::now() + HOLD_AFTER_TAP;
    }

    /// Shows `page`.
    fn turn_to(&mut self, page: Page) {
        if page == Page::Scene && self.page != Page::Scene {
            self.scene_started = Instant::now();
        }
        self.page = page;
    }

    /// Returns the metric that the dashboard plots.
    fn metric(&self) -> Metric {
        match self.page {
            Page::Plot(metric) => metric,
            Page::Scene => Metric::Temperature,
        }
    }

    /// Reads the history from `storage`, once the clock is synchronized, if
    /// it has not been read yet.
    ///
    /// Yields after every day, so that the network keeps running while it
    /// reads a long interval.
    async fn load(&mut self, storage: &mut Storage, display: &mut Display) {
        if self.loaded {
            return;
        }
        let Some(now) = nexigon::unix_seconds() else {
            return;
        };
        self.show(display, storage, true).await;
        let now = now as u32;
        let history_start = now.saturating_sub(self.history.interval().seconds());
        let trend_start = now.saturating_sub(TREND_SECONDS);
        let started = Instant::now();
        for day in history_start.min(trend_start) / DAY_SECONDS..=now / DAY_SECONDS {
            let (history, trend) = (&mut self.history, &mut self.trend);
            let result = storage.read_day(day, |record| {
                if record.time >= history_start {
                    history.add(record);
                }
                if record.time >= trend_start {
                    trend.add(record);
                }
            });
            // The storage logs the error, and the screen shows it.
            if result.is_err() {
                break;
            }
            yield_now().await;
        }
        info!(
            "read the last {} in {} ms",
            self.history.interval().label(),
            started.elapsed().as_millis()
        );
        self.loaded = true;
    }

    /// Shows the latest measurements, the plot, and the state of the device,
    /// which is reading the history if `loading` is set.
    async fn show(&self, display: &mut Display, storage: &Storage, loading: bool) {
        let now = nexigon::unix_seconds();
        let columns = match now {
            Some(now) if self.loaded => self.history.columns(self.metric(), now as u32),
            _ => [None; BUCKETS],
        };
        let notice = if loading {
            Some("Reading the SD card")
        } else if now.is_none() {
            Some("Waiting for the clock")
        } else {
            None
        };
        let mut status_text = String::<48>::new();
        let status = self.status(storage, loading, &mut status_text);
        let values = Metric::ALL.map(|metric| {
            self.latest
                .and_then(|(_, measurement)| measurement.value(metric))
        });
        let screen = Screen {
            values,
            stale: self.is_stale(),
            metric: self.metric(),
            interval: self.history.interval(),
            columns: &columns,
            notice,
            hint: SENSOR_ADDRESS
                .lock(Cell::get)
                .is_none()
                .then_some("Set sensorAddress in touch-display.config"),
            status,
            status_text: &status_text,
            firmware_version: crate::CONFIG.firmware_version,
        };
        let frame = Frame::new(&screen);
        display
            .paint(|strip| {
                let Ok(()) = frame.draw(strip);
            })
            .await;
    }

    /// Shows the current frame of the scene.
    async fn show_scene(&self, display: &mut Display) {
        let latest = self.latest.map(|(_, measurement)| measurement);
        let value = |metric| latest.and_then(|measurement| measurement.value(metric));
        let weather = Weather {
            temperature_celsius: value(Metric::Temperature),
            humidity_percent: value(Metric::Humidity),
            uv_index: value(Metric::UvIndex),
            pressure_hectopascals: value(Metric::Pressure),
            trend: nexigon::unix_seconds().and_then(|now| self.trend.trend(now as u32)),
        };
        let time = self.scene_started.elapsed().as_millis() as u32;
        let scene = Scene::new(weather, self.is_stale(), time);
        display.paint(|strip| scene.paint(strip)).await;
    }

    /// Indicates whether the latest measurement is missing or outdated.
    fn is_stale(&self) -> bool {
        self.latest
            .is_none_or(|(received, _)| received.elapsed() > STALE_AFTER)
    }

    /// Describes the state of the device in `text` and returns its kind.
    fn status(&self, storage: &Storage, loading: bool, text: &mut String<48>) -> Status {
        // Texts are shorter than the capacity.
        let (status, _) = if loading {
            let interval = self.history.interval().label();
            (
                Status::Busy,
                write!(text, "Reading {interval} from the SD card"),
            )
        } else if storage.has_failed() {
            (
                Status::Error,
                text.write_str("SD card failed, check the card"),
            )
        } else if SENSOR_ADDRESS.lock(Cell::get).is_none() {
            (
                Status::Warning,
                text.write_str("No weather station configured"),
            )
        } else if let Some((rejected, sender)) = self.rejected
            && rejected.elapsed() < STALE_AFTER
            && self.latest.is_none_or(|(received, _)| received < rejected)
        {
            (Status::Warning, write!(text, "Ignoring data from {sender}"))
        } else {
            match self.latest.map(|(received, _)| received.elapsed()) {
                None => (
                    Status::Warning,
                    text.write_str("Waiting for the weather station"),
                ),
                Some(age) if age > STALE_AFTER => (
                    Status::Warning,
                    write!(text, "No data for {} min", age.as_secs() / 60),
                ),
                Some(age) => (
                    Status::Ok,
                    write!(text, "Recording \u{b7} updated {} s ago", age.as_secs()),
                ),
            }
        };
        status
    }
}

/// Measurement of a datagram.
#[derive(Debug, Clone, Copy)]
struct Measurement {
    sequence: Option<u32>,
    measured_at: Option<Timestamp>,
    temperature_celsius: Option<f32>,
    humidity_percent: Option<f32>,
    pressure_hectopascals: Option<f32>,
    uv_volts: Option<f32>,
    /// UV index, which the display shows but the storage derives from the
    /// voltage.
    uv_index: Option<f32>,
}

impl Measurement {
    /// Returns the record of the measurement at `time` in Unix seconds.
    fn record(&self, time: u32) -> Record {
        Record {
            time,
            temperature_celsius: self.temperature_celsius,
            humidity_percent: self.humidity_percent,
            pressure_hectopascals: self.pressure_hectopascals,
            uv_volts: self.uv_volts,
        }
    }

    /// Returns the value of `metric`, if present.
    fn value(&self, metric: Metric) -> Option<f32> {
        match metric {
            Metric::Temperature => self.temperature_celsius,
            Metric::Humidity => self.humidity_percent,
            Metric::UvIndex => self.uv_index,
            Metric::Pressure => self.pressure_hectopascals,
        }
    }
}

impl<'a> Decode<'a> for Measurement {
    const IS_RECORD: bool = true;

    fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError> {
        let fields = reader.record(&[
            &["sequence"],
            &["measuredAt"],
            &["temperatureCelsius"],
            &["humidityPercent"],
            &["pressureHectopascals"],
            &["uvVolts"],
            &["uvIndex"],
        ])?;
        Ok(Self {
            sequence: fields.optional(0)?,
            measured_at: fields.optional(1)?,
            temperature_celsius: fields.optional(2)?,
            humidity_percent: fields.optional(3)?,
            pressure_hectopascals: fields.optional(4)?,
            uv_volts: fields.optional(5)?,
            uv_index: fields.optional(6)?,
        })
    }
}
