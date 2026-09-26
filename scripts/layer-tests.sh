#!/bin/sh
# Build and test each layer of the WebAssembly module and the C library on
# its own.
#
# The workspace test builds every crate with --all-features, which hides a
# layer that does not build, or whose tests do not pass, without the
# others: a default C build once broke that way and only the pre-push hook
# caught it. For each layer this builds the module for wasm32 and runs both
# crates' tests with that layer's feature alone:
#
#     cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --release --no-default-features --features <layer>
#     cargo test -p hyper-calendar-wasm --no-default-features --features <layer>
#     cargo test -p hyper-calendar-ffi --no-default-features --features <layer>
#
# With no arguments every layer scripts/layers.sh lists is run, in order;
# with arguments, those layers, which is how CI gives each its own job.
#
#     scripts/layer-tests.sh             # every layer
#     scripts/layer-tests.sh civil tz    # two
set -eu

cd "$(dirname "$0")/.."
. scripts/layers.sh

selected=${*:-$layers}
for layer in $selected; do
    case " $layers " in
        *" $layer "*) ;;
        *)
            echo "$layer is not a layer; scripts/layers.sh lists them" >&2
            exit 2
            ;;
    esac
    echo "== $layer"
    cargo build -q -p hyper-calendar-wasm --target wasm32-unknown-unknown --release \
        --no-default-features --features "$layer"
    cargo test -q -p hyper-calendar-wasm --no-default-features --features "$layer"
    cargo test -q -p hyper-calendar-ffi --no-default-features --features "$layer"
done
