//! The exports the WebAssembly module and the C library share, as one
//! table.
//!
//! Most exports of the two boundaries are the same call in two marshalling
//! dialects: read some numbers and some strings, call one function of this
//! crate, and hand back its line or its number. Each such export is one
//! row here, and each boundary expands the rows into its own `extern "C"`
//! functions, so that neither writes the glue by hand. An export whose
//! shape is its own — several out-parameters in C where WebAssembly writes
//! a line, a different type on each side, a check that has to come before
//! a string is read — is written by hand in its boundary crate instead.
//!
//! A row is the C library's rustdoc, the WebAssembly module's, and the
//! signature:
//!
//! ```text
//! c { /// ... }
//! wasm { /// ... }
//! fn hc_lectionary(fixed: i64) -> line = $crate::holiday_lines::lectionary_line;
//! ```
//!
//! Each argument has a kind, which says how each boundary takes it:
//!
//! | Kind | C | WebAssembly | The function is given |
//! | --- | --- | --- | --- |
//! | `i64`, `u64`, `u32`, `i32`, `f64` | the type | the type | the value |
//! | `int` | `int` | `i32` | the value |
//! | `flag` | `int` | `i32` | `true` for non-zero |
//! | `name(len)` | NUL-terminated; null is `HC_ERROR_NULL_POINTER` | a pointer and a length `len` | the text |
//! | `text(len)` | NUL-terminated; null is the empty string | a pointer and a length `len` | the text |
//! | `opt(len)` | NUL-terminated; null is none | a pointer and a length `len` | the text, `None` for null or empty |
//!
//! A string that is not UTF-8 is `HC_ERROR_NOT_UTF8` and `HC_ERR_NOT_UTF8`,
//! and the WebAssembly module takes a null pointer with a zero length as
//! the empty string and one with a length as `HC_ERR_NULL_POINTER`. The
//! strings are read in the order of the arguments, and the first that
//! fails answers.
//!
//! The answer has one of two shapes:
//!
//! - `line`: the function returns an [`Answer<String>`](crate::boundary::Answer).
//!   The C library writes it NUL-terminated into a caller's buffer, with
//!   the length it needs in `written`; the module copies it into linear
//!   memory, or measures it for a null buffer, and returns its length.
//! - `value(out: kind)`: the function returns an `Answer` of a number or a
//!   `bool`. The C library writes it to the out-parameter `out`, of the
//!   kind's type, having refused a null one first; the module returns it
//!   as an `i64`, a sentinel for a refusal or for a value at or below its
//!   floor.
//!
//! The rows are grouped by layer, the Cargo feature of both boundaries that
//! carries them, and a boundary expands one layer's rows where it keeps
//! that layer: `hc::exports!("holiday", c_exports)`. The layer's feature
//! gates that module, so the rows need no gate of their own. The line
//! functions are this crate's, which the layer's features of this crate
//! bring in.
//!
//! `crates/hyper-calendar/tests/abi.rs` reads the table back to render the
//! boundaries' README tables, and holds every row to its documentation.

/// The table of shared exports, one layer's rows at a time; see the
/// module source for what a row says.
#[doc(hidden)]
#[macro_export]
macro_rules! exports {
    ("civil", $backend:ident) => { $backend! {
        c {
            /// The ISO 8601 weekday of a fixed day, Monday = 1 through Sunday = 7.
        }
        wasm {
            /// The ISO weekday of a fixed day, Monday = 1 through Sunday = 7.
        }
        fn hc_weekday(fixed: i64) -> value(out_weekday: u8) =
            |fixed| Ok($crate::Weekday::from_rd($crate::Rd(fixed)).iso_number());

        c {
            /// The 1-based day of the Gregorian year on a fixed day.
        }
        wasm {
            /// The 1-based day of the year on a fixed day, or an error sentinel.
        }
        fn hc_day_of_year(fixed: i64) -> value(out_day_of_year: u32) =
            |fixed| {
                $crate::civil::Date::from_ordinal(fixed)
                    .map(|date| u32::from(date.day_of_year()))
                    .map_err($crate::boundary::Refusal::from)
            };

        c {
            /// Whether the Gregorian year on a fixed day is a leap year: writes 1
            /// or 0.
        }
        wasm {
            /// Whether the Gregorian year on a fixed day is a leap year: 1, 0, or an
            /// error sentinel.
        }
        fn hc_is_leap_year(fixed: i64) -> value(out_is_leap: int) =
            |fixed| {
                $crate::civil::Date::from_ordinal(fixed)
                    .map(|date| date.is_leap_year())
                    .map_err($crate::boundary::Refusal::from)
            };

        c {
            /// The fixed day a POSIX timestamp falls on, in UTC.
        }
        wasm {
            /// The fixed day a POSIX timestamp falls on, in UTC.
        }
        fn hc_fixed_from_unix(unix_seconds: i64) -> value(out_fixed: i64) =
            |unix_seconds: i64| {
                Ok($crate::Rd::from_unix_days(unix_seconds.div_euclid(86_400)).get())
            };

        c {
            /// Whether a POSIX timestamp names a day that ends with an inserted leap
            /// second.
            ///
            /// A timestamp in a day whose start or whose end is not an `int64_t` —
            /// the first and last part-days of the range — is
            /// [`HC_ERROR_OUT_OF_RANGE`](super::HC_ERROR_OUT_OF_RANGE).
        }
        wasm {
            /// Whether the UTC day containing a POSIX timestamp ends with an inserted
            /// leap second: 1, 0, or an error sentinel.
            ///
            /// A timestamp in a day whose start or whose end is not an `i64` — the
            /// first and last part-days of the range — is [`HC_ERR_OUT_OF_RANGE`](super::HC_ERR_OUT_OF_RANGE).
        }
        fn hc_day_has_leap_second(unix_seconds: i64) -> value(out_has_leap: int) =
            $crate::time_lines::day_has_leap_second;

        c {
            /// The POSIX timestamp of midnight UTC on a fixed day.
            ///
            /// A day whose midnight does not fit an `int64_t` is
            /// [`HC_ERROR_OUT_OF_RANGE`](super::HC_ERROR_OUT_OF_RANGE). There is no floor, as the WebAssembly
            /// module has; see the README.
        }
        wasm {
            /// The POSIX timestamp of midnight UTC on a fixed day.
            ///
            /// A day whose midnight would be at or below [`HC_ERR_FLOOR`](super::HC_ERR_FLOOR) seconds —
            /// before fixed day −104 165 947 503, about 285 million years back — or
            /// would overflow an `i64` — after fixed day 106 751 991 886 463 — is
            /// [`HC_ERR_OUT_OF_RANGE`](super::HC_ERR_OUT_OF_RANGE).
            ///
            /// [`HC_ERR_FLOOR`]: super::HC_ERR_FLOOR
        }
        fn hc_unix_from_fixed(fixed: i64) -> value(out_unix_seconds: i64) =
            |fixed: i64| {
                fixed.checked_sub($crate::hc_calendar::fixed::RD_OF_UNIX_EPOCH)
                    .and_then(|days| days.checked_mul(86_400))
                    .ok_or($crate::boundary::Refusal::OutOfRange)
            };
    } };
    ("timestamps", $backend:ident) => { $backend! {
        c {
            /// A TAI instant as a TAI64, TAI64N or TAI64NA label in lower-case
            /// hexadecimal, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// `format` is `tai64`, `tai64n` or `tai64na`, in any case; anything
            /// else is `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`.
            /// Attoseconds from 10¹⁸, or a second that no TAI64 label can hold (the
            /// labels run below 2⁶³), is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A TAI instant as a TAI64, TAI64N or TAI64NA label in lower-case
            /// hexadecimal, as one UTF-8 line, returning the byte length written.
            ///
            /// `format` is `tai64` (16 digits: the second), `tai64n` (24: the
            /// nanosecond) or `tai64na` (32: the attosecond), in any case; anything
            /// else is `HC_ERR_UNKNOWN`. TAI64 and TAI64N name the second or the
            /// nanosecond that contains the instant, and TAI64NA the instant
            /// itself. Attoseconds from 10¹⁸, or a second that no TAI64 label can
            /// hold (the labels run below 2⁶³), is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_tai64_encode(tai_seconds: i64, attoseconds: u64, format: name(format_len)) -> line =
            $crate::time_lines::tai64_encode_line;

        c {
            /// The full GNSS week a broadcast week names, by a rollover rule and a
            /// reference instant.
            ///
            /// `numbering` is as for `hc_gnss_week`; `rule` is `not-before`, the
            /// first full week at or after the reference's week, or `nearest`, the
            /// one within half a rollover period of it, in any case, and anything
            /// else is `HC_ERROR_UNKNOWN`. `reference_tai_seconds` is the reference
            /// as whole TAI seconds; one before week zero counts as week zero. A
            /// broadcast week that does not fit the field, or an answer past week
            /// 2³² − 1, is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The full GNSS week a broadcast week names, by a rollover rule and a
            /// reference instant, or an error sentinel.
            ///
            /// `numbering` is as for `hc_gnss_week`. `rule` is `not-before`, the
            /// first full week at or after the reference's week — for a date the
            /// receiver knows it is not before, such as its firmware's build date —
            /// or `nearest`, the one within half a rollover period of it, in any
            /// case; anything else is `HC_ERR_UNKNOWN`. `reference_tai_seconds` is
            /// the reference as whole TAI seconds; one before week zero counts as
            /// week zero. A broadcast week that does not fit the field, or an
            /// answer past week 2³² − 1, is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_gnss_resolve_week(
            numbering: name(numbering_len),
            broadcast: u32,
            rule: name(rule_len),
            reference_tai_seconds: i64,
        ) -> value(out_week: u32) =
            $crate::time_lines::gnss_resolve_week;

        c {
            /// A POSIX instant as a TAI64 or TAI64N label in the
            /// `tai64-posix-plus-10` convention, in lower-case hexadecimal, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The convention is daemontools' on a clock that keeps POSIX time:
            /// 2⁶² + 10 + the POSIX seconds, with no leap-second table, so POSIX 0
            /// is `400000000000000a`; it is not `hc_tai64_encode`'s true TAI.
            /// `format` is `tai64` or `tai64n`, in any case; `tai64na`, which the
            /// convention does not write, and anything else is `HC_ERROR_UNKNOWN`,
            /// and null `HC_ERROR_NULL_POINTER`. Attoseconds from 10¹⁸, or a second
            /// whose label would fall outside 0 to 2⁶³ − 1, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A POSIX instant as a TAI64 or TAI64N label in the
            /// `tai64-posix-plus-10` convention, in lower-case hexadecimal, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// The convention is the label daemontools' `tai64n` writes on a clock
            /// that keeps POSIX time: 2⁶² + 10 + the POSIX seconds, with no
            /// leap-second table, so POSIX 0 is `400000000000000a`. It is not
            /// `hc_tai64_encode`'s true TAI, and nothing in the bytes says which
            /// wrote them. `format` is `tai64` (16 digits, the second) or `tai64n`
            /// (24, the nanosecond), in any case; `tai64na`, which the convention
            /// does not write, and anything else is `HC_ERR_UNKNOWN`. Attoseconds
            /// from 10¹⁸, or a second whose label would fall outside 0 to 2⁶³ − 1,
            /// is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_tai64_posix_plus_10_encode(
            unix_seconds: i64,
            attoseconds: u64,
            format: name(format_len),
        ) -> line =
            $crate::time_lines::tai64_posix_plus_10_encode_line;

        c {
            /// The 60-bit UUID timestamp of a POSIX instant, and the time fields a
            /// version 1 and a version 6 UUID write it in, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// Tab-separated: the timestamp, the 100-nanosecond interval that
            /// contains the instant counted from 1582-10-15 00:00 UTC, the
            /// fraction below 100 ns dropped; then the first three groups of a
            /// version 1 UUID that carries it and of a version 6 one, RFC 9562's
            /// hex-and-dash form in lower case, such as `c232ab00-9414-11ec` and
            /// `1ec9414c-232a-6b00`, for the caller's clock sequence and node to
            /// follow. The instant is counted as POSIX time counts, with no leap
            /// second. Attoseconds from 10¹⁸, or an instant before 1582-10-15 or
            /// after the field's last interval on 5236-03-31, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The 60-bit UUID timestamp of a POSIX instant, and the time fields a
            /// version 1 and a version 6 UUID write it in, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the timestamp, the 100-nanosecond interval that
            /// contains the instant counted from 1582-10-15 00:00 UTC, the
            /// fraction below 100 ns dropped; then the first three groups of a
            /// version 1 UUID that carries it and of a version 6 one, RFC 9562's
            /// hex-and-dash form in lower case, such as `c232ab00-9414-11ec` and
            /// `1ec9414c-232a-6b00`, for the caller's clock sequence and node to
            /// follow. The instant is counted as POSIX time counts, with no leap
            /// second. Attoseconds from 10¹⁸, or an instant before 1582-10-15 or
            /// after the field's last interval on 5236-03-31, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_uuid_timestamp_encode(unix_seconds: i64, attoseconds: u64) -> line =
            $crate::time_lines::uuid_timestamp_encode_line;

        c {
            /// The NTP date and timestamp of a POSIX instant, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// Tab-separated: the era, 0 for 1900 to 2036; the era offset; the
            /// fraction in 2⁻⁶⁴ s units, floored; the 128-bit date in RFC 5905's
            /// Figure 3 layout, era, offset and fraction, as 32 lower-case
            /// hexadecimal digits; and the 64-bit timestamp of the packet headers,
            /// the offset and the top 32 bits of the fraction with the era
            /// dropped, as 16. The seconds count whole 86 400-second days from
            /// 1900, as RFC 5905 §6's table does, with no leap second. Attoseconds
            /// from 10¹⁸ is `HC_ERROR_OUT_OF_RANGE`, and a second so late that its
            /// count from 1900 leaves an `int64_t` is `HC_ERROR_OVERFLOW`. Writes
            /// the required length, including the terminator, into `written`.
        }
        wasm {
            /// The NTP date and timestamp of a POSIX instant, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the era, 0 for 1900 to 2036; the era offset; the
            /// fraction in 2⁻⁶⁴ s units, floored; the 128-bit date in RFC 5905's
            /// Figure 3 layout, era, offset and fraction, as 32 lower-case
            /// hexadecimal digits; and the 64-bit timestamp of the packet headers,
            /// the offset and the top 32 bits of the fraction with the era
            /// dropped, as 16. The seconds count whole 86 400-second days from
            /// 1900, as RFC 5905 §6's table does, with no leap second. Attoseconds
            /// from 10¹⁸, or a second so late that its count from 1900 leaves an
            /// `i64`, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length
            /// the text needs.
        }
        fn hc_ntp_encode(unix_seconds: i64, attoseconds: u64) -> line =
            $crate::time_lines::ntp_encode_line;

        c {
            /// The Swatch Internet Time at a POSIX instant, 0 through 999: the
            /// thousandth of the day of Biel Mean Time, UTC+1 all year, that it
            /// falls in, so @000 begins at 23:00 UTC.
            ///
            /// Attoseconds from 10¹⁸ are `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The Swatch Internet Time at a POSIX instant, 0 through 999, or an
            /// error sentinel.
            ///
            /// The day of Biel Mean Time, UTC+1 all year, in a thousand beats of
            /// 86.4 s: @000 begins at 23:00 UTC. Attoseconds from 10¹⁸ are
            /// `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_swatch_beat(unix_seconds: i64, attoseconds: u64) -> value(out_beat: u16) =
            $crate::time_lines::swatch_beat;

        c {
            /// TT(BIPM) at a TAI instant, read from a realisation the caller
            /// supplies, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `series` is the realisation as NUL-terminated text, one line per
            /// sample: the Modified Julian Date at 0 h UTC and TT(BIPMxx) − TAI −
            /// 32.184 s there in microseconds, separated by a tab, the dates
            /// ascending, as the first and third columns of the BIPM's `TTBIPM`
            /// files give them; text in any other shape is `HC_ERROR_MALFORMED`,
            /// and null `HC_ERROR_NULL_POINTER`. The line is the WebAssembly
            /// module's: TT(BIPMxx) − TT(TAI) in seconds, TT(BIPMxx) − TAI as whole
            /// seconds and attoseconds, and the TT(BIPMxx) reading of the instant
            /// as whole seconds and attoseconds. An instant outside the series, or
            /// an empty series, is `HC_ERROR_NO_DATA`; `strict` non-zero refuses a
            /// sample outside the leap-second table the same way. Attoseconds from
            /// 10¹⁸ are `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// TT(BIPM) at a TAI instant, read from a realisation the caller
            /// supplies, as one UTF-8 line, returning the byte length written.
            ///
            /// `series` is the realisation as text, one line per sample: the
            /// Modified Julian Date at 0 h UTC and TT(BIPMxx) − TAI − 32.184 s there
            /// in microseconds, separated by a tab, the dates ascending, as the
            /// first and third columns of the BIPM's `TTBIPM` files give them.
            /// Blank lines are skipped, and text in any other shape is
            /// `HC_ERR_MALFORMED`. The instant is whole seconds from
            /// 1970-01-01 00:00:00 TAI and attoseconds. Tab-separated: TT(BIPMxx) −
            /// TT(TAI) in seconds, interpolated linearly in TAI between the
            /// samples; TT(BIPMxx) − TAI as whole seconds and attoseconds; and the
            /// TT(BIPMxx) reading of the instant as whole seconds from
            /// 1970-01-01 00:00:00 of that scale and attoseconds. An instant before
            /// the first sample or after the last, or an empty series, is
            /// `HC_ERR_NO_DATA`: the series is never extrapolated. `strict`
            /// non-zero places the samples by the leap-second table and refuses one
            /// outside it with `HC_ERR_NO_DATA`; zero holds the table's ends.
            /// Attoseconds from 10¹⁸ are `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_tt_bipm(
            series: name(series_len),
            tai_seconds: i64,
            attoseconds: u64,
            strict: flag,
        ) -> line =
            $crate::time_lines::tt_bipm_line;

        c {
            /// .NET's `DateTime.Ticks` of a POSIX instant as a `Utc` value, the
            /// 100-nanosecond intervals from 0001-01-01 00:00, floored to the tick.
            ///
            /// Attoseconds from 10¹⁸, or an instant before 0001-01-01 or after
            /// 9999-12-31 23:59:59.9999999, are `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// .NET's `DateTime.Ticks` of a POSIX instant as a `Utc` value, or an
            /// error sentinel.
            ///
            /// The ticks are 100-nanosecond intervals from 0001-01-01 00:00,
            /// floored to the tick. Attoseconds from 10¹⁸, or an instant before
            /// 0001-01-01 or after 9999-12-31 23:59:59.9999999, are
            /// `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_dotnet_ticks_from_unix(unix_seconds: i64, attoseconds: u64) -> value(out_ticks: i64) =
            $crate::time_code_lines::dotnet_ticks_from_unix;

        c {
            /// The reading a count of .NET ticks names, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the whole seconds from
            /// 1970-01-01 00:00 and the attoseconds, POSIX time for a `Utc` value.
            /// Ticks outside 0 to 3 155 378 975 999 999 999 are
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The reading a count of .NET ticks names, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the whole seconds from 1970-01-01 00:00 and the
            /// attoseconds, POSIX time for a `Utc` value, and for a `Local` or
            /// `Unspecified` one the wall clock of a zone the value does not name.
            /// Ticks outside 0 to 3 155 378 975 999 999 999 are
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_unix_from_dotnet_ticks(ticks: i64) -> line =
            $crate::time_code_lines::unix_from_dotnet_ticks_line;

        c {
            /// A time of the civil day on a six-hour clock, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// `reckoning` is `ethiopian-hours` or `swahili-hours`, in any case;
            /// anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. `seconds_of_day` from 86 400 is
            /// `HC_ERROR_OUT_OF_RANGE`. The line is the WebAssembly module's.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// A time of the civil day on a six-hour clock, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// `reckoning` is `ethiopian-hours` or `swahili-hours`, in any case;
            /// anything else is `HC_ERR_UNKNOWN`. `seconds_of_day` is the time on
            /// the caller's wall clock, 0 to 86 399; a later one is
            /// `HC_ERR_OUT_OF_RANGE`. Tab-separated: the hour on the dial, 1 to 12;
            /// the minute and the second, the civil clock's; the half, `day` or
            /// `night`; and the part of the day the reckoning's source names, in
            /// its language and in English, empty for `ethiopian-hours`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_six_hour_clock(reckoning: name(reckoning_len), seconds_of_day: u32) -> line =
            $crate::time_code_lines::six_hour_clock_line;

        c {
            /// The civil time of day of a six-hour reading, as seconds after
            /// midnight, 0 through 86 399.
            ///
            /// `reckoning` is as for `hc_six_hour_clock`. An hour outside 1 to 12,
            /// or a minute or second above 59, is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The civil time of day of a six-hour reading, as seconds after
            /// midnight, 0 through 86 399, or an error sentinel.
            ///
            /// `reckoning` is as for `hc_six_hour_clock`. The reading is the hour
            /// on the dial, 1 to 12, the minute and second, and `night` non-zero
            /// for the night half. An hour outside 1 to 12, or a minute or second
            /// above 59, is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_civil_from_six_hour_clock(
            reckoning: name(reckoning_len),
            hour: u32,
            minute: u32,
            second: u32,
            night: flag,
        ) -> value(out_seconds: u32) =
            $crate::time_code_lines::civil_from_six_hour_clock;

        c {
            /// The French Republican decimal time of a time of the civil clock, as
            /// one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the decimal hour, minute and
            /// second and the rest of the decimal second in attoseconds. A second
            /// past 86 400, or attoseconds from 10¹⁸, is `HC_ERROR_OUT_OF_RANGE`,
            /// and 86 400, 23:59:60, `HC_ERROR_NO_DATA`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// The French Republican decimal time of a time of the civil clock, as
            /// one UTF-8 line, returning the byte length written.
            ///
            /// Article XI of the decree of 4 frimaire an II: ten decimal hours to
            /// the day from midnight, a hundred decimal minutes to the hour and a
            /// hundred decimal seconds to the minute, 0.864 s each, all exact.
            /// Tab-separated: the decimal hour, 0 to 9; the minute and the second,
            /// 0 to 99; and the rest of the decimal second in attoseconds of
            /// ordinary time. `seconds_of_day` is whole seconds after midnight and
            /// `attoseconds` the fraction. A second past 86 400, or attoseconds
            /// from 10¹⁸, is `HC_ERR_OUT_OF_RANGE`; 86 400, 23:59:60, a leap second
            /// the ten hours have no place for, is `HC_ERR_NO_DATA`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_french_decimal_time(seconds_of_day: u32, attoseconds: u64) -> line =
            $crate::time_lines::french_decimal_time_line;

        c {
            /// The civil time of day of a French Republican decimal time, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: whole seconds after midnight
            /// and attoseconds. An hour past 9, a minute or second past 99, or
            /// attoseconds of a decimal second or more is `HC_ERROR_OUT_OF_RANGE`.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// The civil time of day of a French Republican decimal time, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// `hc_french_decimal_time`'s inverse, exactly. Tab-separated: whole
            /// seconds after midnight and attoseconds. An hour past 9, a minute or
            /// second past 99, or attoseconds of a decimal second, 0.864 s, or more
            /// is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_civil_from_french_decimal_time(
            hour: u32,
            minute: u32,
            second: u32,
            attoseconds: u64,
        ) -> line =
            $crate::time_lines::civil_from_french_decimal_time_line;
    } };
    ("time-codes", $backend:ident) => { $backend! {
        c {
            /// A binary CCSDS time code read, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// `hex` is the code's octets, the P-field and exactly the T-field it
            /// announces, as hexadecimal; null is `HC_ERROR_NULL_POINTER`. The line
            /// is the WebAssembly module's: the code, `cuc`, `cds` or `ccs`, the
            /// TAI seconds and attoseconds, and the UTC label's POSIX second, leap
            /// flag and attoseconds, the other scale from the leap-second table.
            /// `strict` non-zero refuses an instant outside the table with
            /// `HC_ERROR_NO_DATA`. Text that is not a code is `HC_ERROR_MALFORMED`,
            /// and a Level 2, 3 or 4 code `HC_ERROR_NO_DATA`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// A binary CCSDS time code read, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// `hex` is the code's octets, the P-field and exactly the T-field it
            /// announces, as hexadecimal, two digits an octet, in either case.
            /// Tab-separated: the code, `cuc`, `cds` or `ccs`; the instant as TAI
            /// seconds from 1970-01-01 00:00:00 TAI and attoseconds; and its UTC
            /// label as the POSIX second, `1` for an inserted leap second or `0`,
            /// and attoseconds. CUC counts TAI and CDS and CCS count UTC; the other
            /// scale is from the leap-second table, and `strict` non-zero refuses
            /// an instant outside it with `HC_ERR_NO_DATA`, as does 23:59:60 past
            /// it. Text that is not a code, or fields out of their range, is
            /// `HC_ERR_MALFORMED`; a Level 2 code, whose epoch is its agency's, or
            /// a Level 3 or 4 code is `HC_ERR_NO_DATA`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_ccsds_decode(hex: name(hex_len), strict: flag) -> line =
            $crate::time_code_lines::ccsds_decode_line;

        c {
            /// The binary CCSDS time code of a TAI instant in the format a P-field
            /// names, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the code's octets in
            /// lower-case hexadecimal, the P-field first. CDS and CCS count the
            /// instant's UTC label, from the leap-second table under `strict`. A
            /// `p_field` that is not one P-field is `HC_ERROR_MALFORMED`, null
            /// `HC_ERROR_NULL_POINTER`, a Level 2, 3 or 4 format
            /// `HC_ERROR_NO_DATA`, and attoseconds from 10¹⁸ or an instant the
            /// format cannot count `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// The binary CCSDS time code of a TAI instant in the format a P-field
            /// names, as one UTF-8 line, returning the byte length written.
            ///
            /// The line is the code's octets in lower-case hexadecimal, the P-field
            /// first. A CUC code counts the TAI instant; CDS and CCS count its UTC
            /// label from the leap-second table, `strict` non-zero refusing an
            /// instant outside it with `HC_ERR_NO_DATA`. The finer part of the
            /// second is floored to the format's resolution. A `p_field` that is
            /// not one P-field is `HC_ERR_MALFORMED`, and a Level 2, 3 or 4 format
            /// `HC_ERR_NO_DATA`; attoseconds from 10¹⁸, or an instant the format
            /// cannot count — before 1958, past its last count, or outside the
            /// years 1 to 9999 — are `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns
            /// the length the text needs.
        }
        fn hc_ccsds_encode(
            tai_seconds: i64,
            attoseconds: u64,
            p_field: name(p_field_len),
            strict: flag,
        ) -> line =
            $crate::time_code_lines::ccsds_encode_line;

        c {
            /// A CCSDS ASCII time code, A or B, read, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the variation, the instant in
            /// the five cells of `hc_ccsds_decode`, the precision, the digits of
            /// the fraction and whether the terminator `Z` follows. Text that is not
            /// a code is `HC_ERROR_MALFORMED`, and null `HC_ERROR_NULL_POINTER`;
            /// `strict` is as for `hc_ccsds_decode`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// A CCSDS ASCII time code, A or B, read, as one UTF-8 line, returning
            /// the byte length written.
            ///
            /// Tab-separated: the variation, `a` or `b`; the instant, the start of
            /// the span a truncated code names, in the five cells of
            /// `hc_ccsds_decode`; how far the time part runs, `hour`, `minute`,
            /// `second` or `fraction`; the digits of the fraction, else empty; and
            /// `1` if the terminator `Z` follows, else `0`. Text that is not a
            /// code, a calendar or time subset alone included, is
            /// `HC_ERR_MALFORMED`; `strict` is as for `hc_ccsds_decode`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_ccsds_ascii_parse(code: name(code_len), strict: flag) -> line =
            $crate::time_code_lines::ccsds_ascii_parse_line;

        c {
            /// The CCSDS ASCII time code of a TAI instant's UTC label, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `variation` is `a` or `b` and `precision` `hour`, `minute`, `second`
            /// or the digits of the fraction, `1` to `18`, in any case; anything
            /// else is `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`.
            /// `terminator` non-zero ends the code with `Z`. The line is the
            /// WebAssembly module's. Attoseconds from 10¹⁸, or an instant outside
            /// the years 1 to 9999, are `HC_ERROR_OUT_OF_RANGE`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// The CCSDS ASCII time code of a TAI instant's UTC label, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// `variation` is `a`, `YYYY-MM-DDThh:mm:ss`, or `b`,
            /// `YYYY-DDDThh:mm:ss`; `precision` is `hour`, `minute`, `second`, or
            /// the digits of the fraction, `1` to `18`; either in any case, and
            /// anything else is `HC_ERR_UNKNOWN`. `terminator` non-zero ends the
            /// code with `Z`. The reading is truncated, never rounded, and the UTC
            /// label is from the leap-second table, as for `hc_ccsds_encode`.
            /// Attoseconds from 10¹⁸, or an instant outside the years 1 to 9999,
            /// are `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_ccsds_ascii_format(
            tai_seconds: i64,
            attoseconds: u64,
            variation: name(variation_len),
            precision: name(precision_len),
            terminator: flag,
            strict: flag,
        ) -> line =
            $crate::time_code_lines::ccsds_ascii_format_line;

        c {
            /// One minute's frame of a long-wave radio time code read, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `code` is `jjy`, `dcf77`, `wwvb-am` or `wwvb-pm`, in any case;
            /// anything else is `HC_ERROR_UNKNOWN`. `frame` is one character a
            /// second, `0`, `1` and for `jjy` and `wwvb-am` `M`; null for either is
            /// `HC_ERROR_NULL_POINTER`. `century` is a multiple of 100 from 0 to
            /// 9900; any other is `HC_ERROR_OUT_OF_RANGE`. The line is the
            /// WebAssembly module's. A frame that is not the code's is
            /// `HC_ERROR_MALFORMED`, and JJY's call-sign frame `HC_ERROR_NO_DATA`.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// One minute's frame of a long-wave radio time code read, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// `code` is `jjy`, `dcf77`, `wwvb-am` or `wwvb-pm`, in any case;
            /// anything else is `HC_ERR_UNKNOWN`. `frame` is one character a
            /// second: `0`, `1`, and for `jjy` and `wwvb-am` `M` for a marker.
            /// `century` is the first year of the century the two-digit year is
            /// in, a multiple of 100 from 0 to 9900; any other is
            /// `HC_ERR_OUT_OF_RANGE`. Tab-separated: the POSIX second of the minute
            /// the frame names (for `dcf77` the minute it announces); that minute's
            /// fixed day, hour and minute in the code's own time; the code's offset
            /// from UTC in hours; the frame's seconds; the leap second announced,
            /// `none`, `positive` or `negative`; the zone or summer-time state,
            /// `cet` or `cest` for `dcf77`, `standard`, `begins-today`,
            /// `in-effect` or `ends-today` for WWVB, empty for `jjy`; DCF77's A1,
            /// `1` when the zone changes at the end of the hour, else `0`, empty
            /// for the other codes; UT1 − UTC in tenths of a second for `wwvb-am`;
            /// and the phase code's six-bit `dst_next` word for `wwvb-pm`. A frame
            /// that is not the code's — a wrong length, symbol, BCD digit or
            /// parity, or a date that does not exist — is `HC_ERR_MALFORMED`, and
            /// JJY's call-sign frame, which carries no year, `HC_ERR_NO_DATA`. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_radio_decode(code: name(code_len), frame: name(frame_len), century: i64) -> line =
            $crate::time_code_lines::radio_decode_line;

        c {
            /// The frame of a long-wave radio time code for a minute, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The arguments are the WebAssembly module's, and `summer` may be null
            /// for the empty string, as `jjy` takes it; a null `code` is
            /// `HC_ERROR_NULL_POINTER`. A second that does not begin a minute of
            /// the years 1 to 9999, or a `leap`, `dut1_tenths` or `dst_next` the
            /// code cannot say, is `HC_ERROR_OUT_OF_RANGE`, and a `summer` state it
            /// does not name `HC_ERROR_UNKNOWN`. A `summer` of `zone:` and a zone's
            /// name reads the state, and for `wwvb-pm` the `dst_next` word, from
            /// the rules `hc_fixed_from_unix_in_zone` reads for it, as the
            /// WebAssembly module does, in a library built
            /// with `tz` too; without it, `HC_ERROR_UNKNOWN`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// The frame of a long-wave radio time code for a minute, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// `code` is as for `hc_radio_decode`, and the line is a frame it
            /// reads. `unix_seconds` begins the minute, a whole minute of the years
            /// 1 to 9999; for `dcf77` the frame is the one sent during the minute
            /// before, which announces it. `leap` is the leap second announced, 1
            /// inserted, −1 omitted, which only `jjy` and `wwvb-pm` can say, or 0.
            /// `summer` is DCF77's zone, `cet` or `cest`, or WWVB's summer-time
            /// state, `standard`, `begins-today`, `in-effect` or `ends-today`, and
            /// empty for `jjy`; another is `HC_ERR_UNKNOWN`. Or it is `zone:` and a
            /// zone's name, `zone:Europe/Berlin` for DCF77 or `zone:America/New_York`
            /// for WWVB (a zone the built-in table lacks, such as `America/Denver`,
            /// once `hc_zone_load` has its file), in a module built with `tz` too:
            /// the state is then read from the rules `hc_fixed_from_unix_in_zone`
            /// reads for the name —
            /// for `dcf77` Z1 Z2 from the offset and flag at the minute and A1 from
            /// a change between CET and CEST within the hour from it, in place of
            /// `zone_change`, and a minute at which the zone keeps neither is
            /// `HC_ERR_OUT_OF_RANGE`; for WWVB bit 57 from the flag at 24:00 UTC
            /// ending the minute's day and bit 58 from the flag at 00:00 UTC
            /// beginning it, and for `wwvb-pm` `dst_next` from the zone's next
            /// change after that day, in place of the caller's. A name the module
            /// does not know, `jjy`, and a module without `tz` are
            /// `HC_ERR_UNKNOWN`. `zone_change` is DCF77's A1, read by that code
            /// alone; `dut1_tenths` is WWVB's amplitude UT1 − UTC, −9 to 9;
            /// `dst_next` is the phase code's six-bit word, one its Table 8 lists
            /// for the direction `summer` gives. A second that does not begin a minute of those years, or a
            /// `leap`, `dut1_tenths` or `dst_next` outside what the code says, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_radio_encode(
            code: name(code_len),
            unix_seconds: i64,
            leap: int,
            summer: text(summer_len),
            zone_change: flag,
            dut1_tenths: int,
            dst_next: u32,
        ) -> line =
            $crate::time_code_lines::radio_encode;

        c {
            /// One frame of an IRIG serial time code read, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// `signal` is a signal designation of IRIG 200-16 such as `B124` and
            /// `frame` one character an index count, `0`, `1` and `M`, Pr first,
            /// as for the WebAssembly module's `hc_irig_decode`; null for either is
            /// `HC_ERROR_NULL_POINTER`, and a designation Table 4-1 does not permit
            /// `HC_ERROR_UNKNOWN`. A code with the year's two digits reads them in
            /// the century of `year`, and one without is read in `year`, 1 to 9999;
            /// any other is `HC_ERROR_OUT_OF_RANGE`. The line is the module's: the
            /// fixed day, the day of the year, the hour, minute, second and
            /// hundredths, the year's digits, the control bits and the straight
            /// binary seconds. A frame that is not the code's is
            /// `HC_ERROR_MALFORMED`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// One frame of an IRIG serial time code read, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// `signal` is a signal designation of IRIG Standard 200-16, a format
            /// letter, A, B, D, E, G or H, and three digits — modulation, carrier
            /// and coded expression — as `B124`, each digit one its Table 4-1
            /// permits the format, in any case; anything else is `HC_ERR_UNKNOWN`.
            /// The coded expression says which fields the frame carries: the year,
            /// the control bits, the straight binary seconds. `frame` is one
            /// character an index count, as the radio codes are written: `M` for a
            /// position identifier or the reference bit Pr, `0` and `1`, Pr first.
            /// A code that carries the year's two digits reads them in the century
            /// `year` is in; a code without them is read in `year`, 1 to 9999; any
            /// other is `HC_ERR_OUT_OF_RANGE`. Tab-separated: the fixed day; the day
            /// of the year; the hour, minute and second, 60 for a leap second, and
            /// the hundredths, 0 where the format does not send them; the year's
            /// two digits, the control bits as a number with control bit 1 lowest,
            /// and the straight binary seconds of the day, each empty for a code
            /// without them. The code carries no time scale, so the reading is of
            /// whatever clock the generator keeps. A frame that is not the code's —
            /// a wrong length or symbol, a BCD digit or time out of range, seconds
            /// that disagree, a day the year lacks, a year's digits not `year`'s,
            /// or a leap second not at the end of a month — is `HC_ERR_MALFORMED`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_irig_decode(signal: name(signal_len), frame: name(frame_len), year: i64) -> line =
            $crate::time_code_lines::irig_decode_line;

        c {
            /// The frame of an IRIG serial time code whose reference bit falls at a
            /// reading of the civil clock, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The arguments are the WebAssembly module's `hc_irig_encode`'s: the
            /// designation, a fixed day of the years 1 to 9999, whole seconds after
            /// its midnight, 86 400 being 23:59:60, hundredths from 0 to 99, and
            /// the control bits. A null `signal` is `HC_ERROR_NULL_POINTER`, and a
            /// designation not permitted `HC_ERROR_UNKNOWN`. A reading the format
            /// has no frame at, control bits it has no room for, or a value out of
            /// those ranges is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The frame of an IRIG serial time code whose reference bit falls at a
            /// reading of the civil clock, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// `signal` is as for `hc_irig_decode`, and the line is a frame it
            /// reads. The reading is a fixed day of the years 1 to 9999, whole
            /// seconds after its midnight, 86 400 being 23:59:60, and hundredths of
            /// a second, 0 to 99. `control` is the control bits, control bit 1
            /// lowest, 0 for a code without them. A reading the format has no frame
            /// at — for B one off the second, for D one off the hour — control bits
            /// the code has no room for, or a value out of those ranges is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_irig_encode(
            signal: name(signal_len),
            fixed: i64,
            seconds_of_day: u32,
            hundredths: u32,
            control: u32,
        ) -> line =
            $crate::time_code_lines::irig_encode_line;

        c {
            /// The reading at which the frame of an IRIG code that holds a reading
            /// of the civil clock begins, with the frame's length, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. A null `signal` is
            /// `HC_ERROR_NULL_POINTER`, and a designation not permitted
            /// `HC_ERROR_UNKNOWN`. Seconds past 86 400 or hundredths past 99 are
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The reading at which the frame of an IRIG code that holds a reading
            /// of the civil clock begins, with the frame's length, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// `signal` is as for `hc_irig_decode`. The frame begins at the last
            /// multiple of the format's frame length from midnight at or before the
            /// reading — a second for B, an hour for D — which is the reading
            /// `hc_irig_encode` takes, so a page need keep no frame lengths of its
            /// own. Tab-separated: the start as whole seconds after midnight and
            /// hundredths, and the frame's length in microseconds, Table 3-2's.
            /// 86 400 is 23:59:60, a frame of its own where the frame is a second
            /// or less, and within the day's last frame where it is longer. Seconds
            /// past 86 400 or hundredths past 99 are `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_irig_frame_start(signal: name(signal_len), seconds_of_day: u32, hundredths: u32) -> line =
            $crate::time_code_lines::irig_frame_start_line;

        c {
            /// Every IRIG format `hc_irig_decode` and `hc_irig_encode` read, with
            /// its frame's length, its rate and its fields, as NUL-terminated UTF-8
            /// lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Every IRIG format `hc_irig_decode` and `hc_irig_encode` read, with
            /// its frame's length, its rate and its fields, as UTF-8 lines,
            /// returning the byte length written.
            ///
            /// One line per format, A, B, D, E, G and H, tab-separated: its letter;
            /// the index count interval in microseconds, 1 000 for A; the index
            /// counts in a frame, 100 or 60; the frame's length in microseconds,
            /// whose multiples from midnight are the readings `hc_irig_encode`
            /// writes a frame at; the fields of the BCD time of year, most
            /// significant first, separated by spaces, the last being the frame's
            /// length; the control bits the format has room for; and the
            /// modulations, the carriers and the coded expressions Table 4-1 of
            /// IRIG 200-16 permits it, the three digits of a signal designation,
            /// each list separated by spaces. A null `buffer` returns the length
            /// the text needs.
        }
        fn hc_irig_formats() -> line = || Ok($crate::time_code_lines::irig_formats_lines());
    } };
    ("calendars", $backend:ident) => { $backend! {
        c {
            /// One fixed day in every registered calendar, as NUL-terminated UTF-8
            /// lines in a caller-owned buffer.
            ///
            /// One line per calendar, in registry order, tab-separated: the calendar
            /// identifier, its English name, the era code, the era's name in the
            /// locale (or the calendar's own name for it), the year, the month
            /// ordinal, `1` for a leap month, the month's name in the locale (or the
            /// calendar's own name for it), the day, `1` for a leap day, the
            /// calendar's extra fields as `name=value` pairs joined by `;`, the error
            /// code, the error name, the standing (`in-use`, `proleptic`, `extended`
            /// or `unrecorded`), where the calendar's day begins (`midnight`, `noon`,
            /// `sunset`, `sunrise`, `daybreak` — sunrise in summer and dawn in
            /// winter, as the medieval Icelandic day — or `local-time HH:MM:SS`),
            /// the date as the locale
            /// writes it (令和8年9月21日, 2023癸卯年闰二月初一), the locale used, and
            /// which civil day names a day that does not begin at midnight (`start`
            /// for the one it begins on, `end` for the one it ends on, empty for
            /// midnight). A
            /// calendar that refuses the day is still a line: its date columns,
            /// standing and formatted date are empty and the error code and name say
            /// why. `locale` is a NUL-terminated BCP 47 tag, `native` for each
            /// calendar's own language, or null; a calendar the tag's data does not
            /// name is rendered in English, else in the tag with the calendar's own
            /// names, never in the calendar's own language, and the last column
            /// says which. A tag that does not
            /// parse is the root locale `und`, whose month names are CLDR's
            /// `M01`..`M12` — ask for `en` for English. A `locale` that is not UTF-8
            /// is `HC_ERROR_NOT_UTF8`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// One fixed day in every registered calendar, as UTF-8 lines, returning
            /// the byte length written.
            ///
            /// One line per calendar, in registry order, tab-separated: the calendar
            /// identifier, its English name, the era code, the era's name in the
            /// locale (or the calendar's own name for it), the year, the month
            /// ordinal, `1` for a leap month, the month's name in the locale (or the
            /// calendar's own name for it), the day, `1` for a leap day, the
            /// calendar's extra fields as `name=value` pairs joined by `;`, the error
            /// code, the error name, the standing (`in-use`, `proleptic`, `extended`
            /// or `unrecorded`), where the calendar's day begins (`midnight`, `noon`,
            /// `sunset`, `sunrise`, `daybreak` — sunrise in summer and dawn in
            /// winter, as the medieval Icelandic day — or `local-time HH:MM:SS`),
            /// the date as the locale
            /// writes it (令和8年9月21日, 2023癸卯年闰二月初一), the locale used, and
            /// which civil day names a day that does not begin at midnight (`start`
            /// for the one it begins on, `end` for the one it ends on, empty for
            /// midnight). A
            /// calendar that refuses the day is still a line: its date columns,
            /// standing and formatted date are empty and the error code and name say
            /// why. `locale` is a BCP 47 tag, or `native` for each calendar's own
            /// language; a calendar the tag's data does not name is rendered in
            /// English, else in the tag with the calendar's own names, never in the
            /// calendar's own language, and the last column says which. A tag that does not parse is the root locale
            /// `und`, whose month names are CLDR's `M01`..`M12` — ask for `en` for
            /// English. A null `buffer` returns the length the text needs. The locale
            /// argument fails as `hc_parse_iso_date` does.
        }
        fn hc_describe_day(fixed: i64, locale: text(locale_len)) -> line =
            |fixed, locale| {
                Ok($crate::lines::describe_day(&$crate::registry(), $crate::Rd(fixed), locale))
            };

        c {
            /// The extra fields of one fixed day, as NUL-terminated UTF-8 lines in
            /// a caller-owned buffer.
            ///
            /// `id` is a NUL-terminated registry identifier, or null or empty for
            /// every registered calendar; an identifier the library does not know
            /// is `HC_ERROR_UNKNOWN`. One line per extra field, in registry order
            /// and in the order the calendar sets them, tab-separated: the
            /// calendar identifier, the field's identifier (`samvatsara`,
            /// `julian-day-number`: a key, never reader-facing text), its value as
            /// an integer, the field's label in the locale or else in English
            /// (`Samvatsara (southern reckoning)`, `Julian Day Number`), the value as a reader reads it
            /// — the name of the position it holds where its values are named,
            /// *Parabhava*, else the number in the locale's digits — `1` when the
            /// formatted date of `hc_describe_day` already writes the field, else
            /// `0`, and the locale used. A calendar that refuses the day, or whose
            /// date has no extra fields, writes no line. `locale` is as for
            /// `hc_describe_day`, `native` included. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The extra fields of one fixed day, as UTF-8 lines, returning the
            /// byte length written.
            ///
            /// `id` is a registry identifier, or empty for every registered
            /// calendar; an identifier the module does not know is
            /// `HC_ERR_UNKNOWN`. One line per extra field, in registry order and
            /// in the order the calendar sets them, tab-separated: the calendar
            /// identifier, the field's identifier (`samvatsara`,
            /// `julian-day-number`: a key, never reader-facing text), its value as
            /// an integer, the field's label in the locale or else in English
            /// (`Samvatsara (southern reckoning)`, `Julian Day Number`), the value
            /// as a reader reads it
            /// — the name of the position it holds where its values are named,
            /// *Parabhava*, else the number in the locale's digits — `1` when the
            /// formatted date of `hc_describe_day` already writes the field, else
            /// `0`, and the locale used. A calendar that refuses the day, or whose
            /// date has no extra fields, writes no line. `locale` is as for
            /// `hc_describe_day`, `native` included. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_day_extras(fixed: i64, id: opt(id_len), locale: text(locale_len)) -> line =
            |fixed, id, locale| {
                $crate::lines::day_extras(&$crate::registry(), $crate::Rd(fixed), id, locale)
            };

        c {
            /// The days from `from_fixed` up to but not including `to_fixed` as one
            /// calendar's eras, years, months or days, as NUL-terminated UTF-8 lines
            /// in a caller-owned buffer.
            ///
            /// `id` is a NUL-terminated registry identifier — `gregory`, `chinese`,
            /// `japanese` — and `unit` is `0` for eras, `1` for years, `2` for
            /// months and `3` for days; a null `id` is `HC_ERROR_NULL_POINTER` and an
            /// identifier or unit the library does not know is `HC_ERROR_UNKNOWN`.
            /// One line per span, in order and touching end to start, tab-separated:
            /// the first day of the span, the day after its last, its label in the
            /// locale (令和元年, `Adar I`, 閏二月, 初四), `1` for an intercalary unit,
            /// the standing of its first day, the error code, the error name and the
            /// locale used. The first and last spans are whole units and may reach
            /// outside the range asked for. A span the calendar refuses — days before
            /// its epoch or past its table, a unit it does not have — has an empty
            /// label, leap flag and standing and carries the refusal's code and name.
            /// An empty range writes an empty string, and a range of more than
            /// 100 000 spans, `hyper_calendar::lines`' `MAX_CALENDAR_UNITS`, is
            /// `HC_ERROR_OUT_OF_RANGE` with nothing written: the text would grow
            /// without bound, one line a unit, and a caller that wants more asks in
            /// pieces. `locale` is as for `hc_describe_day`, `native` included.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// The days from `from_fixed` up to but not including `to_fixed` as one
            /// calendar's eras, years, months or days, as UTF-8 lines, returning the
            /// byte length written.
            ///
            /// `id` is a registry identifier — `gregory`, `chinese`, `japanese` —
            /// and `unit` is `0` for eras, `1` for years, `2` for months and `3` for
            /// days; an identifier or unit the module does not know is
            /// `HC_ERR_UNKNOWN`. One line per span, in order and touching end to
            /// start, tab-separated: the first day of the span, the day after its
            /// last, its label in the locale (令和元年, `Adar I`, 閏二月, 初四), `1`
            /// for an intercalary unit, the standing of its first day, the error
            /// code, the error name and the locale used. The first and last spans
            /// are whole units and may reach outside the range asked for. A span the
            /// calendar refuses — days before its epoch or past its table, a unit it
            /// does not have — has an empty label, leap flag and standing and
            /// carries the refusal's code and name. An empty range writes nothing,
            /// and a range of more than 100 000 spans, `hyper_calendar::lines`'
            /// `MAX_CALENDAR_UNITS`, is `HC_ERR_OUT_OF_RANGE`: the text would grow
            /// without bound, one line a unit, and a caller that wants more asks in
            /// pieces. `locale` is as for `hc_describe_day`, `native` included. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_calendar_units(
            id: name(id_len),
            unit: u32,
            from_fixed: i64,
            to_fixed: i64,
            locale: text(locale_len),
        ) -> line =
            $crate::lines::calendar_units_by_id;

        c {
            /// A date as a locale writes it in one calendar, read back, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `calendar` is a NUL-terminated registry identifier; null is
            /// `HC_ERROR_NULL_POINTER` and one the registry does not carry
            /// `HC_ERROR_UNKNOWN`. `locale` is as for `hc_describe_day`, and the
            /// text is read in the locale that export's line for the calendar is
            /// written in; `text` is NUL-terminated, null being the empty text. The
            /// line is `hc_describe_day`'s for the calendar and the day the text
            /// names, then that fixed day. A text that is not one day is still a
            /// line: its date columns, standing, formatted date and fixed day are
            /// empty, and the error columns say why — `ambiguous` (103),
            /// `two-digit-year` (104), `year-not-written` (105),
            /// `weekday-mismatch` (106), `field-mismatch` (107),
            /// `not-recognised` (102) or `empty` (101),
            /// or the calendar's own code and name for fields it has no day for.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// A date as a locale writes it in one calendar, read back, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// `calendar` is a registry identifier, as `hc_calendar_units` takes
            /// it; one the registry does not carry is `HC_ERR_UNKNOWN`. `locale` is
            /// as for `hc_describe_day`, and the text is read in the locale that
            /// export's line for the calendar is written in, so that what its
            /// formatted date writes, this reads: 令和8年9月28日, *28 Eylül 2026*,
            /// ٢٨ سبتمبر ٢٠٢٦, and a reader's own spelling of them — a name at any
            /// width, Latin digits, Han numerals and 元年, a weekday that must be
            /// the day's. The line is `hc_describe_day`'s for the calendar and the
            /// day the text names, then that fixed day. A text that is not one day
            /// is still a line: its date columns, standing, formatted date and
            /// fixed day are empty, and the error columns say why — `ambiguous`
            /// (103), `two-digit-year` (104), `year-not-written` (105),
            /// `weekday-mismatch` (106), `field-mismatch` (107),
            /// `not-recognised` (102) or `empty` (101),
            /// or the calendar's own code and name for fields it has no day for.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_parse_date(
            calendar: name(calendar_len),
            locale: text(locale_len),
            text: text(text_len),
        ) -> line =
            $crate::lines::parse_date;

        c {
            /// Every registered calendar, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// One line per calendar, in registry order, tab-separated: the
            /// identifier, what the locale calls the calendar (和暦, or empty where
            /// it has no name), its English name, the earliest and latest fixed days
            /// it converts (empty where unbounded), whether it has eras, years,
            /// months and days as `1` or `0` each, the languages its sources are
            /// written in as BCP 47 tags joined by `;` (empty for a day count, a
            /// proposal or the Gregorian family), and its standing on `today`
            /// (`in-use`, `proleptic`, `extended` or `unrecorded`). `locale` is as
            /// for `hc_describe_day`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Every registered calendar, as UTF-8 lines, returning the byte length
            /// written.
            ///
            /// One line per calendar, in registry order, tab-separated: the
            /// identifier, what the locale calls the calendar (和暦, or empty where
            /// it has no name), its English name, the earliest and latest fixed days
            /// it converts (empty where unbounded), whether it has eras, years,
            /// months and days as `1` or `0` each, the languages its sources are
            /// written in as BCP 47 tags joined by `;` (empty for a day count, a
            /// proposal or the Gregorian family), and its standing on `today`
            /// (`in-use`, `proleptic`, `extended` or `unrecorded`). `locale` is as
            /// for `hc_describe_day`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_calendars(today: i64, locale: text(locale_len)) -> line =
            |today, locale| {
                Ok($crate::lines::calendars(&$crate::registry(), $crate::Rd(today), locale))
            };

        c {
            /// Every registered calendar by name alone, as NUL-terminated UTF-8
            /// lines in a caller-owned buffer.
            ///
            /// One line per calendar, in registry order, tab-separated: the
            /// identifier, what the locale calls the calendar (和暦, or empty where
            /// it has no name), its English name, the locale used (the tag of the
            /// data entry the name came from, empty where the name is), the crate
            /// that registers it (`hc-calendars-solar`, `hc-calendars-lunar`,
            /// `hc-calendars-equinox`, `hc-calendars-indic` or
            /// `hc-calendars-regional`), and the languages of its sources as
            /// `hc_calendars` gives them, BCP 47 tags joined by `;` or empty, so
            /// that a menu can list a reader's own calendars first. The names are
            /// `hc_calendars`', without the
            /// range, the units and the standing on a day, so nothing is converted:
            /// for a menu of calendars, which is asked for far more often than a
            /// day is described. `locale` is as for `hc_describe_day`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// Every registered calendar by name alone, as UTF-8 lines, returning
            /// the byte length written.
            ///
            /// One line per calendar, in registry order, tab-separated: the
            /// identifier, what the locale calls the calendar (和暦, or empty where
            /// it has no name), its English name, the locale used (the tag of the
            /// data entry the name came from, empty where the name is), the crate
            /// that registers it (`hc-calendars-solar`, `hc-calendars-lunar`,
            /// `hc-calendars-equinox`, `hc-calendars-indic` or
            /// `hc-calendars-regional`), and the languages of its sources as
            /// `hc_calendars` gives them, BCP 47 tags joined by `;` or empty, so
            /// that a menu can list a reader's own calendars first. The names are
            /// `hc_calendars`', without the
            /// range, the units and the standing on a day, so nothing is converted:
            /// for a menu of calendars, which a page asks for far more often than it
            /// describes a day. `locale` is as for `hc_describe_day`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_calendar_list(locale: text(locale_len)) -> line =
            |locale| Ok($crate::lines::calendar_list(&$crate::registry(), locale));

        c {
            /// Every locale the library carries, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// One line per locale, in tag order, tab-separated: the BCP 47 tag, the
            /// language's name in English and in itself, whether the locale's own
            /// data names the Gregorian months, the weekdays and the Gregorian eras
            /// as `1` or `0` each, and the identifiers of the calendars it has
            /// vocabulary of its own for beyond the shared Gregorian months, joined
            /// by `;`. Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// Every locale the module carries, as UTF-8 lines, returning the byte
            /// length written.
            ///
            /// One line per locale, in tag order, tab-separated: the BCP 47 tag, the
            /// language's name in English and in itself, whether the locale's own
            /// data names the Gregorian months, the weekdays and the Gregorian eras
            /// as `1` or `0` each, and the identifiers of the calendars it has
            /// vocabulary of its own for beyond the shared Gregorian months, joined
            /// by `;`. A null `buffer` returns the length the text needs.
        }
        fn hc_locales() -> line = || Ok($crate::lines::locales());

        c {
            /// The ISO 8601 weekday of the first day of the week in a locale, Monday
            /// = 1 through Sunday = 7.
            ///
            /// `locale` is a NUL-terminated BCP 47 tag or null, read as `hc-i18n`
            /// reads CLDR 48's week data: a `-u-fw-` key first, then the tag's
            /// region (`en-US` is 7, `en-GB` 1), then, for a tag without a region,
            /// the region its language's likely subtags give (`ja` is 7, `fr` 1).
            /// A null `locale`, a tag that does not parse, and `native` are the
            /// root locale `und`, whose week begins on the world's Monday. A
            /// `locale` that is not UTF-8 is `HC_ERROR_NOT_UTF8`.
        }
        wasm {
            /// The ISO weekday of the first day of the week in a locale, Monday = 1
            /// through Sunday = 7, or an error sentinel.
            ///
            /// `locale` is a BCP 47 tag, read as `hc-i18n` reads CLDR 48's week
            /// data: a `-u-fw-` key first, then the tag's region (`en-US` is 7,
            /// `en-GB` 1), then, for a tag without a region, the region its
            /// language's likely subtags give (`ja` is 7, `fr` 1). A tag that does
            /// not parse, and `native`, are the root locale `und`, whose week
            /// begins on the world's Monday. The locale argument fails as
            /// `hc_parse_iso_date` does; the answer is an `i64`, as every export
            /// that can return a sentinel is.
        }
        fn hc_first_day_of_week(locale: text(locale_len)) -> value(out_weekday: u8) =
            |locale| Ok($crate::lines::first_day_of_week(locale));

        c {
            /// The day periods of a time of the civil clock in a locale, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `locale` is a NUL-terminated
            /// BCP 47 tag, or null for the root locale. `seconds_of_day` from
            /// 86 400 is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The day periods of a time of the civil clock in a locale, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// From CLDR 48's `dayPeriods` rules and names. Tab-separated: `am` or
            /// `pm`, and the locale's abbreviated name for it in a date's format
            /// context, *PM*; the period the locale's rules give the minute,
            /// `midnight` or `noon` at 00:00:00 or 12:00:00 where the language has
            /// a word for it, else `morning1` to `night2`, empty where the language
            /// has no rules; that period's abbreviated, wide and narrow names, *in
            /// the afternoon*; and the tag of the locale data that answered.
            /// `seconds_of_day` from 86 400 is `HC_ERR_OUT_OF_RANGE`; the locale
            /// argument fails as for `hc_parse_iso_date`. A null `buffer` returns
            /// the length the text needs.
        }
        fn hc_day_period(seconds_of_day: u32, locale: text(locale_len)) -> line =
            $crate::i18n_lines::day_period_line;

        c {
            /// An integer written in a numbering system, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `system` is a CLDR numbering
            /// system `hc_numbering_systems` lists; another is `HC_ERROR_UNKNOWN`,
            /// and null `HC_ERROR_NULL_POINTER`. A value the system cannot write is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// An integer written in a numbering system, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// `system` is a CLDR numbering system `hc_numbering_systems` lists: a
            /// positional one, `latn`, `arab`, `deva`, whose digits replace the
            /// Latin ones; or an algorithmic one, `hebr`'s Hebrew numerals with
            /// their geresh and gershayim, `grek` and `greklow`'s Greek with the
            /// keraia, or the Han styles; another is `HC_ERR_UNKNOWN`.
            /// Tab-separated: the text and the system's identifier. A value the
            /// system cannot write, a Hebrew numeral of nothing among them, is
            /// `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_format_number(system: name(system_len), value: i64) -> line =
            $crate::i18n_lines::format_number_line;

        c {
            /// An integer read back out of a numbering system's notation.
            ///
            /// `system` is as for `hc_format_number`, and `text` is NUL-terminated;
            /// null for either is `HC_ERROR_NULL_POINTER`. Text that is not a
            /// number in the system is `HC_ERROR_MALFORMED`, and one it cannot
            /// hold `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// An integer read back out of a numbering system's notation, or an
            /// error sentinel.
            ///
            /// `system` is as for `hc_format_number`; a system not named is
            /// `HC_ERR_UNKNOWN`. Text that is not a number in the system is
            /// `HC_ERR_MALFORMED`, and a number it cannot hold, or one at or below
            /// the error floor, `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_parse_number(system: name(system_len), text: name(text_len)) -> value(out_value: i64) =
            $crate::i18n_lines::parse_number;

        c {
            /// Every numbering system `hc_format_number` writes, as NUL-terminated
            /// UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Every numbering system `hc_format_number` writes, as UTF-8 lines,
            /// returning the byte length written.
            ///
            /// One line per system, in `hc-i18n`'s order, tab-separated: the CLDR
            /// identifier; `1` for an algorithmic system that spells numbers out,
            /// `0` for a positional one; and a positional system's ten digits, zero
            /// first, empty for an algorithmic one. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_numbering_systems() -> line = || Ok($crate::i18n_lines::numbering_systems_lines());

        c {
            /// The eras a calendar is described with, named in a locale, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `calendar` is a registry
            /// identifier; one the registry does not carry is `HC_ERROR_UNKNOWN`,
            /// and null `HC_ERROR_NULL_POINTER`; a calendar whose eras no locale
            /// data lists is `HC_ERROR_NO_DATA`. `locale` is as for
            /// `hc_describe_day`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The eras a calendar is described with, named in a locale, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// One line per era, in the order the locale data lists them, the
            /// calendar's first era first, tab-separated: the era's code, as a
            /// date's era field writes it (`reiwa`, `ce`); its wide, abbreviated
            /// and narrow names in the locale, each empty where the locale's chain
            /// has none; the calendar's identifier; and the tag of the locale data
            /// that answered. A calendar the registry does not carry is
            /// `HC_ERR_UNKNOWN`, and one whose eras no locale data lists
            /// `HC_ERR_NO_DATA`. `locale` is as for `hc_describe_day`, `native`
            /// included. A null `buffer` returns the length the text needs.
        }
        fn hc_calendar_eras(calendar: name(calendar_len), locale: text(locale_len)) -> line =
            $crate::i18n_lines::calendar_eras_lines;

        c {
            /// The steps by which a country adopted the Gregorian calendar, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// `region` is a NUL-terminated ISO 3166-1 alpha-2 code, in either
            /// case; a null `region` is `HC_ERROR_NULL_POINTER` and one that is not
            /// UTF-8 is `HC_ERROR_NOT_UTF8`. One line per step, oldest first,
            /// tab-separated: the last day of the old reckoning and the first day
            /// of the new as fixed days, the old calendar's registry identifier
            /// (`julian`, `japanese-tenpo`, `dangi`, `chinese`, `islamic-umalqura`,
            /// `rumi`, `swedish-1700`), the scope (`civil` for the civil calendar
            /// of the whole polity as it then was, `partial` for part of the
            /// country, some purposes or part of the calendar, and `ecclesiastical`
            /// for a church's calendar alone), the instrument behind the step with
            /// its date and whether it was read, the new calendar's identifier
            /// (`gregory`, except for Sweden's steps of 1700 to `swedish-1700` and
            /// of 1712 back to `julian`), and who took the step, in English. A
            /// staged adoption is several lines — China in 1912 and 1929, Sweden in
            /// 1700, 1712 and 1753, the Dutch provinces — and a code the library
            /// does not know writes an empty string, which is not a claim that the
            /// country never adopted the calendar. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The steps by which a country adopted the Gregorian calendar, as
            /// UTF-8 lines, returning the byte length written.
            ///
            /// `region` is an ISO 3166-1 alpha-2 code, in either case. One line per
            /// step, oldest first, tab-separated: the last day of the old reckoning
            /// and the first day of the new as fixed days, the old calendar's
            /// registry identifier (`julian`, `japanese-tenpo`, `dangi`, `chinese`,
            /// `islamic-umalqura`, `rumi`, `swedish-1700`), the scope (`civil` for
            /// the civil calendar of the whole polity as it then was, `partial` for
            /// part of the country, some purposes or part of the calendar, and
            /// `ecclesiastical` for a church's calendar alone), the instrument
            /// behind the step with its date and whether it was read, the new
            /// calendar's identifier (`gregory`, except for Sweden's steps of 1700
            /// to `swedish-1700` and of 1712 back to `julian`), and who took the
            /// step, in English. A staged adoption is several lines — China in 1912
            /// and 1929, Sweden in 1700, 1712 and 1753, the Dutch provinces — and a
            /// code the table does not know writes nothing, which is not a claim
            /// that the country never adopted the calendar. The region argument
            /// fails as `hc_parse_iso_date` does. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_gregorian_adoption(region: name(region_len)) -> line =
            |region| Ok($crate::lines::gregorian_adoption(region));

        c {
            /// Which month and weekday names a locale writes for a calendar on a
            /// fixed day, where a government renamed them for a period, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `calendar` is a NUL-terminated registry identifier; null is
            /// `HC_ERROR_NULL_POINTER` and one the registry does not carry
            /// `HC_ERROR_UNKNOWN`. `locale` is as for `hc_describe_day`. The line is
            /// the WebAssembly module's: `in-force`, `undecided` or `ordinary`, then
            /// for a period its identifier, its names for the day's month and
            /// weekday, the weekday name's English meaning, the three fixed days
            /// that bound it, and its sources. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Which month and weekday names a locale writes for a calendar on a
            /// fixed day, where a government renamed them for a period, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// `calendar` is a registry identifier, as `hc_calendar_units` takes
            /// it; one the registry does not carry is `HC_ERR_UNKNOWN`. `locale` is a BCP 47 tag, read
            /// as `hc_describe_day` reads it; `native` names no one language and
            /// takes no period. Tab-separated: `in-force` when a period's names
            /// were in force on the day, `undecided` when a period applies and no
            /// source read says whether it was yet in force, or `ordinary` when
            /// none applies and the locale's own names hold; then, for a period,
            /// its identifier, its name for the day's month and for the day's
            /// weekday, the English meaning of that weekday name, the first day the
            /// names can have been in force, the first day by which every source
            /// read has them in force and the first day the old names were back, as
            /// fixed days, and its sources. For `ordinary` the other eight cells are
            /// empty. The one period carried is Turkmenistan's, `turkmen-2002`, for
            /// the Gregorian calendar in Turkmen. The text arguments fail as
            /// `hc_parse_iso_date`'s does. A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_naming_period_on(
            calendar: name(calendar_len),
            fixed: i64,
            locale: text(locale_len),
        ) -> line =
            |calendar, fixed, locale| {
                $crate::lines::naming_period_line(&$crate::registry(), locale, calendar, fixed)
            };

        c {
            /// The yoga and the karaṇa in progress at a POSIX timestamp, as two
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's: the limb, its number, its
            /// English and Devanagari names, the instants it began and ends and the
            /// instant it was read at, and the sky of the yoga by its identifier,
            /// which this call reads back, and by its full name. `ayanamsa` is an
            /// identifier, `lahiri`, `raman`, `krishnamurti`, `reingold-dershowitz`
            /// or `fagan-bradley`, for the true Sun and Moon in that zodiac; or
            /// `surya-siddhanta`, for the *Sūrya Siddhānta*'s, named on the karaṇa's
            /// line too; in any case. Anything else, a full name such as
            /// `Lahiri (Chitrapaksha)` included, is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. An instant outside the years −1000 to 3000,
            /// or on the Siddhānta's sky outside the days of Kali Yuga 1 to 10 000,
            /// is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// The yoga and the karaṇa in progress at a POSIX timestamp, as two
            /// UTF-8 lines, returning the byte length written.
            ///
            /// The yoga's line first, then the karaṇa's, tab-separated alike: the
            /// limb (`yoga` or `karana`), its number (the yoga 1 for Viṣkambha
            /// through 27, the karaṇa the half-tithi 1 through 60), its name as
            /// Drik Panchang spells it in English and in Devanagari, the instants
            /// it began and ends and the instant it was read at as whole POSIX
            /// seconds, rounded down, in Universal Time, and the sky the yoga was
            /// reckoned on, by the identifier `ayanamsa` takes and by its full
            /// name. `ayanamsa` is an identifier, `lahiri`, `raman`, `krishnamurti`,
            /// `reingold-dershowitz` or `fagan-bradley`, for the true Sun and Moon
            /// in that zodiac, where the karaṇa's last two cells are empty, since
            /// it needs none; or `surya-siddhanta`, for the *Sūrya Siddhānta*'s Sun
            /// and Moon, as `hindu-lunar-surya-siddhanta` reads them, where both
            /// lines name the sky, `surya-siddhanta` and `Sūrya Siddhānta`, since
            /// the book's karaṇa is its own; in any case. Anything else, the empty
            /// string and a full name such as `Lahiri (Chitrapaksha)` included, is
            /// `HC_ERR_UNKNOWN`. An instant outside the years −1000 to 3000, or on
            /// the Siddhānta's sky outside the days of Kali Yuga 1 to 10 000, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_panchanga_at(unix_seconds: i64, ayanamsa: name(ayanamsa_len)) -> line =
            $crate::panchanga_lines::panchanga_at_lines;

        c {
            /// The yoga and the karaṇa a fixed day carries at a place, the ones in
            /// progress at its sunrise, as two NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The lines of `hc_panchanga_at`, read at the day's sunrise at the
            /// latitude and longitude in degrees and the elevation in metres, or on
            /// the Siddhānta's sky at its own sunrise. A day on which the Sun does
            /// not rise there is `HC_ERROR_NO_DATA`; a place off the globe, a day
            /// outside the years −1000 to 3000, and on the Siddhānta's sky a place
            /// beyond 65° of latitude or a day outside Kali Yuga 1 to 10 000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The yoga and the karaṇa a fixed day carries at a place, the ones in
            /// progress at its sunrise, as two UTF-8 lines, returning the byte
            /// length written.
            ///
            /// The lines of `hc_panchanga_at`, read at the day's sunrise at the
            /// latitude and longitude in degrees, north and east positive, and the
            /// elevation in metres. A day on which the Sun does not rise there is
            /// `HC_ERR_NO_DATA`: no other moment is put in its place. On the
            /// Siddhānta's sky the day is read at the book's own sunrise, which a
            /// place within 65° of latitude has every day. A place off the globe, a
            /// day outside the years −1000 to 3000, and on the Siddhānta's sky a
            /// place beyond 65° or a day outside Kali Yuga 1 to 10 000, is
            /// `HC_ERR_OUT_OF_RANGE`; `ayanamsa` is as for `hc_panchanga_at`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_panchanga_of_day(
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            ayanamsa: name(ayanamsa_len),
        ) -> line =
            |fixed, latitude, longitude, elevation, ayanamsa| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::panchanga_lines::panchanga_of_day_lines(fixed, place, ayanamsa)
                    })
            };

        c {
            /// The Hindu lunisolar date of a fixed day at a place, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the Śaka year, the Vikrama
            /// year, the month, 1 or 0 for the intercalary month, the tithi, 1 or 0
            /// for a repeated tithi, the sunrise the day was read at as POSIX
            /// seconds; then, in the `locale`, the month's name, the locale's word
            /// for an intercalary month where it is one, the Śaka and Vikrama
            /// eras' names, and the tag of the data that answered. `locale` is a
            /// NUL-terminated BCP 47 tag, `native` for the calendar's own
            /// languages, or null, as for `hc_describe_day`; one that is not UTF-8
            /// is `HC_ERROR_NOT_UTF8`. `sky` is an ayanāṃśa `hc_panchanga_at` names, for the true
            /// Sun and Moon, or `surya-siddhanta`, for the *Sūrya Siddhānta*'s, in
            /// any case; anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. A place beyond 65° of latitude, where
            /// some day of the year has no sunrise, is `HC_ERROR_OUT_OF_RANGE` on
            /// either sky, as is a place off the globe, a day outside Śaka 1622
            /// through 2221 on the true sky, Chaitra śukla 1 in March 1700 to the
            /// eve of the one in March 2300, and a day outside Kali Yuga 1 to
            /// 10 000 on the Siddhānta's. On the true sky, a day whose sunrise at
            /// the place, or a search that reads it, the model does not find is
            /// `HC_ERROR_NO_DATA`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// The Hindu lunisolar date of a fixed day at a place, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// The amānta date read at the place's sunrise, tab-separated: the Śaka
            /// year, the Vikrama year, the month (1 for Chaitra through 12 for
            /// Phālguna), 1 if it is the intercalary month and 0 if not, the tithi
            /// (1 through 30), 1 if the day is the second to carry it and 0 if not,
            /// and the sunrise it was read at as whole POSIX seconds of Universal
            /// Time, rounded down; then, in the locale, the month's name (`Bhadra`,
            /// भाद्रपद under `hi`), with the locale's word for an intercalary month
            /// before it where it is one (`Adhika Sravana`); that word alone for an
            /// intercalary month, else empty; the Śaka era's name; the Vikrama
            /// Saṃvat's; and the tag of the data that answered. The names are
            /// `hc_describe_day`'s for `hindu-lunar` and resolve the locale as it
            /// does, `native` asking for Sanskrit; a name the locale's data does
            /// not have, such as either era in Sanskrit, is an empty cell. The
            /// locale argument fails as `hc_parse_iso_date` does. `sky` is an
            /// ayanāṃśa `hc_panchanga_at` names, for
            /// the true Sun and Moon in its zodiac, as `hindu-lunar` reads them with
            /// Lahiri's at the Central Station; or `surya-siddhanta`, for the
            /// *Sūrya Siddhānta*'s Sun and Moon at its own sunrise, as
            /// `hindu-lunar-surya-siddhanta` reads them at Ujjain; in any case, and
            /// anything else, the empty string included, is `HC_ERR_UNKNOWN`. The
            /// place is the latitude and longitude in degrees, north and east
            /// positive, and the elevation in metres. A place beyond 65° of
            /// latitude, where some day of the year has no sunrise, is
            /// `HC_ERR_OUT_OF_RANGE` on either sky, as is a day outside Śaka 1622
            /// through 2221 on the true sky, Chaitra śukla 1 in March 1700 to the
            /// eve of the one in March 2300, and outside Kali Yuga 1 to 10 000 on
            /// the Siddhānta's. A place off the globe is
            /// `HC_ERR_OUT_OF_RANGE`. On the true sky, a day whose sunrise at the
            /// place, or a search that reads it, the model does not find is
            /// `HC_ERR_NO_DATA`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_hindu_lunar_date(
            sky: name(sky_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            locale: text(locale_len),
        ) -> line =
            |sky, fixed, latitude, longitude, elevation, locale| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::hindu_lines::hindu_lunar_date_line(sky, fixed, place, locale)
                    })
            };

        c {
            /// The *Sūrya Siddhānta*'s Sun and Moon at a POSIX timestamp, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the Sun's and the Moon's
            /// sidereal longitudes, the elongation, the tithi and the Sun's sign.
            /// An instant outside the days of Kali Yuga 1 to 10 000 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The *Sūrya Siddhānta*'s Sun and Moon at a POSIX timestamp, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the Sun's and the Moon's sidereal longitudes in
            /// degrees, the Moon's elongation from the Sun in degrees, 0 to 360,
            /// the tithi in progress (1 through 30) and the sign the Sun is in (1
            /// for Meṣa through 12 for Mīna). The instant is read as Universal
            /// Time. An instant outside the days of Kali Yuga 1 to 10 000, 3101 BCE
            /// to 6900 CE, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_surya_siddhanta_at(unix_seconds: i64) -> line =
            $crate::hindu_lines::surya_siddhanta_line;

        c {
            /// The *Sūrya Siddhānta*'s sunrise on a fixed day at a place, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The one cell is the instant as POSIX seconds, rounded down. A day
            /// outside Kali Yuga 1 to 10 000, a place beyond 65° of latitude, or one
            /// off the globe, is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// The *Sūrya Siddhānta*'s sunrise on a fixed day at a place, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// The one cell is the instant as whole POSIX seconds of Universal
            /// Time, rounded down. The Siddhānta reads the latitude and the
            /// longitude, in degrees, north and east positive, and no height. A
            /// day outside Kali Yuga 1 to 10 000, a place beyond 65° of latitude,
            /// or one off the globe, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_surya_siddhanta_sunrise(fixed: i64, latitude: f64, longitude: f64) -> line =
            |fixed, latitude, longitude| {
                $crate::astro_lines::location(latitude, longitude, 0.0)
                    .and_then(|place| {
                        $crate::hindu_lines::surya_siddhanta_sunrise_line(fixed, place)
                    })
            };

        c {
            /// Whether the young crescent should have been visible on the evening
            /// that begins a fixed day, from a place, by a named criterion, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: 1 or 0, the moment the evening
            /// is judged at as POSIX seconds, and the Moon's longitude less the
            /// Sun's (0 to 360), arc of light, altitude and arc of vision and the crescent's width there,
            /// empty where there is no such moment. `criterion` is `shaukat`,
            /// `yallop`, `saudi-rule`, `odeh`, `istanbul-2016`, `khgt`,
            /// `mabims-2021-topocentric` or `mabims-2021-geocentric-elongation`, in
            /// any case; anything else is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. A place off the
            /// globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Whether the young crescent should have been visible on the evening
            /// that begins a fixed day, from a place, by a named criterion, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// `criterion` is `shaukat`, `yallop`, `saudi-rule`, `odeh`,
            /// `istanbul-2016`, `khgt`, `mabims-2021-topocentric` or
            /// `mabims-2021-geocentric-elongation`, in any case;
            /// anything else is `HC_ERR_UNKNOWN`. The evening is the one before
            /// `fixed`, since the day begins at sunset. Tab-separated: 1 if the
            /// crescent passes and 0 if not; the moment the criterion judges the
            /// evening at, as whole POSIX seconds of Universal Time, rounded down;
            /// and at that moment the Moon's longitude less the Sun's, 0 to 360,
            /// its arc of light, its geocentric altitude and the arc of vision, in
            /// degrees, and the
            /// crescent's topocentric width in arcminutes. Where there is no such
            /// moment the first cell is 0 and the rest are empty. The place is as
            /// for `hc_hindu_lunar_date`. A place off the globe, or a day outside
            /// the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_crescent_visible(
            criterion: name(criterion_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |criterion, fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::crescent_lines::crescent_line(criterion, fixed, place)
                    })
            };

        c {
            /// The number of the modern Olympiad a Gregorian year belongs to.
            ///
            /// 1 for 1896–1899, under the Olympic Charter's definition, whether or
            /// not its Games were held; a year before 1896 is
            /// `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The number of the modern Olympiad a Gregorian year belongs to, or an
            /// error sentinel.
            ///
            /// 1 for 1896–1899, under the Olympic Charter's definition, whether or
            /// not its Games were held; a year before 1896 is
            /// `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_ioc_olympiad(gregorian_year: i64) -> value(out_olympiad: i64) =
            $crate::calendar_values::ioc_olympiad;

        c {
            /// The modern Olympiad a fixed day belongs to, by the Olympic Charter
            /// in force on that day.
            ///
            /// A day before the opening of 6 April 1896 is `HC_ERROR_OUT_OF_RANGE`,
            /// and one from 10 June to 21 November 1956 `HC_ERROR_NO_DATA`.
        }
        wasm {
            /// The modern Olympiad a fixed day belongs to, by the Olympic Charter
            /// in force on that day, or an error sentinel.
            ///
            /// Before 1 September 2004 the Charter ran an Olympiad from the opening
            /// of one Games to the opening of the next, Olympedia's dates; from
            /// then, by `hc_ioc_olympiad`'s Gregorian year. A day before the opening
            /// of 6 April 1896 is `HC_ERR_OUT_OF_RANGE`; one from 10 June to 21
            /// November 1956, the XV Olympiad if the Melbourne Games opened the XVI
            /// and the XVI if the Stockholm equestrian Games did, is
            /// `HC_ERR_NO_DATA`.
        }
        fn hc_ioc_olympiad_on(fixed: i64) -> value(out_olympiad: i64) =
            $crate::calendar_values::ioc_olympiad_on;

        c {
            /// The king and regnal year labelling a Seleucid year, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. A year outside SE −314 to 160
            /// is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// The king and regnal year labelling a Seleucid year, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// As van Gent's converter of Parker and Dubberstein's table labels it:
            /// SE −314 is 1 Interregnum, the accession year of Nabopolassar, and SE
            /// 1 is 1 Seleucus I Nicator. Tab-separated: the king, in English, and
            /// the regnal year. A year outside SE −314 to 160, 152/151 BCE, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_babylonian_regnal_year(seleucid_year: i64) -> line =
            $crate::calendar_values::babylonian_regnal_year_line;

        c {
            /// How far the equinox that begins a year of a calendar fell from the
            /// moment of the day that decides its new year, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `calendar` is `persian`,
            /// `persian-apparent-noon`, `jalali`, `bahai-astronomical` or
            /// `french-republican-equinox`, in any case; another is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. A year outside
            /// the calendar's range is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// How far the equinox that begins a year of a calendar fell from the
            /// moment of the day that decides its new year, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// `calendar` is `persian`, the nearer noon of Iran Standard Time;
            /// `persian-apparent-noon` or `jalali`, the nearer apparent noon at
            /// Tehran or at Isfahan; `bahai-astronomical`, the Tehran sunset; or
            /// `french-republican-equinox`, the nearer Paris apparent midnight; in
            /// any case. `year` is the calendar's own. Tab-separated: the margin in
            /// minutes, positive when the equinox fell before the moment (always
            /// positive for `french-republican-equinox`, whose size alone matters),
            /// and the calendar's identifier. A margin within the few minutes the
            /// astronomy is good to marks a year decided by a model. Another
            /// calendar is `HC_ERR_UNKNOWN`, and a year outside its range
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_equinox_new_year_margin(calendar: name(calendar_len), year: i64) -> line =
            $crate::calendar_values::equinox_new_year_margin_line;

        c {
            /// A *tekufah* of Shmuel's reckoning in a Hebrew year, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `tekufah` is `tishrei`, `tevet`, `nisan` or `tammuz`, in any case;
            /// anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. The line is the WebAssembly module's. A year
            /// outside 1 to 9999 is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// A *tekufah* of Shmuel's reckoning in a Hebrew year, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// Shmuel's year of 365¼ days in four seasons of 91 days and 7½ hours,
            /// as Maimonides gives it, in Jerusalem mean time. `tekufah` is
            /// `tishrei`, `tevet`, `nisan` or `tammuz`, in any case; anything else
            /// is `HC_ERR_UNKNOWN`. Tab-separated: the fixed day whose Hebrew day it
            /// falls in, the next civil day from six in the evening; the minutes of
            /// Jerusalem mean time since that civil midnight or the one before; and
            /// the *tekufah*'s identifier. A year outside 1 to 9999 is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_shmuel_tekufah(hebrew_year: i64, tekufah: name(tekufah_len)) -> line =
            $crate::calendar_values::shmuel_tekufah_line;

        c {
            /// A fixed day's name in a calendar whose days are named, by one of its
            /// namings, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `calendar` and `naming` are
            /// identifiers, in any case; a calendar the registry does not carry,
            /// one whose days are not named, or a naming it does not have is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. A day the
            /// calendar refuses is `HC_ERROR_OUT_OF_RANGE`, and one the naming does
            /// not name `HC_ERROR_NO_DATA`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// A fixed day's name in a calendar whose days are named, by one of its
            /// namings, as one UTF-8 line, returning the byte length written.
            ///
            /// The French Republican calendars name every day, in Fabre
            /// d'Églantine's table of 1793, `fr-fabre-1793`, in the list the
            /// calendar came to use, `fr`, and in English Wikipedia's gloss of it,
            /// `en`; the Armenian calendar names its thirty days of the month and
            /// its five epagomenal days in Armenian, `hy`, and the thirty in a
            /// romanisation, `hy-Latn`. Tab-separated: the name, the naming's
            /// identifier and English name, and its authority. A calendar the
            /// registry does not carry, one whose days are not named, or a naming
            /// it does not have is `HC_ERR_UNKNOWN`; a day the calendar refuses is
            /// `HC_ERR_OUT_OF_RANGE`, and one the naming does not name, an
            /// epagomenal day in `hy-Latn`, `HC_ERR_NO_DATA`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_day_name(calendar: name(calendar_len), naming: name(naming_len), fixed: i64) -> line =
            $crate::calendar_values::day_name_line;

        c {
            /// The fixed day of the yahrzeit in a Hebrew year of a death on the
            /// Hebrew date a fixed day names.
            ///
            /// `death_fixed` is the fixed day whose daylight carries the Hebrew
            /// date of the death — a death after sunset is the next fixed day —
            /// and the rules for the dates a later year may lack are Reingold and
            /// Dershowitz's. A day or a year outside the Hebrew years 1 to 9999 is
            /// `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The fixed day of the yahrzeit in a Hebrew year of a death on the
            /// Hebrew date a fixed day names, or an error sentinel.
            ///
            /// `death_fixed` is the fixed day whose daylight carries the Hebrew
            /// date of the death — a death after sunset is the next fixed day — and
            /// the rules for the dates a later year may lack (30 Ḥeshvan, 30 Kislev,
            /// Adar) are Reingold and Dershowitz's. A day or a year outside the
            /// Hebrew years 1 to 9999 is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_hebrew_yahrzeit(death_fixed: i64, hebrew_year: i64) -> value(out_fixed: i64) =
            $crate::calendar_values::hebrew_yahrzeit;

        c {
            /// The fixed day of the birthday in a Hebrew year of a birth on the
            /// Hebrew date a fixed day names.
            ///
            /// As `hc_hebrew_yahrzeit`, by Reingold and Dershowitz's
            /// `hebrew-birthday`: a birth in the last month of a year is kept in the
            /// last month of the later one.
        }
        wasm {
            /// The fixed day of the birthday in a Hebrew year of a birth on the
            /// Hebrew date a fixed day names, or an error sentinel.
            ///
            /// As `hc_hebrew_yahrzeit`, by Reingold and Dershowitz's
            /// `hebrew-birthday`: a birth in the last month of a year is kept in the
            /// last month of the later one.
        }
        fn hc_hebrew_birthday(birth_fixed: i64, hebrew_year: i64) -> value(out_fixed: i64) =
            $crate::calendar_values::hebrew_birthday;

        c {
            /// A person's age as the Chinese count reckons it on a fixed day.
            ///
            /// One at birth and one more at each Chinese New Year after, as
            /// Reingold and Dershowitz's `chinese-age` counts it. The birth crosses
            /// as its fixed day. A day before the birth has no age and is
            /// `HC_ERROR_NO_DATA`; a day outside the Chinese calendar's range is
            /// `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// A person's age as the Chinese count reckons it on a fixed day, or an
            /// error sentinel.
            ///
            /// One at birth and one more at each Chinese New Year after, whatever
            /// the day of birth, as Reingold and Dershowitz's `chinese-age` counts
            /// it and Wikipedia gives the pre-modern *suì* of China; nothing here
            /// says how any other country counts. The birth crosses as its fixed
            /// day. A day before the birth has no age and is `HC_ERR_NO_DATA`; a
            /// day outside the Chinese calendar's range is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_chinese_reckoned_age(birth_fixed: i64, on_fixed: i64) -> value(out_age: u32) =
            $crate::calendar_values::chinese_reckoned_age;

        c {
            /// The marriage augury of a Chinese year, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: `widow`, `blind`, `bright` or
            /// `double-bright`, then `1` or `0` for whether 立春 falls after the
            /// year's New Year and whether another falls before the next, then the
            /// Chinese names of the kind of year, their scripts and their regions.
            /// `chinese_year` is the Chinese calendar's own count, 4661 for the year
            /// that began on 10 February 2024; outside its range is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The marriage augury of a Chinese year, as one UTF-8 line, returning
            /// the byte length written.
            ///
            /// Tab-separated: the augury by the published code's names — `widow`
            /// for a year without 立春, `blind` for one only near its end, `bright`
            /// for one only near its start, `double-bright` for both — then `1` or
            /// `0` for whether 立春 falls after the year's New Year and whether
            /// another falls before the next; then the Chinese names the sources
            /// read give that kind of year, separated by `;` — 無春年, 寡婦年 and
            /// 盲年 and their simplified forms for a widow year, 雙春兼閏月 and 双春年
            /// for a double-bright one, none for the other two — the script of each,
            /// `zh-Hant` or `zh-Hans`, in the same order, and the region each is
            /// used in, `north`, `south` or empty where the source says none.
            /// `chinese_year` is the year as the
            /// Chinese calendar counts it, 4661 for the one that began on
            /// 10 February 2024. A year that begins, or whose next begins, outside
            /// the calendar's range is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_chinese_marriage_augury(chinese_year: i64) -> line =
            $crate::calendar_values::chinese_marriage_augury_line;

        c {
            /// A person's age on a fixed day by a named count, born on another.
            ///
            /// `convention` is `chinese-age`, `lichun-age`, `new-year-day-age` or
            /// `year-age`, in any case; anything else is `HC_ERROR_UNKNOWN`, and
            /// null `HC_ERROR_NULL_POINTER`. A day before the birth is
            /// `HC_ERROR_NO_DATA`, and a day the count's calendar does not reach
            /// `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// A person's age on a fixed day by a named count, born on another, or
            /// an error sentinel.
            ///
            /// `convention` is `chinese-age`, one at birth and one more at each
            /// Chinese New Year, `hc_chinese_reckoned_age`'s; `lichun-age`, one more
            /// at each 立春 instead; `new-year-day-age`, one at birth and one more
            /// each 1 January, the Korean 세는 나이; or `year-age`, nothing at birth
            /// and one more each 1 January, the Korean 연 나이; in any case. Anything
            /// else is `HC_ERR_UNKNOWN`. A day before the birth is `HC_ERR_NO_DATA`;
            /// a day outside the Chinese calendar's range, 1645 through 2150, for
            /// the first two, or outside the Gregorian range for the others, is
            /// `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_chinese_age(
            convention: name(convention_len),
            birth_fixed: i64,
            on_fixed: i64,
        ) -> value(out_age: u32) =
            $crate::calendar_values::chinese_age;

        c {
            /// The days of the twenty-four solar terms the Qing almanac printed in
            /// a Gregorian year, as NUL-terminated UTF-8 lines in a caller-owned
            /// buffer.
            ///
            /// The lines are the WebAssembly module's: the term's position, its
            /// name and the fixed day. A year outside 1645 to 1733, or one of
            /// 1667 to 1669, is `HC_ERROR_NO_DATA`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The days of the twenty-four solar terms the Qing almanac printed in
            /// a Gregorian year, as UTF-8 lines, returning the byte length written.
            ///
            /// Liu's transcription of the almanac, from before the bureau took up
            /// Kepler's laws, whose term days often stand a day from the modern
            /// ones. One line per term, 小寒 first and 冬至 last, tab-separated: the
            /// position, 1 to 24; the name in traditional Chinese; and the fixed
            /// day. A year outside 1645 to 1733, or one of the Dàtǒng years 1667 to
            /// 1669, is `HC_ERR_NO_DATA`. A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_chinese_almanac_solar_terms(year: i64) -> line =
            $crate::calendar_values::almanac_solar_terms_lines;

        c {
            /// The place of a Hebrew year in the seven-year sabbatical cycle, 1
            /// through 7.
            ///
            /// 7 is the sabbatical year, *shemittah*, as the years published today
            /// count it: 5782 and 5789 are sabbatical years. A year outside the
            /// Hebrew years 1 to 9999 is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The place of a Hebrew year in the seven-year sabbatical cycle, 1
            /// through 7, or an error sentinel.
            ///
            /// 7 is the sabbatical year, *shemittah*, counted from Rosh Hashanah as
            /// the years published today count it: 5782 (2021–22) and 5789
            /// (2028–29) are sabbatical years. A year outside the Hebrew years 1 to
            /// 9999 is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_hebrew_sabbatical_cycle_year(hebrew_year: i64) -> value(out_place: i64) =
            $crate::calendar_values::hebrew_sabbatical_cycle_year;

        c {
            /// A fixed day in the calendar of the Roman province of Asia as the
            /// calendar writes it, unnumbered days included, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the Julian year in which the
            /// Asian year began, the month, its name, `unnumbered` or `numbered`,
            /// and the day's number or its place among the unnumbered days. A day
            /// outside 23 September AD 4 to the end of the Asian year 9999 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A fixed day in the calendar of the Roman province of Asia as the
            /// calendar writes it, unnumbered days included, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the Julian year, AD, in which the Asian year began;
            /// the month, 1 for Kaisar through 12 for Hyperberetaios; the month's
            /// name; `unnumbered` for a day before day 1 — Sebaste, which opens a
            /// 31-day month, and in a leap Xandikos Sebaste and the intercalary
            /// day, whose order the sources read do not settle — or `numbered`; and
            /// the day's number, 1 to 30, or for an unnumbered day its place among
            /// them, 1 or 2. A day outside 23 September AD 4 to the end of the
            /// Asian year 9999 is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns
            /// the length the text needs.
        }
        fn hc_asian_day(fixed: i64) -> line = $crate::calendar_values::asian_day_line;

        c {
            /// The name of the northern sixty-year cycle a named rule couples with
            /// an expired Śaka year, and the name it expunges that year, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `rule` is `surya-siddhanta-bija`, `surya-siddhanta` or
            /// `arya-siddhanta`, as for the WebAssembly module's
            /// `hc_barhaspatya_year`; null is `HC_ERROR_NULL_POINTER` and another
            /// name `HC_ERROR_UNKNOWN`. The line is the module's: the position and
            /// its name in the `locale`, the expunged position and name, and the
            /// tag that named them. A Śaka year outside −3178 to 6821 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The name of the northern sixty-year cycle, the Bārhaspatya
            /// saṃvatsara, a named rule couples with a Śaka year, and the name it
            /// expunges that year, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// `rule` is `surya-siddhanta-bija`, the *Sūrya Siddhānta* with the
            /// *bīja*, by which the pūrṇimānta calendar names its years and Drik
            /// Panchang heads Vikrama 2081 to 2083; `surya-siddhanta`, the same
            /// without it, whose names, one on, some of the Hindi press's
            /// announcements of 2021–26 print;
            /// or `arya-siddhanta`, the first *Ārya Siddhānta*; in any case, from
            /// Sewell and Dikshit's Art. 59. Anything else is `HC_ERR_UNKNOWN`.
            /// `saka` is an *expired* Śaka year, the year the *Rashtriya Panchang*
            /// prints, and the name is the one current at the apparent Meṣa
            /// saṅkrānti of its solar year. Tab-separated: the name's position, 1
            /// for Prabhava through 60 for Kṣaya, and its name in the locale; the
            /// position and name of the one the rule expunges in that solar year,
            /// both empty in a year that expunges none, named as `hc_day_extras`
            /// names the pūrṇimānta calendar's `barhaspatya-samvatsara`; and the
            /// locale used, as column 7 of `hc_day_extras` gives it, the tag of the
            /// locale data that answered, whose names may be English's: `hi` for
            /// Pingala under `hi`. The locale argument fails as
            /// `hc_parse_iso_date` does. A Śaka year outside −3178 to 6821, Kali
            /// Yuga 1 to 10 000 expired, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_barhaspatya_year(rule: name(rule_len), saka: i64, locale: text(locale_len)) -> line =
            $crate::hindu_lines::barhaspatya_year_line;

        c {
            /// The name of the northern sixty-year cycle in progress at a POSIX
            /// timestamp by a named rule, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// `rule` is as for `hc_barhaspatya_year`. The line is the WebAssembly
            /// module's: the position, its name in the `locale`, the locale used,
            /// and the twelve-year cycle's saṃvatsara and Jupiter's mean sign. An
            /// instant outside the days of Kali Yuga 1 to 10 000 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The name of the northern sixty-year cycle in progress at a POSIX
            /// timestamp by a named rule, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// `rule` is as for `hc_barhaspatya_year`. The name current at the last
            /// apparent Meṣa saṅkrānti of the *Sūrya Siddhānta* runs to the day the
            /// rule ends it, and the next from then; Sewell and Dikshit give the
            /// ends as correct within two ghaṭikās where the saṅkrānti is known.
            /// Tab-separated: the position, 1 to 60, its name in the locale, and
            /// the locale used, as `hc_barhaspatya_year` gives it; then the
            /// saṃvatsara of the twelve-year cycle Sewell and Dikshit's Table XII
            /// couples with it, by its position, 1 for Chaitra to 12 for Phālguna,
            /// and its name as the table spells it, `Asvina`; and the sign Jupiter's
            /// mean longitude stands in while the name is current, by its
            /// identifier, `mesha`. The locale
            /// argument fails as
            /// `hc_parse_iso_date` does. An instant outside the days of Kali Yuga 1
            /// to 10 000, as for `hc_surya_siddhanta_at`, is `HC_ERR_OUT_OF_RANGE`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_barhaspatya_year_at(
            rule: name(rule_len),
            unix_seconds: i64,
            locale: text(locale_len),
        ) -> line =
            $crate::hindu_lines::barhaspatya_year_at_line;

        c {
            /// Rāhu kālam, Yamaganda and Gulika kālam on a fixed day, as three
            /// NUL-terminated UTF-8 lines, each named in a locale, in a
            /// caller-owned buffer.
            ///
            /// `convention` is `rahu-kalam-sunrise` or `rahu-kalam-fixed`, in any
            /// case; anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. The lines are the WebAssembly module's: the
            /// period's identifier and English name, its name in the `locale` and
            /// the tag that named it, its eighth of the day, the clock, `universal`
            /// or `local`, the start and the end, and the four cells of a missing
            /// solar event. `locale` is a NUL-terminated BCP 47 tag, `native` or
            /// null, as for `hc_describe_day`. A place off the globe, or a day
            /// outside the years −1000 to 3000, is `HC_ERROR_OUT_OF_RANGE`. Writes
            /// the required length, including the terminator, into `written`.
        }
        wasm {
            /// Rāhu kālam, Yamaganda and Gulika kālam on a fixed day, as three
            /// UTF-8 lines, each named in a locale, returning the byte length
            /// written.
            ///
            /// `convention` is `rahu-kalam-sunrise`, the daylight from sunrise to
            /// sunset at the place cut into eight, as Drik Panchang computes it, or
            /// `rahu-kalam-fixed`, 06:00 to 18:00 of the local clock cut into eight
            /// parts of an hour and a half, in any case; anything else is
            /// `HC_ERR_UNKNOWN`. The place is as for `hc_solar_time`. One line per
            /// period, in the order a pañcāṅga prints them, tab-separated: its
            /// identifier (`rahu-kalam`, `yamaganda`, `gulika-kalam`), its English
            /// name, its name in the locale and the tag of the data that named it,
            /// राहुकाल under `hi`, the eighth of the day it takes, 1 to 8, the clock
            /// its times are read on, and its start and end, then the four cells of
            /// a missing solar event, as `hc_solar_event` writes them. The locale
            /// argument fails as `hc_parse_iso_date` does. On
            /// `rahu-kalam-sunrise` the clock is `universal` and the times are
            /// whole POSIX seconds, rounded down, empty where the Sun does not rise
            /// or set and the event is named; on `rahu-kalam-fixed` the clock is
            /// `local` and they are seconds after midnight of the day's own clock.
            /// A place off the globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_kalam(
            convention: name(convention_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            locale: text(locale_len),
        ) -> line =
            |convention, fixed, latitude, longitude, elevation, locale| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::panchanga_lines::kalam_lines(convention, fixed, place, locale)
                    })
            };

        c {
            /// The thirty muhūrtas of a fixed day at a place, with Abhijit and Dur
            /// Muhurtam marked, as NUL-terminated UTF-8 lines in a caller-owned
            /// buffer.
            ///
            /// The lines are the WebAssembly module's: the half, the number, the
            /// start and the end, the mark, and the four cells of a missing solar
            /// event. A place off the globe, or a day outside the years −1000 to
            /// 3000, is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The thirty muhūrtas of a fixed day at a place, with Abhijit and Dur
            /// Muhurtam marked, as UTF-8 lines, returning the byte length written.
            ///
            /// The daylight, sunrise to sunset, and the night after it, sunset to
            /// the next sunrise, are each cut into fifteen equal muhūrtas, and each
            /// is a line, the day's first. Tab-separated: the half, `day` or
            /// `night`; the muhūrta's number in it, 1 to 15; its start and its end
            /// as whole POSIX seconds of Universal Time, rounded down; what the
            /// pañcāṅga prints it as — `abhijit` for the eighth of the daylight on
            /// a day but a Wednesday, `dur-muhurtam` for the weekday's one or two,
            /// Drik Panchang's, else empty; and the four cells of a missing solar
            /// event, as `hc_kalam` writes them, where the Sun does not rise or set
            /// and the start and end are empty. The place is the latitude and
            /// longitude in degrees, north and east positive, and the elevation in
            /// metres. A place off the globe, or a day outside the years −1000 to
            /// 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length
            /// the text needs.
        }
        fn hc_muhurtas(fixed: i64, latitude: f64, longitude: f64, elevation: f64) -> line =
            |fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::panchanga_lines::muhurtas_lines(fixed, place))
            };

        c {
            /// The *amṛta siddhi yoga* of a fixed day at a place, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the yoga's name in English and
            /// in Devanagari, the weekday's nakṣatra, the start and the end, whether
            /// it falls on the day, and the ayanāṃśa. `ayanamsa` is as for
            /// `hc_nakshatra_at`. A place off the globe, or a day outside the years
            /// −1000 to 3000, is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// The *amṛta siddhi yoga* of a fixed day at a place, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// The yoga holds for the part of the day, sunrise to the next
            /// sunrise, that the Moon spends in the nakṣatra the weekday pairs
            /// with: Hasta on Sunday, Mṛgaśīrṣa on Monday, Aśvinī on Tuesday,
            /// Anurādhā on Wednesday, Puṣya on Thursday, Revatī on Friday and
            /// Rohiṇī on Saturday. Tab-separated: its name as Drik Panchang prints
            /// it in English and in Devanagari; that nakṣatra, 1 for Aśvinī to 27
            /// for Revatī; the start and the end of the part as whole POSIX seconds
            /// of Universal Time, rounded down, both empty on a day the Moon spends
            /// none of in it; `1` if the yoga falls on the day and `0` if not; and
            /// the ayanāṃśa's identifier. `ayanamsa` is as for `hc_nakshatra_at`,
            /// and the place as for `hc_muhurtas`. A place off the globe, or a day
            /// outside the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_amrita_siddhi(
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            ayanamsa: name(ayanamsa_len),
        ) -> line =
            |fixed, latitude, longitude, elevation, ayanamsa| {
                $crate::astro_lines::location(latitude, longitude, elevation).and_then(|place| {
                    $crate::panchanga_lines::amrita_siddhi_line(fixed, place, ayanamsa)
                })
            };

        c {
            /// The nakṣatra the Moon is in at a POSIX timestamp, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the nakṣatra, when the Moon
            /// entered it and leaves it, the instant read, and the ayanāṃśa by
            /// identifier and full name. `ayanamsa` is `lahiri`, `raman`,
            /// `krishnamurti`, `reingold-dershowitz` or `fagan-bradley`, in any
            /// case; anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. An instant outside the years −1000 to 3000
            /// is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// The nakṣatra the Moon is in at a POSIX timestamp, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the nakṣatra, 1 for Aśvinī through 27 for Revatī, the
            /// arc of 13°20′ of the Moon's sidereal longitude in the zodiac of the
            /// ayanāṃśa; the instants the Moon entered it and leaves it and the
            /// instant read, as whole POSIX seconds of Universal Time, rounded
            /// down; and the ayanāṃśa by its identifier and its full name.
            /// `ayanamsa` is `lahiri`, `raman`, `krishnamurti`, `reingold-dershowitz`
            /// or `fagan-bradley`, in any case; anything else, the empty string
            /// included, is `HC_ERR_UNKNOWN`. An instant outside the years −1000 to
            /// 3000 is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_nakshatra_at(unix_seconds: i64, ayanamsa: name(ayanamsa_len)) -> line =
            $crate::panchanga_lines::nakshatra_at_lines;

        c {
            /// The nakṣatra a fixed day carries at a place, the one the Moon is in
            /// at its sunrise, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line of `hc_nakshatra_at`, read at the day's sunrise. A day on
            /// which the Sun does not rise there is `HC_ERROR_NO_DATA`; a place off
            /// the globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The nakṣatra a fixed day carries at a place, the one the Moon is in
            /// at its sunrise, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// The line of `hc_nakshatra_at`, read at the day's sunrise at the
            /// place, as for `hc_muhurtas`. A day on which the Sun does not rise
            /// there is `HC_ERR_NO_DATA`. A place off the globe, or a day outside
            /// the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`; `ayanamsa` is as
            /// for `hc_nakshatra_at`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_nakshatra_of_day(
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            ayanamsa: name(ayanamsa_len),
        ) -> line =
            |fixed, latitude, longitude, elevation, ayanamsa| {
                $crate::astro_lines::location(latitude, longitude, elevation).and_then(|place| {
                    $crate::panchanga_lines::nakshatra_of_day_lines(fixed, place, ayanamsa)
                })
            };

        c {
            /// The almanac's cycles of a fixed day, 恵方, 三元九運 and 손 없는 날, as
            /// one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `meridian` is as for `hc_term_in_effect`; null is
            /// `HC_ERROR_NULL_POINTER` and a meridian not read `HC_ERROR_UNKNOWN`.
            /// The line is the WebAssembly module's. A day outside the years −1000
            /// to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The almanac's cycles of a fixed day, 恵方, 三元九運 and 손 없는 날, as
            /// one UTF-8 line, returning the byte length written.
            ///
            /// `meridian` is as for `hc_term_in_effect`, and sets where 立春, which
            /// turns the 三元九運 year, falls. Tab-separated: 恵方 of the day's
            /// Gregorian year, its point of the twenty-four (甲, 庚, 丙 or 壬), the
            /// point's reading in Hepburn romaji, its azimuth in degrees clockwise
            /// from north and the nearest of the sixteen compass points in Japanese
            /// and in English; the 三元九運 period in force, its number, 1 to 9, its
            /// name, its era, the 九星 that rules it, the star of the Dipper its
            /// source names, and its first and last years; and `1` if the day is
            /// 손 없는 날 in the Korean lunar calendar, else `0`, empty outside that
            /// calendar's years. A meridian not read is `HC_ERR_UNKNOWN`, and a day
            /// outside the years −1000 to 3000 `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_almanac_cycles(fixed: i64, meridian: name(meridian_len)) -> line =
            $crate::almanac_lines::almanac_cycles_line;

        c {
            /// The almanac's annotations of a fixed day, 干支 to the 選日, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer, one an
            /// annotation, each named in a locale.
            ///
            /// The lines are the WebAssembly module's: the kind, the identifier,
            /// the name in the `locale` and the tag of the data that named it, the
            /// Japanese name the almanac prints, its Hepburn reading, whether the
            /// almanac counts the day auspicious, and for the 暦注下段 whether it
            /// prints the entry. `meridian` is as for `hc_almanac_cycles`; null is
            /// `HC_ERROR_NULL_POINTER` and a meridian not read `HC_ERROR_UNKNOWN`.
            /// `locale` is a NUL-terminated BCP 47 tag, `native` for the almanac's
            /// own language, Japanese, or null, as for `hc_describe_day`; one that
            /// is not UTF-8 is `HC_ERROR_NOT_UTF8`. A day outside the years −1000
            /// to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The almanac's annotations of a fixed day, 干支 to the 選日, as UTF-8
            /// lines, one an annotation, each named in a locale, returning the byte
            /// length written.
            ///
            /// `meridian` is as for `hc_almanac_cycles`, and sets where the solar
            /// terms and the new moons fall. The lines, in the order a printed
            /// almanac page gives them: the sexagenary day, its 納音, 十二直,
            /// 二十八宿, 二十七宿, the year's, the month's and the day's 九星, 六曜,
            /// then each 暦注下段, 選日 and combination of them that falls.
            /// Tab-separated: the kind (`sexagenary`, `nayin`, `twelve-direct`,
            /// `mansion`, `mansion-27`, `year-star`, `month-star`, `day-star`,
            /// `rokuyo`, `lower-register`, `selected-day`, `combination`); the
            /// identifier, the 1-based position
            /// in the cycle or the entry's own (`tenshanichi`); the name in the
            /// locale and the tag of the data that named it, the locale's where it
            /// has one, else English's, else Japanese's, Japanese first under
            /// `native`; the Japanese name the almanac prints; its Hepburn reading;
            /// `1` where the almanac counts the day auspicious, `0` where
            /// inauspicious, else empty; and for the 暦注下段 `1` where an almanac
            /// prints the entry, `0` where 受死日 or 十死日 suppresses it. The
            /// locale argument fails as `hc_parse_iso_date` does; a meridian not
            /// read is `HC_ERR_UNKNOWN`, and a day outside the years −1000 to 3000
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_almanac_day(
            fixed: i64,
            meridian: name(meridian_len),
            locale: text(locale_len),
        ) -> line =
            $crate::almanac_lines::almanac_day_lines;

        c {
            /// Where the 八将神 and the 金神 stand in the year in force on a fixed
            /// day, as NUL-terminated UTF-8 lines in a caller-owned buffer, one a
            /// god and a direction.
            ///
            /// The lines are the WebAssembly module's: the god's identifier, its
            /// name, its reading, the direction's branch and azimuth, what the god
            /// forbids or favours, the year's 干支 and the branch's number; then a
            /// line for each reading of the 遊行 of 大将軍 and 金神,
            /// `daishogun-iinippon`, `konjin-wikipedia-begun-in-season` and
            /// `konjin-wikipedia-days-in-season`, and on 金神の間日 a line
            /// `konjin-rest-day`.
            /// `meridian` is as for `hc_almanac_cycles`, and sets where 立春 turns
            /// the year; null is `HC_ERROR_NULL_POINTER` and a meridian not read
            /// `HC_ERROR_UNKNOWN`. A day outside the years −1000 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Where the 八将神 and the 金神 stand in the year in force on a fixed
            /// day, as UTF-8 lines, one a god and a direction, returning the byte
            /// length written.
            ///
            /// The year is the 干支 year in force on the day, turning at 立春 at the
            /// meridian. The lines: the eight 八将神 in the order the almanacs list
            /// them, 太歳神 to 豹尾神, then 金神 once for each branch the year's stem
            /// gives it, then 大金神 and 姫金神. Tab-separated: the god's identifier
            /// (`taisai`, `daishogun`, `daion`, `saikyo`, `saiha`, `saisetsu`,
            /// `oban`, `hyobi`, `konjin`, `dai-konjin`, `hime-konjin`); its name,
            /// 太歳神; its Hepburn reading as the National Diet Library gives it,
            /// empty for the three 金神; the direction as its earthly branch, 午,
            /// and the branch's azimuth in degrees clockwise from north; what the
            /// Library says the god forbids or favours, in English, empty for the
            /// 金神; the year's 干支, 丙午; and the branch's number, 1 for 子 to 12
            /// for 亥. Then a line for each reading of the 遊行 of 大将軍 and 金神,
            /// the days they leave their directions, each a rule of its own:
            /// `daishogun-iinippon`, `konjin-wikipedia-begun-in-season` and
            /// `konjin-wikipedia-days-in-season`; its identifier, the god's name,
            /// an empty reading, the place gone to — a branch with its azimuth
            /// and number, or 中央 with those empty, all three empty at home —
            /// `home` or `gone` in place of the meaning, empty where the rule does
            /// not say, and the year's 干支. Last, on a day that is 金神の間日, a
            /// line `konjin-rest-day`, 金神の間日, the other cells empty but the
            /// year's. `meridian` is as for `hc_almanac_cycles`; a meridian not
            /// read is `HC_ERR_UNKNOWN`, and a day outside the years −1000 to 3000
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_almanac_directions(fixed: i64, meridian: name(meridian_len)) -> line =
            $crate::almanac_lines::almanac_directions_lines;

        c {
            /// The fixed day of 臘日 in the winter that ends in a Gregorian year, by
            /// a named reckoning, at a meridian.
            ///
            /// `rule` is `second-dragon-after-minor-cold`,
            /// `second-dragon-from-minor-cold`, `dragon-nearest-major-cold-earlier`,
            /// `dragon-nearest-major-cold-later`, `first-dog-after-major-cold`,
            /// `first-dog-from-major-cold`, `lunar-twelfth-ninth`,
            /// `ox-month-ninth-from-minor-cold`, `ox-month-ninth-after-minor-cold`,
            /// `third-dog-after-winter-solstice` or `third-dog-from-winter-solstice`,
            /// in any case. `meridian` is as for `hc_term_in_effect`. Null for
            /// either is `HC_ERROR_NULL_POINTER`, a name not known
            /// `HC_ERROR_UNKNOWN`. A year outside −999 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`, and a winter whose lunar year the calendar
            /// does not reach `HC_ERROR_NO_DATA`.
        }
        wasm {
            /// The fixed day of 臘日 in the winter that ends in a Gregorian year, by
            /// a named reckoning, at a meridian, or an error sentinel.
            ///
            /// `rule` is `second-dragon-after-minor-cold` or
            /// `second-dragon-from-minor-cold`, the second 辰 day after 小寒;
            /// `dragon-nearest-major-cold-earlier` or
            /// `dragon-nearest-major-cold-later`, the 辰 day nearest 大寒;
            /// `first-dog-after-major-cold` or `first-dog-from-major-cold`, the
            /// first 戌 day after 大寒; `lunar-twelfth-ninth`, the ninth of the
            /// twelfth lunar month; `ox-month-ninth-from-minor-cold` or
            /// `ox-month-ninth-after-minor-cold`, the ninth day of 丑月; or
            /// `third-dog-after-winter-solstice` or `third-dog-from-winter-solstice`,
            /// the Han third 戌 day after the 冬至 before; in any case. Where the
            /// wording admits two readings each is a rule: an `-after-` rule does
            /// not count the term's own day and a `-from-` rule does, and
            /// `-earlier` and `-later` take one or the other of two 辰 days equally
            /// near 大寒. `meridian` is as for `hc_term_in_effect`. A name not known
            /// is `HC_ERR_UNKNOWN`. The day falls in January or early February of
            /// `year`. A year outside −999 to 3000 is `HC_ERR_OUT_OF_RANGE`, and a
            /// winter whose lunar year the calendar does not reach, for the lunar
            /// rule, `HC_ERR_NO_DATA`.
        }
        fn hc_rounichi(
            rule: name(rule_len),
            year: i64,
            meridian: name(meridian_len),
        ) -> value(out_fixed: i64) =
            $crate::almanac_lines::rounichi_day;

        c {
            /// What one publisher's list says the 二十八宿 of a fixed day favours
            /// and forbids, as NUL-terminated UTF-8 lines in a caller-owned buffer,
            /// one an undertaking.
            ///
            /// The lines are the WebAssembly module's: the mansion's number and
            /// name, the grade and the undertaking. `list` is `saijigoyomi` or
            /// `linderabell`, in any case; anything else is `HC_ERROR_UNKNOWN`, and
            /// null `HC_ERROR_NULL_POINTER`. A day outside the years −1000 to 3000
            /// is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// What one publisher's list says the 二十八宿 of a fixed day favours
            /// and forbids, as UTF-8 lines, one an undertaking, returning the byte
            /// length written.
            ///
            /// `list` is `saijigoyomi`, 歳事暦's 「暦の吉凶 二十八宿」, or
            /// `linderabell`, うまずたゆまず's 「二十八宿」, in any case; anything
            /// else is `HC_ERR_UNKNOWN`. The mansion is the almanac's 28-day cycle,
            /// which needs no meridian. One line per undertaking in the publisher's
            /// order, 大吉 first, then 吉, 凶 and 大凶, then the list's remark about
            /// the day as a whole where it makes one. Tab-separated: the mansion's
            /// number, 1 for 角 to 28 for 軫; its name, 角; the grade, `best`,
            /// `favoured`, `avoided`, `worst` or `note`; and the undertaking or the
            /// remark as the publisher prints it, in Japanese. A day outside the
            /// years −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_mansion_undertakings(list: name(list_len), fixed: i64) -> line =
            |list, fixed| $crate::almanac_lines::mansion_undertakings_lines(list, fixed);

        c {
            /// Whether a fixed day is one of a person's own 五墓日 or 三箇の悪日, by
            /// the year they were born in, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer, one an entry.
            ///
            /// The lines are the WebAssembly module's: the kind, the identifier,
            /// the entry's name, what the person keeps and whether the day is it.
            /// `birth_year` is the Gregorian year whose 干支 is the birth year's.
            /// `meridian` is as for `hc_almanac_cycles`; null is
            /// `HC_ERROR_NULL_POINTER` and a meridian not read `HC_ERROR_UNKNOWN`.
            /// A day outside the years −1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// Whether a fixed day is one of a person's own 五墓日 or 三箇の悪日, by
            /// the year they were born in, as UTF-8 lines, one an entry, returning
            /// the byte length written.
            ///
            /// One line for each reading of 五墓日 that gives each person a day of
            /// their own, by the 納音 phase of the birth year
            /// (`gomunichi-wikipedia`, `gomunichi-nikkoku`), then one for each of
            /// the three 悪日, 大禍日, 狼藉日 and 滅門日, which fall on a person in
            /// the 節月 of their birth year's branch only. Tab-separated: the kind,
            /// `grave-day` or `three-evil-day`; the reading's or the entry's
            /// identifier (`taikanichi`); the entry's name, 五墓日 or 大禍日; what the
            /// person keeps, the 干支 of their grave day, 乙丑, or the branch of the
            /// 節月 their evil days fall in, 巳; and `1` if the day is that entry
            /// for the person, else `0`. `birth_year` is the Gregorian year whose
            /// 干支 is the person's birth year's, as the tables read count it; a
            /// person whose year is reckoned from 立春 and who was born before it
            /// passes the year before. `meridian` is as for `hc_almanac_cycles`; a
            /// meridian not read is `HC_ERR_UNKNOWN`, and a day outside the years
            /// −1000 to 3000 `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_almanac_person_days(
            fixed: i64,
            birth_year: i64,
            meridian: name(meridian_len),
        ) -> line =
            $crate::almanac_lines::almanac_person_lines;

        c {
            /// What the Tibetan almanac of a version prints for a fixed day, the
            /// five components and the columns after them, as NUL-terminated UTF-8
            /// lines in a caller-owned buffer, one a column.
            ///
            /// The lines are the WebAssembly module's: the kind, the identifier,
            /// the Sanskrit or English name and the Tibetan one, the almanac's
            /// reading and its decimal. `calendar` is `tibetan`,
            /// `tibetan-tsurphu`, `tibetan-bhutan`, `mongolian`, `tibetan-lochen`,
            /// `tibetan-tsurphu-karana` or `tibetan-bhutan-lochen`, in any case;
            /// anything else is `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. A day outside
            /// the version's range is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// What the Tibetan almanac of a version prints for a fixed day, the
            /// five components and the columns after them, as UTF-8 lines, one a
            /// column, returning the byte length written.
            ///
            /// `calendar` is `tibetan`, `tibetan-tsurphu`, `tibetan-bhutan`,
            /// `mongolian`, `tibetan-lochen`, `tibetan-tsurphu-karana` or
            /// `tibetan-bhutan-lochen`, in any case; anything else is
            /// `HC_ERR_UNKNOWN`. Tab-separated: the kind;
            /// its identifier; the Sanskrit or English name and the Tibetan one in
            /// Wylie, as Henning's almanacs print them; the almanac's reading, the
            /// whole part and two sexagesimal places, `2;11,24`; and the reading as
            /// a decimal. The kinds, in order: `weekday`, 1 for Saturday to 7 for
            /// Friday, with the true weekday, empty on the first of two days with
            /// one number; `mansion`, 1 to 27, with the Moon at daybreak;
            /// `yoga`, 1 to 27, with the yoga longitude; `karana`, 1 to 11; the
            /// `half-day` of the month at daybreak, 1 to 60, as the identifier;
            /// the true `sun` and the `mean-sun`, in mansions and in signs; the
            /// head of `rahu`, on `tibetan` and `tibetan-lochen` alone; the
            /// `rab-byung` year, 1 for Prabhava; the `royal-year`, from 127 BCE,
            /// as the identifier; and the `year-symbol`, the `month-symbol` where
            /// the version's rule is given and the `day-symbol`, each with the
            /// animal's number, the element and animal, `Water-Snake`, empty, the
            /// gender and the element's colour. A day outside the version's range
            /// is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_tibetan_almanac_day(calendar: name(calendar_len), fixed: i64) -> line =
            $crate::tibetan_lines::almanac_day_lines;

        c {
            /// Where the Phugpa almanac places the five planets at the end of a
            /// fixed day, as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. A day outside the Tibetan
            /// calendar's range is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// Where the Phugpa almanac places the five planets at the end of a
            /// fixed day, as UTF-8 lines, one a planet, returning the byte length
            /// written.
            ///
            /// From Henning's epoch of 1927. Tab-separated: the planet, `mercury`,
            /// `venus`, `mars`, `jupiter` or `saturn`; its particular day; its
            /// mean heliocentric, true slow and fast longitudes in lunar mansions
            /// as the almanac reads them, `23;4,36`; and the same three as
            /// decimals. A day outside the Tibetan calendar's range, the years
            /// 1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_tibetan_planets(fixed: i64) -> line = $crate::tibetan_lines::planet_lines;

        c {
            /// The Bhutanese calendar's winter solstice of a Gregorian year, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the fixed day, the weekday and
            /// time as the almanac prints it, and the local Julian Date. A year
            /// outside 1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// The Bhutanese calendar's winter solstice of a Gregorian year, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// The instant its mean Sun reaches 250°, which falls in the first days
            /// of January. Tab-separated: the fixed day it falls on; the weekday and
            /// time the almanac prints, days after Saturday's dawn, nāḍī and pala,
            /// `2;51,38`; and the local Julian Date as a decimal. A year outside
            /// 1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_bhutanese_winter_solstice(year: i64) -> line =
            $crate::tibetan_lines::bhutanese_winter_solstice_line;

        c {
            /// The fixed day a festival on a Tibetan date is kept on, by a named
            /// rule for a skipped or repeated number.
            ///
            /// `rule` is `berzin` or `henning-almanac`, in any case, and `calendar`
            /// as for `hc_tibetan_almanac_day`; null for either is
            /// `HC_ERROR_NULL_POINTER`, a name not known `HC_ERROR_UNKNOWN`. A year
            /// outside 1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`, a month the year
            /// does not have or a day outside 1 to 30 `HC_ERROR_INVALID_DATE`, and a
            /// skipped number under `henning-almanac` `HC_ERROR_NO_DATA`.
        }
        wasm {
            /// The fixed day a festival on a Tibetan date is kept on, by a named
            /// rule for a skipped or repeated number, or an error sentinel.
            ///
            /// `rule` is `berzin`, Janson's report of Berzin's rule, the day before
            /// a skipped number and the first of a repeated one; or
            /// `henning-almanac`, as Henning's computed almanacs mark a festival,
            /// the second of a repeated number and no day for a skipped one; in any
            /// case. `calendar` is as for `hc_tibetan_almanac_day`; `year` is the
            /// Tibetan year, numbered by the Western year it begins in; `leap`
            /// non-zero asks for the leap month. A name not known is
            /// `HC_ERR_UNKNOWN`; a year outside 1000 to 3000 is
            /// `HC_ERR_OUT_OF_RANGE`; a month the year does not have or a day
            /// outside 1 to 30 is `HC_ERR_INVALID_DATE`; and a skipped number
            /// under `henning-almanac` is `HC_ERR_NO_DATA`.
        }
        fn hc_tibetan_festival_day(
            rule: name(rule_len),
            calendar: name(calendar_len),
            year: i64,
            month: u32,
            leap: flag,
            day: u32,
        ) -> value(out_fixed: i64) =
            $crate::tibetan_lines::festival_day;

        c {
            /// The sixteen choghadiya of a fixed day at a place, as NUL-terminated
            /// UTF-8 lines in a caller-owned buffer, each named in a locale.
            ///
            /// The lines are the WebAssembly module's: the half, the part, the
            /// kind's identifier, its name in the `locale` and the tag that named
            /// it, its quality, its ruling planet, the start and the end, and the
            /// four cells of a missing solar event. `locale` is a NUL-terminated
            /// BCP 47 tag, `native` or null, as for `hc_describe_day`. A place off
            /// the globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The sixteen choghadiya of a fixed day at a place, as UTF-8 lines,
            /// each named in a locale, returning the byte length written.
            ///
            /// The daylight from sunrise to sunset and the night from sunset to the
            /// next sunrise are each cut into eight, and each part takes one of
            /// seven kinds by the weekday, the night the weekday of the sunset that
            /// begins it, as Drik Panchang prints them. The place is as for
            /// `hc_solar_time`. One line per part, the day's eight then the
            /// night's, tab-separated: the half, `day` or `night`; the part, 1 to
            /// 8; the kind's identifier, `udvega`, `chara`, `labha`, `amrita`,
            /// `kala`, `shubha` or `roga`; its name in the locale and the tag of the
            /// data that named it; its quality, `auspicious`, `neutral` or
            /// `inauspicious`; the planet that rules it, `sun` to `saturn`; the
            /// start and the end as whole POSIX seconds of Universal Time, rounded
            /// down; and the four cells of `hc_solar_event` naming a missing solar
            /// event, the start and end being empty instead where the Sun does not
            /// rise or set. The locale argument fails as `hc_parse_iso_date` does.
            /// A place off the globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_choghadiya(
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            locale: text(locale_len),
        ) -> line =
            |fixed, latitude, longitude, elevation, locale| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::reckoning_lines::choghadiya_lines(fixed, place, locale)
                    })
            };

        c {
            /// The Panchak window in progress at a POSIX timestamp, or the next
            /// one, and its kind under a naming table, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// `naming` is `panchak-five-kinds` or `panchak-raj-midweek` and
            /// `ayanamsa` as for `hc_panchanga_at`; null for either is
            /// `HC_ERROR_NULL_POINTER` and a name not known `HC_ERROR_UNKNOWN`.
            /// `offset_seconds` is the clock, ahead of Universal Time, on which the
            /// weekday the window opens on is read, midnight to midnight. The line
            /// is the WebAssembly module's: within or not, the opening and the
            /// closing, the weekday, and the kind's identifier, its name in the
            /// `locale` and the tag that named it. An offset of a day or more, or a
            /// timestamp outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The Panchak window in progress at a POSIX timestamp, or the next
            /// one, and its kind under a naming table, as one UTF-8 line, returning
            /// the byte length written.
            ///
            /// `naming` is `panchak-five-kinds`, the five kinds Prokerala names,
            /// none on a Wednesday or a Thursday, or `panchak-raj-midweek`, Raj
            /// Panchak on those days too, as India TV has it, in any case; the
            /// `ayanamsa` is as for `hc_panchanga_at`; anything else is
            /// `HC_ERR_UNKNOWN`. The window is the Moon's passage from 300° to
            /// 360° of sidereal longitude, and a window takes its kind from the
            /// weekday it opens on, which the sources do not say is counted from
            /// midnight or from sunrise: here it is the weekday of the opening on a
            /// clock `offset_seconds` ahead of Universal Time, midnight to midnight,
            /// 19 800 for India's. Tab-separated: `1` when the timestamp is within
            /// the window, else `0`; the opening and the closing as whole POSIX
            /// seconds of Universal Time, rounded down; the weekday of the opening,
            /// Monday 1 to Sunday 7; and the kind's identifier (`rog`, `raj`,
            /// `agni`, `chor`, `mrityu`), its name in the locale and the tag that
            /// named it, all three empty on a weekday the table names no kind for.
            /// The locale argument fails as `hc_parse_iso_date` does. An offset of
            /// a day or more either way, or a timestamp outside the years −1000 to
            /// 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length
            /// the text needs.
        }
        fn hc_panchak(
            naming: name(naming_len),
            unix_seconds: i64,
            ayanamsa: name(ayanamsa_len),
            offset_seconds: i32,
            locale: text(locale_len),
        ) -> line =
            $crate::reckoning_lines::panchak_line;

        c {
            /// When in a Gregorian year the Sun, and the Moon where it is asked
            /// for, stand as a condition of the Kumbh Mela requires, and whether
            /// Jupiter's sign, which the caller gives, meets it, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `yoga` is a condition's identifier and `ayanamsa` as for
            /// `hc_panchanga_at`; null for either is `HC_ERROR_NULL_POINTER`. The
            /// library has no ephemeris of Jupiter, so `jupiter` is the sidereal
            /// sign Jupiter is in at the occasion's first moment, by the lower-case
            /// ASCII form of its Sanskrit name, or null or empty for none. A name
            /// not known is `HC_ERROR_UNKNOWN`. The line is the WebAssembly
            /// module's: the condition, the site and its name in the `locale` with
            /// the tag that named it, the river, the signs of Jupiter and the Sun,
            /// the new-moon flag, the occasion's first and last moments, and
            /// whether `jupiter` meets it. A year outside −1000 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// When in a Gregorian year the Sun, and the Moon where it is asked
            /// for, stand as a condition of the Kumbh Mela requires, and whether
            /// Jupiter's sign, which the caller gives, meets it, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// `yoga` is one of the Mela Adhikari's seven conditions,
            /// `kumbh-haridwar`, `kumbh-prayag-vrishabha`, `kumbh-prayag-mesha`,
            /// `kumbh-nashik-simha`, `kumbh-nashik-karka`, `kumbh-ujjain-simha` or
            /// `kumbh-ujjain-tula`, in any case; the `ayanamsa` is as for
            /// `hc_panchanga_at`. The library has no ephemeris of Jupiter, so
            /// `jupiter` is the caller's: the sidereal sign Jupiter is in at the
            /// occasion's first moment, in the same zodiac, as the lower-case
            /// ASCII form of its Sanskrit name, `mesha`, `vrishabha`, `mithuna`,
            /// `karka`, `simha`, `kanya`, `tula`, `vrishchika`, `dhanus`, `makara`,
            /// `kumbha` or `mina`, or empty for none. Any other name is
            /// `HC_ERR_UNKNOWN`. Tab-separated: the condition's identifier; the
            /// site's identifier (`haridwar`, `prayag`, `nashik`, `ujjain`), its
            /// name in the locale and the tag that named it; the river the site
            /// stands on, in English as the source gives it; the signs Jupiter and
            /// the Sun must be in; `1` when the Moon must be with the Sun at the
            /// new moon, else `0`; the occasion's first and last moments, the Sun's
            /// entry into its sign and into the next or the new moon twice, as
            /// whole POSIX seconds of Universal Time, rounded down, empty when the
            /// Sun's stay that year holds no new moon; and `1` when there is an
            /// occasion and `jupiter` is the condition's sign, else `0`, empty
            /// when `jupiter` is. The first moment does not depend on `jupiter`,
            /// so a caller may ask with it empty to learn where to read Jupiter.
            /// The line says whether the sky meets the condition, not when the
            /// bathing days the state announces are. The locale argument fails as
            /// `hc_parse_iso_date` does. A year outside −1000 to 3000 is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_kumbh(
            yoga: name(yoga_len),
            year: i64,
            ayanamsa: name(ayanamsa_len),
            jupiter: text(jupiter_len),
            locale: text(locale_len),
        ) -> line =
            $crate::reckoning_lines::kumbh_line;

        c {
            /// The twelve days of the *Ādi Pushkaram* of each river of a sidereal
            /// sign, for Jupiter's entry into it at a POSIX timestamp, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The library has no ephemeris of Jupiter, so the sign, named as for
            /// `hc_kumbh`, and the moment of the entry are the caller's. `meridian`
            /// is as for `hc_term_in_effect`; null for it or the sign is
            /// `HC_ERROR_NULL_POINTER`, and a name not known `HC_ERROR_UNKNOWN`.
            /// The lines are the WebAssembly module's: the river, its name in the
            /// `locale` and the tag that named it, its region, the sign, the first
            /// and last days, and the four cells of a missing solar event. A place
            /// off the globe, or a timestamp outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The twelve days of the *Ādi Pushkaram* of each river of a sidereal
            /// sign, for Jupiter's entry into it at a POSIX timestamp, as UTF-8
            /// lines, each river named in a locale, returning the byte length
            /// written.
            ///
            /// The library has no ephemeris of Jupiter, so the sign, named as for
            /// `hc_kumbh`, and the moment of the entry are the caller's; where
            /// Jupiter enters, turns back and enters again, the festival follows
            /// the second entry. The first day is the civil day of the entry at
            /// `meridian`, as for `hc_term_in_effect`, or the next when the entry
            /// falls after that day's sunset at the place, which is as for
            /// `hc_solar_time`: a reading fitted to the festivals whose dates were
            /// read, which no source read states. A sign or meridian not named is
            /// `HC_ERR_UNKNOWN`. One line per river of the sign, tab-separated: the
            /// river's identifier, `pushkaram-ganga` to `pushkaram-pranahita`; its
            /// name in the locale and the tag that named it; the region the source
            /// keeps it in for the sign, in English, empty where it names none; the
            /// sign's identifier; the first and last days as fixed days; and the
            /// four cells of `hc_solar_event` naming a missing solar event, the two
            /// days being empty instead where the Sun does not set on the day of
            /// the entry. The locale argument fails as `hc_parse_iso_date` does. A
            /// place off the globe, or a timestamp outside the years −1000 to 3000,
            /// is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_pushkaram(
            sign: name(sign_len),
            entry_unix_seconds: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            meridian: name(meridian_len),
            locale: text(locale_len),
        ) -> line =
            |sign, entry_unix_seconds, latitude, longitude, elevation, meridian, locale| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::reckoning_lines::pushkaram_lines(
                            sign,
                            entry_unix_seconds,
                            place,
                            meridian,
                            locale,
                        )
                    })
            };

        c {
            /// The folk reckonings of a fixed day outside the Japanese almanac, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer, one a
            /// reckoning, each named in a locale.
            ///
            /// The lines are the WebAssembly module's: the kind
            /// (`first-month-count`, `plum-rains`, `vietnamese-day`, `folk-half`,
            /// `folk-named-day`), the identifier, the name in the `locale` and the
            /// tag that named it, and the count. `meridian` is as for
            /// `hc_term_in_effect`; null is `HC_ERROR_NULL_POINTER` and a meridian
            /// not read `HC_ERROR_UNKNOWN`. A day outside the years −1000 to 3000
            /// is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// The folk reckonings of a fixed day outside the Japanese almanac, as
            /// UTF-8 lines, one a reckoning, each named in a locale, returning the
            /// byte length written.
            ///
            /// `meridian` is as for `hc_term_in_effect`, and sets where 芒种 and
            /// 小暑 fall; the published days are China's, `china`. The lines, in
            /// this order, tab-separated as the kind, the identifier, the name in
            /// the locale, the tag of the data that named it, and the count:
            /// `first-month-count`, four lines, `dragons`, `oxen`, `xin` and `cakes`
            /// — 几龙治水, 几牛耕田, 几日得辛 and 几人分饼 of the Chinese year the day
            /// is in, the day of 正月 on which its first 辰, 丑, 辛 and 丙 day falls,
            /// none outside 1645 to 2150; `plum-rains`, a line for each rule by
            /// which the day is 入梅 or 出梅, `ru-mei-bing`, the first 丙 day from
            /// 芒种, `ru-mei-ren`, the first 壬 day, or `chu-mei-wei`, the first 未
            /// day from 小暑, with no count; `vietnamese-day`, `tam-nuong` or
            /// `nguyet-ky`, when the day's number in the Vietnamese lunar calendar
            /// is one of theirs, with the number; `folk-half`, `hizir` from 6 May
            /// or `kasim` from 8 November, the half of the Turkish folk year the
            /// day is in, with its count from 1; and `folk-named-day`,
            /// `hidirellez`, `kasim`, `erbain`, `hamsin`, `cemre-air`,
            /// `cemre-water` or `cemre-earth`, when the day is one, with its count.
            /// Each kind is named in the language its source writes it in, which a
            /// locale without a name of its own, English's included, falls back to,
            /// and which `native` asks for first. The locale argument fails as
            /// `hc_parse_iso_date` does; a meridian not read is `HC_ERR_UNKNOWN`,
            /// and a day outside the years −1000 to 3000 `HC_ERR_OUT_OF_RANGE`. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_folk_day(fixed: i64, meridian: name(meridian_len), locale: text(locale_len)) -> line =
            $crate::reckoning_lines::folk_day_lines;

        c {
            /// The Chinese night watch and its points of a time of the civil clock
            /// by the fixed reckoning, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer, or the empty string by day.
            ///
            /// `seconds_of_day` is whole seconds after midnight of the caller's
            /// wall clock. The line is the WebAssembly module's: the watch, the
            /// points, the watch's name in the `locale` and the tag that named it,
            /// its Han name and its double hour. A time from 86 400 s is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The Chinese night watch, 更, and its points, 點, of a time of the
            /// civil clock by the fixed reckoning, as one UTF-8 line, or none by
            /// day, returning the byte length written.
            ///
            /// `seconds_of_day` is whole seconds after midnight of the caller's
            /// wall clock. The night from 19:00 to 05:00 is five watches of two
            /// hours, each of five points of 24 minutes; from 05:00 to 18:59 the
            /// text is empty. Tab-separated: the watch, 1 (一更) to 5 (五更); the
            /// points struck since it began, 0 to 4; the watch's name in the
            /// locale and the tag that named it; its Han name, 黃昏 to 平旦, and its
            /// double hour, 戌 to 寅, in Chinese as the source writes them. The
            /// locale argument fails as `hc_parse_iso_date` does. A time from
            /// 86 400 s is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_night_watch(seconds_of_day: u32, locale: text(locale_len)) -> line =
            $crate::reckoning_lines::night_watch_line;
    } };
    ("seasons", $backend:ident) => { $backend! {
        c {
            /// The solar term in effect on a fixed day at a meridian, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// Tab-separated: the term's index from 春分 at 0 through 驚蟄 at 23
            /// (the longitude divided by 15°), its name in traditional Chinese, its
            /// name in Japanese, the fixed day the term began at that meridian, the
            /// last fixed day before the next term begins, the authority for the
            /// Chinese names and the authority for the Japanese names. `meridian`
            /// is a NUL-terminated name — `universal`, `japan`, `china`, `korea`,
            /// `india` or `china-before-1929`, in any case — or a longitude in
            /// decimal degrees east of Greenwich, read as local mean solar time;
            /// null or empty is `universal`, anything else `HC_ERROR_UNKNOWN`, and
            /// text that is not UTF-8 `HC_ERROR_NOT_UTF8`. A day outside the years
            /// −1000 to 3000, the era `hc_sky_at` answers for, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The solar term in effect on a fixed day at a meridian, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// Tab-separated: the term's index from 春分 at 0 through 驚蟄 at 23
            /// (the longitude divided by 15°), its name in traditional Chinese, its
            /// name in Japanese, the fixed day the term began at that meridian, the
            /// last fixed day before the next term begins, the authority for the
            /// Chinese names and the authority for the Japanese names. `meridian`
            /// is a name — `universal`, `japan`, `china`, `korea`, `india` or
            /// `china-before-1929`, in any case, or empty for `universal` — or a
            /// longitude in decimal degrees east of Greenwich, read as local mean
            /// solar time; anything else is `HC_ERR_UNKNOWN`, and the pointer and
            /// bytes fail as for `hc_parse_iso_date`. A day outside the years −1000
            /// to 3000, the era `hc_sky_at` answers for, is `HC_ERR_OUT_OF_RANGE`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_term_in_effect(fixed: i64, meridian: text(meridian_len)) -> line =
            $crate::season_lines::term_line;

        c {
            /// The pentad (候) in effect on a fixed day at a meridian, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// Tab-separated: the pentad's index from the first pentad of 春分 at 0
            /// through 71 (the longitude divided by 5°), its name in the Chinese
            /// tradition, its name in the Japanese tradition, the fixed day the
            /// pentad began at that meridian, the last fixed day before the next
            /// pentad begins, the text the Chinese names come from and the text the
            /// Japanese names come from. `meridian` and the day are as for
            /// `hc_term_in_effect`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The pentad (候) in effect on a fixed day at a meridian, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// Tab-separated: the pentad's index from the first pentad of 春分 at 0
            /// through 71 (the longitude divided by 5°), its name in the Chinese
            /// tradition, its name in the Japanese tradition, the fixed day the
            /// pentad began at that meridian, the last fixed day before the next
            /// pentad begins, the text the Chinese names come from and the text the
            /// Japanese names come from. `meridian` and the day are as for
            /// `hc_term_in_effect`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_pentad_in_effect(fixed: i64, meridian: text(meridian_len)) -> line =
            $crate::season_lines::pentad_line;

        c {
            /// The fixed day of 寒食, the Cold Food Day, of a Gregorian year under
            /// a named reckoning.
            ///
            /// `convention` is a NUL-terminated name: `hanshi-solstice-105`, 105
            /// days after the winter solstice at the Chinese meridian, the
            /// reckoning before 1645; `hanshi-eve-of-qingming`, the day before 清明
            /// at the Chinese meridian, as kept after the 時憲曆 of 1645; or
            /// `hansik`, Korea's 한식, 105 days after 동지 at the Korean meridian;
            /// in any case. Any other name is `HC_ERROR_UNKNOWN`, null
            /// `HC_ERROR_NULL_POINTER`, and text that is not UTF-8
            /// `HC_ERROR_NOT_UTF8`. The solstice reckonings count from the solstice
            /// of the year before, so a year outside −999 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE` under every reckoning.
        }
        wasm {
            /// The fixed day of 寒食, the Cold Food Day, of a Gregorian year under
            /// a named reckoning, or an error sentinel.
            ///
            /// `convention` is `hanshi-solstice-105`, 105 days after the winter
            /// solstice at the Chinese meridian, the reckoning before 1645;
            /// `hanshi-eve-of-qingming`, the day before 清明 at the Chinese
            /// meridian, as kept after the 時憲曆 of 1645; or `hansik`, Korea's
            /// 한식, 105 days after 동지 at the Korean meridian; in any case. Any
            /// other name is `HC_ERR_UNKNOWN`, and the pointer and bytes fail as
            /// for `hc_parse_iso_date`. The solstice reckonings count from the
            /// solstice of the year before, so a year outside −999 to 3000 is
            /// `HC_ERR_OUT_OF_RANGE` under every reckoning.
        }
        fn hc_cold_food_day(convention: name(convention_len), year: i64) -> value(out_fixed: i64) =
            $crate::season_lines::cold_food_day;

        c {
            /// The fixed day of 入梅 or 出梅 of a Gregorian year by a named rule of
            /// the Chinese almanac, with the solar term it counts from at a
            /// meridian.
            ///
            /// `rule` is `ru-mei-bing`, `ru-mei-ren` or `chu-mei-wei`, as for the
            /// WebAssembly module's `hc_plum_rains`, and `meridian` as for
            /// `hc_term_in_effect`; null for either is `HC_ERROR_NULL_POINTER`, a
            /// name not known `HC_ERROR_UNKNOWN`, and text that is not UTF-8
            /// `HC_ERROR_NOT_UTF8`. A year outside −1000 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The fixed day of 入梅 or 出梅, the beginning or the end of the plum
            /// rains, of a Gregorian year by a named rule of the Chinese almanac,
            /// with the solar term it counts from at a meridian, or an error
            /// sentinel.
            ///
            /// `rule` is `ru-mei-bing`, 入梅 on the first 丙 day from 芒种, South
            /// China's; `ru-mei-ren`, the first 壬 day, Central China's; or
            /// `chu-mei-wei`, 出梅 on the first 未 day from 小暑, South China's; in
            /// any case, as 中国气象局's 「梅雨与农事」 gives them, the term's own
            /// day counted. Anything else is `HC_ERR_UNKNOWN`. `meridian` is as for
            /// `hc_term_in_effect`; the published days are China's, `china`. This
            /// is not the Japanese 入梅 at 80° of solar longitude. A year outside
            /// −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_plum_rains(
            rule: name(rule_len),
            year: i64,
            meridian: name(meridian_len),
        ) -> value(out_fixed: i64) =
            $crate::season_lines::plum_rains_day;
    } };
    ("holiday", $backend:ident) => { $backend! {
        c {
            /// Whether a fixed day is a day off in a holiday table.
            ///
            /// `code` is a NUL-terminated table identifier — a country's ISO 3166-1
            /// alpha-2 code, an exchange's ISO 10383 Market Identifier Code, a
            /// tradition's slug or `un-days` — `region`, which may be null, a
            /// subdivision's ISO 3166-2 code, and `group`, which may be null, the
            /// identifier of a group of people the table gives days to alone
            /// (`women`, `children`; column 10 of `hc_holiday_tables`). A null
            /// `group` asks for everyone's days, and so does a group the table
            /// gives no day to alone. Writes 1 or 0 to `out_is_day_off`.
            /// A null `code` or `out_is_day_off` is `HC_ERROR_NULL_POINTER`, a
            /// string that is not UTF-8 `HC_ERROR_NOT_UTF8`, and a code that names
            /// no table, or a `group` that names no group of `hc_holiday_groups`,
            /// `HC_ERROR_UNKNOWN`.
        }
        wasm {
            /// Whether a fixed day is a day off in a holiday table: 1, 0, or an
            /// error sentinel.
            ///
            /// `code` names the table — a country's ISO 3166-1 alpha-2 code, an
            /// exchange's ISO 10383 Market Identifier Code, a tradition's slug or
            /// `un-days` — `region`, which may be empty, a subdivision's ISO
            /// 3166-2 code, and `group`, which may be empty, the identifier of a
            /// group of people the table gives days to alone (`women`,
            /// `children`; column 10 of `hc_holiday_tables`); an empty `group`
            /// asks for everyone's days, and so does a group the table gives no day
            /// to alone. A null pointer with a non-zero length is
            /// `HC_ERR_NULL_POINTER`, text that is not UTF-8 `HC_ERR_NOT_UTF8`, and a
            /// code that names no table, or a `group` that names no group of
            /// `hc_holiday_groups`, `HC_ERR_UNKNOWN`.
        }
        fn hc_holiday_is_day_off(
            code: name(code_len),
            region: opt(region_len),
            group: opt(group_len),
            fixed: i64,
        ) -> value(out_is_day_off: int) =
            $crate::holiday_lines::is_day_off;

        c {
            /// A fixed day moved by a number of business days of a holiday table,
            /// in a subdivision and for a group.
            ///
            /// `code`, `region` and `group` are as for `hc_holiday_is_day_off`. A
            /// code that names no table is `HC_ERROR_UNKNOWN`, and a null `code`
            /// `HC_ERROR_NULL_POINTER`. A day with no Gregorian year, or a `count`
            /// past 36 500 either way, is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// A fixed day moved by a number of business days of a holiday table,
            /// in a subdivision and for a group, or an error sentinel.
            ///
            /// `code`, `region` and `group` are as for `hc_holiday_is_day_off`, so
            /// that a group's own days, China's half day for women, count as days
            /// off for that group alone. A business day is neither a holiday of the
            /// scope nor a weekend day the table does not make a working day, as
            /// China's 调休上班 are. A positive `count` moves forward and a negative
            /// one back; the starting day is never counted, and 0 returns it. A
            /// code that names no table is `HC_ERR_UNKNOWN`; a day with no
            /// Gregorian year, or a `count` past 36 500 either way, a hundred years'
            /// worth, is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_holiday_add_business_days(
            code: name(code_len),
            region: opt(region_len),
            group: opt(group_len),
            fixed: i64,
            count: i64,
        ) -> value(out_fixed: i64) =
            $crate::holiday_lines::add_business_days;

        c {
            /// The number of business days of a holiday table, in a subdivision
            /// and for a group, from one fixed day up to but not including another.
            ///
            /// As for `hc_holiday_add_business_days`; negative when `to_fixed` is
            /// before `from_fixed`. Two days more than a hundred years apart are
            /// `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The number of business days of a holiday table, in a subdivision
            /// and for a group, from one fixed day up to but not including another,
            /// or an error sentinel.
            ///
            /// The interval is half-open, so that two counts add: Monday to
            /// Wednesday and Wednesday to Friday make Monday to Friday. It is
            /// negative when `to_fixed` is before `from_fixed`. `code`, `region`
            /// and `group` are as for `hc_holiday_add_business_days`. Two days more
            /// than a hundred years apart, or one with no Gregorian year, are
            /// `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_holiday_business_days_between(
            code: name(code_len),
            region: opt(region_len),
            group: opt(group_len),
            from_fixed: i64,
            to_fixed: i64,
        ) -> value(out_count: i64) =
            $crate::holiday_lines::business_days_between;

        c {
            /// The holidays of a Gregorian year in a table, as NUL-terminated UTF-8
            /// lines in a caller-owned buffer.
            ///
            /// One line per entry, tab-separated: the ISO 8601 date, the name, the
            /// local name, the kind (`public`, `bank`, `religious`, `observance`,
            /// `school`, `workday`, `government` or `half-day`), the confidence
            /// (`exact` or
            /// `approximate`), `1` for a substitute day and `0` otherwise, the date
            /// the substitute stands in for or nothing, the subdivision whose own
            /// entry it is — `region` as the table writes it, `JP-13`, on an entry
            /// the calendar for no region does not have — or nothing, and the
            /// group whose own entry it is — `group` as the table writes it,
            /// `women`, on an entry the calendar for everyone does not have — or
            /// nothing. `region` and `group` match in either case, and either may
            /// be null. Writes the required length, including the terminator, into
            /// `written`. The string arguments fail as for `hc_holiday_is_day_off`.
        }
        wasm {
            /// The holidays of a Gregorian year in a table, as UTF-8 lines,
            /// returning the byte length written.
            ///
            /// One line per entry, tab-separated: the ISO 8601 date, the name, the
            /// local name, the kind (`public`, `bank`, `religious`, `observance`,
            /// `school`, `workday`, `government` or `half-day`), the confidence
            /// (`exact` or
            /// `approximate`), `1` for a substitute day and `0` otherwise, the date
            /// the substitute stands in for or nothing, the subdivision whose own
            /// entry it is — `region` as the table writes it, `JP-13`, on an entry
            /// the calendar for no region does not have — or nothing, and the
            /// group whose own entry it is — `group` as the table writes it,
            /// `women`, on an entry the calendar for everyone does not have — or
            /// nothing. `region` and `group` match in either case, and either may
            /// be empty. After the entries, one line per gap of the year, as
            /// `hc_holidays_on` writes them: an empty date, the name, the local
            /// name, the kind `gap`, an empty confidence, `0`, nothing, and the
            /// subdivision and the group whose own gap it is. A null `buffer`
            /// returns the length the text needs, so a caller can allocate
            /// exactly. The string arguments fail as for
            /// `hc_holiday_is_day_off`.
        }
        fn hc_holidays_in_year(
            code: name(code_len),
            region: opt(region_len),
            group: opt(group_len),
            year: i64,
        ) -> line =
            $crate::holiday_lines::holidays_in_year;

        c {
            /// The identifier of every holiday table, one per line, NUL-terminated.
            ///
            /// Countries first, then exchanges, traditions and the international
            /// sets, each as `hc_holiday_is_day_off` accepts it. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// The identifier of every holiday table, one per line, returning the
            /// byte length written.
            ///
            /// Countries first, then exchanges, traditions and the international
            /// sets, each as `hc_holiday_is_day_off` accepts it. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_holiday_codes() -> line = || Ok($crate::holiday_lines::holiday_codes());

        c {
            /// Every holiday on one fixed day across every table, as NUL-terminated
            /// UTF-8 lines in a caller-owned buffer.
            ///
            /// The tables are the ones `hc_holiday_codes` lists, in that order, each
            /// evaluated nationwide, then in each subdivision its rules name, in
            /// code order, then for each group of people its rules give days to
            /// alone, in identifier order, and last for each subdivision and group
            /// a rule names together. One line per (table, scope, entry),
            /// tab-separated:
            /// the table's identifier, its English name, the holiday's English
            /// name, its local name, the kind (`public`, `bank`, `religious`,
            /// `observance`, `school`, `workday`, `government`, `half-day`, or
            /// `gap`), the
            /// confidence (`exact` or `approximate`), the instrument the rule cites
            /// or nothing, `1` for a substitute day and `0` otherwise, the fixed day
            /// a substitute stands in for or nothing, the subdivision's ISO 3166-2
            /// code, `JP-13`, for an entry the nationwide calendar does not have, or
            /// nothing for a nationwide one, and the group's identifier, `women`,
            /// for an entry the calendar for everyone does not have, or nothing. A
            /// `gap` line is a holiday the table could
            /// not place in the day's year — its calendar's range ended, or no
            /// announcement was read — with the confidence and source empty, so a
            /// caller can say the year is unanswered rather than show nothing. A
            /// day with no Gregorian year is `HC_ERROR_OUT_OF_RANGE`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// Every holiday on one fixed day across every table, as UTF-8 lines,
            /// returning the byte length written.
            ///
            /// The tables are the ones `hc_holiday_codes` lists, in that order, each
            /// evaluated nationwide, then in each subdivision its rules name, in
            /// code order, then for each group of people its rules give days to
            /// alone, in identifier order, and last for each subdivision and group
            /// a rule names together. One line per (table, scope, entry),
            /// tab-separated:
            /// the table's identifier, its English name, the holiday's English
            /// name, its local name, the kind (`public`, `bank`, `religious`,
            /// `observance`, `school`, `workday`, `government`, `half-day`, or
            /// `gap`), the
            /// confidence (`exact` or `approximate`), the instrument the rule cites
            /// or nothing, `1` for a substitute day and `0` otherwise, the fixed day
            /// a substitute stands in for or nothing, the subdivision's ISO 3166-2
            /// code, `JP-13`, for an entry the nationwide calendar does not have, or
            /// nothing for a nationwide one, and the group's identifier, `women`,
            /// for an entry the calendar for everyone does not have, or nothing. A
            /// `gap` line is a holiday the table could
            /// not place in the day's year — its calendar's range ended, or no
            /// announcement was read — with the confidence and source empty, so a
            /// page can say the year is unanswered rather than show nothing. A day
            /// with no Gregorian year is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_holidays_on(fixed: i64) -> line = $crate::holiday_lines::holidays_on;

        c {
            /// Every holiday table with its kind, names and sources, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's: one per table in
            /// `hc_holiday_codes` order, with the code, the kind (`country`,
            /// `subdivision`, `exchange`, `tradition` or `observance`), the name in
            /// the locale, the English name, the locale that answered, the sources,
            /// the ISO 3166-1 country of a subdivision or an exchange where its
            /// table records one, the short name in the locale, the ISO 3166-2
            /// codes of the subdivisions its rules are scoped to, `;`-separated in
            /// code order, or nothing, the identifiers of the groups of people its
            /// rules give days to alone, `;`-separated in identifier order, or
            /// nothing, and those groups' names in the locale, in the same order,
            /// each its English name where `hc-i18n` carries none. A country is
            /// named by its CLDR 48 territory name
            /// in the `locale` where `hc-i18n` carries one, and else, as for a null
            /// `locale`, by CLDR's English one; every other table by its English
            /// name; the tag that answered is in column 5. Column 8 is CLDR 48's
            /// `alt="short"` name of a country, from the data that named it —
            /// `Hong Kong` for `HK` under `en`, 香港 under `ja` — and empty where
            /// the data has none and for every table that is not a country. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// Every holiday table with its kind, names and sources, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// One line per table, in the order `hc_holiday_codes` lists them,
            /// tab-separated: the code; the kind (`country`, `subdivision`,
            /// `exchange`, `tradition` or `observance`), read from the list the
            /// table is in; the name in the locale; the English name; the locale
            /// that answered; the sources the table names; and, for a subdivision
            /// or an exchange, the ISO 3166-1 country it belongs to as its table
            /// records it, else empty; the short name in the locale, else empty;
            /// the ISO 3166-2 codes of the subdivisions its rules are scoped to,
            /// `;`-separated in code order, else empty; the identifiers of the
            /// groups of people its rules give days to alone, `;`-separated in
            /// identifier order, else empty; and those groups' names in the locale,
            /// in the same order, each its English name where `hc-i18n` carries
            /// none. A country is named by its CLDR 48 territory
            /// name in the locale where `hc-i18n` carries one, a country the locale
            /// has no name for by CLDR's English one, and every other table by its
            /// English name; column 5 is the tag that answered. Column 8 is CLDR
            /// 48's `alt="short"` name of a country, from the data that named it —
            /// `Hong Kong` for `HK` under `en`, 香港 under `ja` — and empty where
            /// the data has none and for every table that is not a country. The locale argument fails as `hc_parse_iso_date`
            /// does. A null `buffer` returns the length the text needs.
        }
        fn hc_holiday_tables(locale: text(locale_len)) -> line =
            |locale| Ok($crate::holiday_lines::holiday_tables(locale));

        c {
            /// Every group of people a holiday may be given to alone, named in a
            /// locale, as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `locale` is a NUL-terminated
            /// BCP 47 tag, or null for the root locale. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Every group of people a holiday may be given to alone, named in a
            /// locale, as UTF-8 lines, returning the byte length written.
            ///
            /// One line per group, in `hc-holiday`'s order, tab-separated: the
            /// identifier, which `hc_holidays_on`'s group column writes (`women`);
            /// the name in the locale, where an instrument in the language names
            /// it (妇女 under `zh-Hans`), else empty; the tag that named it; and the
            /// English name. The locale argument fails as for `hc_parse_iso_date`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_holiday_groups(locale: text(locale_len)) -> line =
            |locale| Ok($crate::holiday_lines::holiday_groups_lines(locale));

        c {
            /// `hc_holidays_on`'s lines, each with the day's name in a locale and
            /// the tag that named it, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// `locale` is a NUL-terminated BCP 47 tag, or null for the root locale.
            /// A day with no Gregorian year is `HC_ERROR_OUT_OF_RANGE`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// `hc_holidays_on`'s lines, each with the day's name in a locale and
            /// the tag that named it, as UTF-8 lines, returning the byte length
            /// written.
            ///
            /// The eleven columns of `hc_holidays_on`, then two more: what the
            /// locale calls that day of that table, where a source in the language
            /// names it — the Bohairic Coptic names of the Coptic Orthodox feasts
            /// under `cop` — else empty; and the tag that named it. The locale
            /// argument fails as for `hc_parse_iso_date`. A day with no Gregorian
            /// year is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length
            /// the text needs.
        }
        fn hc_holidays_on_in(fixed: i64, locale: text(locale_len)) -> line =
            $crate::holiday_lines::holidays_on_in_lines;

        c {
            /// The lectionary cycles of a fixed day, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the liturgical year, the
            /// Sunday cycle `A`, `B` or `C`, the Roman weekday cycle `I` or `II`,
            /// the RCL's Proper for a Sunday after Trinity Sunday, else empty, the
            /// Roman Sunday in Ordinary Time, and the week of Ordinary Time on the
            /// universal calendar and on one that keeps the Epiphany on a Sunday.
            /// A day before 1 January 1970, when *Mysterii Paschalis* put the
            /// Roman calendar of 1969 into effect, or after the liturgical year
            /// 4099, is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The lectionary cycles of a fixed day, as one UTF-8 line, returning
            /// the byte length written.
            ///
            /// Tab-separated: the liturgical year, named by the civil year of its
            /// Easter and begun on the First Sunday of Advent before it; the Sunday
            /// cycle of the Roman Lectionary and the Revised Common Lectionary, `A`,
            /// `B` or `C`; the Roman weekday cycle, `I` or `II`; the RCL's
            /// numbered Proper, 3 to 29, for a Sunday after Trinity Sunday, else
            /// empty; the Roman number of a Sunday in Ordinary Time, 2 to 34, else
            /// empty; and the week of Ordinary Time, 1 to 34, on the universal
            /// calendar and on one that keeps the Epiphany on a Sunday, else empty.
            /// A day before 1 January 1970, when *Mysterii Paschalis* put the
            /// Roman calendar of 1969 into effect, or after the liturgical year
            /// 4099, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_lectionary(fixed: i64) -> line = $crate::holiday_lines::lectionary_line;

        c {
            /// The fixed day of Easter Sunday of a Gregorian year by the
            /// astronomical reckoning at the meridian of Jerusalem.
            ///
            /// The first Sunday after the day, by apparent solar time at Jerusalem,
            /// of the first full moon at or after the March equinox, as the World
            /// Council of Churches' Aleppo statement of 1997 proposed. A year
            /// outside 1583 to 2150 is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The fixed day of Easter Sunday of a Gregorian year by the
            /// astronomical reckoning at the meridian of Jerusalem, or an error
            /// sentinel.
            ///
            /// The first Sunday after the day, by apparent solar time at
            /// Jerusalem, of the first full moon at or after the March equinox, as
            /// the World Council of Churches' Aleppo statement of 1997 proposed. A
            /// year outside 1583 to 2150 is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_astronomical_easter(year: i64) -> value(out_fixed: i64) =
            $crate::holiday_lines::astronomical_easter;

        c {
            /// The fixed day of the paschal full moon of a Gregorian year by the
            /// astronomical reckoning at the meridian of Jerusalem.
            ///
            /// The day, by apparent solar time at Jerusalem, of the first full moon
            /// at or after the March equinox: the day `hc_astronomical_easter` is
            /// the first Sunday after, so a full moon on a Sunday puts Easter a
            /// week later. A year outside 1583 to 2150 is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The fixed day of the paschal full moon of a Gregorian year by the
            /// astronomical reckoning at the meridian of Jerusalem, or an error
            /// sentinel.
            ///
            /// The day, by apparent solar time at Jerusalem, of the first full moon
            /// at or after the March equinox: the day `hc_astronomical_easter` is
            /// the first Sunday after, so a full moon on a Sunday puts Easter a
            /// week later. A year outside 1583 to 2150 is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_astronomical_paschal_full_moon(year: i64) -> value(out_fixed: i64) =
            $crate::holiday_lines::astronomical_paschal_full_moon;

        c {
            /// The Holy Year of the Catholic Church a fixed day falls in, if any,
            /// as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: `within` or `outside`, then
            /// within a jubilee its title, kind, Pope and bull, the day the bull was
            /// given, its first and last days in Rome and in the dioceses. A day
            /// before 24 December 1974 or after 27 September 2026, the day the
            /// table's sources were checked, is `HC_ERROR_NO_DATA`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// The Holy Year of the Catholic Church a fixed day falls in, if any,
            /// as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: `within` when the day falls in a jubilee in Rome,
            /// from the opening of the Holy Door of St Peter's to its closing, else
            /// `outside`; then, within one, the jubilee's title, `ordinary` or
            /// `extraordinary`, the Pope who proclaimed it, the bull of indiction
            /// by its opening words, the day the bull was given and the jubilee's
            /// first and last days in Rome, as fixed days, and its first and last
            /// days in the dioceses where the bull dates them, else empty. Outside
            /// one the nine cells after the first are empty. The table holds the
            /// jubilees from 1975 to 2025; a day before 24 December 1974 or after
            /// 27 September 2026, the day its sources were checked, is
            /// `HC_ERR_NO_DATA`. A null `buffer` returns the length the text needs.
        }
        fn hc_holy_year_on(fixed: i64) -> line = $crate::holiday_lines::holy_year_line;

        c {
            /// The rank of every *Common Worship* celebration kept on a fixed day,
            /// as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one per celebration: its
            /// title, which is its name in `hc_holidays_on`'s `common-worship`
            /// table, the rank's identifier and the rank's English name. A day that
            /// keeps none writes an empty string, and a day with no Gregorian year
            /// is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// The rank of every *Common Worship* celebration kept on a fixed day,
            /// as UTF-8 lines, returning the byte length written.
            ///
            /// One line per Principal Feast, Principal Holy Day or Festival of the
            /// Church of England's calendar kept on the day after the transfers its
            /// Rules require, tab-separated: the title as the Rules print it, which
            /// is the name `hc_holidays_on` gives it in the `common-worship` table;
            /// the rank, `principal-feast`, `principal-holy-day` or `festival`; and
            /// the rank's English name. A day that keeps none writes nothing. The
            /// years the Rules leave a Festival without a day are `gap` lines of
            /// `hc_holidays_on`. A day with no Gregorian year is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_common_worship_on(fixed: i64) -> line = $crate::holiday_lines::common_worship_lines;


        c {
            /// What the Roman calendar of the 1960 rubrics does on a fixed day, the
            /// office kept, its commemorations and the days transferred or omitted,
            /// as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's: the role, the title, the
            /// class and its English name, and the day a transferred feast was
            /// impeded on. A day outside the years 1583 to 4099 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// What the Roman calendar of the 1960 rubrics does on a fixed day, the
            /// office kept, its commemorations and the days transferred or omitted,
            /// as UTF-8 lines, returning the byte length written.
            ///
            /// From `hc-holiday`'s `roman_calendar_1960`: the precedence of no. 91,
            /// the transfers of nos. 95–99, the vigils of no. 33 and the
            /// commemorations of nos. 106–114. The office's line first, then each
            /// commemoration in order, each feast of the I class impeded and
            /// transferred, and each listed day neither kept, commemorated nor
            /// transferred. Tab-separated: the role, `office`, `commemoration`,
            /// `transferred` or `omitted`; the title as the translation prints it;
            /// the class, `first`, `second`, `third`, `fourth` or `commemoration`,
            /// and its English name, `I class`; and, on the office's line, the fixed
            /// day a feast of the I class transferred here was impeded on, else
            /// empty. A day outside the years 1583 to 4099, whose Easter the
            /// Gregorian computus gives, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_roman_1960_office_on(fixed: i64) -> line = $crate::holiday_lines::roman_1960_office_lines;
        c {
            /// What a fixed day is in the fasting scheme of a reckoning, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `reckoning` is `orthodox-fasts`, `orthodox-fasts-revised-julian`,
            /// `armenian-fasts`, `armenian-fasts-jerusalem`,
            /// `armenian-fasts-fifty-days`, `coptic-fasts` or `ethiopian-fasts`,
            /// in any case; anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. The line is the WebAssembly module's: `1` or
            /// `0` for a fast day, `period`, `weekly-fast` or `none`, the period's
            /// identifier, English name and kind (`fast`, `fast-free` or
            /// `meat-excluded`), and what the day abstains from, `nothing`, `meat` or
            /// `fast`. A day outside the years
            /// 326 to 4099 of the reckoning's calendar, 1583 to 4099 for
            /// `armenian-fasts`, `armenian-fasts-fifty-days`, `coptic-fasts` and
            /// `ethiopian-fasts`, is
            /// `HC_ERROR_OUT_OF_RANGE`.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// What a fixed day is in the fasting scheme of a reckoning, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// `reckoning` is `orthodox-fasts`, the Eastern Orthodox fixed dates
            /// in the Julian calendar, or `orthodox-fasts-revised-julian`, in the
            /// Revised Julian calendar, both keeping Pascha by the Julian
            /// computus; `armenian-fasts`, the Armenian fasts on the Gregorian
            /// calendar and computus, or `armenian-fasts-jerusalem`, on the
            /// Julian, or `armenian-fasts-fifty-days`, with the weekly fasts
            /// lifted to Pentecost; `coptic-fasts` or `ethiopian-fasts`, dated in
            /// the Coptic and Ethiopic calendars with the Julian Pascha; in any case;
            /// anything else is `HC_ERR_UNKNOWN`. Tab-separated: `1` if the day is
            /// a fast day, else `0`; `period`, `weekly-fast` for a Wednesday or
            /// Friday in no period, or `none`; for a period its identifier, its
            /// English name, and its kind, `fast`,
            /// `fast-free` or `meat-excluded`, else empty; and what the day abstains
            /// from, `nothing`, `meat` or `fast`, which tells a day of the Meatfast
            /// from an ordinary one. A day outside the years 326 to 4099 of the
            /// reckoning's calendar, 1583 to 4099 for the four on the Gregorian
            /// calendar, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_orthodox_fast_on(reckoning: name(reckoning_len), fixed: i64) -> line =
            $crate::holiday_lines::orthodox_fast_line;

        c {
            /// The fasting seasons and fast-free weeks of a year of a reckoning, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// `reckoning` is as for `hc_orthodox_fast_on`. The lines are the
            /// WebAssembly module's, one per period: its identifier, English name
            /// and kind, and its first and last days, empty in a year it does not
            /// happen. A year outside the reckoning's, 326 to 4099 or 1583 to
            /// 4099, is `HC_ERROR_OUT_OF_RANGE`.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// The fasting seasons and fast-free weeks of a year of a reckoning, as
            /// UTF-8 lines, returning the byte length written.
            ///
            /// `reckoning` is as for `hc_orthodox_fast_on`, and `year` is a year of
            /// its calendar. One line per period of the scheme, twelve for the
            /// Eastern Orthodox, thirteen for the Armenian, nine for the Coptic and
            /// the Ethiopian, in the order a day is tested against them,
            /// tab-separated: its identifier,
            /// its English name, its kind (`fast`, `fast-free` or `meat-excluded`),
            /// and its first and last days as fixed
            /// days, both included, empty in a year it does not happen, as the
            /// Apostles' Fast does not on the Revised Julian reckoning when Pascha
            /// is late. Christmastide ends in the next year. A year outside the
            /// reckoning's, 326 to 4099 or 1583 to 4099, is `HC_ERR_OUT_OF_RANGE`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_orthodox_fast_seasons(reckoning: name(reckoning_len), year: i64) -> line =
            $crate::holiday_lines::orthodox_fast_seasons_lines;
    } };
    ("deep-time", $backend:ident) => { $backend! {
        c {
            /// A moment some years before the present, placed in every chronology
            /// at once, as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// One line per entry, tab-separated, every deep-time entry point alike: the
            /// kind (`moment`, `cosmic-epoch`, `cosmic-event`,
            /// `earliest-evidence`, `future-era`, a geologic rank `eon`, `era`,
            /// `period`, `epoch` or `age`, or `archaeological`), the entry's stable
            /// lower-case kebab identifier, the name, the scope (the identifier of
            /// the interval one rank up for a geologic interval, the region for an
            /// archaeological period, the landmark for an earliest-evidence claim),
            /// the older bound's value, standard uncertainty, significant figures
            /// and `1` where the table marks it approximate, the same four for the
            /// younger bound, the unit the values are in, the description, the
            /// source, and the name in the locale. A point in time has the same
            /// start and end; a minimum age has no start, and a standard
            /// uncertainty the source does not state is empty. The units are what
            /// each table counts in: `seconds-since-big-bang` for the cosmic rows,
            /// `megayears-before-present` for the geologic chart's, the
            /// `years-before-1950` of the BP convention for the archaeological and
            /// earliest-evidence rows, `log10-years-from-now` for a future era. The lines are the
            /// moment itself as `since-big-bang` and `before-present`, its cosmic
            /// epoch and the last dated cosmic event before it, its future era if
            /// it lies ahead, its geologic chain from eon down to age, and its
            /// archaeological period, each present only where that chronology
            /// reaches. `years_ago` counts back from the present as the crate
            /// defines it — the Planck 2018 age of the universe — not from 1950 and
            /// not from the caller's clock; the crate ignores the difference
            /// between the three, which lies below the smallest uncertainty in any
            /// of its tables. Negative years are the future: a moment up to a
            /// century ahead is still in the present intervals — the chart's
            /// youngest chain, the Modern period, the last cosmic epoch — as well
            /// as in its future era, and beyond a century it is in its future era
            /// alone. `locale` is a NUL-terminated BCP 47 tag, or null for none; the last column is the geologic
            /// chart's own name for an interval in that language, from the ICS's
            /// translations, or for a cosmic, archaeological or earliest-evidence
            /// row the established term `hc_deep_time::names` carries for it, and
            /// empty where neither has one. A value the crate refuses — not finite,
            /// beyond its range — is `HC_ERROR_OUT_OF_RANGE`; a `locale` that is not
            /// UTF-8 is `HC_ERROR_NOT_UTF8`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// A moment some years before the present, placed in every chronology
            /// at once, as UTF-8 lines, returning the byte length written.
            ///
            /// One line per entry, tab-separated, every deep-time export alike: the
            /// kind (`moment`, `cosmic-epoch`, `cosmic-event`,
            /// `earliest-evidence`, `future-era`, a geologic rank `eon`, `era`,
            /// `period`, `epoch` or `age`, or `archaeological`), the entry's stable
            /// lower-case kebab identifier, the name, the scope (the identifier of
            /// the interval one rank up for a geologic interval, the region for an
            /// archaeological period, the landmark for an earliest-evidence claim),
            /// the older bound's value, standard uncertainty, significant figures
            /// and `1` where the table marks it approximate, the same four for the
            /// younger bound, the unit the values are in, the description, the
            /// source, and the name in the locale. A point in time has the same
            /// start and end; a minimum age has no start, and a standard
            /// uncertainty the source does not state is empty. The units are what
            /// each table counts in: `seconds-since-big-bang` for the cosmic rows,
            /// `megayears-before-present` for the geologic chart's, the
            /// `years-before-1950` of the BP convention for the archaeological and
            /// earliest-evidence rows, `log10-years-from-now` for a future era. The lines are the
            /// moment itself as `since-big-bang` and `before-present`, its cosmic
            /// epoch and the last dated cosmic event before it, its future era if
            /// it lies ahead, its geologic chain from eon down to age, and its
            /// archaeological period, each present only where that chronology
            /// reaches. `years_ago` counts back from the present as the crate
            /// defines it — the Planck 2018 age of the universe — not from 1950 and
            /// not from the caller's clock; the crate ignores the difference
            /// between the three, which lies below the smallest uncertainty in any
            /// of its tables. Negative years are the future: a moment up to a
            /// century ahead is still in the present intervals — the chart's
            /// youngest chain, the Modern period, the last cosmic epoch — as well
            /// as in its future era, and beyond a century it is in its future era
            /// alone. `locale` is a BCP 47 tag; the last column is the geologic
            /// chart's own name for an interval in that language, from the ICS's
            /// translations, or for a cosmic, archaeological or earliest-evidence
            /// row the established term `hc_deep_time::names` carries for it, and
            /// empty where neither has one. A value the crate refuses — not finite,
            /// beyond its range — is `HC_ERR_OUT_OF_RANGE`; the locale argument
            /// fails as `hc_parse_iso_date` does. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_place_years_ago(
            years_ago: f64,
            std_dev_years: f64,
            locale: text(locale_len),
        ) -> line =
            |years_ago, std_dev_years, locale| {
                $crate::deep_time_lines::placement(years_ago, std_dev_years, locale)
                    .map_err(|_| $crate::boundary::Refusal::OutOfRange)
            };

        c {
            /// Every cosmic epoch and every dated cosmic event, as NUL-terminated
            /// UTF-8 lines in a caller-owned buffer.
            ///
            /// The epochs first, Big Bang to the present, then the events, oldest
            /// first, each a line of the columns `hc_place_years_ago` writes, in
            /// `seconds-since-big-bang`. `locale` is as for `hc_place_years_ago`,
            /// and names an entry where an established term is carried. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// Every cosmic epoch and every dated cosmic event, as UTF-8 lines,
            /// returning the byte length written.
            ///
            /// The epochs first, Big Bang to the present, then the events, oldest
            /// first, each a line of the columns `hc_place_years_ago` writes, in
            /// `seconds-since-big-bang`. `locale` is as for `hc_place_years_ago`,
            /// and names an entry where an established term is carried. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_cosmic_events(locale: text(locale_len)) -> line =
            |locale| Ok($crate::deep_time_lines::cosmic(locale));

        c {
            /// Every claim to the earliest evidence of life, of *Homo sapiens* and of
            /// writing, as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The claims to the earliest evidence of life, of *Homo sapiens* and of
            /// writing, grouped by landmark and oldest first, each a line of the
            /// columns `hc_place_years_ago` writes, in `years-before-1950`, with the
            /// landmark as the scope. Each keeps the shape of its source's date: an
            /// age has the same start and end, a minimum age no start, a range two
            /// different bounds; a standard uncertainty the source does not state
            /// is an empty cell, and a disputed claim names the rebuttal in its
            /// description. `locale` is as for `hc_place_years_ago`, and names a
            /// claim where an established term is carried.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// Every claim to the earliest evidence of life, of *Homo sapiens* and of
            /// writing, as UTF-8 lines, returning the byte length written.
            ///
            /// The claims to the earliest evidence of life, of *Homo sapiens* and of
            /// writing, grouped by landmark and oldest first, each a line of the
            /// columns `hc_place_years_ago` writes, in `years-before-1950`, with the
            /// landmark as the scope. Each keeps the shape of its source's date: an
            /// age has the same start and end, a minimum age no start, a range two
            /// different bounds; a standard uncertainty the source does not state
            /// is an empty cell, and a disputed claim names the rebuttal in its
            /// description. `locale` is as for `hc_place_years_ago`, and names a
            /// claim where an established term is carried.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_earliest_evidence(locale: text(locale_len)) -> line =
            |locale| Ok($crate::deep_time_lines::earliest_evidence(locale));

        c {
            /// Every conventional archaeological period, as NUL-terminated UTF-8
            /// lines in a caller-owned buffer.
            ///
            /// The conventional Southwest Asian and European sequence, youngest
            /// first, each a line of the columns `hc_place_years_ago` writes, in
            /// `years-before-1950`, with the region as the scope. `locale` is as for
            /// `hc_place_years_ago`, and names a period where an established term is
            /// carried.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// Every conventional archaeological period, as UTF-8 lines, returning
            /// the byte length written.
            ///
            /// The conventional Southwest Asian and European sequence, youngest
            /// first, each a line of the columns `hc_place_years_ago` writes, in
            /// `years-before-1950`, with the region as the scope. `locale` is as for
            /// `hc_place_years_ago`, and names a period where an established term is
            /// carried.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_archaeological_periods(locale: text(locale_len)) -> line =
            |locale| Ok($crate::deep_time_lines::archaeological_periods(locale));

        c {
            /// Every dated event of the far future, as NUL-terminated UTF-8 lines in
            /// a caller-owned buffer.
            ///
            /// The dated events of the far future, soonest first, each a line of
            /// the columns `hc_place_years_ago` writes, in `years-from-now`, with
            /// the kind of prediction as the scope: `modelled`,
            /// `experimental-bound` or `order-of-magnitude`. An experimental bound
            /// has a start and no end, because the event, if it happens, is no
            /// sooner. `locale` is as for `hc_place_years_ago`; no future event has
            /// a name in another language, so the last column is empty.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// Every dated event of the far future, as UTF-8 lines, returning the
            /// byte length written.
            ///
            /// The dated events of the far future, soonest first, each a line of
            /// the columns `hc_place_years_ago` writes, in `years-from-now`, with
            /// the kind of prediction as the scope: `modelled`,
            /// `experimental-bound` or `order-of-magnitude`. An experimental bound
            /// has a start and no end, because the event, if it happens, is no
            /// sooner. `locale` is as for `hc_place_years_ago`; no future event has
            /// a name in another language, so the last column is empty.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_future_events(locale: text(locale_len)) -> line =
            |locale| Ok($crate::deep_time_lines::future_events(locale));
    } };
    ("tz", $backend:ident) => { $backend! {
        c {
            /// The fixed day a POSIX timestamp falls on by the wall clock of a zone.
            ///
            /// `zone` is a NUL-terminated IANA name, `Asia/Tokyo`, in any case: one
            /// a caller has loaded through `hc_zone_load`, or else one of the
            /// seventeen the library carries with their current rules. A null
            /// `zone` or `out_fixed` is `HC_ERROR_NULL_POINTER`, a name neither
            /// knows `HC_ERROR_UNKNOWN`, an instant outside the years −9 999 994 to
            /// 9 999 994 by UTC, the ones a zone's rules answer for,
            /// `HC_ERROR_OUT_OF_RANGE`, and a name that is not UTF-8
            /// `HC_ERROR_NOT_UTF8`.
        }
        wasm {
            /// The fixed day a POSIX timestamp falls on by the wall clock of a
            /// zone, or an error sentinel.
            ///
            /// `zone` is an IANA name, `Asia/Tokyo`, in any case: one a page has
            /// loaded through `hc_zone_load`, or else one of the seventeen the
            /// module carries with their current rules. A name neither knows is
            /// `HC_ERR_UNKNOWN`, an instant outside the years −9 999 994 to
            /// 9 999 994 by UTC, the ones a zone's rules answer for,
            /// `HC_ERR_OUT_OF_RANGE`, and the name's pointer and bytes fail as for
            /// `hc_parse_iso_date`.
        }
        fn hc_fixed_from_unix_in_zone(
            unix_seconds: i64,
            zone: name(zone_len),
        ) -> value(out_fixed: i64) =
            $crate::zone_lines::day_in_zone;

        c {
            /// The POSIX timestamp at which a fixed day begins by the wall clock of
            /// a zone.
            ///
            /// The day begins at its local midnight. When the clocks go forward
            /// across that midnight, so that it does not exist, the day begins at
            /// the first instant after the gap; when they go back across it, at the
            /// earlier of the two midnights. `zone` is as for
            /// `hc_fixed_from_unix_in_zone`, and fails the same way.
            ///
            /// A day outside the years −9 999 994 to 9 999 994, before fixed day
            /// −3 652 423 173 or after 3 652 422 808, is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The POSIX timestamp at which a fixed day begins by the wall clock of
            /// a zone, or an error sentinel.
            ///
            /// The day begins at its local midnight. When the clocks go forward
            /// across that midnight, so that it does not exist, the day begins at
            /// the first instant after the gap — Cairo's first day of summer time
            /// begins at 01:00 EEST; when they go back across it, at the earlier of
            /// the two midnights. `zone` is as for `hc_fixed_from_unix_in_zone`,
            /// and fails the same way.
            ///
            /// A day outside the years −9 999 994 to 9 999 994, before fixed day
            /// −3 652 423 173 or after 3 652 422 808, is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_unix_from_fixed_in_zone(
            fixed: i64,
            zone: name(zone_len),
        ) -> value(out_unix_seconds: i64) =
            $crate::zone_lines::start_in_zone;

        c {
            /// The offset a zone keeps at a POSIX timestamp, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// `zone` is as for `hc_fixed_from_unix_in_zone`, read from the same
            /// rules, and fails the same way. The line is the WebAssembly module's:
            /// the offset in seconds east of UTC, `1` or `0` for daylight saving or
            /// summer time, the abbreviation the rules give or empty where they give
            /// a numeric one, the POSIX second of the next transition and the
            /// offset after it or both empty where the rules have none, and
            /// `builtin` or `loaded` for the rules that answered. An instant
            /// outside the years of `hc_fixed_from_unix_in_zone` is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The offset a zone keeps at a POSIX timestamp, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// `zone` is as for `hc_fixed_from_unix_in_zone`, read from the same
            /// rules, and fails the same way. Tab-separated: the offset, seconds
            /// east of UTC, which added to `unix_seconds` gives the wall-clock
            /// reading whose day `hc_fixed_from_unix_in_zone` names; `1` when the
            /// rules call the time daylight saving or summer time, else `0`; the
            /// abbreviation the rules give, `CET` or `MDT`, empty where they give a
            /// numeric one such as `+0545`; the POSIX second of the next transition,
            /// the first after `unix_seconds` at which the offset, the flag or the
            /// abbreviation changes, and the offset after it, both empty where the
            /// rules have none — a built-in zone without summer time, or a loaded
            /// file whose record ends with no footer; and which rules answered,
            /// `builtin` or `loaded`. An instant outside the years of
            /// `hc_fixed_from_unix_in_zone` is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_zone_offset(zone: name(zone_len), unix_seconds: i64) -> line =
            $crate::zone_lines::zone_offset_line;

        c {
            /// Every zone of the IANA database's `zone1970.tab` with its principal
            /// location, as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's: one per zone, in the table's
            /// order, with the zone, the latitude in decimal degrees north, the
            /// longitude in decimal degrees east, each the table's whole arcseconds
            /// to six places, the ISO 3166-1 codes of the
            /// countries it overlaps `;`-separated, the one country `zone.tab`
            /// lists the zone under (empty for a zone it has no row for), the
            /// table's comment, the zone's CLDR 48 exemplar city in the `locale`,
            /// and the tag of the data that named the city. A build without the
            /// `calendars` feature, a locale with no city for the zone, and a null
            /// `locale` name the city in English, with `en` in column 8. Writes the
            /// required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Every zone of the IANA database's `zone1970.tab` with its principal
            /// location, as UTF-8 lines, returning the byte length written.
            ///
            /// One line per zone, in the table's order, tab-separated: the zone;
            /// the latitude of its principal location, decimal degrees north; the
            /// longitude, decimal degrees east; the ISO 3166-1 codes of the
            /// countries it overlaps, `;`-separated, the country of the location
            /// first; the one country `zone.tab` lists the zone under, `JP` for
            /// `Asia/Tokyo` where column 4 is `JP;AU`, empty for a zone it has no
            /// row for, which no zone of release 2026d is; the table's comment,
            /// which tells apart a country's zones and is empty where the country
            /// has one; the zone's CLDR 48 exemplar city in the locale; and the tag
            /// of the data that named the city. Each
            /// coordinate is the table's whole arcseconds written in decimal degrees
            /// to six places, `arcseconds × 10⁶ / 3600` millionths rounded half away
            /// from zero, so that multiplying by 3600 and rounding gives the
            /// arcseconds back. A build
            /// without the `calendars` feature, and a locale with no city for the
            /// zone, names the city in English, with `en` in column 8. The locale
            /// argument fails as `hc_parse_iso_date` does. A null `buffer` returns
            /// the length the text needs.
        }
        fn hc_zones(locale: text(locale_len)) -> line =
            |locale| Ok($crate::zone_lines::zones(locale));

        c {
            /// Where one zone is, as the NUL-terminated UTF-8 line `hc_zones`
            /// writes for it, in a caller-owned buffer.
            ///
            /// `zone` is an IANA name in any case: a zone of `zone1970.tab`; a link
            /// `zone.tab` gives a place of its own, such as `Europe/Oslo`; or
            /// another link of the database's `backward` file, such as
            /// `Asia/Calcutta`, which answers with the line of the name it leads
            /// to, so that column 1 is then `Asia/Kolkata`. A name that places
            /// nothing, such as `UTC`, is `HC_ERROR_UNKNOWN`, and a null `zone`
            /// `HC_ERROR_NULL_POINTER`. `locale` is as for `hc_zones`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// Where one zone is, as the UTF-8 line `hc_zones` writes for it,
            /// returning the byte length written.
            ///
            /// `zone` is an IANA name in any case: a zone of `zone1970.tab`; a link
            /// `zone.tab` gives a place of its own, such as `Europe/Oslo`, which
            /// answers with Oslo and not with the zone it links to; or another
            /// link of the database's `backward` file, such as `Asia/Calcutta`,
            /// which answers with the line of the name it leads to, so that column
            /// 1 is then `Asia/Kolkata`. A name that places nothing, such as `UTC`,
            /// is `HC_ERR_UNKNOWN`. Both text arguments fail as for
            /// `hc_parse_iso_date`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_zone_location(zone: name(zone_len), locale: text(locale_len)) -> line =
            $crate::zone_lines::zone_location;
    } };
    ("sky", $backend:ident) => { $backend! {
        c {
            /// The Sun and the Moon at a POSIX timestamp, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// Tab-separated: the Sun's apparent longitude in degrees, the
            /// Earth–Sun distance in astronomical units, the Moon's apparent
            /// longitude and latitude in degrees and its distance in kilometres,
            /// the Moon's elongation from the Sun in degrees (0 at new moon, 180 at
            /// full), the illuminated fraction of its disc, the last new moon
            /// before the instant and the first at or after it as POSIX seconds,
            /// ΔT in seconds, the regime ΔT was answered from (`observed`,
            /// `predicted`, `fitted` or `extrapolated`) and a `source` naming the
            /// series each figure came from. The instant is read as Universal
            /// Time. An instant outside the years −1000 to 3000, the era over
            /// which `hc-astro` states its series valid, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The Sun and the Moon at a POSIX timestamp, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the Sun's apparent longitude in degrees, the
            /// Earth–Sun distance in astronomical units, the Moon's apparent
            /// longitude and latitude in degrees and its distance in kilometres,
            /// the Moon's elongation from the Sun in degrees (0 at new moon, 180 at
            /// full), the illuminated fraction of its disc, the last new moon
            /// before the instant and the first at or after it as POSIX seconds,
            /// ΔT in seconds, the regime ΔT was answered from (`observed`,
            /// `predicted`, `fitted` or `extrapolated`) and a `source` naming the
            /// series each figure came from. The instant is read as Universal
            /// Time. An instant outside the years −1000 to 3000, the era over
            /// which `hc-astro` states its series valid, is `HC_ERR_OUT_OF_RANGE`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_sky_at(unix_seconds: i64) -> line = $crate::sky_lines::sky_line;

        c {
            /// Every solar term whose instant falls in `[from_unix, to_unix)`, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// One line per term, in time order, tab-separated: the Sun's
            /// apparent longitude that defines the term in degrees (0 for 春分
            /// through 345), the instant as whole POSIX seconds, rounded down, in
            /// Universal Time, the term's name in traditional Chinese and its name
            /// in Japanese. The span is half-open and is refused with
            /// `HC_ERROR_OUT_OF_RANGE` when either end lies outside the years
            /// −1000 to 3000 or when it is longer than 400 years; `to_unix` at or
            /// before `from_unix` is an empty answer. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Every solar term whose instant falls in `[from_unix, to_unix)`, as
            /// UTF-8 lines, returning the byte length written.
            ///
            /// One line per term, in time order, tab-separated: the Sun's
            /// apparent longitude that defines the term in degrees (0 for 春分
            /// through 345), the instant as whole POSIX seconds, rounded down, in
            /// Universal Time, the term's name in traditional Chinese and its name
            /// in Japanese. The span is half-open and is refused with
            /// `HC_ERR_OUT_OF_RANGE` when either end lies outside the years −1000
            /// to 3000 or when it is longer than 400 years; `to_unix` at or before
            /// `from_unix` is an empty answer. A null `buffer` returns the length
            /// the text needs.
        }
        fn hc_solar_terms_between(from_unix: i64, to_unix: i64) -> line =
            $crate::sky_lines::term_lines;

        c {
            /// Every new moon, first quarter, full moon and last quarter whose
            /// instant falls in `[from_unix, to_unix)`, as NUL-terminated UTF-8
            /// lines in a caller-owned buffer.
            ///
            /// One line per phase, in time order, tab-separated: the Moon's
            /// elongation from the Sun that defines the phase in degrees (0, 90,
            /// 180 or 270), the instant as whole POSIX seconds, rounded down, in
            /// Universal Time, the phase's name (`new`, `first-quarter`, `full` or
            /// `last-quarter`) and an empty fourth column, so the lines have the
            /// shape of `hc_solar_terms_between`'s. The instants are the phase
            /// series of Meeus chapter 49, the same series that dates the new
            /// moons of `hc_sky_at`. The span fails as for
            /// `hc_solar_terms_between`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Every new moon, first quarter, full moon and last quarter whose
            /// instant falls in `[from_unix, to_unix)`, as UTF-8 lines, returning
            /// the byte length written.
            ///
            /// One line per phase, in time order, tab-separated: the Moon's
            /// elongation from the Sun that defines the phase in degrees (0, 90,
            /// 180 or 270), the instant as whole POSIX seconds, rounded down, in
            /// Universal Time, the phase's name (`new`, `first-quarter`, `full` or
            /// `last-quarter`) and an empty fourth column, so the lines have the
            /// shape of `hc_solar_terms_between`'s. The instants are the phase
            /// series of Meeus chapter 49, the same series that dates the new
            /// moons of `hc_sky_at`. The span fails as for
            /// `hc_solar_terms_between`. A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_moon_phases_between(from_unix: i64, to_unix: i64) -> line =
            $crate::sky_lines::phase_lines;

        c {
            /// The decan the Sun is in at a POSIX timestamp, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the tropical sign's number and
            /// English name, the decan within it, 1 to 3, its ruler's identifier and
            /// English name, and the degrees into the decan. An instant outside the
            /// years −1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// The decan the Sun is in at a POSIX timestamp, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the tropical sign, 1 for Aries through 12 for Pisces,
            /// and its English name; which of the sign's three 10° decans, or
            /// faces, the Sun is in, 1 to 3; the decan's ruler by al-Bīrūnī's
            /// table, the Chaldean order from Mars at the first face of Aries, as
            /// its identifier (`saturn`, `jupiter`, `mars`, `sun`, `venus`,
            /// `mercury`, `moon`) and its English name; and how far into the decan
            /// the Sun is, in degrees from 0 up to 10. The sign and the decan are
            /// read from the Sun's apparent longitude, so they turn at the solar
            /// terms. The instant is read as Universal Time; one outside the years
            /// −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_decan_at(unix_seconds: i64) -> line = $crate::sky_lines::decan_line;

        c {
            /// The drekkāṇa, the Hindu third of a sidereal sign, the Sun is in at a
            /// POSIX timestamp, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line is the WebAssembly module's: the sidereal sign's number and
            /// Sanskrit name, the drekkāṇa within it, its lord's identifier and
            /// English name, the degrees into it, the lord's sign and the ayanāṃśa.
            /// `ayanamsa` is `lahiri`, `raman`, `krishnamurti`,
            /// `reingold-dershowitz` or `fagan-bradley`, in any case; anything else
            /// is `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. An instant
            /// outside the years −1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes
            /// the required length, including the terminator, into `written`.
        }
        wasm {
            /// The drekkāṇa, the Hindu third of a sidereal sign, the Sun is in at a
            /// POSIX timestamp, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// `hc_decan_at`'s sidereal twin. Tab-separated: the sidereal sign, 1
            /// for Meṣa through 12 for Mīna, in the zodiac of the ayanāṃśa, and its
            /// Sanskrit name; which of its three 10° drekkāṇas the Sun is in, 1 to
            /// 3; the drekkāṇa's lord, the ruler of the sign it is given to — the
            /// sign itself, the fifth from it or the ninth, as al-Bīrūnī tabulates
            /// them — as its identifier and its English name; how far into the
            /// drekkāṇa the Sun is, in degrees from 0 up to 10; that sign by its
            /// identifier, `mesha`; and the ayanāṃśa's identifier. `ayanamsa` is
            /// `lahiri`, `raman`, `krishnamurti`, `reingold-dershowitz` or
            /// `fagan-bradley`, in any case; anything else, the empty string
            /// included, is `HC_ERR_UNKNOWN`. The instant is read as Universal
            /// Time; one outside the years −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_drekkana_at(unix_seconds: i64, ayanamsa: name(ayanamsa_len)) -> line =
            $crate::sky_lines::drekkana_line;

        c {
            /// Every named horizon a rising or a setting can be measured against,
            /// as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one per horizon: the
            /// identifier, the English name, what it takes the visible horizon to
            /// be, the source, a short English name for a label, the name in the
            /// `locale` and the tag of the data that named it, the English name
            /// with `en` where the locale has none. `locale` is a NUL-terminated
            /// BCP 47 tag or null, as for `hc_describe_day`; one that is not UTF-8
            /// is `HC_ERROR_NOT_UTF8`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Every named horizon a rising or a setting can be measured against,
            /// as UTF-8 lines, returning the byte length written.
            ///
            /// One line per horizon of `hc_astro::horizon::HORIZONS`,
            /// tab-separated: its identifier (`geometric-dip`, the default of the
            /// other exports; `usno`; `calendrical-calculations`), its English
            /// name, what it takes the visible horizon to be, its source, a short
            /// English name for a label (`geometric dip`, `USNO`, `Calendrical
            /// Calculations`), its name in the locale, and the tag of the data
            /// that named it. Only an observatory's or an almanac office's own
            /// wording is carried, so a horizon a locale has no name for is its
            /// English name, with `en`. The locale argument fails as
            /// `hc_parse_iso_date` does. A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_horizons(locale: text(locale_len)) -> line =
            |locale| Ok($crate::astro_lines::horizons_lines(locale));

        c {
            /// Sunrise on a fixed day at a place against a named horizon, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `horizon` is an identifier `hc_horizons` lists, in any case; anything
            /// else is `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. The line
            /// is the WebAssembly module's: the instant as POSIX seconds, the three
            /// cells of `hc_solar_event` naming a missing `sunrise`, and the
            /// altitude of the Sun's centre at the crossing. A place off the globe,
            /// or a day outside the years −1000 to 3000, is `HC_ERROR_OUT_OF_RANGE`.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// Sunrise on a fixed day at a place against a named horizon, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// `horizon` is an identifier `hc_horizons` lists, in any case; anything
            /// else, the empty string included, is `HC_ERR_UNKNOWN`. The day is the
            /// local one, from local mean midnight at the longitude. The place is
            /// the latitude and longitude in degrees, north and east positive, and
            /// the elevation in metres, which the horizon may or may not take into
            /// account. Tab-separated: the instant the Sun's upper limb rises over
            /// the horizon as whole POSIX seconds of Universal Time, rounded down;
            /// the three cells of `hc_solar_event` naming a missing solar event,
            /// `sunrise` and the day, which are empty when the Sun rises, the first
            /// cell being empty instead when it does not; and the altitude of the
            /// Sun's centre at the crossing in degrees, which the horizon and the
            /// elevation fix. A place off the globe, or a day outside the years
            /// −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_sunrise(
            horizon: name(horizon_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |horizon, fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::astro_lines::sunrise_line(horizon, fixed, place))
            };

        c {
            /// Sunset on a fixed day at a place against a named horizon, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// As `hc_sunrise`, for the upper limb's setting, with `sunset` as the
            /// missing event.
        }
        wasm {
            /// Sunset on a fixed day at a place against a named horizon, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// As `hc_sunrise`, for the upper limb's setting, with `sunset` as the
            /// missing event.
        }
        fn hc_sunset(
            horizon: name(horizon_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |horizon, fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::astro_lines::sunset_line(horizon, fixed, place))
            };

        c {
            /// A local clock's reading at a POSIX timestamp and a place, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `clock` is `local-mean`, `local-apparent`, `temporal` or `italian`,
            /// in any case; anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. The line is the WebAssembly module's: the
            /// local date's fixed day, the hours into it, and three cells naming a
            /// solar event the reading needs that does not happen, empty when it
            /// exists. A place off the globe, or an instant outside the years −1000
            /// to 3000, is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// A local clock's reading at a POSIX timestamp and a place, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// `clock` is `local-mean`, `local-apparent` (the sundial), `temporal`
            /// (unequal hours: 6 at sunrise, 18 at sunset) or `italian` (hours since
            /// the zero hour, half an hour after the sunset of the evening before),
            /// in any case; anything else is `HC_ERR_UNKNOWN`. The place is the
            /// latitude and longitude in degrees, north and east positive, and the
            /// elevation in metres. Tab-separated: the fixed day of the local date
            /// the reading belongs to, the hours into it, and three cells naming a
            /// solar event the reading needs that does not happen — `sunrise`,
            /// `sunset` or `depression`, the local day it is missing on, and the
            /// depression sought in arcminutes — which are empty when the reading
            /// exists; when it does not, the first two cells are empty instead. The
            /// timestamp is read as Universal Time. A place off the globe, or an
            /// instant outside the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_solar_time(
            clock: name(clock_len),
            unix_seconds: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |clock, unix_seconds, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::astro_lines::solar_time_line(clock, unix_seconds, place)
                    })
            };

        c {
            /// A named time of day on a fixed day at a place, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// `event` is `asr-shafii`, `asr-hanafi`, `jewish-dusk-vilna-gaon`,
            /// `jewish-sabbath-ends-cohn`, `italian-zero-hour`,
            /// `japanese-dawn-kansei`, `japanese-dusk-kansei`, `japanese-dawn-naoj`
            /// or `japanese-dusk-naoj`, in any case, with white space around it
            /// ignored; anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. The line is the WebAssembly module's: the
            /// instant as whole POSIX seconds of Universal Time, and the four cells
            /// naming a missing solar event, the depression sought in arcseconds
            /// last. A place off the
            /// globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A named time of day on a fixed day at a place, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// `event` is `asr-shafii` or `asr-hanafi`, the afternoon prayer when a
            /// shadow is its noon length plus once or twice the object's height;
            /// `jewish-dusk-vilna-gaon`, the Sun 4°40′ below the horizon;
            /// `jewish-sabbath-ends-cohn`, 7°5′; `italian-zero-hour`, half an hour
            /// after the Sun's centre is 16′ down; or the Japanese dawn and dusk,
            /// `japanese-dawn-kansei` and `japanese-dusk-kansei` at the 寛政暦's
            /// 7°21′41″, and `japanese-dawn-naoj` and `japanese-dusk-naoj` at the
            /// Observatory's 7°21′40″; in any case, with white space around it
            /// ignored, and anything else is `HC_ERR_UNKNOWN`. The place is as for
            /// `hc_solar_time`. Tab-separated: the instant as whole POSIX seconds of
            /// Universal Time, rounded down, then the three cells of
            /// `hc_solar_time` naming a missing solar event (`sunset`, `depression`
            /// or `no-noon-shadow`) and the depression sought in arcseconds, the one
            /// exact figure for the Japanese dawn and dusk, all four empty when the
            /// time exists; when it does not, the first cell is empty instead. A place off the globe, or a day outside the years
            /// −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_solar_event(
            event: name(event_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |event, fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::astro_lines::solar_event_line(event, fixed, place))
            };

        c {
            /// The Heliocentric Julian Date in TT, HJD_TT, of a Julian Date of TT
            /// for a target, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The target is its right ascension, 0 to 360 degrees, and its
            /// declination, −90 to 90 degrees, on the mean equator and equinox of
            /// J2000. The line is the WebAssembly module's: the HJD_TT and the
            /// light-time correction in seconds. A date outside the years −1000 to
            /// 3000 or not finite, or a direction outside those ranges, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The Heliocentric Julian Date in TT, HJD_TT, of a Julian Date of TT
            /// for a target, as one UTF-8 line, returning the byte length written.
            ///
            /// The target's direction is its right ascension, 0 to 360 degrees, and
            /// its declination, −90 to 90 degrees, on the mean equator and equinox
            /// of J2000; it is taken to be infinitely far away. Tab-separated: the
            /// HJD_TT, and the light-time correction added to the date, in seconds,
            /// negative when the light reaches the Sun before the Earth. The HJD
            /// is good to about 8 s, the Sun's own motion about the barycentre. A
            /// date outside the years −1000 to 3000 or not finite, or a direction
            /// outside those ranges, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_hjd_tt(tt_julian_date: f64, right_ascension: f64, declination: f64) -> line =
            $crate::astro_lines::hjd_tt_line;

        c {
            /// The Heliocentric Julian Date in UTC, HJD_UTC, of a Julian Date of
            /// UTC for a target, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// As `hc_hjd_tt`, with the Earth taken at the TT instant of the UTC
            /// date, TT − UTC being 32.184 s plus TAI − UTC from the leap-second
            /// table. The line is the WebAssembly module's: the HJD_UTC, the
            /// correction in seconds and the TT − UTC used. `strict` non-zero
            /// refuses a date outside the leap-second table with
            /// `HC_ERROR_NO_DATA`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The Heliocentric Julian Date in UTC, HJD_UTC, of a Julian Date of
            /// UTC for a target, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// As `hc_hjd_tt`, with the Earth's position taken at the TT instant of
            /// the UTC date: TT − UTC is 32.184 s plus TAI − UTC from the
            /// leap-second table. Tab-separated: the HJD_UTC, the correction added
            /// to the date in seconds, and the TT − UTC it used, in seconds.
            /// `strict` non-zero refuses a date outside the leap-second table,
            /// before 1961 or past its announced end, with `HC_ERR_NO_DATA`; zero
            /// holds the table's ends and takes TAI − UTC as 0 before 1961. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_hjd_utc(
            utc_julian_date: f64,
            right_ascension: f64,
            declination: f64,
            strict: flag,
        ) -> line =
            $crate::astro_lines::hjd_utc_line;

        c {
            /// The astronomical date and the Greenwich Mean Astronomical Time of a
            /// reading of GMT, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// GMAT is GMT − 12 h, its day beginning at noon and named by the civil
            /// day it begins on, as the *Nautical Almanac* counted G.M.T. to 1924.
            /// The reading is a fixed day of the Gregorian years −9 999 999 to
            /// 9 999 999, whole seconds after its midnight, 86 400 being 23:59:60,
            /// and attoseconds below 10¹⁸. The line is the WebAssembly module's:
            /// the astronomical day, the seconds after its noon and the
            /// attoseconds. 23:59:60, which has no reading twelve hours earlier,
            /// and a value out of those ranges are `HC_ERROR_OUT_OF_RANGE`. Writes
            /// the required length, including the terminator, into `written`.
        }
        wasm {
            /// The astronomical date and the Greenwich Mean Astronomical Time of a
            /// reading of GMT, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// GMT is mean solar time at Greenwich counted from midnight; GMAT
            /// counts it from noon, and names the astronomical day by the civil day
            /// it begins on, so GMAT is GMT − 12 h. The *Nautical Almanac* counted
            /// its G.M.T. so to 1924 and from midnight from 1925; which a document
            /// used is the document's to say. The reading is a fixed day of the
            /// Gregorian years −9 999 999 to 9 999 999, whole seconds after its
            /// midnight, 86 400 being 23:59:60, and attoseconds below 10¹⁸.
            /// Tab-separated: the astronomical day as a fixed day, the whole
            /// seconds after its noon, and the attoseconds. 23:59:60, which has no
            /// reading twelve hours earlier, and a value out of those ranges are
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_gmat_from_gmt(fixed: i64, seconds_of_day: u32, attoseconds: u64) -> line =
            $crate::astro_lines::gmat_from_gmt_line;

        c {
            /// The civil date and the GMT of a reading of Greenwich Mean
            /// Astronomical Time, the inverse of `hc_gmat_from_gmt`, as one
            /// NUL-terminated UTF-8 line in its columns in a caller-owned buffer.
            ///
            /// A second 60, which GMAT does not read, and a value out of
            /// `hc_gmat_from_gmt`'s ranges are `HC_ERROR_OUT_OF_RANGE`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// The civil date and the GMT of a reading of Greenwich Mean
            /// Astronomical Time, the inverse of `hc_gmat_from_gmt`, as one UTF-8
            /// line in its columns, returning the byte length written.
            ///
            /// The reading is the astronomical day as a fixed day, the whole
            /// seconds after its noon and the attoseconds, in the ranges of
            /// `hc_gmat_from_gmt`; a second 60, which GMAT does not read, and a
            /// value out of those ranges are `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_gmt_from_gmat(fixed: i64, seconds_of_day: u32, attoseconds: u64) -> line =
            $crate::astro_lines::gmt_from_gmat_line;

        c {
            /// The Islamic prayer times of a fixed day at a place by a named
            /// method, as eight NUL-terminated UTF-8 lines in a caller-owned
            /// buffer.
            ///
            /// `method` is an identifier `hc_prayer_methods` lists, in any case;
            /// anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. `ramadan` non-zero takes the Ramaḍān
            /// interval of a method that fixes *ʿishāʾ* after *maghrib*. The lines
            /// are the WebAssembly module's: `fajr`, `sunrise`, `zuhr`,
            /// `asr-shafii`, `asr-hanafi`, `maghrib`, `isha` and `midnight`, each
            /// with its instant and the four cells of a missing solar event. A
            /// place off the globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The Islamic prayer times of a fixed day at a place by a named
            /// method, as eight UTF-8 lines, returning the byte length written.
            ///
            /// `method` is an identifier `hc_prayer_methods` lists, in any case;
            /// anything else, the empty string included, is `HC_ERR_UNKNOWN`. The
            /// day and the place are as for `hc_solar_event`. `ramadan` non-zero
            /// takes the Ramaḍān interval of a method that fixes *ʿishāʾ* after
            /// *maghrib*; the caller says whether the day is in Ramaḍān. One line
            /// each of `fajr`, `sunrise`, `zuhr` (the Sun's transit), `asr-shafii`
            /// and `asr-hanafi` (the method does not choose between the shadow
            /// rules), `maghrib`, `isha` and `midnight` (the middle of the night
            /// that begins that evening), tab-separated: the time's identifier, the
            /// instant as whole POSIX seconds of Universal Time, rounded down, and
            /// the four cells of `hc_solar_event` naming a missing solar event,
            /// with the instant empty where the time does not happen. A place off
            /// the globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_prayer_times(
            method: name(method_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            ramadan: flag,
        ) -> line =
            |method, fixed, latitude, longitude, elevation, ramadan| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::hours_lines::prayer_times_lines(method, fixed, place, ramadan)
                    })
            };

        c {
            /// Every prayer-time method `hc_prayer_times` reads, with its
            /// parameters and source, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Every prayer-time method `hc_prayer_times` reads, with its
            /// parameters and source, as UTF-8 lines, returning the byte length
            /// written.
            ///
            /// One line per method, tab-separated: its identifier, its English
            /// name, the Sun's depression at *fajr* in arcminutes, the depression
            /// at *maghrib* in arcminutes (empty for sunset), the depression at
            /// *ʿishāʾ* in arcminutes (empty for a method of an interval), the
            /// interval after *maghrib* in minutes outside and during Ramaḍān
            /// (empty for a method of an angle), the middle of the night's rule,
            /// `sunset-to-sunrise` or `sunset-to-fajr`, and its source. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_prayer_methods() -> line = || Ok($crate::hours_lines::prayer_methods_lines());

        c {
            /// The Jewish times of a fixed day at a place by a reckoning, with the
            /// dawns and nightfalls, as nine NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// `reckoning` is `zmanim-gra`, `mga-72-minutes` or `mga-16-1-degrees`, in any
            /// case; anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. The lines are the WebAssembly module's. A
            /// place off the globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The Jewish times of a fixed day at a place by a reckoning, with the
            /// dawns and nightfalls, as nine UTF-8 lines, returning the byte length
            /// written.
            ///
            /// `reckoning` is `zmanim-gra`, the day from sunrise to sunset, or
            /// `mga-72-minutes` or `mga-16-1-degrees`, the Magen Avraham's from
            /// dawn to nightfall at 72 minutes or at 16.1°, in any case; anything
            /// else is `HC_ERR_UNKNOWN`. The day and the place are as for
            /// `hc_solar_event`. One line for each time in temporal hours,
            /// `sof-zman-shma`, `sof-zman-tfila`, `mincha-gedola`, `mincha-ketana`
            /// and `plag-hamincha`, tab-separated: its identifier, its English name
            /// as Hebcal prints it, its temporal hours from the start of the day,
            /// the instant as whole POSIX seconds of Universal Time, rounded down,
            /// and the four cells of `hc_solar_event` naming a missing solar event;
            /// then the same for `dawn-16-1-degrees`, `dawn-72-minutes`,
            /// `nightfall-8-5-degrees` and `nightfall-72-minutes`, which no
            /// reckoning changes, with the name and hours empty. A place off the
            /// globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_zmanim(
            reckoning: name(reckoning_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |reckoning, fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::hours_lines::zmanim_lines(reckoning, fixed, place))
            };

        c {
            /// The Edo 不定時法 reading of a POSIX timestamp at a place, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the day, the hour's place and
            /// name, its romaji, strokes and branch, the tenths and fraction of the
            /// hour gone, and the four cells of a missing solar event. A place off
            /// the globe, or an instant outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The Edo 不定時法 reading of a POSIX timestamp at a place, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// The daylight from 明け六つ to 暮れ六つ, at the 寛政暦's depression of
            /// 7°21′41″, is cut into six equal hours and the night into six more.
            /// The timestamp is read as Universal Time, and the place is as for
            /// `hc_solar_time`. Tab-separated: the fixed day whose 明け六つ began
            /// the reading's day; the hour's place in the count from 明け六つ, 0 to
            /// 11; its name, 明六つ to 暁七つ; its name in Hepburn romaji; the
            /// strokes of the bell it is named by; its earthly branch; the 天保暦's
            /// tenths of the hour gone, 0 to 9; and the fraction of the hour gone;
            /// then the four cells of `hc_solar_event` naming a missing solar
            /// event, the first eight cells being empty instead where a dawn or a
            /// dusk does not happen. A place off the globe, or an instant outside
            /// the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_edo_time(unix_seconds: i64, latitude: f64, longitude: f64, elevation: f64) -> line =
            |unix_seconds, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::hours_lines::edo_time_line(unix_seconds, place))
            };

        c {
            /// The instant of an Edo 不定時法 reading at a place, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The reading is the day whose 明け六つ begins it, the hour's place
            /// from 明け六つ, 0 to 11, and the fraction of the hour gone, from 0 up
            /// to 1. The line is the WebAssembly module's: the instant and the four
            /// cells of a missing solar event. An hour above 11, a fraction outside
            /// 0 to 1, a place off the globe, or a day outside the years −1000 to
            /// 3000, is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The instant of an Edo 不定時法 reading at a place, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// The reading is the fixed day whose 明け六つ begins it, the hour's
            /// place in the count from 明け六つ, 0 to 11, and the fraction of the
            /// hour gone, from 0 up to 1, as `hc_edo_time` writes them.
            /// Tab-separated: the instant as whole POSIX seconds of Universal Time,
            /// rounded down, and the four cells of `hc_solar_event` naming a
            /// missing solar event. An hour above 11, a fraction outside 0 to 1, a
            /// place off the globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_unix_from_edo_time(
            fixed: i64,
            hour: u32,
            fraction: f64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |fixed, hour, fraction, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::hours_lines::unix_from_edo_time_line(fixed, hour, fraction, place)
                    })
            };

        c {
            /// The planetary hour at a POSIX timestamp and a place, as one
            /// NUL-terminated UTF-8 line, its ruler named in a locale, in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the fixed day of the sunrise
            /// the planetary day began at, the hour, 1 to 24, the ruler's
            /// identifier, its name in the `locale` and the tag that named it, the
            /// daylight flag, the start and the end, and the four cells of a
            /// missing solar event. `locale` is a NUL-terminated BCP 47 tag,
            /// `native` or null, as for `hc_describe_day`. A place off the globe,
            /// or a timestamp outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The planetary hour at a POSIX timestamp and a place, as one UTF-8
            /// line, its ruler named in a locale, returning the byte length
            /// written.
            ///
            /// The daylight from sunrise to sunset and the night to the next
            /// sunrise are each twelve temporal hours, and each hour is ruled by a
            /// planet of the Chaldean order — Saturn, Jupiter, Mars, the Sun,
            /// Venus, Mercury, the Moon — from the weekday's own at sunrise, as
            /// al-Bīrūnī and Lilly give the rule. The timestamp is read as
            /// Universal Time, and the place is as for `hc_solar_time`.
            /// Tab-separated: the fixed day of the sunrise the planetary day began
            /// at, the small hours before sunrise being the day before's; the hour,
            /// 1 to 24 from sunrise, 1 to 12 of the daylight; the ruler's
            /// identifier, `sun` to `saturn`, its name in the locale and the tag of
            /// the data that named it; `1` for an hour of the daylight, else `0`;
            /// the hour's start and end as whole POSIX seconds of Universal Time,
            /// rounded down; and the four cells of `hc_solar_event` naming a
            /// missing solar event, every other cell being empty where a sunrise or
            /// sunset around the timestamp does not happen. The locale argument
            /// fails as `hc_parse_iso_date` does. A place off the globe, or a
            /// timestamp outside the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_planetary_hour(
            unix_seconds: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            locale: text(locale_len),
        ) -> line =
            |unix_seconds, latitude, longitude, elevation, locale| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::sky_lines::planetary_hour_line(unix_seconds, place, locale)
                    })
            };

        c {
            /// The twenty-four planetary hours of the planetary day that begins at
            /// the sunrise of a fixed day at a place, as NUL-terminated UTF-8 lines
            /// in `hc_planetary_hour`'s columns, each ruler named in a locale, in a
            /// caller-owned buffer.
            ///
            /// A place off the globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The twenty-four planetary hours of the planetary day that begins at
            /// the sunrise of a fixed day at a place, as UTF-8 lines in
            /// `hc_planetary_hour`'s columns, each ruler named in a locale,
            /// returning the byte length written.
            ///
            /// The rulers are the weekday's alone; where the sunrise or sunset an
            /// hour is counted from does not happen, its start and end are empty
            /// and the missing event is named. The locale argument fails as
            /// `hc_parse_iso_date` does. A place off the globe, or a day outside
            /// the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_planetary_hours_of_day(
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            locale: text(locale_len),
        ) -> line =
            |fixed, latitude, longitude, elevation, locale| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::sky_lines::planetary_hours_of_day_lines(fixed, place, locale)
                    })
            };
    } };
    ("orbital", $backend:ident) => { $backend! {
        c {
            /// Earth's orbital elements and the June insolation at 65° N at an
            /// epoch, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The epoch is in years before 1950, negative for the future, as
            /// `hc-orbital` counts. Tab-separated: the eccentricity and its spread,
            /// the obliquity in degrees and its spread, the longitude of perihelion
            /// from the moving equinox in degrees (heliocentric, about 102° at
            /// present) and its spread, the climatic precession *e* sin ϖ and its
            /// spread, the daily mean insolation at 65° N at the June solstice in
            /// W m⁻², the solar constant that insolation was computed with in W m⁻²
            /// and the source, which names the series and the constant. Each
            /// spread is the measured disagreement between this series and Berger
            /// & Loutre's 1991 solution for the tier of the span the epoch falls
            /// in, not a Gaussian width; the perihelion's is 180° where the
            /// eccentricity is smaller than the precession's spread. An epoch that
            /// is not finite or lies beyond a million years either side of 1950 is
            /// `HC_ERROR_OUT_OF_RANGE`: the series would return numbers there, and
            /// they would be fiction. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Earth's orbital elements and the June insolation at 65° N at an
            /// epoch, as one UTF-8 line, returning the byte length written.
            ///
            /// The epoch is in years before 1950, negative for the future, as
            /// `hc-orbital` counts. Tab-separated: the eccentricity and its spread,
            /// the obliquity in degrees and its spread, the longitude of perihelion
            /// from the moving equinox in degrees (heliocentric, about 102° at
            /// present) and its spread, the climatic precession *e* sin ϖ and its
            /// spread, the daily mean insolation at 65° N at the June solstice in
            /// W m⁻², the solar constant that insolation was computed with in W m⁻²
            /// and the source, which names the series and the constant. Each
            /// spread is the measured disagreement between this series and Berger
            /// & Loutre's 1991 solution for the tier of the span the epoch falls
            /// in, not a Gaussian width; the perihelion's is 180° where the
            /// eccentricity is smaller than the precession's spread. An epoch that
            /// is not finite or lies beyond a million years either side of 1950 is
            /// `HC_ERR_OUT_OF_RANGE`: the series would return numbers there, and
            /// they would be fiction. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_orbit_at(years_before_1950: f64) -> line = $crate::orbital_lines::orbit_line;

        c {
            /// The line of `hc_orbit_at` at every epoch from `from_years_before_1950`
            /// to `to_years_before_1950` in steps of `step_years`, each with the
            /// epoch as a first column, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// One line per sample, tab-separated: the epoch in years before 1950,
            /// then the eleven columns of `hc_orbit_at`. The samples are `from`,
            /// `from + step`, `from + 2 step` and so on, every one at or before
            /// `to`. Both ends have to lie within a million years either side of
            /// 1950 and `step` has to be finite and positive, else
            /// `HC_ERROR_OUT_OF_RANGE`; more than 10 000 samples is
            /// `HC_ERROR_OUT_OF_RANGE` too, and a caller who wants more asks in
            /// pieces. A `to` before `from` is an empty answer, not an error. Writes
            /// the required length, including the terminator, into `written`.
        }
        wasm {
            /// The line of `hc_orbit_at` at every epoch from `from_years_before_1950`
            /// to `to_years_before_1950` in steps of `step_years`, each with the
            /// epoch as a first column, as UTF-8 lines, returning the byte length
            /// written.
            ///
            /// One line per sample, tab-separated: the epoch in years before 1950,
            /// then the eleven columns of `hc_orbit_at`. The samples are `from`,
            /// `from + step`, `from + 2 step` and so on, every one at or before
            /// `to`, so `from` and `to` themselves are samples when `to - from` is
            /// a multiple of `step`, and a page drawing across the span asks once
            /// rather than once per column. Both ends have to lie within a million
            /// years either side of 1950 and `step` has to be finite and positive,
            /// else `HC_ERR_OUT_OF_RANGE`; more than 10 000 samples is
            /// `HC_ERR_OUT_OF_RANGE` too, and a caller who wants more asks in
            /// pieces. A `to` before `from` is an empty answer of zero bytes, not an
            /// error. A null `buffer` returns the length the text needs.
        }
        fn hc_orbit_series(
            from_years_before_1950: f64,
            to_years_before_1950: f64,
            step_years: f64,
        ) -> line =
            $crate::orbital_lines::series_lines;
    } };
    ("planetary", $backend:ident) => { $backend! {
        c {
            /// Mars at a POSIX instant and an east longitude, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the Mars Sol Date;
            /// Coordinated Mars Time, local mean solar time and local true solar
            /// time at the longitude, each as `HH:MM:SS` on the 24-hour Martian
            /// clock (truncated) and in decimal Martian hours; the equation of time
            /// in Martian minutes; the areocentric solar longitude `Ls` in degrees;
            /// the Mars year under the Clancy convention; the Darian year, month,
            /// sol of the month, month name and sol-of-week name at Airy-0; and the
            /// source. `unix_seconds` is POSIX time with a fraction, read through
            /// the leap-second table with the last offset held;
            /// `east_longitude_degrees` is planetocentric, east-positive, and wraps. An
            /// instant more than 100 Julian years from J2000.0, where Allison and
            /// McEwen's series is an extrapolation, or a value that is not finite,
            /// is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// Mars at a POSIX instant and an east longitude, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the Mars Sol Date; Coordinated Mars Time, local mean
            /// solar time and local true solar time at the longitude, each as
            /// `HH:MM:SS` on the 24-hour Martian clock (truncated) and in decimal
            /// Martian hours; the equation of time in Martian minutes; the
            /// areocentric solar longitude `Ls` in degrees; the Mars year under the
            /// Clancy convention; the Darian year, month, sol of the month, month
            /// name and sol-of-week name at Airy-0; and the source. `unix_seconds`
            /// is POSIX time with a fraction, read through the leap-second table
            /// with the last offset held; `east_longitude_degrees` is planetocentric,
            /// east-positive, and wraps. An instant more than 100 Julian years from
            /// J2000.0, where Allison and McEwen's series is an extrapolation, or a
            /// value that is not finite, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_mars_time(unix_seconds: f64, east_longitude_degrees: f64) -> line =
            $crate::planetary_lines::mars_time_line;

        c {
            /// Every surface mission on Mars and the rules of its sol count, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one per mission in landing
            /// order: the identifier, the name, the landing instant as UTC text and
            /// as a POSIX timestamp, the number of the landing sol (0 or 1), the
            /// clock's midnight (`local-mean-solar-time`, or
            /// `local-true-solar-time-at-landing`), the clock meridian's east
            /// longitude, the achieved site's east longitude, `1` where the
            /// operators published the convention and `0` otherwise, the note and
            /// the source. Where no convention was published the landing sol, the
            /// clock and its meridian are empty. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Every surface mission on Mars and the rules of its sol count, as
            /// UTF-8 lines, returning the byte length written.
            ///
            /// One line per mission, in landing order, tab-separated: the
            /// identifier, the name, the landing instant as UTC text and as a POSIX
            /// timestamp, the number of the landing sol (0 or 1), the clock's
            /// midnight (`local-mean-solar-time`, or
            /// `local-true-solar-time-at-landing`), the clock meridian's east
            /// longitude, the achieved site's east longitude, `1` where the
            /// operators published the convention and `0` otherwise, the note and
            /// the source. Where no convention was published the landing sol, the
            /// clock and its meridian are empty. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_missions() -> line = || Ok($crate::planetary_lines::missions_lines());

        c {
            /// The sol number of a Mars surface mission at a POSIX instant, by the
            /// mission's own clock.
            ///
            /// `mission` is a NUL-terminated identifier `hc_missions` lists,
            /// `viking-1` or `curiosity`, in any ASCII case; a name such as
            /// `Viking 1` is `HC_ERROR_UNKNOWN`. The sol is counted as the mission counted
            /// it: from the midnight, mean or true, on the mission's clock meridian
            /// that began the landing sol, which is sol 0 or sol 1 as the operators
            /// numbered it. A mission the table does not carry is
            /// `HC_ERROR_UNKNOWN`; one whose operators published no sol numbering
            /// is `HC_ERROR_NO_DATA`; an instant before the landing sol began, or
            /// outside `hc_mars_time`'s span, is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The sol number of a Mars surface mission at a POSIX instant, by the
            /// mission's own clock, or an error sentinel.
            ///
            /// `mission` is an identifier `hc_missions` lists, `viking-1` or
            /// `curiosity`, in any ASCII case; a name such as `Viking 1` is
            /// `HC_ERR_UNKNOWN`. The sol is counted as the mission counted it: from the
            /// midnight, mean or true, on the mission's clock meridian that began
            /// the landing sol, which is sol 0 or sol 1 as the operators numbered
            /// it. A mission the table does not carry is `HC_ERR_UNKNOWN`; one
            /// whose operators published no sol numbering is `HC_ERR_NO_DATA`; an
            /// instant before the landing sol began, or outside `hc_mars_time`'s
            /// span, is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_mission_sol(mission: name(mission_len), unix_seconds: f64) -> value(out_sol: i64) =
            $crate::planetary_lines::mission_sol;

        c {
            /// Every body `hc-planetary` carries, with its solar day, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one per body outward from
            /// the Sun with each planet's moons after it: the identifier, the name,
            /// the kind (`star`, `planet`, `dwarf-planet` or `moon`), the
            /// identifier of the body it orbits, the sidereal rotation period in
            /// hours (negative for a retrograde rotator), the solar day in SI
            /// seconds, `measured` or `derived`, the year in local solar days,
            /// whether the clock's zero point is a `standard` or a `convention`
            /// this library declares, what the zero point is, the source, and the
            /// status of a standard still being drawn up (the Moon's Coordinated
            /// Lunar Time). The Sun's three day cells are empty. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// Every body `hc-planetary` carries, with its solar day, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// One line per body, outward from the Sun with each planet's moons
            /// after it, tab-separated: the identifier, the name, the kind
            /// (`star`, `planet`, `dwarf-planet` or `moon`), the identifier of the
            /// body it orbits, the sidereal rotation period in hours (negative for
            /// a retrograde rotator), the solar day in SI seconds, `measured` or
            /// `derived`, the year in local solar days, whether the clock's zero
            /// point is a `standard` or a `convention` this library declares, what
            /// the zero point is, the source, and the status of a standard still
            /// being drawn up (the Moon's Coordinated Lunar Time). The Sun has no
            /// solar day, and its three day cells are empty. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_bodies() -> line = || Ok($crate::planetary_lines::bodies_lines());

        c {
            /// Local mean solar time on a body at a POSIX instant and an east
            /// longitude, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line is the WebAssembly module's: the local day number, the
            /// fraction of it elapsed, the reading as `HH:MM:SS` (truncated) and in
            /// decimal local hours on a 24-hour face, the solar day and the local
            /// hour in SI seconds, whether the zero point is a `standard` or a
            /// `convention`, and what it is. `body` is a NUL-terminated identifier
            /// `hc_bodies` lists, `mars` or `titan`, in any ASCII case; a body it does not
            /// list is `HC_ERROR_UNKNOWN`, and the Sun, which has no solar day,
            /// `HC_ERROR_NO_DATA`. The instant and the longitude fail as for
            /// `hc_mars_time`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Local mean solar time on a body at a POSIX instant and an east
            /// longitude, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the local day number, the fraction of it elapsed,
            /// the reading as `HH:MM:SS` (truncated) and in decimal local hours on
            /// a 24-hour face, the solar day and the local hour in SI seconds,
            /// whether the zero point is a `standard` or a `convention`, and what
            /// it is. `body` is an identifier `hc_bodies` lists, `mars` or `titan`,
            /// in any ASCII case; a body it does not list is `HC_ERR_UNKNOWN`, and the
            /// Sun, which has no solar day, is `HC_ERR_NO_DATA`. The instant and
            /// the longitude fail as for `hc_mars_time`. A null `buffer` returns
            /// the length the text needs.
        }
        fn hc_body_time(
            body: name(body_len),
            unix_seconds: f64,
            east_longitude_degrees: f64,
        ) -> line =
            $crate::planetary_lines::body_time_line;

        c {
            /// The date at a POSIX instant in a calendar of another body's days, as
            /// one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the calendar, the year, the
            /// month, the day of the month, the month's name, the name of the day's
            /// place in the week, the day count the date is numbered by (a circad
            /// number, or Martiana's Darian sol number), the fraction of that day
            /// elapsed, `1` for a leap year, and the source. `calendar` is
            /// `darian-titan`, `gregorian-io`, `gregorian-europa`,
            /// `gregorian-ganymede`, `gregorian-callisto` or `martiana`, in any
            /// ASCII case; another is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. The instant fails as for `hc_mars_time`.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// The date at a POSIX instant in a calendar of another body's days, as
            /// one UTF-8 line, returning the byte length written.
            ///
            /// `calendar` is `darian-titan`, `gregorian-io`, `gregorian-europa`,
            /// `gregorian-ganymede`, `gregorian-callisto` or `martiana`, in any
            /// ASCII case; another is `HC_ERR_UNKNOWN`. Tab-separated: the
            /// calendar, the year, the month, the day of the month, the month's
            /// name, the name of the day's place in the week, the day count the
            /// date is numbered by, the fraction of that day elapsed, `1` for a
            /// leap year, and the source. Titan's and the Galilean moons' days are
            /// *circads*, fixed fractions of the moon's solar day in weeks of
            /// eight, counted from the calendar's epoch by Gangale's calibration;
            /// Martiana's are sols at Airy-0 in weeks of seven, counted as the
            /// Darian sol number, with an empty week cell on the epagomenal sol of
            /// every tenth year. The instant is read as for `hc_mars_time`, and one
            /// more than 100 Julian years from J2000.0, or not finite, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_circad_date(calendar: name(calendar_len), unix_seconds: f64) -> line =
            $crate::planetary_lines::circad_date_line;
    } };
    ("relativity", $backend:ident) => { $backend! {
        c {
            /// A clock moving at a constant speed while some coordinate time
            /// passes, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: β, the Lorentz factor γ, the
            /// proper time the moving clock records in seconds, its rate
            /// dτ/dt = 1/γ, that rate's offset from 1 in microseconds per
            /// 86 400-second day (negative, computed without cancellation), the
            /// `hc-relativity` constant used (`SPEED_OF_LIGHT`), and the source. A
            /// speed at or beyond the speed of light either way, or a value that is
            /// not finite, is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// A clock moving at a constant speed while some coordinate time
            /// passes, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: β, the Lorentz factor γ, the proper time the moving
            /// clock records in seconds, its rate dτ/dt = 1/γ, that rate's offset
            /// from 1 in microseconds per 86 400-second day (negative, computed
            /// without cancellation), the `hc-relativity` constant used
            /// (`SPEED_OF_LIGHT`), and the source. A speed at or beyond the speed
            /// of light either way, or a value that is not finite, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_proper_time(speed_metres_per_second: f64, coordinate_seconds: f64) -> line =
            $crate::relativity_lines::proper_time_line;

        c {
            /// A clock held still at a radius from a body's centre, against one far
            /// from every mass, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line is the WebAssembly module's: the body's identifier, its GM
            /// in m³ s⁻², the name of the `hc-relativity` constant that holds it,
            /// the Schwarzschild radius in metres, the static dilation factor
            /// dτ/dt = √(1 − r_s/r), that factor's offset from 1 in microseconds
            /// per 86 400-second day (negative, computed without cancellation), the
            /// constants used, separated by `;`, and the body's source. `body` is a NUL-terminated
            /// identifier `hc_gravitating_bodies` lists, `earth` or
            /// `sagittarius-a-star`, in any ASCII case; another, a name such as
            /// `Sagittarius A*` included, is `HC_ERROR_UNKNOWN`. A radius that is not finite, not
            /// positive, or at or inside the Schwarzschild radius is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A clock held still at a radius from a body's centre, against one far
            /// from every mass, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// Tab-separated: the body's identifier, its GM in m³ s⁻², the name of
            /// the `hc-relativity` constant that holds it, the Schwarzschild radius
            /// in metres, the static dilation factor dτ/dt = √(1 − r_s/r), that
            /// factor's offset from 1 in microseconds per 86 400-second day
            /// (negative, computed without cancellation), the constants used,
            /// separated by `;`, and the body's source. `body` is an identifier
            /// `hc_gravitating_bodies` lists, `earth` or `sagittarius-a-star`, in
            /// any ASCII case; another, a name such as `Sagittarius A*` included,
            /// is `HC_ERR_UNKNOWN`. A radius that is not finite, not positive, or at
            /// or inside the Schwarzschild radius is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_gravitational_dilation(body: name(body_len), radius_metres: f64) -> line =
            $crate::relativity_lines::gravitational_dilation_line;

        c {
            /// Every body `hc-relativity` carries a gravitational parameter for, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one per body: the
            /// identifier, the English name, GM in m³ s⁻², the name of the
            /// `hc-relativity` constant that holds it, and the source. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// Every body `hc-relativity` carries a gravitational parameter for, as
            /// UTF-8 lines, returning the byte length written.
            ///
            /// One line per body, tab-separated: the identifier, the English name,
            /// GM in m³ s⁻², the name of the `hc-relativity` constant that holds
            /// it, and the source. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_gravitating_bodies() -> line =
            || Ok($crate::relativity_lines::gravitating_bodies_lines());
    } };
    ("places", $backend:ident) => { $backend! {
        c {
            /// Every territory CLDR 48 names, with its name in a locale, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's: one per territory in code
            /// order — the ISO 3166-1 countries, the UN M.49 areas such as `001`,
            /// and CLDR's `EU`, `EZ`, `UN`, `QO`, `XA`, `XB` and `ZZ` — with the
            /// code, the name in the `locale`, the English name, the tag of the
            /// data that named it, that value's CLDR draft level (`approved`,
            /// `contributed` or `provisional`), and the code's CLDR validity status
            /// (`regular`, `macroregion`, `special` or `unknown`). A territory the
            /// locale's chain does not name, a null `locale` and `native` are
            /// named in English, with `en` in column 4. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Every territory CLDR 48 names, with its name in a locale, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// One line per territory, in code order — the ISO 3166-1 countries,
            /// the UN M.49 areas such as `001` and `419`, and CLDR's `EU`, `EZ`,
            /// `UN`, `QO`, `XA`, `XB` and `ZZ` — tab-separated: the code; the name
            /// in the locale, 日本 for `JP` under `ja`; the English name, `en.xml`'s;
            /// the tag of the data that named column 2, `ja` for a request for
            /// `ja-JP`, `pt` for a name `pt-PT` inherits, or `en`; that value's
            /// CLDR draft level, `approved`, `contributed` or `provisional`; and
            /// the code's status in CLDR's validity data, `regular`, `macroregion`,
            /// `special` or `unknown`. A territory the locale's chain does not
            /// name, and every territory under `native` or a tag whose chain
            /// reaches no table, is named in English with `en`. The locale argument
            /// fails as `hc_parse_iso_date` does. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_territories(locale: text(locale_len)) -> line =
            |locale| Ok($crate::place_lines::territories(locale));

        c {
            /// The ISO 3166-2 subdivisions of a country CLDR 48 names, with their
            /// names in a locale, as NUL-terminated UTF-8 lines in a caller-owned
            /// buffer.
            ///
            /// The lines are `hc_territories`' columns, one per subdivision in code
            /// order, column 1 the code as ISO writes it, `JP-13`, and column 6
            /// `regular` or `deprecated`. `country` is a territory's code in any
            /// case; one that is not is `HC_ERROR_UNKNOWN`, a territory with no
            /// subdivision writes nothing, and a null or empty `country` writes
            /// every subdivision, country by country. `locale` is as for
            /// `hc_territories`. A deprecated code neither the locale nor English
            /// names has columns 2 to 5 empty. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The ISO 3166-2 subdivisions of a country CLDR 48 names, with their
            /// names in a locale, as UTF-8 lines, returning the byte length
            /// written.
            ///
            /// One line per subdivision, in code order, in `hc_territories`'
            /// columns: the code as ISO writes it, `JP-13` for CLDR's `jp13`; the
            /// name in the locale, 東京都 under `ja`; the English name; the tag
            /// that answered; the draft level, `provisional` for nearly every
            /// name outside English; and `regular` for a code in use or
            /// `deprecated` for one CLDR keeps from an earlier list. A deprecated
            /// code neither the locale nor English names has columns 2 to 5 empty.
            /// `country` is a territory's code in any case, and one that is not is
            /// `HC_ERR_UNKNOWN`; a territory with no subdivision writes nothing,
            /// and an empty `country` writes all 5 503, country by country. Both
            /// text arguments fail as for `hc_parse_iso_date`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_subdivisions(country: opt(country_len), locale: text(locale_len)) -> line =
            $crate::place_lines::subdivisions;

        c {
            /// One territory or subdivision, as the NUL-terminated UTF-8 line
            /// `hc_territories` or `hc_subdivisions` writes for it, in a
            /// caller-owned buffer.
            ///
            /// `code` is a territory's code, `JP` or `001`, or a subdivision's in
            /// ISO form, `JP-13`, in any case; another, CLDR's own form `jp13`
            /// included, is `HC_ERROR_UNKNOWN`, and a null `code`
            /// `HC_ERROR_NULL_POINTER`. `locale` is as for `hc_territories`. Writes
            /// the required length, including the terminator, into `written`.
        }
        wasm {
            /// One territory or subdivision, as the UTF-8 line `hc_territories` or
            /// `hc_subdivisions` writes for it, returning the byte length written.
            ///
            /// `code` is a territory's code, `JP` or `001`, or a subdivision's in
            /// ISO form, `JP-13`, in any case; another, CLDR's own form `jp13`
            /// included, is `HC_ERR_UNKNOWN`. Both text arguments fail as for
            /// `hc_parse_iso_date`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_place_name(code: name(code_len), locale: text(locale_len)) -> line =
            $crate::place_lines::place_name;
    } };
    ("humanize", $backend:ident) => { $backend! {
        c {
            /// How one POSIX instant reads from another, *3 hours ago* or *in 2
            /// days*, in a locale, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the phrase, the unit it is
            /// counted in, the signed count and the tag of the data the locale
            /// resolved to. `style` is `long`, `short` or `narrow`, in any case;
            /// anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. `automatic` non-zero writes the language's
            /// own word for an offset where it has one, *yesterday*. `locale` is a
            /// NUL-terminated BCP 47 tag, or null for the root locale, whose
            /// phrases are CLDR's `root.xml`'s, `-1 d`; one that does not parse is
            /// the root locale too. Two instants further apart
            /// than an `int64_t` of seconds are `HC_ERROR_OUT_OF_RANGE`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// How one POSIX instant reads from another, *3 hours ago* or *in 2
            /// days*, in a locale, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// Tab-separated: the phrase `then_unix` is from `now_unix`; the unit
            /// it is counted in, CLDR's field name (`second`, `minute`, `hour`,
            /// `day`, `week`, `month` or `year`); the signed count, negative in the
            /// past; and the tag of the `hc-humanize` data the locale resolved to.
            /// The unit is the one the conversational thresholds choose and the
            /// count is truncated, so 90 minutes ago is *1 hour ago*; a month and a
            /// year are the Gregorian means. `style` is `long`, `short` or `narrow`,
            /// in any case; anything else, the empty string included, is
            /// `HC_ERR_UNKNOWN`. `automatic` non-zero writes the language's own
            /// word for the offset where it has one — *yesterday*, *now* —
            /// `Intl.RelativeTimeFormat`'s `numeric: "auto"`; zero always the
            /// numeric pattern. `locale` fails as for `hc_parse_iso_date`; the
            /// empty string, and a tag that does not parse, is the root locale,
            /// whose phrases are CLDR's `root.xml`'s, `-1 d`, not English's. Two
            /// instants further apart than an `i64` of seconds are
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_relative_time(
            then_unix: i64,
            now_unix: i64,
            style: name(style_len),
            automatic: flag,
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::relative_time_line;

        c {
            /// Which calendar day one fixed day is, seen from another, *yesterday*
            /// or *3 days ago*, in a locale, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the phrase, the unit, the
            /// signed count and the tag of the data the locale resolved to.
            /// `style`, `automatic` and `locale` are as for `hc_relative_time`. Two
            /// days further apart than an `int64_t` holds are
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Which calendar day one fixed day is, seen from another, *yesterday*
            /// or *3 days ago*, in a locale, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// Tab-separated: the phrase `then_fixed` is from `now_fixed`; the
            /// unit, `day` up to a week and then `week`, `month` or `year`; the
            /// signed count, negative in the past; and the tag of the
            /// `hc-humanize` data the locale resolved to. The offset is the
            /// difference of the two day numbers, never a span divided, so 23:30 on
            /// one day and 00:30 on the next are *yesterday*; the caller's clock
            /// says which day an instant falls on. With `automatic` non-zero, a day
            /// of −1 is *yesterday* and one of 0 *today*. `style`, `automatic` and
            /// `locale` are as for `hc_relative_time`. Two days further apart than
            /// an `i64` holds are `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns
            /// the length the text needs.
        }
        fn hc_relative_day(
            then_fixed: i64,
            now_fixed: i64,
            style: name(style_len),
            automatic: flag,
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::relative_day_line;

        c {
            /// Which calendar day one fixed day is, seen from another, with a time
            /// of day, *yesterday at 15:05*, in a locale, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the phrase, the day phrase's
            /// unit and signed count, the time as written and the tag of the data
            /// the locale resolved to. `seconds_of_day` from 86 400 is
            /// `HC_ERROR_OUT_OF_RANGE`; the rest is as for `hc_relative_day`.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// Which calendar day one fixed day is, seen from another, with a time
            /// of day, *yesterday at 15:05*, in a locale, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the phrase, the day phrase of `hc_relative_day` and
            /// the time joined by the locale's pattern for a relative day with a
            /// time, CLDR's (`es` *ayer, 15:05*, `ja` *昨日の 15:05*); the day
            /// phrase's unit and signed count; the time as it was written; and the
            /// tag of the `hc-humanize` data the locale resolved to. The time is
            /// `seconds_of_day` after midnight on a 24-hour clock as `H:MM` in the
            /// locale's digits, the seconds dropped; one from 86 400 is
            /// `HC_ERR_OUT_OF_RANGE`. The rest is as for `hc_relative_day`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_relative_day_at(
            then_fixed: i64,
            now_fixed: i64,
            seconds_of_day: u32,
            style: name(style_len),
            automatic: flag,
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::relative_day_at_line;

        c {
            /// A span of seconds phrased in days, hours, minutes and seconds, *2
            /// hours and 30 minutes*, in a locale, as one NUL-terminated UTF-8 line
            /// in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the phrase, `1` for a negative
            /// span and `0` for another, and the tag of the data the locale
            /// resolved to. `style` is `long`, `short`, `narrow` or `compact`, in
            /// any case; anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. `max_components` is the most units written,
            /// 0 for every one. `locale` is as for `hc_relative_time`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// A span of seconds phrased in days, hours, minutes and seconds, *2
            /// hours and 30 minutes*, in a locale, as one UTF-8 line, returning the
            /// byte length written.
            ///
            /// Tab-separated: the phrase; `1` if `seconds` is negative and `0` if
            /// not, the phrase being of the span's length; and the tag of the
            /// `hc-humanize` data the locale resolved to. `style` is `long`, the
            /// unit phrases joined by the locale's list pattern; `short`, the
            /// abbreviated ones; `narrow`, the narrow ones; or `compact`, the bare
            /// suffixes run together, *2h30m*; in any case, and anything else, the
            /// empty string included, is `HC_ERR_UNKNOWN`. A unit the span does not
            /// reach is left out, and at most `max_components` units are written,
            /// the largest first and the rest of the span dropped, not rounded; 0
            /// writes every unit. A span of nothing is *0 seconds*. `locale` is as
            /// for `hc_relative_time`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_duration(
            seconds: i64,
            style: name(style_len),
            max_components: u32,
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::duration_line;
    } };
    ("zone-names", $backend:ident) => { $backend! {
        c {
            /// A time zone's name at a POSIX timestamp in a locale, as a CLDR
            /// pattern field writes it, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the name, the field, the zone,
            /// its offset in seconds and its daylight flag. `zone` is a name the
            /// loaded zones or the built-in table knows; another is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. `field` is one
            /// of `z` to `zzzz`, `O`, `OOOO`, `v`, `vvvv` and `V` to `VVVV`, the
            /// fields UTS #35 defines; another is `HC_ERROR_UNKNOWN`. `locale` is a NUL-terminated BCP 47
            /// tag, or null for the root locale. An instant outside the years the
            /// zones' rules answer for is `HC_ERROR_OUT_OF_RANGE`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// A time zone's name at a POSIX timestamp in a locale, as a CLDR
            /// pattern field writes it, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// `field` is `z` to `zzz`, the short specific name, *PDT*; `zzzz`, the
            /// long, *Pacific Daylight Time*; `O` or `OOOO`, the localized GMT
            /// format, *GMT-7*; `v` and `vvvv`, the generic names, *PT* and
            /// *Pacific Time*; `V`, the short identifier, `VV` the zone, `VVV` its
            /// exemplar city and `VVVV` its generic location; each with the
            /// fallbacks of UTS #35 Part 4, from CLDR 48's metazones and names.
            /// Another field, `OO` and `vv` among them, which UTS #35 does not
            /// define, is `HC_ERR_UNKNOWN`. Tab-separated:
            /// the name; the field; the zone as given; its offset at the instant,
            /// seconds east of UTC; and `1` for its daylight time, else `0`. `zone`
            /// is as for `hc_zone_offset`, and one neither the loaded zones nor
            /// the built-in table knows is `HC_ERR_UNKNOWN`. `locale` fails as for
            /// `hc_parse_iso_date`; one that does not parse is the root locale. An
            /// instant outside `hc_zone_offset`'s years is `HC_ERR_OUT_OF_RANGE`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_zone_name(
            zone: name(zone_len),
            unix_seconds: i64,
            locale: text(locale_len),
            field: name(field_len),
        ) -> line =
            $crate::zone_lines::zone_name_line;

        c {
            /// An instant formatted in a time zone and a locale by a CLDR or a
            /// `strftime` pattern, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `zone` is as for
            /// `hc_zone_name`; `syntax` is `cldr` or `strftime`, in any case, and
            /// another is `HC_ERROR_UNKNOWN`; a pattern `hc-format` does not read is
            /// `HC_ERROR_MALFORMED`. Null `zone`, `syntax` or `pattern` is
            /// `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// An instant formatted in a time zone and a locale by a CLDR or a
            /// `strftime` pattern, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// `syntax` is `cldr`, a pattern of UTS #35 Part 4, `yyyy-MM-dd HH:mm
            /// zzzz`, whose zone fields are `hc_zone_name`'s; or `strftime`, a
            /// POSIX pattern, `%Y-%m-%d %H:%M %Z`, whose `%Z` is the rules'
            /// abbreviation; in any case, and another is `HC_ERR_UNKNOWN`. The
            /// reading is the zone's local one at the instant, from its loaded or
            /// built-in rules, in the locale's vocabulary and digits. Tab-separated:
            /// the text; the syntax; the zone as given; its offset at the instant,
            /// seconds east of UTC; and `1` for its daylight time, else `0`. A
            /// pattern `hc-format` does not read — an unclosed quote, a field or a
            /// conversion it does not write — is `HC_ERR_MALFORMED`; a zone
            /// neither the loaded zones nor the built-in table knows
            /// `HC_ERR_UNKNOWN`; an instant outside `hc_zone_offset`'s years
            /// `HC_ERR_OUT_OF_RANGE`. `locale` fails as for `hc_parse_iso_date`. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_format_pattern(
            zone: name(zone_len),
            unix_seconds: i64,
            locale: text(locale_len),
            syntax: name(syntax_len),
            pattern: name(pattern_len),
        ) -> line =
            $crate::zone_lines::format_pattern_line;
    } };
}
