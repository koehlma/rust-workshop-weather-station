//! Downloads over HTTPS.
//!
//! Nexigon Hub hands out download URLs of assets that point to object
//! storage. A [`Download`] fetches such a URL with a `GET` request over its own
//! TLS connection and streams the body, whose length the server must state.

use core::fmt;

use embassy_net::Stack;
use embedded_io_async::Read;
use embedded_io_async::Write;

use super::tls;

/// Maximum size of the head of a response.
const MAX_HEAD_SIZE: usize = 2048;

/// Error of a download.
#[derive(Debug)]
pub enum Error {
    /// The URL is not an `https` URL.
    InvalidUrl,
    /// The connection failed.
    Connect(tls::ConnectError),
    /// Reading or writing failed.
    Io,
    /// The response is invalid or has no `Content-Length` header.
    InvalidResponse,
    /// The server responded with the given status code.
    Status(u16),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUrl => f.write_str("invalid download URL"),
            Self::Connect(error) => write!(f, "failed to connect: {error}"),
            Self::Io => f.write_str("the download was interrupted"),
            Self::InvalidResponse => f.write_str("invalid response"),
            Self::Status(status) => write!(f, "the download failed with status {status}"),
        }
    }
}

/// Download of a URL.
pub struct Download<'a> {
    connection: tls::Connection<'a>,
    /// Size of the body.
    len: usize,
    /// Bytes of the body not yet read.
    remaining: usize,
}

impl<'a> Download<'a> {
    /// Requests `url` and reads the head of the response.
    pub async fn start(
        stack: Stack<'a>,
        buffers: &'a mut tls::Buffers,
        url: &str,
    ) -> Result<Self, Error> {
        let rest = url.strip_prefix("https://").ok_or(Error::InvalidUrl)?;
        let (host, path) = match rest.find('/') {
            Some(index) => rest.split_at(index),
            None => (rest, "/"),
        };
        let mut connection = tls::connect(stack, buffers, host)
            .await
            .map_err(Error::Connect)?;

        for part in [
            "GET ",
            path,
            " HTTP/1.1\r\nHost: ",
            host,
            "\r\nConnection: close\r\n\r\n",
        ] {
            connection
                .write_all(part.as_bytes())
                .await
                .map_err(|_| Error::Io)?;
        }
        connection.flush().await.map_err(|_| Error::Io)?;

        // Read the head byte by byte to not consume the body.
        let mut head = [0; MAX_HEAD_SIZE];
        let mut len = 0;
        while !head[..len].ends_with(b"\r\n\r\n") {
            if len == head.len() {
                return Err(Error::InvalidResponse);
            }
            connection
                .read_exact(&mut head[len..len + 1])
                .await
                .map_err(|_| Error::Io)?;
            len += 1;
        }
        let head = core::str::from_utf8(&head[..len]).map_err(|_| Error::InvalidResponse)?;
        let mut lines = head.split("\r\n");
        // The status line has the form `HTTP/1.1 200 OK`.
        let status = lines
            .next()
            .and_then(|line| line.get(9..12))
            .and_then(|status| status.parse().ok())
            .ok_or(Error::InvalidResponse)?;
        if status != 200 {
            return Err(Error::Status(status));
        }
        let len = lines
            .filter_map(|line| line.split_once(':'))
            .find(|(name, _)| name.trim().eq_ignore_ascii_case("content-length"))
            .and_then(|(_, value)| value.trim().parse().ok())
            .ok_or(Error::InvalidResponse)?;
        Ok(Self {
            connection,
            len,
            remaining: len,
        })
    }

    /// Size of the body.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Reads the next part of the body into `buffer`, returning its size, or
    /// zero at the end of the body.
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if self.remaining == 0 {
            return Ok(0);
        }
        let len = buffer.len().min(self.remaining);
        let read = self
            .connection
            .read(&mut buffer[..len])
            .await
            .map_err(|_| Error::Io)?;
        if read == 0 {
            return Err(Error::Io);
        }
        self.remaining -= read;
        Ok(read)
    }
}
