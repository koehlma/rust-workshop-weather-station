//! Storage of lines on the microSD card.
//!
//! The card is on the shared [`Bus`] in SPI mode, with its chip select on
//! GP22. [`Storage::append`] appends a line to [`FILE_NAME`] in the root
//! directory of the first FAT partition. It opens and closes the file for
//! every line, so that every line is on the card once it returns.
//!
//! The card starts at [`INIT_FREQUENCY`] and continues at [`FREQUENCY`]. After
//! a failure, the next line starts the card again, for instance, after it has
//! been removed and inserted.

use core::fmt;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi;
use embassy_rp::spi::Spi;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Delay;
use embedded_sdmmc::Mode;
use embedded_sdmmc::SdCard;
use embedded_sdmmc::SdCardError;
use embedded_sdmmc::TimeSource;
use embedded_sdmmc::Timestamp;
use embedded_sdmmc::VolumeIdx;
use embedded_sdmmc::VolumeManager;
use log::info;
use pico_platform::nexigon;

use crate::Bus;

/// Name of the file, in 8.3 format.
pub const FILE_NAME: &str = "WEATHER.LOG";

/// Clock frequency while the card starts, which must be at most 400 kHz.
const INIT_FREQUENCY: u32 = 400_000;

/// Clock frequency once the card has started.
const FREQUENCY: u32 = 16_000_000;

type Card = SdCard<
    SpiDeviceWithConfig<'static, NoopRawMutex, Spi<'static, SPI1, spi::Blocking>, Output<'static>>,
    Delay,
>;

/// Error of the card or its file system.
#[derive(Debug)]
pub struct Error(embedded_sdmmc::Error<SdCardError>);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<embedded_sdmmc::Error<SdCardError>> for Error {
    fn from(error: embedded_sdmmc::Error<SdCardError>) -> Self {
        Self(error)
    }
}

impl From<SdCardError> for Error {
    fn from(error: SdCardError) -> Self {
        Self(embedded_sdmmc::Error::DeviceError(error))
    }
}

/// File on the microSD card.
pub struct Storage {
    bus: &'static Bus,
    volumes: VolumeManager<Card, Clock>,
    /// Indicates whether the card has started.
    started: bool,
}

impl Storage {
    /// Creates the storage without accessing the card.
    pub fn new(bus: &'static Bus, chip_select: Output<'static>) -> Self {
        let device = SpiDeviceWithConfig::new(bus, chip_select, config(INIT_FREQUENCY));
        Self {
            bus,
            volumes: VolumeManager::new(SdCard::new(device, Delay), Clock),
            started: false,
        }
    }

    /// Appends `line`, which should end with a newline, to the file.
    pub fn append(&mut self, line: &[u8]) -> Result<(), Error> {
        if !self.started {
            self.start()?;
        }
        let result = self.write(line);
        self.started = result.is_ok();
        result
    }

    /// Starts the card.
    fn start(&mut self) -> Result<(), Error> {
        // The card needs at least 74 clock cycles with its chip select high
        // before it accepts commands.
        self.bus.lock(|bus| {
            let mut bus = bus.borrow_mut();
            bus.set_config(&config(INIT_FREQUENCY));
            bus.blocking_write(&[0xFF; 10])
                .expect("blocking writes do not fail");
        });
        let bytes = self.volumes.device(|card| {
            card.spi(|device| device.set_config(config(INIT_FREQUENCY)));
            card.mark_card_uninit();
            card.num_bytes()
        })?;
        info!("started SD card with {} MB", bytes / 1_000_000);
        self.volumes
            .device(|card| card.spi(|device| device.set_config(config(FREQUENCY))));
        Ok(())
    }

    /// Appends `line` to the file.
    fn write(&mut self, line: &[u8]) -> Result<(), Error> {
        let volume = self.volumes.open_volume(VolumeIdx(0))?;
        let directory = volume.open_root_dir()?;
        let file = directory.open_file_in_dir(FILE_NAME, Mode::ReadWriteCreateOrAppend)?;
        file.write(line)?;
        file.close()?;
        directory.close()?;
        volume.close()?;
        Ok(())
    }
}

/// Configuration of the bus at `frequency`.
fn config(frequency: u32) -> spi::Config {
    let mut config = spi::Config::default();
    config.frequency = frequency;
    config
}

/// Time of the files, which is the start of 1980 until the clock is
/// synchronized.
struct Clock;

impl TimeSource for Clock {
    fn get_timestamp(&self) -> Timestamp {
        let Some(seconds) = nexigon::unix_seconds() else {
            return Timestamp::from_fat(0x21, 0);
        };
        let (year, month, day) = civil_from_days(seconds / 86_400);
        let time = seconds % 86_400;
        Timestamp::from_calendar(
            year,
            month,
            day,
            (time / 3_600) as u8,
            (time / 60 % 60) as u8,
            (time % 60) as u8,
        )
        .unwrap_or(Timestamp::from_fat(0x21, 0))
    }
}

/// Converts days since the Unix epoch into year, month, and day, with the
/// algorithm of Howard Hinnant.
fn civil_from_days(days: u64) -> (u16, u8, u8) {
    let days = days + 719_468;
    let era = days / 146_097;
    let day_of_era = days % 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    };
    let year = year_of_era + era * 400 + u64::from(month <= 2);
    (year as u16, month as u8, day as u8)
}
