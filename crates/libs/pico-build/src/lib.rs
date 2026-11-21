//! Helpers for the build scripts of the firmware crates in `crates/apps/`.
//!
//! [`link_firmware`] passes the flash layout in `firmware.x` to the linker,
//! [`embed_version`] embeds the version of the firmware, and
//! [`embed_nexigon_config`] checks and embeds the configuration with which a
//! firmware reaches Nexigon Hub. `pico_platform::nexigon_config!` reads what
//! these helpers embed.

use std::env;
use std::fs;
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

/// Passes the flash layout of the firmware in `firmware.x` at the root of the
/// workspace to the linker.
pub fn link_firmware() {
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo sets `OUT_DIR`"));
    let layout = workspace_root().join("firmware.x");
    fs::copy(&layout, out.join("memory.x")).expect("failed to copy firmware.x");
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={}", layout.display());
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
}

/// Embeds `FIRMWARE_VERSION`, by default `dev`, as `FIRMWARE_VERSION`.
pub fn embed_version() {
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

/// Checks the configuration, the identity in `identity/` of the crate, and the
/// release key in `keys/` of the workspace.
///
/// Embeds the host name of Nexigon Hub as `NEXIGON_HUB_HOST` and the version
/// path for updates from `<CRATE>_UPDATE_PATH`, for instance,
/// `WEATHER_STATION_UPDATE_PATH`, as `FIRMWARE_UPDATE_PATH`. Each firmware has
/// its own variable, so that devices only install their own firmware.
pub fn embed_nexigon_config() {
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

    let identity = manifest_dir().join("identity");
    for file in IDENTITY_FILES {
        let path = identity.join(file);
        println!("cargo:rerun-if-changed={}", path.display());
        if !path.exists() {
            panic!(
                "`{}` is missing; run `scripts/provision.sh` first",
                path.display()
            );
        }
    }
    let key = workspace_root().join("keys/release-key.pub");
    println!("cargo:rerun-if-changed={}", key.display());
    if !key.exists() {
        panic!("`keys/release-key.pub` is missing; run `scripts/create-release-key.sh` first");
    }

    let package = env::var("CARGO_PKG_NAME").expect("Cargo sets `CARGO_PKG_NAME`");
    let variable = format!("{}_UPDATE_PATH", package.to_uppercase().replace('-', "_"));
    println!("cargo:rerun-if-env-changed={variable}");
    if let Ok(path) = env::var(&variable) {
        println!("cargo:rustc-env=FIRMWARE_UPDATE_PATH={path}");
    }
}

/// Returns the directory of the crate whose build script runs.
fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Cargo sets `CARGO_MANIFEST_DIR`"))
}

/// Returns the root of the workspace, which contains `crates/apps/<crate>`.
fn workspace_root() -> PathBuf {
    let root = manifest_dir().join("../../..");
    assert!(
        root.join("firmware.x").exists(),
        "firmware crates must be in `crates/apps/`"
    );
    root
}
