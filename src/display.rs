//! Display of the measurements on the 1.8" TFT of the Joy-IT explorer board.
//!
//! The TFT is an ST7735 with 128 × 160 pixels on SPI0. [`Display::show`]
//! draws the temperature, humidity, and UV index of a measurement, with dashes
//! for sensors that failed. The configuration follows the example code of
//! Joy-IT: 8 MHz, mirrored on both axes, and addressed from (0, 0).

use core::fmt;
use core::fmt::Write as _;

use embassy_rp::Peri;
use embassy_rp::gpio::Level;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::PIN_2;
use embassy_rp::peripherals::PIN_3;
use embassy_rp::peripherals::PIN_6;
use embassy_rp::peripherals::PIN_17;
use embassy_rp::peripherals::PIN_18;
use embassy_rp::peripherals::PIN_19;
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi;
use embassy_rp::spi::Spi;
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
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_hal_bus::spi::NoDelay;
use mipidsi::Builder;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7735s;
use mipidsi::options::ColorInversion;
use mipidsi::options::ColorOrder;
use mipidsi::options::Orientation;
use mipidsi::options::Rotation;
use static_cell::StaticCell;

use crate::sensors::Measurement;

/// Clock frequency of the SPI bus, as in the example code of Joy-IT.
const SPI_FREQUENCY: u32 = 8_000_000;

/// Distance of the text from the left edge.
const MARGIN: i32 = 4;

/// Labels of the values, from top to bottom.
const LABELS: [&str; 3] = ["Temperature", "Humidity", "UV index"];

/// Height of a label and its value.
const ROW_HEIGHT: i32 = 52;

/// Distance of a value below its label.
const VALUE_OFFSET: i32 = 12;

/// Area of a value, which is cleared before drawing a new one.
const VALUE_SIZE: Size = Size::new(120, 20);

type Interface = SpiInterface<
    'static,
    ExclusiveDevice<Spi<'static, SPI0, spi::Blocking>, Output<'static>, NoDelay>,
    Output<'static>,
>;

/// TFT of the explorer board.
pub struct Display {
    display: mipidsi::Display<Interface, ST7735s, Output<'static>>,
    /// Keeps the backlight on.
    _backlight: Output<'static>,
}

impl Display {
    /// Starts the display and draws the labels.
    pub fn new(
        spi: Peri<'static, SPI0>,
        clock: Peri<'static, PIN_18>,
        data: Peri<'static, PIN_19>,
        chip_select: Peri<'static, PIN_17>,
        data_command: Peri<'static, PIN_3>,
        reset: Peri<'static, PIN_6>,
        backlight: Peri<'static, PIN_2>,
    ) -> Self {
        let mut config = spi::Config::default();
        config.frequency = SPI_FREQUENCY;
        let bus = Spi::new_blocking_txonly(spi, clock, data, config);
        let Ok(device) = ExclusiveDevice::new_no_delay(bus, Output::new(chip_select, Level::High));
        static BUFFER: StaticCell<[u8; 512]> = StaticCell::new();
        let interface = SpiInterface::new(
            device,
            Output::new(data_command, Level::Low),
            BUFFER.init([0; 512]),
        );
        let mut display = Builder::new(ST7735s, interface)
            .display_size(128, 160)
            // Mirroring both axes maps this offset to (0, 0) of the controller.
            .display_offset(4, 2)
            .orientation(Orientation::new().rotate(Rotation::Deg180))
            .color_order(ColorOrder::Rgb)
            .invert_colors(ColorInversion::Normal)
            .reset_pin(Output::new(reset, Level::High))
            .init(&mut Delay)
            .expect("the SPI bus and the pins of the display do not fail");
        display
            .clear(Rgb565::BLACK)
            .expect("the SPI bus and the pins of the display do not fail");
        let style = MonoTextStyle::new(&FONT_6X10, Rgb565::CYAN);
        for (row, label) in LABELS.iter().enumerate() {
            Text::with_baseline(
                label,
                Point::new(MARGIN, row_top(row)),
                style,
                Baseline::Top,
            )
            .draw(&mut display)
            .expect("the SPI bus and the pins of the display do not fail");
        }
        Self {
            display,
            _backlight: Output::new(backlight, Level::High),
        }
    }

    /// Shows a measurement.
    pub fn show(&mut self, measurement: &Measurement) {
        match &measurement.dht11 {
            Some(reading) => {
                self.draw_value(0, format_args!("{:.1} °C", reading.temperature_celsius));
                self.draw_value(1, format_args!("{:.0} %", reading.humidity_percent));
            }
            None => {
                self.draw_value(0, format_args!("--"));
                self.draw_value(1, format_args!("--"));
            }
        }
        match &measurement.uv {
            Some(reading) => self.draw_value(2, format_args!("{:.1}", reading.uv_index)),
            None => self.draw_value(2, format_args!("--")),
        }
    }

    /// Replaces the value in `row`.
    fn draw_value(&mut self, row: usize, value: fmt::Arguments<'_>) {
        let mut text = heapless::String::<16>::new();
        write!(text, "{value}").expect("values fit into 16 bytes");
        let top_left = Point::new(MARGIN, row_top(row) + VALUE_OFFSET);
        Rectangle::new(top_left, VALUE_SIZE)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(&mut self.display)
            .expect("the SPI bus and the pins of the display do not fail");
        let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
        Text::with_baseline(&text, top_left, style, Baseline::Top)
            .draw(&mut self.display)
            .expect("the SPI bus and the pins of the display do not fail");
    }
}

/// Returns the top of the label in `row`.
fn row_top(row: usize) -> i32 {
    MARGIN + ROW_HEIGHT * row as i32
}
