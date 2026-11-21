//! Taps on the touch screen of the Waveshare Pico-ResTouch-LCD-2.8.
//!
//! The XPT2046 touch controller is on the shared [`Bus`] with its chip select
//! on GP16. It pulls its interrupt line on GP17 low while the screen is
//! touched. [`Touch::new`] enables the interrupt line with one command, and
//! [`Touch::tap`] waits for it. The firmware does not read the position of a
//! touch.

use embassy_rp::Peri;
use embassy_rp::gpio::Input;
use embassy_rp::gpio::Output;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::PIN_17;
use embassy_rp::spi;
use embassy_time::Duration;
use embassy_time::Timer;

use crate::Bus;

/// Clock frequency of the touch controller, which accepts at most 2 MHz.
const SPI_FREQUENCY: u32 = 1_000_000;

/// Command that measures the Y position and then powers the controller down
/// with its interrupt line enabled.
const ENABLE_INTERRUPT: [u8; 3] = [0x90, 0x00, 0x00];

/// Time for which a touch must last to count as a tap.
const DEBOUNCE: Duration = Duration::from_millis(30);

/// Touch screen of the board.
pub struct Touch {
    interrupt: Input<'static>,
    /// Keeps the touch controller deselected between commands.
    _chip_select: Output<'static>,
}

impl Touch {
    /// Enables the interrupt line of the touch controller.
    pub fn new(
        bus: &'static Bus,
        mut chip_select: Output<'static>,
        interrupt: Peri<'static, PIN_17>,
    ) -> Self {
        bus.lock(|bus| {
            let mut bus = bus.borrow_mut();
            let mut config = spi::Config::default();
            config.frequency = SPI_FREQUENCY;
            // Other devices on the bus set their own frequency for each
            // transfer.
            bus.set_config(&config);
            chip_select.set_low();
            bus.blocking_write(&ENABLE_INTERRUPT)
                .expect("blocking writes do not fail");
            chip_select.set_high();
        });
        Self {
            interrupt: Input::new(interrupt, Pull::Up),
            _chip_select: chip_select,
        }
    }

    /// Waits until the screen is released and touched again.
    pub async fn tap(&mut self) {
        loop {
            self.interrupt.wait_for_high().await;
            self.interrupt.wait_for_low().await;
            Timer::after(DEBOUNCE).await;
            if self.interrupt.is_low() {
                return;
            }
        }
    }
}
