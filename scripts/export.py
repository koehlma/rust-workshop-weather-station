#!/usr/bin/env python3
"""Exports the measurements that the touch display stored on a microSD card.

Usage: export.py PATH...

PATH is a file written by the touch display, such as `LOG/2026/20261121.WSL`,
or a directory, which is searched for such files, such as the `LOG` directory
of the card. Writes the measurements as CSV to standard output, with empty
fields for missing values. Reports invalid blocks on standard error and skips
them. The format is described in `crates/apps/touch-display/src/format.rs`.
"""

import csv
import datetime
import pathlib
import struct
import sys
import zlib

BLOCK_SIZE = 512
HEADER = struct.Struct("<4sBBHII")
RECORD = struct.Struct("<IhHIHBB")
RECORDS_PER_BLOCK = (BLOCK_SIZE - HEADER.size) // RECORD.size
MAGIC = b"WSLG"
VERSION = 1

TEMPERATURE = 1 << 0
HUMIDITY = 1 << 1
PRESSURE = 1 << 2
UV = 1 << 3

COLUMNS = [
    "time",
    "temperature_celsius",
    "humidity_percent",
    "pressure_hectopascals",
    "uv_volts",
]


def files(paths):
    """Yields the files of `paths`, searching directories, in order."""
    for path in map(pathlib.Path, paths):
        if path.is_dir():
            yield from sorted(
                file for file in path.rglob("*") if file.suffix.upper() == ".WSL"
            )
        else:
            yield path


def blocks(path):
    """Yields the valid blocks of the file at `path`."""
    data = path.read_bytes()
    if len(data) % BLOCK_SIZE:
        print(f"{path}: ignoring {len(data) % BLOCK_SIZE} trailing bytes", file=sys.stderr)
    for index in range(len(data) // BLOCK_SIZE):
        block = data[index * BLOCK_SIZE : (index + 1) * BLOCK_SIZE]
        magic, version, count, _, stored_index, crc = HEADER.unpack_from(block)
        unchecked = block[:12] + bytes(4) + block[16:]
        if (
            magic != MAGIC
            or version != VERSION
            or count > RECORDS_PER_BLOCK
            or stored_index != index
            or zlib.crc32(unchecked) != crc
        ):
            print(f"{path}: skipping invalid block {index}", file=sys.stderr)
            continue
        yield block, count


def records(block, count):
    """Yields the rows of the records of `block`."""
    for slot in range(count):
        offset = HEADER.size + slot * RECORD.size
        time, temperature, humidity, pressure, uv, present, _ = RECORD.unpack_from(
            block, offset
        )

        def value(bit, raw, scale):
            return raw / scale if present & bit else None

        yield [
            datetime.datetime.fromtimestamp(time, datetime.timezone.utc).isoformat(),
            value(TEMPERATURE, temperature, 100),
            value(HUMIDITY, humidity, 100),
            value(PRESSURE, pressure, 100),
            value(UV, uv, 10_000),
        ]


def main():
    if len(sys.argv) < 2:
        print(__doc__.strip(), file=sys.stderr)
        sys.exit(1)
    writer = csv.writer(sys.stdout, lineterminator="\n")
    writer.writerow(COLUMNS)
    for path in files(sys.argv[1:]):
        for block, count in blocks(path):
            writer.writerows(records(block, count))


if __name__ == "__main__":
    main()
