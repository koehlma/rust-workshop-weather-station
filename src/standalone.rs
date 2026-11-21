//! Operation without Nexigon, when the `nexigon` feature is disabled.
//!
//! [`run`] logs a measurement of the sensors every [`MEASUREMENT_INTERVAL`].

use embassy_time::Duration;
use embassy_time::Timer;
use log::info;

use crate::sensors::Sensors;

/// Interval between measurements.
const MEASUREMENT_INTERVAL: Duration = Duration::from_secs(30);

/// Logs a measurement of the sensors every [`MEASUREMENT_INTERVAL`].
pub async fn run(mut sensors: Sensors) -> ! {
    loop {
        let measurement = sensors.measure().await;
        // Sensors that fail are logged as they are measured.
        if !measurement.is_empty() {
            info!("measured {measurement}");
        }
        Timer::after(MEASUREMENT_INTERVAL).await;
    }
}
