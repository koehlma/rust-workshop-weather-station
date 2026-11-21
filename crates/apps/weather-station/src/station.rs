//! The weather station: its sensors and its display.
//!
//! [`Station::measure`] reads the sensors and shows the measurement on the
//! display.

use crate::display::Display;
use crate::sensors::Measurement;
use crate::sensors::Sensors;

/// Sensors and display of the weather station.
pub struct Station {
    sensors: Sensors,
    display: Display,
}

impl Station {
    /// Combines the sensors and the display.
    pub fn new(sensors: Sensors, display: Display) -> Self {
        Self { sensors, display }
    }

    /// Reads the sensors and shows the measurement.
    ///
    /// The DHT11 returns its previous measurement, so measurements should be
    /// at least 2 seconds apart.
    pub async fn measure(&mut self) -> Measurement {
        let measurement = self.sensors.measure().await;
        self.display.show(&measurement);
        measurement
    }
}
