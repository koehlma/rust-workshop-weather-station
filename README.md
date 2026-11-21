# Weather Station on the Raspberry Pi Pico W

Firmware for a weather station built from a Raspberry Pi Pico W (RP2040) on a Joy-IT explorer board, a SEN-UV01 UV sensor, and a BMP280 pressure sensor. Every 30 seconds, it measures the temperature, humidity, UV radiation, and air pressure and shows them on the display of the board. It is written in Rust with [Embassy](https://embassy.dev) and runs without an operating system or an allocator.

The `nexigon` feature, which is enabled by default, connects the weather station to Nexigon Hub. Every 30 seconds, the device publishes a heartbeat event and the measurement of the sensors. It runs commands that Nexigon Hub sends and installs updates that Nexigon Hub rolls out. A bootloader swaps in updates and rolls back those that fail. Without the feature, the firmware logs the measurements to the USB serial port.

With the feature, the firmware holds one persistent connection to Nexigon Hub, which authenticates the device with a deployment token, a device fingerprint, and a client certificate. On its first connection, Nexigon Hub registers the device in the project of the deployment token.

## Requirements

- A Raspberry Pi Pico W on a Joy-IT explorer board (RB-P-XPLR). The firmware uses its DHT11 sensor on GP0 and its 1.8" display, an ST7735 on SPI0. Turn on their switches on the board.
- A Joy-IT SEN-UV01 UV sensor connected to GP26, the first analog input. Connect its VCC to 3V3 (OUT), GND to GND, and OUT to GP26. Power it from 3.3 V, the most that the analog inputs of the Pico W accept.
- A BMP280 pressure sensor, for instance, the Joy-IT KY-052, connected to I2C0. Connect its +V to 3V3 (OUT), GND to GND, SDA to GP4, and SCL to GP5. Tie CSB to 3V3, which selects I2C, and SDO to 3V3, which selects the address 0x77.
- [Rust](https://rustup.rs). `rust-toolchain.toml` selects the toolchain and installs the `thumbv6m-none-eabi` target and `llvm-objcopy`.
- `python3`, which creates the UF2 file.
- Optionally, [`mise`](https://mise.jdx.dev) to run the tasks in `mise.toml`.

For Nexigon, you also need:

- A 2.4 GHz Wi-Fi network. The Wi-Fi chip of the Pico W does not support 5 GHz.
- `openssl`, which creates keys and signs releases.
- For releases, the [Nexigon CLI](https://docs.nexigon.dev) and an organization API token.

## Configure Nexigon

Skip this section to build the firmware without Nexigon.

Create the identity of the device:

```sh
mise run provision
```

This writes a P-256 key, a self-signed certificate for it, and a fingerprint to `identity/`. Every board needs its own identity. Keep it private: whoever has it can act as the device.

Create the key with which releases are signed:

```sh
mise run create-release-key
```

This writes a P-256 key pair to `keys/`. The firmware embeds the public key and installs only updates signed with the private key. All devices that receive the same updates need the same release key. Keep the private key private: whoever has it can install their own firmware.

Configure the firmware in `mise.local.toml`, which Git ignores:

```toml
[env]
NEXIGON_HUB_URL = "https://nexigon.example.com"
NEXIGON_DEPLOYMENT_TOKEN = "deployment_..."
WIFI_SSID = "..."
WIFI_PASSWORD = "..."
# Optional: version path from which the device installs updates.
NEXIGON_UPDATE_PATH = "default/rp-test/latest"
# Only needed for releases.
NEXIGON_ORG_TOKEN = "org_sk_..."
```

Without `mise`, set these environment variables before running the scripts in `scripts/`. The firmware embeds the configuration, the identity, and the public release key when it is built.

## Build and Flash the Firmware

Build the bootloader and the firmware with Nexigon:

```sh
mise run build
```

Or build them without Nexigon, which needs no configuration:

```sh
mise run build -- --no-default-features
```

The version of the firmware is the time of the build, for instance, `20261120-163751`. Set `FIRMWARE_VERSION` to choose another one.

To flash the bootloader and the firmware, hold the BOOTSEL button while connecting the Pico W over USB. It appears as a drive named `RPI-RP2`. Copy `target/nexigon-mcu-rp-example.uf2` to the drive. The Pico W restarts with the new firmware. With Nexigon, the device afterward receives new versions through Nexigon Hub.

The display shows the latest temperature, humidity, UV index, and air pressure, with dashes for a sensor that failed.

The firmware logs to a USB serial port, for instance, `/dev/ttyACM0` on Linux or `/dev/cu.usbmodem*` on macOS. Open it with a serial terminal, for instance, `tio /dev/ttyACM0`. Without Nexigon, the log looks as follows:

```text
starting firmware 20261120-163751
measured 21.0 °C, 45 % humidity, UV index 0.2 (0.021 V), 1013.2 hPa
measured 21.1 °C, 45 % humidity, UV index 0.2 (0.022 V), 1013.2 hPa
```

With Nexigon, it looks as follows:

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
published heartbeat and 21.0 °C, 45 % humidity, UV index 0.2 (0.021 V), 1013.2 hPa
```

The events appear in the event log of the device in Nexigon Hub.

## Events

With Nexigon, the device publishes two events every 30 seconds:

| Category      | Body                                                          |
| ------------- | ------------------------------------------------------------- |
| `heartbeat`   | `{"uptimeSeconds": 42, "firmwareVersion": "20261120-163751"}` |
| `environment` | Measurement of the sensors                                    |

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

`uvIndex` is `uvVolts` divided by 0.1 V. This approximation suits sensors with the GUVA-S12SD photodiode but is not calibrated. The BMP280 also measures the temperature, which the device does not publish because the DHT11 already does.

If a sensor fails, the device logs a warning and leaves out its fields. If all fail, it publishes only the heartbeat. The DHT11 and the BMP280 fail when they do not answer, for instance, because none is connected. A missing UV sensor goes unnoticed, because the open analog input yields arbitrary voltages. `src/sensors.rs` reads the sensors, and `src/nexigon/events.rs` publishes the events.

## Commands

With Nexigon, the device offers three commands, which Nexigon Hub lists on the page of the device:

| Command             | Input          | Output                               |
| ------------------- | -------------- | ------------------------------------ |
| `set-led`           | `{"on": true}` | None                                 |
| `get-status`        | None           | `{"uptimeSeconds": 42, "led": true}` |
| `nexigon.ota.check` | None           | None                                 |

`set-led` turns the LED of the Pico W on or off. `get-status` reports how long the device has been running and whether the LED is on. `nexigon.ota.check` checks for an update right away.

To add a command, describe it in `publish_manifest` and handle it in `execute`, both in `src/nexigon/commands.rs`. The device runs one command at a time, between its other work, so a command must finish quickly. Nexigon Hub stops waiting for a pong after 15 seconds.

## Release an Update

Releases are versions of a package in a repository of Nexigon Hub. Build and publish a new version:

```sh
mise run build
mise run release default/rp-test
```

The release uploads `target/firmware.bin` and its signature `target/firmware.bin.sig` as assets of a new version of the package `rp-test` in the repository `default`. It tags the version with the firmware version, a locked tag, and moves the tag `latest` to it.

Devices built with `NEXIGON_UPDATE_PATH = "default/rp-test/latest"` install the version that the tag `latest` points to. The `dev.nexigon.ota.config` property of a device overrides the defaults built into the firmware. For instance, it pins a version with `{"path": "default/rp-test/20261120-163751"}` or holds updates with `{"hold": true}`.

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

The core of the firmware reads the sensors and keeps the device running. Everything that concerns Nexigon is in `src/nexigon/`, which only the `nexigon` feature compiles, together with its dependencies. Its entry point is `nexigon::run`, which takes the peripherals it needs and the station with the sensors and the display. Without the feature, `src/standalone.rs` logs the measurements instead. Both variants use the same bootloader and flash layout, so a device can switch between them when flashed over USB.

| File                | Purpose                                                     |
| ------------------- | ----------------------------------------------------------- |
| `src/main.rs`       | Start of the firmware, which hands the station to a variant |
| `src/station.rs`    | Station, which measures and shows the measurements          |
| `src/sensors.rs`    | Measurements of all sensors                                 |
| `src/dht11.rs`      | Readings of the DHT11 sensor                                |
| `src/uv.rs`         | Readings of the UV sensor                                   |
| `src/bmp280.rs`     | Readings of the BMP280 sensor                               |
| `src/display.rs`    | Display of the measurements on the TFT                      |
| `src/watchdog.rs`   | Watchdog, which resets the device when it hangs             |
| `src/standalone.rs` | Logging of measurements without Nexigon                     |
| `src/nexigon/`      | Connection to Nexigon Hub, with the `nexigon` feature       |
| `bootloader/`       | Bootloader that swaps in updates and rolls them back        |

### Connection to Nexigon Hub

After joining the Wi-Fi network, the firmware synchronizes its clock with SNTP. Events carry the time they were emitted.

The firmware then opens a TLS 1.3 connection to Nexigon Hub with [`embedded-tls`](https://github.com/drogue-iot/embedded-tls) and authenticates with its own certificate. The handshake is the most expensive step on the RP2040, which has no hardware acceleration for cryptography. The connection therefore stays open and is only reopened after a failure.

Over TLS, the firmware opens a WebSocket connection to `/api/v1/connect/ws`. The multiplex protocol of Nexigon carries channels over it, with flow control for each channel:

- The device opens an `executor` channel and invokes actions over it. It learns its device ID with `actor_GetActor`, describes its commands in the `dev.nexigon.commands` property with `devices_SetProperty`, and publishes events with `devices_PublishEvents`.
- Nexigon Hub opens a `handler` channel for each command it asks the device to run. The device reads the invocation, runs the command, and answers with its result.
- Nexigon Hub pings the device every few seconds, and the device answers. The device reconnects if it receives nothing for 30 seconds.

All of this runs on one task, which owns the connection. While the device waits for the next heartbeat, a command, or the response to an action, it answers pings and serves the other channel.

### Updates

The flash holds the bootloader, the state of the bootloader, and two slots for the firmware. `firmware.x` describes the layout:

| Region           | Size     | Content                                        |
| ---------------- | -------- | ---------------------------------------------- |
| Bootloader       | 24 KiB   | `bootloader/`, based on `embassy-boot`         |
| Bootloader state | 4 KiB    | Pending update and progress of swapping it in  |
| Active slot      | 1008 KiB | Running firmware                               |
| DFU slot         | 1012 KiB | Downloaded update                              |

The firmware follows the OTA update contract of Nexigon Hub, which Nexigon Agent also implements. A check reads the desired state from the `dev.nexigon.ota.config` property. It resolves the version path of the configuration and the version of the running firmware, the tag `FIRMWARE_VERSION`, in the same package. If they differ, the firmware downloads the asset and its signature from the object storage of Nexigon Hub over a second TLS connection. It writes the asset to the DFU slot, verifies its signature with the release key, marks it for the bootloader, and resets. The bootloader swaps the slots and starts the new firmware, which commits itself once it reaches Nexigon Hub. If the new firmware does not commit itself within 5 minutes, the watchdog resets the device, and the bootloader swaps the old firmware back. A power failure during a swap resumes it. The firmware reports each step in the `dev.nexigon.ota.status` property.

The types of the actions, command invocations, properties, and results in `src/nexigon/generated/nexigon_api.rs` are generated from the API definitions of Nexigon Hub with Sidex's `rust-embedded` backend. They read messages without copying them and borrow from the receive buffers.

| File in `src/nexigon/` | Purpose                                                     |
| ---------------------- | ----------------------------------------------------------- |
| `mod.rs`               | Entry point and the session with Nexigon Hub                |
| `network.rs`           | Wi-Fi and the network stack                                 |
| `events.rs`            | Heartbeat and environment events                            |
| `commands.rs`          | Commands of the device                                      |
| `ota.rs`               | Update checks following the OTA update contract             |
| `firmware.rs`          | Firmware slots, signature verification, and commit deadline |
| `download.rs`          | Downloads over HTTPS                                        |
| `hub.rs`               | Session with Nexigon Hub: actions, commands, and pings      |
| `multiplex.rs`         | Frames of the multiplex protocol                            |
| `websocket.rs`         | WebSocket client                                            |
| `tls.rs`               | TLS connections and client authentication                   |
| `clock.rs`             | Clock synchronization with SNTP                             |
| `config.rs`            | Configuration, identity, and release key                    |

## Limitations

The following limitations concern the connection to Nexigon Hub.

- The private key of the device is stored in flash, from which anyone with physical access can read it. The RP2040 has no secure key storage.
- The RP2040 has no true random number generator. The firmware uses its ring oscillator, the best source of entropy available.
- The firmware does not verify the certificate of the server, so it cannot detect a man-in-the-middle, who would learn the deployment token. Verifying the certificate chain of Let's Encrypt takes the RP2040 more than 80 seconds, longer than Nexigon Hub waits for a handshake. The signature of updates prevents a man-in-the-middle from installing their own firmware.
- The firmware implements a basic version of the OTA update contract. Besides the version path, it supports `hold` and `asset`. It does not support maintenance windows, spreading updates over time, or checkpoint versions. It does not retry a failed update on its own: the update stays failed until the configuration changes or the failure count in the status is cleared.
- The device offers commands only. It rejects other channels, such as those for terminals and port forwarding.
- Command invocations must fit into 1 KiB and responses to actions into 4 KiB.
- Reading the flash with a debugger while the firmware writes an update corrupts the update. The device then hangs until the watchdog resets it, and the update fails as interrupted.
- `vendor/embedded-tls/` holds a patched version of `embedded-tls`. See `vendor/README.md`.

## Regenerate the API Types

To update the types after changes to the API of Nexigon Hub, run the following with a checkout of the public `nexigon` repository and a Sidex CLI that has the `rust-embedded` backend:

```sh
mise run generate:api ../nexigon
```

## License

The firmware is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option. Two parts keep their own licenses:

- The Wi-Fi chip firmware in `firmware/cyw43/` comes from [`cyw43-driver`](https://github.com/georgerobotics/cyw43-driver/tree/main/firmware) and is licensed under the [Infineon Permissive Binary License](firmware/cyw43/LICENSE-permissive-binary-license-1.0.txt).
- The patched `embedded-tls` in `vendor/embedded-tls/` is licensed under the [Apache License, Version 2.0](vendor/embedded-tls/LICENSE).
