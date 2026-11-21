#!/usr/bin/env bash
# Builds the bootloader and a firmware crate of `crates/apps/` and writes to
# `target/<crate>/`:
#
# - `<crate>.uf2`: bootloader and firmware for flashing over USB.
# - `firmware.bin`: firmware image for updates through Nexigon Hub.
# - `firmware.bin.sig`: signature of the image with the release key, if
#   `keys/release-key.pem` exists.
# - `firmware.version`: version of the firmware.
#
# Usage: build.sh CRATE [CARGO_ARGS...]
#
# The version is `FIRMWARE_VERSION` or, by default, the time of the build.
# Further arguments go to `cargo build` of the firmware, for instance,
# `--no-default-features` to build the weather station without Nexigon.

set -euo pipefail

cd "$(dirname "$0")/.."

if [ $# -lt 1 ] || [ ! -d "crates/apps/$1" ] || [ "$1" = bootloader ]; then
    echo "usage: $0 CRATE [CARGO_ARGS...], for instance, $0 weather-station" >&2
    exit 1
fi
crate="$1"
shift
out="target/$crate"
mkdir -p "$out"

export FIRMWARE_VERSION="${FIRMWARE_VERSION:-$(date -u +%Y%m%d-%H%M%S)}"
# Separate builds keep the features of the firmware's dependencies out of the
# bootloader.
cargo build --release -p bootloader
cargo build --release -p "$crate" "$@"

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
"$objcopy" -O binary --remove-section .boot2 "$elf/$crate" "$out/firmware.bin"
python3 scripts/make-uf2.py "$out/$crate.uf2" target/bootloader.bin "$out/firmware.bin"
rm -f "$out/firmware.bin.sig"
# Only releases through Nexigon Hub need the signature.
if [ -f keys/release-key.pem ]; then
    openssl dgst -sha256 -sign keys/release-key.pem -out "$out/firmware.bin.sig" "$out/firmware.bin"
fi
echo "$FIRMWARE_VERSION" > "$out/firmware.version"

echo "Built $crate $FIRMWARE_VERSION into $out/."
