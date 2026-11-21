//! Readings of a Joy-IT SEN-UV01 UV sensor, which has a GUVA-S12SD
//! photodiode.
//!
//! The sensor outputs a voltage that grows with the intensity of UV-A and UV-B
//! radiation. The ADC of the RP2040 measures it with 12 bits against its
//! 3.3 V reference. The UV index is about the voltage divided by 0.1 V. This
//! approximation for modules with this photodiode is not calibrated.

use core::fmt;

use embassy_rp::Peri;
use embassy_rp::adc::Adc;
use embassy_rp::adc::AdcPin;
use embassy_rp::adc::Blocking;
use embassy_rp::adc::Channel;
use embassy_rp::adc::Config;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::ADC;

/// Number of conversions that a reading averages, against the noise of the
/// ADC.
const SAMPLES: u16 = 16;

/// Reference voltage of the ADC.
const REFERENCE_VOLTS: f32 = 3.3;

/// Number of steps of the 12-bit ADC.
const STEPS: f32 = 4096.0;

/// Output voltage of the sensor per step of the UV index.
const VOLTS_PER_UV_INDEX: f32 = 0.1;

/// Error of a reading.
#[derive(Debug)]
pub enum Error {
    /// The ADC failed to convert the voltage.
    Conversion,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conversion => f.write_str("the ADC failed to convert the voltage"),
        }
    }
}

/// Reading of the sensor.
#[derive(Debug, Clone, Copy)]
pub struct Reading {
    /// Output voltage of the sensor.
    pub volts: f32,
    /// Approximate UV index.
    pub uv_index: f32,
}

impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UV index {:.1} ({:.3} V)", self.uv_index, self.volts)
    }
}

/// UV sensor connected to an analog input.
pub struct UvSensor<'d> {
    adc: Adc<'d, Blocking>,
    channel: Channel<'d>,
}

impl<'d> UvSensor<'d> {
    /// Connects to a sensor on the analog input `pin`.
    pub fn new(adc: Peri<'d, ADC>, pin: Peri<'d, impl AdcPin + 'd>) -> Self {
        Self {
            adc: Adc::new_blocking(adc, Config::default()),
            channel: Channel::new_pin(pin, Pull::None),
        }
    }

    /// Reads the UV radiation.
    pub fn read(&mut self) -> Result<Reading, Error> {
        let mut sum = 0_u32;
        for _ in 0..SAMPLES {
            let sample = self
                .adc
                .blocking_read(&mut self.channel)
                .map_err(|_| Error::Conversion)?;
            sum += u32::from(sample);
        }
        let volts = sum as f32 / f32::from(SAMPLES) * REFERENCE_VOLTS / STEPS;
        Ok(Reading {
            volts,
            uv_index: volts / VOLTS_PER_UV_INDEX,
        })
    }
}
