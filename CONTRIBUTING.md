# Contributing to hyper-calendar

`hyper-calendar` is a universal calendar and time library. Contributions are
welcome, particularly calendars, locales and holiday rules — those are the
parts that can only be got right by people who actually use them.

## Before you start

Read [`docs/policy.md`](docs/policy.md). It is short, and it explains the
standing decisions (English as the working language, data separated from
algorithm, stated precision, no guessing) that a change has to fit.

If you are adding a calendar, a locale or a country's holidays, the relevant
"Adding a …" checklist is at the bottom of
[`docs/calendars.md`](docs/calendars.md),
[`docs/i18n.md`](docs/i18n.md) or
[`docs/observances.md`](docs/observances.md).

## Development setup

```sh
git clone https://github.com/kitsuyui/hyper-calendar.git
cd hyper-calendar
lefthook install   # installs the pre-commit and pre-push hooks
```

`lefthook` runs the same checks as CI, locally.

## Running the checks

```sh
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
cargo build -p hyper-calendar --no-default-features --features alloc   # no_std
cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown     # WebAssembly
cargo build -p hyper-calendar-ffi --release                            # shared library
```

All of these must pass. `unwrap()` and `expect()` are deny-level lints outside
tests — see [`docs/policy.md`](docs/policy.md) §8.

**Update your tools before pushing.** CI installs the latest stable Rust and
the latest `typos` on every run, so a local pass with an older toolchain is not
a guarantee — a newer clippy or a newer dictionary will find things yours does
not:

```sh
rustup update stable
cargo install typos-cli
```

The `lefthook` pre-push hook prints both versions so a stale toolchain shows up
in the hook output rather than in a failed pull request.

## What a good change looks like

- **One concern per pull request.** A new calendar, or a bug fix, not both.
- **Round-trip tests across the full supported range**, in a loop, not at three
  hand-picked dates.
- **At least one anchor to a published reference value**, cited in a comment.
  "I checked it against another library" is not a citation; the other library
  may be wrong too.
- **Comments that explain why.** Where the constant came from, what accuracy is
  claimed, what the function deliberately refuses to do. Not a narration of the
  code.
- **Error paths tested.** If a function can return `MonthOutOfRange`, make it.

## Reporting a wrong date

Wrong dates are the most valuable bug reports this project can get. Please
include:

- The calendar and the input.
- What the library returned.
- What it should have returned, **and a source** — a statute, a gazette, an
  almanac, an ephemeris, a national calendar authority.
- Whether the discrepancy is systematic or a single day.

A date that was historically *proclaimed* differently from what the rules
compute is still a valid report; it usually means the calendar needs a
documented range or an exception table.

## Security

See [SECURITY.md](SECURITY.md). Do not open a public issue with exploit
details.

## Licence

By contributing you agree that your contributions are licensed under the
BSD-3-Clause licence.
