//! Persistent connection to Nexigon Hub.
//!
//! A [`Session`] holds a WebSocket connection to `/api/v1/connect/ws`. Over it,
//! the multiplex protocol carries channels. The device opens an `executor`
//! channel to invoke actions. Nexigon Hub opens a `handler` channel for each
//! command it asks the device to run.
//!
//! The session runs all I/O on the task that uses it. While the device waits
//! for a command or a response, the session answers pings and serves the
//! other channel.

use core::future::Future;

use embassy_futures::select::Either;
use embassy_futures::select::select;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::Timer;
use embassy_time::with_timeout;
use embedded_io_async::Read;
use embedded_io_async::Write;
use log::warn;
use sidex_embedded::Decode;
use sidex_embedded::DecodeError;
use sidex_embedded::Encode;
use sidex_embedded::EncodeError;
use sidex_embedded::json;

use crate::config;
use crate::multiplex;
use crate::multiplex::DATA_HEADER_SIZE;
use crate::multiplex::Frame;
use crate::nexigon_api::devices::DeviceCommandDeviceFrame;
use crate::nexigon_api::devices::DeviceCommandDoneData;
use crate::nexigon_api::devices::DeviceCommandHubFrame;
use crate::nexigon_api::devices::DeviceCommandInvokeData;
use crate::nexigon_api::errors::ActionErrorKind;
use crate::nexigon_api::errors::ActionResult;
use crate::websocket;
use crate::websocket::MAX_SMALL_MESSAGE_SIZE;
use crate::websocket::WebSocket;

/// Duration without receiving a frame after which the session fails.
const READ_TIMEOUT: Duration = Duration::from_secs(30);

/// ID of the device's end of the executor channel.
const EXECUTOR_ID: u64 = 1;

/// Frames Nexigon Hub may send over the executor channel before the device
/// grants more. A response takes a few frames.
const EXECUTOR_FRAME_CREDIT: u32 = 64;

/// Frames Nexigon Hub may send over a handler channel. An invocation takes a
/// few frames.
const HANDLER_FRAME_CREDIT: u32 = 16;

/// Buffers of a [`Session`].
pub struct Buffers {
    /// A received frame, which carries at most the byte credit of a channel.
    rx: [u8; 4096 + DATA_HEADER_SIZE],
    /// A frame to send, preceded by its WebSocket header.
    tx: [u8; 2048],
    /// Response of an action.
    response: [u8; 4096],
    /// Invocation of a command.
    invocation: [u8; 1024],
}

impl Buffers {
    pub const fn new() -> Self {
        Self {
            rx: [0; 4096 + DATA_HEADER_SIZE],
            tx: [0; 2048],
            response: [0; 4096],
            invocation: [0; 1024],
        }
    }
}

/// Error of a [`Session`], after which it must not be used.
#[derive(Debug)]
pub enum Error {
    /// The WebSocket connection failed.
    WebSocket(websocket::Error),
    /// Nexigon Hub violated the multiplex protocol.
    Protocol(&'static str),
    /// Nexigon Hub sent nothing for [`READ_TIMEOUT`].
    Timeout,
    /// Nexigon Hub closed the connection.
    Closed,
    /// A request does not fit into its buffer.
    Encode(EncodeError),
    /// A response is not the expected output.
    Decode(DecodeError),
    /// Nexigon Hub rejected an action.
    Action(ActionErrorKind),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WebSocket(error) => write!(f, "WebSocket error: {error}"),
            Self::Protocol(violation) => write!(f, "protocol violation: {violation}"),
            Self::Timeout => f.write_str("Nexigon Hub stopped responding"),
            Self::Closed => f.write_str("Nexigon Hub closed the connection"),
            Self::Encode(error) => write!(f, "failed to encode a request: {error}"),
            Self::Decode(error) => write!(f, "failed to decode a response: {error}"),
            Self::Action(kind) => write!(f, "action failed: {kind:?}"),
        }
    }
}

impl From<websocket::Error> for Error {
    fn from(error: websocket::Error) -> Self {
        Self::WebSocket(error)
    }
}

/// Stream that can wait until it has data to read.
pub trait WaitReadable {
    /// Waits until a read returns data or an error without waiting for the
    /// peer. Cancelling the future must not lose data.
    fn wait_readable(&self) -> impl Future<Output = ()>;
}

/// Session with Nexigon Hub over `S`, typically a TLS connection.
pub struct Session<'a, S> {
    ws: WebSocket<'a, S>,
    hello_received: bool,
    /// Executor channel, once Nexigon Hub has accepted it.
    executor: Option<Channel>,
    /// Data received over the executor channel.
    response: Inbox<'a>,
    /// Handler channel of the current command.
    handler: Option<Channel>,
    /// Data received over the handler channel.
    invocation: Inbox<'a>,
    /// ID of the device's end of the next handler channel.
    next_handler_id: u64,
}

impl<'a, S: Read + Write + WaitReadable> Session<'a, S> {
    /// Opens a session over `stream`.
    ///
    /// `seed` initializes the generator of WebSocket masking keys.
    pub async fn open(stream: S, buffers: &'a mut Buffers, seed: u32) -> Result<Self, Error> {
        let headers = [
            ("Authorization", config::AUTHORIZATION),
            ("X-Device-Fingerprint", config::FINGERPRINT),
            // Nexigon Hub routes commands only to registered connections.
            ("X-Register-Connection", "true"),
        ];
        let ws = WebSocket::connect(
            stream,
            config::HUB_HOST,
            "/api/v1/connect/ws",
            &headers,
            &mut buffers.rx,
            &mut buffers.tx,
            seed,
        )
        .await?;
        let mut session = Self {
            ws,
            hello_received: false,
            executor: None,
            response: Inbox::new(&mut buffers.response),
            handler: None,
            invocation: Inbox::new(&mut buffers.invocation),
            next_handler_id: EXECUTOR_ID + 1,
        };
        session.send_control(Frame::Hello).await?;
        session
            .send_control(Frame::ChannelRequest {
                sender_id: EXECUTOR_ID,
                frame_credit: EXECUTOR_FRAME_CREDIT,
                byte_credit: session.response.capacity(),
                endpoint: b"executor",
            })
            .await?;
        while session.executor.is_none() {
            session.receive().await?;
        }
        Ok(session)
    }

    /// Invokes an action and decodes its output.
    pub async fn invoke<'s, O: Decode<'s> + Clone>(
        &'s mut self,
        action: &str,
        input: &impl Encode,
    ) -> Result<O, Error> {
        let executor = self.executor.ok_or(Error::Closed)?;
        // The previous response has been consumed, so return its credit.
        if self.response.frames > 0 {
            self.send_control(Frame::ChannelAdjust {
                receiver_id: executor.remote_id,
                frame_credit: self.response.frames,
                byte_credit: self.response.bytes,
            })
            .await?;
            self.response.clear();
        }

        // A request is the length-prefixed name followed by the
        // length-prefixed input.
        let name = action.as_bytes();
        let name_len =
            u16::try_from(name.len()).map_err(|_| Error::Encode(EncodeError::BufferFull))?;
        let input_start = 2 + name.len() + 4;
        let buffer = &mut self.ws.buffer()[DATA_HEADER_SIZE..];
        if buffer.len() < input_start {
            return Err(Error::Encode(EncodeError::BufferFull));
        }
        buffer[..2].copy_from_slice(&name_len.to_be_bytes());
        buffer[2..2 + name.len()].copy_from_slice(name);
        let input_len = json::to_slice(input, &mut buffer[input_start..]).map_err(Error::Encode)?;
        buffer[input_start - 4..input_start].copy_from_slice(&(input_len as u32).to_be_bytes());
        self.send_data(EXECUTOR_ID, input_start + input_len).await?;

        // A response is the length-prefixed JSON of the action's result.
        while !self.response.is_complete() {
            self.receive().await?;
        }
        let response = self
            .response
            .message()
            .ok_or(Error::Protocol("response exceeds its buffer"))?;
        match json::from_slice::<ActionResult<'s, O>>(response).map_err(Error::Decode)? {
            ActionResult::Ok(output) => Ok(output),
            ActionResult::Error(error) => {
                warn!("{action} failed: {}", error.message);
                Err(Error::Action(error.kind))
            }
        }
    }

    /// Handles the frames that have arrived without waiting for more.
    ///
    /// Call this regularly during long work, such as a download, so that the
    /// session keeps answering pings. Commands that arrive in the meantime
    /// wait for [`Session::wait_for_command`].
    pub async fn poll(&mut self) -> Result<(), Error> {
        loop {
            match select(self.ws.stream().wait_readable(), core::future::ready(())).await {
                Either::First(()) => self.receive().await?,
                Either::Second(()) => return Ok(()),
            }
        }
    }

    /// Waits until Nexigon Hub asks the device to run a command, returning
    /// `false` if `deadline` passes first.
    ///
    /// Read the command with [`Session::command`] and answer it with
    /// [`Session::complete`].
    pub async fn wait_for_command(&mut self, deadline: Instant) -> Result<bool, Error> {
        loop {
            if self.handler.is_some() && self.invocation.is_complete() {
                return Ok(true);
            }
            if Instant::now() >= deadline {
                return Ok(false);
            }
            // A read cannot be cancelled without losing data, so wait until
            // there is data before reading.
            match select(self.ws.stream().wait_readable(), Timer::at(deadline)).await {
                Either::First(()) => self.receive().await?,
                Either::Second(()) => return Ok(false),
            }
        }
    }

    /// Returns the invocation of the current command.
    pub fn command(&self) -> Result<DeviceCommandInvokeData<'_>, &'static str> {
        let invocation = self
            .invocation
            .message()
            .ok_or("the invocation exceeds the buffer of the device")?;
        match json::from_slice(invocation) {
            Ok(DeviceCommandHubFrame::Invoke(invocation)) => Ok(invocation),
            Err(_) => Err("the invocation is invalid"),
        }
    }

    /// Answers the current command and closes its channel.
    pub async fn complete(&mut self, done: &DeviceCommandDoneData<'_>) -> Result<(), Error> {
        let Some(handler) = self.handler else {
            // Nexigon Hub closed the channel in the meantime.
            return Ok(());
        };
        let frame = DeviceCommandDeviceFrame::Done(*done);
        let buffer = &mut self.ws.buffer()[DATA_HEADER_SIZE..];
        let len = json::to_slice(&frame, buffer.get_mut(4..).unwrap_or_default())
            .map_err(Error::Encode)?;
        buffer[..4].copy_from_slice(&(len as u32).to_be_bytes());
        self.send_data(handler.local_id, 4 + len).await?;
        if let Some(handler) = self.handler.take() {
            self.send_control(Frame::ChannelClosed {
                receiver_id: handler.remote_id,
                reason: b"",
            })
            .await?;
            self.send_control(Frame::ChannelClose {
                receiver_id: handler.remote_id,
                reason: b"",
            })
            .await?;
        }
        self.invocation.clear();
        Ok(())
    }

    /// Sends the first `len` bytes after the data header in the WebSocket
    /// buffer over the channel with the given local ID, once Nexigon Hub has
    /// granted enough credit.
    ///
    /// Does nothing if the channel has been closed in the meantime.
    async fn send_data(&mut self, local_id: u64, len: usize) -> Result<(), Error> {
        loop {
            let channel = if local_id == EXECUTOR_ID {
                self.executor.as_mut()
            } else {
                self.handler.as_mut().filter(|h| h.local_id == local_id)
            };
            let Some(channel) = channel else {
                return Ok(());
            };
            if channel.frame_credit > 0 && channel.byte_credit as usize >= len {
                channel.frame_credit -= 1;
                channel.byte_credit -= len as u32;
                let remote_id = channel.remote_id;
                multiplex::encode_data_header(self.ws.buffer(), remote_id);
                return Ok(self.ws.send(DATA_HEADER_SIZE + len).await?);
            }
            // Receiving uses its own buffers, so the data stays intact.
            self.receive().await?;
        }
    }

    /// Sends a frame other than `ChannelData`.
    async fn send_control(&mut self, frame: Frame<'_>) -> Result<(), Error> {
        let mut buffer = [0; MAX_SMALL_MESSAGE_SIZE];
        let len = frame
            .encode(&mut buffer)
            .ok_or(Error::Protocol("frame too large"))?;
        Ok(self.ws.send_small(&buffer[..len]).await?)
    }

    /// Receives and handles the next frame.
    async fn receive(&mut self) -> Result<(), Error> {
        with_timeout(READ_TIMEOUT, self.ws.receive())
            .await
            .map_err(|_| Error::Timeout)??;
        let frame = Frame::parse(self.ws.message()).ok_or(Error::Protocol("invalid frame"))?;
        if !self.hello_received {
            if !matches!(frame, Frame::Hello) {
                return Err(Error::Protocol("the first frame is not Hello"));
            }
            self.hello_received = true;
            return Ok(());
        }
        match frame {
            Frame::Hello => return Err(Error::Protocol("duplicate Hello")),
            Frame::Close => return Err(Error::Closed),
            Frame::Ping => self.send_control(Frame::Pong).await?,
            Frame::Pong => {}
            Frame::ChannelRequest {
                sender_id,
                frame_credit,
                byte_credit,
                endpoint,
            } => {
                if endpoint != b"handler" {
                    self.send_control(Frame::ChannelReject {
                        receiver_id: sender_id,
                        reason: b"unknown endpoint",
                    })
                    .await?;
                } else if self.handler.is_some() {
                    self.send_control(Frame::ChannelReject {
                        receiver_id: sender_id,
                        reason: b"the device is running another command",
                    })
                    .await?;
                } else {
                    let local_id = self.next_handler_id;
                    self.next_handler_id += 1;
                    self.handler = Some(Channel {
                        local_id,
                        remote_id: sender_id,
                        frame_credit,
                        byte_credit,
                    });
                    self.invocation.clear();
                    self.send_control(Frame::ChannelAccept {
                        receiver_id: sender_id,
                        sender_id: local_id,
                        frame_credit: HANDLER_FRAME_CREDIT,
                        byte_credit: self.invocation.capacity(),
                    })
                    .await?;
                }
            }
            Frame::ChannelAccept {
                receiver_id,
                sender_id,
                frame_credit,
                byte_credit,
            } => {
                if receiver_id != EXECUTOR_ID || self.executor.is_some() {
                    return Err(Error::Protocol("unexpected channel acceptance"));
                }
                self.executor = Some(Channel {
                    local_id: EXECUTOR_ID,
                    remote_id: sender_id,
                    frame_credit,
                    byte_credit,
                });
            }
            Frame::ChannelReject { reason, .. } => {
                warn!(
                    "Nexigon Hub rejected the executor channel: {}",
                    core::str::from_utf8(reason).unwrap_or("<invalid UTF-8>")
                );
                return Err(Error::Protocol("executor channel rejected"));
            }
            Frame::ChannelData {
                receiver_id,
                payload,
            } => {
                if receiver_id == EXECUTOR_ID {
                    self.response.push(payload);
                } else if self.handler.is_some_and(|h| h.local_id == receiver_id) {
                    self.invocation.push(payload);
                }
                // Data for a closed channel may still arrive and is dropped.
            }
            Frame::ChannelAdjust {
                receiver_id,
                frame_credit,
                byte_credit,
            } => {
                for channel in [&mut self.executor, &mut self.handler]
                    .into_iter()
                    .flatten()
                    .filter(|channel| channel.local_id == receiver_id)
                {
                    channel.frame_credit = channel.frame_credit.saturating_add(frame_credit);
                    channel.byte_credit = channel.byte_credit.saturating_add(byte_credit);
                }
            }
            Frame::ChannelClose { receiver_id, .. } | Frame::ChannelClosed { receiver_id, .. }
                if receiver_id == EXECUTOR_ID =>
            {
                return Err(Error::Closed);
            }
            Frame::ChannelClose { receiver_id, .. } => {
                // Nexigon Hub no longer waits for the result.
                if self.handler.is_some_and(|h| h.local_id == receiver_id) {
                    self.handler = None;
                }
            }
            Frame::ChannelClosed { receiver_id, .. } => {
                // Nexigon Hub stopped sending, which only matters before the
                // invocation is complete.
                if self.handler.is_some_and(|h| h.local_id == receiver_id)
                    && !self.invocation.is_complete()
                {
                    self.handler = None;
                }
            }
        }
        Ok(())
    }
}

/// Device's view of an open channel.
#[derive(Clone, Copy)]
struct Channel {
    /// ID of the device's end.
    local_id: u64,
    /// ID of Nexigon Hub's end.
    remote_id: u64,
    /// Frames the device may still send.
    frame_credit: u32,
    /// Bytes the device may still send.
    byte_credit: u32,
}

/// Data received over a channel, which holds one length-prefixed message.
struct Inbox<'a> {
    buffer: &'a mut [u8],
    /// Bytes stored in `buffer`.
    len: usize,
    /// Frames received, which consumed credit.
    frames: u32,
    /// Bytes received, which consumed credit.
    bytes: u32,
    /// More data arrived than fits into `buffer`.
    overflowed: bool,
}

impl<'a> Inbox<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            buffer,
            len: 0,
            frames: 0,
            bytes: 0,
            overflowed: false,
        }
    }

    /// Byte credit the device grants for the channel.
    fn capacity(&self) -> u32 {
        self.buffer.len() as u32
    }

    fn push(&mut self, payload: &[u8]) {
        self.frames += 1;
        self.bytes += payload.len() as u32;
        match self.buffer.get_mut(self.len..self.len + payload.len()) {
            Some(target) if !self.overflowed => {
                target.copy_from_slice(payload);
                self.len += payload.len();
            }
            _ => self.overflowed = true,
        }
    }

    /// Indicates whether the message is complete or has overflowed.
    fn is_complete(&self) -> bool {
        self.overflowed || self.message().is_some()
    }

    /// Returns the message once it is complete, unless it has overflowed.
    fn message(&self) -> Option<&[u8]> {
        if self.overflowed {
            return None;
        }
        let stored = &self.buffer[..self.len];
        let len = u32::from_be_bytes(stored.get(..4)?.try_into().ok()?) as usize;
        stored.get(4..4 + len)
    }

    fn clear(&mut self) {
        self.len = 0;
        self.frames = 0;
        self.bytes = 0;
        self.overflowed = false;
    }
}
