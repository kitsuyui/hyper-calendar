# hyper-calendar-ffi

A C ABI for `hyper-calendar`: convert dates between every calendar the
registry carries, look up the holiday tables of countries, exchanges and
traditions, and handle TAI, UTC and leap seconds, from any language that
can call C. Days cross as fixed days (Rata Die: day 1 is 1 January of
year 1, proleptic Gregorian). It is built as the shared library
`libhyper_calendar_ffi.{so,dylib}` (`hyper_calendar_ffi.dll` on Windows) and
as a static library.

## The shape of the interface

A C boundary has three ways to go wrong, and each is closed deliberately.

- **Panics cannot cross it.** Unwinding across an `extern "C"` frame is
  undefined behaviour. The workspace denies `unwrap` and `expect` outside
  tests, and every entry point returns a status code rather than a value, so
  a failure is something the caller reads, not a trap.
- **Nothing is allocated that the caller cannot free.** There are no returned
  pointers and no opaque handles. Every function writes into storage the
  caller owns: integers through out-parameters, text into a caller-supplied
  buffer. There is no global state to tear down.
- **Truncation is reported, not silent.** A text function that does not fit
  returns `HC_ERROR_BUFFER_TOO_SMALL` and writes the required length,
  including the terminating NUL, so the
  caller can retry with a bigger buffer. It never writes a partial answer and
  calls it success.

## Building and linking

```sh
cargo build -p hyper-calendar-ffi --release
```

The artefacts land in `target/release/`: `libhyper_calendar_ffi.so` on Linux,
`libhyper_calendar_ffi.dylib` on macOS, `hyper_calendar_ffi.dll` on Windows,
and the static `libhyper_calendar_ffi.a`; link with `-lhyper_calendar_ffi`.
There is no generated header. The prototypes below are the interface, and
`HcStatus` is `int`.

```c
#include <stdint.h>
#include <stddef.h>

typedef int HcStatus;
HcStatus hc_gregorian_to_fixed(int64_t year, uint8_t month, uint8_t day,
                               int64_t *out_fixed);

int main(void) {
    int64_t rd;
    if (hc_gregorian_to_fixed(2026, 9, 21, &rd) == 0) {
        /* rd == 739880: the Rata Die, day 1 being 0001-01-01. */
    }
    return 0;
}
```

## Errors and ranges

Every entry point reports failure through its `HcStatus` alone, so an
`int64_t` it writes has the whole of its range. There are no sentinels here:
where the WebAssembly module has to refuse a day whose POSIX time would be at
or below its `HC_ERR_FLOOR`, about 285 million years back, this library
answers, as the section below explains. What an entry point cannot hold it refuses rather than wraps or
clamps, and the only bounds that are not the calendar's own are where an
`int64_t` runs out. The range each entry point that takes an `int64_t` or
writes one answers for, naming each `int64_t` input;
[`crates/hyper-calendar/tests/abi.rs`](../hyper-calendar/tests/abi.rs)
fails when one has no row, or two, or a row that does not name its inputs:

| Writes | Entry points | Answers for |
| --- | --- | --- |
| a fixed day | `hc_gregorian_to_fixed` | `year` −9 999 999 through 9 999 999, which are the fixed days −3 652 424 999 through 3 652 424 634; any other date is `HC_ERROR_INVALID_DATE` |
| a fixed day | `hc_parse_iso_date` | the dates of the years −9 999 999 through 9 999 999; any other text is `HC_ERROR_INVALID_DATE` |
| a year, month and day; a day of the year; 1 or 0; ISO 8601 text | `hc_gregorian_from_fixed`, `hc_day_of_year`, `hc_is_leap_year`, `hc_format_iso_date` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERROR_NO_DATA` |
| a weekday, 1 through 7 | `hc_weekday` | every `fixed` |
| a fixed day | `hc_fixed_from_unix` | every `unix_seconds`; the day is between −106 751 990 448 138 and 106 751 991 886 463 |
| a POSIX timestamp | `hc_unix_from_fixed` | `fixed` −106 751 990 448 137 through 106 751 991 886 463, the days whose midnight fits an `int64_t`; any other is `HC_ERROR_OUT_OF_RANGE` |
| TAI seconds | `hc_tai_from_unix` | every `unix_seconds` up to `INT64_MAX − 37`; TAI runs ahead of UTC, so a later one has no TAI reading an `int64_t` holds and is `HC_ERROR_OVERFLOW`; under `strict`, 1961 through the end of the announced table, else `HC_ERROR_NO_DATA` |
| seconds | `hc_tai_minus_utc` | every `unix_seconds`; under `strict`, as for `hc_tai_from_unix` |
| 1 or 0 | `hc_day_has_leap_second` | `unix_seconds` −9 223 372 036 854 720 000 through 9 223 372 036 854 719 999, the whole days of the `int64_t` range; the part-days at its two ends begin or end where no `int64_t` reaches, and are `HC_ERROR_OUT_OF_RANGE` |
| a POSIX timestamp | `hc_utc_from_tai` | every `tai_seconds`; under `strict`, as for `hc_tai_from_unix` |
| a fixed day | `hc_fixed_from_unix_in_zone` | every `unix_seconds` |
| a POSIX timestamp | `hc_unix_from_fixed_in_zone` | the `fixed` days whose start by the zone's clock fits an `int64_t`: by UTC, the fixed days −106 751 990 448 137 through 106 751 991 886 463, and a zone's offset moves each end by at most a day; any other is `HC_ERROR_OUT_OF_RANGE` |
| a TAI64 label | `hc_tai64_encode` | `tai_seconds` −4 611 686 018 427 387 904 through 4 611 686 018 427 387 903, the seconds of the labels below 2⁶³, and attoseconds below 10¹⁸; any other is `HC_ERROR_OUT_OF_RANGE` |
| TAI seconds | `hc_tai64_decode` | every label below 2⁶³, the seconds −4 611 686 018 427 387 904 through 4 611 686 018 427 387 903; a reserved label is `HC_ERROR_OUT_OF_RANGE` |
| a week, a broadcast week and a time of week | `hc_gnss_week` | `tai_seconds` from the field's week zero to the end of week 4 294 967 295 (for GPS, 315 964 819 through 2 597 596 536 585 618), the attoseconds 0 through 999 999 999 999 999 999; an earlier instant is `HC_ERROR_NO_DATA` and a later one `HC_ERROR_OVERFLOW` |
| TAI seconds | `hc_gnss_to_tai` | every week and every time of week below 604 800 s: at most 2 597 597 356 694 432, the last second of BeiDou week 4 294 967 295 |
| a full week | `hc_gnss_resolve_week` | every broadcast week that fits the field and every `reference_tai_seconds`, one before week zero counting as week zero; a broadcast week that does not fit, or an answer past week 4 294 967 295, is `HC_ERROR_OUT_OF_RANGE` |
| *N*4 and *N*T | `hc_glonass_date` | `tai_seconds` from 1996-01-01 00:00 to the end of 2099-12-31 by GLONASS time, 820 443 629 through 4 102 434 036 when the last published offset is held; outside is `HC_ERROR_OUT_OF_RANGE`, and under `strict` an instant past the leap-second table `HC_ERROR_NO_DATA` |
| a fixed day | `hc_fixed_from_ole_automation` | the values −657 434 through just below 2 958 466, the fixed days 36 160 (1 January 100) through 3 652 059 (31 December 9999); any other is `HC_ERROR_OUT_OF_RANGE` |
| an OLE Automation date | `hc_ole_automation_from_fixed` | `fixed` 36 160 (1 January 100) through 3 652 059 (31 December 9999) and a time of day from 0 to below 86 400 s; any other is `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_excel_1900_day` | `serial` 1 through 2 958 465, the fixed days 693 596 through 3 652 059, serial 60 writing none; any other is `HC_ERROR_OUT_OF_RANGE` |
| a TAI64 label | `hc_tai64_posix_plus_10_encode` | `unix_seconds` −4 611 686 018 427 387 914 through 4 611 686 018 427 387 893, the seconds whose label 2⁶² + 10 + `unix_seconds` is below 2⁶³, and attoseconds below 10¹⁸; any other is `HC_ERROR_OUT_OF_RANGE` |
| POSIX seconds | `hc_tai64_posix_plus_10_decode` | every label below 2⁶³, the POSIX seconds −4 611 686 018 427 387 914 through 4 611 686 018 427 387 893; a reserved label is `HC_ERROR_OUT_OF_RANGE` |
| POSIX seconds | `hc_uuid_timestamp` | every version 1 or version 6 UUID, whose timestamps are the POSIX seconds −12 219 292 800 (1582-10-15) through 103 072 857 660 (5236-03-31); another version is `HC_ERROR_NO_DATA` |
| an era and POSIX seconds | `hc_ntp_resolve` | `reference_unix` −9 223 372 034 707 292 160 through 9 223 372 032 498 303 360, within which every timestamp's date and POSIX second fit an `int64_t`; nearer the ends some timestamps are `HC_ERROR_OVERFLOW`, and the zero timestamp is `HC_ERROR_NO_DATA` everywhere |
| a line | `hc_uuid_timestamp_encode` | `unix_seconds` −12 219 292 800 (1582-10-15) through 103 072 857 660 (5236-03-31), up to the field's last interval, which ends at 21:21:00.6846976 UTC that day, and attoseconds below 10¹⁸; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_ntp_encode` | `unix_seconds` −9 223 372 036 854 775 808 through 9 223 372 034 645 787 007, `INT64_MAX` − 2 208 988 800, the seconds whose count from 1900 fits an `int64_t`; a later one is `HC_ERROR_OVERFLOW`, and attoseconds from 10¹⁸ `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_fat_decode` | every pair of words whose fields name a day and a time, the fixed days 722 815 (1980-01-01) through 769 565 (2107-12-31); a pair whose fields name no day or no time is `HC_ERROR_INVALID_DATE` |
| two FAT words | `hc_fat_encode` | `fixed` 722 815 (1980-01-01) through 769 565 (2107-12-31) and a time of day below 86 400 s; any other is `HC_ERROR_OUT_OF_RANGE` |
| a beat, 0 through 999 | `hc_swatch_beat` | every `unix_seconds`, and attoseconds below 10¹⁸; more attoseconds are `HC_ERROR_OUT_OF_RANGE` |
| an epoch | `hc_epoch_from_tt` | every `tt_seconds`, and attoseconds below 10¹⁸; more are `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_tt_bipm` | `tai_seconds` from 0 h UTC on the first sample's date through the last's, which the caller's series sets, and attoseconds below 10¹⁸; an instant outside the series, or an empty series, is `HC_ERROR_NO_DATA`, and more attoseconds are `HC_ERROR_OUT_OF_RANGE` |
| TT seconds | `hc_tt_from_epoch` | every finite year whose instant is within an `int64_t` of seconds of 1970 TT, about 2.9 × 10¹¹ years either side; beyond is `HC_ERROR_OVERFLOW`, and a year not finite `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_describe_day` | every `fixed`; a calendar that refuses the day says so in its own line |
| lines | `hc_calendar_units` | every `from_fixed` and `to_fixed` whose range is at most 100 000 units; a span a calendar refuses says so in its own line and counts as one, a `to_fixed` at or before `from_fixed` writes no lines, and a range of more units is `HC_ERROR_OUT_OF_RANGE`, as the WebAssembly module's [line caps](../hyper-calendar-wasm/README.md#line-caps) say |
| lines | `hc_calendars` | every `today` |
| a line | `hc_naming_period_on` | every `fixed`; a calendar the registry does not carry is `HC_ERROR_UNKNOWN` |
| a line | `hc_asian_day` | `fixed` 1 360 (23 September AD 4) through 3 652 398, the last day of the Asian year 9999; any other is `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_holidays_in_year` | every `year`; a year the table has no entries for writes no lines |
| 1 or 0; lines | `hc_holiday_is_day_off`, `hc_holidays_on`, `hc_common_worship_on` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_holy_year_on` | `fixed` 720 981 (24 December 1974) through 739 886 (27 September 2026), from the opening of the first jubilee the table carries to the day its sources were checked; any other is `HC_ERROR_NO_DATA` |
| a line | `hc_lectionary` | `fixed` 577 780 through 1 497 096, the liturgical years 1583 to 4099; any other is `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_astronomical_easter` | `year` 1583 through 2150; any other is `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_astronomical_paschal_full_moon` | `year` 1583 through 2150, the years of `hc_astronomical_easter`; any other is `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_cold_food_day` | `year` −999 through 3000 under every reckoning, the years whose winter solstice before and whose April are both in the era of `hc_term_in_effect`; any other is `HC_ERROR_OUT_OF_RANGE`, and a reckoning it does not name `HC_ERROR_UNKNOWN` |
| a line or lines | `hc_term_in_effect`, `hc_pentad_in_effect`, `hc_solar_event`, `hc_panchanga_of_day`, `hc_sunrise`, `hc_sunset`, `hc_crescent_visible` | `fixed` −365 607 through 1 095 727, the years −1000 to 3000; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_hindu_lunar_date` | `fixed` in the Śaka years 1622 through 2221 on the true sky, from Chaitra śukla 1 in March 1700 to the eve of the one in March 2300, whose days move with the place and the ayanamsa (620 627 through 839 773 at the Central Station with Lahiri's); on `surya-siddhanta`, −1 132 604 through 2 519 974, Kali Yuga 1 to 10 000; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_surya_siddhanta_sunrise` | `fixed` −1 132 604 through 2 519 974, Kali Yuga 1 to 10 000; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_surya_siddhanta_at` | `unix_seconds` −159 992 668 800 through 155 590 156 799, the days of Kali Yuga 1 to 10 000; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line or lines | `hc_sky_at`, `hc_decan_at`, `hc_solar_time`, `hc_panchanga_at` | `unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; any other is `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_solar_terms_between`, `hc_moon_phases_between` | `from_unix` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; a `to_unix` at or before it writes no lines, and a later one must be at most 32 535 216 000 and at most 400 years after it; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_chinese_marriage_augury` | `chinese_year` 4282 through 4786, whose New Year and the next both fall in the Chinese calendar's range (1645 through 2150); any other is `HC_ERROR_OUT_OF_RANGE` |
| an age | `hc_chinese_reckoned_age` | `birth_fixed` and `on_fixed` 600 460 through 785 271, the days of the Chinese calendar's range, 1645 through 2150, a birth before or on the day asked; a day before the birth is `HC_ERROR_NO_DATA`, and a day outside the range `HC_ERROR_OUT_OF_RANGE` |
| an Olympiad | `hc_ioc_olympiad` | `gregorian_year` from 1896; an earlier one is `HC_ERROR_OUT_OF_RANGE` |
| a place in the cycle, 1 through 7 | `hc_hebrew_sabbatical_cycle_year` | `hebrew_year` 1 through 9999; any other is `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_hebrew_yahrzeit`, `hc_hebrew_birthday` | `death_fixed` and `birth_fixed` −1 373 427 through 2 278 650 and `hebrew_year` 1 through 9999, the Hebrew years 1 through 9999; any other is `HC_ERROR_OUT_OF_RANGE` |
| a mission sol, from 0 or 1 | `hc_mission_sol` | the instants from the midnight that began the mission's landing sol through 100 Julian years after J2000.0 (2100-01-01T12:00 TT); an earlier instant, or one not finite, is `HC_ERROR_OUT_OF_RANGE`, a mission whose operators published no sol numbering `HC_ERROR_NO_DATA`, and a mission the table does not carry `HC_ERROR_UNKNOWN` |

A day outside the Gregorian range is `HC_ERROR_NO_DATA` from the entry
points in the third row, as it is from every calendar here: the C library
maps a calendar's own "after its supported range" to `HC_ERROR_NO_DATA`
for all of them alike. The WebAssembly module's `hc_gregorian_year` and
its neighbours answer the same day with `HC_ERR_OUT_OF_RANGE`, because
its civil exports have one sentinel for any day they cannot place.
`hc_day_of_year` and `hc_is_leap_year` read the day through the same
Gregorian conversion as `hc_gregorian_from_fixed`, and so answer with its
code: each boundary keeps one code for a day its calendars cannot place,
and the two boundaries' codes differ.

### No floor here, a floor there: a deliberate difference

The two interfaces answer different questions for the same far-past day,
and that is by design, not drift.

The WebAssembly module returns its answer and its error in the same `i64`:
a value-returning export gives either the result or an `HC_ERR_*` sentinel,
because a WebAssembly function has one return value and a trap would tear
down the instance. So every sentinel sits at or below `HC_ERR_FLOOR`,
−9 × 10¹⁵, and an export that answers in seconds must refuse a result that
would reach it, or a binding would read the answer as an error. Its
`hc_unix_from_fixed` and `hc_unix_from_fixed_in_zone` therefore refuse every
day before fixed day −104 165 947 503, about 285 million years back, with
`HC_ERR_OUT_OF_RANGE` ([its README](../hyper-calendar-wasm/README.md#ranges)).

This library returns the error as an `HcStatus` and the answer through an
out-parameter, so no value of the answer is reserved, and a floor would
refuse days for no reason a C caller has. `hc_unix_from_fixed` and
`hc_unix_from_fixed_in_zone` here answer down to where an `int64_t` runs
out: fixed day −106 751 990 448 137, about 292 billion years back, by UTC
for the zone variant. For the days from there to fixed day
−104 165 947 504, this library writes a timestamp and the WebAssembly
module refuses.

The price of each choice is the other's benefit. Here, every call costs a
status check and a pointer, and the whole `int64_t` range is usable; there,
a call is one value, and every result at or below −9 × 10¹⁵ is given up,
which for a count of seconds is everything before about 285 million years
ago. A program that uses both and compares them should expect the
WebAssembly module's refusal below its floor, and not read it as a
disagreement.

### `HC_ERROR_OVERFLOW` and `HC_ERROR_OUT_OF_RANGE` for an answer too large

The entry points above that can be asked for an answer an `int64_t`, or
a 32-bit GNSS week, cannot hold do not all say so with the same code:

- `HC_ERROR_OVERFLOW` comes from the time-scale arithmetic the entry
  point calls, which finds that a valid input's answer leaves the range:
  - `hc_tai_from_unix` for a timestamp after `INT64_MAX − 37`, where
    adding TAI − UTC leaves the range; `hc_tai_minus_utc` would report
    the same code on the same path, though an offset in whole seconds
    never comes near it;
  - `hc_ntp_resolve` for a timestamp whose date or POSIX second leaves an
    `int64_t`, near the ends of the range;
  - `hc_ntp_encode` for a second after `INT64_MAX − 2 208 988 800`, whose
    count from 1900 leaves an `int64_t`;
  - `hc_tt_from_epoch` for a year whose instant is more than an
    `int64_t` of seconds from 1970 TT;
  - `hc_gnss_week` for an instant past week 4 294 967 295, which the
    32-bit week does not hold.
- `HC_ERROR_OUT_OF_RANGE` comes from the entry point's own bound on the
  day, the timestamp or the week:
  - `hc_unix_from_fixed` and `hc_unix_from_fixed_in_zone` for a day whose
    start overflows;
  - `hc_day_has_leap_second` for the part-days at the ends of the range;
  - `hc_gnss_resolve_week` for an answer past week 4 294 967 295.

For these entry points the two codes mean the same thing: the answer
exists and is not an `int64_t`, or for the GNSS weeks not a 32-bit week.
A caller that needs to tell "too large to hold" from other failures
should test for both. The WebAssembly module has no sentinel for an
overflow and answers `HC_ERR_OUT_OF_RANGE` for all of them.

## Lines and cells

Every entry point that answers with more than one value writes UTF-8 lines,
one per entry, each ending in `\n`, with the cells of a line separated by
`\t`, and the whole NUL-terminated like every other text here. The column
orders are fixed and only ever grow at the end; a cell with nothing to say
is empty, and no cell contains a tab or a line break. They are the same
lines the WebAssembly module writes, whose
[README](../hyper-calendar-wasm/README.md) tabulates every column order,
and the sections below name the differences at this boundary: strings in
are NUL-terminated and may be null, and the length comes back through
`written`.

## Layers

The entry points come in layers, each a Cargo feature, the same layers as
the WebAssembly module's: `civil` (the default), `timestamps`, `calendars`, `holiday`,
`seasons`, `deep-time`, `tz`, `sky`, `orbital`, `planetary`, `relativity` and `full`.
One pair sits in a different layer: `hc_tai_from_unix` and
`hc_utc_from_tai` are `civil` here and `timestamps` there.
Each builds on its own —
`calendars` does not need `holiday` — and the table below names the one
each entry point needs. CI runs this crate's tests with each layer's
feature alone, through [`scripts/layer-tests.sh`](../../scripts/layer-tests.sh).

```sh
cargo build -p hyper-calendar-ffi --release --features calendars
cargo build -p hyper-calendar-ffi --release --features full
```

## What is exported

The two tables below are rendered from `src/lib.rs` by
[`crates/hyper-calendar/tests/abi.rs`](../hyper-calendar/tests/abi.rs), which
fails when they drift. An entry point without a row here does not pass CI.

<!-- generated by crates/hyper-calendar/tests/abi.rs: start -->

### Entry points

103 functions. Each is `extern "C"`, takes nothing it has to free and returns an `HcStatus`. The feature column is the Cargo feature the library has to be built with for the entry point to exist.

| Prototype | Feature | What it does |
| --- | --- | --- |
| `HcStatus hc_version(char *buffer, size_t capacity, size_t *written);` | always | The library version, as a NUL-terminated string. |
| `HcStatus hc_gregorian_to_fixed(int64_t year, uint8_t month, uint8_t day, int64_t *out_fixed);` | `civil` | The fixed day number of a proleptic Gregorian date. |
| `HcStatus hc_gregorian_from_fixed(int64_t fixed, int64_t *out_year, uint8_t *out_month, uint8_t *out_day);` | `civil` | The proleptic Gregorian date on a fixed day. |
| `HcStatus hc_weekday(int64_t fixed, uint8_t *out_weekday);` | `civil` | The ISO 8601 weekday of a fixed day, Monday = 1 through Sunday = 7. |
| `HcStatus hc_day_of_year(int64_t fixed, uint32_t *out_day_of_year);` | `civil` | The 1-based day of the Gregorian year on a fixed day. |
| `HcStatus hc_is_leap_year(int64_t fixed, int *out_is_leap);` | `civil` | Whether the Gregorian year on a fixed day is a leap year: writes 1 or 0. |
| `HcStatus hc_fixed_from_unix(int64_t unix_seconds, int64_t *out_fixed);` | `civil` | The fixed day a POSIX timestamp falls on, in UTC. |
| `HcStatus hc_unix_from_fixed(int64_t fixed, int64_t *out_unix_seconds);` | `civil` | The POSIX timestamp of midnight UTC on a fixed day. |
| `HcStatus hc_parse_iso_date(const char *text, int64_t *out_fixed);` | `civil` | Parse an ISO 8601 date from a NUL-terminated UTF-8 string into its fixed day number. |
| `HcStatus hc_format_iso_date(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `civil` | Render a fixed day as an ISO 8601 date into a caller-owned buffer. |
| `HcStatus hc_tai_from_unix(int64_t unix_seconds, int strict, int64_t *out_tai_seconds, uint64_t *out_attoseconds);` | `civil` | Convert a POSIX timestamp to a TAI reading in seconds and attoseconds. |
| `HcStatus hc_tai_minus_utc(int64_t unix_seconds, int strict, int64_t *out_offset);` | `civil` | `TAI - UTC` in whole seconds at a POSIX timestamp. |
| `HcStatus hc_day_has_leap_second(int64_t unix_seconds, int *out_has_leap);` | `civil` | Whether a POSIX timestamp names a day that ends with an inserted leap second. |
| `HcStatus hc_utc_from_tai(int64_t tai_seconds, int strict, int64_t *out_unix_seconds, int *out_is_leap_second);` | `civil` | Convert a TAI reading back to a UTC label, naming a leap second when the instant falls inside one. |
| `HcStatus hc_tai64_encode(int64_t tai_seconds, uint64_t attoseconds, const char *format, char *buffer, size_t capacity, size_t *written);` | `timestamps` | A TAI instant as a TAI64, TAI64N or TAI64NA label in lower-case hexadecimal, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_tai64_decode(const char *hex, int64_t *out_tai_seconds, uint64_t *out_attoseconds);` | `timestamps` | A TAI64, TAI64N or TAI64NA label in hexadecimal read back into the TAI seconds and attoseconds of the instant it names. |
| `HcStatus hc_gnss_week(const char *numbering, int64_t tai_seconds, uint64_t attoseconds, uint32_t *out_week, uint32_t *out_broadcast, uint32_t *out_tow_seconds, uint64_t *out_tow_attoseconds);` | `timestamps` | The GNSS week and time of week of a TAI instant. |
| `HcStatus hc_gnss_to_tai(const char *numbering, uint32_t week, uint32_t tow_seconds, uint64_t tow_attoseconds, int64_t *out_tai_seconds, uint64_t *out_attoseconds);` | `timestamps` | The TAI instant of a full GNSS week and a time of week. |
| `HcStatus hc_gnss_resolve_week(const char *numbering, uint32_t broadcast, const char *rule, int64_t reference_tai_seconds, uint32_t *out_week);` | `timestamps` | The full GNSS week a broadcast week names, by a rollover rule and a reference instant. |
| `HcStatus hc_glonass_date(int64_t tai_seconds, uint64_t attoseconds, int strict, uint32_t *out_four_year_interval, uint32_t *out_day);` | `timestamps` | GLONASS's four-year interval N4 and day N_T at a TAI instant. |
| `HcStatus hc_fixed_from_ole_automation(double value, int64_t *out_fixed, double *out_seconds_of_day);` | `timestamps` | The fixed day and the time of day, in seconds, of an OLE Automation date. |
| `HcStatus hc_ole_automation_from_fixed(int64_t fixed, double seconds_of_day, double *out_value);` | `timestamps` | The OLE Automation date of a fixed day and a time of day in seconds. |
| `HcStatus hc_excel_1900_day(int64_t serial, int64_t *out_fixed, int *out_phantom);` | `timestamps` | What an Excel 1900 serial names. |
| `HcStatus hc_tai64_posix_plus_10_encode(int64_t unix_seconds, uint64_t attoseconds, const char *format, char *buffer, size_t capacity, size_t *written);` | `timestamps` | A POSIX instant as a TAI64 or TAI64N label in the `tai64-posix-plus-10` convention, in lower-case hexadecimal, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_tai64_posix_plus_10_decode(const char *hex, int64_t *out_unix_seconds, uint64_t *out_attoseconds);` | `timestamps` | A TAI64 or TAI64N label in the `tai64-posix-plus-10` convention read back into the POSIX seconds and attoseconds of the instant it names. |
| `HcStatus hc_uuid_timestamp(const char *uuid, int *out_version, uint64_t *out_timestamp, int64_t *out_unix_seconds, uint64_t *out_attoseconds);` | `timestamps` | The version, the 60-bit timestamp and the POSIX instant of a version 1 or version 6 UUID. |
| `HcStatus hc_ntp_resolve(uint32_t seconds, uint32_t fraction, int64_t reference_unix, int32_t *out_era, uint32_t *out_offset, uint64_t *out_fraction, int64_t *out_unix_seconds, uint64_t *out_attoseconds);` | `timestamps` | A 64-bit NTP timestamp placed in its era by a reference POSIX second: the era, the era offset, the fraction in 2⁻⁶⁴ s units, and the POSIX seconds and attoseconds. |
| `HcStatus hc_uuid_timestamp_encode(int64_t unix_seconds, uint64_t attoseconds, char *buffer, size_t capacity, size_t *written);` | `timestamps` | The 60-bit UUID timestamp of a POSIX instant, and the time fields a version 1 and a version 6 UUID write it in, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ntp_encode(int64_t unix_seconds, uint64_t attoseconds, char *buffer, size_t capacity, size_t *written);` | `timestamps` | The NTP date and timestamp of a POSIX instant, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_fat_decode(uint16_t date, uint16_t time, int64_t *out_fixed, uint32_t *out_seconds_of_day);` | `timestamps` | The local reading a FAT date word and time word name: its fixed day and the seconds into it, always even. |
| `HcStatus hc_fat_encode(int64_t fixed, uint32_t seconds_of_day, uint16_t *out_date, uint16_t *out_time);` | `timestamps` | The FAT date and time words of a fixed day and a time of day in whole seconds, the second rounded down to an even one. |
| `HcStatus hc_swatch_beat(int64_t unix_seconds, uint64_t attoseconds, uint16_t *out_beat);` | `timestamps` | The Swatch Internet Time at a POSIX instant, 0 through 999: the thousandth of the day of Biel Mean Time, UTC+1 all year, that it falls in, so @000 begins at 23:00 UTC. |
| `HcStatus hc_epoch_from_tt(const char *notation, int64_t tt_seconds, uint64_t attoseconds, double *out_year);` | `timestamps` | The Julian or Besselian epoch of a TT instant, as a year with a fraction. |
| `HcStatus hc_tt_from_epoch(const char *notation, double year, int64_t *out_tt_seconds, uint64_t *out_attoseconds);` | `timestamps` | The TT instant of a Julian or Besselian epoch, as whole seconds from 1970-01-01 00:00:00 TT and attoseconds. |
| `HcStatus hc_tt_bipm(const char *series, int64_t tai_seconds, uint64_t attoseconds, int strict, char *buffer, size_t capacity, size_t *written);` | `timestamps` | TT(BIPM) at a TAI instant, read from a realisation the caller supplies, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_describe_day(int64_t fixed, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | One fixed day in every registered calendar, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_calendar_units(const char *id, uint32_t unit, int64_t from_fixed, int64_t to_fixed, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The days from `from_fixed` up to but not including `to_fixed` as one calendar's eras, years, months or days, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_calendars(int64_t today, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | Every registered calendar, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_calendar_list(const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | Every registered calendar by name alone, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_locales(char *buffer, size_t capacity, size_t *written);` | `calendars` | Every locale the library carries, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_first_day_of_week(const char *locale, uint8_t *out_weekday);` | `calendars` | The ISO 8601 weekday of the first day of the week in a locale, Monday = 1 through Sunday = 7. |
| `HcStatus hc_gregorian_adoption(const char *region, char *buffer, size_t capacity, size_t *written);` | `calendars` | The steps by which a country adopted the Gregorian calendar, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_naming_period_on(const char *calendar, int64_t fixed, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | Which month and weekday names a locale writes for a calendar on a fixed day, where a government renamed them for a period, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_panchanga_at(int64_t unix_seconds, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The yoga and the karaṇa in progress at a POSIX timestamp, as two NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_panchanga_of_day(int64_t fixed, double latitude, double longitude, double elevation, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The yoga and the karaṇa a fixed day carries at a place, the ones in progress at its sunrise, as two NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_hindu_lunar_date(const char *sky, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `calendars` | The Hindu lunisolar date of a fixed day at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_surya_siddhanta_at(int64_t unix_seconds, char *buffer, size_t capacity, size_t *written);` | `calendars` | The *Sūrya Siddhānta*'s Sun and Moon at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_surya_siddhanta_sunrise(int64_t fixed, double latitude, double longitude, char *buffer, size_t capacity, size_t *written);` | `calendars` | The *Sūrya Siddhānta*'s sunrise on a fixed day at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_crescent_visible(const char *criterion, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `calendars` | Whether the young crescent should have been visible on the evening that begins a fixed day, from a place, by a named criterion, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ioc_olympiad(int64_t gregorian_year, int64_t *out_olympiad);` | `calendars` | The number of the modern Olympiad a Gregorian year belongs to. |
| `HcStatus hc_hebrew_yahrzeit(int64_t death_fixed, int64_t hebrew_year, int64_t *out_fixed);` | `calendars` | The fixed day of the yahrzeit in a Hebrew year of a death on the Hebrew date a fixed day names. |
| `HcStatus hc_hebrew_birthday(int64_t birth_fixed, int64_t hebrew_year, int64_t *out_fixed);` | `calendars` | The fixed day of the birthday in a Hebrew year of a birth on the Hebrew date a fixed day names. |
| `HcStatus hc_chinese_reckoned_age(int64_t birth_fixed, int64_t on_fixed, uint32_t *out_age);` | `calendars` | A person's age as the Chinese count reckons it on a fixed day. |
| `HcStatus hc_chinese_marriage_augury(int64_t chinese_year, char *buffer, size_t capacity, size_t *written);` | `calendars` | The marriage augury of a Chinese year, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_hebrew_sabbatical_cycle_year(int64_t hebrew_year, int64_t *out_place);` | `calendars` | The place of a Hebrew year in the seven-year sabbatical cycle, 1 through 7. |
| `HcStatus hc_asian_day(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `calendars` | A fixed day in the calendar of the Roman province of Asia as the calendar writes it, unnumbered days included, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_holiday_is_day_off(const char *code, const char *region, int64_t fixed, int *out_is_day_off);` | `holiday` | Whether a fixed day is a day off in a holiday table. |
| `HcStatus hc_holidays_in_year(const char *code, const char *region, int64_t year, char *buffer, size_t capacity, size_t *written);` | `holiday` | The holidays of a Gregorian year in a table, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_holiday_codes(char *buffer, size_t capacity, size_t *written);` | `holiday` | The identifier of every holiday table, one per line, NUL-terminated. |
| `HcStatus hc_holidays_on(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | Every holiday on one fixed day across every table, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_holiday_tables(const char *locale, char *buffer, size_t capacity, size_t *written);` | `holiday` | Every holiday table with its kind, names and sources, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_lectionary(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | The lectionary cycles of a fixed day, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_astronomical_easter(int64_t year, int64_t *out_fixed);` | `holiday` | The fixed day of Easter Sunday of a Gregorian year by the astronomical reckoning at the meridian of Jerusalem. |
| `HcStatus hc_astronomical_paschal_full_moon(int64_t year, int64_t *out_fixed);` | `holiday` | The fixed day of the paschal full moon of a Gregorian year by the astronomical reckoning at the meridian of Jerusalem. |
| `HcStatus hc_holy_year_on(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | The Holy Year of the Catholic Church a fixed day falls in, if any, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_common_worship_on(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | The rank of every *Common Worship* celebration kept on a fixed day, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_term_in_effect(int64_t fixed, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `seasons` | The solar term in effect on a fixed day at a meridian, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_pentad_in_effect(int64_t fixed, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `seasons` | The pentad (候) in effect on a fixed day at a meridian, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_cold_food_day(const char *convention, int64_t year, int64_t *out_fixed);` | `seasons` | The fixed day of 寒食, the Cold Food Day, of a Gregorian year under a named reckoning. |
| `HcStatus hc_place_years_ago(double years_ago, double std_dev_years, const char *locale, char *buffer, size_t capacity, size_t *written);` | `deep-time` | A moment some years before the present, placed in every chronology at once, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_cosmic_events(const char *locale, char *buffer, size_t capacity, size_t *written);` | `deep-time` | Every cosmic epoch and every dated cosmic event, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_geologic_intervals(uint32_t rank, const char *locale, char *buffer, size_t capacity, size_t *written);` | `deep-time` | Every interval of one rank of the geologic time scale, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_fixed_from_unix_in_zone(int64_t unix_seconds, const char *zone, int64_t *out_fixed);` | `tz` | The fixed day a POSIX timestamp falls on by the wall clock of a zone. |
| `HcStatus hc_unix_from_fixed_in_zone(int64_t fixed, const char *zone, int64_t *out_unix_seconds);` | `tz` | The POSIX timestamp at which a fixed day begins by the wall clock of a zone. |
| `HcStatus hc_zone_load(const char *name, const uint8_t *tzif, size_t tzif_len);` | `tz` | Give the library a zone's TZif data under an IANA name. |
| `HcStatus hc_zones(const char *locale, char *buffer, size_t capacity, size_t *written);` | `tz` | Every zone of the IANA database's `zone1970.tab` with its principal location, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_zone_location(const char *zone, const char *locale, char *buffer, size_t capacity, size_t *written);` | `tz` | Where one zone is, as the NUL-terminated UTF-8 line `hc_zones` writes for it, in a caller-owned buffer. |
| `HcStatus hc_sky_at(int64_t unix_seconds, char *buffer, size_t capacity, size_t *written);` | `sky` | The Sun and the Moon at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_solar_terms_between(int64_t from_unix, int64_t to_unix, char *buffer, size_t capacity, size_t *written);` | `sky` | Every solar term whose instant falls in `[from_unix, to_unix)`, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_moon_phases_between(int64_t from_unix, int64_t to_unix, char *buffer, size_t capacity, size_t *written);` | `sky` | Every new moon, first quarter, full moon and last quarter whose instant falls in `[from_unix, to_unix)`, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_decan_at(int64_t unix_seconds, char *buffer, size_t capacity, size_t *written);` | `sky` | The decan the Sun is in at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_horizons(char *buffer, size_t capacity, size_t *written);` | `sky` | Every named horizon a rising or a setting can be measured against, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_sunrise(const char *horizon, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | Sunrise on a fixed day at a place against a named horizon, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_sunset(const char *horizon, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | Sunset on a fixed day at a place against a named horizon, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_earth_rotation_angle(double ut1_unix_seconds, double *out_degrees);` | `sky` | The Earth Rotation Angle at a UT1 instant, in degrees, 0 to 360. |
| `HcStatus hc_gmst_iau2006(double ut1_unix_seconds, double *out_degrees);` | `sky` | The Greenwich mean sidereal time by the IAU 2006 convention at a UT1 instant, in degrees, 0 to 360. |
| `HcStatus hc_gmst_iau1982(double ut1_unix_seconds, double *out_degrees);` | `sky` | The Greenwich mean sidereal time by the IAU 1982 convention at a UT1 instant, in degrees, 0 to 360. |
| `HcStatus hc_ut2_minus_ut1(double ut1_unix_seconds, double *out_seconds);` | `sky` | UT2 − UT1 at a UT1 instant, in seconds. |
| `HcStatus hc_solar_time(const char *clock, int64_t unix_seconds, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | A local clock's reading at a POSIX timestamp and a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_solar_event(const char *event, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | A named time of day on a fixed day at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_hjd_tt(double tt_julian_date, double right_ascension, double declination, char *buffer, size_t capacity, size_t *written);` | `sky` | The Heliocentric Julian Date in TT, HJD_TT, of a Julian Date of TT for a target, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_hjd_utc(double utc_julian_date, double right_ascension, double declination, int strict, char *buffer, size_t capacity, size_t *written);` | `sky` | The Heliocentric Julian Date in UTC, HJD_UTC, of a Julian Date of UTC for a target, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_orbit_at(double years_before_1950, char *buffer, size_t capacity, size_t *written);` | `orbital` | Earth's orbital elements and the June insolation at 65° N at an epoch, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_orbit_series(double from_years_before_1950, double to_years_before_1950, double step_years, char *buffer, size_t capacity, size_t *written);` | `orbital` | The line of `hc_orbit_at` at every epoch from `from_years_before_1950` to `to_years_before_1950` in steps of `step_years`, each with the epoch as a first column, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_mars_time(double unix_seconds, double east_longitude_degrees, char *buffer, size_t capacity, size_t *written);` | `planetary` | Mars at a POSIX instant and an east longitude, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_missions(char *buffer, size_t capacity, size_t *written);` | `planetary` | Every surface mission on Mars and the rules of its sol count, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_mission_sol(const char *mission, double unix_seconds, int64_t *out_sol);` | `planetary` | The sol number of a Mars surface mission at a POSIX instant, by the mission's own clock. |
| `HcStatus hc_bodies(char *buffer, size_t capacity, size_t *written);` | `planetary` | Every body `hc-planetary` carries, with its solar day, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_body_time(const char *body, double unix_seconds, double east_longitude_degrees, char *buffer, size_t capacity, size_t *written);` | `planetary` | Local mean solar time on a body at a POSIX instant and an east longitude, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_circad_date(const char *calendar, double unix_seconds, char *buffer, size_t capacity, size_t *written);` | `planetary` | The date at a POSIX instant in a calendar of another body's days, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_proper_time(double speed_metres_per_second, double coordinate_seconds, char *buffer, size_t capacity, size_t *written);` | `relativity` | A clock moving at a constant speed while some coordinate time passes, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_gravitational_dilation(const char *body, double radius_metres, char *buffer, size_t capacity, size_t *written);` | `relativity` | A clock held still at a radius from a body's centre, against one far from every mass, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_gravitating_bodies(char *buffer, size_t capacity, size_t *written);` | `relativity` | Every body `hc-relativity` carries a gravitational parameter for, as NUL-terminated UTF-8 lines in a caller-owned buffer. |

### Status codes

`HcStatus` is `int`. Zero is success and every failure is negative. The values are stable.

| Code | Value | Meaning |
| --- | --- | --- |
| `HC_OK` | 0 | The call succeeded. |
| `HC_ERROR_NULL_POINTER` | -1 | A required pointer was null. |
| `HC_ERROR_OUT_OF_RANGE` | -2 | A field was outside its valid range. |
| `HC_ERROR_INVALID_DATE` | -3 | The date does not exist in the requested calendar. |
| `HC_ERROR_OVERFLOW` | -4 | Arithmetic left the representable range. |
| `HC_ERROR_BUFFER_TOO_SMALL` | -5 | The supplied buffer was too small; the required length, including the terminating NUL, was written out. |
| `HC_ERROR_NO_DATA` | -6 | The value lies outside the range where the requested model has data. |
| `HC_ERROR_UNKNOWN` | -7 | The requested calendar, table or identifier is not known. |
| `HC_ERROR_NOT_UTF8` | -8 | A string argument was not valid UTF-8. |
| `HC_ERROR_MALFORMED` | -9 | Data was not in the format the call expects. |
<!-- generated by crates/hyper-calendar/tests/abi.rs: end -->

## Time scales and day counts

The `timestamps` feature carries `hc_tai64_encode`, `hc_tai64_decode`,
`hc_gnss_week`, `hc_gnss_to_tai`, `hc_gnss_resolve_week`,
`hc_glonass_date`, `hc_fixed_from_ole_automation`,
`hc_ole_automation_from_fixed` and `hc_excel_1900_day`, from the same
`hyper_calendar::time_lines` as the WebAssembly module, whose README
describes each. A TAI instant is two arguments or two out-parameters: an
`int64_t` of whole seconds from 1970-01-01 00:00:00 TAI, floored, and a
`uint64_t` of attoseconds into that second, below 10¹⁸. Where the module
writes a line of numbers this library writes out-parameters — the TAI
seconds and attoseconds of a label, the week, broadcast week and time of
week of an instant, *N*4 and *N*T, the day and seconds of an OLE date —
and only the label itself is text. Names are NUL-terminated and a null
one is `HC_ERROR_NULL_POINTER`: `format` (`tai64`, `tai64n`, `tai64na`),
`numbering` (`gps-lnav-week`, `gps-cnav-week`, `galileo-week`,
`beidou-week`, `navic-week`) and `rule` (`not-before`, `nearest`), in any
case. `hc_excel_1900_day` writes `out_phantom = 1` for serial 60, the
29 February 1900 Excel counts and no calendar has, and leaves `out_fixed`
as it was: the serial is named, not dated.

The same layer carries the other binary and written forms of an instant,
each described in the WebAssembly module's README:
`hc_tai64_posix_plus_10_encode` and `hc_tai64_posix_plus_10_decode`, TAI64
labels in daemontools' convention on an ordinary clock, 2⁶² + 10 + the
POSIX seconds, which read and write a POSIX instant with no leap-second
table; `hc_uuid_timestamp`, the version, 60-bit timestamp and POSIX
instant of a version 1 or version 6 UUID in RFC 9562's string form;
`hc_ntp_resolve`, an NTP timestamp placed in the era within 2³¹ s of a
reference POSIX second, its era, era offset, 64-bit fraction and POSIX
instant through out-parameters; `hc_uuid_timestamp_encode` and
`hc_ntp_encode`, the other way, a POSIX instant's 60-bit UUID timestamp
with the first three groups of a version 1 and a version 6 UUID that carry
it, and its NTP era, era offset, fraction and both wire layouts in
hexadecimal, each as the module's line; `hc_fat_decode` and `hc_fat_encode`, the
FAT date and time words as a fixed day and the even seconds into it;
`hc_swatch_beat`, the Swatch Internet Time beat; and `hc_epoch_from_tt`
and `hc_tt_from_epoch`, the Julian and Besselian epochs of a TT instant,
whole seconds from 1970-01-01 00:00:00 TT and attoseconds, both ways. A
null `notation` for `hc_tt_from_epoch`, as an empty one, reads the year
as SOFA reads an epoch without a letter: Besselian before 1984.0, Julian
from it. `hc_tt_bipm(series, tai_seconds, attoseconds, strict, buffer,
capacity, written)` writes TT(BIPM) at a TAI instant from a realisation
the caller supplies as NUL-terminated text, one line per sample of the
Modified Julian Date and TT(BIPMxx) − TAI − 32.184 s in microseconds: the
offset from TT(TAI), TT(BIPMxx) − TAI and the TT(BIPMxx) reading, never
extrapolated past the series.
`hc_tai_from_unix` and `hc_utc_from_tai` are in `civil` here;
their WebAssembly twins, lines of the same values, are in the module's
`timestamps`, to keep its `civil` small.

## Every calendar

`hc_describe_day(fixed, locale, buffer, capacity, written)` needs the
`calendars` feature and writes one line per calendar the facade registers,
in registry order, in the eighteen columns the WebAssembly module's README
lists: identifier, English name, era code, era label, year, month ordinal,
leap-month flag, month label, day, leap-day flag, extras, error code, error
name, standing, day boundary, the date as the locale writes it, the
locale used, and which civil day names a day that does not begin at
midnight — `start`, `end`, or empty for midnight. A calendar that cannot name the day is still a line, with its
date columns, standing and formatted date empty and the error code and
name — the stable ones `CalendarError` gives every refusal — saying why.
`locale` is a NUL-terminated BCP 47 tag, the word `native` for each
calendar's own language, or null, and the last column says which locale
a line was rendered in. It is chosen in this order, and a calendar's own
language is used only when `native` asks for it:

1. With `native`, each calendar's own language.
2. Otherwise, when the tag's data names the calendar, the tag with the
   calendar's own names.
3. Otherwise, when the tag parses and some data answers for it, English.
4. Otherwise — a tag that does not parse, or that no data answers for —
   the root locale `und`, whose month names are CLDR's `M01`..`M12`; ask
   for `en` for English.

`hc_calendar_units(id, unit, from_fixed, to_fixed, locale, buffer, capacity, written)`
writes the days from `from_fixed` up to but not including `to_fixed` as
one calendar's eras (`unit` 0), years (1), months (2) or days (3), one
line per span in the eight columns the WebAssembly module's README lists:
start, end (exclusive), label, leap flag, standing, error code, error name
and locale used. A null `id` is `HC_ERROR_NULL_POINTER`; an identifier or
unit the library does not know is `HC_ERROR_UNKNOWN`; an empty range is an
empty string. `hc_calendars(today, locale, buffer, capacity, written)`
lists every registered calendar in its eleven columns — identifier, the
locale's name for it (empty where the locale has none: only `native` names
a calendar in its own language), English name, earliest, latest, the four
has-unit flags, native locales and standing on `today` —
`hc_calendar_list(locale, buffer, capacity, written)` the same calendars
by name alone in six columns — identifier, the locale's name for it as
`hc_calendars` has it, English name, the locale used (the tag of the data
the name came from, empty with the name), the crate that registers it and
the native locales as `hc_calendars` has them, `;`-joined BCP 47 tags or
empty, so that a menu can put a reader's own calendars first
— converting no day, for a menu, which is asked for far more often than a
day is described —
`hc_locales(buffer, capacity, written)` every locale in its seven: tag,
English name, native name, the three Gregorian coverage flags and the
calendars it names — `hc_first_day_of_week(locale, out_weekday)` the ISO
weekday, Monday = 1 through Sunday = 7, the locale's week begins on by
CLDR 48's week data, with a null or unparsable tag as `und`, Monday — and
`hc_gregorian_adoption(region, buffer, capacity, written)` the steps by
which the country with the ISO 3166-1 alpha-2 code `region` adopted the
Gregorian calendar, one line per step in seven columns: the last day of the old reckoning and the first of the new as
fixed days, the old calendar's identifier, the scope (`civil`,
`ecclesiastical` or `partial`), the instrument with its date, the new
calendar's identifier and the polity. A code the library does not know is
an empty string, and a null `region` is `HC_ERROR_NULL_POINTER`. The
columns are the same as the module's, and the README there describes
each.

`hc_naming_period_on(calendar, fixed, locale, buffer, capacity, written)`
writes the module's line of the month and weekday names a government
decreed for a period, where one applies to the locale and the calendar on
the day: the state, `in-force`, `undecided` or `ordinary`, and for a
period its identifier, its names for the day's month and weekday, the
weekday name's English meaning, the days that bound it and its sources.
The one period carried is Turkmenistan's of 2002 to 2008. A null
`calendar` is `HC_ERROR_NULL_POINTER`, and one the registry does not carry
`HC_ERROR_UNKNOWN`.

## The pañcāṅga and anniversaries

`hc_panchanga_at(unix_seconds, ayanamsa, buffer, capacity, written)` and
`hc_panchanga_of_day(fixed, latitude, longitude, elevation, ayanamsa,
buffer, capacity, written)` need the `calendars` feature and write the
WebAssembly module's two lines, the yoga's and the karaṇa's, in its eight
columns; `ayanamsa` is a NUL-terminated name, `Lahiri`, `Raman`,
`Krishnamurti` or `Fagan-Bradley`, and a day without a sunrise at the
place is `HC_ERROR_NO_DATA`. `hc_ioc_olympiad(gregorian_year,
out_olympiad)`, `hc_hebrew_yahrzeit(death_fixed, hebrew_year, out_fixed)`
and `hc_hebrew_birthday(birth_fixed, hebrew_year, out_fixed)` write one
`int64_t` each; a Hebrew date crosses as the fixed day whose daylight
carries it. `hc_chinese_reckoned_age(birth_fixed, on_fixed, out_age)`
writes a `uint32_t`, the Chinese count's age, with a day before the birth
`HC_ERROR_NO_DATA`, and `hc_chinese_marriage_augury(chinese_year, buffer,
capacity, written)` the module's line of the augury and its two 立春
flags.

`hc_hindu_lunar_date(sky, fixed, latitude, longitude, elevation, buffer,
capacity, written)` writes the module's line of the amānta lunisolar date
of a day read at the sunrise of a place the caller gives — the Śaka and
Vikrama years, the month, the intercalary flag, the tithi, the repeated
flag and the sunrise — on the true sky in the zodiac of a named ayanamsa,
as `hindu-lunar` reads it at the Central Station, or, with `sky`
`surya-siddhanta`, on the *Sūrya Siddhānta*'s, as
`hindu-lunar-surya-siddhanta` reads it at Ujjain.
`hc_surya_siddhanta_at(unix_seconds, buffer, capacity, written)` writes
the Siddhānta's Sun and Moon at an instant, with the elongation, the tithi
and the Sun's sign, and `hc_surya_siddhanta_sunrise(fixed, latitude,
longitude, buffer, capacity, written)` its sunrise on a day at a place, a
place beyond 65° of latitude being `HC_ERROR_OUT_OF_RANGE`.
`hc_crescent_visible(criterion, fixed, latitude, longitude, elevation,
buffer, capacity, written)` writes whether the young crescent should have
been visible on the evening that begins the day by `shaukat`, `yallop` or
`saudi-rule`, with the moment the evening is judged at and what the
criteria read there. The columns are the WebAssembly module's README's.

`hc_hebrew_sabbatical_cycle_year(hebrew_year, out_place)` writes a Hebrew
year's place in the seven-year sabbatical cycle, 1 through 7, the seventh
being *shemittah*: 5782 and 5789 are sabbatical years.
`hc_asian_day(fixed, buffer, capacity, written)` writes the module's line
of a day in the calendar of the province of Asia as the calendar writes
it: the year, the month and its name, `unnumbered` for Sebaste and the
other days before day 1 or `numbered`, and the day's number or its place
among the unnumbered days.

## Holidays

The four `hc_holiday_*` entry points, `hc_holidays_on`, `hc_holiday_tables`,
`hc_lectionary`, `hc_astronomical_easter`,
`hc_astronomical_paschal_full_moon`, `hc_holy_year_on` and
`hc_common_worship_on` need the `holiday` feature:

```sh
cargo build -p hyper-calendar-ffi --release --features holiday
```

It compiles every table of `hc-holiday` into the library — the countries,
the exchanges, the traditions and the international days. A table is named
by its identifier: a country's ISO 3166-1 alpha-2 code (`JP`), an exchange's
ISO 10383 Market Identifier Code (`XNYS`), a tradition's slug
(`christian-western`) or `un-days`, and `hc_holiday_codes` lists them all.
`hc_holiday_is_day_off` answers for one day; `hc_holidays_in_year` writes a
year as tab-separated, NUL-terminated lines — the ISO date, the name, the
local name, the kind, the confidence, `1` for a substitute day and the date it
stands in for — reporting the length it needs through `written` like every
other text function here.

The string arguments fail the same way in both, and the same way as in the
WebAssembly module: a null `code` is `HC_ERROR_NULL_POINTER`, a `code` or
`region` that is not UTF-8 is `HC_ERROR_NOT_UTF8`, and a `code` that names
no table is `HC_ERROR_UNKNOWN`. A null or empty `region` is no region.

```c
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef int HcStatus;
#define HC_OK 0
HcStatus hc_holidays_in_year(const char *code, const char *region, int64_t year,
                             char *buffer, size_t capacity, size_t *written);

int main(void) {
    size_t need = 0;
    hc_holidays_in_year("JP", NULL, 2026, NULL, 0, &need);   /* HC_ERROR_BUFFER_TOO_SMALL */
    char *lines = malloc(need);
    if (lines != NULL && hc_holidays_in_year("JP", NULL, 2026, lines, need, &need) == HC_OK) {
        fputs(lines, stdout);
    }
    free(lines);
    return 0;
}
```

`hc_holidays_on(fixed, buffer, capacity, written)` writes every entry on
one day across every table `hc_holiday_codes` lists, in that order, each
evaluated nationwide, one line per (table, entry): the table's identifier,
its English name, the holiday's English name, its local name, the kind
(`public`, `bank`, `religious`, `observance`, `school`, `workday`, or
`gap`), the confidence, the instrument the rule cites, `1` for a substitute
and the fixed day it stands in for. A `gap` line is a holiday the table
could not place in the day's year — its calendar's range ended, or the
year's announcement has not been read — reported so the caller can say so.
Each table is evaluated for the one day (`HolidayCalendar::for_day`), which
answers exactly what the whole year would at about a third of the cost.

`hc_holiday_tables(locale, buffer, capacity, written)` describes every
table in `hc_holiday_codes` order, in the eight columns of the WebAssembly
module's README: the code, the kind, the name in the locale, the English
name, the locale that answered, the sources, the country of a subdivision
or an exchange where its table records one, and the short name. A country
is named by CLDR 48's territory name in the `locale` where `hc-i18n`
carries one, and else, as for a null `locale`, by CLDR's English name;
an exchange, a tradition and a set of observances by the table's English
name; the tag that answered is in column 5. Column 8 is
CLDR 48's `alt="short"` name beside a CLDR name in column 3, from the same
data (`Hong Kong` under `en`, 香港 under `ja`), and empty elsewhere. `hc_lectionary(fixed, buffer,
capacity, written)` writes the liturgical year, the Sunday cycle, the
Roman weekday cycle and the RCL Proper of a day, and
`hc_astronomical_easter(year, out_fixed)` the fixed day of Easter by the
astronomical reckoning at Jerusalem, 1583 to 2150, and
`hc_astronomical_paschal_full_moon(year, out_fixed)` the day of the full
moon it is the Sunday after.

`hc_holy_year_on(fixed, buffer, capacity, written)` writes the module's
line of the Holy Year of the Catholic Church a day falls in: `within` or
`outside`, and within a jubilee its title, kind, Pope and bull, the day
the bull was given and its first and last days in Rome and in the
dioceses. The table holds the jubilees of 1975 to 2025, and a day before
24 December 1974 or after 27 September 2026 is `HC_ERROR_NO_DATA`.
`hc_common_worship_on(fixed, buffer, capacity, written)` writes the rank of
each *Common Worship* celebration kept on a day, one line each: the title,
which is its name in `hc_holidays_on`'s `common-worship` table, the rank
and the rank's English name. The Festivals the Rules leave without a day
are `gap` lines of `hc_holidays_on`.

## Almanac

`hc_term_in_effect` and `hc_pentad_in_effect` need the `seasons` feature.
Each writes one line for a fixed day at a meridian: the index (from 春分 at
0: 24 terms, 72 pentads), the Chinese name, the Japanese name, the fixed
day the period began, the last fixed day before the next begins, and the
sources of the Chinese and the Japanese names. `meridian` is a
NUL-terminated name — `universal`, `japan`, `china`, `korea`, `india` or
`china-before-1929`, in any case — or a longitude in decimal degrees east of
Greenwich, read as local mean solar time; null or empty is `universal`, and
anything else is `HC_ERROR_UNKNOWN`.

`hc_cold_food_day(convention, year, out_fixed)`, in the same feature,
writes the fixed day of 寒食 under a NUL-terminated reckoning:
`hanshi-solstice-105`, `hanshi-eve-of-qingming` or `hansik`, as the
WebAssembly module's README describes them, in any case; another is
`HC_ERROR_UNKNOWN`, and a year outside −999 to 3000 `HC_ERROR_OUT_OF_RANGE`.

## Deep time

`hc_place_years_ago`, `hc_cosmic_events` and `hc_geologic_intervals` need
the `deep-time` feature, take a NUL-terminated BCP 47 `locale` (or null)
after their other arguments, and write lines of the same fifteen columns:
kind, name, scope, the older bound's value, uncertainty, significant figures
and approximate flag, the same four for the younger bound, the unit, the
description, the source, and the geological chart's own name for an
interval in the locale's language, from the ICS's translations, empty for
every other row and every language the chart has no names in.
`hc_place_years_ago(years_ago, std_dev_years, locale, ...)` counts back from the present as `hc-deep-time` defines it — the
Planck 2018 age of the universe, not the BP datum of 1950 and not the
caller's clock; the crate ignores the difference, which lies below the
smallest uncertainty in its tables — and places the moment in its cosmic
epoch, the last dated cosmic event, its future era if it lies ahead, its
geologic chain and its archaeological period. A moment up to a century
ahead is still placed in the intervals that end at the present — the
chart's youngest chain, the Modern period, the last cosmic epoch — as well
as in its future era; beyond a century it has its future era alone.
`hc_geologic_intervals` takes
`rank` as `0` for the eons through `4` for the ages.

## Time zones

`hc_fixed_from_unix_in_zone(unix, zone, out_fixed)` and
`hc_unix_from_fixed_in_zone(fixed, zone, out_unix)` need the `tz` feature
and convert by a zone's wall clock rather than UTC: the day an instant falls
on, and the instant a day begins — its local midnight, the first instant
after a gap that swallows it, or the earlier of two midnights when the
clocks go back across it. `zone` is a NUL-terminated IANA name. The library
carries the seventeen zones of `hc-tz`'s built-in table with their current
rules only, listed in the WebAssembly module's README; for any other zone,
or a zone's history, `hc_zone_load(name, tzif, tzif_len)` takes the zone's
TZif file once and the conversions answer for that name from then on, a
loaded zone outranking a built-in one. Bytes that are not TZif are
`HC_ERROR_MALFORMED`.

`hc_zones(locale, buffer, capacity, written)` writes where each of the 312
zones of the IANA database's `zone1970.tab` is, in the seven columns of
the WebAssembly module's README: the zone, the latitude and longitude of
its principal location in decimal degrees, the table's whole arcseconds
written to six places (multiply by 3600 and round for the arcseconds),
its countries `;`-separated,
the table's comment, its CLDR 48 exemplar city and the tag that named the
city. `hc_zone_location(zone, locale, buffer, capacity, written)` writes
the same line for one name: a zone; a link `zone.tab` gives a place of its
own, such as `Europe/Oslo`; or another link of `backward`, such as
`Asia/Calcutta`, which answers with the line of `Asia/Kolkata`. A name
that places nothing, such as `UTC`, is `HC_ERROR_UNKNOWN`. The city is in
the `locale` in a build with `calendars` too, and otherwise, and for a
null `locale` or a locale with no city for the zone, in English with `en`.

## The sky

`hc_sky_at(unix_seconds, buffer, capacity, written)`,
`hc_solar_terms_between(from_unix, to_unix, ...)` and
`hc_moon_phases_between(from_unix, to_unix, ...)` need the `sky` feature
and write the lines the WebAssembly module's README tabulates: for an
instant, the Sun's apparent longitude and distance, the Moon's longitude,
latitude and distance, its elongation and illuminated fraction, the new
moons either side, ΔT with its regime and the series each came from; for
a half-open span, one line per solar term or per principal moon phase —
the defining angle, the instant as whole POSIX seconds, and the term's
traditional Chinese and Japanese names or the phase's `new`,
`first-quarter`, `full` or `last-quarter`. Every instant is Universal
Time. An instant outside the proleptic Gregorian years −1000 through 3000,
the era over which `hc-astro` states its series valid, is
`HC_ERROR_OUT_OF_RANGE`, as is a span longer than 400 years; a span whose
`to` is at or before its `from` is an empty answer.
`hc_decan_at(unix_seconds, buffer, capacity, written)` writes the decan
the Sun is in at an instant, for the same years: the tropical sign's
number and English name, the decan within it, its ruler by al-Bīrūnī's
table as an identifier and an English name, and the degrees into it.

## The Earth's rotation and the Sun's hours

`hc_earth_rotation_angle`, `hc_gmst_iau2006`, `hc_gmst_iau1982` and
`hc_ut2_minus_ut1`, each `(ut1_unix_seconds, out)`, need the `sky` feature
and write a `double`: the angle in degrees or UT2 − UT1 in seconds, at a
UT1 reading counted as POSIX seconds are, from 1970-01-01 00:00 UT1. The
two sidereal times are two conventions and two entry points.
`hc_solar_time(clock, unix_seconds, latitude, longitude, elevation, buffer,
capacity, written)` and `hc_solar_event(event, fixed, latitude, longitude,
elevation, buffer, capacity, written)` write the WebAssembly module's
lines: a local clock's reading — `local-mean`, `local-apparent`,
`temporal`, `italian` — or a named time of day — `asr-shafii`,
`asr-hanafi`, `jewish-dusk-vilna-gaon`, `jewish-sabbath-ends-cohn`,
`italian-zero-hour` — and, where the solar event it needs does not happen,
cells naming what is missing instead of a number.
`hc_horizons(buffer, capacity, written)` lists the named horizons a
rising or a setting is measured against — `geometric-dip`, `usno` and
`calendrical-calculations` — with their English names, descriptions and
sources, and `hc_sunrise(horizon, fixed, latitude, longitude, elevation,
buffer, capacity, written)` and `hc_sunset(...)` write the module's line
of the crossing against the one named: the instant, the cells of a
missing sunrise or sunset, and the altitude of the Sun's centre at the
crossing. `hc_hjd_tt(tt_julian_date, right_ascension, declination,
buffer, capacity, written)` and `hc_hjd_utc(utc_julian_date,
right_ascension, declination, strict, buffer, capacity, written)` write
the module's lines of the Heliocentric Julian Date of a target's J2000
direction in TT and in UTC, two scales and two entry points: the HJD and
the light-time correction in seconds, and for UTC the TT − UTC the
leap-second table gave; under `strict` a date outside the table is
`HC_ERROR_NO_DATA`. All of them answer for the sky layer's years −1000 to 3000.

## The orbit

`hc_orbit_at(years_before_1950, buffer, capacity, written)` and
`hc_orbit_series(from_years_before_1950, to_years_before_1950, step_years,
...)` need the `orbital` feature and write the lines the WebAssembly
module's README tabulates, from `hc-orbital`'s evaluation of Berger's 1978
series: for an epoch in years before 1950, negative for the future, the
eccentricity, the obliquity in degrees, the longitude of perihelion from
the moving equinox in degrees and the climatic precession *e* sin ϖ, each
followed by its spread, then the daily mean insolation at 65° N at the
June solstice in W/m², the solar constant it was computed with (1360,
`SOLAR_CONSTANT_BERGER_LOUTRE_1991`) and the source; for a series, one line
per sample at `from`, `from + step` and so on up to `to`, with the epoch
as a first column before those eleven. An epoch beyond a million years
either side of 1950 is `HC_ERROR_OUT_OF_RANGE`, never a number; so is a
step that is not finite and positive, or a series of more than 10 000
samples. A `to` before `from` is an empty answer.

## Time on other bodies

`hc_mars_time(unix_seconds, east_longitude_degrees, buffer, capacity,
written)`, `hc_missions(buffer, capacity, written)`,
`hc_mission_sol(mission, unix_seconds, out_sol)`, `hc_bodies(buffer,
capacity, written)` and `hc_body_time(body, unix_seconds,
east_longitude_degrees, buffer, capacity, written)` need the `planetary`
feature and write the lines the WebAssembly module's README tabulates,
from `hc-planetary`: Mars time — the Mars Sol Date, Coordinated Mars
Time, local mean and true solar time, the equation of time, `Ls`, the
Clancy Mars year and the Darian date at Airy-0, from NASA GISS's Mars24
restatement of Allison and McEwen — the surface missions and the rules of
their sol counts, and the solar day and local mean solar time of every
body in the table. The instant is POSIX seconds as a `double`, read as
UTC through the leap-second table with the last offset held; the longitude
is planetocentric, east-positive, and wraps. An instant more than 100
Julian years from J2000.0, where the series is an extrapolation, is
`HC_ERROR_OUT_OF_RANGE`. `mission` and `body` are NUL-terminated
identifiers or names, in any ASCII case. No mission convention is
invented: Zhurong's operators published no sol numbering, so its row
leaves the landing sol, the clock and its meridian empty and its sol is
`HC_ERROR_NO_DATA`. The Moon's row carries `hc-planetary`'s statement that
no Coordinated Lunar Time was yet defined, and its clock is a mean solar
clock under a declared zero, not that scale.

`hc_circad_date(calendar, unix_seconds, buffer, capacity, written)` writes
the date at an instant in Gangale's calendars for Titan (`darian-titan`)
and the Galilean moons (`gregorian-io`, `gregorian-europa`,
`gregorian-ganymede`, `gregorian-callisto`), counted in circads, and in
the Martiana calendar (`martiana`), counted in Darian sols: the line the
WebAssembly module's README tabulates, over the same span.

## Relativity

`hc_proper_time(speed_metres_per_second, coordinate_seconds, buffer,
capacity, written)`, `hc_gravitational_dilation(body, radius_metres,
buffer, capacity, written)` and `hc_gravitating_bodies(buffer, capacity,
written)` need the `relativity` feature and write the WebAssembly
module's lines, from `hc-relativity`'s Schwarzschild formulas: β, γ, the
proper time and the rate of a clock at a constant speed; the GM, the
Schwarzschild radius and the static dilation factor of a clock held still
at a radius; each rate's offset in microseconds per day, computed without
cancellation; and the `hc-relativity` constants each line was computed
with, by name (`SPEED_OF_LIGHT`, `SPEED_OF_LIGHT_SQUARED` and the body's
`GM_*`). A speed at or beyond light, or a radius at or inside the
Schwarzschild radius, is `HC_ERROR_OUT_OF_RANGE`; the WebAssembly module's
README names each constant's source.

## Leap seconds, and the `strict` flag

`hc_tai_from_unix`, `hc_tai_minus_utc` and `hc_utc_from_tai` take a `strict`
flag. Non-zero refuses to answer before 1961, when UTC did not exist, and past
the announced validity of the IERS leap-second table, with
`HC_ERROR_NO_DATA`; zero holds the last published offset into the future and
treats UTC as TAI before 1961. Holding the last offset into the future is
a forecast, which is why it is the caller's choice and not a default. `hc_day_has_leap_second` takes no
flag and always uses the second policy. `hc-core`'s README states the
table's horizon.

## What is not here

The default surface is the civil calendar and the TAI–UTC bridge: enough to
turn a POSIX timestamp or a Gregorian date into a fixed day and back, name a
weekday, and ask about leap seconds honestly; the other layers are features,
above. Formatting is by template and only as wide as a locale's data: a
locale that has stated none writes a date as its fields in order. Before
1.0 the only stability promise is the status codes, whose meanings do not
change, and the column orders, which only grow at the end.

An entry point here has a twin of the same name in the WebAssembly module
in [`hyper-calendar-wasm`](../hyper-calendar-wasm), except for those its
README's [twin table](../hyper-calendar-wasm/README.md#twins) lists with
the reason: here, `hc_gregorian_from_fixed`.

Neither boundary exposes these parts of the workspace, for the reasons
that README's "What is not here" gives: `hc-planetary`'s circad and
Martiana calendars in the registry, whose day number is a circad or a sol
and not an Earth day, though `hc_circad_date` dates an instant in them;
`hc-humanize`, `hc-fiscal`, `hc-name-days`, `hc-almanac`, `hc-attributes`
and `hc-units`, for which no line format has been designed; and a whole
UUID with its clock sequence and node, where `hc_uuid_timestamp_encode`
writes the time fields only.
