#!/bin/sh
# Where the coverage job's time goes: the slowest test binaries and the
# slowest tests, timed as .github/workflows/octocov.yml runs them.
#
# The coverage job builds every test binary with llvm-cov's instrumentation
# (`cfg(coverage)` set) and runs the binaries one after another, each on one
# test thread, so the job's test time is the sum of the binaries' times.
# This builds the same binaries, runs each the same way and prints the
# binaries by wall-clock seconds, and then the slowest tests of the run.
# `.octocov.yml` says what the time limit is for; when a run nears it, run
# this to see which crate or test grew before touching a limit or a sample.
#
#     scripts/coverage-times.sh          # 30 binaries and 30 tests
#     TOP=10 scripts/coverage-times.sh   # 10 and 10
#
# Run it on an idle machine: other work running beside it adds to the times.
# The whole run takes about as long as the job's test step, and it needs
# cargo-llvm-cov and jq. A test's own time is read from libtest's
# `--report-time`, which is unstable, so the binaries run with
# RUSTC_BOOTSTRAP=1. The profile data lands where `cargo llvm-cov clean
# --workspace` removes it.
set -eu

cd "$(dirname "$0")/.."
top="${TOP:-30}"
work="$(mktemp -d "${TMPDIR:-/tmp}/coverage-times.XXXXXX")"
trap 'rm -rf "$work"' EXIT

cargo llvm-cov clean --workspace
# The variables `cargo llvm-cov` sets for its own test run: the
# instrumentation flags, `cfg(coverage)` and where the profile data goes.
eval "$(cargo llvm-cov show-env --export-prefix)"

cargo test --workspace --all-features --no-run --message-format=json |
  jq -r 'select(.reason == "compiler-artifact" and .profile.test and .executable != null)
         | [.executable, .target.name, (.manifest_path | sub("/Cargo.toml$"; ""))] | @tsv' \
    >"$work/binaries"

while IFS="$(printf '\t')" read -r executable name directory; do
  start="$(date +%s)"
  # A test binary runs in its package's directory, as `cargo test` runs it.
  (cd "$directory" && RUSTC_BOOTSTRAP=1 "$executable" --test-threads=1 \
    -Zunstable-options --report-time) >"$work/output" 2>&1 || {
    cat "$work/output"
    echo "$name failed" >&2
    exit 1
  }
  echo "$(($(date +%s) - start)) $name" >>"$work/seconds"
  # `test <name> ... ok <1.234s>`: the time first, then the binary.
  sed -n 's/^test \(.*\) \.\.\. ok <\([0-9.]*\)s>$/\2 '"$name"' \1/p' "$work/output" \
    >>"$work/tests"
done <"$work/binaries"

echo "binaries, seconds:"
sort -rn "$work/seconds" | head -n "$top"
echo "total: $(awk '{ sum += $1 } END { print sum }' "$work/seconds") s"
echo
echo "tests, seconds:"
sort -rn "$work/tests" | head -n "$top"
