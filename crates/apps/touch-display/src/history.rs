//! Measurements of an [`Interval`], summarized for the plot.
//!
//! [`History`] divides the interval into [`BUCKETS`] buckets, one for each
//! column of the plot. For every [`Metric`], a bucket holds the minimum, the
//! maximum, and the mean of its measurements.

use crate::format::Record;

/// Number of buckets, one for each column of the plot.
pub const BUCKETS: usize = 288;

/// Longest time without measurements that the plot bridges.
const MAX_BRIDGED_SECONDS: u32 = 2 * 60;

/// Interval that the plot shows, up to the current time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    Hour,
    ThreeHours,
    SixHours,
    Day,
    Week,
    Month,
}

impl Interval {
    /// All intervals, from the shortest to the longest.
    pub const ALL: [Interval; 6] = [
        Interval::Hour,
        Interval::ThreeHours,
        Interval::SixHours,
        Interval::Day,
        Interval::Week,
        Interval::Month,
    ];

    /// Returns the length of the interval in seconds.
    pub fn seconds(self) -> u32 {
        const HOUR: u32 = 3_600;
        match self {
            Self::Hour => HOUR,
            Self::ThreeHours => 3 * HOUR,
            Self::SixHours => 6 * HOUR,
            Self::Day => 24 * HOUR,
            Self::Week => 7 * 24 * HOUR,
            Self::Month => 30 * 24 * HOUR,
        }
    }

    /// Returns the number of the bucket of `time` in Unix seconds. Buckets
    /// may be fractions of seconds long, for instance, 12.5 seconds for an
    /// hour.
    fn bucket(self, time: u32) -> u32 {
        (u64::from(time) * BUCKETS as u64 / u64::from(self.seconds())) as u32
    }

    /// Returns the largest number of empty columns between two measurements
    /// that the plot bridges, which span at most [`MAX_BRIDGED_SECONDS`].
    ///
    /// Columns of short intervals are shorter than the time between two
    /// measurements, so that every second column or so is empty.
    pub fn max_bridged_columns(self) -> usize {
        (u64::from(MAX_BRIDGED_SECONDS) * BUCKETS as u64 / u64::from(self.seconds())) as usize
    }

    /// Returns the name of the interval.
    pub fn label(self) -> &'static str {
        match self {
            Self::Hour => "1 h",
            Self::ThreeHours => "3 h",
            Self::SixHours => "6 h",
            Self::Day => "24 h",
            Self::Week => "7 d",
            Self::Month => "30 d",
        }
    }

    /// Returns the label of the start of the interval.
    pub fn start_label(self) -> &'static str {
        match self {
            Self::Hour => "-1 h",
            Self::ThreeHours => "-3 h",
            Self::SixHours => "-6 h",
            Self::Day => "-24 h",
            Self::Week => "-7 d",
            Self::Month => "-30 d",
        }
    }
}

/// Volts of the UV sensor per UV index, as the weather station converts them.
const VOLTS_PER_UV_INDEX: f32 = 0.1;

/// Value of a measurement that the plot shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    Temperature,
    Humidity,
    UvIndex,
    Pressure,
}

impl Metric {
    /// All metrics, in the order in which the plot shows them.
    pub const ALL: [Metric; 4] = [
        Metric::Temperature,
        Metric::Humidity,
        Metric::UvIndex,
        Metric::Pressure,
    ];

    /// Returns the position of the metric in [`Metric::ALL`].
    pub fn index(self) -> usize {
        match self {
            Self::Temperature => 0,
            Self::Humidity => 1,
            Self::UvIndex => 2,
            Self::Pressure => 3,
        }
    }

    /// Returns the metric that the plot shows after this one.
    pub fn next(self) -> Self {
        Self::ALL[(self.index() + 1) % Self::ALL.len()]
    }

    /// Returns the name of the metric.
    pub fn label(self) -> &'static str {
        match self {
            Self::Temperature => "Temperature",
            Self::Humidity => "Humidity",
            Self::UvIndex => "UV index",
            Self::Pressure => "Pressure",
        }
    }

    /// Returns the unit of the metric.
    pub fn unit(self) -> &'static str {
        match self {
            Self::Temperature => "°C",
            Self::Humidity => "%",
            Self::UvIndex => "",
            Self::Pressure => "hPa",
        }
    }

    /// Returns the number of digits after the decimal point.
    pub fn precision(self) -> usize {
        match self {
            Self::Temperature | Self::UvIndex => 1,
            Self::Humidity | Self::Pressure => 0,
        }
    }

    /// Returns the smallest range of the vertical axis, so that the noise of
    /// a steady value does not fill the plot.
    pub fn min_span(self) -> f32 {
        match self {
            Self::Temperature => 2.0,
            Self::Humidity => 10.0,
            Self::UvIndex => 1.0,
            Self::Pressure => 2.0,
        }
    }

    /// Returns the value of the metric in `record`, if present.
    pub fn value(self, record: &Record) -> Option<f32> {
        match self {
            Self::Temperature => record.temperature_celsius,
            Self::Humidity => record.humidity_percent,
            Self::UvIndex => record.uv_volts.map(|volts| volts / VOLTS_PER_UV_INDEX),
            Self::Pressure => record.pressure_hectopascals,
        }
    }
}

/// Summary of the values of a metric in a bucket.
#[derive(Debug, Clone, Copy)]
pub struct Column {
    pub min: f32,
    pub max: f32,
    pub mean: f32,
    /// Number of measurements.
    pub count: u16,
}

/// Values of a metric in a bucket.
#[derive(Debug, Clone, Copy)]
struct Stats {
    min: f32,
    max: f32,
    sum: f32,
    count: u16,
}

impl Stats {
    const EMPTY: Self = Self {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
        sum: 0.0,
        count: 0,
    };
}

/// Bucket of the ring of [`History`].
#[derive(Debug, Clone, Copy)]
struct Slot {
    /// Number of the bucket since the Unix epoch, or `u32::MAX` if unused.
    bucket: u32,
    stats: [Stats; Metric::ALL.len()],
}

impl Slot {
    const EMPTY: Self = Self {
        bucket: u32::MAX,
        stats: [Stats::EMPTY; Metric::ALL.len()],
    };
}

/// Measurements of an interval.
pub struct History {
    interval: Interval,
    /// Ring of buckets, indexed by the number of the bucket modulo
    /// [`BUCKETS`].
    slots: [Slot; BUCKETS],
}

impl History {
    /// Creates an empty history of `interval`.
    pub const fn new(interval: Interval) -> Self {
        Self {
            interval,
            slots: [Slot::EMPTY; BUCKETS],
        }
    }

    /// Returns the interval of the history.
    pub fn interval(&self) -> Interval {
        self.interval
    }

    /// Removes all measurements and changes the interval to `interval`.
    pub fn reset(&mut self, interval: Interval) {
        *self = Self::new(interval);
    }

    /// Adds `record`, unless its bucket has already been replaced by a newer
    /// one.
    pub fn add(&mut self, record: &Record) {
        let bucket = self.interval.bucket(record.time);
        let slot = &mut self.slots[bucket as usize % BUCKETS];
        if slot.bucket != bucket {
            if slot.bucket != u32::MAX && slot.bucket > bucket {
                return;
            }
            *slot = Slot::EMPTY;
            slot.bucket = bucket;
        }
        for (metric, stats) in Metric::ALL.iter().zip(&mut slot.stats) {
            if let Some(value) = metric.value(record) {
                stats.min = stats.min.min(value);
                stats.max = stats.max.max(value);
                stats.sum += value;
                stats.count = stats.count.saturating_add(1);
            }
        }
    }

    /// Returns the summaries of `metric` in the interval up to `now` in Unix
    /// seconds, oldest first.
    pub fn columns(&self, metric: Metric, now: u32) -> [Option<Column>; BUCKETS] {
        let index = metric.index();
        let last = self.interval.bucket(now);
        core::array::from_fn(|offset| {
            let bucket = (last + 1).checked_sub((BUCKETS - offset) as u32)?;
            let slot = &self.slots[bucket as usize % BUCKETS];
            let stats = slot.stats[index];
            (slot.bucket == bucket && stats.count > 0).then(|| Column {
                min: stats.min,
                max: stats.max,
                mean: stats.sum / f32::from(stats.count),
                count: stats.count,
            })
        })
    }
}
