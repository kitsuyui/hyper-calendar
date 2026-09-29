#!/bin/sh
# The release-mode test run, `cargo test --release --workspace
# --all-features`, in shards that CI runs side by side.
#
# A release build walks every day of the longest sweeps (docs/policy.md §7),
# and three unit-test binaries hold most of that time. Measured on a
# four-core runner on 27 September 2026 (the main run of 8a709a6), a named
# shard builds in 99 to 138 s and `rest` in 264 s. hc-calendars-indic's
# unit tests take 541 s and 2 151 CPU-s; hc-calendars-lunar's take 1 401 s
# and 5 587 CPU-s, a job of 25.7 minutes with the build when it ran as one
# shard in that run, which is why it is split below. hc-calendars-regional's
# binary takes about two minutes, since its Arsacid era rests on
# `babylonian`'s sweep rather than walking the same days again, and
# hc-astro's, with its every-day sweep of the Edo hours over 4 000 years,
# 509 s and 2 032 CPU-s, which in that run was in `rest` with every other
# binary and the doctests, about 18 minutes together.
#
# So indic, lunar and hc-astro each have a shard, `rest` runs every other
# test binary cargo builds, and the doctests, and lunar's binary is split in two by
# libtest's name filters. One integration-test binary has a shard too:
# hyper-calendar's `written_dates`, whose release build writes and reads
# back the first day of every era, its eve and the first day of every month
# of every calendar in every locale, some 850 000 texts, 790 CPU-s on a
# fourteen-core desktop on 29 September 2026, which in `rest` would make it
# the longest shard. Its sweeps keep every core busy, so a shard's time
# is its CPU time over the cores, and the split is by CPU time. On a
# fourteen-core desktop the same build's heaviest tests took, alone, 572
# CPU-s for `islamic_global`'s every-day sweep, 400 for `babylonian`'s, 248
# for `tibetan`'s and 219 for `islamic_observational`'s, and the binary 1 595
# in all. So `hc-calendars-lunar-islamic` runs the tests whose names contain
# `islamic_`, the Hijri calendars' modules, and `hc-calendars-lunar` every
# other test of the binary: 797 and 770 CPU-s there, about half each.
#
# A shard names a package's unit-test binary, or the tests of it whose
# names contain a pattern, or an integration-test binary by its name, and
# `rest` is the complement, so every test runs
# in exactly one shard. A new crate, test file or doctest lands in `rest`
# with nothing to update here, a new test of a split binary in whichever
# half its name puts it in, and a shard whose binary cargo no longer builds
# fails rather than running nothing. Before a split binary runs, the script
# lists its tests (`--list`) whole and under each shard's filters, and fails
# unless every test is in exactly one shard's list. --plan prints the
# partition and makes the same check.
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

# The packages whose unit-test binary is taken out of `rest`. Each is a
# shard, less the tests the splits below take.
packages="hc-calendars-indic hc-calendars-lunar hc-astro"

# The shards that take tests out of a package's binary by name, one per
# line: the shard, the package, and the libtest filter, a substring of the
# test's path. A split shard runs the tests whose names contain its filter;
# the package's shard runs the others, with `--skip` for each.
splits="hc-calendars-lunar-islamic hc-calendars-lunar islamic_"

# The integration-test binaries taken out of `rest`, one per line: the
# shard and the test target's name. A shard runs the whole binary.
test_binaries="hyper-calendar-written-dates written_dates"

tab=$(printf '\t')
scratch=$(mktemp -d "${TMPDIR:-/tmp}/release-shards.XXXXXX")
trap 'rm -rf "$scratch"' EXIT

# Every shard, in the order --json lists them.
shard_names() {
    for package in $packages; do
        echo "$package"
        echo "$splits" | awk -v package="$package" '$2 == package { print $1 }'
    done
    echo "$test_binaries" | awk '{ print $1 }'
    echo rest
}

# The test target `shard` runs, where it is an integration-test binary's.
test_target_of() {
    echo "$test_binaries" | awk -v shard="$1" '$1 == shard { print $2 }'
}

# The package whose binary `shard` runs, or `rest`.
package_of() {
    case " $packages " in
        *" $1 "*) echo "$1"; return ;;
    esac
    found=$(echo "$splits" | awk -v shard="$1" '$1 == shard { print $2 }')
    [ -n "$found" ] || found=$(echo "$test_binaries" | awk -v shard="$1" '$1 == shard { print $1 }')
    echo "${found:-rest}"
}

# The libtest arguments that pick `shard`'s tests out of its package's
# binary, space-separated: the split's filter, or a --skip for each split of
# the package. No filter holds a space.
filters_of() {
    echo "$splits" | awk -v shard="$1" '
        $1 == shard { out = out " " $3 }
        $2 == shard { out = out " --skip " $3 }
        END { print substr(out, 2) }'
}

# One line per test binary cargo builds with the arguments given:
# package or `rest`, package directory, executable, tab-separated.
binaries() {
    cargo test --release --workspace --all-features --no-run \
        --message-format=json-render-diagnostics "$@" > "$scratch/messages.json"
    jq -r --arg packages "$packages" --arg tests "$test_binaries" '
        ($packages | split(" ")) as $named
        | ($tests | split("\n") | map(split(" "))) as $integration
        | select(.reason == "compiler-artifact" and .profile.test and .executable != null)
        | .target as $target
        | ([$named[] | select(
                (gsub("-"; "_")) == $target.name
                and ($target.kind | any(. == "test" or . == "bin" or . == "example") | not))]
            + [$integration[] | select(
                .[1] == $target.name and ($target.kind | any(. == "test"))) | .[0]]
            | first // "rest") as $owner
        | [$owner, (.manifest_path | sub("/Cargo.toml$"; "")), .executable]
        | @tsv' "$scratch/messages.json" | sort
}

# The names of the tests `executable` holds, under the libtest filters
# given, one per line, sorted.
test_names() {
    dir=$1
    executable=$2
    shift 2
    (cd "$dir" && CARGO_MANIFEST_DIR=$dir "$executable" --list --format terse "$@" < /dev/null) |
        sed -n -E 's/: (test|bench)$//p' | sort
}

# Fail unless every test of `package`'s binary is in exactly one of its
# shards' lists: none in two, none in no shard, and no shard empty, which
# is also what a binary that cannot list its tests gives. A shell function's
# variables are the shell's: this one sets `package`, `dir` and
# `executable` to the values run() holds them at, and no other name run()
# reads.
check_partition() {
    package=$1
    dir=$2
    executable=$3
    test_names "$dir" "$executable" > "$scratch/all"
    : > "$scratch/listed"
    summary=""
    for part in $(shard_names); do
        [ "$(package_of "$part")" = "$package" ] || continue
        # shellcheck disable=SC2046 # one argument per word
        test_names "$dir" "$executable" $(filters_of "$part") > "$scratch/one"
        if [ ! -s "$scratch/one" ]; then
            echo "$part lists no test of ${executable##*/}" >&2
            exit 1
        fi
        cat "$scratch/one" >> "$scratch/listed"
        summary="$summary $part $(wc -l < "$scratch/one" | tr -d ' '),"
    done
    sort "$scratch/listed" > "$scratch/sorted"
    twice=$(uniq -d "$scratch/sorted")
    missing=$(sort -u "$scratch/sorted" | comm -23 "$scratch/all" -)
    if [ -n "$twice" ] || [ -n "$missing" ]; then
        [ -z "$twice" ] || printf 'In two shards of %s:\n%s\n' "$package" "$twice" >&2
        [ -z "$missing" ] || printf 'In no shard of %s:\n%s\n' "$package" "$missing" >&2
        exit 1
    fi
    echo "${executable##*/}: $(wc -l < "$scratch/all" | tr -d ' ') tests, each in one shard:${summary%,}"
}

# Whether `package` has a split.
is_split() {
    echo "$splits" | awk -v package="$1" '$2 == package { found = 1 } END { exit !found }'
}

# Check every split binary in the plan.
check_plan() {
    while IFS="$tab" read -r owner dir executable; do
        if [ "$owner" != rest ] && is_split "$owner"; then
            check_partition "$owner" "$dir" "$executable"
        fi
    done < "$scratch/plan"
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
    package=$(package_of "$shard")
    start=$(date +%s)
    target=$(test_target_of "$shard")
    if [ "$package" = rest ]; then
        binaries > "$scratch/plan"
    elif [ -n "$target" ]; then
        binaries --test "$target" > "$scratch/plan"
    else
        binaries --lib > "$scratch/plan"
    fi
    echo "Built in $(($(date +%s) - start)) s; $(getconf _NPROCESSORS_ONLN) cores"
    for named in $packages; do
        [ -z "$target" ] || break
        if ! cut -f1 "$scratch/plan" | grep -qx "$named"; then
            echo "$named builds no unit-test binary; remove it from scripts/release-shards.sh" >&2
            exit 1
        fi
    done
    if [ -n "$target" ] && ! cut -f1 "$scratch/plan" | grep -qx "$shard"; then
        echo "no test binary $target; remove it from scripts/release-shards.sh" >&2
        exit 1
    fi
    filters=$(filters_of "$shard")
    failed=""
    while IFS="$tab" read -r owner dir executable; do
        [ "$owner" = "$package" ] || continue
        if [ "$owner" != rest ] && is_split "$owner"; then
            check_partition "$owner" "$dir" "$executable"
        fi
        echo "Running ${executable##*/} in ${dir#"$PWD"/}${filters:+ with $filters}"
        wall=$(date +%s)
        cpu_seconds
        before=$cpu
        # shellcheck disable=SC2086 # one argument per word
        if ! (cd "$dir" && CARGO_MANIFEST_DIR=$dir "$executable" $filters < /dev/null); then
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
        for shard in $(shard_names); do
            json="$json${json:+,}\"$shard\""
        done
        printf '[%s]\n' "$json"
        ;;
    --plan)
        binaries > "$scratch/plan"
        while IFS="$tab" read -r owner dir executable; do
            name=${executable##*/}
            if [ "$owner" = rest ]; then
                printf 'rest\t%s\n' "$name"
                continue
            fi
            for shard in $(shard_names); do
                [ "$(package_of "$shard")" = "$owner" ] || continue
                filters=$(filters_of "$shard")
                printf '%s\t%s%s\n' "$shard" "$name" "${filters:+ $filters}"
            done
        done < "$scratch/plan"
        check_plan
        ;;
    "")
        echo "usage: scripts/release-shards.sh --json | --plan | <shard>" >&2
        exit 2
        ;;
    *)
        all=$(shard_names | tr '\n' ' ')
        case " $all" in
            *" $1 "*) run "$1" ;;
            *)
                echo "$1 is not a shard; the shards are: $all" >&2
                exit 2
                ;;
        esac
        ;;
esac
