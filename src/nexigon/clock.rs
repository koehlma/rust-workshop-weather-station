//! Wall-clock time, synchronized with SNTP.
//!
//! Events carry the time they were emitted.

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

use embassy_net::Stack;
use embassy_net::dns::DnsQueryType;
use embassy_net::udp::PacketMetadata;
use embassy_net::udp::UdpSocket;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::with_timeout;
use sidex_embedded::types::Timestamp;

/// NTP server to synchronize with.
const NTP_SERVER: &str = "pool.ntp.org";

/// Local UDP port of SNTP requests.
const LOCAL_PORT: u16 = 50123;

/// Seconds from the NTP epoch (1900) to the Unix epoch (1970).
const NTP_TO_UNIX_SECONDS: u32 = 2_208_988_800;

/// Unix time at boot in seconds, or `0` before the first synchronization.
static BOOT_UNIX_SECONDS: AtomicU32 = AtomicU32::new(0);

/// Error synchronizing the clock.
#[derive(Debug)]
pub enum SyncError {
    /// The NTP server could not be resolved.
    Dns,
    /// The request could not be sent or received.
    Network,
    /// The server did not respond in time.
    Timeout,
    /// The response is not a valid SNTP response.
    InvalidResponse,
}

/// Synchronizes the clock with [`NTP_SERVER`].
pub async fn sync(stack: Stack<'_>) -> Result<(), SyncError> {
    let address = *stack
        .dns_query(NTP_SERVER, DnsQueryType::A)
        .await
        .map_err(|_| SyncError::Dns)?
        .first()
        .ok_or(SyncError::Dns)?;

    let mut rx_meta = [PacketMetadata::EMPTY; 1];
    let mut rx_buffer = [0; 64];
    let mut tx_meta = [PacketMetadata::EMPTY; 1];
    let mut tx_buffer = [0; 64];
    let mut socket = UdpSocket::new(
        stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );
    socket.bind(LOCAL_PORT).map_err(|_| SyncError::Network)?;

    // Leap indicator 0, version 4, mode 3 (client).
    let mut request = [0; 48];
    request[0] = 0x23;
    socket
        .send_to(&request, (address, 123))
        .await
        .map_err(|_| SyncError::Network)?;
    let mut response = [0; 48];
    let (len, from) = with_timeout(Duration::from_secs(5), socket.recv_from(&mut response))
        .await
        .map_err(|_| SyncError::Timeout)?
        .map_err(|_| SyncError::Network)?;
    // A server response has mode 4 and a nonzero stratum and transmit time.
    let transmit_seconds =
        u32::from_be_bytes([response[40], response[41], response[42], response[43]]);
    if len < 48
        || from.endpoint.addr != address
        || response[0] & 0x07 != 4
        || response[1] == 0
        || transmit_seconds < NTP_TO_UNIX_SECONDS
    {
        return Err(SyncError::InvalidResponse);
    }

    let unix_seconds = transmit_seconds - NTP_TO_UNIX_SECONDS;
    let boot = unix_seconds - Instant::now().as_secs() as u32;
    BOOT_UNIX_SECONDS.store(boot, Ordering::Relaxed);
    Ok(())
}

/// Returns the current Unix time in seconds, or `None` before the clock has
/// been synchronized.
pub fn unix_seconds() -> Option<u64> {
    match BOOT_UNIX_SECONDS.load(Ordering::Relaxed) {
        0 => None,
        boot => Some(u64::from(boot) + Instant::now().as_secs()),
    }
}

/// Returns the current time, or `None` before the clock has been
/// synchronized.
pub fn now() -> Option<Timestamp> {
    Timestamp::from_unix(unix_seconds()? as i64, 0)
}
