#!/bin/sh
# Copy the TZif files of the zones the WebAssembly module carries into a
# tzdata/ artifact for a page to load with `loadZone`.
#
# The module's built-in table holds only each zone's current rules; the
# IANA database's TZif file for the zone holds every transition it records,
# and a page that hands the file to `hc_zone_load` gets the zone's history.
# This script takes the files from the host's copy of the database —
# /usr/share/zoneinfo, or ZONEINFO — unmodified, and writes VERSION with
# the database release they came from, read from the host's `+VERSION` or
# from the first line of `tzdata.zi`.
#
#     scripts/wasm-tzdata.sh [<output directory>]
#
# The output is <target>/tzdata by default, <target> being CARGO_TARGET_DIR
# or ./target. The zones listed below are the module's built-in seventeen,
# the ones `crates/hc-tz/src/builtin.rs` declares; the tests in
# crates/hyper-calendar-wasm/js hold the two lists to each other.
set -eu

cd "$(dirname "$0")/.."

zoneinfo=${ZONEINFO:-/usr/share/zoneinfo}
out=${1:-${CARGO_TARGET_DIR:-target}/tzdata}
zones="UTC Africa/Cairo America/Los_Angeles America/New_York America/Sao_Paulo Asia/Kathmandu Asia/Kolkata Asia/Seoul Asia/Shanghai Asia/Tokyo Australia/Lord_Howe Australia/Sydney Europe/Berlin Europe/London Europe/Moscow Europe/Paris Pacific/Auckland"

if [ -f "$zoneinfo/+VERSION" ]; then
    version=$(tr -d '[:space:]' < "$zoneinfo/+VERSION")
elif [ -f "$zoneinfo/tzdata.zi" ]; then
    version=$(sed -n '1s/^# version //p' "$zoneinfo/tzdata.zi")
else
    echo "cannot tell the tzdata version: neither $zoneinfo/+VERSION nor $zoneinfo/tzdata.zi exists" >&2
    exit 1
fi
if [ -z "$version" ]; then
    echo "cannot tell the tzdata version from $zoneinfo" >&2
    exit 1
fi

mkdir -p "$out"
for zone in $zones; do
    file="$zoneinfo/$zone"
    if [ ! -f "$file" ]; then
        echo "$file does not exist" >&2
        exit 1
    fi
    if [ "$(head -c 4 "$file")" != "TZif" ]; then
        echo "$file is not a TZif file" >&2
        exit 1
    fi
    mkdir -p "$out/$(dirname "$zone")"
    cp "$file" "$out/$zone"
done

printf '%s\n' "$version" > "$out/VERSION"
cat > "$out/SOURCE" <<EOF
IANA Time Zone Database, release $version.
The files beside this one are the compiled TZif files of the zones the
hyper-calendar WebAssembly module carries built in, copied unmodified
from $zoneinfo on $(date -u +%Y-%m-%d). The database is in the public domain.
https://www.iana.org/time-zones
EOF

count=$(printf '%s\n' $zones | wc -l | tr -d ' ')
echo "$count zones of tzdata $version in $out"
