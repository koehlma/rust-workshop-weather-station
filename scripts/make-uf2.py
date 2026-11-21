#!/usr/bin/env python3
"""Creates the UF2 file that installs the bootloader and the firmware.

The file writes the bootloader to the start of the flash, erases the state of
the bootloader, and writes the firmware to the active slot. The addresses match
`firmware.x` and `bootloader/memory.x`.

Usage: make-uf2.py OUTPUT BOOTLOADER_BIN FIRMWARE_BIN
"""

import struct
import sys

BOOTLOADER_ADDRESS = 0x10000000
STATE_ADDRESS = 0x10006000
STATE_SIZE = 4096
ACTIVE_ADDRESS = 0x10007000
ACTIVE_SIZE = 1008 * 1024

RP2040_FAMILY_ID = 0xE48BFF56
FLAG_FAMILY_ID_PRESENT = 0x00002000
PAYLOAD_SIZE = 256


def blocks(address, data):
    """Splits data into UF2 payloads of 256 bytes, padded with zeros."""
    for offset in range(0, len(data), PAYLOAD_SIZE):
        payload = data[offset : offset + PAYLOAD_SIZE]
        yield address + offset, payload.ljust(PAYLOAD_SIZE, b"\0")


def main():
    output, bootloader_path, firmware_path = sys.argv[1:]
    with open(bootloader_path, "rb") as file:
        bootloader = file.read()
    with open(firmware_path, "rb") as file:
        firmware = file.read()
    if BOOTLOADER_ADDRESS + len(bootloader) > STATE_ADDRESS:
        sys.exit("error: the bootloader does not fit before its state")
    if len(firmware) > ACTIVE_SIZE:
        sys.exit("error: the firmware does not fit into the active slot")

    payloads = [
        *blocks(BOOTLOADER_ADDRESS, bootloader),
        # Erased flash reads as 0xFF, which the bootloader reads as no update.
        *blocks(STATE_ADDRESS, b"\xff" * STATE_SIZE),
        *blocks(ACTIVE_ADDRESS, firmware),
    ]
    with open(output, "wb") as file:
        for number, (address, payload) in enumerate(payloads):
            header = struct.pack(
                "<IIIIIIII",
                0x0A324655,
                0x9E5D5157,
                FLAG_FAMILY_ID_PRESENT,
                address,
                PAYLOAD_SIZE,
                number,
                len(payloads),
                RP2040_FAMILY_ID,
            )
            block = header + payload
            block = block.ljust(512 - 4, b"\0") + struct.pack("<I", 0x0AB16F30)
            file.write(block)


if __name__ == "__main__":
    main()
