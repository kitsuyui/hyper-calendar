#!/bin/sh
# Build every hc-* crate on its own for a target that has no standard
# library, with and without `alloc`.
#
# Policy §6 asks that each crate build under `--no-default-features` with
# `libm`, and every crate does so without `alloc` too: the only code that
# needs an allocator is behind the `alloc` feature. The workspace build
# turns `alloc` on across the graph, so an item gated to `alloc` that
# ungated code uses builds there and fails here. For each crate this runs
#
#     cargo build -p <crate> --target aarch64-unknown-none --no-default-features --features libm
#     cargo build -p <crate> --target aarch64-unknown-none --no-default-features --features libm,alloc
#
# and then each crate whose default features include an optional
# dependency, built with that feature on and the rest off. The target is
# one with no `std` at all, so a use of `std` that a host build would
# accept is refused.
#
#     scripts/no-std-builds.sh               # every crate
#     scripts/no-std-builds.sh hc-holiday    # one
set -eu

cd "$(dirname "$0")/.."

target=aarch64-unknown-none

# The builds go to a target directory of their own, beside the others the
# pre-push hook runs at the same time.
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target}/no-std-builds"
export CARGO_TARGET_DIR

crates=${*:-$(for manifest in crates/hc-*/Cargo.toml; do basename "$(dirname "$manifest")"; done)}
for crate in $crates; do
    for features in libm libm,alloc; do
        echo "== $crate --features $features"
        cargo build -q -p "$crate" --target "$target" --no-default-features --features "$features"
    done
done

# The optional dependencies a crate's defaults turn on, each on its own.
[ $# -gt 0 ] && exit 0
for spec in hc-fiscal:indic hc-attributes:seasons hc-uncertainty:edtf hc-humanize:format hc-core:memo; do
    crate=${spec%%:*}
    feature=${spec#*:}
    echo "== $crate --features libm,$feature"
    cargo build -q -p "$crate" --target "$target" --no-default-features --features "libm,$feature"
done
