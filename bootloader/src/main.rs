//! Bootloader that swaps in updates of the firmware and rolls back failed ones.
//!
//! The firmware runs from the active slot and downloads an update into the DFU
//! slot. After a reset, the bootloader swaps the two slots page by page and
//! starts the new firmware. If the new firmware does not mark itself as booted
//! before the next reset, the bootloader swaps the slots back. A power failure
//! during a swap resumes the swap. See `memory.x` for the layout of the flash.

#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m_rt::entry;
use embassy_boot_rp::BootLoader;
use embassy_boot_rp::BootLoaderConfig;
use embassy_boot_rp::WatchdogFlash;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::Duration;
use panic_reset as _;

/// Size of the flash of the Raspberry Pi Pico W.
const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());

    // The watchdog resets the device if the bootloader or the firmware hangs.
    // The firmware takes over feeding it.
    let flash = WatchdogFlash::<FLASH_SIZE>::start(p.FLASH, p.WATCHDOG, Duration::from_secs(8));
    let flash = Mutex::new(RefCell::new(flash));

    let config = BootLoaderConfig::from_linkerfile_blocking(&flash, &flash, &flash);
    let active_offset = config.active.offset();
    let bootloader: BootLoader = BootLoader::prepare(config);

    // SAFETY: The active slot holds firmware linked for its address.
    unsafe { bootloader.load(embassy_rp::flash::FLASH_BASE as u32 + active_offset) }
}
