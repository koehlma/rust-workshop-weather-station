# Nexigon on the Raspberry Pi Pico W

Example firmware that keeps a Raspberry Pi Pico W (RP2040) connected to Nexigon Hub. Every 30 seconds, it publishes a heartbeat event and the temperature and humidity that a DHT11 sensor measures. It runs commands that Nexigon Hub sends and installs updates that Nexigon Hub rolls out. A bootloader swaps in updates and rolls back those that fail. It is written in Rust with [Embassy](https://embassy.dev) and runs without an operating system or an allocator.

The firmware holds one persistent connection to Nexigon Hub, which authenticates the device with a deployment token, a device fingerprint, and a client certificate. On its first connection, Nexigon Hub registers the device in the project of the deployment token.

## Requirements

- A Raspberry Pi Pico W and a 2.4 GHz Wi-Fi network. The Wi-Fi chip of the Pico W does not support 5 GHz.
- Optionally, a DHT11 sensor connected to GP0, for instance, the one on the Joy-IT explorer board (RB-P-XPLR). Turn on its switch on the board.
- [Rust](https://rustup.rs). `rust-toolchain.toml` selects the toolchain and installs the `thumbv6m-none-eabi` target and `llvm-objcopy`.
- `openssl`, which creates keys and signs releases, and `python3`, which creates the UF2 file.
- Optionally, [`mise`](https://mise.jdx.dev) to run the tasks in `mise.toml`.
- For releases, the [Nexigon CLI](https://docs.nexigon.dev) and an organization API token.

## Build and Flash the Firmware

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

Build the bootloader and the firmware:

```sh
mise run build
```

The version of the firmware is the time of the build, for instance, `20261120-163751`. Set `FIRMWARE_VERSION` to choose another one.

To flash the bootloader and the firmware, hold the BOOTSEL button while connecting the Pico W over USB. It appears as a drive named `RPI-RP2`. Copy `target/nexigon-mcu-rp-example.uf2` to the drive. The Pico W restarts with the new firmware. Afterward, the device receives new versions through Nexigon Hub.

The firmware logs to a USB serial port, for instance, `/dev/ttyACM0` on Linux or `/dev/cu.usbmodem*` on macOS. Open it with a serial terminal, for instance, `tio /dev/ttyACM0`. The log of a working device looks as follows:

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
published heartbeat and 21.0 °C, 45 % humidity
```

The events appear in the event log of the device in Nexigon Hub.

## Events

Every 30 seconds, the device publishes two events:

| Category      | Body                                                          |
| ------------- | ------------------------------------------------------------- |
| `heartbeat`   | `{"uptimeSeconds": 42, "firmwareVersion": "20261120-163751"}` |
| `environment` | `{"temperatureCelsius": 21.0, "humidityPercent": 45.0}`       |

The `environment` event carries the reading of the DHT11 sensor. If the sensor does not answer, for instance, because none is connected, the device logs a warning and publishes only the heartbeat. `src/dht11.rs` reads the sensor.

## Commands

The device offers three commands, which Nexigon Hub lists on the page of the device:

| Command             | Input          | Output                               |
| ------------------- | -------------- | ------------------------------------ |
| `set-led`           | `{"on": true}` | None                                 |
| `get-status`        | None           | `{"uptimeSeconds": 42, "led": true}` |
| `nexigon.ota.check` | None           | None                                 |

`set-led` turns the LED of the Pico W on or off. `get-status` reports how long the device has been running and whether the LED is on. `nexigon.ota.check` checks for an update right away.

To add a command, describe it in `publish_manifest` and handle it in `execute`, both in `src/commands.rs`. The device runs one command at a time, between its other work, so a command must finish quickly. Nexigon Hub stops waiting for a pong after 15 seconds.

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

The types of the actions, command invocations, properties, and results in `src/generated/nexigon_api.rs` are generated from the API definitions of Nexigon Hub with Sidex's `rust-embedded` backend. They read messages without copying them and borrow from the receive buffers.

| File               | Purpose                                                  |
| ------------------ | -------------------------------------------------------- |
| `src/main.rs`      | Wi-Fi, networking, events, and the main loop             |
| `src/commands.rs`  | Commands of the device                                   |
| `src/dht11.rs`     | Readings of the DHT11 sensor                             |
| `src/ota.rs`       | Update checks following the OTA update contract          |
| `src/firmware.rs`  | Firmware slots, signature verification, and the watchdog |
| `src/download.rs`  | Downloads over HTTPS                                     |
| `src/hub.rs`       | Session with Nexigon Hub: actions, commands, and pings   |
| `src/multiplex.rs` | Frames of the multiplex protocol                         |
| `src/websocket.rs` | WebSocket client                                         |
| `src/tls.rs`       | TLS connections and client authentication                |
| `src/clock.rs`     | Clock synchronization with SNTP                          |
| `src/config.rs`    | Configuration, identity, and release key                 |
| `bootloader/`      | Bootloader that swaps in updates and rolls them back     |

## Limitations

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
