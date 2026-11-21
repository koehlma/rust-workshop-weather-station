//! Reception of measurements from the weather station.
//!
//! [`task`] receives measurements as UDP datagrams on [`PORT`]. A datagram
//! holds the body of an environment event of the weather station. The task
//! accepts measurements only from the IP address that [`set_sensor_address`]
//! sets. It appends every accepted measurement as a line of JSON to the
//! [`Storage`] and shows it on the [`Display`]:
//!
//! ```json
//! {"receivedAt":"2026-11-21T14:00:00Z","sender":"10.10.2.30:49152","measurement":{"temperatureCelsius":21.0}}
//! ```
//!
//! `receivedAt` is missing while the clock is not synchronized.

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
use heapless::String;
use log::info;
use log::warn;
use pico_platform::nexigon;
use pico_platform::nexigon::sidex_embedded::Decode;
use pico_platform::nexigon::sidex_embedded::DecodeError;
use pico_platform::nexigon::sidex_embedded::Encode;
use pico_platform::nexigon::sidex_embedded::EncodeError;
use pico_platform::nexigon::sidex_embedded::json;
use pico_platform::nexigon::sidex_embedded::json::Reader;
use pico_platform::nexigon::sidex_embedded::json::Writer;
use pico_platform::nexigon::sidex_embedded::types::Json;
use pico_platform::nexigon::sidex_embedded::types::Str;
use pico_platform::nexigon::sidex_embedded::types::Timestamp;
use portable_atomic::AtomicU32;
use portable_atomic::Ordering;

use crate::display::Display;
use crate::storage::Storage;

/// UDP port on which the display receives measurements.
pub const PORT: u16 = 4242;

/// IP address of the weather station, if configured.
static SENSOR_ADDRESS: Mutex<CriticalSectionRawMutex, Cell<Option<Ipv4Addr>>> =
    Mutex::new(Cell::new(None));

/// Number of measurements stored since the start.
static STORED: AtomicU32 = AtomicU32::new(0);

/// Number of datagrams rejected since the start, because they came from
/// another address or did not hold a measurement.
static REJECTED: AtomicU32 = AtomicU32::new(0);

/// Number of measurements that could not be stored since the start.
static FAILED: AtomicU32 = AtomicU32::new(0);

/// Counts of the measurements since the start.
#[derive(Debug, Clone, Copy)]
pub struct Counts {
    /// Measurements stored on the card.
    pub stored: u32,
    /// Datagrams rejected, because they came from another address or did not
    /// hold a measurement.
    pub rejected: u32,
    /// Measurements that could not be stored on the card.
    pub failed: u32,
}

/// Returns the counts of the measurements since the start.
pub fn counts() -> Counts {
    Counts {
        stored: STORED.load(Ordering::Relaxed),
        rejected: REJECTED.load(Ordering::Relaxed),
        failed: FAILED.load(Ordering::Relaxed),
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
    let mut line = [0; 512];
    loop {
        let (length, metadata) = match socket.recv_from(&mut datagram).await {
            Ok(received) => received,
            Err(error) => {
                warn!("failed to receive a datagram: {error:?}");
                REJECTED.add(1, Ordering::Relaxed);
                continue;
            }
        };
        let sender = metadata.endpoint;
        let Some(measurement) = accept(sender, &datagram[..length]) else {
            REJECTED.add(1, Ordering::Relaxed);
            continue;
        };
        let record = Record {
            received_at: nexigon::now(),
            sender,
            measurement: measurement.json,
        };
        let length = json::to_slice(&record, &mut line[..511])
            .expect("records of datagrams that fit into the buffer fit into the line");
        line[length] = b'\n';
        let status = match storage.append(&line[..=length]) {
            Ok(()) => {
                STORED.add(1, Ordering::Relaxed);
                None
            }
            Err(error) => {
                warn!("failed to store the measurement: {error}");
                FAILED.add(1, Ordering::Relaxed);
                Some("SD card failed")
            }
        };
        show(&mut display, &measurement.values, status);
    }
}

/// Returns the measurement of `datagram`, or `None` if it is not from the
/// weather station or holds no measurement.
fn accept<'a>(sender: IpEndpoint, datagram: &'a [u8]) -> Option<Measurement<'a>> {
    let Some(address) = SENSOR_ADDRESS.lock(Cell::get) else {
        warn!("rejected a datagram from {sender}: no weather station configured");
        return None;
    };
    if sender.addr != IpAddress::Ipv4(address) {
        warn!("rejected a datagram from {sender}: expected {address}");
        return None;
    }
    let measurement = core::str::from_utf8(datagram)
        .ok()
        .and_then(|text| Json::parse(text).ok())
        .and_then(|json| {
            let values = json.decode().ok()?;
            Some(Measurement { json, values })
        });
    if measurement.is_none() {
        warn!("rejected a datagram from {sender}: no measurement");
    }
    measurement
}

/// Shows the values of the latest measurement and the `status` of the card,
/// or the number of stored measurements if it is `None`.
fn show(display: &mut Display, values: &Values, status: Option<&str>) {
    let mut storage = Line::new();
    // Lines that do not fit are cut off.
    let _ = match status {
        Some(status) => storage.write_str(status),
        None => write!(storage, "{} stored on SD card", counts().stored),
    };
    let lines = [
        value_line("Temperature", values.temperature_celsius, 1, "°C"),
        value_line("Humidity", values.humidity_percent, 0, "%"),
        value_line("UV index", values.uv_index, 1, ""),
        value_line("Pressure", values.pressure_hectopascals, 1, "hPa"),
        storage,
    ];
    display.show(&lines.each_ref().map(Line::as_str));
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
struct Measurement<'a> {
    /// Body of the datagram.
    json: Json<'a>,
    /// Values of the body that the display shows.
    values: Values,
}

/// Values of a measurement that the display shows.
struct Values {
    temperature_celsius: Option<f32>,
    humidity_percent: Option<f32>,
    uv_index: Option<f32>,
    pressure_hectopascals: Option<f32>,
}

impl<'a> Decode<'a> for Values {
    const IS_RECORD: bool = true;

    fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError> {
        let fields = reader.record(&[
            &["temperatureCelsius"],
            &["humidityPercent"],
            &["uvIndex"],
            &["pressureHectopascals"],
        ])?;
        Ok(Self {
            temperature_celsius: fields.optional(0)?,
            humidity_percent: fields.optional(1)?,
            uv_index: fields.optional(2)?,
            pressure_hectopascals: fields.optional(3)?,
        })
    }
}

/// Line of the file on the card.
struct Record<'a> {
    received_at: Option<Timestamp>,
    sender: IpEndpoint,
    measurement: Json<'a>,
}

impl Encode for Record<'_> {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        let mut object = writer.object()?;
        if let Some(received_at) = &self.received_at {
            received_at.encode(object.field("receivedAt")?)?;
        }
        let mut sender = String::<21>::new();
        write!(sender, "{}", self.sender).expect("IPv4 endpoints have at most 21 bytes");
        Str::new(&sender).encode(object.field("sender")?)?;
        self.measurement.encode(object.field("measurement")?)?;
        object.end()
    }
}
