#!/bin/sh
# Build the WebAssembly module and run the JavaScript binding's tests.
#
# Two builds: `civil` alone, copied to <target>/wasm-js, for the test that
# a method of a missing layer refuses by name; and `full`, left where
# `cargo build --release` puts it, for everything else. Then `node --test`
# over crates/hyper-calendar-wasm/js/*.test.js, which needs Node and nothing
# else.
# <target> is CARGO_TARGET_DIR or ./target.
set -eu

cd "$(dirname "$0")/.."

target_dir=${CARGO_TARGET_DIR:-target}
built="$target_dir/wasm32-unknown-unknown/release/hyper_calendar_wasm.wasm"

mkdir -p "$target_dir/wasm-js"
cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --release \
    --no-default-features --features civil
cp "$built" "$target_dir/wasm-js/hyper_calendar_wasm.civil.wasm"

cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --release --features full

CARGO_TARGET_DIR=$target_dir node --test "crates/hyper-calendar-wasm/js/*.test.js"
