//! Readings of the air pressure from a BMP280 sensor, such as the Joy-IT
//! KY-052.
//!
//! The sensor is connected to I2C0. With its SDO pin tied to 3.3 V, it
//! answers at the address 0x77. [`Bmp280::read`] starts the sensor on its
//! first successful use, so that a sensor connected later still works.
//!
//! A reading blocks for about 41 ms, most of which the sensor needs to
//! measure. The asynchronous API of the `bme280` crate requires nightly Rust.

use core::fmt;

use bme280::i2c::BME280;
use embassy_rp::i2c::Blocking;
use embassy_rp::i2c::I2c;
use embassy_rp::peripherals::I2C0;
use embassy_time::Delay;

/// Pascals per hectopascal.
const PASCALS_PER_HECTOPASCAL: f32 = 100.0;

/// Error of a reading.
#[derive(Debug)]
pub enum Error {
    /// The sensor did not answer on the I2C bus, for instance, because none
    /// is connected.
    Bus,
    /// The sensor answered with data that cannot be compensated.
    InvalidData,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bus => f.write_str("the sensor did not answer"),
            Self::InvalidData => f.write_str("the sensor returned invalid data"),
        }
    }
}

impl<E> From<bme280::Error<E>> for Error {
    fn from(error: bme280::Error<E>) -> Self {
        match error {
            bme280::Error::Bus(_) | bme280::Error::UnsupportedChip => Self::Bus,
            _ => Self::InvalidData,
        }
    }
}

/// Reading of the sensor.
#[derive(Debug, Clone, Copy)]
pub struct Reading {
    /// Air pressure in hectopascals.
    pub pressure_hectopascals: f32,
}

impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} hPa", self.pressure_hectopascals)
    }
}

/// BMP280 sensor on I2C0.
pub struct Bmp280 {
    sensor: BME280<I2c<'static, I2C0, Blocking>>,
    /// Indicates whether the sensor has been started.
    started: bool,
}

impl Bmp280 {
    /// Connects to the sensor at the address 0x77.
    pub fn new(i2c: I2c<'static, I2C0, Blocking>) -> Self {
        Self {
            sensor: BME280::new_secondary(i2c),
            started: false,
        }
    }

    /// Reads the air pressure, starting the sensor first if needed.
    pub fn read(&mut self) -> Result<Reading, Error> {
        if !self.started {
            self.sensor.init(&mut Delay)?;
            self.started = true;
        }
        let measurements = self.sensor.measure(&mut Delay).inspect_err(|_| {
            // The sensor may have lost its configuration, for instance, when
            // it was disconnected.
            self.started = false;
        })?;
        Ok(Reading {
            pressure_hectopascals: measurements.pressure / PASCALS_PER_HECTOPASCAL,
        })
    }
}
