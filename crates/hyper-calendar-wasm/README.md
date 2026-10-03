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
answer — a locale, a zone, a meridian, an output format, the ayanāṃśa of
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
| a fixed day | `hc_fixed_from_unix_in_zone` | `unix_seconds` −315 631 497 830 400 through 315 507 195 014 399, the instants of the years −9 999 994 to 9 999 994 by UTC, which a zone's rules answer for: they are read on the Gregorian years ±9 999 999, and an instant's answer reads the years around its own, which beyond these would give standard time whatever the rules say; the day is at most one from the day `hc_fixed_from_unix` gives; any other is `HC_ERR_OUT_OF_RANGE`, and a name neither the loaded zones nor the built-in table knows `HC_ERR_UNKNOWN` |
| seconds | `hc_unix_from_fixed` | `fixed` −104 165 947 503 through 106 751 991 886 463: an earlier day's midnight would be at or below `HC_ERR_FLOOR` seconds, and a later one's would overflow an `i64`; either is `HC_ERR_OUT_OF_RANGE` |
| seconds | `hc_unix_from_fixed_in_zone` | `fixed` −3 652 423 173 through 3 652 422 808, the days of the same years as `hc_fixed_from_unix_in_zone`'s; any other is `HC_ERR_OUT_OF_RANGE` |
| seconds | `hc_tai_minus_utc` | every `unix_seconds`; under `strict`, 1961 through the end of the announced leap-second table, and `HC_ERR_NO_DATA` outside it |
| a byte length | `hc_tai_from_unix` | every `unix_seconds` up to 9 223 372 036 854 775 770, `i64::MAX − 37`: TAI runs ahead of UTC, so a later one has no TAI second an `i64` holds and is `HC_ERR_OUT_OF_RANGE`; under `strict`, 1961 through the end of the announced table, else `HC_ERR_NO_DATA` |
| a byte length | `hc_utc_from_tai` | every `tai_seconds`; under `strict`, as for `hc_tai_from_unix` |
| 1 or 0 | `hc_day_has_leap_second` | `unix_seconds` −9 223 372 036 854 720 000 through 9 223 372 036 854 719 999, the whole days of the `i64` range; the part-days at its two ends begin or end where no `i64` reaches, and are `HC_ERR_OUT_OF_RANGE` |
| 1 or 0 | `hc_holiday_is_day_off` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERR_OUT_OF_RANGE` |
| a fixed day | `hc_holiday_add_business_days` | `fixed` −3 652 424 999 through 3 652 424 634 and `count` −36 500 through 36 500, a walk that stays in the years −9 999 999 to 9 999 999; any other is `HC_ERR_OUT_OF_RANGE`, and a code that names no table `HC_ERR_UNKNOWN` |
| a count | `hc_holiday_business_days_between` | `from_fixed` and `to_fixed` −3 652 424 999 through 3 652 424 634, at most a hundred years apart; any other is `HC_ERR_OUT_OF_RANGE`, and a code that names no table `HC_ERR_UNKNOWN` |
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
| a byte length | `hc_ccsds_encode`, `hc_ccsds_ascii_format` | `tai_seconds` of an instant the format can hold, and attoseconds below 10¹⁸: for CUC from 1958-01-01 00:00:00 TAI, `tai_seconds` −378 691 200, to the format's last count; for CDS from 1958-01-01 UTC to the end of the day segment's last day; for CCS and the ASCII codes the years 1 to 9999; any other is `HC_ERR_OUT_OF_RANGE`, and under `strict` an instant outside the leap-second table `HC_ERR_NO_DATA` |
| a byte length | `hc_ccsds_decode_from_epoch` | `epoch_tai_seconds` −62 135 596 800 through 253 402 300 799 and `epoch_unix_day` −719 162 through 2 932 896, the instants and the days of the years 1 to 9999, and `epoch_attoseconds` below 10¹⁸; any other is `HC_ERR_OUT_OF_RANGE`, and a code as for `hc_ccsds_decode` |
| a byte length | `hc_ccsds_encode_from_epoch` | `epoch_tai_seconds` and `epoch_unix_day` as for `hc_ccsds_decode_from_epoch`, and `tai_seconds` of an instant the format can hold: at Level 2 from the epoch to the format's last count, at Level 1 as for `hc_ccsds_encode`; any other is `HC_ERR_OUT_OF_RANGE`, and under `strict` an instant outside the leap-second table `HC_ERR_NO_DATA` |
| a byte length | `hc_radio_decode` | `century` a multiple of 100 from 0 through 9 900; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_irig_decode` | `year` 1 through 9999, whose century a code with the year's two digits reads them in; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_irig_encode` | `fixed` 1 through 3 652 059, the days of the years 1 to 9999, with whole seconds up to 86 400, 23:59:60, and hundredths up to 99; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_radio_encode` | `unix_seconds` a whole minute from −62 135 596 800 (0001-01-01 00:00) through 253 402 300 740 (9999-12-31 23:59); any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_jjy_call_sign_decode` | `year` 1 through 9999, the year the frame's day of the year is read in; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_jjy_call_sign_encode` | `unix_seconds` a whole minute from −62 135 596 800 (0001-01-01 00:00) through 253 402 300 740 (9999-12-31 23:59) that is minute 15 or 45 of an hour of JST; any other is `HC_ERR_OUT_OF_RANGE` |
| ticks, 0 through 3 155 378 975 999 999 999 | `hc_dotnet_ticks_from_unix` | `unix_seconds` −62 135 596 800 through 253 402 300 799, 0001-01-01 to the end of 9999-12-31, and attoseconds below 10¹⁸; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_unix_from_dotnet_ticks` | `ticks` 0 through 3 155 378 975 999 999 999, the range of `DateTime`; any other is `HC_ERR_OUT_OF_RANGE` |
| a second of the day, 0 through 86 399 | `hc_civil_from_six_hour_clock` | no `i64` input: every reading of an hour 1 to 12, a minute and a second 0 to 59, and a half; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_describe_day` | every `fixed`; a calendar that refuses the day says so in its own line |
| a byte length | `hc_day_extras` | every `fixed`; a calendar that refuses the day writes no line, and an `id` the registry does not carry is `HC_ERR_UNKNOWN` |
| a byte length | `hc_calendar_units` | every `from_fixed` and `to_fixed` whose range is at most 100 000 units; a span a calendar refuses says so in its own line and counts as one, a `to_fixed` at or before `from_fixed` writes nothing, and a range of more units is `HC_ERR_OUT_OF_RANGE` (see [line caps](#line-caps)) |
| a byte length | `hc_calendars` | every `today` |
| a byte length | `hc_naming_period_on` | every `fixed`; a calendar the registry does not carry is `HC_ERR_UNKNOWN` |
| a byte length | `hc_parse_date` | no `i64` input: every text, a text that is not one day saying so in its line; a calendar the registry does not carry is `HC_ERR_UNKNOWN` |
| a byte length | `hc_asian_day` | `fixed` 1 360 (23 September AD 4) through 3 652 398, the last day of the Asian year 9999; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_holidays_in_year` | every `year`; a year the table has no entries or gaps for writes nothing |
| a byte length | `hc_holidays_on`, `hc_common_worship_on`, `hc_holidays_on_in` | `fixed` −3 652 424 999 through 3 652 424 634; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_roman_1960_office_on` | `fixed` 577 814 through 1 497 129, the Gregorian years 1583 to 4099; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_holy_year_on` | `fixed` 720 981 (24 December 1974) through 739 886 (27 September 2026), from the opening of the first jubilee the table carries to the day its sources were checked; any other is `HC_ERR_NO_DATA` |
| a byte length | `hc_orthodox_fast_on` | `fixed` 118 705 through 1 497 157 on `orthodox-fasts` and `armenian-fasts-jerusalem` and 118 705 through 1 497 128 on `orthodox-fasts-revised-julian`, the years 326 to 4099 of each reckoning's calendar, whose Pascha the Julian computus gives; 577 814 through 1 497 129, the Gregorian years 1583 to 4099, on `armenian-fasts`, `armenian-fasts-fifty-days`, `coptic-fasts` and `ethiopian-fasts`; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_orthodox_fast_seasons` | `year` 326 through 4099, or 1583 through 4099 on `armenian-fasts`, `coptic-fasts` and `ethiopian-fasts`; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_lectionary` | `fixed` 719 163 through 1 497 096, 1 January 1970, when the Roman calendar of 1969 went into effect, to the liturgical year 4099; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_term_in_effect`, `hc_pentad_in_effect`, `hc_solar_event`, `hc_sunrise`, `hc_sunset`, `hc_crescent_visible`, `hc_kalam`, `hc_muhurtas`, `hc_amrita_siddhi`, `hc_nakshatra_of_day`, `hc_almanac_cycles`, `hc_almanac_day`, `hc_almanac_directions`, `hc_mansion_undertakings`, `hc_prayer_times`, `hc_zmanim`, `hc_temporal_hour`, `hc_unix_from_edo_time`, `hc_choghadiya`, `hc_folk_day`, `hc_planetary_hours_of_day` | `fixed` −365 607 through 1 095 727, the years −1000 to 3000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_panchanga_of_day` | `fixed` −365 607 through 1 095 727, the years −1000 to 3000, on the true sky, and −1 132 604 through 2 519 974, Kali Yuga 1 to 10 000, on `surya-siddhanta`; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_hindu_lunar_date` | `fixed` in the Śaka years 1622 through 2221 on the true sky, from Chaitra śukla 1 in March 1700 to the eve of the one in March 2300, whose days move with the place and the ayanāṃśa (620 627 through 839 773 at the Central Station with Lahiri's); on `surya-siddhanta`, −1 132 604 through 2 519 974, Kali Yuga 1 to 10 000; any other is `HC_ERR_OUT_OF_RANGE`, as is a place beyond 65° of latitude; on the true sky, a day whose sunrise at the place the model does not find is `HC_ERR_NO_DATA` |
| a byte length | `hc_surya_siddhanta_sunrise` | `fixed` −1 132 604 through 2 519 974, Kali Yuga 1 to 10 000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_surya_siddhanta_at`, `hc_barhaspatya_year_at` | `unix_seconds` −159 992 668 800 through 155 590 156 799, the days of Kali Yuga 1 to 10 000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_barhaspatya_year` | `saka` −3178 through 6821, the expired Śaka years of Kali Yuga 1 to 10 000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_sky_at`, `hc_decan_at`, `hc_drekkana_at`, `hc_nakshatra_at`, `hc_solar_time`, `hc_edo_time`, `hc_panchak`, `hc_planetary_hour` | `unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_panchanga_at` | `unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000, on the true sky, and −159 992 668 800 through 155 590 156 799, Kali Yuga 1 to 10 000, on `surya-siddhanta`; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_pushkaram` | `entry_unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_kumbh` | `year` −1000 through 3000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_jupiter_at` | `unix_seconds` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_jupiter_ingresses`, `hc_jupiter_risings` | `from_unix_seconds` and `to_unix_seconds` −93 724 128 000 through 32 535 216 000 (the span `[from, to)` ends within the years −1000 to 3000), at most 3 155 760 000 apart, a hundred Julian years; a `to` not after the `from` writes nothing; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_kumbh_by_sky`, `hc_pushkaram_by_sky`, `hc_pushkarams_in_year` | `year` −1000 through 3000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_gmat_from_gmt`, `hc_gmt_from_gmat` | `fixed` −3 652 424 999 through 3 652 424 634, the Gregorian years −9 999 999 to 9 999 999, with whole seconds up to 86 400 and attoseconds below 10¹⁸; 23:59:60, which neither reckoning shifts, and any other are `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_solar_terms_between`, `hc_moon_phases_between` | `from_unix` −93 724 128 000 through 32 535 215 999, the years −1000 to 3000; a `to_unix` at or before it writes nothing, and a later one must be at most 32 535 216 000 and at most 400 years after it; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_chinese_marriage_augury` | `chinese_year` 4282 through 4786, whose New Year and the next both fall in the Chinese calendar's range (1645 through 2150); any other is `HC_ERR_OUT_OF_RANGE` |
| an age | `hc_chinese_age` | `birth_fixed` and `on_fixed` 600 460 through 785 271, the Chinese calendar's range, under `chinese-age` and `lichun-age`, and −3 652 424 999 through 3 652 424 634 under the others, a birth before or on the day asked; a day before the birth is `HC_ERR_NO_DATA`, and a day outside the range `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_chinese_almanac_solar_terms` | `year` 1645 through 1733 but 1667 to 1669; any other is `HC_ERR_NO_DATA` |
| an Olympiad, from 1 | `hc_ioc_olympiad` | `gregorian_year` from 1896, whose Olympiads run to 2 305 843 009 213 693 478 for the last `i64` year; an earlier year is `HC_ERR_OUT_OF_RANGE` |
| an Olympiad, from 1 | `hc_ioc_olympiad_on` | `fixed` from 692 231, the opening of 6 April 1896, through 3 652 424 634; an earlier day is `HC_ERR_OUT_OF_RANGE`, and one from 10 June to 21 November 1956 `HC_ERR_NO_DATA` |
| a byte length | `hc_babylonian_regnal_year` | `seleucid_year` −314 through 160; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_equinox_new_year_margin` | `year` in the calendar's own range, the years whose first days `hc_calendars` lists it for; any other is `HC_ERR_OUT_OF_RANGE`, and a calendar not named `HC_ERR_UNKNOWN` |
| a byte length | `hc_shmuel_tekufah` | `hebrew_year` 1 through 9999; any other is `HC_ERR_OUT_OF_RANGE`, and a *tekufah* not named `HC_ERR_UNKNOWN` |
| a byte length | `hc_day_name` | every `fixed` the calendar converts; a day it refuses is `HC_ERR_OUT_OF_RANGE`, and one the naming does not name `HC_ERR_NO_DATA` |
| a byte length | `hc_format_number` | every `value` the system writes: from 1 for the Hebrew numerals, every `i64` for a positional system; any other is `HC_ERR_OUT_OF_RANGE`, and a system not named `HC_ERR_UNKNOWN` |
| an integer | `hc_parse_number` | no `i64` input: text in the system's notation, whose value is above `HC_ERR_FLOOR`; any other is `HC_ERR_MALFORMED` or `HC_ERR_OUT_OF_RANGE`, and a system not named `HC_ERR_UNKNOWN` |
| a place in the cycle, 1 through 7 | `hc_hebrew_sabbatical_cycle_year` | `hebrew_year` 1 through 9999; any other is `HC_ERR_OUT_OF_RANGE` |
| a fixed day | `hc_hebrew_yahrzeit`, `hc_hebrew_birthday` | `death_fixed` and `birth_fixed` −1 373 427 through 2 278 650 and `hebrew_year` 1 through 9999, the Hebrew years 1 through 9999, and the anniversary lies among those days; any other is `HC_ERR_OUT_OF_RANGE` |
| an age, from 1 | `hc_chinese_reckoned_age` | `birth_fixed` and `on_fixed` 600 460 through 785 271, the days of the Chinese calendar's range, 1645 through 2150, a birth before or on the day asked; a day before the birth is `HC_ERR_NO_DATA`, and a day outside the range `HC_ERR_OUT_OF_RANGE` |
| a fixed day | `hc_astronomical_easter` | `year` 1583 through 2150, Easter falling between the fixed days 577 913 and 785 015; any other year is `HC_ERR_OUT_OF_RANGE` |
| a fixed day | `hc_astronomical_paschal_full_moon` | `year` 1583 through 2150, the years of `hc_astronomical_easter`; any other year is `HC_ERR_OUT_OF_RANGE` |
| a fixed day | `hc_plum_rains` | `year` −1000 through 3000, the days of 芒种 and 小暑 in the era of `hc_term_in_effect`; any other is `HC_ERR_OUT_OF_RANGE`, and a rule or meridian it does not name `HC_ERR_UNKNOWN` |
| a fixed day | `hc_rounichi` | `year` −999 through 3000, whose winter the era of `hc_term_in_effect` holds; any other is `HC_ERR_OUT_OF_RANGE`, a winter the reckoning does not settle `HC_ERR_NO_DATA`, and a rule or meridian it does not name `HC_ERR_UNKNOWN` |
| a byte length | `hc_almanac_person_days` | `fixed` and `birth_fixed` −365 607 through 1 095 727, the years −1000 to 3000; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_tibetan_almanac_day`, `hc_tibetan_planets` | `fixed` 364 892 through 1 095 802, the Tibetan years 1000 to 3000, and through 1 095 803 on `tibetan-bhutan`, `tibetan-tsurphu-karana` and `tibetan-bhutan-lochen`; any other is `HC_ERR_OUT_OF_RANGE` |
| a byte length | `hc_bhutanese_winter_solstice` | `year` 1000 through 3000; any other is `HC_ERR_OUT_OF_RANGE`, and one that holds no solstice, 1923, 1927, 1931, 1935, 1938, 1942, 1946, 1949, 1953 and 1957, `HC_ERR_NO_DATA` |
| a fixed day | `hc_tibetan_festival_day` | `year` 1000 through 3000; any other is `HC_ERR_OUT_OF_RANGE`, a month or day the year does not have `HC_ERR_INVALID_DATE`, and a skipped number under `henning-almanac` `HC_ERR_NO_DATA` |
| a fixed day | `hc_cold_food_day` | `year` −999 through 3000 under every reckoning, the years whose winter solstice before and whose April are both in the era of `hc_term_in_effect`; any other is `HC_ERR_OUT_OF_RANGE`, and a reckoning it does not name `HC_ERR_UNKNOWN` |
| 0 | `hc_zone_load` | any name and bytes; bytes that are not TZif are `HC_ERR_MALFORMED` |
| a byte length | `hc_zone_offset` | `unix_seconds` −315 631 497 830 400 through 315 507 195 014 399, the instants of `hc_fixed_from_unix_in_zone`; any other is `HC_ERR_OUT_OF_RANGE`, and a name neither the loaded zones nor the built-in table knows `HC_ERR_UNKNOWN` |
| a byte length | `hc_zone_name`, `hc_format_pattern` | `unix_seconds` −315 631 497 830 400 through 315 507 195 014 399, the instants of `hc_zone_offset`; any other is `HC_ERR_OUT_OF_RANGE`, and a zone or a field not known `HC_ERR_UNKNOWN` |
| a byte length | `hc_mars_time`, `hc_body_time` | no `i64` input: the instants within 100 Julian years of J2000.0 (J1900.0, 1899-12-31T12:00 TT, to 2100-01-01T12:00 TT); any other, or an instant or longitude not finite, is `HC_ERR_OUT_OF_RANGE`, and for `hc_body_time` a body `hc_bodies` does not list `HC_ERR_UNKNOWN` and the Sun `HC_ERR_NO_DATA` |
| a byte length | `hc_zones`, `hc_zone_location` | no `i64` input: every locale tag, and for `hc_zone_location` every name `zone1970.tab`, `zone.tab` or `backward` places; a name they do not, such as `UTC`, is `HC_ERR_UNKNOWN` |
| a byte length | `hc_territories`, `hc_subdivisions`, `hc_place_name` | no `i64` input: every locale tag, every territory's code as `country`, and for `hc_place_name` every code of a territory or, in ISO form, of a subdivision CLDR 48 names; any other is `HC_ERR_UNKNOWN` |
| a byte length | `hc_relative_time` | every `then_unix` and `now_unix` less than an `i64` of seconds apart; two further apart are `HC_ERR_OUT_OF_RANGE`, and a style not named `HC_ERR_UNKNOWN` |
| a byte length | `hc_relative_day`, `hc_relative_day_at` | every `then_fixed` and `now_fixed` less than an `i64` of days apart, and for `hc_relative_day_at` seconds of the day below 86 400; any other is `HC_ERR_OUT_OF_RANGE`, and a style not named `HC_ERR_UNKNOWN` |
| a byte length | `hc_duration` | every `seconds`; a style not named is `HC_ERR_UNKNOWN` |
| a byte length | `hc_circad_date` | no `i64` input: the instants within 100 Julian years of J2000.0 (J1900.0, 1899-12-31T12:00 TT, to 2100-01-01T12:00 TT), as for `hc_mars_time`; any other, or one not finite, is `HC_ERR_OUT_OF_RANGE`, and a calendar not listed `HC_ERR_UNKNOWN` |
| a mission sol, from 0 or 1 | `hc_mission_sol` | the instants from the midnight that began the mission's landing sol through 100 Julian years after J2000.0 (2100-01-01T12:00 TT); an earlier instant, or one not finite, is `HC_ERR_OUT_OF_RANGE`, a mission whose operators published no sol numbering `HC_ERR_NO_DATA`, and a mission the table does not carry `HC_ERR_UNKNOWN` |
| a byte length | `hc_version`, `hc_calendar_list`, `hc_locales`, `hc_gregorian_adoption`, `hc_holiday_codes`, `hc_holiday_tables`, `hc_place_years_ago`, `hc_cosmic_events`, `hc_earliest_evidence`, `hc_archaeological_periods`, `hc_future_events`, `hc_geologic_intervals`, `hc_orbit_at`, `hc_orbit_series`, `hc_earth_rotation_angle`, `hc_gmst_iau2006`, `hc_gmst_iau1982`, `hc_ut2_minus_ut1`, `hc_hjd_tt`, `hc_hjd_utc`, `hc_horizons`, `hc_missions`, `hc_bodies`, `hc_proper_time`, `hc_gravitational_dilation`, `hc_gravitating_bodies`, `hc_ccsds_decode`, `hc_ccsds_ascii_parse`, `hc_six_hour_clock`, `hc_prayer_methods`, `hc_night_watch`, `hc_irig_formats`, `hc_french_decimal_time`, `hc_civil_from_french_decimal_time`, `hc_day_period`, `hc_numbering_systems`, `hc_calendar_eras`, `hc_holiday_groups`, `hc_irig_frame_start` | no `i64` input: text, or `f64` values whose range each export's documentation states; a length is never negative, so it never nears the floor |

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
tables; `hc_describe_day` one line per calendar, and `hc_day_extras` one
per extra field of the day, a few hundred across the calendars.

## Lines and cells

Every export that answers with more than one value writes UTF-8 lines, one
per entry, each ending in `\n`, with the cells of a line separated by `\t`.
The columns of each export are stated below and on the export. Before 1.0
they are not frozen: a release may add a column at any position, remove one
or change what one holds, and the pull request that does so lists each such
change under the export's name. A page that reads columns by position reads
them from this README's table of the same version, as the JavaScript
wrapper does. Where a line names an entry that has a stable identifier, the
identifier is in column 1 — a calendar's `id`, a locale's `tag`, a holiday
table's `code` or `table`, a `zone`, a horizon's, a mission's, a body's, a
gravitating body's `id`, an off-Earth `calendar` — or in column 2 after a
column that says what kind of line it is, or whose entry it belongs to:
a deep-time line's `kind` and `id`, a renamed-month line's `state` and
`period`, a day's extra field's calendar `id` and `field`. A cell with nothing to
say is empty, never a placeholder, and no cell contains a tab or a line
break: the few source strings that carry one have it replaced by a space.
Numbers are written in plain decimal notation, however large or small.

Called with a null `buffer`, such an export returns the byte length the text
needs, so the caller can allocate exactly and call again; called with a
buffer that is too small it returns `HC_ERR_BUFFER_TOO_SMALL` and writes
nothing.

## Identifiers

An argument that names something — a calendar, a convention, a holiday
table, a zone, a horizon, a method — is matched by one rule: the white
space around it is ignored and its ASCII letters match in either case, so
`" Gregory "` names `gregory` and `us` names `US`. It is
`hc_core::catalogue::matches`, and every lookup behind an export follows
it (`docs/policy.md` §5). A name no table carries is `HC_ERR_UNKNOWN`.

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
| `dotnetTicksFromUnix(unixSeconds, attoseconds)`, `unixFromDotnetTicks(ticks)` | `hc_dotnet_ticks_from_unix`, `hc_unix_from_dotnet_ticks` | ticks as a `bigint`; a `DotnetReading` |
| `frenchDecimalTime(secondsOfDay, attoseconds)`, `civilFromFrenchDecimalTime(hour, minute, second, attoseconds)` | `hc_french_decimal_time`, `hc_civil_from_french_decimal_time` | a `FrenchDecimalTime`; a `TimeOfDay` |
| `sixHourClock(reckoning, secondsOfDay)`, `civilFromSixHourClock(reckoning, hour, minute, second, night)` | `hc_six_hour_clock`, `hc_civil_from_six_hour_clock` | a `SixHourReading`; seconds of the civil day |
| `ccsdsDecode(hex, strict)`, `ccsdsEncode(taiSeconds, attoseconds, pField, strict)` | `hc_ccsds_decode`, `hc_ccsds_encode` | a `CcsdsCode`; a hexadecimal string |
| `ccsdsDecodeFromEpoch(hex, epoch, strict)`, `ccsdsEncodeFromEpoch(taiSeconds, attoseconds, pField, epoch, strict)` | `hc_ccsds_decode_from_epoch`, `hc_ccsds_encode_from_epoch` | a `CcsdsCode`; a hexadecimal string |
| `ccsdsAsciiParse(code, strict)`, `ccsdsAsciiFormat(taiSeconds, attoseconds, variation, precision, terminator, strict)` | `hc_ccsds_ascii_parse`, `hc_ccsds_ascii_format` | a `CcsdsAsciiCode`; a string |
| `radioDecode(code, frame, century)`, `radioEncode(code, unixSeconds, options)` | `hc_radio_decode`, `hc_radio_encode` | a `RadioMinute`; a frame string |
| `jjyCallSignDecode(frame, year)`, `jjyCallSignEncode(unixSeconds, notice)` | `hc_jjy_call_sign_decode`, `hc_jjy_call_sign_encode` | a `JjyCallSign`; a frame string |
| `irigDecode(signal, frame, year)`, `irigEncode(signal, fixed, secondsOfDay, options)`, `irigFormats()` | `hc_irig_decode`, `hc_irig_encode`, `hc_irig_formats` | an `IrigReading`; a frame string; `IrigFormatInfo[]` |
| `irigFrameStart(signal, secondsOfDay, hundredths)` | `hc_irig_frame_start` | an `IrigFrameStart` |
| `describeDay(fixed, locale)` | `hc_describe_day` | `DescribedDay[]`, one per calendar |
| `parseDate(calendar, locale, text)` | `hc_parse_date` | a `ParsedDate`: a `DescribedDay` and its `fixed` day |
| `dayExtras(fixed, locale, id)` | `hc_day_extras` | `DayExtra[]`, one per extra field |
| `calendarUnits(id, unit, from, to, locale)` | `hc_calendar_units` | `CalendarUnit[]`, one per span |
| `calendars(today, locale)` | `hc_calendars` | `CalendarEntry[]`, one per calendar |
| `calendarList(locale)` | `hc_calendar_list` | `CalendarListEntry[]`, one per calendar |
| `locales()` | `hc_locales` | `LocaleEntry[]`, one per locale |
| `firstDayOfWeek(locale)` | `hc_first_day_of_week` | a number, Monday = 1 through Sunday = 7 |
| `dayPeriod(secondsOfDay, locale)`, `formatNumber(system, value)`, `parseNumber(system, text)`, `numberingSystems()`, `calendarEras(calendar, locale)` | `hc_day_period`, `hc_format_number`, `hc_parse_number`, `hc_numbering_systems`, `hc_calendar_eras` | a `DayPeriodReading`; a string; a number; `NumberingSystemInfo[]`; `CalendarEra[]` |
| `gregorianAdoption(region)` | `hc_gregorian_adoption` | `GregorianAdoption[]`, one per step |
| `namingPeriodOn(calendar, fixed, locale)` | `hc_naming_period_on` | a `NamingPeriodOn` |
| `panchangaAt(unixSeconds, ayanamsa)`, `panchangaOfDay(fixed, latitude, longitude, elevation, ayanamsa)` | `hc_panchanga_at`, `hc_panchanga_of_day` | `PanchangaLimb[]`, the yoga's and the karaṇa's |
| `hinduLunarDate(sky, fixed, latitude, longitude, elevation, locale)`, `suryaSiddhantaAt(unixSeconds)`, `suryaSiddhantaSunrise(fixed, latitude, longitude)` | `hc_hindu_lunar_date`, `hc_surya_siddhanta_at`, `hc_surya_siddhanta_sunrise` | a `HinduLunarDate`; a `SuryaSiddhantaSky`; a number |
| `barhaspatyaYear(rule, saka, locale)`, `barhaspatyaYearAt(rule, unixSeconds, locale)` | `hc_barhaspatya_year`, `hc_barhaspatya_year_at` | a `BarhaspatyaYear`; a `BarhaspatyaNameAt` |
| `muhurtas(fixed, latitude, longitude, elevation)`, `amritaSiddhi(fixed, latitude, longitude, elevation, ayanamsa)`, `nakshatraAt(unixSeconds, ayanamsa)`, `nakshatraOfDay(fixed, latitude, longitude, elevation, ayanamsa)` | `hc_muhurtas`, `hc_amrita_siddhi`, `hc_nakshatra_at`, `hc_nakshatra_of_day` | `Muhurta[]`; an `AmritaSiddhi`; a `NakshatraStay`; a `NakshatraStay` |
| `crescentVisible(criterion, fixed, latitude, longitude, elevation)` | `hc_crescent_visible` | a `CrescentVisibility` |
| `equinoxNewYearMargin(calendar, year)` | `hc_equinox_new_year_margin` | an `EquinoxMargin` |
| `iocOlympiadOn(fixed)`, `babylonianRegnalYear(seleucidYear)`, `shmuelTekufah(hebrewYear, tekufah)`, `dayName(calendar, naming, fixed)` | `hc_ioc_olympiad_on`, `hc_babylonian_regnal_year`, `hc_shmuel_tekufah`, `hc_day_name` | a number; a `RegnalYear`; a `ShmuelTekufah`; a `DayName` |
| `iocOlympiad(gregorianYear)`, `hebrewYahrzeit(deathFixed, hebrewYear)`, `hebrewBirthday(birthFixed, hebrewYear)`, `chineseReckonedAge(birthFixed, onFixed)` | `hc_ioc_olympiad`, `hc_hebrew_yahrzeit`, `hc_hebrew_birthday`, `hc_chinese_reckoned_age` | a number |
| `chineseAge(convention, birthFixed, onFixed)`, `chineseAlmanacSolarTerms(year)` | `hc_chinese_age`, `hc_chinese_almanac_solar_terms` | a number; `AlmanacSolarTerm[]` |
| `chineseMarriageAugury(chineseYear)` | `hc_chinese_marriage_augury` | a `MarriageAugury` |
| `hebrewSabbaticalCycleYear(hebrewYear)` | `hc_hebrew_sabbatical_cycle_year` | a number, 1 through 7 |
| `asianDay(fixed)` | `hc_asian_day` | an `AsianDay` |
| `kalam(convention, fixed, latitude, longitude, elevation, locale)` | `hc_kalam` | `KalamPeriod[]`, Rāhu kālam, Yamaganda and Gulika kālam |
| `choghadiya(fixed, latitude, longitude, elevation, locale)` | `hc_choghadiya` | `ChoghadiyaPart[]`, the day's eight and the night's |
| `panchak(naming, unixSeconds, ayanamsa, offsetSeconds, locale)` | `hc_panchak` | a `PanchakWindow` |
| `kumbh(yoga, year, ayanamsa, jupiter, locale)`, `pushkaram(sign, entryUnixSeconds, latitude, longitude, elevation, meridian, locale)` | `hc_kumbh`, `hc_pushkaram` | a `KumbhOccasion`; `PushkaramDays[]`, one per river |
| `almanacCycles(fixed, meridian)`, `almanacDay(fixed, meridian, locale)` | `hc_almanac_cycles`, `hc_almanac_day` | an `AlmanacCycles`; `AlmanacAnnotation[]`, one per annotation |
| `almanacDirections(fixed, meridian)`, `rounichi(rule, year, meridian)`, `mansionUndertakings(list, fixed)`, `almanacPersonDays(fixed, birthFixed, meridian)` | `hc_almanac_directions`, `hc_rounichi`, `hc_mansion_undertakings`, `hc_almanac_person_days` | `AlmanacDirection[]`; a fixed day; `MansionUndertaking[]`; `AlmanacPersonDay[]` |
| `tibetanAlmanacDay(calendar, fixed)`, `tibetanPlanets(fixed)`, `bhutaneseWinterSolstice(year)`, `tibetanFestivalDay(rule, calendar, year, month, leap, day)` | `hc_tibetan_almanac_day`, `hc_tibetan_planets`, `hc_bhutanese_winter_solstice`, `hc_tibetan_festival_day` | `TibetanAlmanacEntry[]`; `TibetanPlanet[]`; a `BhutaneseWinterSolstice`; a fixed day |
| `folkDay(fixed, meridian, locale)`, `nightWatch(secondsOfDay, locale)` | `hc_folk_day`, `hc_night_watch` | `FolkDay[]`, one per reckoning; a `NightWatch` or `null` |
| `holidayIsDayOff(code, region, fixed, group)` | `hc_holiday_is_day_off` | a boolean |
| `holidayAddBusinessDays(code, region, fixed, count, group)`, `holidayBusinessDaysBetween(code, region, fromFixed, toFixed, group)` | `hc_holiday_add_business_days`, `hc_holiday_business_days_between` | a fixed day; a number |
| `holidaysInYear(code, region, year, group, kind)` | `hc_holidays_in_year` | `HolidayInYear[]` |
| `holidayCodes()` | `hc_holiday_codes` | `string[]` |
| `holidaysOn(fixed)` | `hc_holidays_on` | `HolidayOn[]` |
| `holidayTables(locale)` | `hc_holiday_tables` | `HolidayTable[]` |
| `holidayGroups(locale)`, `holidaysOnIn(fixed, locale)` | `hc_holiday_groups`, `hc_holidays_on_in` | `HolidayGroup[]`; `HolidayOnIn[]` |
| `lectionary(fixed)`, `astronomicalEaster(year)`, `astronomicalPaschalFullMoon(year)` | `hc_lectionary`, `hc_astronomical_easter`, `hc_astronomical_paschal_full_moon` | a `Lectionary`; a fixed day number |
| `holyYearOn(fixed)`, `commonWorshipOn(fixed)`, `roman1960OfficeOn(fixed)` | `hc_holy_year_on`, `hc_common_worship_on`, `hc_roman_1960_office_on` | a `HolyYear` or `null`; `CommonWorshipCelebration[]`; `Roman1960Office[]` |
| `orthodoxFastOn(reckoning, fixed)`, `orthodoxFastSeasons(reckoning, year)` | `hc_orthodox_fast_on`, `hc_orthodox_fast_seasons` | an `OrthodoxFastDay`; `OrthodoxFastSeason[]` |
| `termInEffect(fixed, meridian)`, `pentadInEffect(fixed, meridian)` | `hc_term_in_effect`, `hc_pentad_in_effect` | a `TermInEffect` |
| `coldFoodDay(convention, year)` | `hc_cold_food_day` | a fixed day number |
| `plumRains(rule, year, meridian)` | `hc_plum_rains` | a fixed day number |
| `placeYearsAgo(yearsAgo, stdDevYears, locale)`, `cosmicEvents(locale)`, `archaeologicalPeriods(locale)`, `futureEvents(locale)`, `geologicIntervals(rank, locale)` | `hc_place_years_ago`, `hc_cosmic_events`, `hc_archaeological_periods`, `hc_future_events`, `hc_geologic_intervals` | `DeepTimeRow[]` |
| `earliestEvidence(locale)` | `hc_earliest_evidence` | `EarliestEvidenceRow[]`: a `DeepTimeRow` whose `stdDev` may be `null` and whose `start` is `null` for a minimum age |
| `fixedFromUnixInZone(unixSeconds, zone)`, `unixFromFixedInZone(fixed, zone)` | `hc_fixed_from_unix_in_zone`, `hc_unix_from_fixed_in_zone` | a number |
| `loadZone(name, tzif)` | `hc_zone_load` | nothing |
| `zones(locale)`, `zoneLocation(zone, locale)` | `hc_zones`, `hc_zone_location` | `ZoneLocation[]`; a `ZoneLocation` |
| `zoneOffset(zone, unixSeconds)` | `hc_zone_offset` | a `ZoneOffset` |
| `zoneName(zone, unixSeconds, locale, field)`, `formatPattern(zone, unixSeconds, locale, syntax, pattern)` | `hc_zone_name`, `hc_format_pattern` | a `ZoneName`; a `FormattedInZone` |
| `skyAt(unixSeconds)` | `hc_sky_at` | a `Sky` |
| `solarTermsBetween(fromUnix, toUnix)`, `moonPhasesBetween(fromUnix, toUnix)` | `hc_solar_terms_between`, `hc_moon_phases_between` | `SkyEvent[]` |
| `decanAt(unixSeconds)`, `drekkanaAt(unixSeconds, ayanamsa)` | `hc_decan_at`, `hc_drekkana_at` | a `Decan`; a `Drekkana` |
| `earthRotationAngle(ut1UnixSeconds)`, `gmstIau2006(ut1UnixSeconds)`, `gmstIau1982(ut1UnixSeconds)`, `ut2MinusUt1(ut1UnixSeconds)` | `hc_earth_rotation_angle`, `hc_gmst_iau2006`, `hc_gmst_iau1982`, `hc_ut2_minus_ut1` | a number |
| `solarTime(clock, unixSeconds, latitude, longitude, elevation)`, `solarEvent(event, fixed, latitude, longitude, elevation)` | `hc_solar_time`, `hc_solar_event` | a `SolarTime`; a `SolarEvent` |
| `horizons(locale)`, `sunrise(horizon, fixed, latitude, longitude, elevation)`, `sunset(horizon, fixed, latitude, longitude, elevation)` | `hc_horizons`, `hc_sunrise`, `hc_sunset` | `Horizon[]`; a `SolarCrossing` |
| `hjdTt(ttJulianDate, rightAscension, declination)`, `hjdUtc(utcJulianDate, rightAscension, declination, strict)` | `hc_hjd_tt`, `hc_hjd_utc` | a `HeliocentricJulianDate`; a `HeliocentricJulianDateUtc` |
| `gmatFromGmt(fixed, secondsOfDay, attoseconds)`, `gmtFromGmat(fixed, secondsOfDay, attoseconds)` | `hc_gmat_from_gmt`, `hc_gmt_from_gmat` | a `ClockReading`, its attoseconds a `bigint` |
| `prayerTimes(method, fixed, latitude, longitude, elevation, ramadan)`, `prayerMethods()` | `hc_prayer_times`, `hc_prayer_methods` | `PrayerTime[]`; `PrayerMethod[]` |
| `zmanim(reckoning, fixed, latitude, longitude, elevation)` | `hc_zmanim` | `Zman[]` |
| `temporalHour(reckoning, fixed, latitude, longitude, elevation)` | `hc_temporal_hour` | a `TemporalHour` |
| `edoTime(unixSeconds, latitude, longitude, elevation)`, `unixFromEdoTime(fixed, hour, fraction, latitude, longitude, elevation)` | `hc_edo_time`, `hc_unix_from_edo_time` | an `EdoTime`; a `SolarEvent` |
| `planetaryHour(unixSeconds, latitude, longitude, elevation, locale)`, `planetaryHoursOfDay(fixed, latitude, longitude, elevation, locale)` | `hc_planetary_hour`, `hc_planetary_hours_of_day` | a `PlanetaryHour`; `PlanetaryHour[]`, twenty-four |
| `orbitAt(yearsBefore1950)`, `orbitSeries(fromYearsBefore1950, toYearsBefore1950, stepYears)` | `hc_orbit_at`, `hc_orbit_series` | an `Orbit`; `OrbitSample[]` |
| `jupiterAt(unixSeconds, ayanamsa)`, `jupiterIngresses(fromUnixSeconds, toUnixSeconds, ayanamsa)`, `jupiterRisings(fromUnixSeconds, toUnixSeconds, ayanamsa)`, `kumbhBySky(yoga, year, ayanamsa, locale)`, `pushkaramBySky(sign, year, ayanamsa, rule, latitude, longitude, elevation, meridian, locale)`, `pushkaramsInYear(year, ayanamsa, rule, latitude, longitude, elevation, meridian, locale)` | `hc_jupiter_at`, `hc_jupiter_ingresses`, `hc_jupiter_risings`, `hc_kumbh_by_sky`, `hc_pushkaram_by_sky`, `hc_pushkarams_in_year` | a `JupiterPosition`; `JupiterIngress[]`; `JupiterRising[]`; a `KumbhBySky`; `PushkaramBySky[]`, one per river; `PushkaramBySky[]`, one per river of each sign entered |
| `marsTime(unixSeconds, eastLongitude)`, `missions()`, `missionSol(mission, unixSeconds)` | `hc_mars_time`, `hc_missions`, `hc_mission_sol` | a `MarsTime`; `Mission[]`; a number |
| `bodies()`, `bodyTime(body, unixSeconds, eastLongitude)` | `hc_bodies`, `hc_body_time` | `Body[]`; a `BodyTime` |
| `circadDate(calendar, unixSeconds)` | `hc_circad_date` | a `CircadDate` |
| `properTime(speedMetresPerSecond, coordinateSeconds)`, `gravitationalDilation(body, radiusMetres)`, `gravitatingBodies()` | `hc_proper_time`, `hc_gravitational_dilation`, `hc_gravitating_bodies` | a `ProperTime`; a `GravitationalDilation`; `GravitatingBody[]` |
| `territories(locale)`, `subdivisions(country, locale)`, `placeName(code, locale)` | `hc_territories`, `hc_subdivisions`, `hc_place_name` | `PlaceName[]`; `PlaceName[]`; a `PlaceName` |
| `relativeTime(thenUnix, nowUnix, style, automatic, locale)`, `relativeDay(thenFixed, nowFixed, style, automatic, locale)`, `relativeDayAt(thenFixed, nowFixed, secondsOfDay, style, automatic, locale)`, `duration(seconds, style, maxComponents, locale)` | `hc_relative_time`, `hc_relative_day`, `hc_relative_day_at`, `hc_duration` | a `RelativeTime`; a `RelativeTime`; a `RelativeDayAt`; a `HumanizedDuration` |

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
  argument may be a number or a `BigInt`; an integer number past
  `Number.MAX_SAFE_INTEGER`, which has already lost its last digits, is
  refused as `unsafe-integer` too, an `HcError` whose `code` and `export`
  are `null`, so that one `catch` of `HcError` takes both, and a `BigInt`
  passes such a value exactly. A number that is not an integer, a `BigInt`
  beyond the argument's type and a value of another type are a
  `TypeError`, as a mistake in the calling code. The one exception is a TAI
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
and it is 9.97 MiB (10,453,093 bytes) for the `full` layer of 2026-09-29,
base64 being four thirds of the module; the `places` layer is two fifths of it.

### tzdata beside the module

The module's eighteen built-in zones carry only their current rules (the
time zones section below). For a zone's history, or any other zone, a page
hands `loadZone(name, bytes)` the zone's TZif file, and

```sh
scripts/wasm-tzdata.sh                  # [<output directory>]
```

copies the eighteen from the host's `/usr/share/zoneinfo` (or `ZONEINFO`)
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
| `civil` *(default)* | Gregorian dates, ISO 8601 text, POSIX time, TAI − UTC and leap seconds | `hc-calendar`, `hc-calendars-solar`, `hc-format` | 39,180 | 38 KiB |
| `timestamps` | `hc_tai_from_unix`, `hc_utc_from_tai`, `hc_tai64_encode`, `hc_tai64_decode`, `hc_tai64_posix_plus_10_encode`, `hc_tai64_posix_plus_10_decode`, `hc_gnss_week`, `hc_gnss_to_tai`, `hc_gnss_resolve_week`, `hc_glonass_date`, `hc_fixed_from_ole_automation`, `hc_ole_automation_from_fixed`, `hc_excel_1900_day`, `hc_uuid_timestamp`, `hc_ntp_resolve`, `hc_uuid_timestamp_encode`, `hc_ntp_encode`, `hc_fat_decode`, `hc_fat_encode`, `hc_swatch_beat`, `hc_epoch_from_tt`, `hc_tt_from_epoch`, `hc_tt_bipm`, `hc_dotnet_ticks_from_unix`, `hc_unix_from_dotnet_ticks`, `hc_six_hour_clock`, `hc_civil_from_six_hour_clock`, `hc_french_decimal_time`, `hc_civil_from_french_decimal_time`: POSIX time to and from TAI, TAI64 labels in both conventions, GNSS weeks, GLONASS dates, OLE Automation dates, Excel 1900 serials, UUID timestamps, NTP eras, FAT date and time words, Swatch Internet Time, Julian and Besselian epochs, TT(BIPM) from a caller's series, .NET ticks, and the Ethiopian and Swahili six-hour clocks | nothing beyond `civil`'s crates: `hc-core`'s `tai64`, `gnss`, `uuid`, `ntp`, `internet_time`, `epoch_notation`, `tt_bipm` and `dotnet`, `hc-calendars-solar`'s `spreadsheet`, `hc-format`'s `fat` and `east_african_hours` | 123,090 | 120 KiB |
| `time-codes` | `hc_ccsds_decode`, `hc_ccsds_encode`, `hc_ccsds_decode_from_epoch`, `hc_ccsds_encode_from_epoch`, `hc_ccsds_ascii_parse`, `hc_ccsds_ascii_format`, `hc_radio_decode`, `hc_radio_encode`, `hc_jjy_call_sign_decode`, `hc_jjy_call_sign_encode`, `hc_irig_decode`, `hc_irig_encode`, `hc_irig_formats`, `hc_irig_frame_start`: the CCSDS time codes, binary and ASCII, the long-wave radio time codes of JJY, DCF77 and WWVB, and the IRIG serial time codes, read and written; a layer of its own so that `timestamps` stays small | nothing beyond `civil`'s crates: `hc-core`'s `ccsds`, `hc-format`'s `ccsds`, `radio` and `irig` | 114,558 | 112 KiB |
| `calendars` | `hc_describe_day`, `hc_day_extras`, `hc_calendar_units`, `hc_parse_date`, `hc_calendars`, `hc_calendar_list`, `hc_locales`, `hc_first_day_of_week`, `hc_day_period`, `hc_format_number`, `hc_parse_number`, `hc_numbering_systems`, `hc_calendar_eras`, `hc_gregorian_adoption`, `hc_naming_period_on`: every registered calendar described for one day, walked as eras, years, months and days, and listed, in a locale, and a date written in one read back; the locales and the day each one's week begins on; when each country adopted the Gregorian calendar; and the month and weekday names a government decreed for a period; `hc_panchanga_at`, `hc_panchanga_of_day`, `hc_muhurtas`, `hc_amrita_siddhi`, `hc_nakshatra_at`, `hc_nakshatra_of_day`, `hc_hindu_lunar_date`, `hc_surya_siddhanta_at`, `hc_surya_siddhanta_sunrise`, `hc_crescent_visible`, `hc_ioc_olympiad`, `hc_ioc_olympiad_on`, `hc_babylonian_regnal_year`, `hc_equinox_new_year_margin`, `hc_shmuel_tekufah`, `hc_day_name`, `hc_hebrew_yahrzeit`, `hc_hebrew_birthday`, `hc_hebrew_sabbatical_cycle_year`, `hc_chinese_reckoned_age`, `hc_chinese_marriage_augury`, `hc_chinese_age`, `hc_chinese_almanac_solar_terms`, `hc_asian_day`, `hc_kalam`, `hc_almanac_cycles`, `hc_almanac_day`, `hc_almanac_directions`, `hc_rounichi`, `hc_mansion_undertakings`, `hc_almanac_person_days`, `hc_tibetan_almanac_day`, `hc_tibetan_planets`, `hc_bhutanese_winter_solstice`, `hc_tibetan_festival_day`; `hc_barhaspatya_year`, `hc_barhaspatya_year_at`, `hc_choghadiya`, `hc_panchak`, `hc_kumbh`, `hc_pushkaram`, `hc_folk_day`, `hc_night_watch`: the northern year's name, the choghadiya, Panchak, the Kumbh and Pushkaram conditions, the folk days and the night watches, each named in a locale | every `hc-calendars-*` crate, `hc-astro`, `hc-almanac`, `hc-i18n`, `hc-format`; and every locale's exemplar cities, which only a build with `tz` too carries | 1,437,434 | 1.37 MiB |
| `holiday` | `hc_holiday_is_day_off`, `hc_holiday_add_business_days`, `hc_holiday_business_days_between`, `hc_holidays_in_year`, `hc_holiday_codes`, `hc_holidays_on`, `hc_holiday_tables`, `hc_holiday_groups`, `hc_holidays_on_in`, `hc_lectionary`, `hc_astronomical_easter`, `hc_astronomical_paschal_full_moon`, `hc_holy_year_on`, `hc_common_worship_on`, `hc_roman_1960_office_on`, `hc_orthodox_fast_on`, `hc_orthodox_fast_seasons` | `hc-holiday` and everything it dates by | 2,420,206 | 2.31 MiB |
| `seasons` | `hc_term_in_effect`, `hc_pentad_in_effect`, `hc_cold_food_day`, `hc_plum_rains` | `hc-seasons`, `hc-astro` | 91,396 | 89 KiB |
| `deep-time` | `hc_place_years_ago`, `hc_cosmic_events`, `hc_earliest_evidence`, `hc_archaeological_periods`, `hc_future_events`, `hc_geologic_intervals` | `hc-deep-time`, `hc-uncertainty` | 183,774 | 179 KiB |
| `tz` | `hc_fixed_from_unix_in_zone`, `hc_unix_from_fixed_in_zone`, `hc_zone_load`, `hc_zone_offset`, `hc_zones`, `hc_zone_location`: the day and the offset by a zone's rules, and where each zone is, with its exemplar city in English, or in the locale when the build has `calendars` or `zone-names` too | `hc-tz`, and `hc-i18n`'s English exemplar cities | 99,078 | 97 KiB |
| `sky` | `hc_sky_at`, `hc_solar_terms_between`, `hc_moon_phases_between`, `hc_decan_at`, `hc_drekkana_at`, `hc_earth_rotation_angle`, `hc_gmst_iau2006`, `hc_gmst_iau1982`, `hc_ut2_minus_ut1`, `hc_solar_time`, `hc_solar_event`, `hc_horizons`, `hc_sunrise`, `hc_sunset`, `hc_hjd_tt`, `hc_hjd_utc`, `hc_gmat_from_gmt`, `hc_gmt_from_gmat`, `hc_prayer_times`, `hc_prayer_methods`, `hc_zmanim`, `hc_temporal_hour`, `hc_edo_time`, `hc_unix_from_edo_time`, `hc_planetary_hour`, `hc_planetary_hours_of_day` | `hc-astro`, `hc-seasons`, and `hc-i18n`'s names of the horizons and the planets | 164,253 | 160 KiB |
| `orbital` | `hc_orbit_at`, `hc_orbit_series` | `hc-orbital`, `hc-uncertainty` | 64,650 | 63 KiB |
| `jupiter` | `hc_jupiter_at`, `hc_jupiter_ingresses`, `hc_jupiter_risings`, `hc_kumbh_by_sky`, `hc_pushkaram_by_sky`, `hc_pushkarams_in_year`: where Jupiter is, tropical and sidereal; its entries into the sidereal signs and its heliacal risings; and the Kumbh Mela and Pushkaram found from them, where `hc_kumbh` and `hc_pushkaram` take Jupiter's sign from the caller | `hc-astro`'s `jupiter` and `vsop87_jupiter` (3 625 terms of VSOP87B, 55 kB of tables), `hc-seasons`, `hc-calendars-indic`, `hc-i18n` | 216,294 | 211 KiB |
| `planetary` | `hc_mars_time`, `hc_missions`, `hc_mission_sol`, `hc_bodies`, `hc_body_time`, `hc_circad_date`: Mars time, the Darian date, the surface missions' sols, the solar day and local time of every body in `hc-planetary`'s table, and the dates of the Titan, Galilean and Martiana calendars | `hc-planetary`, `hc-astro` | 96,728 | 94 KiB |
| `relativity` | `hc_proper_time`, `hc_gravitational_dilation`, `hc_gravitating_bodies` | `hc-relativity`, `hc-uncertainty` | 52,822 | 52 KiB |
| `places` | `hc_territories`, `hc_subdivisions`, `hc_place_name`: what each carried locale calls every territory and every ISO 3166-2 subdivision CLDR 48 names | `hc-i18n`'s `place_names`: 2.8 MB of names, 2.6 MB of them the subdivisions' | 2,968,604 | 2.83 MiB |
| `humanize` | `hc_relative_time`, `hc_relative_day`, `hc_relative_day_at`, `hc_duration`: how one instant reads from another, which calendar day a day is seen from another, with a time of day, and how long a span is, in every locale `hc-humanize` carries | `hc-humanize`, `hc-i18n` | 651,206 | 636 KiB |
| `zone-names` | `hc_zone_name`, `hc_format_pattern`: a zone's name at an instant in a locale, as the CLDR fields `z`, `O`, `v` and `V` write it, from CLDR 48's metazones and names in every carried locale | `hc-tz`, `hc-format`'s `patterns::zone`, `hc-i18n`'s `zone_names` and every locale's exemplar cities: about 600 kB of names | 1,643,480 | 1.57 MiB |
| `full` | all of the above, `places` included, and nothing else: the facade's own `full`, whose extra crates no export reads, is not enabled | everything the layers above bring in | 7,887,481 | 7.52 MiB |

The sizes are of the `release-compact` profile for
`wasm32-unknown-unknown`, as [`scripts/wasm-layers.sh`](../../scripts/wasm-layers.sh)
printed them on 2026-10-03 with rustc 1.98.1:

```sh
scripts/wasm-layers.sh
# cargo build -p hyper-calendar-wasm --target wasm32-unknown-unknown --profile release-compact --no-default-features --features <layer>
```

The script leaves each layer at `target/wasm-layers/hyper_calendar_wasm.<feature>.wasm`
and prints the table; CI runs it on every pull request and uploads the
seventeen files, the embedded module and `tzdata/` as one workflow artifact.
CI then runs [`scripts/wasm-size-check.sh`](../../scripts/wasm-size-check.sh),
which fails when any layer is more than 5% larger or smaller than the table
above: a layer that grows by accident is caught, and a change that moves a
layer on purpose comes with a refreshed table.
`hc_alloc`, `hc_free` and `hc_version` are in every build.

`full` is every layer and carries `places`, whose names are two fifths of
it. A page that shows place names only on demand builds a module of the
layers it paints with and fetches the `places` build when it needs them:
each build is a module of its own, instantiated on its own, and neither
needs the other.

The profile's `opt-level = "z"` is a measured choice, not a default. On
2026-09-27, with rustc 1.98.1, the `full` layer built at `z` was
2,570,192 bytes and answered `hc_holidays_on` for 2026-01-01 in 51.7 ms
under Node 22, the shortest of seven calls
([`scripts/wasm-calendar-timing.mjs`](../../scripts/wasm-calendar-timing.mjs));
at `s`, 2,591,571 bytes and 48.4 ms; at `3`, 2,804,609 bytes and 44.0 ms.
The 18 % of time `z` costs against `3` buys 8 % of the size, and a page
loads the module far more often than it asks the costliest question, so
`z` stays.

## What is exported

The two tables below are rendered from the crate's sources, and from the
rows of `hyper_calendar`'s table of shared exports
([`crates/hyper-calendar/src/exports.rs`](../hyper-calendar/src/exports.rs))
each layer module expands, by
[`crates/hyper-calendar/tests/abi.rs`](../hyper-calendar/tests/abi.rs), which
fails when they drift, and [`tests/readme.rs`](tests/readme.rs) holds every
row's feature to the `#[cfg]` of the module that holds or expands the
export. An export without a row here does
not pass CI.

<!-- generated by crates/hyper-calendar/tests/abi.rs: start -->

### Exports

202 functions. Types are the WebAssembly ones: `i64` crosses into JavaScript as a `BigInt`, everything else as a `number`, and a pointer is a byte offset into `memory`. The feature column is the Cargo feature the module has to be built with for the export to exist.

| Export | Feature | What it does |
| --- | --- | --- |
| `hc_alloc(len: usize) -> *mut u8` | always | Allocate `len` bytes of linear memory and return a pointer to them. |
| `hc_free(pointer: *mut u8, len: usize)` | always | Return a block from `hc_alloc` to the allocator. |
| `hc_version(buffer: *mut u8, capacity: usize) -> i64` | always | The library version as UTF-8, returning the byte length written. |
| `hc_gregorian_to_fixed(year: i64, month: u32, day: u32) -> i64` | `civil` | The fixed day number of a proleptic Gregorian date, or an error sentinel. |
| `hc_gregorian_year(fixed: i64) -> i64` | `civil` | The Gregorian year on a fixed day, or an error sentinel. |
| `hc_gregorian_month(fixed: i64) -> i64` | `civil` | The Gregorian month on a fixed day, 1 through 12, or an error sentinel. |
| `hc_gregorian_day(fixed: i64) -> i64` | `civil` | The Gregorian day of the month on a fixed day, or an error sentinel. |
| `hc_tai_minus_utc(unix_seconds: i64, strict: i32) -> i64` | `civil` | `TAI - UTC` in whole seconds at a POSIX timestamp. |
| `hc_format_iso_date(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `civil` | Render a fixed day as an ISO 8601 date, returning the byte length written. |
| `hc_parse_iso_date(buffer: *const u8, len: usize) -> i64` | `civil` | Parse an ISO 8601 date from UTF-8, returning its fixed day number. |
| `hc_weekday(fixed: i64) -> i64` | `civil` | The ISO weekday of a fixed day, Monday = 1 through Sunday = 7. |
| `hc_day_of_year(fixed: i64) -> i64` | `civil` | The 1-based day of the year on a fixed day, or an error sentinel. |
| `hc_is_leap_year(fixed: i64) -> i64` | `civil` | Whether the Gregorian year on a fixed day is a leap year: 1, 0, or an error sentinel. |
| `hc_fixed_from_unix(unix_seconds: i64) -> i64` | `civil` | The fixed day a POSIX timestamp falls on, in UTC. |
| `hc_day_has_leap_second(unix_seconds: i64) -> i64` | `civil` | Whether the UTC day containing a POSIX timestamp ends with an inserted leap second: 1, 0, or an error sentinel. |
| `hc_unix_from_fixed(fixed: i64) -> i64` | `civil` | The POSIX timestamp of midnight UTC on a fixed day. |
| `hc_tai64_decode(hex: *const u8, hex_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A TAI64, TAI64N or TAI64NA label in hexadecimal read back, as one UTF-8 line, returning the byte length written. |
| `hc_gnss_week(numbering: *const u8, numbering_len: usize, tai_seconds: i64, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The GNSS week and time of week of a TAI instant, as one UTF-8 line, returning the byte length written. |
| `hc_gnss_to_tai(numbering: *const u8, numbering_len: usize, week: u32, tow_seconds: u32, tow_attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The TAI instant of a full GNSS week and a time of week, as one UTF-8 line, returning the byte length written. |
| `hc_glonass_date(tai_seconds: i64, attoseconds: u64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | GLONASS's four-year interval N4 and day N_T at a TAI instant, as one UTF-8 line, returning the byte length written. |
| `hc_fixed_from_ole_automation(value: f64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The fixed day and the time of day of an OLE Automation date, as one UTF-8 line, returning the byte length written. |
| `hc_ole_automation_from_fixed(fixed: i64, seconds_of_day: f64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The OLE Automation date of a fixed day and a time of day, as one UTF-8 line, returning the byte length written. |
| `hc_excel_1900_day(serial: i64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | What an Excel 1900 serial names, as one UTF-8 line, returning the byte length written. |
| `hc_tai_from_unix(unix_seconds: i64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A POSIX timestamp as a TAI reading, as one UTF-8 line, returning the byte length written. |
| `hc_utc_from_tai(tai_seconds: i64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A whole TAI second as a UTC label, as one UTF-8 line, returning the byte length written. |
| `hc_tai64_posix_plus_10_decode(hex: *const u8, hex_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A TAI64 or TAI64N label in the `tai64-posix-plus-10` convention read back, as one UTF-8 line, returning the byte length written. |
| `hc_uuid_timestamp(uuid: *const u8, uuid_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The timestamp of a version 1 or version 6 UUID, as one UTF-8 line, returning the byte length written. |
| `hc_ntp_resolve(seconds: u32, fraction: u32, reference_unix: i64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A 64-bit NTP timestamp placed in its era by a reference time, as one UTF-8 line, returning the byte length written. |
| `hc_fat_decode(date: u32, time: u32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The local reading a FAT date word and time word name, as one UTF-8 line, returning the byte length written. |
| `hc_fat_encode(fixed: i64, seconds_of_day: u32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The FAT date and time words of a fixed day and a time of day, as one UTF-8 line, returning the byte length written. |
| `hc_epoch_from_tt(notation: *const u8, notation_len: usize, tt_seconds: i64, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The Julian or Besselian epoch of a TT instant, as one UTF-8 line, returning the byte length written. |
| `hc_tt_from_epoch(notation: *const u8, notation_len: usize, year: f64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The TT instant of a Julian or Besselian epoch, as one UTF-8 line, returning the byte length written. |
| `hc_tai64_encode(tai_seconds: i64, attoseconds: u64, format: *const u8, format_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A TAI instant as a TAI64, TAI64N or TAI64NA label in lower-case hexadecimal, as one UTF-8 line, returning the byte length written. |
| `hc_gnss_resolve_week(numbering: *const u8, numbering_len: usize, broadcast: u32, rule: *const u8, rule_len: usize, reference_tai_seconds: i64) -> i64` | `timestamps` | The full GNSS week a broadcast week names, by a rollover rule and a reference instant, or an error sentinel. |
| `hc_tai64_posix_plus_10_encode(unix_seconds: i64, attoseconds: u64, format: *const u8, format_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A POSIX instant as a TAI64 or TAI64N label in the `tai64-posix-plus-10` convention, in lower-case hexadecimal, as one UTF-8 line, returning the byte length written. |
| `hc_uuid_timestamp_encode(unix_seconds: i64, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The 60-bit UUID timestamp of a POSIX instant, and the time fields a version 1 and a version 6 UUID write it in, as one UTF-8 line, returning the byte length written. |
| `hc_ntp_encode(unix_seconds: i64, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The NTP date and timestamp of a POSIX instant, as one UTF-8 line, returning the byte length written. |
| `hc_swatch_beat(unix_seconds: i64, attoseconds: u64) -> i64` | `timestamps` | The Swatch Internet Time at a POSIX instant, 0 through 999, or an error sentinel. |
| `hc_tt_bipm(series: *const u8, series_len: usize, tai_seconds: i64, attoseconds: u64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | TT(BIPM) at a TAI instant, read from a realisation the caller supplies, as one UTF-8 line, returning the byte length written. |
| `hc_dotnet_ticks_from_unix(unix_seconds: i64, attoseconds: u64) -> i64` | `timestamps` | .NET's `DateTime.Ticks` of a POSIX instant as a `Utc` value, or an error sentinel. |
| `hc_unix_from_dotnet_ticks(ticks: i64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The reading a count of .NET ticks names, as one UTF-8 line, returning the byte length written. |
| `hc_six_hour_clock(reckoning: *const u8, reckoning_len: usize, seconds_of_day: u32, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | A time of the civil day on a six-hour clock, as one UTF-8 line, returning the byte length written. |
| `hc_civil_from_six_hour_clock(reckoning: *const u8, reckoning_len: usize, hour: u32, minute: u32, second: u32, night: i32) -> i64` | `timestamps` | The civil time of day of a six-hour reading, as seconds after midnight, 0 through 86 399, or an error sentinel. |
| `hc_french_decimal_time(seconds_of_day: u32, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The French Republican decimal time of a time of the civil clock, as one UTF-8 line, returning the byte length written. |
| `hc_civil_from_french_decimal_time(hour: u32, minute: u32, second: u32, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `timestamps` | The civil time of day of a French Republican decimal time, as one UTF-8 line, returning the byte length written. |
| `hc_ccsds_decode(hex: *const u8, hex_len: usize, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | A binary CCSDS time code read, as one UTF-8 line, returning the byte length written. |
| `hc_ccsds_encode(tai_seconds: i64, attoseconds: u64, p_field: *const u8, p_field_len: usize, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | The binary CCSDS time code of a TAI instant in the format a P-field names, as one UTF-8 line, returning the byte length written. |
| `hc_ccsds_decode_from_epoch(hex: *const u8, hex_len: usize, epoch_tai_seconds: i64, epoch_attoseconds: u64, epoch_unix_day: i64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | A binary CCSDS time code read, a Level 2 code from the caller's epoch, as one UTF-8 line, returning the byte length written. |
| `hc_ccsds_encode_from_epoch(tai_seconds: i64, attoseconds: u64, p_field: *const u8, p_field_len: usize, epoch_tai_seconds: i64, epoch_attoseconds: u64, epoch_unix_day: i64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | The binary CCSDS time code of a TAI instant in the format a P-field names, a Level 2 format from the caller's epoch, as one UTF-8 line, returning the byte length written. |
| `hc_ccsds_ascii_parse(code: *const u8, code_len: usize, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | A CCSDS ASCII time code, A or B, read, as one UTF-8 line, returning the byte length written. |
| `hc_ccsds_ascii_format(tai_seconds: i64, attoseconds: u64, variation: *const u8, variation_len: usize, precision: *const u8, precision_len: usize, terminator: i32, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | The CCSDS ASCII time code of a TAI instant's UTC label, as one UTF-8 line, returning the byte length written. |
| `hc_radio_decode(code: *const u8, code_len: usize, frame: *const u8, frame_len: usize, century: i64, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | One minute's frame of a long-wave radio time code read, as one UTF-8 line, returning the byte length written. |
| `hc_radio_encode(code: *const u8, code_len: usize, unix_seconds: i64, leap: i32, summer: *const u8, summer_len: usize, zone_change: i32, dut1_tenths: i32, dst_next: u32, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | The frame of a long-wave radio time code for a minute, as one UTF-8 line, returning the byte length written. |
| `hc_jjy_call_sign_decode(frame: *const u8, frame_len: usize, year: i64, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | JJY's call-sign frame of minute 15 or 45 read in a year the caller names, as one UTF-8 line, returning the byte length written. |
| `hc_jjy_call_sign_encode(unix_seconds: i64, stop_start: u32, daytime_only: i32, stop_span: u32, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | JJY's call-sign frame for minute 15 or 45 with a notice of a planned stop, as one UTF-8 line, returning the byte length written. |
| `hc_irig_decode(signal: *const u8, signal_len: usize, frame: *const u8, frame_len: usize, year: i64, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | One frame of an IRIG serial time code read, as one UTF-8 line, returning the byte length written. |
| `hc_irig_encode(signal: *const u8, signal_len: usize, fixed: i64, seconds_of_day: u32, hundredths: u32, control: u32, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | The frame of an IRIG serial time code whose reference bit falls at a reading of the civil clock, as one UTF-8 line, returning the byte length written. |
| `hc_irig_frame_start(signal: *const u8, signal_len: usize, seconds_of_day: u32, hundredths: u32, buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | The reading at which the frame of an IRIG code that holds a reading of the civil clock begins, with the frame's length, as one UTF-8 line, returning the byte length written. |
| `hc_irig_formats(buffer: *mut u8, capacity: usize) -> i64` | `time-codes` | Every IRIG format `hc_irig_decode` and `hc_irig_encode` read, with its frame's length, its rate and its fields, as UTF-8 lines, returning the byte length written. |
| `hc_describe_day(fixed: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | One fixed day in every registered calendar, as UTF-8 lines, returning the byte length written. |
| `hc_day_extras(fixed: i64, id: *const u8, id_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The extra fields of one fixed day, as UTF-8 lines, returning the byte length written. |
| `hc_calendar_units(id: *const u8, id_len: usize, unit: u32, from_fixed: i64, to_fixed: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The days from `from_fixed` up to but not including `to_fixed` as one calendar's eras, years, months or days, as UTF-8 lines, returning the byte length written. |
| `hc_parse_date(calendar: *const u8, calendar_len: usize, locale: *const u8, locale_len: usize, text: *const u8, text_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | A date as a locale writes it in one calendar, read back, as one UTF-8 line, returning the byte length written. |
| `hc_calendars(today: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Every registered calendar, as UTF-8 lines, returning the byte length written. |
| `hc_calendar_list(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Every registered calendar by name alone, as UTF-8 lines, returning the byte length written. |
| `hc_locales(buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Every locale the module carries, as UTF-8 lines, returning the byte length written. |
| `hc_first_day_of_week(locale: *const u8, locale_len: usize) -> i64` | `calendars` | The ISO weekday of the first day of the week in a locale, Monday = 1 through Sunday = 7, or an error sentinel. |
| `hc_day_period(seconds_of_day: u32, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The day periods of a time of the civil clock in a locale, as one UTF-8 line, returning the byte length written. |
| `hc_format_number(system: *const u8, system_len: usize, value: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | An integer written in a numbering system, as one UTF-8 line, returning the byte length written. |
| `hc_parse_number(system: *const u8, system_len: usize, text: *const u8, text_len: usize) -> i64` | `calendars` | An integer read back out of a numbering system's notation, or an error sentinel. |
| `hc_numbering_systems(buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Every numbering system `hc_format_number` writes, as UTF-8 lines, returning the byte length written. |
| `hc_calendar_eras(calendar: *const u8, calendar_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The eras a calendar is described with, named in a locale, as UTF-8 lines, returning the byte length written. |
| `hc_gregorian_adoption(region: *const u8, region_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The steps by which a country adopted the Gregorian calendar, as UTF-8 lines, returning the byte length written. |
| `hc_naming_period_on(calendar: *const u8, calendar_len: usize, fixed: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Which month and weekday names a locale writes for a calendar on a fixed day, where a government renamed them for a period, as one UTF-8 line, returning the byte length written. |
| `hc_panchanga_at(unix_seconds: i64, ayanamsa: *const u8, ayanamsa_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The yoga and the karaṇa in progress at a POSIX timestamp, as two UTF-8 lines, returning the byte length written. |
| `hc_panchanga_of_day(fixed: i64, latitude: f64, longitude: f64, elevation: f64, ayanamsa: *const u8, ayanamsa_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The yoga and the karaṇa a fixed day carries at a place, the ones in progress at its sunrise, as two UTF-8 lines, returning the byte length written. |
| `hc_hindu_lunar_date(sky: *const u8, sky_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The Hindu lunisolar date of a fixed day at a place, as one UTF-8 line, returning the byte length written. |
| `hc_surya_siddhanta_at(unix_seconds: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The *Sūrya Siddhānta*'s Sun and Moon at a POSIX timestamp, as one UTF-8 line, returning the byte length written. |
| `hc_surya_siddhanta_sunrise(fixed: i64, latitude: f64, longitude: f64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The *Sūrya Siddhānta*'s sunrise on a fixed day at a place, as one UTF-8 line, returning the byte length written. |
| `hc_crescent_visible(criterion: *const u8, criterion_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Whether the young crescent should have been visible on the evening that begins a fixed day, from a place, by a named criterion, as one UTF-8 line, returning the byte length written. |
| `hc_ioc_olympiad(gregorian_year: i64) -> i64` | `calendars` | The number of the modern Olympiad a Gregorian year belongs to, or an error sentinel. |
| `hc_ioc_olympiad_on(fixed: i64) -> i64` | `calendars` | The modern Olympiad a fixed day belongs to, by the Olympic Charter in force on that day, or an error sentinel. |
| `hc_babylonian_regnal_year(seleucid_year: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The king and regnal year labelling a Seleucid year, as one UTF-8 line, returning the byte length written. |
| `hc_equinox_new_year_margin(calendar: *const u8, calendar_len: usize, year: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | How far the equinox that begins a year of a calendar fell from the moment of the day that decides its new year, as one UTF-8 line, returning the byte length written. |
| `hc_shmuel_tekufah(hebrew_year: i64, tekufah: *const u8, tekufah_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | A *tekufah* of Shmuel's reckoning in a Hebrew year, as one UTF-8 line, returning the byte length written. |
| `hc_day_name(calendar: *const u8, calendar_len: usize, naming: *const u8, naming_len: usize, fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | A fixed day's name in a calendar whose days are named, by one of its namings, as one UTF-8 line, returning the byte length written. |
| `hc_hebrew_yahrzeit(death_fixed: i64, hebrew_year: i64) -> i64` | `calendars` | The fixed day of the yahrzeit in a Hebrew year of a death on the Hebrew date a fixed day names, or an error sentinel. |
| `hc_hebrew_birthday(birth_fixed: i64, hebrew_year: i64) -> i64` | `calendars` | The fixed day of the birthday in a Hebrew year of a birth on the Hebrew date a fixed day names, or an error sentinel. |
| `hc_chinese_reckoned_age(birth_fixed: i64, on_fixed: i64) -> i64` | `calendars` | A person's age as the Chinese count reckons it on a fixed day, or an error sentinel. |
| `hc_chinese_marriage_augury(chinese_year: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The marriage augury of a Chinese year, as one UTF-8 line, returning the byte length written. |
| `hc_chinese_age(convention: *const u8, convention_len: usize, birth_fixed: i64, on_fixed: i64) -> i64` | `calendars` | A person's age on a fixed day by a named count, born on another, or an error sentinel. |
| `hc_chinese_almanac_solar_terms(year: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The days of the twenty-four solar terms the Qing almanac printed in a Gregorian year, as UTF-8 lines, returning the byte length written. |
| `hc_hebrew_sabbatical_cycle_year(hebrew_year: i64) -> i64` | `calendars` | The place of a Hebrew year in the seven-year sabbatical cycle, 1 through 7, or an error sentinel. |
| `hc_asian_day(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | A fixed day in the calendar of the Roman province of Asia as the calendar writes it, unnumbered days included, as one UTF-8 line, returning the byte length written. |
| `hc_barhaspatya_year(rule: *const u8, rule_len: usize, saka: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The name of the northern sixty-year cycle, the Bārhaspatya saṃvatsara, a named rule couples with a Śaka year, and the name it expunges that year, as one UTF-8 line, returning the byte length written. |
| `hc_barhaspatya_year_at(rule: *const u8, rule_len: usize, unix_seconds: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The name of the northern sixty-year cycle in progress at a POSIX timestamp by a named rule, as one UTF-8 line, returning the byte length written. |
| `hc_kalam(convention: *const u8, convention_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Rāhu kālam, Yamaganda and Gulika kālam on a fixed day, as three UTF-8 lines, each named in a locale, returning the byte length written. |
| `hc_muhurtas(fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The thirty muhūrtas of a fixed day at a place, with Abhijit and Dur Muhurtam marked, as UTF-8 lines, returning the byte length written. |
| `hc_amrita_siddhi(fixed: i64, latitude: f64, longitude: f64, elevation: f64, ayanamsa: *const u8, ayanamsa_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The *amṛta siddhi yoga* of a fixed day at a place, as one UTF-8 line, returning the byte length written. |
| `hc_nakshatra_at(unix_seconds: i64, ayanamsa: *const u8, ayanamsa_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The nakṣatra the Moon is in at a POSIX timestamp, as one UTF-8 line, returning the byte length written. |
| `hc_nakshatra_of_day(fixed: i64, latitude: f64, longitude: f64, elevation: f64, ayanamsa: *const u8, ayanamsa_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The nakṣatra a fixed day carries at a place, the one the Moon is in at its sunrise, as one UTF-8 line, returning the byte length written. |
| `hc_almanac_cycles(fixed: i64, meridian: *const u8, meridian_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The almanac's cycles of a fixed day, 恵方, 三元九運 and 손 없는 날, as one UTF-8 line, returning the byte length written. |
| `hc_almanac_day(fixed: i64, meridian: *const u8, meridian_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The almanac's annotations of a fixed day, 干支 to the 選日, as UTF-8 lines, one an annotation, each named in a locale, returning the byte length written. |
| `hc_almanac_directions(fixed: i64, meridian: *const u8, meridian_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Where the 八将神 and the 金神 stand in the year in force on a fixed day, as UTF-8 lines, one a god and a direction, returning the byte length written. |
| `hc_rounichi(rule: *const u8, rule_len: usize, year: i64, meridian: *const u8, meridian_len: usize) -> i64` | `calendars` | The fixed day of 臘日 in the winter that ends in a Gregorian year, by a named reckoning, at a meridian, or an error sentinel. |
| `hc_mansion_undertakings(list: *const u8, list_len: usize, fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | What one publisher's list says the 二十八宿 of a fixed day favours and forbids, as UTF-8 lines, one an undertaking, returning the byte length written. |
| `hc_almanac_person_days(fixed: i64, birth_fixed: i64, meridian: *const u8, meridian_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Whether a fixed day is one of a person's own 五墓日 or 三箇の悪日, by the day they were born on, as UTF-8 lines, one an entry, returning the byte length written. |
| `hc_tibetan_almanac_day(calendar: *const u8, calendar_len: usize, fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | What the Tibetan almanac of a version prints for a fixed day, the five components and the columns after them, as UTF-8 lines, one a column, returning the byte length written. |
| `hc_tibetan_planets(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | Where the Phugpa almanac places the five planets at the end of a fixed day, as UTF-8 lines, one a planet, returning the byte length written. |
| `hc_bhutanese_winter_solstice(year: i64, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The Bhutanese calendar's winter solstice of a Gregorian year, as one UTF-8 line, returning the byte length written. |
| `hc_tibetan_festival_day(rule: *const u8, rule_len: usize, calendar: *const u8, calendar_len: usize, year: i64, month: u32, leap: i32, day: u32) -> i64` | `calendars` | The fixed day a festival on a Tibetan date is kept on, by a named rule for a skipped or repeated number, or an error sentinel. |
| `hc_choghadiya(fixed: i64, latitude: f64, longitude: f64, elevation: f64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The sixteen choghadiya of a fixed day at a place, as UTF-8 lines, each named in a locale, returning the byte length written. |
| `hc_panchak(naming: *const u8, naming_len: usize, unix_seconds: i64, ayanamsa: *const u8, ayanamsa_len: usize, offset_seconds: i32, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The Panchak window in progress at a POSIX timestamp, or the next one, and its kind under a naming table, as one UTF-8 line, returning the byte length written. |
| `hc_kumbh(yoga: *const u8, yoga_len: usize, year: i64, ayanamsa: *const u8, ayanamsa_len: usize, jupiter: *const u8, jupiter_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | When in a Gregorian year the Sun, and the Moon where it is asked for, stand as a condition of the Kumbh Mela requires, and whether Jupiter's sign, which the caller gives, meets it, as one UTF-8 line, returning the byte length written. |
| `hc_pushkaram(sign: *const u8, sign_len: usize, entry_unix_seconds: i64, latitude: f64, longitude: f64, elevation: f64, meridian: *const u8, meridian_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The twelve days of the *Ādi Pushkaram* of each river of a sidereal sign, for Jupiter's entry into it at a POSIX timestamp, as UTF-8 lines, each river named in a locale, returning the byte length written. |
| `hc_folk_day(fixed: i64, meridian: *const u8, meridian_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The folk reckonings of a fixed day outside the Japanese almanac, as UTF-8 lines, one a reckoning, each named in a locale, returning the byte length written. |
| `hc_night_watch(seconds_of_day: u32, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `calendars` | The Chinese night watch, 更, and its points, 點, of a time of the civil clock by the fixed reckoning, as one UTF-8 line, or none by day, returning the byte length written. |
| `hc_term_in_effect(fixed: i64, meridian: *const u8, meridian_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `seasons` | The solar term in effect on a fixed day at a meridian, as one UTF-8 line, returning the byte length written. |
| `hc_pentad_in_effect(fixed: i64, meridian: *const u8, meridian_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `seasons` | The pentad (候) in effect on a fixed day at a meridian, as one UTF-8 line, returning the byte length written. |
| `hc_cold_food_day(convention: *const u8, convention_len: usize, year: i64) -> i64` | `seasons` | The fixed day of 寒食, the Cold Food Day, of a Gregorian year under a named reckoning, or an error sentinel. |
| `hc_plum_rains(rule: *const u8, rule_len: usize, year: i64, meridian: *const u8, meridian_len: usize) -> i64` | `seasons` | The fixed day of 入梅 or 出梅, the beginning or the end of the plum rains, of a Gregorian year by a named rule of the Chinese almanac, with the solar term it counts from at a meridian, or an error sentinel. |
| `hc_holiday_is_day_off(code: *const u8, code_len: usize, region: *const u8, region_len: usize, group: *const u8, group_len: usize, fixed: i64) -> i64` | `holiday` | Whether a fixed day is a day off in a holiday table: 1, 0, or an error sentinel. |
| `hc_holiday_add_business_days(code: *const u8, code_len: usize, region: *const u8, region_len: usize, group: *const u8, group_len: usize, fixed: i64, count: i64) -> i64` | `holiday` | A fixed day moved by a number of business days of a holiday table, in a subdivision and for a group, or an error sentinel. |
| `hc_holiday_business_days_between(code: *const u8, code_len: usize, region: *const u8, region_len: usize, group: *const u8, group_len: usize, from_fixed: i64, to_fixed: i64) -> i64` | `holiday` | The number of business days of a holiday table, in a subdivision and for a group, from one fixed day up to but not including another, or an error sentinel. |
| `hc_holidays_in_year(code: *const u8, code_len: usize, region: *const u8, region_len: usize, group: *const u8, group_len: usize, kind: *const u8, kind_len: usize, year: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The holidays of a Gregorian year in a table, as UTF-8 lines, returning the byte length written. |
| `hc_holiday_codes(buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The identifier of every holiday table, one per line, returning the byte length written. |
| `hc_holidays_on(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | Every holiday on one fixed day across every table, as UTF-8 lines, returning the byte length written. |
| `hc_holiday_tables(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | Every holiday table with its kind, names and sources, as UTF-8 lines, returning the byte length written. |
| `hc_holiday_groups(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | Every group of people a holiday may be given to alone, named in a locale, as UTF-8 lines, returning the byte length written. |
| `hc_holidays_on_in(fixed: i64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | `hc_holidays_on`'s lines, each with the day's name in a locale and the tag that named it, as UTF-8 lines, returning the byte length written. |
| `hc_lectionary(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The lectionary cycles of a fixed day, as one UTF-8 line, returning the byte length written. |
| `hc_astronomical_easter(year: i64) -> i64` | `holiday` | The fixed day of Easter Sunday of a Gregorian year by the astronomical reckoning at the meridian of Jerusalem, or an error sentinel. |
| `hc_astronomical_paschal_full_moon(year: i64) -> i64` | `holiday` | The fixed day of the paschal full moon of a Gregorian year by the astronomical reckoning at the meridian of Jerusalem, or an error sentinel. |
| `hc_holy_year_on(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The Holy Year of the Catholic Church a fixed day falls in, if any, as one UTF-8 line, returning the byte length written. |
| `hc_common_worship_on(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The rank of every *Common Worship* celebration kept on a fixed day, as UTF-8 lines, returning the byte length written. |
| `hc_roman_1960_office_on(fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | What the Roman calendar of the 1960 rubrics does on a fixed day, the office kept, its commemorations and the days transferred or omitted, as UTF-8 lines, returning the byte length written. |
| `hc_orthodox_fast_on(reckoning: *const u8, reckoning_len: usize, fixed: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | What a fixed day is in the fasting scheme of a reckoning, as one UTF-8 line, returning the byte length written. |
| `hc_orthodox_fast_seasons(reckoning: *const u8, reckoning_len: usize, year: i64, buffer: *mut u8, capacity: usize) -> i64` | `holiday` | The fasting seasons and fast-free weeks of a year of a reckoning, as UTF-8 lines, returning the byte length written. |
| `hc_geologic_intervals(rank: u32, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | Every interval of one rank of the geologic time scale, as UTF-8 lines, returning the byte length written. |
| `hc_place_years_ago(years_ago: f64, std_dev_years: f64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | A moment some years before the present, placed in every chronology at once, as UTF-8 lines, returning the byte length written. |
| `hc_cosmic_events(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | Every cosmic epoch and every dated cosmic event, as UTF-8 lines, returning the byte length written. |
| `hc_earliest_evidence(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | Every claim to the earliest evidence of life, of *Homo sapiens* and of writing, as UTF-8 lines, returning the byte length written. |
| `hc_archaeological_periods(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | Every conventional archaeological period, as UTF-8 lines, returning the byte length written. |
| `hc_future_events(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `deep-time` | Every dated event of the far future, as UTF-8 lines, returning the byte length written. |
| `hc_zone_load(name: *const u8, name_len: usize, tzif: *const u8, tzif_len: usize) -> i64` | `tz` | Give the module a zone's TZif data under an IANA name, returning 0. |
| `hc_fixed_from_unix_in_zone(unix_seconds: i64, zone: *const u8, zone_len: usize) -> i64` | `tz` | The fixed day a POSIX timestamp falls on by the wall clock of a zone, or an error sentinel. |
| `hc_unix_from_fixed_in_zone(fixed: i64, zone: *const u8, zone_len: usize) -> i64` | `tz` | The POSIX timestamp at which a fixed day begins by the wall clock of a zone, or an error sentinel. |
| `hc_zone_offset(zone: *const u8, zone_len: usize, unix_seconds: i64, buffer: *mut u8, capacity: usize) -> i64` | `tz` | The offset a zone keeps at a POSIX timestamp, as one UTF-8 line, returning the byte length written. |
| `hc_zones(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `tz` | Every zone of the IANA database's `zone1970.tab` with its principal location, as UTF-8 lines, returning the byte length written. |
| `hc_zone_location(zone: *const u8, zone_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `tz` | Where one zone is, as the UTF-8 line `hc_zones` writes for it, returning the byte length written. |
| `hc_earth_rotation_angle(ut1_unix_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Earth Rotation Angle at a UT1 instant, as one UTF-8 line, returning the byte length written. |
| `hc_gmst_iau2006(ut1_unix_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Greenwich mean sidereal time by the IAU 2006 convention at a UT1 instant, as one UTF-8 line, returning the byte length written. |
| `hc_gmst_iau1982(ut1_unix_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Greenwich mean sidereal time by the IAU 1982 convention at a UT1 instant, as one UTF-8 line, returning the byte length written. |
| `hc_ut2_minus_ut1(ut1_unix_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | UT2 − UT1 at a UT1 instant, as one UTF-8 line, returning the byte length written. |
| `hc_sky_at(unix_seconds: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Sun and the Moon at a POSIX timestamp, as one UTF-8 line, returning the byte length written. |
| `hc_solar_terms_between(from_unix: i64, to_unix: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Every solar term whose instant falls in `[from_unix, to_unix)`, as UTF-8 lines, returning the byte length written. |
| `hc_moon_phases_between(from_unix: i64, to_unix: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Every new moon, first quarter, full moon and last quarter whose instant falls in `[from_unix, to_unix)`, as UTF-8 lines, returning the byte length written. |
| `hc_decan_at(unix_seconds: i64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The decan the Sun is in at a POSIX timestamp, as one UTF-8 line, returning the byte length written. |
| `hc_drekkana_at(unix_seconds: i64, ayanamsa: *const u8, ayanamsa_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The drekkāṇa, the Hindu third of a sidereal sign, the Sun is in at a POSIX timestamp, as one UTF-8 line, returning the byte length written. |
| `hc_horizons(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Every named horizon a rising or a setting can be measured against, as UTF-8 lines, returning the byte length written. |
| `hc_sunrise(horizon: *const u8, horizon_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Sunrise on a fixed day at a place against a named horizon, as one UTF-8 line, returning the byte length written. |
| `hc_sunset(horizon: *const u8, horizon_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | Sunset on a fixed day at a place against a named horizon, as one UTF-8 line, returning the byte length written. |
| `hc_solar_time(clock: *const u8, clock_len: usize, unix_seconds: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | A local clock's reading at a POSIX timestamp and a place, as one UTF-8 line, returning the byte length written. |
| `hc_solar_event(event: *const u8, event_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | A named time of day on a fixed day at a place, as one UTF-8 line, returning the byte length written. |
| `hc_hjd_tt(tt_julian_date: f64, right_ascension: f64, declination: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Heliocentric Julian Date in TT, HJD_TT, of a Julian Date of TT for a target, as one UTF-8 line, returning the byte length written. |
| `hc_hjd_utc(utc_julian_date: f64, right_ascension: f64, declination: f64, strict: i32, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Heliocentric Julian Date in UTC, HJD_UTC, of a Julian Date of UTC for a target, as one UTF-8 line, returning the byte length written. |
| `hc_gmat_from_gmt(fixed: i64, seconds_of_day: u32, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The astronomical date and the Greenwich Mean Astronomical Time of a reading of GMT, as one UTF-8 line, returning the byte length written. |
| `hc_gmt_from_gmat(fixed: i64, seconds_of_day: u32, attoseconds: u64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The civil date and the GMT of a reading of Greenwich Mean Astronomical Time, the inverse of `hc_gmat_from_gmt`, as one UTF-8 line in its columns, returning the byte length written. |
| `hc_prayer_times(method: *const u8, method_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, ramadan: i32, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Islamic prayer times of a fixed day at a place by a named method, as eight UTF-8 lines, returning the byte length written. |
| `hc_prayer_methods(buffer: *mut u8, capacity: usize) -> i64` | `sky` | Every prayer-time method `hc_prayer_times` reads, with its parameters and source, as UTF-8 lines, returning the byte length written. |
| `hc_zmanim(reckoning: *const u8, reckoning_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Jewish times of a fixed day at a place by a reckoning, with the dawns and nightfalls, as nine UTF-8 lines, returning the byte length written. |
| `hc_temporal_hour(reckoning: *const u8, reckoning_len: usize, fixed: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The length of a temporal hour of a fixed day at a place by a reckoning of the Jewish day, as one UTF-8 line, returning the byte length written. |
| `hc_edo_time(unix_seconds: i64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The Edo 不定時法 reading of a POSIX timestamp at a place, as one UTF-8 line, returning the byte length written. |
| `hc_unix_from_edo_time(fixed: i64, hour: u32, fraction: f64, latitude: f64, longitude: f64, elevation: f64, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The instant of an Edo 不定時法 reading at a place, as one UTF-8 line, returning the byte length written. |
| `hc_planetary_hour(unix_seconds: i64, latitude: f64, longitude: f64, elevation: f64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The planetary hour at a POSIX timestamp and a place, as one UTF-8 line, its ruler named in a locale, returning the byte length written. |
| `hc_planetary_hours_of_day(fixed: i64, latitude: f64, longitude: f64, elevation: f64, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `sky` | The twenty-four planetary hours of the planetary day that begins at the sunrise of a fixed day at a place, as UTF-8 lines in `hc_planetary_hour`'s columns, each ruler named in a locale, returning the byte length written. |
| `hc_orbit_at(years_before_1950: f64, buffer: *mut u8, capacity: usize) -> i64` | `orbital` | Earth's orbital elements and the June insolation at 65° N at an epoch, as one UTF-8 line, returning the byte length written. |
| `hc_orbit_series(from_years_before_1950: f64, to_years_before_1950: f64, step_years: f64, buffer: *mut u8, capacity: usize) -> i64` | `orbital` | The line of `hc_orbit_at` at every epoch from `from_years_before_1950` to `to_years_before_1950` in steps of `step_years`, each with the epoch as a first column, as UTF-8 lines, returning the byte length written. |
| `hc_jupiter_at(unix_seconds: i64, ayanamsa: *const u8, ayanamsa_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `jupiter` | Where Jupiter is at a POSIX timestamp, tropical and sidereal, as one UTF-8 line, returning the byte length written. |
| `hc_jupiter_ingresses(from_unix_seconds: i64, to_unix_seconds: i64, ayanamsa: *const u8, ayanamsa_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `jupiter` | Jupiter's crossings of the boundaries of the sidereal signs in a span of POSIX seconds, as UTF-8 lines, one each, in time order, returning the byte length written. |
| `hc_jupiter_risings(from_unix_seconds: i64, to_unix_seconds: i64, ayanamsa: *const u8, ayanamsa_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `jupiter` | Jupiter's heliacal risings in a span of POSIX seconds, each with the name a year of Jupiter has from it, as UTF-8 lines, returning the byte length written. |
| `hc_kumbh_by_sky(yoga: *const u8, yoga_len: usize, year: i64, ayanamsa: *const u8, ayanamsa_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `jupiter` | When in a Gregorian year the Sun, and the Moon where it is asked for, stand as a condition of the Kumbh Mela requires, and whether Jupiter, whose sign is computed, meets it, as one UTF-8 line, returning the byte length written. |
| `hc_pushkaram_by_sky(sign: *const u8, sign_len: usize, year: i64, ayanamsa: *const u8, ayanamsa_len: usize, rule: *const u8, rule_len: usize, latitude: f64, longitude: f64, elevation: f64, meridian: *const u8, meridian_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `jupiter` | The twelve days of the *Ādi Pushkaram* of each river of a sidereal sign, for Jupiter's entry into it in a Gregorian year, found, as UTF-8 lines, each river named in a locale, returning the byte length written. |
| `hc_pushkarams_in_year(year: i64, ayanamsa: *const u8, ayanamsa_len: usize, rule: *const u8, rule_len: usize, latitude: f64, longitude: f64, elevation: f64, meridian: *const u8, meridian_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `jupiter` | The twelve days of the *Ādi Pushkaram* of each river of every sidereal sign Jupiter enters in a Gregorian year, found, as UTF-8 lines, each river named in a locale, returning the byte length written. |
| `hc_mars_time(unix_seconds: f64, east_longitude_degrees: f64, buffer: *mut u8, capacity: usize) -> i64` | `planetary` | Mars at a POSIX instant and an east longitude, as one UTF-8 line, returning the byte length written. |
| `hc_missions(buffer: *mut u8, capacity: usize) -> i64` | `planetary` | Every surface mission on Mars and the rules of its sol count, as UTF-8 lines, returning the byte length written. |
| `hc_mission_sol(mission: *const u8, mission_len: usize, unix_seconds: f64) -> i64` | `planetary` | The sol number of a Mars surface mission at a POSIX instant, by the mission's own clock, or an error sentinel. |
| `hc_bodies(buffer: *mut u8, capacity: usize) -> i64` | `planetary` | Every body `hc-planetary` carries, with its solar day, as UTF-8 lines, returning the byte length written. |
| `hc_body_time(body: *const u8, body_len: usize, unix_seconds: f64, east_longitude_degrees: f64, buffer: *mut u8, capacity: usize) -> i64` | `planetary` | Local mean solar time on a body at a POSIX instant and an east longitude, as one UTF-8 line, returning the byte length written. |
| `hc_circad_date(calendar: *const u8, calendar_len: usize, unix_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `planetary` | The date at a POSIX instant in a calendar of another body's days, as one UTF-8 line, returning the byte length written. |
| `hc_proper_time(speed_metres_per_second: f64, coordinate_seconds: f64, buffer: *mut u8, capacity: usize) -> i64` | `relativity` | A clock moving at a constant speed while some coordinate time passes, as one UTF-8 line, returning the byte length written. |
| `hc_gravitational_dilation(body: *const u8, body_len: usize, radius_metres: f64, buffer: *mut u8, capacity: usize) -> i64` | `relativity` | A clock held still at a radius from a body's centre, against one far from every mass, as one UTF-8 line, returning the byte length written. |
| `hc_gravitating_bodies(buffer: *mut u8, capacity: usize) -> i64` | `relativity` | Every body `hc-relativity` carries a gravitational parameter for, as UTF-8 lines, returning the byte length written. |
| `hc_territories(locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `places` | Every territory CLDR 48 names, with its name in a locale, as UTF-8 lines, returning the byte length written. |
| `hc_subdivisions(country: *const u8, country_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `places` | The ISO 3166-2 subdivisions of a country CLDR 48 names, with their names in a locale, as UTF-8 lines, returning the byte length written. |
| `hc_place_name(code: *const u8, code_len: usize, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `places` | One territory or subdivision, as the UTF-8 line `hc_territories` or `hc_subdivisions` writes for it, returning the byte length written. |
| `hc_relative_time(then_unix: i64, now_unix: i64, style: *const u8, style_len: usize, automatic: i32, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `humanize` | How one POSIX instant reads from another, *3 hours ago* or *in 2 days*, in a locale, as one UTF-8 line, returning the byte length written. |
| `hc_relative_day(then_fixed: i64, now_fixed: i64, style: *const u8, style_len: usize, automatic: i32, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `humanize` | Which calendar day one fixed day is, seen from another, *yesterday* or *3 days ago*, in a locale, as one UTF-8 line, returning the byte length written. |
| `hc_relative_day_at(then_fixed: i64, now_fixed: i64, seconds_of_day: u32, style: *const u8, style_len: usize, automatic: i32, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `humanize` | Which calendar day one fixed day is, seen from another, with a time of day, *yesterday at 15:05*, in a locale, as one UTF-8 line, returning the byte length written. |
| `hc_duration(seconds: i64, style: *const u8, style_len: usize, max_components: u32, locale: *const u8, locale_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `humanize` | A span of seconds phrased in days, hours, minutes and seconds, *2 hours and 30 minutes*, in a locale, as one UTF-8 line, returning the byte length written. |
| `hc_zone_name(zone: *const u8, zone_len: usize, unix_seconds: i64, locale: *const u8, locale_len: usize, field: *const u8, field_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `zone-names` | A time zone's name at a POSIX timestamp in a locale, as a CLDR pattern field writes it, as one UTF-8 line, returning the byte length written. |
| `hc_format_pattern(zone: *const u8, zone_len: usize, unix_seconds: i64, locale: *const u8, locale_len: usize, syntax: *const u8, syntax_len: usize, pattern: *const u8, pattern_len: usize, buffer: *mut u8, capacity: usize) -> i64` | `zone-names` | An instant formatted in a time zone and a locale by a CLDR or a `strftime` pattern, as one UTF-8 line, returning the byte length written. |

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
as a table of TT(BIPMxx) − TAI − 32.184 s every ten days; in TT(BIPM25)
the difference is 27.67 µs at MJD 60 669, 25 December 2024. The
realisations are revised, so the module
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

`TTBIPM.2025` gives 27.6740 µs for MJD 58 479, 27 December 2018; at 0 h
UTC that day, TAI second 1 545 868 837, the offset is 0.000027674 s and
TT(BIPM25) − TAI is 32.184 027 674 s.

### CCSDS time codes

The CCSDS time codes of CCSDS 301.0-B-4 need the `time-codes` feature, a
layer of their own beside this one, and come from `hc-core`'s and
`hc-format`'s `ccsds`; `docs/systems/ccsds-time-codes.md`
works the standard's example through each. A binary code is an optional
P-field that names its format and a T-field that holds the time: CUC, a
count of TAI seconds and binary fractions from 1958; CDS, a count of UTC
days from 1958 with the millisecond of the day; and CCS, the UTC calendar
reading in decimal digits. The octets cross as hexadecimal text, two
digits an octet, in either case. Every instant is a TAI one, the
`tai_seconds` and `attoseconds` above, and each line gives it again as a
UTC label from the leap-second table, whose ends `strict` non-zero
refuses with `HC_ERR_NO_DATA`, as for `hc_tai_from_unix`.
`hc_ccsds_decode(hex_ptr, hex_len, strict, buffer, capacity)` reads a
P-field and exactly the T-field it announces, and writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | code | `cuc`, `cds` or `ccs` |
| 2 | tai seconds | the instant as whole TAI seconds from 1970-01-01 00:00:00 TAI |
| 3 | tai attoseconds | and its attoseconds |
| 4 | unix seconds | its UTC label: the POSIX second, or for a leap second the one after it |
| 5 | leap second | `1` for an inserted `23:59:60`, else `0` |
| 6 | utc attoseconds | and the label's attoseconds |

A CUC code counts TAI, and its UTC label is from the table; CDS and CCS
count UTC, and a millisecond of the day past 86 399 999 is a leap second,
which the table must end the day in. Octets that are not a code, or fields
out of their range, are `HC_ERR_MALFORMED`; a Level 2 code, whose epoch
only its agency knows, and a Level 3 or 4 code, which only its agency can
read, are `HC_ERR_NO_DATA`. `hc_ccsds_encode(tai_seconds, attoseconds,
p_field_ptr, p_field_len, strict, buffer, capacity)` writes one line of
one cell, the code of an instant in the format a P-field names, in
lower-case hexadecimal, P-field first, the finer part of the second
floored to the format's resolution. CDS counts 1988-01-18T17:20:43.123456
UTC, TAI second 569 524 867, as `412ade03b8ce7301c8` with a 16-bit day and
microseconds; CUC counts 2000-01-01T00:00:00 UTC, TAI second 946 684 832,
as `1c4effa220`.

A Level 2 code has an epoch "necessary to obtain … from an external
source" (§1.3), and a page that has it gives it:
`hc_ccsds_decode_from_epoch(hex_ptr, hex_len, epoch_tai_seconds,
epoch_attoseconds, epoch_unix_day, strict, buffer, capacity)` writes
`hc_ccsds_decode`'s line, a CUC code counted as TAI seconds from the
instant `epoch_tai_seconds` and `epoch_attoseconds`, and a CDS code as
UTC days from the POSIX day `epoch_unix_day`;
`hc_ccsds_encode_from_epoch(tai_seconds, attoseconds, p_field_ptr,
p_field_len, epoch_tai_seconds, epoch_attoseconds, epoch_unix_day, strict,
buffer, capacity)` writes `hc_ccsds_encode`'s. A Level 1 code counts from
1958 January 1 whatever the epoch, and a CCS or Level 3 or 4 code is as
before. An epoch outside the years 1 to 9999, and an instant before the
epoch or past the format's last count from it, are `HC_ERR_OUT_OF_RANGE`.
Annex B3.2 names 1950 January 1, "exactly 2922.0 days" before 1958, as an
agency's epoch: from POSIX day −7 305 the standard's instant is Level 2
CDS day 13 896, P-field `0100 1001`, `49364803b8ce7301c8`, which reads
back as the Level 1 `412ade03b8ce7301c8` does. A CUC count is read as
seconds of TAI; a code whose agency counts another scale is outside
these exports.

The ASCII codes A, `YYYY-MM-DDThh:mm:ss.d→dZ`, and B,
`YYYY-DDDThh:mm:ss.d→dZ`, are read by `hc_ccsds_ascii_parse(code_ptr,
code_len, strict, buffer, capacity)` into one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | variation | `a` or `b` |
| 2 | tai seconds | as for `hc_ccsds_decode`, of the start of the span a truncated code names |
| 3 | tai attoseconds | as for `hc_ccsds_decode` |
| 4 | unix seconds | as for `hc_ccsds_decode` |
| 5 | leap second | as for `hc_ccsds_decode` |
| 6 | utc attoseconds | as for `hc_ccsds_decode` |
| 7 | precision | how far the time part runs: `hour`, `minute`, `second` or `fraction` |
| 8 | digits | for `fraction`, its digits, 1 to 18; else empty |
| 9 | terminator | `1` if the optional `Z` follows, else `0` |

Text that is not a code, a calendar or time subset used alone included,
is `HC_ERR_MALFORMED`. `hc_ccsds_ascii_format(tai_seconds, attoseconds,
variation_ptr, variation_len, precision_ptr, precision_len, terminator,
strict, buffer, capacity)` writes one line of one cell, the code of an
instant's UTC label: `variation` is `a` or `b`, `precision` is `hour`,
`minute`, `second` or the digits of the fraction, `1` to `18`, either in
any case and anything else `HC_ERR_UNKNOWN`, and `terminator` non-zero
ends it with `Z`. The reading is truncated, never rounded, as §3.5.1.3
allows a code to be. The standard's example is `1988-018T17:20:43.123456Z`
in code B.

### Radio time codes

`hc_radio_decode(code_ptr, code_len, frame_ptr, frame_len, century,
buffer, capacity)`, in the `time-codes` feature too, reads one minute's
frame of a long-wave time station,
from `hc-format`'s `radio`: `code` is `jjy` (NICT, JST), `dcf77` (PTB, CET
or CEST), `wwvb-am` (NIST's amplitude code, UTC) or `wwvb-pm` (its phase
code, UTC), in any case, and anything else is `HC_ERR_UNKNOWN`. The frame
is one character a second, `0`, `1`, and for `jjy` and `wwvb-am` `M` for a
marker, in either case; 59, 60 or 61 of them as the minute has seconds,
or for `dcf77` 59 or 60 marks. The two-digit year needs a century, the
first year of it, a multiple of 100 from 0 to 9900; any other is
`HC_ERR_OUT_OF_RANGE`. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | unix seconds | the POSIX second of the minute the frame names: its first marker, or for `dcf77` the minute it announces |
| 2 | fixed | that minute's fixed day in the code's own time |
| 3 | hour | its hour in the code's own time |
| 4 | minute | its minute |
| 5 | offset hours | the code's time less UTC: 9, 1 or 2, or 0 |
| 6 | seconds | the frame's length, 59 to 61 |
| 7 | leap | the leap second announced, `none`, `positive` or `negative`; DCF77's A2 and the amplitude code's bit 56 carry no sign and read as `positive`, the one kind their frames make room for |
| 8 | summer | `cet` or `cest` for `dcf77`; `standard`, `begins-today`, `in-effect` or `ends-today` for WWVB; empty for `jjy` |
| 9 | zone change | DCF77's A1: `1` when CET and CEST change at the end of the hour, else `0`; empty for the other codes |
| 10 | dut1 tenths | UT1 − UTC in tenths of a second for `wwvb-am`; else empty |
| 11 | dst next | the phase code's six-bit `dst_next` word, Table 8's schedule of the next change, for `wwvb-pm`; else empty |

A frame that is not the code's — a wrong length, symbol, BCD digit or
parity bit, a date that does not exist, a weekday not the date's — is
`HC_ERR_MALFORMED`. JJY's frames of minutes 15 and 45 carry the call sign
and no year, and are `HC_ERR_NO_DATA`; `hc_jjy_call_sign_decode`, below,
reads them in a year the caller names. `hc_radio_encode(code_ptr,
code_len, unix_seconds, leap, summer_ptr, summer_len, zone_change,
dut1_tenths, dst_next, buffer, capacity)` writes one line of one cell, the
frame for the minute that begins at `unix_seconds`, a whole minute of the
years 1 to 9999; for `dcf77` it is the frame sent during the minute
before, which announces it. `leap` is 1 for a second inserted at the end
of the month (for `dcf77`, of the hour), −1 for one omitted, which only
`jjy` and `wwvb-pm` can say, or 0; `summer` is column 8's name, which for
`dcf77` also sets the offset and is empty for `jjy`; `zone_change` is read
by `dcf77` alone, `dut1_tenths`, −9 to 9, by `wwvb-am` alone, and
`dst_next`, a word Table 8 lists for the direction `summer` gives, by
`wwvb-pm` alone. The third party's bits, the call bit and the reserved and
notice bits are written 0. A value a code cannot say is
`HC_ERR_OUT_OF_RANGE`, and a `summer` it does not name `HC_ERR_UNKNOWN`.

In a module built with `tz` too, `summer` may instead be `zone:` and a
zone's name, and the state is read from the rules
`hc_fixed_from_unix_in_zone` reads for that name — a loaded TZif file
first, then the built-in table — so that the frames and the page's days
cannot disagree:

- For `dcf77`, `zone:Europe/Berlin`, the zone of German legal time: Z1 Z2
  are CET where the rules give UTC+1 and standard time at the minute, CEST
  where UTC+2 and summer time, and a minute at which the zone keeps
  neither is `HC_ERR_OUT_OF_RANGE`. A1 replaces `zone_change`: PTB sends it
  "for one hour" before a change, "from 01:00:16 h CET (02:00:16 h CEST)
  until 01:59:16 h CET (02:59:16 h CEST)" (`ptb-dcf77-timecode`), so it is
  set in the frames announcing the minutes after the hour before a change
  through the change itself — for 29 March 2026, the frames for 00:01 to
  01:00 UTC.
- For WWVB, any zone that keeps the United States' rule:
  `zone:America/Denver`, the station's, or `zone:America/New_York`, both
  of which the built-in table carries. SP 250-67 sets bit 57 "At 0000 UTC on the day
  ST changes to DST" and bit 58 "at 0000 UTC the following day", and
  clears them the same way when DST ends (`nist-sp250-67`), so bit 57 is
  whether the rules keep summer time at 24:00 UTC ending the minute's UTC
  day and bit 58 whether at 00:00 UTC beginning it. The phase code's
  `dst_next` replaces the caller's too: "When DST is in effect … the
  DST_NEXT field provides advance notification for the end of the DST
  period in the fall, whereas when DST is not in effect … for the
  beginning of the next DST period in the upcoming spring"
  (`nist-wwvb-enhanced-2013`, §4.6), and Table 8's words are read with
  bit 57, so the word names the rules' next change after the minute's UTC
  day: its Sunday and its hour on the clock before it where Table 8 has
  that schedule, word 49 where it does not, and word 50 or 51 for a zone
  that changes neither way within the year. From 00:00 UTC on 8 March
  2026 New York's frames name 1 November at 2 AM, and from 00:00 UTC on
  1 November 14 March 2027, both `011011`, 27.

`jjy` with a zone, a name nobody knows, and a module without `tz`, are
`HC_ERR_UNKNOWN`.
NICT's figure of 17:25 JST on 1 April 2004 is POSIX second 1 080 807 900
and `M01000101M000100111M000001001M001000010M000000100M100000000M`.

### JJY's call-sign frames

At 15 and 45 minutes past each hour JJY sends its call sign in Morse in
seconds 40 to 48, and in place of the year, the weekday and the leap
second, a notice of a planned stop, ST1 to ST6 (`nict-jjy-timecode`).
`hc_jjy_call_sign_decode(frame_ptr, frame_len, year, buffer, capacity)`
reads such a frame, the symbols as for `hc_radio_decode`, in the Gregorian
`year`, 1 to 9999, that its day of the year is counted in; any other year
is `HC_ERR_OUT_OF_RANGE`. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | unix seconds | the POSIX second of the frame's first marker |
| 2 | fixed | that minute's fixed day in JST |
| 3 | hour | its hour in JST |
| 4 | minute | its minute, 15 or 45 |
| 5 | stop start | ST1–ST3 as a number, NICT's 停波開始予告: 0 no stop planned, 1 within seven days, 2 within three to six, 3 within two, 4 within 24 hours, 5 within 12, 6 within 2 |
| 6 | daytime only | ST4: `1` for a stop by day only (昼間のみ), `0` for one all day or none planned |
| 7 | stop span | ST5–ST6 as a number, 停波期間予告: 0 no stop planned, 1 seven days or more or not known, 2 two to six days, 3 less than two |

An ordinary minute's frame, which `hc_radio_decode` reads, a frame that is
not JJY's, ST1–ST3 `111`, which NICT does not define, and a day the year
does not have are `HC_ERR_MALFORMED`.
`hc_jjy_call_sign_encode(unix_seconds, stop_start, daytime_only,
stop_span, buffer, capacity)` writes the frame for the minute that begins
at `unix_seconds`, minute 15 or 45 of an hour of JST, with that notice,
as `hc_radio_encode` writes one; any other minute, a `stop_start` above 6
or a `stop_span` above 3 is `HC_ERR_OUT_OF_RANGE`. NICT's second figure,
17:15 JST on 1 April 2004, no stop planned, is POSIX second 1 080 807 300,
and read in 2004 it is fixed day 731 672; in 2003 its day 92 is 2 April.
A stop within 24 hours, by day only, for two to six days writes seconds
50 to 55 as `100110`.

### IRIG time codes

`hc_irig_decode(signal_ptr, signal_len, frame_ptr, frame_len, year,
buffer, capacity)`, in the `time-codes` feature, reads one frame of an
IRIG serial time code, A, B, D, E, G or H, from `hc-format`'s `irig` after
IRIG Standard 200-16. `signal` is a signal designation, the format letter
and three digits — modulation, carrier and coded expression — as `B124`,
each digit one its Table 4-1 permits the format, in any case; anything
else is `HC_ERR_UNKNOWN`. The coded expression says which fields the
frame carries, and a frame cannot be read without it. The frame is
written as the radio codes' are, one character an index count: `M` for a
position identifier or the reference bit Pr, and `0` and `1`, Pr first.
A code with the year's last two digits reads them in the century `year`
is in; a code without them is read in `year`; `year` is 1 to 9999, and
any other is `HC_ERR_OUT_OF_RANGE`. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | fixed | the day of the reading at Pr |
| 2 | day of year | 1 January being 1, as the frame sends it |
| 3 | hour | 0 to 23 |
| 4 | minute | 0 in format D |
| 5 | second | 60 for a leap second; 0 in D and H, a multiple of 10 in E |
| 6 | hundredths | a multiple of 10 in A, any in G, 0 in the others |
| 7 | year | the year's two digits; empty for a code without them |
| 8 | control | the control bits as a number, control bit 1 lowest; empty for a code without them |
| 9 | straight binary seconds | the seconds of the day; empty for a code without them |

The code carries no time scale, so the reading is a date and a time of
whatever clock the generator keeps. A frame that is not the code's — a
wrong length or symbol, a 1 where the code sends an index marker, a BCD
digit or a time out of range, straight binary seconds that disagree, a
day the year lacks, a year's digits not `year`'s, or a leap second
anywhere but at the end of a month — is `HC_ERR_MALFORMED`.

`hc_irig_encode(signal_ptr, signal_len, fixed, seconds_of_day,
hundredths, control, buffer, capacity)` writes the frame of a code whose
Pr falls at a reading of the civil clock: a fixed day of the years 1 to
9999, whole seconds after its midnight, 86 400 being 23:59:60, and
hundredths, 0 to 99, with the control bits, control bit 1 lowest. It
writes one line of one cell, the frame. A reading the format has no frame
at — for B one off the second, for D one off the hour — control bits the
code has no room for, or a value out of those ranges is
`HC_ERR_OUT_OF_RANGE`. The standard's Figure 5-2, IRIG B on day 173 of
2003 at 21:18:42, is the B124 frame
`M01000001M000101000M100000100M110001110M100000000M110000000M000000000M000000000M010011011M101010010M`.

`hc_irig_formats(buffer, capacity)` lists the six formats, so that a page
need not keep the standard's tables itself: which designations to offer,
and where each format's frames begin, to round a reading down to one
before `hc_irig_encode`. It writes one line a format, A first:

| # | Column | Holds |
| --- | --- | --- |
| 1 | format | the letter, `A`, `B`, `D`, `E`, `G` or `H` |
| 2 | index count microseconds | the index count interval, the rate's reciprocal (Table 3-1): 1 000 for A's 1 000 pulses a second, 60 000 000 for D's one a minute |
| 3 | index counts | the index counts in a frame, 100, or 60 for D and H (Table 3-2) |
| 4 | frame microseconds | the frame's length, 100 000 for A to 3 600 000 000 for D: a frame begins at every multiple of it from midnight, and `hc_irig_encode` writes one at such a reading only |
| 5 | fields | the fields of the BCD time of year, most significant first, separated by spaces: `days hours minutes seconds tenths` for A, `days hours` for D, `tens-of-seconds` for E's seconds; the last is the frame's length |
| 6 | control bits | the control bits the format has room for (Table 3-4): 18, 9 for D and H, 27 for G |
| 7 | modulations | the modulation digits Table 4-1 permits, separated by spaces, `0 1 2` |
| 8 | carriers | the carrier digits it permits, `0 2 3 4 5` for B |
| 9 | expressions | the coded expressions it permits, `0 1 2 3 4 5 6 7` for B: 4 to 7 carry the year, 0, 1, 4 and 5 the control bits, and 0, 3, 4 and 7 the straight binary seconds (Figure 4-1) |

B's line is `B`, `10000`, `100`, `1000000`, `days hours minutes seconds`,
`18`, `0 1 2`, `0 2 3 4 5`, `0 1 2 3 4 5 6 7`. The radio codes need no such
list: every frame of every code is one minute, beginning on the minute.

### The start of an IRIG frame

`hc_irig_frame_start(signal_ptr, signal_len, seconds_of_day, hundredths,
buffer, capacity)`, in the same feature, rounds a reading of the civil
clock down to the start of the frame that holds it, the reading
`hc_irig_encode` takes, so that a page keeps no frame lengths of its own:

| # | Column | Holds |
| --- | --- | --- |
| 1 | seconds of day | the start of the frame, whole seconds after midnight |
| 2 | hundredths | its hundredths of a second |
| 3 | frame micros | the frame's length in microseconds, Table 3-2's: 1 000 000 for B, 3 600 000 000 for D |

01:02:05.57 is the frame of 01:02:05 in B000, of 01:00:00 in D001 and of
01:02:05.50 in A000. 86 400, 23:59:60, is a frame of its own where the
frame is a second or less and falls in the day's last frame where it is
longer.

### .NET ticks

`hc_dotnet_ticks_from_unix(unix_seconds, attoseconds)` answers .NET's
`DateTime.Ticks` of a POSIX instant as a `Utc` value, the 100-nanosecond
intervals from 0001-01-01 00:00 floored to the tick, from `hc-core`'s
`dotnet`: 621 355 968 000 000 000 at 1970-01-01, and 3 155 378 975 999 999
999 at `DateTime.MaxValue`, 9999-12-31 23:59:59.9999999. An instant
outside those years is `HC_ERR_OUT_OF_RANGE`.
`hc_unix_from_dotnet_ticks(ticks, buffer, capacity)` reads ticks back and
writes one line of two cells, the whole seconds from 1970-01-01 00:00 and
the attoseconds: POSIX time for a `Utc` value, and for a `Local` or
`Unspecified` one the wall clock of the zone the value does not name. The
Kind travels beside the ticks, and the caller knows it; `ToBinary`'s
packing of the two is not read.

### Six-hour clocks

`hc_six_hour_clock(reckoning_ptr, reckoning_len, seconds_of_day, buffer,
capacity)` reads a time of the civil day, whole seconds after the caller's
wall-clock midnight, on the Ethiopian or the Swahili twelve-hour dial,
counted from about sunrise and about sunset, from `hc-format`'s
`east_african_hours`. `reckoning` is `ethiopian-hours`, whose day half
runs from 06:00 to 17:59:59, or `swahili-hours`, from 07:00 to 18:59:59,
in any case, and anything else is `HC_ERR_UNKNOWN`; a time from 86 400 s
is `HC_ERR_OUT_OF_RANGE`. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | hour | the hour on the dial, 1 to 12: the civil hour less six |
| 2 | minute | the civil clock's minute |
| 3 | second | the civil clock's second |
| 4 | half | `day` or `night` |
| 5 | period | the part of the day the source names for the civil hour, in its language: `usiku`, `alfajiri`, `asubuhi`, `mchana`, `jioni`; empty for `ethiopian-hours`, which carries none |
| 6 | period english | the source's English gloss; else empty |

`hc_civil_from_six_hour_clock(reckoning_ptr, reckoning_len, hour, minute,
second, night)` is the inverse, the seconds after civil midnight of a
reading, `night` non-zero for the night half; an hour outside 1 to 12 or a
minute or second above 59 is `HC_ERR_OUT_OF_RANGE`. 8 am is 2 o'clock of
the Ethiopian day, and 7:00 pm *saa moja usiku*, the first hour of the
Swahili night.

### French Republican decimal time

`hc_french_decimal_time(seconds_of_day, attoseconds, buffer, capacity)`,
in the `timestamps` feature, writes the decimal time of article XI of the
decree of 4 frimaire an II, from `hc-calendars-solar`'s
`french_republican::DecimalTime`: ten decimal hours from midnight to
midnight, a hundred decimal minutes to the hour and a hundred decimal
seconds, 0.864 s each, to the minute, all exact; noon is 5 hours. It was
compulsory in public acts from 22 September 1794 until the law of 7 April
1795 suspended it. 86 400, 23:59:60, is a leap second the ten hours have
no place for, and is `HC_ERR_NO_DATA`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | hour | the decimal hour, 0 to 9 |
| 2 | minute | the decimal minute, 0 to 99 |
| 3 | second | the decimal second, 0 to 99 |
| 4 | attoseconds | the rest of the decimal second, in attoseconds of ordinary time |

`hc_civil_from_french_decimal_time(hour, minute, second, attoseconds,
buffer, capacity)` is the inverse, one line of two cells, the whole
seconds after midnight and the attoseconds; an hour past 9, a minute or
second past 99, or attoseconds of a decimal second or more is
`HC_ERR_OUT_OF_RANGE`.

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
column 17 of every line names the locale data that answered. A tag that does not parse
falls back to the root locale `und`, as `hc-i18n` does, whose month names
are CLDR's `M01`..`M12` — ask for `en` for English. A null pointer with a
zero length is `und` too.

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the calendar's identifier, `gregory`, `chinese`, `japanese`, ... |
| 2 | name | its English name |
| 3 | era | the era code, `reiwa`, `AD`, `AH`, `roc`, ..., or empty for a calendar without eras |
| 4 | era label | the era's name in the locale, 令和, or the calendar's own name for it, 嘉永 or `Kaei`, else its English name, `Minguo` under `ja`: the name the formatted date writes, never the code of column 3; empty only for an era nothing names |
| 5 | year | the year, as the calendar counts it |
| 6 | month | the month's ordinal from 1, or empty for a calendar without months |
| 7 | leap month | `1` for an intercalary month, else `0` |
| 8 | month label | the month's name in the locale, 閏二月, or the calendar's own name for it, or empty |
| 9 | day | the day of the month, or empty |
| 10 | leap day | `1` for a repeated day, else `0` |
| 11 | extras | the calendar's extra fields as identifiers and integers for a program, `baktun=13;katun=0;...`, or empty; [`hc_day_extras`](#the-extra-fields-of-a-day) labels them for a reader |
| 12 | error code | empty when the day converted; otherwise the refusal's code |
| 13 | error name | empty when the day converted; otherwise its name |
| 14 | standing | `in-use`, `proleptic`, `extended` or `unrecorded`; empty on a refusal |
| 15 | day boundary | where the calendar's day begins: `midnight`, `noon`, `sunset`, `sunrise`, `daybreak` (sunrise in summer and dawn in winter, `icelandic-medieval`'s) or `local-time HH:MM:SS` |
| 16 | formatted | the date as the locale writes it — 令和8年9月21日, 2023癸卯年闰二月初一, `September 21, 2026`, `13.0.13.17.8` — from `hc_format::label`: text for a reader, which holds an extra field only where the calendar's sources write the date with it and never a `name=value` pair, a field's identifier or an era's code; empty on a refusal |
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
the Chinese calendar's year by the related Gregorian year and its stem and
branch, 2023癸卯年, and its days
by their Han names, 初一 … 三十; the Japanese first year of an era as 元年.
Every template names the CLDR pattern it was read from. A locale that has
stated none gets the fields in order, separated by spaces, in the names
the library already has; nothing is invented. An era's name is the
locale's (for an era several calendars count, such as the Śaka era,
the one the calendar that names it for all of them gives), else the
calendar's own (every nengō, 嘉永, romanised as *Kaei* for a Latin-script
locale), else English's — `Minguo` under `ja`, `Old Style` under `de`,
`Saka` under `sa` — and never its code; a month's is the locale's, else
the calendar's own shape name, else its number; the Gregorian family's
`AD` and the Hebrew calendar's `AM` are left unwritten, as those calendars
are printed.

**The extra fields** (column 11) are written in the date only where the
calendar's sources write its dates with them, and then as those sources
do, in the calendar's own notation (`hc_i18n::notation`) or in a
locale's template for the calendar, each naming its source: the Long
Count's `13.0.13.17.8`, the ISO week date `2026-W39-7`, the Tzolkʼin's
`1 Lamat`, a Tamil year's name, `Purattasi 11 of the year Parabhava in
southern reckoning, 1948 Saka`. The rest — a day count's Julian Day Number, the Gregorian
year a Masonic year counts on, the Vikrama year of an amānta date — are
metadata and stay out of the text; a calendar whose sources say nothing
of how its dates are written is written by its year, month and day.
`crates/hyper-calendar/tests/readable_dates.rs` holds every registered
calendar, on a spread of days, in every carried locale and `native`, to
a formatted date and unit labels with no `=`, no field identifier and
no era code.

## The extra fields of a day

`hc_day_extras(fixed, id_ptr, id_len, locale_ptr, locale_len, buffer, capacity)`
needs the `calendars` feature and writes one line per extra field of the
day, in registry order and within a calendar in the order it sets them:
every registered calendar's with an empty `id`, only that calendar's
with a registry identifier, and `HC_ERR_UNKNOWN` for an identifier the
module does not know. A calendar that refuses the day, or whose date has
no extra fields, writes no line. `locale` is as for `hc_describe_day`,
`native` included, and the calendar's locale is chosen by the same rule.
This is where the extra fields live for a page that shows them beside
the date: the identifiers and integers of column 11 of
`hc_describe_day`, labelled.

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the calendar's identifier |
| 2 | field | the field's identifier, `samvatsara`, `barhaspatya-samvatsara`, `julian-day-number`, `tzolkin_name`: a key, as column 11 of `hc_describe_day` has it, and never text for a reader |
| 3 | value | its value, an integer |
| 4 | label | what the locale calls the field, else its English label, `Samvatsara (southern reckoning)`, `Julian Day Number`, `Tzolkʼin day sign`, from `hc_i18n::fields`; English's are the words the calendars' system documents use, and no other language's is carried yet |
| 5 | value label | the value as a reader reads it: the name of the position it holds where the field's values are named — `Parabhava`, பராபவ under `ta`, `Lamat`, 丙午, `Sunday` — a flag's `no` or `yes`, or its calendar's own words for the two, the Burmese half's `waxing` and `waning` — else the number in the locale's numbering system; what a template's `{extra:FIELD}` writes |
| 6 | in date | `1` when column 16 of `hc_describe_day`, the formatted date, already writes the field, or writes a field whose value has the same name in the calendar's own words — the Burmese phase `waning` is the half `waning` the date writes, and the full moon, `waxing` 15, is not — else `0`, so that a page shows the others beside it |
| 7 | locale used | the tag of the locale data that answered, as column 17 of `hc_describe_day` names it |

On 27 September 2026 under `en`, the Tamil solar calendar's two lines
are `hindu-solar-tamil`, `samvatsara`, `40`,
`Samvatsara (southern reckoning)`, `Parabhava`, `1`, `en` and `hindu-solar-tamil`, `tiruvalluvar-year`, `2057`,
`Tiruvalluvar year`, `2057`, `0`, `en`; the Modified Julian Day's one is
`modified-julian-day`, `julian-day-number`, `2461311`,
`Julian Day Number`, `2461311`, `0`, `en`. The pūrṇimānta calendar names
the same year in the northern cycle, under a key of its own:
`hindu-lunar-purnimanta`, `barhaspatya-samvatsara`, `53`,
`Barhaspatya samvatsara (northern cycle)`, `Siddharthin`, `1`, `en`, and
its date is `Asvina 16 of the Barhaspatya year Siddharthin, 1948 Saka`
where the Tamil one is `Purattasi 11 of the year Parabhava in southern
reckoning, 1948 Saka`.

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
| 3 | label | the span's label in the locale: 令和元年, 令和6年, `5784`, `1445 AH`, 2023癸卯年, `Adar I`, 閏二月, 初四; empty on a refusal |
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

## A written date, read back

`hc_parse_date(calendar_ptr, calendar_len, locale_ptr, locale_len, text_ptr, text_len, buffer, capacity)`
needs the `calendars` feature and reads a date as a locale writes it in
one calendar: what column 16 of `hc_describe_day` writes, and a reader's
own spelling of it — 令和8年9月28日 and 令和八年九月二十八日, 康熙五十二年十一月初一,
*Monday, Sep. 28, 2026*, ٢٨ سبتمبر ٢٠٢٦ and *28 Eylül 2026*. `calendar`
is a registry identifier, and one the registry does not carry is
`HC_ERR_UNKNOWN`; `locale` is as for `hc_describe_day`, and the text is
read in the locale that export's line for the calendar is written in. The
reader walks the templates the formatter fills, tries every name of the
locale's chain at every width, and numbers in the locale's digits, Latin
digits and, in a locale written in Han characters, Han numerals and 元;
a match is kept only where the calendar's own fields for the day agree
with everything the text says. `hc-format`'s `label::parse_date` is the
reader; `docs/systems/written-dates.md` in the repository explains it.

It writes one line: the 18 columns of `hc_describe_day` for the calendar
and the day the text names, then column 19, `fixed`, the fixed day. A text
that is not one day is still a line, its date columns, standing,
formatted date and fixed day empty, and the error columns say why:

| Code | Name | The text |
| ---: | --- | --- |
| 101 | `empty` | is empty or white space |
| 102 | `not-recognised` | matches no way the locale writes the calendar's dates |
| 103 | `ambiguous` | reads as two days or more: an era name the calendar gives two eras, a week, a doubled day the text does not mark |
| 104 | `two-digit-year` | writes the year in one or two digits with no era, in a calendar whose years run longer: *September 28, 26* |
| 105 | `year-not-written` | names the year only by a cycle that recurs, 癸卯年, or not at all: a Tzolkʼin day, 9月28日 |
| 106 | `weekday-mismatch` | names a weekday that is not the day's |
| 107 | `field-mismatch` | writes a year, a month and a day, and a field beside them the day does not have: 2025丙午年八月十八, whose year is 乙巳 |
| 1–11 | the calendar's | reads as fields the calendar has no day for, *February 30*, with the calendar's code and name |

## The calendars

`hc_calendars(today, locale_ptr, locale_len, buffer, capacity)` needs the
`calendars` feature and writes one line per registered calendar, in
registry order. `locale` is as for `hc_describe_day`; `today` is the fixed
day the standing is judged on, because the module has no clock. The names
are CLDR 48's, `localeDisplayNames/types/type[@key="calendar"]`, at its
`approved` and `contributed` levels, and for a few calendars CLDR has no
key for, a source's in the language: the Tibetan almanac's `tibetan`,
`tibetan-tsurphu` and `mongolian` in `bo`, `ja`, `mn` and `zh`, 藏历,
楚尔派 and 蒙古历 under `zh-Hans`; a calendar neither names in the locale
has an empty name. No two rows share a name, in column 2 or in
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

A description of a day is one conversion in each of the 229 calendars,
and a few dozen of them search the sky to convert: the Hindu lunar
calendar and the nine built on it for conjunctions and saṅkrāntis at
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
| 2026-09-27, `ja` | `hc_describe_day` | 8.8 ms | 19.9 ms |
| 2026-01-01, `ja` | `hc_describe_day` | — | 18.3 ms |
| 1900-06-15, `ja` | `hc_describe_day` | — | 19.6 ms |
| 2026-09-27, `ja` | `hc_calendars` | 18.8 ms | 42.5 ms |
| 2026-09-27, `native` | `hc_calendars` | — | 42.8 ms |
| `ja` | `hc_calendar_list` | 0.35 ms | 0.71 ms |
| `native` | `hc_calendar_list` | — | 0.92 ms |

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

### Day periods

`hc_day_period(seconds_of_day, locale_ptr, locale_len, buffer, capacity)`,
in the same feature, writes the day periods of a time of the civil clock
in a locale, from CLDR 48's `dayPeriods` rules and names, as `hc-format`'s
`B` and `b` fields read them (`docs/systems/zone-names.md` has the day
periods beside the zone names): the half of the day, and the flexible
period the locale's rules give the minute, *in the afternoon*.

| # | Column | Holds |
| --- | --- | --- |
| 1 | half | `am` or `pm` |
| 2 | half name | the locale's abbreviated name for it in a date's format context, *PM* |
| 3 | period | `midnight` or `noon` at 00:00:00 or 12:00:00 where the language has a word for it, else the flexible period of its rules, `morning1` to `night2`; empty where the language has no rules |
| 4 | abbreviated | the period's abbreviated name |
| 5 | wide | its wide name |
| 6 | narrow | its narrow name |
| 7 | locale used | the tag of the locale data that answered |

At 15:00 English writes `pm`, *PM*, `afternoon1`, *in the afternoon*.
`seconds_of_day` from 86 400 is `HC_ERR_OUT_OF_RANGE`.

### Numbering systems

`hc_format_number(system_ptr, system_len, value, buffer, capacity)` writes
an integer in a CLDR numbering system, from `hc-i18n`'s `numbering`: a
positional one's digits, `arab`'s ٢٠٢٦; the Hebrew numerals of `hebr`,
ה׳תשפ״ו for 5786; the Greek of `grek` and `greklow`, with the keraia; the
Han styles. One line of two cells, the text and the system's identifier.
`hc_parse_number(system_ptr, system_len, text_ptr, text_len)` reads one
back, `HC_ERR_MALFORMED` for text that is not a number in the system, and
`hc_numbering_systems(buffer, capacity)` lists every system, one line
each:

| # | Column | Holds |
| --- | --- | --- |
| 1 | system | the CLDR identifier: `latn`, `hebr`, `grek` |
| 2 | algorithmic | `1` for a system that spells numbers out, `0` for a positional one |
| 3 | digits | a positional system's ten digits, zero first; empty for an algorithmic one |

A system not listed is `HC_ERR_UNKNOWN`, and a value a system cannot
write, a Hebrew numeral of nothing among them, `HC_ERR_OUT_OF_RANGE`.

### A calendar's eras

`hc_calendar_eras(calendar_ptr, calendar_len, locale_ptr, locale_len,
buffer, capacity)` writes the eras a calendar is described with, in the
order the locale data lists them, each named in the locale, as
`hc_describe_day` names a date's era:

| # | Column | Holds |
| --- | --- | --- |
| 1 | code | the era's code, as a date's era field writes it: `reiwa`, `ce` |
| 2 | wide | its wide name in the locale, 令和 under `ja`; empty where the locale's chain has none |
| 3 | abbreviated | its abbreviated name |
| 4 | narrow | its narrow name |
| 5 | calendar | the calendar's identifier |
| 6 | locale used | the tag of the locale data that answered |

A calendar the registry does not carry is `HC_ERR_UNKNOWN`, and one whose
eras no locale data lists `HC_ERR_NO_DATA`.

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
longitudes, so it needs an ayanāṃśa, and moves with it twice over:
`ayanamsa` is an identifier, `lahiri`, `lahiri-rashtriya`, `lahiri-crc-1955`, `lahiri-drik`, `raman`, `krishnamurti`,
`reingold-dershowitz` or `fagan-bradley`, in any case, and anything else, the empty string and a
full name such as `Lahiri (Chitrapaksha)` included, is `HC_ERR_UNKNOWN`. The karaṇa, half a tithi,
needs none. The sky may instead be `surya-siddhanta`: the *Sūrya
Siddhānta*'s Sun and Moon, as `hindu-lunar-surya-siddhanta` and
`hc_surya_siddhanta_at` read them, whose yoga and karaṇa are the book's
own and differ from the true sky's by up to a limb; `hc_panchanga_of_day`
then reads the day at the book's own sunrise, and both lines name the sky
in columns 8 and 9, `surya-siddhanta` and `Sūrya Siddhānta`. On that sky
the instants answer for the days of Kali Yuga 1 to 10 000 and a place must
lie within 65° of the equator. On the true sky the instants answer for the sky layer's era, below, and a
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
| 8 | ayanamsa | the sky the yoga was reckoned on, by the identifier `ayanamsa` takes: `lahiri`, or `surya-siddhanta` on both lines; empty for the karaṇa on the true sky |
| 9 | ayanamsa name | its full name: `Lahiri (Chitrapaksha)`, `Sūrya Siddhānta`; empty with column 8 |

For 1 January 2025 at 23°11′ N, 82°30′ E, the yoga at sunrise is
Vyaghata, ending within a minute and a half of the 17:07 IST Drik Panchang
prints, and the karaṇa Balava, ending within two minutes after its 14:55.

### The nakṣatra

`hc_nakshatra_at(unix_seconds, ayanamsa_ptr, ayanamsa_len, buffer,
capacity)` and `hc_nakshatra_of_day(fixed, latitude, longitude, elevation,
ayanamsa_ptr, ayanamsa_len, buffer, capacity)`, in the same feature, write
the nakṣatra, the pañcāṅga's limb of the Moon, from `hc-calendars-indic`'s
`nakshatra`: the arc of 13°20′ of the Moon's sidereal longitude in the
ayanāṃśa's zodiac, at an instant or at a day's sunrise at a place, with the
sky and the place as for the yoga. One line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | nakshatra | 1 for Aśvinī through 27 for Revatī |
| 2 | nakshatra id | its identifier, the lower-case ASCII form of its name, words joined by a hyphen: `ashvini`, `purva-phalguni`, `revati` |
| 3 | nakshatra name | its name in IAST as the library spells the list of English Wikipedia's "Nakshatra" (`wikipedia-nakshatra`): `Aśvinī`, `Pūrva Phalgunī`, `Revatī` |
| 4 | entered | the instant the Moon entered it, as POSIX seconds, rounded down |
| 5 | leaves | the instant it leaves |
| 6 | read at | the instant read: the one asked for, or the sunrise |
| 7 | ayanamsa | the ayanāṃśa, by its identifier |
| 8 | ayanamsa name | its full name |

### The muhūrtas

`hc_muhurtas(fixed, latitude, longitude, elevation, buffer, capacity)`, in
the same feature, writes the thirty muhūrtas of a day, from
`hc-calendars-indic`'s `muhurta`: the daylight, sunrise to sunset, and the
night after it, sunset to the next sunrise, each cut into fifteen, with the
two a pañcāṅga prints by them marked. Abhijit is the eighth of the
daylight, and Drik Panchang gives none on a Wednesday; Dur Muhurtam is one
or two by the weekday, the muhūrtas Drik Panchang's New Delhi pages of
January 2025 put it in, since no statement of the rule was read. One line
per muhūrta, the day's first, then the four cells of a missing solar event
as `hc_kalam` writes them:

| # | Column | Holds |
| --- | --- | --- |
| 1 | half | `day` or `night` |
| 2 | number | 1 to 15 within the half |
| 3 | name | its name as English Wikipedia's "Muhurta" tabulates the thirty (`wikipedia-muhurta`), in the article's transliteration: `Rudra` to `Bhaga` by day, `Girīśa` to `Samudra` by night |
| 4 | start | the instant it begins, as POSIX seconds, rounded down; empty where the Sun does not rise or set |
| 5 | end | the instant it ends; empty as column 4 |
| 6 | mark | `abhijit`, `dur-muhurtam`, or empty |
| 7 | missing | the solar event the day lacks, `sunrise` or `sunset`; empty where it has both |
| 8 | missing day | the fixed day it is missing on |
| 9 | depression | empty: a muhūrta needs no depression |
| 10 | depression arcseconds | empty, as column 9 |

The article marks its table as needing citations and names no text for
it, so column 3 is a secondary source's names; where Drik Panchang calls
the eighth of the day Abhijit, the table has Vidhi, two names of one
muhūrta from two sources, and column 6 carries the pañcāṅga's.

On Wednesday 1 January 2025 at New Delhi no muhūrta is Abhijit and the
eighth of the day, Vidhi, is Dur Muhurtam; on Thursday the sixth and the
twelfth are Dur Muhurtam and the eighth is Abhijit.

### Amṛta siddhi

`hc_amrita_siddhi(fixed, latitude, longitude, elevation, ayanamsa_ptr,
ayanamsa_len, buffer, capacity)`, in the same feature, writes the *amṛta
siddhi yoga* of a day at a place, from `hc-calendars-indic`'s
`amrita_siddhi`: the auspicious conjunction of the weekday with one
nakṣatra — Hasta on Sunday, Mṛgaśīrṣa on Monday, Aśvinī on Tuesday,
Anurādhā on Wednesday, Puṣya on Thursday, Revatī on Friday, Rohiṇī on
Saturday, Prokerala's pairs, of which Sewell and Dikshit give Sunday's — for
the part of the day, sunrise to the next sunrise, the Moon spends in it.
One line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | name | `Amrita Siddhi Yoga`, as Drik Panchang prints it |
| 2 | devanagari | अमृत सिद्धि योग |
| 3 | nakshatra | the weekday's nakṣatra, 1 to 27 |
| 4 | nakshatra id | its identifier, as for `hc_nakshatra_at` |
| 5 | nakshatra name | its name, as for `hc_nakshatra_at` |
| 6 | start | the instant the yoga begins, as POSIX seconds, rounded down; empty on a day it does not fall |
| 7 | end | the instant it ends; empty as column 6 |
| 8 | falls | `1` if the yoga falls on the day, else `0` |
| 9 | ayanamsa | the ayanāṃśa, by its identifier |

Drik Panchang prints it on Tuesday 7 January 2025 from the Moon's entry
into Aśvinī, and on no day from 8 to 10 January.

### Rāhu kālam, Yamaganda and Gulika kālam

`hc_kalam(convention_ptr, convention_len, fixed, latitude, longitude,
elevation, locale_ptr, locale_len, buffer, capacity)`, in the same
feature, writes the three
inauspicious periods a pañcāṅga marks on a day, each an eighth of the day
that the weekday picks, from `hc-calendars-indic`'s `kalam`. What the day
is differs, and each is its own name, an identifier of
`KalamConvention::ALL`: `rahu-kalam-sunrise`, the daylight
from sunrise to sunset at the place, as Drik Panchang computes it, and
`rahu-kalam-fixed`, 06:00 to 18:00 of the local clock, as South Indian
temple tables print it; in any case, and anything else is
`HC_ERR_UNKNOWN`. `locale` is as for `hc_describe_day`, `native`
included. It writes three lines, Rāhu kālam, Yamaganda and Gulika kālam:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | `rahu-kalam`, `yamaganda` or `gulika-kalam` |
| 2 | english name | its name as Drik Panchang prints it in English |
| 3 | name | its name in the locale, else English's: राहुकाल, यमगण्ड and गुलिक काल under `hi`, as Drik Panchang's Hindi day pañcāṅga labels them (`drik-day-panchang-hi-2026`) |
| 4 | locale used | the tag of the data that named it |
| 5 | part | the eighth of the day it takes, 1 to 8 |
| 6 | clock | `universal` for `rahu-kalam-sunrise`, `local` for `rahu-kalam-fixed` |
| 7 | start | on `universal`, POSIX seconds, rounded down; on `local`, seconds after midnight of the day's own clock; empty where the Sun does not rise or set |
| 8 | end | as column 7 |
| 9 | missing | as column 2 of `hc_solar_event`: `sunrise` or `sunset` where the day has none; else empty |
| 10 | missing day | as column 3 of `hc_solar_event` |
| 11 | depression | always empty, for the shape of `hc_solar_event`'s cells |
| 12 | depression arcseconds | always empty |

On Wednesday 1 January 2025 at New Delhi, Rāhu kālam begins within a
minute of Drik Panchang's 12:25 IST, and on the fixed day it is the fifth
eighth, 43 200 to 48 600, 12:00 to 13:30. The day answers for the sky
layer's era, the years −1000 to 3000, and a place off the globe is
`HC_ERR_OUT_OF_RANGE`.

### Choghadiya

`hc_choghadiya(fixed, latitude, longitude, elevation, locale_ptr,
locale_len, buffer, capacity)`, in the same feature, writes the sixteen
choghadiya of a day at a place, from `hc-calendars-indic`'s `choghadiya`:
the daylight from sunrise to sunset and the night from sunset to the next
sunrise each cut into eight, each part of one of seven kinds that the
weekday sets, the night taking the weekday of the sunset that begins it,
in the sequences Drik Panchang prints. The place is as for
`hc_solar_time`, and the locale, which names the kinds, comes after it. It
writes the day's eight lines, then the night's:

| # | Column | Holds |
| --- | --- | --- |
| 1 | half | `day` or `night` |
| 2 | part | 1 to 8 |
| 3 | id | the kind: `udvega`, `chara`, `labha`, `amrita`, `kala`, `shubha` or `roga` |
| 4 | name | its name in the locale, else English's: लाभ, अमृत and the rest under `hi`, as Drik Panchang's Hindi choghadiya page writes them (`drik-choghadiya-hi`), and Drik Panchang's English names otherwise |
| 5 | locale used | the tag of the data that named column 4 |
| 6 | quality | `auspicious`, `neutral` or `inauspicious` |
| 7 | ruler | the planet it is ruled by, `sun` to `saturn` |
| 8 | start | POSIX seconds of Universal Time, rounded down; empty where the Sun does not rise or set |
| 9 | end | as column 8 |
| 10 | missing | as column 2 of `hc_solar_event`: `sunrise` or `sunset` where the half has none; else empty |
| 11 | missing day | as column 3 of `hc_solar_event` |
| 12 | depression | always empty, for the shape of `hc_solar_event`'s cells |
| 13 | depression arcseconds | always empty |

On Wednesday 1 January 2025 at New Delhi the day opens with Labha, ruled
by Mercury, at Drik Panchang's 07:14 IST, its fifth part is Roga from
12:25, and the night opens with Udvega at 17:36, each within a minute.
Where the Sun does not rise or set, a half's eight lines keep their
kinds, which the weekday alone sets, and name the missing event. The day
answers for the sky layer's era, and a place off the globe is
`HC_ERR_OUT_OF_RANGE`.

### Panchak

`hc_panchak(naming_ptr, naming_len, unix_seconds, ayanamsa_ptr,
ayanamsa_len, offset_seconds, locale_ptr, locale_len, buffer, capacity)`
writes the Panchak window in progress at an instant, or the next one
when the Moon is outside it: the Moon's passage from 300° to 360° of
sidereal longitude, from `hc-calendars-indic`'s `panchak`, in the zodiac
of an ayanāṃśa named as for `hc_panchanga_at`. The almanacs name a window
by the weekday it begins on and agree on five weekdays, so each table is
its own name: `panchak-five-kinds`, Prokerala's five kinds, none on a
Wednesday or a Thursday, and `panchak-raj-midweek`, Raj Panchak on those
days too, as India TV has it. The sources do not say whether the weekday
runs from midnight or from sunrise, so the clock is the caller's: the
weekday of the opening is read midnight to midnight on a clock
`offset_seconds` ahead of Universal Time, 19 800 for India's, less than a
day either way. A caller counting from sunrise asks again with the
weekday before for an opening between midnight and sunrise. It writes
one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | within | `1` when the instant is within the window, else `0` |
| 2 | opens | the Moon at 300°, POSIX seconds of Universal Time, rounded down |
| 3 | closes | the Moon at 360° |
| 4 | weekday | the weekday of the opening on the caller's clock, Monday 1 to Sunday 7 |
| 5 | kind | `rog`, `raj`, `agni`, `chor` or `mrityu`; empty on a weekday the table names no kind for |
| 6 | name | its name in the locale, else English's: राज पंचक and the rest under `hi`, as Amar Ujala writes them (`amarujala-raj-panchak-2026`), and the sources' English otherwise; empty, with columns 5 and 7, where column 5 is |
| 7 | locale used | the tag of the data that named column 6 |

Drik Panchang's first window of 2025 for New Delhi opens on Friday
3 January at 10:47 IST and closes on Tuesday 7 January at 17:50, Chor
Panchak under both tables; its last opens on Wednesday 24 December, Raj
Panchak to India TV and no kind in the five-kind table. The instant
answers for the sky layer's era.

### The Kumbh Mela

`hc_kumbh(yoga_ptr, yoga_len, year, ayanamsa_ptr, ayanamsa_len,
jupiter_ptr, jupiter_len, locale_ptr, locale_len, buffer, capacity)`
writes whether a Gregorian year's sky meets one of the seven conditions
under which the Mela Adhikari of the 2013 Kumbh gives the festival at its
four sites, from `hc-calendars-indic`'s `kumbh`: `kumbh-haridwar`,
`kumbh-prayag-vrishabha`, `kumbh-prayag-mesha`, `kumbh-nashik-simha`,
`kumbh-nashik-karka`, `kumbh-ujjain-simha` and `kumbh-ujjain-tula`, each
Jupiter in one sidereal sign and the Sun in another, some at the new
moon. **This layer has no ephemeris of Jupiter** (the `jupiter` layer's
`hc_kumbh_by_sky` has), so Jupiter's sign is
the caller's: `jupiter` is the sidereal sign Jupiter is in at the
occasion's first moment, in the zodiac of the same ayanāṃśa, named by the
lower-case ASCII form of its Sanskrit name — `mesha`, `vrishabha`,
`mithuna`, `karka`, `simha`, `kanya`, `tula`, `vrishchika`, `dhanus`,
`makara`, `kumbha`, `mina` — or empty. The first moment is read because
the Sun's stay can hold a change of Jupiter's sign; it does not depend on
`jupiter`, so a caller can ask once with it empty and again with the sign
its own ephemeris gives there. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the condition |
| 2 | site | `haridwar`, `prayag`, `nashik` or `ujjain` |
| 3 | site name | its name in the locale, else English's: हरिद्वार, प्रयागराज, नासिक and उज्जैन under `hi`, as Webdunia writes them (`webdunia-kumbh-2027`), and the Mela Adhikari's English otherwise |
| 4 | locale used | the tag of the data that named column 3 |
| 5 | river | the river the site stands on, in English as the source gives it: `Ganga and Yamuna` |
| 6 | jupiter | the sign Jupiter must be in, by its identifier: `vrishabha` |
| 7 | jupiter name | its Sanskrit name, `Vṛṣabha` |
| 8 | sun | the sign the Sun must be in, by its identifier: `makara` |
| 9 | sun name | its Sanskrit name, `Makara` |
| 10 | at new moon | `1` when the Moon must be with the Sun at the new moon, else `0` |
| 11 | from | the Sun's entry into its sign that year, or the new moon within its stay, POSIX seconds rounded down; empty for a new-moon condition whose stay holds no new moon |
| 12 | to | the Sun's entry into the next sign, or the new moon again |
| 13 | holds | `1` when there is an occasion and `jupiter` is column 6's sign, `0` when not, empty when `jupiter` is empty |

The dates of each festival are fixed and announced by the state that
holds it; the line says whether the sky meets a site's condition, not
when the bathing days are. For the Maha Kumbh of 2025 at Prayag, the Sun
entered Makara on the morning of 14 January IST and Vṛṣabha held Jupiter,
so `kumbh-prayag-vrishabha` with `vrishabha` holds. The year answers for
−1000 to 3000.

### Pushkaram

`hc_pushkaram(sign_ptr, sign_len, entry_unix_seconds, latitude,
longitude, elevation, meridian_ptr, meridian_len, locale_ptr, locale_len,
buffer, capacity)` writes the twelve days of the *Ādi Pushkaram* of each
river of a sidereal sign, from `hc-calendars-indic`'s `pushkaram`, for
Jupiter's entry into the sign at an instant. With no ephemeris of
Jupiter in this layer (the `jupiter` layer's `hc_pushkaram_by_sky` finds the
entry), **the sign, named as for `hc_kumbh`, and the moment of the entry
are the caller's**; where Jupiter enters, turns back and enters again,
the festival follows the second entry. The first day is the civil day of
the entry at the meridian, read as for `hc_term_in_effect`, or the next
day when the entry falls after that day's sunset at the place: a reading
fitted to the festivals whose dates were read, which no source read
states. One line a river, in `hc-calendars-indic`'s order:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | `pushkaram-ganga` to `pushkaram-pranahita` |
| 2 | name | the river's name in the locale, else English's: nine of the twelve in Devanagari under `hi`, as Amar Ujala writes the rivers of the Pushkar Kumbh (`amarujala-pushkar-kumbh-2025`), and Wikipedia's English otherwise |
| 3 | locale used | the tag of the data that named column 2 |
| 4 | region | where the source keeps the river for the sign, in English, for a sign with two rivers; else empty |
| 5 | sign | the sign, by its identifier: `simha` |
| 6 | sign name | its Sanskrit name, `Siṃha` |
| 7 | first | the first day, a fixed day; empty where the Sun does not set on the day of the entry |
| 8 | last | the twelfth |
| 9 | missing | as column 2 of `hc_solar_event`: `sunset` where the day of the entry has none |
| 10 | missing day | as column 3 of `hc_solar_event` |
| 11 | depression | always empty |
| 12 | depression arcseconds | always empty |

Jupiter entered Siṃha at 07:07 IST on 14 July 2015, by Drik Panchang, and
the Godavari Pushkaram was kept from 14 to 25 July. The instant answers
for the sky layer's era.

## The Hindu lunisolar date

`hc_hindu_lunar_date(sky_ptr, sky_len, fixed, latitude, longitude,
elevation, locale_ptr, locale_len, buffer, capacity)` needs the
`calendars` feature and writes the
amānta lunisolar date of a day read at the sunrise of a place the caller
gives. The registered `hindu-lunar` reads the day at the Central Station
and `hindu-lunar-surya-siddhanta` at Ujjain; a place is a parameter and
not a calendar of its own, and this is where it is given.
`docs/systems/hindu-calendars.md` says how often the place moves a date.
`sky` is the sky the day is read on: an ayanāṃśa as `hc_panchanga_at`
names them, `Lahiri` for the *Rashtriya Panchang*'s, for the true Sun and
Moon in its zodiac; or `surya-siddhanta`, for the *Sūrya Siddhānta*'s Sun,
Moon and sunrise; in any case, and anything else, the empty string
included, is `HC_ERR_UNKNOWN`. The locale names the month and the eras,
as `hc_describe_day` names them for `hindu-lunar`, and fails as
`hc_parse_iso_date` does; it comes after the day, since it only
qualifies the answer. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | saka year | the Śaka year, which turns at Chaitra śukla 1 |
| 2 | vikrama year | the Vikrama year, 135 more |
| 3 | month | 1 for Chaitra through 12 for Phālguna, named for the saṅkrānti it holds |
| 4 | leap month | `1` for the intercalary (adhika) month, which precedes the ordinary one; else `0` |
| 5 | tithi | the tithi in progress at the sunrise, 1 through 30: śukla 1 to 15, then kṛṣṇa 1 to 15 |
| 6 | leap day | `1` for the second day to carry a tithi; else `0` |
| 7 | sunrise | the sunrise the day was read at, as POSIX seconds of Universal Time, rounded down |
| 8 | month name | the month in the locale, as column 8 of `hc_describe_day` names it: `Bhadra`, भाद्रपद under `hi` and `sa`; an intercalary month with the locale's word before it, `Adhika Sravana`; empty where the locale's data has no name |
| 9 | leap month word | the locale's word for an intercalary month, `Adhika` or अधिक, where the month is one; else empty |
| 10 | saka era | the Śaka era's name in the locale, as column 4 of `hc_describe_day` names it for the calendar: `Saka`; शक under `hi`, CLDR's name for the national calendar's Śaka era, which names the same era for every calendar that counts it where the locale has no name of the calendar's own; else English's, `Saka` under `sa`: an era cell, like a formatted date, is a name and never a code |
| 11 | vikrama era | the Vikrama Saṃvat's name, `Vikrama Samvat`, from the vocabulary of the Kārttikādi and Vikrami calendars, which count the same era; else English's, as under `hi` and `sa` |
| 12 | locale used | the tag of the data that named column 8: `en`, `hi`, `sa` |

The locale resolves as it does for `hc_describe_day`: the locale asked for
where its data names the calendar, else English; `native` asks for the
calendar's own languages, Sanskrit then Hindi. The names are the ones
`hc-i18n` already carries; none is added here.

A month begins at the first sunrise after a conjunction, so a place
must see the Sun rise on every day of the year: one beyond 65° of
latitude is `HC_ERR_OUT_OF_RANGE` on either sky. On the true sky a day
outside the Śaka years 1622 through 2221 is `HC_ERR_OUT_OF_RANGE` too,
and a day whose sunrise at the place, or a search that reads it, the
model does not find is `HC_ERR_NO_DATA`. The Siddhānta's reckoning is arithmetic and answers for Kali Yuga 1 to
10 000. 30 March
2025 is Chaitra śukla 1 of Śaka 1947 on the true sky at the Central
Station and on the Siddhānta's at Ujjain. 27 September 2026 at Tokyo is
Śaka 1948, Bhādrapada (`Bhadra`), the sixteenth tithi, on every ayanāṃśa
and on the Siddhānta's sky.

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
| 6 | sign id | its identifier, as for `hc_drekkana_at`: `mina` |
| 7 | sign name | its Sanskrit name, `Mīna` |

`hc_surya_siddhanta_sunrise(fixed, latitude, longitude, buffer, capacity)`
writes one line of one cell, the instant of the Siddhānta's sunrise on the
day at the place as POSIX seconds, rounded down: six in the morning at the
place's meridian corrected by the Siddhānta's own equation of time and
ascensional difference, with no refraction and no height, which is why it
takes none. A place beyond 65° of latitude, where the Siddhānta's Sun does
not rise every day, is `HC_ERR_OUT_OF_RANGE`. At Ujjain on 30 March 2025
it is 01:01 UT, and the Siddhānta's Moon is then 7.58° past its Sun, in
Mīna: the first tithi.

### The northern year's name

`hc_barhaspatya_year(rule_ptr, rule_len, saka, locale_ptr, locale_len,
buffer, capacity)` writes the name of the northern sixty-year cycle, the
Bārhaspatya *saṃvatsara*, that a rule of Sewell and Dikshit's Art. 59
couples with the year that begins in an *expired* Śaka year — the Śaka
year the *Rashtriya Panchang* prints — the name current at the apparent
Meṣa saṅkrānti of its solar year, from `hc-calendars-indic`'s
`barhaspatya`. The two *Sūrya Siddhānta* rules give names one apart from
2018 to 2027, so each is its own name: `surya-siddhanta-bija`, the *Sūrya Siddhānta* with the *bīja*,
by which the pūrṇimānta calendar names its years and Drik Panchang heads
Vikrama 2081 to 2083 Pingala, Kalayukta and Siddharthi; `surya-siddhanta`,
the same without it, by which some of the Hindi press's announcements of
2021–26 name them Kalayukta, Siddharthi and Raudra, where others print the
first rule's names; and `arya-siddhanta`, the first *Ārya Siddhānta*.
It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | position | the name's position in the cycle, 1 for Prabhava through 60 for Kṣaya |
| 2 | name | its name in the locale, as `hc_day_extras` names the pūrṇimānta calendar's `barhaspatya-samvatsara` |
| 3 | expunged | the position of the name the rule expunges in that solar year, the one that begins and ends in it; empty in a year that expunges none |
| 4 | expunged name | its name, as column 2 |
| 5 | locale used | the tag of the locale data that answered, as column 7 of `hc_day_extras` gives it; the names are English's where that locale names no year, so `hc_barhaspatya_year("surya-siddhanta-bija", 1946, "hi")` writes `hi` beside `Pingala` |

The Śaka year answers for −3178 to 6821, the expired years of Kali Yuga 1
to 10 000. By the rule with the *bīja* Śaka 1946 is Pingala, 51, and
Śaka 1949 expunges Durmati, 55.

`hc_barhaspatya_year_at(rule_ptr, rule_len, unix_seconds, locale_ptr,
locale_len, buffer, capacity)` writes the name in progress at an instant
by the same rules: the name current at the last apparent Meṣa saṅkrānti
of the *Sūrya Siddhānta* runs to the day the rule ends it, and the names
after it from then on, which Sewell and Dikshit give as correct within
two ghaṭikās where the saṅkrānti is known. It writes one line of three
cells, columns 1, 2 and 5 above, and four more: the saṃvatsara of the
twelve-year cycle that Sewell and Dikshit's Table XII couples with the
name, by its position, 1 for Chaitra to 12 for Phālguna, and its name as
the table spells it, `Asvina` for Pingala; and the sign of Jupiter's mean
longitude while the name is current, by its identifier and its Sanskrit
name, `mesha` and `Meṣa` for Pingala. Drik Panchang ends Pingala at 14:14 IST
on 29 April 2024, and the rule with the *bīja* about two hours later.
The instant answers for the days of Kali Yuga 1 to 10 000, as for
`hc_surya_siddhanta_at`.

## The young crescent

`hc_crescent_visible(criterion_ptr, criterion_len, fixed, latitude,
longitude, elevation, buffer, capacity)` needs the `calendars` feature and
says whether the young crescent should have been visible, in a clear sky,
on the evening that begins a day — the evening of the day before, since
such a day begins at sunset — from a place, by a named criterion of
`hc-calendars-lunar`: `shaukat`, the arc of light and the altitude at a
solar depression of 4.5°, as the observational calendars `islamic-rgsa`
and `hebrew-observational` judge; `yallop`, B. D. Yallop's *q*-test at
Bruin's best time; `saudi-rule`, the Moon past conjunction at sunset
and setting after the Sun; `odeh`, M. Sh. Odeh's *V* at Bruin's best time;
`istanbul-2016` and `khgt`, an elongation of 8° and an altitude of 5° at
sunset, the altitude topocentric for the first and geocentric for the
second; or `mabims-2021-topocentric` and
`mabims-2021-geocentric-elongation`, the two readings of Neo-MABIMS's 3°
and 6.4° at sunset; in any case, and anything else is
`HC_ERR_UNKNOWN`. It is a forecast of an observation, not a record of
one. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | visible | `1` if the crescent passes the criterion; else `0` |
| 2 | evaluated at | the moment the criterion judges the evening at, as POSIX seconds, rounded down: the Sun at 4.5° down for `shaukat`, Bruin's best time for `yallop` and `odeh`, sunset for the others; empty where there is none — the Sun does not set, twilight does not end, or at Bruin's best time the Moon sets first — and then column 1 is `0` |
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
`HC_ERR_OUT_OF_RANGE`. That is the Charter's definition of 2004; before it,
an Olympiad ran from the opening of one Games to the opening of the next,
and `hc_ioc_olympiad_on(fixed)` answers a day by the definition in force
on it, from `hc-calendars-regional`'s `olympiad::ioc_olympiad_before_2004`
and Olympedia's opening dates, and from 1 September 2004 by the year. A
day from 10 June to 21 November 1956 is the XV Olympiad if the Melbourne
Games opened the XVI and the XVI if the Stockholm equestrian Games did, on
which no source read rules, and is `HC_ERR_NO_DATA`.

### Babylonian regnal years

`hc_babylonian_regnal_year(seleucid_year, buffer, capacity)`, in the
`calendars` feature, writes the king and the regnal year labelling a
Seleucid year, as van Gent's converter of Parker and Dubberstein's table
labels it, from `hc-calendars-lunar`'s `babylonian::regnal_year`: SE −314
is 1 Interregnum, the accession year of Nabopolassar, SE −313 is 1
Nabopolassar and SE 1 is 1 Seleucus I Nicator. A year outside SE −314 to
160, 152/151 BCE, is `HC_ERR_OUT_OF_RANGE`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | king | the king, in English, or `Interregnum` |
| 2 | regnal year | the year of the reign |

### Equinox new-year margins

`hc_equinox_new_year_margin(calendar_ptr, calendar_len, year, buffer,
capacity)`, in the same feature, writes how far the equinox that begins a
year fell from the moment of the day that decides the new year, in
minutes, from each calendar's `new_year_margin` in `hc-calendars-equinox`:
`persian`, the nearer noon of Iran Standard Time; `persian-apparent-noon`
and `jalali`, the nearer apparent noon at Tehran and at Isfahan;
`bahai-astronomical`, the Tehran sunset; and `french-republican-equinox`,
the nearer Paris apparent midnight. A margin within the few minutes the
astronomy is good to marks a year the calendar decides by a model; 183 BE,
2026, is under a fifth of a minute on the wrong side of the sunset.

| # | Column | Holds |
| --- | --- | --- |
| 1 | minutes | the margin in minutes, positive when the equinox fell before the moment; always positive for `french-republican-equinox`, whose size alone matters |
| 2 | calendar | the calendar's identifier |

### Shmuel's tekufot

`hc_shmuel_tekufah(hebrew_year, tekufah_ptr, tekufah_len, buffer,
capacity)`, in the same feature, writes a *tekufah* of Shmuel's reckoning,
from `hc-calendars-lunar`'s `hebrew::shmuel_tekufah_day`: a year of 365¼
days in four seasons of 91 days and 7½ hours, as Maimonides gives it, in
Jerusalem mean time, the first *tekufat Nisan* at the beginning of the
fourth day. `tekufah` is `tishrei`, `tevet`, `nisan` or `tammuz`; the
year's Tishrei and Tevet come before its Nisan, so Tishrei 5786 is 7
October 2025. A year outside 1 to 9999 is `HC_ERR_OUT_OF_RANGE`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | fixed | the fixed day whose Hebrew day it falls in: the civil day, or the next from the reckoning's nightfall |
| 2 | minutes | the minutes of Jerusalem mean time since the midnight of the moment's civil day, column 5 |
| 3 | tekufah | the *tekufah*'s identifier |
| 4 | after nightfall | `1` when the moment falls after the reckoning's nightfall and before midnight, so that its Hebrew day is the next civil day's, else `0` |
| 5 | civil | the fixed day of Jerusalem mean time the moment falls on |

The reckoning's day turns at its own nightfall, not at the Sun's setting:
Maimonides counts "day and night" as twenty-four hours, "twelve [hours]
of daylight and twelve [hours] of night" (*Hilkhot Kiddush HaChodesh*
6:2, `maimonides-kiddush-hachodesh`), and puts every *tekufat Nisan* at
one of four hours, nightfall, midnight, daybreak or noon (9:4), so its
nightfall is six equal hours before midnight, 18:00 of Jerusalem mean
time. The anchor, *tekufat Nisan* 5769 at that nightfall on Tuesday
7 April 2009, is "the beginning of the night of the fourth day"
(*Hilkhot Berakhot* 10:18), Wednesday's Hebrew day, though the Sun set at
Jerusalem later that evening: column 1 is 8 April 2009, column 2 1080,
column 4 `1` and column 5 7 April. 4930's Nisan, "on the night of the fifth day at midnight"
(9:5), is minute 0 of Thursday, before no nightfall of its own day, and
column 4 is `0`.

### Named days

`hc_day_name(calendar_ptr, calendar_len, naming_ptr, naming_len, fixed,
buffer, capacity)`, in the same feature, writes a fixed day's name in a
calendar whose days are named, by one of its namings: the French
Republican calendars' `fr-fabre-1793`, the table annexed to Fabre
d'Églantine's report of 3 brumaire an II, `fr`, the list the calendar came
to use, and `en`, English Wikipedia's gloss of it, from
`hc-calendars-solar`'s `french_republican_days`; the Armenian calendar's
`hy`, its thirty day names and five epagomenal ones in Armenian, and
`hy-Latn`, the thirty in English Wikipedia's romanisation, from
`hc-calendars-solar`'s `armenian`. The first day of An II, 22 September
1793, is Raisin in `fr`. A calendar whose days are not named, or a naming
it does not have, is `HC_ERR_UNKNOWN`; an epagomenal day in `hy-Latn`,
which has none, is `HC_ERR_NO_DATA`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | name | the day's name: `Raisin`, Արեգ |
| 2 | naming | the naming's identifier |
| 3 | naming name | the naming's name in English |
| 4 | authority | where the names come from |

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
| 4 | chinese names | the Chinese names the sources read give that kind of year, separated by `;`: 無春年, 寡婦年 and 盲年, with their simplified forms, for a widow year (Wikipedia, 「立春」), 雙春兼閏月 (the Hong Kong Observatory) and 双春年 for a double-bright one; empty for `blind` and `bright`, which no source read names. The Chinese 盲年 is the code's `widow`, not its `blind` |
| 5 | name scripts | the script of each name, in the same order: `zh-Hant` or `zh-Hans` |
| 6 | name regions | where each name is used, in the same order, `north` or `south`, or empty where the source does not say |

`hc_chinese_age(convention_ptr, convention_len, birth_fixed, on_fixed)`,
in the same feature, returns a person's age on `on_fixed`, born on
`birth_fixed`, by one of four counts, each its own name: `chinese-age`,
one at birth and one more at each Chinese New Year, as
`hc_chinese_reckoned_age`; `lichun-age`, one more at each 立春, which 果壳
gives as the custom of some places; `new-year-day-age`, one at birth and
one more each 1 January, the Korean 세는 나이; and `year-age`, nothing at
birth and one more each 1 January, the Korean 연 나이. A child born on 1
June 2009 is one until 3 February 2010 and two from 立春 on the 4th by
`lichun-age`. A day before the birth is `HC_ERR_NO_DATA`.

### The Qing almanac's solar terms

`hc_chinese_almanac_solar_terms(year, buffer, capacity)` writes the days of
the twenty-four solar terms the Qing almanac printed in a Gregorian year
of 1645–1733, from Liu's transcription, whose term days often stand a day
from the modern ones; the Dàtǒng years 1667–1669 and every other year are
`HC_ERR_NO_DATA`. One line per term, 小寒 first:

| # | Column | Holds |
| --- | --- | --- |
| 1 | position | 1 for 小寒 to 24 for 冬至 |
| 2 | name | the term's name in traditional Chinese |
| 3 | fixed | the fixed day the almanac printed it on |

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
`1` for a substitute day, the date it stands in for, the region, the group,
the identifier and the source — and, called with a null buffer, returns the
length the text needs so the caller can allocate exactly. The region is the subdivision whose own entry the
line is: asked for `JP` in the region `JP-13`, the lines are Japan's
nationwide days and Tokyo's 都民の日, and only 都民の日 carries `JP-13`. A
region matches in either case, and one the table's sources were not read
for gives the nationwide days and a gap line for its own. The group is the group of people whose own entry the
line is, in the same way: asked for `CN` for the group `women`, the lines
are China's days for everyone and the half day of 8 March, and only that
line carries `women`. The identifier is the holiday's stable identifier
within its table, lower-case ASCII and hyphenated — `new-years-day` — and
the source is the instrument the rule cites, `A/RES/73/161`, or empty
where the table's own sources speak for it; the two come last, so that a
parser that reads the first nine cells reads what it always read. The same
identifier ends the entry in `hc_holidays_on` and `hc_holidays_on_in`, and
is the fourth cell of `hc_common_worship_on`, so lines are joined on it
and not on the name: names differ in spelling between the states of one
country, and an identifier folds case, punctuation, apostrophes and
diacritics (`Mothers' Day` and `Mother's Day` are `mothers-day`), which a
rule overrides only where two days of one table would fold together. A gap
carries its rule's identifier and source, and `unread-subdivision` and no
source for a subdivision not read, and `unread-weekend` and no source for a
year whose weekend law in the region was not read. The kind filter, the fourth argument,
keeps the entries of the kinds it lists, `;`-separated in any case —
`public;bank` — and the gaps of the rules of those kinds, and a
subdivision not read, and a weekend not read, is a gap whatever it lists;
empty, it is every kind.
It is for the tables whose lists are long: the United States' states carry
about 1,300 observances beside a few public days. A kind is `public`, `bank`, `religious`,
`observance`, `school`, `workday`, `government` or `half-day`. After the
entries come the year's gaps, as `hc_holidays_on` writes them: an empty
date and confidence, the kind `gap`, and the region and group whose own
gap it is — a holiday whose calendar's range ended, whose announcement was
not read, whose sources were not read for the year, or the days of a
subdivision not read. `CN` for `women` in 1998, before the statute's text
read, is everyone's days and a gap line for the half day of 8 March.

The string arguments fail the same way in both, and the same way as in the C
library: a null pointer with a non-zero length is `HC_ERR_NULL_POINTER`, a
code, region, group or kind that is not UTF-8 is `HC_ERR_NOT_UTF8`, and a
code that names no table, a group that names no group of
`hc_holiday_groups`, or a kind with a word that names no kind, is
`HC_ERR_UNKNOWN`. An empty region is no region, and
an empty group is everyone, as is a group the table gives no day to
alone.

```js
hc.holidaysInYear("JP", "", 2026);
// [{ date: "2026-01-01", name: "New Year's Day", localName: "元日", kind: "public",
//    confidence: "exact", substitute: false, observedFor: null, region: null, group: null,
//    id: "new-years-day", source: null }, ...]
hc.holidaysInYear("US", "US-TX", 2026, "", "public;bank").every((day) => day.kind !== "observance");  // true
hc.holidayIsDayOff("XNYS", "", hc.gregorianToFixed(2026, 4, 3));   // true: Good Friday
hc.holidaysInYear("JP", "JP-13", 2026).find((day) => day.region);
// { date: "2026-10-01", name: "Tokyo Citizens' Day", localName: "都民の日",
//   kind: "school", confidence: "exact", substitute: false, observedFor: null, region: "JP-13",
//   group: null }
hc.holidaysInYear("CN", "", 2026, "women").find((day) => day.group);
// { date: "2026-03-08", name: "Women's Day", localName: "妇女节", kind: "half-day",
//   confidence: "exact", substitute: false, observedFor: null, region: null, group: "women" }
hc.holidayIsDayOff("CN", "", hc.gregorianToFixed(2026, 6, 1), "children");  // true
```

### Subdivisions and groups

A subdivision is not a table. Its days are rules of its country's table,
scoped to its ISO 3166-2 code, and it is asked for by the `region`
argument: Tokyo is `JP` in the region `JP-13`, Scotland `GB` in `GB-SCT`,
the District of Columbia `US` in `US-DC`. The same holds for every
country. So:

- `hc_holiday_codes` lists tables only, and no subdivision code is one of
  its lines. `hc_holiday_tables` names a country's subdivisions in its
  row, in column 9 (`regions`): `JP`'s row lists `JP-13` among the
  prefectures that have days of their own. The `subdivision` kind of
  column 2 is for a table whose code is itself an ISO 3166-2 code, and no
  table is one.
- `hc_holidays_in_year` and `hc_holiday_is_day_off` asked for `JP` with
  an empty region answer for the nationwide days alone, not the union of
  every prefecture's. Asked for `JP` in `JP-13`, they answer for Tokyo:
  the nationwide days and Tokyo's own, and a year's line of Tokyo's own
  day carries `JP-13` in column 8 of its 11, the group being the ninth,
  and the identifier and the source the last two.
- `hc_holidays_on` takes no region. It writes every table's nationwide
  lines, and after them each subdivision's own lines, still under the
  country's table code — 都民の日 is a line of `JP`, not of a table
  `JP-13` — with the subdivision's code in column 10. A page that wants
  one country's nationwide days keeps the lines whose column 10 is
  empty; one that wants Tokyo keeps those and the ones that say `JP-13`.

A municipality is a region too, within its subdivision: its code is the
subdivision's, a hyphen and its code within the subdivision in the
country's own standard (ADR 0014), so 川崎市 is `JP-14-130`, from its
JIS X 0402 code 14130. Asked for it, a table answers for the nationwide
days, Kanagawa's and Kawasaki's own; `hc_holiday_tables` lists it among
the regions; `hc_holidays_on` writes its own lines with its code, less
what its prefecture has; and a year's line carries the widest region
whose own entry it is, the prefecture's code for the prefecture's day. A
city no rule names and no list gives keeps its prefecture's days, and its
own are a gap.

A group of people is asked for in the same way, by the `group` argument,
and is independent of the region. Some statutes give a day to a group
alone: China's Article 3 gives women half of 8 March, youth of fourteen and
over half of 4 May, children under fourteen 1 June and active servicemen
half of 1 August; Taiwan's Article 6 leaves Police Day, Fire Fighters' Day,
Armed Forces Day and Coast Guard Day to each service's authority. A group
is named by an identifier of `hc_holiday::group` — `women`, `youth`,
`children`, `military`, `police`, `firefighters`, `coast-guard`,
`indigenous-peoples` — matched in either case. So:

- `hc_holiday_tables` lists the groups a table's rules name in column 10
  (`groups`), `children;military;women;youth` for `CN`, and their names in
  the locale in column 11, 少年儿童;现役军人;妇女;青年 under `zh-CN`,
  English where `hc-i18n` names a group in no other language.
- `hc_holidays_in_year` and `hc_holiday_is_day_off` asked for no group
  answer for everyone's days alone, not the union of every group's. Asked
  for `CN` and `women`, they answer for women: everyone's days and the
  half day of 8 March, whose year's line carries `women` in its last
  column. A group and a region together answer for the group in the
  subdivision.
- `hc_holidays_on` writes, after a table's nationwide and subdivisions'
  lines, each group's own lines, with the group in column 11, and last the
  lines of a rule scoped to both a subdivision and a group, with both.
- A half day is the kind `half-day`: work stops for part of the day, and
  the day is still a business day, as an exchange's early close is. China's
  Children's Day is `public` for children, a day off for them alone. A
  Taiwanese service's day is a `gap` for its group in every year from
  2025, the authority's rule that gives it not having been read.

### One day, every table

`hc_holidays_on(fixed, buffer, capacity)` writes every entry on one day
across every table `hc_holiday_codes` lists, in that order, each evaluated
nationwide, then in each subdivision its rules are scoped to, the
regions of column 9 of `hc_holiday_tables`, in code order, then for each
group of its column 10, in identifier order, and last for each subdivision
and group a rule names together. One line per (table, scope, entry): a
subdivision's lines are the entries it has that the nationwide calendar
does not, so a nationwide holiday is written once and Tokyo's 都民の日 on
1 October is a line of `JP` with `JP-13` in column 10; a group's are the
entries it has that the calendar for everyone does not, so China's half day
of 8 March is a line of `CN` with `women` in column 11.

| # | Column | Holds |
| --- | --- | --- |
| 1 | table | the table's identifier, `JP`, `XNYS`, `christian-western`, `un-days` |
| 2 | table name | its English name |
| 3 | name | the holiday's English name |
| 4 | local name | its name in the local language, or empty |
| 5 | kind | `public`, `bank`, `religious`, `observance`, `school`, `workday`, `government`, `half-day`, or `gap` |
| 6 | confidence | `exact` or `approximate`; empty for a gap |
| 7 | source | the instrument the rule cites, `A/RES/73/161`, or empty |
| 8 | substitute | `1` for a weekend substitute, else `0` |
| 9 | observed for | the fixed day a substitute stands in for, or empty |
| 10 | region | the ISO 3166-2 code of the subdivision whose own entry this is, `JP-13`, or empty for a nationwide one |
| 11 | group | the identifier of the group of people whose own entry this is, `women`, or empty for one everyone has |
| 12 | id | the holiday's stable identifier within its table, `new-years-day`, which `hc_holidays_in_year` and `hc_common_worship_on` write for the same entry; a gap's is its rule's, and `unread-subdivision` for a subdivision not read |

A `gap` line is a holiday the table could not place in the day's year — its
calendar's range ended, or the year's announcement has not been read — with
columns 6 and 7 empty. It is reported rather than left out so a page can say
"no announcement read for this year" instead of showing nothing; policy §4.
A day with no Gregorian year is `HC_ERR_OUT_OF_RANGE`.

The call evaluates each table for the one day
(`HolidayCalendar::for_day_with`), which answers exactly what the whole
year would, through one `EvaluationContext` shared by every table and
inside one `hc_core::memo::scope`, so the astronomy the tables have in
common is done once, and only for the months around the day. The context
keeps the sunrises the Hindu festivals are read at and the dates the rules
convert; the scope keeps the winter solstices and new moons that every
conversion of a lunisolar date searches for, which the Chinese, Korean and
Vietnamese tables and the functions that date the Japanese 旧暦 days would
otherwise search for again for every date they convert. Measured on
2026-09-29 in the `release-compact` profile with
[`examples/holidays_on_timing.rs`](../hyper-calendar/examples/holidays_on_timing.rs),
one 2026 day across all 311 tables and their subdivisions takes about
31 ms natively on 1 January, the costliest, and 22 ms on 25 September,
against 0.21 s for every table's whole year; in WebAssembly under Node 22,
measured with `scripts/wasm-calendar-timing.mjs`, 63 ms and 48 ms. The
subdivisions' lines are a small part of that: measured on 2026-09-28, when
there were 298 tables, against the same build without them, they took
about 3 ms and 2 ms of the WebAssembly times and under 1 % of the native
ones.

### The tables

`hc_holiday_tables(locale_ptr, locale_len, buffer, capacity)` describes
every table, one line each in the order `hc_holiday_codes` lists them, so
that a menu can show a name rather than a code. Everything in a line is the
table's own data: the kind is the list the table is in, and a table in the
countries' list whose code is an ISO 3166-2 code would be a `subdivision`,
though none is yet — a subdivision's days are rules of its country's
table, asked for by `region`. A country's table is named in the locale by
CLDR 48's territory names at the `approved` and `contributed` levels,
from `hc-i18n`'s `place_names` (its `territories` feature), which walks the
locale's chain, then the fallbacks CLDR's language matching gives it:
日本 under `ja`, Deutschland under `de` and `de-AT`, with the tag
of the data that answered, `ja` or `de`, in column 5. English is CLDR's
English too, so under `en` column 3 is `Hong Kong SAR China` where the
table's own name in column 4 is `Hong Kong`. A country the locale has no
release-level CLDR name for, nor its fallbacks (Kabyle's Hong Kong, every
country in Coptic; Tibetan's France is Simplified Chinese's 法国, `zh-Hans`
in column 5), and every country under `native`, which names no one
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
from the locale and the CLDR parents that answered column 5: `Hong Kong` for `HK` under
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
| 9 | regions | the ISO 3166-2 codes of the subdivisions the table's rules are scoped to, `;`-separated in code order, the regions `hc_holidays_in_year` and `hc_holiday_is_day_off` answer for beyond the nationwide days; empty for a table with none |
| 10 | groups | the identifiers of the groups of people the table's rules give days to alone, `;`-separated in identifier order, the groups `hc_holidays_in_year` and `hc_holiday_is_day_off` answer for beyond everyone's days; empty for a table with none |
| 11 | group names | those groups' names in the locale, in the same order: `hc-i18n`'s where it names the group in a language of the locale's chain, 妇女 for `women` under `zh-CN`, else the English name |
| 12 | region groups | the pairs of a subdivision and a group that a rule is scoped to both of, `region:group`, `;`-separated in code and then identifier order, the scopes whose own days neither the region alone nor the group alone has; empty for a table with none, which is every table today |
| 13 | read subdivisions | the ISO 3166-2 codes of the subdivisions the table's sources were read for, `;`-separated in code order: column 9's, and those read and found to keep no day of their own; a region outside the list keeps the nationwide days and has a gap for its own; empty for a table with no subdivisions and for a country whose subdivisions were not read |
| 14 | weekend | the table's weekend laws, `;`-separated in the table's order, each four fields separated by `/`: the weekend days as ISO 8601 weekday numbers joined by `+` (Monday 1 to Sunday 7, `5+6` for Friday and Saturday) or `unread` for years whose law was not read, the first day in force and the last, `YYYY-MM-DD`, each empty for none, and the ISO 3166-2 codes of the regions it is the weekend of, joined by `,`, empty for the whole table |

Column 14 is the weekend a caller's `region` selects
([ADR 0015](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/adr/0015-a-region-may-keep-a-weekend-of-its-own.md)).
Where several entries cover a day the one for the nearest region wins, and
a region with none of its own has the entry with no regions; a table that
writes none keeps Saturday and Sunday. `MY` writes `6+7///` for the
country, `5+6/2013-11-25//MY-02,MY-03,MY-11` for the three states that keep
Friday and Saturday and `5+6/2014-01-01/2024-12-31/MY-01` for Johor's
decade, with `unread` entries for the years before `1994-12-31` (Johor)
and `2013-11-24` (the three and Perlis) whose law was not read; `AE` ends
with `5+6+7/2022-01-01//AE-SH`, the Government of Sharjah's. The weekend
changes what `hc_holiday_add_business_days`, `hc_holiday_business_days_between`
and `hc_holidays_in_year` answer for such a region, and an `unread` day
refuses the arithmetic with `HC_ERR_OUT_OF_RANGE` and is a gap line,
named `The weekend`, of the year. The JavaScript binding reads the column
into `weekend`, a list of `{ days, first, last, regions }` with `days` null
for `unread`.

### Business days

`hc_holiday_add_business_days(code_ptr, code_len, region_ptr, region_len,
group_ptr, group_len, fixed, count)` returns a fixed day moved by `count`
business days of a table, and `hc_holiday_business_days_between(code_ptr,
code_len, region_ptr, region_len, group_ptr, group_len, from_fixed,
to_fixed)` the number of business days from one day up to but not
including another, from `hc-holiday`'s `HolidayCalendar`. `region` and
`group` scope the table as for `hc_holiday_is_day_off`, so that a group's
own days, China's half day for women among them, are days off for that
group alone (ADR 0012). A business day is neither a holiday of the scope
nor a weekend day the table does not make a working day, as China's
调休上班 are. A positive count moves forward and a negative one back; the
starting day is never counted, and 0 returns it. The interval of the
second is half-open, so that two counts add. In Japan's Golden Week of
2026, five business days after Tuesday 28 April is Monday 11 May. The weekend
is the `region`'s where it keeps one of its own (column 14 of
`hc_holiday_tables`): one business day after Thursday 5 March 2026 is Friday
6 March in `MY` and Sunday 8 March in `MY-02`, Kedah, whose weekend is
Friday and Saturday. A count past 36 500, or two days more than a hundred
years apart, or a walk that reaches a day whose weekend law the region's
sources did not read, is `HC_ERR_OUT_OF_RANGE`.

### Groups and names in a locale

`hc_holiday_groups(locale_ptr, locale_len, buffer, capacity)` writes every
group of people a holiday may be given to alone, in `hc-holiday`'s order,
named in the locale where an instrument in the language names it, from
`hc-i18n`'s `holiday_groups`:

| # | Column | Holds |
| --- | --- | --- |
| 1 | group | the identifier `hc_holidays_on`'s group column writes: `women` |
| 2 | name | its name in the locale, 妇女 under `zh-Hans`; empty where none names it |
| 3 | locale used | the tag that named it |
| 4 | english name | the English name |

`hc_holidays_on_in(fixed, locale_ptr, locale_len, buffer, capacity)` writes
`hc_holidays_on`'s lines, each with two more cells: what the locale calls
that day of that table where a source in the language names it by the
holiday's identifier, from `hc-i18n`'s `holiday_names` — the Bohairic
Coptic names of seven Coptic Orthodox feasts under `cop`, ⲡⲓⲭⲗⲟⲙ ⲛ̀ⲧⲉ ϯⲣⲟⲙⲡⲓ
for `nayrouz-new-year` — else empty, and the tag that named it. The
identifier, which `hc_holidays_on` ends with, comes after the two, so that
the first eleven cells and the two names keep the places they had: the line
is `hc_holidays_on`'s eleven columns, the name, the tag, and the identifier.

### The liturgical year

`hc_lectionary(fixed, buffer, capacity)` writes the lectionary cycles a
day falls in, from `hc-holiday`'s `lectionary`: the rules, not the
readings. Every cycle turns at the First Sunday of Advent, and a liturgical
year is named by the civil year of its Easter, so the year that began on
30 November 2025 is 2026, Year A of the Sunday cycle and Year II of the
weekdays, and Christ the King, 22 November 2026, is Proper 29 and the
34th Sunday in Ordinary Time. A day before 1 January 1970, when
*Mysterii Paschalis* put the Roman calendar of 1969 and its cycles into
effect, or after the liturgical year 4099, is `HC_ERR_OUT_OF_RANGE`; the
RCL's Proper is empty before Advent 1992.

| # | Column | Holds |
| --- | --- | --- |
| 1 | liturgical year | the civil year of the liturgical year's Easter |
| 2 | sunday cycle | `A`, `B` or `C`, the Roman Lectionary's and the Revised Common Lectionary's |
| 3 | weekday cycle | `I` or `II`, the Roman weekday cycle of Ordinary Time |
| 4 | proper | the RCL's numbered Proper, 3 to 29, for a Sunday after Trinity Sunday; else empty |
| 5 | sunday in ordinary time | the Roman number of a Sunday in Ordinary Time, 2 (14–20 January) to 34 (Christ the King, 20–26 November); else empty |
| 6 | week of ordinary time | the week of Ordinary Time, 1 to 34, on the universal calendar, whose Baptism of the Lord is the Sunday after 6 January; else empty |
| 7 | week, epiphany on a sunday | the same on a calendar that keeps the Epiphany on the Sunday between 2 and 8 January, whose Baptism is on the Monday after an Epiphany of 7 or 8 January; else empty |

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
| 4 | id | the celebration's identifier, `christmas-day`, the last column of its entry in `hc_holidays_on`: the two are joined on it and not on the title |

St George's Day was kept on Monday 28 April 2025, Easter being 20 April,
as a Festival. A day with no Gregorian year is `HC_ERR_OUT_OF_RANGE`.

### The 1960 office

`hc_roman_1960_office_on(fixed, buffer, capacity)`, in the same feature,
writes what the Roman calendar of the 1960 rubrics does on a day, from
`hc-holiday`'s `roman_calendar_1960`: the office kept, by the table of
precedence of no. 91, then each commemoration made (nos. 106–114), each
feast of the I class impeded and transferred (nos. 95–99), and each day
the calendar lists here that is neither kept, commemorated nor
transferred. `hc_holidays_on` gives the days of the `roman-1960` table
without their precedence; this gives the ordo of one day.
`docs/systems/roman-calendar-1960.md` says what is and is not applied.

| # | Column | Holds |
| --- | --- | --- |
| 1 | role | `office`, `commemoration`, `transferred` or `omitted` |
| 2 | title | the title as the translation prints it |
| 3 | class | `first`, `second`, `third`, `fourth` or `commemoration` |
| 4 | class name | the class's English name: `I class` |
| 5 | transferred from | on the office's line, the fixed day a feast of the I class transferred here was impeded on; else empty |

25 March 1962 was the Third Sunday of Lent, which kept the office; the
Annunciation, a feast of the I class, was transferred, and on Monday 26
March its line names the Sunday as the day it came from. A day outside the
years 1583 to 4099 is `HC_ERR_OUT_OF_RANGE`.

### The Orthodox fasts

`hc_orthodox_fast_on(reckoning_ptr, reckoning_len, fixed, buffer,
capacity)`, in the same feature, says what a day is in a church's fasting
scheme, from `hc-holiday`'s `orthodox_fasts`: its fasting seasons, its
one-day fasts, its fast-free days, and the Wednesday and Friday fasts
outside them. The Eastern Orthodox scheme is the OCA's outline, its fixed
dates read in the Julian calendar, `orthodox-fasts`, or in the Revised
Julian, `orthodox-fasts-revised-julian`, both keeping Pascha by the Julian
computus. The Armenian scheme is `armenian-fasts`, on the Gregorian
calendar and computus Etchmiadzin keeps, or `armenian-fasts-jerusalem`,
on the Julian, or `armenian-fasts-fifty-days`, with the weekly fasts
lifted to Pentecost as arak29's page has them; the Coptic and Ethiopian are `coptic-fasts` and
`ethiopian-fasts`, dated in their own calendars with the Julian Pascha
(`hc-holiday`'s `oriental_fasts`). In any case; anything else is
`HC_ERR_UNKNOWN`. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | fast day | `1` if the day is a fast day, else `0` |
| 2 | status | `period` for a day in a period, `weekly-fast` for a Wednesday or Friday in none, else `none` |
| 3 | period | the period's identifier, such as `great-lent` or `bright-week`; else empty |
| 4 | period name | its English name, as the OCA's outline gives it for the Eastern Orthodox; else empty |
| 5 | kind | `fast`, `fast-free` or `meat-excluded`, the Meatfast's, when no day is a fast day and none allows meat; else empty |
| 6 | abstinence | what the day abstains from: `nothing`, `meat` or `fast`; a day of the Meatfast is `0` in column 1 and `meat` here |

`hc_orthodox_fast_seasons(reckoning_ptr, reckoning_len, year, buffer,
capacity)` writes the periods, twelve for the Eastern Orthodox, that begin in a year of the
reckoning's calendar, one line each in the order a day is tested against
them, fast-free weeks first: the identifier, the English name and the
kind of columns 3 to 5 above, then the first and last days as fixed days,
both included, and both empty in a year the period does not happen —
which only the Apostles' Fast does, on the Revised Julian reckoning when
Pascha is late, as in 2024. Christmastide ends in the next year. Great
Lent began on 3 March 2025, Wednesday 18 February 2026, in Cheesefare
week, is `0`, `meat-excluded` and `meat`, and the Apostles' Fast of 2025 ran from
16 June to 11 July on the Julian reckoning and to 28 June on the Revised
Julian. On `coptic-fasts` the Apostles' Fast of 2026 runs from 1 June to
11 July, and on `armenian-fasts` Great Lent of 2026 from 16 February to
4 April. A year outside 326 to 4099, or 1583 to 4099 on the three
Gregorian reckonings, is `HC_ERR_OUT_OF_RANGE`.

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

`hc_plum_rains(rule_ptr, rule_len, year, meridian_ptr, meridian_len)`
answers the fixed day of 入梅 or 出梅, the plum rains' beginning or end in
the Chinese almanac, counted in day signs from a solar term at a meridian
read as above, the term's own day counted, from `hc-seasons`'s `meiyu`.
The regions differ on the stem, so each rule is its own name, as 中国气象局
gives them: `ru-mei-bing`, 入梅 on the first 丙 day from 芒种, South
China's; `ru-mei-ren`, the first 壬 day, Central China's; and
`chu-mei-wei`, 出梅 on the first 未 day from 小暑, South China's. The
published days are China's, `china`; none of this is the Japanese 入梅 at
80° of solar longitude. It answers for the years −1000 to 3000. In 2026 入梅
fell on 11 June and 出梅 on 8 July.

### The almanac's cycles

`hc_almanac_cycles(fixed, meridian_ptr, meridian_len, buffer, capacity)`
needs the `calendars` feature and writes three cycles of `hc-almanac` for
a day: 恵方, the year's lucky direction, by the heavenly stem of the
day's Gregorian year, as 恵方参り on New Year's Day and the 恵方巻 of 節分
take it; 三元九運, the period of twenty years in force from 上元一運 in
1864, its year turning at 立春 at `meridian`, which is read as above; and
손 없는 날, the days numbered 9, 10, 19, 20, 29 and 30 in the Korean lunar
calendar, `dangi`. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | eho | 恵方's point of the twenty-four: 甲, 庚, 丙 or 壬 |
| 2 | eho romaji | the point's reading in Hepburn romaji |
| 3 | azimuth | the point's azimuth, degrees clockwise from north: 75, 255, 165 or 345 |
| 4 | sixteen-point | the nearest of the sixteen compass points in Japanese: 東北東, 西南西, 南南東 or 北北西 |
| 5 | direction | the same in English |
| 6 | period | the 三元九運 period, 1 to 9 |
| 7 | period name | its name, 一運 to 九運 |
| 8 | era | 上元, 中元 or 下元 |
| 9 | star | the 九星 that rules it, 一白水星 to 九紫火星 |
| 10 | ruler | the star of the Dipper its source names as ruler, 贪狼 to 右弼 |
| 11 | first year | the year whose 立春 begins the period |
| 12 | last year | its last year; it ends at the next 立春 |
| 13 | without son | `1` if the day is 손 없는 날, else `0`; empty outside `dangi`'s years, 1645 to 2150 |

On 節分, 3 February 2026, the 恵方巻 face 丙, 南南東, in 九運, which runs
from 2024 to 2043. The day answers for the years −1000 to 3000; a
meridian not read is `HC_ERR_UNKNOWN`.

### The almanac's day

`hc_almanac_day(fixed, meridian_ptr, meridian_len, locale_ptr, locale_len,
buffer, capacity)` needs the `calendars` feature and writes the rest of
what `hc-almanac` gives a day, one annotation a line, in the order a
printed almanac page gives them: the sexagenary day and its 納音, 十二直,
二十八宿 and the 二十七宿 of 宿曜道, the year's, the month's and the day's 九星, 六曜,
then each of the 暦注下段, the 選日 and the modern combinations of them
that falls on the day, in `hc-almanac`'s listing order. `meridian` is read
as above and sets where the solar terms and the new moons fall; the
locale, a BCP 47 tag, comes after it. It computes nothing `hc-almanac`
does not, and leaves out 七曜, which is the weekday. Each line is an
annotation, so a day writes as many lines as it has, the kind in column 1
and the identifier in column 2:

| # | Column | Holds |
| --- | --- | --- |
| 1 | kind | `sexagenary`, `nayin`, `twelve-direct`, `mansion`, `mansion-27`, `year-star`, `month-star`, `day-star`, `rokuyo`, `lower-register`, `selected-day` or `combination` |
| 2 | id | for a cycle, the term's 1-based position in it: 甲子 1 to 癸亥 60, 海中金 1 to 大海水 30, 建 1 to 閉 12, 角 1 to 軫 28 (and to 27 without 牛), 一白水星 1 to 九紫火星 9, 先勝 1 to 赤口 6; for a table entry its `hc-almanac` identifier, lower-case kebab: `tenshanichi`, `ichiryu-manbai`, `pardon-and-grain` |
| 3 | name | the name in the locale, from `hc-i18n`: the locale's own where its data has one, else English's, else Japanese's; under `native`, the almanac's own language, Japanese first. The sexagenary day is in the locale's reading of the cycle by the same rule: 甲子, `jia-zi`, 갑자 |
| 4 | locale used | the tag of the data that named column 3: `ja`, `en`, or for the sexagenary day any locale with a reading of the cycle, `zh-Hant`, `ko`, `vi` |
| 5 | japanese | the name the Japanese almanac prints, whatever the locale |
| 6 | reading | its Hepburn reading, as `hc-almanac` writes it: `shakkō`, the Sino-Japanese `hitsu` for a mansion, the kun readings `kinoe ne` for the sexagenary day; empty for a combination, which has none |
| 7 | auspicious | `1` where the almanac counts the day auspicious, `0` where inauspicious: every 暦注下段, the 選日 that have a verdict, the mansion by the commonest Japanese listing (only 鬼宿 and 牛宿 are agreed by every source), a combination that is not a clash; else empty |
| 8 | printed | for the 暦注下段, `1` where an almanac prints the entry and `0` where 受死日 or 十死日, which are printed alone, suppress it; empty for every other kind |

Japanese names every term, `ja` as the almanacs print it. English writes
the terms in the romanisation of column 6, as English writes 六曜, and the
mansions by the asterisms' English names, Horn to Chariot; the
combinations are commerce, and have no English name, so under `en` they
are Japanese with `ja` in column 4. `zh-Hans` names the 納音 alone, as
『三命通會』 heads them: 炉中火, 路旁土. No other locale has names for the
almanac: Chinese and Korean almanacs name some of the same cycles, but no
source for them was read, and none is translated, so every other locale
writes English's. On 21 December 2025, a 甲子, the almanacs print 赤口 with
天恩日, 天赦日 and 一粒万倍日; under `ja` that day's lines include
`rokuyo 6 赤口 ja 赤口 shakkō` and `combination pardon-and-grain
天赦日＋一粒万倍日 ja`. The day answers for the years −1000 to 3000; a
meridian not read is `HC_ERR_UNKNOWN`.

### The year's directions

`hc_almanac_directions(fixed, meridian_ptr, meridian_len, buffer,
capacity)`, in the `calendars` feature, writes where the 方位神 stand in
the 干支 year in force on a day, the year turning at 立春 at the meridian,
from `hc-almanac`'s `direction_deities`: the eight 八将神 by the year's
branch, 太歳神 to 豹尾神, then 金神 once for each branch the year's stem
gives it, then 大金神 and 姫金神 by the branch. The UI of an almanac page
draws them as the year's chart; `hc_almanac_cycles` gives 恵方, the
direction of 歳徳神, beside them.

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | `taisai`, `daishogun`, `daion`, `saikyo`, `saiha`, `saisetsu`, `oban`, `hyobi`, then `konjin`, `dai-konjin` and `hime-konjin` |
| 2 | name | the god's name: 太歳神, 大将軍, 金神 |
| 3 | reading | its Hepburn reading as the National Diet Library gives it, `taisaijin`; empty for the three 金神, whose readings `hc-almanac` does not carry |
| 4 | branch | the direction as an earthly branch, 午 |
| 5 | azimuth | the branch's azimuth in degrees clockwise from north, 180 |
| 6 | meaning | what the Library says the god forbids or favours, in English; empty for the 金神 |
| 7 | year | the 干支 year in force on the day, 丙午 |
| 8 | branch number | the branch's number, 1 for 子 to 12 for 亥 |

After them comes a line for each reading of their 遊行, the five days at a
time 大将軍 and 金神 leave their directions, each a rule of its own from
`hc-almanac`'s `direction_deities::WanderingRule`: `daishogun-iinippon`,
いい日本再発見's list, and `konjin-wikipedia-begun-in-season` and
`konjin-wikipedia-days-in-season`, the two readings of Japanese Wikipedia's
table. Such a line has the rule's identifier in column 1, the god's name
in column 2, the place gone to in columns 4, 5 and 8 — 中央, the middle of
the house, with columns 5 and 8 empty, all three empty when the god is at
home — and `home` or `gone` in column 6, empty where the rule does not
say. On 金神の間日, the days its direction may be crossed, a last line
`konjin-rest-day` follows.

2026 is a 丙午 year from 立春 on 4 February: 太歳神 stands on 午 and 歳破神
opposite it on 子, and 歳刑神 on 午 as Japanese Wikipedia's table has it,
which 古文書ネット confirms; in 2025, 乙巳, 金神 stood on 辰 and 巳. A
meridian not read is `HC_ERR_UNKNOWN`, and a day outside the years −1000
to 3000 `HC_ERR_OUT_OF_RANGE`.

### 臘日

`hc_rounichi(rule_ptr, rule_len, year, meridian_ptr, meridian_len)`, in
the `calendars` feature, returns the fixed day of 臘日 in the winter that
ends in Gregorian `year`, in January or early February, by one of
`hc-almanac`'s reckonings, each reading of an ambiguous wording a rule of
its own: `second-dragon-after-minor-cold` and
`second-dragon-from-minor-cold`, the second 辰 day after 小寒;
`dragon-nearest-major-cold-earlier` and `dragon-nearest-major-cold-later`,
the 辰 day nearest 大寒, which こよみる calls the current mainstream and the
神社暦's; `first-dog-after-major-cold` and `first-dog-from-major-cold`, the
first 戌 day after 大寒; `lunar-twelfth-ninth`, the ninth of the twelfth
lunar month; `ox-month-ninth-from-minor-cold` and
`ox-month-ninth-after-minor-cold`, Japanese Wikipedia's 「丑節9日」; and
`third-dog-after-winter-solstice` and `third-dog-from-winter-solstice`, the
Qin and Han third 戌 day after 冬至. An `-after-` rule does not count the
term's own day and a `-from-` rule counts it; `-earlier` and `-later` take
one or the other of two 辰 days six days either side of 大寒. Many almanacs
leave 臘日 out, and none is a default. A winter whose lunar year the
calendar does not reach is `HC_ERR_NO_DATA`. こよみる's candidates of 2026
are 18 January by the second 辰 and the nearest 辰 and 24 January by the
first 戌.

### The undertakings of each 二十八宿

`hc_mansion_undertakings(list_ptr, list_len, fixed, buffer, capacity)`, in
the `calendars` feature, writes what one publisher's list says the day's
二十八宿 favours and forbids, as the publisher prints it, from
`hc-almanac`'s `mansion_undertakings`: `saijigoyomi`, 歳事暦's 「暦の吉凶
二十八宿」, or `linderabell`, うまずたゆまず's 「二十八宿」. No national body
publishes such a list, so there is no default; the two differ only in 觜宿.
The mansion is the almanac's 28-day cycle, `hc_almanac_day`'s `mansion`
line. One line per undertaking, in the publisher's order:

| # | Column | Holds |
| --- | --- | --- |
| 1 | mansion | the mansion's number, 1 for 角 to 28 for 軫 |
| 2 | mansion name | its name, 角 |
| 3 | grade | `best` (大吉), `favoured` (吉), `avoided` (凶), `worst` (大凶), or `note` for the list's remark about the day as a whole |
| 4 | undertaking | the undertaking or the remark, in Japanese as printed: 衣類裁断, 葬式 |

8 January 2026 is 角宿, for which 歳事暦 favours 衣類裁断 first and avoids
葬式 and 納骨.

### A person's own days

`hc_almanac_person_days(fixed, birth_fixed, meridian_ptr, meridian_len,
buffer, capacity)`, in the `calendars` feature, says whether a day is one
of the 暦注下段 that fall on a person by the year they were born in:
五墓日 by each reading that gives one day to each 納音 phase,
`gomunichi-wikipedia` (Japanese Wikipedia, こよみる and 歳事暦) and
`gomunichi-nikkoku` (精選版日本国語大辞典), and the three 悪日, 大禍日,
狼藉日 and 滅門日, which fall on a person only in the 節月 whose branch is
their birth year's. `hc_almanac_day` writes the same entries for
everyone. `birth_fixed` is the fixed day the person was born on, and
their year is the 干支 year in force on it at the meridian, which turns at
立春, as `hc_almanac_directions` reads the year's, and as こよみる says the
evil days are read, those born between 1 January and 節分 by "前年の干支"
(`koyomil-taikanichi`; `koyomil-gomunichi` says the same of 五墓日): a person born on
1 February 1928, before that year's 立春 on the 5th at Japan's meridian,
is of 丁卯, 1927's year, and of the fire phase, and one born in June of
戊辰, 1928, of the wood phase.

| # | Column | Holds |
| --- | --- | --- |
| 1 | kind | `grave-day` or `three-evil-day` |
| 2 | id | the reading's identifier, `gomunichi-wikipedia`, or the entry's, `taikanichi`, `rojakunichi`, `metsumonnichi` |
| 3 | name | the entry's name, 五墓日 or 大禍日 |
| 4 | keeps | the 干支 of the person's grave day, 乙丑 for the wood phase, or the branch of the 節月 their evil days fall in, 巳 |
| 5 | applies | `1` if the day is that entry for the person, else `0` |

こよみる gives 25 February 2025 as a 五墓日 of a person born in 1928, of the
wood phase (`birth_fixed` 703 974, 1 June 1928); Japanese Wikipedia's person born in a 巳 year keeps 大禍日 on
申 days of 巳月, and 15 May 2025 was one.

### The Tibetan almanac

`hc_tibetan_almanac_day(calendar_ptr, calendar_len, fixed, buffer,
capacity)`, in the `calendars` feature, writes what a version's almanac
prints beside the date, from `hc-calendars-regional`'s `tibetan_almanac`
(Janson's "Tibetan calendar mathematics", Section 10, checked against
Henning's computed almanacs): the five components (*lnga-bsdus*) — the
weekday, the lunar day, the lunar mansion, the *yoga* and the *karaṇa* —
and the columns after them, one a line. `calendar` is `tibetan`,
`tibetan-tsurphu`, `tibetan-bhutan`, `mongolian`, `tibetan-lochen`,
`tibetan-tsurphu-karana` or `tibetan-bhutan-lochen`; the Tsurphu versions
read the *karaṇa* Sun.
`hc_describe_day` gives the date itself.

| # | Column | Holds |
| --- | --- | --- |
| 1 | kind | `weekday`, `mansion`, `yoga`, `karana`, `half-day`, `sun`, `mean-sun`, `rahu` (on `tibetan` and `tibetan-lochen`, the versions from the epoch of 806, alone), `rab-byung`, `royal-year`, `year-symbol`, `month-symbol` (where Janson gives the version's rule, all but `tibetan-bhutan`) and `day-symbol` |
| 2 | id | the weekday, 1 for Saturday to 7 for Friday; the mansion or yoga, 1 to 27; the karaṇa, 1 to 11; the half-day of the month at daybreak, 1 to 60; the rab byung position, 1 for Prabhava; the royal year, counted from 127 BCE; for a symbol, the animal, 1 for the Mouse to 12 for the Pig; empty for the longitudes |
| 3 | name | the Sanskrit name, or the English weekday, as Henning's almanacs print them, `Shatabhishaj`; for a symbol the element and animal, `Water-Snake` |
| 4 | tibetan | the Tibetan name in Wylie, `mon gru`; empty for a symbol |
| 5 | reading | the almanac's reading, the whole part and two sexagesimal places: the true weekday, the end of the lunar day in days after Saturday's dawn, empty on the first of two days with one number; the Moon at daybreak and the yoga longitude, in mansions; the true Sun and Rāhu in mansions, the mean Sun in signs; for a symbol, `male` or `female` |
| 6 | value | the reading as a decimal; for a symbol, the element's colour, `black` |

Henning's Tsurphu almanac prints 11 February 2013, its first day, as
Monday, Shatabhishaj/mon gru, Parigha/yongs 'joms and Vava/gdab pa, with
the true weekday 2;11,24 and the yoga's 18;26,38, and
`tibetan-tsurphu-karana` writes the same. A day outside the version's
range, the Tibetan years 1000 to 3000, is `HC_ERR_OUT_OF_RANGE`.

### The Tibetan planets

`hc_tibetan_planets(fixed, buffer, capacity)` writes where the Phugpa
almanac places the five planets at the end of a day, from Henning's epoch
of 1927 (Janson, Appendix D), one a line: the planet, `mercury`, `venus`,
`mars`, `jupiter` or `saturn`; its particular day; its mean heliocentric,
true slow and fast longitudes in mansions as the almanac reads them; and
the same three as decimals. Henning's worked example of 6 January 2011
gives Mars the particular day 525.

| # | Column | Holds |
| --- | --- | --- |
| 1 | planet | `mercury`, `venus`, `mars`, `jupiter` or `saturn` |
| 2 | particular day | its day in the planet's heliocentric cycle (*sgos zhag*) |
| 3 | mean heliocentric | the mean heliocentric longitude, in mansions |
| 4 | true slow | the true slow longitude (*dal dag*) |
| 5 | fast | the fast, geocentric, longitude (*myur ba*) |
| 6 | mean heliocentric value | column 3 as a decimal |
| 7 | true slow value | column 4 as a decimal |
| 8 | fast value | column 5 as a decimal |

### The Bhutanese winter solstice

`hc_bhutanese_winter_solstice(year, buffer, capacity)` writes the
Bhutanese calendar's winter solstice of a Gregorian year, the instant its
mean Sun reaches 250° (Janson, Appendix A.4), which falls in the first
days of January today:

| # | Column | Holds |
| --- | --- | --- |
| 1 | fixed | the fixed day it falls on |
| 2 | reading | the weekday and time the almanac prints, days after Saturday's dawn, nāḍī and pala: `2;51,38` for 2001 |
| 3 | julian date | the local Julian Date, as a decimal |

The mean Sun's year is the calendar's, *m*₁ / *s*₁ = 6 714 405 / 18 382
days, 365.270 645, which is 0.028 145 days longer than the Gregorian
calendar's 365.2425, so the solstice comes a day later every 35½ years.
Janson's first 3 January is 2020; 319 years before, the drift is nine
days, and the solstice of the winter 1700 ends in is 24 December 1700,
while 2100's is 4 January and 3000's 30 January. The year is the one the
solstice falls in, so between the December solstices and the January ones
a year of 365 days can hold none, and ten do: 1923, 1927, 1931, 1935,
1938, 1942, 1946, 1949, 1953 and 1957 are `HC_ERR_NO_DATA`.

### Tibetan festivals on a skipped or repeated date

`hc_tibetan_festival_day(rule_ptr, rule_len, calendar_ptr, calendar_len,
year, month, leap, day)` returns the fixed day a festival on a Tibetan date
is kept on when the number is skipped or repeated, by a rule for the date
alone: `berzin`, the day before a skipped number and the first of a
repeated one, as Janson reports it from Berzin, unchecked against
published calendars; or `henning-almanac`, the second of a repeated number
and no day for a skipped one, as Henning's computed almanacs mark their
festivals. The holiday tables `buddhist-tibetan-berzin` and
`buddhist-tibetan-henning` of `hc_holidays_on` choose between the same two
by their code. Henning's almanacs do not mark the Birth of the Buddha in
1990, whose 7th of month 4 is skipped, so that year's day under
`henning-almanac` is `HC_ERR_NO_DATA`.

### Folk days

`hc_folk_day(fixed, meridian_ptr, meridian_len, locale_ptr, locale_len,
buffer, capacity)`, in the `calendars` feature, writes the folk
reckonings of a day that are not the Japanese almanac's, each named in a
locale: the four counts of the Chinese first month, from `hc-almanac`'s
`first_month_counts`; 入梅 and 出梅 by each rule of `hc_plum_rains`; Tam
Nương and Nguyệt Kỵ of the Vietnamese lunar calendar, from `hc-almanac`'s
`vietnamese_days`; and the Turkish folk year of Hızır and Kasım, from
`hc-seasons`'s `hizir_kasim`. `meridian` sets where 芒种 and 小暑 fall. One
line a reckoning, the kind in column 1 and the identifier in column 2:

| # | Column | Holds |
| --- | --- | --- |
| 1 | kind | `first-month-count`, four lines, always; `plum-rains`, when the day is 入梅 or 出梅 by a rule; `vietnamese-day`, when the day is Tam Nương or Nguyệt Kỵ; `folk-half`, always; and `folk-named-day`, when the day is a named day of the Turkish folk year |
| 2 | id | for `first-month-count` `dragons` (几龙治水), `oxen` (几牛耕田), `xin` (几日得辛) and `cakes` (几人分饼); for `plum-rains` `ru-mei-bing`, `ru-mei-ren` or `chu-mei-wei`; for `vietnamese-day` `tam-nuong` or `nguyet-ky`; for `folk-half` `hizir`, from 6 May, or `kasim`, from 8 November; for `folk-named-day` `hidirellez`, `kasim`, `erbain`, `hamsin`, `cemre-air`, `cemre-water` or `cemre-earth` |
| 3 | name | its name in the locale, from `hc-i18n`: the locale's, else English's, else the language its source writes it in — Chinese, Vietnamese or Turkish, which no other language names here — which `native` asks for first |
| 4 | locale used | the tag of the data that named column 3: `zh-Hans`; `zh-Hant` for three of the four counts of the first month under a Traditional tag, 幾龍治水, 幾牛耕地 and 幾日得辛 as NOWnews writes them (`nownews-er-long-zhi-shui-2020`); `vi`, `tr` |
| 5 | count | for `first-month-count` the day of 正月 on which the Chinese year the day is in has its first 辰, 丑, 辛 or 丙 day, 1 to 12 or 1 to 10; for `vietnamese-day` the lunar day; for the Turkish kinds the day's count in its half, from 1; empty for `plum-rains` |

The Chinese counts need the `chinese` calendar's years, 1645 to 2150, and
the Vietnamese days the `vietnamese` calendar's; outside them those lines
are left out. In 2026, 七龙治水, 四牛耕田, 十日得辛 and 五人分饼; 23 March
2026, the 5th of the second lunar month, is Nguyệt Kỵ; and 20 February
2026 is Kasım 105, `birinci cemre`. The day answers for the sky layer's
era; a meridian not read is `HC_ERR_UNKNOWN`.

### The night watches

`hc_night_watch(seconds_of_day, locale_ptr, locale_len, buffer,
capacity)` writes the Chinese night watch, 更, and its points, 點, of a
time of the caller's civil clock, whole seconds after midnight, by the
fixed reckoning of `hc-format`'s `night_watches`: 19:00 to 05:00 in five
watches of two hours, each of five points of 24 minutes, counted as
struck. From 05:00 to 18:59 it writes nothing, and a time from 86 400 s is
`HC_ERR_OUT_OF_RANGE`. At night it writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | watch | 1 (一更) to 5 (五更) |
| 2 | points | the points struck since the watch began, 0 to 4 |
| 3 | name | the watch's name in the locale, 一更 to 五更, which Chinese alone names, under `zh-Hant` and `zh-Hans` (`chinanews-wu-geng-2014`) |
| 4 | locale used | the tag of the data that named column 3, `zh-Hant` or `zh-Hans` |
| 5 | han name | its Han name, 黃昏, 人定, 夜半, 雞鳴 or 平旦, in Chinese as the source writes it |
| 6 | branch | its double hour, 戌 to 寅 |

"三更两点" is 23:48: `3 2 三更 zh-Hant 夜半 子`.

## Deep time

`hc_place_years_ago`, `hc_cosmic_events`, `hc_earliest_evidence`,
`hc_archaeological_periods`, `hc_future_events` and `hc_geologic_intervals`
need the `deep-time` feature. All six take a locale, a BCP 47 tag, after
their other arguments, and write lines of the same sixteen columns, so a
page parses them once:

| # | Column | Holds |
| --- | --- | --- |
| 1 | kind | `moment`, `cosmic-epoch`, `cosmic-event`, `earliest-evidence`, `future-era`, `future-event`, a geologic rank (`eon`, `era`, `period`, `epoch`, `age`) or `archaeological` |
| 2 | id | the entry's stable identifier, lower-case kebab like a calendar's: `recombination`, `maastrichtian`, `bronze-age`, `earliest-writing-uruk-iv`; match on this, not on the name |
| 3 | name | the entry's English name |
| 4 | scope | the id of the interval one rank up for a geologic interval; the region for an archaeological period; the landmark (`earliest-life`, `earliest-homo-sapiens`, `earliest-writing`) for an earliest-evidence claim; the kind of prediction (`modelled`, `experimental-bound`, `order-of-magnitude`) for a future event; else empty |
| 5 | start | the older bound's value; for a future event, the sooner |
| 6 | start σ | its standard uncertainty; empty only on an `hc_earliest_evidence` row whose source states none |
| 7 | start figures | the significant figures it claims, or empty where the table claims none |
| 8 | start approximate | `1` where the table marks it approximate — the chart's `~`, a source's "about" — else `0` |
| 9 | end | the younger bound's value |
| 10 | end σ | as column 6 |
| 11 | end figures | as column 7 |
| 12 | end approximate | as column 8 |
| 13 | unit | what columns 5 to 12 are in, below |
| 14 | description | the table's description of the entry, or empty |
| 15 | source | where the numbers came from |
| 16 | localised name | the geological chart's own name for an interval in the locale's language — 第四系／紀, 显生宇, `Quartär` — or, for a cosmic, archaeological or earliest-evidence row, the established term `hc_deep_time::names` carries for it — 宇宙の晴れ上がり, 青銅器時代 — or empty: for a language with neither, for the handful of intervals the chart names in no language, for an entry no established term was read for, and for every future row |

The localised interval names are the International Commission on
Stratigraphy's own translations, from the chart's vocabulary (`chart.ttl`,
CC BY 4.0) in fourteen of the module's locales — `cs`, `de`, `es`, `fr`,
`id`, `it`, `ja`, `ko`, `nl`, `pl`, `pt`, `ru`, `tr` and `zh-Hans` — with
the Japanese checked against the Geological Society of Japan's chart and the
Chinese against the ICS's Chinese chart; `hc_deep_time::names` says what was
left out and why. The other rows are named in Japanese only, each from the
source it was read in — the Astronomical Society of Japan's 天文学辞典, the
Japanese Wikipedia, Nature's Japanese highlights — and an entry with no
established term is not translated. The English name stays in column 3.

A point in time — an event, the moment itself — has the same start and end.
The unit is the one each table counts in, so the values are the tables' own
figures rather than conversions: `seconds-since-big-bang` for the cosmic
rows and the moment's `since-big-bang` row, `seconds-before-present` for
its `before-present` row, `megayears-before-present` for the geologic
chart's rows, `years-before-1950` for the archaeological and
earliest-evidence rows, `years-from-now` for a future event and
`log10-years-from-now` for a future era. A future event that is an
experimental bound — the proton lifetime's — has a start and no end: if it
happens at all, it is no sooner.

**The earliest evidence.** `hc_earliest_evidence` lists the published
claims to the earliest evidence of life, of *Homo sapiens* and of writing,
one row per claim, grouped by landmark and oldest first — Jack Hills, Akilia,
Nuvvuagittuq, Isua and the Pilbara for life; Jebel Irhoud and Omo-Kibish for
*Homo sapiens*; Abydos tomb U-j and Uruk IV for writing — so that a
landmark is never one number where the literature has several.
Each row keeps the shape of its source's date: an age has the same start
and end; a minimum age ("at least 233 ± 22 kyr") has an empty start and the
minimum as its end; a range ("at least 3,770 and possibly 4,280 million
years") has two different bounds. A σ cell is empty unless the source says
what its `±` is — Richter et al.'s 315 ± 34 ka is kept in the description
for that reason — and a disputed claim names the rebuttal in its
description. `docs/systems/earliest-evidence.md` gives the sources.

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
first. `hc_archaeological_periods(locale_ptr, locale_len, buffer, capacity)`
lists the conventional archaeological periods, youngest first, with the
region as the scope, and `hc_future_events(locale_ptr, locale_len, buffer,
capacity)` the dated events of the far future, soonest first.
`hc_geologic_intervals(rank, locale_ptr, locale_len, buffer, capacity)`
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

`zone` is an IANA name in any case. **The module carries eighteen zones
and only their current rules**: `hc-tz`'s built-in table, which is the
POSIX `TZ` footer of each zone's file in the IANA database, release 2026d
(unchanged since 2026c).
A POSIX string states one pair of rules for every year, so the table is
right about today and wrong about the past — it does not know the United
States moved its transitions in 2007 or that Brazil stopped changing its
clocks in 2019. The eighteen: `UTC`, `Africa/Cairo`, `America/Denver`,
`America/Los_Angeles`, `America/New_York`, `America/Sao_Paulo`,
`Asia/Kathmandu`, `Asia/Kolkata`, `Asia/Seoul`, `Asia/Shanghai`,
`Asia/Tokyo`, `Australia/Lord_Howe`, `Australia/Sydney`, `Europe/Berlin`,
`Europe/London`, `Europe/Moscow`, `Europe/Paris` and `Pacific/Auckland`. The module has no file system, so
that is all it can know on its own.

For any other zone, or for a zone's history, a page fetches the zone's TZif
file — the IANA database's own format, `/usr/share/zoneinfo/Europe/Rome` on
most systems — and hands its bytes to `hc_zone_load(name_ptr, name_len,
tzif_ptr, tzif_len)` once; the two conversions, `hc_zone_offset` and
`hc_radio_encode`'s `zone:` then answer for that name,
with every transition the file records, and a loaded zone outranks a
built-in one of the same name. The bytes are copied into the module and
parsed on each call. Bytes that are not TZif are `HC_ERR_MALFORMED` and
nothing is kept; a name nobody knows is `HC_ERR_UNKNOWN`. Every zone
answers for the instants and the days of the years −9 999 994 to
9 999 994 by UTC, five inside the Gregorian years ±9 999 999 its rules are
read on, since an answer reads the rules of the years around its own;
beyond them a rule would give standard time whatever it says, and the
three exports refuse with `HC_ERR_OUT_OF_RANGE` ("Ranges" above).

```js
const tzif = await (await fetch("tzdata/Europe/Rome")).arrayBuffer();
hc.loadZone("Europe/Rome", tzif);
const today = hc.fixedFromUnixInZone(Math.floor(Date.now() / 1000), "Europe/Rome");
```

The `tzdata/` artifact that CI uploads, and `scripts/wasm-tzdata.sh`
writes, holds the eighteen built-in zones' files with the database's
release in `VERSION`; see "tzdata beside the module" above.

### A zone's offset

`hc_zone_offset(zone_ptr, zone_len, unix_seconds, buffer, capacity)`
writes one line: the offset a zone keeps at an instant, read from exactly
the rules `hc_fixed_from_unix_in_zone` reads for the name, so that a page
that shows a zone's time, or encodes a time signal, has no need of the
browser's `Intl` data, and its clock and its days cannot disagree. The
instant plus column 1, floored to the day, is the fixed day
`hc_fixed_from_unix_in_zone` gives.

| # | Column | Holds |
| --- | --- | --- |
| 1 | offset | seconds east of UTC: `3600` for CET, `-21600` for MDT |
| 2 | dst | `1` when the rules call the time daylight saving or summer time, else `0`: the zone's own flag, not a comparison of offsets |
| 3 | abbreviation | the rules' abbreviation, `CET` or `MDT`; empty where they give a numeric one, `+0545` or `-03`, which tzdata writes "If there is no common English abbreviation" (`iana-tz-theory`) |
| 4 | next transition | the POSIX second of the first change after `unix_seconds` of the offset, the flag or the abbreviation; empty where the rules have none — a built-in zone without summer time, or a loaded file whose record ends with no footer |
| 5 | next offset | the offset from that instant; empty with column 4 |
| 6 | rules | which rules answered: `builtin`, the built-in POSIX footer, or `loaded`, a TZif file given to `hc_zone_load` |

A built-in zone's next transition is its POSIX rule's next start or end;
a loaded file's is its next recorded transition that changes something,
then its footer's (RFC 8536 §3.3). Europe/Berlin at 1 774 745 999, the
second before its change of 29 March 2026, is
`3600 0 CET 1774746000 7200 builtin`, and at 1 774 746 000
`7200 1 CEST 1792890000 3600 builtin`; Asia/Kathmandu is
`20700 0` with columns 3 to 5 empty.

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
and `backzone` from release 2026d, unmodified; `docs/systems/zone-locations.md` explains
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

Column 5 is one country for a label: the country `zone.tab` lists the
name under, which gives each name one. Column 4 is `zone1970.tab`'s list
of the countries the zone overlaps, `JP;AU` for `Asia/Tokyo` (Australia's
Eyre Bird Observatory keeps Tokyo's clock); `zone.tab` lists
`Asia/Tokyo` under `JP` alone. Column 5 is the first of column 4 for
every zone but `Europe/Simferopol`, which `zone1970.tab` gives as `RU;UA`
and `zone.tab` as `UA`. A link `zone.tab` places has its own country,
`NO` for `Europe/Oslo`. The cell is empty for a name `zone.tab` has no row
for; release 2026d has a row for every name that answers, but `zone.tab`
is deprecated, and a later release may drop rows.

Column 7 is the zone's exemplar city from CLDR 48, the city by which CLDR
names a zone for a reader. English is always carried: `en.xml`'s value,
else `root.xml`'s, else the city UTS #35 derives from the zone's name, its
last field with underscores as spaces. In a build with the `calendars`
feature too, the city is the locale's where `hc-i18n` carries one — 東京
under `ja`, Wien under `de-AT` — with the tag of the data that answered in
column 8; a locale whose file marks the city as inherited answers with the
root name under its own tag (`Berlin` under `de`). A zone the locale has no
city for, every zone under `native` or a tag whose chain reaches no table,
and every zone in a build without `calendars`, is named in English with
`en`. So column 7 is never empty.

| # | Column | Holds |
| --- | --- | --- |
| 1 | zone | the name of the row that answered: the zone, or the link `zone.tab` places, or for another link the name it leads to |
| 2 | latitude | decimal degrees north of the principal location, negative south, to six places |
| 3 | longitude | decimal degrees east, negative west, to six places |
| 4 | countries | the ISO 3166-1 codes of the countries the zone overlaps, `;`-separated, the location's first |
| 5 | country | the one ISO 3166-1 code `zone.tab` lists the name under: `JP` for `Asia/Tokyo`; empty where `zone.tab` has no row for it |
| 6 | comment | the table's comment, which tells a country's zones apart; empty where the country has one zone |
| 7 | exemplar city | CLDR 48's exemplar city in the locale, else in English |
| 8 | locale used | the tag of the data that named column 7: `ja`, `de`, `zh-Hant`, or `en` |

```js
const zone = Intl.DateTimeFormat().resolvedOptions().timeZone;
const place = hc.zoneLocation(zone, navigator.language); // throws `unknown` for UTC
const today = hc.fixedFromUnix(Math.floor(Date.now() / 1000));
const sunrise = hc.sunrise("usno", today, place.latitude, place.longitude, 0);
```

### A zone's name

`hc_zone_name(zone_ptr, zone_len, unix_seconds, locale_ptr, locale_len,
field_ptr, field_len, buffer, capacity)` needs the `zone-names` feature, a
layer of its own for its 600 kB of names, and writes a zone's name at an
instant in a locale, as the CLDR pattern field `field` writes it, from
CLDR 48's metazones and zone names in every locale `hc-i18n` carries, by
the fallbacks of UTS #35 Part 4 that `hc-format` follows
(`docs/systems/zone-names.md`): `z` to `zzz`, the short specific name,
*PDT*, else the short localized GMT format; `zzzz`, the long, *Pacific
Daylight Time*; `O` and `OOOO`, the localized GMT format, *GMT-7* and
*GMT-07:00*; `v` and `vvvv`, the generic names, *PT* and *Pacific Time*,
else the generic location; `V`, the short zone identifier, `uslax`; `VV`,
the zone as given; `VVV`, its exemplar city, *Los Angeles*; and `VVVV`,
its generic location, *Los Angeles Time*. The offset and the daylight
flag are the zone's own, loaded or built in, as `hc_zone_offset` reads
them. A field of another letter or length, `OO` among them, is
`HC_ERR_UNKNOWN`, and so is a zone neither the loaded zones nor the
built-in table knows. A locale whose data has no name falls back as
CLDR's inheritance does, then to English's; one that does not parse is the
root locale, which writes the GMT format.

| # | Column | Holds |
| --- | --- | --- |
| 1 | name | the name, or its fallback |
| 2 | field | the field as given |
| 3 | zone | the zone as given |
| 4 | offset | its offset at the instant, seconds east of UTC |
| 5 | daylight | `1` for its daylight time, else `0` |

Tokyo is *Japan Standard Time* under `en` and 日本標準時 under `ja`.

### Formatting by pattern

`hc_format_pattern(zone_ptr, zone_len, unix_seconds, locale_ptr,
locale_len, syntax_ptr, syntax_len, pattern_ptr, pattern_len, buffer,
capacity)`, in the same feature, formats an instant in a zone and a locale
by a pattern of `hc-format`'s `patterns`: `cldr`, a pattern of UTS #35
Part 4, whose zone fields are `hc_zone_name`'s; or `strftime`, a POSIX
pattern, whose `%Z` is the rules' abbreviation. The line is
`hc_zone_name`'s, the text in column 1 and the syntax in column 2. Tokyo
at 00:00 UTC on 1 January 2026 is `2026-01-01 09:00 Japan Standard Time`
by `yyyy-MM-dd HH:mm zzzz` and `2026-01-01 09:00 JST` by `%Y-%m-%d %H:%M
%Z`. A pattern `hc-format` does not read is `HC_ERR_MALFORMED`, and a
syntax that is neither `HC_ERR_UNKNOWN`.

POSIX's `%EC`, `%Ey` and `%EY` write the era, the year in it and both of
the calendar the locale's `-u-ca-` key names: the Buddhist and Minguo
calendars in any build, and in a module built with `calendars` too the
Japanese eras for `japanese`, CLDR's key for them. In Tokyo, `%EC|%Ey|%EY`
under `ja-u-ca-japanese` is `平成|31|平成31年` on 30 April 2019 and
`令和|1|令和元年` on 1 May, the first day of 令和
(`wikipedia-ja-gengo-list`), and `Reiwa|8|8 Reiwa` under
`en-u-ca-japanese` on 29 September 2026. A locale with no such key, or a
module without `calendars`, writes the unmodified conversions, as POSIX
says.

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
| 2 | sign id | its identifier, the lower case of its English name: `aries` |
| 3 | sign name | its English name, `Aries` |
| 4 | decan | which of the sign's three decans, 1 to 3 |
| 5 | ruler | the decan's ruler: `saturn`, `jupiter`, `mars`, `sun`, `venus`, `mercury` or `moon` |
| 6 | ruler name | the ruler's English name |
| 7 | degrees into decan | how far into the decan the Sun is, from 0 up to 10 degrees |

An hour after the September equinox of 2026, which the NAOJ puts at
00:05 UTC on the 23rd, the Sun is 0.04° into the first face of Libra, the
Moon's.

### Drekkāṇas

`hc_drekkana_at(unix_seconds, ayanamsa_ptr, ayanamsa_len, buffer,
capacity)`, in the same feature, is `hc_decan_at`'s sidereal twin, from
`hc-seasons`'s `zodiac::drekkana`: the Hindu thirds of the sidereal signs
in the ayanāṃśa's zodiac, each given to the sign itself, the fifth from it
or the ninth, and ruled by that sign's lord, as al-Bīrūnī's table of
section 451 has them. `ayanamsa` is as for `hc_panchanga_at`'s true sky.
One line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | sign | the sidereal sign, 1 for Meṣa through 12 for Mīna |
| 2 | sign id | its identifier, the lower-case ASCII form of its Sanskrit name: `mesha`, `kanya` |
| 3 | sign name | its Sanskrit name in IAST, `Meṣa`, `Kanyā` |
| 4 | drekkana | which of its three drekkāṇas, 1 to 3 |
| 5 | lord | the drekkāṇa's lord, as `hc_decan_at`'s ruler is written |
| 6 | lord name | the lord's English name |
| 7 | degrees into drekkana | how far into the drekkāṇa the Sun is, from 0 up to 10 degrees |
| 8 | lord sign | the sign the drekkāṇa is given to, by its identifier: `mesha` |
| 9 | lord sign name | its Sanskrit name, `Meṣa` |
| 10 | ayanamsa | the ayanāṃśa, by its identifier |

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
`jewish-sabbath-ends-cohn`, 7°5′; `italian-zero-hour`; and the Japanese
dawn and dusk, `japanese-dawn-kansei` and `japanese-dusk-kansei`,
明け六つ and 暮れ六つ at the 寛政暦's 7°21′41″, and `japanese-dawn-naoj` and
`japanese-dusk-naoj`, the Observatory's 夜明 and 日暮 at 7°21′40″. The
first five angles are Reingold and Dershowitz's; `hc-astro` did not read
the authorities' own texts. The Japanese ones are the 寛政暦書's, as the
Observatory's 暦Wiki gives them, and `docs/systems/hours-of-the-day.md`
works them. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | instant | the time as POSIX seconds of Universal Time, rounded down, or empty when it does not happen |
| 2 | missing | as column 3 of `hc_solar_time`, or `no-noon-shadow` where the Sun is not up at noon to cast the ʿaṣr shadow |
| 3 | missing day | as column 4 |
| 4 | depression | as column 5, and empty for a depression that is not a whole number of arcminutes, the Japanese dawn and dusk's |
| 5 | depression arcseconds | for `depression`, the depression sought in arcseconds, the one exact figure for the Japanese dawn and dusk; else empty |

At Kyoto on 20 March 2020 the Observatory's 夜明 is within ten seconds of
the 5:28:47 JST こよみのページ gives, and at Helsinki at midsummer, where
the Sun stays within 6.4° of the horizon, `japanese-dusk-kansei` names the
missing depression as 26 501 arcseconds. The four cells from column 2 on
are the missing solar event of every later line in this section.

### Horizons

"Sunrise" is the moment the Sun's upper limb meets the visible horizon,
and authorities disagree on where that horizon is: how much the air lifts
the Sun, and whether a height above the sea lowers the horizon. Each
answer is a named convention of `hc-astro`'s `horizon` module, and
`hc_horizons(locale_ptr, locale_len, buffer, capacity)` lists them, one
line each, named in the locale, a BCP 47 tag:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | `geometric-dip`, the horizon every other export here uses; `usno`; `calendrical-calculations` |
| 2 | english name | its English name |
| 3 | description | what it takes the visible horizon to be: the refraction, the size of the Sun and the Moon, and what a height does |
| 4 | source | where the convention comes from |
| 5 | short name | a short English name for a label, which still tells the three apart: `geometric dip`, `USNO`, `Calendrical Calculations` |
| 6 | name | its name in the locale, from `hc-i18n`, where an observatory or almanac office names it in that language; else column 2 |
| 7 | locale used | the tag of the data that named column 6: `zh-Hant`, `zh-Hans`, `fr`, or `en` for the English name |

`hc-i18n` carries a horizon's name only where a source in the language
names the convention: for Japanese, the National Astronomical
Observatory's own wording for its conventions; for another language, its
national observatory's or almanac office's wording, or the title of the
convention's own source in it; nothing is translated. Only `usno` has such
names: 美國海軍天文氣象台 and 美国海军天文气象台, the Hong Kong Observatory's in its
traditional and simplified pages, for `zh-Hant` and `zh-Hans`, and
`Observatoire naval de Washington D.C.`, the heading of the IMCCE's page
on it, for `fr`. Every other locale — Japanese, whose Observatory names
none of the three conventions, Korean, German, Spanish, Arabic, Persian
and Hebrew among them — and every locale for `geometric-dip` and
`calendrical-calculations`, whose book has no translated title that was
found, is written in English, with `en` in column 7.
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
| 4 | depression | always empty, so that the first four columns have the shape of `hc_solar_event`'s |
| 5 | altitude | the geometric altitude of the Sun's centre at the crossing, in degrees: −50′ at sea level on every horizon, lower for a height where the horizon counts it |

For Jerusalem, 31.78° N, 35.24° E, 740 m, on 1 January 2024, the `usno`
horizon rises and sets within half a minute of the 06:39 and 16:46 (UT+2)
the USNO publishes, and `calendrical-calculations`, lowered 61′ for the
height, rises some five minutes sooner. A place off the globe, or a day
outside the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`.

### Prayer times

`hc_prayer_times(method_ptr, method_len, fixed, latitude, longitude,
elevation, ramadan, buffer, capacity)` writes the Islamic prayer times of
a local day at a place by a named method of `hc-astro`'s `solar_time`,
which `hc_prayer_methods(buffer, capacity)` lists; the method is always
named, never assumed, and one it does not list, the empty string
included, is `HC_ERR_UNKNOWN`. `ramadan` non-zero takes the Ramaḍān
interval of a method that fixes *ʿishāʾ* after *maghrib*, Umm al-Qura's
two; the module has no calendar in this layer, so the caller says. It
writes eight lines, `fajr`, `sunrise`, `zuhr`, `asr-shafii`,
`asr-hanafi`, `maghrib`, `isha` and `midnight`:

| # | Column | Holds |
| --- | --- | --- |
| 1 | time | the time's identifier |
| 2 | instant | as column 1 of `hc_solar_event` |
| 3 | missing | as column 2 of `hc_solar_event`: where the Sun does not reach the method's angle, as at high latitudes in summer, the time is missing, since none of the methods carried states a rule for those latitudes |
| 4 | missing day | as column 3 of `hc_solar_event` |
| 5 | depression | as column 4 of `hc_solar_event` |
| 6 | depression arcseconds | as column 5 of `hc_solar_event` |

*Fajr* is the Sun's centre at the method's depression before sunrise;
*ẓuhr* is the Sun's transit; *ʿaṣr* is both the Shafiʿi and the Hanafi
shadow rule, since the choice between them is a school's, not the
method's; *maghrib* is sunset or the method's depression; *ʿishāʾ* its
depression or its interval after *maghrib*; and `midnight` the middle of
the night that begins that evening, by the method's rule. By the
`singapore` method on 1 January 2026, each of *fajr*, sunrise, *maghrib*
and *ʿishāʾ* is MUIS's printed time or up to a minute and a half earlier,
as its whole 2026 timetable is. `hc_prayer_methods` writes one line a
method:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the identifier `hc_prayer_times` takes: `mwl`, `isna`, `egypt`, `umm-al-qura`, `umm-al-qura-before-1430`, `karachi`, `tehran`, `jafari`, `france`, `russia`, `singapore` |
| 2 | english name | the authority's name |
| 3 | fajr | the Sun's depression at *fajr*, in arcminutes |
| 4 | maghrib | the depression at *maghrib*, in arcminutes; empty for sunset |
| 5 | isha | the depression at *ʿishāʾ*, in arcminutes; empty for a method of an interval |
| 6 | isha minutes | the interval of *ʿishāʾ* after *maghrib*, in minutes; empty for a method of an angle |
| 7 | isha ramadan minutes | the interval in Ramaḍān; empty for a method of an angle |
| 8 | midnight | `sunset-to-sunrise` or `sunset-to-fajr` |
| 9 | source | where the parameters come from: Pray Times' compilation, the authorities' own documents not read |

### Zmanim

`hc_zmanim(reckoning_ptr, reckoning_len, fixed, latitude, longitude,
elevation, buffer, capacity)` writes the Jewish times of a local day in
temporal hours by one of three reckonings of the day, each its own name,
the identifier of `hc-astro`'s `ZMANIM_RECKONINGS`: `zmanim-gra`, the
Vilna Gaon's, sunrise to sunset; `mga-72-minutes`, the Magen Avraham's
from a dawn 72 minutes before sunrise to a nightfall 72 minutes after
sunset; and `mga-16-1-degrees`, his with both at 16.1°; in any case, and
anything else is `HC_ERR_UNKNOWN`. It writes nine lines: the five
times in temporal hours, `sof-zman-shma`, `sof-zman-tfila`,
`mincha-gedola`, `mincha-ketana` and `plag-hamincha`, then the dawns and
nightfalls no reckoning changes, `dawn-16-1-degrees`, `dawn-72-minutes`,
`nightfall-8-5-degrees` and `nightfall-72-minutes`:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the time's identifier |
| 2 | english name | its name as Hebcal prints it in English; empty for a dawn or a nightfall |
| 3 | hours | its temporal hours from the start of the day: 3, 4, 6.5, 9.5 or 10.75; empty for a dawn or a nightfall |
| 4 | instant | as column 1 of `hc_solar_event` |
| 5 | missing | as column 2 of `hc_solar_event` |
| 6 | missing day | as column 3 of `hc_solar_event` |
| 7 | depression | as column 4 of `hc_solar_event` |
| 8 | depression arcseconds | as column 5 of `hc_solar_event` |

For New York City on 1 January 2025 each is within a minute of Hebcal's:
the latest Shema at 9:04 EST by the Magen Avraham and 9:40 by the GRA,
dawn at 16.1° at 5:52 and nightfall at 17:25 and 17:52. London has no
16.1° dawn on 21 June 2025, and the `mga-16-1-degrees` times that need it
are missing there.

### The temporal hour

`hc_temporal_hour(reckoning_ptr, reckoning_len, fixed, latitude,
longitude, elevation, buffer, capacity)`, in the same feature, writes the
length of the temporal hour `hc_zmanim` counts its times in: a twelfth of
the day the reckoning counts, from `hc-astro`'s `temporal_hour_gra`,
`temporal_hour_mga_72_minutes` and `temporal_hour_mga_16_1_degrees`. The
reckoning and the day are as for `hc_zmanim`, and it writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | reckoning | the reckoning's identifier, `zmanim-gra`, `mga-72-minutes` or `mga-16-1-degrees` |
| 2 | seconds | the hour's length in seconds, a decimal; empty where the day's start or end does not happen |
| 3 | missing | as column 2 of `hc_solar_event` |
| 4 | missing day | as column 3 of `hc_solar_event` |
| 5 | depression | as column 4 of `hc_solar_event` |
| 6 | depression arcseconds | as column 5 of `hc_solar_event` |

For New York City on 1 January 2025 each is within 20 seconds of the hour
Hebcal's printed times give (`hebcal-zmanim-api`): sunrise 7:20 to sunset
16:40 is 46⅔ minutes an hour by the GRA; the 72-minute dawn 6:08 to the
latest Shema 9:04 is three hours of 58⅔ by the Magen Avraham; and the
16.1° dawn 5:52 to 8:56 is three of 61⅓. In London on 21 June 2025,
`mga-16-1-degrees` has no length and names the missing depression.

### The Edo hours

`hc_edo_time(unix_seconds, latitude, longitude, elevation, buffer,
capacity)` reads an instant, as Universal Time, at a place on the Edo
不定時法: the daylight from 明け六つ to 暮れ六つ, at the 寛政暦's depression
of 7°21′41″, cut into six equal hours, and the night to the next 明け六つ
into six more, each hour named by the strokes of the bell that opened it.
It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | day | the fixed day whose 明け六つ began the reading's day: the night after midnight belongs to the day before the civil date |
| 2 | hour | the hour's place in the count from 明け六つ, 0 to 11: 3 is 昼九つ, 6 暮れ六つ, 9 暁九つ |
| 3 | name | its name, 明六つ, 朝五つ, 朝四つ, 昼九つ, 昼八つ, 夕七つ, 暮六つ, 夜五つ, 夜四つ, 暁九つ, 暁八つ or 暁七つ |
| 4 | romaji | the name in Hepburn romaji, `ake mutsu` |
| 5 | strokes | the strokes of the bell it is named by, 4 to 9 |
| 6 | branch | the earthly branch it is paired with, 卯 for 明け六つ; a pairing of names, which says nothing about where a branch's span begins |
| 7 | tenths | the 天保暦's tenths of the hour gone, 分, 0 to 9 |
| 8 | fraction | the fraction of the hour gone, 0 to 1 |
| 9 | missing | as column 2 of `hc_solar_event`, the first eight cells empty where a dawn or a dusk the reading needs does not happen |
| 10 | missing day | as column 3 of `hc_solar_event` |
| 11 | depression | as column 4 of `hc_solar_event` |
| 12 | depression arcseconds | as column 5 of `hc_solar_event` |

`hc_unix_from_edo_time(fixed, hour, fraction, latitude, longitude,
elevation, buffer, capacity)` is the inverse: the reading's day, hour and
fraction, from 0 up to 1, give one line in the shape of `hc_solar_event`'s.
An hour above 11 or a fraction outside that is `HC_ERR_OUT_OF_RANGE`.
Kyoto's 明け六つ on 20 March 2020 begins 明六つ; `hc_solar_event`'s
`japanese-dawn-kansei` is the same instant. Every line here answers for
the sky layer's era, and a place off the globe is `HC_ERR_OUT_OF_RANGE`.

### Planetary hours

`hc_planetary_hour(unix_seconds, latitude, longitude, elevation,
locale_ptr, locale_len, buffer, capacity)` writes the planetary hour at
an instant and a place, from `hc-seasons`'s `planetary_hours`: the
daylight from sunrise to sunset and the night to the next sunrise are
each twelve temporal hours, and each hour is ruled by a planet of the
Chaldean order — Saturn, Jupiter, Mars, the Sun, Venus, Mercury, the
Moon — from the weekday's own at sunrise, as al-Bīrūnī states the rule
and Lilly tabulates it. The small hours before sunrise belong to the
planetary day before. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | day | the fixed day of the sunrise the planetary day began at |
| 2 | hour | 1 to 24 from sunrise: 1 to 12 of the daylight, 13 to 24 of the night |
| 3 | ruler | `sun`, `moon`, `mercury`, `venus`, `mars`, `jupiter` or `saturn` |
| 4 | name | the ruler's name in the locale, else English's: सूर्य, चन्द्रमा and the rest under `hi`, as Drik Panchang's Hindi choghadiya page names each kind's planet (`drik-choghadiya-hi`), and `hc-seasons`'s English otherwise |
| 5 | locale used | the tag of the data that named column 4 |
| 6 | daytime | `1` for an hour of the daylight, else `0` |
| 7 | start | POSIX seconds of Universal Time, rounded down |
| 8 | end | as column 7 |
| 9 | missing | as column 2 of `hc_solar_event`, the first eight cells empty where a sunrise or sunset around the instant does not happen |
| 10 | missing day | as column 3 of `hc_solar_event` |
| 11 | depression | as column 4 of `hc_solar_event` |
| 12 | depression arcseconds | as column 5 of `hc_solar_event` |

`hc_planetary_hours_of_day(fixed, latitude, longitude, elevation,
locale_ptr, locale_len, buffer, capacity)` writes the twenty-four hours
of the planetary day that begins at a fixed day's sunrise, in the same
columns, each ruler the weekday's; where a sunrise or sunset an hour is
counted from does not happen, its start and end are empty and the
missing event is named. Lilly's Monday 15 March 1646, Old Style, 25 March
1647 Gregorian, at London opens with the Moon's hour, and its fourth
hour, at 9:30 local apparent time, is Mars's. Both answer for the sky
layer's era.

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

## Greenwich Mean Astronomical Time

`hc_gmat_from_gmt(fixed, seconds_of_day, attoseconds, buffer, capacity)`,
in the `sky` feature, writes the astronomical date and the Greenwich Mean
Astronomical Time of a reading of GMT, mean solar time at Greenwich
counted from midnight, from `hc-astro`'s `gmat`. The astronomical day
begins at noon and is named by the civil day it begins on, so GMAT is
GMT − 12 h. The *Nautical Almanac* counted its G.M.T. so to 1924 and from
midnight from 1925; which a document used is the document's to say. The
reading is a fixed day of the Gregorian years −9 999 999 to 9 999 999,
whole seconds after its midnight, 86 400 being 23:59:60, and attoseconds
below 10¹⁸. It writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | fixed | the astronomical day |
| 2 | seconds of day | whole seconds after its noon, 0 to 86 399 |
| 3 | attoseconds | the attoseconds of the second, as given |

`hc_gmt_from_gmat(fixed, seconds_of_day, attoseconds, buffer, capacity)`
is the inverse, a reading of GMAT to one of GMT, in the same columns, the
seconds counted from the civil midnight. 23:59:60 has no reading twelve
hours earlier and GMAT reads no second 60, so both are
`HC_ERR_OUT_OF_RANGE`. The almanac for 1924 gives the lunar eclipse of
20 February at "February 20ᵈ 4ʰ 12ᵐ 25ˢ·7" G.M.T. from noon, 16:12:25.7
civil: `hc_gmat_from_gmt` of that day and 58 345 s writes the same day
and 15 145 s.

## Jupiter

Jupiter sets the twelve-year festivals of India, and a sign of the zodiac
is a question about a longitude. The `jupiter` layer answers it from the
complete VSOP87B series for Jupiter (Bretagnon and Francou, 1988: 3 625
terms, 55 kB of tables, which no other layer carries), with the Earth from
the series the sky layer takes the Sun from, the light-time, annual
aberration, nutation and the step to the FK5 frame. Against JPL Horizons'
DE441 the apparent longitude is within 0.5″ from 1500 to 2500 and within
12″ at the ends of the era, −1000 and 3000;
`docs/systems/jupiter-ephemeris.md` has the measurements.
Every export takes an `ayanamsa`, named as for `hc_panchanga_at`, and
answers for the years −1000 to 3000.

`hc_kumbh` and `hc_pushkaram`, in `calendars`, take Jupiter from the
caller. The computed forms are here, in a layer of their own because the
tables are not small and `calendars` need not carry them; a page that loads
both has the caller's form and the computed one.

### Where Jupiter is

`hc_jupiter_at(unix_seconds, ayanamsa_ptr, ayanamsa_len, buffer,
capacity)` writes one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | longitude | the apparent geocentric ecliptic longitude in degrees, in the true equinox of the date: the tropical one |
| 2 | latitude | in degrees |
| 3 | distance | from the Earth, in astronomical units |
| 4 | sidereal longitude | the longitude less the ayanāṃśa, in degrees |
| 5 | sign | the sidereal sign, by its identifier: `vrishabha` |
| 6 | sign name | its Sanskrit name, `Vṛṣabha` |
| 7 | degrees into sign | from 0 up to 30 |
| 8 | daily motion | the longitude's change in degrees a day, by the difference over a day; negative in retrograde |
| 9 | retrograde | `1` when column 8 is negative, else `0` |
| 10 | heliocentric longitude | Jupiter's, geometric, in the mean ecliptic and equinox of the date, in degrees |
| 11 | heliocentric latitude | in degrees |
| 12 | heliocentric distance | from the Sun, in astronomical units |

Jupiter's opposition on 7 December 2024 is at 0 h UT longitude 76.3751533°
by Horizons, latitude −0.6718041° and 4.0894152 AU; this line gives them
to within 0.4″, 0.03″ and 90 km, in retrograde in Vṛṣabha at 52.2° sidereal.
The sidereal longitude is the apparent longitude less the mean ayanāṃśa,
as `hc_sky_at`'s Sun reads: against Drik Panchang's 49 entries of 2001
to 2030 it leaves the same 25″ for every one, which is this library's
Lahiri against Drik Panchang's.

### Jupiter's ingresses

`hc_jupiter_ingresses(from_unix_seconds, to_unix_seconds, ayanamsa_ptr,
ayanamsa_len, buffer, capacity)` writes Jupiter's crossings of the sidereal
boundaries in `[from, to)`, one line each, in time order. Jupiter turns back
out of a sign it has just entered about two years in three, and enters again,
so a year's lines come in runs: 2019 has three, into Dhanus on 29 March (UT),
back into Vṛścika on 22 April and into Dhanus again on 5 November. An
ingress is the same second whatever span asks for it, and the same second
`hc_pushkaram_by_sky` and `hc_pushkarams_in_year` give as the entry of a
festival; so is a rising, and its setting, of `hc_jupiter_risings`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | moment | whole POSIX seconds of Universal Time, rounded down |
| 2 | from | the sign left, by its identifier |
| 3 | from name | its Sanskrit name |
| 4 | to | the sign entered |
| 5 | to name | its Sanskrit name |
| 6 | direction | `forward` into the next sign, `retrograde` back into the one before |

Each is within 7 minutes of Drik Panchang's (`drik-guru-gochar`) with the
25″ taken out, and 40 minutes to 4½ hours from it without. A span longer
than a hundred Julian years, whose search takes a second or so, is
`HC_ERR_OUT_OF_RANGE`.

### Jupiter's risings

`hc_jupiter_risings(from_unix_seconds, to_unix_seconds, ayanamsa_ptr,
ayanamsa_len, buffer, capacity)` writes Jupiter's heliacal risings in
`[from, to)`, each with the name the *Bṛhatsaṃhitā* gives the year of
Jupiter that begins with it. A rising is Jupiter's longitude, west of the
Sun's after their conjunction, passing 11°, the arc of visibility that
Varāhamihira, Bhāskara I and the *Sūrya Siddhānta* give; the setting
before it is the longitude east of the Sun's coming within 11°. The year is
named after the lunar month whose nakṣatras hold the rising: Kārttika for
Kṛttikā and Rohiṇī, Mārgaśīrṣa for Mṛgaśīrṣa and Ārdrā, and on round, each
two nakṣatras but the three of Phālguna, Bhādrapada and Āśvayuja. This is
Jupiter's rising on the true sky by that fixed arc, which no almanac of a
Siddhānta prints, whose Jupiter and arc differ; Drik Panchang's is local and
lands up to four days from it. A year runs from one rising to the next,
about 399 days.

| # | Column | Holds |
| --- | --- | --- |
| 1 | rising | whole POSIX seconds of Universal Time, rounded down |
| 2 | setting | the setting before it |
| 3 | sidereal longitude | Jupiter's at the rising, in degrees |
| 4 | nakshatra | the nakṣatra it is in, 1 for Aśvinī to 27 for Revatī |
| 5 | nakshatra id | its identifier: `pushya` |
| 6 | nakshatra name | its name, `Puṣya` |
| 7 | year | the year's name as the twelve-year cycle of `hc_barhaspatya_year_at` spells it, Chaitra first: `Pausha` |
| 8 | year position | 1 for Chaitra through 12 for Phālguna |

Jupiter was lost in the Sun's light from 14 July to 13 August 2026 by the
arc, 15 July to 12 August by Drik Panchang, and rose in Puṣya, so the year
that began is Pauṣa. Two names in a row can come round, and names are
skipped where Jupiter passes over a whole run of nakṣatras: the rule names
each rising by its nakṣatra, which is what the text says and not the
expunction of a year that Sewell and Dikshit describe.

### The Kumbh Mela by the sky

`hc_kumbh_by_sky(yoga_ptr, yoga_len, year, ayanamsa_ptr, ayanamsa_len,
locale_ptr, locale_len, buffer, capacity)` is `hc_kumbh` with Jupiter's
sign **computed** at the occasion's first moment, in the zodiac of the same
ayanāṃśa, in place of the caller's; the conditions are the same seven. It
writes one line, the thirteen columns of `hc_kumbh`, column 13 never empty,
and two more:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | the condition |
| 2 | site | `haridwar`, `prayag`, `nashik` or `ujjain` |
| 3 | site name | its name in the locale, as `hc_kumbh` names it |
| 4 | locale used | the tag of the data that named column 3 |
| 5 | river | the river the site stands on, in English |
| 6 | jupiter | the sign Jupiter must be in, by its identifier |
| 7 | jupiter name | its Sanskrit name |
| 8 | sun | the sign the Sun must be in |
| 9 | sun name | its Sanskrit name |
| 10 | at new moon | `1` when the Moon must be with the Sun at the new moon, else `0` |
| 11 | from | the Sun's entry into its sign that year, or the new moon within its stay, POSIX seconds rounded down; empty for a new-moon condition whose stay holds no new moon |
| 12 | to | the Sun's entry into the next sign, or the new moon again |
| 13 | holds | `1` when there is an occasion and Jupiter is column 6's sign at its first moment, else `0` |
| 14 | jupiter then | the sidereal sign Jupiter is in at that moment, by its identifier; empty with no occasion |
| 15 | jupiter longitude then | its sidereal longitude in degrees then, which says how far from a boundary the answer is |

For the Maha Kumbh of 2025 Jupiter is in Vṛṣabha at the Sun's entry into
Makara, so `kumbh-prayag-vrishabha` holds. Every Kumbh year that Wikipedia
lists from 1974 to 2028 meets a condition of its site; Prayag's 1977 meets
the second, with Jupiter in Meṣa, and so do 2000, 2012 and 2024, when no
festival was held. `docs/systems/jupiter-festivals.md` has them.

### Pushkaram by the sky

`hc_pushkaram_by_sky(sign_ptr, sign_len, year, ayanamsa_ptr, ayanamsa_len,
rule_ptr, rule_len, latitude, longitude, elevation, meridian_ptr,
meridian_len, locale_ptr, locale_len, buffer, capacity)` is `hc_pushkaram`
for the entry of Jupiter into a sign that falls in a Gregorian year, found
rather than given. `rule` says which entry counts where Jupiter enters, turns
back and enters again: `pushkaram-final-entry`, the entry after which it
stays, which every festival whose dates were read began at, or
`pushkaram-first-entry`, the first, which Wikipedia's table of the festivals
to come follows for two. No line where Jupiter makes no such entry that year,
which is most years. The lines are those of `hc_pushkaram`, then two:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | `pushkaram-ganga` to `pushkaram-pranahita` |
| 2 | name | the river's name in the locale, as `hc_pushkaram` names it |
| 3 | locale used | the tag of the data that named column 2 |
| 4 | region | where the source keeps the river for the sign, in English; else empty |
| 5 | sign | the sign, by its identifier |
| 6 | sign name | its Sanskrit name |
| 7 | first | the first day, a fixed day; empty where the Sun does not set on the day of the entry |
| 8 | last | the twelfth |
| 9 | missing | as column 2 of `hc_solar_event` |
| 10 | missing day | as column 3 of `hc_solar_event` |
| 11 | depression | always empty |
| 12 | depression arcseconds | always empty |
| 13 | entry | the moment Jupiter enters the sign, POSIX seconds of Universal Time rounded down |
| 14 | rule | `rule` in lower case |

The nine festivals whose dates were read, from the Godavari's of 2015 to the
announced Godavari's of 2027, come out on their days; the entry stands 40 to
70 minutes before Drik Panchang's, which is the 25″.

### Pushkarams of a year by the sky

`hc_pushkarams_in_year(year, ayanamsa_ptr, ayanamsa_len, rule_ptr, rule_len,
latitude, longitude, elevation, meridian_ptr, meridian_len, locale_ptr,
locale_len, buffer, capacity)` is `hc_pushkaram_by_sky` for every sign
Jupiter enters in the year, in one call: the lines are those the
sign-by-sign calls write for each sign entered, byte for byte, one sign
after another **in the order of the entries**, with the sign of each line in
its columns 5 and 6. A year in which Jupiter enters two signs has the lines
of both (1999: Mīna on 12 January, Meṣa on 26 May); one in which it enters a
sign, turns back and enters it again has the entry `rule` names, as for
`hc_pushkaram_by_sky`; and a year in which it makes no entry by the rule is
an answer of zero bytes (1971, and 8 years of the 200 from 1900 to 2099 by
the final entry). The search that finds the entries is the one every sign's
call makes, so the page that asked twelve times asks once. The columns are
those of `hc_pushkaram_by_sky`:

| # | Column | Holds |
| --- | --- | --- |
| 1 | id | `pushkaram-ganga` to `pushkaram-pranahita` |
| 2 | name | the river's name in the locale, as `hc_pushkaram` names it |
| 3 | locale used | the tag of the data that named column 2 |
| 4 | region | where the source keeps the river for the sign, in English; else empty |
| 5 | sign | the sign, by its identifier |
| 6 | sign name | its Sanskrit name |
| 7 | first | the first day, a fixed day; empty where the Sun does not set on the day of the entry |
| 8 | last | the twelfth |
| 9 | missing | as column 2 of `hc_solar_event` |
| 10 | missing day | as column 3 of `hc_solar_event` |
| 11 | depression | always empty |
| 12 | depression arcseconds | always empty |
| 13 | entry | the moment Jupiter enters the sign, POSIX seconds of Universal Time rounded down |
| 14 | rule | `rule` in lower case |

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
years of J2000.0, 1899-12-31T12:00 TT (J1900.0) to 2100-01-01T12:00 TT,
the span over which Allison and McEwen state their series good to about
0.008° of `Ls`, three seconds of true solar time; outside it the series
extrapolates with no secular change of Mars's orbit, and an instant there
is `HC_ERR_OUT_OF_RANGE`, never a number. The body table states no span of
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
that mission's clock, `mission` being an identifier the list gives, in
any ASCII case: `viking-1`, not `Viking 1`. The conventions are the ones NASA GISS's
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
west. `body` is an identifier `hc_bodies` gives, `mars` or `titan`, in
any ASCII case; another is `HC_ERR_UNKNOWN`, and the Sun, which has no solar day,
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

## Place names

A holiday table's menu of subdivisions (`hc_holiday_tables`' column 9,
`JP-13;JP-47`) and a zone's countries (`hc_zones`' columns 4 and 5) are
codes. The `places` feature names them, from CLDR 48, in every locale
`hc-i18n` carries, so that a page needs no names of its own:

- `hc_territories(locale_ptr, locale_len, buffer, capacity)` writes a line
  for each of the 295 territories CLDR names — the countries, the UN M.49
  areas (`001` the world, `419` Latin America) and CLDR's `EU`, `EZ`,
  `UN`, `QO`, `XA`, `XB` and `ZZ` — in code order;
- `hc_subdivisions(country_ptr, country_len, locale_ptr, locale_len,
  buffer, capacity)` a line for each ISO 3166-2 subdivision of `country`
  that a carried locale's CLDR file names, and each municipality a holiday
  table lists, in code order, or for all 5 503 and the 21 municipalities
  when `country` is empty; a `country` that is not a territory's code is
  `HC_ERR_UNKNOWN`, and a territory with none, `AQ`, writes nothing;
- `hc_place_name(code_ptr, code_len, locale_ptr, locale_len, buffer,
  capacity)` the line of one code, `JP`, `JP-13` or `JP-14-130`.

A municipality is a region of a table, within its subdivision (ADR 0014),
and CLDR names none: `hc_holiday_tables`' column 9 lists `JP-14-130`, 川崎市,
beside `JP-14`. Their names are `hc-i18n`'s `municipal_names`, from the
sources the tables cite for the cities — the instruments' and JIS X 0402's
names in `ja`, the Hepburn romanisation with its suffix in `ja-Latn`,
`Kawasaki-shi`, and the English name in `en`, `Kawasaki` — for the twenty
designated cities and 長崎市, which is every municipality any table lists,
and a test holds the tables to it. `hc_place_name` writes a municipality's
line in the columns below, and `hc_subdivisions` writes it in code order
after its prefecture, so that the lookup of column 9 that names the
prefectures names the cities too. The lookup takes the first of the
locale's chain that names it, `ja-JP` finding `ja`, and then `en`; the
draft level is empty, which is CLDR's, and the status is `municipal`.

Codes match as every identifier does (`jp-13` finds `JP-13`), and are
written as ISO writes them: CLDR's own ids are lower case with no hyphen,
`jp13`, and are not accepted. The names come from `common/main/<locale>.xml`
for the territories and `common/subdivisions/<locale>.xml` for the
subdivisions, at the draft levels `approved`, `contributed` and
`provisional`: every subdivision name outside English is provisional but
England's, Scotland's and Wales's, and the line says so. The lookup walks
the locale's chain as CLDR's inheritance does — `ja-JP` finds `ja`, `pt-PT`
its own names and then `pt`'s, `zh-TW` finds `zh-Hant`, whose parent is
root, not `zh-Hans` — and then takes English, so that under `zh-TW` Tokyo
is `Tokyo`, from `en`. `native` asks for English.
`docs/systems/place-names.md` gives the lookup with examples, and what is
and is not carried.

| # | Column | Holds |
| --- | --- | --- |
| 1 | code | the code as ISO writes it: `JP`, `001`, or `JP-13` for CLDR's `jp13` |
| 2 | name | the name in the locale, else English's: 東京都 for `JP-13` under `ja`; empty only for a deprecated subdivision neither names |
| 3 | english name | CLDR's English name, `en.xml`'s; empty for the 104 deprecated subdivisions it does not name |
| 4 | locale used | the tag of the data that named column 2: `ja`, `pt` for a name `pt-PT` inherits, or `en`; empty with column 2 |
| 5 | draft | that value's CLDR draft level: `approved`, `contributed` or `provisional`; empty with column 2, and for a municipality, whose names are not CLDR's |
| 6 | status | the code's status in CLDR's validity data: `regular`; `deprecated` for a subdivision CLDR keeps from an earlier ISO list, 476 of them, and for one code the holiday tables use, `GB-EAW`; `macroregion`, `special` or `unknown` for a territory that is not a country; and `municipal` for a municipality, which CLDR does not carry |

```js
const table = hc.holidayTables("ja").find((row) => row.code === "JP");
const names = new Map(hc.subdivisions("JP", "ja").map((row) => [row.code, row.name]));
table.regions.map((code) => names.get(code));     // ["北海道", "札幌市", "仙台市", …, "沖縄県"]
hc.placeName("DE-BY", "de");
// { code: "DE-BY", name: "Bayern", englishName: "Bavaria", localeUsed: "de",
//   draft: "provisional", status: "regular" }
```

## Human-readable time

The `humanize` feature phrases time as a person reads it, from
`hc-humanize`, in every locale it carries, with CLDR 48's plural rules:
*3 hours ago*, *yesterday at 15:05*, *2 hours and 30 minutes*. The crate
does not read a clock, so every export takes both ends from the caller,
and it knows no time zone, so a relative day takes the two fixed days a
page has already read off its own clock — `hc_fixed_from_unix_in_zone`
gives them. `style` is `long`, `short` or `narrow`, CLDR's widths, and
for a duration `compact` as well. A locale that does not parse, and the
empty one, is the root locale, whose phrases are CLDR's `root.xml`'s,
`-1 d`, not English's; the binding's methods default to it, so a page
passes the reader's locale.

### Relative time

`hc_relative_time(then_unix, now_unix, style_ptr, style_len, automatic,
locale_ptr, locale_len, buffer, capacity)` writes how the instant
`then_unix` reads from `now_unix`, both POSIX seconds, and
`hc_relative_day(then_fixed, now_fixed, style_ptr, style_len, automatic,
locale_ptr, locale_len, buffer, capacity)` which calendar day `then_fixed`
is, seen from `now_fixed`. The instant's unit is the one `hc-humanize`'s
conversational thresholds choose, the count truncated, so 90 minutes ago
is *1 hour ago*; a month and a year of a span are the Gregorian means. A
day's offset is the difference of the two day numbers, never a span
divided: 23:30 on one day and 00:30 on the next are *yesterday*, not *an
hour ago*; days are counted up to a week, and then weeks, months and
years. `automatic` non-zero writes the language's own word for an offset
where it has one — *yesterday*, *today*, *now* — as
`Intl.RelativeTimeFormat`'s `numeric: "auto"` does; zero writes the
numeric pattern, *1 day ago*. Both write one line:

| # | Column | Holds |
| --- | --- | --- |
| 1 | phrase | the phrase: *3 hours ago*, *in 2 days*, *yesterday*, 今日 under `ja` |
| 2 | unit | the unit it is counted in, CLDR's field name: `second`, `minute`, `hour`, `day`, `week`, `month` or `year` |
| 3 | count | the signed count, negative in the past |
| 4 | locale used | the tag of the `hc-humanize` data the locale resolved to |

### A relative day at a time

`hc_relative_day_at(then_fixed, now_fixed, seconds_of_day, style_ptr,
style_len, automatic, locale_ptr, locale_len, buffer, capacity)` adds a
time of day to `hc_relative_day`'s phrase, joined by the locale's pattern
for a relative day with a time, CLDR's: *yesterday at 15:05*, `es` *ayer,
15:05*, `ja` *昨日の 15:05*. The time is `seconds_of_day` after midnight
on a 24-hour clock as `H:MM` in the locale's digits, the seconds dropped;
it has no hour cycle and no day period. One from 86 400 is
`HC_ERR_OUT_OF_RANGE`.

| # | Column | Holds |
| --- | --- | --- |
| 1 | phrase | the day phrase and the time joined: *yesterday at 15:05* |
| 2 | unit | the day phrase's unit, as for `hc_relative_day` |
| 3 | count | the day phrase's signed count |
| 4 | time | the time as it was written into the phrase, `15:05` |
| 5 | locale used | the tag of the `hc-humanize` data the locale resolved to |

### Durations

`hc_duration(seconds, style_ptr, style_len, max_components, locale_ptr,
locale_len, buffer, capacity)` phrases a span of seconds in days, hours,
minutes and seconds: `long` joins the unit phrases with the locale's list
pattern, *2 hours and 30 minutes*; `short` and `narrow` write the
abbreviated and narrow phrases; `compact` runs the bare suffixes
together, *2h30m*, which in a locale with no compact convention are the
root's Latin ones. A unit the span does not reach is left out, and at
most `max_components` units are written, the largest first, the rest of
the span dropped rather than rounded; 0 writes every unit. A span of
nothing is *0 seconds*.

| # | Column | Holds |
| --- | --- | --- |
| 1 | phrase | the phrase, of the span's length |
| 2 | negative | `1` if `seconds` was negative, else `0` |
| 3 | locale used | the tag of the `hc-humanize` data the locale resolved to |

```js
const now = Math.floor(Date.now() / 1000);
hc.relativeTime(now - 3 * 3600, now, "long", false, "en").phrase;   // "3 hours ago"
const today = hc.fixedFromUnix(now);
hc.relativeDayAt(today - 1, today, 15 * 3600 + 5 * 60, "long", true, "en").phrase;
// "yesterday at 15:05"
hc.duration(9000, "long", 0, "de").phrase;                          // "2 Stunden und 30 Minuten"
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
- **`hc-humanize`, `hc-fiscal`, `hc-name-days`, `hc-attributes` and
  `hc-units`.** No line format has been designed for them yet; each
  would be a layer of its own. Of `hc-almanac`, 七曜 is not written, since
  it is the weekday; the meanings, glosses and 五行 of its annotations,
  which it gives in English only, are not written either; and of 恵方 not
  the two branches its point lies between, which the point names.
- **CCSDS Level 3 and 4 codes**, which only their agency can read:
  `HC_ERR_NO_DATA`. A Level 2 code is read from the caller's epoch by
  `hc_ccsds_decode_from_epoch`, on TAI for CUC; a CUC count on another
  scale is not.
- **The radio frames' notices that name no minute**: DCF77's third-party
  bits and call bit; the phase code's notice and reserved bits; JJY's spare
  bits. `hc_radio_decode` does not write them and `hc_radio_encode` writes
  them 0. JJY's call-sign frames and their notice of a planned stop are
  `hc_jjy_call_sign_decode`'s and `hc_jjy_call_sign_encode`'s.
- **An ephemeris of Jupiter in the `calendars` layer.** `hc_kumbh`
  takes Jupiter's sidereal sign and `hc_pushkaram` the sign and the
  moment Jupiter enters it from the caller. The `jupiter` layer carries
  the VSOP87B series for Jupiter and says when Jupiter is where; it is a
  layer of its own because its tables are 55 kB that no other layer needs.
- **A year's folk days as a list.** `hc_folk_day` answers for one day, as
  the almanac's exports do; the days in a year of the Turkish named days,
  which `hc-seasons` gives by year, and the first-month counts by
  Gregorian year are not written, and a page finds them by asking the
  days, or for 入梅 and 出梅 `hc_plum_rains`.
- **A whole UUID**: the 16 octets of a version 1 or 6 UUID, with the
  clock sequence and node that `hc-core`'s `uuid::encode` takes and no
  page has yet asked for. `hc_uuid_timestamp_encode` writes the time
  fields only, and leaves the rest to the caller's generator.
