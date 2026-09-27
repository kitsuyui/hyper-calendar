# hyper-calendar

`hyper-calendar` is a calendar and time library written in Rust. It
converts dates between calendars, answers holiday questions, and converts
instants between time scales. It builds as a Rust library, a WebAssembly
module and a C shared library.

It has no user interface, no clock and no network access. The caller
supplies the current time. The library computes and formats, and nothing
else.

The project is at version 0.1. As of 2026-09-27 it has not been published
to crates.io, so a dependency names the Git repository. Every item it lists
as supported is tested and anchored to a published reference.

## What it covers

The counts below were read from [`docs/supported.md`](docs/supported.md)
and from `hc_i18n::data::LOCALES` on 2026-09-27. `supported.md` is
generated from the code, and a test fails when it drifts, so it is the
authority when a count here is out of date.

| What | Count | Where it is listed |
| --- | ---: | --- |
| Calendars in the registry, each with its own identifier | 197 | [supported.md § Calendars](docs/supported.md#calendars) |
| National holiday tables: 190 of the 193 UN member states, plus Hong Kong, Macau, Palestine, Taiwan and the Holy See | 195 | [supported.md § Holidays by country](docs/supported.md#holidays-by-country) |
| Religious and cultural tradition tables | 59 | [supported.md § Religious and cultural traditions](docs/supported.md#religious-and-cultural-traditions) |
| International observance tables (the 236 UN international days) | 1 | [supported.md § International observances](docs/supported.md#international-observances) |
| Exchange trading calendars, keyed by ISO 10383 Market Identifier Code | 42 | [supported.md § Exchange calendars](docs/supported.md#exchange-calendars) |
| Locales with their own vocabulary, besides the root | 41 | [docs/i18n.md](docs/i18n.md) and the [`hc-i18n` README](crates/hc-i18n) |
| Exactly defined units of time | 53 | [supported.md § Units](docs/supported.md#exactly-defined-units-of-time) |
| Readings of the sexagenary cycle, the sixty stem-branch pairs | 9 | [supported.md § Readings](docs/supported.md#readings-of-the-sexagenary-cycle) |
| Uniform time scales: TAI, TT, TCG, TDB, TCB, GPS, Galileo, BeiDou and NavIC time | 9 | [docs/time-scales.md](docs/time-scales.md) |

Beyond those tables, the library carries:

- UTC with its leap seconds, POSIX time, and timestamp formats such as TAI64,
  NTP, UUID versions 1 and 6, FAT and GNSS week numbers.
- The 24 solar terms, the 72 pentads and the Japanese almanac annotations.
- Time zones: fixed offsets, POSIX TZ strings and TZif files.
- ISO 8601, RFC 3339 and RFC 2822 text, `strftime` patterns, and relative
  phrases such as "3 days ago".
- Significant figures, error bars and Extended Date/Time Format (EDTF,
  ISO 8601-2) dates.
- Spans from the Planck time to cosmological ages, and the geological time
  scale.
- Mars time, the sols of ten Mars surface missions, and calendars for Titan
  and the Galilean moons.
- Special and general relativistic time dilation.

What is not carried yet, and what is out of scope, is listed in
[`docs/calendars.md`](docs/calendars.md) and
[`docs/observances.md`](docs/observances.md).

## Two pivots

The design rests on two canonical representations.

A **day** is a Rata Die number, `Rd`: an integer that counts days, with
day 1 being 1 January of year 1 in the proleptic Gregorian calendar
(the Gregorian rules extended backwards before 1582). Every calendar
converts to and from `Rd`, so *n* calendars need 2*n* conversions rather
than *n*².

An **instant** is a reading on a uniform time scale, and every such scale
converts through TAI (International Atomic Time). The scale is part of the
type, so a TT reading cannot be passed where a TAI reading is expected. UTC
is not one of these scales, because its labels repeat a second at each leap
second. It has its own type, which can name `23:59:60`.

[`docs/architecture.md`](docs/architecture.md) explains both.

## Crate map

The workspace is split into one crate per capability. The `hyper-calendar`
crate, called the facade, re-exports each one behind a Cargo feature. The
Feature column names that feature; "always" means the facade always
includes the crate.

| Crate | What it holds | Feature |
| --- | --- | --- |
| [`hc-core`](crates/hc-core) | Exact `Duration`, `Instant<S>` on the uniform scales, the leap-second table, epochs, timestamp formats | always |
| [`hc-units`](crates/hc-units) | Exactly defined units of time, as exact ratios of the second | `units` |
| [`hc-calendar`](crates/hc-calendar) | `Rd`, the `Calendar` and `DynCalendar` traits, the registry, the sexagenary cycle | `civil` |
| [`hc-calendars-solar`](crates/hc-calendars-solar) | Gregorian, Julian, the national reform calendars, ISO 8601, Coptic, Ethiopic, era counts, reform proposals, day counts | `civil` |
| [`hc-calendars-lunar`](crates/hc-calendars-lunar) | Hijri, Hebrew, Babylonian, Chinese, Korean, Vietnamese, Tibetan, Javanese, the Japanese lunisolar calendars | `lunar` |
| [`hc-calendars-equinox`](crates/hc-calendars-equinox) | Solar Hijri, Badíʿ and French Republican calendars fixed by an observed equinox | `equinox` |
| [`hc-calendars-indic`](crates/hc-calendars-indic) | Hindu lunisolar and solar calendars, Bikram Sambat, the Fasli years | `indic` |
| [`hc-calendars-regional`](crates/hc-calendars-regional) | Japanese imperial eras, Qing eras, Maya, Aztec, Burmese, Balinese Pawukon, Olympiads | `regional` |
| [`hc-astro`](crates/hc-astro) | ΔT, UT1, the Sun and Moon, rise and set, sidereal time | `astro` |
| [`hc-seasons`](crates/hc-seasons) | The 24 solar terms, the 72 pentads, 雑節, 六曜, the zodiac, the seasons | `seasons` |
| [`hc-almanac`](crates/hc-almanac) | 暦注: the 28 mansions, the nine stars, the twelve directs, the selected days | `almanac` |
| [`hc-fiscal`](crates/hc-fiscal) | Fiscal, tax and academic years | `fiscal` |
| [`hc-attributes`](crates/hc-attributes) | Birthstones, birth flowers, moon names, traditional month names | `attributes` |
| [`hc-name-days`](crates/hc-name-days) | Name-day lists by authority and edition, and a loader for licensed lists | `name-days` |
| [`hc-tz`](crates/hc-tz) | UTC offsets, POSIX TZ strings, a TZif reader, built-in zones, where each zone is | `tz` |
| [`hc-format`](crates/hc-format) | ISO 8601, RFC 3339 and RFC 2822 text, `strftime` patterns | `format` |
| [`hc-i18n`](crates/hc-i18n) | BCP 47 locales, plural rules, numbering systems, names, country names, zones' cities | `i18n` |
| [`hc-humanize`](crates/hc-humanize) | Relative times, spelled-out durations, Python `humanize` phrasing | `humanize` |
| [`hc-holiday`](crates/hc-holiday) | The holiday rule engine and the country, tradition, UN and exchange tables | `holiday` |
| [`hc-uncertainty`](crates/hc-uncertainty) | Significant figures, error bars, intervals, fuzzy instants, EDTF | `uncertainty` |
| [`hc-deep-time`](crates/hc-deep-time) | Planck time to cosmology, the geological time scale | `deep-time` |
| [`hc-orbital`](crates/hc-orbital) | Milankovitch orbital elements and insolation | `orbital` |
| [`hc-planetary`](crates/hc-planetary) | Mars time, mission sols, circad calendars, a table of 22 bodies | `planetary` |
| [`hc-relativity`](crates/hc-relativity) | Time dilation and worldlines | `relativity` |
| `hyper-calendar` | The facade, and the `civil` layer shaped like Python's `datetime` | — |
| [`hyper-calendar-wasm`](crates/hyper-calendar-wasm) | The WebAssembly module and its JavaScript binding | — |
| [`hyper-calendar-ffi`](crates/hyper-calendar-ffi) | The C shared and static library | — |

Some features pull in others; `holiday`, for example, needs every calendar
crate. [`supported.md` § Facade features](docs/supported.md#facade-features)
lists what each feature implies.

## Using it from Rust

```toml
[dependencies]
hyper-calendar = { git = "https://github.com/kitsuyui/hyper-calendar" }
```

The default features are `std`, `civil`, `format` and `i18n`: Gregorian
dates, ISO 8601 text, and localisation. Every other capability is opt-in:

```toml
# Lunar and lunisolar calendars, and the astronomy they need
hyper-calendar = { git = "https://github.com/kitsuyui/hyper-calendar", features = ["lunar"] }

# Holidays and observances
hyper-calendar = { git = "https://github.com/kitsuyui/hyper-calendar", features = ["holiday"] }

# Everything
hyper-calendar = { git = "https://github.com/kitsuyui/hyper-calendar", features = ["full"] }

# no_std: no standard library, with software floating-point math
hyper-calendar = { git = "https://github.com/kitsuyui/hyper-calendar", default-features = false, features = ["alloc", "libm", "civil"] }
```

One day, in several calendars (features `lunar` and `regional`, or `full`):

```rust
use hyper_calendar::civil::Date;
use hyper_calendar::hc_calendar::CalendarId;
use hyper_calendar::registry;

let day = Date::new(2026, 9, 27)?.fixed();
assert_eq!(day.0, 739_886); // the Rata Die number

let calendars = registry();

// 16 Tishri 5787. The Hebrew year counts its months from Tishri.
let hebrew = calendars.get(CalendarId("hebrew")).expect("registered");
let fields = hebrew.fixed_to_fields(day)?;
assert_eq!((fields.year, fields.day), (5787, Some(16)));

// Reiwa 8, 27 September.
let japanese = calendars.get(CalendarId("japanese")).expect("registered");
let fields = japanese.fixed_to_fields(day)?;
assert_eq!((fields.era, fields.year), (Some("reiwa"), 8));

// Every registered calendar at once. A calendar that was not in use on the
// day returns an error for it rather than being left out.
let described = calendars.describe_day(day);
assert_eq!(described.len(), 197);
# Ok::<(), hyper_calendar::CalendarError>(())
```

A leap second, which POSIX time cannot name:

```rust
use hyper_calendar::hc_core::unix::{tai_from_utc, LeapPolicy, UtcInstant};

// 2016-12-31 23:59:60 UTC, and the second after it, 2017-01-01 00:00:00.
// POSIX time gives both the same timestamp; only the flag differs.
let leap = UtcInstant { unix_seconds: 1_483_228_800, leap_second: true, subsec_attos: 0 };
let new_year = UtcInstant { unix_seconds: 1_483_228_800, leap_second: false, subsec_attos: 0 };

let at_leap = tai_from_utc(leap, LeapPolicy::Strict).expect("2016-12-31 had one");
let at_new_year = tai_from_utc(new_year, LeapPolicy::Strict).expect("in the table");

// In TAI they are one second apart.
let gap = at_new_year.since_epoch().whole_seconds() - at_leap.since_epoch().whole_seconds();
assert_eq!(gap, 1);

// In POSIX time they are the same second.
assert_eq!(leap.to_unix_lossy(), new_year.to_unix_lossy());
```

A 1 g rocket to the Andromeda galaxy (feature `relativity`):

```rust
use hyper_calendar::hc_relativity::{
    constants::{LIGHT_YEAR, STANDARD_GRAVITY},
    worldline::{flip_and_burn_coordinate_time, flip_and_burn_proper_time},
};

// Accelerate at 1 g to the midpoint, then decelerate at 1 g.
let distance = 2.5e6 * LIGHT_YEAR;
let julian_year = 31_557_600.0;
let ship = flip_and_burn_proper_time(STANDARD_GRAVITY, distance).expect("finite") / julian_year;
let earth = flip_and_burn_coordinate_time(STANDARD_GRAVITY, distance).expect("finite") / julian_year;
assert!((ship - 28.6).abs() < 0.05); // about 28.6 years aboard
assert!(earth > 2.5e6); // over 2.5 million years at home
```

These three blocks are doctests. The facade includes this file with
`#[doc = include_str!]` under `cfg(doctest)`, so
`cargo test -p hyper-calendar --doc --all-features` compiles and runs them.

## Using it from WebAssembly and JavaScript

```sh
cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown \
  --profile release-compact --features full
```

The module is
`target/wasm32-unknown-unknown/release-compact/hyper_calendar_wasm.wasm`.
It exports plain C-style functions over integers and linear memory, with no
`wasm-bindgen` glue. The dependency-free ES module
[`crates/hyper-calendar-wasm/js/hyper-calendar.js`](crates/hyper-calendar-wasm/js/hyper-calendar.js)
wraps each export in a method. Under Node 22, from the repository root:

```js
import { readFile } from "node:fs/promises";
import { load } from "./crates/hyper-calendar-wasm/js/hyper-calendar.js";

const wasm = "target/wasm32-unknown-unknown/release-compact/hyper_calendar_wasm.wasm";
const hc = await load(await readFile(wasm));

const day = hc.gregorianToFixed(2026, 9, 27);
console.log(day);                                   // 739886
console.log(hc.formatIsoDate(day));                 // 2026-09-27
const japanese = hc.describeDay(day, "ja").find((row) => row.id === "japanese");
console.log(japanese.formatted);                    // 令和8年9月27日
```

The module is built in layers, one Cargo feature each, so a page loads only
what it uses. [`crates/hyper-calendar-wasm`](crates/hyper-calendar-wasm)
lists every export, its layer and its size, and says how memory, text and
errors cross the boundary.

## Using it from C

```sh
cargo build -p hyper-calendar-ffi --release
```

This builds `libhyper_calendar_ffi.dylib` on macOS,
`libhyper_calendar_ffi.so` on Linux and `hyper_calendar_ffi.dll` on
Windows, and the static `libhyper_calendar_ffi.a`, in `target/release/`.
There is no generated header. Declare the prototypes you call:

```c
#include <stdint.h>
#include <stdio.h>

typedef int HcStatus; /* 0 is success */
HcStatus hc_gregorian_to_fixed(int64_t year, uint8_t month, uint8_t day,
                               int64_t *out_fixed);
HcStatus hc_format_iso_date(int64_t fixed, char *buf, size_t cap,
                            size_t *out_len);

int main(void) {
    int64_t day;
    char text[32];
    size_t len;
    if (hc_gregorian_to_fixed(2026, 9, 27, &day) != 0) return 1;
    if (hc_format_iso_date(day, text, sizeof text, &len) != 0) return 1;
    printf("%lld %s\n", (long long)day, text); /* 739886 2026-09-27 */
    return 0;
}
```

```sh
cc example.c -Ltarget/release -lhyper_calendar_ffi -o example
DYLD_LIBRARY_PATH=target/release ./example   # LD_LIBRARY_PATH on Linux
```

[`crates/hyper-calendar-ffi`](crates/hyper-calendar-ffi) lists every entry
point and its status codes.

## Policies

[`docs/policy.md`](docs/policy.md) is the standing decision record. Each
paragraph below summarises one section.

**English is the repository language** (§1). Code, documentation and
messages are in English. Text in other languages is data: month names,
era names, holiday names and the tests that check them.

**Data is separated from algorithm** (§2). A calendar, a holiday rule and
a locale are data read by shared code. Adding a country's holidays should
add a table, not a branch. Shared arithmetic has one implementation, in
the crate that owns the idea.

**Precision is stated** (§3). Exact values are computed exactly, with
integers. A value from a fitted model states its accuracy and the era it
holds for. A value that is not known is marked as not known.

**The library refuses to guess** (§4). It does not predict leap seconds or
extrapolate UT1. It flags computed Hijri holidays as approximate. It
returns `Nonexistent` for a local time that a daylight-saving change
skipped.

**Competing conventions get names** (§5). Where authorities disagree, each
convention is its own calendar identifier, such as `japanese-northern`
and `japanese-southern`, not a parameter with a default.

**Modularity is a compile-time property** (§6). Each capability is a crate,
and the facade exposes it as a feature. The default features are small.
Every crate builds without the standard library.

**Correctness is demonstrated** (§7). Every conversion is round-trip tested
over its range. Every algorithm is anchored to a published reference
value. CI runs tests, lints, `no_std`, WebAssembly and C builds, and a
dependency audit on every pull request.

**`unwrap` and `expect` are forbidden outside tests** (§8). Fallible
operations return `Result`. The arithmetic operators on durations and dates
are the stated exception: they panic on overflow, and each has a
`checked_*` twin.

**No dependencies without a reason** (§9). The one external dependency is
`libm`, optional, for floating-point math without the standard library.
Algorithms are written from published rules, not copied from other
libraries' code.

**Recurring events need an authority** (§10). A periodic event, such as the
Olympic Games, is in scope when an external body defines its set. A list
this project would have to curate is out of scope.

**Every rule from the literature cites it** (§11). Each rule, table and
reference date names its source, in the code and in
[`docs/references.bib`](docs/references.bib). A source that was not read
is named as not read.

**A complex system is written up before it is coded** (§12). A calendar or
holiday regime that a maintainer cannot be expected to know gets a document
under [`docs/systems/`](docs/systems/README.md) first.

**Scope** (§13). The library computes and formats. It has no user
interface, no I/O beyond reading a TZif file, no clock, no network access
and no state that outlives a call: `hc_core::memo` caches pure results
within one, in thread-local storage emptied when the call's scope ends.

## Documents

| Document | What it answers |
| --- | --- |
| [`docs/supported.md`](docs/supported.md) | What exists: every calendar, table, unit and feature. Generated from the code |
| [`docs/architecture.md`](docs/architecture.md) | How the crates fit together and why |
| [`docs/policy.md`](docs/policy.md) | The rules a change must follow |
| [`docs/adr/`](docs/adr/README.md) | The design decisions that could have gone another way |
| [`docs/calendars.md`](docs/calendars.md) | Calendars: status, plans, and what is out of scope |
| [`docs/observances.md`](docs/observances.md) | Holidays and observances: status, plans, and what is out of scope |
| [`docs/time-scales.md`](docs/time-scales.md) | TAI, UTC, UT1, ΔT, epochs and timestamp formats |
| [`docs/i18n.md`](docs/i18n.md) | Locales, plural rules, names and text direction |
| [`docs/python-parity.md`](docs/python-parity.md) | How Python's `datetime` and `humanize` map onto this library |
| [`docs/scales-beyond-seconds.md`](docs/scales-beyond-seconds.md) | Uncertain times, deep time and the ice-age cycles |
| [`docs/off-earth.md`](docs/off-earth.md) | Time on other bodies, and relativity |
| [`docs/systems/`](docs/systems/README.md) | One document per complex calendar or holiday system, with worked examples |
| [`docs/references.bib`](docs/references.bib) | Every source cited more than once, in BibTeX |
| Each crate's `README.md` | What the crate claims, how accurate it is, and what it refuses |

## Development

```sh
lefthook install
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

[`CONTRIBUTING.md`](CONTRIBUTING.md) lists every check CI runs. Each of
`docs/calendars.md`, `docs/observances.md` and `docs/i18n.md` ends with a
checklist for adding a calendar, a country or a locale.

## Licence

BSD-3-Clause. See [`LICENSE`](LICENSE).
