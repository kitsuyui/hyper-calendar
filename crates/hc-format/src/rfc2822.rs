//! The date format of email and HTTP: `Tue, 21 Sep 2026 14:30:05 +0900`.
//!
//! The format is RFC 5322 §3.3 (which obsoletes RFC 2822, which obsoleted
//! RFC 822; the name has stuck). RFC 7231's `IMF-fixdate`, the HTTP `Date`
//! header, is a fixed-width subset of it, and [`write_imf_fixdate`] produces
//! exactly that.
//!
//! # A parser has to accept more than a writer produces
//!
//! RFC 5322 §4.3 is explicit that a *parser* must accept the obsolete syntax
//! even though a generator must never emit it: two- and three-digit years,
//! the named zones `UT`, `GMT`, `EST` and friends, single-letter military
//! zones, and comments and folding whitespace anywhere between tokens. Mail
//! archives are full of all of it. This module accepts all of it and writes
//! none of it.
//!
//! # `-0000`
//!
//! RFC 5322 §3.3 gives `-0000` the same meaning RFC 3339 does: the time is
//! UTC but the offset of the generating zone is unknown. So do the
//! unrecognised and military zone names, which §4.3 says to treat as
//! `-0000`. All of them parse to [`ZoneInfo::UnknownLocalOffset`].

use core::fmt;

use hc_calendar::{CivilDateTime, CivilTime, Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_tz::UtcOffset;

use crate::error::{ErrorKind, FormatError, FormatResult, ParseResult};
use crate::scan::Scanner;
use crate::value::{OffsetDateTime, ZoneInfo, write_fixed};

/// The month abbreviations RFC 5322 defines. They are English and fixed, and
/// no locale may change them: a `Date:` header is protocol, not prose.
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// The day-of-week abbreviations, in ISO order so that
/// [`Weekday::iso_number`] indexes them.
const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

/// The obsolete named zones of RFC 5322 §4.3, with the offsets it assigns.
///
/// The list is exactly the one in the RFC. Every other alphabetic zone name,
/// including the single-letter military zones, is to be treated as `-0000`,
/// because the military ones were defined with the wrong sign in RFC 822 and
/// cannot be trusted.
const NAMED_ZONES: [(&str, i32); 11] = [
    ("UT", 0),
    ("GMT", 0),
    ("UTC", 0),
    ("EST", -5 * 3_600),
    ("EDT", -4 * 3_600),
    ("CST", -6 * 3_600),
    ("CDT", -5 * 3_600),
    ("MST", -7 * 3_600),
    ("MDT", -6 * 3_600),
    ("PST", -8 * 3_600),
    ("PDT", -7 * 3_600),
];

/// Parse an RFC 5322 (formerly RFC 2822) date-time.
///
/// # Errors
///
/// See [`crate::ParseError`].
pub fn parse(text: &str) -> ParseResult<OffsetDateTime> {
    let mut scanner = Scanner::new(text);
    if scanner.is_empty() {
        return Err(scanner.error(ErrorKind::Empty));
    }
    skip_folding(&mut scanner)?;

    // The day of the week is optional, and when present it is decorative:
    // the date itself decides which day it is, so a disagreement is not an
    // error the RFC asks a parser to raise.
    if scanner.peek().is_some_and(|b| b.is_ascii_alphabetic()) {
        take_alphabetic(&mut scanner, "weekday")?;
        skip_folding(&mut scanner)?;
        scanner.expect(b',', ",")?;
        skip_folding(&mut scanner)?;
    }

    let day_start = scanner.pos();
    let day = match scanner.digit_run() {
        1 => scanner.take_digits(1)? as u8,
        2 => scanner.take_digits(2)? as u8,
        _ => return Err(Scanner::error_at(ErrorKind::DigitCount(2), day_start)),
    };
    skip_folding(&mut scanner)?;

    let month_start = scanner.pos();
    let month_name = take_alphabetic(&mut scanner, "month")?;
    let month = month_index(month_name)
        .ok_or_else(|| Scanner::error_at(ErrorKind::UnknownName("month"), month_start))?;
    skip_folding(&mut scanner)?;

    let year_start = scanner.pos();
    let year = match scanner.digit_run() {
        // RFC 5322 §4.3: a two-digit year 00-49 means 2000-2049, 50-99 means
        // 1950-1999, and any three-digit year means 1900 plus its value.
        2 => {
            let value = scanner.take_digits(2)? as i64;
            if value < 50 {
                2_000 + value
            } else {
                1_900 + value
            }
        }
        3 => 1_900 + scanner.take_digits(3)? as i64,
        run @ 4..=9 => scanner.take_digits(run)? as i64,
        _ => return Err(Scanner::error_at(ErrorKind::DigitCount(4), year_start)),
    };
    if gregorian::days_in_month(year, month).is_none_or(|limit| day < 1 || day > limit) {
        return Err(Scanner::error_at(ErrorKind::OutOfRange("day"), day_start));
    }
    skip_folding(&mut scanner)?;

    let hour_start = scanner.pos();
    let hour = scanner.take_digits(2)? as u8;
    scanner.expect(b':', ":")?;
    let minute = scanner.take_digits(2)? as u8;
    let second = if scanner.eat(b':') {
        scanner.take_digits(2)? as u8
    } else {
        0
    };
    skip_folding(&mut scanner)?;

    let zone = scan_zone(&mut scanner)?;
    skip_folding(&mut scanner)?;
    scanner.finish()?;

    let day_number = gregorian::to_fixed(year, month, day)
        .map_err(|_| Scanner::error_at(ErrorKind::OutOfRange("date"), day_start))?;
    let time = civil_time(hour, minute, second)
        .ok_or_else(|| Scanner::error_at(ErrorKind::OutOfRange("time"), hour_start))?;
    Ok(OffsetDateTime {
        local: CivilDateTime::new(day_number, time),
        zone,
        written_as_end_of_day: false,
    })
}

/// Build the civil time, keeping the leap second where it is legal.
fn civil_time(hour: u8, minute: u8, second: u8) -> Option<CivilTime> {
    CivilTime::hms(hour, minute, second).ok()
}

/// Write a value in the form RFC 5322 §3.3 tells a generator to produce.
///
/// That means the four-digit year, the numeric offset, and the seconds
/// always present: none of the obsolete forms, which §4.3 permits a parser
/// to read and forbids a generator to write.
///
/// # Errors
///
/// [`FormatError::Unrepresentable`] when the value carries no zone or its
/// year is outside `0000..=9999`, and [`FormatError::Sink`] when the sink
/// refuses.
pub fn write<W: fmt::Write>(out: &mut W, value: OffsetDateTime) -> FormatResult<()> {
    let (year, month, day) = gregorian::from_fixed(value.local.day)
        .map_err(|_| FormatError::Unrepresentable("the date"))?;
    if !(0..=9_999).contains(&year) {
        return Err(FormatError::Unrepresentable(
            "a year outside 0000..=9999 in RFC 5322",
        ));
    }
    write_date_and_time(out, value.local.day, year, month, day, value.local.time)?;
    out.write_char(' ')?;
    write_zone(out, value.zone)
}

/// Write a UTC reading as an HTTP `IMF-fixdate`: `Tue, 21 Sep 2026 14:30:05
/// GMT`.
///
/// RFC 7231 §7.1.1.1 requires exactly this shape, with the literal `GMT`
/// rather than a numeric offset, and requires the reading to be in UTC. The
/// caller is responsible for having converted; this writes what it is given.
///
/// # Errors
///
/// See [`write`](fn@write).
pub fn write_imf_fixdate<W: fmt::Write>(out: &mut W, utc: CivilDateTime) -> FormatResult<()> {
    let (year, month, day) =
        gregorian::from_fixed(utc.day).map_err(|_| FormatError::Unrepresentable("the date"))?;
    if !(0..=9_999).contains(&year) {
        return Err(FormatError::Unrepresentable(
            "a year outside 0000..=9999 in an HTTP date",
        ));
    }
    write_date_and_time(out, utc.day, year, month, day, utc.time)?;
    out.write_str(" GMT")?;
    Ok(())
}

fn write_date_and_time<W: fmt::Write>(
    out: &mut W,
    rd: Rd,
    year: i64,
    month: u8,
    day: u8,
    time: CivilTime,
) -> FormatResult<()> {
    let weekday = Weekday::from_rd(rd);
    let weekday_name = WEEKDAYS
        .get(usize::from(weekday.iso_number()) - 1)
        .copied()
        .unwrap_or("Mon");
    let month_name = MONTHS
        .get(usize::from(month) - 1)
        .copied()
        .ok_or(FormatError::Unrepresentable("the month"))?;
    out.write_str(weekday_name)?;
    out.write_str(", ")?;
    write_fixed(out, u64::from(day), 2)?;
    out.write_char(' ')?;
    out.write_str(month_name)?;
    out.write_char(' ')?;
    write_fixed(out, year as u64, 4)?;
    out.write_char(' ')?;
    write_fixed(out, u64::from(time.hour()), 2)?;
    out.write_char(':')?;
    write_fixed(out, u64::from(time.minute()), 2)?;
    out.write_char(':')?;
    write_fixed(out, u64::from(time.second()), 2)?;
    Ok(())
}

fn write_zone<W: fmt::Write>(out: &mut W, zone: ZoneInfo) -> FormatResult<()> {
    let offset = match zone {
        ZoneInfo::Unspecified => {
            return Err(FormatError::Unrepresentable(
                "an RFC 5322 date with no offset",
            ));
        }
        ZoneInfo::UnknownLocalOffset => {
            out.write_str("-0000")?;
            return Ok(());
        }
        ZoneInfo::Zulu => UtcOffset::UTC,
        ZoneInfo::Offset(offset) => offset,
    };
    out.write_char(if offset.is_negative() { '-' } else { '+' })?;
    write_fixed(out, u64::from(offset.abs_hours()), 2)?;
    write_fixed(out, u64::from(offset.abs_minutes()), 2)?;
    Ok(())
}

/// The offset an RFC 5322 zone name stands for, or `None` when the name is
/// not one the RFC lists.
#[must_use]
pub fn named_zone_offset(name: &str) -> Option<UtcOffset> {
    NAMED_ZONES
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
        .and_then(|(_, seconds)| UtcOffset::from_seconds(*seconds).ok())
}

fn month_index(name: &str) -> Option<u8> {
    MONTHS
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .map(|index| index as u8 + 1)
}

fn scan_zone(scanner: &mut Scanner<'_>) -> ParseResult<ZoneInfo> {
    let start = scanner.pos();
    match scanner.peek() {
        Some(b'+' | b'-') => {
            let negative = scanner.peek() == Some(b'-');
            scanner.advance(1);
            let hours = scanner.take_digits(2)? as i32;
            let minutes = scanner.take_digits(2)? as i32;
            if minutes > 59 {
                return Err(Scanner::error_at(
                    ErrorKind::OutOfRange("offset minutes"),
                    start + 3,
                ));
            }
            let magnitude = hours * 3_600 + minutes * 60;
            if negative && magnitude == 0 {
                return Ok(ZoneInfo::UnknownLocalOffset);
            }
            let signed = if negative { -magnitude } else { magnitude };
            UtcOffset::from_seconds(signed)
                .map(ZoneInfo::Offset)
                .map_err(|_| Scanner::error_at(ErrorKind::OutOfRange("UTC offset"), start))
        }
        Some(byte) if byte.is_ascii_alphabetic() => {
            let name = take_alphabetic(scanner, "zone")?;
            match named_zone_offset(name) {
                Some(offset) if offset.is_utc() => Ok(ZoneInfo::Zulu),
                Some(offset) => Ok(ZoneInfo::Offset(offset)),
                // RFC 5322 §4.3: every other alphabetic zone, including the
                // single-letter military ones, is to be read as `-0000`.
                None => Ok(ZoneInfo::UnknownLocalOffset),
            }
        }
        Some(_) => Err(scanner.error(ErrorKind::OneOf("+-"))),
        None => Err(scanner.error(ErrorKind::UnexpectedEnd)),
    }
}

fn take_alphabetic<'a>(scanner: &mut Scanner<'a>, what: &'static str) -> ParseResult<&'a str> {
    let bytes = scanner.rest();
    let run = bytes
        .iter()
        .take_while(|byte| byte.is_ascii_alphabetic())
        .count();
    if run == 0 {
        return Err(scanner.error(ErrorKind::UnknownName(what)));
    }
    let text = core::str::from_utf8(bytes.get(..run).unwrap_or(&[]))
        .map_err(|_| scanner.error(ErrorKind::UnknownName(what)))?;
    scanner.advance(run);
    Ok(text)
}

/// Skip RFC 5322 comment-and-folding-whitespace: spaces, tabs, line breaks
/// and nested `(…)` comments.
///
/// # Errors
///
/// [`ErrorKind::UnexpectedEnd`] when a comment is never closed.
fn skip_folding(scanner: &mut Scanner<'_>) -> ParseResult<()> {
    loop {
        scanner.skip_ascii_whitespace();
        if scanner.peek() != Some(b'(') {
            return Ok(());
        }
        let open = scanner.pos();
        scanner.advance(1);
        let mut depth = 1usize;
        while depth > 0 {
            match scanner.peek() {
                Some(b'\\') => scanner.advance(2),
                Some(b'(') => {
                    depth += 1;
                    scanner.advance(1);
                }
                Some(b')') => {
                    depth -= 1;
                    scanner.advance(1);
                }
                Some(_) => scanner.advance(1),
                None => return Err(Scanner::error_at(ErrorKind::UnexpectedEnd, open)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;

    fn render(text: &str) -> String {
        let value = parse(text).unwrap();
        let mut out = String::new();
        write(&mut out, value).unwrap();
        out
    }

    #[test]
    fn the_canonical_form_round_trips() {
        let text = "Mon, 21 Sep 2026 14:30:05 +0900";
        assert_eq!(render(text), text);
    }

    #[test]
    fn the_weekday_is_derived_from_the_date_not_copied_from_the_text() {
        // 21 September 2026 really is a Monday; the header says Friday.
        assert_eq!(
            render("Fri, 21 Sep 2026 14:30:05 +0900"),
            "Mon, 21 Sep 2026 14:30:05 +0900"
        );
    }

    #[test]
    fn the_weekday_is_optional() {
        let value = parse("21 Sep 2026 14:30:05 +0900").unwrap();
        assert_eq!(
            gregorian::from_fixed(value.local.day).unwrap(),
            (2026, 9, 21)
        );
    }

    #[test]
    fn a_one_digit_day_is_accepted() {
        let value = parse("Tue, 1 Sep 2026 14:30:05 +0000").unwrap();
        assert_eq!(
            gregorian::from_fixed(value.local.day).unwrap(),
            (2026, 9, 1)
        );
    }

    #[test]
    fn the_seconds_are_optional() {
        assert_eq!(
            render("Mon, 21 Sep 2026 14:30 +0900"),
            "Mon, 21 Sep 2026 14:30:00 +0900"
        );
    }

    #[test]
    fn obsolete_two_digit_years_follow_the_rule_in_section_four_point_three() {
        for (text, year) in [
            ("21 Sep 26 00:00:00 +0000", 2026),
            ("21 Sep 49 00:00:00 +0000", 2049),
            ("21 Sep 50 00:00:00 +0000", 1950),
            ("21 Sep 99 00:00:00 +0000", 1999),
            ("21 Sep 102 00:00:00 +0000", 2002),
        ] {
            let value = parse(text).unwrap();
            assert_eq!(
                gregorian::from_fixed(value.local.day).unwrap().0,
                year,
                "{text}"
            );
        }
    }

    #[test]
    fn the_named_zones_the_rfc_lists_carry_their_offsets() {
        for (name, seconds) in [
            ("GMT", 0),
            ("UT", 0),
            ("EST", -5 * 3_600),
            ("PDT", -7 * 3_600),
        ] {
            let text = alloc::format!("21 Sep 2026 00:00:00 {name}");
            let value = parse(&text).unwrap();
            assert_eq!(value.zone.offset().unwrap().seconds(), seconds, "{name}");
        }
    }

    #[test]
    fn an_unrecognised_or_military_zone_means_the_offset_is_unknown() {
        for name in ["A", "Z", "J", "CEST"] {
            let text = alloc::format!("21 Sep 2026 00:00:00 {name}");
            assert_eq!(
                parse(&text).unwrap().zone,
                ZoneInfo::UnknownLocalOffset,
                "{name}"
            );
        }
    }

    #[test]
    fn minus_zero_means_the_offset_is_unknown() {
        let value = parse("21 Sep 2026 00:00:00 -0000").unwrap();
        assert_eq!(value.zone, ZoneInfo::UnknownLocalOffset);
        assert_eq!(
            render("21 Sep 2026 00:00:00 -0000"),
            "Mon, 21 Sep 2026 00:00:00 -0000"
        );
    }

    #[test]
    fn comments_and_folding_whitespace_are_skipped_wherever_they_appear() {
        let text = "Mon, (the twenty-first) 21 Sep 2026\r\n 14:30:05 +0900 (JST)";
        assert_eq!(render(text), "Mon, 21 Sep 2026 14:30:05 +0900");
    }

    #[test]
    fn nested_comments_are_skipped_too() {
        let text = "21 Sep 2026 00:00:00 +0000 (outer (inner) still outer)";
        assert!(parse(text).is_ok());
    }

    #[test]
    fn an_unclosed_comment_points_at_where_it_opened() {
        let error = parse("21 Sep 2026 00:00:00 +0000 (never closed").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnexpectedEnd);
        assert_eq!(error.offset(), 27);
    }

    #[test]
    fn an_unknown_month_name_says_which_field_it_was() {
        let error = parse("21 Xxx 2026 00:00:00 +0000").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnknownName("month"));
        assert_eq!(error.offset(), 3);
    }

    #[test]
    fn a_day_that_does_not_exist_is_refused_at_the_day_field() {
        let error = parse("31 Sep 2026 00:00:00 +0000").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::OutOfRange("day"));
        assert_eq!(error.offset(), 0);
    }

    #[test]
    fn an_offset_parses_into_an_instant() {
        let value = parse("Mon, 21 Sep 2026 14:30:05 +0900").unwrap();
        assert_eq!(value.to_unix().unwrap().seconds(), 1_789_968_605);
    }

    #[test]
    fn the_http_date_uses_the_literal_gmt() {
        let mut out = String::new();
        write_imf_fixdate(
            &mut out,
            CivilDateTime::new(Rd(739_880), CivilTime::hms(5, 30, 5).unwrap()),
        )
        .unwrap();
        assert_eq!(out, "Mon, 21 Sep 2026 05:30:05 GMT");
    }

    #[test]
    fn a_reading_with_no_zone_has_no_rfc_5322_spelling() {
        let value = OffsetDateTime::local(CivilDateTime::midnight(Rd(739_880)));
        let mut out = String::new();
        assert_eq!(
            write(&mut out, value).unwrap_err(),
            FormatError::Unrepresentable("an RFC 5322 date with no offset")
        );
    }

    #[test]
    fn the_named_zone_table_is_reachable_on_its_own() {
        assert_eq!(named_zone_offset("est").unwrap().seconds(), -5 * 3_600);
        assert!(named_zone_offset("JST").is_none());
    }
}
