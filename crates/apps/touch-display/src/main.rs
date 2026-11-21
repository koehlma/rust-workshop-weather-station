//! Firmware of a Raspberry Pi Pico W on a Waveshare Pico-ResTouch-LCD-2.8.
//!
//! The firmware shows a placeholder text on the [`display`] and connects the
//! device to Nexigon Hub with its own identity in `identity/`. It publishes a
//! heartbeat every 30 seconds, runs commands, and installs updates through
//! `pico_platform::nexigon`. Logs go to the USB serial port.

#![no_std]
#![no_main]

mod display;

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb;
use embassy_rp::watchdog::Watchdog;
use embassy_time::Timer;
use log::info;
use panic_reset as _;
use pico_platform::logger;
use pico_platform::nexigon;
use pico_platform::nexigon::Config;
use pico_platform::watchdog;

use crate::display::Display;

/// Configuration and identity of the device.
static CONFIG: Config = pico_platform::nexigon_config!();

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // The bootloader has started the watchdog, which must be fed from now on.
    spawner.spawn(watchdog::task(Watchdog::new(p.WATCHDOG)).unwrap());
    spawner.spawn(logger::task(usb::Driver::new(p.USB, Irqs)).unwrap());
    // Give the host time to open the USB serial port, so that it receives
    // the first messages.
    Timer::after_secs(3).await;
    info!("starting firmware {}", CONFIG.firmware_version);

    let mut display = Display::new(
        p.SPI1,
        display::Pins {
            clock: p.PIN_10,
            data: p.PIN_11,
            chip_select: p.PIN_9,
            data_command: p.PIN_8,
            reset: p.PIN_15,
            backlight: p.PIN_13,
            touch_chip_select: p.PIN_16,
            sd_chip_select: p.PIN_22,
        },
    );
    display.show_placeholder(CONFIG.firmware_version);

    nexigon::run(
        spawner,
        &CONFIG,
        nexigon::Peripherals {
            flash: p.FLASH,
            wifi: nexigon::WifiPeripherals {
                power: p.PIN_23,
                data: p.PIN_24,
                chip_select: p.PIN_25,
                clock: p.PIN_29,
                pio: p.PIO0,
                dma: p.DMA_CH0,
            },
        },
        (),
    )
    .await
}
