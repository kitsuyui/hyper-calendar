# hyper-calendar

A universal calendar and time library in Rust, targeting WebAssembly and
shared libraries.

It aims to cover the whole of what "a time" can mean: the international
standards (Unix time, ISO 8601, TAI and the rest of the uniform scales), the
Python-shaped conveniences (`datetime`, `date`, `time`, `timedelta`,
`humanize`), as many of the world's calendars as can be modelled at all, the
scales that ordinary calendars cannot reach in either direction, clocks on
other planets, and the relativistic corrections that make a science-fiction
timeline computable.

It contains no user interface. It computes and it formats; everything else is
someone else's job.

> **Status: early.** The foundations, the calendar abstraction and the first
> wave of calendars are implemented and tested. Nothing is listed as supported
> that is not tested and anchored to a published reference.
>
> **[`docs/supported.md`](docs/supported.md) is the index of everything that
> exists** — every calendar identifier with its range and day boundary, every
> holiday table, every unit, every feature. It is generated from the code and
> a test fails when it drifts, so it is the one list that cannot be out of
> date. [`docs/calendars.md`](docs/calendars.md) and
> [`docs/observances.md`](docs/observances.md) are the other half: what is
> *not* here, what is planned, and what is out of scope with reasons.

## What it is for

```rust
use hyper_calendar::{registry, Rd};

// The same day, in several calendars at once.
let today = Rd::from_unix_days(20_352);
for (calendar, described) in registry().describe_day(today) {
    match described {
        Ok(fields) => println!("{calendar}: {}-{:?}-{:?}", fields.year, fields.month, fields.day),
        // A calendar that was not in use on that day says so, rather than
        // being left out of the list.
        Err(refusal) => println!("{calendar}: {refusal}"),
    }
}
```

```rust
use hyper_calendar::hc_core::unix::{tai_from_utc, LeapPolicy, UtcInstant};

// UTC really does have a 23:59:60, and this library can name it.
// A leap second carries the POSIX timestamp of the second that *follows* it,
// so these two differ only in the flag.
let leap = UtcInstant { unix_seconds: 1_483_228_800, leap_second: true, subsec_attos: 0 };
let new_year = UtcInstant { unix_seconds: 1_483_228_800, leap_second: false, subsec_attos: 0 };

let at_leap = tai_from_utc(leap, LeapPolicy::Strict).expect("2016-12-31 had one");
let at_new_year = tai_from_utc(new_year, LeapPolicy::Strict).unwrap();

// In TAI they are a second apart, because 23:59:60 is a second of its own.
let gap = at_new_year.since_epoch().whole_seconds() - at_leap.since_epoch().whole_seconds();
assert_eq!(gap, 1);

// In POSIX time they are the same instant. That loss is what POSIX time is.
assert_eq!(leap.to_unix_lossy(), new_year.to_unix_lossy());
```

```rust
use hyper_calendar::hc_relativity::{
    constants::{LIGHT_YEAR, STANDARD_GRAVITY},
    worldline::{flip_and_burn_coordinate_time, flip_and_burn_proper_time},
};

// A 1g rocket to Andromeda: 2.5 million years for Earth, about 29 for the crew.
let distance = 2.5e6 * LIGHT_YEAR;
let year = 31_557_600.0;
let ship = flip_and_burn_proper_time(STANDARD_GRAVITY, distance).unwrap() / year;
let home = flip_and_burn_coordinate_time(STANDARD_GRAVITY, distance).unwrap() / year;
println!("{ship:.1} ship years, {home:.0} Earth years");
assert!(ship < 29.0 && home > 2.5e6);
```

> These three blocks are doctests. The crate includes this file with
> `#[doc = include_str!]` under `cfg(doctest)`, so a README example that stops
> compiling fails CI.

## The eight requirements, and where they live

| # | Requirement | Where |
| --- | --- | --- |
| 1 | Unix time, ISO 8601, TAI and the international date/weekday specifications, completely | [`hc-core`](crates/hc-core), [`hc-format`](crates/hc-format), [`docs/time-scales.md`](docs/time-scales.md) |
| 2 | The `datetime` / `date` / `time` / `timedelta` / `humanize` surface | [`hc-calendar`](crates/hc-calendar), [`hc-format`](crates/hc-format), [`hc-humanize`](crates/hc-humanize) |
| 3 | Every calendar we can know — lunar, Buddhist, Hijri, Japanese eras, Human Era — plus the 24 solar terms, the 72 pentads, national holidays and religious observances, listed and covered in stages | [`docs/calendars.md`](docs/calendars.md), [`docs/observances.md`](docs/observances.md), the `hc-calendars-*`, [`hc-seasons`](crates/hc-seasons) and [`hc-holiday`](crates/hc-holiday) crates |
| 4 | Data separated from algorithm, several calendars usable at once | [`docs/architecture.md`](docs/architecture.md), `hc-calendar`'s `Calendar` trait and `CalendarRegistry` |
| 5 | i18n, m17n, L10n | [`hc-i18n`](crates/hc-i18n), [`docs/i18n.md`](docs/i18n.md) |
| 6 | Planck time to cosmology, with significant figures, error bars and vague ranges | [`hc-uncertainty`](crates/hc-uncertainty), [`hc-deep-time`](crates/hc-deep-time), [`docs/scales-beyond-seconds.md`](docs/scales-beyond-seconds.md) |
| 7 | Mars sols and other bodies, and relativistic time dilation | [`hc-planetary`](crates/hc-planetary), [`hc-relativity`](crates/hc-relativity), [`docs/off-earth.md`](docs/off-earth.md) |
| 8 | Modular, so only what is needed gets compiled in | One crate per capability, and the `hyper-calendar` feature set — listed in [`docs/supported.md`](docs/supported.md); [ADR 0004](docs/adr/0004-one-crate-per-capability.md) |

## Design in one paragraph

Two pivots. Every **calendar** converts to and from a single integer day number
(Rata Die), so `n` calendars need `2n` conversions instead of `n²` and none of
them has to know the others exist. Every uniform **time scale** converts through
TAI, carried in the type system so a TT value cannot be mistaken for a TAI one.
UTC is deliberately not one of those scales — its seconds are SI seconds but its
labelling repeats one, so it goes through an explicit leap-second table and a
type that can say `23:59:60`. Everything else — holidays, locales, solar terms,
uncertainty — is data interpreted by a small shared algorithm.

The long version is [`docs/architecture.md`](docs/architecture.md); the rules it
serves are [`docs/policy.md`](docs/policy.md).

## Using it

```toml
[dependencies]
hyper-calendar = "0.1"
```

The default features are `std`, `civil`, `format` and `i18n` — Gregorian-family
dates, ISO 8601 parsing and formatting, and localisation. Everything else is
opt-in:

```toml
# Lunisolar calendars and the astronomical engine they need
hyper-calendar = { version = "0.1", features = ["lunar"] }

# The 24 solar terms, the 72 pentads and holidays
hyper-calendar = { version = "0.1", features = ["holiday"] }

# Everything
hyper-calendar = { version = "0.1", features = ["full"] }

# Embedded: no allocator-free path yet, but no std either
hyper-calendar = { version = "0.1", default-features = false, features = ["alloc", "libm", "civil"] }
```

### WebAssembly

```sh
cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --profile release-compact
```

The WebAssembly surface is a raw C-style ABI with no `wasm-bindgen`, so the
artefact carries no glue it did not ask for. A dependency-free JavaScript
binding, one method per export, is in
[`crates/hyper-calendar-wasm/js`](crates/hyper-calendar-wasm/js), with a
generator for a single-file module a page can open from disk and a script
that builds every layer and prints its size. See
[`crates/hyper-calendar-wasm`](crates/hyper-calendar-wasm).

### C shared library

```sh
cargo build -p hyper-calendar-ffi --release
```

Produces `libhyper_calendar_ffi.{so,dylib}` (`hyper_calendar_ffi.dll` on
Windows) and a static library. See
[`crates/hyper-calendar-ffi`](crates/hyper-calendar-ffi).

## What it will not do

Stated plainly, because a library that hides its limits is worse than one that
does less:

- **It will not predict leap seconds.** Past the announced IERS table, the
  strict policy returns an error. You can ask for extrapolation; you cannot get
  it by accident.
- **It will not pretend a Hijri holiday is exact.** Crescent visibility is
  decided per country, sometimes the night before. Computed dates are flagged
  approximate.
- **It will not claim historical calendars match what was proclaimed.**
  Pre-modern lunisolar dates depend on ΔT, which is itself reconstructed. Every
  astronomical calendar says so.
- **It will not invent a local time that does not exist.** A skipped DST hour
  returns `Nonexistent`, and an ambiguous one returns both candidates.
- **It will not give you a number without telling you how well it is known.**

## Development

```sh
lefthook install
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

See [`CONTRIBUTING.md`](CONTRIBUTING.md). Contributions of calendars, locales
and holiday rules are especially welcome — they are the parts that can only be
got right by people who use them.

## Licence

BSD-3-Clause. See [`LICENSE`](LICENSE).
