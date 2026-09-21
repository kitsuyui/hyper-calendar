//! The `strftime`/`strptime` vocabulary.
//!
//! The conversions implemented are the POSIX set plus the GNU flags that are
//! universally relied on: `-` to drop padding, `_` to pad with spaces, `0` to
//! pad with zeros, `^` to upper-case a name, an explicit field width, and
//! `%:z` for a colon-separated offset.
//!
//! # Deliberate gaps
//!
//! * `%E…` and `%O…`, the POSIX locale-alternative modifiers, are not
//!   implemented: they name era and numbering-system alternatives that
//!   `hc-i18n` exposes directly and that no caller has ever wanted spelled
//!   this way.
//! * `%U` and `%W` are written but *ignored when parsing*: a Sunday- or
//!   Monday-anchored week number cannot reconstruct a date without a weekday,
//!   and silently guessing one would be worse than dropping the field.
//! * `%^` upper-casing is ASCII-only. Locale-correct casing is
//!   [`hc_i18n::casing`]'s job and needs an allocator.

use core::fmt;

use hc_i18n::Locale;
use hc_i18n::names::{DayPeriod, NameContext, NameWidth};
use hc_tz::OffsetStyle;

use crate::error::{ErrorKind, FormatError, FormatResult, ParseResult};
use crate::iso8601::Strictness;
use crate::patterns::{
    Fields, FormatContext, ParsedFields, day_period_candidates, day_period_name, month_candidates,
    month_name, weekday_candidates, weekday_name,
};
use crate::scan::Scanner;
use crate::value::{OffsetDateTime, ZoneInfo};

/// How deep a composite conversion such as `%c` may expand.
///
/// The expansions are compile-time constants and none of them nests more
/// than once, so this only has to stop a future typo from looping.
const MAX_EXPANSION_DEPTH: u8 = 4;

/// How a field is padded to its width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pad {
    None,
    Space,
    Zero,
}

#[derive(Debug, Clone, Copy, Default)]
struct Spec {
    pad: Option<Pad>,
    width: Option<usize>,
    upper: bool,
    colon: bool,
}

impl Spec {
    fn pad_or(self, default: Pad) -> Pad {
        self.pad.unwrap_or(default)
    }

    fn width_or(self, default: usize) -> usize {
        self.width.unwrap_or(default)
    }
}

/// The strictness a `%z` field is read under: every ISO spelling, plus
/// `-0000`, which `strftime` implementations emit for "offset unknown".
const OFFSET_STRICTNESS: Strictness = Strictness {
    allow_negative_zero_offset: true,
    allow_lowercase_designators: true,
    ..Strictness::ISO
};

/// Format a civil reading against a `strftime` pattern.
///
/// # Errors
///
/// [`FormatError::UnknownField`] for a conversion this crate does not
/// implement, [`FormatError::Unrepresentable`] for `%s` without a zone or a
/// date outside the Gregorian range, and [`FormatError::Sink`] when the sink
/// refuses.
pub fn format<W: fmt::Write>(
    out: &mut W,
    pattern: &str,
    context: &FormatContext<'_>,
) -> FormatResult<()> {
    let fields = context
        .fields()
        .map_err(|_| FormatError::Unrepresentable("the date"))?;
    expand(out, pattern, context, &fields, 0)
}

fn expand<W: fmt::Write>(
    out: &mut W,
    pattern: &str,
    context: &FormatContext<'_>,
    fields: &Fields,
    depth: u8,
) -> FormatResult<()> {
    if depth > MAX_EXPANSION_DEPTH {
        return Err(FormatError::Unrepresentable("a pattern nested too deeply"));
    }
    let bytes = pattern.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes.get(index) != Some(&b'%') {
            let tail = pattern.get(index..).unwrap_or("");
            let run = tail.find('%').unwrap_or(tail.len());
            out.write_str(tail.get(..run).unwrap_or(""))?;
            index += run;
            continue;
        }
        index += 1;
        let mut spec = Spec::default();
        while let Some(flag) = bytes.get(index) {
            match flag {
                b'-' => spec.pad = Some(Pad::None),
                b'_' => spec.pad = Some(Pad::Space),
                b'0' => spec.pad = Some(Pad::Zero),
                b'^' | b'#' => spec.upper = true,
                _ => break,
            }
            index += 1;
        }
        let width_start = index;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
        if index > width_start {
            spec.width = pattern
                .get(width_start..index)
                .and_then(|text| text.parse::<usize>().ok());
        }
        if bytes.get(index) == Some(&b':') {
            spec.colon = true;
            index += 1;
        }
        let Some(&conversion) = bytes.get(index) else {
            return Err(FormatError::UnknownField('%'));
        };
        index += 1;
        write_conversion(out, char::from(conversion), spec, context, fields, depth)?;
    }
    Ok(())
}

fn write_conversion<W: fmt::Write>(
    out: &mut W,
    conversion: char,
    spec: Spec,
    context: &FormatContext<'_>,
    fields: &Fields,
    depth: u8,
) -> FormatResult<()> {
    let locale = context.locale;
    match conversion {
        'Y' => number(out, fields.year, spec, 4, Pad::Zero),
        'C' => number(out, fields.year.div_euclid(100), spec, 2, Pad::Zero),
        'y' => number(out, fields.year.rem_euclid(100), spec, 2, Pad::Zero),
        'G' => number(out, fields.iso_year, spec, 4, Pad::Zero),
        'g' => number(out, fields.iso_year.rem_euclid(100), spec, 2, Pad::Zero),
        'm' => number(out, i64::from(fields.month), spec, 2, Pad::Zero),
        'd' => number(out, i64::from(fields.day), spec, 2, Pad::Zero),
        'e' => number(out, i64::from(fields.day), spec, 2, Pad::Space),
        'j' => number(out, i64::from(fields.day_of_year), spec, 3, Pad::Zero),
        'H' => number(out, i64::from(fields.hour), spec, 2, Pad::Zero),
        'k' => number(out, i64::from(fields.hour), spec, 2, Pad::Space),
        'I' => number(out, i64::from(fields.hour12()), spec, 2, Pad::Zero),
        'l' => number(out, i64::from(fields.hour12()), spec, 2, Pad::Space),
        'M' => number(out, i64::from(fields.minute), spec, 2, Pad::Zero),
        'S' => number(out, i64::from(fields.second), spec, 2, Pad::Zero),
        'u' => number(
            out,
            i64::from(fields.weekday.iso_number()),
            spec,
            1,
            Pad::None,
        ),
        'w' => number(
            out,
            i64::from(fields.weekday.sunday_first_number()),
            spec,
            1,
            Pad::None,
        ),
        'U' => number(
            out,
            i64::from(fields.week_of_year_sunday()),
            spec,
            2,
            Pad::Zero,
        ),
        'W' => number(
            out,
            i64::from(fields.week_of_year_monday()),
            spec,
            2,
            Pad::Zero,
        ),
        'V' => number(out, i64::from(fields.iso_week), spec, 2, Pad::Zero),
        'a' => text(
            out,
            weekday_name(
                locale,
                fields.weekday,
                NameWidth::Abbreviated,
                NameContext::Format,
            ),
            spec,
        ),
        'A' => text(
            out,
            weekday_name(locale, fields.weekday, NameWidth::Wide, NameContext::Format),
            spec,
        ),
        'b' | 'h' => text(
            out,
            month_name(
                locale,
                fields.month,
                NameWidth::Abbreviated,
                NameContext::Format,
            ),
            spec,
        ),
        'B' => text(
            out,
            month_name(locale, fields.month, NameWidth::Wide, NameContext::Format),
            spec,
        ),
        'p' => text(
            out,
            day_period_name(locale, fields.day_period(), NameWidth::Abbreviated),
            spec,
        ),
        'P' => {
            let name = day_period_name(locale, fields.day_period(), NameWidth::Abbreviated);
            for character in name.chars() {
                out.write_char(character.to_ascii_lowercase())?;
            }
            Ok(())
        }
        'z' => write_offset(out, context.zone, spec.colon),
        'Z' => write_zone_name(out, context, spec),
        's' => {
            let value = OffsetDateTime {
                local: context.date_time,
                zone: context.zone,
                written_as_end_of_day: false,
            };
            let unix = value
                .to_unix()
                .map_err(|_| FormatError::Unrepresentable("%s without a zone"))?;
            number(out, unix.seconds(), spec, 1, Pad::None)
        }
        'n' => Ok(out.write_char('\n')?),
        't' => Ok(out.write_char('\t')?),
        '%' => Ok(out.write_char('%')?),
        'F' => expand(out, "%Y-%m-%d", context, fields, depth + 1),
        'T' => expand(out, "%H:%M:%S", context, fields, depth + 1),
        'R' => expand(out, "%H:%M", context, fields, depth + 1),
        'D' | 'x' => expand(out, "%m/%d/%y", context, fields, depth + 1),
        'X' => expand(out, "%H:%M:%S", context, fields, depth + 1),
        'r' => expand(out, "%I:%M:%S %p", context, fields, depth + 1),
        'c' => expand(out, "%a %b %e %H:%M:%S %Y", context, fields, depth + 1),
        other => Err(FormatError::UnknownField(other)),
    }
}

fn write_offset<W: fmt::Write>(out: &mut W, zone: ZoneInfo, colon: bool) -> FormatResult<()> {
    let style = if colon {
        OffsetStyle::Extended
    } else {
        OffsetStyle::Basic
    };
    match zone {
        // `strftime` has no designator for "no zone", and glibc writes
        // nothing; writing `+0000` instead would be an assertion the value
        // does not make.
        ZoneInfo::Unspecified => Ok(()),
        ZoneInfo::Zulu => {
            out.write_str(if colon { "+00:00" } else { "+0000" })?;
            Ok(())
        }
        other => other.write(out, style),
    }
}

fn write_zone_name<W: fmt::Write>(
    out: &mut W,
    context: &FormatContext<'_>,
    spec: Spec,
) -> FormatResult<()> {
    if let Some(abbreviation) = context.zone_abbreviation {
        return text(out, abbreviation, spec);
    }
    match context.zone {
        ZoneInfo::Zulu | ZoneInfo::UnknownLocalOffset => text(out, "UTC", spec),
        ZoneInfo::Unspecified => Ok(()),
        ZoneInfo::Offset(offset) => Ok(out.write_str(offset.format(OffsetStyle::Basic).as_str())?),
    }
}

fn text<W: fmt::Write>(out: &mut W, value: &str, spec: Spec) -> FormatResult<()> {
    if spec.upper {
        for character in value.chars() {
            out.write_char(character.to_ascii_uppercase())?;
        }
    } else {
        out.write_str(value)?;
    }
    Ok(())
}

fn number<W: fmt::Write>(
    out: &mut W,
    value: i64,
    spec: Spec,
    default_width: usize,
    default_pad: Pad,
) -> FormatResult<()> {
    let width = spec.width_or(default_width);
    let pad = spec.pad_or(default_pad);
    let mut digits = [0u8; 20];
    let mut length = 0usize;
    let mut magnitude = value.unsigned_abs();
    loop {
        if let Some(slot) = digits.get_mut(length) {
            *slot = b'0' + (magnitude % 10) as u8;
        }
        length += 1;
        magnitude /= 10;
        if magnitude == 0 || length == digits.len() {
            break;
        }
    }
    let negative = value < 0;
    let printed = length + usize::from(negative);
    if matches!(pad, Pad::Space) {
        for _ in printed..width {
            out.write_char(' ')?;
        }
    }
    if negative {
        out.write_char('-')?;
    }
    if matches!(pad, Pad::Zero) {
        for _ in printed..width {
            out.write_char('0')?;
        }
    }
    for position in (0..length).rev() {
        let digit = digits.get(position).copied().unwrap_or(b'0');
        out.write_char(char::from(digit))?;
    }
    Ok(())
}

// --- parsing ---------------------------------------------------------------

/// Parse text against a `strptime` pattern, with the POSIX `C` vocabulary.
///
/// # Errors
///
/// See [`ParseError`].
pub fn parse(pattern: &str, text: &str) -> ParseResult<ParsedFields> {
    parse_with_locale(pattern, text, None)
}

/// Parse text against a `strptime` pattern, taking names from a locale.
///
/// The `C` names are accepted as well as the locale's: a file written by one
/// program and read by another rarely agrees about which locale was in
/// force, and refusing `Sep` because the locale says `sept.` helps nobody.
///
/// # Errors
///
/// See [`ParseError`].
pub fn parse_with_locale(
    pattern: &str,
    text: &str,
    locale: Option<&Locale>,
) -> ParseResult<ParsedFields> {
    let mut scanner = Scanner::new(text);
    let mut fields = ParsedFields::default();
    consume(pattern, &mut scanner, &mut fields, locale, 0)?;
    scanner.finish()?;
    Ok(fields)
}

fn consume(
    pattern: &str,
    scanner: &mut Scanner<'_>,
    fields: &mut ParsedFields,
    locale: Option<&Locale>,
    depth: u8,
) -> ParseResult<()> {
    if depth > MAX_EXPANSION_DEPTH {
        return Err(scanner.error(ErrorKind::PatternMismatch));
    }
    let bytes = pattern.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        let Some(&byte) = bytes.get(index) else { break };
        if byte != b'%' {
            index += 1;
            if byte.is_ascii_whitespace() {
                scanner.skip_ascii_whitespace();
                continue;
            }
            if scanner.peek() != Some(byte) {
                return Err(scanner.error(ErrorKind::PatternMismatch));
            }
            scanner.advance(1);
            continue;
        }
        index += 1;
        // Flags and widths steer output only; a parser reads what is there.
        while bytes
            .get(index)
            .is_some_and(|flag| matches!(flag, b'-' | b'_' | b'0' | b'^' | b'#'))
        {
            index += 1;
        }
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
        if bytes.get(index) == Some(&b':') {
            index += 1;
        }
        let Some(&conversion) = bytes.get(index) else {
            return Err(scanner.error(ErrorKind::PatternMismatch));
        };
        index += 1;
        read_conversion(char::from(conversion), scanner, fields, locale, depth)?;
    }
    Ok(())
}

fn read_conversion(
    conversion: char,
    scanner: &mut Scanner<'_>,
    fields: &mut ParsedFields,
    locale: Option<&Locale>,
    depth: u8,
) -> ParseResult<()> {
    match conversion {
        'Y' => fields.year = Some(signed_number(scanner, 10)?),
        'C' => fields.century = Some(number_field(scanner, 2, "century")?),
        'y' => fields.year_of_century = Some(number_field(scanner, 2, "year")?),
        'G' => fields.iso_year = Some(signed_number(scanner, 10)?),
        'g' => {
            let within = number_field(scanner, 2, "year")?;
            fields.iso_year = Some(if within >= 69 {
                1_900 + within
            } else {
                2_000 + within
            });
        }
        'm' => fields.month = Some(ranged(scanner, 2, 1, 12, "month")? as u8),
        'd' | 'e' => fields.day = Some(ranged(scanner, 2, 1, 31, "day")? as u8),
        'j' => fields.day_of_year = Some(ranged(scanner, 3, 1, 366, "day of year")? as u16),
        'H' | 'k' => fields.hour = Some(ranged(scanner, 2, 0, 23, "hour")? as u8),
        'I' | 'l' => fields.hour12 = Some(ranged(scanner, 2, 1, 12, "hour")? as u8),
        'M' => fields.minute = Some(ranged(scanner, 2, 0, 59, "minute")? as u8),
        'S' => fields.second = Some(ranged(scanner, 2, 0, 60, "second")? as u8),
        'V' => fields.iso_week = Some(ranged(scanner, 2, 1, 53, "week")? as u8),
        'u' => fields.iso_weekday = Some(ranged(scanner, 1, 1, 7, "weekday")? as u8),
        'w' => {
            let sunday_first = ranged(scanner, 1, 0, 6, "weekday")? as u8;
            fields.iso_weekday = Some(if sunday_first == 0 { 7 } else { sunday_first });
        }
        // Read and discarded: see the module documentation.
        'U' | 'W' => {
            ranged(scanner, 2, 0, 53, "week")?;
        }
        'a' | 'A' => {
            let weekday = match_weekday(scanner, locale)
                .ok_or_else(|| scanner.error(ErrorKind::UnknownName("weekday")))?;
            fields.iso_weekday = Some(weekday);
        }
        'b' | 'B' | 'h' => {
            let month = match_month(scanner, locale)
                .ok_or_else(|| scanner.error(ErrorKind::UnknownName("month")))?;
            fields.month = Some(month);
        }
        'p' | 'P' => {
            let period = match_day_period(scanner, locale)
                .ok_or_else(|| scanner.error(ErrorKind::UnknownName("day period")))?;
            fields.day_period = Some(period);
        }
        'z' => fields.zone = Some(read_offset(scanner)?),
        'Z' => fields.zone = read_zone_name(scanner),
        's' => fields.unix_seconds = Some(signed_number(scanner, 19)?),
        'n' | 't' => scanner.skip_ascii_whitespace(),
        '%' => {
            if scanner.peek() != Some(b'%') {
                return Err(scanner.error(ErrorKind::Literal("%")));
            }
            scanner.advance(1);
        }
        'F' => consume("%Y-%m-%d", scanner, fields, locale, depth + 1)?,
        'T' | 'X' => consume("%H:%M:%S", scanner, fields, locale, depth + 1)?,
        'R' => consume("%H:%M", scanner, fields, locale, depth + 1)?,
        'D' | 'x' => consume("%m/%d/%y", scanner, fields, locale, depth + 1)?,
        'r' => consume("%I:%M:%S %p", scanner, fields, locale, depth + 1)?,
        'c' => consume("%a %b %e %H:%M:%S %Y", scanner, fields, locale, depth + 1)?,
        other => {
            return Err(scanner.error(ErrorKind::UnsupportedPatternField(other)));
        }
    }
    Ok(())
}

pub(crate) fn number_field(
    scanner: &mut Scanner<'_>,
    max_digits: usize,
    field: &'static str,
) -> ParseResult<i64> {
    scanner.skip_ascii_whitespace();
    let start = scanner.pos();
    let run = scanner.digit_run().min(max_digits);
    if run == 0 {
        return Err(Scanner::error_at(ErrorKind::Digit, start));
    }
    let value = scanner.take_digits(run)?;
    i64::try_from(value).map_err(|_| Scanner::error_at(ErrorKind::OutOfRange(field), start))
}

fn signed_number(scanner: &mut Scanner<'_>, max_digits: usize) -> ParseResult<i64> {
    scanner.skip_ascii_whitespace();
    let negative = match scanner.peek() {
        Some(b'-') => {
            scanner.advance(1);
            true
        }
        Some(b'+') => {
            scanner.advance(1);
            false
        }
        _ => false,
    };
    let value = number_field(scanner, max_digits, "number")?;
    Ok(if negative { -value } else { value })
}

pub(crate) fn ranged(
    scanner: &mut Scanner<'_>,
    max_digits: usize,
    low: i64,
    high: i64,
    field: &'static str,
) -> ParseResult<i64> {
    let start = scanner.pos();
    let value = number_field(scanner, max_digits, field)?;
    if value < low || value > high {
        return Err(Scanner::error_at(ErrorKind::OutOfRange(field), start));
    }
    Ok(value)
}

pub(crate) fn starts_with_ignore_case(haystack: &[u8], needle: &str) -> bool {
    let needle = needle.as_bytes();
    haystack.len() >= needle.len()
        && haystack
            .iter()
            .zip(needle.iter())
            .all(|(found, wanted)| found.eq_ignore_ascii_case(wanted))
}

/// Match the longest candidate name, so that `September` wins over `Sep`.
fn match_longest<const N: usize>(
    scanner: &mut Scanner<'_>,
    count: usize,
    candidates: impl Fn(usize) -> [&'static str; N],
) -> Option<usize> {
    let mut best: Option<(usize, usize)> = None;
    for index in 0..count {
        for name in candidates(index) {
            if name.is_empty() || !starts_with_ignore_case(scanner.rest(), name) {
                continue;
            }
            if best.is_none_or(|(length, _)| name.len() > length) {
                best = Some((name.len(), index));
            }
        }
    }
    let (length, index) = best?;
    scanner.advance(length);
    Some(index)
}

pub(crate) fn match_month(scanner: &mut Scanner<'_>, locale: Option<&Locale>) -> Option<u8> {
    match_longest(scanner, 12, |index| {
        month_candidates(locale, index as u8 + 1)
    })
    .map(|index| index as u8 + 1)
}

pub(crate) fn match_weekday(scanner: &mut Scanner<'_>, locale: Option<&Locale>) -> Option<u8> {
    match_longest(scanner, 7, |index| {
        weekday_candidates(
            locale,
            hc_calendar::Weekday::from_iso_number(index as u8 + 1)
                .unwrap_or(hc_calendar::Weekday::Monday),
        )
    })
    .map(|index| index as u8 + 1)
}

pub(crate) fn match_day_period(
    scanner: &mut Scanner<'_>,
    locale: Option<&Locale>,
) -> Option<DayPeriod> {
    match_longest(scanner, 2, |index| {
        day_period_candidates(
            locale,
            if index == 0 {
                DayPeriod::Am
            } else {
                DayPeriod::Pm
            },
        )
    })
    .map(|index| {
        if index == 0 {
            DayPeriod::Am
        } else {
            DayPeriod::Pm
        }
    })
}

pub(crate) fn read_offset(scanner: &mut Scanner<'_>) -> ParseResult<ZoneInfo> {
    let (zone, _) = crate::iso8601::scan_offset(scanner, &OFFSET_STRICTNESS)?;
    if zone.is_qualified() {
        Ok(zone)
    } else {
        Err(scanner.error(ErrorKind::OneOf("Z+-")))
    }
}

/// Read a `%Z` zone name.
///
/// A zone abbreviation does not determine an offset — `CST` is three
/// different zones — so only the handful RFC 5322 assigns offsets to are
/// resolved; anything else is consumed and the zone left unstated, which is
/// the truth about what was learned.
fn read_zone_name(scanner: &mut Scanner<'_>) -> Option<ZoneInfo> {
    let run = scanner
        .rest()
        .iter()
        .take_while(|byte| byte.is_ascii_alphabetic())
        .count();
    if run == 0 {
        return None;
    }
    let name = core::str::from_utf8(scanner.rest().get(..run)?).ok()?;
    let resolved = crate::rfc2822::named_zone_offset(name).map(|offset| {
        if offset.is_utc() {
            ZoneInfo::Zulu
        } else {
            ZoneInfo::Offset(offset)
        }
    });
    scanner.advance(run);
    resolved
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use hc_calendar::{CivilDateTime, CivilTime, Rd};
    use hc_tz::UtcOffset;

    fn context() -> FormatContext<'static> {
        FormatContext::new(CivilDateTime::new(
            Rd(739_880),
            CivilTime::hms(14, 30, 5).unwrap(),
        ))
    }

    fn render(pattern: &str) -> String {
        let mut out = String::new();
        format(&mut out, pattern, &context()).unwrap();
        out
    }

    #[test]
    fn the_everyday_pattern_formats_as_expected() {
        assert_eq!(render("%Y-%m-%d %H:%M:%S"), "2026-09-21 14:30:05");
    }

    #[test]
    fn the_composite_conversions_expand_to_their_definitions() {
        assert_eq!(render("%F %T"), "2026-09-21 14:30:05");
        assert_eq!(render("%D"), "09/21/26");
        assert_eq!(render("%R"), "14:30");
        assert_eq!(render("%r"), "02:30:05 PM");
        assert_eq!(render("%c"), "Mon Sep 21 14:30:05 2026");
    }

    #[test]
    fn the_week_and_day_of_year_fields_agree_with_the_calendar() {
        assert_eq!(render("%j"), "264");
        assert_eq!(render("%V"), "39");
        assert_eq!(render("%G"), "2026");
        assert_eq!(render("%U %W"), "38 38");
        assert_eq!(render("%u %w"), "1 1");
    }

    #[test]
    fn the_padding_flags_do_what_they_say() {
        assert_eq!(render("%d"), "21");
        assert_eq!(render("%e"), "21");
        let first = FormatContext::new(CivilDateTime::new(
            Rd(739_622),
            CivilTime::hms(1, 2, 3).unwrap(),
        ));
        let mut out = String::new();
        format(&mut out, "[%e][%-d][%_d][%0d][%H][%-H][%k]", &first).unwrap();
        assert_eq!(out, "[ 6][6][ 6][06][01][1][ 1]");
    }

    #[test]
    fn an_explicit_width_overrides_the_default() {
        assert_eq!(render("%6Y"), "002026");
    }

    #[test]
    fn a_percent_sign_escapes_itself() {
        assert_eq!(render("100%% sure on %Y"), "100% sure on 2026");
    }

    #[test]
    fn upper_casing_is_ascii_only_and_opt_in() {
        assert_eq!(render("%b"), "Sep");
        assert_eq!(render("%^b"), "SEP");
        assert_eq!(render("%^A"), "MONDAY");
    }

    #[test]
    fn a_locale_changes_the_names_and_nothing_else() {
        let japanese = Locale::parse("ja").unwrap();
        let mut out = String::new();
        let localised = context().with_locale(&japanese);
        format(&mut out, "%Y-%m-%d (%a) %B", &localised).unwrap();
        assert_eq!(out, "2026-09-21 (月) 9月");
    }

    #[test]
    fn the_offset_is_written_in_the_shape_the_conversion_asks_for() {
        let tokyo = context().with_zone(ZoneInfo::Offset(UtcOffset::from_hms(9, 0, 0).unwrap()));
        let mut out = String::new();
        format(&mut out, "%z %:z", &tokyo).unwrap();
        assert_eq!(out, "+0900 +09:00");
    }

    #[test]
    fn an_unqualified_reading_writes_no_offset_at_all() {
        assert_eq!(render("[%z]"), "[]");
    }

    #[test]
    fn a_posix_timestamp_needs_a_zone_to_exist() {
        let mut out = String::new();
        assert_eq!(
            format(&mut out, "%s", &context()).unwrap_err(),
            FormatError::Unrepresentable("%s without a zone")
        );
        let mut out = String::new();
        format(&mut out, "%s", &context().with_zone(ZoneInfo::Zulu)).unwrap();
        assert_eq!(out, "1790001005");
    }

    #[test]
    fn the_zone_name_comes_from_the_zone_not_from_the_offset() {
        let tokyo = context()
            .with_zone(ZoneInfo::Offset(UtcOffset::from_hms(9, 0, 0).unwrap()))
            .with_zone_abbreviation("JST");
        let mut out = String::new();
        format(&mut out, "%Z", &tokyo).unwrap();
        assert_eq!(out, "JST");
    }

    #[test]
    fn an_unimplemented_conversion_names_itself() {
        let mut out = String::new();
        assert_eq!(
            format(&mut out, "%Q", &context()).unwrap_err(),
            FormatError::UnknownField('Q')
        );
    }

    #[test]
    fn a_formatted_value_parses_back_to_the_same_reading() {
        let parsed = parse("%Y-%m-%d %H:%M:%S", "2026-09-21 14:30:05").unwrap();
        let value = parsed.to_offset_date_time().unwrap();
        assert_eq!(value.local, context().date_time);
    }

    #[test]
    fn names_are_matched_longest_first() {
        let parsed = parse("%d %B %Y", "21 September 2026").unwrap();
        assert_eq!(parsed.month, Some(9));
        let short = parse("%d %b %Y", "21 Sep 2026").unwrap();
        assert_eq!(short.month, Some(9));
    }

    #[test]
    fn the_locale_names_and_the_c_names_are_both_accepted() {
        let japanese = Locale::parse("ja").unwrap();
        assert_eq!(
            parse_with_locale("%B %d %Y", "9月 21 2026", Some(&japanese))
                .unwrap()
                .month,
            Some(9)
        );
        assert_eq!(
            parse_with_locale("%B %d %Y", "September 21 2026", Some(&japanese))
                .unwrap()
                .month,
            Some(9)
        );
    }

    #[test]
    fn a_twelve_hour_time_needs_the_day_period_to_round_trip() {
        let parsed = parse("%I:%M %p on %Y-%m-%d", "02:30 PM on 2026-09-21").unwrap();
        assert_eq!(parsed.to_offset_date_time().unwrap().local.time.hour(), 14);
    }

    #[test]
    fn an_offset_field_parses_into_a_zone() {
        let parsed = parse("%Y-%m-%dT%H:%M:%S%z", "2026-09-21T14:30:05+0900").unwrap();
        assert_eq!(parsed.zone.unwrap().offset().unwrap().seconds(), 9 * 3_600);
    }

    #[test]
    fn a_posix_timestamp_parses_on_its_own() {
        let parsed = parse("%s", "1790001005").unwrap();
        let value = parsed.to_offset_date_time().unwrap();
        assert_eq!(value.zone, ZoneInfo::Zulu);
        assert_eq!(value.to_unix().unwrap().seconds(), 1_790_001_005);
    }

    #[test]
    fn a_literal_that_does_not_match_reports_where_it_stopped() {
        let error = parse("%Y-%m-%d", "2026/09/21").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::PatternMismatch);
        assert_eq!(error.offset(), 4);
    }

    #[test]
    fn an_out_of_range_field_reports_its_own_offset() {
        let error = parse("%Y-%m-%d", "2026-13-01").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::OutOfRange("month"));
        assert_eq!(error.offset(), 5);
    }

    #[test]
    fn trailing_input_after_the_pattern_is_an_error() {
        let error = parse("%Y", "2026 and more").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::TrailingText);
        assert_eq!(error.offset(), 4);
    }

    #[test]
    fn an_unknown_month_name_says_what_it_was_looking_for() {
        let error = parse("%d %B %Y", "21 Smarch 2026").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnknownName("month"));
        assert_eq!(error.offset(), 3);
    }

    #[test]
    fn whitespace_in_a_pattern_matches_any_run_of_whitespace() {
        assert!(parse("%Y %m %d", "2026   09\t21").is_ok());
    }

    #[test]
    fn an_ordinal_pattern_resolves_through_the_day_of_the_year() {
        let parsed = parse("%Y-%j", "2026-264").unwrap();
        assert_eq!(parsed.to_offset_date_time().unwrap().local.day, Rd(739_880));
    }

    #[test]
    fn a_week_date_pattern_resolves_through_the_iso_week() {
        let parsed = parse("%G-W%V-%u", "2026-W38-1").unwrap();
        assert_eq!(parsed.to_offset_date_time().unwrap().local.day, Rd(739_873));
    }

    #[test]
    fn an_unparsable_conversion_names_the_field() {
        let error = parse("%Q", "x").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnsupportedPatternField('Q'));
    }
}
