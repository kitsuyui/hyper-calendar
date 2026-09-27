# hyper-calendar-wasm

Calendar conversion, holidays, time scales and astronomy from
`hyper-calendar`, as one dependency-free WebAssembly module with a small
JavaScript binding. [Loading from JavaScript](#loading-from-javascript)
shows a page calling it; the sections before it say how memory, text and
errors cross the boundary.

## Why a raw ABI and not `wasm-bindgen`

The workspace has no external dependencies ([ADR
0005](../../docs/adr/0005-no-external-dependencies.md)), and that is worth
more here than anywhere else. A calendar library is a leaf dependency of a web
application; whatever it drags in, the bundle carries. So the exports are
plain `extern "C"` functions over integers and linear memory, which every
WebAssembly host can call with no glue at all. The cost is that the JavaScript
side does the string marshalling. That is written once, in the binding under
[`js/`](js/) described below, which the caller can read.

## Memory and text

The module owns its allocator. `hc_alloc` hands out a block, the caller writes
into it or reads out of it, and `hc_free` takes it back. Every block must be
freed with the same length it was allocated with.

Text is UTF-8 and is *not* NUL-terminated. Functions that produce text return
the byte length written, because a length is cheaper and safer than a scan;
functions that consume text take a pointer and a length.

A name that says what is asked for — a calendar, a table, a criterion, a
body, a horizon, a notation, the sky of `hc_hindu_lunar_date` — comes
before the day or instant it is asked of. A name that only qualifies the
answer — a locale, a zone, a meridian, an output format, the ayanamsa of
`hc_panchanga_at` and `hc_panchanga_of_day` — comes after it.

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
in seconds returns `HC_ERR_OUT_OF_RANGE` for an answer at or below the
floor or beyond `i64`, instead of wrapping or clamping; day-number exports
are bounded only by their calendar.

The module has no sentinel for an overflow. Where the C library in
[`hyper-calendar-ffi`](../hyper-calendar-ffi) reports `HC_ERROR_OVERFLOW`
for an answer an integer cannot hold, the same call here returns
`HC_ERR_OUT_OF_RANGE`.

### Ranges

What each export answers for; outside it, the export returns the sentinel
named. The JavaScript binding throws that sentinel as an `HcError`, so a day
out of range is `out-of-range`, never an unrecognised number.

| Answers with | Exports | Answers for |
| --- | --- | --- |
| a fixed day | `hc_gregorian_to_fixed` | `year` −9 999 999 through 9 999 999, which are the fixed days −3 652 424 999 through 3 652 424 634; any other date is `HC_ERR_INVALID_DATE` |
| a fixed day | `hc_parse_iso_date` | the dates of the years −9 999 999 through 9 999 999; any other text is `HC_ERR_INVALID_DATE` |
| a Gregorian year, month, day, day of the year, or 1 or 0 | `hc_gregorian_year`, `hc_gregorian_month`, `hc_gregorian_day`, `hc_day_of_year`, `hc_is_leap_year` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_format_iso_date` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERR_OUT_OF_RANGE` |
| a weekday, 1 through 7 | `hc_weekday` | every `fixed` |
| a fixed day | `hc_fixed_from_unix` | every `unix_seconds`; the day is between −106 751 990 448 138 and 106 751 991 886 463 |
| a fixed day | `hc_fixed_from_unix_in_zone` | every `unix_seconds`; the day is at most one from the day `hc_fixed_from_unix` gives |
| seconds | `hc_unix_from_fixed` | `fixed` −104 165 947 503 through 106 751 991 886 463: an earlier day's midnight would be at or below `HC_ERR_FLOOR` seconds, and a later one's would overflow an `i64`; either is `HC_ERR_OUT_OF_RANGE` |
| seconds | `hc_unix_from_fixed_in_zone` | the `fixed` days whose start by the zone's clock is above `HC_ERR_FLOOR` and fits an `i64`: by UTC, `hc_unix_from_fixed`'s days, and a zone's offset moves each end by at most a day (Tokyo's last day is 106 751 991 886 464); any other is `HC_ERR_OUT_OF_RANGE` |
| seconds | `hc_tai_minus_utc` | every `unix_seconds`; under `strict`, 1961 through the end of the announced leap-second table, and `HC_ERR_NO_DATA` outside it |
| a byte length | `hc_tai_from_unix` | every `unix_seconds` up to 9 223 372 036 854 775 770, `i64::MAX − 37`: TAI runs ahead of UTC, so a later one has no TAI second an `i64` holds and is `HC_ERR_OUT_OF_RANGE`; under `strict`, 1961 through the end of the announced table, else `HC_ERR_NO_DATA` |
| a byte length | `hc_utc_from_tai` | every `tai_seconds`; under `strict`, as for `hc_tai_from_unix` |
| 1 or 0 | `hc_day_has_leap_second` | `unix_seconds` −9 223 372 036 854 720 000 through 9 223 372 036 854 719 999, the whole days of the `i64` range; the part-days at its two ends begin or end where no `i64` reaches, and are `HC_ERR_OUT_OF_RANGE` |
| 1 or 0 | `hc_holiday_is_day_off` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERR_OUT_OF_RANGE` |
| a weekday, 1 through 7 | `hc_first_day_of_week` | every locale tag |
| a full week, 0 through 4 294 967 295 | `hc_gnss_resolve_week` | every broadcast week that fits the field and every `reference_tai_seconds`, one before week zero counting as week zero; a broadcast week that does not fit, or an answer past week 4 294 967 295, is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_tai64_encode` | `tai_seconds` −4 611 686 018 427 387 904 through 4 611 686 018 427 387 903, the seconds of the labels below 2⁶³, and attoseconds below 10¹⁸; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_tai64_decode` | every label below 2⁶³, the seconds −4 611 686 018 427 387 904 through 4 611 686 018 427 387 903; a reserved label is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_gnss_week` | `tai_seconds` from the field's week zero to the end of week 4 294 967 295 (for GPS, 315 964 819 through 2 597 596 536 585 618), and attoseconds below 10¹⁸; an earlier instant is `HC_ERR_NO_DATA` and a later one `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_gnss_to_tai` | every week and every time of week below 604 800 s: at most 2 597 597 356 694 432 TAI seconds, the last second of BeiDou week 4 294 967 295 |
| a byte length | `hc_glonass_date` | `tai_seconds` from 1996-01-01 00:00 to the end of 2099-12-31 by GLONASS time, 820 443 629 through 4 102 434 036 when the last published offset is held; outside is `HC_ERR_OUT_OF_RANGE`, and under `strict` an instant past the leap-second table `HC_ERR_NO_DATA` |
| a byte length | `hc_fixed_from_ole_automation` | the values −657 434 through just below 2 958 466, the fixed days 36 160 (1 January 100) through 3 652 059 (31 December 9999); any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_ole_automation_from_fixed` | `fixed` 36 160 (1 January 100) through 3 652 059 (31 December 9999) and a time of day from 0 to below 86 400 s; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_excel_1900_day` | `serial` 1 through 2 958 465, the fixed days 693 596 through 3 652 059, serial 60 writing no day; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_tai64_posix_plus_10_encode` | `unix_seconds` −4 611 686 018 427 387 914 through 4 611 686 018 427 387 893, the seconds whose label 2⁶² + 10 + `unix_seconds` is below 2⁶³, and attoseconds below 10¹⁸; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_tai64_posix_plus_10_decode` | every label below 2⁶³, the POSIX seconds −4 611 686 018 427 387 914 through 4 611 686 018 427 387 893; a reserved label is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_uuid_timestamp` | no `i64` input: every version 1 or version 6 UUID, whose timestamps are the POSIX seconds −12 219 292 800 (1582-10-15) through 103 072 857 660 (5236-03-31); another version is `HC_ERR_NO_DATA` |
| a byte length | `hc_ntp_resolve` | `reference_unix` −9 223 372 034 707 292 160 through 9 223 372 032 498 303 360, within which every timestamp's date and POSIX second fit an `i64`; nearer the ends of the `i64` range some timestamps are `HC_ERR_OUT_OF_RANGE`, and the zero timestamp is `HC_ERR_NO_DATA` everywhere |
| a byte length | `hc_uuid_timestamp_encode` | `unix_seconds` −12 219 292 800 (1582-10-15) through 103 072 857 660 (5236-03-31), up to the field's last interval, which ends at 21:21:00.6846976 UTC that day, and attoseconds below 10¹⁸; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_ntp_encode` | `unix_seconds` −9 223 372 036 854 775 808 through 9 223 372 034 645 787 007, `i64::MAX` − 2 208 988 800, the seconds whose count from 1900 fits an `i64`, and attoseconds below 10¹⁸; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_fat_decode` | no `i64` input: every pair of words whose fields name a day and a time, the fixed days 722 815 (1980-01-01) through 769 565 (2107-12-31); a word above 65 535 is `HC_ERR_OUT_OF_RANGE`, and a pair whose fields name no day or no time `HC_ERR_INVALID_DATE` |
| a byte length | `hc_fat_encode` | `fixed` 722 815 (1980-01-01) through 769 565 (2107-12-31) and a time of day below 86 400 s; any other is `HC_ERR_OUT_OF_RANGE` |
| a beat, 0 through 999 | `hc_swatch_beat` | every `unix_seconds`, and attoseconds below 10¹⁸; more attoseconds are `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_epoch_from_tt` | every `tt_seconds`, and attoseconds below 10¹⁸; more are `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_tt_bipm` | `tai_seconds` from 0 h UTC on the first sample's date through the last's, which the caller's series sets, and attoseconds below 10¹⁸; an instant outside the series, or an empty series, is `HC_ERR_NO_DATA`, and more attoseconds are `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_tt_from_epoch` | no `i64` input: every finite year whose instant is within an `i64` of seconds of 1970 TT, about 2.9 × 10¹¹ years either side; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_describe_day` | every `fixed`; a calendar that refuses the day says so in its own line |
| a byte length | `hc_calendar_units` | every `from_fixed` and `to_fixed` whose range is at most 100 000 units; a span a calendar refuses says so in its own line and counts as one, a `to_fixed` at or before `from_fixed` writes nothing, and a range of more units is `HC_ERR_OUT_OF_RANGE` (see [line caps](#line-caps)) |
| a byte length | `hc_calendars` | every `today` |
| a byte length | `hc_naming_period_on` | every `fixed`; a calendar the registry does not carry is `HC_ERR_UNKNOWN` |
| a byte length | `hc_asian_day` | `fixed` 1 360 (23 September AD 4) through 3 652 398, the last day of the Asian year 9999; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_holidays_in_year` | every `year`; a year the table has no entries for writes nothing |
| a byte length | `hc_holidays_on`, `hc_common_worship_on` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_holy_year_on` | `fixed` 720 981 (24 December 1974) through 739 886 (27 September 2026), from the opening of the first jubilee the table carries to the day its sources were checked; any other is `HC_ERR_NO_DATA` |
| a byte length | `hc_lectionary` | `fixed` 577 780 through 1 497 096, the liturgical years 1583 to 4099; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_term_in_effect`, `hc_pentad_in_effect`, `hc_solar_event`, `hc_panchanga_of_day`, `hc_sunrise`, `hc_sunset`, `hc_crescent_visible` | `fixed` −365 607 through 1 095 727, the years −1000 to 3000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_hindu_lunar_date` | `fixed` in the Śaka years 1622 through 2221 on the true sky, from Chaitra śukla 1 in March 1700 to the eve of the one in March 2300, whose days move with the place and the ayanamsa (620 627 through 839 773 at the Central Station with Lahiri's); on `surya-siddhanta`, −1 132 604 through 2 519 974, Kali Yuga 1 to 10 000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_surya_siddhanta_sunrise` | `fixed` −1 132 604 through 2 519 974, Kali Yuga 1 to 10 000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_surya_siddhanta_at` | `unix_seconds` −159 992 668 800 through 155 590 156 799, the days of Kali Yuga 1 to 10 000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_sky_at`, `hc_decan_at`, `hc_solar_time`, `hc_panchanga_at` | `unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_solar_terms_between`, `hc_moon_phases_between` | `from_unix` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; a `to_unix` at or before it writes nothing, and a later one must be at most 32 535 216 000 and at most 400 years after it; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_chinese_marriage_augury` | `chinese_year` 4282 through 4786, whose New Year and the next both fall in the Chinese calendar's range (1645 through 2150); any other is `HC_ERR_OUT_OF_RANGE` |
| an Olympiad, from 1 | `hc_ioc_olympiad` | `gregorian_year` from 1896, whose Olympiads run to 2 305 843 009 213 693 478 for the last `i64` year; an earlier year is `HC_ERR_OUT_OF_RANGE` |
| a place in the cycle, 1 through 7 | `hc_hebrew_sabbatical_cycle_year` | `hebrew_year` 1 through 9999; any other is `HC_ERR_OUT_OF_RANGE` |
| a fixed day | `hc_hebrew_yahrzeit`, `hc_hebrew_birthday` | `death_fixed` and `birth_fixed` −1 373 427 through 2 278 650 and `hebrew_year` 1 through 9999, the Hebrew years 1 through 9999, and the anniversary lies among those days; any other is `HC_ERR_OUT_OF_RANGE` |
| an age, from 1 | `hc_chinese_reckoned_age` | `birth_fixed` and `on_fixed` 600 460 through 785 271, the days of the Chinese calendar's range, 1645 through 2150, a birth before or on the day asked; a day before the birth is `HC_ERR_NO_DATA`, and a day outside the range `HC_ERR_OUT_OF_RANGE` |
| a fixed day | `hc_astronomical_easter` | `year` 1583 through 2150, Easter falling between the fixed days 577 913 and 785 015; any other year is `HC_ERR_OUT_OF_RANGE` |
| a fixed day | `hc_astronomical_paschal_full_moon` | `year` 1583 through 2150, the years of `hc_astronomical_easter`; any other year is `HC_ERR_OUT_OF_RANGE` |
| a fixed day | `hc_cold_food_day` | `year` −999 through 3000 under every reckoning, the years whose winter solstice before and whose April are both in the era of `hc_term_in_effect`; any other is `HC_ERR_OUT_OF_RANGE`, and a reckoning it does not name `HC_ERR_UNKNOWN` |
| 0 | `hc_zone_load` | any name and bytes; bytes that are not TZif are `HC_ERR_MALFORMED` |
| a byte length | `hc_mars_time`, `hc_body_time` | no `i64` input: the instants within 100 Julian years of J2000.0 (1900-01-01T12:00 to 2100-01-01T12:00 TT); any other, or an instant or longitude not finite, is `HC_ERR_OUT_OF_RANGE`, and for `hc_body_time` a body `hc_bodies` does not list `HC_ERR_UNKNOWN` and the Sun `HC_ERR_NO_DATA` |
| a byte length | `hc_zones`, `hc_zone_location` | no `i64` input: every locale tag, and for `hc_zone_location` every name `zone1970.tab`, `zone.tab` or `backward` places; a name they do not, such as `UTC`, is `HC_ERR_UNKNOWN` |
| a byte length | `hc_circad_date` | no `i64` input: the instants within 100 Julian years of J2000.0 (1900-01-01T12:00 to 2100-01-01T12:00 TT), as for `hc_mars_time`; any other, or one not finite, is `HC_ERR_OUT_OF_RANGE`, and a calendar not listed `HC_ERR_UNKNOWN` |
| a mission sol, from 0 or 1 | `hc_mission_sol` | the instants from the midnight that began the mission's landing sol through 100 Julian years after J2000.0 (2100-01-01T12:00 TT); an earlier instant, or one not finite, is `HC_ERR_OUT_OF_RANGE`, a mission whose operators published no sol numbering `HC_ERR_NO_DATA`, and a mission the table does not carry `HC_ERR_UNKNOWN` |
| a byte length | `hc_version`, `hc_calendar_list`, `hc_locales`, `hc_gregorian_adoption`, `hc_holiday_codes`, `hc_holiday_tables`, `hc_place_years_ago`, `hc_cosmic_events`, `hc_geologic_intervals`, `hc_orbit_at`, `hc_orbit_series`, `hc_earth_rotation_angle`, `hc_gmst_iau2006`, `hc_gmst_iau1982`, `hc_ut2_minus_ut1`, `hc_hjd_tt`, `hc_hjd_utc`, `hc_horizons`, `hc_missions`, `hc_bodies`, `hc_proper_time`, `hc_gravitational_dilation`, `hc_gravitating_bodies` | no `i64` input: text, or `f64` values whose range each export's documentation states; a length is never negative, so it never nears the floor |

[`crates/hyper-calendar/tests/abi.rs`](../hyper-calendar/tests/abi.rs)
walks every `i64` export in the source and fails when one has no row
here, or two, or when its row does not name each of its `i64` inputs.

### Line caps

An export that writes one line per item of a range the caller chooses
would, unbounded, write as much as it is asked for: a trillion days as
days is tens of terabytes, and the allocation traps the instance rather
than returning a sentinel. Each such export has a cap, and refuses a
range past it with `HC_ERR_OUT_OF_RANGE` before it builds the text, so
that the work a call can cost is bounded whatever its arguments. A caller
that wants more asks in pieces.

| One line per | Export | Cap | About |
| --- | --- | --- | --- |
| span | `hc_calendar_units` | 100 000 lines, `hyper_calendar::lines::MAX_CALENDAR_UNITS` | 270 years as days, 8 000 years as months; some 4 MB |
| solar term | `hc_solar_terms_between` | 400 years of span, `MAX_SPAN_SECONDS` | 9 600 lines |
| principal phase | `hc_moon_phases_between` | 400 years of span, `MAX_SPAN_SECONDS` | 19 800 lines |
| sample | `hc_orbit_series` | 10 000 lines, `MAX_SERIES_SAMPLES` | 5 MB |

Every other export that writes lines writes one per entry of a fixed
table — the calendars, the locales, the holiday tables, the missions, the
bodies — or of one year or one day, and has no range to cap:
`hc_holidays_in_year` writes a year's entries of one table, some tens of
lines, whatever the year; `hc_holidays_on` one day's entries across the
tables; `hc_describe_day` one line per calendar.

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
hc.calendarList("ja").find((row) => row.id === "japanese");
// { id: "japanese", name: "和暦", englishName: "Japanese (imperial eras)",
//   localeUsed: "ja", crate: "hc-calendars-regional" }
hc.holidaysOn(rd);                                   // [{ table: "JP", name: ..., kind: "public", ... }, ...]
hc.fixedFromUnixInZone(Math.floor(Date.now() / 1000), "Asia/Tokyo");

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
| `gregorianYear(fixed)`, `gregorianMonth(fixed)`, `gregorianDay(fixed)`, `weekday(fixed)`, `dayOfYear(fixed)` | `hc_gregorian_year`, `hc_gregorian_month`, `hc_gregorian_day`, `hc_weekday`, `hc_day_of_year` | a number |
| `isLeapYear(fixed)`, `dayHasLeapSecond(unixSeconds)` | `hc_is_leap_year`, `hc_day_has_leap_second` | a boolean |
| `fixedFromUnix(unixSeconds)`, `unixFromFixed(fixed)`, `taiMinusUtc(unixSeconds, strict)` | `hc_fixed_from_unix`, `hc_unix_from_fixed`, `hc_tai_minus_utc` | a number |
| `formatIsoDate(fixed)`, `parseIsoDate(text)` | `hc_format_iso_date`, `hc_parse_iso_date` | a string; a fixed day number |
| `taiFromUnix(unixSeconds, strict)`, `utcFromTai(taiSeconds, strict)` | `hc_tai_from_unix`, `hc_utc_from_tai` | a `TaiInstant`; a `UtcLabel` |
| `tai64Encode(taiSeconds, attoseconds, format)`, `tai64Decode(hex)` | `hc_tai64_encode`, `hc_tai64_decode` | a string; a `Tai64Label` |
| `gnssWeek(numbering, taiSeconds, attoseconds)`, `gnssToTai(numbering, week, towSeconds, towAttoseconds)`, `gnssResolveWeek(numbering, broadcast, rule, referenceTaiSeconds)` | `hc_gnss_week`, `hc_gnss_to_tai`, `hc_gnss_resolve_week` | a `GnssWeek`; a `TaiInstant`; a number |
| `glonassDate(taiSeconds, attoseconds, strict)` | `hc_glonass_date` | a `GlonassDate` |
| `fixedFromOleAutomation(value)`, `oleAutomationFromFixed(fixed, secondsOfDay)`, `excel1900Day(serial)` | `hc_fixed_from_ole_automation`, `hc_ole_automation_from_fixed`, `hc_excel_1900_day` | an `OleAutomationDay`; a number; an `Excel1900Day` |
| `tai64PosixPlus10Encode(unixSeconds, attoseconds, format)`, `tai64PosixPlus10Decode(hex)` | `hc_tai64_posix_plus_10_encode`, `hc_tai64_posix_plus_10_decode` | a string; a `PosixTai64Label` |
| `uuidTimestamp(uuid)`, `ntpResolve(seconds, fraction, referenceUnix)` | `hc_uuid_timestamp`, `hc_ntp_resolve` | a `UuidTimestamp`; an `NtpDate` |
| `uuidTimestampEncode(unixSeconds, attoseconds)`, `ntpEncode(unixSeconds, attoseconds)` | `hc_uuid_timestamp_encode`, `hc_ntp_encode` | `UuidTimeFields`; an `NtpEncoding` |
| `fatDecode(date, time)`, `fatEncode(fixed, secondsOfDay)` | `hc_fat_decode`, `hc_fat_encode` | a `FatReading`; `FatWords` |
| `swatchBeat(unixSeconds, attoseconds)` | `hc_swatch_beat` | a number, 0 through 999 |
| `epochFromTt(notation, ttSeconds, attoseconds)`, `ttFromEpoch(notation, year)` | `hc_epoch_from_tt`, `hc_tt_from_epoch` | an `Epoch`; an `EpochInstant` |
| `ttBipm(series, taiSeconds, attoseconds, strict)` | `hc_tt_bipm` | a `TtBipmReading` |
| `describeDay(fixed, locale)` | `hc_describe_day` | `DescribedDay[]`, one per calendar |
| `calendarUnits(id, unit, from, to, locale)` | `hc_calendar_units` | `CalendarUnit[]`, one per span |
| `calendars(today, locale)` | `hc_calendars` | `CalendarEntry[]`, one per calendar |
| `calendarList(locale)` | `hc_calendar_list` | `CalendarListEntry[]`, one per calendar |
| `locales()` | `hc_locales` | `LocaleEntry[]`, one per locale |
| `firstDayOfWeek(locale)` | `hc_first_day_of_week` | a number, Monday = 1 through Sunday = 7 |
| `gregorianAdoption(region)` | `hc_gregorian_adoption` | `GregorianAdoption[]`, one per step |
| `namingPeriodOn(calendar, fixed, locale)` | `hc_naming_period_on` | a `NamingPeriodOn` |
| `panchangaAt(unixSeconds, ayanamsa)`, `panchangaOfDay(fixed, latitude, longitude, elevation, ayanamsa)` | `hc_panchanga_at`, `hc_panchanga_of_day` | `PanchangaLimb[]`, the yoga's and the karaṇa's |
| `hinduLunarDate(sky, fixed, latitude, longitude, elevation)`, `suryaSiddhantaAt(unixSeconds)`, `suryaSiddhantaSunrise(fixed, latitude, longitude)` | `hc_hindu_lunar_date`, `hc_surya_siddhanta_at`, `hc_surya_siddhanta_sunrise` | a `HinduLunarDate`; a `SuryaSiddhantaSky`; a number |
| `crescentVisible(criterion, fixed, latitude, longitude, elevation)` | `hc_crescent_visible` | a `CrescentVisibility` |
| `iocOlympiad(gregorianYear)`, `hebrewYahrzeit(deathFixed, hebrewYear)`, `hebrewBirthday(birthFixed, hebrewYear)`, `chineseReckonedAge(birthFixed, onFixed)` | `hc_ioc_olympiad`, `hc_hebrew_yahrzeit`, `hc_hebrew_birthday`, `hc_chinese_reckoned_age` | a number |
| `chineseMarriageAugury(chineseYear)` | `hc_chinese_marriage_augury` | a `MarriageAugury` |
| `hebrewSabbaticalCycleYear(hebrewYear)` | `hc_hebrew_sabbatical_cycle_year` | a number, 1 through 7 |
| `asianDay(fixed)` | `hc_asian_day` | an `AsianDay` |
| `holidayIsDayOff(code, region, fixed)` | `hc_holiday_is_day_off` | a boolean |
| `holidaysInYear(code, region, year)` | `hc_holidays_in_year` | `HolidayInYear[]` |
| `holidayCodes()` | `hc_holiday_codes` | `string[]` |
| `holidaysOn(fixed)` | `hc_holidays_on` | `HolidayOn[]` |
| `holidayTables(locale)` | `hc_holiday_tables` | `HolidayTable[]` |
| `lectionary(fixed)`, `astronomicalEaster(year)`, `astronomicalPaschalFullMoon(year)` | `hc_lectionary`, `hc_astronomical_easter`, `hc_astronomical_paschal_full_moon` | a `Lectionary`; a fixed day number |
| `holyYearOn(fixed)`, `commonWorshipOn(fixed)` | `hc_holy_year_on`, `hc_common_worship_on` | a `HolyYear` or `null`; `CommonWorshipCelebration[]` |
| `termInEffect(fixed, meridian)`, `pentadInEffect(fixed, meridian)` | `hc_term_in_effect`, `hc_pentad_in_effect` | a `TermInEffect` |
| `coldFoodDay(convention, year)` | `hc_cold_food_day` | a fixed day number |
| `placeYearsAgo(yearsAgo, stdDevYears, locale)`, `cosmicEvents(locale)`, `geologicIntervals(rank, locale)` | `hc_place_years_ago`, `hc_cosmic_events`, `hc_geologic_intervals` | `DeepTimeRow[]` |
| `fixedFromUnixInZone(unixSeconds, zone)`, `unixFromFixedInZone(fixed, zone)` | `hc_fixed_from_unix_in_zone`, `hc_unix_from_fixed_in_zone` | a number |
| `loadZone(name, tzif)` | `hc_zone_load` | nothing |
| `zones(locale)`, `zoneLocation(zone, locale)` | `hc_zones`, `hc_zone_location` | `ZoneLocation[]`; a `ZoneLocation` |
| `skyAt(unixSeconds)` | `hc_sky_at` | a `Sky` |
| `solarTermsBetween(fromUnix, toUnix)`, `moonPhasesBetween(fromUnix, toUnix)` | `hc_solar_terms_between`, `hc_moon_phases_between` | `SkyEvent[]` |
| `decanAt(unixSeconds)` | `hc_decan_at` | a `Decan` |
| `earthRotationAngle(ut1UnixSeconds)`, `gmstIau2006(ut1UnixSeconds)`, `gmstIau1982(ut1UnixSeconds)`, `ut2MinusUt1(ut1UnixSeconds)` | `hc_earth_rotation_angle`, `hc_gmst_iau2006`, `hc_gmst_iau1982`, `hc_ut2_minus_ut1` | a number |
| `solarTime(clock, unixSeconds, latitude, longitude, elevation)`, `solarEvent(event, fixed, latitude, longitude, elevation)` | `hc_solar_time`, `hc_solar_event` | a `SolarTime`; a `SolarEvent` |
| `horizons()`, `sunrise(horizon, fixed, latitude, longitude, elevation)`, `sunset(horizon, fixed, latitude, longitude, elevation)` | `hc_horizons`, `hc_sunrise`, `hc_sunset` | `Horizon[]`; a `SolarCrossing` |
| `hjdTt(ttJulianDate, rightAscension, declination)`, `hjdUtc(utcJulianDate, rightAscension, declination, strict)` | `hc_hjd_tt`, `hc_hjd_utc` | a `HeliocentricJulianDate`; a `HeliocentricJulianDateUtc` |
| `orbitAt(yearsBefore1950)`, `orbitSeries(fromYearsBefore1950, toYearsBefore1950, stepYears)` | `hc_orbit_at`, `hc_orbit_series` | an `Orbit`; `OrbitSample[]` |
| `marsTime(unixSeconds, eastLongitude)`, `missions()`, `missionSol(mission, unixSeconds)` | `hc_mars_time`, `hc_missions`, `hc_mission_sol` | a `MarsTime`; `Mission[]`; a number |
| `bodies()`, `bodyTime(body, unixSeconds, eastLongitude)` | `hc_bodies`, `hc_body_time` | `Body[]`; a `BodyTime` |
| `circadDate(calendar, unixSeconds)` | `hc_circad_date` | a `CircadDate` |
| `properTime(speedMetresPerSecond, coordinateSeconds)`, `gravitationalDilation(body, radiusMetres)`, `gravitatingBodies()` | `hc_proper_time`, `hc_gravitational_dilation`, `hc_gravitating_bodies` | a `ProperTime`; a `GravitationalDilation`; `GravitatingBody[]` |

Each method does what a page would otherwise write by hand:

- **Text** crosses as UTF-8 in `hc_alloc` blocks that are freed with the
  length they were allocated with, whether or not the call succeeds.
- **Lines** are decoded into objects by the column tables below, with an
  empty cell as `null`, a `0`/`1` cell as a boolean, and the extra fields
  of a calendar as an object. The column order is the README's, and the
  tests assert it, so a column moved in the source fails the build.
- **`i64`** crosses as `BigInt`; every result leaves as a number, which
  every day number, year and timestamp fits, and one that does not is an
  `unsafe-integer` error rather than a rounded value. An `i64` or `u64`
  argument may be a number or a `BigInt`. The one exception is a TAI
  instant and its parts, whose seconds and attoseconds leave as `BigInt`:
  attoseconds run to 10¹⁸, past what a number holds exactly.
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
and it is 3.14 MiB (3,292,635 bytes) for the `full` layer of 2026-09-27,
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
before it loads the holiday tables. CI holds each to that:
[`scripts/layer-tests.sh`](../../scripts/layer-tests.sh) builds every layer
[`scripts/layers.sh`](../../scripts/layers.sh) lists for `wasm32` alone,
and runs this crate's and the C library's tests with that feature alone,
one job a layer.

| Feature | Exports | Brings in | Bytes | Size |
| --- | --- | --- | ---: | ---: |
| `civil` *(default)* | Gregorian dates, ISO 8601 text, POSIX time, TAI − UTC and leap seconds | `hc-calendar`, `hc-calendars-solar`, `hc-format` | 35,652 | 35 KiB |
| `timestamps` | `hc_tai_from_unix`, `hc_utc_from_tai`, `hc_tai64_encode`, `hc_tai64_decode`, `hc_tai64_posix_plus_10_encode`, `hc_tai64_posix_plus_10_decode`, `hc_gnss_week`, `hc_gnss_to_tai`, `hc_gnss_resolve_week`, `hc_glonass_date`, `hc_fixed_from_ole_automation`, `hc_ole_automation_from_fixed`, `hc_excel_1900_day`, `hc_uuid_timestamp`, `hc_ntp_resolve`, `hc_uuid_timestamp_encode`, `hc_ntp_encode`, `hc_fat_decode`, `hc_fat_encode`, `hc_swatch_beat`, `hc_epoch_from_tt`, `hc_tt_from_epoch`, `hc_tt_bipm`: POSIX time to and from TAI, TAI64 labels in both conventions, GNSS weeks, GLONASS dates, OLE Automation dates, Excel 1900 serials, UUID timestamps, NTP eras, FAT date and time words, Swatch Internet Time, Julian and Besselian epochs, and TT(BIPM) from a caller's series | nothing beyond `civil`'s crates: `hc-core`'s `tai64`, `gnss`, `uuid`, `ntp`, `internet_time`, `epoch_notation` and `tt_bipm`, `hc-calendars-solar`'s `spreadsheet`, `hc-format`'s `fat` | 113,322 | 111 KiB |
| `calendars` | `hc_describe_day`, `hc_calendar_units`, `hc_calendars`, `hc_calendar_list`, `hc_locales`, `hc_first_day_of_week`, `hc_gregorian_adoption`, `hc_naming_period_on`: every registered calendar described for one day, walked as eras, years, months and days, and listed, in a locale; the locales and the day each one's week begins on; when each country adopted the Gregorian calendar; and the month and weekday names a government decreed for a period; `hc_panchanga_at`, `hc_panchanga_of_day`, `hc_hindu_lunar_date`, `hc_surya_siddhanta_at`, `hc_surya_siddhanta_sunrise`, `hc_crescent_visible`, `hc_ioc_olympiad`, `hc_hebrew_yahrzeit`, `hc_hebrew_birthday`, `hc_hebrew_sabbatical_cycle_year`, `hc_chinese_reckoned_age`, `hc_chinese_marriage_augury`, `hc_asian_day` | every `hc-calendars-*` crate, `hc-astro`, `hc-i18n`, `hc-format`; and every locale's exemplar cities, which only a build with `tz` too carries | 872,905 | 852 KiB |
| `holiday` | `hc_holiday_is_day_off`, `hc_holidays_in_year`, `hc_holiday_codes`, `hc_holidays_on`, `hc_holiday_tables`, `hc_lectionary`, `hc_astronomical_easter`, `hc_astronomical_paschal_full_moon`, `hc_holy_year_on`, `hc_common_worship_on` | `hc-holiday` and everything it dates by | 1,186,664 | 1.13 MiB |
| `seasons` | `hc_term_in_effect`, `hc_pentad_in_effect`, `hc_cold_food_day` | `hc-seasons`, `hc-astro` | 90,029 | 88 KiB |
| `deep-time` | `hc_place_years_ago`, `hc_cosmic_events`, `hc_geologic_intervals` | `hc-deep-time`, `hc-uncertainty` | 152,906 | 149 KiB |
| `tz` | `hc_fixed_from_unix_in_zone`, `hc_unix_from_fixed_in_zone`, `hc_zone_load`, `hc_zones`, `hc_zone_location`: the day by a zone's wall clock, and where each zone is, with its exemplar city in English, or in the locale when the build has `calendars` too | `hc-tz`, and `hc-i18n`'s English exemplar cities | 92,831 | 91 KiB |
| `sky` | `hc_sky_at`, `hc_solar_terms_between`, `hc_moon_phases_between`, `hc_decan_at`, `hc_earth_rotation_angle`, `hc_gmst_iau2006`, `hc_gmst_iau1982`, `hc_ut2_minus_ut1`, `hc_solar_time`, `hc_solar_event`, `hc_horizons`, `hc_sunrise`, `hc_sunset`, `hc_hjd_tt`, `hc_hjd_utc` | `hc-astro`, `hc-seasons` | 115,479 | 113 KiB |
| `orbital` | `hc_orbit_at`, `hc_orbit_series` | `hc-orbital`, `hc-uncertainty` | 64,097 | 63 KiB |
| `planetary` | `hc_mars_time`, `hc_missions`, `hc_mission_sol`, `hc_bodies`, `hc_body_time`, `hc_circad_date`: Mars time, the Darian date, the surface missions' sols, the solar day and local time of every body in `hc-planetary`'s table, and the dates of the Titan, Galilean and Martiana calendars | `hc-planetary`, `hc-astro` | 96,097 | 94 KiB |
| `relativity` | `hc_proper_time`, `hc_gravitational_dilation`, `hc_gravitating_bodies` | `hc-relativity`, `hc-uncertainty` | 52,505 | 51 KiB |
| `full` | all of the above | everything | 2,362,385 | 2.25 MiB |

The sizes are of the `release-compact` profile for
`wasm32-unknown-unknown`, as [`scripts/wasm-layers.sh`](../../scripts/wasm-layers.sh)
printed them on 2026-09-27 with rustc 1.98.1:

```sh
scripts/wasm-layers.sh
# cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --profile release-compact --no-default-features --features <layer>
```

The script leaves each layer at `target/wasm-layers/hyper_calendar_wasm.<feature>.wasm`
and prints the table; CI runs it on every pull request and uploads the
twelve files, the embedded module and `tzdata/` as one workflow artifact.
CI then runs [`scripts/wasm-size-check.sh`](../../scripts/wasm-size-check.sh),
which fails when any layer is more than 5% larger or smaller than the table
above: a layer that grows by accident is caught, and a change that moves a
layer on purpose comes with a refreshed table.
`hc_alloc`, `hc_free` and `hc_version` are in every build.

The profile's `opt-level = "z"` is a measured choice, not a default. On
2026-09-27, with the same rustc, the `full` layer built at `z` was
1,927,503 bytes and answered `hc_holidays_on` for
2026-01-01 in 125 ms under Node 22; at `s`, 1,942,023 bytes and 121 ms; at
`3`, 2,093,074 bytes and 119 ms. The 5 % of time `z` costs against `3`
buys 8 % of the size, and a page loads the module far more often than it
asks the costliest question, so `z` stays.

## What is exported

The two tables below are rendered from `src/lib.rs` by
[`crates/hyper-calendar/tests/abi.rs`](../hyper-calendar/tests/abi.rs), which
fails when they drift, and [`tests/readme.rs`](tests/readme.rs) holds every
row's feature to the source's `#[cfg]`. An export without a row here does
not pass CI.

<!-- generated by crates/hyper-calendar/tests/abi.rs: start -->

### Exports

107 functions. Types are the WebAssembly ones: `i64` crosses into JavaScript as a `BigInt`, everything else as a `number`, and a pointer is a byte offset into `memory`. The feature column is the Cargo feature the module has to be built with for the export to exist.

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
| `hc_tai64_encode(tai_seconds: i64, attoseconds: u64, format: *const u8, format_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A TAI instant as a TAI64, TAI64N or TAI64NA label in lower-case hexadecimal, as one UTF-8 line, returning the byte length written. |
| `hc_tai64_decode(hex: *const u8, hex_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A TAI64, TAI64N or TAI64NA label in hexadecimal read back, as one UTF-8 line, returning the byte length written. |
| `hc_gnss_week(numbering: *const u8, numbering_len: usize, tai_seconds: i64, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The GNSS week and time of week of a TAI instant, as one UTF-8 line, returning the byte length written. |
| `hc_gnss_to_tai(numbering: *const u8, numbering_len: usize, week: u32, tow_seconds: u32, tow_attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The TAI instant of a full GNSS week and a time of week, as one UTF-8 line, returning the byte length written. |
| `hc_gnss_resolve_week(numbering: *const u8, numbering_len: usize, broadcast: u32, rule: *const u8, rule_len: usize, reference_tai_seconds: i64) -> i64` | `timestamps` | The full GNSS week a broadcast week names, by a rollover rule and a reference instant, or an error sentinel. |
| `hc_glonass_date(tai_seconds: i64, attoseconds: u64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | GLONASS's four-year interval N4 and day N_T at a TAI instant, as one UTF-8 line, returning the byte length written. |
| `hc_fixed_from_ole_automation(value: f64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The fixed day and the time of day of an OLE Automation date, as one UTF-8 line, returning the byte length written. |
| `hc_ole_automation_from_fixed(fixed: i64, seconds_of_day: f64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The OLE Automation date of a fixed day and a time of day, as one UTF-8 line, returning the byte length written. |
| `hc_excel_1900_day(serial: i64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | What an Excel 1900 serial names, as one UTF-8 line, returning the byte length written. |
| `hc_tai_from_unix(unix_seconds: i64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A POSIX timestamp as a TAI reading, as one UTF-8 line, returning the byte length written. |
| `hc_utc_from_tai(tai_seconds: i64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A whole TAI second as a UTC label, as one UTF-8 line, returning the byte length written. |
| `hc_tai64_posix_plus_10_encode(unix_seconds: i64, attoseconds: u64, format: *const u8, format_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A POSIX instant as a TAI64 or TAI64N label in the `tai64-posix-plus-10` convention, in lower-case hexadecimal, as one UTF-8 line, returning the byte length written. |
| `hc_tai64_posix_plus_10_decode(hex: *const u8, hex_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A TAI64 or TAI64N label in the `tai64-posix-plus-10` convention read back, as one UTF-8 line, returning the byte length written. |
| `hc_uuid_timestamp(uuid: *const u8, uuid_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The timestamp of a version 1 or version 6 UUID, as one UTF-8 line, returning the byte length written. |
| `hc_ntp_resolve(seconds: u32, fraction: u32, reference_unix: i64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A 64-bit NTP timestamp placed in its era by a reference time, as one UTF-8 line, returning the byte length written. |
| `hc_uuid_timestamp_encode(unix_seconds: i64, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The 60-bit UUID timestamp of a POSIX instant, and the time fields a version 1 and a version 6 UUID write it in, as one UTF-8 line, returning the byte length written. |
| `hc_ntp_encode(unix_seconds: i64, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The NTP date and timestamp of a POSIX instant, as one UTF-8 line, returning the byte length written. |
| `hc_fat_decode(date: u32, time: u32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The local reading a FAT date word and time word name, as one UTF-8 line, returning the byte length written. |
| `hc_fat_encode(fixed: i64, seconds_of_day: u32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The FAT date and time words of a fixed day and a time of day, as one UTF-8 line, returning the byte length written. |
| `hc_swatch_beat(unix_seconds: i64, attoseconds: u64) -> i64` | `timestamps` | The Swatch Internet Time at a POSIX instant, 0 through 999, or an error sentinel. |
| `hc_epoch_from_tt(notation: *const u8, notation_len: usize, tt_seconds: i64, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The Julian or Besselian epoch of a TT instant, as one UTF-8 line, returning the byte length written. |
| `hc_tt_from_epoch(notation: *const u8, notation_len: usize, year: f64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The TT instant of a Julian or Besselian epoch, as one UTF-8 line, returning the byte length written. |
| `hc_tt_bipm(series: *const u8, series_len: usize, tai_seconds: i64, attoseconds: u64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | TT(BIPM) at a TAI instant, read from a realisation the caller supplies, as one UTF-8 line, returning the byte length written. |
| `hc_describe_day(fixed: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | One fixed day in every registered calendar, as UTF-8 lines, returning the byte length written. |
| `hc_calendar_units(id: *const u8, id_len: usize, unit: u32, from_fixed: i64, to_fixed: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The days from `from_fixed` up to but not including `to_fixed` as one calendar's eras, years, months or days, as UTF-8 lines, returning the byte length written. |
| `hc_calendars(today: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Every registered calendar, as UTF-8 lines, returning the byte length written. |
| `hc_calendar_list(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Every registered calendar by name alone, as UTF-8 lines, returning the byte length written. |
| `hc_locales(buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Every locale the module carries, as UTF-8 lines, returning the byte length written. |
| `hc_first_day_of_week(locale: *const u8, locale_len: usize) -> i64` | `calendars` | The ISO weekday of the first day of the week in a locale, Monday = 1 through Sunday = 7, or an error sentinel. |
| `hc_gregorian_adoption(region: *const u8, region_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The steps by which a country adopted the Gregorian calendar, as UTF-8 lines, returning the byte length written. |
| `hc_naming_period_on(calendar: *const u8, calendar_len: usize, fixed: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Which month and weekday names a locale writes for a calendar on a fixed day, where a government renamed them for a period, as one UTF-8 line, returning the byte length written. |
| `hc_panchanga_at(unix_seconds: i64, ayanamsa: *const u8, ayanamsa_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The yoga and the karaṇa in progress at a POSIX timestamp, as two UTF-8 lines, returning the byte length written. |
| `hc_panchanga_of_day(fixed: i64, latitude: f64, longitude: f64, elevation: f64, ayanamsa: *const u8, ayanamsa_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The yoga and the karaṇa a fixed day carries at a place, the ones in progress at its sunrise, as two UTF-8 lines, returning the byte length written. |
| `hc_hindu_lunar_date(sky: *const u8, sky_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The Hindu lunisolar date of a fixed day at a place, as one UTF-8 line, returning the byte length written. |
| `hc_surya_siddhanta_at(unix_seconds: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The *Sūrya Siddhānta*'s Sun and Moon at a POSIX timestamp, as one UTF-8 line, returning the byte length written. |
| `hc_surya_siddhanta_sunrise(fixed: i64, latitude: f64, longitude: f64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The *Sūrya Siddhānta*'s sunrise on a fixed day at a place, as one UTF-8 line, returning the byte length written. |
| `hc_crescent_visible(criterion: *const u8, criterion_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Whether the young crescent should have been visible on the evening that begins a fixed day, from a place, by a named criterion, as one UTF-8 line, returning the byte length written. |
| `hc_ioc_olympiad(gregorian_year: i64) -> i64` | `calendars` | The number of the modern Olympiad a Gregorian year belongs to, or an error sentinel. |
| `hc_hebrew_yahrzeit(death_fixed: i64, hebrew_year: i64) -> i64` | `calendars` | The fixed day of the yahrzeit in a Hebrew year of a death on the Hebrew date a fixed day names, or an error sentinel. |
| `hc_hebrew_birthday(birth_fixed: i64, hebrew_year: i64) -> i64` | `calendars` | The fixed day of the birthday in a Hebrew year of a birth on the Hebrew date a fixed day names, or an error sentinel. |
| `hc_chinese_reckoned_age(birth_fixed: i64, on_fixed: i64) -> i64` | `calendars` | A person's age as the Chinese count reckons it on a fixed day, or an error sentinel. |
| `hc_chinese_marriage_augury(chinese_year: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The marriage augury of a Chinese year, as one UTF-8 line, returning the byte length written. |
| `hc_hebrew_sabbatical_cycle_year(hebrew_year: i64) -> i64` | `calendars` | The place of a Hebrew year in the seven-year sabbatical cycle, 1 through 7, or an error sentinel. |
| `hc_asian_day(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | A fixed day in the calendar of the Roman province of Asia as the calendar writes it, unnumbered days included, as one UTF-8 line, returning the byte length written. |
| `hc_holiday_is_day_off(code: *const u8, code_len: usize, region: *const u8, region_len: usize, fixed: i64) -> i64` | `holiday` | Whether a fixed day is a day off in a holiday table: 1, 0, or an error sentinel. |
| `hc_holidays_in_year(code: *const u8, code_len: usize, region: *const u8, region_len: usize, year: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The holidays of a Gregorian year in a table, as UTF-8 lines, returning the byte length written. |
| `hc_holiday_codes(buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The identifier of every holiday table, one per line, returning the byte length written. |
| `hc_holidays_on(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | Every holiday on one fixed day across every table, as UTF-8 lines, returning the byte length written. |
| `hc_holiday_tables(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | Every holiday table with its kind, names and sources, as UTF-8 lines, returning the byte length written. |
| `hc_lectionary(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The lectionary cycles of a fixed day, as one UTF-8 line, returning the byte length written. |
| `hc_astronomical_easter(year: i64) -> i64` | `holiday` | The fixed day of Easter Sunday of a Gregorian year by the astronomical reckoning at the meridian of Jerusalem, or an error sentinel. |
| `hc_astronomical_paschal_full_moon(year: i64) -> i64` | `holiday` | The fixed day of the paschal full moon of a Gregorian year by the astronomical reckoning at the meridian of Jerusalem, or an error sentinel. |
| `hc_holy_year_on(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The Holy Year of the Catholic Church a fixed day falls in, if any, as one UTF-8 line, returning the byte length written. |
| `hc_common_worship_on(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The rank of every *Common Worship* celebration kept on a fixed day, as UTF-8 lines, returning the byte length written. |
| `hc_term_in_effect(fixed: i64, meridian: *const u8, meridian_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `seasons` | The solar term in effect on a fixed day at a meridian, as one UTF-8 line, returning the byte length written. |
| `hc_pentad_in_effect(fixed: i64, meridian: *const u8, meridian_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `seasons` | The pentad (候) in effect on a fixed day at a meridian, as one UTF-8 line, returning the byte length written. |
| `hc_cold_food_day(convention: *const u8, convention_len: usize, year: i64) -> i64` | `seasons` | The fixed day of 寒食, the Cold Food Day, of a Gregorian year under a named reckoning, or an error sentinel. |
| `hc_place_years_ago(years_ago: f64, std_dev_years: f64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | A moment some years before the present, placed in every chronology at once, as UTF-8 lines, returning the byte length written. |
| `hc_cosmic_events(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | Every cosmic epoch and every dated cosmic event, as UTF-8 lines, returning the byte length written. |
| `hc_geologic_intervals(rank: u32, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | Every interval of one rank of the geologic time scale, as UTF-8 lines, returning the byte length written. |
| `hc_fixed_from_unix_in_zone(unix_seconds: i64, zone: *const u8, zone_len: usize) -> i64` | `tz` | The fixed day a POSIX timestamp falls on by the wall clock of a zone, or an error sentinel. |
| `hc_unix_from_fixed_in_zone(fixed: i64, zone: *const u8, zone_len: usize) -> i64` | `tz` | The POSIX timestamp at which a fixed day begins by the wall clock of a zone, or an error sentinel. |
| `hc_zone_load(name: *const u8, name_len: usize, tzif: *const u8, tzif_len: usize) -> i64` | `tz` | Give the module a zone's TZif data under an IANA name, returning 0. |
| `hc_zones(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `tz` | Every zone of the IANA database's `zone1970.tab` with its principal location, as UTF-8 lines, returning the byte length written. |
| `hc_zone_location(zone: *const u8, zone_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `tz` | Where one zone is, as the UTF-8 line `hc_zones` writes for it, returning the byte length written. |
| `hc_sky_at(unix_seconds: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Sun and the Moon at a POSIX timestamp, as one UTF-8 line, returning the byte length written. |
| `hc_solar_terms_between(from_unix: i64, to_unix: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Every solar term whose instant falls in `[from_unix, to_unix)`, as UTF-8 lines, returning the byte length written. |
| `hc_moon_phases_between(from_unix: i64, to_unix: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Every new moon, first quarter, full moon and last quarter whose instant falls in `[from_unix, to_unix)`, as UTF-8 lines, returning the byte length written. |
| `hc_decan_at(unix_seconds: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The decan the Sun is in at a POSIX timestamp, as one UTF-8 line, returning the byte length written. |
| `hc_horizons(buffer: *mut u8, capacity: usize) -> i64` | `sky` | Every named horizon a rising or a setting can be measured against, as UTF-8 lines, returning the byte length written. |
| `hc_sunrise(horizon: *const u8, horizon_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Sunrise on a fixed day at a place against a named horizon, as one UTF-8 line, returning the byte length written. |
| `hc_sunset(horizon: *const u8, horizon_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Sunset on a fixed day at a place against a named horizon, as one UTF-8 line, returning the byte length written. |
| `hc_earth_rotation_angle(ut1_unix_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Earth Rotation Angle at a UT1 instant, as one UTF-8 line, returning the byte length written. |
| `hc_gmst_iau2006(ut1_unix_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Greenwich mean sidereal time by the IAU 2006 convention at a UT1 instant, as one UTF-8 line, returning the byte length written. |
| `hc_gmst_iau1982(ut1_unix_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Greenwich mean sidereal time by the IAU 1982 convention at a UT1 instant, as one UTF-8 line, returning the byte length written. |
| `hc_ut2_minus_ut1(ut1_unix_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | UT2 − UT1 at a UT1 instant, as one UTF-8 line, returning the byte length written. |
| `hc_solar_time(clock: *const u8, clock_len: usize, unix_seconds: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | A local clock's reading at a POSIX timestamp and a place, as one UTF-8 line, returning the byte length written. |
| `hc_solar_event(event: *const u8, event_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | A named time of day on a fixed day at a place, as one UTF-8 line, returning the byte length written. |
| `hc_hjd_tt(tt_julian_date: f64, right_ascension: f64, declination: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Heliocentric Julian Date in TT, HJD_TT, of a Julian Date of TT for a target, as one UTF-8 line, returning the byte length written. |
| `hc_hjd_utc(utc_julian_date: f64, right_ascension: f64, declination: f64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Heliocentric Julian Date in UTC, HJD_UTC, of a Julian Date of UTC for a target, as one UTF-8 line, returning the byte length written. |
| `hc_orbit_at(years_before_1950: f64, buffer: *mut u8, capacity: usize) -> i64` | `orbital` | Earth's orbital elements and the June insolation at 65° N at an epoch, as one UTF-8 line, returning the byte length written. |
| `hc_orbit_series(from_years_before_1950: f64, to_years_before_1950: f64, step_years: f64, buffer: *mut u8, capacity: usize) -> i64` | `orbital` | The line of `hc_orbit_at` at every epoch from `from_years_before_1950` to `to_years_before_1950` in steps of `step_years`, each with the epoch as a first column, as UTF-8 lines, returning the byte length written. |
| `hc_mars_time(unix_seconds: f64, east_longitude_degrees: f64, buffer: *mut u8, capacity: usize) -> i64` | `planetary` | Mars at a POSIX instant and an east longitude, as one UTF-8 line, returning the byte length written. |
| `hc_missions(buffer: *mut u8, capacity: usize) -> i64` | `planetary` | Every surface mission on Mars and the rules of its sol count, as UTF-8 lines, returning the byte length written. |
| `hc_mission_sol(mission: *const u8, mission_len: usize, unix_seconds: f64) -> i64` | `planetary` | The sol number of a Mars surface mission at a POSIX instant, by the mission's own clock, or an error sentinel. |
| `hc_bodies(buffer: *mut u8, capacity: usize) -> i64` | `planetary` | Every body `hc-planetary` carries, with its solar day, as UTF-8 lines, returning the byte length written. |
| `hc_body_time(body: *const u8, body_len: usize, unix_seconds: f64, east_longitude_degrees: f64, buffer: *mut u8, capacity: usize) -> i64` | `planetary` | Local mean solar time on a body at a POSIX instant and an east longitude, as one UTF-8 line, returning the byte length written. |
| `hc_circad_date(calendar: *const u8, calendar_len: usize, unix_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `planetary` | The date at a POSIX instant in a calendar of another body's days, as one UTF-8 line, returning the byte length written. |
| `hc_proper_time(speed_metres_per_second: f64, coordinate_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `relativity` | A clock moving at a constant speed while some coordinate time passes, as one UTF-8 line, returning the byte length written. |
| `hc_gravitational_dilation(body: *const u8, body_len: usize, radius_metres: f64, buffer: *mut u8, capacity: usize) -> i64` | `relativity` | A clock held still at a radius from a body's centre, against one far from every mass, as one UTF-8 line, returning the byte length written. |
| `hc_gravitating_bodies(buffer: *mut u8, capacity: usize) -> i64` | `relativity` | Every body `hc-relativity` carries a gravitational parameter for, as UTF-8 lines, returning the byte length written. |

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

### Twins

A function exported by both this module and the C library in
[`hyper-calendar-ffi`](../hyper-calendar-ffi) has the same name on both
and means the same thing; the C one writes its answer through
out-parameters where this one returns it. The exports that have no twin of
the same name are these, each with what stands in for it and why.
[`crates/hyper-calendar/tests/abi.rs`](../hyper-calendar/tests/abi.rs)
fails when an export has neither a twin of its name nor a row here.

| What | WebAssembly | C | Why they differ |
| --- | --- | --- | --- |
| The Gregorian date of a fixed day | `hc_gregorian_year`, `hc_gregorian_month`, `hc_gregorian_day` | `hc_gregorian_from_fixed` | A WebAssembly export returns one `i64`, so the three fields are three exports; the C entry point writes all three through out-parameters in one call. |
| A block of the module's memory | `hc_alloc`, `hc_free` | — | A page has to put text into the module's linear memory before a call can read it. A C caller owns its own memory and passes pointers to it, so the C library allocates nothing. |

## Time scales and day counts

The `timestamps` feature carries the labels and counts that name an
instant outside the civil calendar, in a layer of its own so that
`civil`, the one a page paints with first, does not carry their float
formatting and 128-bit arithmetic. They are `hyper_calendar::time_lines`, shared
with the C library. **A TAI instant** crosses as two integers: `tai_seconds`,
whole seconds from 1970-01-01 00:00:00 TAI — the origin of `hc-core`'s
`Instant<Tai>`, not the POSIX epoch, which is 8.000 082 s of TAI later —
floored, and `attoseconds`, the attoseconds into that second, 0 through
999 999 999 999 999 999, never negative. Attoseconds from 10¹⁸ are
`HC_ERR_OUT_OF_RANGE`. `hc_tai_minus_utc` gives the step between the
POSIX count and this one. The binding hands both parts back as `BigInt`s:
attoseconds do not fit a JavaScript number, and a TAI64 label's seconds
need not either.

### TAI64 labels

`hc_tai64_encode(tai_seconds, attoseconds, format_ptr, format_len, buffer,
capacity)` writes one line of one cell, the label in lower-case
hexadecimal: `format` is `tai64` (16 digits, the second containing the
instant), `tai64n` (24, the nanosecond) or `tai64na` (32, the instant
itself), in any case, and anything else is `HC_ERR_UNKNOWN`. A second that
no TAI64 label can hold is `HC_ERR_OUT_OF_RANGE`: the labels run below 2⁶³,
about 146 billion years either side of 1970. The origin is 2⁶², so `4000000000000000` is the
second that began 1970 TAI (D. J. Bernstein, "TAI64, TAI64N, and TAI64NA",
which `hc-core`'s `tai64` cites). `hc_tai64_decode(hex_ptr, hex_len,
buffer, capacity)` reads a label of 16, 24 or 32 hexadecimal digits in
either case back into one line; any other text is `HC_ERR_MALFORMED`, and
a reserved label, from 2⁶³, or a counter above 999 999 999 is
`HC_ERR_OUT_OF_RANGE`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | format | `tai64`, `tai64n` or `tai64na`, by the label's length |
| 2 | tai seconds | the TAI seconds of the instant the label names: the start of its second, nanosecond or attosecond |
| 3 | attoseconds | the attoseconds into that second |

### GNSS weeks

The satellite systems broadcast time as a week and a time of week from a
week-zero epoch, and the week field is short. `numbering` names the field,
as `hc-core`'s `gnss` does: `gps-lnav-week` (ten bits, modulo 1024),
`gps-cnav-week` (thirteen), `galileo-week` (twelve), `beidou-week`
(thirteen) or `navic-week` (ten), in any case; anything else is
`HC_ERR_UNKNOWN`. `hc_gnss_week(numbering_ptr, numbering_len, tai_seconds,
attoseconds, buffer, capacity)` splits a TAI instant into one line; an
instant before week zero is `HC_ERR_NO_DATA`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | week | the full week since the field's week zero |
| 2 | broadcast week | the week as the field broadcasts it: the full week modulo 2 to the field's bits |
| 3 | tow seconds | the whole seconds into the week, 0 through 604 799 |
| 4 | tow attoseconds | the attoseconds into that second |

`hc_gnss_to_tai(numbering_ptr, numbering_len, week, tow_seconds,
tow_attoseconds, buffer, capacity)` joins them again into one line of two
cells, the TAI seconds and the attoseconds; a time of week from
604 800 s is `HC_ERR_OUT_OF_RANGE`. A receiver has only the broadcast week,
which names a family of weeks a rollover apart, and
`hc_gnss_resolve_week(numbering_ptr, numbering_len, broadcast, rule_ptr,
rule_len, reference_tai_seconds)` picks one with a date the caller knows:
`rule` is `not-before`, the first full week at or after the reference's
week, for a date the receiver is known not to be before, such as its
firmware's build; or `nearest`, the one within half a rollover of it, for a
date that may be early or late, such as a file's timestamp. It answers the
full week as a number. GPS week 2048 began at 23:59:42 UTC on 6 April 2019,
when the legacy field read 0 again: with a reference in that week, a
broadcast 0 resolves to 2048 by either rule, and a broadcast 1023 by
`nearest` to 2047.

### GLONASS dates

GLONASS keeps UTC(SU) + 3 h, leap seconds included, and its navigation
message dates a day as a four-year interval and a day within it.
`hc_glonass_date(tai_seconds, attoseconds, strict, buffer, capacity)`
writes them for a TAI instant, reaching UTC through the leap-second table
as `hc_tai_minus_utc` does: `strict` non-zero refuses outside the table
with `HC_ERR_NO_DATA`. An instant before 1996 or from 2100, a common year
that breaks the intervals, is `HC_ERR_OUT_OF_RANGE` rather than counted
wrong.

| # | Column | Holds |
| --- | --- | --- |
| 1 | four-year interval | *N*4, 1 for 1996–1999 |
| 2 | day | *N*T, 1 on 1 January of the interval's leap year |

### OLE Automation dates

`hc_fixed_from_ole_automation(value, buffer, capacity)` reads an OLE
Automation date, the `DATE` of COM and `DateTime.ToOADate`: its integer
part counts days from 30 December 1899, and its fraction is the time of
day, read as a magnitude when the value is negative, so −1.25 is 06:00 on
29 December 1899. A value that is not finite, or outside 1 January 100 to
31 December 9999, is `HC_ERR_OUT_OF_RANGE`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | fixed | the fixed day |
| 2 | seconds of day | the seconds into it, as the double carries them |

`hc_ole_automation_from_fixed(fixed, seconds_of_day, buffer, capacity)`
writes the value back as one line of one cell; a time of day that is not
finite, negative or not below 86 400 s is `HC_ERR_OUT_OF_RANGE`.

### Excel 1900 serials

`hc_excel_1900_day(serial, buffer, capacity)` reads a serial of Excel's
1900 date system, which gives serial 60 to 29 February 1900, a day that
never was, because Lotus 1-2-3 counted 1900 a leap year: serial 60 is
named, with no fixed day, and every serial from 61 is one more than a true
count would be. A serial below 1 or above 2 958 465, 31 December 9999, is
`HC_ERR_OUT_OF_RANGE`. `docs/systems/spreadsheet-dates.md` has the systems
and their sources.

| # | Column | Holds |
| --- | --- | --- |
| 1 | fixed | the fixed day the serial names, or empty for serial 60 |
| 2 | phantom | `1` for serial 60, else `0` |

### The TAI–UTC bridge

`hc_tai_from_unix(unix_seconds, strict, buffer, capacity)` and
`hc_utc_from_tai(tai_seconds, strict, buffer, capacity)` are the C
library's entry points of the same names as lines. They are in this layer,
`timestamps`, though the C library has them in its `civil`: their 128-bit
arithmetic is what this layer exists to keep out of `civil`, which they
would grow by a sixth, and a page that only needs `TAI − UTC` has
`hc_tai_minus_utc` there. `hc_tai_from_unix` writes one line of two cells, the TAI
seconds and the attoseconds of the TAI instant above; from 1961 to 1972
the offset is not a whole number of seconds, so the attoseconds are not
zero.
`hc_utc_from_tai` reads a whole TAI second back into one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | unix seconds | the POSIX second, or for a leap second the one after it |
| 2 | leap second | `1` when the TAI second is an inserted `23:59:60`, which POSIX time cannot express, else `0` |

`strict` non-zero refuses before 1961 and past the announced leap-second
table with `HC_ERR_NO_DATA`, as for `hc_tai_minus_utc`. The last second of
2016 is the anchor: 23:59:59 UTC was TAI + 36 s, so TAI second
1 483 228 836 is 23:59:60 and writes `1483228800` and `1`.

### TAI64 labels on a POSIX clock

TAI64 labels in the wild are not all true TAI. daemontools' `tai64n`, which
stamps the lines `multilog` writes, takes the system clock as TAI seconds
since 1970-01-01 00:00:10 TAI, and on an ordinary clock, which keeps POSIX
time, the label it writes is 2⁶² + 10 + the POSIX seconds, with no
leap-second table at all (D. J. Bernstein, "The tai64n program", which
`hc-core`'s `tai64` cites). `hc-core` names that convention
`tai64-posix-plus-10`, and it has exports of its own rather than a flag on
`hc_tai64_encode`, because nothing in the bytes says which convention
wrote them and the caller knows which clock the log came from.
`hc_tai64_posix_plus_10_encode(unix_seconds, attoseconds, format_ptr,
format_len, buffer, capacity)` writes the label of a POSIX instant as one
line of one cell, `format` being `tai64` (16 digits) or `tai64n` (24);
`tai64na`, which the convention does not write, is `HC_ERR_UNKNOWN`. POSIX 0
is `400000000000000a`, which `hc_tai64_decode` reads as ten seconds after
1970 TAI. `hc_tai64_posix_plus_10_decode(hex_ptr, hex_len, buffer,
capacity)` reads a label of 16 or 24 digits back; any other text is
`HC_ERR_MALFORMED`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | format | `tai64` or `tai64n`, by the label's length |
| 2 | unix seconds | the POSIX seconds of the instant the label names |
| 3 | attoseconds | the attoseconds into that second, a whole number of nanoseconds |

### UUID timestamps

`hc_uuid_timestamp(uuid_ptr, uuid_len, buffer, capacity)` reads the 60-bit
timestamp of a version 1 or version 6 UUID, RFC 9562's count of
100-nanosecond intervals from 15 October 1582, which version 1 writes low
bits first and version 6 high bits first. The UUID is RFC 9562's string
form: 32 hexadecimal digits in either case, bare or hyphenated
8-4-4-4-12, optionally after `urn:uuid:`; any other text is
`HC_ERR_MALFORMED`, and a UUID of another version or variant, which carries
no timestamp, `HC_ERR_NO_DATA`. The count converts as the RFC's own
pseudocode does, POSIX-style with no leap second, and is what the
generator wrote, which RFC 9562 lets it alter. Both of the RFC's test
vectors, `C232AB00-9414-11EC-B3C8-9F6BDECED846` and
`1EC9414C-232A-6B00-B3C8-9F6BDECED846`, carry 138 648 505 420 000 000
intervals, POSIX 1 645 557 742. `docs/systems/binary-timestamps.md` works
the example through.

| # | Column | Holds |
| --- | --- | --- |
| 1 | version | `1` or `6` |
| 2 | timestamp | the 60-bit count of 100-nanosecond intervals from 1582-10-15 00:00 UTC |
| 3 | unix seconds | the POSIX seconds of the start of that interval |
| 4 | attoseconds | the attoseconds into that second, a whole number of 100 ns |

### NTP eras

An NTP packet's 64-bit timestamp is 32 bits of seconds from
1900-01-01 00:00 UTC and 32 of fraction, and the seconds wrap on
7 February 2036; RFC 5905's 128-bit date carries the era the timestamp
leaves out, and "eras cannot be produced by NTP directly". `hc_ntp_resolve(seconds,
fraction, reference_unix, buffer, capacity)` supplies the era from outside,
as §6 says a client within 68 years of the server does: the one that puts
the timestamp within 2³¹ s of `reference_unix`, from 2³¹ s before it,
included, to 2³¹ s after it, excluded. The zero timestamp, which RFC 5905
reserves for unknown or unsynchronised time, is `HC_ERR_NO_DATA`. The
timestamp 63 104 read against a clock in 2030 is 8 February 2036, era 1,
as the RFC's Figure 4 has it; against a clock in 1920 it is 17:31:44 on
1 January 1900.

| # | Column | Holds |
| --- | --- | --- |
| 1 | era | the era number, 0 for 1900 to 2036, negative before 1900 |
| 2 | era offset | the seconds into the era, the timestamp's own |
| 3 | fraction | the fraction of the second in units of 2⁻⁶⁴ s |
| 4 | unix seconds | the POSIX seconds of the date |
| 5 | attoseconds | the attoseconds into that second |

### UUID timestamps from an instant

The other way, from a POSIX instant, `unix_seconds` and the attoseconds
into it: `hc_uuid_timestamp_encode(unix_seconds, attoseconds, buffer,
capacity)` writes the 60-bit UUID timestamp, the 100 ns interval that
contains the instant, and the first three groups of a version 1 and of a
version 6 UUID that carry it, for a generator to follow with its own clock
sequence and node; nothing random is made here. POSIX 1 645 557 742, RFC
9562's example instant, is 138 648 505 420 000 000, `c232ab00-9414-11ec` and
`1ec9414c-232a-6b00`, the groups of the RFC's two vectors in lower case.
Before 1582-10-15 or after the field's last interval on 5236-03-31 is
`HC_ERR_OUT_OF_RANGE`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | timestamp | the 60-bit count of 100-nanosecond intervals from 1582-10-15 00:00 UTC |
| 2 | version 1 fields | `time_low`, `time_mid` and the version with `time_high`, hyphenated, lower case |
| 3 | version 6 fields | `time_high`, `time_mid` and the version with `time_low`, hyphenated, lower case |

### NTP dates from an instant

`hc_ntp_encode(unix_seconds, attoseconds, buffer, capacity)` writes a POSIX
instant's RFC 5905 date, its era, era offset and 2⁻⁶⁴ s fraction, and both
wire layouts: the 128-bit date of Figure 3 and the 64-bit timestamp of the
packet headers, which drops the era and keeps the top 32 bits of the
fraction. POSIX 0 is era 0, offset 2 208 988 800, as the RFC's Figure 4 has
1 January 1970; 8 February 2036 is era 1, offset 63 104, whose timestamp is
the same 64 bits as 63 104 s into 1900's era.

| # | Column | Holds |
| --- | --- | --- |
| 1 | era | the era number, 0 for 1900 to 2036, negative before 1900 |
| 2 | era offset | the seconds into the era |
| 3 | fraction | the fraction of the second in units of 2⁻⁶⁴ s, floored |
| 4 | date | the 128-bit date, era, offset and fraction, as 32 lower-case hexadecimal digits |
| 5 | timestamp | the 64-bit timestamp, offset and the fraction's top 32 bits, as 16 |

### FAT date and time words

The FAT file system packs a local wall-clock reading into two 16-bit
words: the day, month and years from 1980 in the date word, the hour, the
minute and the second halved in the time word, as Microsoft's
`DosDateTimeToFileTime` gives the layouts. `hc_fat_decode(date, time,
buffer, capacity)` writes the reading; the words record no zone, so the day
is a local one and no instant is claimed. A word above 65 535 is
`HC_ERR_OUT_OF_RANGE`, and one whose fields name no day or no time — month
0 or 13, 30 February, hour 24, minute 60, halved second 30 — is
`HC_ERR_INVALID_DATE`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | fixed | the fixed day of the local date |
| 2 | seconds of day | the seconds into it, always even |

`hc_fat_encode(fixed, seconds_of_day, buffer, capacity)` writes the words
back as one line of two cells, the date word and the time word, the
second rounded down to an even one; a day outside 1980 to 2107 or a time
of day from 86 400 s is `HC_ERR_OUT_OF_RANGE`. 26 September 2026 at
23:59:58 is the date word 23 866 and the time word 49 021.

### Swatch Internet Time

`hc_swatch_beat(unix_seconds, attoseconds)` answers the beat as a number,
0 through 999: Swatch divided the day into a thousand beats of 86.4 s,
beginning at midnight of Biel Mean Time, which is UTC+1 all year and not
Biel's mean solar time, so @000 begins at 23:00 UTC and the POSIX epoch is
@041. Swatch defines no unit below the beat, and the export writes none.

### Julian and Besselian epochs

An epoch written `J2000.0` or `B1950.0` is an instant as a year with a
fraction, in one of two notations SOFA's *Time Scale and Calendar Tools*
describes, and `hc-core`'s `epoch_notation` carries both: `J`, Julian years
of exactly 365.25 days of TT from J2000.0, 2000 January 1.5 TT; and `B`,
Besselian years of 365.242 198 781 days from B1900.0, JD 2415020.31352, the
constants ERFA's `eraEpb` gives. A **TT instant** crosses as a TAI one
does, as whole seconds from 1970-01-01 00:00:00 TT and attoseconds; TT is
TAI + 32.184 s. `hc_epoch_from_tt(notation_ptr, notation_len, tt_seconds,
attoseconds, buffer, capacity)` writes the epoch; `notation` is `J` or
`julian-epoch`, or `B` or `besselian-epoch`, in any case, and anything else
is `HC_ERR_UNKNOWN`. SOFA's example, JD 2457073.05631 TT, which is
1 424 352 065.184 TT seconds, is J2015.1349933196 and B2015.1365941021.

| # | Column | Holds |
| --- | --- | --- |
| 1 | notation | `J` or `B` |
| 2 | epoch | the year with its fraction: `2000` for J2000.0 |

`hc_tt_from_epoch(notation_ptr, notation_len, year, buffer, capacity)`
reads an epoch back into one line of three cells: the notation's letter,
the TT seconds and the attoseconds. An empty `notation` reads the year as
SOFA says an epoch without a letter is read, Besselian before 1984.0 and
Julian from it, and the first cell says which it was.

### TT(BIPM)

TT(BIPM) is the BIPM's better realisation of Terrestrial Time, recomputed
each year from the primary and secondary frequency standards and published
as a table of TT(BIPMxx) − TAI − 32.184 s every ten days; today the
difference is about 27.67 µs. The realisations are revised, so the module
carries none of them: the caller supplies the one it trusts, and the
library interpolates it linearly in TAI and never extrapolates past its
ends. `hc_tt_bipm(series_ptr, series_len, tai_seconds, attoseconds,
strict, buffer, capacity)` reads the series as text, one line per sample:
the Modified Julian Date at 0 h UTC and the difference there in
microseconds, separated by a tab, the dates ascending, which are the first
and third columns of the BIPM's `TTBIPM` files. Blank lines are skipped;
text in any other shape is `HC_ERR_MALFORMED`. The samples are placed on
the TAI scale by the leap-second table: `strict` refuses a sample outside
it, and zero holds the table's ends. An instant before the first sample or
after the last, or an empty series, is `HC_ERR_NO_DATA`. It writes one
line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | offset | TT(BIPMxx) − TT(TAI) at the instant, in seconds |
| 2 | minus tai seconds | TT(BIPMxx) − TAI, 32.184 s plus column 1, as whole seconds |
| 3 | minus tai attoseconds | and the attoseconds |
| 4 | reading seconds | the TT(BIPMxx) reading of the instant, whole seconds from 1970-01-01 00:00:00 of that scale |
| 5 | reading attoseconds | and the attoseconds |

`TTBIPM.2025` gives 27.6740 µs for MJD 58 479, 22 December 2018; at 0 h
UTC that day, TAI second 1 545 868 837, the offset is 0.000027674 s and
TT(BIPM25) − TAI is 32.184 027 674 s.

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

## The calendar list

`hc_calendar_list(locale_ptr, locale_len, buffer, capacity)` needs the
`calendars` feature and writes one line per registered calendar, in
registry order, with nothing that depends on a day: the names
`hc_calendars` writes, by the same rule, and the languages of its
sources, without the range, the units and the standing. Those cost a conversion of the day in every calendar, some
of them searches of the sky; a menu of calendars needs none of it, and a
page lists the calendars far more often than it describes a day, so this
converts nothing and answers in under a millisecond. `locale` is as
for `hc_describe_day`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the calendar's identifier |
| 2 | name | what the locale calls the calendar, as column 2 of `hc_calendars` has it, or empty |
| 3 | english name | its English name, as column 3 of `hc_calendars` |
| 4 | locale used | the tag of the locale data the name came from, `ja` for 和暦 asked for in `ja-JP`; empty where the name is |
| 5 | crate | the crate that registers it: `hc-calendars-solar`, `hc-calendars-lunar`, `hc-calendars-equinox`, `hc-calendars-indic` or `hc-calendars-regional` |
| 6 | native locales | the languages its sources are written in, as column 10 of `hc_calendars` has them: BCP 47 tags joined by `;`, primary first, or empty where there are none. A menu that lists a reader's own calendars first matches these against the reader's language, without asking `hc_calendars` for a day |

### What the calls cost

A description of a day is one conversion in each of the 192 calendars,
and a few dozen of them search the sky to convert: the Hindu lunar
calendar and the seven built on it for conjunctions and saṅkrāntis at
sunrise, the observational Hebrew and Hijri calendars for crescents
evening by evening, the Hindu solar, Faṣlī and equinox calendars for
ingresses, the Chinese family for new moons and solar terms. Computing
such a calendar's range takes one or two further searches, and the lines
ask for it several times a calendar; the calendars of one family read
the same sunrises and conjunctions on the same day. So the ranges of the
registered calendars are written down, each beside the test that computes
it again; `hc_describe_day` and `hc_calendars` open one
`hc_core::memo::scope` for the call, inside which a sunrise, a
conjunction, a saṅkrānti, a tithi, a crescent or a Hindu lunar month is
computed once, keyed by its exact arguments, so every value is the one
the calculation gives; ΔT's sample years are compiled in rather than
recomputed at every step of a search; and a locale is rendered once per
name looked up rather than once per locale compared.
`crates/hyper-calendar/tests/line_digests.rs` holds the text of both
calls, for 14 days and 5 locales, to fixed digests, so none of this
changes a byte of what they write.

Measured on 2026-09-27 with rustc 1.98.1 in the `release-compact`
profile, on a machine at a load average of about 5, each figure the
shortest of seven calls; a dash is a figure not taken:

| Asked for | Export | Natively | Under Node 22 |
| --- | --- | ---: | ---: |
| 2026-09-27, `ja` | `hc_describe_day` | 6.0 ms | 13.9 ms |
| 2026-01-01, `ja` | `hc_describe_day` | — | 13.2 ms |
| 1900-06-15, `ja` | `hc_describe_day` | — | 14.3 ms |
| 2026-09-27, `ja` | `hc_calendars` | 8.9 ms | 22.9 ms |
| 2026-09-27, `native` | `hc_calendars` | — | 23.0 ms |
| `ja` | `hc_calendar_list` | 0.31 ms | 0.62 ms |
| `native` | `hc_calendar_list` | — | 0.83 ms |

`hc_calendars` costs more than `hc_describe_day` because it converts a
day inside each calendar's range and asks whether its year is leap, a
whole year of months for a lunisolar calendar, only to learn whether the
calendar has years; `hc_calendar_list` converts no day. The native
figures are
[`examples/calendar_timing.rs`](../hyper-calendar/examples/calendar_timing.rs),
which also prints the ten costliest calendars step by step, alone and
inside the call's memo; the WebAssembly ones are
[`scripts/wasm-calendar-timing.mjs`](../../scripts/wasm-calendar-timing.mjs)
over the `full` layer `scripts/wasm-layers.sh` builds:

```sh
cargo run -p hyper-calendar --example calendar_timing --profile release-compact \
    --features lunar,equinox,indic,regional,i18n,format
scripts/wasm-layers.sh && node scripts/wasm-calendar-timing.mjs
```

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

## Renamed months and weekdays

A locale's month names are its language's, but a state sometimes renamed
them by law, and the new names held only while the law did.
`hc_naming_period_on(calendar_ptr, calendar_len, fixed, locale_ptr,
locale_len, buffer, capacity)` needs the `calendars` feature and says
which names a locale writes for a calendar on a day, from `hc-i18n`'s
`dated` module. `calendar` is a registry identifier, and one the registry
does not carry is `HC_ERR_UNKNOWN`; `locale` is as for `hc_describe_day`,
and `native`, which names no one language, takes no period. The one
period carried is Turkmenistan's, `turkmen-2002`, for the Gregorian
calendar in Turkmen: the People's Council voted it on 8 August 2002, every
source read has it in force from 1 January 2003, and the old names were
back on 1 July 2008. Between the vote and the new year no source read
says whether the names were yet in force, and the line says `undecided`
rather than guess. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | state | `in-force`, `undecided`, or `ordinary` when no period applies and the locale's own names hold |
| 2 | period | the period's identifier, `turkmen-2002`; empty for `ordinary`, as are the columns after it |
| 3 | month | the period's name for the day's month |
| 4 | weekday | the period's name for the day's weekday |
| 5 | weekday meaning | that weekday name's meaning in English, as the source glosses it |
| 6 | earliest | the first day the names can have been in force, as a fixed day |
| 7 | in force by | the first day by which every source read has them in force |
| 8 | ended | the first day the old names were back |
| 9 | source | where the names and the days come from |

21 March 2005, a Monday, is Başgün of Nowruz in `tk` and `tk-TM`, and an
ordinary day in `ru`.

## The pañcāṅga

`hc_panchanga_at(unix_seconds, ayanamsa_ptr, ayanamsa_len, buffer,
capacity)` and `hc_panchanga_of_day(fixed, latitude, longitude, elevation,
ayanamsa_ptr, ayanamsa_len, buffer, capacity)` need the `calendars` feature
and write the yoga and the karaṇa, the two limbs of the pañcāṅga beside the
tithi, the nakṣatra and the weekday, from `hc-calendars-indic`'s
`panchanga`: at an instant read as Universal Time, or on a day at a place,
where a pañcāṅga reads them at sunrise. A day on which the Sun does not
rise at the place is `HC_ERR_NO_DATA`; no other moment is put in the
sunrise's place. The yoga is the sum of the Sun's and the Moon's sidereal
longitudes, so it needs an ayanamsa, and moves with it twice over:
`ayanamsa` is `Lahiri (Chitrapaksha)`, `Raman`, `Krishnamurti` or
`Fagan-Bradley`, or the first word of one, in any case, and anything else,
the empty string included, is `HC_ERR_UNKNOWN`. The karaṇa, half a tithi,
needs none. The instants answer for the sky layer's era, below, and a
place is a latitude and a longitude in degrees, north and east positive,
and an elevation in metres; one off the globe is `HC_ERR_OUT_OF_RANGE`.
Each call writes two lines, the yoga's and then the karaṇa's:

| # | Column | Holds |
| --- | --- | --- |
| 1 | limb | `yoga` or `karana` |
| 2 | number | the yoga, 1 for Viṣkambha through 27 for Vaidhṛti; the karaṇa's half-tithi, 1 for the first half of śukla 1 through 60 |
| 3 | name | its name as Drik Panchang spells it in English: `Vyaghata`, `Balava` |
| 4 | devanagari | its name in Devanagari, as Drik Panchang's Hindi edition prints it: व्याघात, बालव |
| 5 | began | the instant it began, as POSIX seconds, rounded down |
| 6 | ends | the instant it ends |
| 7 | read at | the instant it was read at: the one asked for, or the sunrise |
| 8 | ayanamsa | the ayanamsa the yoga was reckoned with, by its full name; empty for the karaṇa |

For 1 January 2025 at 23°11′ N, 82°30′ E, the yoga at sunrise is
Vyaghata, ending within a minute and a half of the 17:07 IST Drik Panchang
prints, and the karaṇa Balava, ending within two minutes after its 14:55.

## The Hindu lunisolar date

`hc_hindu_lunar_date(sky_ptr, sky_len, fixed, latitude, longitude,
elevation, buffer, capacity)` needs the `calendars` feature and writes the
amānta lunisolar date of a day read at the sunrise of a place the caller
gives. The registered `hindu-lunar` reads the day at the Central Station
and `hindu-lunar-surya-siddhanta` at Ujjain; a place is a parameter and
not a calendar of its own, and this is where it is given.
`docs/systems/hindu-calendars.md` says how often the place moves a date.
`sky` is the sky the day is read on: an ayanamsa as `hc_panchanga_at`
names them, `Lahiri` for the *Rashtriya Panchang*'s, for the true Sun and
Moon in its zodiac; or `surya-siddhanta`, for the *Sūrya Siddhānta*'s Sun,
Moon and sunrise; in any case, and anything else, the empty string
included, is `HC_ERR_UNKNOWN`. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | saka year | the Śaka year, which turns at Chaitra śukla 1 |
| 2 | vikrama year | the Vikrama year, 135 more |
| 3 | month | 1 for Chaitra through 12 for Phālguna, named for the saṅkrānti it holds |
| 4 | leap month | `1` for the intercalary (adhika) month, which precedes the ordinary one; else `0` |
| 5 | tithi | the tithi in progress at the sunrise, 1 through 30: śukla 1 to 15, then kṛṣṇa 1 to 15 |
| 6 | leap day | `1` for the second day to carry a tithi; else `0` |
| 7 | sunrise | the sunrise the day was read at, as POSIX seconds of Universal Time, rounded down |

A month begins at the first sunrise after a conjunction, so a place
must see the Sun rise on every day of the year: one beyond 65° of
latitude is `HC_ERR_OUT_OF_RANGE` on either sky. On the true sky a day
outside the Śaka years 1622 through 2221 is `HC_ERR_OUT_OF_RANGE` too.
The Siddhānta's reckoning is arithmetic and answers for Kali Yuga 1 to
10 000. 30 March
2025 is Chaitra śukla 1 of Śaka 1947 on the true sky at the Central
Station and on the Siddhānta's at Ujjain.

### The Sūrya Siddhānta's sky

`hc_surya_siddhanta_at(unix_seconds, buffer, capacity)` writes the
Siddhānta's Sun and Moon at an instant read as Universal Time, for the
days of Kali Yuga 1 to 10 000:

| # | Column | Holds |
| --- | --- | --- |
| 1 | sun | the Sun's sidereal longitude in the Siddhānta's zodiac, in degrees |
| 2 | moon | the Moon's, likewise |
| 3 | elongation | the Moon's elongation from the Sun, 0 to 360 degrees |
| 4 | tithi | the tithi in progress, 1 through 30 |
| 5 | sign | the sign the Sun is in, 1 for Meṣa through 12 for Mīna |

`hc_surya_siddhanta_sunrise(fixed, latitude, longitude, buffer, capacity)`
writes one line of one cell, the instant of the Siddhānta's sunrise on the
day at the place as POSIX seconds, rounded down: six in the morning at the
place's meridian corrected by the Siddhānta's own equation of time and
ascensional difference, with no refraction and no height, which is why it
takes none. A place beyond 65° of latitude, where the Siddhānta's Sun does
not rise every day, is `HC_ERR_OUT_OF_RANGE`. At Ujjain on 30 March 2025
it is 01:01 UT, and the Siddhānta's Moon is then 7.58° past its Sun, in
Mīna: the first tithi.

## The young crescent

`hc_crescent_visible(criterion_ptr, criterion_len, fixed, latitude,
longitude, elevation, buffer, capacity)` needs the `calendars` feature and
says whether the young crescent should have been visible, in a clear sky,
on the evening that begins a day — the evening of the day before, since
such a day begins at sunset — from a place, by a named criterion of
`hc-calendars-lunar`: `shaukat`, the arc of light and the altitude at a
solar depression of 4.5°, as the observational calendars `islamic-rgsa`
and `hebrew-observational` judge; `yallop`, B. D. Yallop's *q*-test at
Bruin's best time; or `saudi-rule`, the Moon past conjunction at sunset
and setting after the Sun; in any case, and anything else is
`HC_ERR_UNKNOWN`. It is a forecast of an observation, not a record of
one. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | visible | `1` if the crescent passes the criterion; else `0` |
| 2 | evaluated at | the moment the criterion judges the evening at, as POSIX seconds, rounded down: the Sun at 4.5° down for `shaukat`, Bruin's best time for `yallop`, sunset for `saudi-rule`; empty where there is none — the Sun does not set, twilight does not end, or for `yallop` the Moon sets first — and then column 1 is `0` |
| 3 | elongation | the Moon's longitude less the Sun's at that moment, 0 to 360 degrees, as column 6 of `hc_sky_at`: 0 at new moon, so just under 360 on an evening before it; else empty |
| 4 | arc of light | the Moon's true angular separation from the Sun, 0 to 180 degrees; else empty |
| 5 | altitude | the Moon's geocentric altitude, in degrees; else empty |
| 6 | arc of vision | the Moon's altitude less the Sun's, in degrees; else empty |
| 7 | width | the crescent's topocentric width, in arcminutes; else empty |

A place off the globe, or a day outside the years −1000 to 3000, is
`HC_ERR_OUT_OF_RANGE`. The Babylonian criterion of `babylonian` is not
offered: it is judged at Babylon alone.

## The Asian calendar's days

The calendar of the Roman province of Asia, `asian` in the registry, opens
each 31-day month with an unnumbered day, Sebaste, and then counts 1 to
30; a leap Xandikos has two unnumbered days, Sebaste and the intercalary
day, whose order the sources read do not settle.
`hc_describe_day` gives the day's place in the month, counting the
unnumbered days first, so that its day 1 of Kaisar is Sebaste.
`hc_asian_day(fixed, buffer, capacity)` needs the `calendars` feature and
writes the day as the calendar writes it:

| # | Column | Holds |
| --- | --- | --- |
| 1 | year | the Julian year, AD, in which the Asian year began, a label of this library's |
| 2 | month | 1 for Kaisar through 12 for Hyperberetaios |
| 3 | month name | the month's name, `Kaisar` |
| 4 | written | `unnumbered` for a day before day 1, else `numbered` |
| 5 | number | the day's number, 1 to 30, or for an unnumbered day its place among them, 1 or 2 |

23 September AD 4, the fixed day 1 360 and the first the calendar
carries, is Sebaste of Kaisar, and 7 October is its day 14, as the
Metropolis *hemerologion* has it. A day outside 23 September AD 4 to the
end of the Asian year 9999 is `HC_ERR_OUT_OF_RANGE`;
[`docs/systems/asian-calendar.md`](../../docs/systems/asian-calendar.md)
gives the sources.

## Anniversaries and Olympiads

`hc_hebrew_yahrzeit(death_fixed, hebrew_year)` and
`hc_hebrew_birthday(birth_fixed, hebrew_year)` need the `calendars`
feature and answer the fixed day of the anniversary in a Hebrew year of a
Hebrew date. The date crosses as the fixed day whose daylight carries it —
a death or a birth after sunset is the next fixed day, since the Hebrew
day begins at sunset — so that no month numbering has to be agreed. The
rules for the dates a later year may lack, 30 Ḥeshvan, 30 Kislev and the
two Adars, are Reingold and Dershowitz's, which `hc-calendars-lunar` states
are the book's and not a ruling. A day or a year outside the Hebrew years
1 to 9999 is `HC_ERR_OUT_OF_RANGE`. `hc_ioc_olympiad(gregorian_year)`
answers the number of the modern Olympiad a year belongs to, 1 for
1896–1899, by the Olympic Charter's definition, whether or not its Games
were held: 2020 and 2021 are both the XXXII. A year before 1896 is
`HC_ERR_OUT_OF_RANGE`.

`hc_hebrew_sabbatical_cycle_year(hebrew_year)` answers a Hebrew year's
place in the seven-year sabbatical cycle, 1 through 7, the seventh being
the sabbatical year, *shemittah*, counted from Rosh Hashanah as the years
published today count it: 5782 (2021–22) and 5789 (2028–29) are
sabbatical years, and 5786 is the fourth of its cycle. A year outside 1 to
9999 is `HC_ERR_OUT_OF_RANGE`.

`hc_chinese_reckoned_age(birth_fixed, on_fixed)` answers a person's age as
the Chinese count reckons it, from `hc-calendars-lunar`'s `chinese`: one
at birth and one more at each Chinese New Year after, whatever the day of
birth — Reingold and Dershowitz's `chinese-age`, the pre-modern *suì* of
China as Wikipedia's "East Asian age reckoning" gives it, and no claim
about any other country's count. A child born on 15 June 2000 is 12 on
22 January 2012 and 13 from the New Year of the 23rd. A day before the
birth has no age and is `HC_ERR_NO_DATA`; a day outside the Chinese
calendar's range, 1645 to 2150, is `HC_ERR_OUT_OF_RANGE`.

### The marriage augury

`hc_chinese_marriage_augury(chinese_year, buffer, capacity)` writes where
立春, the Beginning of Spring, falls in a Chinese year, which the almanacs
read for marriage: none in the year, once near its end, once near its
start, or both. `chinese_year` is the calendar's own count, 4661 for the
year that began on 10 February 2024, a widow year by the South China
Morning Post of 3 February 2024. The names are the published code's
(`chinese-year-marriage-augury`); Wikipedia's "Lichun" gives "blind year"
for a year with no 立春, which the code calls `widow`. A year outside the
calendar's range is `HC_ERR_OUT_OF_RANGE`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | augury | `widow`, `blind`, `bright` or `double-bright` |
| 2 | lichun at start | `1` when the year's first 立春 comes after its New Year, else `0` |
| 3 | lichun at end | `1` when another 立春 comes before the next New Year, else `0` |

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
once, and only for the months around the day. Measured on 2026-09-27 in
the `release-compact` profile, one 2026 day across all 286 tables takes
about 44 ms natively on 1 January, the costliest, and 30 ms on
25 September, against 0.19 s for every table's whole year; in WebAssembly
under Node 22, 107 ms and 78 ms.

### The tables

`hc_holiday_tables(locale_ptr, locale_len, buffer, capacity)` describes
every table, one line each in the order `hc_holiday_codes` lists them, so
that a menu can show a name rather than a code. Everything in a line is the
table's own data: the kind is the list the table is in, and a table in the
countries' list whose code is an ISO 3166-2 code would be a `subdivision`,
though none is yet — a subdivision's days are rules of its country's
table, asked for by `region`. A country's table is named in the locale by
CLDR 48's territory names, which `hc-i18n` carries for the 195 countries in
every locale it carries that CLDR names them in (its `territories`
feature): 日本 under `ja`, Deutschland under `de` and `de-AT`, with the tag
of the data that answered, `ja` or `de`, in column 5. English is CLDR's
English too, so under `en` column 3 is `Hong Kong SAR China` where the
table's own name in column 4 is `Hong Kong`. A country the locale has no
release-level CLDR name for (Kabyle's Hong Kong, Tibetan's France, every
country in Coptic), and every country under `native`, which names no one
language, is named as CLDR's English names it, with `en` in column 5, so
that `en` stands for one name of a country wherever it appears. Every
exchange, tradition and set of observances, which CLDR does not name and
the library does not translate, is named by the table's own English
name, with `en` in column 5. So column 3 is never empty. An exchange
names its country only where its table includes the country's for its
days off — Tokyo, Hong Kong, Shanghai, London among them — and most list
every closed day themselves and name none.

Column 8 is the short name CLDR 48 gives a country beside the plain one,
its `alt="short"` value at the `approved` and `contributed` levels, read
from the same file that answered column 5: `Hong Kong` for `HK` under
`en`, 香港 under `ja` beside 中華人民共和国香港特別行政区, `UK` for `GB`,
and likewise for Macao (`MO`) and Palestine (`PS`) in most locales, and
for Bosnia (`BA`), Myanmar (`MM`), South Korea (`KR`), Saudi Arabia (`SA`)
and the United States (`US`) in a few. CLDR shortens only these, so the
cell is empty for most countries; it is empty too where the file writes
the short value as the inheritance marker `↑↑↑`, which resolves to the
plain name column 3 already carries, and for every table that is not a
country, whose name is the table's own and not CLDR's. A country named
from CLDR's English by fallback has English's short name.

| # | Column | Holds |
| --- | --- | --- |
| 1 | code | the table's identifier, as `hc_holiday_codes` lists it |
| 2 | kind | `country`, `subdivision`, `exchange`, `tradition` or `observance` |
| 3 | name | the table's name in the locale: a country's CLDR name, else the English name of column 4 |
| 4 | english name | its English name, never empty and never shared by two tables of a kind |
| 5 | locale used | the tag of the data that named column 3: `ja`, `de`, `zh-Hant`, or `en` for an English name |
| 6 | source | the statute, gazette or calendar the table names as its sources |
| 7 | country | for a subdivision or an exchange, the ISO 3166-1 code of the country its table records, else empty; the code of a row whose column 3 names the country in the same locale |
| 8 | short name | CLDR's `alt="short"` name for a country column 3 names from CLDR, from the same locale's data, else empty |

### The liturgical year

`hc_lectionary(fixed, buffer, capacity)` writes the lectionary cycles a
day falls in, from `hc-holiday`'s `lectionary`: the rules, not the
readings. Every cycle turns at the First Sunday of Advent, and a liturgical
year is named by the civil year of its Easter, so the year that began on
30 November 2025 is 2026, Year A of the Sunday cycle and Year II of the
weekdays, and Christ the King, 22 November 2026, is Proper 29. A day
outside the liturgical years 1583 to 4099 is `HC_ERR_OUT_OF_RANGE`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | liturgical year | the civil year of the liturgical year's Easter |
| 2 | sunday cycle | `A`, `B` or `C`, the Roman Lectionary's and the Revised Common Lectionary's |
| 3 | weekday cycle | `I` or `II`, the Roman weekday cycle of Ordinary Time |
| 4 | proper | the RCL's numbered Proper, 3 to 29, for a Sunday after Trinity Sunday; else empty |

`hc_astronomical_easter(year)` answers the fixed day of Easter by the
astronomical reckoning at the meridian of Jerusalem that the World Council
of Churches' Aleppo statement of 1997 proposed: the first Sunday after the
day, by apparent solar time at Jerusalem, of the first full moon at or
after the March equinox. It answers for 1583 to 2150 and is
`HC_ERR_OUT_OF_RANGE` outside them; 2001's full moon fell on Sunday
8 April, and its Easter on 15 April. `hc_astronomical_paschal_full_moon(year)`
answers that full moon's day itself, for the same years: 8 April 2001, and
21 March 2019, the day of the equinox, as the statement's table has them.

### Holy Years

`hc_holy_year_on(fixed, buffer, capacity)` needs the `holiday` feature
and says whether a day falls in a Holy Year of the Catholic Church, from
`hc-holiday`'s `holy_years`: the jubilees whose bulls of indiction were
read, 1975, the Jubilee of the Redemption of 1983–84, 2000, the Jubilee of
Mercy of 2015–16 and 2025, each from the opening of the Holy Door of St Peter's to its closing on the
days its bull names. A day before 24 December 1974 or after 27 September
2026, the day the sources were checked, is `HC_ERR_NO_DATA`, since a
jubilee proclaimed later is not in the table. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | state | `within` a jubilee in Rome, else `outside`, with the columns after it empty |
| 2 | title | the jubilee's name, `Ordinary Jubilee of the Year 2025` |
| 3 | kind | `ordinary` or `extraordinary` |
| 4 | pope | the Pope who proclaimed it |
| 5 | bull | the bull of indiction, by its opening words |
| 6 | given | the day the bull was given, as a fixed day |
| 7 | opens | the jubilee's first day in Rome |
| 8 | closes | its last day in Rome |
| 9 | dioceses open | its first day in the dioceses, where the bull dates it; else empty |
| 10 | dioceses close | its last day there; else empty |

*Spes non confundit* opened the jubilee of 2025 on 24 December 2024 and
closed it on 6 January 2026, and dates it in the dioceses from
29 December 2024 to 28 December 2025.

### Ranks of the Common Worship calendar

`hc_holidays_on` writes the Church of England's *Common Worship* calendar
as the `common-worship` table: each Principal Feast, Principal Holy Day
and Festival on the day it is kept after the transfers its Rules require,
and, as `gap` lines, the Festivals the Rules leave without a day in some
years — St George, St Mark and Philip and James when Easter is 17 or 22
to 25 April. `hc_common_worship_on(fixed, buffer, capacity)`, in the same
feature, adds the rank: one line per celebration kept on the day, and
nothing on a day that keeps none.

| # | Column | Holds |
| --- | --- | --- |
| 1 | title | the title as the Rules print it, which is the name `hc_holidays_on` gives it |
| 2 | rank | `principal-feast`, `principal-holy-day` or `festival` |
| 3 | rank name | the rank's English name |

St George's Day was kept on Monday 28 April 2025, Easter being 20 April,
as a Festival. A day with no Gregorian year is `HC_ERR_OUT_OF_RANGE`.

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

`hc_cold_food_day(convention_ptr, convention_len, year)` answers the fixed
day of 寒食, the Cold Food Day, which is counted from a solar term and has
been counted three ways, each its own name (`docs/policy.md` §5):
`hanshi-solstice-105`, 105 days after the winter solstice at 120°E, the
Chinese reckoning before 1645; `hanshi-eve-of-qingming`, the day before
清明 at 120°E, as kept after the 時憲曆 of 1645; and `hansik`, Korea's 한식,
105 days after 동지 at UTC+9. Names match in any case; anything else is
`HC_ERR_UNKNOWN`. It answers for the years −999 to 3000 and is
`HC_ERR_OUT_OF_RANGE` outside them. 한식 fell on 5 April 2024 and 6 April
2026, as the Korea Astronomy and Space Science Institute's 월력요항 has it.
`docs/systems/solar-term-counts.md` works the count through.

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

### Where each zone is

A page that knows only the reader's zone —
`Intl.DateTimeFormat().resolvedOptions().timeZone`, `Asia/Tokyo` — can ask
the module where that is, for the exports that need a place: sunrise and
sunset, the young crescent, the pañcāṅga and the Hindu date.
`hc_zones(locale_ptr, locale_len, buffer, capacity)` writes a line for each
of the 312 zones of the IANA database's `zone1970.tab`, in its order, and
`hc_zone_location(zone_ptr, zone_len, locale_ptr, locale_len, buffer,
capacity)` the same line for one name. The place is the zone's *principal
location*, the one the database gives it — Tokyo for `Asia/Tokyo`, New
York for `America/New_York` — and not the reader's; it is a default, for a
page to use until the reader names a place.

The tables are `hc-tz`'s copy of `zone1970.tab`, `zone.tab`, `backward`
and `backzone` from release 2026c, unmodified; `docs/systems/zone-locations.md` explains
the lookup with examples. The table writes each coordinate in degrees,
minutes and seconds, `+353916+1394441`; columns 2 and 3 are the table's
whole arcseconds written in decimal degrees to six places, by integer
arithmetic: `arcseconds × 10⁶ / 3600` millionths of a degree, rounded half
away from zero, so Tokyo is `35.654444` and `139.744722` and São Paulo's
latitude `-23.533333`. One arcsecond is 0.000278°, so six places identify
it: a caller who needs the exact arcseconds multiplies by 3600 and
rounds.

`hc_zone_location` answers any name a browser can report:

- a zone of `zone1970.tab`, which answers with its own line;
- a link that `zone.tab` gives a place of its own — `Europe/Oslo`,
  `Europe/Stockholm`, `Asia/Muscat`, 106 of them — which answers with that
  place, Oslo and not the Berlin its link leads to;
- another link of `backward`, which answers with the line of the name it
  leads to, so that column 1 says which: `Asia/Calcutta` answers as
  `Asia/Kolkata`, `US/Eastern` as `America/New_York`. Where the file's
  comment names the link the old name really stands for, that is followed:
  `Iceland` answers as `Atlantic/Reykjavik`, not the `Africa/Abidjan` its
  data line names; and where `backzone` links an old name within its
  country, that link: `America/Coral_Harbour` answers as
  `America/Atikokan`, in Canada, not as `backward`'s `America/Panama`.

A name that places nothing — `UTC`, `Etc/GMT+5`, a name nobody knows — is
`HC_ERR_UNKNOWN`. Names match in any ASCII case.

Column 6 is the zone's exemplar city from CLDR 48, the city by which CLDR
names a zone for a reader. English is always carried: `en.xml`'s value,
else `root.xml`'s, else the city UTS #35 derives from the zone's name, its
last field with underscores as spaces. In a build with the `calendars`
feature too, the city is the locale's where `hc-i18n` carries one — 東京
under `ja`, Wien under `de-AT` — with the tag of the data that answered in
column 7; a locale whose file marks the city as inherited answers with the
root name under its own tag (`Berlin` under `de`). A zone the locale has no
city for, every zone under `native` or a tag whose chain reaches no table,
and every zone in a build without `calendars`, is named in English with
`en`. So column 6 is never empty.

| # | Column | Holds |
| --- | --- | --- |
| 1 | zone | the name of the row that answered: the zone, or the link `zone.tab` places, or for another link the name it leads to |
| 2 | latitude | decimal degrees north of the principal location, negative south, to six places |
| 3 | longitude | decimal degrees east, negative west, to six places |
| 4 | countries | the ISO 3166-1 codes of the countries the zone overlaps, `;`-separated, the location's first |
| 5 | comment | the table's comment, which tells a country's zones apart; empty where the country has one zone |
| 6 | exemplar city | CLDR 48's exemplar city in the locale, else in English |
| 7 | locale used | the tag of the data that named column 6: `ja`, `de`, `zh-Hant`, or `en` |

```js
const zone = Intl.DateTimeFormat().resolvedOptions().timeZone;
const place = hc.zoneLocation(zone, navigator.language); // throws `unknown` for UTC
const today = hc.fixedFromUnix(Math.floor(Date.now() / 1000));
const sunrise = hc.sunrise("usno", today, place.latitude, place.longitude, 0);
```

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

### Decans

`hc_decan_at(unix_seconds, buffer, capacity)` needs the `sky` feature and
writes the decan the Sun is in at an instant, from `hc-seasons`'s
`zodiac::decans`: each tropical sign cut into three faces of 10° of the
Sun's apparent longitude, each ruled by a planet in al-Bīrūnī's table,
which is the Chaldean order from Mars at the first face of Aries. The sign and the decan turn at the solar terms,
and the instant has the range of `hc_sky_at`. Nothing here is an
astrological claim. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | sign | the tropical sign, 1 for Aries through 12 for Pisces |
| 2 | sign name | its English name |
| 3 | decan | which of the sign's three decans, 1 to 3 |
| 4 | ruler | the decan's ruler: `saturn`, `jupiter`, `mars`, `sun`, `venus`, `mercury` or `moon` |
| 5 | ruler name | the ruler's English name |
| 6 | degrees into decan | how far into the decan the Sun is, from 0 up to 10 degrees |

An hour after the September equinox of 2026, which the NAOJ puts at
00:05 UTC on the 23rd, the Sun is 0.04° into the first face of Libra, the
Moon's.

## The Earth's rotation

`hc_earth_rotation_angle`, `hc_gmst_iau2006`, `hc_gmst_iau1982` and
`hc_ut2_minus_ut1`, each `(ut1_unix_seconds, buffer, capacity)`, need the
`sky` feature and write one line of one cell, from `hc-astro`'s `earth`
and `ut_variants`:

| # | Column | Holds |
| --- | --- | --- |
| 1 | value | the Earth Rotation Angle or the sidereal time in degrees, 0 to 360, or UT2 − UT1 in seconds |

The instant is a UT1 reading, counted as POSIX time counts UTC — 86 400
seconds a day from 1970-01-01 00:00 UT1 — as a double, so that the angle,
which turns 15″ a second, is not held to whole seconds. The Earth Rotation
Angle is IERS Conventions 2010's equation 5.14. The mean sidereal time is
two conventions and so two exports: `hc_gmst_iau2006` is the angle plus
equation 5.32's polynomial in TT, taken as UT1 + ΔT; `hc_gmst_iau1982` is
Meeus's (12.4), a polynomial in UT1 alone, about 0.14 ms of time from the
other in 2006. UT2 − UT1 is the seasonal variation the time services
adopted in 1955, as the USNO states it. A value that is not finite, or
outside the sky layer's years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. At
JD 2 454 388.5 UT1, `ut1_unix_seconds` 1 192 406 400, the angle is ERFA's
`eraEra00` test value, 0.402 283 724 002 815 810 2 rad, to 10⁻⁸ degree.

## The Sun's hours

`hc_solar_time(clock_ptr, clock_len, unix_seconds, latitude, longitude,
elevation, buffer, capacity)` needs the `sky` feature and reads a local
clock at an instant, read as Universal Time, at a place, from
`hc-astro`'s `solar_time`. `clock` is `local-mean` (Universal Time moved an
hour for every 15° east), `local-apparent` (the sundial: local mean time
plus the equation of time), `temporal` (the daylight and the night each
cut into twelve unequal hours, 6 at sunrise and 18 at sunset) or `italian`
(hours since the zero hour, half an hour after the Sun's centre is 16′
below the horizon on the evening before), in any case; anything else is
`HC_ERR_UNKNOWN`. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | day | the fixed day of the local date the reading belongs to, or empty when there is no reading |
| 2 | hours | the hours into it on that clock, or empty |
| 3 | missing | the solar event the reading needs and does not have: `sunrise`, `sunset` or `depression`; else empty |
| 4 | missing day | the local day it is missing on, as a fixed day; else empty |
| 5 | depression | for `depression`, the depression of the Sun sought, in arcminutes; else empty |

**A missing solar event is an answer.** Above the polar circles the Sun
can stay up or down all day, and there is no temporal hour and no zero
hour: the line names what is missing rather than give a number, as a
calendar's refusal is a line in `hc_describe_day`. A place is a latitude
and a longitude in degrees, north and east positive, and an elevation in
metres; one off the globe, or an instant outside the years −1000 to 3000,
is `HC_ERR_OUT_OF_RANGE`.

`hc_solar_event(event_ptr, event_len, fixed, latitude, longitude,
elevation, buffer, capacity)` answers a named time of day on a local day.
Each convention is its own name, as `docs/policy.md` §5 has it:
`asr-shafii` and `asr-hanafi`, the Islamic afternoon prayer when a shadow
is its noon length plus once or twice the object's height;
`jewish-dusk-vilna-gaon`, the Sun 4°40′ below the horizon;
`jewish-sabbath-ends-cohn`, 7°5′; and `italian-zero-hour`. The angles are
Reingold and Dershowitz's; `hc-astro` did not read the authorities' own
texts. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | instant | the time as POSIX seconds of Universal Time, rounded down, or empty when it does not happen |
| 2 | missing | as column 3 of `hc_solar_time`, or `no-noon-shadow` where the Sun is not up at noon to cast the ʿaṣr shadow |
| 3 | missing day | as column 4 |
| 4 | depression | as column 5 |

### Horizons

"Sunrise" is the moment the Sun's upper limb meets the visible horizon,
and authorities disagree on where that horizon is: how much the air lifts
the Sun, and whether a height above the sea lowers the horizon. Each
answer is a named convention of `hc-astro`'s `horizon` module, and
`hc_horizons(buffer, capacity)` lists them, one line each:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | `geometric-dip`, the horizon every other export here uses; `usno`; `calendrical-calculations` |
| 2 | english name | its English name |
| 3 | description | what it takes the visible horizon to be: the refraction, the size of the Sun and the Moon, and what a height does |
| 4 | source | where the convention comes from |

`docs/systems/rise-and-set.md` works each one and says how it was
measured.

### Sunrise and sunset

`hc_sunrise(horizon_ptr, horizon_len, fixed, latitude, longitude,
elevation, buffer, capacity)` and `hc_sunset(...)`, with the same
arguments, need the `sky` feature and answer the Sun's upper limb rising
over or setting under a named horizon on a local day, the day that runs
from local mean midnight at the longitude. `horizon` is an identifier
`hc_horizons` lists, in any case, and anything else, the empty string
included, is `HC_ERR_UNKNOWN`: the horizon is always named, never
assumed. The elevation is in metres, and whether it counts is the
horizon's to say; `usno` ignores it. Each writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | instant | the crossing as POSIX seconds of Universal Time, rounded down, or empty when the Sun does not rise or set that day |
| 2 | missing | `sunrise` or `sunset` when it does not happen; else empty |
| 3 | missing day | the local day it is missing on, as a fixed day; else empty |
| 4 | depression | always empty, so that the line has the shape of `hc_solar_event`'s |
| 5 | altitude | the geometric altitude of the Sun's centre at the crossing, in degrees: −50′ at sea level on every horizon, lower for a height where the horizon counts it |

For Jerusalem, 31.78° N, 35.24° E, 740 m, on 1 January 2024, the `usno`
horizon rises and sets within half a minute of the 06:39 and 16:46 (UT+2)
the USNO publishes, and `calendrical-calculations`, lowered 61′ for the
height, rises some five minutes sooner. A place off the globe, or a day
outside the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`.

## The Heliocentric Julian Date

`hc_hjd_tt(tt_julian_date, right_ascension, declination, buffer,
capacity)` and `hc_hjd_utc(utc_julian_date, right_ascension, declination,
strict, buffer, capacity)` need the `sky` feature and correct a Julian
Date to the moment light from a distant object would have reached the
Sun, from `hc-astro`'s `hjd`. A Julian Date can be written in any time
scale, and a Heliocentric Julian Date has to say which, so there are two
exports: HJD_TT, a Julian Date of TT plus the light-time correction with
the Earth's position at that TT instant, and HJD_UTC, a Julian Date of UTC
plus the correction with the Earth's position at the TT instant of the
same event. TT − UTC is 32.184 s plus TAI − UTC from the leap-second
table; `strict` refuses a date outside the table with `HC_ERR_NO_DATA`,
and zero holds the table's ends and takes TAI − UTC as 0 before 1961. The
object's direction is its right ascension, 0 to 360 degrees, and its
declination, −90 to 90 degrees, on the mean equator and equinox of J2000,
and it is taken to be infinitely far away. A date outside the years −1000
to 3000 or not finite, or a direction outside those ranges, is
`HC_ERR_OUT_OF_RANGE`. `hc_hjd_utc` writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | hjd | the Heliocentric Julian Date, in the scale of the date given |
| 2 | correction | the light-time correction added to the date, in seconds, negative when the light reaches the Sun before the Earth |
| 3 | tt minus utc | the TT − UTC the Earth's position was taken at, in seconds |

and `hc_hjd_tt` writes one line of the first two of those cells.

The HJD is good to about 8 s, the Sun's own motion about the solar
system's barycentre; the barycentric date, BJD_TDB, is not here.
For 29 February 1992, 03:15:56.2, and an object at 12h 56m 27.4s,
+42° 10′ 17″, the correction is 350.9 s, as the IDL Astronomy Library's
`helio_jd` gives it.

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

## Mars time

`hc_mars_time`, `hc_missions`, `hc_mission_sol`, `hc_bodies` and
`hc_body_time` need the `planetary` feature and answer from
`hc-planetary`. Every instant is POSIX time in seconds as a double, so a
page passes `Date.now() / 1000` as it is; it is read as UTC and carried to
TAI through the leap-second table with the last published offset held
into the future, and UTC taken as TAI before 1961, as `hc-planetary` reads
a landing. Every longitude is planetocentric and east-positive, in
degrees, and wraps. The layer answers for the instants within 100 Julian
years of J2000.0, 1900-01-01T12:00 to 2100-01-01T12:00 TT, the span over
which Allison and McEwen state their series good to about 0.008° of `Ls`,
three seconds of true solar time; outside it the series extrapolates with
no secular change of Mars's orbit, and an instant there is
`HC_ERR_OUT_OF_RANGE`, never a number. The body table states no span of
its own, and its README calls a century of propagation already more than
its fact-sheet figures bear, so the same span holds for it.

`hc_mars_time(unix_seconds, east_longitude_degrees, buffer, capacity)`
writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | mars sol date | the Mars Sol Date, sols from 1873-12-29, as Mars24's equation C-2 counts them |
| 2 | mtc | Coordinated Mars Time, the mean solar time at Airy-0, as `HH:MM:SS` on the 24-hour Martian clock, truncated so that a sol never reads 24:00:00 |
| 3 | mtc hours | the same in decimal Martian hours |
| 4 | lmst | local mean solar time at the longitude, `HH:MM:SS` |
| 5 | lmst hours | the same in decimal Martian hours |
| 6 | ltst | local true solar time at the longitude, `HH:MM:SS`: what a sundial there reads |
| 7 | ltst hours | the same in decimal Martian hours |
| 8 | equation of time | true minus mean solar time, in Martian minutes; −51 to +40 over a Mars year |
| 9 | ls | the areocentric solar longitude `Ls`, in degrees: 0 at the northern spring equinox |
| 10 | mars year | the Mars year under the Clancy convention, year 1 from the `Ls = 0` of 1955-04-11 |
| 11 | darian year | the Darian year at Airy-0, 183 more than the Mars year |
| 12 | darian month | the Darian month, 1 to 24 |
| 13 | darian sol | the sol of the month, 1 to 28 |
| 14 | darian month name | the month's name, Sagittarius to Vrishika |
| 15 | darian sol of week | the sol's name, Sol Solis to Sol Saturni |
| 16 | source | the series and the constants, by name |

A Martian hour is a twenty-fourth of a sol, 3 699 SI seconds, and its
minutes and seconds are sixtieths of it, as every Mars mission has kept
them. The Darian date is the prime meridian's, turning at mean midnight at
Airy-0; it is Gangale's proposal, not a calendar anyone keeps, and it is
carried as `hc-planetary` carries it.

The constants, as `hc-planetary` names them, and where each is from:

| Constant | Value | Source |
| --- | --- | --- |
| `MSD_EPOCH_JULIAN_DATE_TT`, `MSD_AT_EPOCH` | JD 2 451 549.5 TT, 44 796.0 | NASA GISS, *Mars24 Sunclock — Algorithm and Worked Examples*, eq. C-2 |
| `SOL_IN_DAYS` | 1.027 491 251 7 | the same, eq. C-2 |
| `MSD_MIDNIGHT_ADJUSTMENT` | 0.000 962 6 | the same, as revised in 2015; Allison and McEwen's 2000 value, 0.000 72, is `MSD_MIDNIGHT_ADJUSTMENT_2000` and is not used here |
| the mean anomaly, the fictitious mean Sun, `PERTURBERS`, the equation of centre and of time | eqs. B-1 to C-1 | the same |
| `MARS_SOL_SECONDS` | 88 775.244 s | Mars24, *Technical Notes on Mars Solar Time* |
| `MARS_TROPICAL_YEAR_SOLS` | 668.5921 | the same |
| `MARS_YEAR_1_START_MSD` | 28 892.6593 | a seed, not a citation: Clancy et al. (2000) date Mars Year 1 to 1955-04-11 without a time of day, and this is `hc-planetary`'s own `Ls = 0` solution there, re-solved at every year boundary |
| `DARIAN_EPOCH_MARS_SOL_DATE`, the months, the week and the leap rule | −94 129 | Gangale, "The Darian Calendar for Mars"; the epoch reproduces the published Darian dates of the Viking 1 and Perseverance landings |

`hc-planetary`'s README gives each constant's derivation and
`docs/systems/mars-timekeeping.md` the system. Mars24's two worked
examples reproduce through the export: 2000-01-06T00:00:00Z at the prime
meridian is MSD 44 795.999 76, MTC 23:59:39, `Ls` 277.187 58°, and LTST
23:38:54; 2004-01-03T13:46:31Z at 184.702° W, Spirit's planned site, is
LTST 00:00:00.

```js
const landing = hc.marsTime(1_613_681_028, 77.45);   // Perseverance, 2021-02-18T20:43:48Z
landing.lmst;                                         // "15:53:25"
landing.darian;                                       // { year: 219, month: 1, sol: 13, monthName: "Sagittarius", ... }
landing.marsYear;                                     // 36
```

The calendars of other bodies' days — Titan's, the Galilean moons' and
Martiana — are [`hc_circad_date`](#calendars-of-other-bodies), one export
beside `hc_mars_time` with its own line, and not columns of this one.

### Mission sols

`hc_missions(buffer, capacity)` writes one line per surface mission, in
landing order:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the name in lower case, a hyphen for each space: `viking-1`, `mars-pathfinder` |
| 2 | name | the name the mission is usually called by |
| 3 | landing utc | the landing instant in UTC, spacecraft event time where the distinction is documented |
| 4 | landing unix | the same as a POSIX timestamp |
| 5 | landing sol | the number the mission gave its landing sol, 0 or 1; empty where no convention was published |
| 6 | clock | the clock's midnight: `local-mean-solar-time`, or `local-true-solar-time-at-landing`, a true solar midnight on the landing sol after which the clock ticked at the mean rate; empty where no convention was published |
| 7 | clock longitude | the east longitude the clock was built on, the planned site rather than the achieved one; empty where no convention was published |
| 8 | site longitude | the achieved site's east longitude |
| 9 | published | `1` where the operators published the convention, `0` otherwise |
| 10 | note | what is worth knowing about the clock |
| 11 | source | where the row is from |

`hc_mission_sol(mission, mission_len, unix_seconds)` returns the sol by
that mission's clock, `mission` being an identifier or a name the list
gives, in any ASCII case. The conventions are the ones NASA GISS's
*Mars24 Technical Notes* state under "Lander Mission Times": Viking 1 and
2, Phoenix, Curiosity, InSight and Perseverance number the landing sol 0,
Pathfinder, Spirit and Opportunity 1; Viking and Pathfinder clocks began
at local true solar midnight, the rest at local mean solar midnight, each
on its planned meridian. The landings and sites are Mars24's *Mars Lander
Missions*. None is invented: Zhurong's operators published no clock and
no sol numbering, so its row leaves those cells empty and its sol is
`HC_ERR_NO_DATA`, although `hc-planetary` states a choice of its own for
it. Spirit's and Opportunity's operational "hybrid local solar time" ran
more than 41 and 37 minutes from site LMST; their sol numbers are the
missions', which the offset does not change. An instant before the
landing sol began is `HC_ERR_OUT_OF_RANGE`: the count does not reach back
past it.

```js
hc.missionSol("curiosity", 1_344_230_277);   // 0: landed 2012-08-06T05:17:57Z, on sol 0
hc.missionSol("Mars Pathfinder", 868_035_415);   // 1
hc.missionSol("zhurong", Date.now() / 1000);   // throws HcError "no-data"
```

## Other bodies

`hc_bodies(buffer, capacity)` writes one line per body `hc-planetary`
carries, outward from the Sun with each planet's moons after it:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the name in lower case: `titan` |
| 2 | name | the English name |
| 3 | kind | `star`, `planet`, `dwarf-planet` or `moon` |
| 4 | primary | the identifier of the body it orbits; empty for the Sun |
| 5 | sidereal rotation | the sidereal rotation period in hours, negative for a retrograde rotator |
| 6 | solar day | the solar day in SI seconds; empty for the Sun, which has none |
| 7 | solar day origin | `measured` where the day is itself a published constant (Earth's 86 400 s, the Mars24 sol, the Moon's mean synodic month), `derived` where it follows from `1/P_solar = 1/P_sidereal − 1/P_year` with the year around the Sun; empty for the Sun |
| 8 | year in local days | the year around the Sun in the body's own solar days; empty for the Sun |
| 9 | zero point | `standard` where the clock's zero is an international one (Earth, Mars) and `convention` where it is a zero this library declares |
| 10 | zero point note | what the zero point is |
| 11 | source | where the row's figures are from, and what is contested about them |
| 12 | status | the status of a timekeeping standard still being drawn up: on the Moon's row, `hc-planetary`'s statement that no Coordinated Lunar Time was yet defined as of 2026-09-26; empty on every other row |

The figures are the NASA NSSDC *Planetary Fact Sheets*, with satellite
rotation rates from the IAU WGCCRE (Archinal et al. 2018) and Ceres from
Konopliv et al. (2018), each row's source cell saying which; the Moon's
solar day is `hc-astro`'s `MEAN_SYNODIC_MONTH`. They are quoted to four to
seven figures, which is fine for how long a day on Titan is and not for a
day count carried across a century.

### Local time on a body

`hc_body_time(body, body_len, unix_seconds, east_longitude_degrees,
buffer, capacity)` writes one line of local **mean** solar time:

| # | Column | Holds |
| --- | --- | --- |
| 1 | day | the local day number, counted from the body's zero point: the Rata Die on Earth, the Mars Sol Date on Mars, the Meeus lunation on the Moon, and a day this library declares elsewhere |
| 2 | fraction | the fraction of the local day elapsed, 0 to 1 |
| 3 | time | the reading as `HH:MM:SS` on a 24-hour local face, truncated |
| 4 | hours | the same in decimal local hours |
| 5 | solar day | the solar day in SI seconds |
| 6 | local hour | a twenty-fourth of it, in SI seconds |
| 7 | zero point | `standard` or `convention`, as in `hc_bodies` |
| 8 | zero point note | what the zero point is |

Only the rate is physical wherever the zero point is a `convention`: the
library declares J2000.0 local mean midnight at the prime meridian, which
is reproducible and nobody's standard. A retrograde rotator's longitude
needs no care from the caller — east is east, and the Sun rises in the
west. `body` is an identifier or a name `hc_bodies` gives, in any ASCII
case; another is `HC_ERR_UNKNOWN`, and the Sun, which has no solar day,
`HC_ERR_NO_DATA`. The Moon's clock is a mean solar clock under a declared
zero and is not Coordinated Lunar Time.

```js
hc.bodyTime("titan", Date.now() / 1000).localHourSeconds / 3600;   // 15.97
hc.bodyTime("mars", 947_116_800).time;   // "23:59:39", Coordinated Mars Time at Airy-0
```

### Calendars of other bodies

`hc_circad_date(calendar_ptr, calendar_len, unix_seconds, buffer,
capacity)` writes the date at an instant in one of the calendars
`hc-planetary` keeps for bodies whose day is not Earth's, other than the
Darian date `hc_mars_time` already writes. `calendar` is, in any ASCII
case:

- `darian-titan`, Gangale's Darian calendar for Titan, and
  `gregorian-io`, `gregorian-europa`, `gregorian-ganymede` and
  `gregorian-callisto`, his Gregorian-based calendars for the Galilean
  moons. Their day is a *circad*, a fixed fraction of the moon's solar day
  near an Earth day, in weeks of eight, counted from each calendar's epoch
  by the source's calibration (`docs/systems/circad-calendars.md`).
- `martiana`, Gangale's variant of the Darian calendar with Aitken's week:
  the Darian months and sol count, the seven-sol week never shortened, the
  leap sol in every odd year and an epagomenal sol, outside the week, in
  every tenth.

Another name is `HC_ERR_UNKNOWN`. The instant is read as for
`hc_mars_time`, through the leap-second table with the last offset held,
and only within 100 Julian years of J2000.0: the calendars' calibrations
are of 2002, and the solar days they divide are fact-sheet figures. One
line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | calendar | the calendar's identifier, as above |
| 2 | year | the year, in the calendar's own count |
| 3 | month | the month, from 1 |
| 4 | day | the day of the month, from 1: a circad, or a sol |
| 5 | month name | the month's name, as the source gives it |
| 6 | week name | the name of the day's place in the week; empty for Martiana's epagomenal sol, which belongs to no week |
| 7 | count | the day count the date is numbered by: the circad number from the calendar's epoch, or Martiana's Darian sol number |
| 8 | fraction | the fraction of that circad or sol elapsed, 0 to 1 |
| 9 | leap | `1` for a year with the calendar's intercalary week or sol, else `0` |
| 10 | source | where the calendar comes from |

Gangale's calibration of the Titan calendar is the anchor: the superior
conjunction of 18 December 2002 at 10:42 UTC was 209 Aries 13, Julian
Circad 144 096, a Solis.

## Relativity

`hc_proper_time`, `hc_gravitational_dilation` and `hc_gravitating_bodies`
need the `relativity` feature and answer from `hc-relativity`, whose
metric is Schwarzschild's: non-rotating, uncharged, spherically
symmetric, with no quadrupole, no frame dragging and no Sagnac term. Each
line names the `hc-relativity` constants it was computed with, separated
by `;`, so a figure can be traced to its source. A rate's offset from 1 is
computed without cancellation, by `√(1 − x) − 1 = −x / (1 + √(1 − x))`,
since a rate of 1 − 3·10⁻¹⁰ written as a double keeps only six figures of
its distance from 1.

`hc_proper_time(speed_metres_per_second, coordinate_seconds, buffer,
capacity)` writes one line for a clock moving at a constant speed while
the coordinate time passes in the frame it moves through:

| # | Column | Holds |
| --- | --- | --- |
| 1 | beta | the speed as a fraction of the speed of light |
| 2 | lorentz factor | γ = 1/√(1 − β²) |
| 3 | proper seconds | the time the moving clock records, in seconds |
| 4 | rate | dτ/dt = 1/γ |
| 5 | microseconds per day | the rate's offset from 1 in microseconds per 86 400-second day; negative, the moving clock runs slow |
| 6 | constants | `SPEED_OF_LIGHT` |
| 7 | source | the functions used |

A speed at or beyond the speed of light either way, or a value that is not
finite, is `HC_ERR_OUT_OF_RANGE`.

### A clock at a radius

`hc_gravitational_dilation(body, body_len, radius_metres, buffer,
capacity)` writes one line for a clock held still at a radius from a
body's centre, against one far from every mass:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the body's identifier |
| 2 | gm | its standard gravitational parameter GM, in m³ s⁻² |
| 3 | gm constant | the `hc-relativity` constant that holds it |
| 4 | schwarzschild radius | 2GM/c², in metres |
| 5 | factor | the static dilation factor dτ/dt = √(1 − r_s/r) |
| 6 | microseconds per day | the factor's offset from 1 in microseconds per 86 400-second day; negative, the deeper clock runs slow |
| 7 | constants | the constants used: the body's `GM_*` and `SPEED_OF_LIGHT_SQUARED` |
| 8 | source | where the body's GM is from |

Two radii compare by their offsets: a clock at `hc-relativity`'s
`GPS_ORBIT_RADIUS`, 26 561 750 m, gains 45.65 µs a day on one at
`EARTH_EQUATORIAL_RADIUS`, 6 378 137 m, before its motion is counted; its
circular speed through `hc_proper_time` loses 7.21, and the net 38.4 is
the textbook GPS figure the crate anchors. A radius that is not finite,
not positive, or at or inside the Schwarzschild radius is
`HC_ERR_OUT_OF_RANGE`, and a body without a GM `HC_ERR_UNKNOWN`.

### The gravitating bodies

`hc_gravitating_bodies(buffer, capacity)` writes one line per body
`hc-relativity` carries a GM for:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the identifier `hc_gravitational_dilation` takes |
| 2 | name | the English name |
| 3 | gm | GM, in m³ s⁻² |
| 4 | gm constant | the `hc-relativity` constant that holds it |
| 5 | source | where the value is from |

The constants, as `hc-relativity` names them, and where each is from:

| Constant | Value | Source |
| --- | --- | --- |
| `SPEED_OF_LIGHT`, `SPEED_OF_LIGHT_SQUARED` | 299 792 458 m/s, and its square | exact by the 2019 SI |
| `GM_SUN` | 1.327 124 400 412 794 2·10²⁰ m³ s⁻² | JPL DE440 (Park et al. 2021), `BODY10_GM` of NAIF's `gm_de440.tpc` |
| `GM_EARTH` | 3.986 004 418·10¹⁴ m³ s⁻² | IERS Conventions (2010) and WGS 84 |
| `GM_MOON` | 4.902 800 118 457 55·10¹² m³ s⁻² | JPL DE440, `BODY301_GM` |
| `GM_MARS` | 4.282 837·10¹³ m³ s⁻² | JPL DE440, the Mars system, `BODY4_GM` to seven figures |
| `GM_JUPITER` | 1.267 127 64·10¹⁷ m³ s⁻² | JPL DE440, the Jupiter system, `BODY5_GM` |
| `GM_SAGITTARIUS_A_STAR` | 4.297·10⁶ × `GM_SUN` | GRAVITY Collaboration, A&A 657, L12 (2022); good to about 1 % with the systematic error |

`hc-relativity`'s README says how well each is known and why every
formula takes a GM rather than a mass.

```js
hc.properTime(0.6 * 299_792_458, 10).properSeconds;   // 8
hc.gravitationalDilation("earth", 26_561_750).microsecondsPerDay
  - hc.gravitationalDilation("earth", 6_378_137).microsecondsPerDay;   // 45.65
```

## What is not here

Formatting is by template, and a template is only as wide as its locale's
data: a locale that has stated none writes a date as its fields in order,
and a month no locale names comes back as a number. The C ABI in
[`hyper-calendar-ffi`](../hyper-calendar-ffi) covers the same ground with
NUL-terminated strings, out-parameters and status codes instead of
sentinels; a name that appears in both means the same thing in both, the
[twins](#twins) above are the exceptions, and the lines are the same
lines, made once in the facade.

Neither boundary exposes these parts of the workspace:

- **`hc-planetary`'s circad calendars and the Martiana calendar in the
  registry.** Their day number is a circad or a sol, not an Earth day, so
  they are not in the registry that `hc_describe_day` and
  `hc_calendar_units` walk, and a fixed day cannot be handed to them; an
  instant can, and `hc_circad_date` answers it.
- **`hc-humanize`, `hc-fiscal`, `hc-name-days`, `hc-almanac` (the 暦注
  notes), `hc-attributes` and `hc-units`.** No line format has been
  designed for them yet; each would be a layer of its own.
- **A whole UUID**: the 16 octets of a version 1 or 6 UUID, with the
  clock sequence and node that `hc-core`'s `uuid::encode` takes and no
  page has yet asked for. `hc_uuid_timestamp_encode` writes the time
  fields only, and leaves the rest to the caller's generator.
