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
//! The [`Display`] shows the latest measurement and a plot of the history of
//! one [`Metric`], which changes every [`METRIC_INTERVAL`]. A tap on the touch
//! screen switches to the next [`Interval`], whose history the task then
//! reads from the storage.
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
use crate::screen::Screen;
use crate::storage;
use crate::storage::DAY_SECONDS;
use crate::storage::Storage;
use crate::touch::Touch;

/// UDP port on which the display receives measurements.
pub const PORT: u16 = 4242;

/// Maximum number of measurements that wait for the clock.
const UNTIMED_CAPACITY: usize = 16;

/// Time for which the plot shows a metric before it shows the next one.
const METRIC_INTERVAL: Duration = Duration::from_secs(10);

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
pub async fn task(
    stack: Stack<'static>,
    mut display: Display,
    mut touch: Touch,
    mut storage: Storage,
) -> ! {
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
        history: History::new(Interval::Day),
        loaded: false,
        metric: Metric::Temperature,
    };
    let mut next_metric = Instant::now() + METRIC_INTERVAL;
    loop {
        receiver.load(&mut storage, &mut display).await;
        receiver.show(&mut display, &storage, None);
        let deadline = storage
            .checkpoint_deadline()
            .map_or(next_metric, |checkpoint| checkpoint.min(next_metric));
        let event = select3(
            socket.recv_from(&mut datagram),
            Timer::at(deadline),
            touch.tap(),
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
                if storage
                    .checkpoint_deadline()
                    .is_some_and(|checkpoint| checkpoint <= Instant::now())
                {
                    storage.checkpoint();
                }
                if next_metric <= Instant::now() {
                    receiver.metric = receiver.metric.next();
                    next_metric = Instant::now() + METRIC_INTERVAL;
                }
            }
            Either3::Third(()) => {
                let interval = receiver.history.interval().next();
                info!("showing the last {}", interval.label());
                receiver.history.reset(interval);
                receiver.loaded = false;
                next_metric = Instant::now() + METRIC_INTERVAL;
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
    /// Latest accepted measurement.
    latest: Option<Measurement>,
    /// Measurements of the interval that the plot shows.
    history: History,
    /// Indicates whether the history holds the measurements of the storage.
    loaded: bool,
    /// Metric that the plot shows.
    metric: Metric,
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
        self.latest = Some(measurement);
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
                    let _ = self.untimed.push_back((Instant::now(), measurement));
                }
            },
        }
    }

    /// Adds `record` to `storage` and, once loaded, to the history.
    fn store(&mut self, storage: &mut Storage, record: &Record) {
        storage.push(record);
        if self.loaded {
            self.history.add(record);
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
        self.show(display, storage, Some("Reading the SD card"));
        let now = now as u32;
        let start = now.saturating_sub(self.history.interval().seconds());
        let started = Instant::now();
        for day in start / DAY_SECONDS..=now / DAY_SECONDS {
            let history = &mut self.history;
            let result = storage.read_day(day, |record| {
                if record.time >= start {
                    history.add(record);
                }
            });
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

    /// Shows the latest measurement, the plot, and `note` or the state of
    /// `storage`.
    fn show(&self, display: &mut Display, storage: &Storage, note: Option<&str>) {
        let columns = match nexigon::unix_seconds() {
            Some(now) if self.loaded => self.history.columns(self.metric, now as u32),
            _ => [None; BUCKETS],
        };
        let value = |metric: Metric| self.latest.and_then(|latest| latest.value(metric));
        let mut details = String::<64>::new();
        for metric in Metric::ALL
            .into_iter()
            .filter(|metric| *metric != self.metric)
        {
            let _ = match (metric, value(metric)) {
                (Metric::UvIndex, Some(value)) => write!(details, "UV {value:.1}   "),
                (_, Some(value)) => write!(
                    details,
                    "{value:.precision$} {}   ",
                    metric.unit(),
                    precision = metric.precision()
                ),
                (_, None) => write!(details, "{} -   ", metric.label()),
            };
        }
        let counts = storage::counts();
        let mut status = String::<40>::new();
        // Text that does not fit is cut off.
        let _ = match note {
            Some(note) => status.write_str(note),
            None if storage.has_failed() => status.write_str("SD card failed"),
            None => write!(
                status,
                "Stored {}, buffered {}",
                counts.stored, counts.buffered
            ),
        };
        display.show(&Screen {
            metric: self.metric,
            value: value(self.metric),
            details: details.trim_end(),
            interval: self.history.interval(),
            columns: &columns,
            status: &status,
            firmware_version: crate::CONFIG.firmware_version,
        });
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
