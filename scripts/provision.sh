#!/usr/bin/env bash
# Creates the identity of a device in `crates/apps/<crate>/identity/`: a P-256
# key, a self-signed certificate for it, and a fingerprint. The firmware of the
# crate embeds these files.
#
# Usage: provision.sh CRATE

set -euo pipefail

cd "$(dirname "$0")/.."

if [ $# -ne 1 ] || [ ! -d "crates/apps/$1" ] || [ "$1" = bootloader ]; then
    echo "usage: $0 CRATE, for instance, $0 weather-station" >&2
    exit 1
fi
crate="$1"
identity="crates/apps/$crate/identity"

if [ -e "$identity" ]; then
    echo "error: $identity/ already exists; delete it to create a new device identity" >&2
    exit 1
fi

mkdir "$identity"
openssl ecparam -name prime256v1 -genkey -noout -out "$identity/device-key.pem"
openssl ec -in "$identity/device-key.pem" -outform DER -out "$identity/device-key.der" 2>/dev/null
openssl req -new -x509 -key "$identity/device-key.pem" -subj "/CN=$crate" \
    -days 36500 -outform DER -out "$identity/device-cert.der"
# Fingerprints are `d_sk_` followed by 44 base58 digits.
printf 'd_sk_%s' "$(LC_ALL=C tr -dc '1-9A-HJ-NP-Za-km-z' </dev/urandom | head -c 44)" \
    > "$identity/fingerprint.txt"

echo "Created device identity in $identity/."
