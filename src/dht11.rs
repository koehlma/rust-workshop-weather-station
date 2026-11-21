//! Readings of a DHT11 temperature and humidity sensor.
//!
//! The sensor shares one data line with the host, which a pull-up keeps high.
//! The host starts a reading by pulling the line low for at least 18 ms. The
//! sensor answers with a low and a high pulse of 80 µs each, followed by 40
//! bits. Each bit starts with a low pulse of 50 µs. The length of the high
//! pulse that follows encodes the bit: 26 to 28 µs for 0 and 70 µs for 1.

use core::fmt;

use embassy_rp::Peri;
use embassy_rp::gpio::Level;
use embassy_rp::gpio::OutputOpenDrain;
use embassy_rp::gpio::Pin;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::Timer;

/// Time for which the host pulls the line low to start a reading.
const START_DURATION: Duration = Duration::from_millis(20);

/// Maximum length of a pulse of the sensor.
const PULSE_TIMEOUT: Duration = Duration::from_micros(150);

/// Length above which a high pulse encodes a 1.
const ONE_THRESHOLD: Duration = Duration::from_micros(48);

/// Error of a reading.
#[derive(Debug)]
pub enum Error {
    /// The sensor did not answer in time, for instance, because none is
    /// connected.
    Timeout,
    /// The checksum does not match the data.
    Checksum,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timeout => f.write_str("the sensor did not answer"),
            Self::Checksum => f.write_str("the checksum of the reading is invalid"),
        }
    }
}

/// Reading of the sensor.
#[derive(Debug, Clone, Copy)]
pub struct Reading {
    /// Temperature in degrees Celsius.
    pub temperature_celsius: f32,
    /// Relative humidity in percent.
    pub humidity_percent: f32,
}

/// DHT11 sensor connected to a GPIO pin.
pub struct Dht11<'d> {
    pin: OutputOpenDrain<'d>,
}

impl<'d> Dht11<'d> {
    /// Connects to a sensor on `pin`.
    pub fn new(pin: Peri<'d, impl Pin>) -> Self {
        let mut pin = OutputOpenDrain::new(pin, Level::High);
        pin.set_pullup(true);
        Self { pin }
    }

    /// Reads the temperature and humidity.
    ///
    /// The sensor measures when it is read and returns the previous
    /// measurement, so readings should be at least 2 seconds apart.
    pub async fn read(&mut self) -> Result<Reading, Error> {
        self.pin.set_low();
        Timer::after(START_DURATION).await;
        // Interrupts would distort the measured pulses. Receiving takes about
        // 5 ms and, with timeouts, at most 13 ms.
        let data = cortex_m::interrupt::free(|_| {
            self.pin.set_high();
            self.receive()
        })?;
        let sum = data[..4]
            .iter()
            .fold(0_u8, |sum, byte| sum.wrapping_add(*byte));
        if sum != data[4] {
            return Err(Error::Checksum);
        }
        let humidity_percent = f32::from(data[0]) + f32::from(data[1]) / 10.0;
        // The highest bit of the decimal part marks temperatures below zero.
        let temperature = f32::from(data[2]) + f32::from(data[3] & 0x7F) / 10.0;
        let temperature_celsius = if data[3] & 0x80 != 0 {
            -temperature
        } else {
            temperature
        };
        Ok(Reading {
            temperature_celsius,
            humidity_percent,
        })
    }

    /// Receives the answer of the sensor after the host has released the line.
    fn receive(&self) -> Result<[u8; 5], Error> {
        // The sensor pulls the line low after 20 to 40 µs.
        self.pulse(Level::High)?;
        self.pulse(Level::Low)?;
        self.pulse(Level::High)?;
        let mut data = [0; 5];
        for bit in 0..40 {
            self.pulse(Level::Low)?;
            if self.pulse(Level::High)? > ONE_THRESHOLD {
                data[bit / 8] |= 0x80 >> (bit % 8);
            }
        }
        Ok(data)
    }

    /// Waits until the line leaves `level` and returns how long it stayed.
    fn pulse(&self, level: Level) -> Result<Duration, Error> {
        let start = Instant::now();
        while self.pin.get_level() == level {
            if start.elapsed() > PULSE_TIMEOUT {
                return Err(Error::Timeout);
            }
        }
        Ok(start.elapsed())
    }
}
