//! The tab-separated lines the WebAssembly module and the C library write
//! about date-times, durations and intervals as text, written once.
//!
//! [`hc_format`] reads and writes the whole of ISO 8601 — calendar, ordinal
//! and week dates, reduced accuracy, date-times with a zone, durations,
//! intervals and repeating intervals — and RFC 3339, RFC 2822, the profile of
//! Python's `datetime.isoformat` and the pattern languages of `strptime` and
//! of CLDR. This module is the boundary's shape for them:
//!
//! * [`parse_datetime_line`] reads a date-time in one of six syntaxes into a
//!   *reading*, the cells [`READING_COLUMNS`] names, whose instant is
//!   absent when the text states no zone: `2026-09-21T14:30:05` is a reading
//!   on somebody's wall clock and is never taken for UTC;
//! * [`format_datetime_line`] writes an instant, with an offset and a
//!   precision, in one of eight;
//! * [`iso_date_parts_line`] and [`format_iso_date_line`] read and write the
//!   ISO 8601 calendar, ordinal and week dates, including those that
//!   name no day (`2026-W39`, `2026-09`);
//! * [`iso_duration_line`], [`format_iso_duration_line`] and
//!   [`iso_interval_line`] read and write durations, intervals and repeating
//!   intervals;
//! * [`parse_pattern_line`] reads a text against a `strftime`, Python
//!   `strptime` or CLDR pattern into the fields it found and the reading they
//!   resolve to.
//!
//! A text that is not in the syntax is [`Refusal::Malformed`], a date or time
//! that does not exist, 31 February or `24:00:01`, [`Refusal::InvalidDate`],
//! and a value this library cannot hold [`Refusal::OutOfRange`].

use alloc::string::{String, ToString};

use hc_calendar::{CivilDateTime, Rd};
use hc_core::UnixTime;
use hc_core::catalogue::matches;
use hc_format::iso8601::{self, Strictness, interval};
use hc_format::patterns::{ParsedFields, cldr, strftime};
use hc_format::python::{self, TimeSpec};
use hc_format::value::{DateParts, IsoDate, IsoDateTime, IsoTime, OffsetDateTime, ZoneInfo};
use hc_format::{
    ErrorKind, Fraction, IsoDuration, ParseError, Style, ValueError, rfc2822, rfc3339,
};
use hc_tz::{OffsetStyle, UtcOffset};

use crate::boundary::{Answer, Line, Refusal, line};

/// How many columns a reading has: [`parse_datetime_line`]'s line, and the
/// last cells of [`parse_pattern_line`]'s.
pub const READING_COLUMNS: usize = 8;

/// How many columns [`format_datetime_line`] writes.
pub const FORMATTED_COLUMNS: usize = 2;

/// How many columns [`iso_date_parts_line`] writes.
pub const DATE_PARTS_COLUMNS: usize = 9;

/// How many columns [`iso_duration_line`] writes.
pub const DURATION_COLUMNS: usize = 14;

/// How many columns [`iso_interval_line`] writes.
pub const INTERVAL_COLUMNS: usize = 10;

/// How many columns [`parse_pattern_line`] writes.
pub const PATTERN_COLUMNS: usize = 22 + READING_COLUMNS;

/// The refusal a parse error is: a date or a time that does not exist is
/// [`Refusal::InvalidDate`], a value the library cannot hold
/// [`Refusal::OutOfRange`], and any other text [`Refusal::Malformed`].
fn parse_refusal(error: &ParseError) -> Refusal {
    match error.kind() {
        ErrorKind::Invalid(_) | ErrorKind::OutOfRange(_) => Refusal::InvalidDate,
        ErrorKind::Unrepresentable(_) => Refusal::OutOfRange,
        _ => Refusal::Malformed,
    }
}

/// The refusal a parse error of a duration or an interval is: no field of
/// one is a calendar date, so a text that is not one is [`Refusal::Malformed`],
/// whatever the grammar's own reason.
fn text_refusal(error: &ParseError) -> Refusal {
    match error.kind() {
        ErrorKind::Unrepresentable(_) => Refusal::OutOfRange,
        _ => Refusal::Malformed,
    }
}

/// The refusal a value error is: a text that names no reading is
/// [`Refusal::Malformed`], arithmetic that left the range
/// [`Refusal::OutOfRange`], a calendar's own refusal its own.
fn value_refusal(error: ValueError) -> Refusal {
    match error {
        ValueError::Calendar(error) => Refusal::from(error),
        ValueError::Zone(_) | ValueError::Time(_) => Refusal::OutOfRange,
        _ => Refusal::Malformed,
    }
}

/// The word a zone is written as in a reading: `none` for no designator, `utc`
/// for `Z`, `offset` for a numeric one, and `unknown-local` for RFC 3339's
/// `-00:00`.
const fn zone_word(zone: ZoneInfo) -> &'static str {
    match zone {
        ZoneInfo::Unspecified => "none",
        ZoneInfo::Zulu => "utc",
        ZoneInfo::Offset(_) => "offset",
        ZoneInfo::UnknownLocalOffset => "unknown-local",
    }
}

/// The cells of a reading, the shape every date-time here is written in:
///
/// 1. the local fixed day;
/// 2. the local second of the day, 0 through 86 400 (`23:59:60` is the
///    86 400th second, written as it was);
/// 3. the attoseconds into that second;
/// 4. the zone, `none`, `utc`, `offset` or `unknown-local`;
/// 5. the offset in seconds east of UTC, empty when no zone was stated;
/// 6. the POSIX second of the instant, empty when no zone was stated;
/// 7. `1` for an inserted leap second, `23:59:60`, which POSIX counts as the
///    second after it;
/// 8. `1` when the text wrote the end of a day as `24:00`.
fn reading_cells(line: &mut Line<'_>, value: &OffsetDateTime) {
    let time = value.local.time;
    let second_of_day =
        u32::from(time.hour()) * 3_600 + u32::from(time.minute()) * 60 + u32::from(time.second());
    line.value(value.local.day.0)
        .value(second_of_day)
        .value(time.subsec_attos())
        .cell(zone_word(value.zone));
    line.value_or_empty(value.zone.offset().map(UtcOffset::seconds))
        .value_or_empty(value.to_unix().ok().map(UnixTime::seconds))
        .flag(time.is_leap_second())
        .flag(value.written_as_end_of_day);
}

/// A reading as a line.
fn reading_line(value: &OffsetDateTime) -> String {
    line(|line| reading_cells(line, value))
}

/// The line of `hc_parse_datetime`: a date-time read in a syntax, as a
/// reading ([`READING_COLUMNS`]). `syntax` is `iso8601`, everything ISO
/// 8601-1 allows of a date and a time — basic and extended, ordinal and week
/// dates, `24:00`, `23:59:60`, a decimal fraction of the lowest component;
/// `iso8601-full`, complete extended values with a zone only; `rfc3339`, the
/// internet profile; `rfc2822`, email and HTTP dates, obsolete syntax
/// included; `python`, `datetime.fromisoformat` of Python 3.13, a date alone
/// being midnight; and `auto`, which tells an ISO date-time from an email
/// date by its letters.
///
/// A text with no zone is a reading and not an instant: cells 5 and 6 are
/// empty. A date alone or a date of reduced accuracy names no reading and is
/// [`Refusal::Malformed`] under every syntax but `python`; [`iso_date_parts_line`]
/// reads them.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a syntax not named; [`Refusal::Malformed`] for text
/// not in it; [`Refusal::InvalidDate`] for a date or time that does not exist;
/// [`Refusal::OutOfRange`] for one the library cannot hold.
pub fn parse_datetime_line(syntax: &str, text: &str) -> Answer<String> {
    let value = if matches(syntax, "iso8601") {
        iso8601::parse(text)
            .map_err(|error| parse_refusal(&error))?
            .to_offset_date_time()
            .map_err(value_refusal)?
    } else if matches(syntax, "iso8601-full") {
        iso8601::parse_with(text, Strictness::FULL)
            .map_err(|error| parse_refusal(&error))?
            .to_offset_date_time()
            .map_err(value_refusal)?
    } else if matches(syntax, "rfc3339") {
        rfc3339::parse(text).map_err(|error| parse_refusal(&error))?
    } else if matches(syntax, "rfc2822") {
        rfc2822::parse(text).map_err(|error| parse_refusal(&error))?
    } else if matches(syntax, "python") {
        python::parse_date_time(text).map_err(|error| parse_refusal(&error))?
    } else if matches(syntax, "auto") {
        hc_format::parse::date_time(text).map_err(|error| parse_refusal(&error))?
    } else {
        return Err(Refusal::Unknown);
    };
    Ok(reading_line(&value))
}

/// The digits an expanded year of this magnitude is written with: six, and
/// more for a year of a million or beyond.
fn expanded_digits(year: i64) -> u8 {
    let mut digits = 6u8;
    let mut limit = 1_000_000u64;
    while year.unsigned_abs() >= limit && digits < 18 {
        digits += 1;
        limit *= 10;
    }
    digits
}

/// How much of a time to write: what a caller's `precision` names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Precision {
    /// As many digits as the instant needs and no more.
    Auto,
    Hours,
    Minutes,
    Seconds,
    /// This many digits of a second.
    Digits(u8),
}

/// The precision a name stands for: `auto`, `hours`, `minutes`, `seconds`,
/// `milliseconds`, `microseconds` or `nanoseconds`, in any case.
fn precision_of(name: &str) -> Answer<Precision> {
    [
        ("auto", Precision::Auto),
        ("hours", Precision::Hours),
        ("minutes", Precision::Minutes),
        ("seconds", Precision::Seconds),
        ("milliseconds", Precision::Digits(3)),
        ("microseconds", Precision::Digits(6)),
        ("nanoseconds", Precision::Digits(9)),
    ]
    .into_iter()
    .find(|(word, _)| matches(name, word))
    .map(|(_, precision)| precision)
    .ok_or(Refusal::Unknown)
}

/// The fraction of a second `precision` keeps, truncated, not rounded.
fn kept_fraction(attos: u64, digits: u8) -> Option<Fraction> {
    let scale = 10u64.pow(u32::from(18 - digits));
    Fraction::new(attos / scale * scale, digits)
}

/// The form an ISO 8601 date is written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DateForm {
    Calendar,
    Ordinal,
    Week,
}

fn date_form_of(name: &str) -> Answer<DateForm> {
    [
        ("calendar", DateForm::Calendar),
        ("ordinal", DateForm::Ordinal),
        ("week", DateForm::Week),
    ]
    .into_iter()
    .find(|(word, _)| matches(name, word))
    .map(|(_, form)| form)
    .ok_or(Refusal::Unknown)
}

fn style_of(name: &str) -> Answer<Style> {
    if matches(name, "extended") {
        Ok(Style::Extended)
    } else if matches(name, "basic") {
        Ok(Style::Basic)
    } else {
        Err(Refusal::Unknown)
    }
}

/// The ISO 8601 date of a fixed day, in a form and a style.
fn iso_date_of(fixed: Rd, form: DateForm, style: Style) -> Answer<IsoDate> {
    let mut date = IsoDate::from_fixed(fixed).map_err(value_refusal)?;
    date.style = style;
    date.parts = match form {
        DateForm::Calendar => date.parts,
        DateForm::Ordinal => {
            let (year, day_of_year) =
                hc_calendars_solar::ordinal::from_fixed(fixed).map_err(Refusal::from)?;
            DateParts::Ordinal { year, day_of_year }
        }
        DateForm::Week => {
            let (year, week, weekday) =
                hc_calendars_solar::iso_week::from_fixed(fixed).map_err(Refusal::from)?;
            DateParts::Week {
                year,
                week,
                weekday: Some(weekday),
            }
        }
    };
    // The week-numbering year of a week date can lie outside `0000..=9999`
    // where the calendar year does not, and the other way round: the style
    // follows the year that is written.
    if !matches!(date.parts, DateParts::Calendar { .. }) {
        let written = date.parts.year();
        date.year_style = if (0..=9_999).contains(&written) {
            hc_format::YearStyle::Plain
        } else {
            hc_format::YearStyle::Expanded(expanded_digits(written))
        };
    }
    Ok(date)
}

/// The line of `hc_format_iso_date`'s wider sibling, `hc_format_iso_date_as`:
/// a fixed day written as an ISO 8601 `calendar` (`2026-09-21`), `ordinal`
/// (`2026-264`) or `week` (`2026-W39-1`) date, in the `extended` style or the
/// `basic` one with no separators (`20260921`, `2026264`, `2026W391`), then
/// the form and the style. A year outside `0000..=9999` is written with a sign
/// and six or more digits, the expanded form ISO 8601 gives it.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a form or style not named, and
/// [`Refusal::OutOfRange`] for a day outside the Gregorian range or a week
/// date whose year does not fit.
pub fn format_iso_date_line(fixed: i64, form: &str, style: &str) -> Answer<String> {
    let form = date_form_of(form)?;
    let style = style_of(style)?;
    let date = iso_date_of(Rd(fixed), form, style)?;
    let mut text = String::new();
    date.write(&mut text).map_err(|_| Refusal::OutOfRange)?;
    Ok(line(|line| {
        line.cell(&text)
            .cell(match form {
                DateForm::Calendar => "calendar",
                DateForm::Ordinal => "ordinal",
                DateForm::Week => "week",
            })
            .cell(if style == Style::Basic {
                "basic"
            } else {
                "extended"
            });
    }))
}

/// The line of `hc_iso_date_parts`: an ISO 8601 date read into its parts,
/// including one that names no day — `2026`, `2026-09`, `2026-W39` — which
/// no instant and no fixed day stand for. The cells: the form, `calendar`,
/// `ordinal` or `week`; the year, the week-numbering year for a week date;
/// the month, the day of the month, the day of the year, the week and the
/// weekday (1 Monday to 7 Sunday), each empty where the form has none or
/// the text left it out; the fixed day, empty when the date names none; and
/// `basic` or `extended`.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not in the syntax and
/// [`Refusal::InvalidDate`] for a date that does not exist.
pub fn iso_date_parts_line(text: &str) -> Answer<String> {
    let date = iso8601::parse_date(text).map_err(|error| parse_refusal(&error))?;
    let fixed = date.to_fixed().ok();
    Ok(line(|line| {
        let (form, month, day, day_of_year, week, weekday) = match date.parts {
            DateParts::Calendar { month, day, .. } => (
                "calendar",
                month.map(i64::from),
                day.map(i64::from),
                None,
                None,
                None,
            ),
            DateParts::Ordinal { day_of_year, .. } => (
                "ordinal",
                None,
                None,
                Some(i64::from(day_of_year)),
                None,
                None,
            ),
            DateParts::Week { week, weekday, .. } => (
                "week",
                None,
                None,
                None,
                Some(i64::from(week)),
                weekday.map(i64::from),
            ),
        };
        line.cell(form)
            .value(date.parts.year())
            .value_or_empty(month)
            .value_or_empty(day)
            .value_or_empty(day_of_year)
            .value_or_empty(week)
            .value_or_empty(weekday)
            .value_or_empty(fixed.map(|day| day.0))
            .cell(if date.style == Style::Basic {
                "basic"
            } else {
                "extended"
            });
    }))
}

/// The line of `hc_format_datetime`: an instant — the POSIX second and the
/// attoseconds into it — written in the zone of a numeric offset, in a
/// syntax and to a precision; then the syntax. `syntax` is `iso8601`,
/// `iso8601-basic`, `iso8601-ordinal` and `iso8601-week` (the date in that
/// form, `T`, the time, and `Z` for a zero offset or `+09:00`, `+0900` in the
/// basic one), `rfc3339`, `rfc2822` (`Mon, 21 Sep 2026 14:30:05 +0900`),
/// `imf-fixdate` (HTTP's, `Mon, 21 Sep 2026 05:30:05 GMT`, always the UTC
/// reading whatever the offset) and `python` (`datetime.isoformat`, `+00:00`
/// for UTC, never `Z`). `precision` is `auto`, which writes the digits the
/// instant needs and none for a whole second; `hours`, `minutes` or
/// `seconds`; or `milliseconds`, `microseconds` or `nanoseconds`; a time is
/// truncated to it, never rounded. RFC 3339 has no `hours` or `minutes`, RFC
/// 2822 and HTTP no sub-second digits and Python no nanoseconds; a
/// precision a syntax does not have is [`Refusal::Unknown`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for a syntax or precision not named or not had by
/// the syntax, and [`Refusal::OutOfRange`] for an attosecond count of 10¹⁸ or
/// more, an offset beyond ±25:59:59, the range of `hc-tz`'s offsets, or an instant whose year `0000..=9999`
/// does not hold where the syntax needs one (RFC 3339, RFC 2822, HTTP).
pub fn format_datetime_line(
    syntax: &str,
    unix_seconds: i64,
    attos: u64,
    offset_seconds: i32,
    precision: &str,
) -> Answer<String> {
    let precision = precision_of(precision)?;
    let unix = UnixTime::new(unix_seconds, attos).map_err(|_| Refusal::OutOfRange)?;
    let offset = UtcOffset::from_seconds(offset_seconds).map_err(|_| Refusal::OutOfRange)?;
    let zone = if offset.is_utc() {
        ZoneInfo::Zulu
    } else {
        ZoneInfo::Offset(offset)
    };
    let value = OffsetDateTime::from_unix(unix, zone).map_err(value_refusal)?;
    let mut out = String::new();
    let word = if let Some(form) = syntax.strip_prefix_ignore_case("iso8601") {
        let (date_form, style, word) = if form.is_empty() {
            (DateForm::Calendar, Style::Extended, "iso8601")
        } else if matches(form, "-basic") {
            (DateForm::Calendar, Style::Basic, "iso8601-basic")
        } else if matches(form, "-ordinal") {
            (DateForm::Ordinal, Style::Extended, "iso8601-ordinal")
        } else if matches(form, "-week") {
            (DateForm::Week, Style::Extended, "iso8601-week")
        } else {
            return Err(Refusal::Unknown);
        };
        let date = iso_date_of(value.local.day, date_form, style)?;
        let time = value.local.time;
        let (minute, second, fraction) = match precision {
            Precision::Auto => (
                Some(time.minute()),
                Some(time.second()),
                Fraction::minimal(time.subsec_attos()),
            ),
            Precision::Hours => (None, None, None),
            Precision::Minutes => (Some(time.minute()), None, None),
            Precision::Seconds => (Some(time.minute()), Some(time.second()), None),
            Precision::Digits(digits) => (
                Some(time.minute()),
                Some(time.second()),
                kept_fraction(time.subsec_attos(), digits),
            ),
        };
        let iso_time = IsoTime {
            hour: time.hour(),
            minute,
            second,
            fraction,
            style,
            mark: hc_format::DecimalMark::Point,
        };
        let zone_style = if style == Style::Basic {
            OffsetStyle::Basic
        } else {
            OffsetStyle::Extended
        };
        IsoDateTime {
            date,
            time: Some(iso_time),
            zone,
            zone_style,
        }
        .write(&mut out)
        .map_err(|_| Refusal::OutOfRange)?;
        word
    } else if matches(syntax, "rfc3339") {
        let precision = match precision {
            Precision::Auto => rfc3339::SubsecondPrecision::Auto,
            Precision::Seconds => rfc3339::SubsecondPrecision::Seconds,
            Precision::Digits(digits) => rfc3339::SubsecondPrecision::Digits(digits),
            Precision::Hours | Precision::Minutes => return Err(Refusal::Unknown),
        };
        rfc3339::write(&mut out, value, precision).map_err(|_| Refusal::OutOfRange)?;
        "rfc3339"
    } else if matches(syntax, "rfc2822") || matches(syntax, "imf-fixdate") {
        if !matches!(precision, Precision::Auto | Precision::Seconds) {
            return Err(Refusal::Unknown);
        }
        if matches(syntax, "rfc2822") {
            rfc2822::write(&mut out, value).map_err(|_| Refusal::OutOfRange)?;
            "rfc2822"
        } else {
            let utc: CivilDateTime = value.to_utc_civil().map_err(value_refusal)?;
            rfc2822::write_imf_fixdate(&mut out, utc).map_err(|_| Refusal::OutOfRange)?;
            "imf-fixdate"
        }
    } else if matches(syntax, "python") {
        let spec = match precision {
            Precision::Auto => TimeSpec::Auto,
            Precision::Hours => TimeSpec::Hours,
            Precision::Minutes => TimeSpec::Minutes,
            Precision::Seconds => TimeSpec::Seconds,
            Precision::Digits(3) => TimeSpec::Milliseconds,
            Precision::Digits(6) => TimeSpec::Microseconds,
            Precision::Digits(_) => return Err(Refusal::Unknown),
        };
        python::write_date_time(&mut out, value.local, ZoneInfo::Offset(offset), 'T', spec)
            .map_err(|_| Refusal::OutOfRange)?;
        "python"
    } else {
        return Err(Refusal::Unknown);
    };
    Ok(line(|line| {
        line.cell(&out).cell(word);
    }))
}

/// A `strip_prefix` that ignores ASCII case.
trait StripPrefixIgnoreCase {
    fn strip_prefix_ignore_case(&self, prefix: &str) -> Option<&str>;
}

impl StripPrefixIgnoreCase for str {
    fn strip_prefix_ignore_case(&self, prefix: &str) -> Option<&str> {
        let head = self.trim().get(..prefix.len())?;
        head.eq_ignore_ascii_case(prefix)
            .then(|| &self.trim()[prefix.len()..])
    }
}

/// The unsigned count a duration component is written with, or none for
/// the absent: negative is absent.
fn component(value: i64) -> Option<u64> {
    u64::try_from(value).ok()
}

/// The cells after a duration's own: the text it is written as, whether it
/// is nominal, and its exact length.
fn duration_exact_cells(line: &mut Line<'_>, duration: IsoDuration) {
    let mut text = String::new();
    let written = duration.write(&mut text).is_ok();
    line.cell(if written { &text } else { "" })
        .flag(duration.is_nominal());
    match duration.to_exact_duration() {
        Ok(exact) => line
            .value(exact.whole_seconds())
            .value(exact.subsec_attos()),
        Err(_) => line.empties(2),
    };
}

/// The line of `hc_iso_duration`: an ISO 8601 duration read into its
/// components. The cells: `1` for a leading minus, ISO 8601-2's; the years,
/// months, weeks, days, hours, minutes and seconds, each empty where the text
/// did not write it; the digits of the decimal fraction of the lowest
/// component, as written (`5`, `250`), empty for none; the form,
/// `designators` (`P1Y2M3DT4H5M6S`) or `alternative` (`P0001-02-03T04:05:06`);
/// then the duration written in canonical form; `1` when it is nominal, with
/// years or months, which have no fixed length; and its exact length, whole
/// seconds and the attoseconds after them, empty for a nominal one.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not in the syntax and
/// [`Refusal::InvalidDate`] for a date that does not exist.
pub fn iso_duration_line(text: &str) -> Answer<String> {
    let duration = iso8601::duration::parse(text).map_err(|error| text_refusal(&error))?;
    Ok(line(|line| {
        line.flag(duration.negative);
        for value in [
            duration.years,
            duration.months,
            duration.weeks,
            duration.days,
            duration.hours,
            duration.minutes,
            duration.seconds,
        ] {
            line.value_or_empty(value);
        }
        match duration.fraction {
            Some(fraction) => {
                let mut digits = String::new();
                let _ = fraction.write_digits(&mut digits);
                line.cell(&digits)
            }
            None => line.empty(),
        };
        line.cell(match duration.form {
            iso8601::DurationForm::Designators => "designators",
            _ => "alternative",
        });
        duration_exact_cells(line, duration);
    }))
}

/// The line of `hc_format_iso_duration`: the duration the components make,
/// written in the designator form, then the text and the same cells as
/// [`iso_duration_line`] from the nominal flag. A component below zero is
/// absent. `fraction` is the digits of a decimal fraction of the lowest
/// component present, empty for none.
///
/// # Errors
///
/// [`Refusal::Malformed`] for components ISO 8601 has no spelling for — none
/// at all, a fraction on a component that is not the lowest, weeks beside
/// other components — and for a fraction that is not up to 18 digits.
#[allow(clippy::too_many_arguments)]
pub fn format_iso_duration_line(
    negative: bool,
    years: i64,
    months: i64,
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    fraction: &str,
) -> Answer<String> {
    let fraction = if fraction.is_empty() {
        None
    } else {
        let digits = u8::try_from(fraction.len()).map_err(|_| Refusal::Malformed)?;
        if !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Refusal::Malformed);
        }
        let value: u64 = fraction.parse().map_err(|_| Refusal::Malformed)?;
        let scale = 10u64
            .checked_pow(u32::from(18u8.saturating_sub(digits)))
            .ok_or(Refusal::Malformed)?;
        Some(
            Fraction::new(value.checked_mul(scale).ok_or(Refusal::Malformed)?, digits)
                .ok_or(Refusal::Malformed)?,
        )
    };
    if [years, months, weeks, days, hours, minutes, seconds]
        .iter()
        .all(|value| *value < 0)
    {
        // ISO 8601 has no empty duration; zero is `PT0S`, written with a
        // component.
        return Err(Refusal::Malformed);
    }
    let duration = IsoDuration {
        negative,
        years: component(years),
        months: component(months),
        weeks: component(weeks),
        days: component(days),
        hours: component(hours),
        minutes: component(minutes),
        seconds: component(seconds),
        fraction,
        ..IsoDuration::default()
    };
    let mut text = String::new();
    duration.write(&mut text).map_err(|_| Refusal::Malformed)?;
    // Written text is the proof the combination exists: read it back.
    let canonical = iso8601::duration::parse(&text).map_err(|_| Refusal::Malformed)?;
    Ok(line(|line| {
        line.cell(&text).flag(canonical.is_nominal());
        match canonical.to_exact_duration() {
            Ok(exact) => line
                .value(exact.whole_seconds())
                .value(exact.subsec_attos()),
            Err(_) => line.empties(2),
        };
    }))
}

/// The cells of an interval's end: the date-time written back, and the whole
/// POSIX second it names where it names an instant, empty where the text
/// states no zone or no time.
fn endpoint_cells(line: &mut Line<'_>, value: IsoDateTime) {
    line.cell(&value.to_string());
    match value
        .to_offset_date_time()
        .and_then(|reading| reading.to_unix())
    {
        Ok(unix) => line.value(unix.seconds()),
        Err(_) => line.empty(),
    };
}

/// The line of `hc_iso_interval`: an ISO 8601 interval — `start/end`,
/// `start/duration`, `duration/end` or a duration alone — or a repeating one,
/// `R5/…`, `R/…`. The cells: the repetitions, empty for an interval that is
/// not repeating, a count, or `inf` for `R/`; the shape, `start-end`,
/// `start-duration`, `duration-end` or `duration`; the start as written back
/// and its POSIX second; the end likewise; the duration as written back,
/// whether it is nominal and its exact whole seconds and attoseconds. A
/// cell the shape has no part for is empty, and so is a POSIX second where the
/// text states no zone or no time: it is a reading and not an instant. The
/// POSIX second is whole; a decimal fraction of the second is in the text.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not in the syntax and
/// [`Refusal::InvalidDate`] for a date that does not exist.
pub fn iso_interval_line(text: &str) -> Answer<String> {
    let trimmed = text.trim_start();
    let (repetitions, interval) = if trimmed.starts_with('R') {
        let repeating = interval::parse_repeating(text).map_err(|error| text_refusal(&error))?;
        (Some(repeating.repetitions), repeating.interval)
    } else {
        (
            None,
            interval::parse(text).map_err(|error| text_refusal(&error))?,
        )
    };
    Ok(line(|line| {
        match repetitions {
            None => line.empty(),
            Some(None) => line.cell("inf"),
            Some(Some(count)) => line.value(count),
        };
        let (start, end) = match interval {
            iso8601::Interval::StartEnd(start, end) => (Some(start), Some(end)),
            iso8601::Interval::StartDuration(start, _) => (Some(start), None),
            iso8601::Interval::DurationEnd(_, end) => (None, Some(end)),
            iso8601::Interval::Duration(_) => (None, None),
        };
        line.cell(match interval {
            iso8601::Interval::StartEnd(..) => "start-end",
            iso8601::Interval::StartDuration(..) => "start-duration",
            iso8601::Interval::DurationEnd(..) => "duration-end",
            iso8601::Interval::Duration(_) => "duration",
        });
        match start {
            Some(value) => endpoint_cells(line, value),
            None => {
                line.empties(2);
            }
        }
        match end {
            Some(value) => endpoint_cells(line, value),
            None => {
                line.empties(2);
            }
        }
        match interval.duration() {
            Some(duration) => duration_exact_cells(line, duration),
            None => {
                line.empties(4);
            }
        }
    }))
}

/// How a pattern is written: the three languages `hc-format` reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatternSyntax {
    Strftime,
    Python,
    Cldr,
}

fn pattern_syntax_of(name: &str) -> Answer<PatternSyntax> {
    if matches(name, "strftime") || matches(name, "strptime") {
        Ok(PatternSyntax::Strftime)
    } else if matches(name, "python") {
        Ok(PatternSyntax::Python)
    } else if matches(name, "cldr") {
        Ok(PatternSyntax::Cldr)
    } else {
        Err(Refusal::Unknown)
    }
}

/// The fields a pattern-driven parse found, the cells
/// [`parse_pattern_line`] writes before the reading.
fn field_cells(line: &mut Line<'_>, fields: &ParsedFields) {
    line.value_or_empty(fields.year)
        .value_or_empty(fields.century)
        .value_or_empty(fields.year_of_century)
        .value_or_empty(fields.month)
        .value_or_empty(fields.day)
        .value_or_empty(fields.day_of_year)
        .value_or_empty(fields.iso_year)
        .value_or_empty(fields.iso_week)
        .value_or_empty(fields.iso_weekday)
        .value_or_empty(fields.week_of_year_sunday)
        .value_or_empty(fields.week_of_year_monday)
        .value_or_empty(fields.hour)
        .value_or_empty(fields.hour12)
        .cell_or_empty(fields.day_period.map(|period| match period {
            hc_i18n::names::DayPeriod::Am => "am",
            hc_i18n::names::DayPeriod::Pm => "pm",
        }))
        .value_or_empty(fields.minute)
        .value_or_empty(fields.second)
        .value_or_empty(fields.subsec_attos)
        .cell_or_empty(fields.zone.map(zone_word))
        .value_or_empty(
            fields
                .zone
                .and_then(ZoneInfo::offset)
                .map(UtcOffset::seconds),
        )
        .value_or_empty(fields.unix_seconds)
        .cell_or_empty(fields.era.map(|era| if era == 0 { "bce" } else { "ce" }))
        .value_or_empty(fields.rd.map(|day| day.0));
}

/// The line of `hc_parse_pattern`: a text read against a pattern, as the
/// fields the pattern named and the reading they resolve to. `syntax` is
/// `strftime` (POSIX `strptime`, whose names are the C locale's), `python`
/// (`datetime.strptime` of CPython: its alternatives, backtracking and
/// resolution, with what the text leaves out taken from 1900-01-01) or `cldr`
/// (a pattern of UTS #35 Part 4, `yyyy-MM-dd HH:mm`). The cells are, each
/// empty where the pattern did not read it: the year; the century; the year
/// of the century; the month; the day; the day of the year; the week-numbering
/// year; the ISO week; the ISO weekday; the `%U` and `%W` week numbers; the hour
/// on a 24-hour clock and on a 12-hour one; `am` or `pm`; the minute; the
/// second; the attoseconds; the zone, `utc`, `offset`, `unknown-local`; its
/// offset in seconds; the POSIX second a `%s` read; `ce` or `bce`; and the
/// fixed day a CLDR `g` read; then the eight cells of a reading, empty where the
/// fields name no whole date and time, since a pattern of `%H:%M` names no
/// day; a date or time the fields name that does not exist, 30 February, is
/// [`Refusal::InvalidDate`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for a syntax not named and [`Refusal::Malformed`]
/// for a text that does not match the pattern, or a pattern the library does
/// not read.
pub fn parse_pattern_line(syntax: &str, pattern: &str, text: &str) -> Answer<String> {
    parse_pattern_with(syntax, pattern, text, None)
}

/// [`parse_pattern_line`] with the names of a locale: the month names, the
/// weekdays and the day periods the locale writes, besides the C locale's.
/// Only `strftime` and `cldr` take a locale; `python`'s `strptime` reads the
/// C locale's names alone, and is [`Refusal::Unknown`] with one.
///
/// # Errors
///
/// As [`parse_pattern_line`].
#[cfg(feature = "i18n")]
pub fn parse_pattern_in_line(
    syntax: &str,
    pattern: &str,
    text: &str,
    locale: &hc_i18n::Locale,
) -> Answer<String> {
    parse_pattern_with(syntax, pattern, text, Some(locale))
}

fn parse_pattern_with(
    syntax: &str,
    pattern: &str,
    text: &str,
    locale: Option<&hc_i18n::Locale>,
) -> Answer<String> {
    let syntax = pattern_syntax_of(syntax)?;
    let fields = match (syntax, locale) {
        (PatternSyntax::Strftime, locale) => strftime::parse_with_locale(pattern, text, locale),
        (PatternSyntax::Cldr, locale) => cldr::parse_with_locale(pattern, text, locale),
        (PatternSyntax::Python, None) => python::strptime(text, pattern),
        (PatternSyntax::Python, Some(_)) => return Err(Refusal::Unknown),
    }
    .map_err(|error| parse_refusal(&error))?;
    let resolved = fields.to_offset_date_time();
    // A pattern that names no whole date and time has no reading, and the
    // line says so with empty cells; fields that name a day or a time that
    // does not exist are refused, not left for the caller to find out.
    let resolved = match resolved {
        Ok(value) => Some(value),
        Err(ValueError::MissingField(_)) => None,
        Err(error) => return Err(value_refusal(error)),
    };
    Ok(line(|line| {
        field_cells(line, &fields);
        match &resolved {
            Some(value) => reading_cells(line, value),
            None => {
                line.empties(READING_COLUMNS);
            }
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::cells;
    use alloc::vec::Vec;

    fn owned(line: &str) -> Vec<String> {
        cells(line).iter().map(|cell| (*cell).to_owned()).collect()
    }

    fn reading(syntax: &str, text: &str) -> Vec<String> {
        owned(&parse_datetime_line(syntax, text).expect("a reading"))
    }

    /// RFC 3339 §5.8's examples, with the instants Python's `datetime`
    /// gives them: 1985-04-12T23:20:50.52Z is 482 196 050 s (RD 724 743),
    /// 1996-12-19T16:39:57-08:00 is 851 042 397 s, and the 1937 reading
    /// 12:00:27.87 at +00:20 is −1 041 337 172.13 s, which Python's `int`
    /// truncates toward zero and the POSIX second here floors, −1 041 337 173,
    /// with the .87 in the attosecond cell.
    #[test]
    fn rfc_3339_readings_are_instants() {
        assert_eq!(
            reading("rfc3339", "1985-04-12T23:20:50.52Z"),
            [
                "724743",
                "84050",
                "520000000000000000",
                "utc",
                "0",
                "482196050",
                "0",
                "0"
            ]
        );
        assert_eq!(
            reading("rfc3339", "1996-12-19T16:39:57-08:00"),
            [
                "729012",
                "59997",
                "0",
                "offset",
                "-28800",
                "851042397",
                "0",
                "0"
            ]
        );
        assert_eq!(
            reading("rfc3339", "1937-01-01T12:00:27.87+00:20"),
            [
                "707110",
                "43227",
                "870000000000000000",
                "offset",
                "1200",
                "-1041337173",
                "0",
                "0"
            ]
        );
        // -00:00 is an instant whose local offset is not known (RFC 3339 4.3).
        assert_eq!(
            reading("rfc3339", "2026-09-21T14:30:05-00:00")[3],
            "unknown-local"
        );
        // `1990-12-31T23:59:60Z`, RFC 3339's leap second, is the second
        // POSIX counts as 1991-01-01T00:00:00Z, 662 688 000.
        assert_eq!(
            reading("rfc3339", "1990-12-31T23:59:60Z"),
            ["726832", "86400", "0", "utc", "0", "662688000", "1", "0"]
        );
        assert_eq!(
            parse_datetime_line("rfc3339", "20260921T143005Z"),
            Err(Refusal::Malformed)
        );
    }

    /// A text with no zone is a reading and never UTC; ISO 8601's other
    /// forms; Python's `fromisoformat` takes a date as midnight.
    #[test]
    fn a_reading_with_no_zone_has_no_instant() {
        assert_eq!(
            reading("iso8601", "2026-09-21T14:30:05"),
            ["739880", "52205", "0", "none", "", "", "0", "0"]
        );
        assert_eq!(reading("iso8601", "20260921T143005+0900")[5], "1789968605");
        assert_eq!(
            reading("iso8601", "2026-264T14:30:05+09:00")[5],
            "1789968605"
        );
        assert_eq!(
            reading("iso8601", "2026-W39-1T14:30:05+09:00")[5],
            "1789968605"
        );
        assert_eq!(reading("iso8601", "2026-09-21T24:00")[7], "1");
        assert_eq!(
            reading("iso8601-full", "2026-09-21T14:30:05+09:00")[5],
            "1789968605"
        );
        assert_eq!(
            parse_datetime_line("iso8601-full", "2026-09-21T14:30:05"),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            reading("python", "2026-09-21"),
            ["739880", "0", "0", "none", "", "", "0", "0"]
        );
        assert_eq!(
            parse_datetime_line("iso8601", "2026-09-21"),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            parse_datetime_line("iso8601", "2026-W39"),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            parse_datetime_line("iso8601", "2026-02-30T00:00:00Z"),
            Err(Refusal::InvalidDate)
        );
        assert_eq!(
            parse_datetime_line("iso8601", "yesterday"),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            parse_datetime_line("ISO8601", "2026-09-21T14:30Z").map(|line| owned(&line)[1].clone()),
            Ok("52200".to_owned())
        );
        assert_eq!(parse_datetime_line("julian", "x"), Err(Refusal::Unknown));
    }

    /// An RFC 822-family example date (`Fri, 21 Nov 1997 09:55:06 -0600`; RFC
    /// 5322's appendix was not read) is 880 127 706 s by Python's `email.utils`; RFC 9110's IMF-fixdate example is `Sun, 06 Nov
    /// 1994 08:49:37 GMT`, 784 111 777 s; `auto` tells the two apart.
    #[test]
    fn an_email_date_is_read_as_one() {
        assert_eq!(
            reading("rfc2822", "Fri, 21 Nov 1997 09:55:06 -0600"),
            [
                "729349",
                "35706",
                "0",
                "offset",
                "-21600",
                "880127706",
                "0",
                "0"
            ]
        );
        assert_eq!(
            reading("rfc2822", "Sun, 06 Nov 1994 08:49:37 GMT")[5],
            "784111777"
        );
        assert_eq!(
            reading("auto", "Sun, 06 Nov 1994 08:49:37 GMT")[5],
            "784111777"
        );
        assert_eq!(reading("auto", "1985-04-12T23:20:50.52Z")[5], "482196050");
        assert_eq!(
            parse_datetime_line("rfc2822", "2026-09-21"),
            Err(Refusal::Malformed)
        );
    }

    /// 2026-09-21T14:30:05+09:00 is 1 789 968 605 s; its week date is
    /// 2026-W39-1 and its ordinal date 2026-264 (Python's `isocalendar` and
    /// `%j`); `datetime.isoformat` writes `+00:00` for UTC.
    #[test]
    fn an_instant_is_written_in_each_syntax() {
        let write = |syntax: &str, offset: i32, attos: u64, precision: &str| {
            owned(
                &format_datetime_line(syntax, 1_789_968_605, attos, offset, precision)
                    .expect("a text"),
            )[0]
            .clone()
        };
        assert_eq!(
            write("iso8601", 32_400, 0, "auto"),
            "2026-09-21T14:30:05+09:00"
        );
        assert_eq!(write("iso8601", 0, 0, "auto"), "2026-09-21T05:30:05Z");
        assert_eq!(
            write("iso8601-basic", 32_400, 0, "auto"),
            "20260921T143005+0900"
        );
        assert_eq!(
            write("iso8601-ordinal", 32_400, 0, "auto"),
            "2026-264T14:30:05+09:00"
        );
        assert_eq!(
            write("iso8601-week", 32_400, 0, "auto"),
            "2026-W39-1T14:30:05+09:00"
        );
        assert_eq!(
            write("iso8601", 32_400, 500_000_000_000_000_000, "auto"),
            "2026-09-21T14:30:05.5+09:00"
        );
        assert_eq!(
            write("iso8601", 32_400, 999_999_999_000_000_000, "milliseconds"),
            "2026-09-21T14:30:05.999+09:00"
        );
        assert_eq!(
            write("iso8601", 32_400, 0, "minutes"),
            "2026-09-21T14:30+09:00"
        );
        assert_eq!(write("iso8601", 32_400, 0, "hours"), "2026-09-21T14+09:00");
        assert_eq!(
            write("rfc3339", 32_400, 0, "milliseconds"),
            "2026-09-21T14:30:05.000+09:00"
        );
        assert_eq!(
            write("rfc2822", 32_400, 0, "auto"),
            "Mon, 21 Sep 2026 14:30:05 +0900"
        );
        assert_eq!(
            write("imf-fixdate", 32_400, 0, "seconds"),
            "Mon, 21 Sep 2026 05:30:05 GMT"
        );
        assert_eq!(
            write("python", 0, 123_456_000_000_000_000, "auto"),
            "2026-09-21T05:30:05.123456+00:00"
        );
        assert_eq!(
            write("python", 0, 0, "milliseconds"),
            "2026-09-21T05:30:05.000+00:00"
        );
        for (syntax, precision) in [
            ("rfc3339", "minutes"),
            ("rfc2822", "milliseconds"),
            ("python", "nanoseconds"),
            ("iso8601", "centuries"),
            ("iso9000", "auto"),
            ("iso8601-julian", "auto"),
        ] {
            assert_eq!(
                format_datetime_line(syntax, 0, 0, 0, precision),
                Err(Refusal::Unknown),
                "{syntax} {precision}"
            );
        }
        assert_eq!(
            format_datetime_line("iso8601", 0, 1_000_000_000_000_000_000, 0, "auto"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            format_datetime_line("iso8601", 0, 0, 100_000, "auto"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            format_datetime_line("rfc3339", i64::MAX / 4, 0, 0, "auto"),
            Err(Refusal::OutOfRange)
        );
    }

    /// An offset with seconds, 05:30:15 here, has no digits in ISO 8601's, RFC
    /// 3339's or RFC 5322's offsets, so the instant cannot be written at it
    /// without saying another moment: `20:00:20+05:30` is 15 s later than
    /// 14:30:05Z. Only `datetime.isoformat` writes the seconds. An offset of
    /// whole minutes is written as before.
    #[test]
    fn an_offset_with_seconds_is_refused_where_the_syntax_has_no_digits_for_it() {
        let unix = 1_790_001_005;
        for syntax in ["iso8601", "iso8601-basic", "rfc3339", "rfc2822"] {
            for offset in [19_815, -17_762] {
                assert_eq!(
                    format_datetime_line(syntax, unix, 0, offset, "auto"),
                    Err(Refusal::OutOfRange),
                    "{syntax} {offset}"
                );
            }
        }
        assert_eq!(
            owned(&format_datetime_line("python", unix, 0, 19_815, "auto").expect("a text"))[0],
            "2026-09-21T20:00:20+05:30:15"
        );
        assert_eq!(
            owned(&format_datetime_line("rfc3339", unix, 0, 19_800, "auto").expect("a text"))[0],
            "2026-09-21T20:00:05+05:30"
        );
        // The python text reads back to the instant it was written from.
        assert_eq!(
            reading("python", "2026-09-21T20:00:20+05:30:15")[5],
            unix.to_string()
        );
    }

    /// A second 60 is a leap second only at the end of a UTC day that was
    /// given one: 2016-12-31 and 2015-06-30 (`hc_core::leap`, from the IANA
    /// `leap-seconds.list`), not 2026-09-21, 2016-06-30 or 2015-12-31, and not
    /// at an offset, where 23:59:60 is not the UTC second 60. A day past the
    /// table is not known to have one.
    #[test]
    fn a_second_60_is_read_only_on_a_day_that_ended_in_a_leap_second() {
        assert_eq!(
            reading("rfc3339", "2016-12-31T23:59:60Z"),
            ["736329", "86400", "0", "utc", "0", "1483228800", "1", "0"]
        );
        assert_eq!(reading("rfc2822", "30 Jun 2015 23:59:60 +0000")[6], "1");
        assert_eq!(reading("iso8601", "20150630T235960Z")[6], "1");
        for (syntax, text) in [
            ("rfc3339", "2026-09-21T23:59:60Z"),
            ("rfc3339", "2016-06-30T23:59:60Z"),
            ("iso8601", "2016-12-31T23:59:60+01:00"),
            ("iso8601", "2015-12-31T23:59:60Z"),
            ("rfc2822", "31 Dec 2015 23:59:60 +0000"),
            ("rfc3339", "1972-06-30T23:59:60+09:00"),
            ("python", "2016-06-30T23:59:60+00:00"),
        ] {
            assert_eq!(
                parse_datetime_line(syntax, text),
                Err(Refusal::InvalidDate),
                "{syntax} {text}"
            );
        }
        assert_eq!(
            parse_datetime_line("rfc3339", "2030-12-31T23:59:60Z"),
            Err(Refusal::OutOfRange)
        );
        // A reading with no zone names no instant, so no day is refused.
        assert_eq!(reading("iso8601", "2026-09-21T23:59:60")[6], "1");
    }

    /// RFC 3339's grammar has a calendar date only, offset hours of 00 through
    /// 23, and no seconds in an offset; ISO 8601 has the others.
    #[test]
    fn the_rfc_3339_syntax_refuses_what_its_grammar_does_not_have() {
        for text in [
            "2026-W39-1T14:30:05Z",
            "2026-264T14:30:05Z",
            "2026-09-21T14:30:05+24:00",
            "2026-09-21T14:30:05+05:30:15",
        ] {
            assert!(parse_datetime_line("rfc3339", text).is_err(), "{text}");
        }
        assert_eq!(reading("iso8601", "2026-W39-1T14:30:05+24:00")[4], "86400");
    }

    /// Python's `isocalendar`: 2021-01-03 (RD 737 793) is 2020-W53-7, and
    /// 2026-09-21 is 2026-W39-1 and day 264.
    #[test]
    fn a_date_is_written_as_a_week_or_an_ordinal_date() {
        let date = |fixed: i64, form: &str, style: &str| {
            owned(&format_iso_date_line(fixed, form, style).expect("a date"))[0].clone()
        };
        assert_eq!(date(739_880, "week", "extended"), "2026-W39-1");
        assert_eq!(date(739_880, "week", "basic"), "2026W391");
        assert_eq!(date(739_880, "ordinal", "extended"), "2026-264");
        assert_eq!(date(739_880, "ordinal", "basic"), "2026264");
        assert_eq!(date(739_880, "calendar", "basic"), "20260921");
        assert_eq!(date(737_793, "week", "extended"), "2020-W53-7");
        assert_eq!(
            format_iso_date_line(0, "julian", "basic"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            format_iso_date_line(0, "week", "compact"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            format_iso_date_line(i64::MAX, "week", "basic"),
            Err(Refusal::OutOfRange)
        );
    }

    /// A date of reduced accuracy is read into its parts and names no day.
    #[test]
    fn a_reduced_date_names_no_day() {
        let parts = |text: &str| owned(&iso_date_parts_line(text).expect("parts"));
        assert_eq!(
            parts("2026-W39"),
            ["week", "2026", "", "", "", "39", "", "", "extended"]
        );
        assert_eq!(
            parts("2026-W39-1"),
            ["week", "2026", "", "", "", "39", "1", "739880", "extended"]
        );
        assert_eq!(
            parts("2026-09"),
            ["calendar", "2026", "9", "", "", "", "", "", "extended"]
        );
        assert_eq!(
            parts("2026"),
            ["calendar", "2026", "", "", "", "", "", "", "extended"]
        );
        assert_eq!(
            parts("2026-264"),
            [
                "ordinal", "2026", "", "", "264", "", "", "739880", "extended"
            ]
        );
        assert_eq!(
            parts("20260921"),
            ["calendar", "2026", "9", "21", "", "", "", "739880", "basic"]
        );
        assert_eq!(iso_date_parts_line("2026-W54-1"), Err(Refusal::InvalidDate));
        assert_eq!(iso_date_parts_line("Sep 21"), Err(Refusal::Malformed));
    }

    /// ISO 8601's example `P3Y6M4DT12H30M5S`, `PT36H` (129 600 s) and `P1W`
    /// (604 800 s) as Python's `timedelta` counts them.
    #[test]
    fn a_duration_is_its_components() {
        let duration = |text: &str| owned(&iso_duration_line(text).expect("a duration"));
        assert_eq!(
            duration("P3Y6M4DT12H30M5S"),
            [
                "0",
                "3",
                "6",
                "",
                "4",
                "12",
                "30",
                "5",
                "",
                "designators",
                "P3Y6M4DT12H30M5S",
                "1",
                "",
                ""
            ]
        );
        assert_eq!(
            duration("PT36H"),
            [
                "0",
                "",
                "",
                "",
                "",
                "36",
                "",
                "",
                "",
                "designators",
                "PT36H",
                "0",
                "129600",
                "0"
            ]
        );
        assert_eq!(duration("P1W")[12..], ["604800", "0"]);
        assert_eq!(duration("PT0,5S")[8], "5");
        assert_eq!(duration("PT0.5S")[12..], ["0", "500000000000000000"]);
        assert_eq!(duration("-P1D")[0], "1");
        assert_eq!(duration("-P1D")[12], "-86400");
        assert_eq!(iso_duration_line("P"), Err(Refusal::Malformed));
        assert_eq!(iso_duration_line("1 hour"), Err(Refusal::Malformed));
    }

    #[test]
    fn components_are_written_as_a_duration() {
        let written = |args: (bool, i64, i64, i64, i64, i64, i64, i64, &str)| {
            owned(
                &format_iso_duration_line(
                    args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8,
                )
                .expect("a duration"),
            )
        };
        assert_eq!(
            written((false, 3, 6, -1, 4, 12, 30, 5, ""))[0],
            "P3Y6M4DT12H30M5S"
        );
        assert_eq!(
            written((false, -1, -1, -1, -1, -1, -1, 1, "5"))[..4],
            ["PT1.5S", "0", "1", "500000000000000000"]
        );
        assert_eq!(written((true, -1, -1, -1, 1, -1, -1, -1, ""))[0], "-P1D");
        assert_eq!(
            format_iso_duration_line(false, -1, -1, -1, -1, -1, -1, -1, ""),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            format_iso_duration_line(false, -1, -1, -1, -1, -1, -1, 1, "x"),
            Err(Refusal::Malformed)
        );
    }

    /// Wikipedia's ISO 8601 examples (read 2026-10-03): the interval
    /// `2007-03-01T13:00:00Z/2008-05-11T15:30:00Z`, and the repeating one
    /// `R5/2008-03-01T13:00:00Z/P1Y2M10DT2H30M`; the instants are Python's.
    #[test]
    fn an_interval_is_its_ends() {
        let interval = |text: &str| owned(&iso_interval_line(text).expect("an interval"));
        assert_eq!(
            interval("2007-03-01T13:00:00Z/2008-05-11T15:30:00Z"),
            [
                "",
                "start-end",
                "2007-03-01T13:00:00Z",
                "1172754000",
                "2008-05-11T15:30:00Z",
                "1210519800",
                "",
                "",
                "",
                ""
            ]
        );
        let repeating = interval("R5/2008-03-01T13:00:00Z/P1Y2M10DT2H30M");
        assert_eq!(
            repeating[..4],
            ["5", "start-duration", "2008-03-01T13:00:00Z", "1204376400"]
        );
        assert_eq!(repeating[4..6], ["", ""]);
        assert_eq!(repeating[6..8], ["P1Y2M10DT2H30M", "1"]);
        assert_eq!(
            interval("R/2008-03-01T13:00:00Z/PT1H")[..2],
            ["inf", "start-duration"]
        );
        assert_eq!(interval("PT1H")[..2], ["", "duration"]);
        assert_eq!(interval("P1D/2008-03-01")[1], "duration-end");
        assert_eq!(interval("2008-03-01/2008-03-02")[3], "");
        assert_eq!(iso_interval_line("2008-03-01"), Err(Refusal::Malformed));
        assert_eq!(iso_interval_line("R5"), Err(Refusal::Malformed));
    }

    /// `strptime`'s own: `%Y-%m-%d %H:%M:%S`; Python's resolution of a time
    /// with no date, 1900-01-01 (RD 693 596); CLDR's `XXX` zone, and `%s`.
    #[test]
    fn a_text_is_read_against_a_pattern() {
        let fields = |syntax: &str, pattern: &str, text: &str| {
            owned(&parse_pattern_line(syntax, pattern, text).expect("fields"))
        };
        let reading_of = |cells: &[String]| cells[PATTERN_COLUMNS - READING_COLUMNS..].to_vec();
        let c = fields("strftime", "%Y-%m-%d %H:%M:%S", "2026-09-21 14:30:05");
        assert_eq!(c.len(), PATTERN_COLUMNS);
        assert_eq!(c[0], "2026");
        assert_eq!(c[3..5], ["9", "21"]);
        assert_eq!(c[11], "14");
        assert_eq!(reading_of(&c)[..4], ["739880", "52205", "0", "none"]);
        let p = fields("python", "%H:%M", "14:30");
        assert_eq!(p[..3], ["1900", "", ""]);
        assert_eq!(reading_of(&p)[..2], ["693596", "52200"]);
        let bare = fields("strftime", "%H:%M", "14:30");
        assert_eq!(reading_of(&bare), ["", "", "", "", "", "", "", ""]);
        let x = fields(
            "cldr",
            "yyyy-MM-dd'T'HH:mm:ssXXX",
            "2026-09-21T14:30:05+09:00",
        );
        assert_eq!(x[17..19], ["offset", "32400"]);
        assert_eq!(reading_of(&x)[5], "1789968605");
        let s = fields("strftime", "%s", "1789968605");
        assert_eq!(s[19], "1789968605");
        assert_eq!(reading_of(&s)[..4], ["739880", "19805", "0", "utc"]);
        let month = fields("python", "%a %b %d %H:%M:%S %Y", "Mon Sep 21 14:30:05 2026");
        assert_eq!(reading_of(&month)[..2], ["739880", "52205"]);
        assert_eq!(
            parse_pattern_line("strftime", "%Y", "twenty"),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            parse_pattern_line("strftime", "%Y-%m-%d", "2026-02-30"),
            Err(Refusal::InvalidDate)
        );
        assert_eq!(
            parse_pattern_line("regex", "%Y", "2026"),
            Err(Refusal::Unknown)
        );
    }
}
