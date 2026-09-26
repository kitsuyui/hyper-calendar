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
//! 2026-09-26.
//!
//! # What `fromisoformat` accepts
//!
//! * Dates `YYYY-MM-DD`, `YYYYMMDD`, `YYYY-Www-D` and `YYYYWwwD`. Not the
//!   reduced `YYYY-MM` or `YYYY`, not expanded years, not ordinal dates —
//!   Python refuses all three and so does this.
//! * Times `HH`, `HH:MM`, `HH:MM:SS` and their basic forms, with a fraction of
//!   the second after `.` or `,`, and an optional leading `T` on a time on
//!   its own. Not a fraction of an hour or a minute.
//! * Any single character between the date and the time, as Python allows.
//! * Offsets `Z`, `±HH`, `±HHMM`, `±HH:MM` and the forms with seconds.
//!
//! Two extensions, both because this library can hold what Python cannot:
//! a fraction keeps up to eighteen digits instead of being truncated to six,
//! and `23:59:60` is accepted. Offsets with a fraction of a second, which
//! Python accepts, are refused: `hc_tz::UtcOffset` counts whole seconds.
//!
//! # `strptime`
//!
//! [`strptime`] is [`crate::patterns::strftime::parse`] with Python's
//! defaults: a field the text does not name comes from 1900-01-01 00:00:00.

use core::fmt;

use hc_calendar::{CivilDateTime, CivilTime, Rd};
use hc_tz::{OffsetStyle, UtcOffset};

use crate::error::{ErrorKind, FormatError, FormatResult, ParseError, ParseResult};
use crate::iso8601::{self, Strictness};
use crate::patterns::{FormatContext, ParsedFields, strftime};
use crate::scan::Scanner;
use crate::value::{DateParts, IsoDate, IsoTime, OffsetDateTime, ZoneInfo};

/// What Python's `fromisoformat` accepts, as a [`Strictness`]. The ordinal
/// date and the fraction of an hour or minute, which `Strictness` cannot
/// refuse on its own, are refused after scanning.
const PROFILE: Strictness = Strictness {
    allow_basic: true,
    allow_reduced_date: false,
    allow_reduced_time: true,
    allow_expanded_year: false,
    allow_comma_decimal: true,
    allow_end_of_day: false,
    allow_leap_second: true,
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
    Ok(OffsetDateTime {
        local: CivilDateTime::new(day, time),
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
    Ok(strftime::parse(pattern, text)?.with_default_date(1_900, 1, 1))
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
    if matches!(date.parts, DateParts::Ordinal { .. }) {
        return Err(Scanner::error_at(
            ErrorKind::Forbidden("an ordinal date"),
            start,
        ));
    }
    Ok(date)
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
    let (zone, _) = iso8601::scan_offset(scanner, &PROFILE)?;
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
        for text in [
            "2019-12",
            "2019",
            "+002019-12-04",
            "2019-338",
            "2019338",
            "2021-W01",
        ] {
            assert!(parse_date(text).is_err(), "{text}");
            assert!(parse_date_time(text).is_err(), "{text}");
        }
        assert_eq!(
            parse_date("2019-338").unwrap_err().kind(),
            ErrorKind::Forbidden("an ordinal date")
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
}
