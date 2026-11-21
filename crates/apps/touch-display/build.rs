//! Passes the memory layout to the linker and embeds the version and the
//! configuration of the firmware.

fn main() {
    pico_build::link_firmware();
    pico_build::embed_version();
    pico_build::embed_nexigon_config();
}
