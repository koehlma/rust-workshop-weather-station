//! Sensors of the weather station and their combined measurements.
//!
//! [`Sensors::measure`] reads the DHT11, the UV sensor, and the BMP280. A
//! sensor that fails is logged and missing from the [`Measurement`].

use core::fmt;

use log::warn;

use crate::bmp280;
use crate::bmp280::Bmp280;
use crate::dht11;
use crate::dht11::Dht11;
use crate::uv;
use crate::uv::UvSensor;

/// Sensors of the weather station.
pub struct Sensors {
    dht11: Dht11<'static>,
    uv: UvSensor<'static>,
    bmp280: Bmp280,
}

/// Readings of the sensors that answered.
#[derive(Debug, Clone, Copy)]
pub struct Measurement {
    /// Temperature and humidity.
    pub dht11: Option<dht11::Reading>,
    /// UV radiation.
    pub uv: Option<uv::Reading>,
    /// Air pressure.
    pub bmp280: Option<bmp280::Reading>,
}

impl Sensors {
    /// Combines the sensors.
    pub fn new(dht11: Dht11<'static>, uv: UvSensor<'static>, bmp280: Bmp280) -> Self {
        Self { dht11, uv, bmp280 }
    }

    /// Reads all sensors.
    pub async fn measure(&mut self) -> Measurement {
        let dht11 = self
            .dht11
            .read()
            .await
            .inspect_err(|error| warn!("failed to read the DHT11 sensor: {error}"))
            .ok();
        let uv = self
            .uv
            .read()
            .inspect_err(|error| warn!("failed to read the UV sensor: {error}"))
            .ok();
        let bmp280 = self
            .bmp280
            .read()
            .inspect_err(|error| warn!("failed to read the BMP280 sensor: {error}"))
            .ok();
        Measurement { dht11, uv, bmp280 }
    }
}

impl Measurement {
    /// Indicates whether no sensor answered.
    pub fn is_empty(&self) -> bool {
        self.dht11.is_none() && self.uv.is_none() && self.bmp280.is_none()
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let readings: [Option<&dyn fmt::Display>; 3] = [
            self.dht11.as_ref().map(|reading| reading as _),
            self.uv.as_ref().map(|reading| reading as _),
            self.bmp280.as_ref().map(|reading| reading as _),
        ];
        for (index, reading) in readings.into_iter().flatten().enumerate() {
            if index > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{reading}")?;
        }
        Ok(())
    }
}
