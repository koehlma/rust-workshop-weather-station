//! Horizontal strip of the screen in memory.
//!
//! A full frame of 320 × 240 pixels takes 150 KB, more than the RP2040 can
//! spare. [`Strip`] holds [`STRIP_HEIGHT`] rows instead. The display draws a
//! frame strip by strip and sends each strip in one transfer, so that the
//! screen never shows a partially drawn frame. The bounding box of a strip is
//! its rows, so that drawing code can skip what lies outside of it.

use core::convert::Infallible;

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

/// Width of the screen.
pub const SCREEN_WIDTH: u32 = 320;

/// Height of the screen.
pub const SCREEN_HEIGHT: u32 = 240;

/// Number of rows of a strip, a divisor of [`SCREEN_HEIGHT`].
pub const STRIP_HEIGHT: u32 = 24;

/// Rows of the screen in memory.
pub struct Strip {
    pixels: [Rgb565; (SCREEN_WIDTH * STRIP_HEIGHT) as usize],
    /// First row of the strip.
    top: i32,
}

impl Strip {
    /// Creates a strip at the top of the screen.
    pub const fn new() -> Self {
        Self {
            pixels: [Rgb565::BLACK; (SCREEN_WIDTH * STRIP_HEIGHT) as usize],
            top: 0,
        }
    }

    /// Moves the strip to the rows from `top` on.
    pub fn move_to(&mut self, top: i32) {
        self.top = top;
    }

    /// Returns the pixels of the strip, row by row.
    pub fn pixels(&self) -> &[Rgb565] {
        &self.pixels
    }

    /// Returns the index of `point` in the pixels, if the strip holds it.
    fn index(&self, point: Point) -> Option<usize> {
        let row = point.y - self.top;
        let inside =
            (0..SCREEN_WIDTH as i32).contains(&point.x) && (0..STRIP_HEIGHT as i32).contains(&row);
        inside.then(|| row as usize * SCREEN_WIDTH as usize + point.x as usize)
    }
}

impl Dimensions for Strip {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(
            Point::new(0, self.top),
            Size::new(SCREEN_WIDTH, STRIP_HEIGHT),
        )
    }
}

impl DrawTarget for Strip {
    type Color = Rgb565;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            if let Some(index) = self.index(point) {
                self.pixels[index] = color;
            }
        }
        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        // Colors are computed lazily, so areas outside of the strip cost
        // nothing.
        if self.bounding_box().intersection(area).is_zero_sized() {
            return Ok(());
        }
        for (point, color) in area.points().zip(colors) {
            if let Some(index) = self.index(point) {
                self.pixels[index] = color;
            }
        }
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let area = self.bounding_box().intersection(area);
        let Some(bottom_right) = area.bottom_right() else {
            return Ok(());
        };
        let (left, right) = (area.top_left.x as usize, bottom_right.x as usize);
        for y in area.top_left.y..=bottom_right.y {
            let row = (y - self.top) as usize * SCREEN_WIDTH as usize;
            self.pixels[row + left..=row + right].fill(color);
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.pixels.fill(color);
        Ok(())
    }
}
