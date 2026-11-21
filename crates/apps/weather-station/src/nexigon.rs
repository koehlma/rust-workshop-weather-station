//! Connection of the weather station to Nexigon Hub, with the `nexigon`
//! feature.
//!
//! [`run`] connects the [`Station`] through `pico_platform::nexigon`, which
//! publishes every measurement as an environment event next to the
//! heartbeat. The configuration and the identity in `identity/` are embedded
//! at compile time.

use embassy_executor::Spawner;
use log::info;
use pico_platform::nexigon::Application;
use pico_platform::nexigon::Config;
use pico_platform::nexigon::Peripherals;
use pico_platform::nexigon::sidex_embedded::Encode;
use pico_platform::nexigon::sidex_embedded::EncodeError;
use pico_platform::nexigon::sidex_embedded::json::Writer;

use crate::sensors::Measurement;
use crate::station::Station;

/// Configuration and identity of the weather station.
static CONFIG: Config = pico_platform::nexigon_config!();

/// Connects the station to Nexigon Hub.
pub async fn run(spawner: Spawner, peripherals: Peripherals, station: Station) -> ! {
    pico_platform::nexigon::run(spawner, &CONFIG, peripherals, station).await
}

impl Application for Station {
    type Event = Environment;

    const EVENT_CATEGORY: &'static str = "environment";

    async fn next_event(&mut self) -> Option<Environment> {
        let measurement = self.measure().await;
        // Sensors that fail are logged as they are measured.
        if measurement.is_empty() {
            return None;
        }
        info!("measured {measurement}");
        Some(Environment(measurement))
    }
}

/// Body of environment events, with the fields of the sensors that answered.
pub struct Environment(Measurement);

impl Encode for Environment {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        let mut object = writer.object()?;
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
        object.end()
    }
}
