//! Reception of measurements from the weather station.
//!
//! [`task`] receives measurements as UDP datagrams on [`PORT`]. A datagram
//! holds the body of an environment event of the weather station, the time
//! of the measurement in `measuredAt`, and a `sequence` number, which the
//! weather station increments with every datagram since its start. The task
//! accepts measurements only from the IP address that [`set_sensor_address`]
//! sets, and rejects repeated sequence numbers. It adds every accepted
//! measurement to the [`Storage`] and shows the latest one on the
//! [`Display`].
//!
//! A measurement without `measuredAt` gets the time of its reception. While
//! the clock of the touch display is not synchronized either, up to
//! [`UNTIMED_CAPACITY`] such measurements wait in memory.

use core::cell::Cell;
use core::fmt::Write as _;
use core::net::Ipv4Addr;

use embassy_net::IpAddress;
use embassy_net::IpEndpoint;
use embassy_net::Stack;
use embassy_net::udp::PacketMetadata;
use embassy_net::udp::UdpSocket;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::Instant;
use embassy_time::with_deadline;
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
use crate::storage;
use crate::storage::Storage;

/// UDP port on which the display receives measurements.
pub const PORT: u16 = 4242;

/// Maximum number of measurements that wait for the clock.
const UNTIMED_CAPACITY: usize = 16;

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
    display.show(&["Waiting for measurements"]);
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
    };
    loop {
        let received = match storage.checkpoint_deadline() {
            Some(deadline) => with_deadline(deadline, socket.recv_from(&mut datagram))
                .await
                .ok(),
            None => Some(socket.recv_from(&mut datagram).await),
        };
        match received {
            None => storage.checkpoint(),
            Some(Err(error)) => {
                warn!("failed to receive a datagram: {error:?}");
                REJECTED.add(1, Ordering::Relaxed);
                continue;
            }
            Some(Ok((length, metadata))) => {
                match receiver.accept(metadata.endpoint, &datagram[..length]) {
                    Some(measurement) => receiver.add(&mut storage, measurement),
                    None => {
                        REJECTED.add(1, Ordering::Relaxed);
                        continue;
                    }
                }
            }
        }
        receiver.show(&mut display, &storage);
    }
}

/// State of the reception.
struct Receiver {
    /// Sequence number of the latest accepted datagram.
    last_sequence: Option<u32>,
    /// Measurements that wait for the clock, with the time of their
    /// reception.
    untimed: Deque<(Instant, Measurement), UNTIMED_CAPACITY>,
    /// Latest accepted measurement.
    latest: Option<Measurement>,
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
                    storage.push(&waiting.record(now.saturating_sub(age) as u32));
                }
                storage.push(&measurement.record(time.unwrap_or(now as u32)));
            }
            None => match time {
                Some(time) => storage.push(&measurement.record(time)),
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

    /// Shows the latest measurement and the counts of the storage.
    fn show(&self, display: &mut Display, storage: &Storage) {
        let Some(latest) = &self.latest else {
            return;
        };
        let counts = storage::counts();
        let mut status = Line::new();
        // Lines that do not fit are cut off.
        let _ = if storage.has_failed() {
            write!(status, "SD card failed")
        } else {
            write!(
                status,
                "Stored {}, buffered {}",
                counts.stored, counts.buffered
            )
        };
        let lines = [
            value_line("Temperature", latest.temperature_celsius, 1, "°C"),
            value_line("Humidity", latest.humidity_percent, 0, "%"),
            value_line("UV index", latest.uv_index, 1, ""),
            value_line("Pressure", latest.pressure_hectopascals, 1, "hPa"),
            status,
        ];
        display.show(&lines.each_ref().map(Line::as_str));
    }
}

/// Line of text on the display.
type Line = String<32>;

/// Returns a line with `label` and `value`, or a dash if the value is missing.
fn value_line(label: &str, value: Option<f32>, precision: usize, unit: &str) -> Line {
    let mut line = Line::new();
    // Lines that do not fit are cut off.
    let _ = match value {
        Some(value) => write!(line, "{label:<13}{value:.precision$} {unit}"),
        None => write!(line, "{label:<13}-"),
    };
    line
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
