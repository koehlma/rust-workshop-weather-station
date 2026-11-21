//! Animated scene of the [`Weather`], 320 × 240 pixels in landscape.
//!
//! A sky fills the screen above a bar with the humidity, the UV index, and
//! the air pressure with its trend. The sky is bluer the stronger the sun and
//! tinted icy when it is cold and warm when it is hot. The sun shines in the
//! top right corner with rotating rays, clouds drift from left to right, and
//! rain or snow falls. The temperature and a description of the weather are
//! in the top left corner.
//!
//! [`Scene::new`] computes a frame at a time since the start of the
//! animation. Positions follow from that time, so the animation runs at the
//! same speed at any frame rate.

use core::fmt::Write as _;

use embedded_graphics::mono_font::MonoFont;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
use embedded_graphics::primitives::Line;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::primitives::RoundedRectangle;
use embedded_graphics::text::Alignment;
use heapless::String;
use micromath::F32Ext as _;
use profont::PROFONT_7_POINT;
use profont::PROFONT_12_POINT;
use profont::PROFONT_14_POINT;
use profont::PROFONT_24_POINT;

use crate::style::CARD;
use crate::style::GOOD;
use crate::style::MUTED;
use crate::style::TEXT;
use crate::style::WARNING;
use crate::style::rgb;
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
    /// Color of each row of the sky, which the display approximates by
    /// dithering.
    sky: [Rgb888; SKY_BOTTOM as usize],
}

impl Scene {
    /// Computes the frame of `weather` at `time` milliseconds since the start
    /// of the animation.
    pub fn new(weather: Weather, stale: bool, time: u32) -> Self {
        let sun = weather.sun();
        // The sky is grayer the weaker the sun and darker when overcast.
        let mut clearness = 0.35 + 0.65 * sun;
        if weather.overcast() || weather.precipitation().is_some() {
            clearness *= 0.5;
        }
        let mut top = mix(CLEAR_TOP, DIM_TOP, clearness);
        let mut bottom = mix(CLEAR_BOTTOM, DIM_BOTTOM, clearness);
        if let Some(temperature) = weather.temperature_celsius {
            let (tint, strength) = if temperature <= 0.0 {
                (ICY, ((-temperature) / 10.0).clamp(0.15, 0.4))
            } else if temperature >= 25.0 {
                (WARM, ((temperature - 25.0) / 10.0).clamp(0.15, 0.4))
            } else {
                (bottom, 0.0)
            };
            bottom = mix(tint, bottom, strength);
            top = mix(tint, top, strength / 3.0);
        }
        let sky = core::array::from_fn(|row| {
            let fraction = row as f32 / (SKY_BOTTOM - 1) as f32;
            mix(bottom, top, fraction)
        });
        Self {
            weather,
            stale,
            time,
            sky,
        }
    }

    /// Returns the time since the start of the animation in seconds.
    fn seconds(&self) -> f32 {
        self.time as f32 / 1000.0
    }

    /// Returns how far something that moves at `speed` pixels per second has
    /// moved.
    fn distance(&self, speed: u32) -> i32 {
        (u64::from(self.time) * u64::from(speed) / 1000) as i32
    }

    /// Draws the sky with its gradient.
    ///
    /// The display has 5 bits for red and blue, which turns a smooth gradient
    /// into bands. An ordered dither mixes the two nearest colors instead.
    fn draw_sky<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        let visible = target.bounding_box();
        let Some(bottom_right) = visible.bottom_right() else {
            return Ok(());
        };
        for y in visible.top_left.y..=bottom_right.y.min(SKY_BOTTOM - 1) {
            let color = self.sky[y as usize];
            let red = Dithered::new(color.r(), 5);
            let green = Dithered::new(color.g(), 6);
            let blue = Dithered::new(color.b(), 5);
            let thresholds = &BAYER[(y & 3) as usize * 4..][..4];
            let colors = (0..WIDTH).map(|x| {
                let threshold = thresholds[(x & 3) as usize];
                Rgb565::new(red.at(threshold), green.at(threshold), blue.at(threshold))
            });
            let row = Rectangle::new(Point::new(0, y), Size::new(WIDTH as u32, 1));
            target.fill_contiguous(&row, colors)?;
        }
        Ok(())
    }

    /// Draws the sun with its glow and rotating rays.
    fn draw_sun<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        let sun = self.weather.sun();
        if sun <= 0.0 || !visible(target, SUN_BOUNDS) {
            return Ok(());
        }
        // The glow brightens the sky, because mixing yellow into blue turns
        // gray.
        let sky = self.sky[SUN_CENTER.y as usize];
        for (radius, brightness) in [(29, 24), (24, 52)] {
            Circle::with_center(SUN_CENTER, 2 * radius)
                .into_styled(PrimitiveStyle::with_fill(convert(brighten(
                    sky, brightness,
                ))))
                .draw(target)?;
        }
        let rays = PrimitiveStyle::with_stroke(convert(RAYS), 2);
        let length = 6.0 + 10.0 * sun;
        for index in 0..RAYS_COUNT {
            let angle =
                index as f32 * core::f32::consts::TAU / RAYS_COUNT as f32 + self.seconds() * 0.25;
            let (sin, cos) = (angle.sin(), angle.cos());
            let point =
                |radius: f32| SUN_CENTER + Point::new((cos * radius) as i32, (sin * radius) as i32);
            Line::new(point(30.0), point(30.0 + length))
                .into_styled(rays)
                .draw(target)?;
        }
        Circle::with_center(SUN_CENTER, 2 * 19)
            .into_styled(PrimitiveStyle::with_fill(convert(SUN)))
            .draw(target)
    }

    /// Draws the clouds, which drift from left to right.
    fn draw_clouds<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        let dark = self.weather.overcast() || self.weather.precipitation().is_some();
        let (body, shadow) = if dark {
            (DARK_CLOUD, DARK_CLOUD_SHADOW)
        } else {
            (CLOUD, CLOUD_SHADOW)
        };
        let span = WIDTH + 2 * CLOUD_MARGIN;
        for cloud in &CLOUDS[..self.weather.clouds()] {
            let x = (cloud.phase + self.distance(cloud.speed)) % span - CLOUD_MARGIN;
            let center = Point::new(x, cloud.y);
            let bounds = Rectangle::with_center(center, Size::new(80, 50));
            if !visible(target, bounds) {
                continue;
            }
            draw_cloud(
                target,
                center + Point::new(2, 3),
                cloud.scale,
                convert(shadow),
            )?;
            draw_cloud(target, center, cloud.scale, convert(body))?;
        }
        Ok(())
    }

    /// Draws the rain or snow that falls, if any.
    fn draw_precipitation<D: DrawTarget<Color = Rgb565>>(
        &self,
        target: &mut D,
    ) -> Result<(), D::Error> {
        let Some(precipitation) = self.weather.precipitation() else {
            return Ok(());
        };
        let fall = SKY_BOTTOM - FALL_TOP;
        match precipitation {
            Precipitation::Rain => {
                let style = PrimitiveStyle::with_stroke(convert(RAIN), 1);
                for index in 0..RAIN_DROPS {
                    let speed = 170 + hash(index, 1) % 60;
                    let y = FALL_TOP + (hash(index, 2) as i32 + self.distance(speed)) % fall;
                    let x = (hash(index, 3) as i32 + y / 3) % (WIDTH + 20) - 10;
                    let drop = Line::new(Point::new(x, y), Point::new(x - 2, y - 7));
                    if visible(target, drop.bounding_box()) {
                        drop.into_styled(style).draw(target)?;
                    }
                }
            }
            Precipitation::Snow => {
                for index in 0..SNOW_FLAKES {
                    let speed = 18 + hash(index, 1) % 16;
                    let y = FALL_TOP + (hash(index, 2) as i32 + self.distance(speed)) % fall;
                    let phase = self.seconds() * 1.3 + index as f32;
                    let x = (hash(index, 3) % WIDTH as u32) as i32 + (4.0 * phase.sin()) as i32;
                    let size = 2 + hash(index, 4) % 2;
                    let flake = Rectangle::new(Point::new(x, y), Size::new(size, size));
                    target.fill_solid(&flake, convert(SNOW))?;
                }
            }
        }
        Ok(())
    }

    /// Draws the temperature and the description of the weather.
    fn draw_headline<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        if !visible(target, HEADLINE_BOUNDS) {
            return Ok(());
        }
        let color = if self.stale { MUTED } else { TEXT };
        match self.weather.temperature_celsius {
            Some(temperature) => {
                let mut value = String::<12>::new();
                // Values that do not fit are cut off.
                let _ = write!(value, "{temperature:.1}");
                shadowed(target, &value, HEADLINE_LEFT, 10, &PROFONT_24_POINT, color)?;
                let unit_left = HEADLINE_LEFT + width(&value, &PROFONT_24_POINT) + 2;
                shadowed(target, "°C", unit_left, 14, &PROFONT_14_POINT, color)?;
            }
            None => {
                let placeholder = Rectangle::new(Point::new(HEADLINE_LEFT, 24), Size::new(28, 4));
                target.fill_solid(&placeholder, MUTED)?;
            }
        }
        let mut description = String::<32>::new();
        // Descriptions are shorter than the capacity.
        let _ = write!(description, "{}", self.weather.description());
        shadowed(
            target,
            &description,
            HEADLINE_LEFT,
            44,
            &PROFONT_12_POINT,
            color,
        )
    }

    /// Draws the bar with the humidity, the UV index, and the air pressure.
    fn draw_bar<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        let bar = Rectangle::new(
            Point::new(0, SKY_BOTTOM),
            Size::new(WIDTH as u32, (HEIGHT - SKY_BOTTOM) as u32),
        );
        if !visible(target, bar) {
            return Ok(());
        }
        target.fill_solid(&bar, CARD)?;
        let weather = &self.weather;
        let columns = [
            ("HUMIDITY", weather.humidity_percent, 0, " %"),
            ("UV INDEX", weather.uv_index, 1, ""),
            ("PRESSURE", weather.pressure_hectopascals, 0, " hPa"),
        ];
        for (index, (caption, value, precision, unit)) in columns.into_iter().enumerate() {
            let left = BAR_LEFT + index as i32 * BAR_PITCH;
            text(
                target,
                caption,
                left,
                SKY_BOTTOM + 8,
                &PROFONT_7_POINT,
                MUTED,
                Alignment::Left,
            )?;
            let top = SKY_BOTTOM + 21;
            let Some(value) = value else {
                let placeholder = Rectangle::new(Point::new(left, top + 7), Size::new(16, 2));
                target.fill_solid(&placeholder, MUTED)?;
                continue;
            };
            let mut formatted = String::<16>::new();
            // Values that do not fit are cut off.
            let _ = write!(formatted, "{value:.precision$}{unit}");
            let color = if self.stale { MUTED } else { TEXT };
            text(
                target,
                &formatted,
                left,
                top,
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
            let right = BAR_LEFT + 2 * BAR_PITCH + BAR_PITCH - 12;
            let font = &PROFONT_7_POINT;
            text(
                target,
                label,
                right,
                SKY_BOTTOM + 8,
                font,
                color,
                Alignment::Right,
            )?;
        }
        Ok(())
    }
}

impl Drawable for Scene {
    type Color = Rgb565;
    type Output = ();

    fn draw<D: DrawTarget<Color = Rgb565>>(&self, target: &mut D) -> Result<(), D::Error> {
        self.draw_sky(target)?;
        self.draw_sun(target)?;
        self.draw_precipitation(target)?;
        self.draw_clouds(target)?;
        self.draw_headline(target)?;
        self.draw_bar(target)
    }
}

/// Cloud that drifts across the sky.
struct Cloud {
    /// Row of the center.
    y: i32,
    /// Size relative to the template of [`draw_cloud`].
    scale: f32,
    /// Speed in pixels per second.
    speed: u32,
    /// Horizontal position at the start of the animation.
    phase: i32,
}

/// Clouds in the order in which they appear with growing humidity.
const CLOUDS: [Cloud; 5] = [
    Cloud {
        y: 84,
        scale: 1.0,
        speed: 9,
        phase: 120,
    },
    Cloud {
        y: 128,
        scale: 0.8,
        speed: 6,
        phase: 300,
    },
    Cloud {
        y: 104,
        scale: 1.15,
        speed: 7,
        phase: 0,
    },
    Cloud {
        y: 150,
        scale: 0.9,
        speed: 11,
        phase: 210,
    },
    Cloud {
        y: 72,
        scale: 0.7,
        speed: 13,
        phase: 360,
    },
];

/// Draws a cloud of `scale` centered at `center`: overlapping circles on a
/// rounded base.
fn draw_cloud<D: DrawTarget<Color = Rgb565>>(
    target: &mut D,
    center: Point,
    scale: f32,
    color: Rgb565,
) -> Result<(), D::Error> {
    let style = PrimitiveStyle::with_fill(color);
    let scaled = |value: i32| (value as f32 * scale) as i32;
    for (x, y, radius) in [(-20, 2, 13), (-5, -7, 17), (12, -3, 14), (25, 4, 10)] {
        let circle_center = center + Point::new(scaled(x), scaled(y));
        Circle::with_center(circle_center, 2 * scaled(radius) as u32)
            .into_styled(style)
            .draw(target)?;
    }
    let base = Rectangle::new(
        center + Point::new(scaled(-30), scaled(2)),
        Size::new(scaled(64) as u32, scaled(14) as u32),
    );
    RoundedRectangle::with_equal_corners(base, Size::new(7, 7))
        .into_styled(style)
        .draw(target)
}

/// Draws `content` with a shadow, so that it is readable on a light sky.
fn shadowed<D: DrawTarget<Color = Rgb565>>(
    target: &mut D,
    content: &str,
    x: i32,
    y: i32,
    font: &MonoFont<'_>,
    color: Rgb565,
) -> Result<(), D::Error> {
    text(target, content, x + 1, y + 1, font, SHADOW, Alignment::Left)?;
    text(target, content, x, y, font, color, Alignment::Left)
}

/// Indicates whether `bounds` overlap the area that `target` holds.
fn visible<D: DrawTarget<Color = Rgb565>>(target: &D, bounds: Rectangle) -> bool {
    !target.bounding_box().intersection(&bounds).is_zero_sized()
}

/// Returns a pseudo-random number for the `index`-th element and the
/// `salt`-th property, so that elements are spread evenly but irregularly.
fn hash(index: u32, salt: u32) -> u32 {
    let mut value = index.wrapping_mul(0x9E37_79B9) ^ salt.wrapping_mul(0x85EB_CA6B);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7FEB_352D);
    value ^= value >> 15;
    value % 1000
}

/// Channel of a color between two values of the display, which an ordered
/// dither mixes.
#[derive(Clone, Copy)]
struct Dithered {
    /// Lower value of the display.
    base: u8,
    /// Distance to the lower value in sixteenths of a step.
    fraction: u8,
    /// Highest value of the display.
    max: u8,
}

impl Dithered {
    /// Approximates the 8-bit `value` with `bits` bits.
    fn new(value: u8, bits: u32) -> Self {
        let max = (1u32 << bits) - 1;
        let scaled = u32::from(value) * max * 16 / 255;
        Self {
            base: (scaled >> 4) as u8,
            fraction: (scaled & 15) as u8,
            max: max as u8,
        }
    }

    /// Returns the value of the display at a pixel with `threshold` from
    /// [`BAYER`].
    fn at(self, threshold: u8) -> u8 {
        (self.base + u8::from(self.fraction > threshold)).min(self.max)
    }
}

/// Thresholds of a 4 × 4 ordered dither, row by row.
const BAYER: [u8; 16] = [0, 8, 2, 10, 12, 4, 14, 6, 3, 11, 1, 9, 15, 7, 13, 5];

/// Adds `amount` to each channel of `color`.
fn brighten(color: Rgb888, amount: u8) -> Rgb888 {
    Rgb888::new(
        color.r().saturating_add(amount),
        color.g().saturating_add(amount),
        color.b().saturating_add(amount),
    )
}

/// Mixes `fraction` of `color` with the rest of `other`.
fn mix(color: Rgb888, other: Rgb888, fraction: f32) -> Rgb888 {
    let channel = |a: u8, b: u8| (f32::from(a) * fraction + f32::from(b) * (1.0 - fraction)) as u8;
    Rgb888::new(
        channel(color.r(), other.r()),
        channel(color.g(), other.g()),
        channel(color.b(), other.b()),
    )
}

/// Converts `color` to the colors of the display.
fn convert(color: Rgb888) -> Rgb565 {
    color.into()
}

/// Width of the screen.
const WIDTH: i32 = 320;

/// Height of the screen.
const HEIGHT: i32 = 240;

/// Bottom edge of the sky, where the bar starts.
const SKY_BOTTOM: i32 = 196;

/// Top edge of the area in which rain and snow fall.
const FALL_TOP: i32 = 70;

/// Number of rain drops.
const RAIN_DROPS: u32 = 56;

/// Number of snow flakes.
const SNOW_FLAKES: u32 = 60;

/// Distance outside of the screen at which clouds turn around.
const CLOUD_MARGIN: i32 = 50;

/// Center of the sun.
const SUN_CENTER: Point = Point::new(262, 48);

/// Area of the sun and its rays.
const SUN_BOUNDS: Rectangle = Rectangle::new(Point::new(262 - 48, 0), Size::new(96, 96));

/// Number of rays of the sun.
const RAYS_COUNT: u32 = 12;

/// Left edge of the temperature and the description.
const HEADLINE_LEFT: i32 = 14;

/// Area of the temperature and the description.
const HEADLINE_BOUNDS: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(200, 64));

/// Left edge of the first column of the bar.
const BAR_LEFT: i32 = 12;

/// Distance between the columns of the bar.
const BAR_PITCH: i32 = 104;

/// Top of a clear sky.
const CLEAR_TOP: Rgb888 = Rgb888::new(34, 98, 190);

/// Bottom of a clear sky.
const CLEAR_BOTTOM: Rgb888 = Rgb888::new(146, 194, 238);

/// Top of a dim sky.
const DIM_TOP: Rgb888 = Rgb888::new(46, 56, 76);

/// Bottom of a dim sky.
const DIM_BOTTOM: Rgb888 = Rgb888::new(112, 124, 144);

/// Tint of a cold sky.
const ICY: Rgb888 = Rgb888::new(176, 226, 255);

/// Tint of a hot sky.
const WARM: Rgb888 = Rgb888::new(255, 184, 112);

/// Color of the sun.
const SUN: Rgb888 = Rgb888::new(255, 208, 84);

/// Color of the rays of the sun.
const RAYS: Rgb888 = Rgb888::new(255, 220, 120);

/// Color of a cloud.
const CLOUD: Rgb888 = Rgb888::new(244, 246, 250);

/// Color of the shadow of a cloud.
const CLOUD_SHADOW: Rgb888 = Rgb888::new(190, 200, 216);

/// Color of a dark cloud.
const DARK_CLOUD: Rgb888 = Rgb888::new(148, 156, 170);

/// Color of the shadow of a dark cloud.
const DARK_CLOUD_SHADOW: Rgb888 = Rgb888::new(104, 112, 128);

/// Color of rain.
const RAIN: Rgb888 = Rgb888::new(176, 208, 246);

/// Color of snow.
const SNOW: Rgb888 = Rgb888::new(248, 250, 255);

/// Color of the shadow of text.
const SHADOW: Rgb565 = rgb(16, 24, 40);
