//! Colors and text of the screens.

use embedded_graphics::mono_font::MonoFont;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Alignment;
use embedded_graphics::text::Baseline;
use embedded_graphics::text::Text;
use embedded_graphics::text::TextStyleBuilder;

/// Converts a color with 8 bits per channel.
pub const fn rgb(red: u8, green: u8, blue: u8) -> Rgb565 {
    Rgb565::new(red >> 3, green >> 2, blue >> 3)
}

/// Mixes `alpha` / 255 of `color` with the rest of `background`.
pub fn blend(color: Rgb565, background: Rgb565, alpha: u8) -> Rgb565 {
    let mix = |color: u8, background: u8| {
        let alpha = u16::from(alpha);
        ((u16::from(color) * alpha + u16::from(background) * (255 - alpha) + 127) / 255) as u8
    };
    Rgb565::new(
        mix(color.r(), background.r()),
        mix(color.g(), background.g()),
        mix(color.b(), background.b()),
    )
}

/// Background of the screen.
pub const BACKGROUND: Rgb565 = rgb(11, 13, 18);

/// Background of tiles and chips.
pub const CARD: Rgb565 = rgb(27, 31, 40);

/// Background of the tile of the plotted metric.
pub const CARD_SELECTED: Rgb565 = rgb(38, 44, 56);

/// Color of text.
pub const TEXT: Rgb565 = rgb(236, 239, 244);

/// Color of secondary text.
pub const MUTED: Rgb565 = rgb(128, 137, 153);

/// Color of gridlines.
pub const GRID: Rgb565 = rgb(64, 72, 88);

/// Color of the state when everything works.
pub const GOOD: Rgb565 = rgb(64, 201, 122);

/// Color of the state when something needs attention.
pub const WARNING: Rgb565 = rgb(255, 184, 64);

/// Color of the state when something failed.
pub const ERROR: Rgb565 = rgb(255, 92, 92);

/// Draws `content` with its top at `y`, aligned at `x`.
pub fn text<D: DrawTarget<Color = Rgb565>>(
    target: &mut D,
    content: &str,
    x: i32,
    y: i32,
    font: &MonoFont<'_>,
    color: Rgb565,
    alignment: Alignment,
) -> Result<(), D::Error> {
    draw_text(
        target,
        content,
        Point::new(x, y),
        font,
        color,
        alignment,
        Baseline::Top,
    )
}

/// Draws `content` with its middle at `y`, aligned at `x`.
pub fn text_middle<D: DrawTarget<Color = Rgb565>>(
    target: &mut D,
    content: &str,
    x: i32,
    y: i32,
    font: &MonoFont<'_>,
    color: Rgb565,
    alignment: Alignment,
) -> Result<(), D::Error> {
    draw_text(
        target,
        content,
        Point::new(x, y),
        font,
        color,
        alignment,
        Baseline::Middle,
    )
}

/// Returns the width of `content` in `font`.
pub fn width(content: &str, font: &MonoFont<'_>) -> i32 {
    let advance = font.character_size.width + font.character_spacing;
    (content.chars().count() as u32 * advance) as i32
}

/// Draws `content` at `position`.
fn draw_text<D: DrawTarget<Color = Rgb565>>(
    target: &mut D,
    content: &str,
    position: Point,
    font: &MonoFont<'_>,
    color: Rgb565,
    alignment: Alignment,
    baseline: Baseline,
) -> Result<(), D::Error> {
    let style = MonoTextStyle::new(font, color);
    let text_style = TextStyleBuilder::new()
        .alignment(alignment)
        .baseline(baseline)
        .build();
    Text::with_text_style(content, position, style, text_style).draw(target)?;
    Ok(())
}
