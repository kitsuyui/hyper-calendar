//! Python's `datetime.strptime`, resolved the way CPython's `_strptime`
//! module resolves it.
//!
//! CPython does not read a pattern directive by directive. It turns the
//! pattern into one regular expression — each directive an ordered list of
//! alternatives, each run of whitespace `\s+` — matches it at the start of
//! the text, with backtracking, and refuses what the match leaves over. Two
//! parsers that read the same directives can therefore disagree about the
//! same text: `%Y%m%d` matches `20191204` in Python because `%Y` is exactly
//! four digits, and a parser that reads a year greedily does not.
//! This module is that regular expression, written out as code: a
//! directive's alternatives are tried in CPython's order and the rest of
//! the pattern is tried after each, so the first overall match is the one
//! CPython finds.
//!
//! The alternatives are those of `_strptime.TimeRE` for the 3.13 line,
//! checked against CPython 3.12 and 3.14 by running both on the same
//! patterns and texts (see `docs/python-parity.md`). The documentation
//! (<https://docs.python.org/3/library/datetime.html>, "strftime() and
//! strptime() Format Codes") states the rules this follows: whitespace in
//! the pattern matches any amount of whitespace, parsing ignores case, a
//! `%y` of 69 to 99 is 1969 to 1999 and of 0 to 68 is 2000 to 2068, and
//! `%U` and `%W` count only beside a year and a weekday.
//!
//! After the match the fields are resolved as `_strptime` does: a
//! day of the year is added to 1 January, so day 366 of a common year is 1
//! January of the next; the week of the year and the ISO week are counted
//! from 1 January and from 4 January without being checked, so week 53 of
//! a 52-week year is week 1 of the next; the ISO directives must come
//! together; and a pattern with no year is dated 1900.
//!
//! # What differs from CPython
//!
//! * `\d` in CPython's regular expressions is any Unicode decimal digit;
//!   here it is `0`–`9`.
//! * `%Z` accepts `UTC` and `GMT`. CPython also accepts the names of the
//!   machine's own zone, which a library with no hidden zone does not have.
//!   Neither sets the zone, as in CPython.
//! * `%z` with a fraction of a second is refused: `UtcOffset` counts whole
//!   seconds.
//! * A year is 0000 to 9999 as four digits give it; CPython refuses year 0.
//!   A second of 60 is kept as a leap second; CPython's `datetime` refuses it.
//! * An offset of 24 hours or more is refused, as CPython's `timezone` does.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_tz::UtcOffset;

use crate::error::{ErrorKind, ParseError, ParseResult};
use crate::patterns::ParsedFields;
use crate::value::ZoneInfo;

/// The directives `_strptime.TimeRE` knows.
const DIRECTIVES: &[u8] = b"AaBbcdfGHIjMmpSUuVWwXxYyZz%";

/// CPython's names in the C locale, for `%A`, `%a`, `%B`, `%b`.
const WEEKDAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];
const WEEKDAYS_SHORT: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const MONTHS_SHORT: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// The alternatives of a numeric directive, in CPython's order. In an
/// alternative `d` is any digit, `[a-b]` a range and anything else itself.
fn alternatives(directive: u8) -> &'static [&'static str] {
    match directive {
        b'd' => &["3[0-1]", "[1-2]d", "0[1-9]", "[1-9]", " [1-9]"],
        b'H' => &["2[0-3]", "[0-1]d", "d"],
        b'I' => &["1[0-2]", "0[1-9]", "[1-9]", " [1-9]"],
        b'G' | b'Y' => &["dddd"],
        b'j' => &[
            "36[0-6]", "3[0-5]d", "[1-2]dd", "0[1-9]d", "00[1-9]", "[1-9]d", "0[1-9]", "[1-9]",
        ],
        b'm' => &["1[0-2]", "0[1-9]", "[1-9]"],
        b'M' => &["[0-5]d", "d"],
        b'S' => &["6[0-1]", "[0-5]d", "d"],
        b'U' | b'W' => &["5[0-3]", "[0-4]d", "d"],
        b'w' => &["[0-6]"],
        b'u' => &["[1-7]"],
        b'V' => &["5[0-3]", "0[1-9]", "[1-4]d", "d"],
        b'y' => &["dd"],
        _ => &[],
    }
}

/// The length of `alternative` matched at the start of `text`.
fn match_alternative(alternative: &str, text: &[u8]) -> Option<usize> {
    let pattern = alternative.as_bytes();
    let mut index = 0;
    let mut length = 0;
    while let Some(&byte) = pattern.get(index) {
        let found = *text.get(length)?;
        match byte {
            b'[' => {
                let low = *pattern.get(index + 1)?;
                let high = *pattern.get(index + 3)?;
                if !(low..=high).contains(&found) {
                    return None;
                }
                index += 5;
            }
            b'd' => {
                if !found.is_ascii_digit() {
                    return None;
                }
                index += 1;
            }
            literal => {
                if found != literal {
                    return None;
                }
                index += 1;
            }
        }
        length += 1;
    }
    Some(length)
}

/// A continuation: the pattern still to match once the current one is done.
struct Continuation<'a> {
    pattern: &'a str,
    next: Option<&'a Continuation<'a>>,
}

/// The capture of one directive: where it matched.
#[derive(Debug, Clone, Copy)]
struct Capture {
    directive: u8,
    start: usize,
    end: usize,
}

const MAX_CAPTURES: usize = 32;

struct Matcher<'t> {
    text: &'t str,
    captures: [Option<Capture>; MAX_CAPTURES],
    count: usize,
}

fn is_space(character: char) -> bool {
    character.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&character)
}

/// The text of a compound directive, in the C locale.
fn expansion(directive: char) -> Option<&'static str> {
    match directive {
        'c' => Some("%a %b %d %H:%M:%S %Y"),
        'x' => Some("%m/%d/%y"),
        'X' => Some("%H:%M:%S"),
        _ => None,
    }
}

impl Matcher<'_> {
    fn push(&mut self, capture: Capture) -> bool {
        match self.captures.get_mut(self.count) {
            Some(slot) => {
                *slot = Some(capture);
                self.count += 1;
                true
            }
            None => false,
        }
    }

    /// Match `pattern`, then whatever follows it, at `position`; the end of
    /// the whole match.
    fn run(
        &mut self,
        pattern: &str,
        next: Option<&Continuation<'_>>,
        position: usize,
    ) -> Option<usize> {
        let Some(first) = pattern.chars().next() else {
            return match next {
                Some(continuation) => self.run(continuation.pattern, continuation.next, position),
                None => Some(position),
            };
        };
        let rest = pattern.get(first.len_utf8()..)?;
        if is_space(first) {
            // A whole run of whitespace in the pattern is one `\s+`, which
            // takes the longest run in the text it can and gives it back
            // one character at a time.
            let rest = rest.trim_start_matches(is_space);
            let tail = self.text.get(position..)?;
            let run: usize = tail
                .chars()
                .take_while(|character| is_space(*character))
                .map(char::len_utf8)
                .sum();
            let taken = tail.get(..run)?;
            let mut ends: [usize; 64] = [0; 64];
            let mut length = 0usize;
            let mut end = 0usize;
            for character in taken.chars() {
                end += character.len_utf8();
                if let Some(slot) = ends.get_mut(length) {
                    *slot = end;
                    length += 1;
                }
            }
            for index in (0..length).rev() {
                let end = ends.get(index).copied().unwrap_or(0);
                if let Some(done) = self.run(rest, next, position + end) {
                    return Some(done);
                }
            }
            return None;
        }
        if first != '%' {
            let found = self.text.get(position..)?.chars().next()?;
            if !same_letter(found, first) {
                return None;
            }
            return self.run(rest, next, position + found.len_utf8());
        }
        // The directive is validated before matching starts.
        let directive = rest.chars().next()?;
        let rest = rest.get(directive.len_utf8()..)?;
        if directive == '%' {
            if self.text.get(position..)?.chars().next()? != '%' {
                return None;
            }
            return self.run(rest, next, position + 1);
        }
        if let Some(expanded) = expansion(directive) {
            let continuation = Continuation {
                pattern: rest,
                next,
            };
            return self.run(expanded, Some(&continuation), position);
        }
        let directive = u8::try_from(directive).ok()?;
        let mut lengths = [0usize; 16];
        let count = self.candidates(directive, position, &mut lengths);
        for length in lengths.iter().take(count).copied() {
            let slot = self.count;
            if !self.push(Capture {
                directive,
                start: position,
                end: position + length,
            }) {
                return None;
            }
            if let Some(done) = self.run(rest, next, position + length) {
                return Some(done);
            }
            self.count = slot;
            if let Some(entry) = self.captures.get_mut(slot) {
                *entry = None;
            }
        }
        None
    }

    /// The lengths `directive` can match at `position`, in the order the
    /// regular expression tries them.
    fn candidates(&self, directive: u8, position: usize, out: &mut [usize; 16]) -> usize {
        let Some(tail) = self.text.get(position..) else {
            return 0;
        };
        let bytes = tail.as_bytes();
        let mut count = 0usize;
        let mut add = |length: usize| {
            if let Some(slot) = out.get_mut(count) {
                *slot = length;
                count += 1;
            }
        };
        match directive {
            b'f' => {
                let run = bytes
                    .iter()
                    .take_while(|byte| byte.is_ascii_digit())
                    .count();
                for length in (1..=run.min(6)).rev() {
                    add(length);
                }
            }
            b'z' => {
                if bytes.first() == Some(&b'Z') {
                    add(1);
                }
                let mut lengths = [0usize; 8];
                for length in zone_offsets(bytes, &mut lengths).iter().copied() {
                    add(length);
                }
            }
            b'A' => name_lengths(tail, &WEEKDAYS, &mut add),
            b'a' => name_lengths(tail, &WEEKDAYS_SHORT, &mut add),
            b'B' => name_lengths(tail, &MONTHS, &mut add),
            b'b' => name_lengths(tail, &MONTHS_SHORT, &mut add),
            b'p' => name_lengths(tail, &["AM", "PM"], &mut add),
            b'Z' => name_lengths(tail, &["UTC", "GMT"], &mut add),
            other => {
                for alternative in alternatives(other) {
                    if let Some(length) = match_alternative(alternative, bytes) {
                        add(length);
                    }
                }
            }
        }
        count
    }
}

/// Whether two characters are the same letter, ignoring case.
fn same_letter(found: char, wanted: char) -> bool {
    found == wanted || found.to_lowercase().eq(wanted.to_lowercase())
}

/// The lengths of the names that start `text`, longest first, as
/// `_strptime` sorts them.
fn name_lengths(text: &str, names: &[&str], add: &mut impl FnMut(usize)) {
    let mut lengths = [0usize; 12];
    let mut count = 0usize;
    for name in names {
        let prefix = text.as_bytes().get(..name.len());
        if prefix.is_some_and(|prefix| prefix.eq_ignore_ascii_case(name.as_bytes()))
            && let Some(slot) = lengths.get_mut(count)
        {
            *slot = name.len();
            count += 1;
        }
    }
    let found = lengths.get_mut(..count).unwrap_or(&mut []);
    found.sort_unstable_by(|left, right| right.cmp(left));
    for length in found.iter().copied() {
        add(length);
    }
}

/// The lengths of `[+-]\d\d:?[0-5]\d(:?[0-5]\d(\.\d{1,6})?)?` that match at
/// the start of `bytes`, in the order a backtracking match tries them: with
/// the fraction longest first, with the seconds, and without.
fn zone_offsets<'a>(bytes: &[u8], out: &'a mut [usize; 8]) -> &'a [usize] {
    let digit = |index: usize| bytes.get(index).is_some_and(u8::is_ascii_digit);
    let below_six = |index: usize| {
        bytes
            .get(index)
            .is_some_and(|byte| (b'0'..=b'5').contains(byte))
    };
    let mut count = 0usize;
    if matches!(bytes.first(), Some(b'+' | b'-')) && digit(1) && digit(2) {
        let mut at = 3;
        if bytes.get(at) == Some(&b':') {
            at += 1;
        }
        if below_six(at) && digit(at + 1) {
            let minutes_end = at + 2;
            let mut push = |length: usize| {
                if let Some(slot) = out.get_mut(count) {
                    *slot = length;
                    count += 1;
                }
            };
            // The optional seconds group.
            let mut seconds = minutes_end;
            if bytes.get(seconds) == Some(&b':') {
                seconds += 1;
            }
            if below_six(seconds) && digit(seconds + 1) {
                let seconds_end = seconds + 2;
                if bytes.get(seconds_end) == Some(&b'.') {
                    let run = (seconds_end + 1..)
                        .take_while(|index| digit(*index))
                        .count()
                        .min(6);
                    for digits in (1..=run).rev() {
                        push(seconds_end + 1 + digits);
                    }
                }
                push(seconds_end);
            }
            push(minutes_end);
        }
    }
    out.get(..count).unwrap_or(&[])
}

/// Check the pattern the way compiling it would: every directive known, none
/// used twice, no stray `%`.
fn validate(pattern: &str, seen: &mut [bool; 128], depth: u8) -> ParseResult<()> {
    let mut characters = pattern.chars();
    while let Some(character) = characters.next() {
        if character != '%' {
            continue;
        }
        let Some(directive) = characters.next() else {
            return Err(ParseError::new(ErrorKind::UnsupportedPatternField('%'), 0));
        };
        let byte = u8::try_from(directive)
            .ok()
            .filter(|byte| DIRECTIVES.contains(byte))
            .ok_or(ParseError::new(
                ErrorKind::UnsupportedPatternField(directive),
                0,
            ))?;
        if directive == '%' {
            continue;
        }
        if let Some(expanded) = expansion(directive) {
            if depth > 2 {
                return Err(ParseError::new(ErrorKind::PatternMismatch, 0));
            }
            validate(expanded, seen, depth + 1)?;
            continue;
        }
        let slot = seen
            .get_mut(usize::from(byte))
            .ok_or(ParseError::new(ErrorKind::PatternMismatch, 0))?;
        if *slot {
            return Err(ParseError::new(
                ErrorKind::Forbidden("a directive used twice"),
                0,
            ));
        }
        *slot = true;
    }
    Ok(())
}

fn digits(text: &str) -> i64 {
    text.bytes()
        .filter(u8::is_ascii_digit)
        .fold(0i64, |sum, byte| sum * 10 + i64::from(byte - b'0'))
}

fn invalid(what: &'static str, offset: usize) -> ParseError {
    ParseError::new(ErrorKind::Invalid(what), offset)
}

fn out_of_range(offset: usize) -> ParseError {
    invalid("date", offset)
}

/// `datetime.strptime(text, pattern)`, as fields.
///
/// # Errors
///
/// A [`ParseError`] when the pattern names a directive `_strptime` does not
/// know or uses one twice, when the text does not match, when text is left
/// over, and for a combination of directives `_strptime` refuses.
pub(super) fn parse(pattern: &str, text: &str) -> ParseResult<ParsedFields> {
    let mut seen = [false; 128];
    validate(pattern, &mut seen, 0)?;
    let mut matcher = Matcher {
        text,
        captures: [None; MAX_CAPTURES],
        count: 0,
    };
    let Some(end) = matcher.run(pattern, None, 0) else {
        return Err(ParseError::new(ErrorKind::PatternMismatch, 0));
    };
    if end != text.len() {
        return Err(ParseError::new(ErrorKind::TrailingText, end));
    }
    resolve(&matcher)
}

fn resolve(matcher: &Matcher<'_>) -> ParseResult<ParsedFields> {
    let mut year: Option<i64> = None;
    let (mut month, mut day) = (1i64, 1i64);
    let (mut hour, mut minute, mut second, mut micros) = (0i64, 0i64, 0i64, 0i64);
    let mut weekday: Option<i64> = None;
    let (mut julian, mut iso_year, mut iso_week): (Option<i64>, Option<i64>, Option<i64>) =
        (None, None, None);
    let mut week_of_year: Option<(i64, bool)> = None;
    let pm = matcher
        .captures
        .iter()
        .take(matcher.count)
        .flatten()
        .find(|capture| capture.directive == b'p')
        .map(|capture| {
            matcher
                .text
                .get(capture.start..capture.end)
                .is_some_and(|text| text.eq_ignore_ascii_case("pm"))
        });
    let mut zone: Option<ZoneInfo> = None;
    for capture in matcher.captures.iter().take(matcher.count).flatten() {
        let text = matcher.text.get(capture.start..capture.end).unwrap_or("");
        let value = digits(text);
        match capture.directive {
            b'y' => {
                year = Some(if value <= 68 {
                    value + 2000
                } else {
                    value + 1900
                })
            }
            b'Y' => year = Some(value),
            b'G' => iso_year = Some(value),
            b'm' => month = value,
            b'B' | b'b' => {
                let names = if capture.directive == b'B' {
                    &MONTHS
                } else {
                    &MONTHS_SHORT
                };
                month = position_of(names, text) + 1;
            }
            b'd' => day = value,
            b'H' => hour = value,
            // 12 AM is hour 0 and 12 PM hour 12; without a `%p` it reads as AM.
            b'I' => {
                hour = match pm {
                    Some(true) if value != 12 => value + 12,
                    Some(true) => value,
                    _ if value == 12 => 0,
                    _ => value,
                };
            }
            b'M' => minute = value,
            b'S' => second = value,
            b'f' => {
                // Padded on the right to six digits.
                micros = value * 10_i64.pow(6 - u32::try_from(text.len()).unwrap_or(6));
            }
            b'A' | b'a' => {
                let names = if capture.directive == b'A' {
                    &WEEKDAYS
                } else {
                    &WEEKDAYS_SHORT
                };
                weekday = Some(position_of(names, text));
            }
            b'w' => weekday = Some(if value == 0 { 6 } else { value - 1 }),
            b'u' => weekday = Some(value - 1),
            b'j' => julian = Some(value),
            b'U' => week_of_year = Some((value, false)),
            b'W' => week_of_year = Some((value, true)),
            b'V' => iso_week = Some(value),
            b'z' => zone = Some(read_offset(text, capture.start)?),
            _ => {}
        }
    }
    // Directives that only make sense together.
    if iso_year.is_some() {
        if julian.is_some() {
            return Err(invalid("the day of the year beside an ISO year", 0));
        }
        if iso_week.is_none() || weekday.is_none() {
            return Err(invalid("an ISO year without an ISO week and a weekday", 0));
        }
    } else if iso_week.is_some() {
        return Err(invalid("an ISO week without an ISO year", 0));
    }
    if second > 60 {
        return Err(ParseError::new(ErrorKind::OutOfRange("second"), 0));
    }

    let year = year.unwrap_or(1900);
    let january_first = |year: i64| gregorian::to_fixed(year, 1, 1).map_err(|_| out_of_range(0));
    let mut fields = ParsedFields::default();
    let mut day_number = julian;
    let mut base_year = year;
    if day_number.is_none()
        && let Some(weekday) = weekday
    {
        if let Some((week, starts_monday)) = week_of_year {
            let first = weekday_of(january_first(year)?);
            let (first, weekday) = if starts_monday {
                (first, weekday)
            } else {
                ((first + 1) % 7, (weekday + 1) % 7)
            };
            let week_zero_length = (7 - first) % 7;
            day_number = Some(if week == 0 {
                1 + weekday - first
            } else {
                1 + week_zero_length + 7 * (week - 1) + weekday
            });
        } else if let (Some(iso_year), Some(iso_week)) = (iso_year, iso_week) {
            // A week 53 exists only in a year with 53 weeks: one that
            // begins on a Thursday, or on a Wednesday if it is a leap year.
            if iso_week == 53 {
                let first = weekday_of(january_first(iso_year)?);
                let leap = gregorian::is_leap_year(iso_year);
                if !(first == 3 || (leap && first == 2)) {
                    return Err(invalid("an ISO week 53 in a year of 52 weeks", 0));
                }
            }
            let fourth = gregorian::to_fixed(iso_year, 1, 4).map_err(|_| out_of_range(0))?;
            let correction = weekday_of(fourth) + 1 + 3;
            day_number = Some(iso_week * 7 + weekday + 1 - correction);
            base_year = iso_year;
        }
    }
    if let Some(number) = day_number {
        let day = january_first(base_year)?
            .checked_add_days(number - 1)
            .map_err(|_| out_of_range(0))?;
        fields.rd = Some(day);
    } else {
        fields.year = Some(year);
        fields.month = u8::try_from(month).ok();
        fields.day = u8::try_from(day).ok();
    }
    fields.hour = u8::try_from(hour).ok();
    fields.minute = u8::try_from(minute).ok();
    fields.second = u8::try_from(second).ok();
    fields.subsec_attos = Some(u64::try_from(micros).unwrap_or(0) * 1_000_000_000_000);
    fields.zone = zone;
    Ok(fields)
}

/// The index of `name` among `names`, ignoring case.
fn position_of(names: &[&str], name: &str) -> i64 {
    names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .map_or(0, |index| index as i64)
}

/// The weekday of a fixed day, Monday 0.
fn weekday_of(day: Rd) -> i64 {
    (day.0 - 1).rem_euclid(7)
}

/// A `%z` capture as `_strptime` reads it.
fn read_offset(text: &str, offset: usize) -> ParseResult<ZoneInfo> {
    if text == "Z" {
        return Ok(ZoneInfo::Zulu);
    }
    let bytes = text.as_bytes();
    let negative = bytes.first() == Some(&b'-');
    let number = |from: usize| -> i64 {
        bytes.get(from..from + 2).map_or(0, |pair| {
            i64::from(pair[0] - b'0') * 10 + i64::from(pair[1] - b'0')
        })
    };
    let mut at = 3;
    let colon = bytes.get(at) == Some(&b':');
    if colon {
        at += 1;
    }
    let minutes = number(at);
    at += 2;
    // The seconds must use the colon, or not, as the minutes did.
    let seconds_colon = bytes.get(at) == Some(&b':');
    if seconds_colon {
        at += 1;
    }
    if bytes.len() > at && seconds_colon != colon {
        return Err(invalid("an offset that uses ':' inconsistently", offset));
    }
    let seconds = if bytes.len() > at { number(at) } else { 0 };
    if bytes.len() > at + 2 {
        return Err(ParseError::new(
            ErrorKind::Unrepresentable("an offset with a fraction of a second"),
            offset + at + 2,
        ));
    }
    let total = number(1) * 3_600 + minutes * 60 + seconds;
    if total >= 86_400 {
        return Err(ParseError::new(
            ErrorKind::OutOfRange("the offset, which must be under 24 hours"),
            offset,
        ));
    }
    let signed = i32::try_from(if negative { -total } else { total })
        .map_err(|_| invalid("offset", offset))?;
    UtcOffset::from_seconds(signed)
        .map(ZoneInfo::Offset)
        .map_err(|_| invalid("offset", offset))
}
