#!/usr/bin/env bash
# Publishes the firmware of a crate built by `scripts/build.sh` as a new
# version of a package in Nexigon Hub. The version gets the assets
# `firmware.bin` and `firmware.bin.sig`, the locked tag of the firmware
# version, and the tag `latest`, which moves to it.
#
# Usage: release.sh CRATE REPOSITORY/PACKAGE
#
# Requires `nexigon-cli` (or `NEXIGON_CLI`) and an organization API token in
# `NEXIGON_ORG_TOKEN`.

set -euo pipefail

cd "$(dirname "$0")/.."

if [ $# -ne 2 ] || [[ "$2" != */* ]]; then
    echo "usage: $0 CRATE REPOSITORY/PACKAGE, for instance, $0 weather-station default/rp-sensors" >&2
    exit 1
fi
out="target/$1"
package="$2"
repository="${package%%/*}"
version="$(cat "$out/firmware.version")"
cli="${NEXIGON_CLI:-nexigon-cli}"

config="$(mktemp)"
trap 'rm -f "$config"' EXIT
chmod 600 "$config"
printf 'hub-url = "%s"\ntoken = "%s"\n' "$NEXIGON_HUB_URL" "$NEXIGON_ORG_TOKEN" >"$config"

nexigon() {
    "$cli" --config "$config" "$@"
}

# Prints a field of the JSON object on standard input.
field() {
    python3 -c 'import json, sys; print(json.load(sys.stdin)[sys.argv[1]])' "$1"
}

image="$(nexigon repositories assets upload "$repository" "$out/firmware.bin" | field assetId)"
signature="$(nexigon repositories assets upload "$repository" "$out/firmware.bin.sig" | field assetId)"
version_id="$(
    nexigon repositories versions create "$package" \
        --tag "$version,locked" \
        --metadata "{\"imageVersion\": \"$version\"}" |
        field versionId
)"
nexigon repositories versions assets add "$version_id" "$image" firmware.bin >/dev/null
nexigon repositories versions assets add "$version_id" "$signature" firmware.bin.sig >/dev/null
nexigon repositories versions tag "$version_id" --tag latest,reassign >/dev/null

echo "Released firmware $version as $package/$version ($version_id), tagged latest."
