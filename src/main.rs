//! Firmware of a weather station built from a Raspberry Pi Pico W, a DHT11
//! sensor, and a SEN-UV01 UV sensor.
//!
//! Every 30 seconds, the firmware measures the temperature, humidity, and UV
//! radiation with [`sensors`]. With the `nexigon` feature, [`nexigon`]
//! connects the device to Nexigon Hub, which receives the measurements as
//! events, runs commands, and rolls out updates. Without it, [`standalone`]
//! logs the measurements. Logs go to the USB serial port.

#![no_std]
#![no_main]

mod dht11;
#[cfg(feature = "nexigon")]
mod nexigon;
mod sensors;
#[cfg(not(feature = "nexigon"))]
mod standalone;
mod uv;
mod watchdog;

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb;
use embassy_rp::watchdog::Watchdog;
use embassy_time::Timer;
use log::info;
use panic_reset as _;

use crate::dht11::Dht11;
use crate::sensors::Sensors;
use crate::uv::UvSensor;

/// Version of the firmware, by default the time of the build, for instance,
/// `20261120-163000`.
pub const FIRMWARE_VERSION: &str = env!("FIRMWARE_VERSION");

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // The bootloader has started the watchdog, which must be fed from now on.
    spawner.spawn(watchdog::task(Watchdog::new(p.WATCHDOG)).unwrap());
    spawner.spawn(logger_task(usb::Driver::new(p.USB, Irqs)).unwrap());
    // Give the host time to open the USB serial port, so that it receives
    // the first messages.
    Timer::after_secs(3).await;
    info!("starting firmware {FIRMWARE_VERSION}");

    let sensors = Sensors::new(
        // The Joy-IT explorer board connects its DHT11 sensor to GP0.
        Dht11::new(p.PIN_0),
        // GP26 is the first analog input, which the explorer board leaves free.
        UvSensor::new(p.ADC, p.PIN_26),
    );

    #[cfg(feature = "nexigon")]
    nexigon::run(
        spawner,
        nexigon::Peripherals {
            flash: p.FLASH,
            wifi_power: p.PIN_23,
            wifi_data: p.PIN_24,
            wifi_chip_select: p.PIN_25,
            wifi_clock: p.PIN_29,
            wifi_pio: p.PIO0,
            wifi_dma: p.DMA_CH0,
        },
        sensors,
    )
    .await;

    #[cfg(not(feature = "nexigon"))]
    standalone::run(sensors).await;
}

#[embassy_executor::task]
async fn logger_task(driver: usb::Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}
