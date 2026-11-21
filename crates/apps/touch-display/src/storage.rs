//! Storage of measurements on the microSD card.
//!
//! The card is on the shared [`Bus`] in SPI mode, with its chip select on
//! GP22. [`Storage`] collects [`Record`]s in a [`Block`] in memory and writes
//! the block to a file of the day, `LOG/<year>/<yyyymmdd>.WSL`, in the
//! format of [`crate::format`]. Days are in UTC.
//!
//! A full block is written at once and starts the next block. A partial block
//! is written every [`CHECKPOINT_INTERVAL`] at the same position, so that a
//! power failure loses at most the measurements of that interval. When it
//! opens the file of a day, [`Storage`] continues its last block if that is
//! partial. After a failure, for instance, after the card has been removed,
//! the next write starts the card again. [`Storage::read_day`] reads the
//! records of a day, those on the card and those in memory.
//!
//! The card starts at [`INIT_FREQUENCY`] and continues at [`FREQUENCY`].

use core::fmt;
use core::fmt::Write as _;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi;
use embassy_rp::spi::Spi;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Delay;
use embassy_time::Duration;
use embassy_time::Instant;
use embedded_sdmmc::Mode;
use embedded_sdmmc::RawDirectory;
use embedded_sdmmc::RawFile;
use embedded_sdmmc::RawVolume;
use embedded_sdmmc::SdCard;
use embedded_sdmmc::SdCardError;
use embedded_sdmmc::TimeSource;
use embedded_sdmmc::Timestamp;
use embedded_sdmmc::VolumeIdx;
use embedded_sdmmc::VolumeManager;
use heapless::String;
use log::info;
use log::warn;
use pico_platform::nexigon;
use portable_atomic::AtomicU32;
use portable_atomic::Ordering;

use crate::Bus;
use crate::format::BLOCK_SIZE;
use crate::format::Block;
use crate::format::Record;

/// Directory of the files in the root directory of the card.
const DIRECTORY: &str = "LOG";

/// Extension of the files.
const EXTENSION: &str = "WSL";

/// Maximum time for which measurements stay in memory only.
pub const CHECKPOINT_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Clock frequency while the card starts, which must be at most 400 kHz.
const INIT_FREQUENCY: u32 = 400_000;

/// Clock frequency once the card has started.
const FREQUENCY: u32 = 16_000_000;

/// Seconds of a day.
pub const DAY_SECONDS: u32 = 86_400;

/// Number of measurements written to the card since the start.
static STORED: AtomicU32 = AtomicU32::new(0);

/// Number of measurements in memory that are not on the card yet.
static BUFFERED: AtomicU32 = AtomicU32::new(0);

/// Number of measurements that could not be stored since the start.
static DROPPED: AtomicU32 = AtomicU32::new(0);

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

/// Counts of the measurements since the start.
#[derive(Debug, Clone, Copy)]
pub struct Counts {
    /// Measurements written to the card.
    pub stored: u32,
    /// Measurements in memory that are not on the card yet.
    pub buffered: u32,
    /// Measurements that could not be stored.
    pub dropped: u32,
}

/// Returns the counts of the measurements since the start.
pub fn counts() -> Counts {
    Counts {
        stored: STORED.load(Ordering::Relaxed),
        buffered: BUFFERED.load(Ordering::Relaxed),
        dropped: DROPPED.load(Ordering::Relaxed),
    }
}

/// Measurements on the microSD card.
pub struct Storage {
    bus: &'static Bus,
    volumes: VolumeManager<Card, Clock>,
    /// Indicates whether the card has started.
    started: bool,
    /// Indicates whether the last access of the card failed.
    failed: bool,
    volume: Option<RawVolume>,
    /// File to which the block is written, with its day in days since the
    /// Unix epoch.
    file: Option<(RawFile, u32)>,
    /// Block that collects the measurements.
    block: Block,
    /// Day of the block in days since the Unix epoch.
    day: u32,
    /// Index of the block in the file of its day, once known.
    index: Option<u32>,
    /// Number of records of the block that are on the card.
    written: usize,
    /// Time of the oldest record of the block that is not on the card.
    pending_since: Option<Instant>,
}

impl Storage {
    /// Creates the storage without accessing the card.
    pub fn new(bus: &'static Bus, chip_select: Output<'static>) -> Self {
        let device = SpiDeviceWithConfig::new(bus, chip_select, config(INIT_FREQUENCY));
        Self {
            bus,
            volumes: VolumeManager::new(SdCard::new(device, Delay), Clock),
            started: false,
            failed: false,
            volume: None,
            file: None,
            block: Block::new(),
            day: 0,
            index: None,
            written: 0,
            pending_since: None,
        }
    }

    /// Adds `record`. Writes the block if it becomes full or if `record`
    /// belongs to another day than the block.
    pub fn push(&mut self, record: &Record) {
        let day = record.time / DAY_SECONDS;
        if day != self.day && !self.block.is_empty() {
            let _ = self.write();
            // The block of the previous day is complete, even if it is not
            // on the card.
            self.discard();
        }
        if day != self.day {
            self.day = day;
            self.index = None;
        }
        if self.block.is_empty() && self.index.is_none() {
            // Continue the last block of the file if it is partial.
            if let Err(error) = self.open(day) {
                warn!("failed to open the file of the day: {error}");
                self.fail();
            }
        }
        if self.block.is_full() {
            warn!("dropped a measurement: the block cannot be written");
            DROPPED.add(1, Ordering::Relaxed);
            return;
        }
        self.block.push(record);
        self.pending_since.get_or_insert_with(Instant::now);
        self.update_buffered();
        if self.block.is_full() {
            let _ = self.write();
        }
    }

    /// Returns the time at which the block must be written, if it holds
    /// records that are not on the card.
    pub fn checkpoint_deadline(&self) -> Option<Instant> {
        self.pending_since.map(|since| since + CHECKPOINT_INTERVAL)
    }

    /// Writes the block if it holds records that are not on the card.
    pub fn checkpoint(&mut self) {
        if self.pending_since.is_some() {
            let _ = self.write();
        }
    }

    /// Indicates whether the last access of the card failed.
    pub fn has_failed(&self) -> bool {
        self.failed
    }

    /// Calls `f` with the records of `day` in days since the Unix epoch,
    /// those on the card and those in memory, oldest first.
    pub fn read_day(&mut self, day: u32, mut f: impl FnMut(&Record)) -> Result<(), Error> {
        let result = self.read_card(day, &mut f);
        if let Err(error) = &result {
            warn!("failed to read from the SD card: {error}");
            self.fail();
        }
        if day == self.day {
            self.block.records().for_each(|record| f(&record));
        }
        result
    }

    /// Calls `f` with the records of `day` on the card that are not in
    /// memory.
    fn read_card(&mut self, day: u32, f: &mut impl FnMut(&Record)) -> Result<(), Error> {
        let volume = self.volume()?;
        // The block in memory replaces the blocks from its index on.
        let end = if day == self.day { self.index } else { None };
        let (file, opened) = match self.file {
            Some((file, file_day)) if file_day == day => (file, false),
            _ => match self.open_file(volume, day, Mode::ReadOnly) {
                Ok(file) => (file, true),
                Err(Error(embedded_sdmmc::Error::NotFound)) => return Ok(()),
                Err(error) => return Err(error),
            },
        };
        let result = self.read_blocks(file, end, f);
        if opened {
            let _ = self.volumes.close_file(file);
        }
        result
    }

    /// Calls `f` with the records of the valid blocks of `file` before
    /// `end`, or of all blocks if `end` is `None`.
    fn read_blocks(
        &self,
        file: RawFile,
        end: Option<u32>,
        f: &mut impl FnMut(&Record),
    ) -> Result<(), Error> {
        let blocks = self.volumes.file_length(file)? / BLOCK_SIZE as u32;
        let end = end.map_or(blocks, |end| end.min(blocks));
        self.volumes.file_seek_from_start(file, 0)?;
        let mut bytes = [0; BLOCK_SIZE];
        for index in 0..end {
            self.volumes.read(file, &mut bytes)?;
            match Block::decode(&bytes, index) {
                Some(block) => block.records().for_each(|record| f(&record)),
                None => warn!("skipped invalid block {index}"),
            }
        }
        Ok(())
    }

    /// Writes the block to the card. Starts the next block if it is full.
    fn write(&mut self) -> Result<(), Error> {
        let result = self.write_block();
        match &result {
            Ok(()) => {
                self.failed = false;
                STORED.add((self.block.len() - self.written) as u32, Ordering::Relaxed);
                self.written = self.block.len();
                self.pending_since = None;
                if self.block.is_full() {
                    self.block.clear();
                    self.written = 0;
                    self.index = self.index.map(|index| index + 1);
                }
            }
            Err(error) => {
                warn!("failed to write to the SD card: {error}");
                self.fail();
                // Try again after another interval.
                self.pending_since = Some(Instant::now());
            }
        }
        self.update_buffered();
        result
    }

    /// Writes the block at its index to the file of its day.
    fn write_block(&mut self) -> Result<(), Error> {
        let (file, blocks) = self.open(self.day)?;
        // A block cannot start after the end of the file, for instance,
        // after the card has been replaced.
        let index = self.index.unwrap_or(blocks).min(blocks);
        self.index = Some(index);
        self.volumes
            .file_seek_from_start(file, index * BLOCK_SIZE as u32)?;
        self.volumes.write(file, self.block.encode(index))?;
        self.volumes.flush_file(file)?;
        Ok(())
    }

    /// Drops the records of the block that are not on the card and starts
    /// a new block.
    fn discard(&mut self) {
        let lost = self.block.len() - self.written;
        if lost > 0 {
            warn!("dropped {lost} measurements that could not be written");
            DROPPED.add(lost as u32, Ordering::Relaxed);
        }
        self.block.clear();
        self.written = 0;
        self.pending_since = None;
        self.update_buffered();
    }

    /// Publishes the number of records that are not on the card.
    fn update_buffered(&self) {
        BUFFERED.store((self.block.len() - self.written) as u32, Ordering::Relaxed);
    }

    /// Opens the volume, starting the card if necessary.
    fn volume(&mut self) -> Result<RawVolume, Error> {
        if !self.started {
            self.start()?;
        }
        match self.volume {
            Some(volume) => Ok(volume),
            None => {
                let volume = self.volumes.open_raw_volume(VolumeIdx(0))?;
                self.volume = Some(volume);
                Ok(volume)
            }
        }
    }

    /// Opens the file of `day` for writing and returns it with its number of
    /// blocks.
    ///
    /// If the block is empty and has no index yet, it continues the last
    /// block of the file if that is partial.
    fn open(&mut self, day: u32) -> Result<(RawFile, u32), Error> {
        if let Some((file, file_day)) = self.file
            && file_day != day
        {
            let _ = self.volumes.close_file(file);
            self.file = None;
        }
        let volume = self.volume()?;
        let file = match self.file {
            Some((file, _)) => file,
            None => {
                let file = self.open_file(volume, day, Mode::ReadWriteCreateOrAppend)?;
                self.file = Some((file, day));
                file
            }
        };
        let blocks = self.volumes.file_length(file)? / BLOCK_SIZE as u32;
        if self.block.is_empty() && self.index.is_none() {
            self.index = Some(blocks);
            if let Some(last) = blocks.checked_sub(1) {
                let mut bytes = [0; BLOCK_SIZE];
                self.volumes
                    .file_seek_from_start(file, last * BLOCK_SIZE as u32)?;
                self.volumes.read(file, &mut bytes)?;
                if let Some(block) = Block::decode(&bytes, last).filter(|block| !block.is_full()) {
                    info!("continuing block {last} with {} measurements", block.len());
                    self.written = block.len();
                    self.block = block;
                    self.index = Some(last);
                }
            }
        }
        Ok((file, blocks))
    }

    /// Opens the file of `day` on `volume` in `mode`. Creates the file and
    /// its directories unless `mode` is [`Mode::ReadOnly`].
    fn open_file(&self, volume: RawVolume, day: u32, mode: Mode) -> Result<RawFile, Error> {
        let create = mode != Mode::ReadOnly;
        let (year, month, day) = civil_from_days(day.into());
        let mut year_name = String::<4>::new();
        let mut file_name = String::<12>::new();
        write!(year_name, "{year:04}").expect("years have four digits");
        write!(file_name, "{year:04}{month:02}{day:02}.{EXTENSION}")
            .expect("file names have 12 characters");

        let root = self.volumes.open_root_dir(volume)?;
        let directory = self.open_dir(root, DIRECTORY, create);
        let _ = self.volumes.close_dir(root);
        let directory = directory?;
        let year_directory = self.open_dir(directory, &year_name, create);
        let _ = self.volumes.close_dir(directory);
        let year_directory = year_directory?;
        let file = self
            .volumes
            .open_file_in_dir(year_directory, file_name.as_str(), mode);
        let _ = self.volumes.close_dir(year_directory);
        let file = file?;
        if create {
            info!("opened {DIRECTORY}/{year_name}/{file_name}");
        }
        Ok(file)
    }

    /// Opens the directory `name` in `parent`. Creates it if it is missing
    /// and `create` is set.
    fn open_dir(
        &self,
        parent: RawDirectory,
        name: &str,
        create: bool,
    ) -> Result<RawDirectory, Error> {
        match self.volumes.open_dir(parent, name) {
            Err(embedded_sdmmc::Error::NotFound) if create => {
                self.volumes.make_dir_in_dir(parent, name)?;
                Ok(self.volumes.open_dir(parent, name)?)
            }
            result => Ok(result?),
        }
    }

    /// Closes the file and the volume.
    fn close(&mut self) {
        if let Some((file, _)) = self.file.take() {
            let _ = self.volumes.close_file(file);
        }
        if let Some(volume) = self.volume.take() {
            let _ = self.volumes.close_volume(volume);
        }
    }

    /// Closes the file, so that the next access starts the card again.
    fn fail(&mut self) {
        self.close();
        self.started = false;
        self.failed = true;
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
        self.started = true;
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
        let (year, month, day) = civil_from_days(seconds / u64::from(DAY_SECONDS));
        let time = seconds % u64::from(DAY_SECONDS);
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
