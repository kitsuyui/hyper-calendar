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
                Ok($crate::Rd::from_unix_days(
                    $crate::hc_core::duration::days_and_seconds(unix_seconds).0
                ).get())
            };

        c {
            /// Whether a POSIX timestamp names a day that ends with an inserted leap
            /// second.
            ///
            /// A timestamp in a day whose start or whose end is not an `int64_t` —
            /// the first and last part-days of the range — is
            /// [`HC_ERROR_OUT_OF_RANGE`](super::HC_ERROR_OUT_OF_RANGE). A day past the
            /// announced leap-second table, whose end no one has said is a leap
            /// second's, is [`HC_ERROR_NO_DATA`](super::HC_ERROR_NO_DATA) when `strict`
            /// is non-zero, and answered no when it is zero, as `hc_tai_from_unix`
            /// holds the last published offset.
        }
        wasm {
            /// Whether the UTC day containing a POSIX timestamp ends with an inserted
            /// leap second: 1, 0, or an error sentinel.
            ///
            /// A timestamp in a day whose start or whose end is not an `i64` — the
            /// first and last part-days of the range — is [`HC_ERR_OUT_OF_RANGE`](super::HC_ERR_OUT_OF_RANGE).
            /// A day past the announced leap-second table, whose end no one has said
            /// is a leap second's, is [`HC_ERR_NO_DATA`](super::HC_ERR_NO_DATA) when
            /// `strict` is non-zero, and answered 0 when it is zero, as `hc_tai_from_unix`
            /// holds the last published offset.
        }
        fn hc_day_has_leap_second(unix_seconds: i64, strict: flag) -> value(out_has_leap: int) =
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
                    .and_then($crate::hc_core::duration::seconds_in_days)
                    .ok_or($crate::boundary::Refusal::OutOfRange)
            };

        c {
            /// Python's `time.gmtime(seconds)`: the UTC reading of a POSIX second
            /// as the nine fields of a `struct_time`, one NUL-terminated UTF-8 line
            /// in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. A second outside the Gregorian
            /// years ±9 999 999 is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// Python's `time.gmtime(seconds)`: the UTC reading of a POSIX second
            /// as the nine fields of a `struct_time`, one UTF-8 line, returning the
            /// byte length written.
            ///
            /// Tab-separated, in Python's order: `tm_year`; `tm_mon`, 1 to 12;
            /// `tm_mday`; `tm_hour`; `tm_min`; `tm_sec`; `tm_wday`, Monday 0 to
            /// Sunday 6; `tm_yday`, from 1; and `tm_isdst`, 0 for UTC. A second
            /// outside the Gregorian years ±9 999 999 is `HC_ERR_OUT_OF_RANGE`. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_gmtime(unix_seconds: i64) -> line =
            $crate::python_lines::gmtime_line;

        c {
            /// Python's `calendar.timegm(tuple)`: the POSIX second of a UTC reading
            /// given as its year, month, day, hour, minute and second, written to
            /// `out_unix_seconds`.
            ///
            /// The day, hour, minute and second are not checked and add up, as
            /// Python's do: a day 32 is the first of the next month. A month
            /// outside 1 to 12 is `HC_ERROR_INVALID_DATE`, a year outside
            /// ±9 999 999 `HC_ERROR_OUT_OF_RANGE`, and a sum that leaves an
            /// `int64_t` `HC_ERROR_OVERFLOW`.
        }
        wasm {
            /// Python's `calendar.timegm(tuple)`: the POSIX second of a UTC reading
            /// given as its year, month, day, hour, minute and second, or an error
            /// sentinel.
            ///
            /// The day, hour, minute and second are not checked and add up, as
            /// Python's do: a day 32 is the first of the next month. A month
            /// outside 1 to 12 is `HC_ERR_INVALID_DATE`, and a year outside
            /// ±9 999 999 or a sum that leaves an `i64` `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_timegm(
            year: i64,
            month: i64,
            day: i64,
            hour: i64,
            minute: i64,
            second: i64,
        ) -> value(out_unix_seconds: i64) =
            |year, month, day, hour, minute, second| {
                $crate::python_lines::timegm([year, month, day, hour, minute, second])
            };

        c {
            /// Python's `calendar.isleap(year)`: whether a proleptic Gregorian year
            /// is a leap year, for any year; writes 1 or 0.
        }
        wasm {
            /// Python's `calendar.isleap(year)`: whether a proleptic Gregorian year
            /// is a leap year, for any year: 1 or 0.
        }
        fn hc_isleap(year: i64) -> value(out_is_leap: int) =
            $crate::python_lines::isleap;

        c {
            /// Python's `calendar.leapdays(y1, y2)`: the number of leap years from
            /// `y1` up to but not including `y2`, counted backwards when `y2` is
            /// before `y1`, written to `out_leap_days`.
            ///
            /// A year outside ±9 999 999 is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// Python's `calendar.leapdays(y1, y2)`: the number of leap years from
            /// `y1` up to but not including `y2`, counted backwards when `y2` is
            /// before `y1`, or an error sentinel.
            ///
            /// A year outside ±9 999 999 is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_leapdays(y1: i64, y2: i64) -> value(out_leap_days: i64) =
            $crate::python_lines::leapdays;

        c {
            /// Python's `calendar.weekday(year, month, day)`: the weekday of a
            /// Gregorian date with Monday 0 and Sunday 6, written to `out_weekday`,
            /// where `hc_weekday` answers Monday 1 to Sunday 7 for a fixed day.
            ///
            /// A date that does not exist is `HC_ERROR_INVALID_DATE` and a year
            /// outside ±9 999 999 `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// Python's `calendar.weekday(year, month, day)`: the weekday of a
            /// Gregorian date with Monday 0 and Sunday 6, or an error sentinel,
            /// where `hc_weekday` answers Monday 1 to Sunday 7 for a fixed day.
            ///
            /// A date that does not exist is `HC_ERR_INVALID_DATE` and a year
            /// outside ±9 999 999 `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_calendar_weekday(year: i64, month: u32, day: u32) -> value(out_weekday: u8) =
            $crate::python_lines::calendar_weekday;

        c {
            /// Python's `calendar.monthrange(year, month)`: the weekday of the first
            /// day of a Gregorian month, Monday 0, and the number of days in the
            /// month, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. A month outside 1 to 12 is
            /// `HC_ERROR_INVALID_DATE` and a year outside ±9 999 999
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Python's `calendar.monthrange(year, month)`: the weekday of the first
            /// day of a Gregorian month, Monday 0, and the number of days in the
            /// month, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the first day's weekday, Monday 0 to Sunday 6, and
            /// the days, 28 to 31. A month outside 1 to 12 is `HC_ERR_INVALID_DATE`
            /// and a year outside ±9 999 999 `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_monthrange(year: i64, month: u32) -> line =
            $crate::python_lines::monthrange_line;

        c {
            /// Python's `calendar.monthcalendar(year, month)`, the weeks of a
            /// Gregorian month as NUL-terminated UTF-8 lines in a caller-owned
            /// buffer, with the first weekday of the week as an argument.
            ///
            /// The lines are the WebAssembly module's. `first_weekday` is Monday 0
            /// to Sunday 6, what `calendar.setfirstweekday` sets; above 6, or a
            /// month outside 1 to 12, is `HC_ERROR_INVALID_DATE`, and a year outside
            /// ±9 999 999 `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Python's `calendar.monthcalendar(year, month)`, the weeks of a
            /// Gregorian month as UTF-8 lines, returning the byte length written,
            /// with the first weekday of the week as an argument.
            ///
            /// One line a week, from the week holding the 1st to the week holding
            /// the last day, seven tab-separated cells each: the day of the month,
            /// or `0` for a day outside the month. `first_weekday` is Monday 0 to
            /// Sunday 6, what `calendar.setfirstweekday` sets; above 6, or a month
            /// outside 1 to 12, is `HC_ERR_INVALID_DATE`, and a year outside
            /// ±9 999 999 `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length
            /// the text needs.
        }
        fn hc_monthcalendar(year: i64, month: u32, first_weekday: u32) -> line =
            $crate::python_lines::monthcalendar_lines;

        c {
            /// A fixed day's week of the year under a week rule, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `first_weekday` is the ISO 8601
            /// number of the day the week begins on, Monday 1 to Sunday 7, and
            /// `min_days` the fewest days of a year or month a week needs to be its
            /// first, 1 to 7: ISO 8601 is 1 and 4, `strftime`'s `%U` 7 and 7 and its
            /// `%W` 1 and 7, and a locale's pair is `hc_locale_info`'s columns 14
            /// and 15. Either outside 1 to 7, or a day outside the Gregorian years,
            /// is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// A fixed day's week of the year under a week rule, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the week-numbering year; the week of that year, from
            /// 1; the number of weeks the year has, 52 or 53; and the week of the
            /// month, 1 for the month's first week and 0 for the days before it.
            /// `first_weekday` is the ISO 8601 number of the day the week begins
            /// on, Monday 1 to Sunday 7, and `min_days` the fewest days of a year or
            /// month a week needs to be its first, 1 to 7, UTS #35's `firstDay`
            /// and `minDays`: ISO 8601 is 1 and 4 (1 January 2021 is week 53 of
            /// 2020), `strftime`'s `%U` is 7 and 7 and its `%W` 1 and 7 (both number
            /// the days before week 1 as the year's week 0, which this reads as the
            /// last week of the year before), the United States is 7 and 1 (1
            /// January 2021 is week 1 of 2021) and a locale's pair is
            /// `hc_locale_info`'s columns 14 and 15. Either outside 1 to 7, or a day
            /// outside the Gregorian years, is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_week_of_year(fixed: i64, first_weekday: u32, min_days: u32) -> line =
            $crate::python_lines::week_of_year_line;

        c {
            /// The fixed day a week date names under a week rule, written to
            /// `out_fixed`: the inverse of `hc_week_of_year`.
            ///
            /// `week_year` is the week-numbering year, `week` the week of it from 1 and
            /// `weekday` the ISO 8601 number of the day, Monday 1 to Sunday 7.
            /// `first_weekday` and `min_days` are the rule's, as for `hc_week_of_year`:
            /// ISO 8601's week date is 1 and 4. A weekday outside 1 to 7, a week of 0 or
            /// one the year does not have (week 53 of a year of 52 weeks) is
            /// `HC_ERROR_INVALID_DATE`; a rule outside 1 to 7, or a year whose weeks
            /// leave the Gregorian years, `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The fixed day a week date names under a week rule, or an error
            /// sentinel: the inverse of `hc_week_of_year`.
            ///
            /// `week_year` is the week-numbering year, `week` the week of it from 1 and
            /// `weekday` the ISO 8601 number of the day, Monday 1 to Sunday 7.
            /// `first_weekday` and `min_days` are the rule's, UTS #35's `firstDay` and
            /// `minDays`, as for `hc_week_of_year`: ISO 8601's week date is 1 and 4
            /// (week 53 of 2020, Friday, is 1 January 2021) and the United States' is 7
            /// and 1. A weekday outside 1 to 7, a week of 0 or one the year does not
            /// have (week 53 of a year of 52 weeks, which names no day as 31 February
            /// names none) is `HC_ERR_INVALID_DATE`; a rule outside 1 to 7, or a year
            /// whose weeks leave the Gregorian years, `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_fixed_from_week(
            week_year: i64,
            week: u32,
            weekday: u32,
            first_weekday: u32,
            min_days: u32,
        ) -> value(out_fixed: i64) =
            $crate::python_lines::fixed_from_week;

        c {
            /// Python's `time.asctime` of a Gregorian reading, `Sun Jun 20 23:21:05
            /// 1993`, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. Each field is checked as
            /// `datetime(*tuple[:6])` checks them; one out of range, a second of 60
            /// included, is `HC_ERROR_INVALID_DATE`, and a year outside 1 to 9999,
            /// which `datetime` cannot hold, `HC_ERROR_OUT_OF_RANGE`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// Python's `time.asctime` of a Gregorian reading, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// One cell: the weekday and month abbreviations in English, the day of the
            /// month padded with a space to two characters, the time and the year,
            /// `Sun Jun 20 23:21:05 1993`. The weekday is the date's, as
            /// `datetime.ctime` computes it, not a field of the call. Each field is
            /// checked as `datetime(*tuple[:6])` checks them; one out of range, a
            /// second of 60 included, is `HC_ERR_INVALID_DATE`, and a year outside 1
            /// to 9999, which `datetime` cannot hold, `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_asctime(
            year: i64,
            month: i64,
            day: i64,
            hour: i64,
            minute: i64,
            second: i64,
        ) -> line =
            |year, month, day, hour, minute, second| {
                $crate::python_lines::asctime_line([year, month, day, hour, minute, second])
            };
    } };
    ("datetime", $backend:ident) => { $backend! {
        c {
            /// A date-time read in a syntax, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `syntax` is `iso8601`,
            /// `iso8601-full`, `rfc3339`, `rfc2822`, `python` or `auto`, in any case;
            /// another is `HC_ERROR_UNKNOWN`, and null `syntax` or `text`
            /// `HC_ERROR_NULL_POINTER`. Text that is not in the syntax, a date or a date of
            /// reduced accuracy where a date-time was meant, is `HC_ERROR_MALFORMED`; a
            /// date or time that does not exist, 31 February, `24:00:01`, or a second 60
            /// that is not UTC's `23:59:60` at the text's offset or is on a day that did
            /// not end in an inserted second, `HC_ERROR_INVALID_DATE`; one this library
            /// cannot hold, or a second 60 past the leap-second table,
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A date-time read in a syntax, as one UTF-8 line of eight cells, returning
            /// the byte length written.
            ///
            /// Tab-separated, a *reading*: the local fixed day; the local second of the
            /// day, 0 through 86 400 (`23:59:60` is the 86 400th second); the attoseconds
            /// into that second; the zone, `none` for no designator, `utc` for `Z`, `offset`
            /// for a numeric one and `unknown-local` for RFC 3339's `-00:00`; the offset in
            /// seconds east of UTC; the POSIX second of the instant, floored, the fraction
            /// being in the attosecond cell; `1` for an inserted leap second, which POSIX
            /// counts as the second after it; and `1` when the text wrote the end of a day
            /// as `24:00`. The offset and the POSIX second are empty when the text states
            /// no zone: `2026-09-21T14:30:05` is a reading on somebody's wall clock, never
            /// UTC, and no instant stands for it.
            ///
            /// `syntax` is `iso8601`, everything ISO 8601-1 allows of a date and a time,
            /// basic and extended, ordinal and week dates, `24:00`, `23:59:60` and a decimal
            /// fraction of the lowest component; `iso8601-full`, complete extended values
            /// with a zone only; `rfc3339`, the internet profile, whose lower-case `t` and
            /// `z` and space are read; `rfc2822`, email and HTTP dates, obsolete syntax
            /// included; `python`, `datetime.fromisoformat` of Python 3.13, which takes a
            /// date alone as midnight; and `auto`, which tells an ISO date-time from an email
            /// date by its letters; in any case, and another, the empty string included, is
            /// `HC_ERR_UNKNOWN`. Text that is not in the syntax, a date alone or a date of
            /// reduced accuracy under any syntax but `python` (`hc_iso_date_parts` reads
            /// those), is `HC_ERR_MALFORMED`; a date or a time that does not exist is
            /// `HC_ERR_INVALID_DATE`, and so is a second 60 that the zone's clock does not
            /// read at UTC's `23:59:60` — RFC 3339 §5.7 shifts the leap second by the
            /// offset, so `1990-12-31T15:59:60-08:00` is read and `15:59:60+08:00` is not —
            /// or that is on a day the leap-second table says did not end in an inserted
            /// second (a reading with no zone is read at `23:59:60` alone); one this
            /// library cannot hold, and a second 60 past the table's validity, whose day no
            /// one has announced, is `HC_ERR_OUT_OF_RANGE`. `rfc3339` has a calendar date only, an offset hour of
            /// 00 through 23 and no seconds in an offset, and refuses the ISO forms that
            /// have more. A null `buffer` returns the length the text needs.
        }
        fn hc_parse_datetime(syntax: name(syntax_len), text: text(text_len)) -> line =
            $crate::datetime_lines::parse_datetime_line;

        c {
            /// An instant written as a date-time in a syntax, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `unix_seconds` and `attoseconds` are
            /// the instant, `offset_seconds` the zone it is written in. `syntax` is
            /// `iso8601`, `iso8601-basic`, `iso8601-ordinal`, `iso8601-week`, `rfc3339`,
            /// `rfc2822`, `imf-fixdate` or `python`, and `precision` `auto`, `hours`,
            /// `minutes`, `seconds`, `milliseconds`, `microseconds` or `nanoseconds`, in
            /// any case; another, or one the syntax has not, is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. `attoseconds` of 10¹⁸ or more, an offset beyond
            /// ±25:59:59, a year `0000..=9999` does not hold where the syntax needs one, and
            /// an offset with seconds where the syntax has no digits for them (every
            /// syntax but `python`) are `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// An instant written as a date-time in a syntax, to a precision and in the
            /// zone of a numeric offset, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// Tab-separated: the text, and the syntax. `unix_seconds` and `attoseconds`
            /// (from 0 to 10¹⁸ − 1, the remainder of the second, never negative) are the
            /// instant and `offset_seconds` the zone it is written in. `syntax` is
            /// `iso8601` (`2026-09-21T14:30:05+09:00`, `Z` for a zero offset),
            /// `iso8601-basic` (`20260921T143005+0900`), `iso8601-ordinal`
            /// (`2026-264T14:30:05+09:00`), `iso8601-week` (`2026-W39-1T14:30:05+09:00`),
            /// `rfc3339`, `rfc2822` (`Mon, 21 Sep 2026 14:30:05 +0900`), `imf-fixdate`
            /// (HTTP's, `Mon, 21 Sep 2026 05:30:05 GMT`, always the UTC reading whatever the
            /// offset) or `python` (`datetime.isoformat`: `+00:00` for UTC, never `Z`).
            /// `precision` is `auto`, the digits the instant needs and none for a whole
            /// second, or `hours`, `minutes`, `seconds`, `milliseconds`, `microseconds` or
            /// `nanoseconds`; a time is truncated to it, never rounded. RFC 3339 has no
            /// `hours` or `minutes`, RFC 2822 and HTTP no sub-second digits and Python no
            /// `nanoseconds`. Both in any case; another, one a syntax has not, and the empty
            /// string, is `HC_ERR_UNKNOWN`. `attoseconds` of 10¹⁸ or more, an offset beyond
            /// ±25:59:59, a year outside `0000..=9999` in RFC 3339, RFC 2822 or HTTP, and an
            /// offset with seconds in a syntax with no digits for them (every one but
            /// `python`, whose text is `+05:30:15`) is `HC_ERR_OUT_OF_RANGE`: the wall clock
            /// of the whole offset written with the minutes alone would read back at another
            /// instant. A null `buffer` returns the length the text needs.
        }
        fn hc_format_datetime(
            syntax: name(syntax_len),
            unix_seconds: i64,
            attoseconds: u64,
            offset_seconds: int,
            precision: name(precision_len),
        ) -> line =
            $crate::datetime_lines::format_datetime_line;

        c {
            /// A fixed day written as an ISO 8601 calendar, ordinal or week date, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `form` is `calendar`, `ordinal` or
            /// `week` and `style` `extended` or `basic`, in any case; another is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. A day outside the
            /// Gregorian range is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// A fixed day written as an ISO 8601 calendar, ordinal or week date, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the date, the form and the style. `form` is `calendar`
            /// (`2026-09-21`), `ordinal` (`2026-264`) or `week` (`2026-W39-1`, whose year is
            /// the week-numbering year: 2021-01-03 is `2020-W53-7`); `style` is `extended`
            /// or `basic`, with no separators (`20260921`, `2026264`, `2026W391`); in any
            /// case; another is `HC_ERR_UNKNOWN`. A year outside `0000..=9999` is written
            /// with a sign and six or more digits, the expanded form ISO 8601 gives it.
            /// `hc_format_iso_date` is the calendar date in the extended style, as plain
            /// text. A day outside the Gregorian range is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_format_iso_date_as(
            fixed: i64,
            form: name(form_len),
            style: name(style_len),
        ) -> line =
            $crate::datetime_lines::format_iso_date_line;

        c {
            /// An ISO 8601 date read into its parts, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. Text that is not an ISO 8601 date is
            /// `HC_ERROR_MALFORMED` and a date that does not exist, week 54 or 31 February,
            /// `HC_ERROR_INVALID_DATE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// An ISO 8601 date read into its parts, including one that names no day,
            /// as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the form, `calendar`, `ordinal` or `week`; the year, the
            /// week-numbering year for a week date; the month; the day of the month; the day
            /// of the year; the week; the weekday, 1 Monday to 7 Sunday; the fixed day, empty
            /// when the date names none (`2026`, `2026-09`, `2026-W39`); and `basic` or
            /// `extended`. A part the form has none of, or the text left out, is empty.
            /// Text that is not an ISO 8601 date is `HC_ERR_MALFORMED` and a date that does
            /// not exist, week 54 or 31 February, `HC_ERR_INVALID_DATE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_iso_date_parts(text: name(text_len)) -> line =
            $crate::datetime_lines::iso_date_parts_line;

        c {
            /// An ISO 8601 duration read into its components, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. Text that is not an ISO 8601 duration
            /// is `HC_ERROR_MALFORMED`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// An ISO 8601 duration read into its components, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: `1` for a leading minus, which is ISO 8601-2's; the years,
            /// months, weeks, days, hours, minutes and seconds, each empty where the text
            /// did not write it; the digits of the decimal fraction of the lowest component,
            /// as written (`5` for `PT0,5S`), empty for none; the form, `designators`
            /// (`P1Y2M3DT4H5M6S`) or `alternative` (`P0001-02-03T04:05:06`); the duration
            /// written in canonical form; `1` when it is nominal, with years or months,
            /// which have no fixed length; and its exact length, whole seconds and the
            /// attoseconds after them, empty for a nominal one. A day is 86 400 seconds and
            /// a week seven days here: this is the nominal timeline, not elapsed physical
            /// time across a leap second. Text that is not an ISO 8601 duration is
            /// `HC_ERR_MALFORMED`. A null `buffer` returns the length the text needs.
        }
        fn hc_iso_duration(text: name(text_len)) -> line =
            $crate::datetime_lines::iso_duration_line;

        c {
            /// A duration written from its components, as one NUL-terminated UTF-8 line in
            /// a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. A component below zero is absent.
            /// `fraction` is read as the empty string when null. Components ISO 8601 has no
            /// spelling for are `HC_ERROR_MALFORMED`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// A duration written in ISO 8601 from its components, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the duration in the designator form (`P3Y6M4DT12H30M5S`), `1`
            /// when it is nominal, and its exact whole seconds and attoseconds, empty for a
            /// nominal one. A component below zero is absent, so `-1` leaves out a
            /// unit. `negative` non-zero writes a leading minus. `fraction` is the digits of a
            /// decimal fraction of the lowest component present, `5` for a half, up to 18
            /// digits, the empty string for none. Components ISO 8601 has no spelling for
            /// (none at all, a week beside other units, a fraction on a unit that is not
            /// the lowest, a fraction that is not digits) are `HC_ERR_MALFORMED`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_format_iso_duration(
            negative: flag,
            years: i64,
            months: i64,
            weeks: i64,
            days: i64,
            hours: i64,
            minutes: i64,
            seconds: i64,
            fraction: text(fraction_len),
        ) -> line =
            $crate::datetime_lines::format_iso_duration_line;

        c {
            /// An ISO 8601 interval, or a repeating one, read into its ends, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. Text that is not an interval is
            /// `HC_ERROR_MALFORMED`. Writes the required length, including the terminator,
            /// into `written`.
        }
        wasm {
            /// An ISO 8601 interval, or a repeating one, read into its ends, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// `start/end`, `start/duration`, `duration/end`, a duration alone, and each of
            /// them with a repetition, `R5/...` or `R/...`. Tab-separated: the repetitions,
            /// empty for an interval that does not repeat, a count, or `inf` for `R/`; the
            /// shape, `start-end`, `start-duration`, `duration-end` or `duration`; the start
            /// written back and its POSIX second; the end likewise; the duration written back,
            /// `1` when it is nominal, and its exact whole seconds and attoseconds, empty
            /// for a nominal one. A cell the shape has no part for is empty, and so is a POSIX
            /// second where the text states no zone or no time, since that is a reading and
            /// not an instant; the second is whole, and a decimal fraction of it is in the
            /// text. Text that is not an interval is `HC_ERR_MALFORMED`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_iso_interval(text: name(text_len)) -> line =
            $crate::datetime_lines::iso_interval_line;

    } };
    ("patterns", $backend:ident) => { $backend! {
        c {
            /// A text read against a pattern, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `syntax` is `strftime`, `python` or
            /// `cldr`, in any case; another is `HC_ERROR_UNKNOWN`, and null `syntax`
            /// `HC_ERROR_NULL_POINTER`. `pattern` and `text` are read as the empty string
            /// when null. A text that does not match, or a pattern not read, is
            /// `HC_ERROR_MALFORMED`; fields that name a date or time that does not exist,
            /// or a second 60 that is not a leap second UTC inserted (as for
            /// `hc_parse_datetime`), `HC_ERROR_INVALID_DATE`; a second 60 past the
            /// leap-second table `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// A text read against a `strptime` or CLDR pattern, as one UTF-8 line of
            /// thirty cells, returning the byte length written.
            ///
            /// The fields the pattern read, each empty where it did not, then a reading.
            /// Tab-separated: the year; the century; the year of the century; the month; the
            /// day; the day of the year; the week-numbering year; the ISO week; the ISO
            /// weekday; the `%U` and `%W` week numbers; the hour on a 24-hour clock and on a
            /// 12-hour one; `am` or `pm`; the minute; the second; the attoseconds; the zone,
            /// `utc`, `offset` or `unknown-local`; its offset in seconds; the POSIX second
            /// `%s` read; `ce` or `bce`; the fixed day CLDR's `g` read; and the eight cells of a
            /// reading, as `hc_parse_datetime` writes them, empty where the fields name no
            /// whole date and time (`%H:%M` names no day). `syntax` is `strftime`, POSIX
            /// `strptime`, whose month and weekday names are the C locale's; `python`,
            /// CPython's `datetime.strptime`, with its alternatives, backtracking and
            /// resolution and a date the text leaves out taken from 1900-01-01; or `cldr`, a
            /// pattern of UTS #35 Part 4 such as `yyyy-MM-dd'T'HH:mm:ssXXX`; in any case, and
            /// another is `HC_ERR_UNKNOWN`. A text that does not match the pattern, or a pattern
            /// the library does not read, is `HC_ERR_MALFORMED`; fields that name a date or a
            /// time that does not exist, 30 February, and a second 60 that is not a leap
            /// second UTC inserted, read at the zone's offset on a day the leap-second table
            /// ends in one, as `hc_parse_datetime` reads it, `HC_ERR_INVALID_DATE`; a second 60
            /// past the table's validity `HC_ERR_OUT_OF_RANGE`.
            /// `hc_parse_pattern_in` reads the names of a locale. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_parse_pattern(
            syntax: name(syntax_len),
            pattern: text(pattern_len),
            text: text(text_len),
        ) -> line =
            $crate::datetime_lines::parse_pattern_line;

        c {
            /// A text read against a `strptime` or CLDR pattern in the names of a locale, as
            /// one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is `hc_parse_pattern`'s, which the WebAssembly module's README
            /// describes. `syntax` is `strftime` or `cldr`; `python` is
            /// `HC_ERROR_UNKNOWN`, since CPython's `strptime` reads the C locale's names
            /// alone. `locale` is a NUL-terminated BCP 47 tag, or null for the root locale;
            /// one that does not parse is `HC_ERROR_MALFORMED`. The rest is as for
            /// `hc_parse_pattern`. Writes the required length, including the terminator,
            /// into `written`.
        }
        wasm {
            /// A text read against a `strptime` or CLDR pattern in the names of a locale, as
            /// one UTF-8 line, returning the byte length written.
            ///
            /// `hc_parse_pattern`'s line. The month and weekday names, the day periods and the
            /// eras the locale writes are read besides the C locale's, which a file written
            /// by one program and read by another rarely disagrees about; `syntax` is
            /// `strftime` or `cldr`, and `python` is `HC_ERR_UNKNOWN`, since CPython's
            /// `strptime` reads the C locale's names alone. A tag that does not parse is
            /// `HC_ERR_MALFORMED`. `locale` fails as for `hc_parse_iso_date`. The rest is as
            /// for `hc_parse_pattern`. A null `buffer` returns the length the text needs.
        }
        fn hc_parse_pattern_in(
            syntax: name(syntax_len),
            pattern: text(pattern_len),
            text: text(text_len),
            locale: text(locale_len),
        ) -> line =
            $crate::i18n_lines::parse_pattern_in_line;
    } };
    ("timestamps", $backend:ident) => { $backend! {
        c {
            /// `TAI - UTC` at a POSIX instant, exactly, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the whole seconds and the attoseconds
            /// after them. From 1972 `TAI - UTC` is a whole number of seconds, 37 since 2017;
            /// from 1961 to 1971 it is not, and moves by 3·10⁻⁸ s in a second, so the answer
            /// depends on `attoseconds`, where `hc_tai_minus_utc` gives the floor at the
            /// start of the second. `attoseconds` of 10¹⁸ or more is `HC_ERROR_OUT_OF_RANGE`,
            /// and `strict` non-zero refuses before 1961 and past the announced leap-second
            /// table with `HC_ERROR_NO_DATA`, as `hc_tai_from_unix` does. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// `TAI - UTC` at a POSIX instant, exactly, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// Tab-separated: the whole seconds and the attoseconds after them. From 1972
            /// `TAI - UTC` is a whole number of seconds, 37 since 2017; from 1961 to 1971 it
            /// is not, and moves by 3·10⁻⁸ s in a second, so the answer depends on
            /// `attoseconds`, where `hc_tai_minus_utc` gives the floor at the start of the
            /// second. `attoseconds` of 10¹⁸ or more is `HC_ERR_OUT_OF_RANGE`, and `strict`
            /// non-zero refuses before 1961 and past the announced leap-second table with
            /// `HC_ERR_NO_DATA`, as `hc_tai_from_unix` does. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_tai_minus_utc_exact(unix_seconds: i64, attoseconds: u64, strict: flag) -> line =
            $crate::time_lines::tai_minus_utc_exact_line;

        c {
            /// The UTC label of a TAI instant, exactly, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the POSIX second, the attoseconds into
            /// it, and 1 for an inserted leap second, `23:59:60`, which POSIX time cannot
            /// express and names by the second after it, else 0. From 1961 to 1971 the UTC
            /// reading of a whole TAI second is not a whole second, and `hc_utc_from_tai`
            /// gives its floor: TAI 8 s is 1969-12-31T23:59:59.999918 UTC, which that reads
            /// as −1. `tai_attoseconds` of 10¹⁸ or more is `HC_ERROR_OUT_OF_RANGE`, and
            /// `strict` non-zero refuses before 1961 and past the announced leap-second table
            /// with `HC_ERROR_NO_DATA`. Writes the required length, including the terminator,
            /// into `written`.
        }
        wasm {
            /// The UTC label of a TAI instant, exactly, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// Tab-separated: the POSIX second, the attoseconds into it, and `1` for an
            /// inserted leap second, `23:59:60`, which POSIX time cannot express and names by
            /// the second after it, else `0`. From 1961 to 1971 the UTC reading of a whole
            /// TAI second is not a whole second, and `hc_utc_from_tai` gives its floor: TAI
            /// 8 s is 1969-12-31T23:59:59.999918 UTC, which that reads as −1.
            /// `tai_attoseconds` of 10¹⁸ or more is `HC_ERR_OUT_OF_RANGE`, and `strict`
            /// non-zero refuses before 1961 and past the announced leap-second table with
            /// `HC_ERR_NO_DATA`. A null `buffer` returns the length the text needs.
        }
        fn hc_utc_from_tai_exact(tai_seconds: i64, tai_attoseconds: u64, strict: flag) -> line =
            $crate::time_lines::utc_from_tai_exact_line;

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

        c {
            /// Every epoch `hc-core` carries, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one an epoch in the table's
            /// order: its identifier, its description, its TAI reading as whole
            /// seconds from 1970-01-01 00:00:00 TAI and attoseconds, and the
            /// document that defines it. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Every epoch `hc-core` carries, as UTF-8 lines, returning the byte
            /// length written.
            ///
            /// One line an epoch, in the table's order — the POSIX epoch, the GNSS
            /// epochs, J2000, the origin of TCG and TCB, the Julian Day and the
            /// Modified Julian Date, Rata Die, and the software epochs from .NET's
            /// ticks to PostgreSQL's timestamps — tab-separated: its identifier
            /// (`unix`, `j2000`, `postgresql`), its description, its TAI reading as
            /// whole seconds from 1970-01-01 00:00:00 TAI and the attoseconds into
            /// that second, as every TAI instant crosses, and the document that
            /// defines it, with the key of its entry in `docs/references.bib`. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_epochs() -> line = || Ok($crate::time_lines::epochs_lines());
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
            /// `HC_ERR_MALFORMED`; a Level 2 code, whose epoch is its agency's and
            /// which `hc_ccsds_decode_from_epoch` reads from the caller's, or a
            /// Level 3 or 4 code is `HC_ERR_NO_DATA`. A null `buffer` returns the
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
            /// `HC_ERR_NO_DATA`, a Level 2 one being `hc_ccsds_encode_from_epoch`'s;
            /// attoseconds from 10¹⁸, or an instant the format
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
            /// A binary CCSDS time code read, a Level 2 code from the caller's
            /// epoch, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The arguments and the line are the WebAssembly module's, the line
            /// `hc_ccsds_decode`'s. An epoch outside the years 1 to 9999 is
            /// `HC_ERROR_OUT_OF_RANGE`; the rest fail as for `hc_ccsds_decode`,
            /// but for a Level 2 code, which is read. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// A binary CCSDS time code read, a Level 2 code from the caller's
            /// epoch, as one UTF-8 line, returning the byte length written.
            ///
            /// The line is `hc_ccsds_decode`'s. A Level 2 code's epoch is "obtained
            /// from an external source" (CCSDS 301.0-B-4 §1.3): a CUC code counts
            /// TAI seconds from the instant `epoch_tai_seconds` and
            /// `epoch_attoseconds`, and a CDS code UTC days from the POSIX day
            /// `epoch_unix_day`, −7 305 for 1950 January 1; a Level 1 code counts
            /// from 1958 January 1, whatever the epoch. An epoch outside the years
            /// 1 to 9999, or attoseconds from 10¹⁸, is `HC_ERR_OUT_OF_RANGE`; the
            /// rest fail as for `hc_ccsds_decode`, but for a Level 2 code, which is
            /// read. A null `buffer` returns the length the text needs.
        }
        fn hc_ccsds_decode_from_epoch(
            hex: name(hex_len),
            epoch_tai_seconds: i64,
            epoch_attoseconds: u64,
            epoch_unix_day: i64,
            strict: flag,
        ) -> line =
            $crate::time_code_lines::ccsds_decode_from_epoch_line;

        c {
            /// The binary CCSDS time code of a TAI instant in the format a P-field
            /// names, a Level 2 format from the caller's epoch, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The arguments and the line are the WebAssembly module's, the line
            /// `hc_ccsds_encode`'s. An epoch outside the years 1 to 9999, or an
            /// instant before it, is `HC_ERROR_OUT_OF_RANGE`; the rest fail as for
            /// `hc_ccsds_encode`, but for a Level 2 format, which is written.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// The binary CCSDS time code of a TAI instant in the format a P-field
            /// names, a Level 2 format from the caller's epoch, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// The line is `hc_ccsds_encode`'s, and the epoch is as for
            /// `hc_ccsds_decode_from_epoch`: a CUC code counts the TAI seconds
            /// since `epoch_tai_seconds` and `epoch_attoseconds`, a CDS code the
            /// UTC days since the POSIX day `epoch_unix_day`, and a Level 1 format
            /// counts from 1958 January 1, whatever the epoch. An epoch outside the
            /// years 1 to 9999, attoseconds from 10¹⁸, an instant before the epoch
            /// or past the format's last count from it are `HC_ERR_OUT_OF_RANGE`;
            /// the rest fail as for `hc_ccsds_encode`, but for a Level 2 format,
            /// which is written. A null `buffer` returns the length the text needs.
        }
        fn hc_ccsds_encode_from_epoch(
            tai_seconds: i64,
            attoseconds: u64,
            p_field: name(p_field_len),
            epoch_tai_seconds: i64,
            epoch_attoseconds: u64,
            epoch_unix_day: i64,
            strict: flag,
        ) -> line =
            $crate::time_code_lines::ccsds_encode_from_epoch_line;

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
            /// `HC_ERROR_MALFORMED`, and JJY's call-sign frame, which
            /// `hc_jjy_call_sign_decode` reads, `HC_ERROR_NO_DATA`.
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
            /// JJY's call-sign frame, which carries no year, `HC_ERR_NO_DATA`:
            /// `hc_jjy_call_sign_decode` reads it in a year the caller names. A
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
            /// zone's name, `zone:Europe/Berlin` for DCF77 or `zone:America/Denver`,
            /// the station's, for WWVB (a zone the built-in table lacks once
            /// `hc_zone_load` has its file), in a module built with `tz` too:
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
            /// JJY's call-sign frame of minute 15 or 45 read in a year the caller
            /// names, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `frame` is as for `hc_radio_decode`, and null is
            /// `HC_ERROR_NULL_POINTER`. The line is the WebAssembly module's: the
            /// minute and the stop notice. A `year` outside 1 to 9999 is
            /// `HC_ERROR_OUT_OF_RANGE`, and a frame that is not a call-sign frame
            /// `HC_ERROR_MALFORMED`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// JJY's call-sign frame of minute 15 or 45 read in a year the caller
            /// names, as one UTF-8 line, returning the byte length written.
            ///
            /// `frame` is one character a second, `0`, `1` and `M`, as for
            /// `hc_radio_decode`; the frame carries no year, and `year`, 1 to 9999,
            /// is the Gregorian year its day of the year is read in; any other is
            /// `HC_ERR_OUT_OF_RANGE`. Tab-separated: the POSIX second of its first
            /// marker; that minute's fixed day, hour and minute in JST; and NICT's
            /// notice of a planned stop, ST1–ST3 as 0 to 6, ST4 as `1` for a stop
            /// by day only or `0`, and ST5–ST6 as 0 to 3. An ordinary minute's
            /// frame, which `hc_radio_decode` reads, a frame that is not JJY's, an
            /// ST1–ST3 of `111` and a day the year does not have are
            /// `HC_ERR_MALFORMED`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_jjy_call_sign_decode(frame: name(frame_len), year: i64) -> line =
            $crate::time_code_lines::jjy_call_sign_decode_line;

        c {
            /// JJY's call-sign frame for minute 15 or 45 with a notice of a planned
            /// stop, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The arguments and the line are the WebAssembly module's. Another
            /// minute, a `stop_start` above 6 or a `stop_span` above 3 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// JJY's call-sign frame for minute 15 or 45 with a notice of a planned
            /// stop, as one UTF-8 line, returning the byte length written.
            ///
            /// `unix_seconds` begins minute 15 or 45 of an hour of JST, a whole
            /// minute of the years 1 to 9999; `stop_start` is ST1–ST3, 0 for no
            /// stop planned to 6 for one within 2 hours; `daytime_only` non-zero
            /// sets ST4, a stop by day only; `stop_span` is ST5–ST6, 0 to 3. The
            /// line is one cell, the frame, as `hc_radio_encode` writes one. Any
            /// other minute, a `stop_start` above 6 or a `stop_span` above 3 is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_jjy_call_sign_encode(
            unix_seconds: i64,
            stop_start: u32,
            daytime_only: flag,
            stop_span: u32,
        ) -> line =
            $crate::time_code_lines::jjy_call_sign_encode_line;

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
            /// as `1` or `0` each, the identifiers of the calendars it has
            /// vocabulary of its own for beyond the shared Gregorian months, joined
            /// by `;`, its parent in the fallback chain (empty for none), its
            /// default numbering system and its direction, `ltr` or `rtl`. Writes
            /// the required length, including the terminator, into `written`.
        }
        wasm {
            /// Every locale the module carries, as UTF-8 lines, returning the byte
            /// length written.
            ///
            /// One line per locale, in tag order, tab-separated: the BCP 47 tag, the
            /// language's name in English and in itself, whether the locale's own
            /// data names the Gregorian months, the weekdays and the Gregorian eras
            /// as `1` or `0` each, the identifiers of the calendars it has
            /// vocabulary of its own for beyond the shared Gregorian months, joined
            /// by `;`, its parent in the fallback chain, as `hc_locale_chain`
            /// takes it (empty for none), its default numbering system, a
            /// `hc_format_number` identifier, and its direction, `ltr` or `rtl`. A
            /// null `buffer` returns the length the text needs.
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
            /// The fallback chain of a locale, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `locale` is a NUL-terminated BCP
            /// 47 tag, or null for the root locale; one that does not parse is
            /// `HC_ERROR_MALFORMED`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The fallback chain of a locale, the order its data is looked up along, as
            /// UTF-8 lines, returning the byte length written.
            ///
            /// One line per step, the requested locale first and `und` last,
            /// tab-separated: the step from 0; its tag; the rule that led to it from the
            /// step before, `requested`, `language-alias` (a legacy language subtag, `iw`
            /// or `tl`, is replaced by the language CLDR's `languageAlias` gives it),
            /// `likely-script` (a language carried per script,
            /// `zh-TW`, takes the script CLDR's likely subtags give it), `extensions`
            /// (the `-u-` keys), `variant`, `parent-locales` (a parent CLDR 48's
            /// `parentLocales` name: `en-AU` to `en-001`, `zh-Hant` to root), `region` and
            /// `script` (truncation) or `root`; and `1` when `hc-i18n` carries an entry of
            /// data for exactly that tag, else `0`. A tag that does not parse is
            /// `HC_ERR_MALFORMED`, where the lines that only read from a locale take the
            /// root locale for it; the empty string is the root locale. `locale` fails as
            /// for `hc_parse_iso_date`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_locale_chain(locale: text(locale_len)) -> line =
            $crate::i18n_lines::locale_chain_lines;

        c {
            /// What a locale is, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line is the WebAssembly module's. `locale` is as for
            /// `hc_locale_chain`, and `HC_ERROR_MALFORMED` for one that does not parse.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// What a locale is, its subtags, its week, its numbering, direction and
            /// casing and the plural rules that apply, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// Tab-separated: the tag written canonically; the language, script, region
            /// and variant subtags as given, empty where absent; the `-u-` keys the tag
            /// carries, `ca`, `nu`, `fw` as an ISO weekday number and `hc`, empty where
            /// absent; the tag of the entry of data that answers for it; its parent, the
            /// next step of `hc_locale_chain`, and the rule that gave it; the numbering
            /// system numbers are written in by default; the ISO weekday number the week
            /// begins on, as `hc_first_day_of_week` has it, and CLDR's `minDays`, the
            /// fewest days of a year a week needs to be its first (4 in Germany, 1 in the
            /// United States); `ltr` or `rtl`; `standard` or `turkic` casing; `1` when
            /// month and weekday names are written with a capital; the language of the
            /// cardinal plural rules that apply, `pt-PT`, or `und` where none is carried
            /// and every number is `other`; and the canonical tag, the tag with a legacy
            /// language subtag replaced by the language CLDR's `languageAlias` gives it
            /// (`he-IL` for `iw-IL`; the tag itself where it has none). Weekend days are
            /// not here: `hc-i18n` carries
            /// CLDR's `firstDay` and `minDays` and no weekend data; the weekend laws of the
            /// holiday tables, with their sources, are in column 14 of
            /// `hc_holiday_tables`. `locale` is as for `hc_locale_chain`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_locale_info(locale: text(locale_len)) -> line =
            $crate::i18n_lines::locale_info_line;

        c {
            /// The plural category a number has in a locale, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `number` is a plain decimal, an
            /// optional minus sign and digits, and a point and digits where a trailing
            /// zero is meant: *1* and *1.0* are different questions. `kind` is
            /// `cardinal`, the form after a count, or `ordinal`, the form of a position,
            /// in any case; another is `HC_ERROR_UNKNOWN`; null `number` or `kind` is
            /// `HC_ERROR_NULL_POINTER`.
            /// Text that is not a decimal, or a tag that does not parse, is
            /// `HC_ERROR_MALFORMED`, and digits beyond a `uint64_t` are
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The plural category a number has in a locale, by CLDR 48's cardinal or
            /// ordinal rules, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: `zero`, `one`, `two`, `few`, `many` or `other`; the
            /// language of the rules that decided it, `ru`, `pt-PT`, or `und` where none
            /// is carried and everything is `other`; and the operands of UTS #35 read from
            /// the number as written, `i`, `v`, `w`, `f` and `t`. `number` is a plain
            /// decimal, an optional minus sign and digits, with a point and digits where a
            /// trailing zero is meant: *1* is `one` in English and *1.0* is `other`.
            /// `kind` is `cardinal`, the form after a count (`plurals.xml`), or `ordinal`,
            /// the form of a position (`ordinals.xml`: English 1 is `one`, 2 `two`, 3
            /// `few` and 4 `other`), in any case; the compact-notation operands `c` and
            /// `e` are not carried. Any other kind, the empty string included, is
            /// `HC_ERR_UNKNOWN`. Text that is not a decimal and a tag that
            /// does not parse are `HC_ERR_MALFORMED`; digits beyond a `u64` are
            /// `HC_ERR_OUT_OF_RANGE`. `locale` is as for `hc_locale_chain`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_plural_category(
            locale: text(locale_len),
            number: name(number_len),
            kind: name(kind_len),
        ) -> line =
            $crate::i18n_lines::plural_category_line;

        c {
            /// The names a locale has for a calendar in a width and a context, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `calendar` is a registry
            /// identifier; `width` is `wide`, `abbreviated`, `short` or `narrow` and
            /// `context` `format` or `standalone`, in any case; any other, or a calendar
            /// the registry does not carry, is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. A tag that does not parse is
            /// `HC_ERROR_MALFORMED`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The names a locale has for a calendar in a width and a context, one per
            /// line, returning the byte length written.
            ///
            /// Tab-separated: the kind of name, `month`; `month-in-leap-year` where the
            /// locale names a month differently in a year with the calendar's intercalary
            /// month (*Adar II*); every other cycle of the calendar by its kind,
            /// `weekday`, `stem`, `branch`; `quarter`; and `day-period` for `am` and `pm`;
            /// the position from 1, the ISO number for a weekday; the name; and the tag of
            /// the entry of data that answered. A position the locale and the calendar do
            /// not name has no line, so the months of a calendar that numbers them are not
            /// listed. `width` is `wide`, `abbreviated`, `short` or `narrow` and `context`
            /// `format`, inside a date, where Russian writes *сентября*, or `standalone`,
            /// *сентябрь*, in any case. A calendar the registry does not carry, a width or
            /// a context not named is `HC_ERR_UNKNOWN`; a tag that does not parse
            /// `HC_ERR_MALFORMED`. `locale` is as for `hc_locale_chain`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_names(
            locale: text(locale_len),
            calendar: name(calendar_len),
            width: name(width_len),
            context: name(context_len),
        ) -> line =
            $crate::i18n_lines::names_lines;

        c {
            /// A text recased as a locale cases it, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `mode` is `lower`, `upper`,
            /// `capitalise-first`, `lowercase-first`, `sentence-start` or `in-sentence`,
            /// in any case; another is `HC_ERROR_UNKNOWN`, and null `mode`
            /// `HC_ERROR_NULL_POINTER`; `text` is read as the empty string when null. A
            /// tag that does not parse is `HC_ERROR_MALFORMED`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// A text recased as a locale cases it, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// Tab-separated: the text recased; the mode; `standard` or `turkic`; and the
            /// tag of the entry of data that answered. `lower` and `upper` differ from
            /// Unicode's default mappings only in Turkish and Azerbaijani, where `iyi` is
            /// `İYİ`; `capitalise-first` and `lowercase-first` recase the first character
            /// only, since a title case of every word needs a word-break rule `hc-i18n`
            /// does not have; `sentence-start` sets a month or weekday name as it opens a
            /// sentence, always with a capital, and `in-sentence` inside one, with a
            /// capital in English and German and none in French. `mode` is read in any
            /// case; another, the empty string included, is `HC_ERR_UNKNOWN`. A tag that
            /// does not parse is `HC_ERR_MALFORMED`. `locale` is as for
            /// `hc_locale_chain`. A null `buffer` returns the length the text needs.
        }
        fn hc_case(
            locale: text(locale_len),
            mode: name(mode_len),
            text: text(text_len),
        ) -> line =
            $crate::i18n_lines::case_line;

        c {
            /// A text made safe to embed in text running a locale's direction, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `mode` is `field`, `first-strong` or
            /// `strip`, in any case; another is `HC_ERROR_UNKNOWN`, and null `mode`
            /// `HC_ERROR_NULL_POINTER`; `text` is read as the empty string when null. A
            /// tag that does not parse is `HC_ERROR_MALFORMED`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// A text made safe to embed in text running a locale's direction, by the
            /// Unicode bidirectional isolates, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// Tab-separated: the text, with the isolates U+2066 to U+2069 around it where
            /// the mode says; the locale's direction, `ltr` or `rtl`; the text's own, by the
            /// first-strong rule of UAX 9, empty where it has no strong character, as a
            /// date made of digits has none; `1` when isolates were added; and the mode.
            /// `field` isolates only where the two directions disagree, so a Latin date in
            /// Arabic prose is wrapped in an LTR isolate and Arabic in English prose in an
            /// RTL one, and a field running the locale's own direction, or with no strong
            /// character, is left alone; `first-strong` always wraps the text in U+2068 and
            /// U+2069 and leaves the direction to the renderer; `strip` removes every
            /// isolate and directional mark, for comparing or hashing. `mode` is read in any
            /// case; another is `HC_ERR_UNKNOWN`. A tag that does not parse is
            /// `HC_ERR_MALFORMED`. `locale` is as for `hc_locale_chain`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_isolate(
            locale: text(locale_len),
            mode: name(mode_len),
            text: text(text_len),
        ) -> line =
            $crate::i18n_lines::isolate_line;


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
            /// identifier, `lahiri`, `lahiri-rashtriya`, `lahiri-crc-1955`, `lahiri-drik`, `raman`, `krishnamurti`, `reingold-dershowitz`
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
            /// name. `ayanamsa` is an identifier, `lahiri`, `lahiri-rashtriya`, `lahiri-crc-1955`, `lahiri-drik`, `raman`, `krishnamurti`,
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
            /// The tithi in progress at a POSIX timestamp, with the moments it began
            /// and ends, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `sky` is `true`, an ayanāṃśa
            /// `hc_panchanga_at` names, the tithi not moving with it, or
            /// `surya-siddhanta`, in any case; anything else is `HC_ERROR_UNKNOWN`,
            /// and null `HC_ERROR_NULL_POINTER`. An instant outside the years −1000 to
            /// 3000 on the true sky, or the days of Kali Yuga 1 to 10 000 on the
            /// Siddhānta's, is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The tithi in progress at a POSIX timestamp, with the moments it began
            /// and ends, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated, eight columns: the tithi's number, 1 for śukla pratipadā
            /// through 30 for amāvasyā; the fortnight, `shukla` or `krishna`; its day
            /// in the fortnight, 1 to 15; its name in IAST, `Caturdaśī`, Pūrṇimā for
            /// the fifteenth of the bright fortnight and Amāvasyā for the thirtieth;
            /// the instants it began and ends, the Moon having gained 12° on the Sun
            /// (a tithi runs 0.8 to 1.2 days), and the instant read, each as whole
            /// POSIX seconds of Universal Time, rounded down; and the sky, `true` or
            /// `surya-siddhanta`. `sky` is `true`, or an ayanāṃśa `hc_panchanga_at`
            /// names, which the tithi, taken from the elongation, does not depend on,
            /// or `surya-siddhanta`, for the *Sūrya Siddhānta*'s Sun and Moon, in any
            /// case; anything else is `HC_ERR_UNKNOWN`. An instant outside the years
            /// −1000 to 3000 on the true sky, or the days of Kali Yuga 1 to 10 000 on
            /// the Siddhānta's, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_tithi_at(unix_seconds: i64, sky: name(sky_len)) -> line =
            $crate::panchanga_lines::tithi_at_lines;

        c {
            /// The tithis in progress between a fixed day's sunrise at a place and
            /// the next, as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `sky` is as for `hc_tithi_at`;
            /// a place off the globe, a day outside the years −1000 to 3000, and on
            /// the Siddhānta's sky a place beyond 65° of latitude or a day outside
            /// Kali Yuga 1 to 10 000, is `HC_ERROR_OUT_OF_RANGE`; a day, or its
            /// morrow, on which the Sun does not rise there is `HC_ERROR_NO_DATA`.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// The tithis in progress between a fixed day's sunrise at a place and
            /// the next, as UTF-8 lines, returning the byte length written.
            ///
            /// One line a tithi in order, the eight columns of `hc_tithi_at` with the
            /// instant read the day's sunrise, then three flags: `1` when the tithi
            /// holds that sunrise, the one the day carries, as `hc_hindu_lunar_date`
            /// reads it; `1` when it holds the next sunrise too, a repeated
            /// (*adhika*) tithi, the day's again tomorrow; `1` when it holds neither
            /// sunrise, a skipped (*kṣaya*) tithi, which begins after one sunrise and
            /// ends before the next and which no civil day carries. Two or three
            /// lines a day. The place is the latitude and longitude in degrees, north
            /// and east positive, and the elevation in metres. `sky` is as for
            /// `hc_tithi_at`, and on the Siddhānta's the day is read at the book's own
            /// sunrise. A day, or its morrow, on which the Sun does not rise at the
            /// place is `HC_ERR_NO_DATA`: no other moment is put in its place. A
            /// place off the globe, a day outside the years −1000 to 3000, and on the
            /// Siddhānta's sky a place beyond 65° of latitude or a day outside Kali
            /// Yuga 1 to 10 000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_tithis_of_day(
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            sky: name(sky_len),
        ) -> line =
            |fixed, latitude, longitude, elevation, sky| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::panchanga_lines::tithis_of_day_lines(fixed, place, sky)
                    })
            };

        c {
            /// Every named ayanāṃśa, as NUL-terminated UTF-8 lines in a caller-owned
            /// buffer.
            ///
            /// The lines are the WebAssembly module's, one an ayanāṃśa. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// Every named ayanāṃśa, as UTF-8 lines, returning the byte length
            /// written.
            ///
            /// One line an ayanāṃśa, in the table's order, tab-separated: the
            /// identifier `hc_panchanga_at` reads (`lahiri`, `lahiri-drik`,
            /// `raman`); the full name; the Julian date the anchor is quoted for;
            /// the anchor in degrees there; and where it is from, the Swiss
            /// Ephemeris's sidereal modes, the Calendar Reform Committee's report,
            /// Drik Panchang's pages or the *Rashtriya Panchang*; and what the anchor
            /// stands for, `mean` (the precession alone) or `true` (the mean value plus
            /// the nutation in longitude of the day, which the Committee's and the
            /// *Rashtriya Panchang*'s printed values are). The value at another moment
            /// is the anchor carried by the IAU 2006 general precession, and for a
            /// `true` one by the nutation of each day besides, which `hc_ayanamsa_at`
            /// gives. The readings of Lahiri's differ by up to 25″, so each is its own
            /// name (`docs/policy.md` §5). A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_ayanamsas() -> line =
            || Ok($crate::panchanga_lines::ayanamsas_lines());

        c {
            /// A named ayanāṃśa's value at a POSIX timestamp, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// `ayanamsa` is as for `hc_panchanga_at`; null is `HC_ERROR_NULL_POINTER`
            /// and one not named, the Siddhānta's included, `HC_ERROR_UNKNOWN`. The
            /// line is the WebAssembly module's. An instant outside the years −1000
            /// to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// A named ayanāṃśa's value at a POSIX timestamp, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the ayanāṃśa in degrees at the instant, the angle
            /// between the tropical and the sidereal zero point, about 24.2° in the
            /// 2020s and growing by about 50″ a year; the identifier; the name; the
            /// anchor's Julian date and degrees, as `hc_ayanamsas` has them; the
            /// instant read as whole POSIX seconds; and `mean` or `true`, what the
            /// anchor stands for, as `hc_ayanamsas` has it. `ayanamsa` is as for
            /// `hc_panchanga_at`; one not named is `HC_ERR_UNKNOWN`. An instant
            /// outside the years −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_ayanamsa_at(unix_seconds: i64, ayanamsa: name(ayanamsa_len)) -> line =
            $crate::panchanga_lines::ayanamsa_at_line;

        c {
            /// The value at a POSIX timestamp of an ayanāṃśa the caller anchors, as
            /// one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's, with `custom` as the identifier
            /// and name. An instant or an anchor's Julian date outside the years
            /// −1000 to 3000, an anchor that is not finite and degrees outside −360°
            /// to 360° are `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The value at a POSIX timestamp of an ayanāṃśa the caller anchors, as
            /// one UTF-8 line, returning the byte length written.
            ///
            /// For a school this table has not named: `degrees_at_anchor` degrees at
            /// the Julian date `anchor_julian_date`, carried to the instant by the
            /// IAU 2006 general precession as the named ones are. The line is
            /// `hc_ayanamsa_at`'s, with `custom` as the identifier and the name and
            /// `mean` as the kind. An
            /// instant or an anchor's Julian date outside the years −1000 to 3000, an
            /// anchor that is not finite and degrees outside −360° to 360° are
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_ayanamsa_from_anchor(
            unix_seconds: i64,
            anchor_julian_date: f64,
            degrees_at_anchor: f64,
        ) -> line =
            $crate::panchanga_lines::ayanamsa_from_anchor_line;

        c {
            /// The two readings of a festival's day where the sects part, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one a reading. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// The two readings of a festival's day where the sects part, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// One line a reading, `smarta` first, tab-separated: the identifier
            /// `hc_janmashtami` reads; the English name; how the day is taken, in
            /// words; and where the reading is from. The Smārta reading takes the day
            /// that holds the part of the day the rite belongs to, which for
            /// Janmāṣṭamī is the night; the Vaiṣṇava reading, the first day whose
            /// sunrise carries the tithi. Two competing conventions, each its own name
            /// (`docs/policy.md` §5).
        }
        fn hc_festival_readings() -> line =
            || Ok($crate::panchanga_lines::festival_readings_lines());

        c {
            /// The day of Kṛṣṇa Janmāṣṭamī in a Gregorian year at a place by a
            /// reading, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `reading` is `smarta` or
            /// `vaishnava`; one not named, or an ayanāṃśa not named, is
            /// `HC_ERROR_UNKNOWN`; a place beyond the latitude where the Sun
            /// rises every day, or a year outside 1700 to 2299, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The day of Kṛṣṇa Janmāṣṭamī in a Gregorian year at a place by a
            /// reading, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the reading's identifier, `smarta` or `vaishnava`, as
            /// `hc_festival_readings` lists them; the year; the fixed day; its
            /// Gregorian year, month and day; the tithi the day carries at sunrise
            /// there (23, Aṣṭamī, or 24 for a Vaiṣṇava day that is Navamī); and the
            /// ayanāṃśa. Śrāvaṇa kṛṣṇa 8 falls in August or September, in the Śaka year
            /// the Gregorian one less 78. The place is the latitude and longitude in
            /// degrees, north and east positive, and the elevation in metres;
            /// `ayanamsa` is as for `hc_panchanga_at`. A reading or an ayanāṃśa not
            /// named is `HC_ERR_UNKNOWN`; a place beyond the latitude where the Sun
            /// rises every day, or a year outside 1700 to 2299, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_janmashtami(
            year: i64,
            reading: name(reading_len),
            latitude: f64,
            longitude: f64,
            elevation: f64,
            ayanamsa: name(ayanamsa_len),
        ) -> line =
            |year, reading, latitude, longitude, elevation, ayanamsa| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::panchanga_lines::janmashtami_line(year, reading, place, ayanamsa)
                    })
            };

        c {
            /// The Vaiṣṇava day of a tithi of a month of a Śaka year at a place, as
            /// one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. A month outside 1 to 12, a Śaka
            /// year outside 1622 to 2221 or a place beyond the latitude where the Sun
            /// rises every day is `HC_ERROR_OUT_OF_RANGE`; a tithi outside 1 to 30 is
            /// `HC_ERROR_INVALID_DATE`; an ayanāṃśa not named is `HC_ERROR_UNKNOWN`.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// The Vaiṣṇava day of a tithi of a month of a Śaka year at a place, as
            /// one UTF-8 line, returning the byte length written.
            ///
            /// The first day of the amānta month whose sunrise carries the tithi or a
            /// later one: the day a tithi holds a sunrise, and the day after for a
            /// tithi that holds none. Tab-separated: the Śaka year; the month, 1 for
            /// Chaitra through 12 for Phālguna; the tithi, 1 for śukla pratipadā
            /// through 30 for amāvasyā; the fixed day; its Gregorian year, month and
            /// day; the tithi the day carries at sunrise; and the ayanāṃśa. The place
            /// and `ayanamsa` are as for `hc_janmashtami`. A month outside 1 to 12, a
            /// Śaka year outside 1622 to 2221 or a place beyond the latitude where the
            /// Sun rises every day is `HC_ERR_OUT_OF_RANGE`; a tithi outside 1 to 30 is
            /// `HC_ERR_INVALID_DATE`; an ayanāṃśa not named is `HC_ERR_UNKNOWN`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_vaishnava_day(
            saka_year: i64,
            month: u32,
            tithi: u32,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            ayanamsa: name(ayanamsa_len),
        ) -> line =
            |saka_year, month, tithi, latitude, longitude, elevation, ayanamsa| {
                let month = u8::try_from(month)
                    .map_err(|_| $crate::boundary::Refusal::OutOfRange)?;
                let tithi = u8::try_from(tithi)
                    .map_err(|_| $crate::boundary::Refusal::InvalidDate)?;
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::panchanga_lines::vaishnava_day_line(
                            saka_year, month, tithi, place, ayanamsa,
                        )
                    })
            };

        c {
            /// The part of a tithi that Bhadra, the karaṇa Viṣṭi, does not cover, for
            /// a tithi of a month of a Śaka year at a place, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. A month outside 1 to 12, a Śaka
            /// year outside 1622 to 2221 or a place beyond the latitude where the Sun
            /// rises every day is `HC_ERROR_OUT_OF_RANGE`; a tithi outside 1 to 30 is
            /// `HC_ERROR_INVALID_DATE`; an ayanāṃśa not named is `HC_ERROR_UNKNOWN`.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// The part of a tithi that Bhadra, the karaṇa Viṣṭi, does not cover, for
            /// a tithi of a month of a Śaka year at a place, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// The span Rakṣā Bandhana waits for: the second half of the full moon's
            /// tithi, and for the fourth tithi of a fortnight the first half; a tithi
            /// Viṣṭi never falls on has both moments empty. Tab-separated: the Śaka
            /// year, the month (1 for Chaitra through 12 for Phālguna), the tithi
            /// (1 through 30), the fixed day of the month's first day, the moments the
            /// span begins and ends as whole POSIX seconds of Universal Time rounded
            /// down, and the ayanāṃśa. The place and `ayanamsa` are as for
            /// `hc_janmashtami`, and the errors as for `hc_vaishnava_day`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_vishti_free_span(
            saka_year: i64,
            month: u32,
            tithi: u32,
            latitude: f64,
            longitude: f64,
            elevation: f64,
            ayanamsa: name(ayanamsa_len),
        ) -> line =
            |saka_year, month, tithi, latitude, longitude, elevation, ayanamsa| {
                let month = u8::try_from(month)
                    .map_err(|_| $crate::boundary::Refusal::OutOfRange)?;
                let tithi = u8::try_from(tithi)
                    .map_err(|_| $crate::boundary::Refusal::InvalidDate)?;
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::panchanga_lines::vishti_free_span_line(
                            saka_year, month, tithi, place, ayanamsa,
                        )
                    })
            };

        c {
            /// Rāhu and Ketu at a POSIX timestamp, as one NUL-terminated UTF-8 line in
            /// a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. An ayanāṃśa not named is
            /// `HC_ERROR_UNKNOWN`; an instant outside the years −1000 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Rāhu and Ketu at a POSIX timestamp, as one UTF-8 line, returning the
            /// byte length written.
            ///
            /// The mean ascending node and the descending node opposite it.
            /// Tab-separated: `mean`, the kind of node (the true node is not carried);
            /// Rāhu's sidereal longitude in degrees, the sign it stands in as a number
            /// from 1 for Meṣa, its identifier and its Sanskrit name; the same four
            /// for Ketu; the ayanāṃśa, as for `hc_panchanga_at`; and the instant read.
            /// An ayanāṃśa not named is `HC_ERR_UNKNOWN`; an instant outside the years
            /// −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_rahu_at(unix_seconds: i64, ayanamsa: name(ayanamsa_len)) -> line =
            $crate::panchanga_lines::rahu_at_line;

        c {
            /// The entries of the mean node into the sidereal signs in a span of POSIX
            /// timestamps, as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. An ayanāṃśa not named is
            /// `HC_ERROR_UNKNOWN`; a span with an end outside the years −1000 to 3000
            /// or longer than a hundred Julian years is `HC_ERROR_OUT_OF_RANGE`. Writes
            /// the required length, including the terminator, into `written`.
        }
        wasm {
            /// The entries of the mean node into the sidereal signs in the half-open
            /// span `[from, to)` of POSIX timestamps, as UTF-8 lines, returning the
            /// byte length written.
            ///
            /// One line an entry in time order, tab-separated: the moment as whole
            /// POSIX seconds of Universal Time rounded down; the sign Rāhu leaves, by
            /// identifier and Sanskrit name; the sign it enters, by the same two; and
            /// the sign Ketu enters, by the same two, opposite Rāhu's. Rāhu moves
            /// backward, a sign in about 566 days, so a century has about sixty-five.
            /// An empty span is no line. An ayanāṃśa not named is `HC_ERR_UNKNOWN`; a
            /// span with an end outside the years −1000 to 3000 or longer than a
            /// hundred Julian years is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns
            /// the length the text needs.
        }
        fn hc_rahu_ingresses(from: i64, to: i64, ayanamsa: name(ayanamsa_len)) -> line =
            $crate::panchanga_lines::rahu_ingresses_lines;

        c {
            /// The first day of a year of a historical Indian era over the lunisolar
            /// months, written to the out-parameter `out_fixed`.
            ///
            /// `calendar` is `vikram-samvat-kartikadi`, `rajyabhisheka-saka`,
            /// `saptarshi`, `gupta`, `valabhi`, `kalachuri` or `lakshmana-sena`; one
            /// not named is `HC_ERROR_UNKNOWN` and a year outside the months the era is
            /// read over `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The first day of a year of a historical Indian era over the lunisolar
            /// months, as a fixed day, or an error sentinel.
            ///
            /// `calendar` is `vikram-samvat-kartikadi`, `rajyabhisheka-saka`,
            /// `saptarshi`, `gupta`, `valabhi`, `kalachuri` or `lakshmana-sena`. The
            /// day is the era's own: a year opens at Kārttika śukla 1, Āśvina śukla 1
            /// or Jyeṣṭha śukla 13 as its source states, not at a month's first day.
            /// One not named is `HC_ERR_UNKNOWN` and a year outside the months the era
            /// is read over `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_era_new_year(calendar: name(calendar_len), year: i64) -> value(out_fixed: i64) =
            $crate::panchanga_lines::era_new_year;

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
            /// sidereal longitudes, the elongation, the tithi and the Sun's sign by
            /// number, identifier and Sanskrit name.
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
            /// for Meṣa through 12 for Mīna), its identifier, `mina`, and its
            /// Sanskrit name, `Mīna`. The instant is read as Universal
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
            /// The modern Olympic Games of a season, as NUL-terminated UTF-8 lines in
            /// a caller-owned buffer.
            ///
            /// `season` is `summer` or `winter`, in any case; anything else is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. The lines are the
            /// WebAssembly module's, one an edition in order. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// The modern Olympic Games of a season, as UTF-8 lines, returning the
            /// byte length written.
            ///
            /// One line an edition, in order, as Olympedia's list of editions gives
            /// them, tab-separated: the number, which a Summer Games not held keeps
            /// (the VI of 1916) and a Winter Games not held has none of, so it is
            /// empty; the year the Games were awarded to, 2020 for the Tokyo Games
            /// held in 2021; the host city as Olympedia spells it, `Athina`; their
            /// status, `celebrated`, `not-held` or `scheduled`; and the fixed days of
            /// the opening and closing ceremonies, each empty where Olympedia dates
            /// none (Paris 1900 had no ceremony, St. Louis 1904 no closing, and a
            /// Games not held or not yet held neither). `hc_ioc_olympiad_on` gives the
            /// Olympiad a day belongs to. A season not named is `HC_ERR_UNKNOWN`. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_olympic_games(season: name(season_len)) -> line =
            $crate::calendar_values::olympic_games_lines;

        c {
            /// Every era of a table, one a line, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// `table` is `japanese`, `chinese-regnal`, `korean-regnal` or
            /// `vietnamese-regnal-nguyen`, in any case; anything else is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. The lines are
            /// the WebAssembly module's.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// Every era of a table, one a line, as UTF-8 lines, returning the byte
            /// length written.
            ///
            /// The tables' own eras, not the locale data's list `hc_calendar_eras`
            /// names in a locale: `japanese` has the 248 eras from 大化 to 令和 where
            /// CLDR lists 236, `chinese-regnal` the 37 of the Ming, the Southern
            /// Ming, the Shun and the Qing, `korean-regnal` the three of the
            /// Korean Empire, which no locale data lists, and
            /// `vietnamese-regnal-nguyen` the twelve of the Nguyễn dynasty from Gia
            /// Long to Bảo Đại. Tab-separated, twelve
            /// columns: the era's code (`reiwa`, `showa-1312`, `gia-long`); its name
            /// in the characters of its source; its reading, in hiragana, pinyin or
            /// hangul, or the Vietnamese name with its diacritics; its romanisation,
            /// the Vietnamese name again; the court (`unified`, `northern` or
            /// `southern`) or the dynasty (`ming`, `southern-ming`, `shun`, `qing`,
            /// `korean-empire` or `nguyen`); the Gregorian year of its first year
            /// (元年), and of its last where the table has one, the Chinese table's,
            /// the Korean table's and the Vietnamese table's for a kept era; the
            /// fixed day it began, the day the Japanese era was proclaimed, the
            /// Korean chosen or the Vietnamese first in force, and the last fixed day
            /// it was in force (the day before the next era of its court's stream,
            /// a Unified era read in the Northern one, which carried 建武 to 暦応;
            /// before the lapse of 白雉 and 朱鳥; for 元中 before the reunion of 1392;
            /// and for a Nguyễn era the day before the next kept era or the
            /// abdication of 30 August 1945), each empty where the table has none,
            /// as for the Chinese eras, an era known to the month alone and the
            /// Vietnamese era not kept; the status, `attested`, `disputed` or
            /// `month-only` for the Japanese, `kept` or `not-kept` for the Chinese
            /// and the Vietnamese; the first day under the other reading, 光武
            /// backdated to 1 January 1897, empty elsewhere; and the table's note.
            /// The table is read before anything else: one not named is
            /// `HC_ERR_UNKNOWN`. A null `buffer` returns the length the text needs.
        }
        fn hc_era_table(table: name(table_len)) -> line =
            $crate::calendar_values::era_table_lines;

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
            /// falls in, the next civil day from the reckoning's nightfall, the
            /// start of its twelve hours of night, 18:00 of that mean time; the
            /// minutes of Jerusalem mean time since the midnight of the moment's
            /// civil day; the *tekufah*'s identifier; `1` when the moment is after
            /// that nightfall and before midnight, else `0`; and the moment's civil
            /// day, a fixed day. A year outside 1 to 9999 is
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
            /// identifier and its Sanskrit name, `mesha` and `Meṣa`. The locale
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
            /// name, the start and the end, the mark, and the four cells of a
            /// missing solar event. A place off the globe, or a day outside the years −1000 to
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
            /// `night`; the muhūrta's number in it, 1 to 15; its name as English
            /// Wikipedia's "Muhurta" tabulates them, `Rudra` to `Samudra`; its
            /// start and its end as whole POSIX seconds of Universal Time, rounded
            /// down; what the pañcāṅga prints it as — `abhijit` for the eighth of
            /// the daylight on a day but a Wednesday, `dur-muhurtam` for the
            /// weekday's one or two, Drik Panchang's, else empty; and the four
            /// cells of a missing solar event, as `hc_kalam` writes them, where the
            /// Sun does not rise or set and the start and end are empty. The place is the latitude and
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
            /// in Devanagari, the weekday's nakṣatra by number, identifier and name,
            /// the start and the end, whether
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
            /// for Revatī, its identifier and its name, as for `hc_nakshatra_at`;
            /// the start and the end of the part as whole POSIX seconds
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
            /// The line is the WebAssembly module's: the nakṣatra by number,
            /// identifier and name, when the Moon entered it and leaves it, the
            /// instant read, and the ayanāṃśa by
            /// identifier and full name. `ayanamsa` is `lahiri`, `lahiri-rashtriya`, `lahiri-crc-1955`, `lahiri-drik`, `raman`,
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
            /// ayanāṃśa, its identifier, `ashvini`, and its name, `Aśvinī`; the
            /// instants the Moon entered it and leaves it and the
            /// instant read, as whole POSIX seconds of Universal Time, rounded
            /// down; and the ayanāṃśa by its identifier and its full name.
            /// `ayanamsa` is `lahiri`, `lahiri-rashtriya`, `lahiri-crc-1955`, `lahiri-drik`, `raman`, `krishnamurti`, `reingold-dershowitz`
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
            /// the day they were born on, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer, one an entry.
            ///
            /// The lines are the WebAssembly module's: the kind, the identifier,
            /// the entry's name, what the person keeps and whether the day is it.
            /// `birth_fixed` is the fixed day of the birth, whose 干支 year, turning
            /// at 立春, is read at `meridian`. `meridian` is as for `hc_almanac_cycles`; null is
            /// `HC_ERROR_NULL_POINTER` and a meridian not read `HC_ERROR_UNKNOWN`.
            /// A day or a birth outside the years −1000 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Whether a fixed day is one of a person's own 五墓日 or 三箇の悪日, by
            /// the day they were born on, as UTF-8 lines, one an entry, returning
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
            /// for the person, else `0`. `birth_fixed` is the fixed day the person
            /// was born on, and their year is the 干支 year in force on it at
            /// `meridian`, turning at 立春, as the almanac counts a person's year: a
            /// birth on 1 February 1928, before that year's 立春, is of 丁卯.
            /// `meridian` is as for `hc_almanac_cycles`; a meridian not read is
            /// `HC_ERR_UNKNOWN`, and a day or a birth outside the years −1000 to
            /// 3000 `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_almanac_person_days(
            fixed: i64,
            birth_fixed: i64,
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
            /// reading and its decimal, with the attributes of the lunar day and of
            /// the calendar day after the symbols. `calendar` is `tibetan`,
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
            /// gender and the element's colour. Then the attributes Janson's rules
            /// and Henning's almanacs give a day, each with its number as the
            /// identifier: of the lunar day that ends on the calendar day, none on
            /// the first of two days with one number, `lunar-day-animal` (1 for the
            /// Mouse, with its name), `lunar-day-element` (1 for Wood, its name and
            /// colour, where `month-symbol` is given), `lunar-day-trigram` (1 for
            /// *li* to 8 for *zon*, its Tibetan and Chinese names, its direction and
            /// its element) and `lunar-day-number` (1 to 9, its colour, element and
            /// direction); of the calendar day `day-trigram`, `day-number-janson` and
            /// `day-number-henning`, the last on the versions Henning's almanacs
            /// print it for, not the Tsurphu or the Mongolian; `chinese-mansion`, 1
            /// for *Jiao* to 28; and `element-pair`, the weekday the almanac names
            /// the day by, which on the Bhutanese versions is a day ahead of the
            /// `weekday` line's, and the elements of the weekday and of the mansion.
            /// A day outside the version's range is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
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
            /// outside 1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`, and one that holds
            /// no solstice `HC_ERROR_NO_DATA`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The Bhutanese calendar's winter solstice of a Gregorian year, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// The instant its mean Sun reaches 250° in the year, which falls in the
            /// first days of January today; the mean Sun's year is 365.270 645
            /// days, so the day drifts a day later every 35½ years, and a year of
            /// 365 days can hold none: the December one of 24 December 1700, none
            /// in 1923, 1927 … 1957, the January one after. Tab-separated: the
            /// fixed day it falls on; the weekday and time the almanac prints,
            /// days after Saturday's dawn, nāḍī and pala, `2;51,38`; and the local
            /// Julian Date as a decimal. A year outside 1000 to 3000 is
            /// `HC_ERR_OUT_OF_RANGE`, and one that holds no solstice
            /// `HC_ERR_NO_DATA`. A null `buffer` returns the length the text needs.
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
            /// the Sun must be in, each by its identifier and its Sanskrit name,
            /// `vrishabha` and `Vṛṣabha`; `1` when the Moon must be with the Sun at the
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
            /// Every condition of the Kumbh Mela, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one a condition. `locale` is as
            /// for `hc_kumbh`. Writes the required length, including the terminator,
            /// into `written`.
        }
        wasm {
            /// Every condition of the Kumbh Mela, as UTF-8 lines, returning the byte
            /// length written.
            ///
            /// The seven the Mela Adhikari gives, in its order, each the first
            /// eleven cells of `hc_kumbh`'s line that say what the condition is: its
            /// identifier, the one `hc_kumbh` and `hc_kumbh_by_sky` take; its site's
            /// identifier, and the site's name in the locale and the tag that named
            /// it; the river; the signs Jupiter and the Sun must be in, each by
            /// identifier and Sanskrit name; `1` when the Moon must be with the Sun at
            /// the new moon; and the source. The locale argument fails as
            /// `hc_parse_iso_date` does. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_kumbh_yogas(locale: text(locale_len)) -> line =
            |locale| Ok($crate::reckoning_lines::kumbh_yogas_lines(locale));

        c {
            /// Every river of the Pushkaram, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one a river. `locale` is as for
            /// `hc_kumbh`. Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// Every river of the Pushkaram, as UTF-8 lines, returning the byte length
            /// written.
            ///
            /// One line a river, Meṣa's first, tab-separated: its identifier,
            /// `pushkaram-ganga` to `pushkaram-pranahita`; its name in the locale and
            /// the tag that named it; the region the source keeps it in for the sign,
            /// in English, empty where it names none; the sign Jupiter enters, by
            /// identifier and Sanskrit name; and where the pairing comes from. A sign
            /// with two rivers, Vṛścika's Bhima and Tamraparni and Dhanus's Tapti and
            /// Brahmaputra, has a line for each, as `hc_pushkaram` writes. The locale
            /// argument fails as `hc_parse_iso_date` does. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_pushkaram_rivers(locale: text(locale_len)) -> line =
            |locale| Ok($crate::reckoning_lines::pushkaram_rivers_lines(locale));


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
            /// `locale` and the tag that named it, its region, the sign by
            /// identifier and name, the first
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
            /// sign's identifier and Sanskrit name, `simha` and `Siṃha`; the first and last days as fixed days; and the
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

        c {
            /// The day and the moment the year changes at the solar New Year of the
            /// Burmese, Khmer or Lao calendar, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `calendar` is `burmese`, `khmer`
            /// or `lao`, in any case; another is `HC_ERROR_UNKNOWN` and null
            /// `HC_ERROR_NULL_POINTER`. `year` is the calendar's own count — the
            /// Myanmar Era, 1 to 3000; the Buddhist Era, 2444 to 2744; the
            /// Chulasakarat, 1301 to 1401 — and one outside it is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The day and the moment the year changes at the solar New Year of the
            /// Burmese, Khmer or Lao calendar, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// Tab-separated: the calendar; the year as given; the fixed day on
            /// which the new year begins by the country's clock — the day after
            /// Thingyan's *atat* for `burmese`, the *Laeung Sak* day for `khmer`,
            /// Dupertuis's table's day for `lao`; the seconds after midnight at
            /// which the year changes, which the Khmer New Year announcements give
            /// to the second (02:15:00 on 16 April 2024) and the Lao arithmetic
            /// gives as the same quantity, and which is empty for `burmese`, whose
            /// source states the moment as a Julian Date and no announcement of
            /// the clock time was read; the Chulasakarat year that begins, empty
            /// for `burmese`; and, for `burmese` alone, the festival's days — the
            /// *akyo* eve, the *akya* first day, the last *akyat* and the *atat*
            /// day the old year ends on, as the Myanmar holidays list them
            /// (13–16 April 2024 for 1386 ME, the New Year on the 17th). The *Maha
            /// Songkran* moment at which the Khmer and Lao festivals begin, two to
            /// three days earlier, is not carried, the Cambodian *hora*'s rule for
            /// the true Sun not having been read. `calendar` is `burmese`, `khmer`
            /// or `lao`, in any case; another is `HC_ERR_UNKNOWN`. `year` is the
            /// calendar's own count — the Myanmar Era, 1 to 3000; the Buddhist Era,
            /// 2444 to 2744; the Chulasakarat, 1301 to 1401 — and one outside it is
            /// `HC_ERR_OUT_OF_RANGE`. The name's pointer and bytes fail as for
            /// `hc_parse_iso_date`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_solar_new_year(calendar: name(calendar_len), year: i64) -> line =
            $crate::calendar_values::solar_new_year_line;

        c {
            /// The kind of lunar year a year of the Khmer or the Lao calendar is,
            /// as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `calendar` is `khmer` or `lao`, in any case; anything else is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. `year` is the
            /// calendar's own, the Buddhist Era for `khmer` and the Chulasakarat for
            /// `lao`, as `hc_solar_new_year` takes it; one outside the calendar's
            /// range is `HC_ERROR_OUT_OF_RANGE`. The line is the WebAssembly
            /// module's. Writes the required length, including the terminator,
            /// into `written`.
        }
        wasm {
            /// The kind of lunar year a year of the Khmer or the Lao calendar is,
            /// as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the calendar, `khmer` or `lao`; the year as it
            /// numbers it, the Buddhist Era for `khmer` (2444 to 2744) and the
            /// Chulasakarat for `lao` (1301 to 1401); the kind under the
            /// *suryayatra* rule as the country applies it, `normal` (twelve months,
            /// 354 days), `extra-day` (a 30th day in month 7, 355 days) or
            /// `extra-month` (month 8 twice, 384 days); its days; `1` when the year
            /// has the doubled month 8, else `0`; its Thai name and its Khmer name;
            /// and the fixed day and the seconds after midnight at which the solar
            /// New Year changes the year, `hc_solar_new_year`'s. `calendar` is read
            /// in any case; anything else, the empty string included, is
            /// `HC_ERR_UNKNOWN`, and a year outside the range
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_southeast_asian_year_type(calendar: name(calendar_len), year: i64) -> line =
            $crate::calendar_values::southeast_asian_year_type_line;

        c {
            /// A fixed day in the Maya counts under a named correlation constant,
            /// as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `correlation` is `gmt` or `584283`, `gmt2`, `gmt-plus-two` or
            /// `584285`, or `martin-skidmore` or `584286`, in any case; anything
            /// else is `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. A day
            /// before 0.0.0.0.0 or after 19.19.19.17.19 under the constant is
            /// `HC_ERROR_OUT_OF_RANGE`. The line is the WebAssembly module's.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// A fixed day in the Maya counts under a named correlation constant,
            /// as one UTF-8 line, returning the byte length written.
            ///
            /// The correlation is the caller's choice by name, where `hc_describe_day`
            /// carries the three constants as three calendars each: `gmt` or
            /// `584283`, the Goodman–Martínez–Thompson constant; `gmt2`,
            /// `gmt-plus-two` or `584285`; `martin-skidmore` or `584286`; in any
            /// case. Tab-separated: the registry identifier of the Long Count under
            /// it (`maya-longcount`, `maya-longcount-gmt2`, `maya-longcount-584286`);
            /// the constant; the Long Count as `baktun.katun.tun.uinal.kin`, then
            /// its five places as five cells; the tzolkʼin number, 1 to 13, and its
            /// day name; and the haabʼ day, 0 to 19, and its month name, the cycles
            /// placed by the same constant, so that the Calendar Round is the one an
            /// inscription pairs with that Long Count: 0.0.0.0.0 is 4 Ahau 8 Cumku
            /// under every constant. A correlation not named, the empty string
            /// included, is `HC_ERR_UNKNOWN`, and a day before 0.0.0.0.0 or after
            /// 19.19.19.17.19 under it `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns
            /// the length the text needs.
        }
        fn hc_maya_long_count(fixed: i64, correlation: name(correlation_len)) -> line =
            $crate::calendar_values::maya_long_count_line;

        c {
            /// A fixed day in the Akan *Adaduanan*, the 42-day cycle of the six-day
            /// and the seven-day week, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. A day at the ends of the
            /// `int64_t` days, which the cycle's arithmetic cannot place, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A fixed day in the Akan *Adaduanan*, the 42-day cycle of the six-day
            /// and the seven-day week, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// Tab-separated: the complete cycles since the *Fɔdwo* of 23 January
            /// 1978, the cycle's epoch; the day of the cycle, 1 to 42; the *nnanson*
            /// day, 1 for Fo to 6 for Mono, and its name; the weekday, 1 for
            /// Kwasiada (Sunday) to 7 for Memeneda, and its name; the short weekday
            /// name a compound takes (`Kwasi`, `Dwo`); the day's compound name, the
            /// six-day name first (`Kuru-Kwasi`); and the *dabɔne* the day is,
            /// `Fɔdwo`, `Awukudae`, `Fofi` or `Akwasidae`, or empty. The names are
            /// `hc_describe_day`'s calendar `akan`'s, which carries the two
            /// positions as numbers alone. A day at the ends of the `i64` days is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_akan_day(fixed: i64) -> line = $crate::calendar_values::akan_day_line;

        c {
            /// A fixed day's *weton*, the Javanese five-day *pasaran* against the
            /// seven-day week, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line is the WebAssembly module's. A day at the ends of the
            /// `int64_t` days, which the cycle's arithmetic cannot place, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A fixed day's *weton*, the Javanese five-day *pasaran* against the
            /// seven-day week, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the complete 35-day *wetonan* cycles since the epoch,
            /// Rata Die 0; the day of the cycle, 1 to 35; the weekday, 1 for Minggu
            /// (Sunday) to 7 for Setu, and its Javanese name, the *dina*; the
            /// *pasaran* day, 1 for Legi to 5 for Kliwon, and its name; the *neptu*
            /// of the weekday and of the pasaran, and their sum, the day's *neptu*;
            /// and the day's compound name as it is spoken, weekday first (`Jemuwah
            /// Legi`). `hc_describe_day`'s calendar `javanese-pasaran` carries the
            /// positions and the sum as numbers alone. A day at the ends of the
            /// `i64` days is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_weton(fixed: i64) -> line = $crate::calendar_values::weton_line;

        c {
            /// Sri Lanka's Buddhist year of a fixed day, counted from the Vesak Full
            /// Moon Poya Day, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line is the WebAssembly module's. A day outside 2023 to 2027,
            /// the years whose Vesak day is carried, is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Sri Lanka's Buddhist year of a fixed day, counted from the Vesak Full
            /// Moon Poya Day, as one UTF-8 line, returning the byte length written.
            ///
            /// The Common Era year plus 544 from the Vesak Poya and plus 543 before
            /// it, `hc_describe_day`'s calendar `buddhist-lk`. Tab-separated: the
            /// Buddhist year; the Gregorian year the day is in; the fixed day of
            /// that Gregorian year's Vesak Poya, as carried; the fixed
            /// day the Buddhist year began on, that Vesak or the year
            /// before's, empty where it is not carried; and the last fixed day
            /// of the Buddhist year, the eve of the next Vesak, empty likewise. A
            /// day outside 2023 to 2027, the years whose Vesak day an order read
            /// fixes, is `HC_ERR_OUT_OF_RANGE`: a gap, never a computed full moon. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_buddhist_lk_year(fixed: i64) -> line =
            $crate::calendar_values::buddhist_lk_year_line;

        c {
            /// The Tiruvaḷḷuvar year of a fixed day, Tamil Nadu's official count,
            /// written to `out_year`.
            ///
            /// The Gregorian year in which the day's Thai 1 falls, plus 31, on the
            /// Tamil solar calendar `hindu-solar-tamil`. A day outside the
            /// Gregorian years 1700 to 2299 is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The Tiruvaḷḷuvar year of a fixed day, Tamil Nadu's official count, or
            /// an error sentinel.
            ///
            /// The Gregorian year in which the day's Thai 1, the Makara saṅkrānti's
            /// month on the Tamil solar calendar `hindu-solar-tamil`, falls, plus
            /// 31: 2052 from 14 January 2021 to the eve of Thai 1 of 2022, so the
            /// count changes at Thai 1 and not at the Tamil New Year of Chithirai 1,
            /// which `hc_describe_day` writes as the date's `tiruvalluvar-year`
            /// field. A day outside the Gregorian years 1700 to 2299 is
            /// `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_tiruvalluvar_year(fixed: i64) -> value(out_year: i64) =
            $crate::hindu_lines::tiruvalluvar_year;

        c {
            /// The Sun's entries into the nakṣatras within a span, as NUL-terminated
            /// UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `ayanamsa` is as for
            /// `hc_nakshatra_at`; null is `HC_ERROR_NULL_POINTER`. The span is
            /// `[from_unix_seconds, to_unix_seconds)` and fails as
            /// `hc_rahu_ingresses`'s does. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The Sun's entries into the nakṣatras within a span, as UTF-8 lines,
            /// returning the byte length written.
            ///
            /// One line an entry in `[from_unix_seconds, to_unix_seconds)`, in time
            /// order, in the zodiac of the ayanāṃśa: the moment as whole POSIX
            /// seconds of Universal Time, rounded down; the nakṣatra the Sun enters,
            /// 1 for Aśvinī through 27 for Revatī, its identifier and its name as
            /// `hc_nakshatra_at` writes them; and the one it leaves by the same
            /// three. The Sun crosses a nakṣatra in thirteen to fourteen days, the
            /// stay the Malayalam almanacs print as a ñāṭṭuvēla, so a year has
            /// twenty-seven entries. `ayanamsa` is as for `hc_nakshatra_at`;
            /// another is `HC_ERR_UNKNOWN`. A `to` not after the `from` writes
            /// nothing; a span longer than a hundred Julian years, or with an end
            /// outside the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_solar_nakshatra_ingresses(
            from_unix_seconds: i64,
            to_unix_seconds: i64,
            ayanamsa: name(ayanamsa_len),
        ) -> line =
            $crate::panchanga_lines::solar_nakshatra_ingresses_lines;

        c {
            /// A year of a Japanese era as the era's dates write it, 元 for the
            /// first year and the Han numerals of Japanese for every other, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. A year below 1 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A year of a Japanese era as the era's dates write it, 元 for the
            /// first year and the Han numerals of Japanese for every other, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// One cell: 元 for 1, 二 for 2, 三十一 for 31, the numerals of the
            /// `jpan` numbering system `hc_format_number` writes. A year below 1,
            /// or one the numerals do not write, is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_japanese_era_year(year: i64) -> line =
            $crate::i18n_lines::japanese_era_year_line;

        c {
            /// The standard date, time and date-time formats a locale carries for
            /// a calendar, in CLDR's four lengths, as NUL-terminated UTF-8 lines in
            /// a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `calendar` is a registry
            /// identifier; one the registry does not carry is `HC_ERROR_UNKNOWN`,
            /// and null `HC_ERROR_NULL_POINTER`. A tag that does not parse is
            /// `HC_ERROR_MALFORMED`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The standard date, time and date-time formats a locale carries for
            /// a calendar, in CLDR's four lengths, as UTF-8 lines, returning the
            /// byte length written.
            ///
            /// One line a format, eighteen in all, tab-separated: the kind, `date`,
            /// `time`, `date-time` or `available`; the length, `full`, `long`,
            /// `medium` or `short`, or for an available format its skeleton,
            /// `hms`, `Hms`, `Gy`, `d`, `yMMMMd` or `yMMMd`; the pattern in UTS #35's
            /// field letters (`EEEE, d. MMMM y` for `de`'s full date, `HH:mm:ss` its
            /// medium time, `{1}, {0}` its medium date-time, `{1}` the date and `{0}`
            /// the time), empty where the table has none; and the CLDR calendar type
            /// the formats were read under, `gregorian` for the Gregorian family,
            /// `japanese`, `hebrew`, `islamic`, `persian`, `chinese`, `buddhist`,
            /// `roc`, `coptic`, `ethiopic`, `indian`, `dangi`, `iso8601`, and
            /// `generic` for a calendar CLDR gives no formats of its own. The
            /// patterns are CLDR 48's for the locale, resolved through its fallback
            /// chain and `root.xml`; `hc_format_pattern`'s `%c`, `%x` and `%X` write
            /// the medium ones. `calendar` is a registry identifier; one the
            /// registry does not carry is `HC_ERR_UNKNOWN`. A tag that does not
            /// parse is `HC_ERR_MALFORMED`; `locale` is as for `hc_locale_chain`. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_locale_format(locale: text(locale_len), calendar: name(calendar_len)) -> line =
            $crate::i18n_lines::locale_format_lines;

        c {
            /// The plural categories a locale's cardinal or ordinal rules name, each
            /// with a number that falls in it, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `kind` is `cardinal` or
            /// `ordinal`, as for `hc_plural_category`; any other kind is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. A tag that does
            /// not parse is `HC_ERROR_MALFORMED`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The plural categories a locale's cardinal or ordinal rules name, each
            /// with a number that falls in it, as UTF-8 lines, returning the byte
            /// length written.
            ///
            /// One line a category the rules name, in CLDR's order — `zero`, `one`,
            /// `two`, `few`, `many`, `other` — tab-separated: the category; a
            /// number the rules put in it, written as `hc_plural_category` reads
            /// one (`1`, `0.5`, `1000000`), the first of the written numbers 0 to
            /// 1 100 and the first millions, with one and two fraction digits, that
            /// falls there, empty for a category no such number reaches; and the
            /// language of the rules that answered (`ru`, `pt-PT`, or `und` where
            /// none is carried and everything is `other`). Arabic's cardinals answer
            /// all six, Russian's `one`, `few`, `many` and `other`, French's `one`,
            /// `many` and `other`, its `many` a whole million, Japanese's `other`
            /// alone; English's ordinals `one` (1st), `two`, `few` and `other`.
            /// `kind` is `cardinal`, the form after a count, or `ordinal`, the form
            /// of a position, in any case, as for `hc_plural_category`; any other
            /// kind is `HC_ERR_UNKNOWN`. A tag that does not parse is
            /// `HC_ERR_MALFORMED`; `locale` is as for `hc_locale_chain`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_plural_categories(locale: text(locale_len), kind: name(kind_len)) -> line =
            $crate::i18n_lines::plural_categories_lines;
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
            /// Every tradition that names the 72 pentads (候), as NUL-terminated
            /// UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one a tradition in the
            /// table's order: its identifier, its English name, the text its
            /// names come from, and how many of its names carry an alternate
            /// reading the text prints beside them. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Every tradition that names the 72 pentads (候), as UTF-8 lines,
            /// returning the byte length written.
            ///
            /// One line a tradition, in the table's order, tab-separated: its
            /// identifier, `chinese` (the classical set in traditional characters),
            /// `japanese` (the 宝暦暦's, the list the almanac prints today), `jokyo`
            /// (the 貞享暦's, Shibukawa's) or `senmyo` (the 宣明暦's, before 1685);
            /// its English name; the text its names come from; and how many of its
            /// names carry an alternate reading the text prints beside them, such as
            /// 虎(武)始交, the `alternate` of `hc_pentad_in_tradition`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_pentad_traditions() -> line =
            || Ok($crate::season_lines::pentad_traditions_lines());

        c {
            /// The pentad (候) in effect on a fixed day at a meridian, named by a
            /// tradition, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `tradition` is a name `hc_pentad_traditions` lists, in any case; null
            /// is `HC_ERROR_NULL_POINTER` and one not listed `HC_ERROR_UNKNOWN`.
            /// The line is the WebAssembly module's. `meridian` and the day are as
            /// for `hc_term_in_effect`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The pentad (候) in effect on a fixed day at a meridian, named by a
            /// tradition, as one UTF-8 line, returning the byte length written.
            ///
            /// The same pentad as `hc_pentad_in_effect`, which names it in the
            /// Chinese and the Japanese tradition alone. Tab-separated: the pentad's
            /// index from the first pentad of 春分 at 0 through 71; its name in the
            /// tradition; its English gloss; the alternate reading the tradition's
            /// text prints beside the name, empty where it prints none; the fixed
            /// day the pentad began at the meridian and the last fixed day before
            /// the next begins, as for `hc_pentad_in_effect`; the tradition's
            /// identifier; and the text its names come from. `tradition` is a name
            /// `hc_pentad_traditions` lists, `chinese`, `japanese`, `jokyo` or
            /// `senmyo`, in any case; one not listed is `HC_ERR_UNKNOWN`, read
            /// before `meridian`. `meridian` and the day are as for
            /// `hc_term_in_effect`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_pentad_in_tradition(
            fixed: i64,
            tradition: name(tradition_len),
            meridian: text(meridian_len),
        ) -> line =
            $crate::season_lines::pentad_in_tradition_line;

        c {
            /// The 雑節 of a Gregorian year at a meridian, as NUL-terminated UTF-8
            /// lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one a day, the 21 of the
            /// almanac and then the three days an older rule places elsewhere under
            /// ids of their own. `meridian` is as for `hc_term_in_effect`; null or
            /// empty is `universal`, and anything else is `HC_ERROR_UNKNOWN`. A year
            /// outside −1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// The 雑節 of a Gregorian year at a meridian, as UTF-8 lines, returning
            /// the byte length written.
            ///
            /// One line a day, in the order the almanac lists them, tab-separated:
            /// the identifier (`spring-setsubun`, `summer-doyo-entry`); the name in
            /// Japanese, 節分; its Hepburn romaji; a short English description; the
            /// rule that fixes the day, `solar-longitude`, `offset-from-term`,
            /// `nights-from-beginning-of-spring` or `nearest-stem-day`; the fixed
            /// day; the last day of the period it opens, the 土用's last day for a
            /// 土用の入り and the week's last day for a 彼岸入り, else empty; and for a
            /// 土用の入り the first and the second 丑の日 of the period, the second
            /// empty where it has one. The 21 are 節分 ×4, 彼岸 入り, 中日 and 明け ×2,
            /// 社日 ×2, 土用の入り ×4, 八十八夜, 入梅, 半夏生, 二百十日 and 二百二十日;
            /// then `nyubai-classical`, `hangesho-classical`,
            /// `spring-shanichi-classical` and `autumn-shanichi-classical`, the
            /// older rules' days, with the rule `classical` and no period
            /// (`docs/policy.md` §5). `meridian` is as for `hc_term_in_effect`. A
            /// meridian not read is `HC_ERR_UNKNOWN`; a year outside −1000 to 3000 is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_zassetsu_in_year(year: i64, meridian: text(meridian_len)) -> line =
            $crate::season_lines::zassetsu_in_year_lines;

        c {
            /// The other seasonal days and spans of a Gregorian year, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, the kind first. `meridian` is
            /// as for `hc_term_in_effect`; null or empty is `universal`, and
            /// anything else is `HC_ERROR_UNKNOWN`. A year outside −1000 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The other seasonal days and spans of a Gregorian year, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// Seven columns, tab-separated: the kind; the identifier; the name; the
            /// name in the convention's own language; the first fixed day; the last
            /// fixed day, the same for a single day; and the group, the calendar of
            /// the dates or the tradition, else empty. In order: `san-fu`, 初伏, 中伏
            /// and 末伏 (`chu-fu`, `zhong-fu`, `mo-fu`) of the Chinese year at the
            /// meridian, each ending the day before the next begins; `shu-jiu`, the
            /// nine nines 一九 to 九九 counted from the winter solstice of the year,
            /// which run into the next; `dog-days`, the dog days under each of the
            /// three conventions, with the group `gregorian` or `julian`;
            /// `quarter-day`, the 20 quarter and term days of England and Wales, the
            /// English cross-quarters, Ireland, traditional Scotland and the Scottish
            /// Act of 1990, with the tradition as the group; and `folk-day`, the seven
            /// named days of the Turkish folk year. `meridian` is as for
            /// `hc_term_in_effect` and sets only the solar terms the 伏 and the nines
            /// count from. A meridian not read is `HC_ERR_UNKNOWN`; a year outside
            /// −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_seasonal_days_in_year(year: i64, meridian: text(meridian_len)) -> line =
            $crate::season_lines::seasonal_days_lines;

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

        c {
            /// Every pentad (候) that begins in a Gregorian year at a meridian,
            /// named by every tradition at once, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `meridian` is as for
            /// `hc_term_in_effect`, and a year outside −1000 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Every pentad (候) that begins in a Gregorian year at a meridian,
            /// named by every tradition at once, as UTF-8 lines, returning the byte
            /// length written.
            ///
            /// One line a pentad, in date order, 71 to 73 of them (the 72 候 tile
            /// the tropical year, not the Gregorian one, so a year holds one
            /// beginning fewer or one more now and then), tab-separated: the
            /// pentad's index from the first pentad of 春分 at 0 through 71; the
            /// fixed day it began at the meridian; the last fixed day before the
            /// next pentad begins, as `hc_pentad_in_effect` writes them; and its
            /// name in each tradition `hc_pentad_traditions` lists, one column a
            /// tradition in that order, `chinese`, `japanese`, `jokyo` and
            /// `senmyo`. `meridian` is as for `hc_term_in_effect`, and a year
            /// outside −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_pentads_in_year(year: i64, meridian: text(meridian_len)) -> line =
            $crate::season_lines::pentads_in_year_lines;

        c {
            /// The twelve tropical sign periods of a Gregorian year at a meridian,
            /// as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `meridian` is as for
            /// `hc_term_in_effect`, and a year outside −1000 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The twelve tropical sign periods of a Gregorian year at a meridian,
            /// as UTF-8 lines, returning the byte length written.
            ///
            /// One line a sign, in date order from Aquarius around 20 January to
            /// Capricorn around 21 December, the order the signs appear in a
            /// Gregorian year, tab-separated: the sign, 1 for Aries through 12 for
            /// Pisces; its identifier (`aries`) and English name; the instant the
            /// Sun entered it and the instant it left, each as whole POSIX seconds
            /// of Universal Time, rounded down, consecutive signs sharing theirs
            /// exactly; and the first and the last fixed day of the sign at the
            /// meridian, so that a sign's days run `begins..=ends`. The signs turn
            /// at the solar terms of `hc_solar_terms_between`; `hc_decan_at` has the
            /// sign at an instant. `meridian` is as for `hc_term_in_effect`, and a
            /// year outside −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_tropical_signs_in_year(year: i64, meridian: text(meridian_len)) -> line =
            $crate::season_lines::tropical_signs_in_year_lines;

        c {
            /// The twelve sidereal sign periods, the saṅkrāntis, of a Gregorian
            /// year in the zodiac of a named ayanāṃśa at a meridian, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `ayanamsa` is as for
            /// `hc_drekkana_at`; null is `HC_ERROR_NULL_POINTER`. `meridian` is as
            /// for `hc_term_in_effect`, and a year outside −1000 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The twelve sidereal sign periods, the saṅkrāntis, of a Gregorian
            /// year in the zodiac of a named ayanāṃśa at a meridian, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// One line a sign, in date order from the first saṅkrānti on or after
            /// 1 January, Makara's around 14 January with a modern ayanāṃśa,
            /// tab-separated: the sign, 1 for Meṣa through 12 for Mīna; its
            /// identifier (`mesha`) and Sanskrit name; the instant the Sun entered
            /// it and the instant it left, each as whole POSIX seconds of Universal
            /// Time, rounded down; the first and the last fixed day of the sign at
            /// the meridian; and the ayanāṃśa's identifier. `ayanamsa` is as for
            /// `hc_drekkana_at`; anything else, the empty string included, is
            /// `HC_ERR_UNKNOWN`, read before the meridian. `meridian` is as for
            /// `hc_term_in_effect`, and a year outside −1000 to 3000 is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_sidereal_signs_in_year(
            year: i64,
            ayanamsa: name(ayanamsa_len),
            meridian: text(meridian_len),
        ) -> line =
            $crate::season_lines::sidereal_signs_in_year_lines;

        c {
            /// The fixed day of 伝統的七夕, the National Astronomical Observatory's
            /// traditional Tanabata, of a Gregorian year at a meridian.
            ///
            /// `meridian` is as for `hc_term_in_effect`; the Observatory's days are
            /// at `japan`. A year outside −1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// The fixed day of 伝統的七夕, the National Astronomical Observatory's
            /// traditional Tanabata, of a Gregorian year at a meridian, or an error
            /// sentinel.
            ///
            /// The seventh day counted from the day holding the new moon nearest 処暑,
            /// on or before the day holding 処暑, as the Observatory defines it:
            /// 10 August 2024, 29 August 2025, 19 August 2026 at `japan`, where its
            /// table's days are read. `meridian` is as for `hc_term_in_effect`, and
            /// a year outside −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_traditional_tanabata(year: i64, meridian: text(meridian_len)) -> value(out_fixed: i64) =
            $crate::season_lines::traditional_tanabata;

        c {
            /// The principal phases of the Moon that fall inside a Gregorian month
            /// at a meridian, as NUL-terminated UTF-8 lines in a caller-owned
            /// buffer.
            ///
            /// The lines are the WebAssembly module's. `meridian` is as for
            /// `hc_term_in_effect`; a month outside 1 to 12 is
            /// `HC_ERROR_INVALID_DATE` and a year outside −1000 to 3000
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The principal phases of the Moon that fall inside a Gregorian month
            /// at a meridian, as UTF-8 lines, returning the byte length written.
            ///
            /// One line a phase, in time order, four or five of them, since a month
            /// of 30 or 31 days can hold one phase twice, tab-separated: the phase,
            /// `new`, `first-quarter`, `full` or `last-quarter`, as
            /// `hc_moon_phases_between` names them; its instant as whole POSIX
            /// seconds of Universal Time, rounded down; and the fixed day it falls
            /// on at the meridian, which is what decides the month. `meridian` is as
            /// for `hc_term_in_effect`; a month outside 1 to 12 is
            /// `HC_ERR_INVALID_DATE` and a year outside −1000 to 3000
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_principal_phases_in_month(
            year: i64,
            month: u32,
            meridian: text(meridian_len),
        ) -> line =
            $crate::season_lines::principal_phases_in_month_lines;
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
            /// A day with an entry that stops work is 1; a day without one is 0
            /// unless the answer is open, and an open answer is refused, not
            /// guessed: a holiday of the table that could not be placed in the
            /// day's year (a gap of `hc_holidays_in_year`: a calendar's range
            /// ended, no announcement read, a year before the table's sources
            /// begin, or a subdivision whose days were not read) may be this day,
            /// `HC_ERROR_NO_DATA`, and a day on which the region's weekend law was
            /// not read (the `unread-weekend` gap), `HC_ERROR_OUT_OF_RANGE`.
            /// A null `code` or `out_is_day_off` is `HC_ERROR_NULL_POINTER`, a
            /// string that is not UTF-8 `HC_ERROR_NOT_UTF8`, and a code that names
            /// no table, or a `group` that names no group of `hc_holiday_groups`,
            /// or a `region` that is no subdivision of the table's country,
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
            /// `hc_holiday_groups`, or a `region` that is no subdivision of the
            /// table's country, `HC_ERR_UNKNOWN`. A day with an entry that stops
            /// work is 1; a day without one is 0 unless the answer is open, and
            /// an open answer is refused, not guessed: a holiday of the table
            /// that could not be placed in the day's year (a gap of
            /// `hc_holidays_in_year`: a calendar's range ended, no announcement
            /// read, a year before the table's sources begin, or a subdivision
            /// whose days were not read) may be this day, `HC_ERR_NO_DATA`, and a
            /// day on which the region's weekend law was not read (the
            /// `unread-weekend` gap), `HC_ERR_OUT_OF_RANGE`.
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
            /// `HC_ERROR_NULL_POINTER`. A day with no Gregorian year, a `count`
            /// past 36 500 either way, or a walk that reaches a day whose weekend
            /// law in the region was not read, is `HC_ERROR_OUT_OF_RANGE`; a walk
            /// that reaches a day a gap leaves open, which would be counted or
            /// skipped on a guess, as for `hc_holiday_is_day_off`, is
            /// `HC_ERROR_NO_DATA`.
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
            /// Gregorian year, a `count` past 36 500 either way, a hundred years'
            /// worth, or a walk that reaches a day whose weekend law in the region
            /// was not read is `HC_ERR_OUT_OF_RANGE`; a walk that reaches a day a
            /// gap leaves open, which would be counted or skipped on a guess, as
            /// for `hc_holiday_is_day_off`, is `HC_ERR_NO_DATA`.
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
            /// before `from_fixed`. Two days more than a hundred years apart, or an
            /// interval with a day whose weekend law was not read, are
            /// `HC_ERROR_OUT_OF_RANGE`, and an interval with a day a gap leaves
            /// open `HC_ERROR_NO_DATA`.
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
            /// than a hundred years apart, one with no Gregorian year, or an
            /// interval with a day whose weekend law was not read, are
            /// `HC_ERR_OUT_OF_RANGE`, and an interval with a day a gap leaves open
            /// `HC_ERR_NO_DATA`.
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
            /// Whether a fixed day is a weekend day in a holiday table, in a
            /// subdivision or nationwide.
            ///
            /// `code` and `region` are as for `hc_holiday_is_day_off`; `region`
            /// may be null. The weekend is the law in force on the day in the
            /// region (column 14 of `hc_holiday_tables` lists the laws): Kedah's
            /// Friday and Saturday from 25 November 2013, the rest of Malaysia's
            /// Saturday and Sunday, and a table that states none keeps Saturday and
            /// Sunday. Writes 1 or 0 to `out_is_weekend`. A day on which the
            /// region's weekend law was not read (the `unread-weekend` gap of
            /// `hc_holidays_in_year`) is `HC_ERROR_OUT_OF_RANGE`, not a weekend of
            /// no days: every country's and exchange's table begins with such
            /// days, Japan's weekend being read from 1 May 1992
            /// (`hc_holiday_coverage` says from when). A day with no Gregorian
            /// year is `HC_ERROR_OUT_OF_RANGE` too. A null `code` or
            /// `out_is_weekend` is `HC_ERROR_NULL_POINTER`, a string that is not
            /// UTF-8 `HC_ERROR_NOT_UTF8`, and a code that names no table or a
            /// `region` that is no subdivision of its country `HC_ERROR_UNKNOWN`.
        }
        wasm {
            /// Whether a fixed day is a weekend day in a holiday table, in a
            /// subdivision or nationwide: 1, 0, or an error sentinel.
            ///
            /// `code` and `region` are as for `hc_holiday_is_day_off`; `region`
            /// may be empty. The weekend is the law in force on the day in the
            /// region (column 14 of `hc_holiday_tables` lists the laws): Kedah's
            /// Friday and Saturday from 25 November 2013, the rest of Malaysia's
            /// Saturday and Sunday, and a table that states none keeps Saturday and
            /// Sunday. A day on which the region's weekend law was not read (the
            /// `unread-weekend` gap of `hc_holidays_in_year`) is
            /// `HC_ERR_OUT_OF_RANGE`, not a weekend of no days: every country's
            /// and exchange's table begins with such days, Japan's weekend being
            /// read from 1 May 1992 (`hc_holiday_coverage` says from when). A day
            /// with no Gregorian year is `HC_ERR_OUT_OF_RANGE` too. A null pointer with a non-zero length is
            /// `HC_ERR_NULL_POINTER`, text that is not UTF-8 `HC_ERR_NOT_UTF8`, and
            /// a code that names no table or a `region` that is no subdivision of
            /// its country `HC_ERR_UNKNOWN`.
        }
        fn hc_holiday_is_weekend(
            code: name(code_len),
            region: opt(region_len),
            fixed: i64,
        ) -> value(out_is_weekend: int) =
            $crate::holiday_lines::is_weekend;

        c {
            /// The first holiday of a table after a fixed day, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `code`, `region` and `group` are as for `hc_holiday_is_day_off` and
            /// `kind` as for `hc_holidays_in_year`; a null or empty `kind` keeps
            /// the kinds that stop work, `public` and `bank`. The line is one of
            /// `hc_holidays_in_year`'s, with the same fourteen columns, for the
            /// first entry strictly after the day: a substitute day and a bridged
            /// one count, a gap does not. The search reaches sixteen years. A gap
            /// that could hide a nearer entry (a holiday of a wanted kind the
            /// table could not place in any year from the day's to the found
            /// entry's) is `HC_ERROR_NO_DATA`, and so is a table with no such
            /// entry within the reach. A day with no Gregorian year is
            /// `HC_ERROR_OUT_OF_RANGE`; the string arguments fail as for
            /// `hc_holidays_in_year`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The first holiday of a table after a fixed day, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// `code`, `region` and `group` are as for `hc_holiday_is_day_off` and
            /// `kind` as for `hc_holidays_in_year`; an empty `kind` keeps the kinds
            /// that stop work, `public` and `bank`. The line is one of
            /// `hc_holidays_in_year`'s, with the same fourteen columns, for the
            /// first entry strictly after the day: a substitute day and a bridged
            /// one count, a gap does not. The search reaches sixteen years. A gap
            /// that could hide a nearer entry (a holiday of a wanted kind the
            /// table could not place in any year from the day's to the found
            /// entry's) is `HC_ERR_NO_DATA`, and so is a table with no such entry
            /// within the reach. A day with no Gregorian year is
            /// `HC_ERR_OUT_OF_RANGE`; the string arguments fail as for
            /// `hc_holidays_in_year`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_holiday_next(
            code: name(code_len),
            region: opt(region_len),
            group: opt(group_len),
            kind: opt(kind_len),
            fixed: i64,
        ) -> line =
            $crate::holiday_lines::next_holiday_line;

        c {
            /// The last holiday of a table before a fixed day, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// As `hc_holiday_next`, for the last entry strictly before the day,
            /// searching back sixteen years; a gap that could hide a nearer entry
            /// is `HC_ERROR_NO_DATA`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The last holiday of a table before a fixed day, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// As `hc_holiday_next`, for the last entry strictly before the day,
            /// searching back sixteen years; a gap that could hide a nearer entry
            /// is `HC_ERR_NO_DATA`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_holiday_previous(
            code: name(code_len),
            region: opt(region_len),
            group: opt(group_len),
            kind: opt(kind_len),
            fixed: i64,
        ) -> line =
            $crate::holiday_lines::previous_holiday_line;

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
            /// nothing; then the holiday's identifier within its table,
            /// `new-years-day`, which `hc_holidays_on` and `hc_common_worship_on`
            /// write for the same entry, the instrument its rule cites, or
            /// nothing, and `1` for an entry a bridge policy made, Japan's 国民の休日
            /// between two holidays, and `0` otherwise; then, on a gap, the first
            /// and the last day its holiday could fall on, the only days it
            /// refuses, ISO 8601, or nothing for a gap with nothing to narrow it
            /// and on an entry. `region` and `group` match
            /// in either case, and either may be null. `kind` keeps the entries of the kinds it lists,
            /// `;`-separated, `public;bank`, and the gaps of the rules of those
            /// kinds; a null or empty `kind` is every kind, and a word that names
            /// no kind is `HC_ERROR_UNKNOWN`. Writes the required length,
            /// including the terminator, into `written`. The string arguments
            /// fail as for `hc_holiday_is_day_off`.
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
            /// nothing; then the holiday's identifier within its table,
            /// `new-years-day`, which `hc_holidays_on` and `hc_common_worship_on`
            /// write for the same entry, the instrument its rule cites, or
            /// nothing, and `1` for an entry a bridge policy made, Japan's 国民の休日
            /// between two holidays, and `0` otherwise; then, on a gap, the first
            /// and the last day its holiday could fall on, the only days it
            /// refuses, ISO 8601, or nothing for a gap with nothing to narrow it
            /// and on an entry. `region` and `group` match
            /// in either case, and either may be empty. `kind` keeps the entries of the kinds it lists,
            /// `;`-separated, `public;bank`, and the gaps of the rules of those
            /// kinds; an empty `kind` is every kind, and a word that names no
            /// kind is `HC_ERR_UNKNOWN`. After the entries, one line per gap of
            /// the year, as `hc_holidays_on` writes them: an empty date, the name,
            /// the local name, the kind `gap`, an empty confidence, `0`, nothing,
            /// the subdivision and the group whose own gap it is, the identifier
            /// of the holiday and the instrument its rule cites, `0`, and the
            /// first and the last day the holiday could fall on; a subdivision
            /// not read is a gap of the identifier `unread-subdivision`, and a
            /// year whose weekend law in the region was not read one of
            /// `unread-weekend`, whatever the `kind`. A null `buffer` returns the length the text needs, so
            /// a caller can allocate exactly. The string arguments fail as for
            /// `hc_holiday_is_day_off`.
        }
        fn hc_holidays_in_year(
            code: name(code_len),
            region: opt(region_len),
            group: opt(group_len),
            kind: opt(kind_len),
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
            /// evaluated nationwide, then in each subdivision its rules or its
            /// weekend law name, in code order, then for each group of people its rules give days to
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
            /// for an entry the calendar for everyone does not have, or nothing,
            /// the holiday's identifier within its table,
            /// `new-years-day`, which `hc_holidays_in_year` and
            /// `hc_common_worship_on` write for the same entry, and last `1` for
            /// an entry a bridge policy made and `0` otherwise. A
            /// `gap` line is a holiday the table could
            /// not place that could fall on the day — its calendar's range ended,
            /// or no announcement was read — with the confidence and source
            /// empty, so a
            /// caller can say the day is unanswered rather than show nothing. A
            /// day with no Gregorian year is `HC_ERROR_OUT_OF_RANGE`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// Every holiday on one fixed day across every table, as UTF-8 lines,
            /// returning the byte length written.
            ///
            /// The tables are the ones `hc_holiday_codes` lists, in that order, each
            /// evaluated nationwide, then in each subdivision its rules or its
            /// weekend law name, in code order, then for each group of people its rules give days to
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
            /// for an entry the calendar for everyone does not have, or nothing,
            /// the holiday's identifier within its table,
            /// `new-years-day`, which `hc_holidays_in_year` and
            /// `hc_common_worship_on` write for the same entry, and last `1` for
            /// an entry a bridge policy made and `0` otherwise. A
            /// `gap` line is a holiday the table could
            /// not place that could fall on the day — its calendar's range ended,
            /// or no announcement was read — with the confidence and source
            /// empty, so a
            /// page can say the day is unanswered rather than show nothing. A day
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
            /// codes of the regions it answers for, `;`-separated in code order, or
            /// nothing (the subdivisions its rules, its weekend laws and its
            /// substitution policies are scoped to, each a `region` every export
            /// accepts), the identifiers of the groups of people its
            /// rules give days to alone, `;`-separated in identifier order, or
            /// nothing, and those groups' names in the locale, in the same order,
            /// each its English name where `hc-i18n` carries none, the
            /// subdivision and group pairs a rule is scoped to both of,
            /// `region:group`, `;`-separated, or nothing, and the subdivisions the
            /// table's sources were read for, `;`-separated in code order, or
            /// nothing, and the table's weekend laws (column 14; see below). A
            /// country is
            /// named by its CLDR 48 territory name
            /// in the `locale` where `hc-i18n` carries one, and else, as for a null
            /// `locale`, by CLDR's English one; every other table by its English
            /// name; the tag that answered is in column 5. Column 8 is CLDR 48's
            /// `alt="short"` name of a country, from the data that named it —
            /// `Hong Kong` for `HK` under `en`, 香港 under `ja` — and empty where
            /// the data has none and for every table that is not a country. Column
            /// 14 lists the table's weekend laws, `;`-separated, each four fields
            /// separated by `/`: the weekend days as ISO 8601 weekday numbers
            /// joined by `+` (`5+6` for Friday and Saturday), or `unread` for years
            /// whose law was not read; the first day in force and the last, as
            /// `YYYY-MM-DD`, each empty for none; and the ISO 3166-2 codes of the
            /// regions it is the weekend of, joined by `,`, empty for the whole
            /// table. A table that lists none keeps Saturday and Sunday. Column 15
            /// lists the table's weekend-substitution laws the same way, each
            /// seven fields separated by `/`: the weekdays that trigger a
            /// substitute, joined by `+`; the direction, `forward`, `backward`,
            /// `nearest` or `nearest-working-day`; the flags `skip-occupied` and
            /// `on-collision`, joined by `+`, empty for neither; the weekdays a
            /// substitute avoids besides the trigger; the first year in force and
            /// the last, each empty for none; and the regions it is the law of,
            /// joined by `,`. A table that lists none leaves its holidays on the
            /// weekend. Column 16 lists the tables whose days off the table keeps
            /// as its own (an exchange's country), `;`-separated, each three
            /// fields separated by `/`: the included table's code, the
            /// subdivision of it the days are taken in, empty for its nationwide
            /// days, and the first year the inclusion is read for, empty for
            /// none. Writes the required
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
            /// the ISO 3166-2 codes of the regions it answers for,
            /// `;`-separated in code order, else empty (the subdivisions its
            /// rules, its weekend laws and its substitution policies are scoped
            /// to, each a `region` every export accepts); the identifiers of the
            /// groups of people its rules give days to alone, `;`-separated in
            /// identifier order, else empty; those groups' names in the locale,
            /// in the same order, each its English name where `hc-i18n` carries
            /// none; the subdivision and group pairs a rule is scoped to both of,
            /// `region:group`, `;`-separated in code and then identifier order, else
            /// empty; the subdivisions the table's sources were read for,
            /// `;`-separated in code order, else empty; the table's weekend
            /// laws (column 14; see below); and its weekend-substitution laws
            /// (column 15). A country is named by its CLDR 48
            /// territory
            /// name in the locale where `hc-i18n` carries one, a country the locale
            /// has no name for by CLDR's English one, and every other table by its
            /// English name; column 5 is the tag that answered. Column 8 is CLDR
            /// 48's `alt="short"` name of a country, from the data that named it —
            /// `Hong Kong` for `HK` under `en`, 香港 under `ja` — and empty where
            /// the data has none and for every table that is not a country. Column
            /// 14 lists the table's weekend laws, `;`-separated, each four fields
            /// separated by `/`: the weekend days as ISO 8601 weekday numbers
            /// joined by `+` (`5+6` for Friday and Saturday), or `unread` for years
            /// whose law was not read; the first day in force and the last, as
            /// `YYYY-MM-DD`, each empty for none; and the ISO 3166-2 codes of the
            /// regions it is the weekend of, joined by `,`, empty for the whole
            /// table. A table that lists none keeps Saturday and Sunday. Column 15
            /// lists the table's weekend-substitution laws the same way, each
            /// seven fields separated by `/`: the weekdays that trigger a
            /// substitute, joined by `+`; the direction, `forward`, `backward`,
            /// `nearest` or `nearest-working-day`; the flags `skip-occupied` and
            /// `on-collision`, joined by `+`, empty for neither; the weekdays a
            /// substitute avoids besides the trigger; the first year in force and
            /// the last, each empty for none; and the regions it is the law of,
            /// joined by `,`. A table that lists none leaves its holidays on the
            /// weekend. Column 16 lists the tables whose days off the table keeps
            /// as its own (an exchange's country), `;`-separated, each three
            /// fields separated by `/`: the included table's code, the
            /// subdivision of it the days are taken in, empty for its nationwide
            /// days, and the first year the inclusion is read for, empty for
            /// none; before that year the days are a gap. The locale argument
            /// fails as `hc_parse_iso_date` does. A null `buffer` returns the
            /// length the text needs.
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
            /// The years a holiday table answers for, nationwide and in each
            /// subdivision it answers for, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `code` is as for
            /// `hc_holiday_is_day_off`; a null `code` is `HC_ERROR_NULL_POINTER`,
            /// a string that is not UTF-8 `HC_ERROR_NOT_UTF8`, and a code that
            /// names no table `HC_ERROR_UNKNOWN`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The years a holiday table answers for, nationwide and in each
            /// subdivision it answers for, as UTF-8 lines, returning the byte
            /// length written.
            ///
            /// One line per scope, the whole country first and then each
            /// subdivision the table answers for or reads from a year, in code
            /// order, tab-separated: the subdivision's ISO 3166-2 code, empty for
            /// the whole country; the first year any rule of the scope is read
            /// for, empty where some rule has no first year; the first year from
            /// which no rule of the scope is a gap for want of reading, empty where
            /// no rule declares one; the last year no announced list in the scope
            /// has run out in, empty where none runs out; `1` where every rule of
            /// the scope is read in some year and `0` where one is read in none;
            /// the first year the scope's weekend law is read, empty where the
            /// table's weekend has no first year; and the rule, or the
            /// subdivision, that sets the `answered from` year, empty where
            /// nothing does. A year before the first is a gap, which
            /// `hc_holidays_in_year` writes (the `unread-weekend` gap for the
            /// weekend law). The ranges of the calendars the rules count in are
            /// not repeated here. The string argument fails as for
            /// `hc_holiday_is_day_off`. A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_holiday_coverage(code: name(code_len)) -> line =
            $crate::holiday_lines::holiday_coverage_lines;

        c {
            /// The rules of a holiday table, one line each, as NUL-terminated
            /// UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. `code` is as for
            /// `hc_holiday_is_day_off`; a null `code` is `HC_ERROR_NULL_POINTER`,
            /// a string that is not UTF-8 `HC_ERROR_NOT_UTF8`, and a code that
            /// names no table `HC_ERROR_UNKNOWN`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The rules of a holiday table, one line each, as UTF-8 lines,
            /// returning the byte length written.
            ///
            /// One line per rule, in the table's order, tab-separated: the
            /// rule's identifier within the table; the English name; the name in
            /// the local language, empty where the English one is the local one;
            /// the kind (`public`, `bank`, `religious`, `observance`, `school`,
            /// `workday`, `government` or `half-day`); the confidence (`exact` or
            /// `approximate`); the first Gregorian year the day existed in and
            /// the last, each empty for none; the first year the sources read
            /// answer for, empty for none; the subdivisions it applies to,
            /// `;`-separated, empty for the whole country; the groups of people
            /// it is given to alone, `;`-separated, empty for everyone; whether
            /// the table's substitution reaches it, `none` where it never does,
            /// else three fields separated by `/`: the weekdays that trigger a
            /// substitute for this rule alone as ISO 8601 numbers joined by `+`,
            /// the direction it moves for this rule alone, each empty where the
            /// table's policy (column 15 of `hc_holiday_tables`) is the rule's
            /// own, and the first year the policy reaches it, empty where it
            /// does from the policy's own first year; and the instrument the rule
            /// cites, empty where the table's sources speak for it. The string
            /// argument fails as for `hc_holiday_is_day_off`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_holiday_rules(code: name(code_len)) -> line =
            $crate::holiday_lines::holiday_rules_lines;

        c {
            /// `hc_holidays_on`'s lines, each with the day's name in a locale and
            /// the tag that named it, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's: the eleven columns of
            /// `hc_holidays_on`, the name, the tag, the holiday's identifier and
            /// the bridge flag.
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
            /// The eleven columns of `hc_holidays_on`, then four more: what the
            /// locale calls that day of that table, where a source in the language
            /// names it by the holiday's identifier — the Bohairic Coptic names of
            /// the Coptic Orthodox feasts under `cop` — else empty; the tag that
            /// named it; the holiday's identifier, the column `hc_holidays_on`
            /// puts before its last, moved after the two so that they keep their
            /// places; and the bridge flag, `hc_holidays_on`'s last. The locale
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
            /// table, the rank's identifier, the rank's English name and the
            /// celebration's identifier, which `hc_holidays_on` writes in its last
            /// column for the same entry. A day that
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
            /// the rank, `principal-feast`, `principal-holy-day` or `festival`; the
            /// rank's English name; and the celebration's identifier,
            /// `christmas-day`, which is the last column of the entry in
            /// `hc_holidays_on` and is what joins the two. A day that keeps none
            /// writes nothing. The
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
            /// Every interval of one rank of the geologic time scale, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// `rank` is 0 for the eons, 1 for the eras, 2 for the periods, 3 for
            /// the epochs and 4 for the ages; anything else is `HC_ERROR_UNKNOWN`.
            /// The intervals come youngest first, each a line of the columns
            /// `hc_place_years_ago` writes, in `megayears-before-present` with the
            /// chart's own figures and uncertainties, the chart as the source and
            /// the chart's name in `locale` last. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Every interval of one rank of the geologic time scale, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// `rank` is 0 for the eons, 1 for the eras, 2 for the periods, 3 for
            /// the epochs and 4 for the ages; anything else is `HC_ERR_UNKNOWN`.
            /// The intervals come youngest first, each a line of the columns
            /// `hc_place_years_ago` writes, in `megayears-before-present` with the
            /// chart's own figures and uncertainties, the chart as the source and
            /// the chart's name in `locale` last. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_geologic_intervals(rank: u32, locale: text(locale_len)) -> line =
            $crate::deep_time_lines::intervals_line;

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

        c {
            /// The CODATA constants the Planck units are built from, and the Planck units, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// One line each for the speed of light, the reduced Planck constant, the Newtonian
            /// constant of gravitation and the Planck time, length, mass, energy and temperature,
            /// from the 2022 CODATA adjustment. `c` and ℏ are defined exactly and have a standard
            /// uncertainty of 0; `G` is the one measured constant, and every Planck unit inherits
            /// its 2.2·10⁻⁵, halved or thirded by the root.
            ///
            /// One line each, the cells tab-separated: symbol, name, unit, value, std dev, figures,
            /// relative uncertainty, defined, text, source.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// The CODATA constants the Planck units are built from, and the Planck units, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// One line each for the speed of light, the reduced Planck constant, the Newtonian
            /// constant of gravitation and the Planck time, length, mass, energy and temperature,
            /// from the 2022 CODATA adjustment. `c` and ℏ are defined exactly and have a standard
            /// uncertainty of 0; `G` is the one measured constant, and every Planck unit inherits
            /// its 2.2·10⁻⁵, halved or thirded by the root.
            ///
            /// One line each, the cells tab-separated: symbol, name, unit, value, std dev, figures,
            /// relative uncertainty, defined, text, source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_planck_units() -> line =
            || Ok($crate::deep_time_lines::planck_units_lines());

        c {
            /// A calendar age or year in one datum written in another, as NUL-terminated UTF-8 one
            /// line in a caller-owned buffer.
            ///
            /// The datums are `bp` (calendar years before 1950 CE), `b2k` (before 2000 CE, the
            /// ice-core scale) and `ce` (a calendar year in astronomical numbering, where year 0 is
            /// 1 BCE and −9700 is 9701 BCE). Moving between them adds or subtracts a whole number
            /// of years, so the standard deviation is unchanged. The Holocene's base is 11 700 b2k
            /// and 11 650 BP. A conventional radiocarbon age, `radiocarbon-bp`, is not a count of
            /// calendar years and needs a calibration curve (IntCal20 and its companions) that the
            /// crate does not carry: it is `HC_ERROR_NO_DATA`, on either side. A datum that is not
            /// one of these is `HC_ERROR_UNKNOWN`, and a number or standard deviation that is not
            /// finite, or a negative standard deviation, is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `from` is the datum of `years`: `bp`, `b2k`, `ce` or `radiocarbon-bp`. `to` is the
            /// datum to write it in.
            ///
            /// Tab-separated: from, to, years, std dev, converted, converted std dev, label,
            /// source.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A calendar age or year in one datum written in another, as one UTF-8 line, returning
            /// the byte length written.
            ///
            /// The datums are `bp` (calendar years before 1950 CE), `b2k` (before 2000 CE, the
            /// ice-core scale) and `ce` (a calendar year in astronomical numbering, where year 0 is
            /// 1 BCE and −9700 is 9701 BCE). Moving between them adds or subtracts a whole number
            /// of years, so the standard deviation is unchanged. The Holocene's base is 11 700 b2k
            /// and 11 650 BP. A conventional radiocarbon age, `radiocarbon-bp`, is not a count of
            /// calendar years and needs a calibration curve (IntCal20 and its companions) that the
            /// crate does not carry: it is `HC_ERR_NO_DATA`, on either side. A datum that is not
            /// one of these is `HC_ERR_UNKNOWN`, and a number or standard deviation that is not
            /// finite, or a negative standard deviation, is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `from` is the datum of `years`: `bp`, `b2k`, `ce` or `radiocarbon-bp`. `to` is the
            /// datum to write it in.
            ///
            /// Tab-separated: from, to, years, std dev, converted, converted std dev, label,
            /// source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_bp_convert(
            years: f64,
            std_dev_years: f64,
            from: name(from_len),
            to: name(to_len),
        ) -> line =
            $crate::deep_time_lines::bp_convert_line;

        c {
            /// A magnitude of time in one unit written in another, with its uncertainty carried
            /// through, as NUL-terminated UTF-8 one line in a caller-owned buffer.
            ///
            /// The units are `planck-time`, `yoctosecond`, `zeptosecond`, `attosecond`,
            /// `femtosecond`, `picosecond`, `nanosecond`, `microsecond`, `millisecond`, `second`,
            /// `minute`, `hour`, `day`, `julian-year`, `kiloyear`, `megayear` and `gigayear`, in
            /// any ASCII case. Every one but the Planck time is a defined multiple of the second
            /// and rescales the standard deviation exactly; the Planck time is CODATA's measurement
            /// and brings its 1.1·10⁻⁵ into the answer, so a round trip through it returns the same
            /// number with a wider bar. A unit that is not one of these is `HC_ERROR_UNKNOWN`; a
            /// number or standard deviation that is not finite, a negative standard deviation, or a
            /// result that leaves the range of a double is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `from` is the unit of `value`. `to` is the unit to write it in.
            ///
            /// Tab-separated: from, to, value, std dev, converted, converted std dev, text,
            /// seconds, seconds std dev, log10 seconds, log10 std dev, exact, source.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A magnitude of time in one unit written in another, with its uncertainty carried
            /// through, as one UTF-8 line, returning the byte length written.
            ///
            /// The units are `planck-time`, `yoctosecond`, `zeptosecond`, `attosecond`,
            /// `femtosecond`, `picosecond`, `nanosecond`, `microsecond`, `millisecond`, `second`,
            /// `minute`, `hour`, `day`, `julian-year`, `kiloyear`, `megayear` and `gigayear`, in
            /// any ASCII case. Every one but the Planck time is a defined multiple of the second
            /// and rescales the standard deviation exactly; the Planck time is CODATA's measurement
            /// and brings its 1.1·10⁻⁵ into the answer, so a round trip through it returns the same
            /// number with a wider bar. A unit that is not one of these is `HC_ERR_UNKNOWN`; a
            /// number or standard deviation that is not finite, a negative standard deviation, or a
            /// result that leaves the range of a double is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `from` is the unit of `value`. `to` is the unit to write it in.
            ///
            /// Tab-separated: from, to, value, std dev, converted, converted std dev, text,
            /// seconds, seconds std dev, log10 seconds, log10 std dev, exact, source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_deep_convert(
            value: f64,
            std_dev: f64,
            from: name(from_len),
            to: name(to_len),
        ) -> line =
            $crate::deep_time_lines::deep_convert_line;

        c {
            /// Two magnitudes of time compared across the decades between them, as NUL-terminated
            /// UTF-8 one line in a caller-owned buffer.
            ///
            /// The units are those of `hc_deep_convert`. The two are treated as independent, so
            /// comparing a magnitude with itself reports a non-zero deviation around a ratio of 1.
            /// A unit that is not one of those is `HC_ERROR_UNKNOWN`; a number or standard
            /// deviation that is not finite, a negative standard deviation, a span of no length or
            /// a negative one (there is no logarithm of it), or a ratio that leaves the range of a
            /// double is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// Tab-separated: first seconds, first std dev, second seconds, second std dev, ratio,
            /// ratio std dev, log10 ratio, log10 std dev, decades, overlap, order, source.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Two magnitudes of time compared across the decades between them, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// The units are those of `hc_deep_convert`. The two are treated as independent, so
            /// comparing a magnitude with itself reports a non-zero deviation around a ratio of 1.
            /// A unit that is not one of those is `HC_ERR_UNKNOWN`; a number or standard deviation
            /// that is not finite, a negative standard deviation, a span of no length or a negative
            /// one (there is no logarithm of it), or a ratio that leaves the range of a double is
            /// `HC_ERR_OUT_OF_RANGE`.
            ///
            /// Tab-separated: first seconds, first std dev, second seconds, second std dev, ratio,
            /// ratio std dev, log10 ratio, log10 std dev, decades, overlap, order, source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_deep_compare(
            first_value: f64,
            first_std_dev: f64,
            first_unit: name(first_unit_len),
            second_value: f64,
            second_std_dev: f64,
            second_unit: name(second_unit_len),
        ) -> line =
            $crate::deep_time_lines::deep_compare_line;
    } };
    ("tz", $backend:ident) => { $backend! {
        c {
            /// The fixed day a POSIX timestamp falls on by the wall clock of a zone.
            ///
            /// `zone` is a NUL-terminated IANA name, `Asia/Tokyo`, in any case: one
            /// a caller has loaded through `hc_zone_load`, or else one of the
            /// eighteen the library carries with their current rules. A null
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
            /// loaded through `hc_zone_load`, or else one of the eighteen the
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

        c {
            /// Python's `time.localtime(seconds)` in a zone: the wall-clock reading
            /// of a POSIX second as the nine fields of a `struct_time`, one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is `hc_gmtime`'s, with `tm_isdst` 1 where the zone's rules
            /// call the time daylight saving and 0 where not. `zone` is a
            /// NUL-terminated IANA name read as for `hc_zone_offset`; null is
            /// `HC_ERROR_NULL_POINTER`, a name neither loaded nor built in
            /// `HC_ERROR_UNKNOWN`, and an instant outside the years the rules answer
            /// for `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// Python's `time.localtime(seconds)` in a zone: the wall-clock reading
            /// of a POSIX second as the nine fields of a `struct_time`, one UTF-8
            /// line, returning the byte length written.
            ///
            /// The line is `hc_gmtime`'s, with `tm_isdst` 1 where the zone's rules
            /// call the time daylight saving and 0 where not. `zone` is an IANA name
            /// read as for `hc_zone_offset`; a name neither loaded nor built in is
            /// `HC_ERR_UNKNOWN`, and an instant outside the years −9 999 994 to
            /// 9 999 994 by UTC `HC_ERR_OUT_OF_RANGE`. The name's pointer and bytes
            /// fail as for `hc_parse_iso_date`. A null `buffer` returns the length
            /// the text needs.
        }
        fn hc_localtime(unix_seconds: i64, zone: name(zone_len)) -> line =
            $crate::python_lines::localtime_line;

        c {
            /// Python's `time.mktime(tuple)` in a zone: the POSIX second of a
            /// wall-clock reading given as its year, month, day, hour, minute and
            /// second, written to `out_unix_seconds`.
            ///
            /// Each field is checked as `datetime(*tuple[:6])` checks them; one out
            /// of range is `HC_ERROR_INVALID_DATE`, and so is a second of 60, on every
            /// day and in every zone, as `datetime(2016, 12, 31, 23, 59, 60)` raises: a
            /// wall-clock reading carries no leap second. `zone` is as for `hc_localtime`.
            /// `policy` is `earliest`, `latest`, `reject` or `push-forward`, in any
            /// case, where Python reads `tm_isdst`: a reading two instants name,
            /// when the clocks go back, is the one it chooses, and one no instant
            /// names, when they go forward, is what it makes of it; another word
            /// is `HC_ERROR_UNKNOWN`, and under `reject` either reading
            /// `HC_ERROR_INVALID_DATE`. A reading outside the years the rules
            /// answer for is `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// Python's `time.mktime(tuple)` in a zone: the POSIX second of a
            /// wall-clock reading given as its year, month, day, hour, minute and
            /// second, or an error sentinel.
            ///
            /// Each field is checked as `datetime(*tuple[:6])` checks them; one out
            /// of range is `HC_ERR_INVALID_DATE`, and so is a second of 60, on every
            /// day and in every zone, as `datetime(2016, 12, 31, 23, 59, 60)` raises: a
            /// wall-clock reading carries no leap second. `zone` is as for `hc_localtime`.
            /// `policy` is `earliest`, `latest`, `reject` or `push-forward`, in any
            /// case, where Python reads `tm_isdst`: a reading two instants name,
            /// on the morning the clocks go back, is the one it chooses, and one
            /// no instant names, when they go forward, is what it makes of it —
            /// `push-forward` moves it on by the gap, as `java.time` and Temporal's
            /// `compatible` do; another word is `HC_ERR_UNKNOWN`, and under
            /// `reject` either reading `HC_ERR_INVALID_DATE`. A reading outside
            /// the years −9 999 994 to 9 999 994 is `HC_ERR_OUT_OF_RANGE`. The
            /// names' pointers and bytes fail as for `hc_parse_iso_date`.
        }
        fn hc_mktime(
            year: i64,
            month: i64,
            day: i64,
            hour: i64,
            minute: i64,
            second: i64,
            zone: name(zone_len),
            policy: name(policy_len),
        ) -> value(out_unix_seconds: i64) =
            |year, month, day, hour, minute, second, zone, policy| {
                $crate::python_lines::mktime([year, month, day, hour, minute, second], zone, policy)
            };

        c {
            /// What a wall-clock reading means in a zone before a policy reduces it to
            /// one instant, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. The fields are checked as for
            /// `hc_mktime`; a field out of range, a second of 60 included, is
            /// `HC_ERROR_INVALID_DATE`, `zone` is as for `hc_localtime`, and a reading
            /// the zone's rules do not reach is `HC_ERROR_OUT_OF_RANGE`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// What a wall-clock reading means in a zone before a policy reduces it to
            /// one instant, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: `unique` where the zone's clock showed the reading once,
            /// `ambiguous` where the clocks went back and showed it twice,
            /// `nonexistent` where they went forward past it; the first instant, as a
            /// POSIX second, and its offset in seconds east of UTC; the second instant
            /// and its offset. A unique reading has the one instant in both places; an
            /// ambiguous one has its two occurrences, the earlier first; a skipped one
            /// has the instant it would be under the offset in force after the gap,
            /// before the gap opened, and the instant it would be under the offset in
            /// force before it, after the gap closed. The `earliest`, `latest`, `reject`
            /// and `push-forward` policies of `hc_mktime` (listed by
            /// `hc_mktime_policies`) choose between the two or refuse. The fields are
            /// checked as for `hc_mktime`: one out of range, a second of 60 included, is
            /// `HC_ERR_INVALID_DATE`; `zone` is as for `hc_localtime`, a name neither
            /// loaded nor built in `HC_ERR_UNKNOWN`, and a reading outside the years the
            /// rules answer for `HC_ERR_OUT_OF_RANGE`. The name's pointer and bytes fail
            /// as for `hc_parse_iso_date`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_local_resolution(
            year: i64,
            month: i64,
            day: i64,
            hour: i64,
            minute: i64,
            second: i64,
            zone: name(zone_len),
        ) -> line =
            |year, month, day, hour, minute, second, zone| {
                $crate::python_lines::local_resolution_line(
                    [year, month, day, hour, minute, second],
                    zone,
                )
            };

        c {
            /// The policies `hc_mktime` reads for a reading two instants name or none
            /// names, as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's, one a policy. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// The policies `hc_mktime` reads for a reading two instants name or none
            /// names, as UTF-8 lines, returning the byte length written.
            ///
            /// One line a policy, tab-separated: the identifier `hc_mktime` reads
            /// (`earliest`, `latest`, `reject`, `push-forward`) and what the policy
            /// makes of a repeated reading and of a skipped one, in words. Python reads
            /// `tm_isdst` where these read a word, and the four are each their own
            /// convention (`docs/policy.md` §5). A null `buffer` returns the length the
            /// text needs.
        }
        fn hc_mktime_policies() -> line =
            || Ok($crate::python_lines::mktime_policies_lines());
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
            /// The line is the WebAssembly module's: the tropical sign's number,
            /// identifier and English name, the decan within it, 1 to 3, its ruler's identifier and
            /// English name, and the degrees into the decan. An instant outside the
            /// years −1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// The decan the Sun is in at a POSIX timestamp, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the tropical sign, 1 for Aries through 12 for Pisces,
            /// its identifier, `aries`, and its English name, `Aries`; which of the sign's three 10° decans, or
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
            /// The line is the WebAssembly module's: the sidereal sign's number,
            /// identifier and Sanskrit name, the drekkāṇa within it, its lord's
            /// identifier and English name, the degrees into it, the lord's sign by
            /// identifier and Sanskrit name, and the ayanāṃśa.
            /// `ayanamsa` is `lahiri`, `lahiri-rashtriya`, `lahiri-crc-1955`, `lahiri-drik`, `raman`, `krishnamurti`,
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
            /// for Meṣa through 12 for Mīna, in the zodiac of the ayanāṃśa, its
            /// identifier, `kanya`, and its Sanskrit name, `Kanyā`; which of its three 10° drekkāṇas the Sun is in, 1 to
            /// 3; the drekkāṇa's lord, the ruler of the sign it is given to — the
            /// sign itself, the fifth from it or the ninth, as al-Bīrūnī tabulates
            /// them — as its identifier and its English name; how far into the
            /// drekkāṇa the Sun is, in degrees from 0 up to 10; that sign by its
            /// identifier and its Sanskrit name, `mesha` and `Meṣa`; and the
            /// ayanāṃśa's identifier. `ayanamsa` is
            /// `lahiri`, `lahiri-rashtriya`, `lahiri-crc-1955`, `lahiri-drik`, `raman`, `krishnamurti`, `reingold-dershowitz` or
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
            /// The length of a temporal hour of a fixed day at a place by a
            /// reckoning of the Jewish day, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// `reckoning` is as for `hc_zmanim`; anything else is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. The line is
            /// the WebAssembly module's: the reckoning, the length in seconds, and
            /// the four cells of a missing solar event. A place off the globe, or a
            /// day outside the years −1000 to 3000, is `HC_ERROR_OUT_OF_RANGE`.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// The length of a temporal hour of a fixed day at a place by a
            /// reckoning of the Jewish day, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// A temporal hour is a twelfth of the day the reckoning counts:
            /// `zmanim-gra` sunrise to sunset, `mga-72-minutes` and
            /// `mga-16-1-degrees` the Magen Avraham's dawn to nightfall, as for
            /// `hc_zmanim`, whose times are counted in it; anything else is
            /// `HC_ERR_UNKNOWN`. The day and the place are as for
            /// `hc_solar_event`. Tab-separated: the reckoning's identifier; the
            /// length in seconds, a decimal; and the four cells of `hc_solar_event`
            /// naming a missing solar event, the length being empty where the
            /// day's start or end does not happen. A place off the globe, or a day
            /// outside the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_temporal_hour(
            reckoning: name(reckoning_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |reckoning, fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation).and_then(|place| {
                    $crate::hours_lines::temporal_hour_line(reckoning, fixed, place)
                })
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

        c {
            /// Moonrise on a local day at a place against a named horizon, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `horizon`, the place and the
            /// day are as for `hc_sunrise`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// Moonrise on a local day at a place against a named horizon, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the moment the Moon's upper limb rises over the
            /// horizon, as whole POSIX seconds of Universal Time, rounded down;
            /// then `moonrise` and the fixed day, which are empty when the Moon
            /// rises, the first cell being empty instead when it does not — which
            /// happens about once a month everywhere, the Moon rising some fifty
            /// minutes later each day and so skipping a local day. The horizon's
            /// Moon rule, the limb's altitude scaled by the parallax or the fixed
            /// depression a tradition states, is `hc_horizons`' sixth column.
            /// `horizon`, the place and the day are as for `hc_sunrise`: a place
            /// off the globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_moonrise(
            horizon: name(horizon_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |horizon, fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::astro_lines::moonrise_line(horizon, fixed, place))
            };

        c {
            /// Moonset on a local day at a place against a named horizon, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// As `hc_moonrise`, for the upper limb's setting, with `moonset` as the
            /// missing event.
        }
        wasm {
            /// Moonset on a local day at a place against a named horizon, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// As `hc_moonrise`, for the upper limb's setting, with `moonset` as the
            /// missing event.
        }
        fn hc_moonset(
            horizon: name(horizon_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |horizon, fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::astro_lines::moonset_line(horizon, fixed, place))
            };

        c {
            /// UT1R at a UT1 instant by the IERS 2010 zonal tide model, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. The instant is as for
            /// `hc_earth_rotation_angle`, and fails as it does. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// UT1R at a UT1 instant by the IERS 2010 zonal tide model, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// UT1 with the 41 zonal tides of periods under 35 days of IERS
            /// Conventions 2010, Table 8.1, removed, as the IAU defined UT1R in
            /// 1982; the tidal model is in the name, since the Conventions' table
            /// is not Yoder's of 1981. Tab-separated: UT1R − UT1 in seconds, within
            /// ±2.75 ms, and the UT1R reading, counted as the UT1 given is, 86 400
            /// seconds a day from 1970-01-01 00:00 UT1. The instant is as for
            /// `hc_earth_rotation_angle`, and fails as it does. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_ut1r_iers2010(ut1_unix_seconds: f64) -> line =
            $crate::astro_lines::ut1r_iers2010_line;

        c {
            /// UT1S at a UT1 instant by the IERS 2010 zonal tide model, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// As `hc_ut1r_iers2010`, with all 62 tides removed.
        }
        wasm {
            /// UT1S at a UT1 instant by the IERS 2010 zonal tide model, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// As `hc_ut1r_iers2010`, with all 62 zonal tides of Table 8.1 removed,
            /// to the 18.6-year nodal term: UT1S − UT1 in seconds, within ±0.173 s,
            /// and the UT1S reading.
        }
        fn hc_ut1s_iers2010(ut1_unix_seconds: f64) -> line =
            $crate::astro_lines::ut1s_iers2010_line;

        c {
            /// The effect on UT1 of the zonal tides whose period is under a limit,
            /// at a UT1 instant, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line is the WebAssembly module's. The instant is as for
            /// `hc_earth_rotation_angle`, and fails as it does; a limit that is not
            /// positive is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The effect on UT1 of the zonal tides whose period is under a limit,
            /// at a UT1 instant, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// The one cell is the sum, in seconds, of the terms of IERS Conventions
            /// 2010, Table 8.1, whose period in days is under `period_limit_days`:
            /// 35 is UT1R's 41 tides and an infinite limit UT1S's 62, and a limit
            /// between them is a model of the caller's own. The regularised reading
            /// is UT1 *minus* this, as the Conventions say the corrections are
            /// subtracted. The instant is as for `hc_earth_rotation_angle`, and
            /// fails as it does; a limit that is not positive, NaN included, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_zonal_tide_ut1_effect(ut1_unix_seconds: f64, period_limit_days: f64) -> line =
            $crate::astro_lines::zonal_tide_ut1_effect_line;

        c {
            /// The equation of time at a POSIX timestamp, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. An instant outside the years
            /// −1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The equation of time at a POSIX timestamp, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// The one cell is apparent solar time less mean solar time, in
            /// seconds, positive when a sundial is ahead of the mean clock and up
            /// to about sixteen minutes either way over the year, from the hour
            /// angle of the apparent Sun that `hc_solar_time`'s `local-apparent`
            /// clock and the rise and set exports solve: +822.6 s on 1992 October 13
            /// at 0h, Meeus's example 28.a. The timestamp is read as Universal Time;
            /// one outside the years −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_equation_of_time(unix_seconds: i64) -> line =
            $crate::astro_lines::equation_of_time_line;

        c {
            /// Apparent solar noon on a local day at a place, as POSIX seconds of
            /// Universal Time, written to `out_unix_seconds`.
            ///
            /// The Sun's upper transit of the local meridian, which every day has,
            /// under the midnight sun too. The place is as for `hc_sunrise`; one
            /// off the globe, or a day outside the years −1000 to 3000, is
            /// `HC_ERROR_OUT_OF_RANGE`.
        }
        wasm {
            /// Apparent solar noon on a local day at a place, as whole POSIX seconds
            /// of Universal Time, rounded down, or an error sentinel.
            ///
            /// The Sun's upper transit of the local meridian, which every day has,
            /// under the midnight sun too, so there is no missing event to name: the
            /// moment `hc_solar_time`'s `local-apparent` clock reads 12. The day is
            /// the local one, from local mean midnight at the longitude, and the
            /// place is as for `hc_sunrise`; one off the globe, or a day outside the
            /// years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`.
        }
        fn hc_solar_noon(fixed: i64, latitude: f64, longitude: f64, elevation: f64) -> value(out_unix_seconds: i64) =
            |fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::astro_lines::solar_noon(fixed, place))
            };

        c {
            /// Apparent solar midnight opening a local day at a place, as POSIX
            /// seconds of Universal Time, written to `out_unix_seconds`.
            ///
            /// The Sun's lower transit, half a day before the day's noon; the place
            /// and the day are as for `hc_solar_noon`, and fail as it does.
        }
        wasm {
            /// Apparent solar midnight opening a local day at a place, as whole
            /// POSIX seconds of Universal Time, rounded down, or an error sentinel.
            ///
            /// The Sun's lower transit, half a day before the day's `hc_solar_noon`,
            /// which every day has; the place and the day are as for
            /// `hc_solar_noon`, and fail as it does.
        }
        fn hc_solar_midnight(fixed: i64, latitude: f64, longitude: f64, elevation: f64) -> value(out_unix_seconds: i64) =
            |fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::astro_lines::solar_midnight(fixed, place))
            };

        c {
            /// The start of a named twilight on a local day at a place, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `twilight` is `civil`, `nautical` or `astronomical`, in any case;
            /// anything else is `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`.
            /// The line is the WebAssembly module's: the instant as POSIX seconds,
            /// and the four cells of `hc_solar_event` naming the missing
            /// `depression`. The place is as for `hc_sunrise`; one off the globe, or
            /// a day outside the years −1000 to 3000, is `HC_ERROR_OUT_OF_RANGE`.
            /// Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// The start of a named twilight on a local day at a place, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// `twilight` is `civil`, the Sun's centre 6° below the horizon;
            /// `nautical`, 12°; or `astronomical`, 18°; in any case; anything else,
            /// the empty string included, is `HC_ERR_UNKNOWN`. The day is the local
            /// one, from local mean midnight at the longitude, and the place is as
            /// for `hc_sunrise`, the elevation unused since a depression is measured
            /// from the geometric horizon. Tab-separated: the instant the Sun's
            /// centre rises to the depression as whole POSIX seconds of Universal
            /// Time, rounded down; and the four cells of `hc_solar_event` naming a
            /// missing solar event — `depression`, the day, and the depression
            /// sought in arcminutes and in arcseconds — which are empty when the
            /// twilight begins that morning, the first cell being empty instead
            /// when the Sun never gets that low (a light summer night) or never gets
            /// that high (the polar winter). A place off the globe, or a day
            /// outside the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_dawn(
            twilight: name(twilight_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |twilight, fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::astro_lines::dawn_line(twilight, fixed, place))
            };

        c {
            /// The end of a named twilight on a local day at a place, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// As `hc_dawn`, for the evening: the instant the Sun's centre sets to
            /// the twilight's depression.
        }
        wasm {
            /// The end of a named twilight on a local day at a place, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// As `hc_dawn`, for the evening: the instant the Sun's centre sets to
            /// the twilight's depression, or the missing `depression` named.
        }
        fn hc_dusk(
            twilight: name(twilight_len),
            fixed: i64,
            latitude: f64,
            longitude: f64,
            elevation: f64,
        ) -> line =
            |twilight, fixed, latitude, longitude, elevation| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| $crate::astro_lines::dusk_line(twilight, fixed, place))
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

        c {
            /// The daily mean insolation at any latitude and solar longitude, for the orbit of an
            /// epoch, as NUL-terminated UTF-8 one line in a caller-owned buffer.
            ///
            /// The Sun's true longitude is 0 at the March equinox, 90 at the June solstice, 180 at
            /// the September equinox and 270 at the December solstice. It is not a date: Berger's
            /// program turns a date into a longitude with a 365-day year, which differs from a
            /// calendar's by up to a day, and the honest input is the longitude. `hc_orbit_at` is
            /// the case of 65° N at 90°. The insolation is 0 in the polar night. An epoch the crate
            /// refuses (not finite, or beyond a million years either side of 1950), a latitude that
            /// is not finite or is beyond ±90°, or a longitude that is not finite or is outside 0
            /// to 360, is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `years_before_present` is years before 1950, negative for the future.
            ///
            /// Tab-separated: years before 1950, latitude, solar longitude, insolation, solar
            /// constant, source.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// The daily mean insolation at any latitude and solar longitude, for the orbit of an
            /// epoch, as one UTF-8 line, returning the byte length written.
            ///
            /// The Sun's true longitude is 0 at the March equinox, 90 at the June solstice, 180 at
            /// the September equinox and 270 at the December solstice. It is not a date: Berger's
            /// program turns a date into a longitude with a 365-day year, which differs from a
            /// calendar's by up to a day, and the honest input is the longitude. `hc_orbit_at` is
            /// the case of 65° N at 90°. The insolation is 0 in the polar night. An epoch the crate
            /// refuses (not finite, or beyond a million years either side of 1950), a latitude that
            /// is not finite or is beyond ±90°, or a longitude that is not finite or is outside 0
            /// to 360, is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `years_before_present` is years before 1950, negative for the future.
            ///
            /// Tab-separated: years before 1950, latitude, solar longitude, insolation, solar
            /// constant, source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_daily_insolation(
            years_before_present: f64,
            latitude_degrees: f64,
            solar_longitude_degrees: f64,
        ) -> line =
            $crate::orbital_lines::insolation_line;
    } };
    ("jupiter", $backend:ident) => { $backend! {
        c {
            /// Where Jupiter is at a POSIX timestamp, tropical and sidereal, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// `ayanamsa` is as for `hc_panchanga_at`; null for it is
            /// `HC_ERROR_NULL_POINTER` and a name not known `HC_ERROR_UNKNOWN`. The
            /// line is the WebAssembly module's: the apparent geocentric ecliptic
            /// longitude, latitude and distance, the sidereal longitude, the sidereal
            /// sign and the degrees into it, the daily motion and whether Jupiter is
            /// in retrograde, and its heliocentric position. An instant outside the
            /// years −1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// Where Jupiter is at a POSIX timestamp, tropical and sidereal, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// Computed from the complete VSOP87B series for Jupiter, with light-time,
            /// annual aberration and nutation: the apparent position as an almanac
            /// gives it, to about 1″ from 1000 CE to 2100 CE and to 9″ at the ends of
            /// the era. The `ayanamsa` is as for `hc_panchanga_at`; one not known is
            /// `HC_ERR_UNKNOWN`. Tab-separated: the apparent geocentric ecliptic
            /// longitude in degrees, in the true equinox of the date, the tropical one;
            /// the latitude in degrees; the distance in astronomical units; the
            /// sidereal longitude in degrees, that less the ayanāṃśa; the sidereal sign
            /// by its identifier (`mesha` to `mina`) and its Sanskrit name; the degrees
            /// into the sign; the longitude's change in degrees a day, negative in
            /// retrograde; `1` when Jupiter is in retrograde, else `0`; and its
            /// heliocentric longitude, latitude and distance, geometric, in the mean
            /// ecliptic and equinox of the date, the distance in astronomical units. An
            /// instant outside the years −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_jupiter_at(unix_seconds: i64, ayanamsa: name(ayanamsa_len)) -> line =
            $crate::jupiter_lines::jupiter_line;

        c {
            /// Jupiter's crossings of the boundaries of the sidereal signs in a span of
            /// POSIX seconds, as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The span is `[from_unix_seconds, to_unix_seconds)`; `ayanamsa` is as for
            /// `hc_panchanga_at`; null for it is `HC_ERROR_NULL_POINTER`. The lines are
            /// the WebAssembly module's: the moment, the sign left and the sign entered
            /// by identifier and Sanskrit name, and `forward` or `retrograde`. An end
            /// outside the years −1000 to 3000, or a span longer than a hundred Julian
            /// years, is `HC_ERROR_OUT_OF_RANGE`; a `to` not after `from` is an empty
            /// answer. Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// Jupiter's crossings of the boundaries of the sidereal signs in a span of
            /// POSIX seconds, as UTF-8 lines, one each, in time order, returning the
            /// byte length written.
            ///
            /// The span is `[from_unix_seconds, to_unix_seconds)`. Jupiter turns back
            /// out of a sign about two years in three, so the lines of a year come in
            /// runs. Tab-separated: the moment, as whole POSIX seconds of Universal
            /// Time, rounded down; the sign left, by identifier and Sanskrit name; the
            /// sign entered, by identifier and Sanskrit name; and `forward` when Jupiter
            /// moves on to the next sign, `retrograde` when it turns back into the one
            /// before. The `ayanamsa` is as for `hc_panchanga_at`; one not known is
            /// `HC_ERR_UNKNOWN`. An end outside the years −1000 to 3000, or a span
            /// longer than a hundred Julian years, is `HC_ERR_OUT_OF_RANGE`; a `to` not
            /// after `from` is an empty answer of zero bytes. A null `buffer` returns
            /// the length the text needs.
        }
        fn hc_jupiter_ingresses(
            from_unix_seconds: i64,
            to_unix_seconds: i64,
            ayanamsa: name(ayanamsa_len),
        ) -> line =
            $crate::jupiter_lines::ingress_lines;

        c {
            /// Jupiter's heliacal risings in a span of POSIX seconds, each with the
            /// name a year of Jupiter has from it, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The span is `[from_unix_seconds, to_unix_seconds)`; `ayanamsa` is as for
            /// `hc_panchanga_at`; null for it is `HC_ERROR_NULL_POINTER`. The lines are
            /// the WebAssembly module's: the rising and the setting before it, the
            /// sidereal longitude and the nakṣatra, and the year's name. An end outside
            /// the years −1000 to 3000, or a span longer than a hundred Julian years,
            /// is `HC_ERROR_OUT_OF_RANGE`; a `to` not after `from` is an empty answer.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// Jupiter's heliacal risings in a span of POSIX seconds, each with the
            /// name a year of Jupiter has from it, as UTF-8 lines, returning the byte
            /// length written.
            ///
            /// The span is `[from_unix_seconds, to_unix_seconds)`. A rising is when
            /// Jupiter's longitude, west of the Sun's after their conjunction, has
            /// passed 11°, the arc of visibility Varāhamihira, Bhāskara I and the
            /// *Sūrya Siddhānta* give; a year of Jupiter runs from one rising to the
            /// next, about 399 days. Tab-separated: the rising and the setting before
            /// it, when Jupiter came within 11° east of the Sun and was lost in its
            /// light, as whole POSIX seconds of Universal Time, rounded down; Jupiter's
            /// sidereal longitude at the rising, in degrees; the nakṣatra it is in, by
            /// number from 1, identifier and name; and the year's name by the
            /// *Bṛhatsaṃhitā*, ch. 8, which names the year after the lunar month whose
            /// nakṣatras hold the rising (`Karttika` for Kṛttikā and Rohiṇī), as the
            /// twelve-year cycle of `hc_barhaspatya_year_at` spells it, and its position
            /// from 1, Chaitra, to 12. A name is skipped where Jupiter passes over a
            /// whole run of nakṣatras between two risings. This is Jupiter's rising on
            /// the true sky by that fixed arc, not what any Siddhāntic almanac prints,
            /// whose Jupiter and arc differ; Drik Panchang's visibility, which is
            /// local, differs by up to four days. The `ayanamsa` is as for
            /// `hc_panchanga_at`; one not known is `HC_ERR_UNKNOWN`. An end outside the
            /// years −1000 to 3000, or a span longer than a hundred Julian years, is
            /// `HC_ERR_OUT_OF_RANGE`; a `to` not after `from` is an empty answer of
            /// zero bytes. A null `buffer` returns the length the text needs.
        }
        fn hc_jupiter_risings(
            from_unix_seconds: i64,
            to_unix_seconds: i64,
            ayanamsa: name(ayanamsa_len),
        ) -> line =
            $crate::jupiter_lines::rising_lines;

        c {
            /// When in a Gregorian year the Sun, and the Moon where it is asked for,
            /// stand as a condition of the Kumbh Mela requires, and whether Jupiter, whose
            /// sign is computed, meets it, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// `yoga` and `ayanamsa` are as for `hc_kumbh`; null for either is
            /// `HC_ERROR_NULL_POINTER`. The line is the WebAssembly module's: the
            /// columns of `hc_kumbh`, with the last always `1` or `0`, then Jupiter's
            /// sidereal sign and longitude at the occasion's first moment. A year
            /// outside −1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// When in a Gregorian year the Sun, and the Moon where it is asked for,
            /// stand as a condition of the Kumbh Mela requires, and whether Jupiter, whose
            /// sign is computed, meets it, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// The same as `hc_kumbh` with Jupiter's sidereal sign computed at the
            /// occasion's first moment from the VSOP87B series, in the zodiac of the
            /// same ayanāṃśa, in place of the caller's. The caller's form is the
            /// `calendars` layer's `hc_kumbh`, which takes the sign as an argument.
            /// Tab-separated, the thirteen columns of `hc_kumbh`, the last, whether the
            /// occasion exists and Jupiter is in the condition's sign, never empty;
            /// then Jupiter's sidereal sign by identifier, and its sidereal longitude
            /// in degrees, both empty when the Sun's stay that year holds no new moon
            /// and the condition asks for one. The locale argument fails as
            /// `hc_parse_iso_date` does. A condition or ayanāṃśa not known is
            /// `HC_ERR_UNKNOWN`; a year outside −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_kumbh_by_sky(
            yoga: name(yoga_len),
            year: i64,
            ayanamsa: name(ayanamsa_len),
            locale: text(locale_len),
        ) -> line =
            $crate::jupiter_lines::kumbh_by_sky_line;

        c {
            /// Every condition of the Kumbh Mela that a Gregorian year's sky meets or
            /// does not, as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// `ayanamsa` is as for `hc_panchanga_at` and null is
            /// `HC_ERROR_NULL_POINTER`; `locale` is as for `hc_kumbh`. The lines are
            /// the WebAssembly module's. A year outside −1000 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Every condition of the Kumbh Mela that a Gregorian year's sky meets or
            /// does not, as UTF-8 lines, returning the byte length written.
            ///
            /// The line of `hc_kumbh_by_sky` for each of the seven conditions
            /// `hc_kumbh_yogas` lists, in that order and byte for byte what the
            /// seven calls give, in one call: the first cell names the condition and
            /// the thirteenth is `1` where the year's sky meets it. In 2025 that is
            /// Prayag's `kumbh-prayag-vrishabha` alone, the Maha Kumbh. The
            /// `ayanamsa` is as for `hc_panchanga_at`; one not known is
            /// `HC_ERR_UNKNOWN`. The locale argument fails as `hc_parse_iso_date` does.
            /// A year outside −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
            /// returns the length the text needs.
        }
        fn hc_kumbhs_in_year_by_sky(
            year: i64,
            ayanamsa: name(ayanamsa_len),
            locale: text(locale_len),
        ) -> line =
            $crate::jupiter_lines::kumbhs_in_year_by_sky_lines;

        c {
            /// Jupiter's stations in a span of POSIX seconds, as NUL-terminated UTF-8
            /// lines in a caller-owned buffer.
            ///
            /// The span is `[from_unix_seconds, to_unix_seconds)`; `ayanamsa` is as for
            /// `hc_panchanga_at`; null for it is `HC_ERROR_NULL_POINTER`. The lines are
            /// the WebAssembly module's. An end outside the years −1000 to 3000, or a
            /// span longer than a hundred Julian years, is `HC_ERROR_OUT_OF_RANGE`; a
            /// `to` not after `from` is an empty answer. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// Jupiter's stations in a span of POSIX seconds, as UTF-8 lines, one each,
            /// in time order, returning the byte length written.
            ///
            /// The span is `[from_unix_seconds, to_unix_seconds)`. A station is where
            /// Jupiter's apparent longitude stops changing, about every four months.
            /// Tab-separated: the moment, as whole POSIX seconds of Universal Time,
            /// rounded down; `retrograde` where it turns back, *vakri*, or `direct`
            /// where it resumes, *mārgī*, Drik Panchang's "becomes progressive"; the
            /// sidereal sign it turns in, by identifier and Sanskrit name; and its
            /// sidereal longitude there in degrees. The station is found in the tropical
            /// longitude, as Drik Panchang's dates are, which they agree with to
            /// within 7 minutes on 18 stations from 2010 to 2027. The `ayanamsa` is as
            /// for `hc_panchanga_at`; one not known is `HC_ERR_UNKNOWN`. An end outside
            /// the years −1000 to 3000, or a span longer than a hundred Julian years,
            /// is `HC_ERR_OUT_OF_RANGE`; a `to` not after `from` is an empty answer of
            /// zero bytes. A null `buffer` returns the length the text needs.
        }
        fn hc_jupiter_stations(
            from_unix_seconds: i64,
            to_unix_seconds: i64,
            ayanamsa: name(ayanamsa_len),
        ) -> line =
            $crate::jupiter_lines::station_lines;

        c {
            /// The twelve days of the *Ādi Pushkaram* of each river of a sidereal sign,
            /// for Jupiter's entry into it in a Gregorian year, found, as NUL-terminated
            /// UTF-8 lines in a caller-owned buffer.
            ///
            /// `sign`, `ayanamsa` and `meridian` are named as for `hc_kumbh` and
            /// `hc_term_in_effect`, and `rule` is `pushkaram-final-entry` or
            /// `pushkaram-first-entry`, in any case; null for any is
            /// `HC_ERROR_NULL_POINTER`, and a name not known `HC_ERROR_UNKNOWN`. The
            /// lines are the WebAssembly module's: those of `hc_pushkaram`, then the
            /// moment of the entry and the rule. No line where Jupiter makes no such
            /// entry that year. A place off the globe, or a year outside −1000 to
            /// 3000, is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
            /// the terminator, into `written`.
        }
        wasm {
            /// The twelve days of the *Ādi Pushkaram* of each river of a sidereal sign,
            /// for Jupiter's entry into it in a Gregorian year, found, as UTF-8 lines,
            /// each river named in a locale, returning the byte length written.
            ///
            /// The same as `hc_pushkaram` with the entry computed from the VSOP87B
            /// series for Jupiter in place of the caller's: the entry of Jupiter into
            /// `sign`, from the sign before it, that falls in the Gregorian year, by
            /// `ayanamsa`, the year of the entry's date at `meridian`, the clock that
            /// cuts the twelve days: an entry at 19:45 UT on 31 December 2149 is 1
            /// January 2150 at the Indian meridian. Where Jupiter enters, turns back and enters again, `rule`
            /// says which entry counts: `pushkaram-final-entry`, the one after which
            /// Jupiter stays, which every festival whose dates were read began at, or
            /// `pushkaram-first-entry`, in any case. The first day is as for
            /// `hc_pushkaram`. One line per river of the sign, the columns of
            /// `hc_pushkaram`, then the moment of the entry as whole POSIX seconds of
            /// Universal Time, rounded down, and the rule's identifier; no line, an
            /// answer of zero bytes, where Jupiter makes no such entry that year. A
            /// sign, ayanāṃśa, rule or meridian not known is `HC_ERR_UNKNOWN`. The
            /// locale argument fails as `hc_parse_iso_date` does. A place off the
            /// globe, or a year outside −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A
            /// null `buffer` returns the length the text needs.
        }
        fn hc_pushkaram_by_sky(
            sign: name(sign_len),
            year: i64,
            ayanamsa: name(ayanamsa_len),
            rule: name(rule_len),
            latitude: f64,
            longitude: f64,
            elevation: f64,
            meridian: name(meridian_len),
            locale: text(locale_len),
        ) -> line =
            |sign, year, ayanamsa, rule, latitude, longitude, elevation, meridian, locale| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::jupiter_lines::pushkaram_by_sky_lines(
                            sign,
                            year,
                            ayanamsa,
                            rule,
                            place,
                            meridian,
                            locale,
                        )
                    })
            };

        c {
            /// The twelve days of the *Ādi Pushkaram* of each river of every sidereal
            /// sign Jupiter enters in a Gregorian year, found, as NUL-terminated UTF-8
            /// lines in a caller-owned buffer.
            ///
            /// `ayanamsa`, `rule` and `meridian` are as for `hc_pushkaram_by_sky`; null
            /// for any is `HC_ERROR_NULL_POINTER`, and a name not known
            /// `HC_ERROR_UNKNOWN`. The lines are the WebAssembly module's: those of
            /// `hc_pushkaram_by_sky`, for each sign Jupiter enters that year, in the
            /// order of the entries, each line carrying its sign. No line in a year
            /// without an entry by the rule. A place off the globe, or a year outside
            /// −1000 to 3000, is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The twelve days of the *Ādi Pushkaram* of each river of every sidereal
            /// sign Jupiter enters in a Gregorian year, found, as UTF-8 lines, each
            /// river named in a locale, returning the byte length written.
            ///
            /// What `hc_pushkaram_by_sky` gives for each sign Jupiter enters that year,
            /// the year of the entry's date at `meridian`, one sign after another in the order of the entries, in one call and one
            /// search of the sky where the sign-by-sign form makes twelve: the same
            /// bytes. One line per river of each sign, the columns of
            /// `hc_pushkaram_by_sky`, whose sign columns say which sign a line is
            /// for. A year in which Jupiter enters two signs has the lines of both; one
            /// in which it enters a sign, turns back and enters it again has the entry
            /// `rule` names, `pushkaram-final-entry` or `pushkaram-first-entry`, in any
            /// case, as for `hc_pushkaram_by_sky`. No line, an answer of zero bytes, in
            /// a year with no entry by the rule. An ayanāṃśa, rule or meridian not known
            /// is `HC_ERR_UNKNOWN`. The locale argument fails as `hc_parse_iso_date`
            /// does. A place off the globe, or a year outside −1000 to 3000, is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text needs.
        }
        fn hc_pushkarams_in_year(
            year: i64,
            ayanamsa: name(ayanamsa_len),
            rule: name(rule_len),
            latitude: f64,
            longitude: f64,
            elevation: f64,
            meridian: name(meridian_len),
            locale: text(locale_len),
        ) -> line =
            |year, ayanamsa, rule, latitude, longitude, elevation, meridian, locale| {
                $crate::astro_lines::location(latitude, longitude, elevation)
                    .and_then(|place| {
                        $crate::jupiter_lines::pushkarams_in_year_lines(
                            year,
                            ayanamsa,
                            rule,
                            place,
                            meridian,
                            locale,
                        )
                    })
            };

        c {
            /// The rules for which entry of Jupiter into a sign a Pushkaram follows, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines are the WebAssembly module's. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The rules for which entry of Jupiter into a sign a Pushkaram follows, as
            /// UTF-8 lines, returning the byte length written.
            ///
            /// One line a rule, the one the festivals read follow first,
            /// tab-separated: the identifier, `pushkaram-final-entry` or
            /// `pushkaram-first-entry`, which `hc_pushkaram_by_sky` and
            /// `hc_pushkarams_in_year` take; and a sentence saying which entry it
            /// counts, where Jupiter enters a sign, turns back and enters it again.
            /// Each is its own rule (`docs/policy.md` §5). A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_pushkaram_rules() -> line =
            || Ok($crate::jupiter_lines::pushkaram_rules_lines());
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

        c {
            /// A clock on a circular orbit against one held still on the ground, as NUL-terminated
            /// UTF-8 one line in a caller-owned buffer.
            ///
            /// The line is the one kind of figure GPS is built on: the gravitational part of the
            /// rate, in microseconds per 86 400-second day (positive: the higher clock runs fast),
            /// the kinematic part (negative), their weak-field sum, and the exact Schwarzschild
            /// figure for the same two clocks. A clock at `GPS_ORBIT_RADIUS` against one at
            /// `EARTH_EQUATORIAL_RADIUS` gains +45.65 from the potential, loses 7.21 from its
            /// speed, and nets +38.44. `body` is an identifier `hc_gravitating_bodies` lists, in
            /// any ASCII case; another is `HC_ERROR_UNKNOWN`. A radius that is not finite or not
            /// positive, a ground radius at or inside the Schwarzschild radius, or an orbit radius
            /// at or inside the photon sphere 3GM/c², where no circular orbit exists, is
            /// `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `body` is the body's identifier.
            ///
            /// Tab-separated: id, gm, gm constant, circular speed, gravitational microseconds per
            /// day, kinematic microseconds per day, weak-field microseconds per day, exact
            /// microseconds per day, constants, source.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A clock on a circular orbit against one held still on the ground, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// The line is the one kind of figure GPS is built on: the gravitational part of the
            /// rate, in microseconds per 86 400-second day (positive: the higher clock runs fast),
            /// the kinematic part (negative), their weak-field sum, and the exact Schwarzschild
            /// figure for the same two clocks. A clock at `GPS_ORBIT_RADIUS` against one at
            /// `EARTH_EQUATORIAL_RADIUS` gains +45.65 from the potential, loses 7.21 from its
            /// speed, and nets +38.44. `body` is an identifier `hc_gravitating_bodies` lists, in
            /// any ASCII case; another is `HC_ERR_UNKNOWN`. A radius that is not finite or not
            /// positive, a ground radius at or inside the Schwarzschild radius, or an orbit radius
            /// at or inside the photon sphere 3GM/c², where no circular orbit exists, is
            /// `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `body` is the body's identifier.
            ///
            /// Tab-separated: id, gm, gm constant, circular speed, gravitational microseconds per
            /// day, kinematic microseconds per day, weak-field microseconds per day, exact
            /// microseconds per day, constants, source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_orbit_rate_offset(
            body: name(body_len),
            orbit_radius_metres: f64,
            ground_radius_metres: f64,
        ) -> line =
            $crate::relativity_lines::orbit_rate_offset_line;

        c {
            /// A rocket of constant proper acceleration burning from rest, as NUL-terminated UTF-8
            /// one line in a caller-owned buffer.
            ///
            /// The line is the hyperbolic motion of a constant proper acceleration
            /// `proper_acceleration` in m s⁻² after `proper_seconds` aboard: the time that passes
            /// elsewhere, the distance covered, β, `1 − β` computed without cancellation, and the
            /// Lorentz factor. One year at 1 g, 9.80665 m s⁻², is 0.5636 light-years at three
            /// quarters of the speed of light, and 1.19 years pass elsewhere. An acceleration that
            /// is not finite and positive, a proper time that is not finite or is negative, or a
            /// burn long enough that a value leaves the range of a double, is
            /// `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `proper_acceleration` is metres per second squared.
            ///
            /// Tab-separated: acceleration, proper seconds, coordinate seconds, distance, distance
            /// light years, beta, one minus beta, lorentz factor, constants, source.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// A rocket of constant proper acceleration burning from rest, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// The line is the hyperbolic motion of a constant proper acceleration
            /// `proper_acceleration` in m s⁻² after `proper_seconds` aboard: the time that passes
            /// elsewhere, the distance covered, β, `1 − β` computed without cancellation, and the
            /// Lorentz factor. One year at 1 g, 9.80665 m s⁻², is 0.5636 light-years at three
            /// quarters of the speed of light, and 1.19 years pass elsewhere. An acceleration that
            /// is not finite and positive, a proper time that is not finite or is negative, or a
            /// burn long enough that a value leaves the range of a double, is
            /// `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `proper_acceleration` is metres per second squared.
            ///
            /// Tab-separated: acceleration, proper seconds, coordinate seconds, distance, distance
            /// light years, beta, one minus beta, lorentz factor, constants, source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_rocket(proper_acceleration: f64, proper_seconds: f64) -> line =
            $crate::relativity_lines::rocket_line;

        c {
            /// A flip-and-burn voyage between two points at rest, as NUL-terminated UTF-8 one line
            /// in a caller-owned buffer.
            ///
            /// The ship accelerates at a constant proper acceleration for half the distance, turns
            /// over, and decelerates for the other half, arriving at rest. At 1 g to Andromeda, 2.5
            /// million light-years, it is 28.60 years aboard and 2 500 001.94 at home. An
            /// acceleration that is not finite and positive, a distance that is not finite or is
            /// negative, or a voyage that leaves the range of a double, is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `proper_acceleration` is metres per second squared.
            ///
            /// Tab-separated: acceleration, distance, proper seconds, coordinate seconds, proper
            /// years, coordinate years, peak beta, one minus peak beta, peak lorentz factor,
            /// constants, source.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// A flip-and-burn voyage between two points at rest, as one UTF-8 line, returning the
            /// byte length written.
            ///
            /// The ship accelerates at a constant proper acceleration for half the distance, turns
            /// over, and decelerates for the other half, arriving at rest. At 1 g to Andromeda, 2.5
            /// million light-years, it is 28.60 years aboard and 2 500 001.94 at home. An
            /// acceleration that is not finite and positive, a distance that is not finite or is
            /// negative, or a voyage that leaves the range of a double, is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `proper_acceleration` is metres per second squared.
            ///
            /// Tab-separated: acceleration, distance, proper seconds, coordinate seconds, proper
            /// years, coordinate years, peak beta, one minus peak beta, peak lorentz factor,
            /// constants, source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_flip_and_burn(proper_acceleration: f64, distance_metres: f64) -> line =
            $crate::relativity_lines::flip_and_burn_line;

        c {
            /// The relativistic Doppler shift of a source moving at β, seen at an angle, as
            /// NUL-terminated UTF-8 one line in a caller-owned buffer.
            ///
            /// A cosine of +1 is a source coming straight at the observer, −1 going straight away,
            /// 0 transverse in the observer's frame. At β = 0.6 head-on the frequency is doubled,
            /// receding it is halved, and across the line of sight it is 4/5. A β that is not
            /// finite or whose magnitude is 1 or more, or a cosine that is not finite or whose
            /// magnitude is above 1, is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `beta` is the source's speed as a fraction of the speed of light. `cos_theta` is the
            /// cosine of the angle between the source's velocity and the direction from source to
            /// observer, in the observer's frame.
            ///
            /// Tab-separated: beta, cos theta, factor, redshift, head-on factor, transverse factor,
            /// source.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// The relativistic Doppler shift of a source moving at β, seen at an angle, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// A cosine of +1 is a source coming straight at the observer, −1 going straight away,
            /// 0 transverse in the observer's frame. At β = 0.6 head-on the frequency is doubled,
            /// receding it is halved, and across the line of sight it is 4/5. A β that is not
            /// finite or whose magnitude is 1 or more, or a cosine that is not finite or whose
            /// magnitude is above 1, is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `beta` is the source's speed as a fraction of the speed of light. `cos_theta` is the
            /// cosine of the angle between the source's velocity and the direction from source to
            /// observer, in the observer's frame.
            ///
            /// Tab-separated: beta, cos theta, factor, redshift, head-on factor, transverse factor,
            /// source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_doppler(beta: f64, cos_theta: f64) -> line =
            $crate::relativity_lines::doppler_line;

        c {
            /// The composition of two collinear velocities, as NUL-terminated UTF-8 one line in a
            /// caller-owned buffer.
            ///
            /// Both velocities are fractions of the speed of light, positive one way and negative
            /// the other; they do not add, their rapidities do. Two ships at 0.999 compose to 0.999
            /// 999 5, and `1 − β` of that, 5.005·10⁻⁷, is carried without the cancellation that
            /// would lose eight of its figures. A β that is not finite or whose magnitude is 1 or
            /// more is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// Tab-separated: first beta, second beta, composed beta, composed speed, one minus
            /// composed beta, first rapidity, second rapidity, composed rapidity, composed lorentz
            /// factor, source.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// The composition of two collinear velocities, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// Both velocities are fractions of the speed of light, positive one way and negative
            /// the other; they do not add, their rapidities do. Two ships at 0.999 compose to 0.999
            /// 999 5, and `1 − β` of that, 5.005·10⁻⁷, is carried without the cancellation that
            /// would lose eight of its figures. A β that is not finite or whose magnitude is 1 or
            /// more is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// Tab-separated: first beta, second beta, composed beta, composed speed, one minus
            /// composed beta, first rapidity, second rapidity, composed rapidity, composed lorentz
            /// factor, source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_velocity_add(first_beta: f64, second_beta: f64) -> line =
            $crate::relativity_lines::velocity_add_line;

        c {
            /// The Schwarzschild radius of a body, as NUL-terminated UTF-8 one line in a
            /// caller-owned buffer.
            ///
            /// The radius is 2GM/c² from the body's standard gravitational parameter, which the
            /// table carries more exactly than its mass. `body` is an identifier
            /// `hc_gravitating_bodies` lists, in any ASCII case; another, a name such as
            /// `Sagittarius A*` included, is `HC_ERROR_UNKNOWN`.
            ///
            /// `body` is the body's identifier.
            ///
            /// Tab-separated: id, gm, gm constant, schwarzschild radius, constants, source.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The Schwarzschild radius of a body, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// The radius is 2GM/c² from the body's standard gravitational parameter, which the
            /// table carries more exactly than its mass. `body` is an identifier
            /// `hc_gravitating_bodies` lists, in any ASCII case; another, a name such as
            /// `Sagittarius A*` included, is `HC_ERR_UNKNOWN`.
            ///
            /// `body` is the body's identifier.
            ///
            /// Tab-separated: id, gm, gm constant, schwarzschild radius, constants, source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_schwarzschild_radius(body: name(body_len)) -> line =
            $crate::relativity_lines::schwarzschild_radius_line;

        c {
            /// A clock moving at a constant speed that is not exactly known, as NUL-terminated
            /// UTF-8 one line in a caller-owned buffer.
            ///
            /// The standard deviation of the proper time is t β σ_β / √(1 − β²), first order, so a
            /// speed known to a metre a second near the speed of light is an error of seconds a
            /// year. A speed at or beyond the speed of light either way, a standard deviation that
            /// is not finite or is negative, or a coordinate time that is not finite or whose
            /// proper time leaves the range of a duration, is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `speed_std_dev` is the speed's standard deviation, in metres per second.
            ///
            /// Tab-separated: beta, beta std dev, proper seconds, proper std dev, text, constants,
            /// source.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// A clock moving at a constant speed that is not exactly known, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// The standard deviation of the proper time is t β σ_β / √(1 − β²), first order, so a
            /// speed known to a metre a second near the speed of light is an error of seconds a
            /// year. A speed at or beyond the speed of light either way, a standard deviation that
            /// is not finite or is negative, or a coordinate time that is not finite or whose
            /// proper time leaves the range of a duration, is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `speed_std_dev` is the speed's standard deviation, in metres per second.
            ///
            /// Tab-separated: beta, beta std dev, proper seconds, proper std dev, text, constants,
            /// source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_proper_time_uncertain(
            speed_metres_per_second: f64,
            speed_std_dev: f64,
            coordinate_seconds: f64,
        ) -> line =
            $crate::relativity_lines::proper_time_uncertain_line;
    } };
    ("uncertainty", $backend:ident) => { $backend! {
        c {
            /// An ISO 8601-2 value placed on the timeline, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The first line is the value itself and the lines after it are the parts it is made
            /// of: the two sides of an interval, or each member of a `[...]` or `{...}` set. The
            /// first cell is the role: `value`, `start`, `end` or `member`. A set is placed by the
            /// hull of its members, which is wider than the set: `[1667,1670]` is somewhere from
            /// 1667 to the end of 1670, and the gap is lost. Times of day, seasons, sub-year
            /// divisions, component-level qualifiers and exponential years are refused rather than
            /// half-read. Text that is not a supported EDTF value is `HC_ERROR_MALFORMED`, and a
            /// year past what a fixed day can count is `HC_ERROR_OVERFLOW`.
            ///
            /// `text` is an ISO 8601-2 Extended Date/Time Format value.
            ///
            /// One line each, the cells tab-separated: role, kind, text, precision, qualifier, long
            /// form, first day, last day, support, estimate, span days.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// An ISO 8601-2 value placed on the timeline, as UTF-8 lines, returning the byte
            /// length written.
            ///
            /// The first line is the value itself and the lines after it are the parts it is made
            /// of: the two sides of an interval, or each member of a `[...]` or `{...}` set. The
            /// first cell is the role: `value`, `start`, `end` or `member`. A set is placed by the
            /// hull of its members, which is wider than the set: `[1667,1670]` is somewhere from
            /// 1667 to the end of 1670, and the gap is lost. Times of day, seasons, sub-year
            /// divisions, component-level qualifiers and exponential years are refused rather than
            /// half-read. Text that is not a supported EDTF value is `HC_ERR_MALFORMED`, and a year
            /// past what a fixed day can count is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `text` is an ISO 8601-2 Extended Date/Time Format value.
            ///
            /// One line each, the cells tab-separated: role, kind, text, precision, qualifier, long
            /// form, first day, last day, support, estimate, span days.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_edtf_parse(text: name(text_len)) -> line =
            $crate::uncertainty_lines::edtf_lines;

        c {
            /// What can hold between two EDTF values placed on the timeline, as NUL-terminated
            /// UTF-8 one line in a caller-owned buffer.
            ///
            /// Allen's thirteen relations between intervals are tested over the two supports: where
            /// a bound is unknown, every ordering it could have is considered, so the set is what
            /// remains possible, never a guess. A date known to the year is the year; `1984~` is
            /// widened by its own length on each side, 1982-12-31 to 1986-01-02; an open interval
            /// has no bound on its open side. Text that is not a supported EDTF value is
            /// `HC_ERROR_MALFORMED`, and a year past what a fixed day can count is
            /// `HC_ERROR_OVERFLOW`.
            ///
            /// Tab-separated: relations, symbols, definitely before, possibly before, definitely
            /// after, possibly after, possibly concurrent.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// What can hold between two EDTF values placed on the timeline, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Allen's thirteen relations between intervals are tested over the two supports: where
            /// a bound is unknown, every ordering it could have is considered, so the set is what
            /// remains possible, never a guess. A date known to the year is the year; `1984~` is
            /// widened by its own length on each side, 1982-12-31 to 1986-01-02; an open interval
            /// has no bound on its open side. Text that is not a supported EDTF value is
            /// `HC_ERR_MALFORMED`, and a year past what a fixed day can count is
            /// `HC_ERR_OUT_OF_RANGE`.
            ///
            /// Tab-separated: relations, symbols, definitely before, possibly before, definitely
            /// after, possibly after, possibly concurrent.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_edtf_relations(first: name(first_len), second: name(second_len)) -> line =
            $crate::uncertainty_lines::edtf_relations_line;

        c {
            /// A number with a count of significant figures, as NUL-terminated UTF-8 one line in a
            /// caller-owned buffer.
            ///
            /// 17 figures is the most a double holds and means every digit is claimed, as for a
            /// count or a definition; the shortest numeral that reads back as the same double is
            /// printed without padding. A number that is not finite, or a count of figures that is
            /// not from 1 to 17, is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `figures` is 1 to 17.
            ///
            /// Tab-separated: value, figures, text, rounded, exponent, last place.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// A number with a count of significant figures, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// 17 figures is the most a double holds and means every digit is claimed, as for a
            /// count or a definition; the shortest numeral that reads back as the same double is
            /// printed without padding. A number that is not finite, or a count of figures that is
            /// not from 1 to 17, is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `figures` is 1 to 17.
            ///
            /// Tab-separated: value, figures, text, rounded, exponent, last place.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_significant(value: f64, figures: u32) -> line =
            $crate::uncertainty_lines::significant_line;

        c {
            /// Arithmetic on two numbers with figure counts, as NUL-terminated UTF-8 one line in a
            /// caller-owned buffer.
            ///
            /// `add` and `sub` are significant down to the coarser of the two last places, so
            /// `100.0 + 0.001` keeps four figures and `1.0000 − 0.9999` keeps one; `mul` and `div`
            /// carry the smaller figure count; `pow` raises the first number to the second as an
            /// integer and keeps the first's count, the second's figures being ignored. An
            /// operation that is not one of these is `HC_ERROR_UNKNOWN`. A number that is not
            /// finite, a count of figures that is not from 1 to 17, a division by zero, a power
            /// whose exponent is not an integer within an `i32`, or a result that leaves the range
            /// of a double, is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `operation` is `add`, `sub`, `mul`, `div` or `pow`.
            ///
            /// Tab-separated: operation, text, value, figures, exponent, last place.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Arithmetic on two numbers with figure counts, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// `add` and `sub` are significant down to the coarser of the two last places, so
            /// `100.0 + 0.001` keeps four figures and `1.0000 − 0.9999` keeps one; `mul` and `div`
            /// carry the smaller figure count; `pow` raises the first number to the second as an
            /// integer and keeps the first's count, the second's figures being ignored. An
            /// operation that is not one of these is `HC_ERR_UNKNOWN`. A number that is not finite,
            /// a count of figures that is not from 1 to 17, a division by zero, a power whose
            /// exponent is not an integer within an `i32`, or a result that leaves the range of a
            /// double, is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `operation` is `add`, `sub`, `mul`, `div` or `pow`.
            ///
            /// Tab-separated: operation, text, value, figures, exponent, last place.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_significant_op(
            operation: name(operation_len),
            first: f64,
            first_figures: u32,
            second: f64,
            second_figures: u32,
        ) -> line =
            $crate::uncertainty_lines::significant_op_line;

        c {
            /// A Gaussian quantity, `value ± σ`, as NUL-terminated UTF-8 one line in a caller-owned
            /// buffer.
            ///
            /// A standard deviation of 0 is an exact value, and its significant text is the
            /// shortest numeral that reads back as the same double. A value or a standard deviation
            /// that is not finite, or a negative standard deviation, is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// Tab-separated: value, std dev, text, significant, relative, low 1σ, high 1σ, low 2σ,
            /// high 2σ, low 3σ, high 3σ.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// A Gaussian quantity, `value ± σ`, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// A standard deviation of 0 is an exact value, and its significant text is the
            /// shortest numeral that reads back as the same double. A value or a standard deviation
            /// that is not finite, or a negative standard deviation, is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// Tab-separated: value, std dev, text, significant, relative, low 1σ, high 1σ, low 2σ,
            /// high 2σ, low 3σ, high 3σ.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_uncertain(value: f64, std_dev: f64) -> line =
            $crate::uncertainty_lines::uncertain_line;

        c {
            /// Arithmetic on Gaussian quantities, with the errors propagated to first order, as
            /// NUL-terminated UTF-8 one line in a caller-owned buffer.
            ///
            /// `add`, `sub`, `mul` and `div` are of independent quantities, with the errors
            /// combined in quadrature; `combine` is the inverse-variance weighted mean of two
            /// measurements of one quantity, where an exact one wins outright; `z-score` is how
            /// many combined standard deviations separate the two; `scale` multiplies the first by
            /// the second's value as an exact factor, its standard deviation ignored; `pow` raises
            /// the first to the second's value as an exact exponent; `ln` and `exp` are of the
            /// first, the second ignored. An operation that is not one of these is
            /// `HC_ERROR_UNKNOWN`. A value or standard deviation that is not finite, a negative
            /// standard deviation, a division by a value of 0, a logarithm of a value that is not
            /// positive, a power outside the real numbers, two exact values that disagree under
            /// `combine`, a z-score of two exact values, or a result that leaves the range of a
            /// double, is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `operation` is `add`, `sub`, `mul`, `div`, `combine`, `z-score`, `scale`, `pow`,
            /// `ln` or `exp`.
            ///
            /// Tab-separated: operation, value, std dev, text, significant.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Arithmetic on Gaussian quantities, with the errors propagated to first order, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// `add`, `sub`, `mul` and `div` are of independent quantities, with the errors
            /// combined in quadrature; `combine` is the inverse-variance weighted mean of two
            /// measurements of one quantity, where an exact one wins outright; `z-score` is how
            /// many combined standard deviations separate the two; `scale` multiplies the first by
            /// the second's value as an exact factor, its standard deviation ignored; `pow` raises
            /// the first to the second's value as an exact exponent; `ln` and `exp` are of the
            /// first, the second ignored. An operation that is not one of these is
            /// `HC_ERR_UNKNOWN`. A value or standard deviation that is not finite, a negative
            /// standard deviation, a division by a value of 0, a logarithm of a value that is not
            /// positive, a power outside the real numbers, two exact values that disagree under
            /// `combine`, a z-score of two exact values, or a result that leaves the range of a
            /// double, is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `operation` is `add`, `sub`, `mul`, `div`, `combine`, `z-score`, `scale`, `pow`,
            /// `ln` or `exp`.
            ///
            /// Tab-separated: operation, value, std dev, text, significant.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_uncertain_op(
            operation: name(operation_len),
            first: f64,
            first_std_dev: f64,
            second: f64,
            second_std_dev: f64,
        ) -> line =
            $crate::uncertainty_lines::uncertain_op_line;

        c {
            /// Arithmetic on intervals of time, as NUL-terminated UTF-8 one line in a caller-owned
            /// buffer.
            ///
            /// The two intervals are `[first_low_seconds, first_high_seconds]` and
            /// `[second_low_seconds, second_high_seconds]` in whole seconds. `add` and `sub` (the
            /// crossed bounds: `[lo₁ − hi₂, hi₁ − lo₂]`), `intersect` (empty when they do not meet)
            /// and `hull` (the smallest interval holding both) write an interval, as whole seconds
            /// and attoseconds each; `overlaps` and `contains` (whether the first holds the second)
            /// answer a question, and write only the last cell. The width is the high bound minus
            /// the low, and the midpoint is floored to an attosecond. An empty interval has no
            /// bounds, width or midpoint. An operation that is not one of these is
            /// `HC_ERROR_UNKNOWN`; an interval whose low bound is above its high bound, which is
            /// the empty interval written backwards, is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `operation` is `add`, `sub`, `intersect`, `hull`, `overlaps` or `contains`.
            ///
            /// Tab-separated: operation, empty, low seconds, low attoseconds, high seconds, high
            /// attoseconds, width seconds, width attoseconds, midpoint seconds, midpoint
            /// attoseconds, holds.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Arithmetic on intervals of time, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// The two intervals are `[first_low_seconds, first_high_seconds]` and
            /// `[second_low_seconds, second_high_seconds]` in whole seconds. `add` and `sub` (the
            /// crossed bounds: `[lo₁ − hi₂, hi₁ − lo₂]`), `intersect` (empty when they do not meet)
            /// and `hull` (the smallest interval holding both) write an interval, as whole seconds
            /// and attoseconds each; `overlaps` and `contains` (whether the first holds the second)
            /// answer a question, and write only the last cell. The width is the high bound minus
            /// the low, and the midpoint is floored to an attosecond. An empty interval has no
            /// bounds, width or midpoint. An operation that is not one of these is
            /// `HC_ERR_UNKNOWN`; an interval whose low bound is above its high bound, which is the
            /// empty interval written backwards, is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `operation` is `add`, `sub`, `intersect`, `hull`, `overlaps` or `contains`.
            ///
            /// Tab-separated: operation, empty, low seconds, low attoseconds, high seconds, high
            /// attoseconds, width seconds, width attoseconds, midpoint seconds, midpoint
            /// attoseconds, holds.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_interval(
            operation: name(operation_len),
            first_low_seconds: i64,
            first_high_seconds: i64,
            second_low_seconds: i64,
            second_high_seconds: i64,
        ) -> line =
            $crate::uncertainty_lines::interval_line;
    } };
    ("units", $backend:ident) => { $backend! {
        c {
            /// Every unit of time with an exactly defined length, as NUL-terminated UTF-8 lines in
            /// a caller-owned buffer.
            ///
            /// One line each, shortest first. A length is a numerator and a denominator in lowest
            /// terms, written in decimal, because they are 128-bit integers and a quectosecond is
            /// 10⁻³⁰ of a second: no double holds that, nor a flick's 705 600 000th. A unit that is
            /// measured rather than defined, such as the sidereal day or the tropical year, is not
            /// here: it lives with the model that measured it.
            ///
            /// One line each, the cells tab-separated: id, name, symbol, seconds numerator, seconds
            /// denominator, family, authority.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// Every unit of time with an exactly defined length, as UTF-8 lines, returning the
            /// byte length written.
            ///
            /// One line each, shortest first. A length is a numerator and a denominator in lowest
            /// terms, written in decimal, because they are 128-bit integers and a quectosecond is
            /// 10⁻³⁰ of a second: no double holds that, nor a flick's 705 600 000th. A unit that is
            /// measured rather than defined, such as the sidereal day or the tropical year, is not
            /// here: it lives with the model that measured it.
            ///
            /// One line each, the cells tab-separated: id, name, symbol, seconds numerator, seconds
            /// denominator, family, authority.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_units() -> line =
            || Ok($crate::units_lines::units_lines());

        c {
            /// A count of one unit of time written in another, exactly, as NUL-terminated UTF-8 one
            /// line in a caller-owned buffer.
            ///
            /// The count is `count_numerator / count_denominator`, and may be negative. Both counts
            /// are written as a numerator and a denominator in lowest terms. A unit that `hc_units`
            /// does not list is `HC_ERROR_UNKNOWN`; a denominator of 0 is `HC_ERROR_OUT_OF_RANGE`;
            /// and a count or a length whose numerator or denominator leaves 128 bits is
            /// `HC_ERROR_OVERFLOW`.
            ///
            /// `from` is the unit of the count: an identifier `hc_units` lists. `to` is the unit to
            /// write it in.
            ///
            /// Tab-separated: from, to, count numerator, count denominator, converted numerator,
            /// converted denominator, whole, seconds numerator, seconds denominator, attosecond
            /// exact.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A count of one unit of time written in another, exactly, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// The count is `count_numerator / count_denominator`, and may be negative. Both counts
            /// are written as a numerator and a denominator in lowest terms. A unit that `hc_units`
            /// does not list is `HC_ERR_UNKNOWN`; a denominator of 0 is `HC_ERR_OUT_OF_RANGE`; and
            /// a count or a length whose numerator or denominator leaves 128 bits is
            /// `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `from` is the unit of the count: an identifier `hc_units` lists. `to` is the unit to
            /// write it in.
            ///
            /// Tab-separated: from, to, count numerator, count denominator, converted numerator,
            /// converted denominator, whole, seconds numerator, seconds denominator, attosecond
            /// exact.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_unit_convert(
            count_numerator: i64,
            count_denominator: i64,
            from: name(from_len),
            to: name(to_len),
        ) -> line =
            $crate::units_lines::unit_convert_line;

        c {
            /// Every frame rate and sample rate the crate carries as an exact period, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The NTSC rates are exact: 29.97 is 30 000 / 1 001. Each identifier is what
            /// `hc_frame_period` reads for its rate.
            ///
            /// One line each, the cells tab-separated: id, kind, hertz numerator, hertz
            /// denominator.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// Every frame rate and sample rate the crate carries as an exact period, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// The NTSC rates are exact: 29.97 is 30 000 / 1 001. Each identifier is what
            /// `hc_frame_period` reads for its rate.
            ///
            /// One line each, the cells tab-separated: id, kind, hertz numerator, hertz
            /// denominator.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_rates() -> line =
            || Ok($crate::units_lines::rates_lines());

        c {
            /// The length of one frame or one sample, exactly, as NUL-terminated UTF-8 one line in
            /// a caller-owned buffer.
            ///
            /// `rate` is an identifier of `hc_rates`, such as `29.97` or `48000`, or an exact rate
            /// written `n` or `n/d` events per second, such as `30000/1001`. A rate that is
            /// neither, or a unit `hc_units` does not list, is `HC_ERROR_UNKNOWN`; a rate that is
            /// not positive, or a denominator of 0, is `HC_ERROR_OUT_OF_RANGE`; and a length whose
            /// numerator or denominator leaves 128 bits is `HC_ERROR_OVERFLOW`.
            ///
            /// `rate` is an identifier `hc_rates` lists, or an exact rate written `n` or `n/d`
            /// events per second. `in_unit` is the unit to count the length in.
            ///
            /// Tab-separated: rate, kind, hertz numerator, hertz denominator, period numerator,
            /// period denominator, count numerator, count denominator, unit, whole, whole flicks.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The length of one frame or one sample, exactly, as one UTF-8 line, returning the
            /// byte length written.
            ///
            /// `rate` is an identifier of `hc_rates`, such as `29.97` or `48000`, or an exact rate
            /// written `n` or `n/d` events per second, such as `30000/1001`. A rate that is
            /// neither, or a unit `hc_units` does not list, is `HC_ERR_UNKNOWN`; a rate that is not
            /// positive, or a denominator of 0, is `HC_ERR_OUT_OF_RANGE`; and a length whose
            /// numerator or denominator leaves 128 bits is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `rate` is an identifier `hc_rates` lists, or an exact rate written `n` or `n/d`
            /// events per second. `in_unit` is the unit to count the length in.
            ///
            /// Tab-separated: rate, kind, hertz numerator, hertz denominator, period numerator,
            /// period denominator, count numerator, count denominator, unit, whole, whole flicks.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_frame_period(rate: name(rate_len), in_unit: name(in_unit_len)) -> line =
            $crate::units_lines::frame_period_line;

        c {
            /// A note at a tempo, exactly, as NUL-terminated UTF-8 one line in a caller-owned
            /// buffer.
            ///
            /// The tempo is `bpm_numerator / bpm_denominator` beats per minute, where a beat is the
            /// note of `beat_halvings` halvings of a whole note. The note is `note_halvings`
            /// halvings, with `dots` augmentation dots, each adding half of what came before (one
            /// dot makes it 3/2 as long, two 7/4), and, when `tuplet_count` and `tuplet_space` are
            /// both above 0, `tuplet_count` of it in the time of `tuplet_space`: a triplet is 3 in
            /// the time of 2. Both 0 is no tuplet. A tempo that is not positive, a denominator of
            /// 0, a tuplet with only one of its two numbers 0, or halvings or dots that do not fit
            /// a byte, is `HC_ERROR_OUT_OF_RANGE`; a length whose numerator or denominator leaves
            /// 128 bits, or halvings or dots of 127 or more, is `HC_ERROR_OVERFLOW`.
            ///
            /// `note_halvings` is 0 whole, 1 half, 2 quarter, 3 eighth. `dots` is augmentation
            /// dots. `tuplet_space` is the tuplet's "in the time of" number; 0 for none.
            /// `tuplet_count` is the tuplet's count: 3 for a triplet; 0 for none. `beat_halvings`
            /// is the note the beat is counted in: 2 for a quarter.
            ///
            /// Tab-separated: bpm numerator, bpm denominator, beat numerator, beat denominator,
            /// note fraction numerator, note fraction denominator, note numerator, note
            /// denominator, midi microseconds, midi exact.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// A note at a tempo, exactly, as one UTF-8 line, returning the byte length written.
            ///
            /// The tempo is `bpm_numerator / bpm_denominator` beats per minute, where a beat is the
            /// note of `beat_halvings` halvings of a whole note. The note is `note_halvings`
            /// halvings, with `dots` augmentation dots, each adding half of what came before (one
            /// dot makes it 3/2 as long, two 7/4), and, when `tuplet_count` and `tuplet_space` are
            /// both above 0, `tuplet_count` of it in the time of `tuplet_space`: a triplet is 3 in
            /// the time of 2. Both 0 is no tuplet. A tempo that is not positive, a denominator of
            /// 0, a tuplet with only one of its two numbers 0, or halvings or dots that do not fit
            /// a byte, is `HC_ERR_OUT_OF_RANGE`; a length whose numerator or denominator leaves 128
            /// bits, or halvings or dots of 127 or more, is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `note_halvings` is 0 whole, 1 half, 2 quarter, 3 eighth. `dots` is augmentation
            /// dots. `tuplet_space` is the tuplet's "in the time of" number; 0 for none.
            /// `tuplet_count` is the tuplet's count: 3 for a triplet; 0 for none. `beat_halvings`
            /// is the note the beat is counted in: 2 for a quarter.
            ///
            /// Tab-separated: bpm numerator, bpm denominator, beat numerator, beat denominator,
            /// note fraction numerator, note fraction denominator, note numerator, note
            /// denominator, midi microseconds, midi exact.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_tempo(
            bpm_numerator: i64,
            bpm_denominator: i64,
            note_halvings: u32,
            dots: u32,
            tuplet_space: u32,
            tuplet_count: u32,
            beat_halvings: u32,
        ) -> line =
            $crate::units_lines::tempo_line;
    } };
    ("fiscal", $backend:ident) => { $backend! {
        c {
            /// Every fiscal, tax and academic year system the crate carries, country by country, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// A validity bound is a label of the system's own calendar: Iran's are Solar Hijri
            /// years and Nepal's Bikram Sambat. `valid from` is the year the system was
            /// established, before which it is absent; `read from` is the first year the sources
            /// read reach, and every label of the system between the two, or inside one of the
            /// `unread` spans (`first-last`, separated by `;`), is a gap and not an answer. The
            /// authority `unread` is a page read that states the year with the instrument that
            /// fixes it not read. Nepal is in a build that has the `calendars` layer
            /// too, since its year starts on 1 Shrawan of the Bikram Sambat; in a build without it
            /// the country is absent, which `hc_fiscal_year_on` and `hc_fiscal_year_span` report as
            /// `HC_ERROR_NO_DATA`, where a code no table has is `HC_ERROR_UNKNOWN`. No
            /// label convention is a default: the year is named for the year it starts in, or for
            /// the one it ends in, and every line says which.
            ///
            /// One line each, the cells tab-separated: country, country name, table, kind, name,
            /// local name, authority, national, start calendar, start month, start day, label
            /// convention, valid from, valid until, approximate, note, sources checked, sources, read
            /// from, unread.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// Every fiscal, tax and academic year system the crate carries, country by country, as
            /// UTF-8 lines, returning the byte length written.
            ///
            /// A validity bound is a label of the system's own calendar: Iran's are Solar Hijri
            /// years and Nepal's Bikram Sambat. `valid from` is the year the system was
            /// established, before which it is absent; `read from` is the first year the sources
            /// read reach, and every label of the system between the two, or inside one of the
            /// `unread` spans (`first-last`, separated by `;`), is a gap and not an answer. The
            /// authority `unread` is a page read that states the year with the instrument that
            /// fixes it not read. Nepal is in a build that has the `calendars` layer
            /// too, since its year starts on 1 Shrawan of the Bikram Sambat; in a build without it
            /// the country is absent, which `hc_fiscal_year_on` and `hc_fiscal_year_span` report as
            /// `HC_ERR_NO_DATA`, where a code no table has is `HC_ERR_UNKNOWN`. No
            /// label convention is a default: the year is named for the year it starts in, or for
            /// the one it ends in, and every line says which.
            ///
            /// One line each, the cells tab-separated: country, country name, table, kind, name,
            /// local name, authority, national, start calendar, start month, start day, label
            /// convention, valid from, valid until, approximate, note, sources checked, sources, read
            /// from, unread.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_fiscal_profiles() -> line =
            || Ok($crate::fiscal_lines::profiles_lines());

        c {
            /// What the year systems of a country say a fixed day is, as NUL-terminated UTF-8 lines
            /// in a caller-owned buffer.
            ///
            /// One line each. The status is `in-force`, or `outside-validity` where the system was
            /// not in force in the year the day falls in (the United States' October year had not
            /// begun in 1970), or `gap` where it was in force and the sources read do not reach
            /// that year (the `read from` and `unread` cells of `hc_fiscal_profiles`), or
            /// `outside-calendar-range` where the start's calendar does not reach the day; the
            /// cells after the status are then empty. A country the tables do
            /// not carry, or a kind that is not one of the four, is `HC_ERROR_UNKNOWN`; a country
            /// that has no system of the kind asked is `HC_ERROR_NO_DATA`; a fixed day beyond the
            /// Gregorian years ±9 999 999 is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `country` is an ISO 3166-1 alpha-2 code, in any case. `kind` is a kind
            /// `hc_fiscal_profiles` writes, or empty for every kind the country has.
            ///
            /// One line each, the cells tab-separated: country, table, kind, name, status, label,
            /// first, last, day of year, days in year, weekday, month, quarter, half, start
            /// calendar, label convention, approximate, sources checked.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// What the year systems of a country say a fixed day is, as UTF-8 lines, returning the
            /// byte length written.
            ///
            /// One line each. The status is `in-force`, or `outside-validity` where the system was
            /// not in force in the year the day falls in (the United States' October year had not
            /// begun in 1970), or `gap` where it was in force and the sources read do not reach
            /// that year (the `read from` and `unread` cells of `hc_fiscal_profiles`), or
            /// `outside-calendar-range` where the start's calendar does not reach the day; the
            /// cells after the status are then empty. A country the tables do
            /// not carry, or a kind that is not one of the four, is `HC_ERR_UNKNOWN`; a country
            /// that has no system of the kind asked is `HC_ERR_NO_DATA`; a fixed day beyond the
            /// Gregorian years ±9 999 999 is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `country` is an ISO 3166-1 alpha-2 code, in any case. `kind` is a kind
            /// `hc_fiscal_profiles` writes, or empty for every kind the country has.
            ///
            /// One line each, the cells tab-separated: country, table, kind, name, status, label,
            /// first, last, day of year, days in year, weekday, month, quarter, half, start
            /// calendar, label convention, approximate, sources checked.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_fiscal_year_on(
            country: name(country_len),
            kind: text(kind_len),
            fixed: i64,
        ) -> line =
            $crate::fiscal_lines::year_on_lines;

        c {
            /// The span of the year a label names in each year system of a country, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// One line each. The label is the system's own: a label of Iran's is a Solar Hijri
            /// year, a label of Japan's 年度 the Gregorian year it begins in, and a label of the
            /// United States' fiscal year the one it ends in. The status is `in-force`,
            /// `outside-validity` where the system was not in force in that year, `gap` where it
            /// was and the sources read do not reach that year (the cells after the label are
            /// empty in both) or `outside-calendar-range` where the start's calendar does not
            /// reach it. A country the tables do not carry, or a kind that is not one of
            /// the four, is `HC_ERROR_UNKNOWN`; a country with no system of the kind asked is
            /// `HC_ERROR_NO_DATA`.
            ///
            /// `country` is an ISO 3166-1 alpha-2 code, in any case. `kind` is a kind
            /// `hc_fiscal_profiles` writes, or empty for every kind the country has. `label` is a
            /// year label of the system's own calendar and convention.
            ///
            /// One line each, the cells tab-separated: country, table, kind, name, status, label,
            /// first, last, days.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The span of the year a label names in each year system of a country, as UTF-8 lines,
            /// returning the byte length written.
            ///
            /// One line each. The label is the system's own: a label of Iran's is a Solar Hijri
            /// year, a label of Japan's 年度 the Gregorian year it begins in, and a label of the
            /// United States' fiscal year the one it ends in. The status is `in-force`,
            /// `outside-validity` where the system was not in force in that year, `gap` where it
            /// was and the sources read do not reach that year (the cells after the label are
            /// empty in both) or `outside-calendar-range` where the start's calendar does not
            /// reach it. A country the tables do not carry, or a kind that is not one of
            /// the four, is `HC_ERR_UNKNOWN`; a country with no system of the kind asked is
            /// `HC_ERR_NO_DATA`.
            ///
            /// `country` is an ISO 3166-1 alpha-2 code, in any case. `kind` is a kind
            /// `hc_fiscal_profiles` writes, or empty for every kind the country has. `label` is a
            /// year label of the system's own calendar and convention.
            ///
            /// One line each, the cells tab-separated: country, table, kind, name, status, label,
            /// first, last, days.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_fiscal_year_span(
            country: name(country_len),
            kind: text(kind_len),
            label: i64,
        ) -> line =
            $crate::fiscal_lines::year_span_lines;

        c {
            /// Every named year of whole weeks, as NUL-terminated UTF-8 lines in a caller-owned
            /// buffer.
            ///
            /// The NRF 4-5-4 retail calendar, ISO 8601's week-numbering year and a 4-4-5 year
            /// ending the last Saturday of December. The two anchor rules are two names and not one
            /// parameter: they put the year end up to a week apart and sometimes in different
            /// months.
            ///
            /// One line each, the cells tab-separated: id, name, weekday, month, anchor rule, label
            /// convention, shape, note, source, sources checked.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// Every named year of whole weeks, as UTF-8 lines, returning the byte length written.
            ///
            /// The NRF 4-5-4 retail calendar, ISO 8601's week-numbering year and a 4-4-5 year
            /// ending the last Saturday of December. The two anchor rules are two names and not one
            /// parameter: they put the year end up to a week apart and sometimes in different
            /// months.
            ///
            /// One line each, the cells tab-separated: id, name, weekday, month, anchor rule, label
            /// convention, shape, note, source, sources checked.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_week_year_systems() -> line =
            || Ok($crate::fiscal_lines::week_year_systems_lines());

        c {
            /// Where a fixed day is in a year of whole weeks, as NUL-terminated UTF-8 one line in a
            /// caller-owned buffer.
            ///
            /// In a 53-week year the extra week is the last period's. A system
            /// `hc_week_year_systems` does not list is `HC_ERROR_UNKNOWN`, and a day beyond the
            /// Gregorian years ±9 999 999 is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `system` is an identifier `hc_week_year_systems` lists.
            ///
            /// Tab-separated: system, name, label, first, last, weeks, long, week, week first, week
            /// last, period, period first, period last, quarter.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// Where a fixed day is in a year of whole weeks, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// In a 53-week year the extra week is the last period's. A system
            /// `hc_week_year_systems` does not list is `HC_ERR_UNKNOWN`, and a day beyond the
            /// Gregorian years ±9 999 999 is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `system` is an identifier `hc_week_year_systems` lists.
            ///
            /// Tab-separated: system, name, label, first, last, weeks, long, week, week first, week
            /// last, period, period first, period last, quarter.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_week_year_on(system: name(system_len), fixed: i64) -> line =
            $crate::fiscal_lines::week_year_on_line;
    } };
    ("name-days", $backend:ident) => { $backend! {
        c {
            /// Every name-day list the crate ships and every country it declines to ship one for,
            /// as NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// Each list is a named edition of a named authority, and the crate reports what the
            /// lists say and asserts none of them. A gap is a country whose list the crate declines
            /// to ship, with the reason in its words: a list a university sells by the copy, a
            /// church calendar that names saints and not given names, several published lists that
            /// no body chooses between. A gap's identifier is the country's code, or two codes
            /// joined by a hyphen where one reasoning covers both; `hc_name_days_on` and
            /// `hc_name_day` read a code of either. A column that does not apply to the kind is
            /// empty.
            ///
            /// One line each, the cells tab-separated: kind, id, country, language, name,
            /// authority, decided, provenance, valid from, valid until, licence, leap day, total
            /// names, source, retrieved, reason, explanation.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// Every name-day list the crate ships and every country it declines to ship one for,
            /// as UTF-8 lines, returning the byte length written.
            ///
            /// Each list is a named edition of a named authority, and the crate reports what the
            /// lists say and asserts none of them. A gap is a country whose list the crate declines
            /// to ship, with the reason in its words: a list a university sells by the copy, a
            /// church calendar that names saints and not given names, several published lists that
            /// no body chooses between. A gap's identifier is the country's code, or two codes
            /// joined by a hyphen where one reasoning covers both; `hc_name_days_on` and
            /// `hc_name_day` read a code of either. A column that does not apply to the kind is
            /// empty.
            ///
            /// One line each, the cells tab-separated: kind, id, country, language, name,
            /// authority, decided, provenance, valid from, valid until, licence, leap day, total
            /// names, source, retrieved, reason, explanation.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_name_day_lists() -> line =
            || Ok($crate::name_day_lines::lists_lines());

        c {
            /// What the lists of a country name on a day, as NUL-terminated UTF-8 lines in a
            /// caller-owned buffer.
            ///
            /// The kind is `list` for an edition in force in the year the day falls in (a country
            /// may keep two at once: Latvia's traditional and extended lists), `outside` for a
            /// shipped edition whose years do not include it, with no names, and `gap` for a
            /// country the crate carries no list for. A day no list has a name for is a `list` line
            /// with no names, and never the nearest edition's. A country the crate has neither a
            /// list nor a gap for is `HC_ERROR_UNKNOWN`; a day beyond the Gregorian years ±9 999
            /// 999 is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `country` is a two-letter code in any case.
            ///
            /// One line each, the cells tab-separated: kind, id, name, authority, valid from, valid
            /// until, licence, names, count, unlisted names day, notes, source, reason,
            /// explanation.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// What the lists of a country name on a day, as UTF-8 lines, returning the byte length
            /// written.
            ///
            /// The kind is `list` for an edition in force in the year the day falls in (a country
            /// may keep two at once: Latvia's traditional and extended lists), `outside` for a
            /// shipped edition whose years do not include it, with no names, and `gap` for a
            /// country the crate carries no list for. A day no list has a name for is a `list` line
            /// with no names, and never the nearest edition's. A country the crate has neither a
            /// list nor a gap for is `HC_ERR_UNKNOWN`; a day beyond the Gregorian years ±9 999 999
            /// is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `country` is a two-letter code in any case.
            ///
            /// One line each, the cells tab-separated: kind, id, name, authority, valid from, valid
            /// until, licence, names, count, unlisted names day, notes, source, reason,
            /// explanation.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_name_days_on(country: name(country_len), fixed: i64) -> line =
            $crate::name_day_lines::names_on_lines;

        c {
            /// The days of a year on which the lists of a country give a name, as NUL-terminated
            /// UTF-8 lines in a caller-owned buffer.
            ///
            /// The match is exact and case-sensitive: the list's own spelling, diacritics included,
            /// and no diminutive the authority did not print. A name may fall on several days (the
            /// extended Latvian list has some) or on none, which is a `list` line with no days. The
            /// kinds are those of `hc_name_days_on`: `outside` for an edition not in force in the
            /// year, with no days, and `gap`. A country the crate has neither a list nor a gap for
            /// is `HC_ERROR_UNKNOWN`; a year beyond the Gregorian years ±9 999 999 or past an `i32`
            /// is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `country` is a two-letter code in any case. `given_name` is the name, in the list's
            /// own spelling.
            ///
            /// One line each, the cells tab-separated: kind, id, name, authority, valid from, valid
            /// until, licence, dates, fixed days, count, source, reason, explanation.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// The days of a year on which the lists of a country give a name, as UTF-8 lines,
            /// returning the byte length written.
            ///
            /// The match is exact and case-sensitive: the list's own spelling, diacritics included,
            /// and no diminutive the authority did not print. A name may fall on several days (the
            /// extended Latvian list has some) or on none, which is a `list` line with no days. The
            /// kinds are those of `hc_name_days_on`: `outside` for an edition not in force in the
            /// year, with no days, and `gap`. A country the crate has neither a list nor a gap for
            /// is `HC_ERR_UNKNOWN`; a year beyond the Gregorian years ±9 999 999 or past an `i32`
            /// is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `country` is a two-letter code in any case. `given_name` is the name, in the list's
            /// own spelling.
            ///
            /// One line each, the cells tab-separated: kind, id, name, authority, valid from, valid
            /// until, licence, dates, fixed days, count, source, reason, explanation.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_name_day(
            country: name(country_len),
            given_name: name(given_name_len),
            year: i64,
        ) -> line =
            $crate::name_day_lines::days_of_lines;
    } };
    ("attributes", $backend:ident) => { $backend! {
        c {
            /// Every attribution list the crate ships, with what it declines to ship, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// There is no "the birthstone of March": there are eight lists, each with an authority,
            /// a date, a region and the years it was current, and they disagree in eleven months
            /// out of twelve. A gap is a subject the crate declined to ship, such as Japan's
            /// day-by-day 誕生花 or Robert Graves's "Celtic tree calendar"; its columns about a list
            /// are empty. A gap belongs to no subject here and is written only for the empty one. A
            /// subject that is not one of the seven is `HC_ERROR_UNKNOWN`.
            ///
            /// `subject` is `birthstone`, `birth-flower`, `moon-name`, `lunation-name`,
            /// `month-name`, `zodiac-stone` or `weekday`, or empty for every list and every gap.
            ///
            /// One line each, the cells tab-separated: kind, subject, id, name, body, region,
            /// region name, established, revised, valid from, valid until, provenance, key kind,
            /// source, caveat, reason, explanation.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// Every attribution list the crate ships, with what it declines to ship, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// There is no "the birthstone of March": there are eight lists, each with an authority,
            /// a date, a region and the years it was current, and they disagree in eleven months
            /// out of twelve. A gap is a subject the crate declined to ship, such as Japan's
            /// day-by-day 誕生花 or Robert Graves's "Celtic tree calendar"; its columns about a list
            /// are empty. A gap belongs to no subject here and is written only for the empty one. A
            /// subject that is not one of the seven is `HC_ERR_UNKNOWN`.
            ///
            /// `subject` is `birthstone`, `birth-flower`, `moon-name`, `lunation-name`,
            /// `month-name`, `zodiac-stone` or `weekday`, or empty for every list and every gap.
            ///
            /// One line each, the cells tab-separated: kind, subject, id, name, body, region,
            /// region name, established, revised, valid from, valid until, provenance, key kind,
            /// source, caveat, reason, explanation.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_attribution_authorities(subject: text(subject_len)) -> line =
            $crate::attribution_lines::authorities_lines;

        c {
            /// What every list of a subject attributes to one key, as NUL-terminated UTF-8 lines in
            /// a caller-owned buffer.
            ///
            /// One line per list. There is no line for "the" birthstone: a question about March has
            /// six answers, and the last cell says whether they agree. A contested list's caveat is
            /// on its line, and a caller who shows the answer should show it. A leap month is no
            /// key: no tradition attributes anything to an intercalary one. A subject that is not
            /// one of the seven is `HC_ERROR_UNKNOWN`, and a key outside its range is
            /// `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `subject` is one of the seven `hc_attribution_authorities` names. `key` is the month
            /// from 1, the lunation from 1, the sign from Aries = 1, or the ISO weekday from Monday
            /// = 1 to Sunday = 7.
            ///
            /// One line each, the cells tab-separated: subject, id, name, key, key kind,
            /// attributions, count, gloss, valid from, valid until, provenance, caveat, agreed.
            ///
            /// A null name is `HC_ERROR_NULL_POINTER`. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// What every list of a subject attributes to one key, as UTF-8 lines, returning the
            /// byte length written.
            ///
            /// One line per list. There is no line for "the" birthstone: a question about March has
            /// six answers, and the last cell says whether they agree. A contested list's caveat is
            /// on its line, and a caller who shows the answer should show it. A leap month is no
            /// key: no tradition attributes anything to an intercalary one. A subject that is not
            /// one of the seven is `HC_ERR_UNKNOWN`, and a key outside its range is
            /// `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `subject` is one of the seven `hc_attribution_authorities` names. `key` is the month
            /// from 1, the lunation from 1, the sign from Aries = 1, or the ISO weekday from Monday
            /// = 1 to Sunday = 7.
            ///
            /// One line each, the cells tab-separated: subject, id, name, key, key kind,
            /// attributions, count, gloss, valid from, valid until, provenance, caveat, agreed.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_attributions(subject: name(subject_len), key: i64) -> line =
            $crate::attribution_lines::attributions_lines;

        c {
            /// What every list attributes to the month, the weekday and the sign of a day, as
            /// NUL-terminated UTF-8 lines in a caller-owned buffer.
            ///
            /// The lines of `hc_attributions` for the birthstones, birth flowers, full-moon names,
            /// month names, zodiac stones and weekday attributions, each list on its own line, the
            /// subjects in that order; the lunation names are left out, because a day has no
            /// lunation number without the March equinox of its year. The month is the Gregorian
            /// month, the weekday its ISO weekday, and the sign the tropical sign the Sun is in at
            /// the day, judged at the meridian, which a day whose sign changes within about ten
            /// minutes of local midnight can move by a day. A meridian that is not read is
            /// `HC_ERROR_UNKNOWN`, and a day outside the years −1000 to 3000 is
            /// `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `meridian` is `universal`, `japan`, `china`, `korea`, `india`, `china-before-1929`,
            /// a longitude in degrees east, or empty for `universal`.
            ///
            /// One line each, the cells tab-separated: subject, id, name, key, key kind,
            /// attributions, count, gloss, valid from, valid until, provenance, caveat, agreed.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// What every list attributes to the month, the weekday and the sign of a day, as UTF-8
            /// lines, returning the byte length written.
            ///
            /// The lines of `hc_attributions` for the birthstones, birth flowers, full-moon names,
            /// month names, zodiac stones and weekday attributions, each list on its own line, the
            /// subjects in that order; the lunation names are left out, because a day has no
            /// lunation number without the March equinox of its year. The month is the Gregorian
            /// month, the weekday its ISO weekday, and the sign the tropical sign the Sun is in at
            /// the day, judged at the meridian, which a day whose sign changes within about ten
            /// minutes of local midnight can move by a day. A meridian that is not read is
            /// `HC_ERR_UNKNOWN`, and a day outside the years −1000 to 3000 is
            /// `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `meridian` is `universal`, `japan`, `china`, `korea`, `india`, `china-before-1929`,
            /// a longitude in degrees east, or empty for `universal`.
            ///
            /// One line each, the cells tab-separated: subject, id, name, key, key kind,
            /// attributions, count, gloss, valid from, valid until, provenance, caveat, agreed.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_attributions_on(fixed: i64, meridian: text(meridian_len)) -> line =
            $crate::attribution_lines::attributions_on_lines;

        c {
            /// The Harvest Moon of a year, as NUL-terminated UTF-8 one line in a caller-owned
            /// buffer.
            ///
            /// The Harvest Moon is the full moon nearest the September equinox, a rule and not a
            /// table row: it falls in September in about three years of four and in October in the
            /// rest. In 2025 it is 7 October, and September's full moon is the Corn Moon. The days
            /// are judged at the meridian, and a full moon within about a minute of a day's end can
            /// move by a day. A meridian that is not read is `HC_ERROR_UNKNOWN`, and a year outside
            /// −999 to 3000 is `HC_ERROR_OUT_OF_RANGE`.
            ///
            /// `meridian` is `universal`, `japan`, `china`, `korea`, `india`, `china-before-1929`,
            /// a longitude in degrees east, or empty for `universal`.
            ///
            /// Tab-separated: year, harvest moon, hunters moon, month, september moon, meridian,
            /// source.
            ///
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// The Harvest Moon of a year, as one UTF-8 line, returning the byte length written.
            ///
            /// The Harvest Moon is the full moon nearest the September equinox, a rule and not a
            /// table row: it falls in September in about three years of four and in October in the
            /// rest. In 2025 it is 7 October, and September's full moon is the Corn Moon. The days
            /// are judged at the meridian, and a full moon within about a minute of a day's end can
            /// move by a day. A meridian that is not read is `HC_ERR_UNKNOWN`, and a year outside
            /// −999 to 3000 is `HC_ERR_OUT_OF_RANGE`.
            ///
            /// `meridian` is `universal`, `japan`, `china`, `korea`, `india`, `china-before-1929`,
            /// a longitude in degrees east, or empty for `universal`.
            ///
            /// Tab-separated: year, harvest moon, hunters moon, month, september moon, meridian,
            /// source.
            ///
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_harvest_moon(year: i64, meridian: text(meridian_len)) -> line =
            $crate::attribution_lines::harvest_moon_line;
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
            /// A municipality the holiday tables list, which CLDR does not name,
            /// stands in code order after its prefecture, `JP-14-130` after
            /// `JP-14`, its name from `hc-i18n`'s `municipal_names` — 川崎市 under
            /// `ja`, `Kawasaki-shi` under `ja-Latn`, `Kawasaki` otherwise — with an
            /// empty draft level and the status `municipal`. `country` is a
            /// territory's code in any case, and one that is not is
            /// `HC_ERR_UNKNOWN`; a territory with no subdivision writes nothing,
            /// and an empty `country` writes all 5 503 of CLDR's and the 21
            /// municipalities, country by country. Both
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
            /// ISO form, `JP-13`, or a municipality's under its subdivision's,
            /// `JP-14-130`, in any case; another, CLDR's own form `jp13`
            /// included, is `HC_ERROR_UNKNOWN`, and a null `code`
            /// `HC_ERROR_NULL_POINTER`. `locale` is as for `hc_territories`. Writes
            /// the required length, including the terminator, into `written`.
        }
        wasm {
            /// One territory or subdivision, as the UTF-8 line `hc_territories` or
            /// `hc_subdivisions` writes for it, returning the byte length written.
            ///
            /// `code` is a territory's code, `JP` or `001`, or a subdivision's in
            /// ISO form, `JP-13`, or a municipality's under its subdivision's,
            /// `JP-14-130`, in any case; another, CLDR's own form `jp13`
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
            /// the root locale too. A locale `hc-i18n` carries that has no phrases
            /// (`bo`, `kab`, `pa-Arab` and thirteen more) is `HC_ERROR_NO_DATA`, not the
            /// root's. Two instants further apart
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
            /// whose phrases are CLDR's `root.xml`'s, `-1 d`, not English's. A
            /// locale `hc-i18n` carries that has no phrases (`bo`, `kab`,
            /// `pa-Arab` and ten more) is `HC_ERR_NO_DATA`, not the root's. Two
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

        c {
            /// The unit a span is said in and its count, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the unit, the signed count, a half
            /// flag and the names of the thresholds and the rounding applied. `thresholds`
            /// is `default`, `exact` or `with-quarters` and `rounding` `ceil`, `floor`,
            /// `nearest`, `truncate` or `nearest-half`, in any case; another is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. A count that does not
            /// fit an `int64_t` is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
            /// including the terminator, into `written`.
        }
        wasm {
            /// The unit a span of seconds is said in and its count, by `hc-humanize`'s
            /// `unit_choice`, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the unit, CLDR's field name (`second`, `minute`, `hour`,
            /// `day`, `week`, `month`, `quarter` under `with-quarters`, or `year`); the
            /// signed count, its whole part; `1` when a half is added to it in the direction
            /// of the sign, which only `nearest-half` does, else `0`; and the names of the
            /// thresholds and the rounding that were applied. `thresholds` is `default`, the
            /// conversational table in which 45 seconds is already a minute; `exact`, which
            /// moves to the next unit only once a whole one fits; or `with-quarters`;
            /// `rounding` is `ceil`, `floor`, `nearest`, `truncate` or `nearest-half`; in any
            /// case, and another, the empty string included, is `HC_ERR_UNKNOWN`. A span
            /// promoted past the table's shortest unit is never counted as zero. A count
            /// that does not fit an `i64` is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns
            /// the length the text needs.
        }
        fn hc_unit_choice(
            seconds: i64,
            thresholds: name(thresholds_len),
            rounding: name(rounding_len),
        ) -> line =
            $crate::humanize_lines::unit_choice_line;

        c {
            /// How one POSIX instant reads from another under a threshold table and a
            /// rounding of the caller's, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line is the WebAssembly module's: the phrase, the unit, the signed
            /// count, a half flag and the tag of the data the locale resolved to.
            /// `thresholds` and `rounding` are as for `hc_unit_choice`; the rest is as for
            /// `hc_relative_time`. Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// How one POSIX instant reads from another under a threshold table and a
            /// rounding of the caller's, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// `hc_relative_time` with `Thresholds::DEFAULT` and truncation, which it
            /// fixes, made arguments. Tab-separated: the phrase; the unit; the signed count,
            /// negative in the past; `1` when a half is added to the count (*1½ hours ago*
            /// under `nearest-half`), else `0`; and the tag of the `hc-humanize` data the
            /// locale resolved to. `thresholds` and `rounding` are as for `hc_unit_choice`
            /// and another name is `HC_ERR_UNKNOWN`; the rest is as for `hc_relative_time`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_relative_time_with(
            then_unix: i64,
            now_unix: i64,
            style: name(style_len),
            automatic: flag,
            locale: text(locale_len),
            thresholds: name(thresholds_len),
            rounding: name(rounding_len),
        ) -> line =
            $crate::humanize_lines::relative_time_with_line;

        c {
            /// A span hedged as a round number, *about 3 hours*, *just over a week*,
            /// *nearly a year*, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the phrase, the hedge, the unit, the
            /// count and the tag of the data the locale resolved to. `style` is `long`,
            /// `short` or `narrow`, in any case; `thresholds` is `default`, `exact` or
            /// `with-quarters`; `policy` is `default` or `bounded`; another is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. `locale` is as for
            /// `hc_relative_time`. Writes the required length, including the terminator,
            /// into `written`.
        }
        wasm {
            /// A span of seconds hedged as a round number, *about 3 hours*, *just over a
            /// week*, *nearly a year*, by `hc-humanize`'s `approximate`, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the phrase; the hedge, `exactly`, `about`, `just-over`, `over`
            /// or `nearly`; the unit; the count, which *nearly* carries up to the next; and
            /// the tag of the `hc-humanize` data the locale resolved to. The unit is the
            /// `thresholds`' (`default`, `exact` or `with-quarters`, as for `hc_unit_choice`)
            /// and the hedge is read off the fraction of a unit left over: under `default`
            /// less than 2 % is no hedge, under 8 % *about*, under 35 % *just over*, under
            /// 70 % *over*, and the rest *nearly* the next count; `bounded` never says
            /// *about*, for a phrase that has to be a true bound. The sign is dropped, a
            /// hedge describing a length. `style` is `long`, `short` or `narrow`, in any
            /// case; another name, the empty string included, is `HC_ERR_UNKNOWN`. `locale`
            /// fails as for `hc_parse_iso_date`; the empty string, and a tag that does not
            /// parse, is the root locale. A count that does not fit an `i64` is
            /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text needs.
        }
        fn hc_approximate_duration(
            seconds: i64,
            style: name(style_len),
            locale: text(locale_len),
            thresholds: name(thresholds_len),
            policy: name(policy_len),
        ) -> line =
            $crate::humanize_lines::approximate_duration_line;

        c {
            /// The CLDR list patterns a style joins the parts of a duration with, in
            /// a locale, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `style` is `long`, `short` or
            /// `narrow`, in any case; anything else is `HC_ERROR_UNKNOWN`, and null
            /// `HC_ERROR_NULL_POINTER`. `locale` is as for `hc_relative_time`. Writes
            /// the required length, including the terminator, into `written`.
        }
        wasm {
            /// The CLDR list patterns a style joins the parts of a duration with, in
            /// a locale, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the pattern for exactly two items, the one that joins
            /// the first item to the rest, the one for the middle and the one for
            /// the last, each with `{0}` and `{1}` where the items go — `{0} and
            /// {1}`, `{0}, {1}`, `{0}, {1}`, `{0}, and {1}` for `en` in the `long`
            /// style — and the tag of the `hc-humanize` data the locale resolved
            /// to. `style` is `long`, `short` or `narrow`, in any case, which take
            /// CLDR's `standard`, `unit` and `unit-narrow` lists, each falling back
            /// to the wider where a locale has none; anything else is
            /// `HC_ERR_UNKNOWN`. `locale` is as for `hc_relative_time`, and fails as
            /// for `hc_parse_iso_date`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_list_forms(style: name(style_len), locale: text(locale_len)) -> line =
            $crate::humanize_lines::list_forms_line;
    } };
    ("natural", $backend:ident) => { $backend! {
        c {
            /// A whole number as the Associated Press writes it, *zero* to *nine*
            /// spelled out and every other number as its digits, in a locale, by
            /// Python's `humanize` and its catalogues, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language of
            /// the catalogue that wrote it, `en`.
            /// `locale` is a NUL-terminated BCP 47 tag, or null for the root locale,
            /// which has no catalogue and so writes English: the first step of its
            /// fallback chain that a `humanize` catalogue is for and that translates
            /// the numerals serves (`pt-AO` is `pt-PT`, `zh-Hant` is `zh-HK`), a bare
            /// language with two catalogues, `pt`, and a language with none, English;
            /// the line's second cell is the language of the catalogue used, never a
            /// mixture.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// A whole number as the Associated Press writes it, *zero* to *nine*
            /// spelled out and every other number, negatives included, as its
            /// digits, in a locale, by Python's `humanize` `apnumber`, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// Tab-separated: the text, and the language of the catalogue that wrote
            /// it, `en` or `de-DE`.
            /// `locale` is a BCP 47 tag, the empty string or a tag that does not
            /// parse being the root locale, which has no catalogue and so writes
            /// English: the first step of its fallback chain that one of the 35
            /// `humanize` catalogues is for and that translates every numeral serves
            /// (`pt-AO` is `pt-PT`, `zh-Hant` is `zh-HK`); a bare language with two
            /// catalogues, `pt`, picks neither and a language with none is English;
            /// the second cell is the language of the catalogue used, `ru-RU`, `en`,
            /// never a mixture of two. `locale` fails as for `hc_parse_iso_date`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_apnumber(value: i64, locale: text(locale_len)) -> line =
            $crate::humanize_lines::apnumber_line;

        c {
            /// A number as a fraction, *3/10*, *1 3/10*, in the English of Python's
            /// `humanize`, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language, `en`.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// A number as a fraction, *3/10*, *1 3/10*, *-1 3/10*, by Python's
            /// `humanize` `fractional`, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// The fractional part is the nearest fraction with a denominator of at
            /// most 1000; a whole number is written as one. Tab-separated: the text,
            /// and the language, `en`. A value that is not finite is *NaN*, *+Inf* or
            /// *-Inf*. A null `buffer` returns the length the text needs.
        }
        fn hc_fractional(value: f64) -> line =
            $crate::humanize_lines::fractional_line;

        c {
            /// A number in scientific notation, *3.00 x 10⁻¹*, in the English of
            /// Python's `humanize`, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language, `en`.
            /// A `precision` above 255 is `HC_ERROR_OUT_OF_RANGE`. Writes the
            /// required length, including the terminator, into `written`.
        }
        wasm {
            /// A number in scientific notation, *3.00 x 10⁻¹*, by Python's
            /// `humanize` `scientific`, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// `precision` is the digits after the point, 2 in Python; one above
            /// 255 is `HC_ERR_OUT_OF_RANGE`. The exponent is written in
            /// superscript digits. Tab-separated: the text, and the language, `en`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_scientific(value: f64, precision: u32) -> line =
            $crate::humanize_lines::scientific_line;

        c {
            /// A number with an SI prefix and a unit, *1.50 kV*, *220 μF*, by
            /// Python's `humanize`, as one NUL-terminated UTF-8 line in a
            /// caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language of
            /// the first catalogue for the locale. `unit` is read as the empty
            /// string when null. A `precision` above 255 is `HC_ERROR_OUT_OF_RANGE`.
            /// The prefixes are symbols no catalogue translates; a catalogue changes
            /// the decimal mark of the scientific form a magnitude beyond them falls
            /// back to. `locale` is a NUL-terminated BCP 47 tag, or null for the
            /// root locale, whose catalogue is English's; the first step of its
            /// fallback chain that a catalogue is for serves, as for `hc_apnumber`.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// A number with an SI prefix and a unit, *1.50 kV*, *200 MW*, *220 μF*,
            /// by Python's `humanize` `metric`, as one UTF-8 line, returning the byte
            /// length written.
            ///
            /// `precision` is the significant digits, 3 in Python. A magnitude of
            /// 10³³ or more, or below 10⁻³⁰, has no prefix and is written as
            /// `hc_scientific` with one digit fewer, then the unit; a `precision` of
            /// 0 there, or one above 255, is `HC_ERR_OUT_OF_RANGE`. No space is
            /// written before a degree, minute or second sign, or when there is
            /// neither prefix nor unit. Tab-separated: the text, and the language of
            /// the first catalogue for the locale, which no word of `metric` is
            /// translated by: only the decimal mark of the scientific form is, so
            /// the catalogue is found as for `hc_apnumber`. `locale` fails as for
            /// `hc_parse_iso_date`. A null `buffer` returns the length the text
            /// needs.
        }
        fn hc_metric(
            value: f64,
            unit: text(unit_len),
            precision: u32,
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::metric_line;

        c {
            /// A size in bytes, *3.0 MB*, *2.9 KiB*, *300B*, in a locale, by
            /// Python's `humanize` and its catalogues, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language of
            /// the catalogue that wrote it.
            /// `style` is `decimal`, `binary` or `gnu`, in any case; anything else is
            /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. `NaN`, and
            /// `decimals` above 255, are `HC_ERROR_OUT_OF_RANGE`.
            /// `locale` is a NUL-terminated BCP 47 tag, or null for the root locale,
            /// which has no catalogue and so writes English: the first step of its
            /// fallback chain that a `humanize` catalogue is for and that translates
            /// *Byte* and the suffixes serves (`pt-AO` is `pt-PT`, `zh-Hant` is `zh-HK`), a bare
            /// language with two catalogues, `pt`, and a language with none, English;
            /// the line's second cell is the language of the catalogue used, never a
            /// mixture.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// A size in bytes, *3.0 MB*, *2.9 KiB*, *300B*, by Python's `humanize`
            /// `naturalsize`, as one UTF-8 line, returning the byte length written.
            ///
            /// `style` is `decimal`, powers of 1000 and `kB`; `binary`, powers of
            /// 1024 and `KiB`; or `gnu`, powers of 1024, one letter and no space; in
            /// any case, and anything else, the empty string included, is
            /// `HC_ERR_UNKNOWN`. One byte is *1 Byte* and below the base *N Bytes*.
            /// `decimals` is Python's `format="%.{decimals}f"`, 1 by default; above
            /// 255 it is `HC_ERR_OUT_OF_RANGE`, as `NaN` is. Tab-separated: the text,
            /// and the language of the catalogue that wrote it.
            /// `locale` is a BCP 47 tag, the empty string or a tag that does not
            /// parse being the root locale, which has no catalogue and so writes
            /// English: the first step of its fallback chain that one of the 35
            /// `humanize` catalogues is for and that translates *Byte* and the suffixes serves
            /// (`pt-AO` is `pt-PT`, `zh-Hant` is `zh-HK`); a bare language with two
            /// catalogues, `pt`, picks neither and a language with none is English;
            /// the second cell is the language of the catalogue used, `ru-RU`, `en`,
            /// never a mixture of two. `locale` fails as for `hc_parse_iso_date`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_naturalsize(
            value: f64,
            style: name(style_len),
            decimals: u32,
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::naturalsize_line;

        c {
            /// Items joined as a list, *one, two and three*, by Python's `humanize`,
            /// as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language,
            /// always `en`. `items` holds one item to a line and is read as no
            /// items when null. `locale` is a NUL-terminated BCP 47 tag, or null,
            /// and changes nothing: `humanize`'s `natural_list` has its `, ` and
            /// ` and ` as literals in `lists.py`, which no catalogue translates, so
            /// every locale gets English and the line says so. Writes the required
            /// length, including the terminator, into `written`.
        }
        wasm {
            /// Items joined as a list, *one, two and three*, with no comma before
            /// the *and*, by Python's `humanize` `natural_list`, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// `items` holds one item to a line, separated by a line feed; the
            /// empty string is no items and writes the empty text. Tab-separated:
            /// the text, and the language, always `en`: `locale` is read, and fails
            /// as for `hc_parse_iso_date`, but changes nothing, because
            /// `natural_list`'s `, ` and ` and ` are literals in `humanize`'s
            /// `lists.py` and no catalogue translates them. A null `buffer` returns
            /// the length the text needs.
        }
        fn hc_naturallist(items: text(items_len), locale: text(locale_len)) -> line =
            $crate::humanize_lines::naturallist_line;

        c {
            /// An integer of any length as a count with a word, *12.4 thousand*,
            /// *1.0 googol*, in a locale, by Python's `humanize` and its
            /// catalogues, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language of
            /// the catalogue that wrote it.
            /// `digits` is an optional sign and ASCII digits; text that is not an
            /// integer is `HC_ERROR_MALFORMED`, and an integer beyond the largest
            /// double, about 1.8 × 10³⁰⁸, or `decimals` above 255, is
            /// `HC_ERROR_OUT_OF_RANGE`.
            /// `locale` is a NUL-terminated BCP 47 tag, or null for the root locale,
            /// which has no catalogue and so writes English: the first step of its
            /// fallback chain that a `humanize` catalogue is for and that translates
            /// the words of the powers serves (`pt-AO` is `pt-PT`, `zh-Hant` is `zh-HK`), a bare
            /// language with two catalogues, `pt`, and a language with none, English;
            /// the line's second cell is the language of the catalogue used, never a
            /// mixture.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// An integer of any length as a count with a word, *12.4 thousand*,
            /// *1.2 billion*, *1.0 googol*, by Python's `humanize` `intword`, as one
            /// UTF-8 line, returning the byte length written.
            ///
            /// `digits` is an optional sign and ASCII digits, so that a value no
            /// `i64` holds can be given; the words are *thousand* to *decillion*
            /// (10³³) and the *googol* (10¹⁰⁰). `decimals` is Python's
            /// `format="%.{decimals}f"`, 1 by default; a value that rounds to the
            /// next power is written in it, *1.0 million* for 999 999. Text that is
            /// not an integer is `HC_ERR_MALFORMED`; an integer beyond the largest
            /// double, about 1.8 × 10³⁰⁸, or `decimals` above 255, is
            /// `HC_ERR_OUT_OF_RANGE`. Tab-separated: the text, and the language of
            /// the catalogue that wrote it.
            /// `locale` is a BCP 47 tag, the empty string or a tag that does not
            /// parse being the root locale, which has no catalogue and so writes
            /// English: the first step of its fallback chain that one of the 35
            /// `humanize` catalogues is for and that translates every word of the powers serves
            /// (`pt-AO` is `pt-PT`, `zh-Hant` is `zh-HK`); a bare language with two
            /// catalogues, `pt`, picks neither and a language with none is English;
            /// the second cell is the language of the catalogue used, `ru-RU`, `en`,
            /// never a mixture of two. `locale` fails as for `hc_parse_iso_date`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_intword(
            digits: name(digits_len),
            decimals: u32,
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::intword_line;

        c {
            /// `humanize`'s `naturaldelta` of a span, *3 hours*, *a moment*, *1 year, 3
            /// months*, without tense, as one NUL-terminated UTF-8 line in a caller-owned
            /// buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language of the
            /// catalogue that wrote it. The span is `seconds` plus `microseconds`, which
            /// may differ in sign; `microseconds` from 1 000 000 in magnitude is
            /// `HC_ERROR_OUT_OF_RANGE`. The sign of the span is ignored, as Python's
            /// `abs` ignores it. `months` non-zero uses months of 30.5 days between days
            /// and years. `minimum_unit` is `seconds`, `milliseconds` or `microseconds`, in
            /// any case; another unit name is `HC_ERROR_UNKNOWN` and a unit above seconds
            /// `HC_ERROR_OUT_OF_RANGE`, which Python raises as a `ValueError`; null is
            /// `HC_ERROR_NULL_POINTER`. A span of more than about 10²⁶ years is
            /// `HC_ERROR_OUT_OF_RANGE`.
            /// `locale` is a NUL-terminated BCP 47 tag, or null for the root locale,
            /// which has no catalogue and so writes English; the first step of its
            /// fallback chain that a `humanize` catalogue is for and that translates
            /// the units serves, as for `hc_apnumber`, and the line's second cell is the
            /// language of the catalogue used.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// `humanize`'s `naturaldelta` of a span, *3 hours*, *a moment*, *1 year, 3
            /// months*, without tense, as one UTF-8 line, returning the byte length
            /// written.
            ///
            /// Tab-separated: the text, and the language of the catalogue that wrote it.
            /// The span is `seconds` plus `microseconds`, which may differ in sign;
            /// `microseconds` from 1 000 000 in magnitude is `HC_ERR_OUT_OF_RANGE`. The
            /// arithmetic is `humanize` 4.16.0's: the sign is ignored; years are 365 days,
            /// months 30.5 days rounded half to even, and a second unit above is reached at
            /// 60, 3 600 and 86 400 seconds rounded. `months` non-zero uses months between
            /// days and years, zero days. `minimum_unit` is `seconds`, `milliseconds` or
            /// `microseconds`, in any case; another unit name, the empty string included,
            /// is `HC_ERR_UNKNOWN`, and a unit above seconds `HC_ERR_OUT_OF_RANGE`, which
            /// Python raises as a `ValueError`. A span of more than about 10²⁶ years is
            /// `HC_ERR_OUT_OF_RANGE`.
            /// `locale` is read as for `hc_apnumber`: the first step of its fallback
            /// chain that one of the 35 `humanize` catalogues is for and that
            /// translates every word of the function (the units, *ago*, *and*, the fine units below a second) serves, a bare language with two catalogues, `pt`, and
            /// a language with none are English, and the second cell is the language
            /// of the catalogue used, never a mixture. It fails as for
            /// `hc_parse_iso_date`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_naturaldelta(
            seconds: i64,
            microseconds: int,
            months: flag,
            minimum_unit: name(minimum_unit_len),
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::naturaldelta_line;

        c {
            /// `humanize`'s `naturaltime` of a span, *3 hours ago*, *3 hours from now*,
            /// *now*, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language of the
            /// catalogue that wrote it. The span is how long *ago*, so a positive one is in
            /// the past and a negative one in the future, as in Python. The rest is as for
            /// `hc_naturaldelta`. Writes the required length, including the terminator,
            /// into `written`.
        }
        wasm {
            /// `humanize`'s `naturaltime` of a span, *3 hours ago*, *3 hours from now*,
            /// *now*, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the text, and the language of the catalogue that wrote it.
            /// The span is how long *ago*, so a positive one is in the past and a negative
            /// one in the future, as in Python's `naturaltime(timedelta)`; a span the
            /// `hc_naturaldelta` writes as *a moment* is *now*. The rest is as for
            /// `hc_naturaldelta`. A null `buffer` returns the length the text needs.
        }
        fn hc_naturaltime(
            seconds: i64,
            microseconds: int,
            months: flag,
            minimum_unit: name(minimum_unit_len),
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::naturaltime_line;

        c {
            /// `humanize`'s `precisedelta` of a span, *1 year, 2 months and 3 days*, as
            /// one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language of the
            /// catalogue that wrote it. `minimum_unit` is the smallest unit written,
            /// `microseconds`, `milliseconds`, `seconds`, `minutes`, `hours`, `days`, `months`
            /// or `years`, in any case; `suppress` is the units folded into the next
            /// smaller, separated by commas, null or empty for none; either is
            /// `HC_ERROR_UNKNOWN` for a name no unit has, and a minimum unit suppressed
            /// with no larger unit left, a `microseconds` from 1 000 000 in magnitude and
            /// `decimals` above 255 are `HC_ERROR_OUT_OF_RANGE`. Null `minimum_unit` is
            /// `HC_ERROR_NULL_POINTER`. `decimals` is the places of the fraction of the
            /// smallest unit, 2 in Python.
            /// `locale` is a NUL-terminated BCP 47 tag, or null for the root locale,
            /// which has no catalogue and so writes English; the first step of its
            /// fallback chain that a `humanize` catalogue is for and that translates
            /// the units serves, as for `hc_apnumber`, and the line's second cell is the
            /// language of the catalogue used.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// `humanize`'s `precisedelta` of a span, *1 year, 2 months and 3 days*, as
            /// one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the text, and the language of the catalogue that wrote it.
            /// The arithmetic is `humanize` 4.16.0's, step for step, so its quirks are
            /// kept: a month is 30.5 days and the half day it leaves is dropped, the
            /// smallest unit's value is rounded with the format before it is tested, and
            /// a unit rounding pushes to the next one's size is carried. The span is
            /// `seconds` plus `microseconds` and its sign is ignored. `minimum_unit` is
            /// the smallest unit written, `microseconds`, `milliseconds`, `seconds`,
            /// `minutes`, `hours`, `days`, `months` or `years`, in any case; `suppress` is
            /// the units folded into the next smaller, separated by commas, the empty
            /// string for none; either is `HC_ERR_UNKNOWN` for a name no unit has. A
            /// minimum unit suppressed with no larger unit left, `microseconds` from
            /// 1 000 000 in magnitude and `decimals` above 255 are `HC_ERR_OUT_OF_RANGE`.
            /// `decimals` is the places of the fraction of the smallest unit, 2 in
            /// Python.
            /// `locale` is read as for `hc_apnumber`: the first step of its fallback
            /// chain that one of the 35 `humanize` catalogues is for and that
            /// translates every word of the function (the units, *ago*, *and*, the fine units below a second) serves, a bare language with two catalogues, `pt`, and
            /// a language with none are English, and the second cell is the language
            /// of the catalogue used, never a mixture. It fails as for
            /// `hc_parse_iso_date`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_precisedelta(
            seconds: i64,
            microseconds: int,
            minimum_unit: name(minimum_unit_len),
            suppress: text(suppress_len),
            decimals: u32,
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::precisedelta_line;

        c {
            /// `humanize`'s `naturalday` of a fixed day seen from another, *today*,
            /// *tomorrow*, *yesterday*, or the day by a `strftime` pattern, as one
            /// NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language that wrote
            /// it, the catalogue's for *today*, *tomorrow* and *yesterday* and `en` for a day
            /// written by `strftime`, whatever the catalogue. `pattern` is a POSIX `strftime` pattern in
            /// the C locale, so the month name is always English's, as Python's is; null or
            /// empty is Python's `%b %d`. A pattern `hc-format` does not write is
            /// `HC_ERROR_MALFORMED`; a day outside the Gregorian range
            /// `HC_ERROR_OUT_OF_RANGE`.
            /// `locale` is a NUL-terminated BCP 47 tag, or null for the root locale,
            /// which has no catalogue and so writes English; the first step of its
            /// fallback chain that a `humanize` catalogue is for and that translates
            /// *today*, *tomorrow* and *yesterday* serves, as for `hc_apnumber`, and the line's second cell is the
            /// language of the catalogue used.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// `humanize`'s `naturalday` of a fixed day seen from another, *today*,
            /// *tomorrow*, *yesterday*, or the day by a `strftime` pattern, as one UTF-8
            /// line, returning the byte length written.
            ///
            /// Tab-separated: the text, and the language that wrote it, the catalogue's for
            /// *today*, *tomorrow* and *yesterday* and `en` for a day written by `strftime`,
            /// whatever the catalogue. The distance is the difference of the two day numbers. `pattern` is
            /// a POSIX `strftime` pattern in the C locale, so the month name is always
            /// English's, as Python's `strftime` writes it; the empty string is Python's
            /// default `%b %d`. A pattern `hc-format` does not write is `HC_ERR_MALFORMED`;
            /// a day outside the Gregorian range `HC_ERR_OUT_OF_RANGE`.
            /// `locale` is read as for `hc_apnumber`: the first step of its fallback
            /// chain that one of the 35 `humanize` catalogues is for and that
            /// translates *today*, *tomorrow* and *yesterday* serves, a bare language with two catalogues, `pt`, and
            /// a language with none are English, and the second cell is the language
            /// of the catalogue used, never a mixture. It fails as for
            /// `hc_parse_iso_date`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_naturalday(
            day: i64,
            today: i64,
            pattern: text(pattern_len),
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::naturalday_line;

        c {
            /// `humanize`'s `naturaldate` of a fixed day seen from another, as
            /// `hc_naturalday` with `%b %d`, and with the year added from five twelfths of a
            /// year away, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language that wrote
            /// it, the catalogue's for *today*, *tomorrow* and *yesterday* and `en` for any
            /// other day. A day outside the Gregorian range is
            /// `HC_ERROR_OUT_OF_RANGE`.
            /// `locale` is a NUL-terminated BCP 47 tag, or null for the root locale,
            /// which has no catalogue and so writes English; the first step of its
            /// fallback chain that a `humanize` catalogue is for and that translates
            /// *today*, *tomorrow* and *yesterday* serves, as for `hc_apnumber`, and the line's second cell is the
            /// language of the catalogue used.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// `humanize`'s `naturaldate` of a fixed day seen from another, as
            /// `hc_naturalday` with `%b %d`, and with `%b %d %Y` once the day is 153 or more
            /// days away, five twelfths of 365 rounded up, as one UTF-8 line, returning the
            /// byte length written.
            ///
            /// Tab-separated: the text, and the language that wrote it, the catalogue's for
            /// *today*, *tomorrow* and *yesterday* and `en` for any other day. A day outside the Gregorian range is `HC_ERR_OUT_OF_RANGE`.
            /// `locale` is read as for `hc_apnumber`: the first step of its fallback
            /// chain that one of the 35 `humanize` catalogues is for and that
            /// translates *today*, *tomorrow* and *yesterday* serves, a bare language with two catalogues, `pt`, and
            /// a language with none are English, and the second cell is the language
            /// of the catalogue used, never a mixture. It fails as for
            /// `hc_parse_iso_date`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_naturaldate(day: i64, today: i64, locale: text(locale_len)) -> line =
            $crate::humanize_lines::naturaldate_line;

        c {
            /// `humanize`'s `ordinal` of an integer, *1st*, *2nd*, *103rd*, *111th*, in a
            /// locale, as one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language of the
            /// catalogue that wrote the suffix. `gender` is `male` or `female`, in any
            /// case; another is `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`.
            /// `locale` is a NUL-terminated BCP 47 tag, or null for the root locale,
            /// which has no catalogue and so writes English; the first step of its
            /// fallback chain that a `humanize` catalogue is for and that translates
            /// the suffixes serves, as for `hc_apnumber`, and the line's second cell is the
            /// language of the catalogue used.
            /// Writes the required length, including the terminator, into `written`.
        }
        wasm {
            /// `humanize`'s `ordinal` of an integer, *1st*, *2nd*, *103rd*, *111th*, in a
            /// locale, as one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the text, and the language of the catalogue that wrote the
            /// suffix. The suffix is the one of the last digit in the gender, and of 11, 12
            /// and 13 that of 0, as `humanize` writes it. `gender` is `male` or `female`,
            /// in any case; another, the empty string included, is `HC_ERR_UNKNOWN`.
            /// `locale` is read as for `hc_apnumber`: the first step of its fallback
            /// chain that one of the 35 `humanize` catalogues is for and that
            /// translates *Byte* and every suffix serves, a bare language with two catalogues, `pt`, and
            /// a language with none are English, and the second cell is the language
            /// of the catalogue used, never a mixture. It fails as for
            /// `hc_parse_iso_date`.
            /// A null `buffer` returns the length the text needs.
        }
        fn hc_ordinal(
            value: i64,
            gender: name(gender_len),
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::ordinal_line;

        c {
            /// `humanize`'s `intcomma` of an integer written in digits, *1,234,567*, as
            /// one NUL-terminated UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language of the
            /// catalogue whose separators wrote it, `1.234.567` for `de`. `digits` is an
            /// optional sign and ASCII digits; text that is not an integer is
            /// `HC_ERROR_MALFORMED` and one of more than 39 digits `HC_ERROR_OUT_OF_RANGE`.
            /// `locale` is a NUL-terminated BCP 47 tag, or null for the root locale; the
            /// first catalogue along its fallback chain serves, and English where none is
            /// for it. Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// `humanize`'s `intcomma` of an integer written in digits, *1,234,567*, as
            /// one UTF-8 line, returning the byte length written.
            ///
            /// Tab-separated: the text, and the language of the catalogue whose separators
            /// wrote it: `1.234.567` for `de`, from `humanize`'s `i18n.py`. `digits` is an
            /// optional sign and ASCII digits; text that is not an integer is
            /// `HC_ERR_MALFORMED` and one of more than 39 digits, which an `i128` holds,
            /// `HC_ERR_OUT_OF_RANGE`. `locale` is read as for `hc_apnumber`, except that
            /// the first catalogue along its fallback chain serves whether or not it has
            /// other separators than English's. It fails as for `hc_parse_iso_date`. A null
            /// `buffer` returns the length the text needs.
        }
        fn hc_intcomma(digits: name(digits_len), locale: text(locale_len)) -> line =
            $crate::humanize_lines::intcomma_line;

        c {
            /// `humanize`'s `intcomma` of a float, *1,234,567.25*, as one NUL-terminated
            /// UTF-8 line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's: the text and the language of the
            /// catalogue whose separators wrote it. `ndigits` is Python's `ndigits`, the
            /// places after the point; negative writes the number as Python's `repr` does.
            /// `ndigits` above 255 is `HC_ERROR_OUT_OF_RANGE`. `locale` is as for
            /// `hc_intcomma`. Writes the required length, including the terminator, into
            /// `written`.
        }
        wasm {
            /// `humanize`'s `intcomma` of a float, *1,234,567.25*, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the text, and the language of the catalogue whose separators
            /// wrote it. `ndigits` is Python's `ndigits`, the places after the point;
            /// negative writes the number as Python's `repr` does, the shortest digits
            /// that read back, `.0` on a whole number and an exponent from 10¹⁶, and groups
            /// only the digits before the point. A value that is not finite is `NaN`,
            /// `+Inf` or `-Inf`. `ndigits` above 255 is `HC_ERR_OUT_OF_RANGE`. `locale` is
            /// as for `hc_intcomma`. A null `buffer` returns the length the text needs.
        }
        fn hc_intcomma_float(
            value: f64,
            ndigits: int,
            locale: text(locale_len),
        ) -> line =
            $crate::humanize_lines::intcomma_float_line;

        c {
            /// A number held within a floor and a ceiling and written with a
            /// format, by Python's `humanize` `clamp`, as one NUL-terminated UTF-8
            /// line in a caller-owned buffer.
            ///
            /// The line is the WebAssembly module's. `format` is `display`,
            /// `fixed:N` or `percent:N`, in any case; another word is
            /// `HC_ERROR_UNKNOWN`, a count that is not a number `HC_ERROR_MALFORMED`
            /// and one above 255 `HC_ERROR_OUT_OF_RANGE`; null is
            /// `HC_ERROR_NULL_POINTER`. `floor` and `ceil` are decimal numbers, or
            /// null or empty for no bound; other text is `HC_ERROR_MALFORMED`. A bound
            /// is a float, as `value` is: under `display` a whole-number bound is
            /// written as one, `<0.0` for a `floor` of 0, where Python's `clamp`
            /// given the integer 0 writes `<0`. `floor_token` and `ceil_token` are the text written before a bound,
            /// null for none. Writes the required length, including the
            /// terminator, into `written`.
        }
        wasm {
            /// A number held within a floor and a ceiling and written with a
            /// format, by Python's `humanize` `clamp`, as one UTF-8 line,
            /// returning the byte length written.
            ///
            /// Tab-separated: the text — the value written with `format`, or,
            /// below `floor` or above `ceil`, that bound written the same way after
            /// its token, `<0.01`, `>99.00%` — and `en`, as `clamp` writes no word
            /// a catalogue translates. `format` is Python's `format` argument in
            /// the three shapes a string takes: `display`, `"{:}"`, the value as
            /// `str` writes a float; `fixed:N`, `"{:.Nf}"`; or `percent:N`,
            /// `"{:.N%}"`, the value times a hundred and a percent sign, in any
            /// case. Python also takes a function, which has no shape at a
            /// boundary: a caller who needs one formats the value the line gives.
            /// Another word is `HC_ERR_UNKNOWN`, a count that is not a number
            /// `HC_ERR_MALFORMED` and one above 255 `HC_ERR_OUT_OF_RANGE`. `floor`
            /// and `ceil` are decimal numbers, or empty for no bound, `floor` tested
            /// first as in Python; other text is `HC_ERR_MALFORMED`. A bound is a
            /// float, as `value` is: under `display` a whole-number bound is written as
            /// one, `<0.0` for a `floor` of 0, where Python's `clamp` given the integer
            /// 0 writes `<0`; `fixed:N` and `percent:N` write the same for both. A value that is
            /// not finite is `NaN`, `+Inf` or `-Inf`, as `hc_fractional` writes them.
            /// The texts fail as for `hc_parse_iso_date`. A null `buffer` returns the
            /// length the text needs.
        }
        fn hc_clamp(
            value: f64,
            format: name(format_len),
            floor: opt(floor_len),
            ceil: opt(ceil_len),
            floor_token: text(floor_token_len),
            ceil_token: text(ceil_token_len),
        ) -> line =
            $crate::humanize_lines::clamp_line;
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
            /// built-in rules, in the locale's vocabulary and digits; `%E` writes
            /// the era of the calendar the locale's `-u-ca-` key names, the
            /// Buddhist or the Minguo, or in a module built with `calendars` too
            /// the Japanese eras for `japanese`. Tab-separated:
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
