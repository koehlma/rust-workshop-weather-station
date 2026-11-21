//! Taps on the touch screen of the Waveshare Pico-ResTouch-LCD-2.8.
//!
//! The XPT2046 touch controller is on the shared [`Bus`] with its chip select
//! on GP16. It pulls its interrupt line on GP17 low while the screen is
//! touched. [`Touch::tap`] waits for a touch and measures its position. The
//! controller powers down between measurements with its interrupt line
//! enabled. [`task`] measures taps in a task of their own and sends them to
//! [`TAPS`], so that no tap gets lost while the display draws.
//!
//! The calibration is the one of Waveshare's example code for the orientation
//! of the display, `U2D_R2L`. Its accuracy varies between panels, so the
//! targets on the screen are large.

use embassy_rp::Peri;
use embassy_rp::gpio::Input;
use embassy_rp::gpio::Output;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::PIN_17;
use embassy_rp::spi;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Duration;
use embassy_time::Timer;
use embedded_graphics::prelude::Point;
use log::warn;

use crate::Bus;
use crate::strip::SCREEN_HEIGHT;
use crate::strip::SCREEN_WIDTH;

/// Clock frequency of the touch controller, which accepts at most 2 MHz.
const SPI_FREQUENCY: u32 = 1_000_000;

/// Command that measures the X position with 12 bits.
const MEASURE_X: u8 = 0xD0;

/// Command that measures the Y position with 12 bits.
const MEASURE_Y: u8 = 0x90;

/// Number of measurements of a position, whose median counts.
const SAMPLES: usize = 5;

/// Range of plausible measurements. Measurements outside of it occur when the
/// screen is released during a measurement.
const PLAUSIBLE: core::ops::RangeInclusive<u16> = 100..=4000;

/// Time for which a touch must last to count as a tap.
const DEBOUNCE: Duration = Duration::from_millis(15);

/// Positions of taps that wait to be handled.
pub static TAPS: Channel<CriticalSectionRawMutex, Point, 4> = Channel::new();

/// Measures taps on `touch` and sends their positions to [`TAPS`].
#[embassy_executor::task]
pub async fn task(mut touch: Touch) -> ! {
    loop {
        let point = touch.tap().await;
        if TAPS.try_send(point).is_err() {
            warn!("dropped a tap, because earlier taps wait to be handled");
        }
    }
}

/// Touch screen of the board.
pub struct Touch {
    bus: &'static Bus,
    chip_select: Output<'static>,
    interrupt: Input<'static>,
}

impl Touch {
    /// Takes the touch screen, whose chip select must be high.
    pub fn new(
        bus: &'static Bus,
        chip_select: Output<'static>,
        interrupt: Peri<'static, PIN_17>,
    ) -> Self {
        let mut touch = Self {
            bus,
            chip_select,
            interrupt: Input::new(interrupt, Pull::Up),
        };
        // The interrupt line is enabled after the first measurement.
        touch.measure(MEASURE_Y);
        touch
    }

    /// Waits until the screen is released and touched again, and returns the
    /// position of the touch.
    pub async fn tap(&mut self) -> Point {
        loop {
            self.interrupt.wait_for_high().await;
            self.interrupt.wait_for_low().await;
            Timer::after(DEBOUNCE).await;
            if self.interrupt.is_high() {
                continue;
            }
            if let Some(point) = self.position() {
                return point;
            }
        }
    }

    /// Measures the position of the touch, or returns `None` if the
    /// measurements are implausible.
    fn position(&mut self) -> Option<Point> {
        let x = self.median(MEASURE_X)?;
        let y = self.median(MEASURE_Y)?;
        // Rows and columns of the display are swapped relative to the touch
        // screen.
        let column = 350.0 - 0.089616 * f32::from(y);
        let row = 0.063399 * f32::from(x) - 5.0;
        Some(Point::new(
            (column as i32).clamp(0, SCREEN_WIDTH as i32 - 1),
            (row as i32).clamp(0, SCREEN_HEIGHT as i32 - 1),
        ))
    }

    /// Returns the median of [`SAMPLES`] measurements with `command`, or
    /// `None` if it is implausible.
    fn median(&mut self, command: u8) -> Option<u16> {
        let mut samples = [0; SAMPLES];
        for sample in &mut samples {
            *sample = self.measure(command);
        }
        samples.sort_unstable();
        let median = samples[SAMPLES / 2];
        PLAUSIBLE.contains(&median).then_some(median)
    }

    /// Runs one measurement with `command` and returns its 12 bits.
    fn measure(&mut self, command: u8) -> u16 {
        self.bus.lock(|bus| {
            let mut bus = bus.borrow_mut();
            let mut config = spi::Config::default();
            config.frequency = SPI_FREQUENCY;
            // Other devices on the bus set their own frequency for each
            // transfer.
            bus.set_config(&config);
            self.chip_select.set_low();
            let mut response = [0; 3];
            bus.blocking_transfer(&mut response, &[command, 0, 0])
                .expect("blocking transfers do not fail");
            self.chip_select.set_high();
            (u16::from(response[1]) << 8 | u16::from(response[2])) >> 3
        })
    }
}
