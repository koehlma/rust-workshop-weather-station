//! Display of the Waveshare Pico-ResTouch-LCD-2.8.
//!
//! The display is an ST7789 with 240 × 320 pixels on the shared [`Bus`].
//! [`Display::new`] starts it in landscape, as in the example code of
//! Waveshare: RGB, without color inversion, and mirrored horizontally with
//! rows and columns swapped. It draws a title and the version of the firmware.
//! [`Display::show`] draws lines of text between them.

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::Peri;
use embassy_rp::gpio::Level;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::PIN_8;
use embassy_rp::peripherals::PIN_9;
use embassy_rp::peripherals::PIN_13;
use embassy_rp::peripherals::PIN_15;
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi;
use embassy_rp::spi::Spi;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Delay;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::iso_8859_1::FONT_6X10;
use embedded_graphics::mono_font::iso_8859_1::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::Baseline;
use embedded_graphics::text::Text;
use mipidsi::Builder;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7789;
use mipidsi::options::ColorInversion;
use mipidsi::options::ColorOrder;
use mipidsi::options::Orientation;
use mipidsi::options::Rotation;
use static_cell::StaticCell;

use crate::Bus;

/// Clock frequency of the display. Waveshare tested writes at up to 60 MHz.
const SPI_FREQUENCY: u32 = 32_000_000;

/// Distance of the text from the edges.
const MARGIN: i32 = 12;

/// Top of the lines of [`Display::show`].
const LINES_TOP: i32 = 52;

/// Height of a line of [`Display::show`].
const LINE_HEIGHT: i32 = 28;

/// Area of the lines of [`Display::show`], which is cleared before drawing.
const LINES_AREA: Rectangle = Rectangle::new(Point::new(0, LINES_TOP), Size::new(320, 168));

type Interface = SpiInterface<
    'static,
    SpiDeviceWithConfig<'static, NoopRawMutex, Spi<'static, SPI1, spi::Blocking>, Output<'static>>,
    Output<'static>,
>;

/// Pins of the display, besides the bus.
pub struct Pins {
    /// Chip select, GP9.
    pub chip_select: Peri<'static, PIN_9>,
    /// Data or command select, GP8.
    pub data_command: Peri<'static, PIN_8>,
    /// Reset, GP15.
    pub reset: Peri<'static, PIN_15>,
    /// Backlight, GP13.
    pub backlight: Peri<'static, PIN_13>,
}

/// Display of the board.
pub struct Display {
    display: mipidsi::Display<Interface, ST7789, Output<'static>>,
    /// Keeps the backlight on.
    _backlight: Output<'static>,
}

impl Display {
    /// Starts the display and draws the title and the version of the firmware.
    pub fn new(bus: &'static Bus, pins: Pins, firmware_version: &str) -> Self {
        let backlight = Output::new(pins.backlight, Level::High);
        let mut config = spi::Config::default();
        config.frequency = SPI_FREQUENCY;
        let device =
            SpiDeviceWithConfig::new(bus, Output::new(pins.chip_select, Level::High), config);
        static BUFFER: StaticCell<[u8; 512]> = StaticCell::new();
        let interface = SpiInterface::new(
            device,
            Output::new(pins.data_command, Level::Low),
            BUFFER.init([0; 512]),
        );
        let mut display = Builder::new(ST7789, interface)
            .display_size(240, 320)
            .orientation(Orientation::new().rotate(Rotation::Deg90))
            .color_order(ColorOrder::Rgb)
            .invert_colors(ColorInversion::Normal)
            .reset_pin(Output::new(pins.reset, Level::High))
            .init(&mut Delay)
            .expect("the SPI bus and the pins of the display do not fail");
        display
            .clear(Rgb565::BLACK)
            .expect("the SPI bus and the pins of the display do not fail");
        let title = MonoTextStyle::new(&FONT_10X20, Rgb565::CYAN);
        Text::with_baseline(
            "Weather Station",
            Point::new(MARGIN, MARGIN),
            title,
            Baseline::Top,
        )
        .draw(&mut display)
        .expect("the SPI bus and the pins of the display do not fail");
        let caption = MonoTextStyle::new(&FONT_6X10, Rgb565::CSS_GRAY);
        Text::with_baseline(
            firmware_version,
            Point::new(MARGIN, 240 - MARGIN),
            caption,
            Baseline::Bottom,
        )
        .draw(&mut display)
        .expect("the SPI bus and the pins of the display do not fail");
        Self {
            display,
            _backlight: backlight,
        }
    }

    /// Replaces the lines between the title and the version of the firmware.
    /// Six lines of 30 characters fit.
    pub fn show(&mut self, lines: &[&str]) {
        LINES_AREA
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(&mut self.display)
            .expect("the SPI bus and the pins of the display do not fail");
        let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
        for (index, line) in lines.iter().enumerate() {
            let top = LINES_TOP + index as i32 * LINE_HEIGHT;
            Text::with_baseline(line, Point::new(MARGIN, top), style, Baseline::Top)
                .draw(&mut self.display)
                .expect("the SPI bus and the pins of the display do not fail");
        }
    }
}
