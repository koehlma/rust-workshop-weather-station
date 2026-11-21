//! Configuration and identity with which a device reaches Nexigon Hub.
//!
//! [`nexigon_config!`](crate::nexigon_config) creates the [`Config`] from what
//! `pico_build::embed_nexigon_config` and the identity of a firmware crate
//! provide at compile time.

/// Configuration and identity of a device.
pub struct Config {
    /// Host name of Nexigon Hub.
    pub hub_host: &'static str,
    /// Value of the `Authorization` header, holding the deployment token that
    /// authorizes the device to connect.
    pub authorization: &'static str,
    /// Name of the Wi-Fi network.
    pub wifi_ssid: &'static str,
    /// Password of the Wi-Fi network.
    pub wifi_password: &'static str,
    /// Fingerprint identifying the device in its project.
    pub fingerprint: &'static str,
    /// Self-signed certificate of the device in DER format.
    pub device_cert: &'static [u8],
    /// Private key of the device certificate in SEC1 DER format.
    pub device_key: &'static [u8],
    /// Public key with which releases are signed, an uncompressed P-256 point.
    pub release_key: &'static [u8],
    /// Version of the firmware, with which releases tag their package version.
    pub firmware_version: &'static str,
    /// Version path from which the device installs updates unless the
    /// `dev.nexigon.ota.config` property names another, for instance,
    /// `default/rp-sensors/latest`.
    pub update_path: Option<&'static str>,
}

/// Creates the [`Config`] of the calling firmware crate.
///
/// The build script of the crate must call `pico_build::embed_version` and
/// `pico_build::embed_nexigon_config`. The identity comes from `identity/` of
/// the crate and the release key from `keys/` of the workspace.
#[macro_export]
macro_rules! nexigon_config {
    () => {
        $crate::nexigon::Config {
            hub_host: env!("NEXIGON_HUB_HOST"),
            authorization: concat!("Bearer ", env!("NEXIGON_DEPLOYMENT_TOKEN")),
            wifi_ssid: env!("WIFI_SSID"),
            wifi_password: env!("WIFI_PASSWORD"),
            fingerprint: include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/identity/fingerprint.txt"
            )),
            device_cert: include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/identity/device-cert.der"
            )),
            device_key: include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/identity/device-key.der"
            )),
            release_key: include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../keys/release-key.pub"
            )),
            firmware_version: env!("FIRMWARE_VERSION"),
            update_path: option_env!("FIRMWARE_UPDATE_PATH"),
        }
    };
}
