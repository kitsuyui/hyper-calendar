#!/bin/sh
# Build every layer of the WebAssembly module and print a size table.
#
# Each feature of crates/hyper-calendar-wasm is built on its own, with the
# size-first `release-compact` profile unless PROFILE says otherwise, and
# copied to <target>/wasm-layers/hyper_calendar_wasm.<feature>.wasm, where
# <target> is CARGO_TARGET_DIR or ./target. The table printed at the end is
# the one the crate's README carries; when the sizes move, paste it there
# with the date.
#
#     scripts/wasm-layers.sh              # release-compact
#     PROFILE=release scripts/wasm-layers.sh
set -eu

cd "$(dirname "$0")/.."

profile=${PROFILE:-release-compact}
target_dir=${CARGO_TARGET_DIR:-target}
out="$target_dir/wasm-layers"
layers="civil timestamps calendars seasons holiday deep-time tz sky orbital planetary relativity full"

mkdir -p "$out"

command="cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --profile $profile --no-default-features --features <layer>"
rows=""
for layer in $layers; do
    cargo build -q -p hyper-calendar-wasm --target wasm32-unknown-unknown \
        --profile "$profile" --no-default-features --features "$layer"
    built="$target_dir/wasm32-unknown-unknown/$profile/hyper_calendar_wasm.wasm"
    copy="$out/hyper_calendar_wasm.$layer.wasm"
    cp "$built" "$copy"
    bytes=$(wc -c < "$copy" | tr -d ' ')
    human=$(awk -v b="$bytes" 'BEGIN {
        if (b >= 1048576) printf "%.2f MiB", b / 1048576;
        else printf "%.0f KiB", b / 1024
    }')
    rows="$rows| \`$layer\` | $bytes | $human |
"
done

printf '%s\n' "$command" "$(date -u +%Y-%m-%d) with $(rustc --version)" ''
printf '| Feature | Bytes | Size |\n| --- | ---: | ---: |\n%s' "$rows"
printf '\nThe files are in %s\n' "$out"
