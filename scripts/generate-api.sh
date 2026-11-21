#!/usr/bin/env bash
# Regenerates `src/nexigon/generated/nexigon_api.rs` from the API definitions of
# Nexigon Hub with Sidex's `rust-embedded` backend. Pass the path of a checkout
# of the `nexigon` repository.

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "usage: $0 <path-to-nexigon-repository>" >&2
    exit 1
fi

root="$(cd "$(dirname "$0")/.." && pwd)"
output="$(mktemp -d)"
trap 'rm -rf "$output"' EXIT

cd "$1/crates/libs/nexigon-api"
sidex generate rust-embedded "$output" \
    --config-override 'groups=["device"]' \
    --config-override 'derive=["Debug"]'
cp "$output/mod.rs" "$root/src/nexigon/generated/nexigon_api.rs"
rustfmt --edition 2024 "$root/src/nexigon/generated/nexigon_api.rs"
