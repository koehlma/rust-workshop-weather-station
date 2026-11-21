//! Animated scene of the [`Weather`], 320 × 240 pixels in landscape.
//!
//! A landscape of rolling hills lies under a sky. The sun glows in the top
//! right corner, soft clouds drift behind the hills, and rain or snow falls
//! to the ground. The temperature and a description of the weather are in the
//! top left corner, and a translucent bar at the bottom shows the humidity,
//! the UV index, and the air pressure with its trend. The sky is bluer the
//! stronger the sun, tinted icy when it is cold and warm when it is hot, and
//! the hills are snowy at or below 0 °C.
//!
//! [`Scene::paint`] shades every pixel of a [`Strip`] with 8 bits per channel
//! and dithers it to the 16-bit colors of the display, so that gradients, the
//! glow of the sun, and the edges of clouds stay smooth. It then blends rain
//! or snow over the strip and draws the text. Positions follow from the time
//! since the start of the animation, so the animation runs at the same speed
//! at any frame rate.

use core::fmt::Write as _;

use embedded_graphics::mono_font::MonoFont;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::Alignment;
use heapless::String;
use heapless::Vec;
use micromath::F32Ext as _;
use profont::PROFONT_7_POINT;
use profont::PROFONT_12_POINT;
use profont::PROFONT_14_POINT;
use profont::PROFONT_24_POINT;

use crate::strip::STRIP_HEIGHT;
use crate::strip::Strip;
use crate::style::GOOD;
use crate::style::MUTED;
use crate::style::TEXT;
use crate::style::WARNING;
use crate::style::text;
use crate::style::width;
use crate::weather::Precipitation;
use crate::weather::Trend;
use crate::weather::Weather;

/// Frame of the animation.
pub struct Scene {
    weather: Weather,
    /// Indicates whether the measurements are too old to be current.
    stale: bool,
    /// Time since the start of the animation in milliseconds.
    time: u32,
    /// Color of each row of the sky.
    sky: [Color; HEIGHT as usize],
    /// Top of the far hills in each column, in sixteenths of a pixel.
    far: [i16; WIDTH as usize],
    /// Top of the near hills in each column, in sixteenths of a pixel.
    near: [i16; WIDTH as usize],
    /// Color of the far hills.
    far_color: Color,
    /// Color of the near hills at their top and further down.
    near_colors: (Color, Color),
    /// Sun, if it shines.
    sun: Option<Sun>,
    /// Clouds from back to front.
    clouds: Vec<Cloud, { CLOUDS.len() }>,
}

impl Scene {
    /// Computes the frame of `weather` at `time` milliseconds since the start
    /// of the animation.
    pub fn new(weather: Weather, stale: bool, time: u32) -> Self {
        let sun = weather.sun();
        let precipitation = weather.precipitation();
        let gloomy = weather.overcast() || precipitation.is_some();
        // The sky is clearer the stronger the sun and grayer when gloomy.
        let mut clearness = 0.3 + 0.7 * sun;
        if gloomy {
            clearness *= 0.45;
        }
        let clearness = (clearness * 256.0) as u32;
        let mut top = DIM_TOP.mix(CLEAR_TOP, clearness);
        let mut horizon = DIM_HORIZON.mix(CLEAR_HORIZON, clearness);
        if let Some(temperature) = weather.temperature_celsius {
            let tint = if temperature <= 0.0 {
                Some((ICY, (-temperature * 12.0).clamp(40.0, 110.0)))
            } else if temperature >= 25.0 {
                Some((WARM, ((temperature - 25.0) * 12.0).clamp(40.0, 110.0)))
            } else {
                None
            };
            if let Some((tint, amount)) = tint {
                horizon = horizon.mix(tint, amount as u32);
                top = top.mix(tint, amount as u32 / 3);
            }
        }
        let sky = core::array::from_fn(|y| {
            let fraction = (y as u32 * 256 / HORIZON as u32).min(256);
            top.mix(horizon, fraction)
        });

        let snowy = precipitation == Some(Precipitation::Snow)
            || weather.temperature_celsius.is_some_and(|t| t <= 0.0);
        let (far_color, near_colors) = if snowy {
            (SNOW_FAR, (SNOW_NEAR_TOP, SNOW_NEAR_BOTTOM))
        } else {
            let far = horizon.mix(HAZE, 150);
            let near_top = DIM_GRASS.mix(GRASS, clearness);
            (far, (near_top, GRASS_SHADOW))
        };
        let far = core::array::from_fn(|x| {
            let x = x as f32;
            let height = 148.0 + 9.0 * (x * 0.021 + 1.3).sin() + 5.0 * (x * 0.053 + 0.4).sin();
            (height * 16.0) as i16
        });
        let near = core::array::from_fn(|x| {
            let x = x as f32;
            let height = 176.0 + 11.0 * (x * 0.014 + 2.2).sin() + 4.0 * (x * 0.067 + 1.0).sin();
            (height * 16.0) as i16
        });

        let sun = (sun > 0.0).then(|| Sun::new(sun));
        let count = match precipitation {
            Some(_) => CLOUDS.len(),
            None => weather.clouds(),
        };
        let (light, shadow) = if gloomy {
            (DARK_CLOUD, DARK_CLOUD_SHADOW)
        } else {
            (CLOUD, CLOUD_SHADOW)
        };
        let mut clouds: Vec<Cloud, { CLOUDS.len() }> = CLOUDS[..count]
            .iter()
            .map(|template| Cloud::new(template, time, light, shadow))
            .collect();
        // Smaller clouds are further away, so they are drawn first.
        clouds.sort_unstable_by_key(|cloud| cloud.bottom - cloud.top);
        Self {
            weather,
            stale,
            time,
            sky,
            far,
            near,
            far_color,
            near_colors,
            sun,
            clouds,
        }
    }

    /// Paints the frame onto `strip`.
    pub fn paint(&self, strip: &mut Strip) {
        let top = strip.top();
        for row in 0..STRIP_HEIGHT as usize {
            let y = top + row as i32;
            self.shade_row(y, strip.row_mut(row));
        }
        match self.weather.precipitation() {
            Some(Precipitation::Rain) => self.paint_rain(strip),
            Some(Precipitation::Snow) => self.paint_snow(strip),
            None => {}
        }
        let Ok(()) = self.draw_headline(strip);
        let Ok(()) = self.draw_bar(strip);
    }

    /// Shades the pixels of row `y`.
    fn shade_row(&self, y: i32, pixels: &mut [Rgb565]) {
        // Clouds that cover the row, with their color in the row.
        let clouds: Vec<(&Cloud, Color), { CLOUDS.len() }> = self
            .clouds
            .iter()
            .filter(|cloud| (cloud.top..=cloud.bottom).contains(&y))
            .map(|cloud| (cloud, cloud.color_at(y)))
            .collect();
        let sky = self.sky[y as usize];
        let vignette_y = vignette_term(y, VIGNETTE_HEIGHT, VIGNETTE_Y_SCALE);
        let thresholds = &BAYER[(y & 3) as usize * 4..][..4];
        for (x, pixel) in (0..WIDTH).zip(pixels.iter_mut()) {
            let mut color = sky;
            if let Some(sun) = &self.sun {
                color = sun.shade(x, y, color);
            }
            for (cloud, cloud_color) in &clouds {
                color = cloud.shade(x, y, color, *cloud_color);
            }
            color = self.shade_hills(x, y, color);
            if x < VIGNETTE_WIDTH {
                let strength =
                    256 - vignette_term(x, VIGNETTE_WIDTH, VIGNETTE_X_SCALE) - vignette_y;
                if strength > 0 {
                    color = color.mix(VIGNETTE, strength as u32 * 90 / 256);
                }
            }
            if y >= BAR_TOP {
                color = color.mix(BAR, 200);
                if y == BAR_TOP {
                    color = color.mix(Color::new(255, 255, 255), 28);
                }
            }
            *pixel = color.dither(thresholds[(x & 3) as usize]);
        }
    }

    /// Shades the hills at `(x, y)` over `color`, with smooth edges.
    fn shade_hills(&self, x: i32, y: i32, color: Color) -> Color {
        let far = i32::from(self.far[x as usize]);
        let near = i32::from(self.near[x as usize]);
        if (y + 1) * 16 <= far.min(near) {
            return color;
        }
        let color = color.mix(self.far_color, coverage(y, far));
        let near_coverage = coverage(y, near);
        if near_coverage == 0 {
            return color;
        }
        // The near hills darken further down.
        let depth = ((y * 16 - near) >> 4).clamp(0, 40) as u32;
        let (top, bottom) = self.near_colors;
        // `depth * 256 / 40` without a division.
        color.mix(top.mix(bottom, (depth * 410) >> 6), near_coverage)
    }

    /// Returns the row of the ground in column `x`.
    fn ground(&self, x: i32) -> i32 {
        i32::from(self.near[x.clamp(0, WIDTH - 1) as usize]) / 16
    }

    /// Blends rain over `strip`.
    fn paint_rain(&self, strip: &mut Strip) {
        let span = (HEIGHT - RAIN_TOP) as u32;
        for index in 0..RAIN_DROPS {
            let speed = 230 + hash(index, 1) % 80;
            let fall = (hash(index, 2) + distance(self.time, speed)) % span;
            let head_y = RAIN_TOP + fall as i32;
            let head_x = (hash(index, 3) % (WIDTH as u32 + 40)) as i32 - 20 + head_y / 4;
            for step in 0..RAIN_LENGTH {
                let point = Point::new(head_x - step / 4, head_y - step);
                if point.y >= self.ground(point.x) {
                    continue;
                }
                let alpha = 150 - step as u32 * 12;
                if let Some(pixel) = strip.pixel_mut(point) {
                    *pixel = Color::from(*pixel).mix(RAIN, alpha).nearest();
                }
            }
        }
    }

    /// Blends snow over `strip`.
    fn paint_snow(&self, strip: &mut Strip) {
        let span = (HEIGHT - SNOW_TOP) as u32;
        let seconds = self.time as f32 / 1000.0;
        for index in 0..SNOW_FLAKES {
            let speed = 16 + hash(index, 1) % 22;
            let fall = (hash(index, 2) + distance(self.time, speed)) % span;
            let y = SNOW_TOP + fall as i32;
            let sway = 5.0 * (seconds * 1.1 + index as f32 * 0.7).sin();
            let x = (hash(index, 3) % WIDTH as u32) as i32 + sway as i32;
            if y >= self.ground(x) {
                continue;
            }
            let large = hash(index, 4).is_multiple_of(3);
            let mut blend = |point: Point, alpha: u32| {
                if let Some(pixel) = strip.pixel_mut(point) {
                    *pixel = Color::from(*pixel).mix(SNOW, alpha).nearest();
                }
            };
            blend(Point::new(x, y), 240);
            let edge = if large { 150 } else { 70 };
            for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                blend(Point::new(x + offset.0, y + offset.1), edge);
            }
        }
    }

    /// Draws the temperature and the description of the weather.
    fn draw_headline(&self, strip: &mut Strip) -> Result<(), core::convert::Infallible> {
        let color = if self.stale { MUTED } else { TEXT };
        let left = HEADLINE_LEFT;
        match self.weather.temperature_celsius {
            Some(temperature) => {
                let mut value = String::<12>::new();
                // Values that do not fit are cut off.
                let _ = write!(value, "{temperature:.1}");
                shadowed(strip, &value, left, 12, &PROFONT_24_POINT, color)?;
                let unit_left = left + width(&value, &PROFONT_24_POINT) + 3;
                shadowed(strip, "°C", unit_left, 16, &PROFONT_14_POINT, color)?;
            }
            None => {
                let placeholder = Rectangle::new(Point::new(left, 26), Size::new(28, 4));
                strip.fill_solid(&placeholder, MUTED)?;
            }
        }
        let mut description = String::<32>::new();
        // Descriptions are shorter than the capacity.
        let _ = write!(description, "{}", self.weather.description());
        shadowed(strip, &description, left, 47, &PROFONT_12_POINT, color)
    }

    /// Draws the humidity, the UV index, and the air pressure into the bar.
    fn draw_bar(&self, strip: &mut Strip) -> Result<(), core::convert::Infallible> {
        let weather = &self.weather;
        let columns = [
            ("HUMIDITY", weather.humidity_percent, 0, " %"),
            ("UV INDEX", weather.uv_index, 1, ""),
            ("PRESSURE", weather.pressure_hectopascals, 0, " hPa"),
        ];
        let caption_top = BAR_TOP + 8;
        let value_top = BAR_TOP + 21;
        for (index, (caption, value, precision, unit)) in columns.into_iter().enumerate() {
            let left = BAR_LEFT + index as i32 * BAR_PITCH;
            text(
                strip,
                caption,
                left,
                caption_top,
                &PROFONT_7_POINT,
                MUTED,
                Alignment::Left,
            )?;
            let Some(value) = value else {
                let placeholder = Rectangle::new(Point::new(left, value_top + 7), Size::new(16, 2));
                strip.fill_solid(&placeholder, MUTED)?;
                continue;
            };
            let mut formatted = String::<16>::new();
            // Values that do not fit are cut off.
            let _ = write!(formatted, "{value:.precision$}{unit}");
            let color = if self.stale { MUTED } else { TEXT };
            text(
                strip,
                &formatted,
                left,
                value_top,
                &PROFONT_12_POINT,
                color,
                Alignment::Left,
            )?;
        }
        if let Some(trend) = weather.trend {
            let (label, color) = match trend {
                Trend::Falling => ("FALLING", WARNING),
                Trend::Steady => ("STEADY", MUTED),
                Trend::Rising => ("RISING", GOOD),
            };
            let right = BAR_LEFT + 3 * BAR_PITCH - 12;
            text(
                strip,
                label,
                right,
                caption_top,
                &PROFONT_7_POINT,
                color,
                Alignment::Right,
            )?;
        }
        Ok(())
    }
}

/// Sun with its glow.
struct Sun {
    /// Opacity of the glow for each step of the squared distance from the
    /// center, from 0 to 256.
    glow: [u8; GLOW_STEPS],
}

impl Sun {
    /// Creates a sun of `strength` from 0 to 1.
    fn new(strength: f32) -> Self {
        let glow = core::array::from_fn(|step| {
            let distance = (step as f32 / GLOW_STEPS as f32).sqrt();
            let falloff = (1.0 - distance) * (1.0 - distance);
            (falloff * (110.0 + 120.0 * strength)) as u8
        });
        Self { glow }
    }

    /// Shades the sun at `(x, y)` over `color`.
    fn shade(&self, x: i32, y: i32, color: Color) -> Color {
        let (dx, dy) = (x - SUN_X, y - SUN_Y);
        let squared = dx * dx + dy * dy;
        if squared >= GLOW_RADIUS * GLOW_RADIUS {
            return color;
        }
        let step = (squared * GLOW_SCALE) >> 16;
        let color = color.mix(GLOW, u32::from(self.glow[step as usize]));
        // The disk has an edge of two pixels, from `(r - 1)²` to `(r + 1)²`.
        let outer = (SUN_RADIUS + 1) * (SUN_RADIUS + 1);
        if squared >= outer {
            return color;
        }
        let coverage = ((outer - squared) * 256 / (4 * SUN_RADIUS)).min(256);
        color.mix(SUN, coverage as u32)
    }
}

/// Template of a cloud.
struct CloudTemplate {
    /// Row of the center.
    y: i32,
    /// Size relative to the blobs of the shape.
    scale: f32,
    /// Speed in pixels per second.
    speed: u32,
    /// Horizontal position at the start of the animation.
    phase: u32,
    /// Blobs of the shape: offsets from the center and radius.
    shape: &'static [(i32, i32, i32)],
}

/// Cloud at its position in a frame.
struct Cloud {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    /// Row below which the cloud fades, so that its base is flat.
    base: i32,
    blobs: Vec<Blob, MAX_BLOBS>,
    /// Color of the lit top.
    light: Color,
    /// Color of the shaded bottom.
    shadow: Color,
}

/// Round part of a cloud.
struct Blob {
    x: i32,
    y: i32,
    /// Squared radius.
    squared: i32,
    /// `65536 / squared`, which turns a distance into a density without a
    /// division.
    scale: i32,
}

impl Cloud {
    /// Places the cloud of `template` at `time`.
    fn new(template: &CloudTemplate, time: u32, light: Color, shadow: Color) -> Self {
        let span = WIDTH as u32 + 2 * CLOUD_MARGIN as u32;
        let x = ((template.phase + distance(time, template.speed)) % span) as i32 - CLOUD_MARGIN;
        let scaled = |value: i32| (value as f32 * template.scale) as i32;
        let blobs: Vec<Blob, MAX_BLOBS> = template
            .shape
            .iter()
            .map(|&(dx, dy, radius)| {
                let radius = scaled(radius);
                Blob {
                    x: x + scaled(dx),
                    y: template.y + scaled(dy),
                    squared: radius * radius,
                    scale: 65536 / (radius * radius),
                }
            })
            .collect();
        let radius = |blob: &Blob| (blob.squared as f32).sqrt() as i32;
        let left = blobs.iter().map(|b| b.x - radius(b)).min().unwrap_or(x);
        let right = blobs.iter().map(|b| b.x + radius(b)).max().unwrap_or(x);
        let top = blobs
            .iter()
            .map(|b| b.y - radius(b))
            .min()
            .unwrap_or(template.y);
        let bottom = blobs
            .iter()
            .map(|b| b.y + radius(b))
            .max()
            .unwrap_or(template.y);
        Self {
            left,
            right,
            top,
            bottom,
            base: template.y + scaled(8),
            blobs,
            light,
            shadow,
        }
    }

    /// Returns the color of the cloud in row `y`, lit from above.
    fn color_at(&self, y: i32) -> Color {
        let height = (self.bottom - self.top).max(1);
        let fraction = ((y - self.top) * 256 / height).clamp(0, 256) as u32;
        self.light.mix(self.shadow, fraction)
    }

    /// Shades the cloud at `(x, y)` over `color` with `cloud_color`.
    fn shade(&self, x: i32, y: i32, color: Color, cloud_color: Color) -> Color {
        if x < self.left || x > self.right {
            return color;
        }
        // The densities of the blobs add up, so that they merge smoothly.
        let mut density = 0;
        for blob in &self.blobs {
            let (dx, dy) = (x - blob.x, y - blob.y);
            let squared = dx * dx + dy * dy;
            if squared < blob.squared {
                density += ((blob.squared - squared) * blob.scale) >> 8;
            }
        }
        if y > self.base {
            density -= (y - self.base) * 40;
        }
        // The edge fades over a few pixels.
        let alpha = ((density - 20) * 4).clamp(0, 256);
        color.mix(cloud_color, alpha as u32)
    }
}

/// Color with 8 bits per channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Mixes `amount` / 256 of `other` into the color.
    fn mix(self, other: Self, amount: u32) -> Self {
        match amount {
            0 => return self,
            256.. => return other,
            _ => {}
        }
        let channel =
            |a: u8, b: u8| ((u32::from(a) * (256 - amount) + u32::from(b) * amount) >> 8) as u8;
        Self::new(
            channel(self.r, other.r),
            channel(self.g, other.g),
            channel(self.b, other.b),
        )
    }

    /// Converts the color to the display with an ordered dither of
    /// `threshold`, a value of [`BAYER`].
    ///
    /// The display has 5 bits for red and blue and 6 bits for green, which
    /// turns smooth gradients into bands. The dither mixes the two nearest
    /// colors instead.
    fn dither(self, threshold: u8) -> Rgb565 {
        // `value * max * 16 / 255` without a division, in sixteenths of a
        // step of the display.
        let channel = |value: u8, max: u32| {
            let scaled = (u32::from(value) * max * 16 * 257) >> 16;
            let base = scaled >> 4;
            (base + u32::from((scaled & 15) as u8 > threshold)).min(max) as u8
        };
        Rgb565::new(
            channel(self.r, 31),
            channel(self.g, 63),
            channel(self.b, 31),
        )
    }

    /// Converts the color to the nearest color of the display.
    fn nearest(self) -> Rgb565 {
        self.dither(7)
    }
}

impl From<Rgb565> for Color {
    fn from(color: Rgb565) -> Self {
        Self::new(
            color.r() << 3 | color.r() >> 2,
            color.g() << 2 | color.g() >> 4,
            color.b() << 3 | color.b() >> 2,
        )
    }
}

/// Returns the share of row `y` below the edge at `edge` sixteenths of a
/// pixel, from 0 to 256.
fn coverage(y: i32, edge: i32) -> u32 {
    ((y * 16 + 16 - edge) * 16).clamp(0, 256) as u32
}

/// Returns `256 * (value / extent)²`, the share of the vignette along one
/// axis, with `scale` being `256 * 65536 / extent²`.
fn vignette_term(value: i32, extent: i32, scale: i32) -> i32 {
    if value >= extent {
        return 256;
    }
    (value * value * scale) >> 16
}

/// Draws `content` with a shadow, so that it is readable on a light sky.
fn shadowed(
    strip: &mut Strip,
    content: &str,
    x: i32,
    y: i32,
    font: &MonoFont<'_>,
    color: Rgb565,
) -> Result<(), core::convert::Infallible> {
    text(strip, content, x + 1, y + 1, font, SHADOW, Alignment::Left)?;
    text(strip, content, x, y, font, color, Alignment::Left)
}

/// Returns how far something that moves at `speed` pixels per second has
/// moved after `time` milliseconds.
fn distance(time: u32, speed: u32) -> u32 {
    (u64::from(time) * u64::from(speed) / 1000) as u32
}

/// Returns a pseudo-random number below 1000 for the `index`-th element and
/// the `salt`-th property, so that elements spread evenly but irregularly.
fn hash(index: u32, salt: u32) -> u32 {
    let mut value = index.wrapping_mul(0x9E37_79B9) ^ salt.wrapping_mul(0x85EB_CA6B);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7FEB_352D);
    value ^= value >> 15;
    value % 1000
}

/// Width of the screen.
const WIDTH: i32 = 320;

/// Height of the screen.
const HEIGHT: i32 = 240;

/// Row at which the sky reaches the color of the horizon.
const HORIZON: i32 = 180;

/// Top edge of the bar.
const BAR_TOP: i32 = 196;

/// Left edge of the first column of the bar.
const BAR_LEFT: i32 = 12;

/// Distance between the columns of the bar.
const BAR_PITCH: i32 = 104;

/// Left edge of the temperature and the description.
const HEADLINE_LEFT: i32 = 14;

/// Extent of the vignette behind the headline.
const VIGNETTE_WIDTH: i32 = 230;

/// Height of the vignette behind the headline.
const VIGNETTE_HEIGHT: i32 = 100;

/// Scale of [`vignette_term`] for [`VIGNETTE_WIDTH`].
const VIGNETTE_X_SCALE: i32 = 256 * 65536 / (VIGNETTE_WIDTH * VIGNETTE_WIDTH);

/// Scale of [`vignette_term`] for [`VIGNETTE_HEIGHT`].
const VIGNETTE_Y_SCALE: i32 = 256 * 65536 / (VIGNETTE_HEIGHT * VIGNETTE_HEIGHT);

/// Center of the sun.
const SUN_X: i32 = 248;

/// Center of the sun.
const SUN_Y: i32 = 58;

/// Radius of the disk of the sun.
const SUN_RADIUS: i32 = 15;

/// Radius of the glow of the sun.
const GLOW_RADIUS: i32 = 84;

/// Number of steps of the glow of the sun.
const GLOW_STEPS: usize = 64;

/// `GLOW_STEPS * 65536 / GLOW_RADIUS²`, which turns a squared distance into a
/// step without a division.
const GLOW_SCALE: i32 = (GLOW_STEPS as i32 * 65536) / (GLOW_RADIUS * GLOW_RADIUS);

/// Distance outside of the screen at which clouds turn around.
const CLOUD_MARGIN: i32 = 70;

/// Maximum number of blobs of a cloud.
const MAX_BLOBS: usize = 6;

/// Shapes of clouds: offsets of blobs from the center and their radius.
const SHAPES: [&[(i32, i32, i32)]; 3] = [
    &[
        (-26, 6, 13),
        (-10, -4, 19),
        (10, -8, 17),
        (27, 3, 14),
        (0, 8, 16),
    ],
    &[
        (-20, 4, 15),
        (-2, -6, 20),
        (18, 0, 15),
        (34, 6, 11),
        (-34, 8, 10),
    ],
    &[
        (-14, 2, 14),
        (4, -6, 17),
        (20, 4, 13),
        (-28, 8, 10),
        (6, 8, 14),
    ],
];

/// Clouds in the order in which they appear with growing humidity. Rain and
/// snow bring all of them.
const CLOUDS: [CloudTemplate; 7] = [
    CloudTemplate {
        y: 92,
        scale: 1.0,
        speed: 7,
        phase: 140,
        shape: SHAPES[0],
    },
    CloudTemplate {
        y: 128,
        scale: 0.75,
        speed: 5,
        phase: 330,
        shape: SHAPES[1],
    },
    CloudTemplate {
        y: 108,
        scale: 1.2,
        speed: 9,
        phase: 20,
        shape: SHAPES[2],
    },
    CloudTemplate {
        y: 70,
        scale: 0.6,
        speed: 4,
        phase: 250,
        shape: SHAPES[0],
    },
    CloudTemplate {
        y: 140,
        scale: 1.1,
        speed: 11,
        phase: 420,
        shape: SHAPES[1],
    },
    CloudTemplate {
        y: 40,
        scale: 1.3,
        speed: 6,
        phase: 80,
        shape: SHAPES[2],
    },
    CloudTemplate {
        y: 52,
        scale: 1.1,
        speed: 8,
        phase: 360,
        shape: SHAPES[0],
    },
];

/// Top edge of the area in which rain falls.
const RAIN_TOP: i32 = 60;

/// Number of rain drops.
const RAIN_DROPS: u32 = 70;

/// Length of a rain drop.
const RAIN_LENGTH: i32 = 10;

/// Top edge of the area in which snow falls.
const SNOW_TOP: i32 = 50;

/// Number of snow flakes.
const SNOW_FLAKES: u32 = 80;

/// Thresholds of a 4 × 4 ordered dither, row by row.
const BAYER: [u8; 16] = [0, 8, 2, 10, 12, 4, 14, 6, 3, 11, 1, 9, 15, 7, 13, 5];

/// Top of a clear sky.
const CLEAR_TOP: Color = Color::new(28, 88, 178);

/// Horizon of a clear sky.
const CLEAR_HORIZON: Color = Color::new(170, 210, 240);

/// Top of a dim sky.
const DIM_TOP: Color = Color::new(42, 52, 72);

/// Horizon of a dim sky.
const DIM_HORIZON: Color = Color::new(122, 134, 152);

/// Tint of a cold sky.
const ICY: Color = Color::new(190, 232, 255);

/// Tint of a hot sky.
const WARM: Color = Color::new(255, 178, 110);

/// Color into which distant hills fade.
const HAZE: Color = Color::new(64, 96, 122);

/// Grass of the near hills in sunlight.
const GRASS: Color = Color::new(70, 122, 82);

/// Grass of the near hills under a dim sky.
const DIM_GRASS: Color = Color::new(52, 70, 64);

/// Grass of the near hills further down.
const GRASS_SHADOW: Color = Color::new(22, 40, 36);

/// Snow of the far hills.
const SNOW_FAR: Color = Color::new(184, 198, 218);

/// Snow of the near hills at their top.
const SNOW_NEAR_TOP: Color = Color::new(238, 243, 250);

/// Snow of the near hills further down.
const SNOW_NEAR_BOTTOM: Color = Color::new(176, 190, 212);

/// Color of the glow of the sun.
const GLOW: Color = Color::new(255, 240, 200);

/// Color of the disk of the sun.
const SUN: Color = Color::new(255, 230, 150);

/// Lit top of a cloud.
const CLOUD: Color = Color::new(255, 255, 255);

/// Shaded bottom of a cloud.
const CLOUD_SHADOW: Color = Color::new(196, 206, 222);

/// Lit top of a dark cloud.
const DARK_CLOUD: Color = Color::new(172, 180, 194);

/// Shaded bottom of a dark cloud.
const DARK_CLOUD_SHADOW: Color = Color::new(108, 116, 132);

/// Color of rain.
const RAIN: Color = Color::new(206, 224, 255);

/// Color of snow.
const SNOW: Color = Color::new(255, 255, 255);

/// Color into which the vignette darkens the sky behind the headline.
const VIGNETTE: Color = Color::new(10, 16, 30);

/// Color of the translucent bar.
const BAR: Color = Color::new(10, 12, 18);

/// Color of the shadow of text.
const SHADOW: Rgb565 = Rgb565::new(1, 2, 4);
