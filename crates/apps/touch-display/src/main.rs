//! Firmware of a Raspberry Pi Pico W on a Waveshare Pico-ResTouch-LCD-2.8,
//! which stores the measurements of the weather station on a microSD card.
//!
//! The [`receiver`] receives the measurements of the weather station over
//! UDP and writes them to the microSD card through [`storage`] in the binary
//! [`format`]. The [`display`] shows the latest measurement and a plot of the
//! [`history`] in the layout of [`screen`]. A tap on the [`touch`] screen
//! changes the interval of the plot. The display, the microSD slot, and the
//! touch controller share SPI1.
//!
//! The firmware connects the device to Nexigon Hub with its own identity in
//! `identity/` through `pico_platform::nexigon`. It runs commands, installs
//! updates, and publishes a heartbeat and a storage event every 30 seconds.
//! The device property `touch-display.config` configures the IP address of
//! the weather station:
//!
//! ```json
//! { "sensorAddress": "10.10.2.30" }
//! ```
//!
//! Logs go to the USB serial port.

#![no_std]
#![no_main]

mod display;
mod format;
mod history;
mod receiver;
mod screen;
mod storage;
mod touch;

use core::cell::RefCell;
use core::net::Ipv4Addr;

use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Level;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::SPI1;
use embassy_rp::peripherals::USB;
use embassy_rp::spi;
use embassy_rp::spi::Spi;
use embassy_rp::usb;
use embassy_rp::watchdog::Watchdog;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;
use log::info;
use log::warn;
use panic_reset as _;
use pico_platform::logger;
use pico_platform::nexigon;
use pico_platform::nexigon::Application;
use pico_platform::nexigon::Config;
use pico_platform::nexigon::sidex_embedded::Decode;
use pico_platform::nexigon::sidex_embedded::DecodeError;
use pico_platform::nexigon::sidex_embedded::Encode;
use pico_platform::nexigon::sidex_embedded::EncodeError;
use pico_platform::nexigon::sidex_embedded::json::Reader;
use pico_platform::nexigon::sidex_embedded::json::Writer;
use pico_platform::nexigon::sidex_embedded::types::Json;
use pico_platform::nexigon::sidex_embedded::types::Str;
use pico_platform::watchdog;
use static_cell::StaticCell;

use crate::display::Display;
use crate::storage::Storage;
use crate::touch::Touch;

/// Configuration and identity of the device.
static CONFIG: Config = pico_platform::nexigon_config!();

/// SPI1, which the display, the microSD slot, and the touch controller share.
type Bus = Mutex<NoopRawMutex, RefCell<Spi<'static, SPI1, spi::Blocking>>>;

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

    // Deselect the touch controller and the microSD card before using the
    // bus.
    let touch_chip_select = Output::new(p.PIN_16, Level::High);
    let sd_chip_select = Output::new(p.PIN_22, Level::High);
    static BUS: StaticCell<Bus> = StaticCell::new();
    let bus = BUS.init(Mutex::new(RefCell::new(Spi::new_blocking(
        p.SPI1,
        p.PIN_10,
        p.PIN_11,
        p.PIN_12,
        spi::Config::default(),
    ))));
    let display = Display::new(
        bus,
        display::Pins {
            chip_select: p.PIN_9,
            data_command: p.PIN_8,
            reset: p.PIN_15,
            backlight: p.PIN_13,
        },
    );
    let touch = Touch::new(bus, touch_chip_select, p.PIN_17);
    let storage = Storage::new(bus, sd_chip_select);

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
        TouchDisplay {
            parts: Some((display, touch, storage)),
        },
    )
    .await
}

/// Application of the device.
struct TouchDisplay {
    /// Display, touch screen, and storage, until [`receiver::task`] takes
    /// them.
    parts: Option<(Display, Touch, Storage)>,
}

impl Application for TouchDisplay {
    type Event = StorageEvent;

    const EVENT_CATEGORY: &'static str = "storage";

    const CONFIG_PROPERTY: Option<&'static str> = Some("touch-display.config");

    fn start(&mut self, spawner: Spawner, stack: Stack<'static>) {
        let (display, touch, storage) = self.parts.take().expect("the application starts once");
        spawner.spawn(receiver::task(stack, display, touch, storage).unwrap());
    }

    fn configure(&mut self, config: Option<Json<'_>>) {
        let address = config.and_then(|config| {
            let address = config
                .decode::<DisplayConfig>()
                .ok()
                .map(|config| config.sensor_address.as_ref().map(parse_address));
            match address {
                Some(Some(Some(address))) => Some(address),
                Some(None) => None,
                _ => {
                    warn!("invalid configuration: {config:?}");
                    None
                }
            }
        });
        receiver::set_sensor_address(address);
    }

    async fn next_event(&mut self) -> Option<StorageEvent> {
        Some(StorageEvent {
            storage: storage::counts(),
            receiver: receiver::counts(),
        })
    }
}

/// Parses an IPv4 address, returning `None` if it is invalid.
fn parse_address(address: &Str<'_>) -> Option<Ipv4Addr> {
    address.as_str()?.parse().ok()
}

/// Configuration of the device.
struct DisplayConfig<'a> {
    /// IP address of the weather station.
    sensor_address: Option<Str<'a>>,
}

impl<'a> Decode<'a> for DisplayConfig<'a> {
    const IS_RECORD: bool = true;

    fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError> {
        let fields = reader.record(&[&["sensorAddress"]])?;
        Ok(Self {
            sensor_address: fields.optional(0)?,
        })
    }
}

/// Body of storage events, with the counts of the measurements and datagrams
/// since the start.
struct StorageEvent {
    storage: storage::Counts,
    receiver: receiver::Counts,
}

impl Encode for StorageEvent {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        let mut object = writer.object()?;
        self.storage
            .stored
            .encode(object.field("storedMeasurements")?)?;
        self.storage
            .buffered
            .encode(object.field("bufferedMeasurements")?)?;
        self.storage
            .dropped
            .encode(object.field("droppedMeasurements")?)?;
        self.receiver
            .rejected
            .encode(object.field("rejectedDatagrams")?)?;
        self.receiver.lost.encode(object.field("lostDatagrams")?)?;
        object.end()
    }
}
