//! WebSocket client as specified by RFC 6455.
//!
//! The client implements what the connection to Nexigon Hub needs: the opening
//! handshake, unfragmented binary messages, and answers to pings. It does not
//! check the `Sec-WebSocket-Accept` header of the server's response, which
//! would need SHA-1. The connection runs over TLS, which authenticates it.

use core::fmt;
use core::fmt::Write as _;

use embedded_io_async::Error as _;
use embedded_io_async::ErrorKind;
use embedded_io_async::Read;
use embedded_io_async::ReadExactError;
use embedded_io_async::Write;

/// Maximum size of the header of an outgoing frame: two bytes, an extended
/// length of up to eight bytes, and the masking key.
const MAX_HEADER_SIZE: usize = 14;

/// Maximum size of a message sent with [`WebSocket::send_small`], which is
/// also the maximum payload of a control frame.
pub const MAX_SMALL_MESSAGE_SIZE: usize = 125;

const OPCODE_CONTINUATION: u8 = 0x0;
const OPCODE_BINARY: u8 = 0x2;
const OPCODE_CLOSE: u8 = 0x8;
const OPCODE_PING: u8 = 0x9;
const OPCODE_PONG: u8 = 0xA;

/// Error of a WebSocket connection.
#[derive(Debug)]
pub enum Error {
    /// Reading from or writing to the stream failed.
    Io(ErrorKind),
    /// The stream ended.
    Eof,
    /// The server rejected the handshake with the given HTTP status code.
    Rejected(u16),
    /// The server violated the protocol.
    Protocol(&'static str),
    /// A message does not fit into its buffer.
    TooLarge,
    /// The server closed the connection.
    Closed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(kind) => write!(f, "I/O error: {kind:?}"),
            Self::Eof => f.write_str("the connection ended"),
            Self::Rejected(status) => write!(f, "handshake rejected with status {status}"),
            Self::Protocol(violation) => write!(f, "protocol violation: {violation}"),
            Self::TooLarge => f.write_str("message too large"),
            Self::Closed => f.write_str("the server closed the connection"),
        }
    }
}

impl<E: embedded_io_async::Error> From<ReadExactError<E>> for Error {
    fn from(error: ReadExactError<E>) -> Self {
        match error {
            ReadExactError::UnexpectedEof => Self::Eof,
            ReadExactError::Other(error) => Self::Io(error.kind()),
        }
    }
}

/// WebSocket connection over a stream.
pub struct WebSocket<'a, S> {
    stream: S,
    /// Payload of the last received message.
    rx: &'a mut [u8],
    /// Length of the last received message.
    received: usize,
    /// Header space followed by the payload of the next message.
    tx: &'a mut [u8],
    /// State of the generator of masking keys.
    mask_state: u32,
}

impl<'a, S: Read + Write> WebSocket<'a, S> {
    /// Opens a WebSocket connection to `path` at `host`, sending `headers`
    /// with the request.
    ///
    /// Received messages must fit into `rx`, and sent messages into `tx`
    /// minus the space for a frame header. `seed` initializes the generator
    /// of masking keys.
    pub async fn connect(
        stream: S,
        host: &str,
        path: &str,
        headers: &[(&str, &str)],
        rx: &'a mut [u8],
        tx: &'a mut [u8],
        seed: u32,
    ) -> Result<Self, Error> {
        let mut ws = Self {
            stream,
            rx,
            received: 0,
            tx,
            mask_state: seed | 1,
        };

        let mut key = [0; 16];
        for chunk in key.chunks_mut(4) {
            chunk.copy_from_slice(&ws.next_mask());
        }
        let mut encoded_key = [0; 24];
        encode_base64(&key, &mut encoded_key);
        let encoded_key = core::str::from_utf8(&encoded_key).expect("Base64 is ASCII");

        let mut request = Cursor::new(&mut ws.tx[..]);
        write!(
            request,
            "GET {path} HTTP/1.1\r\n\
             Host: {host}\r\n\
             Upgrade: websocket\r\n\
             Connection: Upgrade\r\n\
             Sec-WebSocket-Key: {encoded_key}\r\n\
             Sec-WebSocket-Version: 13\r\n"
        )
        .map_err(|_| Error::TooLarge)?;
        for (name, value) in headers {
            write!(request, "{name}: {value}\r\n").map_err(|_| Error::TooLarge)?;
        }
        request.write_str("\r\n").map_err(|_| Error::TooLarge)?;
        let len = request.len;
        ws.stream
            .write_all(&ws.tx[..len])
            .await
            .map_err(|error| Error::Io(error.kind()))?;
        ws.stream
            .flush()
            .await
            .map_err(|error| Error::Io(error.kind()))?;

        // Read the response byte by byte to not consume the first frame.
        let mut len = 0;
        while !ws.rx[..len].ends_with(b"\r\n\r\n") {
            if len == ws.rx.len() {
                return Err(Error::TooLarge);
            }
            ws.stream.read_exact(&mut ws.rx[len..len + 1]).await?;
            len += 1;
        }
        // The status line has the form `HTTP/1.1 101 Switching Protocols`.
        let status = ws.rx[..len]
            .get(9..12)
            .and_then(|status| core::str::from_utf8(status).ok())
            .and_then(|status| status.parse().ok())
            .ok_or(Error::Protocol("invalid HTTP response"))?;
        if status != 101 {
            return Err(Error::Rejected(status));
        }
        Ok(ws)
    }

    /// Receives the next binary message, answering pings in between.
    ///
    /// The message is available with [`WebSocket::message`].
    pub async fn receive(&mut self) -> Result<(), Error> {
        loop {
            let mut header = [0; 2];
            self.stream.read_exact(&mut header).await?;
            let fin = header[0] & 0x80 != 0;
            let opcode = header[0] & 0x0F;
            if header[1] & 0x80 != 0 {
                return Err(Error::Protocol("masked frame from the server"));
            }
            let len = match header[1] & 0x7F {
                126 => {
                    let mut len = [0; 2];
                    self.stream.read_exact(&mut len).await?;
                    usize::from(u16::from_be_bytes(len))
                }
                127 => {
                    let mut len = [0; 8];
                    self.stream.read_exact(&mut len).await?;
                    usize::try_from(u64::from_be_bytes(len)).map_err(|_| Error::TooLarge)?
                }
                len => usize::from(len),
            };
            if !fin || opcode == OPCODE_CONTINUATION {
                return Err(Error::Protocol("fragmented message"));
            }
            if len > self.rx.len() {
                return Err(Error::TooLarge);
            }
            self.stream.read_exact(&mut self.rx[..len]).await?;
            match opcode {
                OPCODE_BINARY => {
                    self.received = len;
                    return Ok(());
                }
                OPCODE_PING => {
                    let mut payload = [0; MAX_SMALL_MESSAGE_SIZE];
                    let payload = payload
                        .get_mut(..len)
                        .ok_or(Error::Protocol("ping too large"))?;
                    payload.copy_from_slice(&self.rx[..len]);
                    self.send_frame_small(OPCODE_PONG, payload).await?;
                }
                OPCODE_PONG => {}
                OPCODE_CLOSE => return Err(Error::Closed),
                _ => return Err(Error::Protocol("unexpected message type")),
            }
        }
    }

    /// Underlying stream.
    pub fn stream(&self) -> &S {
        &self.stream
    }

    /// Payload of the last received message.
    pub fn message(&self) -> &[u8] {
        &self.rx[..self.received]
    }

    /// Buffer for the payload of the next message sent with
    /// [`WebSocket::send`].
    pub fn buffer(&mut self) -> &mut [u8] {
        &mut self.tx[MAX_HEADER_SIZE..]
    }

    /// Sends the first `len` bytes of [`WebSocket::buffer`] as a binary
    /// message.
    pub async fn send(&mut self, len: usize) -> Result<(), Error> {
        let mask = self.next_mask();
        let (header, payload) = self.tx.split_at_mut(MAX_HEADER_SIZE);
        let payload = payload.get_mut(..len).ok_or(Error::TooLarge)?;
        let start = write_header(header, OPCODE_BINARY, len, mask);
        mask_payload(payload, mask);
        write(&mut self.stream, &self.tx[start..MAX_HEADER_SIZE + len]).await
    }

    /// Sends a binary message of up to [`MAX_SMALL_MESSAGE_SIZE`] bytes
    /// without touching [`WebSocket::buffer`].
    pub async fn send_small(&mut self, message: &[u8]) -> Result<(), Error> {
        self.send_frame_small(OPCODE_BINARY, message).await
    }

    async fn send_frame_small(&mut self, opcode: u8, payload: &[u8]) -> Result<(), Error> {
        let mut frame = [0; MAX_HEADER_SIZE + MAX_SMALL_MESSAGE_SIZE];
        let (header, buffer) = frame.split_at_mut(MAX_HEADER_SIZE);
        let buffer = buffer.get_mut(..payload.len()).ok_or(Error::TooLarge)?;
        buffer.copy_from_slice(payload);
        let mask = self.next_mask();
        let start = write_header(header, opcode, payload.len(), mask);
        mask_payload(buffer, mask);
        write(
            &mut self.stream,
            &frame[start..MAX_HEADER_SIZE + payload.len()],
        )
        .await
    }

    /// Generates the next masking key with xorshift.
    ///
    /// Masking prevents attacks on intermediaries of plaintext connections.
    /// Over TLS, it needs no cryptographic strength.
    fn next_mask(&mut self) -> [u8; 4] {
        let mut x = self.mask_state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.mask_state = x;
        x.to_be_bytes()
    }
}

/// Writes the header of a masked frame to the end of `header` and returns the
/// index of its first byte.
fn write_header(header: &mut [u8], opcode: u8, len: usize, mask: [u8; 4]) -> usize {
    let mut frame_header = [0; MAX_HEADER_SIZE];
    frame_header[0] = 0x80 | opcode;
    let mut size = 2;
    if len < 126 {
        frame_header[1] = 0x80 | len as u8;
    } else if let Ok(len) = u16::try_from(len) {
        frame_header[1] = 0x80 | 126;
        frame_header[2..4].copy_from_slice(&len.to_be_bytes());
        size = 4;
    } else {
        frame_header[1] = 0x80 | 127;
        frame_header[2..10].copy_from_slice(&(len as u64).to_be_bytes());
        size = 10;
    }
    frame_header[size..size + 4].copy_from_slice(&mask);
    size += 4;
    let start = header.len() - size;
    header[start..].copy_from_slice(&frame_header[..size]);
    start
}

fn mask_payload(payload: &mut [u8], mask: [u8; 4]) {
    for (i, byte) in payload.iter_mut().enumerate() {
        *byte ^= mask[i % 4];
    }
}

async fn write<S: Write>(stream: &mut S, bytes: &[u8]) -> Result<(), Error> {
    stream
        .write_all(bytes)
        .await
        .map_err(|error| Error::Io(error.kind()))?;
    stream
        .flush()
        .await
        .map_err(|error| Error::Io(error.kind()))
}

/// Encodes 16 bytes with Base64 into 24 characters.
fn encode_base64(input: &[u8; 16], output: &mut [u8; 24]) {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for (i, chunk) in input.chunks(3).enumerate() {
        let bytes = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let group = u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]);
        for j in 0..4 {
            output[i * 4 + j] = ALPHABET[(group >> (18 - 6 * j) & 0x3F) as usize];
        }
    }
    // The last chunk has one byte, so two characters are padding.
    output[22] = b'=';
    output[23] = b'=';
}

/// Writer of formatted text into a byte buffer.
struct Cursor<'b> {
    buffer: &'b mut [u8],
    len: usize,
}

impl<'b> Cursor<'b> {
    fn new(buffer: &'b mut [u8]) -> Self {
        Self { buffer, len: 0 }
    }
}

impl fmt::Write for Cursor<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end = self.len + s.len();
        self.buffer
            .get_mut(self.len..end)
            .ok_or(fmt::Error)?
            .copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}
