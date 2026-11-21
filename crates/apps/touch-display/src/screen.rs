//! Layout of the screen in landscape, 320 × 240 pixels.
//!
//! From top to bottom, the screen shows the latest value of the plotted
//! [`Metric`] and the [`Interval`] of the plot, the latest values of the other
//! metrics, a plot of the interval, and the state of the storage with the
//! version of the firmware. Each column of the plot is a bucket of the
//! [`History`](crate::history::History): a band from its minimum to its
//! maximum and a line through its mean.

use core::fmt::Write as _;

use embedded_graphics::mono_font::MonoFont;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::iso_8859_1::FONT_5X8;
use embedded_graphics::mono_font::iso_8859_1::FONT_6X10;
use embedded_graphics::mono_font::iso_8859_1::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::Alignment;
use embedded_graphics::text::Baseline;
use embedded_graphics::text::Text;
use embedded_graphics::text::TextStyleBuilder;
use heapless::String;

use crate::history::BUCKETS;
use crate::history::Column;
use crate::history::Interval;
use crate::history::Metric;

/// Width of the screen.
const WIDTH: i32 = 320;

/// Distance of the text from the left and right edges.
const MARGIN: i32 = 8;

/// Left edge of the plot, right of the labels of the vertical axis.
const PLOT_LEFT: i32 = WIDTH - BUCKETS as i32;

/// Top edge of the plot.
const PLOT_TOP: i32 = 48;

/// Height of the plot.
const PLOT_HEIGHT: i32 = 156;

/// Bottom edge of the plot.
const PLOT_BOTTOM: i32 = PLOT_TOP + PLOT_HEIGHT - 1;

/// Top of the line with the state of the storage.
const STATUS_TOP: i32 = 226;

/// Color of the band from the minimum to the maximum of a column.
const BAND: Rgb565 = Rgb565::CSS_DARK_CYAN;

/// Color of the line through the means of the columns.
const LINE: Rgb565 = Rgb565::CYAN;

/// Color of the axes and their labels.
const AXIS: Rgb565 = Rgb565::CSS_GRAY;

/// Content of the screen.
pub struct Screen<'a> {
    /// Plotted metric.
    pub metric: Metric,
    /// Latest value of the plotted metric, if any.
    pub value: Option<f32>,
    /// Latest values of the other metrics.
    pub details: &'a str,
    /// Interval of the plot.
    pub interval: Interval,
    /// Columns of the plot, oldest first.
    pub columns: &'a [Option<Column>; BUCKETS],
    /// State of the storage.
    pub status: &'a str,
    /// Version of the firmware.
    pub firmware_version: &'a str,
}

/// Draws `screen` onto `target`, replacing what it showed before.
pub fn draw<D: DrawTarget<Color = Rgb565>>(
    target: &mut D,
    screen: &Screen,
) -> Result<(), D::Error> {
    let mut title = String::<32>::new();
    // Text that does not fit is cut off.
    let _ = match screen.value {
        Some(value) => write!(
            title,
            "{} {value:.precision$} {}",
            screen.metric.label(),
            screen.metric.unit(),
            precision = screen.metric.precision(),
        ),
        None => write!(title, "{} -", screen.metric.label()),
    };
    clear(target, 0, 0, WIDTH, PLOT_TOP)?;
    text(
        target,
        &title,
        MARGIN,
        6,
        &FONT_10X20,
        Rgb565::WHITE,
        Alignment::Left,
    )?;
    let interval = screen.interval.label();
    text(
        target,
        interval,
        WIDTH - MARGIN,
        6,
        &FONT_10X20,
        LINE,
        Alignment::Right,
    )?;
    text(
        target,
        screen.details,
        MARGIN,
        30,
        &FONT_6X10,
        AXIS,
        Alignment::Left,
    )?;

    draw_plot(target, screen)?;

    let [start, middle] = screen.interval.labels();
    let labels_top = PLOT_BOTTOM + 4;
    clear(target, 0, labels_top, WIDTH, STATUS_TOP - labels_top)?;
    text(
        target,
        start,
        PLOT_LEFT,
        labels_top,
        &FONT_6X10,
        AXIS,
        Alignment::Left,
    )?;
    let center = PLOT_LEFT + BUCKETS as i32 / 2;
    text(
        target,
        middle,
        center,
        labels_top,
        &FONT_6X10,
        AXIS,
        Alignment::Center,
    )?;
    text(
        target,
        "now",
        WIDTH - 1,
        labels_top,
        &FONT_6X10,
        AXIS,
        Alignment::Right,
    )?;

    clear(target, 0, STATUS_TOP, WIDTH, 240 - STATUS_TOP)?;
    text(
        target,
        screen.status,
        MARGIN,
        STATUS_TOP,
        &FONT_6X10,
        Rgb565::WHITE,
        Alignment::Left,
    )?;
    text(
        target,
        screen.firmware_version,
        WIDTH - MARGIN,
        STATUS_TOP,
        &FONT_6X10,
        AXIS,
        Alignment::Right,
    )
}

/// Draws the plot and the labels of its vertical axis.
fn draw_plot<D: DrawTarget<Color = Rgb565>>(
    target: &mut D,
    screen: &Screen,
) -> Result<(), D::Error> {
    clear(target, 0, PLOT_TOP, PLOT_LEFT - 1, PLOT_HEIGHT)?;
    let axes = [
        Rectangle::new(
            Point::new(PLOT_LEFT - 1, PLOT_TOP),
            Size::new(1, PLOT_HEIGHT as u32 + 1),
        ),
        Rectangle::new(
            Point::new(PLOT_LEFT - 1, PLOT_BOTTOM + 1),
            Size::new(BUCKETS as u32 + 1, 1),
        ),
    ];
    for axis in axes {
        target.fill_solid(&axis, AXIS)?;
    }
    let range = range(screen.metric, screen.columns);
    for (offset, column) in screen.columns.iter().enumerate() {
        let x = PLOT_LEFT + offset as i32;
        clear(target, x, PLOT_TOP, 1, PLOT_HEIGHT)?;
        let (Some(column), Some((low, high))) = (column, range) else {
            continue;
        };
        let y = |value: f32| {
            let fraction = (value - low) / (high - low);
            let y = PLOT_BOTTOM - (fraction * (PLOT_HEIGHT - 1) as f32 + 0.5) as i32;
            y.clamp(PLOT_TOP, PLOT_BOTTOM)
        };
        let (top, bottom) = (y(column.max), y(column.min));
        target.fill_solid(
            &Rectangle::new(Point::new(x, top), Size::new(1, (bottom - top + 1) as u32)),
            BAND,
        )?;
        let mean = y(column.mean).min(PLOT_BOTTOM - 1);
        target.fill_solid(&Rectangle::new(Point::new(x, mean), Size::new(1, 2)), LINE)?;
    }
    let Some((low, high)) = range else {
        let center = Point::new(PLOT_LEFT + BUCKETS as i32 / 2, PLOT_TOP + PLOT_HEIGHT / 2);
        let style = MonoTextStyle::new(&FONT_6X10, AXIS);
        let alignment = TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Middle)
            .build();
        Text::with_text_style("No measurements", center, style, alignment).draw(target)?;
        return Ok(());
    };
    let precision = match screen.metric {
        // Pressures take too much space with a decimal.
        Metric::Pressure => 0,
        metric => metric.precision(),
    };
    for (value, top) in [(high, PLOT_TOP), (low, PLOT_BOTTOM - 7)] {
        let mut label = String::<8>::new();
        let _ = write!(label, "{value:.precision$}");
        text(
            target,
            &label,
            PLOT_LEFT - 3,
            top,
            &FONT_5X8,
            AXIS,
            Alignment::Right,
        )?;
    }
    Ok(())
}

/// Returns the range of the vertical axis for `columns`, or `None` if they
/// hold no measurements.
fn range(metric: Metric, columns: &[Option<Column>]) -> Option<(f32, f32)> {
    let (low, high) =
        columns
            .iter()
            .flatten()
            .fold(None, |range: Option<(f32, f32)>, column| {
                let (low, high) = range.unwrap_or((column.min, column.max));
                Some((low.min(column.min), high.max(column.max)))
            })?;
    let span = (high - low).max(metric.min_span());
    let center = (low + high) / 2.0;
    // Leave a margin of 5 % above and below the measurements.
    let half = span * 0.55;
    Some((center - half, center + half))
}

/// Fills a rectangle with the background color.
fn clear<D: DrawTarget<Color = Rgb565>>(
    target: &mut D,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<(), D::Error> {
    let area = Rectangle::new(Point::new(x, y), Size::new(width as u32, height as u32));
    target.fill_solid(&area, Rgb565::BLACK)
}

/// Draws `content` with its top at `y`, aligned at `x`.
fn text<D: DrawTarget<Color = Rgb565>>(
    target: &mut D,
    content: &str,
    x: i32,
    y: i32,
    font: &MonoFont<'_>,
    color: Rgb565,
    alignment: Alignment,
) -> Result<(), D::Error> {
    let style = MonoTextStyle::new(font, color);
    let text_style = TextStyleBuilder::new()
        .alignment(alignment)
        .baseline(Baseline::Top)
        .build();
    Text::with_text_style(content, Point::new(x, y), style, text_style).draw(target)?;
    Ok(())
}
