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
//! `G` era, `y` year, `Y` week-numbering year, `u` extended year, `U`
//! cyclic year, `r` related Gregorian year, `Q`/`q` quarter, `M`/`L`
//! month, `w` week of year, `W` week of month, `d` day, `D` day of year,
//! `F` day of week in month, `g` Julian day, `E`/`e`/`c` weekday, `a`
//! day period, `b` day period with noon and midnight, `B` flexible day
//! period, `h`/`H`/`K`/`k` hour, `m` minute, `s` second, `S` fractional
//! second, `A` milliseconds in day, `z`/`Z`/`O`/`v`/`V`/`X`/`x` zone.
//!
//! The numeric fields are written in the locale's default numbering system
//! (`hc_i18n::NumberingSystem::for_locale`, `-u-nu-` included) where that
//! one is positional, as `O` always has been: `d MMMM y` is *٢١ سبتمبر ٢٠٢٦*
//! in `ar-EG` and *21 septembre 2026* in `fr`. A system
//! that spells numbers out, Han or Hebrew numerals, is written in Latin
//! digits. Parsing reads the locale's digits and Latin ones.
//!
//! `w`, `Y` and `W` count weeks by the locale's week rule, UTS #35 Part 4's
//! "Week of Year": the locale's first day of the week (`-u-fw-` first) and
//! CLDR 48's `minDays` for its region, `hc_i18n::week::WeekRule`; with no
//! locale, ISO 8601's, Monday and four. `W` is 0 for a day before its
//! month's first week. `e` and `c` already followed the first day.
//!
//! The fields UTS #35 Part 4 (version 48.2, "Date Field Symbol Table")
//! defines for other calendars take its Gregorian meaning here, since
//! these patterns are Gregorian: `U`, where "the calendar does not provide
//! cyclic year name data", "behaves like `y`", and `r`, "for the Gregorian
//! calendar", "is the same as the `u` year". `g` is the Julian day number
//! of the local date, which "demarcates days at local zone midnight": the
//! day whose noon falls in it, 2 451 545 for 2000-01-01.
//!
//! `b` writes *noon* at 12:00:00 and *midnight* at 00:00:00 where the
//! locale's language has the word (`hc_i18n::day_periods`, CLDR 48's
//! `dayPeriods.xml`), and am or pm otherwise; `B` writes the flexible
//! period of the language's rules, *in the afternoon*, *at night*, and am or
//! pm where it has none. With no locale both are the `C` am and pm.
//!
//! The zone fields follow UTS #35's "Using Time Zone Names": `z` the
//! specific name (*Pacific Daylight Time*), `v` the generic one (*Pacific
//! Time*), `V` the zone's identifier (`uslax`), its identifier as given,
//! its exemplar city and its location name (*Los Angeles Time*), and `O`
//! the localized GMT format in the locale's own words and digits (`fr`'s
//! *UTC+2*), each with the fallbacks UTS #35 gives; `z` takes the name
//! the context was given first. The names need the `zone-names` feature,
//! and a locale's other than English's `localized-zone-names`: without
//! them the fields take their fallbacks. `docs/systems/zone-names.md` in
//! the repository works the rules through.
//!
//! # Gaps
//!
//! * `z`, `v` and `V` are not resolved when parsing. Resolving them would
//!   mean mapping an abbreviation back to a zone, and abbreviations are not
//!   unique: `CST` is three different zones. Parsing consumes such a field
//!   and leaves the zone unstated unless RFC 5322 assigns the name an
//!   offset.
//! * Not carried: `O` and `Z` read an offset in Latin digits only, though `O`
//!   writes the locale's. Not yet done.
//! * `B` fixes no hour when parsing: a period that lies wholly before or
//!   after noon gives am or pm, and one that spans midnight gives neither.

use core::fmt;

use hc_i18n::Locale;
use hc_i18n::day_periods::{self, FlexibleDayPeriod};
use hc_i18n::names::{self, DayPeriod, NameContext, NameWidth};
use hc_i18n::week::WeekRule;
use hc_tz::{OffsetStyle, UtcOffset};

use crate::error::{ErrorKind, FormatError, FormatResult, ParseResult};
use crate::patterns::strftime::{
    match_day_period, match_longest, match_month, match_weekday, read_offset,
    starts_with_ignore_case,
};
use crate::patterns::{
    Fields, FormatContext, ParsedFields, day_period_name, era_candidates, era_name, month_name,
    quarter_name, weekday_name, zone,
};

/// The Julian day number of the day before the first of January of year 1,
/// `Rd(0)`: a day's `g` is its `Rd` plus this. The Julian date of
/// 1970-01-01 00:00 is 2 440 587.5, so its Julian day number, the day
/// whose noon it begins, is 2 440 588, and its `Rd` 719 163.
const JULIAN_DAY_OF_RD_ZERO: i64 = 1_721_425;
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
    let digits = zone::digits(context.locale);
    check_calendar(context)?;
    let alternative = match crate::patterns::era_calendar(context) {
        Some(calendar) => Some(
            calendar
                .fixed_to_fields(context.date_time.day)
                .map_err(|_| FormatError::Unrepresentable("the date in the locale's calendar"))?,
        ),
        None => None,
    };
    for token in Lexer::new(pattern) {
        match token.map_err(FormatError::UnterminatedLiteral)? {
            Token::Literal(text) => out.write_str(text)?,
            Token::Quote => out.write_char('\'')?,
            Token::Field(letter, count) => {
                write_field(
                    out,
                    letter,
                    count,
                    context,
                    &fields,
                    &digits,
                    alternative.as_ref(),
                )?;
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
    digits: &[char; 10],
    alternative: Option<&hc_calendar::DateFields>,
) -> FormatResult<()> {
    let locale = context.locale;
    let width = name_width(count);
    match letter {
        'G' => match alternative.as_ref() {
            Some(date) => Ok(write_alternative_era(out, context, date, width)?),
            None => Ok(out.write_str(era_name(locale, fields.era_index(), width))?),
        },
        'y' => match alternative.as_ref() {
            Some(date) => write_year(out, year_of_era(date.year), count, digits),
            None => write_year(out, fields.era_year(), count, digits),
        },
        'Y' => {
            let (week_year, _) = week_rule(locale).week_of_year(context.date_time.day);
            write_year(out, week_year, count, digits)
        }
        'u' => write_signed(
            out,
            alternative.as_ref().map_or(fields.year, |date| date.year),
            count,
            digits,
        ),
        'Q' | 'q' => {
            let context_kind = if letter == 'q' {
                NameContext::Standalone
            } else {
                NameContext::Format
            };
            if count <= 2 {
                write_padded(out, u64::from(fields.quarter()), count, digits)
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
                write_padded(out, u64::from(fields.month), count, digits)
            } else {
                Ok(out.write_str(month_name(locale, fields.month, width, context_kind))?)
            }
        }
        'w' => {
            let (_, week) = week_rule(locale).week_of_year(context.date_time.day);
            write_padded(out, u64::from(week), count, digits)
        }
        'W' => {
            let week = week_rule(locale).week_of_month(context.date_time.day);
            write_padded(out, u64::from(week), count, digits)
        }
        'd' => write_padded(out, u64::from(fields.day), count, digits),
        'D' => write_padded(out, u64::from(fields.day_of_year), count, digits),
        'F' => write_padded(out, u64::from((fields.day - 1) / 7 + 1), count, digits),
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
                write_padded(out, u64::from(local_weekday(locale, fields)), count, digits)
            } else {
                Ok(out.write_str(weekday_name(locale, fields.weekday, width, context_kind))?)
            }
        }
        'a' => Ok(out.write_str(day_period_name(locale, fields.day_period(), width))?),
        'b' | 'B' => {
            Ok(out.write_str(extended_day_period(locale, fields, width, letter == 'B'))?)
        }
        'h' => write_padded(out, u64::from(fields.hour12()), count, digits),
        'H' => write_padded(out, u64::from(fields.hour), count, digits),
        'K' => write_padded(out, u64::from(fields.hour % 12), count, digits),
        'k' => write_padded(
            out,
            u64::from(if fields.hour == 0 { 24 } else { fields.hour }),
            count,
            digits,
        ),
        'm' => write_padded(out, u64::from(fields.minute), count, digits),
        's' => write_padded(out, u64::from(fields.second), count, digits),
        'S' => write_fraction(out, fields.subsec_attos, count, digits),
        'A' => write_padded(out, u64::from(fields.millis_in_day()), count, digits),
        'z' => zone::write_specific(out, context, count >= 4),
        'Z' => write_zone_offset(out, context, count),
        'O' => zone::write_localized_gmt(out, locale, context.zone, count >= 4),
        'v' => zone::write_generic(out, context, count >= 4),
        'V' => zone::write_zone_id(out, context, count),
        // UTS #35: a calendar with no cyclic year names writes `U` as `y`,
        // and the Gregorian calendar's related Gregorian year `r` is `u`.
        'U' => match alternative.as_ref() {
            Some(date) => write_year(out, year_of_era(date.year), count, digits),
            None => write_year(out, fields.era_year(), count, digits),
        },
        'r' => write_signed(out, fields.year, count, digits),
        'g' => write_signed(
            out,
            context.date_time.day.0 + JULIAN_DAY_OF_RD_ZERO,
            count,
            digits,
        ),
        'X' => write_iso_offset(out, context.zone, count, true),
        'x' => write_iso_offset(out, context.zone, count, false),
        other => Err(FormatError::UnknownField(other)),
    }
}

/// The calendars a `-u-ca-` key may name, with nothing said by the context:
/// the Gregorian calendar and ISO 8601's, which the fields are, and the
/// Buddhist and the Minguo, which share its months and days and which
/// `G`, `y`, `u` and `U` are written in ([`crate::patterns::era_calendar`]).
/// Any other would need its own months and days, which this engine does not
/// write, and is refused rather than written as the Gregorian calendar.
fn check_calendar(context: &FormatContext<'_>) -> FormatResult<()> {
    let key = context.locale.and_then(|locale| locale.calendar());
    match key {
        Some(key)
            if context.era_calendar.is_none()
                && !matches!(key, "gregory" | "iso8601" | "buddhist" | "roc") =>
        {
            Err(FormatError::Unrepresentable(
                "a calendar other than the Gregorian, Buddhist and Minguo",
            ))
        }
        _ => Ok(()),
    }
}

/// The year within its era: a year before the era's first, 0 and below, counts
/// back from 1 as the Gregorian calendar's does.
const fn year_of_era(year: i64) -> i64 {
    if year < 1 { 1 - year } else { year }
}

/// `G` in an era calendar: the era's name at the field's width in the locale,
/// else the calendar's own, else English's ([`crate::label::era_label`]).
fn write_alternative_era<W: fmt::Write>(
    out: &mut W,
    context: &FormatContext<'_>,
    date: &hc_calendar::DateFields,
    width: NameWidth,
) -> fmt::Result {
    let (Some(calendar), Some(code)) = (crate::patterns::era_calendar(context), date.era) else {
        return Ok(());
    };
    let locale = context
        .locale
        .copied()
        .unwrap_or_else(hc_i18n::names::english);
    match crate::label::era_label(calendar, &locale, code, width) {
        Some(name) => out.write_str(name),
        None => Ok(()),
    }
}

/// `b` and `B`: midnight or noon at exactly 00:00 or 12:00 where the
/// locale's language has the word, and otherwise am or pm for `b` and the
/// flexible period of the language's rules for `B`, am or pm where it has
/// none; with no locale, the `C` am and pm.
fn extended_day_period(
    locale: Option<&Locale>,
    fields: &Fields,
    width: NameWidth,
    flexible: bool,
) -> &'static str {
    let am_pm = day_period_name(locale, fields.day_period(), width);
    let Some(locale) = locale else {
        return am_pm;
    };
    let exact = fields.minute == 0 && fields.second == 0 && fields.subsec_attos == 0;
    let fixed = match fields.hour {
        0 if exact => Some(FlexibleDayPeriod::Midnight),
        12 if exact => Some(FlexibleDayPeriod::Noon),
        _ => None,
    };
    let named = |period| day_periods::period_name(locale, period, width);
    if let Some(period) = fixed
        && day_periods::has_fixed_period(locale, period)
        && let Some(name) = named(period)
    {
        return name;
    }
    if flexible
        && let Some(period) = day_periods::flexible_period(
            locale,
            u16::from(fields.hour) * 60 + u16::from(fields.minute),
        )
        && let Some(name) = named(period)
    {
        return name;
    }
    am_pm
}

/// A `b` or `B` name read back: am or pm where the name says one.
fn match_extended_day_period(
    scanner: &mut Scanner<'_>,
    locale: Option<&Locale>,
) -> Option<Option<DayPeriod>> {
    if let Some(locale) = locale {
        let found = match_longest(scanner, FlexibleDayPeriod::ALL.len(), |index| {
            let period = FlexibleDayPeriod::ALL[index];
            [NameWidth::Wide, NameWidth::Abbreviated, NameWidth::Narrow]
                .map(|width| day_periods::period_name(locale, period, width).unwrap_or(""))
        });
        if let Some(index) = found {
            return Some(day_periods::period_half(
                locale,
                FlexibleDayPeriod::ALL[index],
            ));
        }
    }
    match_day_period(scanner, locale).map(Some)
}

/// The rule `w`, `Y` and `W` count weeks by: the locale's first day of the
/// week and `minDays` ([`WeekRule::for_locale`], CLDR 48's `weekData`), and
/// ISO 8601's, Monday and four days, with no locale: the `C` locale is
/// POSIX's, and CLDR's root rule, Monday and one day, is not what a program
/// that names no locale means by a week number.
fn week_rule(locale: Option<&Locale>) -> WeekRule {
    locale.map_or(WeekRule::ISO, WeekRule::for_locale)
}

/// The weekday numbered from the locale's own first day of the week.
fn local_weekday(locale: Option<&Locale>, fields: &Fields) -> u8 {
    let first = locale.map_or(hc_calendar::Weekday::Monday, names::first_day_of_week);
    (fields.weekday.iso_number() + 7 - first.iso_number()) % 7 + 1
}

fn write_year<W: fmt::Write>(
    out: &mut W,
    year: i64,
    count: usize,
    digits: &[char; 10],
) -> FormatResult<()> {
    // TR 35: exactly two letters means the last two digits, any other count
    // is a minimum width.
    if count == 2 {
        return write_padded(out, year.unsigned_abs() % 100, 2, digits);
    }
    write_signed(out, year, count, digits)
}

fn write_signed<W: fmt::Write>(
    out: &mut W,
    value: i64,
    count: usize,
    digits: &[char; 10],
) -> FormatResult<()> {
    if value < 0 {
        out.write_char('-')?;
    }
    write_padded(out, value.unsigned_abs(), count, digits)
}

/// A number in the locale's digits, at least `width` places wide.
fn write_padded<W: fmt::Write>(
    out: &mut W,
    value: u64,
    width: usize,
    digits: &[char; 10],
) -> FormatResult<()> {
    zone::write_number(out, value, width, digits)?;
    Ok(())
}

fn write_fraction<W: fmt::Write>(
    out: &mut W,
    attos: u64,
    places: usize,
    digits: &[char; 10],
) -> FormatResult<()> {
    let places = places.clamp(1, 18);
    let scale = 10u64.pow(18 - places as u32);
    zone::write_number(out, attos / scale, places, digits)?;
    Ok(())
}

fn offset_of(zone: ZoneInfo) -> Option<UtcOffset> {
    zone.offset()
}

/// `Z`…`ZZZZZ`: UTS #35 Part 4 makes `Z`…`ZZZ` the same as `xxxx`,
/// `ZZZZ` the long localized GMT format and `ZZZZZ` the same as `XXXXX`.
fn write_zone_offset<W: fmt::Write>(
    out: &mut W,
    context: &FormatContext<'_>,
    count: usize,
) -> FormatResult<()> {
    let zone = context.zone;
    match count {
        1..=3 => write_iso_offset(out, zone, 4, false),
        4 => zone::write_localized_gmt(out, context.locale, zone, true),
        _ => write_iso_offset(out, zone, 5, true),
    }
}

/// `X`…`XXXXX` and `x`…`xxxxx`, by UTS #35 Part 4 (version 48.2, the
/// Date Field Symbol Table): one letter the basic form with the hours and
/// the minutes where they are not zero (`+05`, `+0530`), two the basic form
/// with both (`+0530`), three the extended form (`+05:30`), four the basic
/// form and five the extended one with the seconds where the offset has
/// them (`-045602`, `-04:56:02`). The capital letters write `Z` for a zero
/// offset. The one- to three-letter forms have no seconds field, and an
/// offset's seconds are dropped there.
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
        1 if offset.abs_minutes() == 0 => OffsetStyle::Hours,
        1 | 2 => OffsetStyle::Basic,
        3 => OffsetStyle::Extended,
        4 => crate::patterns::exact_offset_style(offset, false),
        _ => crate::patterns::exact_offset_style(offset, true),
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
    let digits = zone::digits(locale);
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
                read_field(letter, count, &mut scanner, &mut fields, locale, &digits)?;
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
    digits: &[char; 10],
) -> ParseResult<()> {
    match letter {
        'G' => {
            let index = match_era(scanner, locale)
                .ok_or_else(|| scanner.error(ErrorKind::UnknownName("era")))?;
            fields.era = Some(index);
        }
        'y' => {
            if count == 2 {
                fields.year_of_century = Some(number(scanner, digits, 2, "year")?);
            } else {
                fields.year = Some(number(scanner, digits, 10, "year")?);
            }
        }
        'Y' => {
            fields.iso_year = Some(number(scanner, digits, 10, "year")?);
            fields.week_rule = locale.map(WeekRule::for_locale);
        }
        'u' => {
            let negative = scanner.peek() == Some(b'-');
            if negative {
                scanner.advance(1);
            }
            let value = number(scanner, digits, 10, "year")?;
            fields.year = Some(if negative { -value } else { value });
        }
        'M' | 'L' => {
            if count <= 2 {
                fields.month = Some(bounded(scanner, digits, 2, 1, 12, "month")? as u8);
            } else {
                fields.month = Some(
                    match_month(scanner, locale)
                        .ok_or_else(|| scanner.error(ErrorKind::UnknownName("month")))?,
                );
            }
        }
        'd' => fields.day = Some(bounded(scanner, digits, 2, 1, 31, "day")? as u8),
        'D' => {
            fields.day_of_year = Some(bounded(scanner, digits, 3, 1, 366, "day of year")? as u16)
        }
        'w' => {
            fields.iso_week = Some(bounded(scanner, digits, 2, 1, 53, "week")? as u8);
            fields.week_rule = locale.map(WeekRule::for_locale);
        }
        'E' => {
            fields.iso_weekday = Some(
                match_weekday(scanner, locale)
                    .ok_or_else(|| scanner.error(ErrorKind::UnknownName("weekday")))?,
            );
        }
        'e' | 'c' => {
            if count <= 2 {
                let local = bounded(scanner, digits, 2, 1, 7, "weekday")? as u8;
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
        'b' | 'B' => {
            let half = match_extended_day_period(scanner, locale)
                .ok_or_else(|| scanner.error(ErrorKind::UnknownName("day period")))?;
            if half.is_some() {
                fields.day_period = half;
            }
        }
        'U' => fields.year = Some(number(scanner, digits, 10, "year")?),
        'r' => {
            let negative = scanner.peek() == Some(b'-');
            if negative {
                scanner.advance(1);
            }
            let value = number(scanner, digits, 10, "year")?;
            fields.year = Some(if negative { -value } else { value });
        }
        'g' => {
            let value = number(scanner, digits, 12, "Julian day")?;
            fields.rd = Some(hc_calendar::Rd(value - JULIAN_DAY_OF_RD_ZERO));
        }
        'h' => fields.hour12 = Some(bounded(scanner, digits, 2, 1, 12, "hour")? as u8),
        'H' => fields.hour = Some(bounded(scanner, digits, 2, 0, 23, "hour")? as u8),
        'K' => {
            let value = bounded(scanner, digits, 2, 0, 11, "hour")? as u8;
            fields.hour12 = Some(if value == 0 { 12 } else { value });
        }
        'k' => {
            let value = bounded(scanner, digits, 2, 1, 24, "hour")? as u8;
            fields.hour = Some(value % 24);
        }
        'm' => fields.minute = Some(bounded(scanner, digits, 2, 0, 59, "minute")? as u8),
        's' => fields.second = Some(bounded(scanner, digits, 2, 0, 60, "second")? as u8),
        'S' => {
            let places = count.clamp(1, 18);
            let start = scanner.pos();
            let (raw, available) = digit_run(scanner, digits, places);
            if available == 0 {
                return Err(Scanner::error_at(ErrorKind::Digit, start));
            }
            fields.subsec_attos = Some(raw * 10u64.pow(18 - available as u32));
        }
        // Consumed and discarded: these narrow a date that the other fields
        // have already fixed, and none of them can fix one on its own.
        'Q' | 'q' => {
            if count <= 2 {
                bounded(scanner, digits, 2, 1, 4, "quarter")?;
            } else {
                match_quarter(scanner, locale)
                    .ok_or_else(|| scanner.error(ErrorKind::UnknownName("quarter")))?;
            }
        }
        'W' => {
            bounded(scanner, digits, 1, 0, 6, "week of month")?;
        }
        'F' => {
            bounded(scanner, digits, 1, 1, 5, "day of week in month")?;
        }
        'A' => {
            number(scanner, digits, 9, "milliseconds")?;
        }
        'X' | 'x' | 'Z' | 'O' => fields.zone = Some(read_zone(scanner, letter)?),
        'z' | 'v' | 'V' => fields.zone = read_tolerant_zone(scanner),
        other => return Err(scanner.error(ErrorKind::UnsupportedPatternField(other))),
    }
    Ok(())
}

/// Up to `max` digits, in the locale's or in Latin ones: their value and how
/// many there were.
fn digit_run(scanner: &mut Scanner<'_>, digits: &[char; 10], max: usize) -> (u64, usize) {
    let mut value: u64 = 0;
    let mut places = 0;
    let mut bytes = 0;
    if scanner.peek().is_some_and(|byte| byte.is_ascii_digit()) {
        places = scanner.digit_run().min(max);
        for byte in &scanner.rest()[..places] {
            value = value * 10 + u64::from(byte - b'0');
        }
        bytes = places;
    } else if let Ok(text) = core::str::from_utf8(scanner.rest()) {
        for character in text.chars() {
            let Some(digit) = digits.iter().position(|candidate| *candidate == character) else {
                break;
            };
            if places == max {
                break;
            }
            value = value * 10 + digit as u64;
            places += 1;
            bytes += character.len_utf8();
        }
    }
    scanner.advance(bytes);
    (value, places)
}

/// A non-negative number of at most `max_digits` digits, in the locale's
/// digits or in Latin ones.
fn number(
    scanner: &mut Scanner<'_>,
    digits: &[char; 10],
    max_digits: usize,
    field: &'static str,
) -> ParseResult<i64> {
    scanner.skip_ascii_whitespace();
    let start = scanner.pos();
    let (value, places) = digit_run(scanner, digits, max_digits);
    if places == 0 {
        return Err(Scanner::error_at(ErrorKind::Digit, start));
    }
    i64::try_from(value).map_err(|_| Scanner::error_at(ErrorKind::OutOfRange(field), start))
}

/// [`number`] within `low..=high`.
fn bounded(
    scanner: &mut Scanner<'_>,
    digits: &[char; 10],
    max_digits: usize,
    low: i64,
    high: i64,
    field: &'static str,
) -> ParseResult<i64> {
    let start = scanner.pos();
    let value = number(scanner, digits, max_digits, field)?;
    if value < low || value > high {
        return Err(Scanner::error_at(ErrorKind::OutOfRange(field), start));
    }
    Ok(value)
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

    /// UTS #35 Part 4's table: `X` and `x` write the minutes where they
    /// are not zero (`+0530`); `XXXX`, `XXXXX`, `Z` and `ZZZZZ` write
    /// "hours, minutes and optional seconds", so `+0900` and `+09:00`, and
    /// the table's own `-075258` and `-07:52:58` for an offset with seconds.
    #[test]
    fn the_iso_zone_fields_write_minutes_and_seconds_where_tr_35_does() {
        let at = |seconds: i32| {
            context().with_zone(ZoneInfo::Offset(UtcOffset::from_seconds(seconds).unwrap()))
        };
        let write = |seconds: i32, pattern: &str| {
            let mut out = String::new();
            format(&mut out, pattern, &at(seconds)).unwrap();
            out
        };
        let fields = "X|x|XX|XXX|XXXX|XXXXX|xxxx|xxxxx|Z|ZZZZZ";
        assert_eq!(
            write(5 * 3600 + 30 * 60, fields),
            "+0530|+0530|+0530|+05:30|+0530|+05:30|+0530|+05:30|+0530|+05:30"
        );
        assert_eq!(
            write(9 * 3600, fields),
            "+09|+09|+0900|+09:00|+0900|+09:00|+0900|+09:00|+0900|+09:00"
        );
        assert_eq!(
            write(-(7 * 3600 + 52 * 60 + 58), fields),
            "-0752|-0752|-0752|-07:52|-075258|-07:52:58|-075258|-07:52:58|-075258|-07:52:58"
        );
        assert_eq!(
            write(0, "X|XXXX|XXXXX|x|xxxx|xxxxx|Z|ZZZZZ"),
            "Z|Z|Z|+00|+0000|+00:00|+0000|Z"
        );
        // Each shape reads back as the offset it states.
        for (pattern, text, seconds) in [
            ("X", "+0530", 5 * 3600 + 30 * 60),
            ("X", "+09", 9 * 3600),
            ("XXXX", "-075258", -(7 * 3600 + 52 * 60 + 58)),
            ("ZZZZZ", "-07:52:58", -(7 * 3600 + 52 * 60 + 58)),
            ("Z", "+0900", 9 * 3600),
        ] {
            let zone = parse(pattern, text).unwrap().zone;
            assert_eq!(
                zone,
                Some(ZoneInfo::Offset(UtcOffset::from_seconds(seconds).unwrap())),
                "{pattern} {text}"
            );
        }
    }

    /// Node 22.15's `Intl.DateTimeFormat` (ICU 76.1, CLDR 46), read
    /// 2026-10-03, for 2026-09-21: the numeric fields are in the locale's
    /// default numbering system, or the one `-u-nu-` names, in `ar-EG`,
    /// `fa`, `mr`, `bn`, `ne` and `en-u-nu-arab`, and in Latin digits in the
    /// locales whose default is `latn`.
    #[test]
    fn the_numeric_fields_are_written_in_the_locales_digits() {
        for (tag, pattern, expected) in [
            ("ar-EG", "d MMMM y", "٢١ سبتمبر ٢٠٢٦"),
            ("fa", "d MMMM y", "۲۱ سپتامبر ۲۰۲۶"),
            ("mr", "d MMMM, y", "२१ सप्टेंबर, २०२६"),
            ("bn", "d MMMM, y", "২১ সেপ্টেম্বর, ২০২৬"),
            ("en-u-nu-arab", "MMMM d, y", "September ٢١, ٢٠٢٦"),
            ("th-u-ca-buddhist-nu-thai", "d MMMM y", "๒๑ กันยายน ๒๕๖๙"),
            ("ar", "d MMMM y", "21 سبتمبر 2026"),
            ("th", "d MMMM y", "21 กันยายน 2026"),
        ] {
            let locale = Locale::parse(tag).unwrap();
            let context = context().with_locale(&locale);
            assert_eq!(render_in(pattern, &context), expected, "{tag}");
        }
    }

    /// The digits of a locale are read back as the numbers they write, and
    /// Latin ones still are.
    #[test]
    fn a_locales_digits_are_read_back() {
        let arabic = Locale::parse("ar-EG").unwrap();
        for text in ["٢١ سبتمبر ٢٠٢٦", "21 سبتمبر 2026"] {
            let parsed = parse_with_locale("d MMMM y", text, Some(&arabic)).unwrap();
            assert_eq!(
                (parsed.day, parsed.month, parsed.year),
                (Some(21), Some(9), Some(2026))
            );
        }
        let parsed = parse_with_locale("HH:mm:ss.SSS", "١٣:٠٥:٠٩.٥٠٠", Some(&arabic)).unwrap();
        assert_eq!(
            (
                parsed.hour,
                parsed.minute,
                parsed.second,
                parsed.subsec_attos
            ),
            (Some(13), Some(5), Some(9), Some(500_000_000_000_000_000))
        );
        assert!(parse_with_locale("d", "x", Some(&arabic)).is_err());
    }

    /// `-u-ca-buddhist` and `-u-ca-roc` write the era and the year of their
    /// calendars in `G`, `y` and `u`, as ICU does (Node 22.15, read
    /// 2026-10-03: `th-u-ca-buddhist` *พ.ศ. 2569*, `zh-TW-u-ca-roc`
    /// *民國115年9月21日*); the months and days are the Gregorian ones both
    /// calendars share.
    #[test]
    fn an_era_calendar_key_writes_its_eras_and_years() {
        for (tag, pattern, expected) in [
            ("th-u-ca-buddhist", "G y", "พ.ศ. 2569"),
            ("th-u-ca-buddhist", "d MMMM u", "21 กันยายน 2569"),
            ("th-u-ca-buddhist", "y r", "2569 2026"),
            ("zh-TW-u-ca-roc", "Gy年M月d日", "民國115年9月21日"),
            ("th", "G y", "ค.ศ. 2026"),
        ] {
            let locale = Locale::parse(tag).unwrap();
            let context = context().with_locale(&locale);
            assert_eq!(render_in(pattern, &context), expected, "{tag} {pattern}");
        }
    }

    /// A calendar the engine has no months for is refused, not written as
    /// the Gregorian one; the Gregorian and ISO keys are the engine's own.
    #[test]
    fn a_calendar_key_the_engine_cannot_write_is_refused() {
        for tag in ["ar-u-ca-islamic", "he-u-ca-hebrew", "ja-u-ca-japanese"] {
            let locale = Locale::parse(tag).unwrap();
            let mut out = String::new();
            let context = context().with_locale(&locale);
            assert!(format(&mut out, "d MMMM y", &context).is_err(), "{tag}");
        }
        for tag in ["en-u-ca-gregory", "en-u-ca-iso8601"] {
            let locale = Locale::parse(tag).unwrap();
            let context = context().with_locale(&locale);
            assert_eq!(
                render_in("d MMMM y", &context),
                "21 September 2026",
                "{tag}"
            );
        }
    }

    /// UTS #35 Part 4, "Week of Year": week 1 is the first week, starting on
    /// the locale's first day, with `minDays` days of the year. 1 January
    /// 2021 is a Friday: with Sunday and one day, the week of 27 December
    /// holds two days of 2021 and is week 1 of 2021, and 26 December is week
    /// 52 of 2020; with Monday and four days, the ISO rule, that week holds
    /// three and is week 53 of 2020; with Saturday and one day (`ar`) the
    /// week of 26 December holds six and is week 1. 1 November 2026 is a
    /// Sunday, alone in its week of the month under Monday and four days.
    #[test]
    fn the_week_fields_follow_the_locales_week_rule() {
        let at = |year, month, day| {
            FormatContext::new(CivilDateTime::new(
                hc_calendars_solar::gregorian::to_fixed(year, month, day).unwrap(),
                CivilTime::hms(12, 0, 0).unwrap(),
            ))
        };
        let write = |tag: Option<&str>, (year, month, day), pattern: &str| {
            let locale = tag.map(|tag| Locale::parse(tag).unwrap());
            let mut context = at(year, month, day);
            if let Some(locale) = locale.as_ref() {
                context = context.with_locale(locale);
            }
            let mut out = String::new();
            format(&mut out, pattern, &context).unwrap();
            out
        };
        let pattern = "Y-'W'w e";
        assert_eq!(write(Some("en-US"), (2021, 1, 1), pattern), "2021-W1 6");
        assert_eq!(write(Some("en-US"), (2020, 12, 27), pattern), "2021-W1 1");
        assert_eq!(write(Some("en-US"), (2020, 12, 26), pattern), "2020-W52 7");
        assert_eq!(write(Some("de"), (2021, 1, 1), pattern), "2020-W53 5");
        assert_eq!(write(None, (2021, 1, 1), pattern), "2020-W53 5");
        assert_eq!(write(Some("ar"), (2021, 1, 1), pattern), "2021-W1 7");
        assert_eq!(write(Some("ja"), (2021, 1, 1), pattern), "2021-W1 6");
        assert_eq!(write(Some("de"), (2026, 11, 1), "W"), "0");
        assert_eq!(write(Some("en-US"), (2026, 11, 1), "W"), "1");
        assert_eq!(write(Some("de"), (2026, 11, 2), "W"), "1");
        // UTS #35's own example: 1 January 1998 is a Thursday, and with
        // Sunday first and four days (`pt-PT`) the first three days of 1998
        // are in week 53 of 1997; with Sunday and one day (`en-US`) they are
        // in week 1.
        assert_eq!(write(Some("pt-PT"), (1998, 1, 3), "Y w"), "1997 53");
        assert_eq!(write(Some("en-US"), (1998, 1, 3), "Y w"), "1998 1");
        assert_eq!(write(Some("de"), (1997, 12, 29), "Y w"), "1998 1");
        // The first day follows `-u-fw-` and keeps the region's minimum days.
        assert_eq!(
            write(Some("en-US-u-fw-mon"), (2021, 1, 1), pattern),
            "2021-W1 5"
        );
    }

    /// A week date written with a locale's rule is read back as the day it
    /// names, in every rule and in a locale's digits.
    #[test]
    fn a_week_date_in_a_locales_rule_round_trips() {
        for tag in ["en-US", "de", "ar", "ar-EG", "fa", "ja", "pt-PT", "he"] {
            let locale = Locale::parse(tag).unwrap();
            for step in 0..800 {
                let day = Rd(739_300 + step);
                let context =
                    FormatContext::new(CivilDateTime::new(day, CivilTime::hms(0, 0, 0).unwrap()))
                        .with_locale(&locale);
                let mut text = String::new();
                format(&mut text, "Y-'W'w-e", &context).unwrap();
                let parsed = parse_with_locale("Y-'W'w-e", &text, Some(&locale)).unwrap();
                assert_eq!(
                    parsed.to_offset_date_time().unwrap().local.day,
                    day,
                    "{tag} {text}"
                );
            }
        }
    }

    /// The localized GMT format at a zero offset is the locale's
    /// `gmtZeroFormat` in the short and the long form alike: ICU4J's
    /// `TimeZoneFormat.formatOffsetLocalizedGMT` and
    /// `formatOffsetShortLocalizedGMT` both document "GMT zero format", and
    /// ICU 76.1 (Node 22's `timeZoneName: "longOffset"`) writes `GMT` in
    /// `en`, `de` and `ja` and `غرينتش` in `ar` for the one UTC instant.
    #[test]
    fn the_localized_gmt_format_writes_a_zero_offset_as_the_zero_format() {
        let utc = context().with_zone(ZoneInfo::Zulu);
        assert_eq!(render_in("O|OOOO|ZZZZ", &utc), "GMT|GMT|GMT");
        let offset_zero =
            context().with_zone(ZoneInfo::Offset(UtcOffset::from_seconds(0).unwrap()));
        assert_eq!(
            render_in("O|OOOO|ZZZZ|Z|ZZZZZ", &offset_zero),
            "GMT|GMT|GMT|+0000|Z"
        );
        // A different offset keeps the hour format.
        let plus = context().with_zone(ZoneInfo::Offset(UtcOffset::from_hms(1, 0, 0).unwrap()));
        assert_eq!(render_in("O|OOOO", &plus), "GMT+1|GMT+01:00");
    }

    #[cfg(feature = "localized-zone-names")]
    #[test]
    fn a_locales_zero_format_is_its_own_word() {
        let utc = context().with_zone(ZoneInfo::Zulu);
        for (tag, text) in [
            ("en", "GMT"),
            ("de", "GMT"),
            ("ja", "GMT"),
            ("ar", "غرينتش"),
        ] {
            let locale = Locale::parse(tag).unwrap();
            assert_eq!(
                render_in("O|OOOO", &utc.with_locale(&locale)),
                format!("{text}|{text}"),
                "{tag}"
            );
        }
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

    fn zoned(
        day: i64,
        hour: u8,
        offset_hours: i32,
        zone: &'static str,
        daylight: bool,
    ) -> FormatContext<'static> {
        FormatContext::new(CivilDateTime::new(
            Rd(day),
            CivilTime::hms(hour, 0, 0).unwrap(),
        ))
        .with_zone(ZoneInfo::Offset(
            UtcOffset::from_hms(offset_hours, 0, 0).unwrap(),
        ))
        .with_zone_id(zone)
        .with_daylight(daylight)
    }

    fn render_in(pattern: &str, context: &FormatContext<'_>) -> String {
        let mut out = String::new();
        format(&mut out, pattern, context).unwrap();
        out
    }

    /// UTS #35 Part 4's own examples, "Using Time Zone Names" and the
    /// symbol table, in English: 2026-07-01 (`Rd` 739 798) and 2026-01-15
    /// (`Rd` 739 631).
    #[cfg(feature = "zone-names")]
    #[test]
    fn the_zone_fields_write_tr_35s_examples() {
        let english = Locale::parse("en").unwrap();
        let summer = zoned(739_798, 12, -7, "America/Los_Angeles", true).with_locale(&english);
        assert_eq!(
            render_in("z|zzzz|v|vvvv|V|VV|VVV|VVVV|O|OOOO", &summer),
            "PDT|Pacific Daylight Time|PT|Pacific Time|uslax|America/Los_Angeles|Los Angeles|\
             Los Angeles Time|GMT-7|GMT-07:00"
        );
        let winter = zoned(739_631, 12, -8, "America/Los_Angeles", false).with_locale(&english);
        assert_eq!(render_in("z|zzzz", &winter), "PST|Pacific Standard Time");
        // "Pacific Time (Canada)": Vancouver is not the metazone's preferred
        // zone for the United States, English's likely country, but is
        // Canada's.
        let vancouver = zoned(739_798, 12, -7, "America/Vancouver", true).with_locale(&english);
        assert_eq!(
            render_in("vvvv|VVVV|V", &vancouver),
            "Pacific Time (Canada)|Vancouver Time|cavan"
        );
        // The generic location format: Asia/Shanghai is China's primary
        // zone, Europe/Rome Italy's only one, and Argentina has several.
        let at = |zone, offset| zoned(739_798, 12, offset, zone, false).with_locale(&english);
        assert_eq!(render_in("VVVV", &at("Asia/Shanghai", 8)), "China Time");
        assert_eq!(render_in("VVVV", &at("Europe/Rome", 2)), "Italy Time");
        assert_eq!(
            render_in("VVVV", &at("America/Buenos_Aires", -3)),
            "Buenos Aires Time"
        );
        // A zone's own name comes before its metazone's.
        let london = zoned(739_798, 12, 1, "Europe/London", true).with_locale(&english);
        assert_eq!(render_in("zzzz", &london), "British Summer Time");
        let london = zoned(739_631, 12, 0, "Europe/London", false).with_locale(&english);
        assert_eq!(render_in("zzzz|z", &london), "Greenwich Mean Time|GMT");
        // In summer, at UTC+1, `en.xml` has no short daylight name for
        // London or its `GMT` metazone, which has no daylight names at all:
        // the specific name falls to the localized GMT format rather than
        // take the standard *GMT*, which would state another offset.
        // Dublin's own long daylight name is `en.xml`'s *Irish Standard
        // Time*, the summer's, as CLDR reads Irish summer time as daylight.
        let london = zoned(739_798, 12, 1, "Europe/London", true).with_locale(&english);
        assert_eq!(render_in("z|zzzz", &london), "GMT+1|British Summer Time");
        let dublin = zoned(739_798, 12, 1, "Europe/Dublin", true).with_locale(&english);
        assert_eq!(render_in("z|zzzz", &dublin), "GMT+1|Irish Standard Time");
        let dublin = zoned(739_631, 12, 0, "Europe/Dublin", false).with_locale(&english);
        assert_eq!(render_in("z|zzzz", &dublin), "GMT|Greenwich Mean Time");
        // The IANA name reaches CLDR's identifier, Asia/Calcutta.
        let kolkata = at("Asia/Kolkata", 5);
        assert_eq!(render_in("V|vvvv", &kolkata), "inccu|India Standard Time");
    }

    /// Dublin with the main-format tzdata footer, `IST-1GMT0,M10.5.0,
    /// M3.5.0/1` (the tz database's `europe` file gives Ireland a negative
    /// saving: UTC+1 `IST` is the standard time, and winter's `GMT` is
    /// flagged as the saving): the specific names follow CLDR 48's
    /// `en.xml`, whose *Irish Standard Time* is Dublin's daylight name, so
    /// they read the summer reading, `TimeZone::is_summer_time_at`, and not
    /// the flag. July is *Irish Standard Time* and January *Greenwich Mean
    /// Time*, as they are in the rearguard format.
    #[cfg(feature = "zone-names")]
    #[test]
    fn dublin_in_the_main_format_names_its_summer_as_daylight_time() {
        use hc_tz::{PosixTimeZone, TimeZone};
        let english = Locale::parse("en").unwrap();
        for (footer, label) in [
            ("IST-1GMT0,M10.5.0,M3.5.0/1", "main"),
            ("GMT0IST,M3.5.0/1,M10.5.0", "rearguard"),
        ] {
            let dublin = PosixTimeZone::parse("Europe/Dublin", footer).unwrap();
            for (day, hour, offset, text) in [
                (739_798, 12, 1, "GMT+1|Irish Standard Time"),
                (739_631, 12, 0, "GMT|Greenwich Mean Time"),
            ] {
                let instant =
                    hc_core::UnixTime::from_seconds((day - 719_163) * 86_400 + 12 * 3_600);
                let context = zoned(day, hour, offset, "Europe/Dublin", false)
                    .with_daylight(dublin.is_summer_time_at(instant))
                    .with_zone_rules(&dublin)
                    .with_locale(&english);
                assert_eq!(render_in("z|zzzz", &context), text, "{label} {day}");
            }
        }
    }

    /// UTS #35 Part 4's last type fallback, whose example is "Mountain
    /// Standard Time" for Phoenix: where the offset and the daylight offset do not
    /// change within 184 days either side of the instant, the generic name
    /// is written as the standard one. Phoenix keeps MST all year (`MST7`);
    /// Denver changes on 8 March 2026, 52 days after 2026-01-15 (`Rd`
    /// 739 631), so its generic name stands; Tokyo keeps JST (`JST-9`).
    /// With no rules in the context, the generic name stands too.
    #[cfg(feature = "zone-names")]
    #[test]
    fn a_zone_that_keeps_one_offset_for_184_days_takes_its_standard_name() {
        use hc_tz::PosixTimeZone;
        let english = Locale::parse("en").unwrap();
        let phoenix = PosixTimeZone::parse("America/Phoenix", "MST7").unwrap();
        let denver = PosixTimeZone::parse("America/Denver", "MST7MDT,M3.2.0,M11.1.0").unwrap();
        let tokyo = PosixTimeZone::parse("Asia/Tokyo", "JST-9").unwrap();
        let winter = |zone, offset| zoned(739_631, 12, offset, zone, false).with_locale(&english);
        assert_eq!(
            render_in("vvvv|v|VVVV", &winter("America/Phoenix", -7)),
            "Mountain Time (Phoenix)|MT (Phoenix)|Phoenix Time"
        );
        assert_eq!(
            render_in(
                "vvvv|v|VVVV",
                &winter("America/Phoenix", -7).with_zone_rules(&phoenix)
            ),
            "Mountain Standard Time|MST|Phoenix Time"
        );
        assert_eq!(
            render_in(
                "vvvv|v",
                &winter("America/Denver", -7).with_zone_rules(&denver)
            ),
            "Mountain Time|MT"
        );
        let summer = zoned(739_798, 12, -6, "America/Denver", true)
            .with_locale(&english)
            .with_zone_rules(&denver);
        assert_eq!(
            render_in("vvvv|zzzz", &summer),
            "Mountain Time|Mountain Daylight Time"
        );
        assert_eq!(render_in("vvvv", &winter("Asia/Tokyo", 9)), "Japan Time");
        assert_eq!(
            render_in("vvvv", &winter("Asia/Tokyo", 9).with_zone_rules(&tokyo)),
            "Japan Standard Time"
        );
        // Denver on 2026-09-16 (`Rd` 739 875): the change of 1 November is 46 days
        // ahead, so the generic name stands however the flag reads.
        let september = zoned(739_875, 12, -7, "America/Denver", false)
            .with_locale(&english)
            .with_zone_rules(&denver);
        assert_eq!(render_in("vvvv", &september), "Mountain Time");
    }

    #[test]
    fn a_zone_field_falls_back_where_its_names_are_unavailable() {
        // No daylight flag, so no specific name; and with no zone at all,
        // `V` is `unk` and `VVV` the unknown zone's city.
        let mut unknown =
            context().with_zone(ZoneInfo::Offset(UtcOffset::from_hms(9, 0, 0).unwrap()));
        assert_eq!(
            render_in("z|zzzz|V|VVV", &unknown),
            "GMT+9|GMT+09:00|unk|Unknown Location"
        );
        unknown.zone_id = Some("Etc/GMT-9");
        assert_eq!(render_in("VVVV", &unknown), "GMT+09:00");
    }

    #[test]
    fn the_localized_gmt_format_is_the_locales() {
        // `fr.xml`: "UTC{0}" and "+HH:mm;−HH:mm", a minus sign.
        let french = Locale::parse("fr").unwrap();
        let paris = context()
            .with_zone(ZoneInfo::Offset(UtcOffset::from_hms(-3, -30, 0).unwrap()))
            .with_locale(&french);
        assert_eq!(
            render_in("O|OOOO|ZZZZ", &paris),
            "UTC−3:30|UTC−03:30|UTC−03:30"
        );
        let utc = context().with_zone(ZoneInfo::Zulu).with_locale(&french);
        assert_eq!(render_in("O", &utc), "UTC");
    }

    #[cfg(feature = "localized-zone-names")]
    #[test]
    fn the_zone_names_are_the_locales() {
        let japanese = Locale::parse("ja").unwrap();
        let tokyo = zoned(739_798, 12, 9, "Asia/Tokyo", false).with_locale(&japanese);
        assert_eq!(render_in("zzzz|VVV", &tokyo), "日本標準時|東京");
        let german = Locale::parse("de").unwrap();
        let berlin = zoned(739_798, 12, 2, "Europe/Berlin", true).with_locale(&german);
        assert_eq!(
            render_in("zzzz|z", &berlin),
            "Mitteleuropäische Sommerzeit|MESZ"
        );
        // `en_GB.xml` gives London's short daylight name, *BST*, which
        // `en.xml` does not; `ar-EG` writes the offset in its digits.
        let british = Locale::parse("en-GB").unwrap();
        let london = zoned(739_798, 12, 1, "Europe/London", true).with_locale(&british);
        assert_eq!(render_in("z", &london), "BST");
        let egyptian = Locale::parse("ar-EG").unwrap();
        let london = zoned(739_798, 12, 1, "Europe/London", true).with_locale(&egyptian);
        assert_eq!(render_in("z", &london), "غرينتش+١");
    }

    /// `g`, the Julian day number of the local date: 2 440 588 for
    /// 1970-01-01 and 2 451 545 for 2000-01-01, whose noon is J2000.0.
    #[test]
    fn the_julian_day_and_the_related_and_cyclic_years() {
        let day = |rd| FormatContext::new(CivilDateTime::midnight(Rd(rd)));
        assert_eq!(render_in("g", &day(719_163)), "2440588");
        assert_eq!(render_in("g", &day(730_120)), "2451545");
        assert_eq!(render_in("r|U|UU", &context()), "2026|2026|26");
    }

    /// `b` and `B` against `dayPeriods.xml` and the locales' files:
    /// English says *noon* and *midnight* and has *in the afternoon* and
    /// *at night*; German has *Mitternacht* but no word for 12:00, so noon
    /// is *PM* for `b` and the afternoon period *mittags* (12:00–13:00)
    /// for `B`; Japanese's 夜中 runs from 23:00 across midnight.
    #[test]
    fn the_extended_day_periods_follow_each_languages_rules() {
        let at = |hour, minute, tag: &str| {
            let locale = Locale::parse(tag).unwrap();
            let context = FormatContext::new(CivilDateTime::new(
                Rd(739_880),
                CivilTime::hms(hour, minute, 0).unwrap(),
            ))
            .with_locale(&locale);
            render_in("b|bbbb|B|BBBB", &context)
        };
        assert_eq!(at(12, 0, "en"), "noon|noon|noon|noon");
        assert_eq!(at(0, 0, "en"), "midnight|midnight|midnight|midnight");
        assert_eq!(at(15, 30, "en"), "PM|PM|in the afternoon|in the afternoon");
        assert_eq!(at(22, 0, "en"), "PM|PM|at night|at night");
        assert_eq!(
            at(0, 0, "de"),
            "Mitternacht|Mitternacht|Mitternacht|Mitternacht"
        );
        assert_eq!(at(12, 0, "de"), "PM|PM|mittags|mittags");
        assert_eq!(at(1, 0, "ja"), "午前|午前|夜中|夜中");
        // With no locale, `b` and `B` are the `C` am and pm.
        assert_eq!(render_in("b|B", &context()), "PM|PM");
    }

    #[test]
    fn the_new_fields_parse_back() {
        let parsed = parse("g", "2451545").unwrap();
        assert_eq!(parsed.to_offset_date_time().unwrap().local.day, Rd(730_120));
        let english = Locale::parse("en").unwrap();
        let evening = parse_with_locale("h:mm B", "8:15 in the evening", Some(&english)).unwrap();
        assert_eq!(evening.day_period, Some(DayPeriod::Pm));
        let noon = parse_with_locale("h b", "12 noon", Some(&english)).unwrap();
        assert_eq!(noon.day_period, Some(DayPeriod::Pm));
        assert_eq!(parse("r-MM-dd", "2026-09-21").unwrap().year, Some(2026));
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
            format(&mut out, "n", &context()).unwrap_err(),
            FormatError::UnknownField('n')
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
            parse("n", "morning").unwrap_err().kind(),
            ErrorKind::UnsupportedPatternField('n')
        );
    }
}
