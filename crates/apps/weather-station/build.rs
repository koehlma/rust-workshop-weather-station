//! Passes the memory layout to the linker and embeds the version and, with
//! the `nexigon` feature, the configuration of the firmware.

fn main() {
    pico_build::link_firmware();
    pico_build::embed_version();
    if std::env::var_os("CARGO_FEATURE_NEXIGON").is_some() {
        pico_build::embed_nexigon_config();
    }
}
