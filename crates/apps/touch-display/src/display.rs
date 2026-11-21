//! Display of the Waveshare Pico-ResTouch-LCD-2.8.
//!
//! The display is an ST7789 with 240 × 320 pixels on SPI1, which it shares
//! with the XPT2046 touch controller and the microSD slot. [`Display::new`]
//! deselects both and starts the display in landscape, as in the example code
//! of Waveshare: RGB, without color inversion, and mirrored horizontally with
//! rows and columns swapped. [`Display::show_placeholder`] draws a placeholder
//! text.

use embassy_rp::Peri;
use embassy_rp::gpio::Level;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::PIN_8;
use embassy_rp::peripherals::PIN_9;
use embassy_rp::peripherals::PIN_10;
use embassy_rp::peripherals::PIN_11;
use embassy_rp::peripherals::PIN_13;
use embassy_rp::peripherals::PIN_15;
use embassy_rp::peripherals::PIN_16;
use embassy_rp::peripherals::PIN_22;
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi;
use embassy_rp::spi::Spi;
use embassy_time::Delay;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::iso_8859_1::FONT_6X10;
use embedded_graphics::mono_font::iso_8859_1::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Alignment;
use embedded_graphics::text::Text;
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_hal_bus::spi::NoDelay;
use mipidsi::Builder;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7789;
use mipidsi::options::ColorInversion;
use mipidsi::options::ColorOrder;
use mipidsi::options::Orientation;
use mipidsi::options::Rotation;
use static_cell::StaticCell;

/// Clock frequency of the SPI bus. Waveshare tested writes at up to 60 MHz.
const SPI_FREQUENCY: u32 = 32_000_000;

/// Center of the display in landscape.
const CENTER: Point = Point::new(160, 120);

type Interface = SpiInterface<
    'static,
    ExclusiveDevice<Spi<'static, SPI1, spi::Blocking>, Output<'static>, NoDelay>,
    Output<'static>,
>;

/// Pins of the display and the devices that share its bus.
pub struct Pins {
    /// SPI clock, GP10.
    pub clock: Peri<'static, PIN_10>,
    /// SPI data to the display, GP11.
    pub data: Peri<'static, PIN_11>,
    /// Chip select of the display, GP9.
    pub chip_select: Peri<'static, PIN_9>,
    /// Data or command select of the display, GP8.
    pub data_command: Peri<'static, PIN_8>,
    /// Reset of the display, GP15.
    pub reset: Peri<'static, PIN_15>,
    /// Backlight of the display, GP13.
    pub backlight: Peri<'static, PIN_13>,
    /// Chip select of the touch controller, GP16.
    pub touch_chip_select: Peri<'static, PIN_16>,
    /// Chip select of the microSD slot, GP22.
    pub sd_chip_select: Peri<'static, PIN_22>,
}

/// Display of the board.
pub struct Display {
    display: mipidsi::Display<Interface, ST7789, Output<'static>>,
    /// Keep the backlight on and the other devices on the bus deselected.
    _outputs: [Output<'static>; 3],
}

impl Display {
    /// Starts the display and clears it.
    pub fn new(spi: Peri<'static, SPI1>, pins: Pins) -> Self {
        let outputs = [
            Output::new(pins.touch_chip_select, Level::High),
            Output::new(pins.sd_chip_select, Level::High),
            Output::new(pins.backlight, Level::High),
        ];
        let mut config = spi::Config::default();
        config.frequency = SPI_FREQUENCY;
        let bus = Spi::new_blocking_txonly(spi, pins.clock, pins.data, config);
        let Ok(device) =
            ExclusiveDevice::new_no_delay(bus, Output::new(pins.chip_select, Level::High));
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
            _outputs: outputs,
        }
    }

    /// Draws a placeholder text with the version of the firmware.
    pub fn show_placeholder(&mut self, firmware_version: &str) {
        let title = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
        Text::with_alignment("Hello from Nexigon!", CENTER, title, Alignment::Center)
            .draw(&mut self.display)
            .expect("the SPI bus and the pins of the display do not fail");
        let caption = MonoTextStyle::new(&FONT_6X10, Rgb565::CYAN);
        Text::with_alignment(
            firmware_version,
            CENTER + Point::new(0, 24),
            caption,
            Alignment::Center,
        )
        .draw(&mut self.display)
        .expect("the SPI bus and the pins of the display do not fail");
    }
}
