//! TLS connections to Nexigon Hub and to the object storage of downloads.
//!
//! The device authenticates with its own certificate. It does not verify the
//! certificate of the server: checking the chain of Let's Encrypt in software
//! takes the RP2040 longer than Nexigon Hub waits for a handshake to finish.
//! Updates are signed, so that a server cannot install its own firmware.
//!
//! The RP2040 signs in software, which takes hundreds of milliseconds. The
//! duration is logged.

use core::fmt;

use embassy_net::Stack;
use embassy_net::dns::DnsQueryType;
use embassy_net::tcp::TcpSocket;
use embassy_rp::clocks::RoscRng;
use embassy_time::Duration;
use embassy_time::Instant;
use embedded_tls::Aes128GcmSha256;
use embedded_tls::Certificate;
use embedded_tls::CryptoProvider;
use embedded_tls::SignatureScheme;
use embedded_tls::TlsConfig;
use embedded_tls::TlsConnection;
use embedded_tls::TlsContext;
use embedded_tls::TlsError;
use log::info;
use p256::SecretKey;
use p256::ecdsa::DerSignature;
use p256::ecdsa::SigningKey;
use signature::SignerMut;

use crate::config;
use crate::hub;

/// Cipher suite of connections, which every TLS 1.3 server supports.
pub type CipherSuite = Aes128GcmSha256;

/// TLS connection.
pub type Connection<'a> = TlsConnection<'a, TcpSocket<'a>, CipherSuite>;

impl hub::WaitReadable for Connection<'_> {
    async fn wait_readable(&self) {
        if !self.has_buffered_data() {
            self.as_ref().wait_read_ready().await;
        }
    }
}

/// Duration without receiving anything after which the socket closes.
const SOCKET_TIMEOUT: Duration = Duration::from_secs(60);

/// Buffers of a [`Connection`].
pub struct Buffers {
    tcp_rx: [u8; 4096],
    tcp_tx: [u8; 4096],
    /// TLS records are up to 16 KiB plus overhead.
    tls_rx: [u8; 16640],
    tls_tx: [u8; 4096],
}

impl Buffers {
    pub const fn new() -> Self {
        Self {
            tcp_rx: [0; 4096],
            tcp_tx: [0; 4096],
            tls_rx: [0; 16640],
            tls_tx: [0; 4096],
        }
    }
}

/// Error opening a [`Connection`].
#[derive(Debug)]
pub enum ConnectError {
    /// The host name could not be resolved.
    Dns,
    /// The TCP connection failed.
    Tcp,
    /// The TLS handshake failed.
    Tls(TlsError),
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dns => f.write_str("failed to resolve the host name"),
            Self::Tcp => f.write_str("failed to establish a TCP connection"),
            Self::Tls(error) => write!(f, "TLS handshake failed: {error:?}"),
        }
    }
}

/// Opens a TLS connection to port 443 of `host`.
pub async fn connect<'a>(
    stack: Stack<'a>,
    buffers: &'a mut Buffers,
    host: &str,
) -> Result<Connection<'a>, ConnectError> {
    let address = *stack
        .dns_query(host, DnsQueryType::A)
        .await
        .map_err(|_| ConnectError::Dns)?
        .first()
        .ok_or(ConnectError::Dns)?;
    let mut socket = TcpSocket::new(stack, &mut buffers.tcp_rx, &mut buffers.tcp_tx);
    socket.set_timeout(Some(SOCKET_TIMEOUT));
    socket
        .connect((address, 443))
        .await
        .map_err(|_| ConnectError::Tcp)?;
    let mut tls = TlsConnection::new(socket, &mut buffers.tls_rx, &mut buffers.tls_tx);
    let tls_config = TlsConfig::new().with_server_name(host);
    tls.open(TlsContext::new(&tls_config, Provider::new()))
        .await
        .map_err(ConnectError::Tls)?;
    Ok(tls)
}

/// Provides the cryptography of a TLS connection.
///
/// Without a verifier, `embedded-tls` accepts any certificate of the server.
struct Provider {
    rng: RoscRng,
}

impl Provider {
    fn new() -> Self {
        Self {
            // The RP2040 has no true random number generator. The ring
            // oscillator is the best available source of entropy.
            rng: RoscRng,
        }
    }
}

impl CryptoProvider for Provider {
    type CipherSuite = CipherSuite;
    type Signature = DerSignature;

    fn rng(&mut self) -> impl embedded_tls::CryptoRngCore {
        &mut self.rng
    }

    fn signer(&mut self) -> Result<(impl SignerMut<DerSignature>, SignatureScheme), TlsError> {
        let key = SecretKey::from_sec1_der(config::DEVICE_KEY)
            .map_err(|_| TlsError::InvalidPrivateKey)?;
        Ok((
            TimedSigner(SigningKey::from(&key)),
            SignatureScheme::EcdsaSecp256r1Sha256,
        ))
    }

    fn client_cert(&mut self) -> Option<Certificate<impl AsRef<[u8]>>> {
        Some(Certificate::X509(config::DEVICE_CERT))
    }
}

/// Signer of the device logging the duration of signing.
struct TimedSigner(SigningKey);

impl SignerMut<DerSignature> for TimedSigner {
    fn try_sign(&mut self, message: &[u8]) -> Result<DerSignature, signature::Error> {
        let started = Instant::now();
        let result = self.0.try_sign(message);
        info!("signed handshake in {} ms", started.elapsed().as_millis());
        result
    }
}
