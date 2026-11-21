//! Binary format of the measurements on the microSD card.
//!
//! Files consist of [`Block`]s of [`BLOCK_SIZE`] bytes, one sector of the
//! card. All integers are little-endian. A block starts with a header of
//! [`HEADER_SIZE`] bytes:
//!
//! | Offset | Size | Content                                                   |
//! | ------ | ---- | --------------------------------------------------------- |
//! | 0      | 4    | Magic `WSLG`                                              |
//! | 4      | 1    | Format version, [`VERSION`]                               |
//! | 5      | 1    | Number of records, at most [`RECORDS_PER_BLOCK`]          |
//! | 6      | 2    | Reserved, zero                                            |
//! | 8      | 4    | Index of the block in its file                            |
//! | 12     | 4    | CRC-32 (IEEE) of the block with this field set to zero    |
//!
//! [`RECORDS_PER_BLOCK`] records of [`RECORD_SIZE`] bytes follow. Slots after
//! the last record are zero. A [`Record`] is laid out as follows:
//!
//! | Offset | Size | Content                                   |
//! | ------ | ---- | ----------------------------------------- |
//! | 0      | 4    | Time of the measurement in Unix seconds   |
//! | 4      | 2    | Temperature in 0.01 °C, signed            |
//! | 6      | 2    | Relative humidity in 0.01 %               |
//! | 8      | 4    | Air pressure in Pa                        |
//! | 12     | 2    | Output voltage of the UV sensor in 0.1 mV |
//! | 14     | 1    | Values present, one bit for each value    |
//! | 15     | 1    | Reserved, zero                            |
//!
//! The bits of the present values are [`TEMPERATURE`], [`HUMIDITY`],
//! [`PRESSURE`], and [`UV`]. Missing values are zero.

/// Size of a block, which is the size of a sector of the card.
pub const BLOCK_SIZE: usize = 512;

/// Size of the header of a block.
pub const HEADER_SIZE: usize = 16;

/// Size of a record.
pub const RECORD_SIZE: usize = 16;

/// Number of records in a full block.
pub const RECORDS_PER_BLOCK: usize = (BLOCK_SIZE - HEADER_SIZE) / RECORD_SIZE;

/// Magic at the start of every block.
const MAGIC: [u8; 4] = *b"WSLG";

/// Version of the format.
pub const VERSION: u8 = 1;

/// Bit of the temperature in the present values.
pub const TEMPERATURE: u8 = 1 << 0;
/// Bit of the humidity in the present values.
pub const HUMIDITY: u8 = 1 << 1;
/// Bit of the air pressure in the present values.
pub const PRESSURE: u8 = 1 << 2;
/// Bit of the UV voltage in the present values.
pub const UV: u8 = 1 << 3;

/// Measurement of the weather station.
#[derive(Debug, Clone, Copy, Default)]
pub struct Record {
    /// Time of the measurement in Unix seconds.
    pub time: u32,
    /// Temperature in degrees Celsius.
    pub temperature_celsius: Option<f32>,
    /// Relative humidity in percent.
    pub humidity_percent: Option<f32>,
    /// Air pressure in hectopascals.
    pub pressure_hectopascals: Option<f32>,
    /// Output voltage of the UV sensor in volts.
    pub uv_volts: Option<f32>,
}

impl Record {
    /// Writes the record to `bytes`.
    fn encode(&self, bytes: &mut [u8]) {
        let mut present = 0;
        let mut value = |bit: u8, value: Option<f32>, scale: f32| match value {
            Some(value) => {
                present |= bit;
                round(value * scale)
            }
            None => 0,
        };
        let temperature = value(TEMPERATURE, self.temperature_celsius, 100.0);
        let humidity = value(HUMIDITY, self.humidity_percent, 100.0);
        let pressure = value(PRESSURE, self.pressure_hectopascals, 100.0);
        let uv = value(UV, self.uv_volts, 10_000.0);
        bytes[0..4].copy_from_slice(&self.time.to_le_bytes());
        // Values outside of the range of a field are clamped to it.
        let temperature = temperature.clamp(i16::MIN.into(), i16::MAX.into()) as i16;
        let humidity = humidity.clamp(0, u16::MAX.into()) as u16;
        let pressure = pressure.clamp(0, u32::MAX.into()) as u32;
        let uv = uv.clamp(0, u16::MAX.into()) as u16;
        bytes[4..6].copy_from_slice(&temperature.to_le_bytes());
        bytes[6..8].copy_from_slice(&humidity.to_le_bytes());
        bytes[8..12].copy_from_slice(&pressure.to_le_bytes());
        bytes[12..14].copy_from_slice(&uv.to_le_bytes());
        bytes[14] = present;
        bytes[15] = 0;
    }
}

/// Rounds `value` to the nearest integer, saturating at the limits of `i64`.
fn round(value: f32) -> i64 {
    if value >= 0.0 {
        (value + 0.5) as i64
    } else {
        (value - 0.5) as i64
    }
}

/// Block of records, which may be partially filled.
#[derive(Clone)]
pub struct Block {
    bytes: [u8; BLOCK_SIZE],
}

impl Block {
    /// Creates an empty block.
    pub const fn new() -> Self {
        Self {
            bytes: [0; BLOCK_SIZE],
        }
    }

    /// Returns the number of records.
    pub fn len(&self) -> usize {
        usize::from(self.bytes[5])
    }

    /// Indicates whether the block holds no records.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Indicates whether the block holds [`RECORDS_PER_BLOCK`] records.
    pub fn is_full(&self) -> bool {
        self.len() == RECORDS_PER_BLOCK
    }

    /// Appends `record`, which must fit.
    pub fn push(&mut self, record: &Record) {
        let index = self.len();
        assert!(index < RECORDS_PER_BLOCK, "the block is not full");
        let start = HEADER_SIZE + index * RECORD_SIZE;
        record.encode(&mut self.bytes[start..start + RECORD_SIZE]);
        self.bytes[5] = (index + 1) as u8;
    }

    /// Removes all records.
    pub fn clear(&mut self) {
        self.bytes = [0; BLOCK_SIZE];
    }

    /// Returns the bytes of the block as the block at `index` of its file.
    pub fn encode(&mut self, index: u32) -> &[u8; BLOCK_SIZE] {
        self.bytes[0..4].copy_from_slice(&MAGIC);
        self.bytes[4] = VERSION;
        self.bytes[8..12].copy_from_slice(&index.to_le_bytes());
        self.bytes[12..16].fill(0);
        let crc = crc32(&self.bytes);
        self.bytes[12..16].copy_from_slice(&crc.to_le_bytes());
        &self.bytes
    }

    /// Reads the block at `index` of its file, or returns `None` if `bytes`
    /// is not a valid block at that index.
    pub fn decode(bytes: &[u8; BLOCK_SIZE], index: u32) -> Option<Self> {
        let mut block = Self { bytes: *bytes };
        let crc = u32::from_le_bytes(block.bytes[12..16].try_into().unwrap());
        block.bytes[12..16].fill(0);
        let valid = block.bytes[0..4] == MAGIC
            && block.bytes[4] == VERSION
            && block.len() <= RECORDS_PER_BLOCK
            && block.bytes[8..12] == index.to_le_bytes()
            && crc32(&block.bytes) == crc;
        valid.then_some(block)
    }
}

/// Computes the CRC-32 of `bytes` with the IEEE polynomial, as `zlib` does.
fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = if crc & 1 == 1 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}
