//! The tab-separated lines the WebAssembly module and the C library write
//! about fiscal, tax and academic years, written once.
//!
//! Everything here reads [`hc_fiscal`], whose subject is not a new calendar
//! but the observation that institutions agree what day it is and disagree
//! what *year* it is. Two traps are carried to the boundary as columns and
//! never defaulted. **The label**: Japan's 2024年度 begins in 2024, the
//! United States' FY 2024 began on 1 October 2023, and both are written
//! "FY2024", so every line that has a year label also says which convention
//! names it, by `start-year` or `end-year`. **The calendar**: a year that
//! begins at Nowruz or on 1 Shrawan begins in a named calendar, so the
//! start's calendar is a column too.
//!
//! A year is never answered for a label outside the span a system was in
//! force: such a system's line says `outside-validity` and carries no
//! label, and a day the start's calendar does not reach says
//! `outside-calendar-range`. A year the system was in force in but the
//! sources read do not reach is a `gap`: the system's line says so, and the
//! first year the sources do reach is the `read_from` cell of
//! [`profiles_lines`] (ADR 0013). The lines say what the tables say, with the
//! authority, the check date and the source; they assert none of them.

use alloc::string::String;

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_fiscal::academic::{self, AcademicProfile};
use hc_fiscal::countries::{self, FiscalProfile};
use hc_fiscal::retail::{self, WeekYearSystem};
use hc_fiscal::{FiscalError, SourceDate, SystemKind, YearSystem};

use crate::boundary::{Answer, Line, Refusal};

/// How many columns a line of [`profiles_lines`] has.
pub const PROFILE_COLUMNS: usize = 20;

/// How many columns a line of [`year_on_lines`] has.
pub const YEAR_ON_COLUMNS: usize = 18;

/// How many columns a line of [`year_span_lines`] has.
pub const YEAR_SPAN_COLUMNS: usize = 9;

/// How many columns a line of [`week_year_systems_lines`] has.
pub const WEEK_YEAR_SYSTEM_COLUMNS: usize = 10;

/// How many columns a line of [`week_year_on_line`] has.
pub const WEEK_YEAR_ON_COLUMNS: usize = 14;

/// Which table a year system is in.
#[derive(Clone, Copy)]
enum Table {
    /// A country's fiscal profile: government, tax and corporate years.
    Fiscal,
    /// An academic profile's school year.
    School,
    /// An academic profile's university year, where it differs.
    University,
}

impl Table {
    const fn id(self) -> &'static str {
        match self {
            Self::Fiscal => "fiscal",
            Self::School => "school",
            Self::University => "university",
        }
    }
}

/// One year system with where it comes from.
struct Entry {
    code: &'static str,
    country: &'static str,
    table: Table,
    system: &'static YearSystem,
    checked: SourceDate,
    sources: &'static str,
}

/// Every year system the crate carries, country by country: the fiscal
/// profiles' first, then the academic profiles', leaving out a system the
/// fiscal profile of the same country already carries under the same name
/// and kind (Japan's school year is in both).
fn entries() -> alloc::vec::Vec<Entry> {
    let mut out = alloc::vec::Vec::new();
    for profile in countries::ALL {
        let profile: &'static FiscalProfile = profile;
        for system in profile.systems {
            out.push(Entry {
                code: profile.code,
                country: profile.english_name,
                table: Table::Fiscal,
                system,
                checked: profile.sources_checked,
                sources: profile.sources,
            });
        }
    }
    for profile in academic::ALL {
        let profile: &'static AcademicProfile = profile;
        for (table, system) in [
            (Table::School, Some(&profile.school)),
            (Table::University, profile.university.as_ref()),
        ] {
            let Some(system) = system else { continue };
            let duplicate = out.iter().any(|entry| {
                entry.code.eq_ignore_ascii_case(profile.code)
                    && entry.system.name == system.name
                    && entry.system.kind == system.kind
            });
            if !duplicate {
                out.push(Entry {
                    code: profile.code,
                    country: profile.english_name,
                    table,
                    system,
                    checked: profile.sources_checked,
                    sources: profile.sources,
                });
            }
        }
    }
    out
}

/// The spans of labels no source reaches, as `first-last;first-last`.
fn unread_text(system: &YearSystem) -> String {
    let mut out = String::new();
    for &(first, last) in system.unread {
        if !out.is_empty() {
            out.push(';');
        }
        out.push_str(&alloc::format!("{first}-{last}"));
    }
    out
}

/// A check date as `YYYY-MM-DD`.
fn date_text(date: SourceDate) -> String {
    alloc::format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
}

/// The refusal an error of the library is, for a call that cannot go on:
/// every one of them is a day or a label outside what the crate can place.
fn refusal(error: FiscalError) -> Refusal {
    match error {
        FiscalError::Overflow => Refusal::Overflow,
        _ => Refusal::OutOfRange,
    }
}

/// What a lookup of a country and a kind finds.
fn selected(country: &str, kind: &str) -> Answer<alloc::vec::Vec<Entry>> {
    let kind = kind.trim();
    let wanted = if kind.is_empty() {
        None
    } else {
        Some(SystemKind::by_id(kind).ok_or(Refusal::Unknown)?)
    };
    let all = entries();
    if !all
        .iter()
        .any(|entry| hc_core::catalogue::matches(country, entry.code))
    {
        return Err(Refusal::Unknown);
    }
    let chosen: alloc::vec::Vec<Entry> = all
        .into_iter()
        .filter(|entry| hc_core::catalogue::matches(country, entry.code))
        .filter(|entry| wanted.is_none_or(|kind| entry.system.kind == kind))
        .collect();
    if chosen.is_empty() {
        return Err(Refusal::NoData);
    }
    Ok(chosen)
}

/// Every year system the crate carries, one line each, country by
/// country: the ISO 3166-1 alpha-2 code, the country in English, the table
/// (`fiscal` for a country's government, tax and corporate years, `school`
/// and `university` for an academic profile's), the kind (`government`,
/// `personal-tax`, `corporate-default` or `academic`), the English name, the
/// name in the local language (empty where English is the local one), the
/// authority that fixes it (`statute`, `regulation`, `convention`,
/// `per-region`, `per-institution` or `unread`, which is a page read that
/// states the year and an instrument that was not), `1` where that is a national
/// rule and
/// not a usual choice, the calendar the start is dated in (`gregory`,
/// `persian`, `ethiopic`, `buddhist` or `indian`), the start's month and day
/// in it, how the year is named (`start-year` or `end-year`), the first and
/// last year label the system was in force (empty where it is open), `1`
/// where the start calendar is an approximation of the astronomical rule, what
/// the entry deliberately does not claim, the date the sources were last
/// checked, the sources, the first year label the sources read reach, and the
/// spans of year labels after it that no source reaches either, as
/// `first-last` separated by `;` (empty where there are none). A year label of
/// the system between its first year and the first year read, or in one of
/// those spans, is a gap, not an answer.
///
/// A validity bound is a label of the system's own calendar: Iran's are
/// Solar Hijri years and Nepal's Bikram Sambat. Nepal is in a build whose
/// facade has `indic` as well as `fiscal`, since its year starts on 1 Shrawan
/// of the Bikram Sambat; in a build without it the country is absent, which
/// [`year_on_lines`] reports as `Refusal::Unknown`.
#[must_use]
pub fn profiles_lines() -> String {
    let mut out = String::new();
    for entry in entries() {
        let system = entry.system;
        let mut line = Line::new(&mut out);
        line.cell(entry.code)
            .cell(entry.country)
            .cell(entry.table.id())
            .cell(system.kind.id())
            .cell(system.name)
            .cell(system.local_name)
            .cell(system.authority.id())
            .flag(system.authority.is_national_rule())
            .cell(system.start.calendar.id().as_str())
            .value(system.start.month)
            .value(system.start.day)
            .cell(system.label.id())
            .value_or_empty(system.valid_from)
            .value_or_empty(system.valid_until)
            .flag(system.start.calendar.is_approximate())
            .cell(system.note)
            .value(date_text(entry.checked))
            .cell(entry.sources)
            .value(system.read_from)
            .cell(&unread_text(system));
        line.end();
    }
    out
}

/// What the year systems of a country say a fixed day is, one line each:
/// the code, the table and the kind as [`profiles_lines`] writes them, the
/// English name, the status, the year label, the first and last fixed day of
/// that year, the day's number in it from 1, how many days it has, the day
/// of the week from Monday = 1 to Sunday = 7, the fiscal month, quarter and
/// half it is in from 1 (empty where the start calendar has no twelve equal
/// months), the calendar the start is dated in, how the year is named, `1`
/// where the start calendar is an approximation, and the date the sources
/// were last checked.
///
/// The status is `in-force`, or `outside-validity` where the system was not
/// in force in the year the day falls in (the United States' October year
/// had not begun in 1970), or `gap` where it was in force but the sources
/// read do not reach that year, or `outside-calendar-range` where the
/// start's calendar does not reach the day; the cells after the status are
/// then empty. `kind` is empty for every kind the country has, or one of
/// [`SystemKind::id`]'s.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a country the tables do not carry and a kind that
/// is not one of the four, [`Refusal::NoData`] for a country that has no
/// system of the kind asked, and [`Refusal::OutOfRange`] for a fixed day
/// beyond the Gregorian years ±9 999 999.
pub fn year_on_lines(country: &str, kind: &str, fixed: i64) -> Answer<String> {
    let day = Rd(fixed);
    gregorian::year_from_fixed(day).map_err(|_| Refusal::OutOfRange)?;
    let mut out = String::new();
    for entry in selected(country, kind)? {
        let system = entry.system;
        let mut line = Line::new(&mut out);
        line.cell(entry.code)
            .cell(entry.table.id())
            .cell(system.kind.id())
            .cell(system.name);
        match system.locate(day) {
            Ok(position) => {
                let span = system.projected_span(position.label).map_err(refusal)?;
                line.cell("in-force")
                    .value(position.label)
                    .value(span.first.0)
                    .value(span.last.0)
                    .value(position.day_of_year)
                    .value(position.days_in_year)
                    .value(position.weekday.iso_number());
                match (
                    system.month_of_year(day),
                    system.quarter(day),
                    system.half(day),
                ) {
                    (Ok(month), Ok(quarter), Ok(half)) => line
                        .value(month)
                        .value(quarter.ordinal())
                        .value(half.ordinal()),
                    _ => line.empties(3),
                };
            }
            Err(error) => {
                let status = match error {
                    FiscalError::OutsideValidity => "outside-validity",
                    FiscalError::NotRead => "gap",
                    FiscalError::Calendar(_) => "outside-calendar-range",
                    other => return Err(refusal(other)),
                };
                line.cell(status).empties(9);
            }
        }
        line.cell(system.start.calendar.id().as_str())
            .cell(system.label.id())
            .flag(system.start.calendar.is_approximate())
            .value(date_text(entry.checked));
        line.end();
    }
    Ok(out)
}

/// The span of the year a label names in each year system of a country, one
/// line each: the code, the table, the kind and the English name as
/// [`profiles_lines`] writes them, the status, the label, the first and last
/// fixed day of the year, and how many days it has.
///
/// The status is `in-force`, `outside-validity` where the system was not in
/// force in that year, `gap` where it was in force but the sources read do
/// not reach that year (the cells after the label are empty in both) or
/// `outside-calendar-range` where the start's calendar does not reach it. The
/// label is the system's own: a label of Iran's is a Solar Hijri year, a
/// label of Japan's 年度 the Gregorian year it begins in, and a label of
/// the United States' fiscal year the one it ends in.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a country the tables do not carry and a kind that
/// is not one of the four, [`Refusal::NoData`] for a country that has no
/// system of the kind asked, and [`Refusal::OutOfRange`] for a label whose
/// year is not one an `i64` of days reaches.
pub fn year_span_lines(country: &str, kind: &str, label: i64) -> Answer<String> {
    let mut out = String::new();
    for entry in selected(country, kind)? {
        let system = entry.system;
        let mut line = Line::new(&mut out);
        line.cell(entry.code)
            .cell(entry.table.id())
            .cell(system.kind.id())
            .cell(system.name);
        match system.span(label) {
            Ok(span) => {
                line.cell("in-force")
                    .value(span.label)
                    .value(span.first.0)
                    .value(span.last.0)
                    .value(span.days());
            }
            Err(error) => {
                let status = match error {
                    FiscalError::OutsideValidity => "outside-validity",
                    FiscalError::NotRead => "gap",
                    FiscalError::Calendar(_) => "outside-calendar-range",
                    other => return Err(refusal(other)),
                };
                line.cell(status).value(label).empties(3);
            }
        }
        line.end();
    }
    Ok(out)
}

/// Every named year of whole weeks, one line each: the identifier, the
/// English name, the weekday the year ends on from Monday = 1 to Sunday = 7,
/// the Gregorian month whose end the rule is applied to, the rule
/// (`last-weekday-of-month` or `weekday-nearest-month-end`), how the year is
/// named (`start-year` or `end-year`), the shape of a quarter's thirteen
/// weeks (`4-4-5`, `4-5-4` or `5-4-4`, empty for a convention that numbers
/// weeks and defines no periods), what the entry does not claim, the source,
/// and the date it was last checked.
///
/// The two rules are two names and not one parameter: they put the year end
/// up to a week apart and sometimes in different months.
#[must_use]
pub fn week_year_systems_lines() -> String {
    let mut out = String::new();
    for system in retail::ALL {
        let WeekYearSystem {
            id,
            name,
            anchor_weekday,
            anchor_month,
            anchor_rule,
            label,
            shape,
            note,
            source,
            sources_checked,
        } = **system;
        let mut line = Line::new(&mut out);
        line.cell(id)
            .cell(name)
            .value(anchor_weekday.iso_number())
            .value(anchor_month)
            .cell(anchor_rule.id())
            .cell(label.id())
            .cell_or_empty(shape.map(|shape| shape.english_name()))
            .cell(note)
            .cell(source)
            .value(date_text(sources_checked));
        line.end();
    }
    out
}

/// Where a fixed day is in a year of whole weeks, as one line: the system's
/// identifier and English name, the label of the year, its first and last
/// day, how many weeks it has (52 or 53), `1` where it has 53, the week the
/// day is in from 1, that week's first and last day, the period from 1 to 12
/// and its first and last day, and the quarter from 1 to 4 (the last four
/// empty for a convention with no periods).
///
/// In a 53-week year the extra week is the last period's.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a system [`week_year_systems_lines`] does not
/// list, and [`Refusal::OutOfRange`] for a day beyond the Gregorian years
/// ±9 999 999.
pub fn week_year_on_line(system: &str, fixed: i64) -> Answer<String> {
    let system = retail::by_id(system).ok_or(Refusal::Unknown)?;
    let day = Rd(fixed);
    gregorian::year_from_fixed(day).map_err(|_| Refusal::OutOfRange)?;
    let label = system.label_at(day).map_err(refusal)?;
    let (first, last) = (
        system.year_start(label).map_err(refusal)?,
        system.year_end(label).map_err(refusal)?,
    );
    let weeks = system.weeks_in_year(label).map_err(refusal)?;
    let week = system.week_of_year(day).map_err(refusal)?;
    let (week_first, week_last) = system.week_span(label, week).map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(system.id)
        .cell(system.name)
        .value(label)
        .value(first.0)
        .value(last.0)
        .value(weeks)
        .flag(weeks == 53)
        .value(week)
        .value(week_first.0)
        .value(week_last.0);
    match system.period_of_year(day) {
        Ok(period) => {
            let (period_first, period_last) = system.period_span(label, period).map_err(refusal)?;
            line.value(period)
                .value(period_first.0)
                .value(period_last.0)
                .value(system.quarter_of_year(day).map_err(refusal)?);
        }
        Err(FiscalError::PeriodsNotDefined) => {
            line.empties(4);
        }
        Err(error) => return Err(refusal(error)),
    }
    line.end();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::cells;

    fn rows(text: &str) -> alloc::vec::Vec<alloc::vec::Vec<&str>> {
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
    fn every_system_is_a_line_with_its_columns() {
        let text = profiles_lines();
        let table = rows(&text);
        assert!(
            table.iter().all(|row| row.len() == PROFILE_COLUMNS),
            "{text}"
        );
        assert_eq!(table.len(), entries().len());
        // Japan's school year is in both profiles and a line once.
        let japan: alloc::vec::Vec<_> = table.iter().filter(|row| row[0] == "JP").collect();
        let schools = japan
            .iter()
            .filter(|row| row[4] == "Japanese school year")
            .count();
        assert_eq!(schools, 1, "{japan:?}");
        // Japan's 会計年度 is statutory, begins on 1 April of the Gregorian
        // calendar and is named for the year it starts in; the 財政法 第十一条.
        let government = japan
            .iter()
            .find(|row| row[4] == "Japanese national fiscal year")
            .copied()
            .cloned()
            .unwrap_or_default();
        assert_eq!(&government[..2], ["JP", "Japan"]);
        assert_eq!(government[6], "statute");
        assert_eq!(government[7], "1");
        assert_eq!(&government[8..12], ["gregory", "4", "1", "start-year"]);
        assert!(government[17].contains("財政法"), "{}", government[17]);
        // The April year was established in 1886 and read from then; 1921 to
        // 1946 are a stretch no source read reaches.
        assert_eq!(&government[12..14], ["1886", ""]);
        assert_eq!(&government[18..], ["1886", "1921-1946"]);
        // The July year before it, and the nine months of 明治18年度 between.
        let july = japan
            .iter()
            .find(|row| row[4] == "Japanese national fiscal year, July basis")
            .copied()
            .cloned()
            .unwrap_or_default();
        assert_eq!(
            &july[8..14],
            ["gregory", "7", "1", "start-year", "1875", "1884"]
        );
        assert_eq!(july[6], "unread");
        assert_eq!(july[7], "0");
        // The United States' federal year is named for the year it ends in.
        let us = table
            .iter()
            .find(|row| row[0] == "US" && row[4] == "United States federal fiscal year")
            .cloned()
            .unwrap_or_default();
        assert_eq!(&us[8..12], ["gregory", "10", "1", "end-year"]);
        assert_eq!(us[12], "1977");
    }

    #[test]
    fn the_same_day_is_in_two_fiscal_years_a_year_apart() {
        // 1 November 2023: Tokyo is in 2023年度 and Washington in FY 2024.
        let day = fixed(2023, 11, 1);
        let text = year_on_lines("JP", "government", day).unwrap_or_default();
        let all = rows(&text);
        // The July year of 1875 to 1884 is a system of its own, and is not
        // in force in 2023.
        assert_eq!(all.len(), 2);
        assert_eq!(all[0][4], "outside-validity");
        let japan: alloc::vec::Vec<_> = all.into_iter().skip(1).collect();
        assert_eq!(japan.len(), 1);
        assert_eq!(japan[0].len(), YEAR_ON_COLUMNS);
        assert_eq!(
            &japan[0][..5],
            [
                "JP",
                "fiscal",
                "government",
                "Japanese national fiscal year",
                "in-force"
            ]
        );
        assert_eq!(japan[0][5], "2023");
        assert_eq!(japan[0][6], fixed(2023, 4, 1).to_string());
        assert_eq!(japan[0][7], fixed(2024, 3, 31).to_string());
        // The day is the 215th of the year, in a 366-day one, a Wednesday;
        // November is fiscal month 8, in the third quarter.
        assert_eq!(&japan[0][8..11], ["215", "366", "3"]);
        assert_eq!(&japan[0][11..14], ["8", "3", "2"]);
        let text = year_on_lines("us", "government", day).unwrap_or_default();
        let us = rows(&text);
        assert_eq!(us[0][4], "in-force");
        assert_eq!(us[0][5], "2024");
        assert_eq!(us[0][6], fixed(2023, 10, 1).to_string());
        assert_eq!(us[0][7], fixed(2024, 9, 30).to_string());
        assert_eq!(&us[0][11..14], ["2", "1", "1"]);
        // The United Kingdom's government year and its personal tax year start
        // five days apart: 1 April and 6 April.
        let text = year_on_lines("GB", "", fixed(2024, 4, 3)).unwrap_or_default();
        let uk = rows(&text);
        let government = uk
            .iter()
            .find(|row| row[2] == "government")
            .cloned()
            .unwrap_or_default();
        let tax = uk
            .iter()
            .find(|row| row[2] == "personal-tax")
            .cloned()
            .unwrap_or_default();
        assert_eq!(government[5], "2024");
        assert_eq!(tax[5], "2023");
        assert!(uk.iter().any(|row| row[2] == "academic"));
        assert!(uk.iter().all(|row| row.len() == YEAR_ON_COLUMNS));
    }

    #[test]
    fn a_system_is_silent_before_it_was_in_force() {
        // The United States' October year began with FY 1977: a day in 1970
        // is in the July system's year and not the October one's.
        let text = year_on_lines("US", "government", fixed(1970, 3, 1)).expect("a line");
        let table = rows(&text);
        let october = table
            .iter()
            .find(|row| row[3] == "United States federal fiscal year")
            .cloned()
            .unwrap_or_default();
        assert_eq!(october[4], "outside-validity", "{table:?}");
        assert!(october[5..14].iter().all(|cell| cell.is_empty()));
        assert!(table.iter().any(|row| row[4] == "in-force"), "{table:?}");
        assert_eq!(year_on_lines("XX", "", 0), Err(Refusal::Unknown));
        assert_eq!(year_on_lines("JP", "tax-ish", 0), Err(Refusal::Unknown));
        assert_eq!(
            year_on_lines("JP", "personal-tax", 739_000),
            Err(Refusal::NoData)
        );
        assert_eq!(year_on_lines("JP", "", i64::MAX), Err(Refusal::OutOfRange));
    }

    #[test]
    fn a_year_no_source_reaches_is_a_gap_and_not_an_answer() {
        // The United States' federal year was the calendar year until the Act
        // of 1842, which the Treasury's letter of 15 December 1842 dates: the
        // first July year is FY 1844. A day of 1800 is in no July or October
        // year, and the calendar-year system, read from 1842, is a gap.
        let text = year_on_lines("US", "government", fixed(1800, 6, 1)).expect("lines");
        let table = rows(&text);
        let status = |name: &str| {
            table
                .iter()
                .find(|row| row[3] == name)
                .map(|row| row[4])
                .unwrap_or_default()
        };
        assert_eq!(
            status("United States federal fiscal year"),
            "outside-validity"
        );
        assert_eq!(
            status("United States federal fiscal year, July basis"),
            "outside-validity"
        );
        assert_eq!(
            status("United States federal fiscal year, calendar-year basis"),
            "gap"
        );
        let gap = table
            .iter()
            .find(|row| row[4] == "gap")
            .cloned()
            .unwrap_or_default();
        assert_eq!(gap.len(), YEAR_ON_COLUMNS);
        assert!(gap[5..14].iter().all(|cell| cell.is_empty()), "{gap:?}");
        // Japan's April year, 1921 to 1946, was not read: a gap.
        let text = year_on_lines("JP", "government", fixed(1930, 6, 1)).expect("lines");
        let japan = rows(&text);
        let april = japan
            .iter()
            .find(|row| row[3] == "Japanese national fiscal year")
            .cloned()
            .unwrap_or_default();
        assert_eq!(april[4], "gap");
        // Iran's year is read from 1350, and 1975 is 1354.
        let text = year_on_lines("IR", "government", fixed(1975, 6, 1)).expect("lines");
        let iran = rows(&text);
        assert_eq!((iran[0][4], iran[0][5]), ("in-force", "1354"));
        // The same through a label.
        let text = year_span_lines("US", "government", 1800).expect("lines");
        let by_label = rows(&text);
        assert!(
            by_label
                .iter()
                .any(|row| row[4] == "gap" && row[5] == "1800" && row[6].is_empty()),
            "{by_label:?}"
        );
        let text = year_span_lines("US", "government", 1844).expect("lines");
        let by_label = rows(&text);
        let july = by_label
            .iter()
            .find(|row| row[4] == "in-force")
            .cloned()
            .unwrap_or_default();
        assert_eq!(july[6], fixed(1843, 7, 1).to_string());
        assert_eq!(july[7], fixed(1844, 6, 30).to_string());
    }

    #[test]
    fn a_label_has_its_span_in_the_systems_own_calendar() {
        // Japan's 2024年度 runs 1 April 2024 to 31 March 2025.
        let text = year_span_lines("JP", "government", 2024).unwrap_or_default();
        let japan: alloc::vec::Vec<_> = rows(&text)
            .into_iter()
            .filter(|row| row[4] == "in-force")
            .collect();
        assert_eq!(japan.len(), 1);
        assert_eq!(japan[0].len(), YEAR_SPAN_COLUMNS);
        assert_eq!(japan[0][6], fixed(2024, 4, 1).to_string());
        assert_eq!(japan[0][7], fixed(2025, 3, 31).to_string());
        assert_eq!(japan[0][8], "365");
        // The United States' FY 2024 began on 1 October 2023.
        let text = year_span_lines("US", "government", 2024).unwrap_or_default();
        let us = rows(&text);
        let federal = us
            .iter()
            .find(|row| row[4] == "in-force")
            .cloned()
            .unwrap_or_default();
        assert_eq!(federal[6], fixed(2023, 10, 1).to_string());
        assert_eq!(federal[7], fixed(2024, 9, 30).to_string());
        assert_eq!(federal[8], "366");
        // Before it was in force, the label is carried and nothing else.
        let early = year_span_lines("US", "government", 1970).unwrap_or_default();
        let early = rows(&early);
        assert!(
            early
                .iter()
                .any(|row| row[4] == "outside-validity" && row[5] == "1970" && row[6].is_empty())
        );
        assert_eq!(year_span_lines("ZZ", "", 2024), Err(Refusal::Unknown));
        assert_eq!(
            year_span_lines("JP", "personal-tax", 2024),
            Err(Refusal::NoData)
        );
    }

    #[test]
    fn iran_begins_its_year_at_nowruz() {
        // 1405 SH begins on 1 Farvardin, 21 March 2026 (Solar Hijri 1405
        // Farvardin 1 is 2026-03-21); the start calendar is approximate.
        let text = year_span_lines("IR", "government", 1405).unwrap_or_default();
        let iran = rows(&text);
        assert_eq!(iran[0][4], "in-force");
        assert_eq!(iran[0][6], fixed(2026, 3, 21).to_string());
        let on = year_on_lines("IR", "government", fixed(2026, 3, 21)).unwrap_or_default();
        let row = cells(&on)
            .iter()
            .map(|cell| cell.to_string())
            .collect::<alloc::vec::Vec<_>>();
        assert_eq!(row[5], "1405");
        assert_eq!(&row[14..16], ["persian-arithmetic-33", "start-year"]);
    }

    #[test]
    fn the_week_years_end_on_a_saturday_by_their_two_rules() {
        let text = week_year_systems_lines();
        let table = rows(&text);
        assert_eq!(table.len(), retail::ALL.len());
        assert!(
            table
                .iter()
                .all(|row| row.len() == WEEK_YEAR_SYSTEM_COLUMNS)
        );
        let nrf = table
            .iter()
            .find(|row| row[0] == "nrf-4-5-4")
            .cloned()
            .unwrap_or_default();
        assert_eq!(
            &nrf[2..7],
            ["6", "1", "weekday-nearest-month-end", "start-year", "4-5-4"]
        );
        // NRF's retail 2023 ran from Sunday 29 January 2023 to Saturday
        // 3 February 2024, 53 weeks; 1 March 2023 is 31 days in, in the 5th
        // week.
        let line = week_year_on_line("nrf-4-5-4", fixed(2023, 3, 1)).unwrap_or_default();
        let row = cells(&line);
        assert_eq!(row.len(), WEEK_YEAR_ON_COLUMNS);
        assert_eq!(row[2], "2023");
        assert_eq!(row[3], fixed(2023, 1, 29).to_string());
        assert_eq!(row[4], fixed(2024, 2, 3).to_string());
        assert_eq!(&row[5..7], ["53", "1"]);
        assert_eq!(row[7], "5");
        assert_eq!(week_year_on_line("nrf-4-5-5", 0), Err(Refusal::Unknown));
        assert_eq!(
            week_year_on_line("nrf-4-5-4", i64::MAX),
            Err(Refusal::OutOfRange)
        );
        // ISO 8601's week year defines no periods.
        let iso = week_year_on_line("iso-8601-week-year", fixed(2026, 6, 1)).unwrap_or_default();
        let row = cells(&iso);
        assert_eq!(row.len(), WEEK_YEAR_ON_COLUMNS);
        assert_eq!(&row[10..], ["", "", "", ""]);
    }
}
