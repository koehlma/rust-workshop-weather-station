#!/usr/bin/env bash
# Creates the key pair with which releases are signed in `keys/`: a P-256
# private key and its public key as an uncompressed point, which the firmware
# embeds. Devices only install updates signed with the private key.

set -euo pipefail

cd "$(dirname "$0")/.."

if [ -e keys ]; then
    echo "error: keys/ already exists; delete it to create a new release key" >&2
    exit 1
fi

mkdir keys
chmod 700 keys
openssl ecparam -name prime256v1 -genkey -noout -out keys/release-key.pem
chmod 600 keys/release-key.pem
# The last 65 bytes of the DER-encoded public key are the uncompressed point.
openssl ec -in keys/release-key.pem -pubout -outform DER 2>/dev/null | tail -c 65 \
    > keys/release-key.pub

echo "Created release key in keys/."
