//! The CLDR / Unicode TR 35 field-pattern vocabulary.
//!
//! A CLDR pattern is a run of letters per field, where the *count* of the
//! letter chooses both the width and, for text fields, which of the four
//! name widths to use: `M` is `9`, `MM` is `09`, `MMM` is `Sep`, `MMMM` is
//! `September`, `MMMMM` is `S`. Anything that is not a letter is a literal,
//! and `'` quotes a run of text that would otherwise be read as fields —
//! with `''` standing for one apostrophe, quoted or not.
//!
//! # Supported fields
//!
//! `G` era, `y` year, `Y` week-numbering year, `u` extended year, `Q`/`q`
//! quarter, `M`/`L` month, `w` week of year, `W` week of month, `d` day,
//! `D` day of year, `F` day of week in month, `E`/`e`/`c` weekday, `a` day
//! period, `h`/`H`/`K`/`k` hour, `m` minute, `s` second, `S` fractional
//! second, `A` milliseconds in day, `z`/`Z`/`O`/`X`/`x` zone.
//!
//! # Deliberate gaps
//!
//! * `b` and `B`, the flexible day periods ("in the morning", "at night"),
//!   need per-locale hour ranges that `hc-i18n` deliberately does not carry.
//! * `v` and `V`, the generic non-location and zone-ID formats, need the
//!   IANA metazone table, which is a different kind of data from anything in
//!   this workspace.
//! * `U` (cyclic year name), `r` (related Gregorian year) and `g` (modified
//!   Julian day) belong to calendars that are not this module's subject.
//! * `z`, `v` and `V` are not resolved when parsing. Resolving them would
//!   mean mapping an abbreviation back to a zone, and abbreviations are not
//!   unique: `CST` is three different zones. Parsing consumes such a field
//!   and leaves the zone unstated unless RFC 5322 assigns the name an
//!   offset.

use core::fmt;

use hc_i18n::Locale;
use hc_i18n::names::{self, NameContext, NameWidth};
use hc_tz::{OffsetStyle, UtcOffset};

use crate::error::{ErrorKind, FormatError, FormatResult, ParseResult};
use crate::patterns::strftime::{
    match_day_period, match_month, match_weekday, number_field, ranged, read_offset,
    starts_with_ignore_case,
};
use crate::patterns::{
    Fields, FormatContext, ParsedFields, day_period_name, era_candidates, era_name, month_name,
    quarter_name, weekday_name,
};
use crate::scan::Scanner;
use crate::value::ZoneInfo;

/// One piece of a pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token<'a> {
    /// Text to reproduce as it stands.
    Literal(&'a str),
    /// One apostrophe, written `''`.
    Quote,
    /// A run of `count` copies of `letter`.
    Field(char, usize),
}

/// Splits a pattern into literals and fields.
///
/// The `Err` carries the byte offset of an apostrophe that is never closed,
/// which is the only way the lexing itself can fail.
struct Lexer<'a> {
    text: &'a str,
    index: usize,
    quote_open: Option<usize>,
}

impl<'a> Lexer<'a> {
    const fn new(text: &'a str) -> Self {
        Self {
            text,
            index: 0,
            quote_open: None,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, usize>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let bytes = self.text.as_bytes();
            let Some(&byte) = bytes.get(self.index) else {
                return self.quote_open.take().map(Err);
            };
            if byte == b'\'' {
                if bytes.get(self.index + 1) == Some(&b'\'') {
                    self.index += 2;
                    return Some(Ok(Token::Quote));
                }
                if self.quote_open.is_some() {
                    self.quote_open = None;
                } else {
                    self.quote_open = Some(self.index);
                }
                self.index += 1;
                continue;
            }
            let start = self.index;
            if self.quote_open.is_some() {
                while bytes.get(self.index).is_some_and(|next| *next != b'\'') {
                    self.index += 1;
                }
            } else if byte.is_ascii_alphabetic() {
                while bytes.get(self.index) == Some(&byte) {
                    self.index += 1;
                }
                return Some(Ok(Token::Field(char::from(byte), self.index - start)));
            } else {
                while bytes
                    .get(self.index)
                    .is_some_and(|next| *next != b'\'' && !next.is_ascii_alphabetic())
                {
                    self.index += 1;
                }
            }
            return Some(Ok(Token::Literal(
                self.text.get(start..self.index).unwrap_or(""),
            )));
        }
    }
}

/// The name width a field of this letter count asks for.
///
/// TR 35 uses the same ladder for every text field: one to three letters is
/// the abbreviation, four is the full name, five is the narrow form, and six
/// — for weekdays only — is the short form.
const fn name_width(count: usize) -> NameWidth {
    match count {
        4 => NameWidth::Wide,
        5 => NameWidth::Narrow,
        6 => NameWidth::Short,
        _ => NameWidth::Abbreviated,
    }
}

/// Format a civil reading against a CLDR pattern.
///
/// # Errors
///
/// [`FormatError::UnterminatedLiteral`] for an unclosed `'`,
/// [`FormatError::UnknownField`] for a field this module does not implement,
/// and [`FormatError::Sink`] when the sink refuses.
pub fn format<W: fmt::Write>(
    out: &mut W,
    pattern: &str,
    context: &FormatContext<'_>,
) -> FormatResult<()> {
    let fields = context
        .fields()
        .map_err(|_| FormatError::Unrepresentable("the date"))?;
    for token in Lexer::new(pattern) {
        match token.map_err(FormatError::UnterminatedLiteral)? {
            Token::Literal(text) => out.write_str(text)?,
            Token::Quote => out.write_char('\'')?,
            Token::Field(letter, count) => {
                write_field(out, letter, count, context, &fields)?;
            }
        }
    }
    Ok(())
}

fn write_field<W: fmt::Write>(
    out: &mut W,
    letter: char,
    count: usize,
    context: &FormatContext<'_>,
    fields: &Fields,
) -> FormatResult<()> {
    let locale = context.locale;
    let width = name_width(count);
    match letter {
        'G' => Ok(out.write_str(era_name(locale, fields.era_index(), width))?),
        'y' => write_year(out, fields.era_year(), count),
        'Y' => write_year(out, fields.iso_year, count),
        'u' => write_signed(out, fields.year, count),
        'Q' | 'q' => {
            let context_kind = if letter == 'q' {
                NameContext::Standalone
            } else {
                NameContext::Format
            };
            if count <= 2 {
                write_padded(out, u64::from(fields.quarter()), count)
            } else {
                Ok(out.write_str(quarter_name(locale, fields.quarter(), width, context_kind))?)
            }
        }
        'M' | 'L' => {
            let context_kind = if letter == 'L' {
                NameContext::Standalone
            } else {
                NameContext::Format
            };
            if count <= 2 {
                write_padded(out, u64::from(fields.month), count)
            } else {
                Ok(out.write_str(month_name(locale, fields.month, width, context_kind))?)
            }
        }
        'w' => write_padded(out, u64::from(fields.iso_week), count),
        'W' => write_padded(out, u64::from(fields.week_of_month()), count),
        'd' => write_padded(out, u64::from(fields.day), count),
        'D' => write_padded(out, u64::from(fields.day_of_year), count),
        'F' => write_padded(out, u64::from((fields.day - 1) / 7 + 1), count),
        'E' => Ok(out.write_str(weekday_name(
            locale,
            fields.weekday,
            width,
            NameContext::Format,
        ))?),
        'e' | 'c' => {
            let context_kind = if letter == 'c' {
                NameContext::Standalone
            } else {
                NameContext::Format
            };
            if count <= 2 {
                write_padded(out, u64::from(local_weekday(locale, fields)), count)
            } else {
                Ok(out.write_str(weekday_name(locale, fields.weekday, width, context_kind))?)
            }
        }
        'a' => Ok(out.write_str(day_period_name(locale, fields.day_period(), width))?),
        'h' => write_padded(out, u64::from(fields.hour12()), count),
        'H' => write_padded(out, u64::from(fields.hour), count),
        'K' => write_padded(out, u64::from(fields.hour % 12), count),
        'k' => write_padded(
            out,
            u64::from(if fields.hour == 0 { 24 } else { fields.hour }),
            count,
        ),
        'm' => write_padded(out, u64::from(fields.minute), count),
        's' => write_padded(out, u64::from(fields.second), count),
        'S' => write_fraction(out, fields.subsec_attos, count),
        'A' => write_padded(out, u64::from(fields.millis_in_day()), count),
        'z' => write_zone_name(out, context, count),
        'Z' => write_zone_offset(out, context.zone, count),
        'O' => write_localised_gmt(out, context.zone, count >= 4),
        'X' => write_iso_offset(out, context.zone, count, true),
        'x' => write_iso_offset(out, context.zone, count, false),
        other => Err(FormatError::UnknownField(other)),
    }
}

/// The weekday numbered from the locale's own first day of the week.
fn local_weekday(locale: Option<&Locale>, fields: &Fields) -> u8 {
    let first = locale.map_or(hc_calendar::Weekday::Monday, names::first_day_of_week);
    (fields.weekday.iso_number() + 7 - first.iso_number()) % 7 + 1
}

fn write_year<W: fmt::Write>(out: &mut W, year: i64, count: usize) -> FormatResult<()> {
    // TR 35: exactly two letters means the last two digits, any other count
    // is a minimum width.
    if count == 2 {
        return write_padded(out, year.unsigned_abs() % 100, 2);
    }
    write_signed(out, year, count)
}

fn write_signed<W: fmt::Write>(out: &mut W, value: i64, count: usize) -> FormatResult<()> {
    if value < 0 {
        out.write_char('-')?;
    }
    write_padded(out, value.unsigned_abs(), count)
}

fn write_padded<W: fmt::Write>(out: &mut W, value: u64, width: usize) -> FormatResult<()> {
    write!(out, "{value:0width$}")?;
    Ok(())
}

fn write_fraction<W: fmt::Write>(out: &mut W, attos: u64, digits: usize) -> FormatResult<()> {
    let digits = digits.clamp(1, 18);
    let scale = 10u64.pow(18 - digits as u32);
    write!(out, "{:0width$}", attos / scale, width = digits)?;
    Ok(())
}

fn offset_of(zone: ZoneInfo) -> Option<UtcOffset> {
    zone.offset()
}

fn write_zone_name<W: fmt::Write>(
    out: &mut W,
    context: &FormatContext<'_>,
    count: usize,
) -> FormatResult<()> {
    let name = if count >= 4 {
        context.zone_name.or(context.zone_abbreviation)
    } else {
        context.zone_abbreviation.or(context.zone_name)
    };
    if let Some(name) = name {
        out.write_str(name)?;
        return Ok(());
    }
    // TR 35's fallback when no name is known is the localised GMT format.
    write_localised_gmt(out, context.zone, count >= 4)
}

fn write_zone_offset<W: fmt::Write>(out: &mut W, zone: ZoneInfo, count: usize) -> FormatResult<()> {
    match count {
        1..=3 => {
            let Some(offset) = offset_of(zone) else {
                return Err(FormatError::Unrepresentable("a zone field with no zone"));
            };
            out.write_str(offset.format(OffsetStyle::Basic).as_str())?;
            Ok(())
        }
        4 => write_localised_gmt(out, zone, true),
        _ => write_iso_offset(out, zone, 3, true),
    }
}

fn write_localised_gmt<W: fmt::Write>(out: &mut W, zone: ZoneInfo, long: bool) -> FormatResult<()> {
    let Some(offset) = offset_of(zone) else {
        return Err(FormatError::Unrepresentable("a zone field with no zone"));
    };
    out.write_str("GMT")?;
    if offset.is_utc() && !long {
        return Ok(());
    }
    if offset.is_utc() {
        out.write_str("+00:00")?;
        return Ok(());
    }
    out.write_char(if offset.is_negative() { '-' } else { '+' })?;
    if long {
        write!(out, "{:02}:{:02}", offset.abs_hours(), offset.abs_minutes())?;
    } else {
        write!(out, "{}", offset.abs_hours())?;
        if offset.abs_minutes() != 0 {
            write!(out, ":{:02}", offset.abs_minutes())?;
        }
    }
    Ok(())
}

fn write_iso_offset<W: fmt::Write>(
    out: &mut W,
    zone: ZoneInfo,
    count: usize,
    zulu_for_zero: bool,
) -> FormatResult<()> {
    let Some(offset) = offset_of(zone) else {
        return Err(FormatError::Unrepresentable("a zone field with no zone"));
    };
    if zulu_for_zero && offset.is_utc() {
        out.write_char('Z')?;
        return Ok(());
    }
    let style = match count {
        1 => OffsetStyle::Hours,
        2 => OffsetStyle::Basic,
        4 => OffsetStyle::BasicSeconds,
        5 => OffsetStyle::ExtendedSeconds,
        _ => OffsetStyle::Extended,
    };
    out.write_str(offset.format(style).as_str())?;
    Ok(())
}

// --- parsing ---------------------------------------------------------------

/// Parse text against a CLDR pattern, with the POSIX `C` vocabulary.
///
/// # Errors
///
/// See [`crate::ParseError`].
pub fn parse(pattern: &str, text: &str) -> ParseResult<ParsedFields> {
    parse_with_locale(pattern, text, None)
}

/// Parse text against a CLDR pattern, taking names from a locale.
///
/// # Errors
///
/// See [`crate::ParseError`].
pub fn parse_with_locale(
    pattern: &str,
    text: &str,
    locale: Option<&Locale>,
) -> ParseResult<ParsedFields> {
    let mut scanner = Scanner::new(text);
    let mut fields = ParsedFields::default();
    for token in Lexer::new(pattern) {
        let token = token.map_err(|_| scanner.error(ErrorKind::PatternMismatch))?;
        match token {
            Token::Literal(literal) => match_literal(&mut scanner, literal)?,
            Token::Quote => {
                if scanner.peek() != Some(b'\'') {
                    return Err(scanner.error(ErrorKind::Literal("'")));
                }
                scanner.advance(1);
            }
            Token::Field(letter, count) => {
                read_field(letter, count, &mut scanner, &mut fields, locale)?;
            }
        }
    }
    scanner.finish()?;
    Ok(fields)
}

fn match_literal(scanner: &mut Scanner<'_>, literal: &str) -> ParseResult<()> {
    for byte in literal.bytes() {
        if byte.is_ascii_whitespace() {
            scanner.skip_ascii_whitespace();
            continue;
        }
        if scanner.peek() != Some(byte) {
            return Err(scanner.error(ErrorKind::PatternMismatch));
        }
        scanner.advance(1);
    }
    Ok(())
}

fn read_field(
    letter: char,
    count: usize,
    scanner: &mut Scanner<'_>,
    fields: &mut ParsedFields,
    locale: Option<&Locale>,
) -> ParseResult<()> {
    match letter {
        'G' => {
            let index = match_era(scanner, locale)
                .ok_or_else(|| scanner.error(ErrorKind::UnknownName("era")))?;
            fields.era = Some(index);
        }
        'y' => {
            if count == 2 {
                fields.year_of_century = Some(number_field(scanner, 2, "year")?);
            } else {
                fields.year = Some(number_field(scanner, 10, "year")?);
            }
        }
        'Y' => fields.iso_year = Some(number_field(scanner, 10, "year")?),
        'u' => {
            let negative = scanner.peek() == Some(b'-');
            if negative {
                scanner.advance(1);
            }
            let value = number_field(scanner, 10, "year")?;
            fields.year = Some(if negative { -value } else { value });
        }
        'M' | 'L' => {
            if count <= 2 {
                fields.month = Some(ranged(scanner, 2, 1, 12, "month")? as u8);
            } else {
                fields.month = Some(
                    match_month(scanner, locale)
                        .ok_or_else(|| scanner.error(ErrorKind::UnknownName("month")))?,
                );
            }
        }
        'd' => fields.day = Some(ranged(scanner, 2, 1, 31, "day")? as u8),
        'D' => fields.day_of_year = Some(ranged(scanner, 3, 1, 366, "day of year")? as u16),
        'w' => fields.iso_week = Some(ranged(scanner, 2, 1, 53, "week")? as u8),
        'E' => {
            fields.iso_weekday = Some(
                match_weekday(scanner, locale)
                    .ok_or_else(|| scanner.error(ErrorKind::UnknownName("weekday")))?,
            );
        }
        'e' | 'c' => {
            if count <= 2 {
                let local = ranged(scanner, 2, 1, 7, "weekday")? as u8;
                let first = locale.map_or(hc_calendar::Weekday::Monday, names::first_day_of_week);
                fields.iso_weekday = Some((first.iso_number() + local - 2) % 7 + 1);
            } else {
                fields.iso_weekday = Some(
                    match_weekday(scanner, locale)
                        .ok_or_else(|| scanner.error(ErrorKind::UnknownName("weekday")))?,
                );
            }
        }
        'a' => {
            fields.day_period = Some(
                match_day_period(scanner, locale)
                    .ok_or_else(|| scanner.error(ErrorKind::UnknownName("day period")))?,
            );
        }
        'h' => fields.hour12 = Some(ranged(scanner, 2, 1, 12, "hour")? as u8),
        'H' => fields.hour = Some(ranged(scanner, 2, 0, 23, "hour")? as u8),
        'K' => {
            let value = ranged(scanner, 2, 0, 11, "hour")? as u8;
            fields.hour12 = Some(if value == 0 { 12 } else { value });
        }
        'k' => {
            let value = ranged(scanner, 2, 1, 24, "hour")? as u8;
            fields.hour = Some(value % 24);
        }
        'm' => fields.minute = Some(ranged(scanner, 2, 0, 59, "minute")? as u8),
        's' => fields.second = Some(ranged(scanner, 2, 0, 60, "second")? as u8),
        'S' => {
            let digits = count.clamp(1, 18);
            let start = scanner.pos();
            let available = scanner.digit_run().min(digits);
            if available == 0 {
                return Err(Scanner::error_at(ErrorKind::Digit, start));
            }
            let raw = scanner.take_digits(available)?;
            fields.subsec_attos = Some(raw * 10u64.pow(18 - available as u32));
        }
        // Consumed and discarded: these narrow a date that the other fields
        // have already fixed, and none of them can fix one on its own.
        'Q' | 'q' => {
            if count <= 2 {
                ranged(scanner, 2, 1, 4, "quarter")?;
            } else {
                match_quarter(scanner, locale)
                    .ok_or_else(|| scanner.error(ErrorKind::UnknownName("quarter")))?;
            }
        }
        'W' => {
            ranged(scanner, 1, 1, 5, "week of month")?;
        }
        'F' => {
            ranged(scanner, 1, 1, 5, "day of week in month")?;
        }
        'A' => {
            number_field(scanner, 9, "milliseconds")?;
        }
        'X' | 'x' | 'Z' | 'O' => fields.zone = Some(read_zone(scanner, letter)?),
        'z' | 'v' | 'V' => fields.zone = read_tolerant_zone(scanner),
        other => return Err(scanner.error(ErrorKind::UnsupportedPatternField(other))),
    }
    Ok(())
}

fn read_zone(scanner: &mut Scanner<'_>, letter: char) -> ParseResult<ZoneInfo> {
    if matches!(letter, 'O' | 'Z') {
        // The localised GMT format writes the prefix before the offset.
        for prefix in ["GMT", "UTC"] {
            if starts_with_ignore_case(scanner.rest(), prefix) {
                scanner.advance(prefix.len());
                if !matches!(scanner.peek(), Some(b'+' | b'-')) {
                    return Ok(ZoneInfo::Zulu);
                }
                break;
            }
        }
    }
    read_offset(scanner)
}

/// Consume a zone name and resolve it only when RFC 5322 assigns it an
/// offset. Anything else leaves the zone unstated rather than guessed at.
fn read_tolerant_zone(scanner: &mut Scanner<'_>) -> Option<ZoneInfo> {
    let run = scanner
        .rest()
        .iter()
        .take_while(|byte| byte.is_ascii_alphabetic() || **byte == b'/' || **byte == b'_')
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

fn match_era(scanner: &mut Scanner<'_>, locale: Option<&Locale>) -> Option<usize> {
    let mut best: Option<(usize, usize)> = None;
    for index in 0..2 {
        for name in era_candidates(locale, index) {
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

fn match_quarter(scanner: &mut Scanner<'_>, locale: Option<&Locale>) -> Option<u8> {
    let mut best: Option<(usize, u8)> = None;
    for quarter in 1..=4u8 {
        for width in [NameWidth::Wide, NameWidth::Abbreviated, NameWidth::Narrow] {
            for context in [NameContext::Format, NameContext::Standalone] {
                let name = quarter_name(locale, quarter, width, context);
                if name.is_empty() || !starts_with_ignore_case(scanner.rest(), name) {
                    continue;
                }
                if best.is_none_or(|(length, _)| name.len() > length) {
                    best = Some((name.len(), quarter));
                }
            }
        }
    }
    let (length, quarter) = best?;
    scanner.advance(length);
    Some(quarter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use hc_calendar::{CivilDateTime, CivilTime, Rd};

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
        assert_eq!(render("yyyy-MM-dd"), "2026-09-21");
        assert_eq!(render("yyyy-MM-dd HH:mm:ss"), "2026-09-21 14:30:05");
    }

    #[test]
    fn the_letter_count_chooses_the_name_width() {
        assert_eq!(render("M MM MMM MMMM MMMMM"), "9 09 Sep September S");
        assert_eq!(render("E EEEE EEEEE"), "Mon Monday M");
        assert_eq!(render("Q QQ QQQ QQQQ"), "3 03 Q3 3rd quarter");
    }

    #[test]
    fn a_two_letter_year_is_the_last_two_digits_and_nothing_else_is() {
        assert_eq!(render("y yy yyy yyyy yyyyy"), "2026 26 2026 2026 02026");
    }

    #[test]
    fn quoted_text_is_reproduced_verbatim() {
        assert_eq!(render("yyyy'年'MM'月'dd'日'"), "2026年09月21日");
        assert_eq!(render("'at' HH:mm"), "at 14:30");
        assert_eq!(render("''"), "'");
        assert_eq!(render("'it''s' yyyy"), "it's 2026");
    }

    #[test]
    fn an_unterminated_literal_points_at_the_apostrophe() {
        let mut out = String::new();
        assert_eq!(
            format(&mut out, "yyyy 'oops", &context()).unwrap_err(),
            FormatError::UnterminatedLiteral(5)
        );
    }

    #[test]
    fn the_hour_fields_differ_at_midnight_and_noon() {
        let at = |hour| {
            let mut out = String::new();
            let context = FormatContext::new(CivilDateTime::new(
                Rd(739_880),
                CivilTime::hms(hour, 0, 0).unwrap(),
            ));
            format(&mut out, "h H K k a", &context).unwrap();
            out
        };
        assert_eq!(at(0), "12 0 0 24 AM");
        assert_eq!(at(12), "12 12 0 12 PM");
        assert_eq!(at(23), "11 23 11 23 PM");
    }

    #[test]
    fn the_fractional_second_field_takes_its_width_from_the_letter_count() {
        let context = FormatContext::new(CivilDateTime::new(
            Rd(739_880),
            CivilTime::new(0, 0, 0, 123_456_789_000_000_000).unwrap(),
        ));
        let mut out = String::new();
        format(&mut out, "S SSS SSSSSS", &context).unwrap();
        assert_eq!(out, "1 123 123456");
    }

    #[test]
    fn the_zone_fields_write_the_shapes_tr_35_defines() {
        let tokyo = context().with_zone(ZoneInfo::Offset(UtcOffset::from_hms(9, 0, 0).unwrap()));
        let mut out = String::new();
        format(&mut out, "Z|ZZZZ|ZZZZZ|O|OOOO|X|XX|XXX", &tokyo).unwrap();
        assert_eq!(
            out,
            "+0900|GMT+09:00|+09:00|GMT+9|GMT+09:00|+09|+0900|+09:00"
        );
    }

    #[test]
    fn the_x_fields_write_zulu_for_utc_and_the_lowercase_ones_do_not() {
        let utc = context().with_zone(ZoneInfo::Zulu);
        let mut out = String::new();
        format(&mut out, "X|XXX|x|xxx", &utc).unwrap();
        assert_eq!(out, "Z|Z|+00|+00:00");
    }

    #[test]
    fn a_zone_name_falls_back_to_the_localised_gmt_format() {
        let tokyo = context().with_zone(ZoneInfo::Offset(UtcOffset::from_hms(9, 0, 0).unwrap()));
        let mut out = String::new();
        format(&mut out, "z|zzzz", &tokyo).unwrap();
        assert_eq!(out, "GMT+9|GMT+09:00");
        let named = tokyo
            .with_zone_abbreviation("JST")
            .with_zone_name("Japan Standard Time");
        let mut out = String::new();
        format(&mut out, "z|zzzz", &named).unwrap();
        assert_eq!(out, "JST|Japan Standard Time");
    }

    #[test]
    fn a_locale_changes_the_names() {
        let japanese = Locale::parse("ja").unwrap();
        let mut out = String::new();
        format(
            &mut out,
            "yyyy年M月d日(EEE)",
            &context().with_locale(&japanese),
        )
        .unwrap();
        assert_eq!(out, "2026年9月21日(月)");
    }

    #[test]
    fn the_era_field_puts_year_zero_before_christ() {
        let year_zero = FormatContext::new(CivilDateTime::midnight(
            hc_calendars_solar::gregorian::to_fixed(0, 1, 1).unwrap(),
        ));
        let mut out = String::new();
        format(&mut out, "y G / u", &year_zero).unwrap();
        assert_eq!(out, "1 BC / 0");
    }

    #[test]
    fn an_unimplemented_field_names_itself() {
        let mut out = String::new();
        assert_eq!(
            format(&mut out, "B", &context()).unwrap_err(),
            FormatError::UnknownField('B')
        );
    }

    #[test]
    fn a_formatted_pattern_parses_back() {
        let parsed = parse("yyyy-MM-dd HH:mm:ss", "2026-09-21 14:30:05").unwrap();
        assert_eq!(
            parsed.to_offset_date_time().unwrap().local,
            context().date_time
        );
    }

    #[test]
    fn names_parse_back_too() {
        let parsed = parse("d MMMM yyyy", "21 September 2026").unwrap();
        assert_eq!(parsed.month, Some(9));
        assert_eq!(parsed.day, Some(21));
    }

    #[test]
    fn a_quoted_literal_must_be_present_in_the_input() {
        assert!(parse("yyyy'年'MM", "2026年09").is_ok());
        assert_eq!(
            parse("yyyy'年'MM", "2026-09").unwrap_err().kind(),
            ErrorKind::PatternMismatch
        );
    }

    #[test]
    fn an_offset_field_parses_into_a_zone() {
        let parsed = parse("yyyy-MM-dd'T'HH:mm:ssXXX", "2026-09-21T14:30:05+09:00").unwrap();
        assert_eq!(parsed.zone.unwrap().offset().unwrap().seconds(), 9 * 3_600);
        let zulu = parse("yyyy-MM-dd'T'HH:mm:ssXXX", "2026-09-21T14:30:05Z").unwrap();
        assert_eq!(zulu.zone, Some(ZoneInfo::Zulu));
    }

    #[test]
    fn the_localised_gmt_format_parses_back() {
        let parsed = parse("OOOO", "GMT+09:00").unwrap();
        assert_eq!(parsed.zone.unwrap().offset().unwrap().seconds(), 9 * 3_600);
    }

    #[test]
    fn an_era_parses_and_flips_the_year() {
        let parsed = parse("y G-MM-dd", "1 BC-01-01").unwrap();
        let value = parsed.to_offset_date_time().unwrap();
        assert_eq!(
            hc_calendars_solar::gregorian::from_fixed(value.local.day).unwrap(),
            (0, 1, 1)
        );
    }

    #[test]
    fn a_format_only_zone_field_leaves_the_zone_unstated() {
        let parsed = parse("yyyy-MM-dd z", "2026-09-21 JST").unwrap();
        assert_eq!(parsed.zone, None);
        let known = parse("yyyy-MM-dd z", "2026-09-21 EST").unwrap();
        assert_eq!(known.zone.unwrap().offset().unwrap().seconds(), -5 * 3_600);
    }

    #[test]
    fn a_field_this_module_cannot_parse_names_itself() {
        assert_eq!(
            parse("B", "morning").unwrap_err().kind(),
            ErrorKind::UnsupportedPatternField('B')
        );
    }
}
