//! Firmware slots and updates.
//!
//! The firmware runs from the active slot. An update is written to the DFU
//! slot, checked against the release key, and marked for the bootloader, which
//! swaps it in after a reset. The new firmware must commit itself, or the
//! bootloader rolls it back after the next reset. The watchdog causes that
//! reset if the firmware hangs or, through [`commit_deadline_task`], does not
//! commit within [`COMMIT_DEADLINE`]. See `firmware.x` for the layout of the
//! flash.

use core::cell::RefCell;
use core::fmt;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

use embassy_boot_rp::BlockingFirmwareUpdater;
use embassy_boot_rp::FirmwareUpdaterConfig;
use embassy_boot_rp::State;
use embassy_rp::Peri;
use embassy_rp::flash::Blocking;
use embassy_rp::flash::ERASE_SIZE;
use embassy_rp::flash::Flash;
use embassy_rp::peripherals::FLASH;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::Timer;
use log::warn;
use p256::ecdsa::DerSignature;
use p256::ecdsa::VerifyingKey;
use sha2::Sha256;
use signature::hazmat::PrehashVerifier;
use static_cell::StaticCell;

/// Size of the flash of the Raspberry Pi Pico W.
const FLASH_SIZE: usize = 2 * 1024 * 1024;

/// Time after the boot within which new firmware must commit itself.
const COMMIT_DEADLINE: Duration = Duration::from_secs(5 * 60);

type FlashDriver = Flash<'static, FLASH, Blocking, FLASH_SIZE>;
type Partition =
    embassy_embedded_hal::flash::partition::BlockingPartition<'static, NoopRawMutex, FlashDriver>;

/// Indicates whether the running firmware has been committed.
static COMMITTED: AtomicBool = AtomicBool::new(false);

/// Error of an update.
#[derive(Debug)]
pub enum Error {
    /// Reading or writing the flash failed.
    Flash(embassy_boot::FirmwareUpdaterError),
    /// The image does not fit into the active slot.
    TooLarge,
    /// The signature does not match the image and the release key.
    Signature,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Flash(error) => write!(f, "flash error: {error:?}"),
            Self::TooLarge => f.write_str("the image does not fit into the active slot"),
            Self::Signature => f.write_str("the signature of the image is invalid"),
        }
    }
}

impl From<embassy_boot::FirmwareUpdaterError> for Error {
    fn from(error: embassy_boot::FirmwareUpdaterError) -> Self {
        Self::Flash(error)
    }
}

/// Firmware slots of the device.
pub struct Firmware {
    updater: BlockingFirmwareUpdater<'static, Partition, Partition>,
    state: State,
    /// Public key with which updates must be signed.
    release_key: &'static [u8],
}

impl Firmware {
    /// Takes control of the flash and reads the state of the bootloader.
    ///
    /// Updates must be signed with the private key of `release_key`, an
    /// uncompressed P-256 point.
    pub fn new(flash: Peri<'static, FLASH>, release_key: &'static [u8]) -> Self {
        static FLASH: StaticCell<Mutex<NoopRawMutex, RefCell<FlashDriver>>> = StaticCell::new();
        static ALIGNED: StaticCell<[u8; 1]> = StaticCell::new();
        let flash = FLASH.init(Mutex::new(RefCell::new(Flash::new_blocking(flash))));
        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(flash, flash);
        let mut updater = BlockingFirmwareUpdater::new(config, ALIGNED.init([0]));
        // Unreadable state counts as a normal boot, which never rolls back.
        let state = updater.get_state().unwrap_or(State::Boot);
        if state == State::Boot {
            COMMITTED.store(true, Ordering::Relaxed);
        }
        Self {
            updater,
            state,
            release_key,
        }
    }

    /// Indicates whether the bootloader has just swapped in this firmware,
    /// which must commit itself.
    pub fn awaits_commit(&self) -> bool {
        self.state == State::Swap
    }

    /// Commits the running firmware, so that the bootloader keeps it.
    ///
    /// This also acknowledges a rollback, after which the bootloader is in
    /// the revert state.
    pub fn commit(&mut self) -> Result<(), Error> {
        if self.state != State::Boot {
            self.updater.mark_booted()?;
            self.state = State::Boot;
        }
        COMMITTED.store(true, Ordering::Relaxed);
        Ok(())
    }

    /// Starts writing an update of `len` bytes to the DFU slot.
    ///
    /// The running firmware must be committed.
    pub fn update(&mut self, len: usize) -> Result<Update<'_>, Error> {
        if len > active_slot_size() {
            return Err(Error::TooLarge);
        }
        Ok(Update {
            updater: &mut self.updater,
            release_key: self.release_key,
            len,
            written: 0,
        })
    }
}

/// Update being written to the DFU slot.
pub struct Update<'f> {
    updater: &'f mut BlockingFirmwareUpdater<'static, Partition, Partition>,
    release_key: &'static [u8],
    len: usize,
    written: usize,
}

impl Update<'_> {
    /// Writes the next part of the image.
    pub fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        if self.written + data.len() > self.len {
            return Err(Error::TooLarge);
        }
        self.updater.write_firmware(self.written, data)?;
        self.written += data.len();
        Ok(())
    }

    /// Checks the written image against `signature` and the release key and
    /// marks it for the bootloader, which swaps it in after the next reset.
    ///
    /// The signature is an ECDSA P-256 signature over the SHA-256 digest of
    /// the image in DER format. Hashing the image as written to the flash also
    /// detects failed writes.
    pub fn finish(self, signature: &[u8]) -> Result<(), Error> {
        if self.written != self.len {
            return Err(Error::TooLarge);
        }
        let mut chunk = [0; 1024];
        let mut digest = [0; 32];
        self.updater
            .hash::<Sha256>(self.len as u32, &mut chunk, &mut digest)?;
        let key = VerifyingKey::from_sec1_bytes(self.release_key).map_err(|_| Error::Signature)?;
        let signature = DerSignature::from_bytes(signature).map_err(|_| Error::Signature)?;
        key.verify_prehash(&digest, &signature)
            .map_err(|_| Error::Signature)?;
        self.updater.mark_updated()?;
        Ok(())
    }
}

/// Size of the active slot, which is one page smaller than the DFU slot.
fn active_slot_size() -> usize {
    unsafe extern "C" {
        static __bootloader_dfu_start: u32;
        static __bootloader_dfu_end: u32;
    }
    // The linker defines the symbols, whose addresses are offsets.
    let start = &raw const __bootloader_dfu_start as usize;
    let end = &raw const __bootloader_dfu_end as usize;
    end - start - ERASE_SIZE
}

/// Resets the device, which activates a marked update.
pub fn reset() -> ! {
    cortex_m::peripheral::SCB::sys_reset()
}

/// Rolls back firmware that the bootloader has just swapped in unless it
/// commits itself within [`COMMIT_DEADLINE`].
///
/// The task stops feeding the watchdog, whose reset makes the bootloader roll
/// back.
#[embassy_executor::task]
pub async fn commit_deadline_task() {
    // Instants count from the boot.
    Timer::at(Instant::from_secs(0) + COMMIT_DEADLINE).await;
    if !COMMITTED.load(Ordering::Relaxed) {
        warn!("the firmware did not commit itself in time, rolling back");
        crate::watchdog::stop_feeding();
    }
}
