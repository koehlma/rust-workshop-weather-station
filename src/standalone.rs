//! Operation without Nexigon, when the `nexigon` feature is disabled.
//!
//! [`run`] logs a reading of the sensor every [`READING_INTERVAL`].

use embassy_time::Duration;
use embassy_time::Timer;
use log::info;
use log::warn;

use crate::dht11::Dht11;

/// Interval between readings of the sensor.
const READING_INTERVAL: Duration = Duration::from_secs(30);

/// Logs a reading of the sensor every [`READING_INTERVAL`].
pub async fn run(mut sensor: Dht11<'static>) -> ! {
    loop {
        match sensor.read().await {
            Ok(reading) => info!("measured {reading}"),
            Err(error) => warn!("failed to read the DHT11 sensor: {error}"),
        }
        Timer::after(READING_INTERVAL).await;
    }
}
