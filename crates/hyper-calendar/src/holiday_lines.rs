//! The tab-separated lines the WebAssembly module and the C library write
//! about the holiday tables and the liturgical year, written once.
//!
//! * The tables themselves, in the order `hc_holiday_codes` lists them,
//!   with the kind each is, its names, its sources and the country an
//!   exchange keeps the holidays of, each read from the table's own data.
//! * The lectionary cycles of a day and the astronomical Easter of a year,
//!   from [`hc_holiday::lectionary`] and [`hc_holiday::computus`].

use alloc::string::String;
use core::fmt::Write;

use hc_calendar::Rd;
use hc_holiday::rule::RuleSet;
use hc_holiday::{computus, countries, exchanges, international, lectionary, traditions};

use crate::boundary::{Answer, Refusal, push_cell};

/// How many columns [`holiday_tables`] writes.
pub const HOLIDAY_TABLES_COLUMNS: usize = 7;

/// How many columns [`lectionary_line`] writes.
pub const LECTIONARY_COLUMNS: usize = 4;

/// Every table, in the order `hc_holiday_codes` lists them: the countries,
/// then the exchanges, the traditions and the international sets.
pub fn tables() -> impl Iterator<Item = &'static RuleSet> {
    countries::ALL
        .iter()
        .chain(exchanges::ALL)
        .chain(traditions::ALL)
        .chain(international::ALL)
        .copied()
}

/// What a table is, as the line writes it: read from the list the table
/// is in, and for a country's list from the shape of its code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableKind {
    /// A country's public holidays, keyed by ISO 3166-1 alpha-2.
    Country,
    /// A subdivision's, keyed by ISO 3166-2, `JP-13`: a table in the
    /// countries' list whose code carries a hyphen. None does yet; every
    /// subdivision's days are rules of its country's table, by region.
    Subdivision,
    /// An exchange's trading calendar, keyed by ISO 10383 MIC.
    Exchange,
    /// A religious or cultural tradition's calendar.
    Tradition,
    /// An international set of observances, `un-days`.
    Observance,
}

impl TableKind {
    /// The word a line carries.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Country => "country",
            Self::Subdivision => "subdivision",
            Self::Exchange => "exchange",
            Self::Tradition => "tradition",
            Self::Observance => "observance",
        }
    }
}

/// Every table with its kind, in [`tables`] order.
pub fn tables_with_kinds() -> impl Iterator<Item = (&'static RuleSet, TableKind)> {
    let national = countries::ALL.iter().map(|set| {
        let kind = if set.code.contains('-') {
            TableKind::Subdivision
        } else {
            TableKind::Country
        };
        (*set, kind)
    });
    national
        .chain(exchanges::ALL.iter().map(|set| (*set, TableKind::Exchange)))
        .chain(
            traditions::ALL
                .iter()
                .map(|set| (*set, TableKind::Tradition)),
        )
        .chain(
            international::ALL
                .iter()
                .map(|set| (*set, TableKind::Observance)),
        )
}

/// The ISO 3166-1 country a subdivision or an exchange belongs to, as its
/// table records it: a subdivision's code before the hyphen, and for an
/// exchange the country whose table it includes for its days off. An
/// exchange that lists every closed day itself names no country in its
/// data, and has none here.
#[must_use]
pub fn country_of(set: &RuleSet, kind: TableKind) -> Option<&'static str> {
    match kind {
        TableKind::Subdivision => set.code.split('-').next(),
        TableKind::Exchange => set
            .includes
            .iter()
            .map(|include| include.set.code)
            .find(|code| countries::by_code(code).is_some()),
        _ => None,
    }
}

/// Whether a BCP 47 tag asks for English: its language subtag is `en`.
fn asks_for_english(locale: &str) -> bool {
    locale
        .split(['-', '_'])
        .next()
        .is_some_and(|language| language.eq_ignore_ascii_case("en"))
}

/// The lines of `hc_holiday_tables`, one per table in [`tables`] order:
/// the code, the kind, the name in the locale, the English name, the
/// locale that answered, the sources, and the country of a subdivision or
/// an exchange.
///
/// A table carries its English name and no other: `hc-i18n` carries no
/// CLDR territory names and the tables no names in other languages. So a
/// tag whose language is `en` has the English name in column 3 and `en` in
/// column 5, and every other tag, `native` included, has both empty, for a
/// page to fall back to column 4 itself, as it does for `hc_calendars`.
#[must_use]
pub fn holiday_tables(locale: &str) -> String {
    let english = asks_for_english(locale);
    let mut out = String::new();
    for (set, kind) in tables_with_kinds() {
        push_cell(&mut out, set.code);
        let _ = write!(out, "\t{}\t", kind.name());
        if english {
            push_cell(&mut out, set.english_name);
        }
        out.push('\t');
        push_cell(&mut out, set.english_name);
        out.push('\t');
        if english {
            out.push_str("en");
        }
        out.push('\t');
        push_cell(&mut out, set.sources);
        out.push('\t');
        if let Some(country) = country_of(set, kind) {
            push_cell(&mut out, country);
        }
        out.push('\n');
    }
    out
}

/// The line of `hc_lectionary`: the liturgical year a day falls in, named
/// by the civil year of its Easter; the Sunday cycle, `A`, `B` or `C`; the
/// Roman weekday cycle, `I` or `II`; and the Revised Common Lectionary's
/// Proper, 3 to 29, for a Sunday after Trinity Sunday, else empty.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside the liturgical years 1583 to 4099,
/// whose Easter the Gregorian computus gives.
pub fn lectionary_line(fixed: i64) -> Answer<String> {
    let day = Rd(fixed);
    let year = lectionary::liturgical_year(day).ok_or(Refusal::OutOfRange)?;
    let sunday = lectionary::sunday_cycle(day).ok_or(Refusal::OutOfRange)?;
    let weekday = lectionary::roman_weekday_cycle(day).ok_or(Refusal::OutOfRange)?;
    let mut out = alloc::format!("{year}\t{}\t{}\t", sunday.letter(), weekday.numeral());
    if let Some(proper) = lectionary::rcl_proper(day) {
        let _ = write!(out, "{proper}");
    }
    out.push('\n');
    Ok(out)
}

/// Easter Sunday of a Gregorian year by the astronomical reckoning at the
/// meridian of Jerusalem, as a fixed day.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside
/// [`computus::ASTRONOMICAL_EASTER_FIRST_YEAR`] to
/// [`computus::ASTRONOMICAL_EASTER_LAST_YEAR`].
pub fn astronomical_easter(year: i64) -> Answer<i64> {
    computus::astronomical_easter(year)
        .map(|day| day.0)
        .ok_or(Refusal::OutOfRange)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeSet;
    use alloc::vec::Vec;

    fn day(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    #[test]
    fn every_table_has_an_english_name_unique_within_its_kind() {
        let mut seen = BTreeSet::new();
        let mut count = 0;
        for (set, kind) in tables_with_kinds() {
            count += 1;
            assert!(!set.english_name.trim().is_empty(), "{}", set.code);
            assert!(
                seen.insert((kind.name(), set.english_name)),
                "two {} tables are called {}",
                kind.name(),
                set.english_name
            );
        }
        assert_eq!(count, tables().count());
    }

    #[test]
    fn the_tables_are_listed_in_code_order_with_their_kinds() {
        let english = holiday_tables("en-GB");
        let rows: Vec<Vec<&str>> = english
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert!(rows.iter().all(|row| row.len() == HOLIDAY_TABLES_COLUMNS));
        let codes: Vec<&str> = tables().map(|set| set.code).collect();
        assert_eq!(rows.iter().map(|row| row[0]).collect::<Vec<_>>(), codes);
        let row = |code: &str| rows.iter().find(|row| row[0] == code).expect("a table");
        assert_eq!(
            row("JP")[1..6],
            ["country", "Japan", "Japan", "en", row("JP")[5]]
        );
        assert_eq!(row("XJPX")[1], "exchange");
        assert_eq!(row("XJPX")[6], "JP");
        assert_eq!(row("XLON")[6], "GB");
        assert_eq!(row("XNYS")[6], "");
        assert_eq!(row("christian-western")[1], "tradition");
        assert_eq!(row("un-days")[1], "observance");
        assert!(rows.iter().all(|row| row[1] != "subdivision"));
        let japanese = holiday_tables("ja");
        let first: Vec<&str> = japanese
            .lines()
            .next()
            .expect("a line")
            .split('\t')
            .collect();
        assert_eq!(first[2], "");
        assert_eq!(first[4], "");
        assert!(!first[3].is_empty());
    }

    /// The system page's worked year: Advent 2025 begins 2026, Year A and
    /// Year II (`docs/systems/lectionary-cycles.md`), and Christ the King,
    /// 22 November 2026, is Proper 29.
    #[test]
    fn the_liturgical_year_2026_is_year_a_and_year_ii() {
        assert_eq!(
            lectionary_line(day(2025, 11, 30)).as_deref(),
            Ok("2026\tA\tII\t\n")
        );
        assert_eq!(
            lectionary_line(day(2025, 11, 29)).as_deref(),
            Ok("2025\tC\tI\t\n")
        );
        assert_eq!(
            lectionary_line(day(2026, 11, 22)).as_deref(),
            Ok("2026\tA\tII\t29\n")
        );
        assert_eq!(lectionary_line(day(1500, 1, 1)), Err(Refusal::OutOfRange));
    }

    /// The World Council of Churches' Aleppo table, as `computus`'s tests
    /// read it: the full moon of Sunday 8 April 2001 puts Easter on
    /// 15 April.
    #[test]
    fn the_astronomical_easter_of_2001_is_the_fifteenth_of_april() {
        assert_eq!(astronomical_easter(2001), Ok(day(2001, 4, 15)));
        assert_eq!(astronomical_easter(1582), Err(Refusal::OutOfRange));
    }
}
