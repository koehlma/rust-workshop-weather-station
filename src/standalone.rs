//! Operation without Nexigon, when the `nexigon` feature is disabled.
//!
//! [`run`] measures every [`MEASUREMENT_INTERVAL`] and logs the measurement.

use embassy_time::Duration;
use embassy_time::Timer;
use log::info;

use crate::station::Station;

/// Interval between measurements.
const MEASUREMENT_INTERVAL: Duration = Duration::from_secs(30);

/// Measures every [`MEASUREMENT_INTERVAL`] and logs the measurement.
pub async fn run(mut station: Station) -> ! {
    loop {
        let measurement = station.measure().await;
        // Sensors that fail are logged as they are measured.
        if !measurement.is_empty() {
            info!("measured {measurement}");
        }
        Timer::after(MEASUREMENT_INTERVAL).await;
    }
}
