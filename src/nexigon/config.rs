//! Configuration and identity with which the device reaches Nexigon Hub,
//! embedded at compile time.

/// Host name of Nexigon Hub, derived from `NEXIGON_HUB_URL`.
pub const HUB_HOST: &str = env!("NEXIGON_HUB_HOST");

/// Value of the `Authorization` header, holding the deployment token that
/// authorizes the device to connect.
pub const AUTHORIZATION: &str = concat!("Bearer ", env!("NEXIGON_DEPLOYMENT_TOKEN"));

/// Name of the Wi-Fi network.
pub const WIFI_SSID: &str = env!("WIFI_SSID");

/// Password of the Wi-Fi network.
pub const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

/// Fingerprint identifying the device in its project.
pub const FINGERPRINT: &str = include_str!("../../identity/fingerprint.txt");

/// Self-signed certificate of the device in DER format.
pub const DEVICE_CERT: &[u8] = include_bytes!("../../identity/device-cert.der");

/// Private key of the device certificate in SEC1 DER format.
pub const DEVICE_KEY: &[u8] = include_bytes!("../../identity/device-key.der");

/// Version path from which the device installs updates unless the
/// `dev.nexigon.ota.config` property names another, from the optional
/// `NEXIGON_UPDATE_PATH`, for instance, `default/rp-test/latest`.
pub const UPDATE_PATH: Option<&str> = option_env!("NEXIGON_UPDATE_PATH");

/// Public key with which releases are signed, an uncompressed P-256 point.
pub const RELEASE_KEY: &[u8] = include_bytes!("../../keys/release-key.pub");
