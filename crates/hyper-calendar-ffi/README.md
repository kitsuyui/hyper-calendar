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
  buffer. The one piece of global state is the table of zones
  `hc_zone_load` fills, which the library owns and keeps for the life of
  the process; there is nothing to tear down.
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
| a year, month and day; a day of the year; 1 or 0; ISO 8601 text | `hc_gregorian_from_fixed`, `hc_day_of_year`, `hc_is_leap_year`, `hc_format_iso_date` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERROR_OUT_OF_RANGE` |
| a weekday, 1 through 7 | `hc_weekday` | every `fixed` |
| a line | `hc_gmtime` | `unix_seconds` of the years −9 999 999 through 9 999 999; any other is `HC_ERROR_OUT_OF_RANGE` |
| a POSIX second | `hc_timegm` | `year` −9 999 999 through 9 999 999 and `month` 1 through 12, with a `day`, `hour`, `minute` and `second` of any size, as Python's; a sum that leaves an `int64_t` is `HC_ERROR_OVERFLOW` and a month outside 1 to 12 `HC_ERROR_INVALID_DATE` |
| 1 or 0 | `hc_isleap` | every `year` |
| a count of leap years | `hc_leapdays` | `y1` and `y2` −9 999 999 through 9 999 999; any other is `HC_ERROR_OUT_OF_RANGE` |
| a weekday, 0 through 6 | `hc_calendar_weekday` | the dates of `year` −9 999 999 through 9 999 999; a date that does not exist is `HC_ERROR_INVALID_DATE`, a year outside them `HC_ERROR_OUT_OF_RANGE` |
| a line or lines | `hc_monthrange`, `hc_monthcalendar` | `year` −9 999 999 through 9 999 999 and `month` 1 through 12, and for `hc_monthcalendar` a `first_weekday` 0 through 6; another month or weekday is `HC_ERROR_INVALID_DATE`, another year `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_week_of_year` | `fixed` of the years −9 999 998 through 9 999 998, with `first_weekday` and `min_days` 1 through 7; any other is `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_fixed_from_week` | `week_year` −9 999 998 through 9 999 998, `week` 1 through the weeks the year has under the rule (52 or 53) and `weekday` 1 through 7, with `first_weekday` and `min_days` 1 through 7; a week or a weekday outside is `HC_ERROR_INVALID_DATE`, any other `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_asctime` | a reading `year` 1 through 9999, `month`, `day`, `hour`, `minute`, `second`, each field as `datetime` checks it; a field out of range, a second of 60 included, is `HC_ERROR_INVALID_DATE`, and a year outside 1 through 9999 `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_fixed_from_unix` | every `unix_seconds`; the day is between −106 751 990 448 138 and 106 751 991 886 463 |
| a POSIX timestamp | `hc_unix_from_fixed` | `fixed` −106 751 990 448 137 through 106 751 991 886 463, the days whose midnight fits an `int64_t`; any other is `HC_ERROR_OUT_OF_RANGE` |
| TAI seconds | `hc_tai_from_unix` | every `unix_seconds` up to `INT64_MAX − 37`; TAI runs ahead of UTC, so a later one has no TAI reading an `int64_t` holds and is `HC_ERROR_OVERFLOW`; under `strict`, 1961 through the end of the announced table, else `HC_ERROR_NO_DATA` |
| seconds | `hc_tai_minus_utc` | every `unix_seconds`; under `strict`, as for `hc_tai_from_unix` |
| 1 or 0 | `hc_day_has_leap_second` | `unix_seconds` −9 223 372 036 854 720 000 through 9 223 372 036 854 719 999, the whole days of the `int64_t` range; the part-days at its two ends begin or end where no `int64_t` reaches, and are `HC_ERROR_OUT_OF_RANGE`; under `strict`, a day past the announced leap-second table is `HC_ERROR_NO_DATA` |
| a POSIX timestamp | `hc_utc_from_tai` | every `tai_seconds`; under `strict`, as for `hc_tai_from_unix` |
| seconds and attoseconds | `hc_tai_minus_utc_exact` | every `unix_seconds`, with `attoseconds` below 10¹⁸ (`HC_ERROR_OUT_OF_RANGE` from there); under `strict`, as for `hc_tai_from_unix` |
| a POSIX second and attoseconds | `hc_utc_from_tai_exact` | every `tai_seconds`, with `tai_attoseconds` below 10¹⁸ (`HC_ERROR_OUT_OF_RANGE` from there); under `strict`, as for `hc_tai_from_unix` |
| a fixed day | `hc_fixed_from_unix_in_zone` | `unix_seconds` −315 631 497 830 400 through 315 507 195 014 399, the instants of the years −9 999 994 to 9 999 994 by UTC, which a zone's rules answer for: they are read on the Gregorian years ±9 999 999, and an instant's answer reads the years around its own, which beyond these would give standard time whatever the rules say; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_interval` | every `first_low_seconds`, `first_high_seconds`, `second_low_seconds` and `second_high_seconds` with each low bound not above its high one; a bound past 128 bits of seconds is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_unit_convert` | every `count_numerator` and `count_denominator` that is not 0 in the denominator; a count or a length past 128 bits is `HC_ERROR_OVERFLOW`, and a unit `hc_units` does not list `HC_ERROR_UNKNOWN` |
| a line | `hc_tempo` | every `bpm_numerator` and `bpm_denominator` whose tempo is positive; any other is `HC_ERROR_OUT_OF_RANGE`, and a length past 128 bits `HC_ERROR_OVERFLOW` |
| a line | `hc_fiscal_year_on` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERROR_OUT_OF_RANGE`; a day a system's start calendar does not reach is that system's `outside-calendar-range` line, and a country or kind not carried `HC_ERROR_UNKNOWN` |
| a line | `hc_fiscal_year_span` | every `label`; a label whose start the calendar does not reach is the system's `outside-calendar-range` line, and a country or kind not carried `HC_ERROR_UNKNOWN` |
| a line | `hc_week_year_on` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERROR_OUT_OF_RANGE`, and a system not listed `HC_ERROR_UNKNOWN` |
| a line | `hc_name_days_on` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERROR_OUT_OF_RANGE`, and a country neither listed nor a gap `HC_ERROR_UNKNOWN` |
| a line | `hc_name_day` | `year` within the Gregorian years ±9 999 999; any other is `HC_ERROR_OUT_OF_RANGE`, and a country neither listed nor a gap `HC_ERROR_UNKNOWN` |
| a line | `hc_attributions` | `key` 1 through 12, or 1 through 7 for `weekday`; any other is `HC_ERROR_OUT_OF_RANGE`, and a subject not named `HC_ERROR_UNKNOWN` |
| a line | `hc_attributions_on` | `fixed` −365 607 through 1 095 727, the years −1000 to 3000; any other is `HC_ERROR_OUT_OF_RANGE`, and a meridian not read `HC_ERROR_UNKNOWN` |
| a line | `hc_harvest_moon` | `year` −999 through 3000; any other is `HC_ERROR_OUT_OF_RANGE`, and a meridian not read `HC_ERROR_UNKNOWN` |
| a line | `hc_zone_offset` | `unix_seconds` −315 631 497 830 400 through 315 507 195 014 399, the instants of `hc_fixed_from_unix_in_zone`; any other is `HC_ERROR_OUT_OF_RANGE`, and a name neither the loaded zones nor the built-in table knows `HC_ERROR_UNKNOWN` |
| a line | `hc_localtime` | `unix_seconds` −315 631 497 830 400 through 315 507 195 014 399, the instants of `hc_zone_offset`; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_local_resolution` | a reading `year`, `month`, `day`, `hour`, `minute`, `second` of the years −9 999 994 through 9 999 994, each field as `datetime` checks it; a field out of range, a second of 60 included, is `HC_ERROR_INVALID_DATE`, a name neither the loaded zones nor the built-in table knows `HC_ERROR_UNKNOWN`, and a reading beyond those years `HC_ERROR_OUT_OF_RANGE` |
| a POSIX second | `hc_mktime` | a reading `year`, `month`, `day`, `hour`, `minute`, `second` of the years −9 999 994 through 9 999 994, each field as `datetime` checks it; a field out of range, a second of 60 included, or a reading no instant or two instants name under `reject`, is `HC_ERROR_INVALID_DATE`, and a reading beyond those years `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_zone_name`, `hc_format_pattern` | `unix_seconds` −315 631 497 830 400 through 315 507 195 014 399, the instants of `hc_zone_offset`; any other is `HC_ERROR_OUT_OF_RANGE`, and a zone or a field not known `HC_ERROR_UNKNOWN` |
| a POSIX timestamp | `hc_unix_from_fixed_in_zone` | `fixed` −3 652 423 173 through 3 652 422 808, the days of the same years as `hc_fixed_from_unix_in_zone`'s; any other is `HC_ERROR_OUT_OF_RANGE` |
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
| a line | `hc_ccsds_encode`, `hc_ccsds_ascii_format` | `tai_seconds` of an instant the format can hold, and attoseconds below 10¹⁸: for CUC from 1958-01-01 00:00:00 TAI, `tai_seconds` −378 691 200, to the format's last count; for CDS from 1958-01-01 UTC to the end of the day segment's last day; for CCS and the ASCII codes the years 1 to 9999; any other is `HC_ERROR_OUT_OF_RANGE`, and under `strict` an instant outside the leap-second table `HC_ERROR_NO_DATA` |
| a line | `hc_ccsds_decode_from_epoch` | `epoch_tai_seconds` −62 135 596 800 through 253 402 300 799 and `epoch_unix_day` −719 162 through 2 932 896, the instants and the days of the years 1 to 9999, and `epoch_attoseconds` below 10¹⁸; any other is `HC_ERROR_OUT_OF_RANGE`, and a code as for `hc_ccsds_decode` |
| a line | `hc_ccsds_encode_from_epoch` | `epoch_tai_seconds` and `epoch_unix_day` as for `hc_ccsds_decode_from_epoch`, and `tai_seconds` of an instant the format can hold: at Level 2 from the epoch to the format's last count, at Level 1 as for `hc_ccsds_encode`; any other is `HC_ERROR_OUT_OF_RANGE`, and under `strict` an instant outside the leap-second table `HC_ERROR_NO_DATA` |
| a line | `hc_radio_decode` | `century` a multiple of 100 from 0 through 9 900; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_irig_decode` | `year` 1 through 9999, whose century a code with the year's two digits reads them in; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_irig_encode` | `fixed` 1 through 3 652 059, the days of the years 1 to 9999, with whole seconds up to 86 400, 23:59:60, and hundredths up to 99; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_radio_encode` | `unix_seconds` a whole minute from −62 135 596 800 (0001-01-01 00:00) through 253 402 300 740 (9999-12-31 23:59); any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_jjy_call_sign_decode` | `year` 1 through 9999, the year the frame's day of the year is read in; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_jjy_call_sign_encode` | `unix_seconds` a whole minute from −62 135 596 800 (0001-01-01 00:00) through 253 402 300 740 (9999-12-31 23:59) that is minute 15 or 45 of an hour of JST; any other is `HC_ERROR_OUT_OF_RANGE` |
| ticks | `hc_dotnet_ticks_from_unix` | `unix_seconds` −62 135 596 800 through 253 402 300 799, 0001-01-01 to the end of 9999-12-31, and attoseconds below 10¹⁸, which write the ticks 0 through 3 155 378 975 999 999 999; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_unix_from_dotnet_ticks` | `ticks` 0 through 3 155 378 975 999 999 999, the range of `DateTime`; any other is `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_describe_day` | every `fixed`; a calendar that refuses the day says so in its own line |
| lines | `hc_day_extras` | every `fixed`; a calendar that refuses the day writes no line, and an `id` the registry does not carry is `HC_ERROR_UNKNOWN` |
| lines | `hc_calendar_units` | every `from_fixed` and `to_fixed` whose range is at most 100 000 units; a span a calendar refuses says so in its own line and counts as one, a `to_fixed` at or before `from_fixed` writes no lines, and a range of more units is `HC_ERROR_OUT_OF_RANGE`, as the WebAssembly module's [line caps](../hyper-calendar-wasm/README.md#line-caps) say |
| lines | `hc_calendars` | every `today` |
| a line | `hc_naming_period_on` | every `fixed`; a calendar the registry does not carry is `HC_ERROR_UNKNOWN` |
| a line | `hc_asian_day` | `fixed` 1 360 (23 September AD 4) through 3 652 398, the last day of the Asian year 9999; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_solar_new_year` | `year` in the calendar's own count: 1 through 3000 for `burmese`, 2444 through 2744 for `khmer` and 1301 through 1401 for `lao`; any other is `HC_ERROR_OUT_OF_RANGE`, and another calendar `HC_ERROR_UNKNOWN` |
| a line | `hc_southeast_asian_year_type` | `year` in the calendar's own count, 2444 through 2744 for `khmer` and 1301 through 1401 for `lao`; any other is `HC_ERROR_OUT_OF_RANGE`, and another calendar `HC_ERROR_UNKNOWN` |
| a line | `hc_maya_long_count` | `fixed` from 0.0.0.0.0 to 19.19.19.17.19 under the correlation: −1 137 142 through 1 742 857 under `gmt`, two days later under `gmt2` and three under `martin-skidmore`; any other is `HC_ERROR_OUT_OF_RANGE`, and a correlation not named `HC_ERROR_UNKNOWN` |
| a line | `hc_akan_day` | every `fixed` from −9 223 372 036 854 053 701, the first day whose distance from the cycle's epoch is an `int64_t`; any earlier is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_weton` | every `fixed` |
| a line | `hc_buddhist_lk_year` | `fixed` 738 521 (1 January 2023) through 740 346 (31 December 2027), the years whose Vesak Poya Day is carried; any other is `HC_ERROR_OUT_OF_RANGE` |
| a Tiruvaḷḷuvar year | `hc_tiruvalluvar_year` | `fixed` within the days `hindu-solar-tamil` converts, the Tamil solar years that open in the Gregorian years 1700 to 2299; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_japanese_era_year` | `year` from 1, as far as the Han numerals write; 0 and below, and a year beyond them, are `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_holidays_in_year` | every `year`; a year the table has no entries or gaps for writes no lines |
| 1 or 0; lines | `hc_holiday_is_day_off`, `hc_holiday_is_weekend`, `hc_holidays_on`, `hc_common_worship_on`, `hc_holidays_on_in` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERROR_OUT_OF_RANGE`, as is a day on which the region's weekend law was not read; for `hc_holiday_is_day_off` a day a gap of the day's year leaves open is `HC_ERROR_NO_DATA` |
| a line | `hc_holiday_next`, `hc_holiday_previous` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERROR_OUT_OF_RANGE`; the search reaches sixteen years, and a gap that could hide a nearer entry, or no entry of the kinds within the reach, is `HC_ERROR_NO_DATA` |
| a fixed day | `hc_holiday_add_business_days` | `fixed` −3 652 424 999 through 3 652 424 634 and `count` −36 500 through 36 500, a walk that stays in the years −9 999 999 to 9 999 999; any other is `HC_ERROR_OUT_OF_RANGE`, as is a walk that reaches a day whose weekend law was not read, a walk that reaches a day a gap leaves open is `HC_ERROR_NO_DATA`, and a code that names no table `HC_ERROR_UNKNOWN` |
| a count | `hc_holiday_business_days_between` | `from_fixed` and `to_fixed` −3 652 424 999 through 3 652 424 634, at most a hundred years apart; any other is `HC_ERROR_OUT_OF_RANGE`, and a code that names no table `HC_ERROR_UNKNOWN` |
| lines | `hc_roman_1960_office_on` | `fixed` 577 814 through 1 497 129, the Gregorian years 1583 to 4099; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_holy_year_on` | `fixed` 720 981 (24 December 1974) through 739 886 (27 September 2026), from the opening of the first jubilee the table carries to the day its sources were checked; any other is `HC_ERROR_NO_DATA` |
| a line | `hc_orthodox_fast_on` | `fixed` 118 705 through 1 497 157 on `orthodox-fasts` and `armenian-fasts-jerusalem` and 118 705 through 1 497 128 on `orthodox-fasts-revised-julian`, the years 326 to 4099 of each reckoning's calendar, whose Pascha the Julian computus gives; 577 814 through 1 497 129, the Gregorian years 1583 to 4099, on `armenian-fasts`, `armenian-fasts-fifty-days`, `coptic-fasts` and `ethiopian-fasts`; any other is `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_orthodox_fast_seasons` | `year` 326 through 4099, or 1583 through 4099 on `armenian-fasts`, `coptic-fasts` and `ethiopian-fasts`; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_lectionary` | `fixed` 719 163 through 1 497 096, 1 January 1970, when the Roman calendar of 1969 went into effect, to the liturgical year 4099; any other is `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_astronomical_easter` | `year` 1583 through 2150; any other is `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_astronomical_paschal_full_moon` | `year` 1583 through 2150, the years of `hc_astronomical_easter`; any other is `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_plum_rains` | `year` −1000 through 3000, the days of 芒种 and 小暑 in the era of `hc_term_in_effect`; any other is `HC_ERROR_OUT_OF_RANGE`, and a rule or meridian it does not name `HC_ERROR_UNKNOWN` |
| a fixed day | `hc_rounichi` | `year` −999 through 3000, whose winter the era of `hc_term_in_effect` holds; any other is `HC_ERROR_OUT_OF_RANGE`, a winter the reckoning does not settle `HC_ERROR_NO_DATA`, and a rule or meridian it does not name `HC_ERROR_UNKNOWN` |
| lines | `hc_almanac_person_days` | `fixed` and `birth_fixed` −365 607 through 1 095 727, the years −1000 to 3000; any other is `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_tibetan_almanac_day`, `hc_tibetan_planets` | `fixed` 364 892 through 1 095 802, the Tibetan years 1000 to 3000, and through 1 095 803 on `tibetan-bhutan`, `tibetan-tsurphu-karana` and `tibetan-bhutan-lochen`; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_bhutanese_winter_solstice` | `year` 1000 through 3000; any other is `HC_ERROR_OUT_OF_RANGE`, and one that holds no solstice, 1923, 1927, 1931, 1935, 1938, 1942, 1946, 1949, 1953 and 1957, `HC_ERROR_NO_DATA` |
| a fixed day | `hc_tibetan_festival_day` | `year` 1000 through 3000; any other is `HC_ERROR_OUT_OF_RANGE`, a month or day the year does not have `HC_ERROR_INVALID_DATE`, and a skipped number under `henning-almanac` `HC_ERROR_NO_DATA` |
| a fixed day | `hc_cold_food_day` | `year` −999 through 3000 under every reckoning, the years whose winter solstice before and whose April are both in the era of `hc_term_in_effect`; any other is `HC_ERROR_OUT_OF_RANGE`, and a reckoning it does not name `HC_ERROR_UNKNOWN` |
| a line or lines | `hc_term_in_effect`, `hc_pentad_in_effect`, `hc_solar_event`, `hc_sunrise`, `hc_sunset`, `hc_moonrise`, `hc_moonset`, `hc_dawn`, `hc_dusk`, `hc_crescent_visible`, `hc_kalam`, `hc_muhurtas`, `hc_amrita_siddhi`, `hc_nakshatra_of_day`, `hc_almanac_cycles`, `hc_almanac_day`, `hc_almanac_directions`, `hc_mansion_undertakings`, `hc_prayer_times`, `hc_zmanim`, `hc_temporal_hour`, `hc_unix_from_edo_time`, `hc_choghadiya`, `hc_folk_day`, `hc_planetary_hours_of_day` | `fixed` −365 607 through 1 095 727, the years −1000 to 3000; any other is `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_panchanga_of_day` | `fixed` −365 607 through 1 095 727, the years −1000 to 3000, on the true sky, and −1 132 604 through 2 519 974, Kali Yuga 1 to 10 000, on `surya-siddhanta`; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_hindu_lunar_date` | `fixed` in the Śaka years 1622 through 2221 on the true sky, from Chaitra śukla 1 in March 1700 to the eve of the one in March 2300, whose days move with the place and the ayanāṃśa (620 627 through 839 773 at the Central Station with Lahiri's); on `surya-siddhanta`, −1 132 604 through 2 519 974, Kali Yuga 1 to 10 000; any other is `HC_ERROR_OUT_OF_RANGE`, as is a place beyond 65° of latitude; on the true sky, a day whose sunrise at the place the model does not find is `HC_ERROR_NO_DATA` |
| a line | `hc_surya_siddhanta_sunrise` | `fixed` −1 132 604 through 2 519 974, Kali Yuga 1 to 10 000; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_surya_siddhanta_at`, `hc_barhaspatya_year_at` | `unix_seconds` −159 992 668 800 through 155 590 156 799, the days of Kali Yuga 1 to 10 000; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_barhaspatya_year` | `saka` −3178 through 6821, the expired Śaka years of Kali Yuga 1 to 10 000; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line or lines | `hc_sky_at`, `hc_decan_at`, `hc_drekkana_at`, `hc_nakshatra_at`, `hc_solar_time`, `hc_edo_time`, `hc_panchak`, `hc_planetary_hour`, `hc_equation_of_time` | `unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; any other is `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_panchanga_at` | `unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000, on the true sky, and −159 992 668 800 through 155 590 156 799, Kali Yuga 1 to 10 000, on `surya-siddhanta`; any other is `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_pushkaram` | `entry_unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_kumbh` | `year` −1000 through 3000; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_jupiter_at` | `unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; any other is `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_jupiter_ingresses`, `hc_jupiter_risings` | `from_unix_seconds` and `to_unix_seconds` −93 724 128 000 through 32 535 216 000 (the span `[from, to)` ends within the years −1000 to 3000), at most 3 155 760 000 apart, a hundred Julian years; a `to` not after the `from` writes nothing; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_kumbh_by_sky`, `hc_pushkaram_by_sky`, `hc_pushkarams_in_year` | `year` −1000 through 3000; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_jupiter_stations` | `from_unix_seconds` −93 724 128 000 through 32 535 215 999 and `to_unix_seconds` up to 32 535 216 000, the span `[from, to)` within the years −1000 to 3000, and no more than a hundred Julian years apart; any other is `HC_ERROR_OUT_OF_RANGE`, a `to` not after `from` an empty answer |
| a line | `hc_kumbhs_in_year_by_sky` | `year` −1000 through 3000; any other is `HC_ERROR_OUT_OF_RANGE`, and an ayanāṃśa not known `HC_ERROR_UNKNOWN` |
| a line | `hc_tithis_of_day` | `fixed` −365 607 through 1 095 727, the years −1000 to 3000, and on `surya-siddhanta` −1 132 604 through 2 519 973; any other is `HC_ERROR_OUT_OF_RANGE`, and a day or morrow without a sunrise `HC_ERROR_NO_DATA` |
| a line | `hc_tithi_at`, `hc_ayanamsa_at`, `hc_ayanamsa_from_anchor` | `unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000, and on `surya-siddhanta` for `hc_tithi_at` −159 992 668 800 through 155 590 156 799, the days of Kali Yuga 1 to 10 000; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_janmashtami` | `year` 1700 through 2299, the Gregorian years whose Śrāvaṇa lies in the Śaka years 1622 to 2221 of the Rashtriya Panchang's calendar; any other is `HC_ERROR_OUT_OF_RANGE`, a reading or an ayanāṃśa not known `HC_ERROR_UNKNOWN`, and a place beyond 65° of latitude `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_vaishnava_day`, `hc_vishti_free_span` | `saka_year` 1622 through 2221 and `month` 1 through 12; a tithi outside 1 to 30 is `HC_ERROR_INVALID_DATE`, any other year or month, a month the year lacks (Mārgaśīrṣa of Śaka 1885 at the Central Station) included, and a place beyond 65° of latitude, `HC_ERROR_OUT_OF_RANGE`, and an ayanāṃśa not known `HC_ERROR_UNKNOWN` |
| a line | `hc_rahu_at` | `unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; any other is `HC_ERROR_OUT_OF_RANGE`, and an ayanāṃśa not known `HC_ERROR_UNKNOWN` |
| a line | `hc_rahu_ingresses`, `hc_solar_nakshatra_ingresses` | `from_unix_seconds` and `to_unix_seconds` −93 724 128 000 through 32 535 216 000 (the span `[from, to)` ends within the years −1000 to 3000), at most 3 155 760 000 apart, a hundred Julian years; a `to` not after the `from` writes nothing; any other is `HC_ERROR_OUT_OF_RANGE`, and an ayanāṃśa not known `HC_ERROR_UNKNOWN` |
| a line | `hc_zassetsu_in_year`, `hc_seasonal_days_in_year` | `year` −1000 through 3000; any other is `HC_ERROR_OUT_OF_RANGE`, and a meridian not read `HC_ERROR_UNKNOWN` |
| a line | `hc_pentad_in_tradition` | `fixed` −365 607 through 1 095 727, the years −1000 to 3000; any other is `HC_ERROR_OUT_OF_RANGE`, and a tradition not listed `HC_ERROR_UNKNOWN` |
| lines | `hc_pentads_in_year` | `year` −1000 through 3000; any other is `HC_ERROR_OUT_OF_RANGE`, and a meridian not read `HC_ERROR_UNKNOWN` |
| lines | `hc_tropical_signs_in_year` | `year` −1000 through 3000; any other is `HC_ERROR_OUT_OF_RANGE`, and a meridian not read `HC_ERROR_UNKNOWN` |
| lines | `hc_sidereal_signs_in_year` | `year` −1000 through 3000; any other is `HC_ERROR_OUT_OF_RANGE`, an ayanāṃśa not known `HC_ERROR_UNKNOWN`, read first, and a meridian not read `HC_ERROR_UNKNOWN` |
| a fixed day | `hc_traditional_tanabata` | `year` −1000 through 3000; any other is `HC_ERROR_OUT_OF_RANGE`, and a meridian not read `HC_ERROR_UNKNOWN` |
| lines | `hc_principal_phases_in_month` | `year` −1000 through 3000 and a month 1 through 12; any other year is `HC_ERROR_OUT_OF_RANGE`, any other month `HC_ERROR_INVALID_DATE`, and a meridian not read `HC_ERROR_UNKNOWN` |
| POSIX seconds | `hc_solar_noon`, `hc_solar_midnight` | `fixed` −365 607 through 1 095 727, the years −1000 to 3000, at a place on the globe; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_gmat_from_gmt`, `hc_gmt_from_gmat` | `fixed` −3 652 424 999 through 3 652 424 634, the Gregorian years −9 999 999 to 9 999 999, with whole seconds up to 86 400 and attoseconds below 10¹⁸; 23:59:60, which neither reckoning shifts, and any other are `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_solar_terms_between`, `hc_moon_phases_between` | `from_unix` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; a `to_unix` at or before it writes no lines, and a later one must be at most 32 535 216 000 and at most 400 years after it; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_chinese_marriage_augury` | `chinese_year` 4282 through 4786, whose New Year and the next both fall in the Chinese calendar's range (1645 through 2150); any other is `HC_ERROR_OUT_OF_RANGE` |
| an age | `hc_chinese_age` | `birth_fixed` and `on_fixed` 600 460 through 785 271, the Chinese calendar's range, under `chinese-age` and `lichun-age`, and −3 652 424 999 through 3 652 424 634 under the others, a birth before or on the day asked; a day before the birth is `HC_ERROR_NO_DATA`, and a day outside the range `HC_ERROR_OUT_OF_RANGE` |
| lines | `hc_chinese_almanac_solar_terms` | `year` 1645 through 1733 but 1667 to 1669; any other is `HC_ERROR_NO_DATA` |
| an age | `hc_chinese_reckoned_age` | `birth_fixed` and `on_fixed` 600 460 through 785 271, the days of the Chinese calendar's range, 1645 through 2150, a birth before or on the day asked; a day before the birth is `HC_ERROR_NO_DATA`, and a day outside the range `HC_ERROR_OUT_OF_RANGE` |
| an Olympiad | `hc_ioc_olympiad` | `gregorian_year` from 1896; an earlier one is `HC_ERROR_OUT_OF_RANGE` |
| an Olympiad | `hc_ioc_olympiad_on` | `fixed` from 692 231, the opening of 6 April 1896, through 3 652 424 634; an earlier day is `HC_ERROR_OUT_OF_RANGE`, and one from 10 June to 21 November 1956 `HC_ERROR_NO_DATA` |
| a line | `hc_babylonian_regnal_year` | `seleucid_year` −314 through 160; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_equinox_new_year_margin` | `year` in the calendar's own range, the years whose first days `hc_calendars` lists it for; any other is `HC_ERROR_OUT_OF_RANGE`, and a calendar not named `HC_ERROR_UNKNOWN` |
| a line | `hc_shmuel_tekufah` | `hebrew_year` 1 through 9999; any other is `HC_ERROR_OUT_OF_RANGE`, and a *tekufah* not named `HC_ERROR_UNKNOWN` |
| a line | `hc_day_name` | every `fixed` the calendar converts; a day it refuses is `HC_ERROR_OUT_OF_RANGE`, and one the naming does not name `HC_ERROR_NO_DATA` |
| a line | `hc_format_number` | every `value` the system writes: from 1 for the Hebrew numerals, every `int64_t` for a positional system; any other is `HC_ERROR_OUT_OF_RANGE`, and a system not named `HC_ERROR_UNKNOWN` |
| an integer | `hc_parse_number` | text in the system's notation whose value fits an `int64_t`; any other is `HC_ERROR_MALFORMED` or `HC_ERROR_OUT_OF_RANGE`, and a system not named `HC_ERROR_UNKNOWN` |
| a place in the cycle, 1 through 7 | `hc_hebrew_sabbatical_cycle_year` | `hebrew_year` 1 through 9999; any other is `HC_ERROR_OUT_OF_RANGE` |
| a fixed day | `hc_era_new_year` | `year` of the era's own count: `vikram-samvat-kartikadi` 1757 through 2356, `rajyabhisheka-saka` 27 through 626 and `saptarshi` 4776 through 5375 over the Rashtriya Panchang's months, and `gupta` −3419 through 6580, `valabhi` −3418 through 6581, `kalachuri` −3347 through 6652 and `lakshmana-sena` −4218 through 5781 over the *Sūrya Siddhānta*'s, Kali Yuga 1 to 10 000; any other year is `HC_ERROR_OUT_OF_RANGE`, and an era not named `HC_ERROR_UNKNOWN` |
| a fixed day | `hc_hebrew_yahrzeit`, `hc_hebrew_birthday` | `death_fixed` and `birth_fixed` −1 373 427 through 2 278 650 and `hebrew_year` 1 through 9999, the Hebrew years 1 through 9999; any other is `HC_ERROR_OUT_OF_RANGE` |
| a mission sol, from 0 or 1 | `hc_mission_sol` | the instants from the midnight that began the mission's landing sol through 100 Julian years after J2000.0 (2100-01-01T12:00 TT); an earlier instant, or one not finite, is `HC_ERROR_OUT_OF_RANGE`, a mission whose operators published no sol numbering `HC_ERROR_NO_DATA`, and a mission the table does not carry `HC_ERROR_UNKNOWN` |
| a line | `hc_relative_time` | every `then_unix` and `now_unix` less than an `int64_t` of seconds apart; two further apart are `HC_ERROR_OUT_OF_RANGE`, and a style not named `HC_ERROR_UNKNOWN` |
| a line | `hc_relative_day`, `hc_relative_day_at` | every `then_fixed` and `now_fixed` less than an `int64_t` of days apart, and for `hc_relative_day_at` seconds of the day below 86 400; any other is `HC_ERROR_OUT_OF_RANGE`, and a style not named `HC_ERROR_UNKNOWN` |
| a line | `hc_duration` | every `seconds`; a style not named is `HC_ERROR_UNKNOWN` |
| a line | `hc_apnumber`, `hc_ordinal` | every `value` |
| a line | `hc_format_datetime` | every `unix_seconds` an `int64_t` holds whose local reading in `offset_seconds` lies in the Gregorian range, and for RFC 3339, RFC 2822 and HTTP years 0000 to 9999; any other is `HC_ERROR_OUT_OF_RANGE`, and a syntax or precision not named `HC_ERROR_UNKNOWN` |
| a line | `hc_format_iso_date_as` | `fixed` −3 652 424 999 through 3 652 424 634, the days of `hc_gregorian_year`; any other is `HC_ERROR_OUT_OF_RANGE`, and a form or style not named `HC_ERROR_UNKNOWN` |
| a line | `hc_format_iso_duration` | `years`, `months`, `weeks`, `days`, `hours`, `minutes` and `seconds` from 0 through an `int64_t`'s largest, a negative one being absent; components ISO 8601 cannot spell are `HC_ERROR_MALFORMED` |
| a line | `hc_naturaldelta`, `hc_naturaltime`, `hc_precisedelta` | every `seconds`, with `microseconds` below 1 000 000 in magnitude; any other is `HC_ERROR_OUT_OF_RANGE`, a unit or a gender not named `HC_ERROR_UNKNOWN`, and a span of more than about 10²⁶ years `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_naturalday`, `hc_naturaldate` | `day` and `today` −3 652 424 999 through 3 652 424 634, the days of `hc_gregorian_year`; any other is `HC_ERROR_OUT_OF_RANGE` |
| a line | `hc_unit_choice`, `hc_approximate_duration` | every `seconds` whose count fits an `int64_t`; a table, rounding, policy or style not named is `HC_ERROR_UNKNOWN` |
| a line | `hc_relative_time_with` | every `then_unix` and `now_unix` less than an `int64_t` of seconds apart, as for `hc_relative_time`; a table or a rounding not named is `HC_ERROR_UNKNOWN` |

A day outside the Gregorian range is `HC_ERROR_OUT_OF_RANGE` from the
entry points in the third row, and the WebAssembly module's
`hc_gregorian_year` and its neighbours answer the same day with
`HC_ERR_OUT_OF_RANGE`: a library error becomes a refusal in one place,
`hyper_calendar::boundary`, whose rustdoc tabulates the codes, so both
boundaries refuse one input alike. A date before a calendar's epoch or
past its supported range is out of range; `HC_ERROR_NO_DATA` is for a
value a table of data does not reach, such as the leap-second table
under `strict`.

### No floor here, a floor there: a deliberate difference

The two interfaces answer different questions for the same far-past day,
and that is by design, not drift.

The WebAssembly module returns its answer and its error in the same `i64`:
a value-returning export gives either the result or an `HC_ERR_*` sentinel,
because a WebAssembly function has one return value and a trap would tear
down the instance. So every sentinel sits at or below `HC_ERR_FLOOR`,
−9 × 10¹⁵, and an export that answers in seconds must refuse a result that
would reach it, or a binding would read the answer as an error. Its
`hc_unix_from_fixed` therefore refuses every day before fixed day
−104 165 947 503, about 285 million years back, with `HC_ERR_OUT_OF_RANGE`
([its README](../hyper-calendar-wasm/README.md#ranges)).

This library returns the error as an `HcStatus` and the answer through an
out-parameter, so no value of the answer is reserved, and a floor would
refuse days for no reason a C caller has. `hc_unix_from_fixed` here
answers down to where an `int64_t` runs out: fixed day
−106 751 990 448 137, about 292 billion years back. For the days from
there to fixed day −104 165 947 504, this library writes a timestamp and
the WebAssembly module refuses. `hc_unix_from_fixed_in_zone` answers in
both for the years a zone's rules answer for, −9 999 994 to 9 999 994,
far inside either bound, and the two agree.

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
  - `hc_unix_from_fixed` for a day whose start overflows;
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
`\t`, and the whole NUL-terminated like every other text here. Before 1.0
a line's columns may change — a column added at any position, removed or
given another meaning — and the pull request that changes one lists the
change under the entry point's name. A stable identifier is in column 1,
or in column 2 after a column naming the kind of line, as the WebAssembly
README states export by export. A cell with nothing to say is empty, and
no cell contains a tab or a line break. They are the same
lines the WebAssembly module writes, whose
[README](../hyper-calendar-wasm/README.md) tabulates every column order,
and the sections below name the differences at this boundary: strings in
are NUL-terminated and may be null, and the length comes back through
`written`.

## Identifiers

An argument that names something — a calendar, a convention, a holiday
table, a zone, a horizon, a method — is matched by one rule: the white
space around it is ignored and its ASCII letters match in either case, so
`" Gregory "` names `gregory` and `us` names `US`. It is
`hc_core::catalogue::matches`, and every lookup behind an entry point
follows it (`docs/policy.md` §5). A name no table carries is
`HC_ERROR_UNKNOWN`.

## Layers

The entry points come in layers, each a Cargo feature, the same layers as
the WebAssembly module's: `civil` (the default), `timestamps`, `time-codes`, `calendars`, `holiday`,
`seasons`, `deep-time`, `tz`, `sky`, `orbital`, `jupiter`, `planetary`, `relativity`, `places`,
`humanize`, `natural`, `datetime`, `patterns`, `zone-names`, `uncertainty`, `units`, `fiscal`, `name-days`, `attributes` and `full`.
One pair sits in a different layer: `hc_tai_from_unix` and
`hc_utc_from_tai` are `civil` here and `timestamps` there.
Each builds on its own —
`calendars` does not need `holiday` — and the table below names the one
each entry point needs. CI runs this crate's tests with each layer's
feature alone, and the facade's own with the facade features the layer
turns on, through [`scripts/layer-tests.sh`](../../scripts/layer-tests.sh).

```sh
cargo build -p hyper-calendar-ffi --release --features calendars
cargo build -p hyper-calendar-ffi --release --features full
```

## What is exported

The two tables below are rendered from the crate's sources, and from the
rows of `hyper_calendar`'s table of shared exports
([`crates/hyper-calendar/src/exports.rs`](../hyper-calendar/src/exports.rs))
each layer module expands, by
[`crates/hyper-calendar/tests/abi.rs`](../hyper-calendar/tests/abi.rs), which
fails when they drift. An entry point without a row here does not pass CI.

<!-- generated by crates/hyper-calendar/tests/abi.rs: start -->

### Entry points

340 functions. Each is `extern "C"`, takes nothing it has to free and returns an `HcStatus`. The feature column is the Cargo feature the library has to be built with for the entry point to exist.

| Prototype | Feature | What it does |
| --- | --- | --- |
| `HcStatus hc_version(char *buffer, size_t capacity, size_t *written);` | always | The library version, as a NUL-terminated string. |
| `HcStatus hc_gregorian_to_fixed(int64_t year, uint8_t month, uint8_t day, int64_t *out_fixed);` | `civil` | The fixed day number of a proleptic Gregorian date. |
| `HcStatus hc_gregorian_from_fixed(int64_t fixed, int64_t *out_year, uint8_t *out_month, uint8_t *out_day);` | `civil` | The proleptic Gregorian date on a fixed day. |
| `HcStatus hc_parse_iso_date(const char *text, int64_t *out_fixed);` | `civil` | Parse an ISO 8601 date from a NUL-terminated UTF-8 string into its fixed day number. |
| `HcStatus hc_format_iso_date(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `civil` | Render a fixed day as an ISO 8601 date into a caller-owned buffer. |
| `HcStatus hc_tai_from_unix(int64_t unix_seconds, int strict, int64_t *out_tai_seconds, uint64_t *out_attoseconds);` | `civil` | Convert a POSIX timestamp to a TAI reading in seconds and attoseconds. |
| `HcStatus hc_tai_minus_utc(int64_t unix_seconds, int strict, int64_t *out_offset);` | `civil` | `TAI - UTC` in whole seconds at a POSIX timestamp. |
| `HcStatus hc_utc_from_tai(int64_t tai_seconds, int strict, int64_t *out_unix_seconds, int *out_is_leap_second);` | `civil` | Convert a TAI reading back to a UTC label, naming a leap second when the instant falls inside one. |
| `HcStatus hc_weekday(int64_t fixed, uint8_t *out_weekday);` | `civil` | The ISO 8601 weekday of a fixed day, Monday = 1 through Sunday = 7. |
| `HcStatus hc_day_of_year(int64_t fixed, uint32_t *out_day_of_year);` | `civil` | The 1-based day of the Gregorian year on a fixed day. |
| `HcStatus hc_is_leap_year(int64_t fixed, int *out_is_leap);` | `civil` | Whether the Gregorian year on a fixed day is a leap year: writes 1 or 0. |
| `HcStatus hc_fixed_from_unix(int64_t unix_seconds, int64_t *out_fixed);` | `civil` | The fixed day a POSIX timestamp falls on, in UTC. |
| `HcStatus hc_day_has_leap_second(int64_t unix_seconds, int strict, int *out_has_leap);` | `civil` | Whether a POSIX timestamp names a day that ends with an inserted leap second. |
| `HcStatus hc_unix_from_fixed(int64_t fixed, int64_t *out_unix_seconds);` | `civil` | The POSIX timestamp of midnight UTC on a fixed day. |
| `HcStatus hc_gmtime(int64_t unix_seconds, char *buffer, size_t capacity, size_t *written);` | `civil` | Python's `time.gmtime(seconds)`: the UTC reading of a POSIX second as the nine fields of a `struct_time`, one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_timegm(int64_t year, int64_t month, int64_t day, int64_t hour, int64_t minute, int64_t second, int64_t *out_unix_seconds);` | `civil` | Python's `calendar.timegm(tuple)`: the POSIX second of a UTC reading given as its year, month, day, hour, minute and second, written to `out_unix_seconds`. |
| `HcStatus hc_isleap(int64_t year, int *out_is_leap);` | `civil` | Python's `calendar.isleap(year)`: whether a proleptic Gregorian year is a leap year, for any year; writes 1 or 0. |
| `HcStatus hc_leapdays(int64_t y1, int64_t y2, int64_t *out_leap_days);` | `civil` | Python's `calendar.leapdays(y1, y2)`: the number of leap years from `y1` up to but not including `y2`, counted backwards when `y2` is before `y1`, written to `out_leap_days`. |
| `HcStatus hc_calendar_weekday(int64_t year, uint32_t month, uint32_t day, uint8_t *out_weekday);` | `civil` | Python's `calendar.weekday(year, month, day)`: the weekday of a Gregorian date with Monday 0 and Sunday 6, written to `out_weekday`, where `hc_weekday` answers Monday 1 to Sunday 7 for a fixed day. |
| `HcStatus hc_monthrange(int64_t year, uint32_t month, char *buffer, size_t capacity, size_t *written);` | `civil` | Python's `calendar.monthrange(year, month)`: the weekday of the first day of a Gregorian month, Monday 0, and the number of days in the month, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_monthcalendar(int64_t year, uint32_t month, uint32_t first_weekday, char *buffer, size_t capacity, size_t *written);` | `civil` | Python's `calendar.monthcalendar(year, month)`, the weeks of a Gregorian month as NUL-terminated UTF-8 lines in a caller-owned buffer, with the first weekday of the week as an argument. |
| `HcStatus hc_week_of_year(int64_t fixed, uint32_t first_weekday, uint32_t min_days, char *buffer, size_t capacity, size_t *written);` | `civil` | A fixed day's week of the year under a week rule, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_fixed_from_week(int64_t week_year, uint32_t week, uint32_t weekday, uint32_t first_weekday, uint32_t min_days, int64_t *out_fixed);` | `civil` | The fixed day a week date names under a week rule, written to `out_fixed`: the inverse of `hc_week_of_year`. |
| `HcStatus hc_asctime(int64_t year, int64_t month, int64_t day, int64_t hour, int64_t minute, int64_t second, char *buffer, size_t capacity, size_t *written);` | `civil` | Python's `time.asctime` of a Gregorian reading, `Sun Jun 20 23:21:05 1993`, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_tai64_decode(const char *hex, int64_t *out_tai_seconds, uint64_t *out_attoseconds);` | `timestamps` | A TAI64, TAI64N or TAI64NA label in hexadecimal read back into the TAI seconds and attoseconds of the instant it names. |
| `HcStatus hc_gnss_week(const char *numbering, int64_t tai_seconds, uint64_t attoseconds, uint32_t *out_week, uint32_t *out_broadcast, uint32_t *out_tow_seconds, uint64_t *out_tow_attoseconds);` | `timestamps` | The GNSS week and time of week of a TAI instant. |
| `HcStatus hc_gnss_to_tai(const char *numbering, uint32_t week, uint32_t tow_seconds, uint64_t tow_attoseconds, int64_t *out_tai_seconds, uint64_t *out_attoseconds);` | `timestamps` | The TAI instant of a full GNSS week and a time of week. |
| `HcStatus hc_glonass_date(int64_t tai_seconds, uint64_t attoseconds, int strict, uint32_t *out_four_year_interval, uint32_t *out_day);` | `timestamps` | GLONASS's four-year interval N4 and day N_T at a TAI instant. |
| `HcStatus hc_fixed_from_ole_automation(double value, int64_t *out_fixed, double *out_seconds_of_day);` | `timestamps` | The fixed day and the time of day, in seconds, of an OLE Automation date. |
| `HcStatus hc_ole_automation_from_fixed(int64_t fixed, double seconds_of_day, double *out_value);` | `timestamps` | The OLE Automation date of a fixed day and a time of day in seconds. |
| `HcStatus hc_excel_1900_day(int64_t serial, int64_t *out_fixed, int *out_phantom);` | `timestamps` | What an Excel 1900 serial names. |
| `HcStatus hc_tai64_posix_plus_10_decode(const char *hex, int64_t *out_unix_seconds, uint64_t *out_attoseconds);` | `timestamps` | A TAI64 or TAI64N label in the `tai64-posix-plus-10` convention read back into the POSIX seconds and attoseconds of the instant it names. |
| `HcStatus hc_uuid_timestamp(const char *uuid, int *out_version, uint64_t *out_timestamp, int64_t *out_unix_seconds, uint64_t *out_attoseconds);` | `timestamps` | The version, the 60-bit timestamp and the POSIX instant of a version 1 or version 6 UUID. |
| `HcStatus hc_ntp_resolve(uint32_t seconds, uint32_t fraction, int64_t reference_unix, int32_t *out_era, uint32_t *out_offset, uint64_t *out_fraction, int64_t *out_unix_seconds, uint64_t *out_attoseconds);` | `timestamps` | A 64-bit NTP timestamp placed in its era by a reference POSIX second: the era, the era offset, the fraction in 2⁻⁶⁴ s units, and the POSIX seconds and attoseconds. |
| `HcStatus hc_fat_decode(uint16_t date, uint16_t time, int64_t *out_fixed, uint32_t *out_seconds_of_day);` | `timestamps` | The local reading a FAT date word and time word name: its fixed day and the seconds into it, always even. |
| `HcStatus hc_fat_encode(int64_t fixed, uint32_t seconds_of_day, uint16_t *out_date, uint16_t *out_time);` | `timestamps` | The FAT date and time words of a fixed day and a time of day in whole seconds, the second rounded down to an even one. |
| `HcStatus hc_epoch_from_tt(const char *notation, int64_t tt_seconds, uint64_t attoseconds, double *out_year);` | `timestamps` | The Julian or Besselian epoch of a TT instant, as a year with a fraction. |
| `HcStatus hc_tt_from_epoch(const char *notation, double year, int64_t *out_tt_seconds, uint64_t *out_attoseconds);` | `timestamps` | The TT instant of a Julian or Besselian epoch, as whole seconds from 1970-01-01 00:00:00 TT and attoseconds. |
| `HcStatus hc_tai_minus_utc_exact(int64_t unix_seconds, uint64_t attoseconds, int strict, char *buffer, size_t capacity, size_t *written);` | `timestamps` | `TAI - UTC` at a POSIX instant, exactly, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_utc_from_tai_exact(int64_t tai_seconds, uint64_t tai_attoseconds, int strict, char *buffer, size_t capacity, size_t *written);` | `timestamps` | The UTC label of a TAI instant, exactly, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_tai64_encode(int64_t tai_seconds, uint64_t attoseconds, const char *format, char *buffer, size_t capacity, size_t *written);` | `timestamps` | A TAI instant as a TAI64, TAI64N or TAI64NA label in lower-case hexadecimal, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_gnss_resolve_week(const char *numbering, uint32_t broadcast, const char *rule, int64_t reference_tai_seconds, uint32_t *out_week);` | `timestamps` | The full GNSS week a broadcast week names, by a rollover rule and a reference instant. |
| `HcStatus hc_tai64_posix_plus_10_encode(int64_t unix_seconds, uint64_t attoseconds, const char *format, char *buffer, size_t capacity, size_t *written);` | `timestamps` | A POSIX instant as a TAI64 or TAI64N label in the `tai64-posix-plus-10` convention, in lower-case hexadecimal, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_uuid_timestamp_encode(int64_t unix_seconds, uint64_t attoseconds, char *buffer, size_t capacity, size_t *written);` | `timestamps` | The 60-bit UUID timestamp of a POSIX instant, and the time fields a version 1 and a version 6 UUID write it in, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ntp_encode(int64_t unix_seconds, uint64_t attoseconds, char *buffer, size_t capacity, size_t *written);` | `timestamps` | The NTP date and timestamp of a POSIX instant, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_swatch_beat(int64_t unix_seconds, uint64_t attoseconds, uint16_t *out_beat);` | `timestamps` | The Swatch Internet Time at a POSIX instant, 0 through 999: the thousandth of the day of Biel Mean Time, UTC+1 all year, that it falls in, so @000 begins at 23:00 UTC. |
| `HcStatus hc_tt_bipm(const char *series, int64_t tai_seconds, uint64_t attoseconds, int strict, char *buffer, size_t capacity, size_t *written);` | `timestamps` | TT(BIPM) at a TAI instant, read from a realisation the caller supplies, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_dotnet_ticks_from_unix(int64_t unix_seconds, uint64_t attoseconds, int64_t *out_ticks);` | `timestamps` | .NET's `DateTime.Ticks` of a POSIX instant as a `Utc` value, the 100-nanosecond intervals from 0001-01-01 00:00, floored to the tick. |
| `HcStatus hc_unix_from_dotnet_ticks(int64_t ticks, char *buffer, size_t capacity, size_t *written);` | `timestamps` | The reading a count of .NET ticks names, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_six_hour_clock(const char *reckoning, uint32_t seconds_of_day, char *buffer, size_t capacity, size_t *written);` | `timestamps` | A time of the civil day on a six-hour clock, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_civil_from_six_hour_clock(const char *reckoning, uint32_t hour, uint32_t minute, uint32_t second, int night, uint32_t *out_seconds);` | `timestamps` | The civil time of day of a six-hour reading, as seconds after midnight, 0 through 86 399. |
| `HcStatus hc_french_decimal_time(uint32_t seconds_of_day, uint64_t attoseconds, char *buffer, size_t capacity, size_t *written);` | `timestamps` | The French Republican decimal time of a time of the civil clock, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_civil_from_french_decimal_time(uint32_t hour, uint32_t minute, uint32_t second, uint64_t attoseconds, char *buffer, size_t capacity, size_t *written);` | `timestamps` | The civil time of day of a French Republican decimal time, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_epochs(char *buffer, size_t capacity, size_t *written);` | `timestamps` | Every epoch `hc-core` carries, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_ccsds_decode(const char *hex, int strict, char *buffer, size_t capacity, size_t *written);` | `time-codes` | A binary CCSDS time code read, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ccsds_encode(int64_t tai_seconds, uint64_t attoseconds, const char *p_field, int strict, char *buffer, size_t capacity, size_t *written);` | `time-codes` | The binary CCSDS time code of a TAI instant in the format a P-field names, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ccsds_decode_from_epoch(const char *hex, int64_t epoch_tai_seconds, uint64_t epoch_attoseconds, int64_t epoch_unix_day, int strict, char *buffer, size_t capacity, size_t *written);` | `time-codes` | A binary CCSDS time code read, a Level 2 code from the caller's epoch, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ccsds_encode_from_epoch(int64_t tai_seconds, uint64_t attoseconds, const char *p_field, int64_t epoch_tai_seconds, uint64_t epoch_attoseconds, int64_t epoch_unix_day, int strict, char *buffer, size_t capacity, size_t *written);` | `time-codes` | The binary CCSDS time code of a TAI instant in the format a P-field names, a Level 2 format from the caller's epoch, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ccsds_ascii_parse(const char *code, int strict, char *buffer, size_t capacity, size_t *written);` | `time-codes` | A CCSDS ASCII time code, A or B, read, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ccsds_ascii_format(int64_t tai_seconds, uint64_t attoseconds, const char *variation, const char *precision, int terminator, int strict, char *buffer, size_t capacity, size_t *written);` | `time-codes` | The CCSDS ASCII time code of a TAI instant's UTC label, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_radio_decode(const char *code, const char *frame, int64_t century, char *buffer, size_t capacity, size_t *written);` | `time-codes` | One minute's frame of a long-wave radio time code read, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_radio_encode(const char *code, int64_t unix_seconds, int leap, const char *summer, int zone_change, int dut1_tenths, uint32_t dst_next, char *buffer, size_t capacity, size_t *written);` | `time-codes` | The frame of a long-wave radio time code for a minute, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_jjy_call_sign_decode(const char *frame, int64_t year, char *buffer, size_t capacity, size_t *written);` | `time-codes` | JJY's call-sign frame of minute 15 or 45 read in a year the caller names, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_jjy_call_sign_encode(int64_t unix_seconds, uint32_t stop_start, int daytime_only, uint32_t stop_span, char *buffer, size_t capacity, size_t *written);` | `time-codes` | JJY's call-sign frame for minute 15 or 45 with a notice of a planned stop, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_irig_decode(const char *signal, const char *frame, int64_t year, char *buffer, size_t capacity, size_t *written);` | `time-codes` | One frame of an IRIG serial time code read, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_irig_encode(const char *signal, int64_t fixed, uint32_t seconds_of_day, uint32_t hundredths, uint32_t control, char *buffer, size_t capacity, size_t *written);` | `time-codes` | The frame of an IRIG serial time code whose reference bit falls at a reading of the civil clock, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_irig_frame_start(const char *signal, uint32_t seconds_of_day, uint32_t hundredths, char *buffer, size_t capacity, size_t *written);` | `time-codes` | The reading at which the frame of an IRIG code that holds a reading of the civil clock begins, with the frame's length, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_irig_formats(char *buffer, size_t capacity, size_t *written);` | `time-codes` | Every IRIG format `hc_irig_decode` and `hc_irig_encode` read, with its frame's length, its rate and its fields, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_describe_day(int64_t fixed, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | One fixed day in every registered calendar, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_day_extras(int64_t fixed, const char *id, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The extra fields of one fixed day, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_calendar_units(const char *id, uint32_t unit, int64_t from_fixed, int64_t to_fixed, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The days from `from_fixed` up to but not including `to_fixed` as one calendar's eras, years, months or days, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_parse_date(const char *calendar, const char *locale, const char *text, char *buffer, size_t capacity, size_t *written);` | `calendars` | A date as a locale writes it in one calendar, read back, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_calendars(int64_t today, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | Every registered calendar, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_calendar_list(const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | Every registered calendar by name alone, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_locales(char *buffer, size_t capacity, size_t *written);` | `calendars` | Every locale the library carries, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_first_day_of_week(const char *locale, uint8_t *out_weekday);` | `calendars` | The ISO 8601 weekday of the first day of the week in a locale, Monday = 1 through Sunday = 7. |
| `HcStatus hc_day_period(uint32_t seconds_of_day, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The day periods of a time of the civil clock in a locale, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_format_number(const char *system, int64_t value, char *buffer, size_t capacity, size_t *written);` | `calendars` | An integer written in a numbering system, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_parse_number(const char *system, const char *text, int64_t *out_value);` | `calendars` | An integer read back out of a numbering system's notation. |
| `HcStatus hc_numbering_systems(char *buffer, size_t capacity, size_t *written);` | `calendars` | Every numbering system `hc_format_number` writes, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_calendar_eras(const char *calendar, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The eras a calendar is described with, named in a locale, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_locale_chain(const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The fallback chain of a locale, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_locale_info(const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | What a locale is, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_plural_category(const char *locale, const char *number, const char *kind, char *buffer, size_t capacity, size_t *written);` | `calendars` | The plural category a number has in a locale, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_names(const char *locale, const char *calendar, const char *width, const char *context, char *buffer, size_t capacity, size_t *written);` | `calendars` | The names a locale has for a calendar in a width and a context, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_case(const char *locale, const char *mode, const char *text, char *buffer, size_t capacity, size_t *written);` | `calendars` | A text recased as a locale cases it, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_isolate(const char *locale, const char *mode, const char *text, char *buffer, size_t capacity, size_t *written);` | `calendars` | A text made safe to embed in text running a locale's direction, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_gregorian_adoption(const char *region, char *buffer, size_t capacity, size_t *written);` | `calendars` | The steps by which a country adopted the Gregorian calendar, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_naming_period_on(const char *calendar, int64_t fixed, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | Which month and weekday names a locale writes for a calendar on a fixed day, where a government renamed them for a period, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_panchanga_at(int64_t unix_seconds, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The yoga and the karaṇa in progress at a POSIX timestamp, as two NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_panchanga_of_day(int64_t fixed, double latitude, double longitude, double elevation, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The yoga and the karaṇa a fixed day carries at a place, the ones in progress at its sunrise, as two NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_tithi_at(int64_t unix_seconds, const char *sky, char *buffer, size_t capacity, size_t *written);` | `calendars` | The tithi in progress at a POSIX timestamp, with the moments it began and ends, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_tithis_of_day(int64_t fixed, double latitude, double longitude, double elevation, const char *sky, char *buffer, size_t capacity, size_t *written);` | `calendars` | The tithis in progress between a fixed day's sunrise at a place and the next, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_ayanamsas(char *buffer, size_t capacity, size_t *written);` | `calendars` | Every named ayanāṃśa, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_ayanamsa_at(int64_t unix_seconds, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | A named ayanāṃśa's value at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ayanamsa_from_anchor(int64_t unix_seconds, double anchor_julian_date, double degrees_at_anchor, char *buffer, size_t capacity, size_t *written);` | `calendars` | The value at a POSIX timestamp of an ayanāṃśa the caller anchors, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_festival_readings(char *buffer, size_t capacity, size_t *written);` | `calendars` | The two readings of a festival's day where the sects part, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_janmashtami(int64_t year, const char *reading, double latitude, double longitude, double elevation, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The day of Kṛṣṇa Janmāṣṭamī in a Gregorian year at a place by a reading, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_vaishnava_day(int64_t saka_year, uint32_t month, uint32_t tithi, double latitude, double longitude, double elevation, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The Vaiṣṇava day of a tithi of a month of a Śaka year at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_vishti_free_span(int64_t saka_year, uint32_t month, uint32_t tithi, double latitude, double longitude, double elevation, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The part of a tithi that Bhadra, the karaṇa Viṣṭi, does not cover, for a tithi of a month of a Śaka year at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_rahu_at(int64_t unix_seconds, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | Rāhu and Ketu at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_rahu_ingresses(int64_t from, int64_t to, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The entries of the mean node into the sidereal signs in a span of POSIX timestamps, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_era_new_year(const char *calendar, int64_t year, int64_t *out_fixed);` | `calendars` | The first day of a year of a historical Indian era over the lunisolar months, written to the out-parameter `out_fixed`. |
| `HcStatus hc_hindu_lunar_date(const char *sky, int64_t fixed, double latitude, double longitude, double elevation, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The Hindu lunisolar date of a fixed day at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_surya_siddhanta_at(int64_t unix_seconds, char *buffer, size_t capacity, size_t *written);` | `calendars` | The *Sūrya Siddhānta*'s Sun and Moon at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_surya_siddhanta_sunrise(int64_t fixed, double latitude, double longitude, char *buffer, size_t capacity, size_t *written);` | `calendars` | The *Sūrya Siddhānta*'s sunrise on a fixed day at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_crescent_visible(const char *criterion, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `calendars` | Whether the young crescent should have been visible on the evening that begins a fixed day, from a place, by a named criterion, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ioc_olympiad(int64_t gregorian_year, int64_t *out_olympiad);` | `calendars` | The number of the modern Olympiad a Gregorian year belongs to. |
| `HcStatus hc_ioc_olympiad_on(int64_t fixed, int64_t *out_olympiad);` | `calendars` | The modern Olympiad a fixed day belongs to, by the Olympic Charter in force on that day. |
| `HcStatus hc_olympic_games(const char *season, char *buffer, size_t capacity, size_t *written);` | `calendars` | The modern Olympic Games of a season, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_era_table(const char *table, char *buffer, size_t capacity, size_t *written);` | `calendars` | Every era of a table, one a line, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_babylonian_regnal_year(int64_t seleucid_year, char *buffer, size_t capacity, size_t *written);` | `calendars` | The king and regnal year labelling a Seleucid year, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_equinox_new_year_margin(const char *calendar, int64_t year, char *buffer, size_t capacity, size_t *written);` | `calendars` | How far the equinox that begins a year of a calendar fell from the moment of the day that decides its new year, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_shmuel_tekufah(int64_t hebrew_year, const char *tekufah, char *buffer, size_t capacity, size_t *written);` | `calendars` | A *tekufah* of Shmuel's reckoning in a Hebrew year, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_day_name(const char *calendar, const char *naming, int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `calendars` | A fixed day's name in a calendar whose days are named, by one of its namings, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_hebrew_yahrzeit(int64_t death_fixed, int64_t hebrew_year, int64_t *out_fixed);` | `calendars` | The fixed day of the yahrzeit in a Hebrew year of a death on the Hebrew date a fixed day names. |
| `HcStatus hc_hebrew_birthday(int64_t birth_fixed, int64_t hebrew_year, int64_t *out_fixed);` | `calendars` | The fixed day of the birthday in a Hebrew year of a birth on the Hebrew date a fixed day names. |
| `HcStatus hc_chinese_reckoned_age(int64_t birth_fixed, int64_t on_fixed, uint32_t *out_age);` | `calendars` | A person's age as the Chinese count reckons it on a fixed day. |
| `HcStatus hc_chinese_marriage_augury(int64_t chinese_year, char *buffer, size_t capacity, size_t *written);` | `calendars` | The marriage augury of a Chinese year, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_chinese_age(const char *convention, int64_t birth_fixed, int64_t on_fixed, uint32_t *out_age);` | `calendars` | A person's age on a fixed day by a named count, born on another. |
| `HcStatus hc_chinese_almanac_solar_terms(int64_t year, char *buffer, size_t capacity, size_t *written);` | `calendars` | The days of the twenty-four solar terms the Qing almanac printed in a Gregorian year, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_hebrew_sabbatical_cycle_year(int64_t hebrew_year, int64_t *out_place);` | `calendars` | The place of a Hebrew year in the seven-year sabbatical cycle, 1 through 7. |
| `HcStatus hc_asian_day(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `calendars` | A fixed day in the calendar of the Roman province of Asia as the calendar writes it, unnumbered days included, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_barhaspatya_year(const char *rule, int64_t saka, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The name of the northern sixty-year cycle a named rule couples with an expired Śaka year, and the name it expunges that year, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_barhaspatya_year_at(const char *rule, int64_t unix_seconds, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The name of the northern sixty-year cycle in progress at a POSIX timestamp by a named rule, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_kalam(const char *convention, int64_t fixed, double latitude, double longitude, double elevation, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | Rāhu kālam, Yamaganda and Gulika kālam on a fixed day, as three NUL-terminated UTF-8 lines, each named in a locale, in a caller-owned buffer. |
| `HcStatus hc_muhurtas(int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `calendars` | The thirty muhūrtas of a fixed day at a place, with Abhijit and Dur Muhurtam marked, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_amrita_siddhi(int64_t fixed, double latitude, double longitude, double elevation, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The *amṛta siddhi yoga* of a fixed day at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_nakshatra_at(int64_t unix_seconds, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The nakṣatra the Moon is in at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_nakshatra_of_day(int64_t fixed, double latitude, double longitude, double elevation, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The nakṣatra a fixed day carries at a place, the one the Moon is in at its sunrise, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_almanac_cycles(int64_t fixed, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `calendars` | The almanac's cycles of a fixed day, 恵方, 三元九運 and 손 없는 날, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_almanac_day(int64_t fixed, const char *meridian, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The almanac's annotations of a fixed day, 干支 to the 選日, as NUL-terminated UTF-8 lines in a caller-owned buffer, one an annotation, each named in a locale. |
| `HcStatus hc_almanac_directions(int64_t fixed, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `calendars` | Where the 八将神 and the 金神 stand in the year in force on a fixed day, as NUL-terminated UTF-8 lines in a caller-owned buffer, one a god and a direction. |
| `HcStatus hc_rounichi(const char *rule, int64_t year, const char *meridian, int64_t *out_fixed);` | `calendars` | The fixed day of 臘日 in the winter that ends in a Gregorian year, by a named reckoning, at a meridian. |
| `HcStatus hc_mansion_undertakings(const char *list, int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `calendars` | What one publisher's list says the 二十八宿 of a fixed day favours and forbids, as NUL-terminated UTF-8 lines in a caller-owned buffer, one an undertaking. |
| `HcStatus hc_almanac_person_days(int64_t fixed, int64_t birth_fixed, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `calendars` | Whether a fixed day is one of a person's own 五墓日 or 三箇の悪日, by the day they were born on, as NUL-terminated UTF-8 lines in a caller-owned buffer, one an entry. |
| `HcStatus hc_tibetan_almanac_day(const char *calendar, int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `calendars` | What the Tibetan almanac of a version prints for a fixed day, the five components and the columns after them, as NUL-terminated UTF-8 lines in a caller-owned buffer, one a column. |
| `HcStatus hc_tibetan_planets(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `calendars` | Where the Phugpa almanac places the five planets at the end of a fixed day, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_bhutanese_winter_solstice(int64_t year, char *buffer, size_t capacity, size_t *written);` | `calendars` | The Bhutanese calendar's winter solstice of a Gregorian year, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_tibetan_festival_day(const char *rule, const char *calendar, int64_t year, uint32_t month, int leap, uint32_t day, int64_t *out_fixed);` | `calendars` | The fixed day a festival on a Tibetan date is kept on, by a named rule for a skipped or repeated number. |
| `HcStatus hc_choghadiya(int64_t fixed, double latitude, double longitude, double elevation, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The sixteen choghadiya of a fixed day at a place, as NUL-terminated UTF-8 lines in a caller-owned buffer, each named in a locale. |
| `HcStatus hc_panchak(const char *naming, int64_t unix_seconds, const char *ayanamsa, int32_t offset_seconds, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The Panchak window in progress at a POSIX timestamp, or the next one, and its kind under a naming table, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_kumbh(const char *yoga, int64_t year, const char *ayanamsa, const char *jupiter, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | When in a Gregorian year the Sun, and the Moon where it is asked for, stand as a condition of the Kumbh Mela requires, and whether Jupiter's sign, which the caller gives, meets it, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_kumbh_yogas(const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | Every condition of the Kumbh Mela, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_pushkaram_rivers(const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | Every river of the Pushkaram, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_pushkaram(const char *sign, int64_t entry_unix_seconds, double latitude, double longitude, double elevation, const char *meridian, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The twelve days of the *Ādi Pushkaram* of each river of a sidereal sign, for Jupiter's entry into it at a POSIX timestamp, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_folk_day(int64_t fixed, const char *meridian, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The folk reckonings of a fixed day outside the Japanese almanac, as NUL-terminated UTF-8 lines in a caller-owned buffer, one a reckoning, each named in a locale. |
| `HcStatus hc_night_watch(uint32_t seconds_of_day, const char *locale, char *buffer, size_t capacity, size_t *written);` | `calendars` | The Chinese night watch and its points of a time of the civil clock by the fixed reckoning, as one NUL-terminated UTF-8 line in a caller-owned buffer, or the empty string by day. |
| `HcStatus hc_solar_new_year(const char *calendar, int64_t year, char *buffer, size_t capacity, size_t *written);` | `calendars` | The day and the moment the year changes at the solar New Year of the Burmese, Khmer or Lao calendar, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_southeast_asian_year_type(const char *calendar, int64_t year, char *buffer, size_t capacity, size_t *written);` | `calendars` | The kind of lunar year a year of the Khmer or the Lao calendar is, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_maya_long_count(int64_t fixed, const char *correlation, char *buffer, size_t capacity, size_t *written);` | `calendars` | A fixed day in the Maya counts under a named correlation constant, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_akan_day(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `calendars` | A fixed day in the Akan *Adaduanan*, the 42-day cycle of the six-day and the seven-day week, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_weton(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `calendars` | A fixed day's *weton*, the Javanese five-day *pasaran* against the seven-day week, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_buddhist_lk_year(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `calendars` | Sri Lanka's Buddhist year of a fixed day, counted from the Vesak Full Moon Poya Day, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_tiruvalluvar_year(int64_t fixed, int64_t *out_year);` | `calendars` | The Tiruvaḷḷuvar year of a fixed day, Tamil Nadu's official count, written to `out_year`. |
| `HcStatus hc_solar_nakshatra_ingresses(int64_t from_unix_seconds, int64_t to_unix_seconds, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `calendars` | The Sun's entries into the nakṣatras within a span, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_japanese_era_year(int64_t year, char *buffer, size_t capacity, size_t *written);` | `calendars` | A year of a Japanese era as the era's dates write it, 元 for the first year and the Han numerals of Japanese for every other, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_locale_format(const char *locale, const char *calendar, char *buffer, size_t capacity, size_t *written);` | `calendars` | The standard date, time and date-time formats a locale carries for a calendar, in CLDR's four lengths, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_plural_categories(const char *locale, const char *kind, char *buffer, size_t capacity, size_t *written);` | `calendars` | The plural categories a locale's cardinal or ordinal rules name, each with a number that falls in it, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_term_in_effect(int64_t fixed, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `seasons` | The solar term in effect on a fixed day at a meridian, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_pentad_in_effect(int64_t fixed, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `seasons` | The pentad (候) in effect on a fixed day at a meridian, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_pentad_traditions(char *buffer, size_t capacity, size_t *written);` | `seasons` | Every tradition that names the 72 pentads (候), as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_pentad_in_tradition(int64_t fixed, const char *tradition, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `seasons` | The pentad (候) in effect on a fixed day at a meridian, named by a tradition, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_zassetsu_in_year(int64_t year, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `seasons` | The 雑節 of a Gregorian year at a meridian, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_seasonal_days_in_year(int64_t year, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `seasons` | The other seasonal days and spans of a Gregorian year, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_cold_food_day(const char *convention, int64_t year, int64_t *out_fixed);` | `seasons` | The fixed day of 寒食, the Cold Food Day, of a Gregorian year under a named reckoning. |
| `HcStatus hc_plum_rains(const char *rule, int64_t year, const char *meridian, int64_t *out_fixed);` | `seasons` | The fixed day of 入梅 or 出梅 of a Gregorian year by a named rule of the Chinese almanac, with the solar term it counts from at a meridian. |
| `HcStatus hc_pentads_in_year(int64_t year, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `seasons` | Every pentad (候) that begins in a Gregorian year at a meridian, named by every tradition at once, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_tropical_signs_in_year(int64_t year, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `seasons` | The twelve tropical sign periods of a Gregorian year at a meridian, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_sidereal_signs_in_year(int64_t year, const char *ayanamsa, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `seasons` | The twelve sidereal sign periods, the saṅkrāntis, of a Gregorian year in the zodiac of a named ayanāṃśa at a meridian, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_traditional_tanabata(int64_t year, const char *meridian, int64_t *out_fixed);` | `seasons` | The fixed day of 伝統的七夕, the National Astronomical Observatory's traditional Tanabata, of a Gregorian year at a meridian. |
| `HcStatus hc_principal_phases_in_month(int64_t year, uint32_t month, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `seasons` | The principal phases of the Moon that fall inside a Gregorian month at a meridian, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_holiday_is_day_off(const char *code, const char *region, const char *group, int64_t fixed, int *out_is_day_off);` | `holiday` | Whether a fixed day is a day off in a holiday table. |
| `HcStatus hc_holiday_add_business_days(const char *code, const char *region, const char *group, int64_t fixed, int64_t count, int64_t *out_fixed);` | `holiday` | A fixed day moved by a number of business days of a holiday table, in a subdivision and for a group. |
| `HcStatus hc_holiday_business_days_between(const char *code, const char *region, const char *group, int64_t from_fixed, int64_t to_fixed, int64_t *out_count);` | `holiday` | The number of business days of a holiday table, in a subdivision and for a group, from one fixed day up to but not including another. |
| `HcStatus hc_holiday_is_weekend(const char *code, const char *region, int64_t fixed, int *out_is_weekend);` | `holiday` | Whether a fixed day is a weekend day in a holiday table, in a subdivision or nationwide. |
| `HcStatus hc_holiday_next(const char *code, const char *region, const char *group, const char *kind, int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | The first holiday of a table after a fixed day, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_holiday_previous(const char *code, const char *region, const char *group, const char *kind, int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | The last holiday of a table before a fixed day, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_holidays_in_year(const char *code, const char *region, const char *group, const char *kind, int64_t year, char *buffer, size_t capacity, size_t *written);` | `holiday` | The holidays of a Gregorian year in a table, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_holiday_codes(char *buffer, size_t capacity, size_t *written);` | `holiday` | The identifier of every holiday table, one per line, NUL-terminated. |
| `HcStatus hc_holidays_on(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | Every holiday on one fixed day across every table, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_holiday_tables(const char *locale, char *buffer, size_t capacity, size_t *written);` | `holiday` | Every holiday table with its kind, names and sources, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_holiday_groups(const char *locale, char *buffer, size_t capacity, size_t *written);` | `holiday` | Every group of people a holiday may be given to alone, named in a locale, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_holiday_coverage(const char *code, char *buffer, size_t capacity, size_t *written);` | `holiday` | The years a holiday table answers for, nationwide and in each subdivision it answers for, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_holiday_rules(const char *code, char *buffer, size_t capacity, size_t *written);` | `holiday` | The rules of a holiday table, one line each, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_holidays_on_in(int64_t fixed, const char *locale, char *buffer, size_t capacity, size_t *written);` | `holiday` | `hc_holidays_on`'s lines, each with the day's name in a locale and the tag that named it, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_lectionary(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | The lectionary cycles of a fixed day, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_astronomical_easter(int64_t year, int64_t *out_fixed);` | `holiday` | The fixed day of Easter Sunday of a Gregorian year by the astronomical reckoning at the meridian of Jerusalem. |
| `HcStatus hc_astronomical_paschal_full_moon(int64_t year, int64_t *out_fixed);` | `holiday` | The fixed day of the paschal full moon of a Gregorian year by the astronomical reckoning at the meridian of Jerusalem. |
| `HcStatus hc_holy_year_on(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | The Holy Year of the Catholic Church a fixed day falls in, if any, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_common_worship_on(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | The rank of every *Common Worship* celebration kept on a fixed day, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_roman_1960_office_on(int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | What the Roman calendar of the 1960 rubrics does on a fixed day, the office kept, its commemorations and the days transferred or omitted, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_orthodox_fast_on(const char *reckoning, int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `holiday` | What a fixed day is in the fasting scheme of a reckoning, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_orthodox_fast_seasons(const char *reckoning, int64_t year, char *buffer, size_t capacity, size_t *written);` | `holiday` | The fasting seasons and fast-free weeks of a year of a reckoning, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_place_years_ago(double years_ago, double std_dev_years, const char *locale, char *buffer, size_t capacity, size_t *written);` | `deep-time` | A moment some years before the present, placed in every chronology at once, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_cosmic_events(const char *locale, char *buffer, size_t capacity, size_t *written);` | `deep-time` | Every cosmic epoch and every dated cosmic event, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_earliest_evidence(const char *locale, char *buffer, size_t capacity, size_t *written);` | `deep-time` | Every claim to the earliest evidence of life, of *Homo sapiens* and of writing, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_archaeological_periods(const char *locale, char *buffer, size_t capacity, size_t *written);` | `deep-time` | Every conventional archaeological period, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_geologic_intervals(uint32_t rank, const char *locale, char *buffer, size_t capacity, size_t *written);` | `deep-time` | Every interval of one rank of the geologic time scale, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_future_events(const char *locale, char *buffer, size_t capacity, size_t *written);` | `deep-time` | Every dated event of the far future, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_planck_units(char *buffer, size_t capacity, size_t *written);` | `deep-time` | The CODATA constants the Planck units are built from, and the Planck units, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_bp_convert(double years, double std_dev_years, const char *from, const char *to, char *buffer, size_t capacity, size_t *written);` | `deep-time` | A calendar age or year in one datum written in another, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_deep_convert(double value, double std_dev, const char *from, const char *to, char *buffer, size_t capacity, size_t *written);` | `deep-time` | A magnitude of time in one unit written in another, with its uncertainty carried through, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_deep_compare(double first_value, double first_std_dev, const char *first_unit, double second_value, double second_std_dev, const char *second_unit, char *buffer, size_t capacity, size_t *written);` | `deep-time` | Two magnitudes of time compared across the decades between them, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_zone_load(const char *name, const uint8_t *tzif, size_t tzif_len);` | `tz` | Give the library a zone's TZif data under an IANA name. |
| `HcStatus hc_fixed_from_unix_in_zone(int64_t unix_seconds, const char *zone, int64_t *out_fixed);` | `tz` | The fixed day a POSIX timestamp falls on by the wall clock of a zone. |
| `HcStatus hc_unix_from_fixed_in_zone(int64_t fixed, const char *zone, int64_t *out_unix_seconds);` | `tz` | The POSIX timestamp at which a fixed day begins by the wall clock of a zone. |
| `HcStatus hc_zone_offset(const char *zone, int64_t unix_seconds, char *buffer, size_t capacity, size_t *written);` | `tz` | The offset a zone keeps at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_zones(const char *locale, char *buffer, size_t capacity, size_t *written);` | `tz` | Every zone of the IANA database's `zone1970.tab` with its principal location, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_zone_location(const char *zone, const char *locale, char *buffer, size_t capacity, size_t *written);` | `tz` | Where one zone is, as the NUL-terminated UTF-8 line `hc_zones` writes for it, in a caller-owned buffer. |
| `HcStatus hc_localtime(int64_t unix_seconds, const char *zone, char *buffer, size_t capacity, size_t *written);` | `tz` | Python's `time.localtime(seconds)` in a zone: the wall-clock reading of a POSIX second as the nine fields of a `struct_time`, one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_mktime(int64_t year, int64_t month, int64_t day, int64_t hour, int64_t minute, int64_t second, const char *zone, const char *policy, int64_t *out_unix_seconds);` | `tz` | Python's `time.mktime(tuple)` in a zone: the POSIX second of a wall-clock reading given as its year, month, day, hour, minute and second, written to `out_unix_seconds`. |
| `HcStatus hc_local_resolution(int64_t year, int64_t month, int64_t day, int64_t hour, int64_t minute, int64_t second, const char *zone, char *buffer, size_t capacity, size_t *written);` | `tz` | What a wall-clock reading means in a zone before a policy reduces it to one instant, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_mktime_policies(char *buffer, size_t capacity, size_t *written);` | `tz` | The policies `hc_mktime` reads for a reading two instants name or none names, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_earth_rotation_angle(double ut1_unix_seconds, double *out_degrees);` | `sky` | The Earth Rotation Angle at a UT1 instant, in degrees, 0 to 360. |
| `HcStatus hc_gmst_iau2006(double ut1_unix_seconds, double *out_degrees);` | `sky` | The Greenwich mean sidereal time by the IAU 2006 convention at a UT1 instant, in degrees, 0 to 360. |
| `HcStatus hc_gmst_iau1982(double ut1_unix_seconds, double *out_degrees);` | `sky` | The Greenwich mean sidereal time by the IAU 1982 convention at a UT1 instant, in degrees, 0 to 360. |
| `HcStatus hc_ut2_minus_ut1(double ut1_unix_seconds, double *out_seconds);` | `sky` | UT2 − UT1 at a UT1 instant, in seconds. |
| `HcStatus hc_sky_at(int64_t unix_seconds, char *buffer, size_t capacity, size_t *written);` | `sky` | The Sun and the Moon at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_solar_terms_between(int64_t from_unix, int64_t to_unix, char *buffer, size_t capacity, size_t *written);` | `sky` | Every solar term whose instant falls in `[from_unix, to_unix)`, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_moon_phases_between(int64_t from_unix, int64_t to_unix, char *buffer, size_t capacity, size_t *written);` | `sky` | Every new moon, first quarter, full moon and last quarter whose instant falls in `[from_unix, to_unix)`, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_decan_at(int64_t unix_seconds, char *buffer, size_t capacity, size_t *written);` | `sky` | The decan the Sun is in at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_drekkana_at(int64_t unix_seconds, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `sky` | The drekkāṇa, the Hindu third of a sidereal sign, the Sun is in at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_horizons(const char *locale, char *buffer, size_t capacity, size_t *written);` | `sky` | Every named horizon a rising or a setting can be measured against, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_sunrise(const char *horizon, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | Sunrise on a fixed day at a place against a named horizon, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_sunset(const char *horizon, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | Sunset on a fixed day at a place against a named horizon, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_solar_time(const char *clock, int64_t unix_seconds, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | A local clock's reading at a POSIX timestamp and a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_solar_event(const char *event, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | A named time of day on a fixed day at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_hjd_tt(double tt_julian_date, double right_ascension, double declination, char *buffer, size_t capacity, size_t *written);` | `sky` | The Heliocentric Julian Date in TT, HJD_TT, of a Julian Date of TT for a target, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_hjd_utc(double utc_julian_date, double right_ascension, double declination, int strict, char *buffer, size_t capacity, size_t *written);` | `sky` | The Heliocentric Julian Date in UTC, HJD_UTC, of a Julian Date of UTC for a target, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_gmat_from_gmt(int64_t fixed, uint32_t seconds_of_day, uint64_t attoseconds, char *buffer, size_t capacity, size_t *written);` | `sky` | The astronomical date and the Greenwich Mean Astronomical Time of a reading of GMT, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_gmt_from_gmat(int64_t fixed, uint32_t seconds_of_day, uint64_t attoseconds, char *buffer, size_t capacity, size_t *written);` | `sky` | The civil date and the GMT of a reading of Greenwich Mean Astronomical Time, the inverse of `hc_gmat_from_gmt`, as one NUL-terminated UTF-8 line in its columns in a caller-owned buffer. |
| `HcStatus hc_prayer_times(const char *method, int64_t fixed, double latitude, double longitude, double elevation, int ramadan, char *buffer, size_t capacity, size_t *written);` | `sky` | The Islamic prayer times of a fixed day at a place by a named method, as eight NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_prayer_methods(char *buffer, size_t capacity, size_t *written);` | `sky` | Every prayer-time method `hc_prayer_times` reads, with its parameters and source, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_zmanim(const char *reckoning, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | The Jewish times of a fixed day at a place by a reckoning, with the dawns and nightfalls, as nine NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_temporal_hour(const char *reckoning, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | The length of a temporal hour of a fixed day at a place by a reckoning of the Jewish day, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_edo_time(int64_t unix_seconds, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | The Edo 不定時法 reading of a POSIX timestamp at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_unix_from_edo_time(int64_t fixed, uint32_t hour, double fraction, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | The instant of an Edo 不定時法 reading at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_planetary_hour(int64_t unix_seconds, double latitude, double longitude, double elevation, const char *locale, char *buffer, size_t capacity, size_t *written);` | `sky` | The planetary hour at a POSIX timestamp and a place, as one NUL-terminated UTF-8 line, its ruler named in a locale, in a caller-owned buffer. |
| `HcStatus hc_planetary_hours_of_day(int64_t fixed, double latitude, double longitude, double elevation, const char *locale, char *buffer, size_t capacity, size_t *written);` | `sky` | The twenty-four planetary hours of the planetary day that begins at the sunrise of a fixed day at a place, as NUL-terminated UTF-8 lines in `hc_planetary_hour`'s columns, each ruler named in a locale, in a caller-owned buffer. |
| `HcStatus hc_moonrise(const char *horizon, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | Moonrise on a local day at a place against a named horizon, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_moonset(const char *horizon, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | Moonset on a local day at a place against a named horizon, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ut1r_iers2010(double ut1_unix_seconds, char *buffer, size_t capacity, size_t *written);` | `sky` | UT1R at a UT1 instant by the IERS 2010 zonal tide model, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ut1s_iers2010(double ut1_unix_seconds, char *buffer, size_t capacity, size_t *written);` | `sky` | UT1S at a UT1 instant by the IERS 2010 zonal tide model, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_zonal_tide_ut1_effect(double ut1_unix_seconds, double period_limit_days, char *buffer, size_t capacity, size_t *written);` | `sky` | The effect on UT1 of the zonal tides whose period is under a limit, at a UT1 instant, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_equation_of_time(int64_t unix_seconds, char *buffer, size_t capacity, size_t *written);` | `sky` | The equation of time at a POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_solar_noon(int64_t fixed, double latitude, double longitude, double elevation, int64_t *out_unix_seconds);` | `sky` | Apparent solar noon on a local day at a place, as POSIX seconds of Universal Time, written to `out_unix_seconds`. |
| `HcStatus hc_solar_midnight(int64_t fixed, double latitude, double longitude, double elevation, int64_t *out_unix_seconds);` | `sky` | Apparent solar midnight opening a local day at a place, as POSIX seconds of Universal Time, written to `out_unix_seconds`. |
| `HcStatus hc_dawn(const char *twilight, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | The start of a named twilight on a local day at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_dusk(const char *twilight, int64_t fixed, double latitude, double longitude, double elevation, char *buffer, size_t capacity, size_t *written);` | `sky` | The end of a named twilight on a local day at a place, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_orbit_at(double years_before_1950, char *buffer, size_t capacity, size_t *written);` | `orbital` | Earth's orbital elements and the June insolation at 65° N at an epoch, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_orbit_series(double from_years_before_1950, double to_years_before_1950, double step_years, char *buffer, size_t capacity, size_t *written);` | `orbital` | The line of `hc_orbit_at` at every epoch from `from_years_before_1950` to `to_years_before_1950` in steps of `step_years`, each with the epoch as a first column, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_daily_insolation(double years_before_present, double latitude_degrees, double solar_longitude_degrees, char *buffer, size_t capacity, size_t *written);` | `orbital` | The daily mean insolation at any latitude and solar longitude, for the orbit of an epoch, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_jupiter_at(int64_t unix_seconds, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `jupiter` | Where Jupiter is at a POSIX timestamp, tropical and sidereal, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_jupiter_ingresses(int64_t from_unix_seconds, int64_t to_unix_seconds, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `jupiter` | Jupiter's crossings of the boundaries of the sidereal signs in a span of POSIX seconds, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_jupiter_risings(int64_t from_unix_seconds, int64_t to_unix_seconds, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `jupiter` | Jupiter's heliacal risings in a span of POSIX seconds, each with the name a year of Jupiter has from it, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_kumbh_by_sky(const char *yoga, int64_t year, const char *ayanamsa, const char *locale, char *buffer, size_t capacity, size_t *written);` | `jupiter` | When in a Gregorian year the Sun, and the Moon where it is asked for, stand as a condition of the Kumbh Mela requires, and whether Jupiter, whose sign is computed, meets it, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_kumbhs_in_year_by_sky(int64_t year, const char *ayanamsa, const char *locale, char *buffer, size_t capacity, size_t *written);` | `jupiter` | Every condition of the Kumbh Mela that a Gregorian year's sky meets or does not, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_jupiter_stations(int64_t from_unix_seconds, int64_t to_unix_seconds, const char *ayanamsa, char *buffer, size_t capacity, size_t *written);` | `jupiter` | Jupiter's stations in a span of POSIX seconds, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_pushkaram_by_sky(const char *sign, int64_t year, const char *ayanamsa, const char *rule, double latitude, double longitude, double elevation, const char *meridian, const char *locale, char *buffer, size_t capacity, size_t *written);` | `jupiter` | The twelve days of the *Ādi Pushkaram* of each river of a sidereal sign, for Jupiter's entry into it in a Gregorian year, found, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_pushkarams_in_year(int64_t year, const char *ayanamsa, const char *rule, double latitude, double longitude, double elevation, const char *meridian, const char *locale, char *buffer, size_t capacity, size_t *written);` | `jupiter` | The twelve days of the *Ādi Pushkaram* of each river of every sidereal sign Jupiter enters in a Gregorian year, found, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_pushkaram_rules(char *buffer, size_t capacity, size_t *written);` | `jupiter` | The rules for which entry of Jupiter into a sign a Pushkaram follows, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_mars_time(double unix_seconds, double east_longitude_degrees, char *buffer, size_t capacity, size_t *written);` | `planetary` | Mars at a POSIX instant and an east longitude, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_missions(char *buffer, size_t capacity, size_t *written);` | `planetary` | Every surface mission on Mars and the rules of its sol count, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_mission_sol(const char *mission, double unix_seconds, int64_t *out_sol);` | `planetary` | The sol number of a Mars surface mission at a POSIX instant, by the mission's own clock. |
| `HcStatus hc_bodies(char *buffer, size_t capacity, size_t *written);` | `planetary` | Every body `hc-planetary` carries, with its solar day, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_body_time(const char *body, double unix_seconds, double east_longitude_degrees, char *buffer, size_t capacity, size_t *written);` | `planetary` | Local mean solar time on a body at a POSIX instant and an east longitude, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_circad_date(const char *calendar, double unix_seconds, char *buffer, size_t capacity, size_t *written);` | `planetary` | The date at a POSIX instant in a calendar of another body's days, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_proper_time(double speed_metres_per_second, double coordinate_seconds, char *buffer, size_t capacity, size_t *written);` | `relativity` | A clock moving at a constant speed while some coordinate time passes, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_gravitational_dilation(const char *body, double radius_metres, char *buffer, size_t capacity, size_t *written);` | `relativity` | A clock held still at a radius from a body's centre, against one far from every mass, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_gravitating_bodies(char *buffer, size_t capacity, size_t *written);` | `relativity` | Every body `hc-relativity` carries a gravitational parameter for, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_orbit_rate_offset(const char *body, double orbit_radius_metres, double ground_radius_metres, char *buffer, size_t capacity, size_t *written);` | `relativity` | A clock on a circular orbit against one held still on the ground, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_rocket(double proper_acceleration, double proper_seconds, char *buffer, size_t capacity, size_t *written);` | `relativity` | A rocket of constant proper acceleration burning from rest, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_flip_and_burn(double proper_acceleration, double distance_metres, char *buffer, size_t capacity, size_t *written);` | `relativity` | A flip-and-burn voyage between two points at rest, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_doppler(double beta, double cos_theta, char *buffer, size_t capacity, size_t *written);` | `relativity` | The relativistic Doppler shift of a source moving at β, seen at an angle, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_velocity_add(double first_beta, double second_beta, char *buffer, size_t capacity, size_t *written);` | `relativity` | The composition of two collinear velocities, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_schwarzschild_radius(const char *body, char *buffer, size_t capacity, size_t *written);` | `relativity` | The Schwarzschild radius of a body, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_proper_time_uncertain(double speed_metres_per_second, double speed_std_dev, double coordinate_seconds, char *buffer, size_t capacity, size_t *written);` | `relativity` | A clock moving at a constant speed that is not exactly known, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_territories(const char *locale, char *buffer, size_t capacity, size_t *written);` | `places` | Every territory CLDR 48 names, with its name in a locale, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_subdivisions(const char *country, const char *locale, char *buffer, size_t capacity, size_t *written);` | `places` | The ISO 3166-2 subdivisions of a country CLDR 48 names, with their names in a locale, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_place_name(const char *code, const char *locale, char *buffer, size_t capacity, size_t *written);` | `places` | One territory or subdivision, as the NUL-terminated UTF-8 line `hc_territories` or `hc_subdivisions` writes for it, in a caller-owned buffer. |
| `HcStatus hc_relative_time(int64_t then_unix, int64_t now_unix, const char *style, int automatic, const char *locale, char *buffer, size_t capacity, size_t *written);` | `humanize` | How one POSIX instant reads from another, *3 hours ago* or *in 2 days*, in a locale, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_relative_day(int64_t then_fixed, int64_t now_fixed, const char *style, int automatic, const char *locale, char *buffer, size_t capacity, size_t *written);` | `humanize` | Which calendar day one fixed day is, seen from another, *yesterday* or *3 days ago*, in a locale, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_relative_day_at(int64_t then_fixed, int64_t now_fixed, uint32_t seconds_of_day, const char *style, int automatic, const char *locale, char *buffer, size_t capacity, size_t *written);` | `humanize` | Which calendar day one fixed day is, seen from another, with a time of day, *yesterday at 15:05*, in a locale, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_duration(int64_t seconds, const char *style, uint32_t max_components, const char *locale, char *buffer, size_t capacity, size_t *written);` | `humanize` | A span of seconds phrased in days, hours, minutes and seconds, *2 hours and 30 minutes*, in a locale, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_unit_choice(int64_t seconds, const char *thresholds, const char *rounding, char *buffer, size_t capacity, size_t *written);` | `humanize` | The unit a span is said in and its count, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_relative_time_with(int64_t then_unix, int64_t now_unix, const char *style, int automatic, const char *locale, const char *thresholds, const char *rounding, char *buffer, size_t capacity, size_t *written);` | `humanize` | How one POSIX instant reads from another under a threshold table and a rounding of the caller's, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_approximate_duration(int64_t seconds, const char *style, const char *locale, const char *thresholds, const char *policy, char *buffer, size_t capacity, size_t *written);` | `humanize` | A span hedged as a round number, *about 3 hours*, *just over a week*, *nearly a year*, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_list_forms(const char *style, const char *locale, char *buffer, size_t capacity, size_t *written);` | `humanize` | The CLDR list patterns a style joins the parts of a duration with, in a locale, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_apnumber(int64_t value, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | A whole number as the Associated Press writes it, *zero* to *nine* spelled out and every other number as its digits, in a locale, by Python's `humanize` and its catalogues, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_fractional(double value, char *buffer, size_t capacity, size_t *written);` | `natural` | A number as a fraction, *3/10*, *1 3/10*, in the English of Python's `humanize`, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_scientific(double value, uint32_t precision, char *buffer, size_t capacity, size_t *written);` | `natural` | A number in scientific notation, *3.00 x 10⁻¹*, in the English of Python's `humanize`, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_metric(double value, const char *unit, uint32_t precision, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | A number with an SI prefix and a unit, *1.50 kV*, *220 μF*, by Python's `humanize`, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_naturalsize(double value, const char *style, uint32_t decimals, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | A size in bytes, *3.0 MB*, *2.9 KiB*, *300B*, in a locale, by Python's `humanize` and its catalogues, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_naturallist(const char *items, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | Items joined as a list, *one, two and three*, by Python's `humanize`, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_intword(const char *digits, uint32_t decimals, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | An integer of any length as a count with a word, *12.4 thousand*, *1.0 googol*, in a locale, by Python's `humanize` and its catalogues, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_naturaldelta(int64_t seconds, int microseconds, int months, const char *minimum_unit, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | `humanize`'s `naturaldelta` of a span, *3 hours*, *a moment*, *1 year, 3 months*, without tense, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_naturaltime(int64_t seconds, int microseconds, int months, const char *minimum_unit, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | `humanize`'s `naturaltime` of a span, *3 hours ago*, *3 hours from now*, *now*, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_precisedelta(int64_t seconds, int microseconds, const char *minimum_unit, const char *suppress, uint32_t decimals, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | `humanize`'s `precisedelta` of a span, *1 year, 2 months and 3 days*, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_naturalday(int64_t day, int64_t today, const char *pattern, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | `humanize`'s `naturalday` of a fixed day seen from another, *today*, *tomorrow*, *yesterday*, or the day by a `strftime` pattern, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_naturaldate(int64_t day, int64_t today, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | `humanize`'s `naturaldate` of a fixed day seen from another, as `hc_naturalday` with `%b %d`, and with the year added from five twelfths of a year away, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_ordinal(int64_t value, const char *gender, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | `humanize`'s `ordinal` of an integer, *1st*, *2nd*, *103rd*, *111th*, in a locale, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_intcomma(const char *digits, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | `humanize`'s `intcomma` of an integer written in digits, *1,234,567*, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_intcomma_float(double value, int ndigits, const char *locale, char *buffer, size_t capacity, size_t *written);` | `natural` | `humanize`'s `intcomma` of a float, *1,234,567.25*, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_clamp(double value, const char *format, const char *floor, const char *ceil, const char *floor_token, const char *ceil_token, char *buffer, size_t capacity, size_t *written);` | `natural` | A number held within a floor and a ceiling and written with a format, by Python's `humanize` `clamp`, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_parse_datetime(const char *syntax, const char *text, char *buffer, size_t capacity, size_t *written);` | `datetime` | A date-time read in a syntax, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_format_datetime(const char *syntax, int64_t unix_seconds, uint64_t attoseconds, int offset_seconds, const char *precision, char *buffer, size_t capacity, size_t *written);` | `datetime` | An instant written as a date-time in a syntax, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_format_iso_date_as(int64_t fixed, const char *form, const char *style, char *buffer, size_t capacity, size_t *written);` | `datetime` | A fixed day written as an ISO 8601 calendar, ordinal or week date, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_iso_date_parts(const char *text, char *buffer, size_t capacity, size_t *written);` | `datetime` | An ISO 8601 date read into its parts, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_iso_duration(const char *text, char *buffer, size_t capacity, size_t *written);` | `datetime` | An ISO 8601 duration read into its components, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_format_iso_duration(int negative, int64_t years, int64_t months, int64_t weeks, int64_t days, int64_t hours, int64_t minutes, int64_t seconds, const char *fraction, char *buffer, size_t capacity, size_t *written);` | `datetime` | A duration written from its components, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_iso_interval(const char *text, char *buffer, size_t capacity, size_t *written);` | `datetime` | An ISO 8601 interval, or a repeating one, read into its ends, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_parse_pattern(const char *syntax, const char *pattern, const char *text, char *buffer, size_t capacity, size_t *written);` | `patterns` | A text read against a pattern, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_parse_pattern_in(const char *syntax, const char *pattern, const char *text, const char *locale, char *buffer, size_t capacity, size_t *written);` | `patterns` | A text read against a `strptime` or CLDR pattern in the names of a locale, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_zone_name(const char *zone, int64_t unix_seconds, const char *locale, const char *field, char *buffer, size_t capacity, size_t *written);` | `zone-names` | A time zone's name at a POSIX timestamp in a locale, as a CLDR pattern field writes it, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_format_pattern(const char *zone, int64_t unix_seconds, const char *locale, const char *syntax, const char *pattern, char *buffer, size_t capacity, size_t *written);` | `zone-names` | An instant formatted in a time zone and a locale by a CLDR or a `strftime` pattern, as one NUL-terminated UTF-8 line in a caller-owned buffer. |
| `HcStatus hc_edtf_parse(const char *text, char *buffer, size_t capacity, size_t *written);` | `uncertainty` | An ISO 8601-2 value placed on the timeline, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_edtf_relations(const char *first, const char *second, char *buffer, size_t capacity, size_t *written);` | `uncertainty` | What can hold between two EDTF values placed on the timeline, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_significant(double value, uint32_t figures, char *buffer, size_t capacity, size_t *written);` | `uncertainty` | A number with a count of significant figures, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_significant_op(const char *operation, double first, uint32_t first_figures, double second, uint32_t second_figures, char *buffer, size_t capacity, size_t *written);` | `uncertainty` | Arithmetic on two numbers with figure counts, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_uncertain(double value, double std_dev, char *buffer, size_t capacity, size_t *written);` | `uncertainty` | A Gaussian quantity, `value ± σ`, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_uncertain_op(const char *operation, double first, double first_std_dev, double second, double second_std_dev, char *buffer, size_t capacity, size_t *written);` | `uncertainty` | Arithmetic on Gaussian quantities, with the errors propagated to first order, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_interval(const char *operation, int64_t first_low_seconds, int64_t first_high_seconds, int64_t second_low_seconds, int64_t second_high_seconds, char *buffer, size_t capacity, size_t *written);` | `uncertainty` | Arithmetic on intervals of time, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_units(char *buffer, size_t capacity, size_t *written);` | `units` | Every unit of time with an exactly defined length, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_unit_convert(int64_t count_numerator, int64_t count_denominator, const char *from, const char *to, char *buffer, size_t capacity, size_t *written);` | `units` | A count of one unit of time written in another, exactly, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_rates(char *buffer, size_t capacity, size_t *written);` | `units` | Every frame rate and sample rate the crate carries as an exact period, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_frame_period(const char *rate, const char *in_unit, char *buffer, size_t capacity, size_t *written);` | `units` | The length of one frame or one sample, exactly, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_tempo(int64_t bpm_numerator, int64_t bpm_denominator, uint32_t note_halvings, uint32_t dots, uint32_t tuplet_space, uint32_t tuplet_count, uint32_t beat_halvings, char *buffer, size_t capacity, size_t *written);` | `units` | A note at a tempo, exactly, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_fiscal_profiles(char *buffer, size_t capacity, size_t *written);` | `fiscal` | Every fiscal, tax and academic year system the crate carries, country by country, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_fiscal_year_on(const char *country, const char *kind, int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `fiscal` | What the year systems of a country say a fixed day is, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_fiscal_year_span(const char *country, const char *kind, int64_t label, char *buffer, size_t capacity, size_t *written);` | `fiscal` | The span of the year a label names in each year system of a country, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_week_year_systems(char *buffer, size_t capacity, size_t *written);` | `fiscal` | Every named year of whole weeks, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_week_year_on(const char *system, int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `fiscal` | Where a fixed day is in a year of whole weeks, as NUL-terminated UTF-8 one line in a caller-owned buffer. |
| `HcStatus hc_name_day_lists(char *buffer, size_t capacity, size_t *written);` | `name-days` | Every name-day list the crate ships and every country it declines to ship one for, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_name_days_on(const char *country, int64_t fixed, char *buffer, size_t capacity, size_t *written);` | `name-days` | What the lists of a country name on a day, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_name_day(const char *country, const char *given_name, int64_t year, char *buffer, size_t capacity, size_t *written);` | `name-days` | The days of a year on which the lists of a country give a name, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_attribution_authorities(const char *subject, char *buffer, size_t capacity, size_t *written);` | `attributes` | Every attribution list the crate ships, with what it declines to ship, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_attributions(const char *subject, int64_t key, char *buffer, size_t capacity, size_t *written);` | `attributes` | What every list of a subject attributes to one key, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_attributions_on(int64_t fixed, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `attributes` | What every list attributes to the month, the weekday and the sign of a day, as NUL-terminated UTF-8 lines in a caller-owned buffer. |
| `HcStatus hc_harvest_moon(int64_t year, const char *meridian, char *buffer, size_t capacity, size_t *written);` | `attributes` | The Harvest Moon of a year, as NUL-terminated UTF-8 one line in a caller-owned buffer. |

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
from it. `hc_epochs(buffer, capacity, written)` writes the module's lines of
every epoch `hc-core` carries, with its TAI reading as whole seconds and
attoseconds and the document that defines it. `hc_tt_bipm(series, tai_seconds, attoseconds, strict, buffer,
capacity, written)` writes TT(BIPM) at a TAI instant from a realisation
the caller supplies as NUL-terminated text, one line per sample of the
Modified Julian Date and TT(BIPMxx) − TAI − 32.184 s in microseconds: the
offset from TT(TAI), TT(BIPMxx) − TAI and the TT(BIPMxx) reading, never
extrapolated past the series.
`hc_tai_from_unix` and `hc_utc_from_tai` are in `civil` here;
their WebAssembly twins, lines of the same values, are in the module's
`timestamps`, to keep its `civil` small.

The CCSDS and radio time codes are in `time-codes`, a layer of their own,
and .NET's ticks and the six-hour clocks in `timestamps`; each writes the
module's line, and its README's sections say what every cell holds.
`hc_ccsds_decode(hex, strict, buffer, capacity, written)` reads a binary
CCSDS code, CUC, CDS or CCS, as hexadecimal text, P-field first, into the
code's name, its TAI instant and its UTC label, the other scale from the
leap-second table; `hc_ccsds_encode(tai_seconds, attoseconds, p_field,
strict, buffer, capacity, written)` writes the code of a TAI instant in
the format a P-field names; `hc_ccsds_ascii_parse(code, strict, buffer,
capacity, written)` and `hc_ccsds_ascii_format(tai_seconds, attoseconds,
variation, precision, terminator, strict, buffer, capacity, written)` read
and write the ASCII codes A and B. A Level 2, 3 or 4 code, whose epoch or
content only its agency knows, is `HC_ERROR_NO_DATA`; a Level 2 code's
epoch can instead come from the caller, and
`hc_ccsds_decode_from_epoch(hex, epoch_tai_seconds, epoch_attoseconds,
epoch_unix_day, strict, buffer, capacity, written)` and
`hc_ccsds_encode_from_epoch(tai_seconds, attoseconds, p_field,
epoch_tai_seconds, epoch_attoseconds, epoch_unix_day, strict, buffer,
capacity, written)` read and write it from a CUC epoch instant and a CDS
epoch day, as the module's README says.
`hc_radio_decode(code, frame, century, buffer, capacity, written)` reads
one minute's frame of `jjy`, `dcf77`, `wwvb-am` or `wwvb-pm` from a
string of `0`, `1` and `M`, and `hc_radio_encode(code, unix_seconds, leap,
summer, zone_change, dut1_tenths, dst_next, buffer, capacity, written)`
writes one; a null `summer` is the empty string `jjy` takes. In a library
built with `tz` too, a `summer` of `zone:` and a zone's name,
`zone:Europe/Berlin` or `zone:America/Denver` (or a zone the built-in
table lacks once `hc_zone_load` has its TZif file), reads the
state — and
DCF77's A1, in place of `zone_change`, and the phase code's `dst_next`,
in place of the caller's — from the rules
`hc_fixed_from_unix_in_zone` reads for that name, as the WebAssembly
module's README explains under "Radio time codes"; without `tz` it is
`HC_ERROR_UNKNOWN`. `hc_jjy_call_sign_decode(frame, year, buffer,
capacity, written)` reads JJY's call-sign frame of minute 15 or 45 in a
year the caller names, with its notice of a planned stop, and
`hc_jjy_call_sign_encode(unix_seconds, stop_start, daytime_only,
stop_span, buffer, capacity, written)` writes one, the module's lines.
`hc_irig_decode(signal, frame, year, buffer, capacity, written)` reads one
frame of an IRIG serial time code, A, B, D, E, G or H, named by its
IRIG 200-16 signal designation such as `B124`, from a string of `0`, `1`
and `M`, Pr first, a code with the year's two digits reading them in the
century of `year`; and `hc_irig_encode(signal, fixed, seconds_of_day,
hundredths, control, buffer, capacity, written)` writes the frame whose
Pr falls at a reading of the civil clock, which must be the start of a
frame, as `hc_irig_frame_start(signal, seconds_of_day, hundredths, buffer,
capacity, written)` rounds a reading down to it; `hc_irig_formats(buffer,
capacity, written)` lists the six formats with their frame's length, their
rate, their fields and the designations Table 4-1 permits them. All four
are in `time-codes`, and write the WebAssembly module's lines.
`hc_dotnet_ticks_from_unix(unix_seconds, attoseconds, out_ticks)` writes
.NET's `DateTime.Ticks` of a POSIX instant as an `int64_t`, and
`hc_unix_from_dotnet_ticks(ticks, buffer, capacity, written)` the reading
the ticks name, in seconds and attoseconds.
`hc_six_hour_clock(reckoning, seconds_of_day, buffer, capacity, written)`
reads a time of the civil day on the `ethiopian-hours` or `swahili-hours`
dial, and `hc_civil_from_six_hour_clock(reckoning, hour, minute, second,
night, out_seconds)` writes the civil seconds after midnight of a
reading as a `uint32_t`. `hc_french_decimal_time(seconds_of_day,
attoseconds, buffer, capacity, written)` writes the module's line of the
French Republican decimal time of a civil time, and
`hc_civil_from_french_decimal_time(hour, minute, second, attoseconds,
buffer, capacity, written)` its inverse.

## Python's `time` and `calendar`

`hc_gmtime(unix_seconds, buffer, capacity, written)`, `hc_timegm(year,
month, day, hour, minute, second, out_unix_seconds)`, `hc_isleap(year,
out_is_leap)`, `hc_leapdays(y1, y2, out_leap_days)`,
`hc_calendar_weekday(year, month, day, out_weekday)`, `hc_monthrange(year,
month, buffer, capacity, written)` and `hc_monthcalendar(year, month,
first_weekday, buffer, capacity, written)` are the functions of Python's
`time` and `calendar` modules, in the `civil` feature, as the facade's
`civil::StructTime` and `civil::calendar` have them; where the WebAssembly
module writes a line this library does too, the nine fields of a
`struct_time`, the first weekday and the days of a month, and a month's
weeks, in the columns its README gives, and the single numbers go to
out-parameters. `hc_week_of_year(fixed, first_weekday, min_days, buffer,
capacity, written)` writes the module's line of a day's week of the year
under a week rule, ISO 8601's being Monday 1 and 4 days, and
`hc_fixed_from_week(week_year, week, weekday, first_weekday, min_days,
out_fixed)` is its inverse, the fixed day a week date names. `hc_asctime(year,
month, day, hour, minute, second, buffer, capacity, written)` writes
`time.asctime` of a reading as the module's one-cell line. `hc_localtime(unix_seconds,
zone, buffer, capacity, written)` and `hc_mktime(year, month, day, hour,
minute, second, zone, policy, out_unix_seconds)`, in the `tz` feature, are
`time.localtime` and `time.mktime` in a named zone, `policy` being
`earliest`, `latest`, `reject` or `push-forward` where Python reads
`tm_isdst`; a second of 60 is `HC_ERROR_INVALID_DATE` on every day and in every
zone, as `datetime` refuses it. `hc_local_resolution(year, month, day, hour,
minute, second, zone, buffer, capacity, written)` writes the module's line of
what a reading means in a zone, `unique`, `ambiguous` or `nonexistent` with
both instants and their offsets, and `hc_mktime_policies(buffer, capacity,
written)` the module's lines of the four policies.

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
2. Otherwise, when the tag's data names the calendar's months and every era
   a date of it writes, from its data, CLDR root's abbreviations (`AH`,
   `BE`) or the calendar's own name for the era, the tag with the
   calendar's own names.
3. Otherwise, English, whole, so that a date is in one language, and the
   last column says `en`. A tag that does not parse, or that no data
   answers for, is the root locale `und`, whose month names, CLDR's
   `M01`..`M12`, name no language, and so is English too.

The date as the locale writes it holds an extra field only where the
calendar's sources write the date with it, and never a `name=value`
pair. `hc_day_extras(fixed, id, locale, buffer, capacity, written)`
writes the day's extra fields one line each, in the seven columns the
WebAssembly module's README lists under "The extra fields of a day":
calendar identifier, field identifier, value, the field's label, the
value as a reader reads it, `1` when the formatted date already writes
it, and the locale used. A null or empty `id` asks for every registered
calendar, and one the library does not know is `HC_ERROR_UNKNOWN`.

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
`hc_locales(buffer, capacity, written)` every locale in its ten: tag,
English name, native name, the three Gregorian coverage flags, the
calendars it names, its parent in the fallback chain, its default
numbering system and its direction — `hc_first_day_of_week(locale, out_weekday)` the ISO
weekday, Monday = 1 through Sunday = 7, the locale's week begins on by
CLDR 48's week data, with a null or unparsable tag as `und`, Monday — and
`hc_day_period(seconds_of_day, locale, buffer, capacity, written)`,
`hc_format_number(system, value, buffer, capacity, written)`,
`hc_parse_number(system, text, out_value)`, `hc_numbering_systems(buffer,
capacity, written)` and `hc_calendar_eras(calendar, locale, buffer,
capacity, written)` write the module's day periods of a time of day, an
integer in a numbering system and back, the numbering systems, and a
calendar's eras named in a locale —
`hc_gregorian_adoption(region, buffer, capacity, written)` the steps by
which the country with the ISO 3166-1 alpha-2 code `region` adopted the
Gregorian calendar, one line per step in seven columns: the last day of the old reckoning and the first of the new as
fixed days, the old calendar's identifier, the scope (`civil`,
`ecclesiastical` or `partial`), the instrument with its date, the new
calendar's identifier and the polity. A code the library does not know is
an empty string, and a null `region` is `HC_ERROR_NULL_POINTER`. The
columns are the same as the module's, and the README there describes
each.

The `datetime` feature is ISO 8601 beyond a calendar date, in the same
lines as the module's: `hc_parse_datetime(syntax, text, buffer, capacity,
written)` reads a date-time as a reading, with no instant where the text
states no zone; `hc_format_datetime(syntax, unix_seconds, attoseconds,
offset_seconds, precision, ...)` writes one in `iso8601`, its basic,
ordinal and week forms, `rfc3339`, `rfc2822`, `imf-fixdate` or `python`;
`hc_format_iso_date_as(fixed, form, style, ...)` writes a calendar, ordinal
or week date; `hc_iso_date_parts(text, ...)` reads a date of any accuracy;
`hc_iso_duration(text, ...)`, `hc_format_iso_duration(negative, years,
months, weeks, days, hours, minutes, seconds, fraction, ...)` and
`hc_iso_interval(text, ...)` read and write durations and intervals, a
component below zero being absent; and `hc_parse_pattern(syntax, pattern,
text, ...)` reads a text against a `strftime`, Python `strptime` or CLDR
pattern, with `hc_parse_pattern_in(syntax, pattern, text, locale, ...)` adding a
locale's names; those two are the `patterns` feature. A text not in the syntax is
`HC_ERROR_MALFORMED`, a date or time that does not exist
`HC_ERROR_INVALID_DATE`. The module's README gives the columns.

How a locale resolves is six more entry points of the same feature, which
write the module's lines and take a NUL-terminated BCP 47 tag, null for the
root locale, one that does not parse being `HC_ERROR_MALFORMED`:
`hc_locale_chain(locale, buffer, capacity, written)` the fallback chain a
name is looked up along, one line per step with the rule that led to it;
`hc_locale_info(locale, ...)` one line of what the locale is, its subtags
and keys, parent, default numbering, first day of the week and `minDays`,
direction, casing and plural rules;
`hc_plural_categories(locale, kind, ...)` the module's lines of the
categories a locale's cardinal or ordinal rules name, each with a number
that falls in it; `hc_locale_format(locale, calendar, ...)` the module's
eighteen lines of the standard date, time and date-time formats the locale
carries for a calendar in CLDR's four lengths, with the available formats,
under the CLDR calendar type the identifier maps to;
`hc_japanese_era_year(year, ...)` a Japanese era's year as its dates write
it, 元 for 1 and the Han numerals after;
`hc_plural_category(locale, number, kind, ...)` the CLDR category a number
written as text has (`kind` `cardinal`, the form after a count, or
`ordinal`, the form of a position);
`hc_names(locale, calendar, width, context, ...)` the names a locale has
for a calendar; `hc_case(locale, mode, text, ...)` a text recased as the
locale cases it; and `hc_isolate(locale, mode, text, ...)` a text wrapped
in the Unicode bidirectional isolates for the locale's direction. The
module's README gives the columns.

`hc_parse_date(calendar, locale, text, buffer, capacity, written)` reads
a date as the locale writes it in the calendar — what column 16 of
`hc_describe_day` writes, and a reader's own spelling of it — and writes
the module's line: the calendar's 18 columns of `hc_describe_day` for the
day the text names, then the fixed day. A text that is not one day is a
line whose error columns say why, as the WebAssembly module's README lists
under "A written date, read back": `ambiguous`, `two-digit-year`,
`year-not-written`, `weekday-mismatch`, `field-mismatch`, `not-recognised`
or `empty`, codes 101 to 107, or the calendar's own refusal. A null `text` is the empty one;
a null `calendar` is `HC_ERROR_NULL_POINTER`, and one the registry does not
carry `HC_ERROR_UNKNOWN`.

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
WebAssembly module's two lines, the yoga's and the karaṇa's, in its nine
columns, the yoga's ayanāṃśa by the identifier `ayanamsa` takes and by
its full name; `ayanamsa` is a NUL-terminated identifier, `lahiri`, `lahiri-rashtriya`, `lahiri-crc-1955`, `lahiri-drik`, `raman`,
`krishnamurti`, `reingold-dershowitz` or `fagan-bradley`, and a day without a sunrise at the
place is `HC_ERROR_NO_DATA`. The sky may instead be the *Sūrya Siddhānta*'s,
`surya-siddhanta`, which both lines then name. `hc_nakshatra_at(unix_seconds,
ayanamsa, buffer, capacity, written)` and `hc_nakshatra_of_day(fixed,
latitude, longitude, elevation, ayanamsa, buffer, capacity, written)` write
the module's line of the nakṣatra the Moon is in, at an instant or at the
day's sunrise; `hc_muhurtas(fixed, latitude, longitude, elevation, buffer,
capacity, written)` its thirty lines of the day's and the night's
muhūrtas, each named as English Wikipedia tabulates them, with Abhijit
and Dur Muhurtam marked; and
`hc_amrita_siddhi(fixed, latitude, longitude, elevation, ayanamsa, buffer,
capacity, written)` its line of the *amṛta siddhi yoga*. `hc_ioc_olympiad(gregorian_year,
out_olympiad)`, `hc_hebrew_yahrzeit(death_fixed, hebrew_year, out_fixed)`
and `hc_hebrew_birthday(birth_fixed, hebrew_year, out_fixed)` write one
`int64_t` each; a Hebrew date crosses as the fixed day whose daylight
carries it. `hc_ioc_olympiad_on(fixed, out_olympiad)` writes the
Olympiad of a day by the Charter in force on it, `HC_ERROR_NO_DATA` from
10 June to 21 November 1956; `hc_babylonian_regnal_year(seleucid_year,
buffer, capacity, written)`, `hc_shmuel_tekufah(hebrew_year, tekufah,
buffer, capacity, written)` and `hc_day_name(calendar, naming, fixed,
buffer, capacity, written)`, `hc_equinox_new_year_margin(calendar, year,
buffer, capacity, written)` write the module's lines of a Seleucid year's
king and regnal year, a *tekufah* of Shmuel's reckoning, a day's name in
the French Republican or Armenian calendar by a naming, and the minutes by
which an equinox missed or made the moment that decides a new year. `hc_chinese_reckoned_age(birth_fixed, on_fixed, out_age)`
writes a `uint32_t`, the Chinese count's age, with a day before the birth
`HC_ERROR_NO_DATA`, and `hc_chinese_marriage_augury(chinese_year, buffer,
capacity, written)` the module's line of the augury, its two 立春
flags and the Chinese names of the kind of year.
`hc_chinese_age(convention, birth_fixed, on_fixed, out_age)` writes the
age by a named count, `chinese-age`, `lichun-age`, `new-year-day-age` or
`year-age`, and `hc_chinese_almanac_solar_terms(year, buffer, capacity,
written)` the module's lines of the Qing almanac's term days of a year of
1645–1733.

`hc_festival_readings(buffer, capacity, written)` writes the module's two
lines of the Smārta and Vaiṣṇava readings of a festival's day, `smarta` and
`vaishnava`; `hc_janmashtami(year, reading, latitude, longitude, elevation,
ayanamsa, buffer, capacity, written)` the module's line of the day of Kṛṣṇa
Janmāṣṭamī in a Gregorian year at a place by one of them;
`hc_vaishnava_day(saka_year, month, tithi, latitude, longitude, elevation,
ayanamsa, buffer, capacity, written)` the line of the first day whose sunrise
carries a tithi of an amānta month of a Śaka year; and
`hc_vishti_free_span(saka_year, month, tithi, latitude, longitude, elevation,
ayanamsa, buffer, capacity, written)` the line of the part of a tithi Bhadra does
not cover. `hc_rahu_at(unix_seconds, ayanamsa, buffer, capacity, written)` and
`hc_rahu_ingresses(from_unix_seconds, to_unix_seconds, ayanamsa, buffer,
capacity, written)` write the module's line of the mean Rāhu and Ketu at an
instant and its lines of the node's entries into the sidereal signs in a span, and
`hc_era_new_year(calendar, year, out_fixed)` the fixed day on which a year of
`vikram-samvat-kartikadi`, `rajyabhisheka-saka`, `saptarshi`, `gupta`, `valabhi`,
`kalachuri` or `lakshmana-sena` begins. A null `reading`, `ayanamsa` or
`calendar` is `HC_ERROR_NULL_POINTER`, and a name not carried
`HC_ERROR_UNKNOWN`. `hc_solar_nakshatra_ingresses(from_unix_seconds,
to_unix_seconds, ayanamsa, buffer, capacity, written)` writes the module's
lines of the Sun's entries into the nakṣatras in a span, the nakṣatra
entered and the one left, with `hc_rahu_ingresses`'s span rule; and
`hc_tiruvalluvar_year(fixed, out_year)` writes Tamil Nadu's Tiruvaḷḷuvar
year of a day, the Gregorian year of its Thai 1 plus 31, on the Tamil solar
calendar's range.

`hc_kalam(convention, fixed, latitude, longitude, elevation, locale,
buffer, capacity, written)` writes the module's three lines of Rāhu kālam,
Yamaganda and Gulika kālam on a day, each named in the locale, by
`rahu-kalam-sunrise`, the daylight at the place, or `rahu-kalam-fixed`,
06:00 to 18:00 of the local clock;
`hc_almanac_cycles(fixed, meridian, buffer, capacity, written)` the
module's line of 恵方, 三元九運 and 손 없는 날 for a day, with 立春 at a
meridian read as for `hc_term_in_effect`; and `hc_almanac_day(fixed,
meridian, locale, buffer, capacity, written)` the module's lines of the
day's other annotations, the sexagenary day and its 納音, 十二直, 二十八宿 and
二十七宿, the three 九星, 六曜, and every 暦注下段, 選日 and combination
that falls, each named in the locale as `hc-i18n` names it, with `native`
for Japanese. All three are in `calendars`, as are four more of the
almanac: `hc_almanac_directions(fixed, meridian, buffer, capacity,
written)`, the module's lines of where the 八将神 and the 金神 stand in the
year in force on the day; `hc_rounichi(rule, year, meridian,
out_fixed)`, the fixed day of 臘日 in the winter that ends in `year` by a
named reckoning, `HC_ERROR_NO_DATA` for a winter it does not settle;
`hc_mansion_undertakings(list, fixed, buffer, capacity, written)`, what
the publisher's list `saijigoyomi` or `linderabell` says the day's 二十八宿
favours and forbids; and `hc_almanac_person_days(fixed, birth_fixed,
meridian, buffer, capacity, written)`, whether the day is one of a
person's own 五墓日 or 三箇の悪日 by the 干支 year, turning at 立春, of the
day they were born on. `hc_tibetan_almanac_day(calendar, fixed, buffer, capacity,
written)`, `hc_tibetan_planets(fixed, buffer, capacity, written)`,
`hc_bhutanese_winter_solstice(year, buffer, capacity, written)` and
`hc_tibetan_festival_day(rule, calendar, year, month, leap, day,
out_fixed)`, also in `calendars`, write the module's lines of the Tibetan
almanac's columns for a day, the Phugpa planets, the Bhutanese winter
solstice of a Gregorian year and the day a festival on a skipped or
repeated date is kept by `berzin` or `henning-almanac`.

`hc_era_table(table, buffer, capacity, written)` and
`hc_olympic_games(season, buffer, capacity, written)`, in `calendars` too,
write the module's lines of every era of the `japanese` (248 eras),
`chinese-regnal` (37), `korean-regnal` (3) or `vietnamese-regnal-nguyen`
(12, each with the day it was first in force) table, which
`hc_calendar_eras` does not list: it lists what the locale data does, 236
Japanese eras and none of the other three; and of the modern Olympic Games of
the `summer` or `winter` season, as Olympedia lists them. A name not known
is `HC_ERROR_UNKNOWN`, and null for it `HC_ERROR_NULL_POINTER`.
`hc_southeast_asian_year_type(calendar, year, buffer, capacity, written)`
writes the module's line of the kind of lunar year a year of the `khmer`
(Buddhist Era) or `lao` (Chulasakarat) calendar is, `normal`, `extra-day`
or `extra-month`, with the solar New Year's day and second;
`hc_maya_long_count(fixed, correlation, buffer, capacity, written)` the
module's line of a day in the Maya counts under the correlation the caller
names, `gmt`, `gmt2` or `martin-skidmore` or the constant as text;
`hc_akan_day(fixed, buffer, capacity, written)` and `hc_weton(fixed,
buffer, capacity, written)` the module's lines of a day of the Akan
*Adaduanan*, with its names and its *dabɔne*, and of the Javanese *weton*
with its *neptu*; and `hc_buddhist_lk_year(fixed, buffer, capacity,
written)` the module's line of Sri Lanka's Buddhist year of a day, from the
Vesak Poya Day, for 2023 to 2027, the years whose Vesak an order read
fixes.

`hc_choghadiya(fixed, latitude, longitude, elevation, locale, buffer,
capacity, written)` writes the module's sixteen lines of the choghadiya,
the eighths of a day's daylight and of the night after it with the kind
the weekday gives each; `hc_panchak(naming, unix_seconds, ayanamsa,
offset_seconds, locale, buffer, capacity, written)` its line of the
Panchak window in progress at an instant, or the next, with its kind by
`panchak-five-kinds` or `panchak-raj-midweek` for the weekday it opens on,
read on a clock `offset_seconds` ahead of UTC; `hc_kumbh(yoga, year,
ayanāṃśa, jupiter, locale, buffer, capacity, written)` its line of when
the Sun, and the Moon where asked, stand as a Kumbh condition requires in
a year; and `hc_pushkaram(sign, entry_unix_seconds, latitude, longitude,
elevation, meridian, locale, buffer, capacity, written)` its lines of the
twelve days of each river of a sign. Those have no ephemeris of
Jupiter, so Jupiter's sidereal sign (`jupiter`, null for none) and the
moment it enters one are the caller's, as the WebAssembly module's README
explains; the `jupiter` layer's `hc_kumbh_by_sky` and `hc_pushkaram_by_sky`
compute them. `hc_folk_day(fixed, meridian, locale, buffer, capacity,
written)` writes the module's lines of a day's folk reckonings — the
first-month counts, 入梅 and 出梅, Tam Nương and Nguyệt Kỵ, and the Turkish
year of Hızır and Kasım — and `hc_night_watch(seconds_of_day, locale,
buffer, capacity, written)` its line of the Chinese night watch of a time
of the civil clock, or the empty string by day.
`hc_barhaspatya_year(rule, saka, locale, buffer, capacity, written)`
writes the module's line of the northern sixty-year cycle's name a rule,
`surya-siddhanta-bija`, `surya-siddhanta` or `arya-siddhanta`, couples
with an expired Śaka year, and the name it expunges; and
`hc_barhaspatya_year_at(rule, unix_seconds, locale, buffer, capacity,
written)` the name in progress at an instant. Each names its terms in the
`locale`, a tag, `native` or null, with the locale used, as
`hc_day_extras` writes it, and all are in `calendars`.

`hc_hindu_lunar_date(sky, fixed, latitude, longitude, elevation, locale,
buffer, capacity, written)` writes the module's line of the amānta
lunisolar date of a day read at the sunrise of a place the caller gives —
the Śaka and Vikrama years, the month, the intercalary flag, the tithi,
the repeated flag and the sunrise, then in the `locale` (a tag, `native`
or null, as for `hc_describe_day`) the month's name, the word for an
intercalary month, the Śaka and Vikrama eras' names and the locale used —
on the true sky in the zodiac of a named ayanāṃśa,
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
been visible on the evening that begins the day by `shaukat`, `yallop`,
`saudi-rule`, `odeh`, `istanbul-2016`, `khgt`, `mabims-2021-topocentric` or
`mabims-2021-geocentric-elongation`, with the moment the evening is judged at and what the
criteria read there. The columns are the WebAssembly module's README's.

`hc_hebrew_sabbatical_cycle_year(hebrew_year, out_place)` writes a Hebrew
year's place in the seven-year sabbatical cycle, 1 through 7, the seventh
being *shemittah*: 5782 and 5789 are sabbatical years.
`hc_asian_day(fixed, buffer, capacity, written)` writes the module's line
of a day in the calendar of the province of Asia as the calendar writes
it: the year, the month and its name, `unnumbered` for Sebaste and the
other days before day 1 or `numbered`, and the day's number or its place
among the unnumbered days. `hc_solar_new_year(calendar, year, buffer,
capacity, written)` writes the module's line of the day and the moment the
year changes at the solar New Year of the `burmese`, `khmer` or `lao`
calendar, `year` in the calendar's own count, with Thingyan's days for
`burmese`; another calendar is `HC_ERROR_UNKNOWN`.

## Holidays

The four `hc_holiday_*` entry points, `hc_holidays_on`, `hc_holiday_tables`,
`hc_lectionary`, `hc_astronomical_easter`,
`hc_astronomical_paschal_full_moon`, `hc_holy_year_on`,
`hc_common_worship_on`, `hc_orthodox_fast_on` and
`hc_orthodox_fast_seasons` need the `holiday` feature:

```sh
cargo build -p hyper-calendar-ffi --release --features holiday
```

It compiles every table of `hc-holiday` into the library — the countries,
the exchanges, the traditions and the international days. A table is named
by its identifier: a country's ISO 3166-1 alpha-2 code (`JP`), an exchange's
ISO 10383 Market Identifier Code (`XNYS`), a tradition's slug
(`christian-western`) or `un-days`, and `hc_holiday_codes` lists them all.
`hc_holiday_is_day_off` answers for one day, 1 or 0, and refuses as
`HC_ERROR_NO_DATA` a day a gap of the day's year leaves open and as
`HC_ERROR_OUT_OF_RANGE` a day whose region's weekend law was not read;
`hc_holiday_is_weekend(code, region, fixed, out_is_weekend)` says whether a
day is a weekend day under that law; `hc_holiday_next` and
`hc_holiday_previous(code, region, group, kind, fixed, buffer, capacity,
written)` write the line of `hc_holidays_in_year` for the first entry after,
or the last before, a day; and
`hc_holiday_add_business_days(code, region, group, fixed, count,
out_fixed)` and `hc_holiday_business_days_between(code, region, group,
from_fixed, to_fixed, out_count)` count its business days in the same
scope, refusing a walk across a gap as `hc_holiday_is_day_off` does; `hc_holidays_in_year` writes a
year as tab-separated, NUL-terminated lines — the ISO date, the name, the
local name, the kind (`public`, `bank`, `religious`, `observance`,
`school`, `workday`, `government` or `half-day`), the confidence, `1` for a
substitute day, the date it stands in for, the region, the group, the
holiday's identifier within its table (`new-years-day`), the instrument
its rule cites and `1` for an entry a bridge policy made, Japan's 国民の休日 —
reporting the length it needs through `written` like
every other text function here. The region is the subdivision whose own entry the line is:
asked for `JP` in `JP-13`, the lines are Japan's nationwide days and
Tokyo's 都民の日, and only 都民の日 carries `JP-13`. A region matches in
either case. One the table's country has and its sources were not read
for, or not before a year (`JP-27` before 1989), gives the nationwide days
and a gap line for its own; a code the country has no subdivision for
(`US-ZZ`, `JP-99`, `JP garbage`), and any region of a tradition's table, is
refused as unknown. After the entries come the
year's gaps, as `hc_holidays_on` writes them: an empty date and
confidence, the kind `gap`, and the region and group whose own gap it is.
The group is the group of people whose own entry the line is, in the
same way: asked for `CN` and `women`, the lines are China's days for
everyone and the half day of 8 March, and only that line carries `women`.

The string arguments fail the same way in both, and the same way as in the
WebAssembly module: a null `code` is `HC_ERROR_NULL_POINTER`, a `code`,
`region` or `group` that is not UTF-8 is `HC_ERROR_NOT_UTF8`, and a `code`
that names no table, or a `group` that names no group of
`hc_holiday_groups`, is `HC_ERROR_UNKNOWN`. A null or empty `region` is no
region, and a null or empty `group` is everyone, as is a group the table
gives no day to alone. The fourth argument, `kind`, keeps the entries of the
kinds it lists, `;`-separated in any case, `public;bank`, and the gaps of
the rules of those kinds; a subdivision not read is a gap whatever it
lists, and a null or empty `kind` is every kind. A word that names no kind
is `HC_ERROR_UNKNOWN`. The United States' states carry about 1,300
observances, which the filter leaves out for a caller that wants the days
off.

```c
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef int HcStatus;
#define HC_OK 0
HcStatus hc_holidays_in_year(const char *code, const char *region, const char *group,
                             int64_t year, char *buffer, size_t capacity, size_t *written);

int main(void) {
    size_t need = 0;
    hc_holidays_in_year("JP", NULL, NULL, 2026, NULL, 0, &need);   /* HC_ERROR_BUFFER_TOO_SMALL */
    char *lines = malloc(need);
    if (lines != NULL && hc_holidays_in_year("JP", NULL, NULL, 2026, lines, need, &need) == HC_OK) {
        fputs(lines, stdout);
    }
    free(lines);
    return 0;
}
```

A subdivision is not a table, for every country alike: its days are rules
of its country's table scoped to its ISO 3166-2 code, and it is asked for
by `region`, Tokyo as `JP` in `JP-13`. `hc_holiday_codes` lists no
subdivision code; `hc_holiday_tables` names a country's subdivisions in
column 9 of the country's row. Asked for `JP` with no region,
`hc_holidays_in_year` and `hc_holiday_is_day_off` answer for the
nationwide days alone, not the union of the prefectures'; asked for `JP`
in `JP-13`, for the nationwide days and Tokyo's own. `hc_holidays_on`
takes no region and writes a subdivision's own lines under its country's
code, with the subdivision in its tenth column, as the WebAssembly
module's README sets out under "Subdivisions and groups".

A group of people a statute gives a day to alone is asked for by `group`,
independently of the region, by an identifier of `hc_holiday::group`:
`women`, `youth`, `children`, `military`, `police`, `firefighters`,
`coast-guard`, `indigenous-peoples`, matched in either case. China's
Article 3 gives women half of 8 March, youth half of 4 May, children
1 June and active servicemen half of 1 August; Taiwan's Article 6 leaves
the services' days to their authorities, which were not read, so each is a
gap for its group. `hc_holiday_tables` lists a table's groups in column 10
and their names in the locale in column 11. With no group,
`hc_holidays_in_year` and `hc_holiday_is_day_off` answer for everyone's
days alone; with `CN` and `children`, 1 June is a day off. A half day is
the kind `half-day`, a business day on which work stops for part of the
day. `hc_holidays_on` writes each group's own lines after the
subdivisions', with the group in its last column.

`hc_holidays_on(fixed, buffer, capacity, written)` writes every entry on
one day across every table `hc_holiday_codes` lists, in that order, each
evaluated nationwide, then in each region it answers for (the
subdivisions its rules, weekend laws and substitution policies are scoped
to), then for each group its rules give days to alone, and last for each
subdivision and group a rule names together, one line per (table, scope,
entry): the table's identifier, its English name, the holiday's English
name, its local name, the kind (`public`, `bank`, `religious`,
`observance`, `school`, `workday`, `government`, `half-day`, or `gap`), the
confidence, the instrument the rule cites, `1` for a substitute, the fixed
day it stands in for, the ISO 3166-2 code of the subdivision whose own
entry it is, `JP-13`, or nothing for a nationwide one, and the identifier
of the group whose own entry it is, `women`, or nothing for one everyone
has, and last the holiday's identifier within its table, `new-years-day`,
which `hc_holidays_in_year` and `hc_common_worship_on` write for the same
entry. A subdivision's lines are only the entries the nationwide calendar
does not have, and a group's the entries the calendar for everyone does
not, so a nationwide holiday is written once. A `gap` line is a holiday the table
could not place in the day's year — its calendar's range ended, or the
year's announcement has not been read — reported so the caller can say so,
on the days it could fall on and on no other.
Each table is evaluated for the one day (`HolidayCalendar::for_day`), which
answers exactly what the whole year would at about a third of the cost.

`hc_holiday_coverage(code, buffer, capacity, written)` writes the module's
lines of the years a table answers for, in the seven columns of the
WebAssembly module's README, and `hc_holiday_groups(locale, buffer, capacity, written)` writes the module's
lines of every group a holiday may be given to alone, named in the locale,
and `hc_holidays_on_in(fixed, locale, buffer, capacity, written)`
`hc_holidays_on`'s lines with three more cells, the day's name in the locale
where a source in the language names it by its identifier, and the tag
that named it, and then the identifier and the bridge flag, which
`hc_holidays_on` ends with, so that the first eleven cells and the two names
keep their places.
`hc_holiday_tables(locale, buffer, capacity, written)` describes every
table in `hc_holiday_codes` order, in the sixteen columns of the WebAssembly
module's README: the code, the kind, the name in the locale, the English
name, the locale that answered, the sources, the country of a subdivision
or an exchange where its table records one, the short name, the
regions the table answers for (the subdivisions its rules, weekend laws and
substitution policies are scoped to), `;`-separated, the groups its
rules give days to alone, `;`-separated, and those groups' names in the
locale, English where `hc-i18n` names a group in no other language, the
pairs of a subdivision and a group a rule is scoped to both of,
`region:group`, `;`-separated, and the subdivisions the table's sources were
read for, `;`-separated in code order, the table's weekend laws (column 14),
its weekend-substitution laws (column 15) and the tables whose days off it
keeps as its own (column 16, `XKRX`'s `KR//2009`). A country
is named by CLDR 48's territory name in the `locale` where `hc-i18n`
carries one, and else, as for a null `locale`, by CLDR's English name;
an exchange, a tradition and a set of observances by the table's English
name; the tag that answered is in column 5. Column 14 lists the table's
weekend laws, `;`-separated, each `days/first/last/regions`: the days as ISO 8601
weekday numbers joined by `+`, or `unread`, the first and last day in force
as `YYYY-MM-DD`, and the ISO 3166-2 codes it is the weekend of, as the
WebAssembly README's table of its sixteen columns describes. Column 8 is
CLDR 48's `alt="short"` name beside a CLDR name in column 3, from the same
data (`Hong Kong` under `en`, 香港 under `ja`), and empty elsewhere. `hc_holiday_rules(code, buffer, capacity, written)` writes the rules of a
table, one line each in the table's order, in the twelve columns of the
WebAssembly module's README (the rule's identifier, its names, kind and
confidence, the years it was established, abolished and read from, its regions
and groups, the part of the table's substitution law that reaches it and its
source); a code that names no table is `HC_ERROR_UNKNOWN`. `hc_lectionary(fixed, buffer,
capacity, written)` writes the liturgical year, the Sunday cycle, the
Roman weekday cycle and the RCL Proper of a day, the Roman number of a
Sunday in Ordinary Time and the week of Ordinary Time on the universal
calendar and on one that keeps the Epiphany on a Sunday, as the
WebAssembly README's table of its seven columns describes, and
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
`hc_roman_1960_office_on(fixed, buffer, capacity, written)` writes the
module's lines of the 1960 ordo of a day: the office kept, its
commemorations, and the feasts transferred or omitted.
`hc_common_worship_on(fixed, buffer, capacity, written)` writes the rank of
each *Common Worship* celebration kept on a day, one line each: the title,
which is its name in `hc_holidays_on`'s `common-worship` table, the rank,
the rank's English name and the celebration's identifier, which
`hc_holidays_on` ends its entry with and is what joins the two. The Festivals the Rules leave without a day
are `gap` lines of `hc_holidays_on`.

`hc_orthodox_fast_on(reckoning, fixed, buffer, capacity, written)` writes
the module's line of what a day is in the Eastern Orthodox fasting
scheme, and `hc_orthodox_fast_seasons(reckoning, year, buffer, capacity,
written)` its twelve periods of a year, each with its first and last day,
the kind of each `fast`, `fast-free` or `meat-excluded`, and the day's
line ending in what it abstains from, `nothing`, `meat` or `fast`;
`reckoning` is `orthodox-fasts`, the fixed dates in the Julian calendar,
or `orthodox-fasts-revised-julian`, in the Revised Julian; `armenian-fasts`
or `armenian-fasts-jerusalem`, the Armenian fasts on the Gregorian or the
Julian calendar, or `armenian-fasts-fifty-days`, with the weekly fasts
lifted to Pentecost; or `coptic-fasts` or `ethiopian-fasts`; and a year
outside 326 to 4099, 1583 to 4099 on `armenian-fasts`,
`armenian-fasts-fifty-days`, `coptic-fasts` and `ethiopian-fasts`, is
`HC_ERROR_OUT_OF_RANGE`.

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

`hc_pentad_traditions(buffer, capacity, written)` and
`hc_pentad_in_tradition(fixed, tradition, meridian, buffer, capacity,
written)`, in the same feature, write the module's lines of the four
traditions that name the 72 pentads (`chinese`, `japanese`, `jokyo` and
`senmyo`) and of the pentad in effect named by one of them, with its English
gloss and the alternate reading the tradition's text prints beside the
name; a tradition not listed is `HC_ERROR_UNKNOWN`, and null for it
`HC_ERROR_NULL_POINTER`. `hc_pentads_in_year(year, meridian, buffer,
capacity, written)` writes every pentad that begins in a Gregorian year at
a meridian, named by all four traditions at once, one a line in the
module's columns. `hc_tropical_signs_in_year(year, meridian, buffer,
capacity, written)` and `hc_sidereal_signs_in_year(year, ayanamsa, meridian,
buffer, capacity, written)` write the module's twelve lines of the sign
periods of a Gregorian year, tropical from Aquarius and sidereal, the
saṅkrāntis, from the first on or after 1 January in the ayanāṃśa's zodiac,
null for which is `HC_ERROR_NULL_POINTER`; `hc_traditional_tanabata(year,
meridian, out_fixed)` the fixed day of the Observatory's 伝統的七夕, at
`japan` for its table; and `hc_principal_phases_in_month(year, month,
meridian, buffer, capacity, written)` the module's lines of the four or five
principal phases inside a Gregorian month, a month outside 1 to 12 being
`HC_ERROR_INVALID_DATE`. `hc_zassetsu_in_year(year, meridian, buffer,
capacity, written)` and `hc_seasonal_days_in_year(year, meridian, buffer,
capacity, written)` write the module's lines of the 雑節 of a year and of the
other seasonal days — the 三伏 and nine nines of the Chinese year, the dog
days, the British and Irish quarter days and the Turkish folk year's named
days — for a year from −1000 to 3000 at a meridian read as above.
`hc_kumbh_yogas(locale, buffer, capacity, written)` and
`hc_pushkaram_rivers(locale, buffer, capacity, written)`, in `calendars`,
write the module's lines of the Kumbh Mela's seven conditions and the 14
rivers of the Pushkaram; the `jupiter` layer's
`hc_pushkaram_rules(buffer, capacity, written)` writes the two rules for which
of Jupiter's entries it follows, `hc_kumbhs_in_year_by_sky(year, ayanamsa,
locale, buffer, capacity, written)` writes `hc_kumbh_by_sky`'s line for all seven conditions
and `hc_jupiter_stations(from_unix_seconds, to_unix_seconds, ayanamsa,
buffer, capacity, written)` the moments Jupiter turns back or resumes.

`hc_cold_food_day(convention, year, out_fixed)`, in the same feature,
writes the fixed day of 寒食 under a NUL-terminated reckoning:
`hanshi-solstice-105`, `hanshi-eve-of-qingming` or `hansik`, as the
WebAssembly module's README describes them, in any case; another is
`HC_ERROR_UNKNOWN`, and a year outside −999 to 3000 `HC_ERROR_OUT_OF_RANGE`.
`hc_plum_rains(rule, year, meridian, out_fixed)` writes the fixed day of
入梅 or 出梅 of a year by `ru-mei-bing`, `ru-mei-ren` or `chu-mei-wei`, with
the solar term at a meridian; a year outside −1000 to 3000 is
`HC_ERROR_OUT_OF_RANGE`.

## Deep time

`hc_place_years_ago`, `hc_cosmic_events`, `hc_earliest_evidence`,
`hc_archaeological_periods`, `hc_future_events` and `hc_geologic_intervals`
need the `deep-time` feature, take a NUL-terminated BCP 47 `locale` (or null)
after their other arguments, and write lines of the same sixteen columns:
kind, the entry's stable lower-case kebab identifier (match on it rather
than on the name), the English name, scope (the identifier one rank up for
a geologic interval, the region for an archaeological period, the landmark
for an earliest-evidence claim, the kind of prediction for a future event),
the older bound's value, standard
uncertainty, significant figures and approximate flag, the same four for
the younger bound, the unit, the description, the source, and the name in
the locale's language: the geological chart's own for an interval, from the
ICS's translations, and for a cosmic, archaeological or earliest-evidence
row the established term `hc_deep_time::names` carries, empty where there is
neither. `hc_earliest_evidence` lists the published claims to the earliest
evidence of life, of *Homo sapiens* and of writing, one line per claim in
`years-before-1950`, each keeping the shape of its source's date: a minimum
age has no start, a range two different bounds, and a σ the source does not
state is empty — the only lines on which a σ can be.
`hc_archaeological_periods` lists the conventional archaeological periods,
youngest first, and `hc_future_events` the dated events of the far future,
soonest first, in `years-from-now`, an experimental bound with no end. The WebAssembly module's README tabulates the columns.
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
carries the eighteen zones of `hc-tz`'s built-in table with their current
rules only, listed in the WebAssembly module's README; for any other zone,
or a zone's history, `hc_zone_load(name, tzif, tzif_len)` takes the zone's
TZif file once and the conversions answer for that name from then on, a
loaded zone outranking a built-in one. Bytes that are not TZif are
`HC_ERROR_MALFORMED`. Every zone answers for the instants and the days of
the years −9 999 994 to 9 999 994 by UTC, and beyond them the three
entry points refuse with `HC_ERROR_OUT_OF_RANGE` ("Errors and ranges"
above).

`hc_zone_offset(zone, unix_seconds, buffer, capacity, written)` writes the
offset the zone keeps at the instant from those same rules, in the six
columns of the WebAssembly module's README under "A zone's offset": the
offset in seconds east of UTC, `1` or `0` for daylight saving or summer
time, the rules' abbreviation or empty where they give a numeric one, the
next transition's POSIX second and the offset after it or both empty where
the rules have none, and `builtin` or `loaded`. Europe/Berlin at
1 774 746 000, its change of 29 March 2026, is
`7200 1 CEST 1792890000 3600 builtin`.

`hc_zones(locale, buffer, capacity, written)` writes where each of the 312
zones of the IANA database's `zone1970.tab` is, in the eight columns of
the WebAssembly module's README: the zone, the latitude and longitude of
its principal location in decimal degrees, the table's whole arcseconds
written to six places (multiply by 3600 and round for the arcseconds),
its countries `;`-separated (`JP;AU` for `Asia/Tokyo`), the one country
`zone.tab` lists it under for a label (`JP`; empty for a name `zone.tab`
has no row for, which none of release 2026d is), the table's comment, its CLDR 48 exemplar city and the tag that named the
city. `hc_zone_location(zone, locale, buffer, capacity, written)` writes
the same line for one name: a zone; a link `zone.tab` gives a place of its
own, such as `Europe/Oslo`; or another link of `backward`, such as
`Asia/Calcutta`, which answers with the line of `Asia/Kolkata`. A name
that places nothing, such as `UTC`, is `HC_ERROR_UNKNOWN`. The city is in
the `locale` in a build with `calendars` or `zone-names` too, and otherwise, and for a
null `locale` or a locale with no city for the zone, in English with `en`.

`hc_zone_name(zone, unix_seconds, locale, field, buffer, capacity,
written)` needs the `zone-names` feature and writes the module's line of
a zone's name at an instant in a locale, as the CLDR pattern field `field`
writes it: `z` to `zzzz`, `O` and `OOOO`, `v` and `vvvv`, `V` to `VVVV`,
from CLDR 48's metazones and names. Another field is `HC_ERROR_UNKNOWN`.
`hc_format_pattern(zone, unix_seconds, locale, syntax, pattern, buffer,
capacity, written)`, in the same feature, formats the instant in the zone
by a `cldr` or a `strftime` pattern, in the same line; `%E` writes the
Japanese eras for a locale whose `-u-ca-` key is `japanese` in a library
built with `calendars` too, as the module's README says.

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
`hc_drekkana_at(unix_seconds, ayanamsa, buffer, capacity, written)` is its
sidereal twin, the Hindu third of the sidereal sign in an ayanāṃśa's
zodiac with its lord, as the WebAssembly module's README gives it.

## The Earth's rotation and the Sun's hours

`hc_earth_rotation_angle`, `hc_gmst_iau2006`, `hc_gmst_iau1982` and
`hc_ut2_minus_ut1`, each `(ut1_unix_seconds, out)`, need the `sky` feature
and write a `double`: the angle in degrees or UT2 − UT1 in seconds, at a
UT1 reading counted as POSIX seconds are, from 1970-01-01 00:00 UT1. The
two sidereal times are two conventions and two entry points.
`hc_ut1r_iers2010(ut1_unix_seconds, buffer, capacity, written)` and
`hc_ut1s_iers2010(...)` write the module's line of UT1 with the zonal tides
of the IERS 2010 model removed, the 41 under 35 days and all 62, as the
difference from UT1 in seconds and the regularised reading;
`hc_zonal_tide_ut1_effect(ut1_unix_seconds, period_limit_days, buffer,
capacity, written)` the tides under a period limit alone, a limit that is
not positive being `HC_ERROR_OUT_OF_RANGE`; and
`hc_equation_of_time(unix_seconds, buffer, capacity, written)` the module's
one cell, apparent less mean solar time in seconds at an instant.
`hc_solar_noon(fixed, latitude, longitude, elevation, out_unix_seconds)` and
`hc_solar_midnight(...)` write the Sun's upper transit on a local day at a
place and the lower transit that opens it as POSIX seconds, which every day
has; `hc_dawn(twilight, fixed, latitude, longitude, elevation, buffer,
capacity, written)` and `hc_dusk(...)` write the module's line of the start
and the end of a `civil`, `nautical` or `astronomical` twilight, the
missing depression named in `hc_solar_event`'s cells when the Sun does not
reach it; null for the twilight is `HC_ERROR_NULL_POINTER`.
`hc_solar_time(clock, unix_seconds, latitude, longitude, elevation, buffer,
capacity, written)` and `hc_solar_event(event, fixed, latitude, longitude,
elevation, buffer, capacity, written)` write the WebAssembly module's
lines: a local clock's reading — `local-mean`, `local-apparent`,
`temporal`, `italian` — or a named time of day — `asr-shafii`,
`asr-hanafi`, `jewish-dusk-vilna-gaon`, `jewish-sabbath-ends-cohn`,
`italian-zero-hour`, and the Japanese dawn and dusk,
`japanese-dawn-kansei`, `japanese-dusk-kansei`, `japanese-dawn-naoj` and
`japanese-dusk-naoj` — and, where the solar event it needs does not happen,
cells naming what is missing instead of a number, `hc_solar_event`'s with
the depression in arcseconds last.
`hc_prayer_times(method, fixed, latitude, longitude, elevation, ramadan,
buffer, capacity, written)` writes the module's eight lines of the Islamic
prayer times of a day by a method `hc_prayer_methods(buffer, capacity,
written)` lists with its parameters; `hc_zmanim(reckoning, fixed,
latitude, longitude, elevation, buffer, capacity, written)` its nine lines
of the Jewish times in temporal hours by `zmanim-gra`, `mga-72-minutes` or
`mga-16-1-degrees`, with the dawns and nightfalls;
`hc_temporal_hour(reckoning, fixed, latitude, longitude, elevation, buffer,
capacity, written)` its line of the length of the temporal hour those
times are counted in; and
`hc_edo_time(unix_seconds, latitude, longitude, elevation, buffer,
capacity, written)` and `hc_unix_from_edo_time(fixed, hour, fraction,
latitude, longitude, elevation, buffer, capacity, written)` the Edo
不定時法 reading of an instant and the instant of a reading.
`hc_horizons(locale, buffer, capacity, written)` lists the named horizons
a rising or a setting is measured against — `geometric-dip`, `usno` and
`calendrical-calculations` — with their English names, descriptions,
sources and short names for a label, and each name in the locale where an
observatory or almanac office gives one, else the English, with the tag of
the data that named it; and `hc_sunrise(horizon, fixed, latitude, longitude, elevation,
buffer, capacity, written)` and `hc_sunset(...)` write the module's line
of the crossing against the one named: the instant, the cells of a
missing sunrise or sunset, and the altitude of the Sun's centre at the
crossing; `hc_moonrise(...)` and `hc_moonset(...)`, with the same
arguments, write the Moon's upper limb's crossing, the instant or the
missing moonrise or moonset and its day, since the Moon skips a local day
about once a month. `hc_hjd_tt(tt_julian_date, right_ascension, declination,
buffer, capacity, written)` and `hc_hjd_utc(utc_julian_date,
right_ascension, declination, strict, buffer, capacity, written)` write
the module's lines of the Heliocentric Julian Date of a target's J2000
direction in TT and in UTC, two scales and two entry points: the HJD and
the light-time correction in seconds, and for UTC the TT − UTC the
leap-second table gave; under `strict` a date outside the table is
`HC_ERROR_NO_DATA`. `hc_planetary_hour(unix_seconds, latitude, longitude,
elevation, locale, buffer, capacity, written)` and
`hc_planetary_hours_of_day(fixed, latitude, longitude, elevation, locale,
buffer, capacity, written)` write the module's lines of the planetary
hour at an instant and of the twenty-four hours of a planetary day, each
ruler named in the `locale`. All of them answer for the sky layer's years
−1000 to 3000. `hc_gmat_from_gmt(fixed, seconds_of_day, attoseconds,
buffer, capacity, written)` and `hc_gmt_from_gmat(...)`, in `sky` too,
write the module's line of a reading of GMT as Greenwich Mean
Astronomical Time, GMT − 12 h, and back, for any day of the Gregorian
years −9 999 999 to 9 999 999.

## Jupiter

`hc_jupiter_at(unix_seconds, ayanamsa, buffer, capacity, written)`,
`hc_jupiter_ingresses(from_unix_seconds, to_unix_seconds, ayanamsa, buffer,
capacity, written)`, `hc_jupiter_risings(from_unix_seconds, to_unix_seconds,
ayanamsa, buffer, capacity, written)`, `hc_kumbh_by_sky(yoga, year,
ayanamsa, locale, buffer, capacity, written)`, `hc_pushkaram_by_sky(sign,
year, ayanamsa, rule, latitude, longitude, elevation, meridian, locale,
buffer, capacity, written)` and `hc_pushkarams_in_year(year, ayanamsa, rule,
latitude, longitude, elevation, meridian, locale, buffer, capacity,
written)` need the `jupiter` feature and write the lines
the WebAssembly module's README tabulates, from the complete VSOP87B series
for Jupiter in `hc-astro` (3 625 terms, 55 kB of tables that no other layer
carries): where Jupiter is at an instant, tropical and sidereal; its
crossings of the sidereal boundaries and its heliacal risings in a span, a
span of at most a hundred Julian years; and `hc_kumbh` and `hc_pushkaram`
with Jupiter's sign and the moment of its entry found rather than given.
`hc_pushkarams_in_year` writes `hc_pushkaram_by_sky`'s lines for every sign
Jupiter enters in the year, in the order of the entries, byte for byte what
the sign-by-sign calls write, from one search of the sky instead of one for
each sign. `rule` is `pushkaram-final-entry` or `pushkaram-first-entry`. A null pointer
for any name is `HC_ERROR_NULL_POINTER`, a name not known
`HC_ERROR_UNKNOWN`, and an instant, span or year outside the years −1000 to
3000 `HC_ERROR_OUT_OF_RANGE`; a `to` not after `from` is an empty answer.

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
identifiers, the first column of `hc_missions` and `hc_bodies`, in any
ASCII case: `viking-1`, not `Viking 1`. No mission convention is
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

## Place names

`hc_territories(locale, buffer, capacity, written)`,
`hc_subdivisions(country, locale, buffer, capacity, written)` and
`hc_place_name(code, locale, buffer, capacity, written)` need the
`places` feature and write the WebAssembly module's lines: one per
territory or ISO 3166-2 subdivision CLDR 48 names, and one per municipality
a holiday table lists (`JP-14-130`, 川崎市, from `hc-i18n`'s
`municipal_names`, with the status `municipal` and no draft level), with its code as ISO
writes it (`JP-13` for CLDR's `jp13`), its name in the locale, its English
name, the tag of the data that named it, that value's CLDR draft level and
the code's CLDR validity status. A null or empty `country` writes every
subdivision; a `country` or a `code` the data does not name is
`HC_ERROR_UNKNOWN`. The WebAssembly module's README gives the columns and
the lookup, and `docs/systems/place-names.md` explains both.

## Human-readable time

`hc_relative_time(then_unix, now_unix, style, automatic, locale, buffer,
capacity, written)`, `hc_relative_day(then_fixed, now_fixed, style,
automatic, locale, buffer, capacity, written)`,
`hc_relative_day_at(then_fixed, now_fixed, seconds_of_day, style,
automatic, locale, buffer, capacity, written)` and `hc_duration(seconds,
style, max_components, locale, buffer, capacity, written)` need the
`humanize` feature and write the WebAssembly module's lines, from
`hc-humanize`: how one instant reads from another, *3 hours ago*; which
calendar day one fixed day is, seen from another, *yesterday*, and with a
time of day, *yesterday at 15:05*; and a span of seconds in days, hours,
minutes and seconds, *2 hours and 30 minutes*. Each line ends with the tag
of the data the locale resolved to; a null locale, or one that does not
parse, is the root locale, whose phrases are CLDR's `root.xml`'s, `-1 d`; a
locale `hc-i18n` carries that has no phrases (`aeb-Latn`, `ayl-Latn`, `ban`,
`bo`, `cop`, `kab`, `mid`, `mix`, `nah`, `pa-Arab`, `rif`, `sa`, `shi-Latn`, `yua`, `zap`, `zgh`) is
`HC_ERROR_NO_DATA`, not the root's.
`style` is `long`, `short` or `narrow`, and for `hc_duration` `compact` as
well; another is `HC_ERROR_UNKNOWN`. The WebAssembly module's README gives
the columns.

`hc_unit_choice(seconds, thresholds, rounding, ...)`,
`hc_relative_time_with(then_unix, now_unix, style, automatic, locale,
thresholds, rounding, ...)` and `hc_approximate_duration(seconds, style,
locale, thresholds, policy, ...)`, in the same feature, take the thresholds
(`default`, `exact` or `with-quarters`), the rounding (`ceil`, `floor`,
`nearest`, `truncate` or `nearest-half`) and the hedge policy (`default` or
`bounded`) that `hc_relative_time` fixes; a name not known is
`HC_ERROR_UNKNOWN`.

The `natural` feature has the functions of Python's `humanize` package with
its 35 gettext catalogues. The ones with words take a `locale`, resolved
along its fallback chain to the first catalogue that translates every one of their words,
English where none does, and write one line of the text and the language of
the catalogue used, `en`, `de-DE`: `hc_apnumber(value, locale, buffer,
capacity, written)`, `hc_metric(value, unit, precision, locale, ...)`,
`hc_naturalsize(value, style, decimals, locale, ...)` with `style` `decimal`,
`binary` or `gnu`, `hc_naturallist(items, locale, ...)` with one item to a
line, `hc_intword(digits, decimals, locale, ...)`, which takes the integer
as digits and so reaches *1.0 googol*, `hc_naturaldelta(seconds,
microseconds, months, minimum_unit, locale, ...)`, `hc_naturaltime(...)`,
`hc_precisedelta(seconds, microseconds, minimum_unit, suppress, decimals,
locale, ...)`, `hc_naturalday(day, today, pattern, locale, ...)`,
`hc_naturaldate(day, today, locale, ...)`, `hc_ordinal(value, gender,
locale, ...)`, `hc_intcomma(digits, locale, ...)` and
`hc_intcomma_float(value, ndigits, locale, ...)`. `hc_fractional(value,
...)` and `hc_scientific(value, precision, ...)` write no word and take no
locale; their language cell is `en`. Text that is not an integer is
`HC_ERROR_MALFORMED`; `NaN` as a size, an integer beyond the largest double,
more than 255 decimals and a minimum unit above seconds are
`HC_ERROR_OUT_OF_RANGE`. `hc_clamp(value, format, floor, ceil,
floor_token, ceil_token, ...)` is `clamp`, `format` being `display`,
`fixed:N` or `percent:N` and `floor` and `ceil` decimal numbers or null for
no bound. The WebAssembly module's README gives the columns.
`hc_list_forms(style, locale, buffer, capacity, written)`, in the
`humanize` feature, writes the CLDR list patterns a style joins a
duration's parts with, in a locale.

## Leap seconds, and the `strict` flag

`hc_tai_from_unix`, `hc_tai_minus_utc`, `hc_utc_from_tai` and
`hc_day_has_leap_second` take a `strict` flag. Non-zero refuses to answer before 1961, when UTC did not exist, and past
the announced validity of the IERS leap-second table, with
`HC_ERROR_NO_DATA`; zero holds the last published offset into the future and
treats UTC as TAI before 1961. Holding the last offset into the future is
a forecast, which is why it is the caller's choice and not a default. For
`hc_day_has_leap_second` the flag concerns the future only: a day past the
table is refused when it is non-zero and answered no when it is zero; before
1961 no day ended in an inserted second, so none is refused. `hc-core`'s
README states the table's horizon.

## What is not here

The default surface is the civil calendar and the TAI–UTC bridge: enough to
turn a POSIX timestamp or a Gregorian date into a fixed day and back, name a
weekday, and ask about leap seconds honestly; the other layers are features,
above. Formatting is by template and only as wide as a locale's data: a
locale that has stated none writes a date as its fields in order. Before
1.0 the only stability promise is the status codes, whose meanings do not
change; a line's columns may change, each change listed in the pull
request that makes it.

An entry point here has a twin of the same name in the WebAssembly module
in [`hyper-calendar-wasm`](../hyper-calendar-wasm), except for those its
README's [twin table](../hyper-calendar-wasm/README.md#twins) lists with
the reason: here, `hc_gregorian_from_fixed`.

Neither boundary exposes these parts of the workspace, for the reasons
that README's "What is not here" gives: `hc-planetary`'s circad and
Martiana calendars in the registry, whose day number is a circad or a sol
and not an Earth day, though `hc_circad_date` dates an instant in them;
the five parts of `hc-humanize` that README lists (`intword` of an `i64`,
`clamp`, `naturaltime` of two readings, a threshold table of the caller's
own rows, and a locale for the thresholds of `hc_relative_day`), the loader
of `hc-name-days` for a list a caller has licensed, and of `hc-almanac` 七曜,
which is the weekday, and the English glosses of its annotations; a
whole UUID with its clock sequence and node, where
`hc_uuid_timestamp_encode` writes the time fields only; and the radio
frames' notices that name no minute other than JJY's stop notice.
