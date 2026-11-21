//! Display of the Waveshare Pico-ResTouch-LCD-2.8.
//!
//! The display is an ST7789 with 240 × 320 pixels on the shared [`Bus`].
//! [`Display::new`] starts it in landscape, as in the example code of
//! Waveshare: RGB, without color inversion, and mirrored horizontally with
//! rows and columns swapped. [`Display::show`] draws a [`Screen`].

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
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use mipidsi::Builder;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7789;
use mipidsi::options::ColorInversion;
use mipidsi::options::ColorOrder;
use mipidsi::options::Orientation;
use mipidsi::options::Rotation;
use static_cell::StaticCell;

use crate::Bus;
use crate::screen;
use crate::screen::Screen;

/// Clock frequency of the display. Waveshare tested writes at up to 60 MHz.
const SPI_FREQUENCY: u32 = 32_000_000;

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
    /// Starts the display and clears it.
    pub fn new(bus: &'static Bus, pins: Pins) -> Self {
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
        Self {
            display,
            _backlight: backlight,
        }
    }

    /// Draws `screen`, replacing what the display showed before.
    pub fn show(&mut self, screen: &Screen) {
        screen::draw(&mut self.display, screen)
            .expect("the SPI bus and the pins of the display do not fail");
    }
}
