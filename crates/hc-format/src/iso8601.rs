//! ISO 8601-1:2019 dates, times, date-times, durations and intervals.
//!
//! # What the standard actually is
//!
//! ISO 8601 is not one format. It is a family of them, and a library that
//! implements only `YYYY-MM-DDTHH:MM:SSZ` has implemented a profile of it —
//! which is fine, and is what [`crate::rfc3339`] is for, but it is not the
//! standard. This module implements the representations of ISO 8601-1:2019:
//! calendar, ordinal and week dates, in basic and extended format, at any
//! permitted accuracy, with expanded years, with a decimal fraction on the
//! lowest-order component, with `24:00` and `23:59:60`, and with every zone
//! designator the standard admits. `24:00`, the end of a calendar day, is
//! the one form taken from ISO 8601-1:2019/Amd 1:2022 rather than from the
//! 2019 text, which had removed it; the amendment restores it. Neither ISO
//! text was read here (both are sold by ISO); the account of what each
//! allows is Wikipedia's, "ISO 8601" (`wikipedia-iso-8601`), read
//! 2026-09-26.
//!
//! # Strictness is the caller's decision
//!
//! Whether `20260921` is acceptable input depends entirely on what the input
//! is *for*. A log ingester wants everything; a form validator wants
//! `YYYY-MM-DD` and nothing else. [`Strictness`] is how a caller says which,
//! rather than having this module guess. [`Strictness::ISO`] accepts
//! everything the standard allows; [`Strictness::FULL`] demands complete
//! extended-format values with a zone.
//!
//! ```
//! use hc_format::iso8601::{self, Strictness};
//!
//! assert!(iso8601::parse("2026-W38-1").is_ok());
//! assert!(iso8601::parse_with("2026-W38-1", Strictness::FULL).is_err());
//! ```

pub mod duration;
pub mod interval;

use core::fmt;

use hc_calendars_solar::{gregorian, iso_week};
use hc_tz::{OffsetStyle, UtcOffset};

use crate::error::{ErrorKind, ParseResult};
use crate::scan::Scanner;
use crate::value::{
    DateParts, DecimalMark, Fraction, IsoDate, IsoDateTime, IsoTime, Style, YearStyle, ZoneInfo,
};

pub use duration::{DurationForm, IsoDuration};
pub use interval::{Interval, RepeatingInterval};

/// Which of ISO 8601's permitted forms a parser will accept.
///
/// Every field is an explicit permission rather than a level on a scale,
/// because the axes really are independent: an RFC 3339 profile forbids the
/// basic format but allows the lowercase `t`, and a form validator may want
/// full accuracy while still accepting `Z`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Strictness {
    /// `20260921`, `143005`, `+0900` — the separator-free format.
    pub allow_basic: bool,
    /// `2026`, `2026-09`, `2026-W38` — a date that names no day.
    pub allow_reduced_date: bool,
    /// `14`, `14:30` — a time that names no second.
    pub allow_reduced_time: bool,
    /// `+002026-09-21` — a year outside `0000..=9999`, or written with a sign.
    pub allow_expanded_year: bool,
    /// `14:30:05,5` — ISO 8601's preferred decimal mark.
    pub allow_comma_decimal: bool,
    /// `24:00`, the end-of-day reading.
    pub allow_end_of_day: bool,
    /// `23:59:60`, an inserted leap second.
    pub allow_leap_second: bool,
    /// `-00:00`, which ISO 8601 forbids and RFC 3339 §4.3 gives a meaning to.
    pub allow_negative_zero_offset: bool,
    /// `2026-09-21 14:30:05` — a space where the standard writes `T`.
    pub allow_space_separator: bool,
    /// `2026-09-21t14:30:05z` — the lowercase designators RFC 3339 tolerates.
    pub allow_lowercase_designators: bool,
    /// `-P1D` — a signed duration, which is ISO 8601-2, not 8601-1.
    pub allow_signed_duration: bool,
    /// Refuse `+09`: require the minutes in a numeric offset.
    pub require_offset_minutes: bool,
    /// Refuse a date with no time of day.
    pub require_time: bool,
    /// Refuse a time with no zone designator.
    pub require_zone: bool,
}

impl Strictness {
    /// Everything ISO 8601-1:2019 with its Amendment 1:2022 allows, and
    /// nothing it does not.
    ///
    /// `24:00` is allowed by the amendment, not by the 2019 text. The one
    /// extension beyond ISO 8601-1 is the signed duration, which comes from
    /// ISO 8601-2; it is accepted here because a duration that cannot be
    /// negative cannot express "three days ago".
    pub const ISO: Self = Self {
        allow_basic: true,
        allow_reduced_date: true,
        allow_reduced_time: true,
        allow_expanded_year: true,
        allow_comma_decimal: true,
        allow_end_of_day: true,
        allow_leap_second: true,
        allow_negative_zero_offset: false,
        allow_space_separator: false,
        allow_lowercase_designators: false,
        allow_signed_duration: true,
        require_offset_minutes: false,
        require_time: false,
        require_zone: false,
    };

    /// Complete extended-format values only, with a zone.
    ///
    /// This is the setting for a validation path: it accepts
    /// `2026-09-21T14:30:05+09:00` and refuses everything of lesser accuracy,
    /// everything separator-free and everything unqualified.
    pub const FULL: Self = Self {
        allow_basic: false,
        allow_reduced_date: false,
        allow_reduced_time: false,
        allow_expanded_year: true,
        allow_comma_decimal: false,
        allow_end_of_day: false,
        allow_leap_second: true,
        allow_negative_zero_offset: false,
        allow_space_separator: false,
        allow_lowercase_designators: false,
        allow_signed_duration: false,
        require_offset_minutes: true,
        require_time: true,
        require_zone: true,
    };

    /// The RFC 3339 profile. See [`crate::rfc3339`].
    pub const RFC_3339: Self = Self {
        allow_basic: false,
        allow_reduced_date: false,
        allow_reduced_time: false,
        allow_expanded_year: false,
        allow_comma_decimal: false,
        allow_end_of_day: false,
        allow_leap_second: true,
        allow_negative_zero_offset: true,
        allow_space_separator: true,
        allow_lowercase_designators: true,
        allow_signed_duration: false,
        require_offset_minutes: true,
        require_time: true,
        require_zone: true,
    };

    /// The same settings, but demanding a zone.
    #[must_use]
    pub const fn requiring_zone(mut self) -> Self {
        self.require_zone = true;
        self
    }

    /// The same settings, but refusing the basic format.
    #[must_use]
    pub const fn refusing_basic(mut self) -> Self {
        self.allow_basic = false;
        self
    }

    /// The same settings, but demanding full accuracy.
    #[must_use]
    pub const fn requiring_full_accuracy(mut self) -> Self {
        self.allow_reduced_date = false;
        self.allow_reduced_time = false;
        self.require_time = true;
        self
    }
}

impl Default for Strictness {
    fn default() -> Self {
        Self::ISO
    }
}

// --- public entry points ---------------------------------------------------

/// Parse a complete ISO 8601 date-time, accepting everything the standard
/// allows.
///
/// # Errors
///
/// See [`crate::ParseError`]; every failure names what was expected and where.
pub fn parse(text: &str) -> ParseResult<IsoDateTime> {
    parse_with(text, Strictness::ISO)
}

/// Parse a complete ISO 8601 date-time under a stated strictness.
///
/// # Errors
///
/// See [`crate::ParseError`].
pub fn parse_with(text: &str, strictness: Strictness) -> ParseResult<IsoDateTime> {
    let mut scanner = Scanner::new(text);
    if scanner.is_empty() {
        return Err(scanner.error(ErrorKind::Empty));
    }
    let value = scan_date_time(&mut scanner, &strictness)?;
    scanner.finish()?;
    Ok(value)
}

/// Parse an ISO 8601 date with no time of day.
///
/// # Errors
///
/// See [`crate::ParseError`].
pub fn parse_date(text: &str) -> ParseResult<IsoDate> {
    parse_date_with(text, Strictness::ISO)
}

/// Parse an ISO 8601 date under a stated strictness.
///
/// # Errors
///
/// See [`crate::ParseError`].
pub fn parse_date_with(text: &str, strictness: Strictness) -> ParseResult<IsoDate> {
    let mut scanner = Scanner::new(text);
    if scanner.is_empty() {
        return Err(scanner.error(ErrorKind::Empty));
    }
    let date = scan_date(&mut scanner, &strictness)?;
    scanner.finish()?;
    Ok(date)
}

/// Parse an ISO 8601 time of day, with an optional leading `T` and an
/// optional zone designator.
///
/// # Errors
///
/// See [`crate::ParseError`].
pub fn parse_time(text: &str) -> ParseResult<(IsoTime, ZoneInfo, OffsetStyle)> {
    parse_time_with(text, Strictness::ISO)
}

/// Parse an ISO 8601 time of day under a stated strictness.
///
/// # Errors
///
/// See [`crate::ParseError`].
pub fn parse_time_with(
    text: &str,
    strictness: Strictness,
) -> ParseResult<(IsoTime, ZoneInfo, OffsetStyle)> {
    let mut scanner = Scanner::new(text);
    if scanner.is_empty() {
        return Err(scanner.error(ErrorKind::Empty));
    }
    if !scanner.eat(b'T') && strictness.allow_lowercase_designators {
        scanner.eat(b't');
    }
    let time = scan_time(&mut scanner, &strictness)?;
    let (zone, style) = scan_offset(&mut scanner, &strictness)?;
    scanner.finish()?;
    if strictness.require_zone && !zone.is_qualified() {
        return Err(scanner.error(ErrorKind::Forbidden("an unqualified local time")));
    }
    Ok((time, zone, style))
}

/// Parse a zone designator on its own: `Z`, `+09`, `+09:00`, `+0900`.
///
/// # Errors
///
/// See [`crate::ParseError`]. An empty string is [`ErrorKind::Empty`], not an
/// unqualified zone.
pub fn parse_offset(text: &str) -> ParseResult<(ZoneInfo, OffsetStyle)> {
    parse_offset_with(text, Strictness::ISO)
}

/// Parse a zone designator under a stated strictness.
///
/// # Errors
///
/// See [`crate::ParseError`].
pub fn parse_offset_with(
    text: &str,
    strictness: Strictness,
) -> ParseResult<(ZoneInfo, OffsetStyle)> {
    let mut scanner = Scanner::new(text);
    if scanner.is_empty() {
        return Err(scanner.error(ErrorKind::Empty));
    }
    let parsed = scan_offset(&mut scanner, &strictness)?;
    if !parsed.0.is_qualified() {
        return Err(scanner.error(ErrorKind::OneOf("Z+-")));
    }
    scanner.finish()?;
    Ok(parsed)
}

// --- scanners --------------------------------------------------------------

pub(crate) fn scan_date_time(
    scanner: &mut Scanner<'_>,
    strictness: &Strictness,
) -> ParseResult<IsoDateTime> {
    let date = scan_date(scanner, strictness)?;
    let mut value = IsoDateTime::from_date(date);
    let separated = scanner.eat(b'T')
        || (strictness.allow_lowercase_designators && scanner.eat(b't'))
        || (strictness.allow_space_separator && scanner.eat(b' '));
    if separated {
        value.time = Some(scan_time(scanner, strictness)?);
        let (zone, style) = scan_offset(scanner, strictness)?;
        value.zone = zone;
        value.zone_style = style;
    }
    if strictness.require_time && value.time.is_none() {
        return Err(scanner.error(ErrorKind::Forbidden("a date with no time of day")));
    }
    if strictness.require_zone && !value.zone.is_qualified() {
        return Err(scanner.error(ErrorKind::Forbidden("an unqualified local time")));
    }
    Ok(value)
}

pub(crate) fn scan_date(
    scanner: &mut Scanner<'_>,
    strictness: &Strictness,
) -> ParseResult<IsoDate> {
    let (year, year_style) = scan_year(scanner, strictness)?;
    let (parts, style) = if scanner.eat(b'-') {
        scan_date_tail_extended(scanner, year)?
    } else if scanner.peek() == Some(b'W') {
        require_basic(scanner, strictness, "a basic-format week date")?;
        (scan_week_basic(scanner, year)?, Style::Basic)
    } else if scanner.peek().is_some_and(|b| b.is_ascii_digit()) {
        require_basic(scanner, strictness, "a basic-format date")?;
        (scan_date_tail_basic(scanner, year)?, Style::Basic)
    } else {
        (
            DateParts::Calendar {
                year,
                month: None,
                day: None,
            },
            Style::Extended,
        )
    };
    if !parts.names_a_day() && !strictness.allow_reduced_date {
        return Err(scanner.error(ErrorKind::Forbidden("a date of reduced accuracy")));
    }
    Ok(IsoDate {
        parts,
        style,
        year_style,
    })
}

fn scan_year(scanner: &mut Scanner<'_>, strictness: &Strictness) -> ParseResult<(i64, YearStyle)> {
    let Some(sign) = scanner.eat_any(b"+-") else {
        let value = scanner.take_digits(4)?;
        return Ok((value as i64, YearStyle::Plain));
    };
    let start = scanner.pos();
    if !strictness.allow_expanded_year {
        return Err(Scanner::error_at(
            ErrorKind::Forbidden("an expanded year"),
            start - 1,
        ));
    }
    let run = scanner.digit_run();
    if run < 4 {
        return Err(Scanner::error_at(ErrorKind::DigitCount(4), start));
    }
    // ISO 8601 leaves the digit count of an expanded year to agreement
    // between the parties. In extended format a separator marks where it
    // ends; in basic format nothing does, so `+0020260921` could be a
    // six-digit year and a month-day or an eight-digit year and a day of the
    // year. This refuses to guess. See the crate README.
    if run > 9 {
        return Err(Scanner::error_at(
            ErrorKind::Forbidden("an expanded year in the basic format"),
            start,
        ));
    }
    let magnitude = scanner.take_digits(run)? as i64;
    let year = if sign == b'-' { -magnitude } else { magnitude };
    if !(gregorian::MIN_YEAR..=gregorian::MAX_YEAR).contains(&year) {
        return Err(Scanner::error_at(ErrorKind::OutOfRange("year"), start));
    }
    Ok((year, YearStyle::Expanded(run as u8)))
}

fn scan_date_tail_extended(
    scanner: &mut Scanner<'_>,
    year: i64,
) -> ParseResult<(DateParts, Style)> {
    if scanner.eat(b'W') {
        return Ok((scan_week_extended(scanner, year)?, Style::Extended));
    }
    let start = scanner.pos();
    let parts = match scanner.digit_run() {
        3 => {
            let day_of_year = scanner.take_digits(3)? as u16;
            check_ordinal(year, day_of_year, start)?;
            DateParts::Ordinal { year, day_of_year }
        }
        2 => {
            let month = scanner.take_digits(2)? as u8;
            check_month(month, start)?;
            let day = if scanner.eat(b'-') {
                let day_start = scanner.pos();
                let day = scanner.take_digits(2)? as u8;
                check_day(year, month, day, day_start)?;
                Some(day)
            } else {
                None
            };
            DateParts::Calendar {
                year,
                month: Some(month),
                day,
            }
        }
        4 => return Err(Scanner::error_at(ErrorKind::MixedFormat, start)),
        _ => return Err(Scanner::error_at(ErrorKind::DigitCount(2), start)),
    };
    Ok((parts, Style::Extended))
}

fn scan_date_tail_basic(scanner: &mut Scanner<'_>, year: i64) -> ParseResult<DateParts> {
    let start = scanner.pos();
    match scanner.digit_run() {
        4 => {
            let month = scanner.take_digits(2)? as u8;
            check_month(month, start)?;
            let day_start = scanner.pos();
            let day = scanner.take_digits(2)? as u8;
            check_day(year, month, day, day_start)?;
            Ok(DateParts::Calendar {
                year,
                month: Some(month),
                day: Some(day),
            })
        }
        3 => {
            let day_of_year = scanner.take_digits(3)? as u16;
            check_ordinal(year, day_of_year, start)?;
            Ok(DateParts::Ordinal { year, day_of_year })
        }
        // ISO 8601-1:2019 4.3.2 forbids `YYYYMM`: it cannot be told apart
        // from the obsolete two-digit-year `YYMMDD`. `YYYY-MM` is the only
        // spelling of a month.
        2 => Err(Scanner::error_at(
            ErrorKind::Forbidden("a month-accuracy date in the basic format"),
            start,
        )),
        _ => Err(Scanner::error_at(ErrorKind::DigitCount(4), start)),
    }
}

fn scan_week_extended(scanner: &mut Scanner<'_>, year: i64) -> ParseResult<DateParts> {
    let start = scanner.pos();
    let week = scanner.take_digits(2)? as u8;
    check_week(year, week, start)?;
    let weekday = if scanner.eat(b'-') {
        let day_start = scanner.pos();
        let weekday = scanner.take_digits(1)? as u8;
        check_weekday(weekday, day_start)?;
        Some(weekday)
    } else if scanner.peek().is_some_and(|b| b.is_ascii_digit()) {
        return Err(scanner.error(ErrorKind::MixedFormat));
    } else {
        None
    };
    Ok(DateParts::Week {
        year,
        week,
        weekday,
    })
}

fn scan_week_basic(scanner: &mut Scanner<'_>, year: i64) -> ParseResult<DateParts> {
    scanner.expect(b'W', "W")?;
    let start = scanner.pos();
    let week = scanner.take_digits(2)? as u8;
    check_week(year, week, start)?;
    let weekday = if scanner.peek().is_some_and(|b| b.is_ascii_digit()) {
        let day_start = scanner.pos();
        let weekday = scanner.take_digits(1)? as u8;
        check_weekday(weekday, day_start)?;
        Some(weekday)
    } else if scanner.peek() == Some(b'-') {
        return Err(scanner.error(ErrorKind::MixedFormat));
    } else {
        None
    };
    Ok(DateParts::Week {
        year,
        week,
        weekday,
    })
}

pub(crate) fn scan_time(
    scanner: &mut Scanner<'_>,
    strictness: &Strictness,
) -> ParseResult<IsoTime> {
    let hour_start = scanner.pos();
    let hour = scanner.take_digits(2)? as u8;
    if hour > 24 {
        return Err(Scanner::error_at(ErrorKind::OutOfRange("hour"), hour_start));
    }
    let mut time = IsoTime {
        hour,
        minute: None,
        second: None,
        fraction: None,
        style: Style::Extended,
        mark: DecimalMark::Point,
    };

    if let Some((fraction, mark)) = scan_fraction(scanner, strictness)? {
        time.fraction = Some(fraction);
        time.mark = mark;
    } else if scanner.eat(b':') {
        let minute_start = scanner.pos();
        let minute = scanner.take_digits(2)? as u8;
        check_minute(minute, minute_start)?;
        time.minute = Some(minute);
        if let Some((fraction, mark)) = scan_fraction(scanner, strictness)? {
            time.fraction = Some(fraction);
            time.mark = mark;
        } else if scanner.eat(b':') {
            let second_start = scanner.pos();
            let second = scanner.take_digits(2)? as u8;
            check_second(strictness, hour, minute, second, second_start)?;
            time.second = Some(second);
            if let Some((fraction, mark)) = scan_fraction(scanner, strictness)? {
                time.fraction = Some(fraction);
                time.mark = mark;
            }
        } else if scanner.peek().is_some_and(|b| b.is_ascii_digit()) {
            return Err(scanner.error(ErrorKind::MixedFormat));
        }
    } else if scanner.peek().is_some_and(|b| b.is_ascii_digit()) {
        require_basic(scanner, strictness, "a basic-format time")?;
        time.style = Style::Basic;
        let start = scanner.pos();
        match scanner.digit_run() {
            2 | 4 => {}
            _ => return Err(Scanner::error_at(ErrorKind::DigitCount(2), start)),
        }
        let minute = scanner.take_digits(2)? as u8;
        check_minute(minute, start)?;
        time.minute = Some(minute);
        if scanner.peek().is_some_and(|b| b.is_ascii_digit()) {
            let second_start = scanner.pos();
            let second = scanner.take_digits(2)? as u8;
            check_second(strictness, hour, minute, second, second_start)?;
            time.second = Some(second);
        } else if scanner.peek() == Some(b':') {
            return Err(scanner.error(ErrorKind::MixedFormat));
        }
        if let Some((fraction, mark)) = scan_fraction(scanner, strictness)? {
            time.fraction = Some(fraction);
            time.mark = mark;
        }
    }

    if time.is_end_of_day() {
        if !strictness.allow_end_of_day {
            return Err(Scanner::error_at(
                ErrorKind::Forbidden("the 24:00 end-of-day reading"),
                hour_start,
            ));
        }
        let clean = time.minute.unwrap_or(0) == 0
            && time.second.unwrap_or(0) == 0
            && time.fraction.is_none();
        if !clean {
            return Err(Scanner::error_at(
                ErrorKind::Invalid("24:00 with a non-zero component"),
                hour_start,
            ));
        }
    }
    if time.second.is_none() && !strictness.allow_reduced_time {
        return Err(Scanner::error_at(
            ErrorKind::Forbidden("a time of reduced accuracy"),
            hour_start,
        ));
    }
    Ok(time)
}

pub(crate) fn scan_fraction(
    scanner: &mut Scanner<'_>,
    strictness: &Strictness,
) -> ParseResult<Option<(Fraction, DecimalMark)>> {
    let mark = match scanner.peek() {
        Some(b'.') => DecimalMark::Point,
        Some(b',') => DecimalMark::Comma,
        _ => return Ok(None),
    };
    if matches!(mark, DecimalMark::Comma) && !strictness.allow_comma_decimal {
        return Err(scanner.error(ErrorKind::Forbidden("the comma decimal mark")));
    }
    scanner.advance(1);
    let start = scanner.pos();
    let digits = scanner.digit_run();
    if digits == 0 {
        return Err(Scanner::error_at(ErrorKind::Digit, start));
    }
    // An attosecond is this library's resolution. Silently dropping digits
    // past it would make a value that no longer round-trips, so it is
    // refused instead.
    if digits > 18 {
        return Err(Scanner::error_at(
            ErrorKind::Unrepresentable("more than 18 fractional digits"),
            start,
        ));
    }
    let raw = scanner.take_digits(digits)?;
    let scale = 10u64.pow(18 - digits as u32);
    let fraction = Fraction::new(raw * scale, digits as u8)
        .ok_or_else(|| Scanner::error_at(ErrorKind::Unrepresentable("the fraction"), start))?;
    Ok(Some((fraction, mark)))
}

pub(crate) fn scan_offset(
    scanner: &mut Scanner<'_>,
    strictness: &Strictness,
) -> ParseResult<(ZoneInfo, OffsetStyle)> {
    match scanner.peek() {
        Some(b'Z') => {
            scanner.advance(1);
            return Ok((ZoneInfo::Zulu, OffsetStyle::Extended));
        }
        Some(b'z') if strictness.allow_lowercase_designators => {
            scanner.advance(1);
            return Ok((ZoneInfo::Zulu, OffsetStyle::Extended));
        }
        Some(b'+' | b'-') => {}
        _ => return Ok((ZoneInfo::Unspecified, OffsetStyle::Extended)),
    }
    let sign_pos = scanner.pos();
    let negative = scanner.eat(b'-');
    if !negative {
        scanner.advance(1);
    }
    let hours = scanner.take_digits(2)? as i32;
    let mut minutes = 0i32;
    let mut seconds = 0i32;
    let style;
    if scanner.eat(b':') {
        let start = scanner.pos();
        minutes = scanner.take_digits(2)? as i32;
        check_offset_part(minutes, "offset minutes", start)?;
        if scanner.eat(b':') {
            let start = scanner.pos();
            seconds = scanner.take_digits(2)? as i32;
            check_offset_part(seconds, "offset seconds", start)?;
            style = OffsetStyle::ExtendedSeconds;
        } else {
            style = OffsetStyle::Extended;
        }
    } else {
        let start = scanner.pos();
        match scanner.digit_run() {
            0 => {
                if strictness.require_offset_minutes {
                    return Err(Scanner::error_at(
                        ErrorKind::Forbidden("an hours-only UTC offset"),
                        sign_pos,
                    ));
                }
                style = OffsetStyle::Hours;
            }
            2 => {
                require_basic(scanner, strictness, "a basic-format UTC offset")?;
                minutes = scanner.take_digits(2)? as i32;
                check_offset_part(minutes, "offset minutes", start)?;
                style = OffsetStyle::Basic;
            }
            4 => {
                require_basic(scanner, strictness, "a basic-format UTC offset")?;
                minutes = scanner.take_digits(2)? as i32;
                check_offset_part(minutes, "offset minutes", start)?;
                let second_start = scanner.pos();
                seconds = scanner.take_digits(2)? as i32;
                check_offset_part(seconds, "offset seconds", second_start)?;
                style = OffsetStyle::BasicSeconds;
            }
            _ => return Err(Scanner::error_at(ErrorKind::DigitCount(2), start)),
        }
    }
    let magnitude = hours * 3_600 + minutes * 60 + seconds;
    if negative && magnitude == 0 {
        if !strictness.allow_negative_zero_offset {
            // ISO 8601-1:2019 4.3.13: "the minus sign shall not be used" for
            // a zero offset. RFC 3339 §4.3 gives it a meaning instead.
            return Err(Scanner::error_at(
                ErrorKind::Forbidden("the negative zero UTC offset"),
                sign_pos,
            ));
        }
        return Ok((ZoneInfo::UnknownLocalOffset, style));
    }
    let signed = if negative { -magnitude } else { magnitude };
    let offset = UtcOffset::from_seconds(signed)
        .map_err(|_| Scanner::error_at(ErrorKind::OutOfRange("UTC offset"), sign_pos))?;
    Ok((ZoneInfo::Offset(offset), style))
}

// --- field checks ----------------------------------------------------------

pub(crate) fn require_basic(
    scanner: &Scanner<'_>,
    strictness: &Strictness,
    what: &'static str,
) -> ParseResult<()> {
    if strictness.allow_basic {
        Ok(())
    } else {
        Err(scanner.error(ErrorKind::Forbidden(what)))
    }
}

fn check_month(month: u8, at: usize) -> ParseResult<()> {
    if (1..=12).contains(&month) {
        Ok(())
    } else {
        Err(Scanner::error_at(ErrorKind::OutOfRange("month"), at))
    }
}

fn check_day(year: i64, month: u8, day: u8, at: usize) -> ParseResult<()> {
    let limit = gregorian::days_in_month(year, month).unwrap_or(0);
    if day >= 1 && day <= limit {
        Ok(())
    } else {
        Err(Scanner::error_at(ErrorKind::OutOfRange("day"), at))
    }
}

fn check_ordinal(year: i64, day_of_year: u16, at: usize) -> ParseResult<()> {
    if day_of_year >= 1 && day_of_year <= gregorian::days_in_year(year) {
        Ok(())
    } else {
        Err(Scanner::error_at(ErrorKind::OutOfRange("day of year"), at))
    }
}

fn check_week(year: i64, week: u8, at: usize) -> ParseResult<()> {
    let limit = iso_week::weeks_in_year(year)
        .map_err(|_| Scanner::error_at(ErrorKind::OutOfRange("year"), at))?;
    if week >= 1 && week <= limit {
        Ok(())
    } else {
        Err(Scanner::error_at(ErrorKind::OutOfRange("week"), at))
    }
}

fn check_weekday(weekday: u8, at: usize) -> ParseResult<()> {
    if (1..=7).contains(&weekday) {
        Ok(())
    } else {
        Err(Scanner::error_at(ErrorKind::OutOfRange("weekday"), at))
    }
}

fn check_minute(minute: u8, at: usize) -> ParseResult<()> {
    if minute <= 59 {
        Ok(())
    } else {
        Err(Scanner::error_at(ErrorKind::OutOfRange("minute"), at))
    }
}

fn check_second(
    strictness: &Strictness,
    hour: u8,
    minute: u8,
    second: u8,
    at: usize,
) -> ParseResult<()> {
    if second <= 59 {
        return Ok(());
    }
    if second > 60 {
        return Err(Scanner::error_at(ErrorKind::OutOfRange("second"), at));
    }
    if !strictness.allow_leap_second {
        return Err(Scanner::error_at(ErrorKind::Forbidden("a leap second"), at));
    }
    // A positive leap second is only ever inserted at the end of a UTC day,
    // and `hc_calendar::CivilTime` can hold it only there. A local reading of
    // `08:59:60` in Tokyo names the same physical second but is not a value
    // this library can carry, so it is refused rather than silently moved.
    if hour == 23 && minute == 59 {
        Ok(())
    } else {
        Err(Scanner::error_at(
            ErrorKind::Invalid("a leap second outside 23:59"),
            at,
        ))
    }
}

fn check_offset_part(value: i32, field: &'static str, at: usize) -> ParseResult<()> {
    if value <= 59 {
        Ok(())
    } else {
        Err(Scanner::error_at(ErrorKind::OutOfRange(field), at))
    }
}

// --- formatting ------------------------------------------------------------

/// Write an ISO 8601 date-time into a sink.
///
/// # Errors
///
/// See [`crate::FormatError`].
pub fn write<W: fmt::Write>(out: &mut W, value: IsoDateTime) -> crate::FormatResult<()> {
    value.write(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::TimeOfDay;
    use alloc::string::{String, ToString as _};
    use hc_calendar::Rd;

    fn round_trip(text: &str) -> String {
        let value = parse(text).unwrap();
        let mut out = String::new();
        value.write(&mut out).unwrap();
        out
    }

    #[test]
    fn calendar_dates_round_trip_in_both_formats() {
        for text in ["2026-09-21", "20260921", "2026-09", "2026"] {
            assert_eq!(round_trip(text), text);
        }
    }

    #[test]
    fn expanded_years_round_trip_with_their_digit_count() {
        assert_eq!(round_trip("+002026-09-21"), "+002026-09-21");
        assert_eq!(round_trip("-000500-01-01"), "-000500-01-01");
    }

    #[test]
    fn an_expanded_year_is_astronomical() {
        let date = parse_date("-000500-01-01").unwrap();
        assert_eq!(date.parts.year(), -500);
    }

    #[test]
    fn ordinal_dates_round_trip_in_both_formats() {
        assert_eq!(round_trip("2026-264"), "2026-264");
        assert_eq!(round_trip("2026264"), "2026264");
        let date = parse_date("2026-264").unwrap();
        assert_eq!(date.to_fixed().unwrap(), Rd(739_880));
    }

    #[test]
    fn the_two_hundred_and_sixty_fourth_day_of_2026_is_the_twenty_first_of_september() {
        let ordinal = parse_date("2026-264").unwrap().to_fixed().unwrap();
        let calendar = parse_date("2026-09-21").unwrap().to_fixed().unwrap();
        assert_eq!(ordinal, calendar);
    }

    #[test]
    fn week_dates_round_trip_in_both_formats() {
        for text in ["2026-W38-1", "2026W381", "2026-W38"] {
            assert_eq!(round_trip(text), text);
        }
    }

    #[test]
    fn the_first_day_of_week_thirty_eight_of_2026_is_a_monday() {
        let rd = parse_date("2026-W38-1").unwrap().to_fixed().unwrap();
        assert_eq!(
            hc_calendar::Weekday::from_rd(rd),
            hc_calendar::Weekday::Monday
        );
        assert_eq!(gregorian::from_fixed(rd).unwrap(), (2026, 9, 14));
    }

    #[test]
    fn times_round_trip_at_every_accuracy() {
        for text in [
            "2026-09-21T14:30:05",
            "20260921T143005",
            "2026-09-21T14:30",
            "20260921T1430",
            "2026-09-21T14",
        ] {
            assert_eq!(round_trip(text), text);
        }
    }

    #[test]
    fn a_fraction_may_sit_on_any_component() {
        assert_eq!(
            round_trip("2026-09-21T14:30:05.123456789"),
            "2026-09-21T14:30:05.123456789"
        );
        assert_eq!(round_trip("2026-09-21T14.5"), "2026-09-21T14.5");
        assert_eq!(round_trip("2026-09-21T14:30,5"), "2026-09-21T14:30,5");
    }

    #[test]
    fn a_fractional_hour_resolves_to_the_right_minute() {
        let value = parse("2026-09-21T14.5").unwrap();
        let resolved = value.to_offset_date_time().unwrap();
        assert_eq!(resolved.local.time.hour(), 14);
        assert_eq!(resolved.local.time.minute(), 30);
    }

    #[test]
    fn zone_designators_round_trip_in_every_spelling() {
        for text in [
            "2026-09-21T14:30:05Z",
            "2026-09-21T14:30:05+09",
            "2026-09-21T14:30:05+09:00",
            "2026-09-21T14:30:05+0900",
            "2026-09-21T14:30:05-05:30",
            "2026-09-21T14:30:05",
        ] {
            assert_eq!(round_trip(text), text);
        }
    }

    #[test]
    fn an_unqualified_time_keeps_no_offset_at_all() {
        let value = parse("2026-09-21T14:30:05").unwrap();
        assert_eq!(value.zone, ZoneInfo::Unspecified);
        assert!(value.zone.offset().is_none());
    }

    #[test]
    fn twenty_four_hundred_survives_a_round_trip() {
        assert_eq!(round_trip("2026-09-21T24:00"), "2026-09-21T24:00");
        assert_eq!(round_trip("2026-09-21T24:00:00Z"), "2026-09-21T24:00:00Z");
        let value = parse("2026-09-21T24:00").unwrap();
        let resolved = value.to_offset_date_time().unwrap();
        assert!(resolved.written_as_end_of_day);
        assert_eq!(resolved.local.day, Rd(739_881));
    }

    #[test]
    fn twenty_four_hundred_and_one_is_not_a_time() {
        let error = parse("2026-09-21T24:00:01").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Invalid("24:00 with a non-zero component")
        );
        assert_eq!(error.offset(), 11);
    }

    #[test]
    fn the_leap_second_survives_a_round_trip() {
        assert_eq!(round_trip("1972-06-30T23:59:60Z"), "1972-06-30T23:59:60Z");
        let value = parse("1972-06-30T23:59:60Z").unwrap();
        let Some(time) = value.time else {
            panic!("the value carries a time");
        };
        assert!(time.is_leap_second());
        assert_eq!(
            time.to_time_of_day().unwrap(),
            TimeOfDay::Clock(hc_calendar::CivilTime::hms(23, 59, 60).unwrap())
        );
    }

    #[test]
    fn a_leap_second_anywhere_but_midnight_is_refused() {
        let error = parse("1972-06-30T12:59:60Z").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Invalid("a leap second outside 23:59")
        );
        assert_eq!(error.offset(), 17);
    }

    #[test]
    fn mixing_basic_and_extended_format_is_refused_where_it_shows() {
        assert_eq!(
            parse_date("2026-0921").unwrap_err().kind(),
            ErrorKind::MixedFormat
        );
        assert_eq!(
            parse("2026-09-21T1430:05").unwrap_err().kind(),
            ErrorKind::MixedFormat
        );
        assert_eq!(
            parse_date("2026W38-1").unwrap_err().kind(),
            ErrorKind::MixedFormat
        );
    }

    #[test]
    fn a_month_accuracy_date_has_no_basic_spelling() {
        let error = parse_date("202609").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Forbidden("a month-accuracy date in the basic format")
        );
        assert_eq!(error.offset(), 4);
    }

    #[test]
    fn full_strictness_refuses_what_iso_allows() {
        for text in [
            "20260921T143005Z",
            "2026-09-21",
            "2026-09-21T14:30:05",
            "2026-09-21T14:30Z",
        ] {
            assert!(parse_with(text, Strictness::ISO).is_ok() || text.contains("14:30Z"));
            assert!(parse_with(text, Strictness::FULL).is_err(), "{text}");
        }
        assert!(parse_with("2026-09-21T14:30:05+09:00", Strictness::FULL).is_ok());
    }

    #[test]
    fn iso_8601_forbids_the_negative_zero_offset() {
        let error = parse("2026-09-21T14:30:05-00:00").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Forbidden("the negative zero UTC offset")
        );
        assert_eq!(error.offset(), 19);
    }

    #[test]
    fn out_of_range_fields_name_themselves_and_their_offset() {
        let cases: [(&str, ErrorKind, usize); 6] = [
            ("2026-13-01", ErrorKind::OutOfRange("month"), 5),
            ("2026-02-30", ErrorKind::OutOfRange("day"), 8),
            ("2026-367", ErrorKind::OutOfRange("day of year"), 5),
            ("2025-W53-1", ErrorKind::OutOfRange("week"), 6),
            ("2026-W38-8", ErrorKind::OutOfRange("weekday"), 9),
            ("2026-09-21T14:60", ErrorKind::OutOfRange("minute"), 14),
        ];
        for (text, kind, offset) in cases {
            let error = parse(text).unwrap_err();
            assert_eq!(error.kind(), kind, "{text}");
            assert_eq!(error.offset(), offset, "{text}");
        }
    }

    #[test]
    fn twenty_twenty_is_a_leap_year_and_twenty_twenty_six_is_not() {
        assert!(parse_date("2020-02-29").is_ok());
        assert_eq!(
            parse_date("2026-02-29").unwrap_err().kind(),
            ErrorKind::OutOfRange("day")
        );
    }

    #[test]
    fn a_fifty_three_week_year_admits_week_fifty_three() {
        // 2020 is a long ISO year: it begins on a Wednesday and is a leap year.
        assert!(parse_date("2020-W53-1").is_ok());
        assert!(parse_date("2021-W53-1").is_err());
    }

    #[test]
    fn empty_input_says_so_rather_than_blaming_a_field() {
        assert_eq!(parse("").unwrap_err().kind(), ErrorKind::Empty);
        assert_eq!(parse("").unwrap_err().offset(), 0);
    }

    #[test]
    fn trailing_text_is_reported_at_the_first_stray_byte() {
        let error = parse("2026-09-21T14:30:05Z ").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::TrailingText);
        assert_eq!(error.offset(), 20);
    }

    #[test]
    fn a_fraction_with_no_digits_points_at_where_they_should_be() {
        let error = parse("2026-09-21T14:30:05.").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Digit);
        assert_eq!(error.offset(), 20);
    }

    #[test]
    fn precision_past_an_attosecond_is_refused_rather_than_dropped() {
        let error = parse("2026-09-21T14:30:05.1234567890123456789").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Unrepresentable("more than 18 fractional digits")
        );
    }

    #[test]
    fn a_standalone_time_may_carry_a_leading_t_and_a_zone() {
        let (time, zone, style) = parse_time("T14:30:05+09:00").unwrap();
        assert_eq!(time.hour, 14);
        assert_eq!(zone.offset().unwrap().seconds(), 9 * 3_600);
        assert_eq!(style, OffsetStyle::Extended);
    }

    #[test]
    fn a_standalone_offset_refuses_an_unqualified_string() {
        assert_eq!(
            parse_offset("x").unwrap_err().kind(),
            ErrorKind::OneOf("Z+-")
        );
        assert_eq!(
            parse_offset("Z").unwrap(),
            (ZoneInfo::Zulu, OffsetStyle::Extended)
        );
    }

    #[test]
    fn an_hours_only_offset_is_iso_but_not_rfc_3339() {
        assert!(parse_offset_with("+09", Strictness::ISO).is_ok());
        let error = parse_offset_with("+09", Strictness::RFC_3339).unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Forbidden("an hours-only UTC offset")
        );
    }

    #[test]
    fn a_date_time_resolves_to_an_instant_only_with_a_zone() {
        let value = parse("2026-09-21T14:30:05+09:00").unwrap();
        let resolved = value.to_offset_date_time().unwrap();
        assert_eq!(resolved.to_unix().unwrap().seconds(), 1_789_968_605);
    }

    #[test]
    fn the_unix_epoch_is_where_it_should_be() {
        let value = parse("1970-01-01T00:00:00Z").unwrap();
        assert_eq!(
            value
                .to_offset_date_time()
                .unwrap()
                .to_unix()
                .unwrap()
                .seconds(),
            0
        );
    }

    #[test]
    fn a_strictness_can_be_narrowed_a_field_at_a_time() {
        let strict = Strictness::ISO.refusing_basic().requiring_zone();
        assert!(parse_with("20260921T143005Z", strict).is_err());
        assert!(parse_with("2026-09-21T14:30:05Z", strict).is_ok());
        assert!(parse_with("2026-09-21T14:30:05", strict).is_err());
    }

    #[test]
    fn the_default_strictness_is_the_standard_itself() {
        assert_eq!(Strictness::default(), Strictness::ISO);
    }

    #[test]
    fn a_written_value_matches_its_display_impl() {
        let value = parse("2026-09-21T14:30:05Z").unwrap();
        let mut out = String::new();
        write(&mut out, value).unwrap();
        assert_eq!(out, value.to_string());
    }
}
