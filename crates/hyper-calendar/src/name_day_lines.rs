//! The tab-separated lines the WebAssembly module and the C library write
//! about name days, written once.
//!
//! Everything here reads [`hc_name_days`], which **reports what lists say
//! and asserts none of them**: a name day is a fact about somebody's list,
//! and the list is always named, with its authority, the years its edition
//! was in force and the terms it may be copied under. A line never says a
//! name is a country's. Where the crate carries no list for a country, a
//! `gap` line says why, in the crate's words: a list a university sells by the
//! copy, a church calendar that names saints and not given names, several
//! published lists that no body chooses between.
//!
//! An edition does not answer for a year it does not cover. A list shipped
//! for a country whose years do not include the year asked is an `outside`
//! line with no names, and never the nearest edition's; where the years
//! before a country's first edition are a gap of their own (Latvia's, before
//! 2023), a `gap` line says so.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use hc_calendar::Rd;
use hc_calendar::gregorian;
use hc_name_days::gaps::{self, Gap, SURVEYED};
use hc_name_days::{ALL, MonthDay, NameDayError, NameDayList, days_of, names_on};

use crate::boundary::{Answer, Line, Refusal};

/// How many columns a line of [`lists_lines`] has.
pub const LIST_COLUMNS: usize = 17;

/// How many columns a line of [`names_on_lines`] has.
pub const NAMES_ON_COLUMNS: usize = 14;

/// How many columns a line of [`days_of_lines`] has.
pub const DAYS_OF_COLUMNS: usize = 13;

/// A two-letter code as the lists and gaps write it: lower-case.
fn lower(code: &str) -> String {
    code.trim().to_ascii_lowercase()
}

/// Whether a gap's identifier, a code or two joined by a hyphen, names a
/// country.
fn gap_names(gap: &Gap, country: &str) -> bool {
    gap.id.split('-').any(|code| code == country)
}

/// The shipped lists of a country, in the table's order.
fn lists_of(country: &str) -> Vec<&'static NameDayList> {
    ALL.iter().filter(|list| list.country == country).collect()
}

/// The gaps that name a country.
fn gaps_of(country: &str) -> Vec<&'static Gap> {
    gaps::ALL
        .iter()
        .filter(|gap| gap_names(gap, country))
        .collect()
}

/// A validity bound as a year, or empty where it is open.
fn year_cell(line: &mut Line<'_>, year: Option<i32>) {
    line.value_or_empty(year);
}

/// Every name-day list the crate ships and every country it declines to
/// ship one for, one line each: the kind (`list` or `gap`), the identifier,
/// the country (the ISO 3166-1 alpha-2 code of a list, the English name of
/// a gap), the BCP 47 language of the names, the English name of the list or
/// what the gap leaves out, the authority whose list it is, the date it was
/// decided to whatever precision the source gives, how it came to exist
/// (`promulgated`, `recorded`, `vernacular` or `contested`), the first and
/// last year the edition was in force (empty where it is open; for a gap of
/// the years before an edition, the last year it covers), the terms it
/// is held under, what it does with 29 February (`no-names`, `own-names`,
/// `shift-after-24-february` or `leap-years-only`), how many names it holds,
/// the citation, the date the file was retrieved or, for a gap, the date the
/// survey was made, the reason for a gap (`licensed-for-a-fee`,
/// `licence-unknown`, `rights-reserved`, `no-keeper-found`, `not-yet-read` or
/// `saints-not-names`), and the reasoning in full. A
/// column that does not apply to the kind is empty.
///
/// A gap's identifier is the country's code, or two codes joined by a hyphen
/// where one reasoning covers both; [`names_on_lines`] and [`days_of_lines`]
/// read a code of either.
#[must_use]
pub fn lists_lines() -> String {
    let mut out = String::new();
    for list in &ALL {
        let mut line = Line::new(&mut out);
        line.cell("list")
            .cell(list.id)
            .cell(list.country)
            .cell(list.language)
            .cell(list.english_name)
            .cell(list.authority)
            .value_or_empty(list.decided)
            .cell(list.provenance.id());
        year_cell(&mut line, list.validity.from);
        year_cell(&mut line, list.validity.to);
        line.value(list.licence)
            .cell(list.leap_day.id())
            .value(list.total_names())
            .cell(list.source)
            .value(list.retrieved)
            .empties(2);
        line.end();
    }
    for gap in gaps::ALL {
        let mut line = Line::new(&mut out);
        line.cell("gap")
            .cell(gap.id)
            .cell(gap.country)
            .empty()
            .cell(gap.subject)
            .empties(4)
            .value_or_empty(gap.before.map(|first| first - 1))
            .empties(3)
            .cell(gap.sources)
            .cell(SURVEYED)
            .cell(gap.reason.id())
            .cell(gap.explanation);
        line.end();
    }
    out
}

/// The notes a list prints beside a day, in the source's words, separated by
/// `;`.
fn notes_on(list: &NameDayList, month: u8, day: u8) -> String {
    let mut out = String::new();
    for note in list
        .notes
        .iter()
        .filter(|note| note.month == month && note.day == day)
    {
        if !out.is_empty() {
            out.push(';');
        }
        if let Some(name) = note.name {
            out.push_str(name);
            out.push_str(": ");
        }
        out.push_str(note.text);
    }
    out
}

/// A gap as a line of [`names_on_lines`]: the same columns, those about
/// names empty.
fn gap_on_line(out: &mut String, gap: &Gap) {
    let mut line = Line::new(out);
    line.cell("gap")
        .cell(gap.id)
        .cell(gap.subject)
        .empties(2)
        .value_or_empty(gap.before.map(|first| first - 1))
        .empties(5)
        .cell(gap.sources)
        .cell(gap.reason.id())
        .cell(gap.explanation);
    line.end();
}

/// What the lists of a country name on a day, one line each: the kind, the
/// identifier, the English name and the authority as [`lists_lines`] writes
/// them, the first and last year of the edition (empty where open), the
/// terms, the names the list gives that day separated by `;` in the list's own
/// spelling and script (empty where there are none), how many, `1` where the
/// authority reserves the day for names not on its list (Latvia's 22 May),
/// what the source prints beside the day that is not a name, the citation, and,
/// for a gap, the reason and the reasoning.
///
/// The kind is `list` for an edition in force in the year the day falls in
/// (a country may keep two at once: Latvia's traditional and extended lists),
/// `outside` for a shipped edition whose years do not include it, with no
/// names, and `gap` for a reason the crate carries no list for the country
/// (or, for the years before a country's first edition, for the years it
/// does not cover).
/// A day that no list has a name for is a `list` line with no names, and never
/// the nearest edition's.
///
/// `country` is a two-letter code in any case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a country the crate has neither a list nor a gap
/// for, [`Refusal::OutOfRange`] for a day beyond the years ±2 147 483 647
/// that the lists count in or the Gregorian years ±9 999 999.
pub fn names_on_lines(country: &str, fixed: i64) -> Answer<String> {
    let country = lower(country);
    let (lists, gaps) = (lists_of(&country), gaps_of(&country));
    if lists.is_empty() && gaps.is_empty() {
        return Err(Refusal::Unknown);
    }
    hc_calendars_solar::gregorian::year_from_fixed(Rd(fixed)).map_err(|_| Refusal::OutOfRange)?;
    let (year, month, day) = gregorian::ymd(Rd(fixed));
    let year = i32::try_from(year).map_err(|_| Refusal::OutOfRange)?;
    let mut out = String::new();
    for list in lists {
        let mut line = Line::new(&mut out);
        line.cell(if list.validity.contains(year) {
            "list"
        } else {
            "outside"
        })
        .cell(list.id)
        .cell(list.english_name)
        .cell(list.authority);
        year_cell(&mut line, list.validity.from);
        year_cell(&mut line, list.validity.to);
        line.value(list.licence);
        match names_on(list, year, month, day) {
            Ok(names) => {
                line.cell(&names.join(";"))
                    .value(names.len())
                    .flag(list.unlisted_names_day == Some(MonthDay::new(month, day)))
                    .cell(&notes_on(list, month, day));
            }
            Err(NameDayError::OutsideValidity { .. }) => {
                line.empties(4);
            }
            Err(NameDayError::InvalidDate { .. }) => return Err(Refusal::InvalidDate),
            Err(_) => return Err(Refusal::OutOfRange),
        }
        line.cell(list.source).empties(2);
        line.end();
    }
    for gap in gaps.into_iter().filter(|gap| gap.covers(year)) {
        gap_on_line(&mut out, gap);
    }
    Ok(out)
}

/// The days of a year on which the lists of a country give a name, one line
/// each: the kind, the identifier, the English name and the authority as
/// [`lists_lines`] writes them, the first and last year of the edition, the
/// terms, the days as `MM-DD` separated by `;`, the same days as fixed days
/// of that year separated by `;`, how many, the citation, and, for a gap, the
/// reason and the reasoning.
///
/// The match is exact and case-sensitive: the list's own spelling, diacritics
/// included, and no diminutive the authority did not print. A name may fall on
/// several days (the extended Latvian list has some) or on none, which is a
/// `list` line with no days. The kinds are those of [`names_on_lines`]:
/// `outside` for an edition not in force in the year, with no days, and `gap`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a country the crate has neither a list nor a gap
/// for, and [`Refusal::OutOfRange`] for a year beyond the Gregorian years
/// ±9 999 999 or past an `i32`.
pub fn days_of_lines(country: &str, name: &str, year: i64) -> Answer<String> {
    let country = lower(country);
    let (lists, gaps) = (lists_of(&country), gaps_of(&country));
    if lists.is_empty() && gaps.is_empty() {
        return Err(Refusal::Unknown);
    }
    let small = i32::try_from(year).map_err(|_| Refusal::OutOfRange)?;
    gregorian::to_fixed(year, 1, 1).map_err(|_| Refusal::OutOfRange)?;
    let mut out = String::new();
    for list in lists {
        let mut line = Line::new(&mut out);
        line.cell(if list.validity.contains(small) {
            "list"
        } else {
            "outside"
        })
        .cell(list.id)
        .cell(list.english_name)
        .cell(list.authority);
        year_cell(&mut line, list.validity.from);
        year_cell(&mut line, list.validity.to);
        line.value(list.licence);
        match days_of(list, name, small) {
            Ok(days) => {
                let days: Vec<MonthDay> = days.collect();
                let text: Vec<String> = days.iter().map(MonthDay::to_string).collect();
                let mut fixed = Vec::new();
                for day in &days {
                    fixed.push(
                        gregorian::to_fixed(year, day.month, day.day)
                            .map_err(|_| Refusal::OutOfRange)?
                            .0
                            .to_string(),
                    );
                }
                line.cell(&text.join(";"))
                    .cell(&fixed.join(";"))
                    .value(days.len());
            }
            Err(NameDayError::OutsideValidity { .. }) => {
                line.empties(3);
            }
            Err(_) => return Err(Refusal::OutOfRange),
        }
        line.cell(list.source).empties(2);
        line.end();
    }
    for gap in gaps.into_iter().filter(|gap| gap.covers(small)) {
        let mut line = Line::new(&mut out);
        line.cell("gap")
            .cell(gap.id)
            .cell(gap.subject)
            .empties(2)
            .value_or_empty(gap.before.map(|first| first - 1))
            .empties(4)
            .cell(gap.sources)
            .cell(gap.reason.id())
            .cell(gap.explanation);
        line.end();
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::cells;

    fn rows(text: &str) -> Vec<Vec<&str>> {
        text.lines()
            .map(|line| line.split('\t').collect())
            .collect()
    }

    fn fixed(year: i64, month: u8, day: u8) -> i64 {
        gregorian::to_fixed(year, month, day)
            .map(|rd| rd.0)
            .unwrap_or_default()
    }

    #[test]
    fn every_list_and_every_gap_is_a_line() {
        let text = lists_lines();
        let table = rows(&text);
        assert_eq!(table.len(), ALL.len() + gaps::ALL.len());
        assert!(table.iter().all(|row| row.len() == LIST_COLUMNS));
        let list = table
            .iter()
            .find(|row| row[1] == "lv-traditional-2026")
            .cloned()
            .unwrap_or_default();
        assert_eq!(&list[..4], ["list", "lv-traditional-2026", "lv", "lv"]);
        assert_eq!(list[7], "promulgated");
        assert_eq!(&list[8..10], ["2026", ""]);
        assert!(list[10].contains("CC0-1.0"), "{}", list[10]);
        assert_eq!(list[11], "no-names");
        assert_eq!(list[6], "2025-04-30");
        // The crate's own count of the 2026 traditional list.
        assert_eq!(
            list[12],
            ALL.iter()
                .find(|list| list.id == "lv-traditional-2026")
                .map(|list| list.total_names().to_string())
                .unwrap_or_default()
        );
        // Finland is a gap, for a licence a university charges for.
        let finland = table
            .iter()
            .find(|row| row[1] == "fi")
            .cloned()
            .unwrap_or_default();
        assert_eq!(&finland[..3], ["gap", "fi", "Finland"]);
        assert_eq!(finland[15], "licensed-for-a-fee");
        assert!(
            finland[16].contains("University of Helsinki"),
            "{}",
            finland[16]
        );
        assert!(finland[5..13].iter().all(|cell| cell.is_empty()));
        assert!(table.iter().any(|row| row[15] == "saints-not-names"));
    }

    #[test]
    fn the_first_of_january_in_latvia_is_laimnesis_solvita_and_solvija() {
        // The crate's own doctest: 1 January is Laimnesis, Solvita, Solvija
        // on the 2026 traditional list.
        let text = names_on_lines("LV", fixed(2026, 1, 1)).unwrap_or_default();
        let table = rows(&text);
        assert_eq!(table.len(), 4);
        assert!(table.iter().all(|row| row.len() == NAMES_ON_COLUMNS));
        let traditional = table
            .iter()
            .find(|row| row[1] == "lv-traditional-2026")
            .cloned()
            .unwrap_or_default();
        assert_eq!(traditional[0], "list");
        assert_eq!(traditional[7], "Laimnesis;Solvita;Solvija");
        assert_eq!(traditional[8], "3");
        assert_eq!(traditional[9], "0");
        // The 2023 editions are not in force in 2026 and say so.
        let old = table
            .iter()
            .find(|row| row[1] == "lv-traditional-2023")
            .cloned()
            .unwrap_or_default();
        assert_eq!(old[0], "outside");
        assert_eq!(&old[4..6], ["2023", "2025"]);
        assert!(old[7..11].iter().all(|cell| cell.is_empty()));
        // 22 May is Emilija and the day for names not on the list.
        let text = names_on_lines("lv", fixed(2026, 5, 22)).unwrap_or_default();
        let table = rows(&text);
        let day = table
            .iter()
            .find(|row| row[1] == "lv-traditional-2026")
            .cloned()
            .unwrap_or_default();
        assert_eq!(day[9], "1");
        assert!(day[7].contains("Emīlija"), "{}", day[7]);
        assert!(!day[10].is_empty());
        // 29 February is a day with no names, not a refusal.
        let text = names_on_lines("lv", fixed(2028, 2, 29)).unwrap_or_default();
        let leap = rows(&text);
        let day = leap
            .iter()
            .find(|row| row[1] == "lv-extended-2026")
            .cloned()
            .unwrap_or_default();
        assert_eq!(&day[7..9], ["", "0"]);
    }

    #[test]
    fn a_country_without_a_list_says_why_and_an_unknown_one_is_refused() {
        let text = names_on_lines("FI", fixed(2026, 1, 1)).unwrap_or_default();
        let table = rows(&text);
        assert_eq!(table.len(), 1);
        assert_eq!(
            &table[0][..3],
            [
                "gap",
                "fi",
                "the Finnish, Finland-Swedish, Orthodox and Sámi name-day lists"
            ]
        );
        assert_eq!(table[0][12], "licensed-for-a-fee");
        assert_eq!(table[0].len(), NAMES_ON_COLUMNS);
        // Two codes share one reasoning.
        let both = names_on_lines("at", fixed(2026, 1, 1)).unwrap_or_default();
        assert_eq!(rows(&both)[0][1], "de-at");
        assert_eq!(
            names_on_lines("jp", fixed(2026, 1, 1)),
            Err(Refusal::Unknown)
        );
        assert_eq!(names_on_lines("", fixed(2026, 1, 1)), Err(Refusal::Unknown));
        assert_eq!(names_on_lines("lv", i64::MAX), Err(Refusal::OutOfRange));
    }

    #[test]
    fn a_name_finds_its_days_in_the_edition_of_the_year() {
        // Jānis is the 24th of June, the crate's own doctest.
        let text = days_of_lines("lv", "Jānis", 2026).unwrap_or_default();
        let table = rows(&text);
        assert!(table.iter().all(|row| row.len() == DAYS_OF_COLUMNS));
        let traditional = table
            .iter()
            .find(|row| row[1] == "lv-traditional-2026")
            .cloned()
            .unwrap_or_default();
        assert_eq!(traditional[7], "06-24");
        assert_eq!(traditional[8], fixed(2026, 6, 24).to_string());
        assert_eq!(traditional[9], "1");
        // Grēta was added by the decision of 30 April 2025, in force from
        // 2026: it is on 23 January of the 2026 list and not of the 2023 one.
        let text = days_of_lines("LV", "Grēta", 2026).unwrap_or_default();
        let table = rows(&text);
        let new = table
            .iter()
            .find(|row| row[1] == "lv-traditional-2026")
            .cloned()
            .unwrap_or_default();
        assert_eq!(new[7], "01-23");
        let old = table
            .iter()
            .find(|row| row[1] == "lv-traditional-2023")
            .cloned()
            .unwrap_or_default();
        assert_eq!(old[0], "outside");
        let text = days_of_lines("LV", "Grēta", 2025).unwrap_or_default();
        let table = rows(&text);
        let old = table
            .iter()
            .find(|row| row[1] == "lv-traditional-2023")
            .cloned()
            .unwrap_or_default();
        assert_eq!(old[0], "list");
        assert_eq!(&old[7..10], ["", "", "0"]);
        // No list in force before the oldest edition: every shipped edition
        // is `outside`, and the years before it are a `gap` that says so, with
        // the last year it covers in the `to` column.
        let text = days_of_lines("lv", "Jānis", 2022).unwrap_or_default();
        let table = rows(&text);
        assert!(
            table
                .iter()
                .all(|row| row[0] == "outside" || row[0] == "gap")
        );
        let gap = table
            .iter()
            .find(|row| row[0] == "gap")
            .cloned()
            .unwrap_or_default();
        assert_eq!((gap[1], gap[5], gap[11]), ("lv", "2022", "not-yet-read"));
        // From 2023 the gap no longer answers.
        let text = days_of_lines("lv", "Jānis", 2023).unwrap_or_default();
        assert!(rows(&text).iter().all(|row| row[0] != "gap"));
        let text = names_on_lines("lv", fixed(2022, 6, 24)).unwrap_or_default();
        assert!(rows(&text).iter().any(|row| row[0] == "gap"));
        let text = names_on_lines("lv", fixed(2026, 6, 24)).unwrap_or_default();
        assert!(rows(&text).iter().all(|row| row[0] != "gap"));
        // The match is exact and case-sensitive: the list's own spelling.
        let text = days_of_lines("lv", "jānis", 2026).unwrap_or_default();
        assert!(
            rows(&text)
                .iter()
                .all(|row| row[9].is_empty() || row[9] == "0")
        );
        assert_eq!(days_of_lines("jp", "Jānis", 2026), Err(Refusal::Unknown));
        assert_eq!(
            days_of_lines("lv", "Jānis", i64::MAX),
            Err(Refusal::OutOfRange)
        );
        let gap = days_of_lines("ee", "Ulmi", 2026).unwrap_or_default();
        assert_eq!(cells(&gap)[0], "gap");
    }
}
