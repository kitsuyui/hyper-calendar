#!/bin/sh
# Build and test each layer of the WebAssembly module and the C library on
# its own.
#
# The workspace test builds every crate with --all-features, which hides a
# layer that does not build, or whose tests do not pass, without the
# others, as a default C build can. For each layer this builds the module
# for wasm32 and runs both crates' tests with that layer's feature alone,
# and the facade's own tests with the facade features the layer turns on
# (the `hc/...` entries of the layer's line in the wasm crate's
# Cargo.toml, followed through the layers it names):
#
#     cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --release --no-default-features --features <layer>
#     cargo test -p hyper-calendar-wasm --no-default-features --features <layer>
#     cargo test -p hyper-calendar-ffi --no-default-features --features <layer>
#     RUSTFLAGS=-D\ warnings cargo test -p hyper-calendar --no-default-features --features std,<facade features>
#
# With no arguments every layer scripts/layers.sh lists is run, in order;
# with arguments, those layers, which is how CI gives each its own job.
#
#     scripts/layer-tests.sh             # every layer
#     scripts/layer-tests.sh civil tz    # two
set -eu

cd "$(dirname "$0")/.."
. scripts/layers.sh

# The builds go to a target directory of their own. The pre-push hook runs
# this beside scripts/wasm-js-test.sh, which builds the module with other
# features to the same path under the shared target; two concurrent builds
# of one artefact would leave whichever finished last, and the JS test would
# read a module built with the wrong layer.
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target}/layer-tests"
export CARGO_TARGET_DIR

# The facade features a layer turns on, comma separated: the `hc/<feature>`
# entries of its line in the wasm crate's [features] table, and of every
# layer that line names.
facade_features() {
    awk -v layer="$1" '
    /^\[features\]/ { in_features = 1; next }
    /^\[/ { in_features = 0 }
    in_features && /^[a-z0-9-]+ = \[/ {
        line = $0
        sub(/^[^[]*\[/, "", line)
        sub(/\].*$/, "", line)
        count = split(line, parts, ",")
        list = ""
        for (i = 1; i <= count; i++) {
            gsub(/[ "]/, "", parts[i])
            list = list " " parts[i]
        }
        entries[$1] = list
    }
    function walk(feature,    items, count, item, i) {
        if (seen[feature]++) return
        count = split(entries[feature], items, " ")
        for (i = 1; i <= count; i++) {
            item = items[i]
            if (item ~ /^hc\//) {
                sub(/^hc\//, "", item)
                facade[item] = 1
            } else {
                walk(item)
            }
        }
    }
    END {
        walk(layer)
        out = ""
        for (feature in facade) out = out (out == "" ? "" : ",") feature
        print out
    }
    ' crates/hyper-calendar-wasm/Cargo.toml
}

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
    facade=$(facade_features "$layer")
    # A warning a layer alone raises, an import only another layer uses, is a
    # failure here: the build with every feature hides it.
    RUSTFLAGS="${RUSTFLAGS:-} -D warnings" \
        cargo test -q -p hyper-calendar --no-default-features --features "std${facade:+,$facade}"
done
