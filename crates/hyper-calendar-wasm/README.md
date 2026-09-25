# hyper-calendar-wasm

A dependency-free WebAssembly surface for `hyper-calendar`.

## Why a raw ABI and not `wasm-bindgen`

The workspace has no external dependencies ([ADR
0005](../../docs/adr/0005-no-external-dependencies.md)), and that is worth
more here than anywhere else. A calendar library is a leaf dependency of a web
application; whatever it drags in, the bundle carries. So the exports are
plain `extern "C"` functions over integers and linear memory, which every
WebAssembly host can call with no glue at all. The cost is that the JavaScript
side does the string marshalling. That is written once, in the binding under
[`js/`](js/) described below, and it is a few hundred lines the caller can
read.

## Memory and text

The module owns its allocator. `hc_alloc` hands out a block, the caller writes
into it or reads out of it, and `hc_free` takes it back. Every block must be
freed with the same length it was allocated with.

Text is UTF-8 and is *not* NUL-terminated. Functions that produce text return
the byte length written, because a length is cheaper and safer than a scan;
functions that consume text take a pointer and a length.

## Errors

A function that returns a day number or a count returns a negative sentinel on
failure rather than trapping, because a trap tears down the instance and takes
any other work in it with it. Every sentinel is at or below `HC_ERR_FLOOR`,
which is more than a thousand times the age of the universe in days, so no
legitimate result can be mistaken for one.

## Lines and cells

Every export that answers with more than one value writes UTF-8 lines, one
per entry, each ending in `\n`, with the cells of a line separated by `\t`.
The column order of each export is fixed, stated below and on the export,
and only ever grows at the end, so a page that reads columns by position
keeps working. A cell with nothing to say is empty, never a placeholder, and
no cell contains a tab or a line break: the few source strings that carry
one have it replaced by a space. Numbers are written in plain decimal
notation, however large or small.

Called with a null `buffer`, such an export returns the byte length the text
needs, so the caller can allocate exactly and call again; called with a
buffer that is too small it returns `HC_ERR_BUFFER_TOO_SMALL` and writes
nothing.

## Building and calling

```sh
cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --profile release-compact
```

The module is `target/wasm32-unknown-unknown/release-compact/hyper_calendar_wasm.wasm`.
Any WebAssembly host can call it as it is; a page calls it through the
binding below, which is the marshalling written once.

## Loading from JavaScript

[`js/hyper-calendar.js`](js/hyper-calendar.js) is an ES module with no
dependencies and no build step, typed by the hand-written
[`js/hyper-calendar.d.ts`](js/hyper-calendar.d.ts) beside it. It lives
with the crate because it is the same surface the crate's README describes,
column for column, and [`js/readme.test.js`](js/readme.test.js) holds the
two to each other. Copy the two files next to the page, or serve them from
wherever the page is served.

```js
import { load, HcError } from "./hyper-calendar.js";

const hc = await load(fetch("hyper_calendar_wasm.wasm"));

const rd = hc.gregorianToFixed(2026, 9, 21);        // 739880
hc.formatIsoDate(rd);                                // "2026-09-21"
hc.describeDay(rd, "ja-JP").find((row) => row.id === "japanese");
// { id: "japanese", era: "reiwa", eraLabel: "令和", year: 8, month: 9, day: 21, ... }
hc.holidaysOn(rd);                                   // [{ table: "JP", name: ..., kind: "public", ... }, ...]
hc.fixedFromUnixInZone(Date.now() / 1000 | 0, "Asia/Tokyo");

try {
  hc.parseIsoDate("2026-02-30");
} catch (error) {
  error instanceof HcError;      // true
  error.name;                    // "invalid-date"
  error.constant;                // "HC_ERR_INVALID_DATE"
  error.code;                    // -9000000000000001n
}
```

`load(source)` takes the module in whatever form the page has it — a
`WebAssembly.Module` or `Instance`, its bytes as an `ArrayBuffer` or
`Uint8Array`, a `Response`, or a `URL` or string to fetch — or a promise of
any of those, and resolves to a `HyperCalendar` with one method per export:

| Method | Export | Answers with |
| --- | --- | --- |
| `alloc(len)`, `free(pointer, len)` | `hc_alloc`, `hc_free` | a pointer; nothing |
| `version()` | `hc_version` | a string |
| `gregorianToFixed(year, month, day)` | `hc_gregorian_to_fixed` | a fixed day number |
| `gregorianYear(rd)`, `gregorianMonth(rd)`, `gregorianDay(rd)`, `weekday(rd)`, `dayOfYear(rd)` | the `hc_gregorian_*`, `hc_weekday`, `hc_day_of_year` | a number |
| `isLeapYear(rd)`, `dayHasLeapSecond(unix)` | `hc_is_leap_year`, `hc_day_has_leap_second` | a boolean |
| `fixedFromUnix(unix)`, `unixFromFixed(rd)`, `taiMinusUtc(unix, strict)` | `hc_fixed_from_unix`, `hc_unix_from_fixed`, `hc_tai_minus_utc` | a number |
| `formatIsoDate(rd)`, `parseIsoDate(text)` | `hc_format_iso_date`, `hc_parse_iso_date` | a string; a fixed day number |
| `describeDay(rd, locale)` | `hc_describe_day` | `DescribedDay[]`, one per calendar |
| `holidayIsDayOff(code, region, rd)` | `hc_holiday_is_day_off` | a boolean |
| `holidaysInYear(code, region, year)` | `hc_holidays_in_year` | `HolidayInYear[]` |
| `holidayCodes()` | `hc_holiday_codes` | `string[]` |
| `holidaysOn(rd)` | `hc_holidays_on` | `HolidayOn[]` |
| `termInEffect(rd, meridian)`, `pentadInEffect(rd, meridian)` | `hc_term_in_effect`, `hc_pentad_in_effect` | a `TermInEffect` |
| `placeYearsAgo(years, stdDev)`, `cosmicEvents()`, `geologicIntervals(rank)` | `hc_place_years_ago`, `hc_cosmic_events`, `hc_geologic_intervals` | `DeepTimeRow[]` |
| `fixedFromUnixInZone(unix, zone)`, `unixFromFixedInZone(rd, zone)` | `hc_fixed_from_unix_in_zone`, `hc_unix_from_fixed_in_zone` | a number |
| `loadZone(name, tzif)` | `hc_zone_load` | nothing |
| `skyAt(unix)` | `hc_sky_at` | a `Sky` |
| `solarTermsBetween(from, to)`, `moonPhasesBetween(from, to)` | `hc_solar_terms_between`, `hc_moon_phases_between` | `SkyEvent[]` |

Each method does what a page would otherwise write by hand:

- **Text** crosses as UTF-8 in `hc_alloc` blocks that are freed with the
  length they were allocated with, whether or not the call succeeds.
- **Lines** are decoded into objects by the column tables below, with an
  empty cell as `null`, a `0`/`1` cell as a boolean, and the extra fields
  of a calendar as an object. The column order is the README's, and the
  tests assert it, so a column moved in the source fails the build.
- **`i64`** crosses as `BigInt`; every result leaves as a number, which
  every day number, year and timestamp fits, and one that does not is an
  `unsafe-integer` error rather than a rounded value. An `i64` argument may
  be a number or a `BigInt`.
- **Sentinels** are thrown as `HcError`: `name` is the sentinel's name in
  lower case without the prefix (`invalid-date`, `out-of-range`,
  `buffer-too-small`, `no-data`, `null-pointer`, `unknown`, `not-utf8`,
  `malformed`), `constant` its `HC_ERR_*` name, `code` its value as a
  `BigInt`, and `export` the export that returned it.
- **Buffers.** An export that writes text is first offered a buffer of
  `initialCapacity` bytes — 64 KiB unless `load(source, { initialCapacity })`
  says otherwise — which every ordinary answer fits, so the module computes
  its text once. When that comes back `HC_ERR_BUFFER_TOO_SMALL`, an export
  that measures is asked the exact length with a null buffer and called
  again; `hc_version` and `hc_format_iso_date`, which cannot measure, are
  offered double until they fit.
- **Layers.** `load()` works with any build. A method whose export the
  build lacks throws `HcError` with the name `not-exported` when it is
  called, not when the module is loaded, so one page can start on `civil`
  and load `full` later. `hc.has("describeDay")` asks first, and
  `hc.layers()` names the features the build carries.

### Tests

[`js/*.test.js`](js/) run under `node --test` with Node's built-ins alone,
against the built module: every method, every line format held to the
README's columns and count, the sentinels as thrown errors, the buffer
protocol with a capacity too small to fit anything, a `civil` build's
refusals by name, and the embedded module below. CI runs them on every
pull request; locally,

```sh
scripts/wasm-js-test.sh      # builds civil and full, then node --test
```

or, with the module already built, `node --test "crates/hyper-calendar-wasm/js/*.test.js"`
from the repository root, with `CARGO_TARGET_DIR` honoured and `HC_WASM`
and `HC_WASM_CIVIL` naming the two builds outright.

### The embedded module

A page opened from disk cannot fetch a `.wasm` file: browsers refuse
`fetch` of a `file:` URL. For that page,

```sh
scripts/wasm-layers.sh                  # or any build of the module
node scripts/wasm-embed.mjs             # --wasm <file> for another build
```

writes `target/wasm-js/hyper-calendar.embedded.js`: the binding above with
the module's bytes inside it as base64, decoded with `atob` and bound by a
`load(options)` that takes no source and fetches nothing. It is one
self-contained ES module; `hyper-calendar.embedded.d.ts` types it. It is
generated, not committed — CI uploads it with the layered builds below —
and it is 1.91 MiB (2,001,994 bytes) for the `full` layer of 2026-09-25,
base64 being four thirds of the module.

### tzdata beside the module

The module's seventeen built-in zones carry only their current rules (the
time zones section below). For a zone's history, or any other zone, a page
hands `loadZone(name, bytes)` the zone's TZif file, and

```sh
scripts/wasm-tzdata.sh                  # [<output directory>]
```

copies the seventeen from the host's `/usr/share/zoneinfo` (or `ZONEINFO`)
into `target/tzdata/`, laid out as the database lays them out
(`tzdata/Asia/Tokyo`), with `VERSION` holding the database release read
from the host's `+VERSION` or `tzdata.zi`, and `SOURCE` saying where they
came from. The files are the IANA Time Zone Database's own compiled TZif
files, copied unmodified; the database is in the public domain. CI uploads
the directory beside the module, from the Ubuntu runner's `tzdata` package,
and the README of `hc-tz` names the release its built-in table was read
from. A page then does

```js
const tzif = await (await fetch("tzdata/Europe/Rome")).arrayBuffer();
hc.loadZone("Europe/Rome", tzif);
hc.fixedFromUnixInZone(331_250_400, "Europe/Rome");   // 1980-07-01, by the 1980 rules
```

## Layers

The exports come in layers, each a Cargo feature, so a page loads what it
paints first and fetches the rest later. Every feature builds on its own;
`calendars` does not need `holiday`, so a page can show every calendar
before it loads the holiday tables.

| Feature | Exports | Brings in | Bytes | Size |
| --- | --- | --- | ---: | ---: |
| `civil` *(default)* | Gregorian dates, ISO 8601 text, POSIX time, the TAI–UTC bridge | `hc-calendar`, `hc-calendars-solar`, `hc-format` | 35,495 | 35 KiB |
| `calendars` | `hc_describe_day`: one day in every registered calendar, in a locale | every `hc-calendars-*` crate, `hc-astro`, `hc-i18n` | 414,920 | 405 KiB |
| `holiday` | the four `hc_holiday*` exports and `hc_holidays_on` | `hc-holiday` and everything it dates by | 1,017,832 | 994 KiB |
| `seasons` | `hc_term_in_effect`, `hc_pentad_in_effect` | `hc-seasons`, `hc-astro` | 88,259 | 86 KiB |
| `deep-time` | `hc_place_years_ago`, `hc_cosmic_events`, `hc_geologic_intervals` | `hc-deep-time`, `hc-uncertainty` | 101,842 | 99 KiB |
| `tz` | `hc_fixed_from_unix_in_zone`, `hc_unix_from_fixed_in_zone`, `hc_zone_load` | `hc-tz` | 57,414 | 56 KiB |
| `sky` | `hc_sky_at`, `hc_solar_terms_between`, `hc_moon_phases_between` | `hc-astro`, `hc-seasons` | 96,550 | 94 KiB |
| `full` | all of the above | everything | 1,466,491 | 1.40 MiB |

The sizes are of the `release-compact` profile for
`wasm32-unknown-unknown`, as [`scripts/wasm-layers.sh`](../../scripts/wasm-layers.sh)
printed them on 2026-09-25 with rustc 1.98.1:

```sh
scripts/wasm-layers.sh
# cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --profile release-compact --no-default-features --features <layer>
```

The script leaves each layer at `target/wasm-layers/hyper_calendar_wasm.<feature>.wasm`
and prints the table; CI runs it on every pull request and uploads the
eight files, the embedded module and `tzdata/` as one workflow artifact.
`hc_alloc`, `hc_free` and `hc_version` are in every build.

## What is exported

The two tables below are rendered from `src/lib.rs` by
[`crates/hyper-calendar/tests/abi.rs`](../hyper-calendar/tests/abi.rs), which
fails when they drift, and [`tests/readme.rs`](tests/readme.rs) holds every
row's feature to the source's `#[cfg]`. An export without a row here does
not pass CI.

<!-- generated by crates/hyper-calendar/tests/abi.rs: start -->

### Exports

32 functions. Types are the WebAssembly ones: `i64` crosses into JavaScript as a `BigInt`, everything else as a `number`, and a pointer is a byte offset into `memory`. The feature column is the Cargo feature the module has to be built with for the export to exist.

| Export | Feature | What it does |
| --- | --- | --- |
| `hc_alloc(len: usize) -> *mut u8` | always | Allocate `len` bytes of linear memory and return a pointer to them. |
| `hc_free(pointer: *mut u8, len: usize)` | always | Return a block from `hc_alloc` to the allocator. |
| `hc_version(buffer: *mut u8, capacity: usize) -> i64` | always | The library version as UTF-8, returning the byte length written. |
| `hc_gregorian_to_fixed(year: i64, month: u32, day: u32) -> i64` | `civil` | The fixed day number of a proleptic Gregorian date, or an error sentinel. |
| `hc_gregorian_year(fixed: i64) -> i64` | `civil` | The Gregorian year on a fixed day, or an error sentinel. |
| `hc_gregorian_month(fixed: i64) -> i64` | `civil` | The Gregorian month on a fixed day, 1 through 12, or an error sentinel. |
| `hc_gregorian_day(fixed: i64) -> i64` | `civil` | The Gregorian day of the month on a fixed day, or an error sentinel. |
| `hc_weekday(fixed: i64) -> i64` | `civil` | The ISO weekday of a fixed day, Monday = 1 through Sunday = 7. |
| `hc_day_of_year(fixed: i64) -> i64` | `civil` | The 1-based day of the year on a fixed day, or an error sentinel. |
| `hc_is_leap_year(fixed: i64) -> i64` | `civil` | Whether the Gregorian year on a fixed day is a leap year: 1, 0, or an error sentinel. |
| `hc_fixed_from_unix(unix_seconds: i64) -> i64` | `civil` | The fixed day a POSIX timestamp falls on, in UTC. |
| `hc_tai_minus_utc(unix_seconds: i64, strict: i32) -> i64` | `civil` | `TAI - UTC` in whole seconds at a POSIX timestamp. |
| `hc_day_has_leap_second(unix_seconds: i64) -> i64` | `civil` | Whether the UTC day containing a POSIX timestamp ends with an inserted leap second: 1, 0, or an error sentinel. |
| `hc_unix_from_fixed(fixed: i64) -> i64` | `civil` | The POSIX timestamp of midnight UTC on a fixed day. |
| `hc_format_iso_date(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `civil` | Render a fixed day as an ISO 8601 date, returning the byte length written. |
| `hc_parse_iso_date(buffer: *const u8, len: usize) -> i64` | `civil` | Parse an ISO 8601 date from UTF-8, returning its fixed day number. |
| `hc_describe_day(fixed: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | One fixed day in every registered calendar, as UTF-8 lines, returning the byte length written. |
| `hc_holiday_is_day_off(code: *const u8, code_len: usize, region: *const u8, region_len: usize, fixed: i64) -> i64` | `holiday` | Whether a fixed day is a day off in a holiday table: 1, 0, or an error sentinel. |
| `hc_holidays_in_year(code: *const u8, code_len: usize, region: *const u8, region_len: usize, year: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The holidays of a Gregorian year in a table, as UTF-8 lines, returning the byte length written. |
| `hc_holiday_codes(buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The identifier of every holiday table, one per line, returning the byte length written. |
| `hc_holidays_on(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | Every holiday on one fixed day across every table, as UTF-8 lines, returning the byte length written. |
| `hc_term_in_effect(fixed: i64, meridian: *const u8, meridian_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `seasons` | The solar term in effect on a fixed day at a meridian, as one UTF-8 line, returning the byte length written. |
| `hc_pentad_in_effect(fixed: i64, meridian: *const u8, meridian_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `seasons` | The pentad (候) in effect on a fixed day at a meridian, as one UTF-8 line, returning the byte length written. |
| `hc_place_years_ago(years_ago: f64, std_dev_years: f64, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | A moment some years before the present, placed in every chronology at once, as UTF-8 lines, returning the byte length written. |
| `hc_cosmic_events(buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | Every cosmic epoch and every dated cosmic event, as UTF-8 lines, returning the byte length written. |
| `hc_geologic_intervals(rank: u32, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | Every interval of one rank of the geologic time scale, as UTF-8 lines, returning the byte length written. |
| `hc_fixed_from_unix_in_zone(unix_seconds: i64, zone: *const u8, zone_len: usize) -> i64` | `tz` | The fixed day a POSIX timestamp falls on by the wall clock of a zone, or an error sentinel. |
| `hc_unix_from_fixed_in_zone(fixed: i64, zone: *const u8, zone_len: usize) -> i64` | `tz` | The POSIX timestamp at which a fixed day begins by the wall clock of a zone, or an error sentinel. |
| `hc_zone_load(name: *const u8, name_len: usize, tzif: *const u8, tzif_len: usize) -> i64` | `tz` | Give the module a zone's TZif data under an IANA name, returning 0. |
| `hc_sky_at(unix_seconds: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Sun and the Moon at a POSIX timestamp, as one UTF-8 line, returning the byte length written. |
| `hc_solar_terms_between(from_unix: i64, to_unix: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Every solar term whose instant falls in `[from_unix, to_unix)`, as UTF-8 lines, returning the byte length written. |
| `hc_moon_phases_between(from_unix: i64, to_unix: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Every new moon, first quarter, full moon and last quarter whose instant falls in `[from_unix, to_unix)`, as UTF-8 lines, returning the byte length written. |

### Error sentinels

A function that returns `i64` returns one of these instead of trapping. All are at or below `HC_ERR_FLOOR`, which no day number reaches.

| Sentinel | Value | Meaning |
| --- | --- | --- |
| `HC_ERR_FLOOR` | -9_000_000_000_000_000 | Any return value at or below this is an error sentinel, not a result. |
| `HC_ERR_INVALID_DATE` | -9_000_000_000_000_001 | The date does not exist. |
| `HC_ERR_OUT_OF_RANGE` | -9_000_000_000_000_002 | A value was outside the supported range. |
| `HC_ERR_BUFFER_TOO_SMALL` | -9_000_000_000_000_003 | The supplied buffer was too small. |
| `HC_ERR_NO_DATA` | -9_000_000_000_000_004 | The requested model has no data for this value. |
| `HC_ERR_NULL_POINTER` | -9_000_000_000_000_005 | A pointer was null with a non-zero length. |
| `HC_ERR_UNKNOWN` | -9_000_000_000_000_006 | The requested table or identifier is not known. |
| `HC_ERR_NOT_UTF8` | -9_000_000_000_000_007 | Text was not valid UTF-8. |
| `HC_ERR_MALFORMED` | -9_000_000_000_000_008 | Data was not in the format the call expects. |
<!-- generated by crates/hyper-calendar/tests/abi.rs: end -->

## Every calendar

`hc_describe_day(fixed, locale_ptr, locale_len, buffer, capacity)` needs the
`calendars` feature and writes one line per calendar the facade registers,
in registry order — the Gregorian family first, then the lunar, equinox,
Indic and regional calendars. `locale` is a BCP 47 tag such as `ja-JP` or
`zh-Hans`; a tag that does not parse, or that no data answers for, falls
back to the root locale `und`, as `hc-i18n` does, whose month names are
CLDR's `M01`..`M12` — ask for `en` for English. A null pointer with a zero
length is `und` too.

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the calendar's identifier, `gregory`, `chinese`, `japanese`, ... |
| 2 | name | its English name |
| 3 | era | the era code, `reiwa`, `AD`, `AH`, `roc`, ..., or empty for a calendar without eras |
| 4 | era label | the era's name in the locale, 令和, or empty |
| 5 | year | the year, as the calendar counts it |
| 6 | month | the month's ordinal from 1, or empty for a calendar without months |
| 7 | leap month | `1` for an intercalary month, else `0` |
| 8 | month label | the month's name in the locale, 閏二月, or the calendar's own name for it, or empty |
| 9 | day | the day of the month, or empty |
| 10 | leap day | `1` for a repeated day, else `0` |
| 11 | extras | the calendar's extra fields, `baktun=13;katun=0;...`, or empty |
| 12 | error code | empty when the day converted; otherwise the refusal's code |
| 13 | error name | empty when the day converted; otherwise its name |
| 14 | standing | `in-use`, `proleptic`, `extended` or `unrecorded`; empty on a refusal |
| 15 | day boundary | where the calendar's day begins: `midnight`, `noon`, `sunset`, `sunrise` or `local-time HH:MM:SS` |
| 16 | formatted | reserved; empty until hc-format renders calendar dates generally |

**Refusals are answers.** A calendar that cannot name the day — the Rumi
calendar for a day after 1925, the Tenpō calendar for one after 1872 — is
still a line, with columns 3 to 11 and 14 empty and columns 12 and 13
carrying the code and name `CalendarError` gives every refusal:

| Code | Name | Meaning |
| --- | --- | --- |
| 1 | `year-out-of-range` | the year is outside the range the calendar represents |
| 2 | `month-out-of-range` | the month does not exist in that year |
| 3 | `day-out-of-range` | the day does not exist in that month |
| 4 | `missing-field` | a required field was not supplied |
| 5 | `unsupported-field` | a field the calendar has no meaning for |
| 6 | `before-epoch` | the day predates the calendar's epoch or adoption |
| 7 | `after-supported-range` | the day is beyond where the calendar's data or rules are defined |
| 8 | `unknown-era` | an era the calendar does not know |
| 9 | `unknown-calendar` | the calendar is not registered |
| 10 | `overflow` | day arithmetic left the representable range |
| 11 | `astronomical-model-failure` | the astronomical model could not answer |

The standing (column 14) is a different question from the refusal: Shōwa
101 converts perfectly well and is `extended`, because nobody writes it.

## Holidays

The `hc_holiday*` exports and `hc_holidays_on` need the `holiday` feature:

```sh
cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --profile release-compact --features holiday
```

It compiles every table of `hc-holiday` into the module — the countries,
the exchanges, the traditions and the international days — which is why it
is a feature and not the default. A table is named by its identifier: a
country's ISO 3166-1 alpha-2 code (`JP`), an exchange's ISO 10383 Market
Identifier Code (`XNYS`), a tradition's slug (`christian-western`) or
`un-days`, and `hc_holiday_codes` lists them all. `hc_holiday_is_day_off`
answers for one day; `hc_holidays_in_year` writes a year as tab-separated
lines — the ISO date, the name, the local name, the kind, the confidence,
`1` for a substitute day and the date it stands in for — and, called with a
null buffer, returns the length the text needs so the caller can allocate
exactly.

The string arguments fail the same way in both, and the same way as in the C
library: a null pointer with a non-zero length is `HC_ERR_NULL_POINTER`, a
code or region that is not UTF-8 is `HC_ERR_NOT_UTF8`, and a code that names
no table is `HC_ERR_UNKNOWN`. An empty region is no region.

```js
hc.holidaysInYear("JP", "", 2026);
// [{ date: "2026-01-01", name: "New Year's Day", localName: "元日", kind: "public",
//    confidence: "exact", substitute: false, observedFor: null }, ...]
hc.holidayIsDayOff("XNYS", "", hc.gregorianToFixed(2026, 4, 3));   // true: Good Friday
```

### One day, every table

`hc_holidays_on(fixed, buffer, capacity)` writes every entry on one day
across every table `hc_holiday_codes` lists, in that order, each evaluated
nationwide, one line per (table, entry):

| # | Column | Holds |
| --- | --- | --- |
| 1 | table | the table's identifier, `JP`, `XNYS`, `christian-western`, `un-days` |
| 2 | table name | its English name |
| 3 | name | the holiday's English name |
| 4 | local name | its name in the local language, or empty |
| 5 | kind | `public`, `bank`, `religious`, `observance`, `school`, `workday`, or `gap` |
| 6 | confidence | `exact` or `approximate`; empty for a gap |
| 7 | source | the instrument the rule cites, `A/RES/73/161`, or empty |
| 8 | substitute | `1` for a weekend substitute, else `0` |
| 9 | observed for | the fixed day a substitute stands in for, or empty |

A `gap` line is a holiday the table could not place in the day's year — its
calendar's range ended, or the year's announcement has not been read — with
columns 6 and 7 empty. It is reported rather than left out so a page can say
"no announcement read for this year" instead of showing nothing; policy §4.
A day with no Gregorian year is `HC_ERR_OUT_OF_RANGE`.

The call evaluates each table for the one day (`HolidayCalendar::for_day`),
which answers exactly what the whole year would and costs about a third of
it; natively, one 2026 day across all 245 tables takes about 0.45 s against
1.5 s by whole years, most of it the astronomy of the lunisolar-dated
tables.

## Almanac

`hc_term_in_effect` and `hc_pentad_in_effect` need the `seasons` feature and
answer for one day at a meridian, because the same instant falls on
different dates in Beijing and Tokyo. `meridian` is text, one of:

| Name | Meridian |
| --- | --- |
| `universal`, or empty | Greenwich, the day boundary of Universal Time |
| `japan` | 135°E, UTC+9, the meridian of the 暦要項 |
| `china` | 120°E, UTC+8, the modern Chinese calendar since 1929 |
| `korea` | UTC+9, the Dangi calendar's |
| `india` | 82°30′E, UTC+5:30, the Indian national calendar's |
| `china-before-1929` | Beijing local mean time, 116°25′E |
| a decimal number | a longitude in degrees east of Greenwich, −180 to 180, read as local mean solar time |

Names match in any case; anything else is `HC_ERR_UNKNOWN`. Each export
writes one line:

| # | `hc_term_in_effect` | `hc_pentad_in_effect` |
| --- | --- | --- |
| 1 | the term's index, 春分 at 0 through 驚蟄 at 23 (longitude ÷ 15°) | the pentad's index, the first pentad of 春分 at 0 through 71 (longitude ÷ 5°) |
| 2 | the name in traditional Chinese | the name in the Chinese tradition |
| 3 | the name in Japanese | the name in the Japanese tradition |
| 4 | the fixed day the term began at that meridian | the fixed day the pentad began |
| 5 | the last fixed day before the next term begins | the last fixed day before the next pentad begins |
| 6 | the authority for the Chinese names | the text the Chinese names come from |
| 7 | the authority for the Japanese names | the text the Japanese names come from |

## Deep time

`hc_place_years_ago`, `hc_cosmic_events` and `hc_geologic_intervals` need the
`deep-time` feature. All three write lines of the same fourteen columns, so
a page parses them once:

| # | Column | Holds |
| --- | --- | --- |
| 1 | kind | `moment`, `cosmic-epoch`, `cosmic-event`, `future-era`, a geologic rank (`eon`, `era`, `period`, `epoch`, `age`) or `archaeological` |
| 2 | name | the entry's name |
| 3 | scope | the interval one rank up for a geologic interval; the region for an archaeological period; else empty |
| 4 | start | the older bound's value |
| 5 | start σ | its standard uncertainty |
| 6 | start figures | the significant figures it claims, or empty where the table claims none |
| 7 | start approximate | `1` where the chart marks it `~`, else `0` |
| 8 | end | the younger bound's value |
| 9 | end σ | its standard uncertainty |
| 10 | end figures | as column 6 |
| 11 | end approximate | as column 7 |
| 12 | unit | what columns 4 to 11 are in, below |
| 13 | description | the table's description of the entry, or empty |
| 14 | source | where the numbers came from |

A point in time — an event, the moment itself — has the same start and end.
The unit is the one each table counts in, so the values are the tables' own
figures rather than conversions: `seconds-since-big-bang` for the cosmic
rows and the moment's `since-big-bang` row, `seconds-before-present` for
its `before-present` row, `megayears-before-present` for the geologic
chart's rows, `years-before-1950` for the archaeological rows and
`log10-years-from-now` for a future era.

**What "present" means.** `hc_place_years_ago(years_ago, std_dev_years,
buffer, capacity)` counts `years_ago` back from the present as `hc-deep-time`
defines it: the Planck 2018 age of the universe, its `AGE_OF_UNIVERSE` — not
from the BP datum of 1950 and not from the caller's clock. Its archaeological
table counts from 1950 and the geologic chart from its own present, and the
crate ignores the difference between the three, which lies below the smallest
uncertainty in any of its tables; this module does not change that. Negative
years are the future. The lines are the moment itself as `since-big-bang` and
`before-present`, its cosmic epoch and the last dated cosmic event before it,
its future era if it lies ahead, its geologic chain from eon down to age, and
its archaeological period, each present only where that chronology reaches. A
value the crate refuses — not finite, a negative uncertainty, beyond its range
— is `HC_ERR_OUT_OF_RANGE`.

`hc_cosmic_events(buffer, capacity)` lists every cosmic epoch, Big Bang to the
present, then every dated cosmic event, oldest first.
`hc_geologic_intervals(rank, buffer, capacity)` lists every interval of one
rank of the ICS chart, youngest first, with the chart as the source; `rank`
is `0` for the eons, `1` for the eras, `2` for the periods, `3` for the
epochs and `4` for the ages, and anything else is `HC_ERR_UNKNOWN`.

## Time zones

`hc_fixed_from_unix` and `hc_unix_from_fixed` count days in UTC, so at
08:00 in Tokyo the module's day is still yesterday's. The `tz` feature adds
the same two conversions by a zone's wall clock:

- `hc_fixed_from_unix_in_zone(unix_seconds, zone_ptr, zone_len)` is the
  fixed day the instant falls on in the zone.
- `hc_unix_from_fixed_in_zone(fixed, zone_ptr, zone_len)` is the instant
  the day begins in the zone: its local midnight; when the clocks go
  forward across that midnight so that it does not exist, the first
  instant after the gap (`LocalResolution::Nonexistent`'s `after_gap`,
  which is also what `Disambiguation::PushForward` gives for a skipped
  midnight); when they go back across it, the earlier of the two
  midnights.

`zone` is an IANA name in any case. **The module carries seventeen zones
and only their current rules**: `hc-tz`'s built-in table, which is the
POSIX `TZ` footer of each zone's file in the IANA database, release 2026c.
A POSIX string states one pair of rules for every year, so the table is
right about today and wrong about the past — it does not know the United
States moved its transitions in 2007 or that Brazil stopped changing its
clocks in 2019. The seventeen: `UTC`, `Africa/Cairo`, `America/Los_Angeles`,
`America/New_York`, `America/Sao_Paulo`, `Asia/Kathmandu`, `Asia/Kolkata`,
`Asia/Seoul`, `Asia/Shanghai`, `Asia/Tokyo`, `Australia/Lord_Howe`,
`Australia/Sydney`, `Europe/Berlin`, `Europe/London`, `Europe/Moscow`,
`Europe/Paris` and `Pacific/Auckland`. The module has no file system, so
that is all it can know on its own.

For any other zone, or for a zone's history, a page fetches the zone's TZif
file — the IANA database's own format, `/usr/share/zoneinfo/Europe/Rome` on
most systems — and hands its bytes to `hc_zone_load(name_ptr, name_len,
tzif_ptr, tzif_len)` once; the two conversions then answer for that name,
with every transition the file records, and a loaded zone outranks a
built-in one of the same name. The bytes are copied into the module and
parsed on each call. Bytes that are not TZif are `HC_ERR_MALFORMED` and
nothing is kept; a name nobody knows is `HC_ERR_UNKNOWN`; an instant whose
local day leaves the range of a day number is `HC_ERR_OUT_OF_RANGE`.

```js
const tzif = await (await fetch("tzdata/Europe/Rome")).arrayBuffer();
hc.loadZone("Europe/Rome", tzif);
const today = hc.fixedFromUnixInZone(Math.floor(Date.now() / 1000), "Europe/Rome");
```

The `tzdata/` artifact that CI uploads, and `scripts/wasm-tzdata.sh`
writes, holds the seventeen built-in zones' files with the database's
release in `VERSION`; see "tzdata beside the module" above.

## The sky

`hc_sky_at`, `hc_solar_terms_between` and `hc_moon_phases_between` need the
`sky` feature and answer from `hc-astro`'s series directly: where the Sun
and the Moon are at an instant, and when the Sun and the Moon reach the
angles the almanacs are built on. Every instant in and out is a POSIX
timestamp read as Universal Time, and every instant out is whole seconds,
rounded down, with no fractional column: the series are not good to better
than a few seconds, and ΔT past the USNO's predictions (October 2033) is
about nine seconds off, so a fraction would claim what is not known.

**The range rule.** `hc-astro`'s README states the era over which its
series hold as roughly 1000 BCE to 3000 CE, with ΔT the limiting factor
outside it. The three exports therefore answer only for instants whose
proleptic Gregorian year is −1000 through 3000 and return
`HC_ERR_OUT_OF_RANGE` for any other, never a number; for a span, both
`from` and the last second before `to` have to lie inside. A span longer
than 400 years is `HC_ERR_OUT_OF_RANGE` too — that is about 4 950
lunations and 9 600 terms, a few seconds of computation — and a span whose
`to` is at or before its `from` is an empty answer of zero bytes, not an
error.

`hc_sky_at(unix_seconds, buffer, capacity)` writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | sun longitude | the Sun's apparent ecliptic longitude in degrees, 0 at the March equinox |
| 2 | sun distance | the Earth–Sun distance in astronomical units |
| 3 | moon longitude | the Moon's apparent ecliptic longitude in degrees |
| 4 | moon latitude | the Moon's ecliptic latitude in degrees, positive north |
| 5 | moon distance | the Earth–Moon distance in kilometres, centre to centre |
| 6 | elongation | the Moon's elongation from the Sun in degrees: 0 at new moon, 90 at first quarter, 180 at full, 270 at last quarter |
| 7 | illuminated fraction | the lit fraction of the Moon's disc, 0 to 1 |
| 8 | previous new moon | the last new moon before the instant, as POSIX seconds |
| 9 | next new moon | the first new moon at or after the instant, as POSIX seconds |
| 10 | ΔT | `TT − UT1` at the instant, in seconds |
| 11 | ΔT regime | which source answered: `observed`, `predicted`, `fitted` or `extrapolated` |
| 12 | source | the series behind the line, as `hc-astro` names them |

The source cell names the Sun's series (VSOP87D's Earth series truncated
at an amplitude of 10⁻⁷, 213 terms, Meeus chapter 25), the Moon's (the
ELP-2000/82 abridgement of Meeus tables 47.A and 47.B, sixty terms, with
the illumination of chapter 48), the phase series behind the new moons
(Meeus chapter 49) and the ΔT source of the regime: the USNO's
`deltat.data` where observed, its `deltat.preds` where predicted, the
Espenak–Meeus polynomials where fitted and their long-term parabola where
extrapolated. `hc-astro`'s README states what each is good to.

`hc_solar_terms_between(from_unix, to_unix, buffer, capacity)` and
`hc_moon_phases_between(from_unix, to_unix, buffer, capacity)` write one
line per event in the half-open span `[from, to)`, in time order, in the
same four columns:

| # | `hc_solar_terms_between` | `hc_moon_phases_between` |
| --- | --- | --- |
| 1 | the Sun's apparent longitude that defines the term, `0` for 春分 through `345` in steps of 15 | the elongation that defines the phase: `0`, `90`, `180` or `270` |
| 2 | the instant as POSIX seconds, rounded down | the same |
| 3 | the term's name in traditional Chinese, 秋分 | `new`, `first-quarter`, `full` or `last-quarter` |
| 4 | the term's name in Japanese, 啓蟄 where the Chinese has 驚蟄 | empty |

A term is an instant, not a date: which day it falls on depends on the
meridian, which is `hc_term_in_effect`'s business. The phases are the
phase series of Meeus chapter 49 — the same series that dates columns 8
and 9 of `hc_sky_at`, so a new moon appears at the same second in both.

```js
const from = Date.UTC(2026, 8, 1) / 1000, to = Date.UTC(2026, 9, 1) / 1000;
hc.solarTermsBetween(from, to);
// [{ angle: 165, instant: 1788..., name: "白露", japaneseName: "白露" },
//  { angle: 180, instant: 1790..., name: "秋分", japaneseName: "秋分" }]
hc.moonPhasesBetween(from, to).map((phase) => phase.name);
// ["last-quarter", "new", "first-quarter", "full"]
hc.skyAt(Date.now() / 1000 | 0).illuminatedFraction;
```

## What is not here

Formatting a date of any calendar in a locale's own way — 令和8年9月25日,
丙午年八月初四 — waits on `hc-format` rendering calendars other than the
Gregorian; column 16 of `hc_describe_day` is reserved for it. The C ABI in
[`hyper-calendar-ffi`](../hyper-calendar-ffi) covers the same ground with
NUL-terminated strings, out-parameters and status codes instead of
sentinels; a name that appears in both means the same thing in both, and
the lines are the same lines.
