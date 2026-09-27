#!/bin/sh
# The release-mode test run, `cargo test --release --workspace
# --all-features`, in shards that CI runs side by side.
#
# A release build walks every day of the longest sweeps (docs/policy.md §7),
# and two unit-test binaries hold most of that time. Measured on a
# four-core runner in September 2026, the build takes about 3 minutes;
# hc-calendars-indic's unit tests take 544 s; hc-calendars-lunar's take
# 428 s, and about 3 minutes more for `babylonian`'s every-day sweep in the
# same binary. hc-calendars-regional's binary takes about a minute, since
# its Arsacid era rests on `babylonian`'s sweep rather than walking the
# same days again, and every other binary and the doctests take about
# 2 1/2 minutes together, besides hc-astro's every-day sweep of the Edo
# hours over 4 000 years, which takes 48 s of ten threads on a desktop and
# was not measured on the runner. So indic and lunar each have a shard, and `rest`
# runs every other test binary cargo builds, and the doctests.
#
# A shard names a package's unit-test binary and `rest` is the complement,
# so every test runs in exactly one shard. A new crate, test file or doctest
# lands in `rest` with nothing to update here, and a shard whose binary
# cargo no longer builds fails rather than running nothing. --plan prints
# the partition.
#
# Every shard builds with --workspace --all-features, as the unsharded run
# does, so that each binary has the features that run gives it: a `-p` build
# of one crate would not, since it is the facade that turns on hc-core's
# memo. A named shard builds the unit-test binaries alone (--lib), which
# resolves the same features.
#
# After each binary the wall and CPU seconds are printed, so the log shows
# whether a shard's sweeps keep the runner's cores busy.
#
#     scripts/release-shards.sh --json    # ["hc-calendars-indic",...,"rest"]
#     scripts/release-shards.sh --plan    # each test binary and its shard
#     scripts/release-shards.sh <shard>   # build and run one shard
#
# Reads cargo's JSON messages with jq.
set -eu

cd "$(dirname "$0")/.."

# The packages whose unit-test binary is a shard of its own. A shard cannot
# be split below one binary here; to shorten one, make its sweeps cheaper or
# name another package.
shards="hc-calendars-indic hc-calendars-lunar"

tab=$(printf '\t')
scratch=$(mktemp -d "${TMPDIR:-/tmp}/release-shards.XXXXXX")
trap 'rm -rf "$scratch"' EXIT

# One line per test binary cargo builds with the arguments given:
# shard, package directory, executable, tab-separated.
binaries() {
    cargo test --release --workspace --all-features --no-run \
        --message-format=json-render-diagnostics "$@" > "$scratch/messages.json"
    jq -r --arg shards "$shards" '
        ($shards | split(" ")) as $named
        | select(.reason == "compiler-artifact" and .profile.test and .executable != null)
        | .target as $target
        | ([$named[] | select(
                (gsub("-"; "_")) == $target.name
                and ($target.kind | any(. == "test" or . == "bin" or . == "example") | not))]
            | first // "rest") as $shard
        | [$shard, (.manifest_path | sub("/Cargo.toml$"; "")), .executable]
        | @tsv' "$scratch/messages.json" | sort
}

# Sets cpu to the CPU seconds, user and system, of the children this shell
# has waited for. `times` runs in this shell, not a command substitution's:
# a subshell starts with no children's time.
cpu_seconds() {
    times > "$scratch/times"
    cpu=$(awk 'NR == 2 {
        total = 0
        for (i = 1; i <= 2; i++) { split($i, part, "m"); total += part[1] * 60 + part[2] }
        printf "%d\n", total
    }' "$scratch/times")
}

run() {
    shard=$1
    start=$(date +%s)
    if [ "$shard" = rest ]; then
        binaries > "$scratch/plan"
    else
        binaries --lib > "$scratch/plan"
    fi
    echo "Built in $(($(date +%s) - start)) s; $(getconf _NPROCESSORS_ONLN) cores"
    for named in $shards; do
        if ! cut -f1 "$scratch/plan" | grep -qx "$named"; then
            echo "$named builds no unit-test binary; remove it from scripts/release-shards.sh" >&2
            exit 1
        fi
    done
    failed=""
    while IFS="$tab" read -r owner dir executable; do
        [ "$owner" = "$shard" ] || continue
        echo "Running ${executable##*/} in ${dir#"$PWD"/}"
        wall=$(date +%s)
        cpu_seconds
        before=$cpu
        if ! (cd "$dir" && CARGO_MANIFEST_DIR=$dir "$executable" < /dev/null); then
            failed="$failed ${executable##*/}"
        fi
        cpu_seconds
        echo "${executable##*/}: $(($(date +%s) - wall)) s, $((cpu - before)) CPU-s"
    done < "$scratch/plan"
    if [ "$shard" = rest ]; then
        cargo test --release --workspace --all-features --doc || failed="$failed doctests"
    fi
    if [ -n "$failed" ]; then
        echo "Failed:$failed" >&2
        exit 1
    fi
}

case ${1:-} in
    --json)
        json=""
        for shard in $shards rest; do
            json="$json${json:+,}\"$shard\""
        done
        printf '[%s]\n' "$json"
        ;;
    --plan)
        binaries > "$scratch/plan"
        awk -F "$tab" '{ sub(".*/", "", $3); print $1 "\t" $3 }' "$scratch/plan"
        ;;
    rest)
        run rest
        ;;
    "")
        echo "usage: scripts/release-shards.sh --json | --plan | <shard>" >&2
        exit 2
        ;;
    *)
        case " $shards " in
            *" $1 "*) run "$1" ;;
            *)
                echo "$1 is not a shard; the shards are: $shards rest" >&2
                exit 2
                ;;
        esac
        ;;
esac
