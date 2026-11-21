//! Layout of the screen, 320 × 240 pixels in landscape.
//!
//! From top to bottom, the screen shows:
//!
//! - A tile with the latest value of each [`Metric`]. The tile of the plotted
//!   metric is highlighted in the color of the metric.
//! - The name of the plotted metric with its minimum, mean, and maximum in the
//!   plotted [`Interval`].
//! - A plot of the interval with gridlines at round values. Each column is a
//!   bucket of the [`History`](crate::history::History): a band from its
//!   minimum to its maximum over an area that fades towards the bottom, and a
//!   line through its mean. Gaps of up to
//!   [`Interval::max_bridged_columns`] empty columns are interpolated.
//! - The start of the interval, a chip for each interval, and the end, now.
//! - The state of the device and the version of the firmware.
//!
//! [`Frame::new`] computes the layout of a [`Screen`] once, and
//! [`Frame::draw`] draws it, for instance, strip by strip. [`target_at`]
//! returns what a touch selects.

use core::fmt::Write as _;

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::primitives::RoundedRectangle;
use embedded_graphics::text::Alignment;
use heapless::String;
use heapless::Vec;
use profont::PROFONT_7_POINT;
use profont::PROFONT_9_POINT;
use profont::PROFONT_18_POINT;

use crate::history::BUCKETS;
use crate::history::Column;
use crate::history::Interval;
use crate::history::Metric;
use crate::style::BACKGROUND;
use crate::style::CARD;
use crate::style::CARD_SELECTED;
use crate::style::ERROR;
use crate::style::GOOD;
use crate::style::GRID;
use crate::style::MUTED;
use crate::style::TEXT;
use crate::style::WARNING;
use crate::style::blend;
use crate::style::rgb;
use crate::style::text;
use crate::style::text_middle;
use crate::style::width;

/// Content of the screen.
pub struct Screen<'a> {
    /// Latest value of each metric, in the order of [`Metric::ALL`].
    pub values: [Option<f32>; Metric::ALL.len()],
    /// Indicates whether the latest values are too old to be current.
    pub stale: bool,
    /// Plotted metric.
    pub metric: Metric,
    /// Plotted interval.
    pub interval: Interval,
    /// Columns of the plot, oldest first.
    pub columns: &'a [Option<Column>; BUCKETS],
    /// Text that replaces the plot, for instance, while the clock is not
    /// synchronized.
    pub notice: Option<&'a str>,
    /// Instruction that a plot without measurements shows, for instance, how
    /// to configure the weather station.
    pub hint: Option<&'a str>,
    /// State of the device.
    pub status: Status,
    /// Description of the state of the device.
    pub status_text: &'a str,
    /// Version of the firmware.
    pub firmware_version: &'a str,
}

/// Kind of the state of the device, which determines its color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Everything works.
    Ok,
    /// The device is busy, for instance, reading the SD card.
    Busy,
    /// Something needs attention, for instance, no measurements arrive.
    Warning,
    /// Something failed, for instance, the SD card.
    Error,
}

/// What a touch selects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// The tile of a metric.
    Metric(Metric),
    /// The chip of an interval.
    Interval(Interval),
    /// The plot.
    Plot,
}

/// Returns what a touch at `point` selects.
///
/// The areas of the tiles and the chips extend halfway to the plot, which
/// tolerates the inaccuracy of the calibration of the touch screen.
pub fn target_at(point: Point) -> Target {
    if point.y < (TILE_TOP + TILE_HEIGHT + PLOT_TOP) / 2 {
        let index = (point.x - TILE_LEFT).div_euclid(TILE_PITCH);
        Target::Metric(Metric::ALL[index.clamp(0, Metric::ALL.len() as i32 - 1) as usize])
    } else if point.y > (PLOT_BOTTOM + CHIPS_TOP) / 2 {
        let index = (point.x - CHIPS_LEFT + CHIP_GAP / 2).div_euclid(CHIP_PITCH);
        Target::Interval(Interval::ALL[index.clamp(0, Interval::ALL.len() as i32 - 1) as usize])
    } else {
        Target::Plot
    }
}

/// Layout of a [`Screen`].
pub struct Frame<'a> {
    screen: &'a Screen<'a>,
    accent: Rgb565,
    /// Plot, if there are measurements to plot.
    plot: Option<Plot>,
}

impl Drawable for Frame<'_> {
    type Color = Rgb565;
    type Output = ();

    fn draw<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        target.clear(BACKGROUND)?;
        self.draw_tiles(target)?;
        self.draw_header(target)?;
        match &self.plot {
            Some(plot) => plot.draw(target)?,
            None => self.draw_notice(target)?,
        }
        self.draw_intervals(target)?;
        self.draw_status(target)
    }
}

impl<'a> Frame<'a> {
    /// Computes the layout of `screen`.
    pub fn new(screen: &'a Screen<'a>) -> Self {
        let accent = accent(screen.metric);
        let plot = match screen.notice {
            Some(_) => None,
            None => Plot::new(screen, accent),
        };
        Self {
            screen,
            accent,
            plot,
        }
    }

    /// Draws a tile with the latest value of each metric.
    fn draw_tiles<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        for (metric, value) in Metric::ALL.into_iter().zip(self.screen.values) {
            let left = TILE_LEFT + metric.index() as i32 * TILE_PITCH;
            let selected = metric == self.screen.metric;
            let card = Rectangle::new(
                Point::new(left, TILE_TOP),
                Size::new(TILE_WIDTH as u32, TILE_HEIGHT as u32),
            );
            let fill = if selected { CARD_SELECTED } else { CARD };
            RoundedRectangle::with_equal_corners(card, Size::new(CORNER_RADIUS, CORNER_RADIUS))
                .into_styled(PrimitiveStyle::with_fill(fill))
                .draw(target)?;
            let label_color = if selected { accent(metric) } else { MUTED };
            let top = TILE_TOP + TILE_PADDING;
            let inner_left = left + TILE_PADDING;
            let inner_right = left + TILE_WIDTH - TILE_PADDING;
            let small = &PROFONT_7_POINT;
            text(
                target,
                caption(metric),
                inner_left,
                top,
                small,
                label_color,
                Alignment::Left,
            )?;
            text(
                target,
                metric.unit(),
                inner_right,
                top,
                small,
                MUTED,
                Alignment::Right,
            )?;
            let value_top = top + 11;
            match value {
                Some(value) => {
                    let mut formatted = String::<12>::new();
                    // Values that do not fit are cut off.
                    let precision = metric.precision();
                    let _ = write!(formatted, "{value:.precision$}");
                    let color = if self.screen.stale { MUTED } else { TEXT };
                    let font = &PROFONT_18_POINT;
                    text(
                        target,
                        &formatted,
                        inner_left,
                        value_top,
                        font,
                        color,
                        Alignment::Left,
                    )?;
                }
                None => {
                    let placeholder = Rectangle::new(
                        Point::new(inner_left + 1, value_top + 11),
                        Size::new(20, 3),
                    );
                    target.fill_solid(&placeholder, MUTED)?;
                }
            }
            if selected {
                let bar = Rectangle::new(
                    Point::new(inner_left, TILE_TOP + TILE_HEIGHT - 5),
                    Size::new((TILE_WIDTH - 2 * TILE_PADDING) as u32, 2),
                );
                target.fill_solid(&bar, self.accent)?;
            }
        }
        Ok(())
    }

    /// Draws the name of the plotted metric and its summary.
    fn draw_header<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        let font = &PROFONT_9_POINT;
        let label = self.screen.metric.label();
        let left = TILE_LEFT + TILE_PADDING;
        text(
            target,
            label,
            left,
            HEADER_TOP,
            font,
            self.accent,
            Alignment::Left,
        )?;
        let unit_left = left + width(label, font) + CHAR_GAP;
        text(
            target,
            self.screen.metric.unit(),
            unit_left,
            HEADER_TOP,
            font,
            MUTED,
            Alignment::Left,
        )?;
        let Some(plot) = &self.plot else {
            return Ok(());
        };
        let precision = self.screen.metric.precision();
        let summary = [
            ("min ", plot.min),
            ("  avg ", plot.mean),
            ("  max ", plot.max),
        ];
        let mut right = PLOT_RIGHT;
        for (label, value) in summary.into_iter().rev() {
            let mut formatted = String::<12>::new();
            // Values that do not fit are cut off.
            let _ = write!(formatted, "{value:.precision$}");
            text(
                target,
                &formatted,
                right,
                HEADER_TOP,
                font,
                TEXT,
                Alignment::Right,
            )?;
            right -= width(&formatted, font);
            text(
                target,
                label,
                right,
                HEADER_TOP,
                font,
                MUTED,
                Alignment::Right,
            )?;
            right -= width(label, font);
        }
        Ok(())
    }

    /// Draws the text that replaces the plot.
    fn draw_notice<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        let mut notice = String::<48>::new();
        // Text that does not fit is cut off.
        let _ = match self.screen.notice {
            Some(text) => notice.write_str(text),
            None => write!(
                notice,
                "No measurements in the last {}",
                self.screen.interval.label()
            ),
        };
        let center = (PLOT_LEFT + PLOT_RIGHT) / 2;
        let middle = (PLOT_TOP + PLOT_BOTTOM) / 2;
        let font = &PROFONT_9_POINT;
        let hint = self.screen.hint.filter(|_| self.screen.notice.is_none());
        let Some(hint) = hint else {
            return text_middle(
                target,
                &notice,
                center,
                middle,
                font,
                MUTED,
                Alignment::Center,
            );
        };
        text_middle(
            target,
            &notice,
            center,
            middle - 8,
            font,
            TEXT,
            Alignment::Center,
        )?;
        text_middle(
            target,
            hint,
            center,
            middle + 8,
            font,
            MUTED,
            Alignment::Center,
        )
    }

    /// Draws the start and the end of the interval and the chips of all
    /// intervals.
    fn draw_intervals<D: DrawTarget<Color = Rgb565>>(
        &self,
        target: &mut D,
    ) -> Result<(), D::Error> {
        let middle = CHIPS_TOP + CHIP_HEIGHT / 2;
        let start = self.screen.interval.start_label();
        text_middle(
            target,
            start,
            PLOT_LEFT,
            middle,
            &PROFONT_7_POINT,
            MUTED,
            Alignment::Left,
        )?;
        text_middle(
            target,
            "now",
            PLOT_RIGHT,
            middle,
            &PROFONT_7_POINT,
            MUTED,
            Alignment::Right,
        )?;
        for (index, interval) in Interval::ALL.into_iter().enumerate() {
            let left = CHIPS_LEFT + index as i32 * CHIP_PITCH;
            let chip = Rectangle::new(
                Point::new(left, CHIPS_TOP),
                Size::new(CHIP_WIDTH as u32, CHIP_HEIGHT as u32),
            );
            let selected = interval == self.screen.interval;
            let (fill, color) = if selected {
                (TEXT, BACKGROUND)
            } else {
                (CARD, MUTED)
            };
            RoundedRectangle::with_equal_corners(chip, Size::new(CORNER_RADIUS, CORNER_RADIUS))
                .into_styled(PrimitiveStyle::with_fill(fill))
                .draw(target)?;
            let center = left + CHIP_WIDTH / 2;
            text_middle(
                target,
                interval.label(),
                center,
                middle,
                &PROFONT_9_POINT,
                color,
                Alignment::Center,
            )?;
        }
        Ok(())
    }

    /// Draws the state of the device and the version of the firmware.
    fn draw_status<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        let (dot, color) = match self.screen.status {
            Status::Ok => (GOOD, MUTED),
            Status::Busy => (MUTED, MUTED),
            Status::Warning => (WARNING, TEXT),
            Status::Error => (ERROR, TEXT),
        };
        let middle = STATUS_TOP + STATUS_HEIGHT / 2;
        let left = TILE_LEFT + TILE_PADDING;
        Circle::with_center(
            Point::new(left + DOT_DIAMETER as i32 / 2, middle),
            DOT_DIAMETER,
        )
        .into_styled(PrimitiveStyle::with_fill(dot))
        .draw(target)?;
        let text_left = left + DOT_DIAMETER as i32 + 6;
        text_middle(
            target,
            self.screen.status_text,
            text_left,
            middle,
            &PROFONT_9_POINT,
            color,
            Alignment::Left,
        )?;
        let version = self.screen.firmware_version;
        let right = TILE_LEFT + 3 * TILE_PITCH + TILE_WIDTH - TILE_PADDING;
        text_middle(
            target,
            version,
            right,
            middle,
            &PROFONT_7_POINT,
            MUTED,
            Alignment::Right,
        )
    }
}

/// Plot of the measurements of the interval.
struct Plot {
    /// Rows and labels of the gridlines, from the bottom up.
    gridlines: Vec<(i32, String<8>), MAX_GRIDLINES>,
    /// Pixels of each column, oldest first.
    spans: [Option<Span>; BUCKETS],
    /// Color of the area below the line, for each row from the top.
    area: [Rgb565; PLOT_HEIGHT as usize],
    /// Color of the band from the minimum to the maximum.
    band: Rgb565,
    /// Color of the line through the means.
    line: Rgb565,
    /// Minimum of the interval.
    min: f32,
    /// Mean of the interval.
    mean: f32,
    /// Maximum of the interval.
    max: f32,
}

/// Pixels of a column of the plot. Rows grow downwards.
#[derive(Debug, Clone, Copy)]
struct Span {
    /// Rows of the band from the maximum to the minimum.
    band: (i32, i32),
    /// Rows of the line, which connects to the mean of the previous column.
    line: (i32, i32),
    /// Row of the mean, below which the area starts.
    mean: i32,
}

impl Plot {
    /// Computes the plot of `screen`, or returns `None` if there are no
    /// measurements.
    fn new(screen: &Screen, accent: Rgb565) -> Option<Self> {
        let columns = screen.columns.iter().flatten();
        let min = columns.clone().map(|column| column.min).reduce(f32::min)?;
        let max = columns.clone().map(|column| column.max).reduce(f32::max)?;
        let (sum, count) = columns.fold((0.0, 0.0), |(sum, count), column| {
            let weight = f32::from(column.count);
            (sum + column.mean * weight, count + weight)
        });
        let axis = Axis::new(min, max, screen.metric.min_span());

        let mut gridlines = Vec::new();
        let mut value = axis.low;
        while value <= axis.high + axis.step / 2.0 {
            let mut label = String::new();
            // Labels that do not fit are cut off.
            let _ = write!(label, "{value:.precision$}", precision = axis.precision());
            gridlines
                .push((axis.row(value), label))
                .expect("axes have at most 5 steps plus one for rounding at each end");
            value += axis.step;
        }

        let bridged = bridge(screen.columns, screen.interval.max_bridged_columns());
        let mut spans = [None; BUCKETS];
        let mut previous: Option<i32> = None;
        for (span, column) in spans.iter_mut().zip(&bridged) {
            let Some(column) = column else {
                previous = None;
                continue;
            };
            let mean = axis.row(column.mean).min(PLOT_BOTTOM - 1);
            let line = match previous {
                Some(previous) => (mean.min(previous), mean.max(previous) + 1),
                None => (mean, mean + 1),
            };
            *span = Some(Span {
                band: (axis.row(column.max), axis.row(column.min)),
                line,
                mean,
            });
            previous = Some(mean);
        }

        let area = core::array::from_fn(|row| {
            // The area fades from 30 % of the accent at the top of the plot
            // to 5 % at the bottom.
            let fade = (PLOT_HEIGHT as usize - row) * 64 / PLOT_HEIGHT as usize;
            blend(accent, BACKGROUND, 13 + fade as u8)
        });
        Some(Self {
            gridlines,
            spans,
            area,
            band: blend(accent, BACKGROUND, 128),
            line: accent,
            min,
            mean: sum / count,
            max,
        })
    }

    /// Draws the plot and the labels of its gridlines.
    fn draw<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        for (row, label) in &self.gridlines {
            let right = PLOT_LEFT - 4;
            text_middle(
                target,
                label,
                right,
                *row,
                &PROFONT_7_POINT,
                MUTED,
                Alignment::Right,
            )?;
        }
        let plot = Rectangle::new(
            Point::new(PLOT_LEFT, PLOT_TOP),
            Size::new(BUCKETS as u32, PLOT_HEIGHT as u32),
        );
        // Only the rows that the target holds are computed.
        let visible = target.bounding_box().intersection(&plot);
        let Some(bottom_right) = visible.bottom_right() else {
            return Ok(());
        };
        let rows = visible.top_left.y..=bottom_right.y;
        let mut gridline_rows = [false; PLOT_HEIGHT as usize];
        for (row, _) in &self.gridlines {
            gridline_rows[(row - PLOT_TOP) as usize] = true;
        }
        for (offset, span) in self.spans.iter().enumerate() {
            let x = PLOT_LEFT + offset as i32;
            let area = Rectangle::new(
                Point::new(x, visible.top_left.y),
                Size::new(1, visible.size.height),
            );
            let dotted = x % 3 == 0;
            let colors = rows.clone().map(|y| {
                let in_range = |(top, bottom): (i32, i32)| (top..=bottom).contains(&y);
                match span {
                    Some(span) if in_range(span.line) => self.line,
                    Some(span) if in_range(span.band) => self.band,
                    Some(span) if y > span.mean => self.area[(y - PLOT_TOP) as usize],
                    _ if dotted && gridline_rows[(y - PLOT_TOP) as usize] => GRID,
                    _ => BACKGROUND,
                }
            });
            target.fill_contiguous(&area, colors)?;
        }
        Ok(())
    }
}

/// Vertical axis of the plot, with gridlines at round values.
struct Axis {
    /// Value at the bottom of the plot.
    low: f32,
    /// Value at the top of the plot.
    high: f32,
    /// Distance between two gridlines.
    step: f32,
}

impl Axis {
    /// Computes an axis that spans at least `min_span` and covers the values
    /// from `min` to `max`.
    fn new(min: f32, max: f32, min_span: f32) -> Self {
        let center = (min + max) / 2.0;
        let half = ((max - min) / 2.0).max(min_span / 2.0);
        let (min, max) = (center - half, center + half);
        // Steps of 1, 2, and 5 times a power of 10, so that there are at
        // most 5 steps.
        let mut step = 0.1;
        for factor in [2.0, 2.5, 2.0].into_iter().cycle() {
            if (max - min) / step <= 5.0 {
                break;
            }
            step *= factor;
        }
        Self {
            low: floor(min / step) * step,
            high: ceil(max / step) * step,
            step,
        }
    }

    /// Returns the number of digits after the decimal point of the labels.
    fn precision(&self) -> usize {
        if self.step < 1.0 { 1 } else { 0 }
    }

    /// Returns the row of `value`.
    fn row(&self, value: f32) -> i32 {
        let fraction = (value - self.low) / (self.high - self.low);
        let row = PLOT_BOTTOM - (fraction * (PLOT_HEIGHT - 1) as f32 + 0.5) as i32;
        row.clamp(PLOT_TOP, PLOT_BOTTOM)
    }
}

/// Returns `columns` with gaps of up to `max_gap` empty columns between two
/// columns filled by linear interpolation.
fn bridge(columns: &[Option<Column>; BUCKETS], max_gap: usize) -> [Option<Column>; BUCKETS] {
    let mut bridged = *columns;
    let mut previous: Option<(usize, Column)> = None;
    for (index, column) in columns.iter().enumerate() {
        let Some(column) = *column else {
            continue;
        };
        if let Some((start, from)) = previous {
            let gap = index - start - 1;
            if gap <= max_gap {
                for step in 1..=gap {
                    let t = step as f32 / (gap + 1) as f32;
                    let lerp = |from: f32, to: f32| from + (to - from) * t;
                    bridged[start + step] = Some(Column {
                        min: lerp(from.min, column.min),
                        max: lerp(from.max, column.max),
                        mean: lerp(from.mean, column.mean),
                        count: 0,
                    });
                }
            }
        }
        previous = Some((index, column));
    }
    bridged
}

/// Returns the caption of the tile of `metric`.
fn caption(metric: Metric) -> &'static str {
    match metric {
        Metric::Temperature => "TEMPERATURE",
        Metric::Humidity => "HUMIDITY",
        Metric::UvIndex => "UV INDEX",
        Metric::Pressure => "PRESSURE",
    }
}

/// Returns the color of `metric`.
fn accent(metric: Metric) -> Rgb565 {
    match metric {
        Metric::Temperature => rgb(255, 138, 61),
        Metric::Humidity => rgb(77, 163, 255),
        Metric::UvIndex => rgb(190, 128, 255),
        Metric::Pressure => rgb(62, 207, 142),
    }
}

/// Returns the largest integer that is not greater than `value`.
fn floor(value: f32) -> f32 {
    let truncated = value as i32 as f32;
    if truncated > value {
        truncated - 1.0
    } else {
        truncated
    }
}

/// Returns the smallest integer that is not less than `value`.
fn ceil(value: f32) -> f32 {
    let truncated = value as i32 as f32;
    if truncated < value {
        truncated + 1.0
    } else {
        truncated
    }
}

/// Radius of the corners of tiles and chips.
const CORNER_RADIUS: u32 = 5;

/// Space between a name and its unit.
const CHAR_GAP: i32 = 6;

/// Left edge of the first tile.
const TILE_LEFT: i32 = 3;

/// Top edge of the tiles.
const TILE_TOP: i32 = 4;

/// Width of a tile, which fits the longest caption and its unit.
const TILE_WIDTH: i32 = 77;

/// Height of a tile.
const TILE_HEIGHT: i32 = 46;

/// Distance from the left edge of a tile to the left edge of the next one.
const TILE_PITCH: i32 = TILE_WIDTH + 2;

/// Distance of the content of a tile from its edges.
const TILE_PADDING: i32 = 5;

/// Top edge of the name of the plotted metric.
const HEADER_TOP: i32 = 56;

/// Left edge of the plot, right of the labels of the gridlines.
const PLOT_LEFT: i32 = 28;

/// Right edge of the plot, aligned with the right edge of the tiles.
const PLOT_RIGHT: i32 = PLOT_LEFT + BUCKETS as i32 - 1;

/// Top edge of the plot.
const PLOT_TOP: i32 = 72;

/// Height of the plot.
const PLOT_HEIGHT: i32 = 120;

/// Bottom edge of the plot.
const PLOT_BOTTOM: i32 = PLOT_TOP + PLOT_HEIGHT - 1;

/// Maximum number of gridlines.
const MAX_GRIDLINES: usize = 8;

/// Top edge of the chips of the intervals.
const CHIPS_TOP: i32 = 198;

/// Width of a chip.
const CHIP_WIDTH: i32 = 32;

/// Height of a chip.
const CHIP_HEIGHT: i32 = 15;

/// Space between two chips.
const CHIP_GAP: i32 = 5;

/// Distance from the left edge of a chip to the left edge of the next one.
const CHIP_PITCH: i32 = CHIP_WIDTH + CHIP_GAP;

/// Left edge of the first chip, so that the chips are centered on the plot.
const CHIPS_LEFT: i32 =
    (PLOT_LEFT + PLOT_RIGHT + 1) / 2 - (Interval::ALL.len() as i32 * CHIP_PITCH - CHIP_GAP) / 2;

/// Top edge of the line with the state of the device.
const STATUS_TOP: i32 = 224;

/// Height of the line with the state of the device.
const STATUS_HEIGHT: i32 = 12;

/// Diameter of the dot that shows the kind of the state.
const DOT_DIAMETER: u32 = 7;
