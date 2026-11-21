//! Logs over the USB serial port.
//!
//! [`task`] sends the messages of the `log` crate to the host, for instance,
//! to `/dev/ttyACM0` on Linux.

use embassy_rp::peripherals::USB;
use embassy_rp::usb;

/// Sends the logs to the USB serial port of `driver`.
#[embassy_executor::task]
pub async fn task(driver: usb::Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}
