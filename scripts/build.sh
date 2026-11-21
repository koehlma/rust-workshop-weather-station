#!/usr/bin/env bash
# Builds the bootloader and the firmware and writes to `target/`:
#
# - `nexigon-mcu-rp-example.uf2`: bootloader and firmware for flashing over USB.
# - `firmware.bin`: firmware image for updates through Nexigon Hub.
# - `firmware.bin.sig`: signature of the image with the release key, if
#   `keys/release-key.pem` exists.
# - `firmware.version`: version of the firmware.
#
# The version is `FIRMWARE_VERSION` or, by default, the time of the build.
# Further arguments go to `cargo build` of the firmware, for instance,
# `--no-default-features` to build it without Nexigon.

set -euo pipefail

cd "$(dirname "$0")/.."

export FIRMWARE_VERSION="${FIRMWARE_VERSION:-$(date -u +%Y%m%d-%H%M%S)}"
# Separate builds keep the features of the firmware's dependencies out of the
# bootloader.
cargo build --release -p bootloader
cargo build --release -p nexigon-mcu-rp-example "$@"

elf=target/thumbv6m-none-eabi/release
sysroot="$(rustc --print sysroot)"
objcopy="$(find "$sysroot/lib/rustlib" -name llvm-objcopy -type f | head -n 1)"
if [ -z "$objcopy" ]; then
    echo "error: llvm-objcopy not found; install it with \`rustup component add llvm-tools\`" >&2
    exit 1
fi
"$objcopy" -O binary "$elf/bootloader" target/bootloader.bin
# The bootloader provides the second-stage boot code, so the image starts at
# the active slot.
"$objcopy" -O binary --remove-section .boot2 "$elf/nexigon-mcu-rp-example" target/firmware.bin
python3 scripts/make-uf2.py target/nexigon-mcu-rp-example.uf2 target/bootloader.bin target/firmware.bin
rm -f target/firmware.bin.sig
# Only releases through Nexigon Hub need the signature.
if [ -f keys/release-key.pem ]; then
    openssl dgst -sha256 -sign keys/release-key.pem -out target/firmware.bin.sig target/firmware.bin
fi
echo "$FIRMWARE_VERSION" > target/firmware.version

echo "Built firmware $FIRMWARE_VERSION."
