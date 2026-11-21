//! Firmware of a weather station built from a Raspberry Pi Pico W on a Joy-IT
//! explorer board, with its DHT11 sensor and display, a SEN-UV01 UV sensor,
//! and a BMP280 pressure sensor.
//!
//! Every 30 seconds, the [`station`] measures the temperature, humidity, UV
//! radiation, and air pressure and shows them on the display. With the
//! `nexigon` feature, [`nexigon`] connects the device to Nexigon Hub, which
//! receives the measurements as events, runs commands, and rolls out updates.
//! Without it, [`standalone`] logs the measurements. Logs go to the USB serial
//! port.

#![no_std]
#![no_main]

mod bmp280;
mod dht11;
mod display;
#[cfg(feature = "nexigon")]
mod nexigon;
mod sensors;
#[cfg(not(feature = "nexigon"))]
mod standalone;
mod station;
mod uv;

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c;
use embassy_rp::i2c::I2c;
use embassy_rp::peripherals::USB;
use embassy_rp::usb;
use embassy_rp::watchdog::Watchdog;
use embassy_time::Timer;
use log::info;
use panic_reset as _;
use pico_platform::logger;
use pico_platform::watchdog;

use crate::bmp280::Bmp280;
use crate::dht11::Dht11;
use crate::display::Display;
use crate::sensors::Sensors;
use crate::station::Station;
use crate::uv::UvSensor;

/// Version of the firmware, by default the time of the build, for instance,
/// `20261120-163000`.
const FIRMWARE_VERSION: &str = env!("FIRMWARE_VERSION");

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
    info!("starting firmware {FIRMWARE_VERSION}");

    let sensors = Sensors::new(
        // The Joy-IT explorer board connects its DHT11 sensor to GP0.
        Dht11::new(p.PIN_0),
        // GP26 is the first analog input, which the explorer board leaves free.
        UvSensor::new(p.ADC, p.PIN_26),
        // GP4 and GP5 are SDA and SCL of the I2C connector of the explorer board.
        Bmp280::new(I2c::new_blocking(
            p.I2C0,
            p.PIN_5,
            p.PIN_4,
            i2c::Config::default(),
        )),
    );
    // The pins of the display on the explorer board.
    let display = Display::new(
        p.SPI0, p.PIN_18, p.PIN_19, p.PIN_17, p.PIN_3, p.PIN_6, p.PIN_2,
    );
    let station = Station::new(sensors, display);

    #[cfg(feature = "nexigon")]
    nexigon::run(
        spawner,
        pico_platform::nexigon::Peripherals {
            flash: p.FLASH,
            wifi: pico_platform::nexigon::WifiPeripherals {
                power: p.PIN_23,
                data: p.PIN_24,
                chip_select: p.PIN_25,
                clock: p.PIN_29,
                pio: p.PIO0,
                dma: p.DMA_CH0,
            },
        },
        station,
    )
    .await;

    #[cfg(not(feature = "nexigon"))]
    standalone::run(station).await;
}
