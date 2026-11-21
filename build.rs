//! Passes the memory layout to the linker and checks the configuration.
//!
//! The firmware embeds its version at compile time. With the `nexigon`
//! feature, it also embeds its configuration from environment variables, the
//! device identity from `identity/`, and the release key from `keys/`, so all
//! of them must be present.

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Environment variables that configure the connection to Nexigon Hub.
const NEXIGON_VARIABLES: &[&str] = &[
    "NEXIGON_HUB_URL",
    "NEXIGON_DEPLOYMENT_TOKEN",
    "WIFI_SSID",
    "WIFI_PASSWORD",
];

/// Files of the device identity created by `scripts/provision.sh`.
const IDENTITY_FILES: &[&str] = &["device-cert.der", "device-key.der", "fingerprint.txt"];

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo sets `OUT_DIR`"));
    fs::write(out.join("memory.x"), include_bytes!("firmware.x"))
        .expect("failed to write memory.x");
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=firmware.x");
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");

    if env::var_os("CARGO_FEATURE_NEXIGON").is_some() {
        check_nexigon_config();
    }

    // Releases tag their package version with the firmware version, so it must
    // be a valid tag.
    println!("cargo:rerun-if-env-changed=FIRMWARE_VERSION");
    let version = env::var("FIRMWARE_VERSION").unwrap_or_else(|_| "dev".to_owned());
    let valid = version.len() <= 128
        && version.starts_with(|c: char| c.is_ascii_alphanumeric())
        && version
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "._+~-".contains(c));
    if !valid {
        panic!("`FIRMWARE_VERSION` must be a valid tag of a package version");
    }
    println!("cargo:rustc-env=FIRMWARE_VERSION={version}");
}

/// Checks the configuration, identity, and release key of the `nexigon`
/// feature and derives the host name of Nexigon Hub.
fn check_nexigon_config() {
    for variable in NEXIGON_VARIABLES {
        println!("cargo:rerun-if-env-changed={variable}");
        if env::var(variable).is_err() {
            panic!("environment variable `{variable}` is not set; see README.md");
        }
    }
    let url = env::var("NEXIGON_HUB_URL").expect("checked above");
    let Some(host) = url.strip_prefix("https://") else {
        panic!("`NEXIGON_HUB_URL` must start with `https://`");
    };
    println!(
        "cargo:rustc-env=NEXIGON_HUB_HOST={}",
        host.trim_end_matches('/')
    );

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for file in IDENTITY_FILES {
        let path = root.join("identity").join(file);
        println!("cargo:rerun-if-changed={}", path.display());
        if !path.exists() {
            panic!("`identity/{file}` is missing; run `scripts/provision.sh` first");
        }
    }
    let key = root.join("keys/release-key.pub");
    println!("cargo:rerun-if-changed={}", key.display());
    if !key.exists() {
        panic!("`keys/release-key.pub` is missing; run `scripts/create-release-key.sh` first");
    }

    println!("cargo:rerun-if-env-changed=NEXIGON_UPDATE_PATH");
}
