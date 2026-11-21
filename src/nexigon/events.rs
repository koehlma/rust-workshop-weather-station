//! Events that the device publishes with `devices_PublishEvents`.
//!
//! [`publish`] sends a heartbeat event and, if there is a reading of the
//! sensor, an environment event with the temperature and humidity.

use core::fmt::Write as _;

use embassy_rp::clocks::RoscRng;
use embassy_time::Instant;
use sidex_embedded::Encode;
use sidex_embedded::EncodeError;
use sidex_embedded::json::Writer;
use sidex_embedded::types::Json;
use sidex_embedded::types::Map;
use sidex_embedded::types::Seq;
use sidex_embedded::types::Str;
use sidex_embedded::types::Timestamp;

use super::api::devices::DeviceEvent;
use super::api::devices::DeviceEventId;
use super::api::devices::DeviceEventSeverity;
use super::api::devices::DeviceId;
use super::api::devices::PublishDeviceEventsAction;
use super::api::outputs::Empty;
use super::clock;
use super::hub;
use super::tls;
use crate::dht11;

/// Publishes a heartbeat event and, if there is a reading of the sensor, an
/// environment event.
pub async fn publish(
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
        Str::new(crate::FIRMWARE_VERSION).encode(object.field("firmwareVersion")?)?;
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
