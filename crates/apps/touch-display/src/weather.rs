//! Weather that the [`scene`](crate::scene) shows, derived from the latest
//! measurement and the trend of the air pressure.
//!
//! The weather station measures neither clouds nor rain, so [`Weather`] maps
//! what it measures to a picture:
//!
//! - The sun shines with a UV index of at least [`SUN_UV_INDEX`].
//! - Clouds grow with the humidity.
//! - Rain falls, or snow at or below [`SNOW_CELSIUS`], when the humidity is at
//!   least [`PRECIPITATION_HUMIDITY`] and the air pressure falls, the classic
//!   barometric sign of worsening weather.
//!
//! [`PressureTrend`] tracks the air pressure of the last [`TREND_SECONDS`].

use core::fmt;

use heapless::Deque;

use crate::format::Record;

/// Lowest UV index at which the sun shines.
pub const SUN_UV_INDEX: f32 = 1.0;

/// Lowest humidity in percent at which rain or snow falls.
pub const PRECIPITATION_HUMIDITY: f32 = 90.0;

/// Highest temperature in degrees Celsius at which snow instead of rain falls.
pub const SNOW_CELSIUS: f32 = 1.0;

/// Time over which the trend of the air pressure is tracked.
pub const TREND_SECONDS: u32 = 3 * 3_600;

/// Shortest time over which a trend counts.
const MIN_TREND_SECONDS: u32 = 2 * 3_600;

/// Smallest change of the air pressure in hectopascals that counts as rising
/// or falling.
const TREND_HECTOPASCALS: f32 = 1.0;

/// Time between two samples of the air pressure.
const SAMPLE_SECONDS: u32 = 10 * 60;

/// Latest measurement and trend of the air pressure.
#[derive(Debug, Clone, Copy, Default)]
pub struct Weather {
    pub temperature_celsius: Option<f32>,
    pub humidity_percent: Option<f32>,
    pub uv_index: Option<f32>,
    pub pressure_hectopascals: Option<f32>,
    pub trend: Option<Trend>,
}

/// Direction in which the air pressure changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trend {
    Falling,
    Steady,
    Rising,
}

/// Rain or snow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precipitation {
    Rain,
    Snow,
}

impl Weather {
    /// Returns the strength of the sun from 0, hidden, to 1, a UV index of 6
    /// or more.
    pub fn sun(&self) -> f32 {
        match self.uv_index {
            Some(uv_index) if uv_index >= SUN_UV_INDEX => (uv_index / 6.0).clamp(0.2, 1.0),
            _ => 0.0,
        }
    }

    /// Returns the number of clouds, from 0 to 5, which grows with the
    /// humidity.
    pub fn clouds(&self) -> usize {
        match self.humidity_percent {
            None => 0,
            Some(humidity) if humidity < 35.0 => 1,
            Some(humidity) if humidity < 55.0 => 2,
            Some(humidity) if humidity < 75.0 => 3,
            Some(humidity) if humidity < 85.0 => 4,
            Some(_) => 5,
        }
    }

    /// Indicates whether the clouds are dark.
    pub fn overcast(&self) -> bool {
        self.clouds() == 5
    }

    /// Returns the rain or snow that falls, if any.
    pub fn precipitation(&self) -> Option<Precipitation> {
        let humid = self
            .humidity_percent
            .is_some_and(|humidity| humidity >= PRECIPITATION_HUMIDITY);
        if !humid || self.trend != Some(Trend::Falling) {
            return None;
        }
        match self.temperature_celsius {
            Some(temperature) if temperature <= SNOW_CELSIUS => Some(Precipitation::Snow),
            _ => Some(Precipitation::Rain),
        }
    }

    /// Returns a description in a few words, for instance, "Mild, humid".
    pub fn description(&self) -> Description {
        Description(*self)
    }
}

/// Description of the [`Weather`] in a few words.
pub struct Description(Weather);

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.precipitation() {
            Some(Precipitation::Rain) => return f.write_str("Rain likely"),
            Some(Precipitation::Snow) => return f.write_str("Snow likely"),
            None => {}
        }
        let temperature = self
            .0
            .temperature_celsius
            .map(|temperature| match temperature {
                t if t < 5.0 => "Cold",
                t if t < 15.0 => "Cool",
                t if t < 22.0 => "Mild",
                t if t < 28.0 => "Warm",
                _ => "Hot",
            });
        let humidity = self.0.humidity_percent.map(|humidity| match humidity {
            h if h < 30.0 => "dry",
            h if h < 60.0 => "comfortable",
            _ => "humid",
        });
        match (temperature, humidity) {
            (Some(temperature), Some(humidity)) => write!(f, "{temperature}, {humidity}"),
            (Some(temperature), None) => f.write_str(temperature),
            (None, Some("dry")) => f.write_str("Dry"),
            (None, Some("comfortable")) => f.write_str("Comfortable"),
            (None, Some(_)) => f.write_str("Humid"),
            (None, None) => f.write_str("No measurements"),
        }
    }
}

/// Air pressure of the last [`TREND_SECONDS`], one sample every
/// [`SAMPLE_SECONDS`].
pub struct PressureTrend {
    /// Samples with their time in Unix seconds, oldest first.
    samples: Deque<(u32, f32), { (TREND_SECONDS / SAMPLE_SECONDS) as usize + 1 }>,
}

impl PressureTrend {
    /// Creates an empty trend.
    pub const fn new() -> Self {
        Self {
            samples: Deque::new(),
        }
    }

    /// Adds the air pressure of `record`, if any, unless the previous sample
    /// is less than [`SAMPLE_SECONDS`] older.
    pub fn add(&mut self, record: &Record) {
        let Some(pressure) = record.pressure_hectopascals else {
            return;
        };
        if self
            .samples
            .back()
            .is_some_and(|(time, _)| record.time < time + SAMPLE_SECONDS)
        {
            return;
        }
        while self
            .samples
            .front()
            .is_some_and(|(time, _)| *time + TREND_SECONDS < record.time)
        {
            self.samples.pop_front();
        }
        self.samples
            .push_back((record.time, pressure))
            .expect("samples of the trend are at least `SAMPLE_SECONDS` apart");
    }

    /// Returns the trend at `now` in Unix seconds, or `None` if the samples
    /// span less than two hours.
    pub fn trend(&self, now: u32) -> Option<Trend> {
        let (start, first) = *self
            .samples
            .iter()
            .find(|(time, _)| time + TREND_SECONDS >= now)?;
        let (end, last) = *self.samples.back()?;
        if end - start < MIN_TREND_SECONDS {
            return None;
        }
        Some(match last - first {
            change if change <= -TREND_HECTOPASCALS => Trend::Falling,
            change if change >= TREND_HECTOPASCALS => Trend::Rising,
            _ => Trend::Steady,
        })
    }
}
