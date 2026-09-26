#!/bin/sh
# Check the built WebAssembly layers against the size table in the crate's
# README.
#
# Run after scripts/wasm-layers.sh, which leaves each layer at
# <target>/wasm-layers/hyper_calendar_wasm.<feature>.wasm. A layer whose
# size differs from the README's Bytes column by more than TOLERANCE percent
# (default 5) fails the check, in either direction: a layer that grew by
# accident is caught, and a table that no longer describes the module is
# refreshed on purpose. The tolerance absorbs the drift between compiler
# releases; a real change moves a layer by more.
#
#     scripts/wasm-layers.sh && scripts/wasm-size-check.sh
set -eu

cd "$(dirname "$0")/.."

target_dir=${CARGO_TARGET_DIR:-target}
out="$target_dir/wasm-layers"
readme=crates/hyper-calendar-wasm/README.md
tolerance=${TOLERANCE:-5}
layers="civil timestamps calendars seasons holiday deep-time tz sky orbital full"

status=0
for layer in $layers; do
    built="$out/hyper_calendar_wasm.$layer.wasm"
    if [ ! -f "$built" ]; then
        echo "missing $built: run scripts/wasm-layers.sh first" >&2
        exit 2
    fi
    actual=$(wc -c < "$built" | tr -d ' ')
    # The row starts with the feature in backticks; the Bytes column is the
    # second-to-last cell, written with thousands separators.
    recorded=$(awk -F'|' -v f="\`$layer\`" '
        { cell = $2; gsub(/^ +| +$/, "", cell); sub(/ .*/, "", cell) }
        cell == f { bytes = $(NF - 2); gsub(/[ ,]/, "", bytes); print bytes; exit }
    ' "$readme")
    if [ -z "$recorded" ]; then
        echo "$layer: no row in $readme" >&2
        status=1
        continue
    fi
    verdict=$(awk -v a="$actual" -v r="$recorded" -v t="$tolerance" 'BEGIN {
        d = (a - r) * 100 / r
        printf "%+.1f%% %s", d, (d > t || d < -t) ? "FAIL" : "ok"
    }')
    printf '%-10s %10s bytes, README %10s: %s\n' "$layer" "$actual" "$recorded" "$verdict"
    case $verdict in
        *FAIL) status=1 ;;
    esac
done

if [ "$status" -ne 0 ]; then
    echo "A layer moved by more than $tolerance% from $readme." >&2
    echo "If the change is intended, paste the table scripts/wasm-layers.sh prints into the README." >&2
fi
exit "$status"
