//! Sensors of the weather station and their combined measurements.
//!
//! [`Sensors::measure`] reads the DHT11 and the UV sensor. A sensor that fails
//! is logged and missing from the [`Measurement`].

use core::fmt;

use log::warn;

use crate::dht11;
use crate::dht11::Dht11;
use crate::uv;
use crate::uv::UvSensor;

/// Sensors of the weather station.
pub struct Sensors {
    dht11: Dht11<'static>,
    uv: UvSensor<'static>,
}

/// Readings of the sensors that answered.
#[derive(Debug, Clone, Copy)]
pub struct Measurement {
    /// Temperature and humidity.
    pub dht11: Option<dht11::Reading>,
    /// UV radiation.
    pub uv: Option<uv::Reading>,
}

impl Sensors {
    /// Combines the sensors.
    pub fn new(dht11: Dht11<'static>, uv: UvSensor<'static>) -> Self {
        Self { dht11, uv }
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
        Measurement { dht11, uv }
    }
}

impl Measurement {
    /// Indicates whether no sensor answered.
    pub fn is_empty(&self) -> bool {
        self.dht11.is_none() && self.uv.is_none()
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut separator = "";
        if let Some(reading) = &self.dht11 {
            write!(f, "{reading}")?;
            separator = ", ";
        }
        if let Some(reading) = &self.uv {
            write!(f, "{separator}{reading}")?;
        }
        Ok(())
    }
}
