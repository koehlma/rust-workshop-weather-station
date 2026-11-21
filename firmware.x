/*
 * Layout of the flash, shared with `bootloader/memory.x`:
 *
 * - BOOT2 and the bootloader take the first 24 KiB.
 * - BOOTLOADER_STATE records whether an update is pending and the progress
 *   of swapping it in.
 * - FLASH is the active slot, from which the firmware runs.
 * - DFU is the slot into which the firmware downloads updates. The swap needs
 *   it to be one page larger than the active slot.
 */
MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    BOOTLOADER_STATE : ORIGIN = 0x10006000, LENGTH = 4K
    FLASH : ORIGIN = 0x10007000, LENGTH = 1008K
    DFU : ORIGIN = 0x10103000, LENGTH = 1012K
    RAM : ORIGIN = 0x20000000, LENGTH = 264K
}

__bootloader_state_start = ORIGIN(BOOTLOADER_STATE) - ORIGIN(BOOT2);
__bootloader_state_end = ORIGIN(BOOTLOADER_STATE) + LENGTH(BOOTLOADER_STATE) - ORIGIN(BOOT2);

__bootloader_dfu_start = ORIGIN(DFU) - ORIGIN(BOOT2);
__bootloader_dfu_end = ORIGIN(DFU) + LENGTH(DFU) - ORIGIN(BOOT2);
