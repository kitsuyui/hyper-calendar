//! The ISO 8601 profile and pattern conventions of CPython's `datetime`.
//!
//! Python's `fromisoformat` and `isoformat` are a *profile* of ISO 8601, in
//! the way RFC 3339 is, and `strptime` resolves a pattern by rules of its own.
//! A caller porting Python code wants those rules exactly, and a caller
//! reading ISO 8601 wants [`crate::iso8601`]; the two are different questions,
//! so the profile is named here rather than being a setting of the general
//! parser (policy §5).
//!
//! The rules are those of the Python 3.13 documentation for the `datetime`
//! module, <https://docs.python.org/3/library/datetime.html>, retrieved
//! 2026-09-26 and 2026-10-03, and, where the documentation is silent, what
//! CPython 3.12.14 and 3.14.7 answered when run (`docs/python-parity.md`
//! lists every difference).
//!
//! # What `fromisoformat` accepts
//!
//! * Dates `YYYY-MM-DD`, `YYYYMMDD`, `YYYY-Www-D` and `YYYYWwwD`, and a week
//!   without a day, `YYYY-Www` or `YYYYWww`, which is the week's Monday.
//!   Not the reduced `YYYY-MM` or `YYYY`, not expanded years, not ordinal
//!   dates — Python refuses all three and so does this.
//! * Times `HH`, `HH:MM`, `HH:MM:SS` and their basic forms, with a fraction of
//!   the second after `.` or `,`, and an optional leading `T` on a time on
//!   its own. Not a fraction of an hour or a minute.
//! * Any single character between the date and the time, as Python allows.
//! * Offsets `Z`, `±HH`, `±HHMM`, `±HH:MM` and the forms with seconds, below
//!   24 hours as Python's `timezone` requires.
//!
//! Two extensions, both because this library can hold what Python cannot:
//! a fraction keeps up to eighteen digits instead of being truncated to six,
//! and `23:59:60` is accepted; year 0 is a date here too. Offsets with a
//! fraction of a second, which Python accepts, are refused:
//! `hc_tz::UtcOffset` counts whole seconds. The behaviours of CPython's C
//! parser that the documentation does not describe — a stray character
//! before the offset, minutes above 59 in an offset, a fraction after the
//! hour — are refused.
//!
//! # `strptime`
//!
//! [`strptime`] is CPython's `_strptime`: a pattern is a regular expression
//! with ordered alternatives, matched with backtracking, and the fields are
//! resolved by its rules. See the `strptime` module's documentation; a field
//! the text does not name comes from 1900-01-01 00:00:00.

mod strptime;

use core::fmt;

use hc_calendar::{CivilDateTime, CivilTime, Rd};
use hc_tz::{OffsetStyle, UtcOffset};

use crate::error::{ErrorKind, FormatError, FormatResult, ParseError, ParseResult};
use crate::iso8601::{self, Strictness};
use crate::patterns::{FormatContext, ParsedFields, strftime};
use crate::scan::Scanner;
use crate::value::{
    DateParts, IsoDate, IsoTime, OffsetDateTime, ZoneInfo, check_leap_second, leap_second_error,
};

/// What Python's `fromisoformat` accepts, as a [`Strictness`]. The ordinal
/// date and the fraction of an hour or minute, which `Strictness` cannot
/// refuse on its own, are refused after scanning.
const PROFILE: Strictness = Strictness {
    allow_basic: true,
    // Reduced dates are scanned so that a week without a day can be read
    // (Python takes it as the Monday); `scan_date` refuses the others.
    allow_reduced_date: true,
    allow_reduced_time: true,
    allow_expanded_year: false,
    allow_comma_decimal: true,
    allow_end_of_day: false,
    allow_leap_second: true,
    allow_week_and_ordinal_dates: true,
    allow_offset_seconds: true,
    allow_offset_hours_over_23: true,
    allow_negative_zero_offset: true,
    allow_space_separator: false,
    allow_lowercase_designators: false,
    allow_signed_duration: false,
    require_offset_minutes: false,
    require_time: false,
    require_zone: false,
};

/// How much of a time `isoformat` writes: Python's `timespec`.
///
/// Components left out are truncated, not rounded, as in Python.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TimeSpec {
    /// `Seconds` when the microsecond is zero, `Microseconds` otherwise.
    #[default]
    Auto,
    /// `HH`.
    Hours,
    /// `HH:MM`.
    Minutes,
    /// `HH:MM:SS`.
    Seconds,
    /// `HH:MM:SS.sss`.
    Milliseconds,
    /// `HH:MM:SS.ffffff`.
    Microseconds,
}

// --- parsing ---------------------------------------------------------------

/// Python's `date.fromisoformat`.
///
/// ```
/// use hc_format::python;
///
/// assert_eq!(python::parse_date("2019-12-04")?, python::parse_date("20191204")?);
/// assert!(python::parse_date("2019-338").is_err());
/// # Ok::<(), hc_format::ParseError>(())
/// ```
///
/// # Errors
///
/// A [`ParseError`] naming what was expected and where, including
/// [`ErrorKind::Forbidden`] for a form ISO 8601 allows and Python does not.
pub fn parse_date(text: &str) -> ParseResult<Rd> {
    let mut scanner = start(text)?;
    let date = scan_date(&mut scanner)?;
    scanner.finish()?;
    fixed(date, 0)
}

/// Python's `time.fromisoformat`: the time, and the zone when one was
/// written.
///
/// # Errors
///
/// As [`parse_date`].
pub fn parse_time(text: &str) -> ParseResult<(CivilTime, ZoneInfo)> {
    let mut scanner = start(text)?;
    scanner.eat(b'T');
    let (time, zone) = scan_time_and_zone(&mut scanner)?;
    scanner.finish()?;
    Ok((time, zone))
}

/// Python's `datetime.fromisoformat`.
///
/// A date on its own is midnight, as in Python. The result is naive — its
/// zone [`ZoneInfo::Unspecified`] — unless the text wrote an offset.
///
/// # Errors
///
/// As [`parse_date`].
pub fn parse_date_time(text: &str) -> ParseResult<OffsetDateTime> {
    let mut scanner = start(text)?;
    let date = scan_date(&mut scanner)?;
    let day = fixed(date, 0)?;
    if scanner.is_empty() {
        return Ok(OffsetDateTime::local(CivilDateTime::midnight(day)));
    }
    // Python lets any one character separate the date from the time.
    let separator = core::str::from_utf8(scanner.rest())
        .ok()
        .and_then(|rest| rest.chars().next())
        .ok_or_else(|| scanner.error(ErrorKind::Literal("T")))?;
    scanner.advance(separator.len_utf8());
    let (time, zone) = scan_time_and_zone(&mut scanner)?;
    scanner.finish()?;
    let local = CivilDateTime::new(day, time);
    check_leap_second(local, zone).map_err(|error| leap_second_error(error, 0))?;
    Ok(OffsetDateTime {
        local,
        zone,
        written_as_end_of_day: false,
    })
}

/// Python's `datetime.strptime(text, pattern)`, as fields.
///
/// The fields are those [`strftime::parse`] reads, with the date Python
/// assumes for whatever the text leaves out filled in: 1900-01-01. Resolve
/// them with [`ParsedFields::to_offset_date_time`].
///
/// ```
/// use hc_format::python;
///
/// let reading = python::strptime("14:30", "%H:%M")?.to_offset_date_time()?;
/// assert_eq!(reading.local.to_string(), "RD 693596 14:30:00");
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
///
/// # Errors
///
/// As [`strftime::parse`].
pub fn strptime(text: &str, pattern: &str) -> ParseResult<ParsedFields> {
    strptime::parse(pattern, text)
}

fn start(text: &str) -> ParseResult<Scanner<'_>> {
    let scanner = Scanner::new(text);
    if scanner.is_empty() {
        return Err(scanner.error(ErrorKind::Empty));
    }
    Ok(scanner)
}

fn scan_date(scanner: &mut Scanner<'_>) -> ParseResult<IsoDate> {
    let start = scanner.pos();
    let date = iso8601::scan_date(scanner, &PROFILE)?;
    match date.parts {
        DateParts::Ordinal { .. } => Err(Scanner::error_at(
            ErrorKind::Forbidden("an ordinal date"),
            start,
        )),
        // `YYYY-Www` is a week date ISO 8601 allows and Python reads as the
        // week's Monday; `YYYY-MM` and `YYYY` it refuses.
        DateParts::Week {
            year,
            week,
            weekday: None,
        } => Ok(IsoDate {
            parts: DateParts::Week {
                year,
                week,
                weekday: Some(1),
            },
            ..date
        }),
        DateParts::Calendar { day: None, .. } => Err(Scanner::error_at(
            ErrorKind::Forbidden("a date of reduced accuracy"),
            start,
        )),
        _ => Ok(date),
    }
}

fn fixed(date: IsoDate, offset: usize) -> ParseResult<Rd> {
    date.to_fixed()
        .map_err(|_| ParseError::new(ErrorKind::Invalid("date"), offset))
}

fn scan_time_and_zone(scanner: &mut Scanner<'_>) -> ParseResult<(CivilTime, ZoneInfo)> {
    let start = scanner.pos();
    let time = iso8601::scan_time(scanner, &PROFILE)?;
    if time.fraction.is_some() && time.second.is_none() {
        return Err(Scanner::error_at(
            ErrorKind::Forbidden("a fraction of an hour or a minute"),
            start,
        ));
    }
    let clock = civil_time(time).ok_or(Scanner::error_at(ErrorKind::Invalid("time"), start))?;
    let offset_start = scanner.pos();
    let (zone, _) = iso8601::scan_offset(scanner, &PROFILE)?;
    // Python's `timezone` takes an offset of less than 24 hours.
    if zone
        .offset()
        .is_some_and(|offset| offset.seconds().unsigned_abs() >= 86_400)
    {
        return Err(Scanner::error_at(
            ErrorKind::OutOfRange("the offset, which must be under 24 hours"),
            offset_start,
        ));
    }
    // Python reads `-00:00` as UTC; it has no "offset unknown".
    let zone = match zone {
        ZoneInfo::UnknownLocalOffset => ZoneInfo::Offset(UtcOffset::UTC),
        other => other,
    };
    Ok((clock, zone))
}

fn civil_time(time: IsoTime) -> Option<CivilTime> {
    match time.to_time_of_day().ok()?.normalise() {
        (clock, false) => Some(clock),
        (_, true) => None,
    }
}

// --- writing ---------------------------------------------------------------

/// Python's `date.isoformat`: `YYYY-MM-DD`.
///
/// A year outside `0000..=9999`, which Python cannot hold, is written in the
/// expanded form ISO 8601 gives it.
///
/// # Errors
///
/// [`FormatError::Unrepresentable`] outside the Gregorian range, and
/// [`FormatError::Sink`] when the sink refuses.
pub fn write_date<W: fmt::Write>(out: &mut W, day: Rd) -> FormatResult<()> {
    IsoDate::from_fixed(day)
        .map_err(|_| FormatError::Unrepresentable("the date"))?
        .write(out)
}

/// Python's `time.isoformat(timespec)`.
///
/// ```
/// use hc_calendar::CivilTime;
/// use hc_format::python::{self, TimeSpec};
///
/// let time = CivilTime::new(12, 34, 56, 123_456_000_000_000_000)?;
/// let mut out = String::new();
/// python::write_time(&mut out, time, TimeSpec::Minutes)?;
/// assert_eq!(out, "12:34");
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
///
/// # Errors
///
/// [`FormatError::Sink`] when the sink refuses.
pub fn write_time<W: fmt::Write>(out: &mut W, time: CivilTime, spec: TimeSpec) -> FormatResult<()> {
    let micros = time.subsec_attos() / 1_000_000_000_000;
    let spec = match spec {
        TimeSpec::Auto if micros == 0 => TimeSpec::Seconds,
        TimeSpec::Auto => TimeSpec::Microseconds,
        other => other,
    };
    write!(out, "{:02}", time.hour())?;
    if spec == TimeSpec::Hours {
        return Ok(());
    }
    write!(out, ":{:02}", time.minute())?;
    if spec == TimeSpec::Minutes {
        return Ok(());
    }
    write!(out, ":{:02}", time.second())?;
    match spec {
        TimeSpec::Milliseconds => write!(out, ".{:03}", micros / 1_000)?,
        TimeSpec::Microseconds => write!(out, ".{micros:06}")?,
        _ => {}
    }
    Ok(())
}

/// A UTC offset as Python's `isoformat` writes it: `+HH:MM`, with `:SS` when
/// the offset has seconds. UTC is `+00:00`, not `Z`.
///
/// # Errors
///
/// [`FormatError::Sink`] when the sink refuses.
pub fn write_offset<W: fmt::Write>(out: &mut W, offset: UtcOffset) -> FormatResult<()> {
    let style = if offset.abs_seconds() == 0 {
        OffsetStyle::Extended
    } else {
        OffsetStyle::ExtendedSeconds
    };
    out.write_str(offset.format(style).as_str())?;
    Ok(())
}

/// Python's `datetime.isoformat(sep, timespec)`.
///
/// The offset is written when `zone` states one, and nothing is written for
/// [`ZoneInfo::Unspecified`], which is Python's naive datetime.
///
/// ```
/// use hc_calendar::{CivilDateTime, CivilTime, Rd};
/// use hc_format::ZoneInfo;
/// use hc_format::python::{self, TimeSpec};
///
/// // datetime(2019, 5, 18, 15, 17, 8, 132263).isoformat()
/// let reading = CivilDateTime::new(Rd(737_197), CivilTime::new(15, 17, 8, 132_263_000_000_000_000)?);
/// let mut out = String::new();
/// python::write_date_time(&mut out, reading, ZoneInfo::Unspecified, 'T', TimeSpec::Auto)?;
/// assert_eq!(out, "2019-05-18T15:17:08.132263");
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
///
/// # Errors
///
/// As [`write_date`].
pub fn write_date_time<W: fmt::Write>(
    out: &mut W,
    local: CivilDateTime,
    zone: ZoneInfo,
    separator: char,
    spec: TimeSpec,
) -> FormatResult<()> {
    write_date(out, local.day)?;
    out.write_char(separator)?;
    write_time(out, local.time, spec)?;
    if let Some(offset) = zone.offset() {
        write_offset(out, offset)?;
    }
    Ok(())
}

/// Python's `ctime()`: `Wed Dec  4 00:00:00 2002`, which is `strftime("%c")`
/// in the C locale.
///
/// # Errors
///
/// As [`strftime::format`].
pub fn write_ctime<W: fmt::Write>(out: &mut W, local: CivilDateTime) -> FormatResult<()> {
    strftime::format(out, "%c", &FormatContext::new(local))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use hc_calendars_solar::gregorian;

    fn day(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    fn civil(date: (i64, u8, u8), hms: (u8, u8, u8), micros: u64) -> CivilDateTime {
        CivilDateTime::new(
            day(date.0, date.1, date.2),
            CivilTime::new(hms.0, hms.1, hms.2, micros * 1_000_000_000_000).unwrap(),
        )
    }

    fn offset(seconds: i32) -> ZoneInfo {
        ZoneInfo::Offset(UtcOffset::from_seconds(seconds).unwrap())
    }

    /// Python's documentation, `date.fromisoformat`:
    ///
    /// ```text
    /// >>> dt.date.fromisoformat('2019-12-04')
    /// datetime.date(2019, 12, 4)
    /// >>> dt.date.fromisoformat('20191204')
    /// datetime.date(2019, 12, 4)
    /// >>> dt.date.fromisoformat('2021-W01-1')
    /// datetime.date(2021, 1, 4)
    /// ```
    #[test]
    fn date_fromisoformat_matches_the_documented_examples() {
        assert_eq!(parse_date("2019-12-04"), Ok(day(2019, 12, 4)));
        assert_eq!(parse_date("20191204"), Ok(day(2019, 12, 4)));
        assert_eq!(parse_date("2021-W01-1"), Ok(day(2021, 1, 4)));
        assert_eq!(parse_date("2021W011"), Ok(day(2021, 1, 4)));
    }

    /// "Reduced precision dates are not currently supported (`YYYY-MM`,
    /// `YYYY`). Extended date representations are not currently supported
    /// (`±YYYYYY-MM-DD`). Ordinal dates are not currently supported
    /// (`YYYY-OOO`)."
    #[test]
    fn the_forms_python_refuses_are_refused() {
        for text in ["2019-12", "2019", "+002019-12-04", "2019-338", "2019338"] {
            assert!(parse_date(text).is_err(), "{text}");
            assert!(parse_date_time(text).is_err(), "{text}");
        }
        assert_eq!(
            parse_date("2019-338").unwrap_err().kind(),
            ErrorKind::Forbidden("an ordinal date")
        );
    }

    /// "Return a date corresponding to a date_string given in any valid ISO
    /// 8601 format, with the following exceptions: Reduced precision dates
    /// are not currently supported (`YYYY-MM`, `YYYY`)." A week without a
    /// day, `YYYY-Www`, is a valid ISO 8601 date and not an exception, and
    /// CPython reads it as the week's Monday (the interpreter's answer for
    /// `2019-W01` is 2018-12-31; the documented `2021-W01-1` is 2021-01-04).
    #[test]
    fn a_week_without_a_day_is_its_monday() {
        assert_eq!(parse_date("2021-W01"), Ok(day(2021, 1, 4)));
        assert_eq!(parse_date("2021W01"), parse_date("2021-W01-1"));
        assert_eq!(parse_date("2019-W01"), Ok(day(2018, 12, 31)));
        assert_eq!(
            parse_date_time("2019-W01T10:20").unwrap().local,
            civil((2018, 12, 31), (10, 20, 0), 0)
        );
        assert!(parse_date("2019-W54").is_err());
        assert!(parse_date("2019-W00").is_err());
    }

    /// Python's `timezone` takes an offset strictly between -24 and +24
    /// hours (CPython 3.12 and 3.14: "offset must be a timedelta strictly
    /// between -timedelta(hours=24) and timedelta(hours=24)"), so
    /// `fromisoformat` refuses `+24:00`; the 23:59 below it is read.
    #[test]
    fn an_offset_of_a_day_or_more_is_refused() {
        assert!(parse_date_time("2019-12-04T10:20:30+24:00").is_err());
        assert!(parse_time("10:20:30-24:00").is_err());
        assert_eq!(
            parse_time("10:20:30+23:59").unwrap().1,
            offset(23 * 3_600 + 59 * 60)
        );
    }

    /// Python's documentation, `datetime.fromisoformat`:
    ///
    /// ```text
    /// >>> dt.datetime.fromisoformat('2011-11-04')
    /// datetime.datetime(2011, 11, 4, 0, 0)
    /// >>> dt.datetime.fromisoformat('20111104')
    /// datetime.datetime(2011, 11, 4, 0, 0)
    /// >>> dt.datetime.fromisoformat('2011-11-04T00:05:23')
    /// datetime.datetime(2011, 11, 4, 0, 5, 23)
    /// >>> dt.datetime.fromisoformat('2011-11-04T00:05:23Z')
    /// datetime.datetime(2011, 11, 4, 0, 5, 23, tzinfo=datetime.timezone.utc)
    /// >>> dt.datetime.fromisoformat('20111104T000523')
    /// datetime.datetime(2011, 11, 4, 0, 5, 23)
    /// >>> dt.datetime.fromisoformat('2011-W01-2T00:05:23.283')
    /// datetime.datetime(2011, 1, 4, 0, 5, 23, 283000)
    /// >>> dt.datetime.fromisoformat('2011-11-04 00:05:23.283')
    /// datetime.datetime(2011, 11, 4, 0, 5, 23, 283000)
    /// >>> dt.datetime.fromisoformat('2011-11-04 00:05:23.283+00:00')
    /// datetime.datetime(2011, 11, 4, 0, 5, 23, 283000, tzinfo=datetime.timezone.utc)
    /// >>> dt.datetime.fromisoformat('2011-11-04T00:05:23+04:00')
    /// datetime.datetime(2011, 11, 4, 0, 5, 23,
    ///     tzinfo=datetime.timezone(datetime.timedelta(seconds=14400)))
    /// ```
    #[test]
    fn datetime_fromisoformat_matches_the_documented_examples() {
        let naive = |text: &str| parse_date_time(text).unwrap();
        let midnight = OffsetDateTime::local(CivilDateTime::midnight(day(2011, 11, 4)));
        assert_eq!(naive("2011-11-04"), midnight);
        assert_eq!(naive("20111104"), midnight);
        let at = civil((2011, 11, 4), (0, 5, 23), 0);
        assert_eq!(naive("2011-11-04T00:05:23"), OffsetDateTime::local(at));
        let utc = naive("2011-11-04T00:05:23Z");
        assert_eq!((utc.local, utc.zone), (at, ZoneInfo::Zulu));
        assert_eq!(naive("20111104T000523"), OffsetDateTime::local(at));
        assert_eq!(
            naive("2011-W01-2T00:05:23.283"),
            OffsetDateTime::local(civil((2011, 1, 4), (0, 5, 23), 283_000))
        );
        let fraction = civil((2011, 11, 4), (0, 5, 23), 283_000);
        assert_eq!(
            naive("2011-11-04 00:05:23.283"),
            OffsetDateTime::local(fraction)
        );
        let plus_zero = naive("2011-11-04 00:05:23.283+00:00");
        assert_eq!(
            (plus_zero.local, plus_zero.zone.offset()),
            (fraction, Some(UtcOffset::UTC))
        );
        let plus_four = naive("2011-11-04T00:05:23+04:00");
        assert_eq!((plus_four.local, plus_four.zone), (at, offset(14_400)));
    }

    /// "The `T` separator may be replaced by any single unicode character."
    #[test]
    fn any_single_character_separates_the_date_from_the_time() {
        let at = OffsetDateTime::local(civil((2011, 11, 4), (0, 5, 23), 0));
        for text in [
            "2011-11-04x00:05:23",
            "2011-11-04\u{3000}00:05:23",
            "2011-11-04_00:05:23",
        ] {
            assert_eq!(parse_date_time(text), Ok(at), "{text}");
        }
        assert!(parse_date_time("2011-11-04  00:05:23").is_err());
    }

    /// Python's documentation, `time.fromisoformat`:
    ///
    /// ```text
    /// >>> dt.time.fromisoformat('04:23:01')
    /// datetime.time(4, 23, 1)
    /// >>> dt.time.fromisoformat('T04:23:01')
    /// datetime.time(4, 23, 1)
    /// >>> dt.time.fromisoformat('T042301')
    /// datetime.time(4, 23, 1)
    /// >>> dt.time.fromisoformat('04:23:01.000384')
    /// datetime.time(4, 23, 1, 384)
    /// >>> dt.time.fromisoformat('04:23:01,000384')
    /// datetime.time(4, 23, 1, 384)
    /// >>> dt.time.fromisoformat('04:23:01+04:00')
    /// datetime.time(4, 23, 1, tzinfo=datetime.timezone(datetime.timedelta(seconds=14400)))
    /// >>> dt.time.fromisoformat('04:23:01Z')
    /// datetime.time(4, 23, 1, tzinfo=datetime.timezone.utc)
    /// >>> dt.time.fromisoformat('04:23:01+00:00')
    /// datetime.time(4, 23, 1, tzinfo=datetime.timezone.utc)
    /// ```
    #[test]
    fn time_fromisoformat_matches_the_documented_examples() {
        let plain = CivilTime::hms(4, 23, 1).unwrap();
        let fraction = CivilTime::new(4, 23, 1, 384_000_000_000_000).unwrap();
        for text in ["04:23:01", "T04:23:01", "T042301"] {
            assert_eq!(
                parse_time(text),
                Ok((plain, ZoneInfo::Unspecified)),
                "{text}"
            );
        }
        for text in ["04:23:01.000384", "04:23:01,000384"] {
            assert_eq!(
                parse_time(text),
                Ok((fraction, ZoneInfo::Unspecified)),
                "{text}"
            );
        }
        assert_eq!(parse_time("04:23:01+04:00"), Ok((plain, offset(14_400))));
        assert_eq!(parse_time("04:23:01Z"), Ok((plain, ZoneInfo::Zulu)));
        assert_eq!(parse_time("04:23:01+00:00"), Ok((plain, offset(0))));
        assert_eq!(parse_time("04:23:01-00:00"), Ok((plain, offset(0))));
    }

    /// "Fractional hours and minutes are not supported."
    #[test]
    fn a_fraction_of_an_hour_or_minute_is_refused() {
        assert_eq!(
            parse_time("04:23.5").unwrap_err().kind(),
            ErrorKind::Forbidden("a fraction of an hour or a minute")
        );
        assert!(parse_time("04.5").is_err());
        assert_eq!(
            parse_time("04"),
            Ok((CivilTime::hms(4, 0, 0).unwrap(), ZoneInfo::Unspecified))
        );
    }

    fn render_time(time: CivilTime, spec: TimeSpec) -> String {
        let mut out = String::new();
        write_time(&mut out, time, spec).unwrap();
        out
    }

    /// Python's documentation, `time.isoformat`:
    ///
    /// ```text
    /// >>> dt.time(hour=12, minute=34, second=56, microsecond=123456).isoformat(timespec='minutes')
    /// '12:34'
    /// >>> my_time = dt.time(hour=12, minute=34, second=56, microsecond=0)
    /// >>> my_time.isoformat(timespec='microseconds')
    /// '12:34:56.000000'
    /// >>> my_time.isoformat(timespec='auto')
    /// '12:34:56'
    /// ```
    #[test]
    fn time_isoformat_matches_the_documented_examples() {
        let fraction = CivilTime::new(12, 34, 56, 123_456_000_000_000_000).unwrap();
        let whole = CivilTime::hms(12, 34, 56).unwrap();
        assert_eq!(render_time(fraction, TimeSpec::Minutes), "12:34");
        assert_eq!(
            render_time(whole, TimeSpec::Microseconds),
            "12:34:56.000000"
        );
        assert_eq!(render_time(whole, TimeSpec::Auto), "12:34:56");
        // Truncated, not rounded.
        assert_eq!(
            render_time(fraction, TimeSpec::Milliseconds),
            "12:34:56.123"
        );
        assert_eq!(render_time(fraction, TimeSpec::Hours), "12");
        assert_eq!(render_time(fraction, TimeSpec::Auto), "12:34:56.123456");
    }

    /// Python's documentation, `datetime.isoformat`:
    ///
    /// ```text
    /// >>> dt.datetime(2019, 5, 18, 15, 17, 8, 132263).isoformat()
    /// '2019-05-18T15:17:08.132263'
    /// >>> dt.datetime(2019, 5, 18, 15, 17, tzinfo=dt.timezone.utc).isoformat()
    /// '2019-05-18T15:17:00+00:00'
    /// >>> dt.datetime(2002, 12, 25, tzinfo=TZ()).isoformat(' ')
    /// '2002-12-25 00:00:00-06:39'
    /// >>> dt.datetime(2009, 11, 27, microsecond=100, tzinfo=TZ()).isoformat()
    /// '2009-11-27T00:00:00.000100-06:39'
    /// >>> my_datetime = dt.datetime(2015, 1, 1, 12, 30, 59, 0)
    /// >>> my_datetime.isoformat(timespec='microseconds')
    /// '2015-01-01T12:30:59.000000'
    /// ```
    #[test]
    fn datetime_isoformat_matches_the_documented_examples() {
        let render = |local, zone, separator, spec| {
            let mut out = String::new();
            write_date_time(&mut out, local, zone, separator, spec).unwrap();
            out
        };
        let minus = offset(-(6 * 3_600 + 39 * 60));
        assert_eq!(
            render(
                civil((2019, 5, 18), (15, 17, 8), 132_263),
                ZoneInfo::Unspecified,
                'T',
                TimeSpec::Auto
            ),
            "2019-05-18T15:17:08.132263"
        );
        assert_eq!(
            render(
                civil((2019, 5, 18), (15, 17, 0), 0),
                ZoneInfo::Zulu,
                'T',
                TimeSpec::Auto
            ),
            "2019-05-18T15:17:00+00:00"
        );
        assert_eq!(
            render(
                civil((2002, 12, 25), (0, 0, 0), 0),
                minus,
                ' ',
                TimeSpec::Auto
            ),
            "2002-12-25 00:00:00-06:39"
        );
        assert_eq!(
            render(
                civil((2009, 11, 27), (0, 0, 0), 100),
                minus,
                'T',
                TimeSpec::Auto
            ),
            "2009-11-27T00:00:00.000100-06:39"
        );
        assert_eq!(
            render(
                civil((2015, 1, 1), (12, 30, 59), 0),
                ZoneInfo::Unspecified,
                'T',
                TimeSpec::Microseconds
            ),
            "2015-01-01T12:30:59.000000"
        );
        // An offset with seconds keeps them, as Python writes `+05:37:30`.
        assert_eq!(
            render(
                civil((1900, 1, 1), (0, 0, 0), 0),
                offset(20_250),
                'T',
                TimeSpec::Seconds
            ),
            "1900-01-01T00:00:00+05:37:30"
        );
    }

    /// Python's documentation: `dt.date(2002, 12, 4).isoformat()` is
    /// `'2002-12-04'` and `.ctime()` is `'Wed Dec  4 00:00:00 2002'`.
    #[test]
    fn date_isoformat_and_ctime_match_the_documented_examples() {
        let mut out = String::new();
        write_date(&mut out, day(2002, 12, 4)).unwrap();
        assert_eq!(out, "2002-12-04");
        let mut out = String::new();
        write_ctime(&mut out, CivilDateTime::midnight(day(2002, 12, 4))).unwrap();
        assert_eq!(out, "Wed Dec  4 00:00:00 2002");
    }

    #[test]
    fn writing_and_parsing_round_trip() {
        let value = civil((2026, 9, 21), (14, 30, 5), 250_000);
        let mut out = String::new();
        write_date_time(&mut out, value, offset(9 * 3_600), 'T', TimeSpec::Auto).unwrap();
        assert_eq!(out, "2026-09-21T14:30:05.250000+09:00");
        let back = parse_date_time(&out).unwrap();
        assert_eq!((back.local, back.zone), (value, offset(9 * 3_600)));
    }

    /// Python's `strptime` fills what the text leaves out from 1900-01-01:
    /// `datetime.strptime("14:30", "%H:%M")` is `datetime(1900, 1, 1, 14, 30)`.
    #[test]
    fn strptime_defaults_to_the_first_of_january_1900() {
        let reading = strptime("14:30", "%H:%M")
            .unwrap()
            .to_offset_date_time()
            .unwrap();
        assert_eq!(reading.local, civil((1900, 1, 1), (14, 30, 0), 0));
        let march = strptime("03", "%m").unwrap().to_offset_date_time().unwrap();
        assert_eq!(march.local.day, day(1900, 3, 1));
        let ordinal = strptime("2026 100", "%Y %j")
            .unwrap()
            .to_offset_date_time()
            .unwrap();
        assert_eq!(ordinal.local.day, day(2026, 4, 10));
    }

    /// `strptime(text, pattern)` as a reading and a zone, or `None` for a
    /// refusal of any kind.
    fn read(text: &str, pattern: &str) -> Option<(CivilDateTime, ZoneInfo)> {
        let value = strptime(text, pattern).ok()?.to_offset_date_time().ok()?;
        Some((value.local, value.zone))
    }

    fn at(date: (i64, u8, u8)) -> CivilDateTime {
        civil(date, (0, 0, 0), 0)
    }

    /// The expected values below are the answers of CPython 3.12.14 and
    /// 3.14.7 to the same text and pattern (their `_strptime` module builds
    /// the regular expression `strptime` module's rules are written in); the
    /// documentation (3.13, "strftime() and strptime() Format Codes") states
    /// the rules they follow. `%Y` is exactly four digits, so a year need
    /// not be followed by a separator, and a field is read as the shortest
    /// alternative that lets the rest of the pattern match.
    #[test]
    fn strptime_reads_digits_as_cpythons_regular_expression_does() {
        assert_eq!(read("20191204", "%Y%m%d").unwrap().0, at((2019, 12, 4)));
        assert_eq!(read("2019124", "%Y%m%d").unwrap().0, at((2019, 12, 4)));
        assert_eq!(read("191204", "%y%m%d").unwrap().0, at((2019, 12, 4)));
        assert_eq!(read("19124", "%y%m%d").unwrap().0, at((2019, 12, 4)));
        assert_eq!(read("2019-12-4", "%Y-%m-%d").unwrap().0, at((2019, 12, 4)));
        // A year is four digits, a `%y` two; Python's documentation says the
        // leading zero of `%y` is optional, the interpreter refuses `9`.
        assert_eq!(read("19", "%Y"), None);
        assert_eq!(read("20190", "%Y"), None);
        assert_eq!(read("9", "%y"), None);
        // Only `%d` and `%I` take a space for a leading zero.
        assert_eq!(read(" 4", "%d").unwrap().0, at((1900, 1, 4)));
        assert_eq!(read(" 1", "%m"), None);
        assert_eq!(read(" 5", "%M"), None);
        assert_eq!(read(" 2019-12-04", "%Y-%m-%d"), None);
        assert_eq!(read("2019-12-04 ", "%Y-%m-%d"), None);
        assert_eq!(read("2019-12-041", "%Y-%m-%d"), None);
        // `%y` maps 69..99 to the 1900s and 0..68 to the 2000s.
        assert_eq!(read("69", "%y").unwrap().0, at((1969, 1, 1)));
        assert_eq!(read("68", "%y").unwrap().0, at((2068, 1, 1)));
        // An hour of 24 matches `2`, then leaves `4` over.
        assert_eq!(read("24", "%H"), None);
        assert_eq!(
            read("1:2", "%H:%M").unwrap().0,
            civil((1900, 1, 1), (1, 2, 0), 0)
        );
    }

    /// Whitespace in a pattern is `\s+`: one or more, never none.
    #[test]
    fn whitespace_in_a_pattern_matches_one_run_of_whitespace() {
        assert_eq!(
            read("2019  12   04", "%Y %m %d").unwrap().0,
            at((2019, 12, 4))
        );
        assert_eq!(read("2019\t12", "%Y %m").unwrap().0, at((2019, 12, 1)));
        assert_eq!(read("201912 04", "%Y %m %d"), None);
        assert_eq!(read("2019-12-04", "%Y-%m-%d ").map(|r| r.0), None);
        assert_eq!(
            read("2019-12-04  ", "%Y-%m-%d ").unwrap().0,
            at((2019, 12, 4))
        );
        assert_eq!(read("December 04,2019", "%B %d, %Y"), None);
        assert_eq!(
            read("December  4, 2019", "%B %d, %Y").unwrap().0,
            at((2019, 12, 4))
        );
        // Matching ignores case, as the documentation says.
        assert_eq!(
            read("dEc 04 2019", "%b %d %Y").unwrap().0,
            at((2019, 12, 4))
        );
        assert_eq!(read("Wednesday", "%a"), None);
        assert_eq!(read("Dec", "%B"), None);
    }

    #[test]
    fn the_hour_and_the_day_period_combine_as_cpython_does() {
        let noon = |text: &str| read(text, "%I %p").unwrap().0.time.hour();
        assert_eq!(noon("12 AM"), 0);
        assert_eq!(noon("12 PM"), 12);
        assert_eq!(noon("1 PM"), 13);
        assert_eq!(noon("01 pm"), 13);
        assert_eq!(noon("10 Am"), 10);
        assert_eq!(read("10AM", "%I %p"), None);
        assert_eq!(read("13 PM", "%I %p"), None);
        assert_eq!(read("00 AM", "%I %p"), None);
        // Without a `%p` the hour is read as AM.
        assert_eq!(read("12", "%I").unwrap().0.time.hour(), 0);
    }

    /// `%j` is added to 1 January without a check, so day 366 of a common
    /// year is the 1st of January that follows; the week of the year and the
    /// ISO week are counted without a check as well, except that an ISO week
    /// 53 must exist ("Invalid week: 53").
    #[test]
    fn the_day_of_the_year_and_the_week_roll_over_as_cpython_does() {
        assert_eq!(read("2019-366", "%Y-%j").unwrap().0, at((2020, 1, 1)));
        assert_eq!(read("2020-366", "%Y-%j").unwrap().0, at((2020, 12, 31)));
        assert_eq!(read("2019-365", "%Y-%j").unwrap().0, at((2019, 12, 31)));
        // No year: 1900, which is not a leap year.
        assert_eq!(read("366", "%j").unwrap().0, at((1901, 1, 1)));
        assert_eq!(read("367", "%j"), None);
        assert_eq!(read("2019-48-3", "%Y-%U-%w").unwrap().0, at((2019, 12, 4)));
        assert_eq!(read("2019-48-3", "%Y-%W-%w").unwrap().0, at((2019, 12, 4)));
        assert_eq!(read("2019-49-3", "%G-%V-%u").unwrap().0, at((2019, 12, 4)));
        assert_eq!(read("2019-01-1", "%G-%V-%u").unwrap().0, at((2018, 12, 31)));
        assert_eq!(read("2020-53-3", "%G-%V-%u").unwrap().0, at((2020, 12, 30)));
        assert_eq!(read("2021-53-3", "%G-%V-%u"), None);
        assert_eq!(read("2019-49", "%G-%V"), None);
        assert_eq!(read("2019-49-3", "%Y-%V-%u"), None);
        assert_eq!(read("2019-49-3 12", "%G-%V-%u %j"), None);
    }

    #[test]
    fn the_offset_follows_cpythons_regular_expression() {
        let zone = |text: &str| read(text, "%z").map(|r| r.1);
        assert_eq!(zone("+0530"), Some(offset(19_800)));
        assert_eq!(zone("-0800"), Some(offset(-28_800)));
        assert_eq!(zone("+05:30"), Some(offset(19_800)));
        assert_eq!(zone("+05:30:15"), Some(offset(19_815)));
        assert_eq!(zone("+053015"), Some(offset(19_815)));
        assert_eq!(zone("-0000"), Some(offset(0)));
        assert_eq!(zone("Z"), Some(ZoneInfo::Zulu));
        assert_eq!(zone("z"), None);
        assert_eq!(zone("+05"), None);
        assert_eq!(zone("+5"), None);
        assert_eq!(zone("+05:3"), None);
        assert_eq!(zone("+0560"), None);
        // Seconds must use the colon as the minutes did.
        assert_eq!(zone("+0530:15"), None);
        assert_eq!(zone("+05:3015"), None);
        // An offset of a day or more is refused by Python's `timezone`.
        assert_eq!(zone("+2400"), None);
        assert_eq!(zone("+2359"), Some(offset(86_340)));
        // A fraction of a second is Python's, not a whole-second offset.
        assert_eq!(zone("+05:30:15.123456"), None);
        // `%Z` takes `UTC` and `GMT`, which set no zone.
        assert_eq!(read("UTC", "%Z").unwrap().1, ZoneInfo::Unspecified);
        assert_eq!(read("gmt", "%Z").unwrap().1, ZoneInfo::Unspecified);
        assert_eq!(read("EST", "%Z"), None);
    }

    #[test]
    fn a_pattern_is_checked_as_cpython_compiles_it() {
        // A directive it does not know, a width or flag, a `%` at the end,
        // and a directive used twice.
        for pattern in ["%C", "%e", "%5Y", "%-d", "%Ey", "%", "%Y %Y", "%c %Y"] {
            assert!(strptime("2019", pattern).is_err(), "{pattern}");
        }
        assert_eq!(read("%", "%%").unwrap().0, at((1900, 1, 1)));
        assert_eq!(
            read("Wed Dec  4 10:20:30 2019", "%c").unwrap().0,
            civil((2019, 12, 4), (10, 20, 30), 0)
        );
        assert_eq!(read("12/04/19", "%x").unwrap().0, at((2019, 12, 4)));
        assert_eq!(
            read("10:20:30", "%X").unwrap().0,
            civil((1900, 1, 1), (10, 20, 30), 0)
        );
        assert_eq!(
            read("123", "%f").unwrap().0.time.subsec_attos(),
            123_000_000_000_000_000
        );
        assert_eq!(read("1234567", "%f"), None);
    }
}
