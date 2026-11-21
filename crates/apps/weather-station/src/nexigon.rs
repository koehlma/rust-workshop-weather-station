//! Connection of the weather station to Nexigon Hub, with the `nexigon`
//! feature.
//!
//! [`run`] connects the station through `pico_platform::nexigon`. Once the
//! device has joined the Wi-Fi network, [`measure_task`] measures every
//! [`MEASUREMENT_INTERVAL`], independently of the connection to Nexigon Hub.
//! It sends every measurement as a UDP datagram to [`DISPLAY_PORT`] of the
//! touch display, whose IP address the device property
//! [`CONFIG_PROPERTY`](WeatherStation::CONFIG_PROPERTY) configures:
//!
//! ```json
//! { "displayAddress": "10.10.2.28" }
//! ```
//!
//! The datagram holds the body of an environment event, the time of the
//! measurement in `measuredAt` once the clock is synchronized, and a
//! `sequence` number, which counts the datagrams since the start. The latest
//! measurement is also published as an environment event with the next
//! heartbeat. The configuration and the identity in `identity/` are embedded
//! at compile time.

use core::cell::Cell;
use core::net::Ipv4Addr;

use embassy_executor::Spawner;
use embassy_net::IpAddress;
use embassy_net::IpEndpoint;
use embassy_net::Stack;
use embassy_net::udp::PacketMetadata;
use embassy_net::udp::UdpSocket;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::Duration;
use embassy_time::Ticker;
use embassy_time::with_timeout;
use log::info;
use log::warn;
use pico_platform::nexigon::Application;
use pico_platform::nexigon::Config;
use pico_platform::nexigon::Peripherals;
use pico_platform::nexigon::sidex_embedded::Decode;
use pico_platform::nexigon::sidex_embedded::DecodeError;
use pico_platform::nexigon::sidex_embedded::Encode;
use pico_platform::nexigon::sidex_embedded::EncodeError;
use pico_platform::nexigon::sidex_embedded::json;
use pico_platform::nexigon::sidex_embedded::json::ObjectWriter;
use pico_platform::nexigon::sidex_embedded::json::Reader;
use pico_platform::nexigon::sidex_embedded::json::Writer;
use pico_platform::nexigon::sidex_embedded::types::Json;
use pico_platform::nexigon::sidex_embedded::types::Str;
use pico_platform::nexigon::sidex_embedded::types::Timestamp;

use crate::sensors::Measurement;
use crate::station::Station;

/// Configuration and identity of the weather station.
static CONFIG: Config = pico_platform::nexigon_config!();

/// Interval between measurements.
const MEASUREMENT_INTERVAL: Duration = Duration::from_secs(30);

/// UDP port on which the touch display receives measurements.
const DISPLAY_PORT: u16 = 4242;

/// Time after which sending a measurement gives up, for instance, because the
/// display does not answer ARP requests.
const SEND_TIMEOUT: Duration = Duration::from_secs(5);

/// IP address of the touch display, if configured.
static DISPLAY_ADDRESS: Mutex<CriticalSectionRawMutex, Cell<Option<Ipv4Addr>>> =
    Mutex::new(Cell::new(None));

/// Latest measurement that has not been published yet.
static LATEST: Mutex<CriticalSectionRawMutex, Cell<Option<Measurement>>> =
    Mutex::new(Cell::new(None));

/// Connects the station to Nexigon Hub.
pub async fn run(spawner: Spawner, peripherals: Peripherals, station: Station) -> ! {
    let application = WeatherStation {
        station: Some(station),
    };
    pico_platform::nexigon::run(spawner, &CONFIG, peripherals, application).await
}

/// Application of the weather station.
struct WeatherStation {
    /// Station, until [`measure_task`] takes it.
    station: Option<Station>,
}

impl Application for WeatherStation {
    type Event = Environment;

    const EVENT_CATEGORY: &'static str = "environment";

    const CONFIG_PROPERTY: Option<&'static str> = Some("weather-station.config");

    fn start(&mut self, spawner: Spawner, stack: Stack<'static>) {
        let station = self.station.take().expect("the application starts once");
        spawner.spawn(measure_task(station, stack).unwrap());
    }

    fn configure(&mut self, config: Option<Json<'_>>) {
        let address = config.and_then(|config| {
            let address = config
                .decode::<StationConfig>()
                .ok()
                .map(|config| config.display_address.as_ref().map(parse_address));
            match address {
                Some(Some(Some(address))) => Some(address),
                Some(None) => None,
                _ => {
                    warn!("invalid configuration: {config:?}");
                    None
                }
            }
        });
        let previous = DISPLAY_ADDRESS.lock(|display| display.replace(address));
        if address != previous {
            match address {
                Some(address) => info!("sending measurements to {address}:{DISPLAY_PORT}"),
                None => info!("no display configured"),
            }
        }
    }

    async fn next_event(&mut self) -> Option<Environment> {
        LATEST.lock(Cell::take).map(Environment)
    }
}

/// Measures every [`MEASUREMENT_INTERVAL`] and sends the measurements to the
/// display.
#[embassy_executor::task]
async fn measure_task(mut station: Station, stack: Stack<'static>) -> ! {
    let mut rx_meta = [PacketMetadata::EMPTY; 1];
    let mut rx_buffer = [0; 16];
    let mut tx_meta = [PacketMetadata::EMPTY; 2];
    let mut tx_buffer = [0; 512];
    let mut socket = UdpSocket::new(
        stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );
    socket
        .bind(0)
        .expect("the socket is not bound yet and port 0 picks a free port");
    let mut datagram = [0; 256];
    let mut sequence = 0;
    let mut ticker = Ticker::every(MEASUREMENT_INTERVAL);
    loop {
        let measured_at = pico_platform::nexigon::now();
        let measurement = station.measure().await;
        // Sensors that fail are logged as they are measured.
        if !measurement.is_empty() {
            info!("measured {measurement}");
            LATEST.lock(|latest| latest.set(Some(measurement)));
            if let Some(address) = DISPLAY_ADDRESS.lock(Cell::get) {
                let body = Datagram {
                    sequence,
                    measured_at,
                    environment: Environment(measurement),
                };
                sequence = sequence.wrapping_add(1);
                let length = json::to_slice(&body, &mut datagram)
                    .expect("measurements fit into the datagram");
                let endpoint = IpEndpoint::new(IpAddress::Ipv4(address), DISPLAY_PORT);
                match with_timeout(SEND_TIMEOUT, socket.send_to(&datagram[..length], endpoint))
                    .await
                {
                    Ok(Ok(())) => {}
                    Ok(Err(error)) => warn!("failed to send measurement to {endpoint}: {error:?}"),
                    Err(_) => warn!("timed out sending measurement to {endpoint}"),
                }
            }
        }
        ticker.next().await;
    }
}

/// Parses an IPv4 address, returning `None` if it is invalid.
fn parse_address(address: &Str<'_>) -> Option<Ipv4Addr> {
    address.as_str()?.parse().ok()
}

/// Configuration of the weather station.
struct StationConfig<'a> {
    /// IP address of the touch display.
    display_address: Option<Str<'a>>,
}

impl<'a> Decode<'a> for StationConfig<'a> {
    const IS_RECORD: bool = true;

    fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError> {
        let fields = reader.record(&[&["displayAddress"]])?;
        Ok(Self {
            display_address: fields.optional(0)?,
        })
    }
}

/// Body of the datagrams to the display.
struct Datagram {
    /// Number of the datagram since the start.
    sequence: u32,
    /// Time of the measurement, if the clock is synchronized.
    measured_at: Option<Timestamp>,
    environment: Environment,
}

impl Encode for Datagram {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        let mut object = writer.object()?;
        self.sequence.encode(object.field("sequence")?)?;
        if let Some(measured_at) = &self.measured_at {
            measured_at.encode(object.field("measuredAt")?)?;
        }
        self.environment.encode_fields(&mut object)?;
        object.end()
    }
}

/// Body of environment events, with the fields of the sensors that answered.
pub struct Environment(Measurement);

impl Encode for Environment {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        let mut object = writer.object()?;
        self.encode_fields(&mut object)?;
        object.end()
    }
}

impl Environment {
    /// Writes the fields of the sensors that answered to `object`.
    fn encode_fields(&self, object: &mut ObjectWriter<'_, '_>) -> Result<(), EncodeError> {
        if let Some(reading) = &self.0.dht11 {
            reading
                .temperature_celsius
                .encode(object.field("temperatureCelsius")?)?;
            reading
                .humidity_percent
                .encode(object.field("humidityPercent")?)?;
        }
        if let Some(reading) = &self.0.uv {
            reading.uv_index.encode(object.field("uvIndex")?)?;
            reading.volts.encode(object.field("uvVolts")?)?;
        }
        if let Some(reading) = &self.0.bmp280 {
            reading
                .pressure_hectopascals
                .encode(object.field("pressureHectopascals")?)?;
        }
        Ok(())
    }
}
