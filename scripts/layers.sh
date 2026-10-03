#!/bin/sh
# The layers of the WebAssembly module and the C library, in one place.
#
# Each is a Cargo feature of both crates/hyper-calendar-wasm and
# crates/hyper-calendar-ffi. scripts/wasm-layers.sh, wasm-size-check.sh and
# layer-tests.sh source this file for the list; run on its own it prints
# the layers one per line, or as a JSON array with --json, which is what
# the CI workflow's per-layer matrix reads.
#
#     scripts/layers.sh           # civil, timestamps, ... one per line
#     scripts/layers.sh --json    # ["civil","timestamps",...]

layers="civil timestamps time-codes calendars seasons holiday deep-time tz sky orbital jupiter planetary relativity places humanize natural datetime patterns zone-names uncertainty units fiscal name-days attributes full"

if [ "${0##*/}" = layers.sh ]; then
    set -eu
    case ${1:-} in
        --json)
            json=""
            for layer in $layers; do
                json="$json${json:+,}\"$layer\""
            done
            printf '[%s]\n' "$json"
            ;;
        "")
            for layer in $layers; do
                printf '%s\n' "$layer"
            done
            ;;
        *)
            echo "usage: scripts/layers.sh [--json]" >&2
            exit 2
            ;;
    esac
fi
