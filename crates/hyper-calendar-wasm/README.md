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
and the rule is:

> **A value-returning export never returns a number at or below
> `HC_ERR_FLOOR` except as an error.**

`HC_ERR_FLOOR` is −9 × 10¹⁵, more than a thousand times the age of the
universe in days, so no day number comes near it. A count of seconds does:
−9 × 10¹⁵ seconds is only about 285 million years. So an export that answers
in seconds refuses a day whose answer would reach the floor with
`HC_ERR_OUT_OF_RANGE`, rather than return a number every binding would read
as a sentinel, and it refuses the same way a result that would overflow an
`i64`, rather than wrap or clamp it. The floor and the ends of an `i64` are
the only bounds on the exports that answer in seconds; the others' are the
calendar's own.

### Ranges

What each export answers for; outside it, the export returns the sentinel
named. The JavaScript binding throws that sentinel as an `HcError`, so a day
out of range is `out-of-range`, never an unrecognised number.

| Answers with | Exports | Answers for |
| --- | --- | --- |
| a fixed day | `hc_gregorian_to_fixed`, `hc_parse_iso_date` | the years −9 999 999 through 9 999 999, which are the fixed days −3 652 424 999 through 3 652 424 634; any other date is `HC_ERR_INVALID_DATE` |
| a Gregorian year, month, day, day of the year, or 1 or 0 | `hc_gregorian_year`, `hc_gregorian_month`, `hc_gregorian_day`, `hc_day_of_year`, `hc_is_leap_year` | the fixed days −3 652 424 999 through 3 652 424 634; any other is `HC_ERR_OUT_OF_RANGE` |
| a weekday, 1 through 7 | `hc_weekday` | every `i64` |
| a fixed day | `hc_fixed_from_unix` | every `i64` timestamp; the day is between −106 751 990 448 138 and 106 751 991 886 463 |
| a fixed day | `hc_fixed_from_unix_in_zone` | every `i64` timestamp; the day is at most one from the day `hc_fixed_from_unix` gives |
| seconds | `hc_unix_from_fixed` | the fixed days −104 165 947 503 through 106 751 991 886 463: an earlier day's midnight would be at or below `HC_ERR_FLOOR` seconds, and a later one's would overflow an `i64`; either is `HC_ERR_OUT_OF_RANGE` |
| seconds | `hc_unix_from_fixed_in_zone` | the days whose start by the zone's clock is above `HC_ERR_FLOOR` and fits an `i64`: by UTC, `hc_unix_from_fixed`'s days, and a zone's offset moves each end by at most a day (Tokyo's last day is 106 751 991 886 464); any other is `HC_ERR_OUT_OF_RANGE` |
| seconds | `hc_tai_minus_utc` | every `i64` timestamp; under `strict`, 1961 through the end of the announced leap-second table, and `HC_ERR_NO_DATA` outside it |
| 1 or 0 | `hc_day_has_leap_second` | the timestamps −9 223 372 036 854 720 000 through 9 223 372 036 854 719 999, the whole days of the `i64` range; the part-days at its two ends begin or end where no `i64` reaches, and are `HC_ERR_OUT_OF_RANGE` |
| 1 or 0 | `hc_holiday_is_day_off` | the fixed days −3 652 424 999 through 3 652 424 634; any other is `HC_ERR_OUT_OF_RANGE` |
| a weekday, 1 through 7 | `hc_first_day_of_week` | every locale tag |
| 0 | `hc_zone_load` | any name and bytes; bytes that are not TZif are `HC_ERR_MALFORMED` |
| a byte length | `hc_version`, `hc_format_iso_date`, `hc_describe_day`, `hc_calendar_units`, `hc_calendars`, `hc_locales`, `hc_gregorian_adoption`, `hc_holidays_in_year`, `hc_holiday_codes`, `hc_holidays_on`, `hc_term_in_effect`, `hc_pentad_in_effect`, `hc_place_years_ago`, `hc_cosmic_events`, `hc_geologic_intervals`, `hc_sky_at`, `hc_solar_terms_between`, `hc_moon_phases_between`, `hc_orbit_at`, `hc_orbit_series` | whatever inputs the export's own documentation accepts; a length is never negative, so it never nears the floor |

[`crates/hyper-calendar/tests/abi.rs`](../hyper-calendar/tests/abi.rs)
walks every `i64` export in the table of exports below and fails when one
has no row here, or two.

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
// { id: "japanese", era: "reiwa", eraLabel: "令和", year: 8, month: 9, day: 21,
//   formatted: "令和8年9月21日", localeUsed: "ja", ... }
hc.calendarUnits("chinese", "month", rd, rd + 90, "zh-Hans");
// [{ start: 739870, end: 739899, label: "八月", leap: false, ... }, ...]
hc.calendars(rd, "native").find((row) => row.id === "hebrew")?.name;   // "לוח השנה העברי"
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
| `calendarUnits(id, unit, from, to, locale)` | `hc_calendar_units` | `CalendarUnit[]`, one per span |
| `calendars(today, locale)` | `hc_calendars` | `CalendarEntry[]`, one per calendar |
| `locales()` | `hc_locales` | `LocaleEntry[]`, one per locale |
| `firstDayOfWeek(locale)` | `hc_first_day_of_week` | a number, Monday = 1 through Sunday = 7 |
| `gregorianAdoption(region)` | `hc_gregorian_adoption` | `GregorianAdoption[]`, one per step |
| `holidayIsDayOff(code, region, rd)` | `hc_holiday_is_day_off` | a boolean |
| `holidaysInYear(code, region, year)` | `hc_holidays_in_year` | `HolidayInYear[]` |
| `holidayCodes()` | `hc_holiday_codes` | `string[]` |
| `holidaysOn(rd)` | `hc_holidays_on` | `HolidayOn[]` |
| `termInEffect(rd, meridian)`, `pentadInEffect(rd, meridian)` | `hc_term_in_effect`, `hc_pentad_in_effect` | a `TermInEffect` |
| `placeYearsAgo(years, stdDev, locale)`, `cosmicEvents(locale)`, `geologicIntervals(rank, locale)` | `hc_place_years_ago`, `hc_cosmic_events`, `hc_geologic_intervals` | `DeepTimeRow[]` |
| `fixedFromUnixInZone(unix, zone)`, `unixFromFixedInZone(rd, zone)` | `hc_fixed_from_unix_in_zone`, `hc_unix_from_fixed_in_zone` | a number |
| `loadZone(name, tzif)` | `hc_zone_load` | nothing |
| `skyAt(unix)` | `hc_sky_at` | a `Sky` |
| `solarTermsBetween(from, to)`, `moonPhasesBetween(from, to)` | `hc_solar_terms_between`, `hc_moon_phases_between` | `SkyEvent[]` |
| `orbitAt(years)`, `orbitSeries(from, to, step)` | `hc_orbit_at`, `hc_orbit_series` | an `Orbit`; `OrbitSample[]` |

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
and it is 2.33 MiB (2,439,430 bytes) for the `full` layer of 2026-09-26,
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
| `civil` *(default)* | Gregorian dates, ISO 8601 text, POSIX time, the TAI–UTC bridge | `hc-calendar`, `hc-calendars-solar`, `hc-format` | 35,833 | 35 KiB |
| `calendars` | `hc_describe_day`, `hc_calendar_units`, `hc_calendars`, `hc_locales`, `hc_first_day_of_week`, `hc_gregorian_adoption`: every registered calendar described for one day, walked as eras, years, months and days, and listed, in a locale; the locales and the day each one's week begins on; and when each country adopted the Gregorian calendar | every `hc-calendars-*` crate, `hc-astro`, `hc-i18n`, `hc-format` | 712,539 | 696 KiB |
| `holiday` | the four `hc_holiday*` exports and `hc_holidays_on` | `hc-holiday` and everything it dates by | 997,958 | 975 KiB |
| `seasons` | `hc_term_in_effect`, `hc_pentad_in_effect` | `hc-seasons`, `hc-astro` | 88,978 | 87 KiB |
| `deep-time` | `hc_place_years_ago`, `hc_cosmic_events`, `hc_geologic_intervals` | `hc-deep-time`, `hc-uncertainty` | 152,694 | 149 KiB |
| `tz` | `hc_fixed_from_unix_in_zone`, `hc_unix_from_fixed_in_zone`, `hc_zone_load` | `hc-tz` | 57,836 | 56 KiB |
| `sky` | `hc_sky_at`, `hc_solar_terms_between`, `hc_moon_phases_between` | `hc-astro`, `hc-seasons` | 96,781 | 95 KiB |
| `orbital` | `hc_orbit_at`, `hc_orbit_series` | `hc-orbital`, `hc-uncertainty` | 63,961 | 62 KiB |
| `full` | all of the above | everything | 1,783,754 | 1.70 MiB |

The sizes are of the `release-compact` profile for
`wasm32-unknown-unknown`, as [`scripts/wasm-layers.sh`](../../scripts/wasm-layers.sh)
printed them on 2026-09-26 with rustc 1.98.1:

```sh
scripts/wasm-layers.sh
# cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --profile release-compact --no-default-features --features <layer>
```

The script leaves each layer at `target/wasm-layers/hyper_calendar_wasm.<feature>.wasm`
and prints the table; CI runs it on every pull request and uploads the
nine files, the embedded module and `tzdata/` as one workflow artifact.
CI then runs [`scripts/wasm-size-check.sh`](../../scripts/wasm-size-check.sh),
which fails when any layer is more than 5% larger or smaller than the table
above: a layer that grows by accident is caught, and a change that moves a
layer on purpose comes with a refreshed table.
`hc_alloc`, `hc_free` and `hc_version` are in every build.

The profile's `opt-level = "z"` is a measured choice, not a default. On
2026-09-25, with the same rustc, the `full` layer built at `z` was
1,485,352 bytes and answered `hc_holidays_on` for 2026-01-01 in 87 ms under
Node 22; at `s`, 1,497,957 bytes and 83 ms; at `3`, 1,614,242 bytes and
74 ms. The 17 % of time `z` costs against `3` buys 8 % of the size, and a
page loads the module far more often than it asks the costliest question,
so `z` stays.

## What is exported

The two tables below are rendered from `src/lib.rs` by
[`crates/hyper-calendar/tests/abi.rs`](../hyper-calendar/tests/abi.rs), which
fails when they drift, and [`tests/readme.rs`](tests/readme.rs) holds every
row's feature to the source's `#[cfg]`. An export without a row here does
not pass CI.

<!-- generated by crates/hyper-calendar/tests/abi.rs: start -->

### Exports

39 functions. Types are the WebAssembly ones: `i64` crosses into JavaScript as a `BigInt`, everything else as a `number`, and a pointer is a byte offset into `memory`. The feature column is the Cargo feature the module has to be built with for the export to exist.

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
| `hc_calendar_units(id: *const u8, id_len: usize, unit: u32, from_fixed: i64, to_fixed: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The days from `from_fixed` up to but not including `to_fixed` as one calendar's eras, years, months or days, as UTF-8 lines, returning the byte length written. |
| `hc_calendars(today: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Every registered calendar, as UTF-8 lines, returning the byte length written. |
| `hc_locales(buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Every locale the module carries, as UTF-8 lines, returning the byte length written. |
| `hc_first_day_of_week(locale: *const u8, locale_len: usize) -> i64` | `calendars` | The ISO weekday of the first day of the week in a locale, Monday = 1 through Sunday = 7, or an error sentinel. |
| `hc_gregorian_adoption(region: *const u8, region_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The steps by which a country adopted the Gregorian calendar, as UTF-8 lines, returning the byte length written. |
| `hc_holiday_is_day_off(code: *const u8, code_len: usize, region: *const u8, region_len: usize, fixed: i64) -> i64` | `holiday` | Whether a fixed day is a day off in a holiday table: 1, 0, or an error sentinel. |
| `hc_holidays_in_year(code: *const u8, code_len: usize, region: *const u8, region_len: usize, year: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The holidays of a Gregorian year in a table, as UTF-8 lines, returning the byte length written. |
| `hc_holiday_codes(buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The identifier of every holiday table, one per line, returning the byte length written. |
| `hc_holidays_on(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | Every holiday on one fixed day across every table, as UTF-8 lines, returning the byte length written. |
| `hc_term_in_effect(fixed: i64, meridian: *const u8, meridian_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `seasons` | The solar term in effect on a fixed day at a meridian, as one UTF-8 line, returning the byte length written. |
| `hc_pentad_in_effect(fixed: i64, meridian: *const u8, meridian_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `seasons` | The pentad (候) in effect on a fixed day at a meridian, as one UTF-8 line, returning the byte length written. |
| `hc_place_years_ago(years_ago: f64, std_dev_years: f64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | A moment some years before the present, placed in every chronology at once, as UTF-8 lines, returning the byte length written. |
| `hc_cosmic_events(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | Every cosmic epoch and every dated cosmic event, as UTF-8 lines, returning the byte length written. |
| `hc_geologic_intervals(rank: u32, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | Every interval of one rank of the geologic time scale, as UTF-8 lines, returning the byte length written. |
| `hc_fixed_from_unix_in_zone(unix_seconds: i64, zone: *const u8, zone_len: usize) -> i64` | `tz` | The fixed day a POSIX timestamp falls on by the wall clock of a zone, or an error sentinel. |
| `hc_unix_from_fixed_in_zone(fixed: i64, zone: *const u8, zone_len: usize) -> i64` | `tz` | The POSIX timestamp at which a fixed day begins by the wall clock of a zone, or an error sentinel. |
| `hc_zone_load(name: *const u8, name_len: usize, tzif: *const u8, tzif_len: usize) -> i64` | `tz` | Give the module a zone's TZif data under an IANA name, returning 0. |
| `hc_sky_at(unix_seconds: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Sun and the Moon at a POSIX timestamp, as one UTF-8 line, returning the byte length written. |
| `hc_solar_terms_between(from_unix: i64, to_unix: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Every solar term whose instant falls in `[from_unix, to_unix)`, as UTF-8 lines, returning the byte length written. |
| `hc_moon_phases_between(from_unix: i64, to_unix: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Every new moon, first quarter, full moon and last quarter whose instant falls in `[from_unix, to_unix)`, as UTF-8 lines, returning the byte length written. |
| `hc_orbit_at(years_before_1950: f64, buffer: *mut u8, capacity: usize) -> i64` | `orbital` | Earth's orbital elements and the June insolation at 65° N at an epoch, as one UTF-8 line, returning the byte length written. |
| `hc_orbit_series(from_years_before_1950: f64, to_years_before_1950: f64, step_years: f64, buffer: *mut u8, capacity: usize) -> i64` | `orbital` | The line of `hc_orbit_at` at every epoch from `from_years_before_1950` to `to_years_before_1950` in steps of `step_years`, each with the epoch as a first column, as UTF-8 lines, returning the byte length written. |

### Error sentinels

A function that returns `i64` returns one of these instead of trapping. All are at or below `HC_ERR_FLOOR`, and no legitimate result is: the ranges above say what each export answers for.

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
Indic and regional calendars.

`locale` is a BCP 47 tag such as `ja-JP` or `zh-Hans`, or the word
`native`. Every rendered cell follows one rule: the locale asked for when
its data names the calendar; else English, so that a Japanese page shows
the Hebrew and Hijri months in English rather than in a script the page did
not ask for; else the locale asked for, with the calendar's own names from
its shape. A named locale never borrows the calendar's own language. Only
`native` asks for each calendar's own language first (`he` for the Hebrew
calendar, `ar` for the Hijri, `zh-Hans` for the Chinese), then English, and
the last column of every line names the locale data that answered. A tag that does not parse
falls back to the root locale `und`, as `hc-i18n` does, whose month names
are CLDR's `M01`..`M12` — ask for `en` for English. A null pointer with a
zero length is `und` too.

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the calendar's identifier, `gregory`, `chinese`, `japanese`, ... |
| 2 | name | its English name |
| 3 | era | the era code, `reiwa`, `AD`, `AH`, `roc`, ..., or empty for a calendar without eras |
| 4 | era label | the era's name in the locale, 令和, or the calendar's own name for it, 嘉永 or `Kaei`, or empty |
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
| 16 | formatted | the date as the locale writes it — 令和8年9月21日, 癸卯年闰二月初一, `September 21, 2026` — from `hc_format::label`; empty on a refusal |
| 17 | locale used | the tag of the locale data that answered: `ja`, `he`, `und` |
| 18 | day named by | which civil day names a day that does not begin at midnight: `start` for the one it begins on (the Julian Day, the Tibetan and Hindu days), `end` for the one it ends on (the Hebrew and Islamic days, whose evening is already the next date); empty for a midnight start |

**Refusals are answers.** A calendar that cannot name the day — the Rumi
calendar for a day after 1925, the Tenpō calendar for one after 1872 — is
still a line, with columns 3 to 11, 14 and 16 empty and columns 12 and 13
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

The standing (column 14) is a different question from the refusal: the
Juche calendar converts 2026 perfectly well and is `extended`, because the
official calendars dropped the era after 2024, and the Chinese calendar is
`in-use`, because the Spring Festival is still dated by it although it
stopped being China's civil calendar in 1912. A calendar answers for the
period its sources record, with the source named in `Usage::source`;
`unrecorded` is what the day counts, the proposals and the calendars whose
sources give no span say, and `crates/hyper-calendar/tests/usage.rs` lists
which those are.

**The formatted date** (column 16) is `hc_format::label::date`, and the
labels of the units below are `hc_format::label::label`: the same
renderer, over the same per-locale templates in `hc-i18n`. Each locale
states how it writes a year with its era (`{era}{year}年`, `{year} {era}`),
a day and a whole date, and a calendar family states what differs for it —
the Chinese calendar's year by its stem and branch, 癸卯年, and its days
by their Han names, 初一 … 三十; the Japanese first year of an era as 元年.
Every template names the CLDR pattern it was read from. A locale that has
stated none gets the fields in order, separated by spaces, in the names
the library already has; nothing is invented. An era's name is the
locale's, else the calendar's own (every nengō, 嘉永, romanised as *Kaei*
for a Latin-script locale), else its code; a month's is the locale's, else
the calendar's own shape name, else its number; the Gregorian family's
`AD` and the Hebrew calendar's `AM` are left unwritten, as those calendars
are printed.

## Units of a calendar

`hc_calendar_units(id_ptr, id_len, unit, from_fixed, to_fixed, locale_ptr, locale_len, buffer, capacity)`
needs the `calendars` feature and writes the days from `from_fixed` up to
but not including `to_fixed` as one calendar's eras (`unit` 0), years (1),
months (2) or days (3): one line per span, in order, each span touching
the next, from the start of the unit that contains `from_fixed` to at
least `to_fixed` — the first and last spans are whole units and may reach
outside the range asked for. This is what a timeline draws as a lane. The
walk goes by the calendar's own lengths, not day by day, and thirty years
of Chinese months come back in well under a second. `locale` is as for
`hc_describe_day`, `native` included; an identifier or unit the module
does not know is `HC_ERR_UNKNOWN`, and an empty range writes nothing.

| # | Column | Holds |
| --- | --- | --- |
| 1 | start | the first fixed day of the span |
| 2 | end | the day after the span's last, so that `end - start` is its length and one span's `end` is the next's `start` |
| 3 | label | the span's label in the locale: 令和元年, 令和6年, `5784`, `1445 AH`, 癸卯年, `Adar I`, 閏二月, 初四; empty on a refusal |
| 4 | leap | `1` for an intercalary unit — a leap year, a leap month, a repeated day — else `0`; empty on a refusal |
| 5 | standing | the standing of the span's first day; empty on a refusal |
| 6 | error code | empty for a unit; otherwise the code of the calendar's refusal of every day in the span |
| 7 | error name | empty for a unit; otherwise its name |
| 8 | locale used | the tag of the locale data that answered |

A refusal is a span like any other: the days before a calendar's epoch
(`before-epoch`), the days past its table (`after-supported-range`), the
years of the Japanese schism the unified stream declines, or the whole
range for a unit the calendar does not have — the months of a calendar
without months, the years of a day count (`unsupported-field`) — so that
a lane can show *this calendar does not reach here* rather than a gap.

## The calendars

`hc_calendars(today, locale_ptr, locale_len, buffer, capacity)` needs the
`calendars` feature and writes one line per registered calendar, in
registry order. `locale` is as for `hc_describe_day`; `today` is the fixed
day the standing is judged on, because the module has no clock. The names
are CLDR 48's, `localeDisplayNames/types/type[@key="calendar"]`, at its
`approved` and `contributed` levels; a calendar CLDR does not name in the
locale has an empty name. No two rows share a name, in column 2 or in
column 3, so a reader can choose a calendar by what the page shows:
calendars that differ only in a convention are named for it, the Maya
counts for their correlation — `Maya long count (GMT, 584283)`,
`(GMT+2, 584285)`, `(Martin and Skidmore, 584286)` — and
`persian-arithmetic` for its cycle, CLDR's `persian` name followed by
`(2820)`; [docs/i18n.md](../../docs/i18n.md#what-a-locale-calls-a-calendar)
says how.

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the calendar's identifier |
| 2 | name | what the locale calls the calendar — 和暦, `Hebrew Calendar` — or empty where it has no name for it, so that a page falls back to column 3 itself; the name is never borrowed from the calendar's own language, which only `native` asks for |
| 3 | english name | its English name, which names the convention where the registry carries a calendar under several — `Maya haab (GMT+2, 584285)` |
| 4 | earliest | the earliest fixed day it converts, or empty where unbounded |
| 5 | latest | the latest fixed day it converts, or empty where unbounded |
| 6 | has era | `1` when its dates carry an era |
| 7 | has year | `1` when its dates carry a year |
| 8 | has month | `1` when it has months |
| 9 | has day | `1` when its dates carry a day of the month |
| 10 | native locales | the languages its sources are written in, as BCP 47 tags joined by `;`, primary first — `he`, `zh-Hans;zh-Hant`, `sa;hi;ta` — or empty for a day count, a proposal or the Gregorian family |
| 11 | standing | its standing on `today` |

## The locales

`hc_locales(buffer, capacity)` needs the `calendars` feature and writes one
line per locale the module carries, in tag order.

| # | Column | Holds |
| --- | --- | --- |
| 1 | tag | the BCP 47 tag: `ja`, `zh-Hans` |
| 2 | english name | the language's name in English |
| 3 | native name | the language's name in itself: 日本語 |
| 4 | gregorian months | `1` when the locale's own data names the Gregorian months |
| 5 | weekdays | `1` when it names the weekdays |
| 6 | gregorian eras | `1` when it names the Gregorian eras |
| 7 | calendars | the identifiers of the calendars it has vocabulary of its own for beyond the shared Gregorian months, joined by `;` |

## The first day of the week

`hc_first_day_of_week(locale_ptr, locale_len)` needs the `calendars`
feature and answers the ISO weekday a locale's week begins on, Monday = 1
through Sunday = 7, as `hc-i18n` reads CLDR 48's `weekData/firstDay`: a
`-u-fw-` key first, then the tag's region, then, for a tag without one, the
region the language's likely subtags give. `en-US`, `en`, `ja` and `ar-SA`
begin on Sunday, `en-GB`, `fr` and `zh-Hans` on Monday, and `ar-EG` on
Saturday. A tag that does not parse is the root locale `und`, whose week
begins on the world's Monday. It answers an `i64`, as every export that can
return a sentinel does, and fails only as a text argument does:
`HC_ERR_NULL_POINTER` or `HC_ERR_NOT_UTF8`.

## Gregorian adoption

`hc_gregorian_adoption(region_ptr, region_len, buffer, capacity)` needs the
`calendars` feature and writes one line per step by which a country took the
Gregorian calendar, oldest first. `region` is an ISO 3166-1 alpha-2 code, in
either case. A staged adoption is several lines: China's declaration of 1912
and its nationwide order of 1929, Sweden's omitted leap day of 1700, its
return to the Julian calendar in 1712 and its change of 1753, the Dutch
provinces one by one, Turkey's Gregorian days of 1917 and Gregorian year of
1926. A code the table does not know writes nothing, which says only that
the table does not know it. The table, its sources and what it leaves out are
in [`docs/systems/gregorian-reform.md`](../../docs/systems/gregorian-reform.md).

| # | Column | Holds |
| --- | --- | --- |
| 1 | last old day | the last day of the old reckoning, as a fixed day |
| 2 | first day | the first day of the new reckoning, as a fixed day: always the next day, since no step broke the week |
| 3 | old calendar | the registry identifier of the calendar kept until then: `julian`, `japanese-tenpo`, `dangi`, `chinese`, `islamic-umalqura`, `rumi` or `swedish-1700` |
| 4 | scope | `civil` for the civil calendar of the whole polity as it then was; `partial` for part of the country, some purposes only, or part of the calendar; `ecclesiastical` for a church's calendar alone, which no row carries yet |
| 5 | source | the instrument behind the step — decree, act, law — with its date, and whether it was read |
| 6 | new calendar | the registry identifier of the calendar kept from then: `gregory`, but `swedish-1700` for Sweden's step of 1700 and `julian` for its step of 1712 |
| 7 | polity | who took the step, in English: the polity then governing, and the part of the country where the scope is partial |

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

The call evaluates each table for the one day
(`HolidayCalendar::for_day_with`), which answers exactly what the whole
year would, through one `EvaluationContext` shared by every table, so the
astronomy the tables have in common — the sunrises the Hindu festivals are
read at, the new moons and solar terms of the Chinese-dated ones — is done
once, and only for the months around the day. Natively, in the
`release-compact` profile, one 2026 day across all 245 tables takes about
40 ms (41 ms on 1 January, the costliest, 36 ms on 25 September) against
0.23 s by whole years; in WebAssembly under Node 22 about 75–90 ms, and
under JavaScriptCore's shell about 70 ms. Before the tables shared a
context it was 0.79 s natively and 1.7 s under Node, nearly all of it
sunrises and solar-longitude searches repeated table by table.

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
`deep-time` feature. All three take a locale, a BCP 47 tag, after their
other arguments, and write lines of the same fifteen columns, so a page
parses them once:

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
| 15 | localised name | the geological chart's own name for an interval in the locale's language — 第四系／紀, 显生宇, `Quartär` — or empty: for a language the chart has no names in, for the handful of intervals it names in no language, and for every cosmic, future and archaeological row, which have no published translation this module carries |

The localised names are the International Commission on Stratigraphy's own
translations, from the chart's vocabulary (`chart.ttl`, CC BY 4.0) in
fourteen of the module's locales — `cs`, `de`, `es`, `fr`, `id`, `it`, `ja`,
`ko`, `nl`, `pl`, `pt`, `ru`, `tr` and `zh-Hans` — with the Japanese checked
against the Geological Society of Japan's chart and the Chinese against the
ICS's Chinese chart; `hc_deep_time::names` says what was left out and why.
The English name stays in column 2.

A point in time — an event, the moment itself — has the same start and end.
The unit is the one each table counts in, so the values are the tables' own
figures rather than conversions: `seconds-since-big-bang` for the cosmic
rows and the moment's `since-big-bang` row, `seconds-before-present` for
its `before-present` row, `megayears-before-present` for the geologic
chart's rows, `years-before-1950` for the archaeological rows and
`log10-years-from-now` for a future era.

**What "present" means.** `hc_place_years_ago(years_ago, std_dev_years,
locale_ptr, locale_len, buffer, capacity)` counts `years_ago` back from the present as `hc-deep-time`
defines it: the Planck 2018 age of the universe, its `AGE_OF_UNIVERSE` — not
from the BP datum of 1950 and not from the caller's clock. Its archaeological
table counts from 1950 and the geologic chart from its own present, and the
crate ignores the difference between the three, which lies below the smallest
uncertainty in any of its tables; this module does not change that. Negative
years are the future, and the near future is still the present: the chart's
youngest intervals, the Modern period and the last cosmic epoch end at the
present with no future boundary, so a moment up to a century ahead — six
hours, three years — is placed in all of them as well as in its future era.
The century is the chart's resolution at its young end, where it prints the
Meghalayan's base as 0.0042 Ma; beyond it the future begins, and the moment
has its future era and the last cosmic event alone. The lines are the moment itself as `since-big-bang` and
`before-present`, its cosmic epoch and the last dated cosmic event before it,
its future era if it lies ahead, its geologic chain from eon down to age, and
its archaeological period, each present only where that chronology reaches. A
value the crate refuses — not finite, a negative uncertainty, beyond its range
— is `HC_ERR_OUT_OF_RANGE`.

`hc_cosmic_events(locale_ptr, locale_len, buffer, capacity)` lists every
cosmic epoch, Big Bang to the present, then every dated cosmic event, oldest
first. `hc_geologic_intervals(rank, locale_ptr, locale_len, buffer, capacity)`
lists every interval of one
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

## The orbit

`hc_orbit_at` and `hc_orbit_series` need the `orbital` feature and answer
from `hc-orbital`: Berger's 1978 trigonometric solution for the slow
cycles of Earth's orbit — the eccentricity, the obliquity and the
longitude of perihelion, the cycles that paced the ice ages — and the
daily insolation that follows from them, over a million years either
side of 1950. Every epoch is in years before 1950, negative for the
future, which is the series' own count and the radiocarbon "before
present" datum. An epoch outside that span is `HC_ERR_OUT_OF_RANGE`,
never a number: a trigonometric fit past its span does not fade, it
lies. `hc-orbital`'s README and `docs/systems/orbital-elements.md` say
what the series is good to.

`hc_orbit_at(years_before_1950, buffer, capacity)` writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | eccentricity | the eccentricity *e* of Earth's orbit |
| 2 | eccentricity spread | its spread |
| 3 | obliquity | the obliquity of the ecliptic ε, in degrees |
| 4 | obliquity spread | its spread, in degrees |
| 5 | longitude of perihelion | ϖ, the longitude of perihelion from the moving equinox, in degrees, 0 to 360: the heliocentric direction of perihelion, about 102° at present; add 180° for the Sun's longitude at perihelion |
| 6 | longitude of perihelion spread | its spread, in degrees; 180 where *e* is smaller than the precession's spread, because the angle of a vector shorter than its own error bar is unknown |
| 7 | climatic precession | *e* sin ϖ, positive when perihelion falls in northern summer |
| 8 | climatic precession spread | its spread |
| 9 | insolation 65°N June | the daily mean insolation at 65° N at the June solstice (solar longitude 90°), in W/m², for the solar constant of column 10 |
| 10 | solar constant | the solar constant the insolation was computed with, in W/m²: 1360, `hc-orbital`'s `SOLAR_CONSTANT_BERGER_LOUTRE_1991`, the value of the tables the spreads were measured against |
| 11 | source | the series and the constant, by name |

A spread is not a standard uncertainty. It is the largest disagreement
measured between this series and Berger & Loutre's 1991 solution, the
one the same author published to replace it, over the tier of the span
the epoch falls in — within 100 000 years of 1950, within 800 000, or
the whole million — so it widens in steps rather than smoothly, and
`hc-orbital`'s `SPREAD_TIERS` carries the figures. The insolation is
proportional to the solar constant, so a page that wants the modern
1361 W/m² scales column 9 by 1361/1360.

`hc_orbit_series(from_years_before_1950, to_years_before_1950,
step_years, buffer, capacity)` writes the same line at `from`,
`from + step`, `from + 2 step` and so on, every sample at or before `to`,
one line per sample with the epoch in years before 1950 as a first
column before the eleven above, so a page drawing a curve asks once
rather than once per pixel. Both ends have to lie within the span and
`step` has to be finite and positive, and at most 10 000 samples are
answered, else `HC_ERR_OUT_OF_RANGE`; a caller who wants more asks in
pieces. A `to` before `from` is an empty answer of zero bytes, not an
error. Each sample is computed from `from` rather than accumulated, so
the error does not grow along the series, and the last is held to `to`
against rounding.

A call is cheap: 163 trigonometric terms and eleven numbers written.
Measured over 10 000 calls at epochs spread across the span, on 2026-09-26,
`hc_orbit_at` costs about 2.5 µs natively and 5–6 µs in WebAssembly under
Node 22, both in the `release-compact` profile the layers are built with
(1.4 µs and 3–4.5 µs at `opt-level = 3`). Through the binding the same
call costs about 40 µs in `release-compact`, because every text-writing
method first offers the module a 64 KiB buffer and `hc_alloc` zeroes it;
a page that draws the orbit alone loads with a smaller
`initialCapacity`, which every line of `hc_orbit_at` fits at 1 024
bytes, or asks for the series, whose 10 000 samples cost about 40 ms to
write and 180 ms to decode. The series saves the calls and the
allocations, not the arithmetic.

```js
const lgm = hc.orbitAt(21_000);          // the Last Glacial Maximum
lgm.eccentricity;                         // 0.018994
lgm.obliquity;                            // 22.949
lgm.climaticPrecession;                   // 0.01729
lgm.insolation65NJune;                    // 468.8, against 477.6 at 1950
hc.orbitSeries(0, 100_000, 1_000).map((sample) => [sample.yearsBefore1950, sample.insolation65NJune]);
```

## What is not here

Formatting is by template, and a template is only as wide as its locale's
data: a locale that has stated none writes a date as its fields in order,
and a month no locale names comes back as a number. The C ABI in
[`hyper-calendar-ffi`](../hyper-calendar-ffi) covers the same ground with
NUL-terminated strings, out-parameters and status codes instead of
sentinels; a name that appears in both means the same thing in both, and
the lines are the same lines, made once in `hyper_calendar::lines`.
