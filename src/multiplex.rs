//! Frames of the multiplex protocol of Nexigon, which carries channels over a
//! WebSocket connection.
//!
//! Every WebSocket message holds one frame. A frame starts with a tag, followed
//! by fixed-size fields in big-endian byte order and, optionally, a final
//! variable-size field. Channels are identified by IDs that each side assigns
//! to its end. Their data is subject to flow control with credits for frames
//! and bytes, which the receiver grants to the sender.

/// Magic bytes of the `Hello` frame, which identify the protocol.
const PROTOCOL_MAGIC: [u8; 16] = [
    27, 27, 46, 134, 44, 205, 244, 157, 82, 109, 227, 13, 167, 171, 225, 140,
];

const TAG_HELLO: u8 = 0x00;
const TAG_CLOSE: u8 = 0xFF;
const TAG_CHANNEL_REQUEST: u8 = 0x10;
const TAG_CHANNEL_ACCEPT: u8 = 0x11;
const TAG_CHANNEL_REJECT: u8 = 0x12;
const TAG_CHANNEL_DATA: u8 = 0x13;
const TAG_CHANNEL_ADJUST: u8 = 0x14;
const TAG_CHANNEL_CLOSE: u8 = 0x15;
const TAG_CHANNEL_CLOSED: u8 = 0x17;
const TAG_PING: u8 = 0x20;
const TAG_PONG: u8 = 0x21;

/// Size of the header of a `ChannelData` frame, which precedes its payload.
pub const DATA_HEADER_SIZE: usize = 9;

/// Frame of the multiplex protocol.
///
/// Channel IDs named `receiver_id` identify the receiver's end of a channel,
/// and those named `sender_id` the sender's end.
#[derive(Debug, Clone, Copy)]
pub enum Frame<'a> {
    /// First frame of each side, identifying the protocol.
    Hello,
    /// The connection is closed.
    Close,
    /// Request to open a channel to an endpoint, granting initial credit.
    ChannelRequest {
        sender_id: u64,
        frame_credit: u32,
        byte_credit: u32,
        endpoint: &'a [u8],
    },
    /// Acceptance of a channel request, granting initial credit.
    ChannelAccept {
        receiver_id: u64,
        sender_id: u64,
        frame_credit: u32,
        byte_credit: u32,
    },
    /// Rejection of a channel request.
    ChannelReject { receiver_id: u64, reason: &'a [u8] },
    /// Data sent over a channel.
    ChannelData { receiver_id: u64, payload: &'a [u8] },
    /// Additional credit for sending over a channel.
    ChannelAdjust {
        receiver_id: u64,
        frame_credit: u32,
        byte_credit: u32,
    },
    /// The sender stops receiving data over the channel.
    ChannelClose { receiver_id: u64, reason: &'a [u8] },
    /// The sender stops sending data over the channel.
    ChannelClosed { receiver_id: u64, reason: &'a [u8] },
    /// Liveness probe, which the receiver answers with `Pong`.
    Ping,
    /// Answer to `Ping`.
    Pong,
}

impl<'a> Frame<'a> {
    /// Parses a frame, returning `None` if it is invalid.
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        let (&tag, fields) = bytes.split_first()?;
        let mut fields = Fields(fields);
        let frame = match tag {
            TAG_HELLO => {
                if fields.bytes(PROTOCOL_MAGIC.len())? != PROTOCOL_MAGIC {
                    return None;
                }
                Self::Hello
            }
            TAG_CLOSE => Self::Close,
            TAG_CHANNEL_REQUEST => Self::ChannelRequest {
                sender_id: fields.u64()?,
                frame_credit: fields.u32()?,
                byte_credit: fields.u32()?,
                endpoint: fields.rest(),
            },
            TAG_CHANNEL_ACCEPT => Self::ChannelAccept {
                receiver_id: fields.u64()?,
                sender_id: fields.u64()?,
                frame_credit: fields.u32()?,
                byte_credit: fields.u32()?,
            },
            TAG_CHANNEL_REJECT => Self::ChannelReject {
                receiver_id: fields.u64()?,
                reason: fields.rest(),
            },
            TAG_CHANNEL_DATA => Self::ChannelData {
                receiver_id: fields.u64()?,
                payload: fields.rest(),
            },
            TAG_CHANNEL_ADJUST => Self::ChannelAdjust {
                receiver_id: fields.u64()?,
                frame_credit: fields.u32()?,
                byte_credit: fields.u32()?,
            },
            TAG_CHANNEL_CLOSE => Self::ChannelClose {
                receiver_id: fields.u64()?,
                reason: fields.rest(),
            },
            TAG_CHANNEL_CLOSED => Self::ChannelClosed {
                receiver_id: fields.u64()?,
                reason: fields.rest(),
            },
            TAG_PING => Self::Ping,
            TAG_PONG => Self::Pong,
            _ => return None,
        };
        Some(frame)
    }

    /// Encodes the frame into `buffer`, returning its size, or `None` if it
    /// does not fit.
    ///
    /// The payload of `ChannelData` frames is encoded after their header.
    pub fn encode(&self, buffer: &mut [u8]) -> Option<usize> {
        let mut writer = Writer { buffer, len: 0 };
        match *self {
            Self::Hello => {
                writer.u8(TAG_HELLO)?;
                writer.bytes(&PROTOCOL_MAGIC)?;
            }
            Self::Close => writer.u8(TAG_CLOSE)?,
            Self::ChannelRequest {
                sender_id,
                frame_credit,
                byte_credit,
                endpoint,
            } => {
                writer.u8(TAG_CHANNEL_REQUEST)?;
                writer.u64(sender_id)?;
                writer.u32(frame_credit)?;
                writer.u32(byte_credit)?;
                writer.bytes(endpoint)?;
            }
            Self::ChannelAccept {
                receiver_id,
                sender_id,
                frame_credit,
                byte_credit,
            } => {
                writer.u8(TAG_CHANNEL_ACCEPT)?;
                writer.u64(receiver_id)?;
                writer.u64(sender_id)?;
                writer.u32(frame_credit)?;
                writer.u32(byte_credit)?;
            }
            Self::ChannelReject {
                receiver_id,
                reason,
            } => {
                writer.u8(TAG_CHANNEL_REJECT)?;
                writer.u64(receiver_id)?;
                writer.bytes(reason)?;
            }
            Self::ChannelData {
                receiver_id,
                payload,
            } => {
                writer.u8(TAG_CHANNEL_DATA)?;
                writer.u64(receiver_id)?;
                writer.bytes(payload)?;
            }
            Self::ChannelAdjust {
                receiver_id,
                frame_credit,
                byte_credit,
            } => {
                writer.u8(TAG_CHANNEL_ADJUST)?;
                writer.u64(receiver_id)?;
                writer.u32(frame_credit)?;
                writer.u32(byte_credit)?;
            }
            Self::ChannelClose {
                receiver_id,
                reason,
            } => {
                writer.u8(TAG_CHANNEL_CLOSE)?;
                writer.u64(receiver_id)?;
                writer.bytes(reason)?;
            }
            Self::ChannelClosed {
                receiver_id,
                reason,
            } => {
                writer.u8(TAG_CHANNEL_CLOSED)?;
                writer.u64(receiver_id)?;
                writer.bytes(reason)?;
            }
            Self::Ping => writer.u8(TAG_PING)?,
            Self::Pong => writer.u8(TAG_PONG)?,
        }
        Some(writer.len)
    }
}

/// Encodes the header of a `ChannelData` frame into the first
/// [`DATA_HEADER_SIZE`] bytes of `buffer`, which must precede the payload.
pub fn encode_data_header(buffer: &mut [u8], receiver_id: u64) {
    buffer[0] = TAG_CHANNEL_DATA;
    buffer[1..DATA_HEADER_SIZE].copy_from_slice(&receiver_id.to_be_bytes());
}

/// Reader of the fields of a frame.
struct Fields<'a>(&'a [u8]);

impl<'a> Fields<'a> {
    fn bytes(&mut self, len: usize) -> Option<&'a [u8]> {
        if self.0.len() < len {
            return None;
        }
        let (bytes, rest) = self.0.split_at(len);
        self.0 = rest;
        Some(bytes)
    }

    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_be_bytes(self.bytes(4)?.try_into().ok()?))
    }

    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_be_bytes(self.bytes(8)?.try_into().ok()?))
    }

    fn rest(&mut self) -> &'a [u8] {
        core::mem::take(&mut self.0)
    }
}

/// Writer of the fields of a frame.
struct Writer<'b> {
    buffer: &'b mut [u8],
    len: usize,
}

impl Writer<'_> {
    fn bytes(&mut self, bytes: &[u8]) -> Option<()> {
        let end = self.len + bytes.len();
        self.buffer.get_mut(self.len..end)?.copy_from_slice(bytes);
        self.len = end;
        Some(())
    }

    fn u8(&mut self, value: u8) -> Option<()> {
        self.bytes(&[value])
    }

    fn u32(&mut self, value: u32) -> Option<()> {
        self.bytes(&value.to_be_bytes())
    }

    fn u64(&mut self, value: u64) -> Option<()> {
        self.bytes(&value.to_be_bytes())
    }
}
