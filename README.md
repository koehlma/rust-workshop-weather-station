# Weather Station on the Raspberry Pi Pico W

Firmware for two devices built on the Raspberry Pi Pico W (RP2040). It is written in Rust with [Embassy](https://embassy.dev) and runs without an operating system or an allocator.

| Crate                         | Device                                                                                                  |
| ----------------------------- | ------------------------------------------------------------------------------------------------------- |
| `crates/apps/weather-station` | Weather station that measures temperature, humidity, UV radiation, and air pressure and shows them      |
| `crates/apps/touch-display`   | Waveshare Pico-ResTouch-LCD-2.8 that stores the measurements on a microSD card and shows the latest one |

Both devices connect to Nexigon Hub, each with its own identity. Every 30 seconds, the weather station measures and sends the measurement directly to the touch display over the local network. Every 30 seconds, a device publishes a heartbeat event, the weather station its latest measurement, and the touch display the number of stored measurements. Device properties in Nexigon Hub configure the IP addresses of the devices. The devices run commands that Nexigon Hub sends and install updates that Nexigon Hub rolls out. A bootloader swaps in updates and rolls back those that fail. The weather station can also be built without Nexigon, in which case it logs its measurements to the USB serial port.

Each device holds one persistent connection to Nexigon Hub, which authenticates it with a deployment token, a device fingerprint, and a client certificate. On its first connection, Nexigon Hub registers the device in the project of the deployment token.

## Requirements

- A Raspberry Pi Pico W for each device. The Wi-Fi chip of the Pico W joins 2.4 GHz networks only.
- [Rust](https://rustup.rs). `rust-toolchain.toml` selects the toolchain and installs the `thumbv6m-none-eabi` target and `llvm-objcopy`.
- `python3`, which creates the UF2 files.
- Optionally, [`mise`](https://mise.jdx.dev) to run the tasks in `mise.toml`.

For Nexigon, you also need:

- A 2.4 GHz Wi-Fi network.
- `openssl`, which creates keys and signs releases.
- For releases, the [Nexigon CLI](https://docs.nexigon.dev) and an organization API token.

The weather station needs:

- A Joy-IT explorer board (RB-P-XPLR) for the Pico W. The firmware uses its DHT11 sensor on GP0 and its 1.8" display, an ST7735 on SPI0. Turn on their switches on the board.
- A Joy-IT SEN-UV01 UV sensor connected to GP26, the first analog input. Connect its VCC to 3V3 (OUT), GND to GND, and OUT to GP26. Power it from 3.3 V, the most that the analog inputs of the Pico W accept.
- A BMP280 pressure sensor, for instance, the Joy-IT KY-052, connected to I2C0. Connect its +V to 3V3 (OUT), GND to GND, SDA to GP4, and SCL to GP5. Tie CSB to 3V3, which selects I2C, and SDO to 3V3, which selects the address 0x77.

The touch display needs a Waveshare Pico-ResTouch-LCD-2.8 and a microSD card with a FAT16 or FAT32 file system, the file system of most cards up to 32 GB. Its display is an ST7789 with 320 × 240 pixels on SPI1, which it shares with its XPT2046 touch controller and its microSD slot. Plug in the Pico W with its USB port on the side of the microSD slot.

To store measurements, both devices need Nexigon and the same Wi-Fi network.

## Configure Nexigon

Skip this section to build the weather station without Nexigon.

Create the key with which releases are signed:

```sh
mise run create-release-key
```

This writes a P-256 key pair to `keys/`. The firmware embeds the public key and installs only updates signed with the private key. All devices that receive the same updates need the same release key. Keep the private key private: whoever has it can install their own firmware.

Create the identity of each device, for instance, of the weather station:

```sh
mise run provision weather-station
```

This writes a P-256 key, a self-signed certificate for it, and a fingerprint to `crates/apps/weather-station/identity/`. Every board needs its own identity. Keep it private: whoever has it can act as the device.

Configure the firmware in `mise.local.toml`, which Git ignores:

```toml
[env]
NEXIGON_HUB_URL = "https://nexigon.example.com"
NEXIGON_DEPLOYMENT_TOKEN = "deployment_..."
WIFI_SSID = "..."
WIFI_PASSWORD = "..."
# Optional: version paths from which the devices install updates.
WEATHER_STATION_UPDATE_PATH = "default/rp-test/latest"
TOUCH_DISPLAY_UPDATE_PATH = "default/rp-display/latest"
# Only needed for releases.
NEXIGON_ORG_TOKEN = "org_sk_..."
```

Each crate has its own update path, named after the crate, so that a device only installs its own firmware. Without `mise`, set these environment variables before running the scripts in `scripts/`. A firmware embeds the configuration, its identity, and the public release key when it is built.

## Build and Flash the Firmware

Build the bootloader and the firmware of a crate:

```sh
mise run build weather-station
mise run build touch-display
```

Or build the weather station without Nexigon, which needs no configuration:

```sh
mise run build weather-station -- --no-default-features
```

The build writes the UF2 file to `target/<crate>/<crate>.uf2`, for instance, `target/touch-display/touch-display.uf2`. The version of the firmware is the time of the build, for instance, `20261120-163751`. Set `FIRMWARE_VERSION` to choose another one.

The UF2 file contains the deployment token, the Wi-Fi password, and the private key of the device. Do not share it.

To flash the bootloader and the firmware, hold the BOOTSEL button while connecting the Pico W over USB. It appears as a drive named `RPI-RP2`. Copy the UF2 file to the drive. The Pico W restarts with the new firmware. With Nexigon, the device afterward receives new versions through Nexigon Hub.

The firmware logs to a USB serial port, for instance, `/dev/ttyACM0` on Linux or `/dev/cu.usbmodem*` on macOS. Open it with a serial terminal, for instance, `tio /dev/ttyACM0`. The log of the weather station with Nexigon looks as follows:

```text
starting firmware 20261120-163751
joining Wi-Fi network ...
network is up: Some(192.168.1.23/24)
synchronized clock: Some(2026-11-20T14:35:00Z)
connecting to Nexigon Hub at nexigon.example.com
signed handshake in ... ms
connected in ... ms
authenticated as device d_...
published commands
measured 21.0 °C, 45 % humidity, UV index 0.2 (0.021 V), 1013.2 hPa
published heartbeat and environment event
```

Without Nexigon, only the measurements follow the first line. The touch display logs `published heartbeat and storage event` instead of the measurements. The events appear in the event log of the device in Nexigon Hub.

## Weather Station

The display of the explorer board shows the latest temperature, humidity, UV index, and air pressure, with dashes for a sensor that failed.

With Nexigon, the weather station publishes two events every 30 seconds:

| Category      | Body                                                                                     |
| ------------- | ---------------------------------------------------------------------------------------- |
| `heartbeat`   | `{"uptimeSeconds": 42, "firmwareVersion": "20261120-163751", "ipAddress": "10.10.2.51"}` |
| `environment` | Latest measurement of the sensors                                                        |

Both devices publish the same heartbeat. Its `ipAddress` is the address of the device in the Wi-Fi network.

The body of an `environment` event looks as follows:

```json
{
  "temperatureCelsius": 21.0,
  "humidityPercent": 45.0,
  "uvIndex": 0.21,
  "uvVolts": 0.021,
  "pressureHectopascals": 1013.2
}
```

| Field                  | Sensor | Value                           |
| ---------------------- | ------ | ------------------------------- |
| `temperatureCelsius`   | DHT11  | Temperature in degrees Celsius  |
| `humidityPercent`      | DHT11  | Relative humidity in percent    |
| `uvIndex`              | UV     | Approximate UV index            |
| `uvVolts`              | UV     | Output voltage of the UV sensor |
| `pressureHectopascals` | BMP280 | Air pressure in hectopascals    |

`uvIndex` is `uvVolts` divided by 0.1 V. This approximation suits sensors with the GUVA-S12SD photodiode but is not calibrated. The BMP280 also measures the temperature, which the device does not publish because the DHT11 already does. The pressure is the absolute pressure at the height of the device, which is lower than the pressure at sea level that weather reports quote.

If a sensor fails, the device logs a warning and leaves out its fields. If all fail, it publishes only the heartbeat and sends nothing to the touch display. The DHT11 and the BMP280 fail when they do not answer, for instance, because none is connected. A missing UV sensor goes unnoticed, because the open analog input yields arbitrary voltages.

## Touch Display

The touch display receives the measurements of the weather station, stores them on the microSD card, and shows the latest one. The weather station sends every measurement as a UDP datagram to port 4242 of the touch display. The datagram holds the body of an `environment` event. The weather station measures and sends even while it is not connected to Nexigon Hub.

Two device properties configure the IP addresses. Each device reads its property before every heartbeat, so a change takes effect within 30 seconds:

| Device          | Property                 | Value                              |
| --------------- | ------------------------ | ---------------------------------- |
| Weather station | `weather-station.config` | `{"displayAddress": "10.10.2.28"}` |
| Touch display   | `touch-display.config`   | `{"sensorAddress": "10.10.2.51"}`  |

`displayAddress` is the address to which the weather station sends its measurements. `sensorAddress` is the only address from which the touch display accepts measurements. The heartbeats of the devices report their addresses. Set the properties in Nexigon Hub, for instance, with the Nexigon CLI:

```sh
nexigon-cli devices properties set --protected true <device-id> weather-station.config '{"displayAddress": "10.10.2.28"}'
nexigon-cli devices properties set --protected true <device-id> touch-display.config '{"sensorAddress": "10.10.2.51"}'
```

Only Nexigon Hub can change a protected property, not the device itself. Without its property, the weather station sends nothing, and the touch display rejects all datagrams.

The touch display appends every accepted measurement to `WEATHER.LOG` in the root directory of the first partition of the card, one JSON object per line:

```json
{
  "receivedAt": "2026-11-21T13:43:50Z",
  "sender": "10.10.2.51:22972",
  "measurement": {
    "temperatureCelsius": 8.9,
    "humidityPercent": 60.0,
    "uvIndex": 0.2,
    "uvVolts": 0.02,
    "pressureHectopascals": 958.1
  }
}
```

`receivedAt` is missing while the clock of the touch display is not synchronized. The touch display closes the file after every line, so the card can be removed between measurements. After a failure, for instance, because no card is inserted, it starts the card again with the next measurement.

With every heartbeat, the touch display publishes a `storage` event with counts since its start:

```json
{ "storedMeasurements": 4, "rejectedDatagrams": 2, "failedWrites": 0 }
```

| Field                | Count                                                   |
| -------------------- | ------------------------------------------------------- |
| `storedMeasurements` | Measurements stored on the card                         |
| `rejectedDatagrams`  | Datagrams from another address or without a measurement |
| `failedWrites`       | Measurements that could not be stored on the card       |

The display runs in landscape, as in the example code of Waveshare. The firmware keeps the touch controller deselected and does not use it.

| Signal                  | GPIO |
| ----------------------- | ---- |
| Display data or command | GP8  |
| Display chip select     | GP9  |
| SPI1 clock              | GP10 |
| SPI1 data out           | GP11 |
| SPI1 data in            | GP12 |
| Display backlight       | GP13 |
| Display reset           | GP15 |
| Touch chip select       | GP16 |
| microSD chip select     | GP22 |

## Commands

With Nexigon, both devices offer three commands, which Nexigon Hub lists on the page of the device:

| Command             | Input          | Output                               |
| ------------------- | -------------- | ------------------------------------ |
| `set-led`           | `{"on": true}` | None                                 |
| `get-status`        | None           | `{"uptimeSeconds": 42, "led": true}` |
| `nexigon.ota.check` | None           | None                                 |

`set-led` turns the LED of the Pico W on or off. `get-status` reports how long the device has been running and whether the LED is on. `nexigon.ota.check` checks for an update right away.

To add a command, describe it in `publish_manifest` and handle it in `execute`, both in `crates/libs/pico-platform/src/nexigon/commands.rs`. The device runs one command at a time, between its other work, so a command must finish quickly. Nexigon Hub stops waiting for a pong after 15 seconds.

## Release an Update

Releases are versions of a package in a repository of Nexigon Hub. Each crate has its own package: `rp-test` for the weather station and `rp-display` for the touch display. Build and publish a new version, for instance, of the weather station:

```sh
mise run build weather-station
mise run release weather-station default/rp-test
```

The release uploads `target/weather-station/firmware.bin` and its signature `firmware.bin.sig` as assets of a new version of the package `rp-test` in the repository `default`. It tags the version with the firmware version, a locked tag, and moves the tag `latest` to it.

Devices built with `WEATHER_STATION_UPDATE_PATH = "default/rp-test/latest"` install the version that the tag `latest` points to. The `dev.nexigon.ota.config` property of a device overrides the defaults built into the firmware. For instance, it pins a version with `{"path": "default/rp-test/20261120-163751"}` or holds updates with `{"hold": true}`.

The device checks for updates when it connects, every 5 minutes, and when it receives the command `nexigon.ota.check`. It reports its progress in the `dev.nexigon.ota.status` property. An update takes a few minutes:

```text
installing version latest
downloading 554552 bytes
...
installed version latest, rebooting
starting firmware 20261120-170000
running a new firmware, which commits itself once it reaches Nexigon Hub
...
update completed
```

## How It Works

The repository is a Cargo workspace. All firmware crates share the bootloader, the flash layout in `firmware.x`, and the platform code in `pico-platform`:

| Crate                         | Purpose                                                                     |
| ----------------------------- | --------------------------------------------------------------------------- |
| `crates/apps/bootloader`      | Bootloader that swaps in updates and rolls them back                        |
| `crates/apps/weather-station` | Firmware of the weather station                                             |
| `crates/apps/touch-display`   | Firmware of the touch display                                               |
| `crates/libs/pico-platform`   | Watchdog, USB logger, and, with the `nexigon` feature, Nexigon Hub          |
| `crates/libs/pico-build`      | Helpers for the build scripts of the firmware crates, which run on the host |

Everything that concerns Nexigon is in `crates/libs/pico-platform/src/nexigon/`, which only the `nexigon` feature of `pico-platform` compiles, together with its dependencies. Its entry point `nexigon::run` takes the configuration of the device, the peripherals that the connection needs, and an `Application`. The application starts its own tasks once the device has joined the Wi-Fi network, with up to two sockets of the network stack at a time, receives the value of its configuration property, and provides the event that the device publishes with every heartbeat. A firmware creates its configuration with `pico_platform::nexigon_config!()`, which embeds what `pico_build::embed_nexigon_config` checks and the identity in `identity/` of the crate.

The weather station enables the `nexigon` feature through its own feature of the same name. Without it, `src/standalone.rs` logs the measurements instead. Both variants use the same bootloader and flash layout, so a device can switch between them when flashed over USB.

| File in `crates/apps/weather-station/src/` | Purpose                                                                        |
| ------------------------------------------ | ------------------------------------------------------------------------------ |
| `main.rs`                                  | Start of the firmware, which hands the station to a variant                    |
| `station.rs`                               | Station, which measures and shows the measurements                             |
| `sensors.rs`                               | Measurements of all sensors                                                    |
| `dht11.rs`                                 | Readings of the DHT11 sensor                                                   |
| `uv.rs`                                    | Readings of the UV sensor                                                      |
| `bmp280.rs`                                | Readings of the BMP280 sensor                                                  |
| `display.rs`                               | Display of the measurements on the TFT                                         |
| `nexigon.rs`                               | Measurements for Nexigon Hub and the touch display, with the `nexigon` feature |
| `standalone.rs`                            | Logging of measurements without Nexigon                                        |

| File in `crates/apps/touch-display/src/` | Purpose                                                  |
| ---------------------------------------- | -------------------------------------------------------- |
| `main.rs`                                | Start of the firmware, configuration, and storage events |
| `receiver.rs`                            | Reception of measurements, which it stores and shows     |
| `storage.rs`                             | File on the microSD card                                 |
| `display.rs`                             | Display of the latest measurement                        |

### Connection to Nexigon Hub

After joining the Wi-Fi network, the firmware synchronizes its clock with SNTP. Events carry the time they were emitted.

The firmware then opens a TLS 1.3 connection to Nexigon Hub with [`embedded-tls`](https://github.com/drogue-iot/embedded-tls) and authenticates with its own certificate. The handshake is the most expensive step on the RP2040, which has no hardware acceleration for cryptography. The connection therefore stays open and is only reopened after a failure. Before reopening it, the firmware joins the Wi-Fi network again if it has lost its address, because the Wi-Fi chip does not rejoin on its own.

Over TLS, the firmware opens a WebSocket connection to `/api/v1/connect/ws`. The multiplex protocol of Nexigon carries channels over it, with flow control for each channel:

- The device opens an `executor` channel and invokes actions over it. It learns its device ID with `actor_GetActor`, describes its commands in the `dev.nexigon.commands` property with `devices_SetProperty`, and publishes events with `devices_PublishEvents`.
- Nexigon Hub opens a `handler` channel for each command it asks the device to run. The device reads the invocation, runs the command, and answers with its result.
- Nexigon Hub pings the device every few seconds, and the device answers. The device reconnects if it receives nothing for 30 seconds.

All of this runs on one task, which owns the connection. While the device waits for the next heartbeat, a command, or the response to an action, it answers pings and serves the other channel.

### Updates

The flash holds the bootloader, the state of the bootloader, and two slots for the firmware. `firmware.x` describes the layout:

| Region           | Size     | Content                                           |
| ---------------- | -------- | ------------------------------------------------- |
| Bootloader       | 24 KiB   | `crates/apps/bootloader`, based on `embassy-boot` |
| Bootloader state | 4 KiB    | Pending update and progress of swapping it in     |
| Active slot      | 1008 KiB | Running firmware                                  |
| DFU slot         | 1012 KiB | Downloaded update                                 |

The firmware follows the OTA update contract of Nexigon Hub, which Nexigon Agent also implements. A check reads the desired state from the `dev.nexigon.ota.config` property. It resolves the version path of the configuration and the version of the running firmware, the tag `FIRMWARE_VERSION`, in the same package. If they differ, the firmware downloads the asset and its signature from the object storage of Nexigon Hub over a second TLS connection. It writes the asset to the DFU slot, verifies its signature with the release key, marks it for the bootloader, and resets. The bootloader swaps the slots and starts the new firmware, which commits itself once it reaches Nexigon Hub. If the new firmware does not commit itself within 5 minutes, the watchdog resets the device, and the bootloader swaps the old firmware back. A power failure during a swap resumes it. The firmware reports each step in the `dev.nexigon.ota.status` property.

The types of the actions, command invocations, properties, and results in `generated/nexigon_api.rs` are generated from the API definitions of Nexigon Hub with Sidex's `rust-embedded` backend. They read messages without copying them and borrow from the receive buffers.

| File in `crates/libs/pico-platform/src/nexigon/` | Purpose                                                     |
| ------------------------------------------------ | ----------------------------------------------------------- |
| `mod.rs`                                         | Entry point, applications, and the session with Nexigon Hub |
| `config.rs`                                      | Configuration and identity of a device                      |
| `network.rs`                                     | Wi-Fi and the network stack                                 |
| `events.rs`                                      | Heartbeat events and the events of applications             |
| `commands.rs`                                    | Commands of the device                                      |
| `ota.rs`                                         | Update checks following the OTA update contract             |
| `firmware.rs`                                    | Firmware slots, signature verification, and commit deadline |
| `download.rs`                                    | Downloads over HTTPS                                        |
| `hub.rs`                                         | Session with Nexigon Hub: actions, commands, and pings      |
| `multiplex.rs`                                   | Frames of the multiplex protocol                            |
| `websocket.rs`                                   | WebSocket client                                            |
| `tls.rs`                                         | TLS connections and client authentication                   |
| `clock.rs`                                       | Clock synchronization with SNTP                             |

## Limitations

The following limitations concern the connection to Nexigon Hub and between the devices.

- The private key of the device is stored in flash, from which anyone with physical access can read it. The RP2040 has no secure key storage.
- The RP2040 has no true random number generator. The firmware uses its ring oscillator, the best source of entropy available.
- The firmware does not verify the certificate of the server, so it cannot detect a man-in-the-middle, who would learn the deployment token. Verifying the certificate chain of Let's Encrypt takes the RP2040 more than 80 seconds, longer than Nexigon Hub waits for a handshake. The signature of updates prevents a man-in-the-middle from installing their own firmware.
- The firmware implements a basic version of the OTA update contract. Besides the version path, it supports `hold` and `asset`. It does not support maintenance windows, spreading updates over time, or checkpoint versions. It does not retry a failed update on its own: the update stays failed until the configuration changes or the failure count in the status is cleared.
- The device offers commands only. It rejects other channels, such as those for terminals and port forwarding.
- Command invocations must fit into 1 KiB and responses to actions into 4 KiB.
- The touch display identifies the weather station by its IP address only, so any device in the network can send it measurements from that address. Datagrams are not acknowledged, so measurements are lost while the touch display is unreachable.
- Reading the flash with a debugger while the firmware writes an update corrupts the update. The device then hangs until the watchdog resets it, and the update fails as interrupted.
- `vendor/embedded-tls/` holds a patched version of `embedded-tls`. See `vendor/README.md`.

## Regenerate the API Types

To update the types after changes to the API of Nexigon Hub, run the following with a checkout of the public `nexigon` repository and a Sidex CLI that has the `rust-embedded` backend:

```sh
mise run generate:api ../nexigon
```

## License

The firmware is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option. Two parts keep their own licenses:

- The Wi-Fi chip firmware in `crates/libs/pico-platform/firmware/cyw43/` comes from [`cyw43-driver`](https://github.com/georgerobotics/cyw43-driver/tree/main/firmware) and is licensed under the [Infineon Permissive Binary License](crates/libs/pico-platform/firmware/cyw43/LICENSE-permissive-binary-license-1.0.txt).
- The patched `embedded-tls` in `vendor/embedded-tls/` is licensed under the [Apache License, Version 2.0](vendor/embedded-tls/LICENSE).
