# Vendored Crates

`embedded-tls/` is [`embedded-tls`](https://github.com/drogue-iot/embedded-tls) 0.19.0 with two fixes and one addition.

Version 0.19.0 aborts the handshake when a server lists a named group or signature scheme that it does not know, which servers with OpenSSL 3.5 do, for instance, with the groups and signatures based on SM2. As RFC 8446 requires, the patched version ignores unknown entries in these lists:

- `src/extensions/extension_data/supported_groups.rs`: the fix of the `main` branch of `embedded-tls` (commit `1c5c630`).
- `src/extensions/extension_data/signature_algorithms.rs` and `signature_algorithms_cert.rs`: the same fix for signature schemes.

The addition lets the firmware wait until a connection has data to read without starting a read, which it cannot cancel without losing data. In `src/asynch.rs`, `TlsConnection` implements `AsRef` for its transport, like `TlsReader` and `TlsWriter` already do, and gains `has_buffered_data`.

The manifest is the original `Cargo.toml` of the crate without its development dependencies. Remove this directory and the `[patch.crates-io]` section of `Cargo.toml` once a release of `embedded-tls` contains the fixes and the addition.
