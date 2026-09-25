//! A loader for lists this crate does not ship.
//!
//! The Finnish lists are the University of Helsinki's, licensed for a fee
//! per copy; the Norwegian list is Almanakkforlaget's, free only for
//! editorial use with credit; the Swedish list has no stated terms at all.
//! None of them can be vendored under this workspace's BSD-3-Clause
//! licence, and none of them is any less a name-day list for that. So the
//! evaluator reads a list a caller supplies at run time — one the caller
//! has licensed, or one the caller wrote — through the same [`NameDays`]
//! trait the vendored tables implement. This crate distributes no such
//! list and no fixture of one; its own tests use a synthetic list and the
//! Latvian tables rendered to text and read back.
//!
//! # The text format
//!
//! One list per text. Lines are trimmed; blank lines and lines starting
//! with `#` are ignored. Every other line is `key = value`.
//!
//! A key of the form `MM-DD` is a day. Its value is the day's names,
//! separated by `;` — so a name may contain a space — and an empty value is
//! a day with no names. Every one of the 366 days of a leap year, 29
//! February included, must appear exactly once: a hole is a statement and
//! has to be made.
//!
//! Any other key is a header field and must come before the first day:
//!
//! | Key | Required | Value |
//! |---|---|---|
//! | `id` | yes | an identifier, as [`NameDayList::id`] |
//! | `validity` | yes | `FROM..TO`, `FROM..`, `..TO` or `..`, in years |
//! | `leap-day` | yes | `no-names`, `own-names`, `shift-after-24-february` or `leap-years-only`, as [`LeapDayRule::id`] |
//! | `source` | yes | the citation |
//! | `country` | no | ISO 3166-1 alpha-2 |
//! | `language` | no | BCP 47 |
//! | `name` | no | the list's name in English |
//! | `authority` | no | whose list it is |
//! | `licence` | no | the terms the caller holds it under |
//! | `unlisted-names-day` | no | `MM-DD`, the day reserved for names not listed |
//!
//! A leap-day rule that requires a slot to be empty — `no-names` on 29
//! February, `shift-after-24-february` on 24 February — is checked against
//! the days, and a list that contradicts its own rule is refused.
//!
//! ```
//! use hc_name_days::load::OwnedNameDayList;
//! use hc_name_days::{LeapDayRule, MonthDay, names_on};
//!
//! let mut text = String::from(
//!     "# A synthetic list.\n\
//!      id = zz-synthetic-2026\n\
//!      validity = 2026..\n\
//!      leap-day = no-names\n\
//!      source = this doctest\n\
//!      unlisted-names-day = 05-22\n",
//! );
//! for month in 1..=12u8 {
//!     let length = hc_calendar::gregorian::days_in_month(2000, month).unwrap_or(0);
//!     for day in 1..=length {
//!         let names = if (month, day) == (1, 1) { "Alpha; Beta" } else { "" };
//!         text.push_str(&format!("{month:02}-{day:02} = {names}\n"));
//!     }
//! }
//! let list = OwnedNameDayList::parse(&text)?;
//! assert_eq!(list.id, "zz-synthetic-2026");
//! assert_eq!(list.leap_day, LeapDayRule::NoNames);
//! assert_eq!(list.unlisted_names_day, Some(MonthDay::new(5, 22)));
//! assert_eq!(names_on(&list, 2026, 1, 1)?, ["Alpha", "Beta"]);
//! assert!(names_on(&list, 2028, 2, 29)?.is_empty());
//! # Ok::<(), Box<dyn core::error::Error>>(())
//! ```
//!
//! # Rendering
//!
//! [`NameDayList::to_text`] writes a vendored list in the same format, so a
//! caller can start a file from one, and so the tests can prove the two
//! directions agree on every slot of every Latvian edition.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{self, Write as _};

use crate::list::{
    DAYS, LeapDayRule, MonthDay, NameDayList, NameDays, Validity, layout_index, layout_month_day,
};

/// A list read from text at run time.
///
/// The header fields are public so that a caller who printed them can read
/// them back; the days are reached through [`NameDays`] and
/// [`OwnedNameDayList::slot`], as for a vendored list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedNameDayList {
    /// The `id` header.
    pub id: String,
    /// The `country` header, or empty.
    pub country: String,
    /// The `language` header, or empty.
    pub language: String,
    /// The `name` header, or empty.
    pub english_name: String,
    /// The `authority` header, or empty.
    pub authority: String,
    /// The `validity` header.
    pub validity: Validity,
    /// The `leap-day` header.
    pub leap_day: LeapDayRule,
    /// The `licence` header, or empty.
    pub licence: String,
    /// The `source` header.
    pub source: String,
    /// The `unlisted-names-day` header, if given.
    pub unlisted_names_day: Option<MonthDay>,
    days: Vec<Vec<String>>,
}

/// Why a text is not a list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// The one-based line the error is on; 0 when it is about the whole
    /// text.
    pub line: usize,
    /// What is wrong.
    pub kind: ParseErrorKind,
}

/// What is wrong with a line, or with the text as a whole.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseErrorKind {
    /// A line without `=`.
    MissingSeparator,
    /// A header key the format does not define.
    UnknownKey(String),
    /// A header given twice.
    DuplicateHeader(String),
    /// A header after the first day line.
    HeaderAfterDays(String),
    /// A required header with no value, or absent altogether.
    MissingHeader(&'static str),
    /// A `validity` value that is not `FROM..TO`.
    BadValidity(String),
    /// A `leap-day` value that names no rule.
    BadLeapDayRule(String),
    /// A day key or `unlisted-names-day` value that is not a date of a
    /// leap year.
    BadDate(String),
    /// A day given twice.
    DuplicateDay(MonthDay),
    /// A name that is empty between two separators.
    EmptyName,
    /// A name given twice on one day.
    DuplicateName(String),
    /// A day the text does not give.
    MissingDay(MonthDay),
    /// Names on the day the leap-day rule requires to be empty.
    NamesOnEmptySlot(MonthDay),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line > 0 {
            write!(f, "line {}: ", self.line)?;
        }
        match &self.kind {
            ParseErrorKind::MissingSeparator => f.write_str("expected `key = value`"),
            ParseErrorKind::UnknownKey(key) => write!(f, "unknown header `{key}`"),
            ParseErrorKind::DuplicateHeader(key) => write!(f, "header `{key}` given twice"),
            ParseErrorKind::HeaderAfterDays(key) => {
                write!(f, "header `{key}` after the first day")
            }
            ParseErrorKind::MissingHeader(key) => write!(f, "header `{key}` is required"),
            ParseErrorKind::BadValidity(value) => {
                write!(f, "`{value}` is not a validity span such as `2025..2029`")
            }
            ParseErrorKind::BadLeapDayRule(value) => {
                write!(f, "`{value}` is not a leap-day rule")
            }
            ParseErrorKind::BadDate(value) => write!(f, "`{value}` is not a date `MM-DD`"),
            ParseErrorKind::DuplicateDay(day) => write!(f, "day {day} given twice"),
            ParseErrorKind::EmptyName => f.write_str("empty name between separators"),
            ParseErrorKind::DuplicateName(name) => write!(f, "`{name}` given twice on one day"),
            ParseErrorKind::MissingDay(day) => write!(f, "day {day} is not given"),
            ParseErrorKind::NamesOnEmptySlot(day) => {
                write!(f, "names on {day}, which the leap-day rule leaves empty")
            }
        }
    }
}

impl core::error::Error for ParseError {}

const HEADERS: [&str; 10] = [
    "id",
    "country",
    "language",
    "name",
    "authority",
    "validity",
    "leap-day",
    "licence",
    "source",
    "unlisted-names-day",
];

impl OwnedNameDayList {
    /// Read a list from the text format.
    ///
    /// # Errors
    ///
    /// A [`ParseError`] naming the line and what is wrong with it; the
    /// first error found, since a text with one is not a list.
    pub fn parse(text: &str) -> Result<Self, ParseError> {
        let mut headers: [Option<String>; HEADERS.len()] = Default::default();
        let mut days: Vec<Option<Vec<String>>> = (0..DAYS).map(|_| None).collect();
        let mut seen_day = false;

        for (offset, raw) in text.lines().enumerate() {
            let line = offset + 1;
            let at = |kind| ParseError { line, kind };
            let trimmed = raw.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let Some((key, value)) = trimmed.split_once('=') else {
                return Err(at(ParseErrorKind::MissingSeparator));
            };
            let key = key.trim();
            let value = value.trim();

            if let Some(day) = parse_month_day(key) {
                let Some(index) = layout_index(day.month, day.day) else {
                    return Err(at(ParseErrorKind::BadDate(key.to_string())));
                };
                let Some(slot) = days.get_mut(index) else {
                    return Err(at(ParseErrorKind::BadDate(key.to_string())));
                };
                let names = parse_names(value).map_err(at)?;
                if slot.is_some() {
                    return Err(at(ParseErrorKind::DuplicateDay(day)));
                }
                *slot = Some(names);
                seen_day = true;
                continue;
            }

            let Some(position) = HEADERS.iter().position(|header| *header == key) else {
                return Err(at(ParseErrorKind::UnknownKey(key.to_string())));
            };
            if seen_day {
                return Err(at(ParseErrorKind::HeaderAfterDays(key.to_string())));
            }
            let Some(field) = headers.get_mut(position) else {
                return Err(at(ParseErrorKind::UnknownKey(key.to_string())));
            };
            if field.is_some() {
                return Err(at(ParseErrorKind::DuplicateHeader(key.to_string())));
            }
            *field = Some(value.to_string());
        }

        let whole = |kind| ParseError { line: 0, kind };
        let header = |name: &'static str| -> Option<String> {
            HEADERS
                .iter()
                .position(|header| *header == name)
                .and_then(|position| headers.get(position).cloned().flatten())
        };
        let required = |name: &'static str| -> Result<String, ParseError> {
            match header(name) {
                Some(value) if !value.is_empty() => Ok(value),
                _ => Err(whole(ParseErrorKind::MissingHeader(name))),
            }
        };
        let optional = |name: &'static str| header(name).unwrap_or_default();

        let id = required("id")?;
        let source = required("source")?;
        let validity_text = required("validity")?;
        let validity = parse_validity(&validity_text)
            .ok_or_else(|| whole(ParseErrorKind::BadValidity(validity_text.clone())))?;
        let rule_text = required("leap-day")?;
        let leap_day = LeapDayRule::by_id(&rule_text)
            .ok_or_else(|| whole(ParseErrorKind::BadLeapDayRule(rule_text.clone())))?;
        let unlisted_names_day = match header("unlisted-names-day") {
            Some(value) if !value.is_empty() => Some(
                parse_month_day(&value)
                    .filter(|day| layout_index(day.month, day.day).is_some())
                    .ok_or_else(|| whole(ParseErrorKind::BadDate(value.clone())))?,
            ),
            _ => None,
        };

        let mut filled = Vec::with_capacity(DAYS);
        for (index, slot) in days.into_iter().enumerate() {
            let Some(names) = slot else {
                let day = layout_month_day(index).unwrap_or(MonthDay::new(0, 0));
                return Err(whole(ParseErrorKind::MissingDay(day)));
            };
            filled.push(names);
        }
        if let Some(index) = leap_day.empty_slot()
            && filled.get(index).is_some_and(|names| !names.is_empty())
        {
            let day = layout_month_day(index).unwrap_or(MonthDay::new(0, 0));
            return Err(whole(ParseErrorKind::NamesOnEmptySlot(day)));
        }

        Ok(Self {
            id,
            country: optional("country"),
            language: optional("language"),
            english_name: optional("name"),
            authority: optional("authority"),
            validity,
            leap_day,
            licence: optional("licence"),
            source,
            unlisted_names_day,
            days: filled,
        })
    }

    /// The names in a slot of the leap-year layout.
    #[must_use]
    pub fn slot(&self, index: usize) -> &[String] {
        self.days.get(index).map_or(&[], Vec::as_slice)
    }

    /// How many names the list carries, counting a name once per day it
    /// appears on.
    #[must_use]
    pub fn total_names(&self) -> usize {
        self.days.iter().map(Vec::len).sum()
    }
}

impl NameDays for OwnedNameDayList {
    type Name = String;

    fn validity(&self) -> Validity {
        self.validity
    }

    fn leap_day(&self) -> LeapDayRule {
        self.leap_day
    }

    fn slot(&self, index: usize) -> &[String] {
        Self::slot(self, index)
    }
}

impl NameDayList {
    /// The list in the loader's text format.
    ///
    /// The notes are written as comments, since the format has no field
    /// for them; everything else survives a round trip through
    /// [`OwnedNameDayList::parse`], which a test proves for every vendored
    /// list.
    #[must_use]
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "# {}", self.english_name);
        let _ = writeln!(out, "id = {}", self.id);
        let _ = writeln!(out, "country = {}", self.country);
        let _ = writeln!(out, "language = {}", self.language);
        let _ = writeln!(out, "name = {}", self.english_name);
        let _ = writeln!(out, "authority = {}", self.authority);
        let _ = writeln!(out, "validity = {}", self.validity);
        let _ = writeln!(out, "leap-day = {}", self.leap_day.id());
        let _ = writeln!(out, "licence = {}", self.licence);
        let _ = writeln!(out, "source = {}", self.source);
        if let Some(day) = self.unlisted_names_day {
            let _ = writeln!(out, "unlisted-names-day = {day}");
        }
        for note in self.notes {
            match note.name {
                Some(name) => {
                    let _ = writeln!(
                        out,
                        "# {:02}-{:02} {name}: {}",
                        note.month, note.day, note.text
                    );
                }
                None => {
                    let _ = writeln!(out, "# {:02}-{:02}: {}", note.month, note.day, note.text);
                }
            }
        }
        for (index, names) in self.days.iter().enumerate() {
            let Some(day) = layout_month_day(index) else {
                continue;
            };
            let _ = write!(out, "{day} =");
            for (position, name) in names.iter().enumerate() {
                let _ = write!(out, "{}{name}", if position == 0 { " " } else { "; " });
            }
            out.push('\n');
        }
        out
    }
}

/// `MM-DD`, without checking that the day exists.
fn parse_month_day(text: &str) -> Option<MonthDay> {
    let (month, day) = text.split_once('-')?;
    if month.len() != 2 || day.len() != 2 {
        return None;
    }
    Some(MonthDay::new(month.parse().ok()?, day.parse().ok()?))
}

/// `FROM..TO`, either side optional.
fn parse_validity(text: &str) -> Option<Validity> {
    let (from, to) = text.split_once("..")?;
    let year = |part: &str| -> Option<Option<i32>> {
        if part.is_empty() {
            Some(None)
        } else {
            part.parse().ok().map(Some)
        }
    };
    Some(Validity {
        from: year(from.trim())?,
        to: year(to.trim())?,
    })
}

/// Names separated by `;`; an empty value is no names.
fn parse_names(value: &str) -> Result<Vec<String>, ParseErrorKind> {
    if value.is_empty() {
        return Ok(Vec::new());
    }
    let mut names: Vec<String> = Vec::new();
    for piece in value.split(';') {
        let name = piece.trim();
        if name.is_empty() {
            return Err(ParseErrorKind::EmptyName);
        }
        if names.iter().any(|seen| seen == name) {
            return Err(ParseErrorKind::DuplicateName(name.to_string()));
        }
        names.push(name.to_string());
    }
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::latvia;
    use crate::list::{FEBRUARY_24, FEBRUARY_29, names_on};
    use alloc::format;

    /// A synthetic text: every day of a leap year, three of them named.
    fn synthetic(header: &str, fill: impl Fn(u8, u8) -> String) -> String {
        let mut text = String::from(header);
        for month in 1..=12u8 {
            let length = hc_calendar::gregorian::days_in_month(2000, month).unwrap_or(0);
            for day in 1..=length {
                let _ = writeln!(text, "{month:02}-{day:02} = {}", fill(month, day));
            }
        }
        text
    }

    const HEADER: &str =
        "id = zz-test\nvalidity = 2025..2029\nleap-day = no-names\nsource = this test\n";

    fn three_names(month: u8, day: u8) -> String {
        match (month, day) {
            (1, 1) => "Alpha; Beta".to_string(),
            (12, 31) => "Omega".to_string(),
            _ => String::new(),
        }
    }

    fn parsed(text: &str) -> OwnedNameDayList {
        match OwnedNameDayList::parse(text) {
            Ok(list) => list,
            Err(error) => panic!("{error}"),
        }
    }

    fn refused(text: &str) -> ParseError {
        match OwnedNameDayList::parse(text) {
            Ok(list) => panic!("parsed {} instead of failing", list.id),
            Err(error) => error,
        }
    }

    #[test]
    fn a_synthetic_list_reads_back_what_was_written() {
        let list = parsed(&synthetic(HEADER, three_names));
        assert_eq!(list.id, "zz-test");
        assert_eq!(list.validity, Validity::between(2025, 2029));
        assert_eq!(list.leap_day, LeapDayRule::NoNames);
        assert_eq!(list.source, "this test");
        assert!(list.country.is_empty());
        assert_eq!(list.unlisted_names_day, None);
        assert_eq!(list.total_names(), 3);
        assert_eq!(
            names_on(&list, 2026, 1, 1),
            Ok(&["Alpha".to_string(), "Beta".to_string()][..])
        );
        assert_eq!(
            names_on(&list, 2026, 12, 31),
            Ok(&["Omega".to_string()][..])
        );
        assert_eq!(names_on(&list, 2028, 2, 29), Ok(&[][..]));
        assert!(names_on(&list, 2030, 1, 1).is_err(), "outside 2025..2029");
    }

    #[test]
    fn every_vendored_list_survives_a_round_trip_through_the_text_format() {
        for list in latvia::ALL {
            let text = list.to_text();
            let owned = parsed(&text);
            assert_eq!(owned.id, list.id);
            assert_eq!(owned.country, list.country);
            assert_eq!(owned.language, list.language);
            assert_eq!(owned.english_name, list.english_name);
            assert_eq!(owned.authority, list.authority);
            assert_eq!(owned.validity, list.validity);
            assert_eq!(owned.leap_day, list.leap_day);
            assert_eq!(owned.licence, list.licence.to_string());
            assert_eq!(owned.source, list.source);
            assert_eq!(owned.unlisted_names_day, list.unlisted_names_day);
            assert_eq!(owned.total_names(), list.total_names());
            for index in 0..DAYS {
                let vendored: Vec<&str> = list.slot(index).to_vec();
                let loaded: Vec<&str> = owned.slot(index).iter().map(String::as_str).collect();
                assert_eq!(vendored, loaded, "{} slot {index}", list.id);
            }
            // And the evaluator gives the same answer through both.
            let year = list.validity.from.unwrap_or(2026);
            assert_eq!(
                names_on(&owned, year, 5, 22).map(|names| names.len()),
                names_on(list, year, 5, 22).map(|names| names.len())
            );
        }
    }

    #[test]
    fn a_name_may_contain_a_space_and_the_separator_is_a_semicolon() {
        let list = parsed(&synthetic(HEADER, |month, day| {
            if (month, day) == (3, 1) {
                "Anna Maria; Jean-Luc".to_string()
            } else {
                String::new()
            }
        }));
        assert_eq!(
            names_on(&list, 2026, 3, 1),
            Ok(&["Anna Maria".to_string(), "Jean-Luc".to_string()][..])
        );
    }

    #[test]
    fn comments_blank_lines_and_padding_are_ignored() {
        let text = synthetic(
            "# a comment\n\n  id = zz-test  \nvalidity= 2025..2029\nleap-day =no-names\nsource = this test\n\n",
            three_names,
        );
        assert_eq!(parsed(&text).id, "zz-test");
    }

    #[test]
    fn the_validity_span_takes_every_shape_the_type_can_print() {
        for (text, expected) in [
            ("2025..2029", Validity::between(2025, 2029)),
            ("2026..", Validity::since(2026)),
            ("..2025", Validity::until(2025)),
            ("..", Validity::UNKNOWN),
        ] {
            let header = format!("id = zz\nvalidity = {text}\nleap-day = no-names\nsource = s\n");
            assert_eq!(
                parsed(&synthetic(&header, three_names)).validity,
                expected,
                "{text}"
            );
        }
        let header = "id = zz\nvalidity = 2025-2029\nleap-day = no-names\nsource = s\n";
        assert_eq!(
            refused(&synthetic(header, three_names)).kind,
            ParseErrorKind::BadValidity("2025-2029".to_string())
        );
    }

    #[test]
    fn a_missing_required_header_is_refused_with_its_name() {
        for missing in ["id", "validity", "leap-day", "source"] {
            let header: String = HEADER
                .lines()
                .filter(|line| !line.starts_with(missing))
                .map(|line| format!("{line}\n"))
                .collect();
            assert_eq!(
                refused(&synthetic(&header, three_names)).kind,
                ParseErrorKind::MissingHeader(missing),
                "{missing}"
            );
        }
        let header = "id = \nvalidity = ..\nleap-day = no-names\nsource = s\n";
        assert_eq!(
            refused(&synthetic(header, three_names)).kind,
            ParseErrorKind::MissingHeader("id")
        );
    }

    #[test]
    fn a_missing_day_is_refused_because_a_hole_has_to_be_stated() {
        let text: String = synthetic(HEADER, three_names)
            .lines()
            .filter(|line| !line.starts_with("02-29"))
            .map(|line| format!("{line}\n"))
            .collect();
        let error = refused(&text);
        assert_eq!(error.line, 0);
        assert_eq!(error.kind, ParseErrorKind::MissingDay(MonthDay::new(2, 29)));
    }

    #[test]
    fn a_list_that_contradicts_its_leap_day_rule_is_refused() {
        let leap_named = synthetic(HEADER, |month, day| {
            if (month, day) == (2, 29) {
                "Nobody".to_string()
            } else {
                String::new()
            }
        });
        assert_eq!(
            refused(&leap_named).kind,
            ParseErrorKind::NamesOnEmptySlot(MonthDay::new(2, 29))
        );
        let shifted = "id = zz\nvalidity = ..\nleap-day = shift-after-24-february\nsource = s\n";
        let feb_24_named = synthetic(shifted, |month, day| {
            if (month, day) == (2, 24) {
                "Nobody".to_string()
            } else {
                String::new()
            }
        });
        assert_eq!(
            refused(&feb_24_named).kind,
            ParseErrorKind::NamesOnEmptySlot(MonthDay::new(2, 24))
        );
        // The same names one day later are fine, and read as the rule says.
        let feb_25_named = parsed(&synthetic(shifted, |month, day| {
            if (month, day) == (2, 25) {
                "Nobody".to_string()
            } else {
                String::new()
            }
        }));
        assert!(feb_25_named.slot(FEBRUARY_24).is_empty());
        assert_eq!(
            names_on(&feb_25_named, 2023, 2, 24),
            Ok(&["Nobody".to_string()][..])
        );
        assert_eq!(
            names_on(&feb_25_named, 2024, 2, 25),
            Ok(&["Nobody".to_string()][..])
        );
        let own = "id = zz\nvalidity = ..\nleap-day = own-names\nsource = s\n";
        let horymir = parsed(&synthetic(own, |month, day| {
            if (month, day) == (2, 29) {
                "Horymír".to_string()
            } else {
                String::new()
            }
        }));
        assert_eq!(horymir.slot(FEBRUARY_29), ["Horymír".to_string()]);
    }

    #[test]
    fn each_malformed_line_is_refused_with_its_number_and_reason() {
        let cases: [(&str, ParseErrorKind); 7] = [
            ("nonsense\n", ParseErrorKind::MissingSeparator),
            (
                "colour = red\n",
                ParseErrorKind::UnknownKey("colour".to_string()),
            ),
            (
                "country = zz\n",
                ParseErrorKind::HeaderAfterDays("country".to_string()),
            ),
            (
                "02-30 = Nobody\n",
                ParseErrorKind::BadDate("02-30".to_string()),
            ),
            (
                "01-01 = Again\n",
                ParseErrorKind::DuplicateDay(MonthDay::new(1, 1)),
            ),
            ("03-03 = A; ; B\n", ParseErrorKind::EmptyName),
            (
                "03-03 = A; A\n",
                ParseErrorKind::DuplicateName("A".to_string()),
            ),
        ];
        for (line, kind) in cases {
            let mut text = synthetic(HEADER, three_names);
            let expected_line = text.lines().count() + 1;
            text.push_str(line);
            let error = refused(&text);
            assert_eq!(error.kind, kind, "{line}");
            assert_eq!(error.line, expected_line, "{line}");
        }
    }

    #[test]
    fn a_header_given_twice_or_naming_no_rule_is_refused() {
        let twice = format!("id = first\n{HEADER}");
        let error = refused(&synthetic(&twice, three_names));
        assert_eq!(
            error.kind,
            ParseErrorKind::DuplicateHeader("id".to_string())
        );
        assert_eq!(error.line, 2);
        let header = "id = zz\nvalidity = ..\nleap-day = sideways\nsource = s\n";
        assert_eq!(
            refused(&synthetic(header, three_names)).kind,
            ParseErrorKind::BadLeapDayRule("sideways".to_string())
        );
        let header =
            "id = zz\nvalidity = ..\nleap-day = no-names\nsource = s\nunlisted-names-day = 13-01\n";
        assert_eq!(
            refused(&synthetic(header, three_names)).kind,
            ParseErrorKind::BadDate("13-01".to_string())
        );
    }

    #[test]
    fn the_errors_print_with_their_line() {
        let error = ParseError {
            line: 7,
            kind: ParseErrorKind::DuplicateDay(MonthDay::new(1, 1)),
        };
        assert_eq!(error.to_string(), "line 7: day 01-01 given twice");
        let whole = ParseError {
            line: 0,
            kind: ParseErrorKind::MissingHeader("id"),
        };
        assert_eq!(whole.to_string(), "header `id` is required");
    }
}
