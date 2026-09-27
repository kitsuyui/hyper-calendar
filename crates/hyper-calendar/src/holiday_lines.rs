//! The tab-separated lines the WebAssembly module and the C library write
//! about the holiday tables and the liturgical year, written once.
//!
//! * The tables themselves, in the order `hc_holiday_codes` lists them,
//!   with the kind each is, its names, its sources and the country an
//!   exchange keeps the holidays of, each read from the table's own data
//!   but for a country's name in a locale, which is CLDR's, from
//!   [`hc_i18n::territories`].
//! * The lectionary cycles of a day, and the astronomical Easter of a year
//!   and the paschal full moon it is the Sunday after,
//!   from [`hc_holiday::lectionary`] and [`hc_holiday::computus`].
//! * The Holy Year a day falls in, from [`hc_holiday::holy_years`], and the
//!   rank of each *Common Worship* celebration kept on a day, from
//!   [`hc_holiday::common_worship`]. The celebrations themselves, and the
//!   years the Rules leave a Festival without a day, are already lines of
//!   `hc_holidays_on`, in the `common-worship` table; the rank is what
//!   these lines add.
//! * The Eastern Orthodox fast a day falls in, and the fasting seasons and
//!   fast-free weeks of a year, under each reckoning of
//!   [`hc_holiday::orthodox_fasts`]: two reckonings, two identifiers
//!   (`docs/policy.md` §5), which the caller names.

use alloc::string::String;
use core::fmt::Write;

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::common_worship::{CELEBRATIONS, COMMON_WORSHIP, Rank};
use hc_holiday::holy_years::{self, HolyYearOn, Jubilee, JubileeKind, TableDate};
use hc_holiday::orthodox_fasts::{self, Abstinence, Period, PeriodKind, Reckoning, Status};
use hc_holiday::rule::RuleSet;
use hc_holiday::{
    HolidayCalendar, computus, countries, exchanges, international, lectionary, traditions,
};
use hc_i18n::Locale;
use hc_i18n::territories::{self, TerritoryName};

use crate::boundary::{Answer, Refusal, names, push_cell};

/// How many columns [`holiday_tables`] writes.
pub const HOLIDAY_TABLES_COLUMNS: usize = 8;

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

/// The locale a tag asks for, read as the calendar lines read one: `None`
/// for `native`, which names no one locale (and a table has no language
/// of its own to ask for), and the root locale for a tag that does not
/// parse.
fn requested_locale(tag: &str) -> Option<Locale> {
    if tag == "native" {
        None
    } else {
        Some(Locale::parse(tag).unwrap_or(Locale::ROOT))
    }
}

/// What a table is called in a locale, and the tag of the data that
/// answered: a country's CLDR name where the locale's fallback chain has
/// one, else its name in CLDR's `en`, answered by `en`, so that `en` stands
/// for one name of a country whichever locale was asked. Every other kind
/// is named by the table's English name, answered by `en` — CLDR names no
/// exchange, tradition or set of observances, and nothing here invents a
/// name for one.
#[must_use]
pub fn table_name(set: &RuleSet, kind: TableKind, locale: Option<&Locale>) -> TerritoryName {
    cldr_name(set, kind, locale).unwrap_or(TerritoryName {
        name: set.english_name,
        tag: "en",
    })
}

/// A country's CLDR name in a locale, where the locale's fallback chain
/// has one, and else in CLDR's `en`: what [`table_name`] answers for every
/// country CLDR names, which is all of them. Under `native`, which names
/// no locale, the `en` one.
fn cldr_name(set: &RuleSet, kind: TableKind, locale: Option<&Locale>) -> Option<TerritoryName> {
    if kind != TableKind::Country {
        return None;
    }
    locale
        .and_then(|locale| territories::territory_name(locale, set.code))
        .or_else(|| {
            let english = Locale::parse("en").ok()?;
            territories::territory_name(&english, set.code)
        })
}

/// A country's CLDR `alt="short"` name, from the same locale's data as the
/// name [`table_name`] gives it — `Hong Kong` for `HK` under `en`, `香港`
/// under `ja` — where that name is CLDR's and the data has a short one;
/// see [`territories::short_name`]. A country named from CLDR's `en` by
/// fallback has `en`'s short name; nothing for any other kind of table,
/// whose English name is its own and not CLDR's.
#[must_use]
pub fn short_table_name(
    set: &RuleSet,
    kind: TableKind,
    locale: Option<&Locale>,
) -> Option<&'static str> {
    cldr_name(set, kind, locale).and_then(|named| territories::short_name(named.tag, set.code))
}

/// The lines of `hc_holiday_tables`, one per table in [`tables`] order:
/// the code, the kind, the name in the locale, the English name, the
/// locale that answered, the sources, the country of a subdivision or an
/// exchange, and the short name in the locale.
///
/// Column 3 follows [`table_name`]: a country is named as the locale's
/// CLDR 48 data names it, where `hc-i18n` carries a name for it — 日本 under
/// `ja`, Deutschland under `de` — and column 5 is the tag of the data that
/// answered, `ja` or `de`, so that a request for `de-AT` says `de`. A
/// country the locale has no name for, and every country under `native`
/// or a tag whose chain reaches no territory names, is named as CLDR's
/// `en` names it — `Hong Kong SAR China`, not the table's own `Hong Kong`
/// — with `en` in column 5, so that `en` means one name of a country
/// wherever it appears. Every exchange, tradition and set of observances
/// is named by the table's own English name, column 4, with `en` in
/// column 5. So column 3 is never empty. Column 7
/// stays a code, the key of the country's own row, whose column 3 names it
/// in the same locale. Column 8 follows [`short_table_name`]: CLDR 48's
/// `alt="short"` name of a country column 3 names from CLDR, from the same
/// locale's data — `Hong Kong` for `HK` under `en`, `香港` under `ja`, `UK`
/// for `GB` — and empty where that data has none, which is most countries,
/// and for every table that is not a country.
#[must_use]
pub fn holiday_tables(locale: &str) -> String {
    let requested = requested_locale(locale);
    let mut out = String::new();
    for (set, kind) in tables_with_kinds() {
        let named = table_name(set, kind, requested.as_ref());
        push_cell(&mut out, set.code);
        let _ = write!(out, "\t{}\t", kind.name());
        push_cell(&mut out, named.name);
        out.push('\t');
        push_cell(&mut out, set.english_name);
        out.push('\t');
        out.push_str(named.tag);
        out.push('\t');
        push_cell(&mut out, set.sources);
        out.push('\t');
        if let Some(country) = country_of(set, kind) {
            push_cell(&mut out, country);
        }
        out.push('\t');
        if let Some(short) = short_table_name(set, kind, requested.as_ref()) {
            push_cell(&mut out, short);
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

/// The paschal full moon of a Gregorian year by the astronomical
/// reckoning, the day at Jerusalem of the first full moon at or after the
/// March equinox, as a fixed day: the day [`astronomical_easter`] is the
/// Sunday after.
///
/// # Errors
///
/// As [`astronomical_easter`].
pub fn astronomical_paschal_full_moon(year: i64) -> Answer<i64> {
    computus::astronomical_paschal_full_moon(year)
        .map(|day| day.0)
        .ok_or(Refusal::OutOfRange)
}

/// How many columns [`holy_year_line`] writes.
pub const HOLY_YEAR_COLUMNS: usize = 10;

/// How many columns [`common_worship_lines`] write.
pub const COMMON_WORSHIP_COLUMNS: usize = 3;

/// The fixed day of a table date, as a cell.
fn push_table_date(out: &mut String, date: TableDate) {
    let (year, month, day) = date;
    if let Ok(day) = gregorian::to_fixed(year, month, day) {
        let _ = write!(out, "{}", day.0);
    }
}

/// A jubilee's cells of [`holy_year_line`], after the first.
fn push_jubilee(out: &mut String, jubilee: &Jubilee) {
    push_cell(out, jubilee.title);
    out.push('\t');
    out.push_str(match jubilee.kind {
        JubileeKind::Ordinary => "ordinary",
        JubileeKind::Extraordinary => "extraordinary",
    });
    for text in [jubilee.pope, jubilee.bull] {
        out.push('\t');
        push_cell(out, text);
    }
    for date in [jubilee.given, jubilee.opens, jubilee.closes] {
        out.push('\t');
        push_table_date(out, date);
    }
    for date in [
        jubilee.particular_churches.map(|(opens, _)| opens),
        jubilee.particular_churches.map(|(_, closes)| closes),
    ] {
        out.push('\t');
        if let Some(date) = date {
            push_table_date(out, date);
        }
    }
}

/// The line of `hc_holy_year_on`: `within` when a day falls in a Holy Year
/// of the Catholic Church in Rome, from the opening of the Holy Door of
/// St Peter's to its closing, or `outside`; then, within one, the
/// jubilee's title, `ordinary` or `extraordinary`, the Pope who proclaimed
/// it, the bull of indiction by its opening words, the day the bull was
/// given and the jubilee's first and last days in Rome, as fixed days, and
/// its first and last days in the dioceses where the bull dates them, else
/// empty. Outside one the nine cells after the first are empty.
///
/// # Errors
///
/// [`Refusal::NoData`] before the first jubilee the table carries, 1975's
/// opening on 24 December 1974, and after the day its sources were checked,
/// [`holy_years::SOURCES_CHECKED`].
pub fn holy_year_line(fixed: i64) -> Answer<String> {
    let mut out = String::new();
    match holy_years::holy_year_on(Rd(fixed)) {
        HolyYearOn::Within(jubilee) => {
            out.push_str("within\t");
            push_jubilee(&mut out, jubilee);
            out.push('\n');
        }
        HolyYearOn::Outside => out.push_str("outside\t\t\t\t\t\t\t\t\t\n"),
        HolyYearOn::NotCarried => return Err(Refusal::NoData),
    }
    Ok(out)
}

/// The identifier a [`Rank`] is written as.
const fn rank_id(rank: Rank) -> &'static str {
    match rank {
        Rank::PrincipalFeast => "principal-feast",
        Rank::PrincipalHolyDay => "principal-holy-day",
        Rank::Festival => "festival",
    }
}

/// The lines of `hc_common_worship_on`: every Principal Feast, Principal
/// Holy Day and Festival of the Church of England's *Common Worship*
/// calendar kept on a day, after the transfers its Rules require, one line
/// each: the title as the Rules print it, which is the name
/// `hc_holidays_on` gives it in the `common-worship` table; the rank's
/// identifier, `principal-feast`, `principal-holy-day` or `festival`; and
/// the rank's English name. A day that keeps none writes nothing.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day with no Gregorian year.
pub fn common_worship_lines(fixed: i64) -> Answer<String> {
    let day = Rd(fixed);
    gregorian::year_from_fixed(day).map_err(|_| Refusal::OutOfRange)?;
    let calendar = HolidayCalendar::for_day(&COMMON_WORSHIP, None, day);
    let mut out = String::new();
    for holiday in calendar.on(day) {
        let Some(celebration) = CELEBRATIONS
            .iter()
            .find(|celebration| celebration.title == holiday.name)
        else {
            continue;
        };
        push_cell(&mut out, celebration.title);
        let _ = writeln!(
            out,
            "\t{}\t{}",
            rank_id(celebration.rank),
            celebration.rank.english_name()
        );
    }
    Ok(out)
}

/// The reckoning of the Orthodox fasts an identifier names, one of
/// [`Reckoning::ALL`], `orthodox-fasts` or `orthodox-fasts-revised-julian`,
/// in any ASCII case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other.
pub fn orthodox_fasts_reckoning(id: &str) -> Answer<&'static Reckoning> {
    Reckoning::ALL
        .iter()
        .find(|reckoning| names(id, reckoning.id))
        .ok_or(Refusal::Unknown)
}

/// The identifier a [`PeriodKind`] is written as.
const fn period_kind_id(kind: PeriodKind) -> &'static str {
    match kind {
        PeriodKind::Fast => "fast",
        PeriodKind::FastFree => "fast-free",
        PeriodKind::MeatExcluded => "meat-excluded",
    }
}

/// The first and last years of either reckoning's calendar the Julian
/// computus gives a Pascha for, and so the only years with a scheme.
const FAST_YEARS: core::ops::RangeInclusive<i64> = 326..=4_099;

/// How many columns [`orthodox_fast_line`] writes.
pub const ORTHODOX_FAST_COLUMNS: usize = 6;

/// The line of `hc_orthodox_fast_on`: what a day is in the fasting scheme
/// of a reckoning — `1` if it is a fast day, else `0`; `period`,
/// `weekly-fast` for a Wednesday or Friday in no period, or `none`; and
/// for a period its identifier, its English name as the OCA's outline
/// gives it and its kind, `fast`, `fast-free` or `meat-excluded`, else three
/// empty cells; and what the day abstains from, `nothing`, `meat` or
/// `fast`, which alone tells a day of the Meatfast, not a fast day but one
/// without meat, from an ordinary one.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a reckoning [`orthodox_fasts_reckoning`] does
/// not name, and [`Refusal::OutOfRange`] for a day outside the years 326
/// to 4099 of the reckoning's calendar.
pub fn orthodox_fast_line(reckoning: &str, fixed: i64) -> Answer<String> {
    let reckoning = orthodox_fasts_reckoning(reckoning)?;
    // A generous bound first, so that no calendar arithmetic meets a day
    // near the ends of an `i64`: the years 300 to 4200.
    if !(100_000..=1_600_000).contains(&fixed) {
        return Err(Refusal::OutOfRange);
    }
    let status = orthodox_fasts::status(reckoning, Rd(fixed)).ok_or(Refusal::OutOfRange)?;
    let mut out = String::new();
    let _ = write!(out, "{}\t", u8::from(status.is_fast_day()));
    match status {
        Status::InPeriod(period) => {
            out.push_str("period\t");
            push_cell(&mut out, period.id);
            out.push('\t');
            push_cell(&mut out, period.english_name);
            let _ = write!(out, "\t{}\t", period_kind_id(period.kind));
        }
        Status::WeeklyFast(_) => out.push_str("weekly-fast\t\t\t\t"),
        Status::NotFasting => out.push_str("none\t\t\t\t"),
    }
    out.push_str(match status.abstinence() {
        Abstinence::Nothing => "nothing",
        Abstinence::Meat => "meat",
        Abstinence::Fast => "fast",
    });
    out.push('\n');
    Ok(out)
}

/// How many columns [`orthodox_fast_seasons_lines`] writes.
pub const ORTHODOX_FAST_SEASON_COLUMNS: usize = 5;

/// The lines of `hc_orthodox_fast_seasons`: every period of the scheme
/// that begins in a year of a reckoning's calendar, one a line in the
/// order a day is tested against them, as its identifier, its English
/// name, its kind (`fast`, `fast-free` or `meat-excluded`), and its first and last days as fixed days, both
/// included; both are empty in a year the period does not happen, which
/// only the Apostles' Fast does, on the Revised Julian reckoning when
/// Pascha is late. Christmastide ends in the next year.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a reckoning [`orthodox_fasts_reckoning`] does
/// not name, and [`Refusal::OutOfRange`] for a year outside 326 to 4099.
pub fn orthodox_fast_seasons_lines(reckoning: &str, year: i64) -> Answer<String> {
    let reckoning = orthodox_fasts_reckoning(reckoning)?;
    if !FAST_YEARS.contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    let mut out = String::new();
    for period in Period::ALL {
        push_cell(&mut out, period.id);
        out.push('\t');
        push_cell(&mut out, period.english_name);
        let _ = write!(out, "\t{}\t", period_kind_id(period.kind));
        if let Some((first, last)) = orthodox_fasts::span(reckoning, period, year) {
            let _ = write!(out, "{}\t{}", first.0, last.0);
        } else {
            out.push('\t');
        }
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::{BTreeMap, BTreeSet};
    use alloc::vec::Vec;

    fn ymd(year: i64, month: u8, day: u8) -> i64 {
        gregorian::to_fixed(year, month, day).expect("a date").0
    }

    /// The worked example of `docs/systems/orthodox-fasts.md`, from the
    /// OCA's outline (`oca-fasting-seasons`): in 2025 Great Lent began on
    /// 3 March, Bright Wednesday, 23 April, was fast-free, and the week
    /// after the Publican and the Pharisee made Wednesday 12 February
    /// fast-free, as the Holy Trinity calendar marks it
    /// (`holy-trinity-calendar`).
    #[test]
    fn the_fasts_of_2025_are_the_worked_examples() {
        let line = |reckoning: &str, fixed: i64| {
            let line = orthodox_fast_line(reckoning, fixed).expect("in range");
            let cells: Vec<String> = line
                .trim_end_matches('\n')
                .split('\t')
                .map(String::from)
                .collect();
            assert_eq!(cells.len(), ORTHODOX_FAST_COLUMNS);
            cells
        };
        assert_eq!(
            line("orthodox-fasts", ymd(2025, 3, 3)),
            [
                "1",
                "period",
                "great-lent",
                "Great Lent & Holy Week",
                "fast",
                "fast"
            ]
        );
        assert_eq!(
            line("orthodox-fasts", ymd(2025, 4, 23))[..3],
            ["0", "period", "bright-week"]
        );
        assert_eq!(
            line("orthodox-fasts-revised-julian", ymd(2025, 2, 12))[..3],
            ["0", "period", "publican-and-pharisee-week"]
        );
        // Wednesday 1 October 2025 is in no period on either reckoning.
        assert_eq!(
            line("ORTHODOX-FASTS", ymd(2025, 10, 1)),
            ["1", "weekly-fast", "", "", "", "fast"]
        );
        // Wednesday 18 February 2026 is in the Meatfast, Cheesefare week,
        // Pascha being 12 April: no fast day, though a Wednesday, but no
        // meat (`oca-fasting-seasons`).
        assert_eq!(
            line("orthodox-fasts", ymd(2026, 2, 18)),
            [
                "0",
                "period",
                "meatfast",
                "Meatfast",
                "meat-excluded",
                "meat"
            ]
        );
        assert_eq!(
            line("orthodox-fasts", ymd(2025, 10, 2)),
            ["0", "none", "", "", "", "nothing"]
        );
        assert_eq!(
            orthodox_fast_line("coptic", ymd(2025, 1, 1)),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            orthodox_fast_line("orthodox-fasts", i64::MIN),
            Err(Refusal::OutOfRange)
        );
    }

    /// The Apostles' Fast of 2025 ran from Monday 16 June to 11 July on the
    /// Julian reckoning and to 28 June on the Revised Julian one
    /// (`wikipedia-apostles-fast`); in 2024 it did not happen on the
    /// Revised Julian reckoning, the Monday after All Saints' being 1 July
    /// (`trueorthodox-apostles-2024`).
    #[test]
    fn the_apostles_fast_moves_and_vanishes() {
        let apostles = |reckoning: &str, year: i64| {
            let text = orthodox_fast_seasons_lines(reckoning, year).expect("in range");
            let rows: Vec<Vec<String>> = text
                .lines()
                .map(|line| line.split('\t').map(String::from).collect())
                .collect();
            assert_eq!(rows.len(), Period::ALL.len());
            for row in &rows {
                assert_eq!(row.len(), ORTHODOX_FAST_SEASON_COLUMNS);
            }
            rows.into_iter()
                .find(|row| row[0] == "apostles-fast")
                .expect("the Apostles' Fast has a line")
        };
        let julian = apostles("orthodox-fasts", 2025);
        assert_eq!(
            julian[2..],
            [
                "fast".to_owned(),
                ymd(2025, 6, 16).to_string(),
                ymd(2025, 7, 11).to_string()
            ]
        );
        let revised = apostles("orthodox-fasts-revised-julian", 2025);
        assert_eq!(revised[4], ymd(2025, 6, 28).to_string());
        let vanished = apostles("orthodox-fasts-revised-julian", 2024);
        assert_eq!(vanished[3..], ["", ""]);
        assert_eq!(
            orthodox_fast_seasons_lines("orthodox-fasts", 325),
            Err(Refusal::OutOfRange)
        );
    }

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

    /// The WebAssembly README's timing of one day states how many tables
    /// the day was read across; that number is this list's length.
    #[test]
    fn the_wasm_readme_counts_the_tables_there_are() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../hyper-calendar-wasm/README.md"
        );
        let readme = std::fs::read_to_string(path).expect("the WebAssembly README");
        let stated = format!("across all {} tables", tables().count());
        assert!(
            readme
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .contains(&stated),
            "the README should say {stated:?}"
        );
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
    }

    /// The rows of [`holiday_tables`] in a locale, by code.
    fn rows_in(locale: &str) -> BTreeMap<String, Vec<String>> {
        holiday_tables(locale)
            .lines()
            .map(|line| {
                let cells: Vec<String> = line.split('\t').map(String::from).collect();
                (cells[0].clone(), cells)
            })
            .collect()
    }

    /// CLDR 48 `ja.xml` has `<territory type="JP">日本</territory>` and
    /// `de.xml` `<territory type="DE">Deutschland</territory>`, both
    /// approved.
    #[test]
    fn japanese_names_japan_and_german_names_germany() {
        let japanese = rows_in("ja");
        assert_eq!(japanese["JP"][2..5], ["日本", "Japan", "ja"]);
        assert_eq!(rows_in("ja-JP")["JP"][2..5], ["日本", "Japan", "ja"]);
        let german = rows_in("de-AT");
        assert_eq!(german["DE"][2..5], ["Deutschland", "Germany", "de"]);
        assert_eq!(german["JP"][2], "Japan");
        assert_eq!(german["JP"][4], "de");
        // English is CLDR's English: its plain value for HK, not the
        // table's own name, which column 4 keeps.
        let english = rows_in("en");
        assert_eq!(
            english["HK"][2..5],
            ["Hong Kong SAR China", "Hong Kong", "en"]
        );
        // Column 8 is CLDR's short name in the same data: `en.xml` and
        // `ja.xml` both shorten Hong Kong, and neither Japan.
        assert_eq!(english["HK"][7], "Hong Kong");
        assert_eq!(english["GB"][7], "UK");
        assert_eq!(english["JP"][7], "");
        assert_eq!(japanese["HK"][2], "中華人民共和国香港特別行政区");
        assert_eq!(japanese["HK"][7], "香港");
        assert_eq!(japanese["MO"][7], "マカオ");
        assert_eq!(japanese["PS"][7], "パレスチナ");
        assert_eq!(japanese["JP"][7], "");
        assert_eq!(rows_in("ja-JP")["HK"][7], "香港");
        assert_eq!(rows_in("en-GB")["US"][7], "US");
        assert_eq!(japanese["XHKG"][7], "", "an exchange has no short name");
        // An exchange and a tradition stay English, whatever the locale;
        // column 7 stays the code of the country's own row.
        assert_eq!(
            japanese["XJPX"][2..5],
            [
                "Tokyo Stock Exchange (JPX)",
                "Tokyo Stock Exchange (JPX)",
                "en"
            ]
        );
        assert_eq!(japanese["XJPX"][6], "JP");
        assert_eq!(japanese["christian-western"][4], "en");
    }

    /// Kabyle's `HK` is unconfirmed in CLDR 48 `kab.xml`, Tibetan's `FR`
    /// likewise in `bo.xml`, and every Coptic value is: each falls back to
    /// CLDR's English name, answered by `en`, beside the names the same
    /// locales do carry — the same name and short name `en` itself gives,
    /// not the table's own English name where the two differ.
    #[test]
    fn a_country_the_locale_does_not_name_falls_back_to_cldr_english() {
        let english = rows_in("en");
        let kabyle = rows_in("kab");
        assert_eq!(
            kabyle["HK"][2..5],
            ["Hong Kong SAR China", "Hong Kong", "en"]
        );
        assert_eq!(kabyle["HK"][7], "Hong Kong");
        assert_eq!(kabyle["JP"][2..5], ["Jappu", "Japan", "kab"]);
        let tibetan = rows_in("bo");
        assert_eq!(tibetan["FR"][2..5], ["France", "France", "en"]);
        assert_eq!(tibetan["JP"][4], "bo");
        for tag in ["cop", "native", "und", "not a tag"] {
            let rows = rows_in(tag);
            for (code, row) in &rows {
                assert_eq!(row[4], "en", "{tag} {code}");
                let in_english = &english[code];
                assert_eq!(
                    (&row[2], &row[7]),
                    (&in_english[2], &in_english[7]),
                    "{tag} {code}"
                );
            }
        }
        // Wherever `en` answers for a country, it gives `en`'s own name.
        for tag in ["kab", "zgh", "bo", "sa"] {
            for (code, row) in rows_in(tag) {
                if row[4] == "en" && row[1] == "country" {
                    assert_eq!(row[2], english[&code][2], "{tag} {code}");
                }
            }
        }
    }

    /// Every table a line names a country by is one `hc-i18n` has names
    /// for, and CLDR's English names them all.
    #[test]
    fn every_country_and_every_exchanges_country_is_a_territory() {
        let english = territories::table("en").expect("English");
        for (set, kind) in tables_with_kinds() {
            let code = match kind {
                TableKind::Country => Some(set.code),
                _ => country_of(set, kind),
            };
            if let Some(code) = code {
                assert!(english.name_of(code).is_some(), "{code}");
            }
        }
        assert_eq!(countries::ALL.len(), territories::REGIONS.len());
    }

    /// The names a page shows, column 3, are distinct within a kind in
    /// every locale `hc-i18n` carries, in `native` and in English, as the
    /// English names are.
    #[test]
    fn no_two_tables_of_a_kind_share_a_name_in_any_locale() {
        let tags = ["en", "native", "und"]
            .into_iter()
            .chain(hc_i18n::data::LOCALES.iter().map(|data| data.tag));
        for tag in tags {
            let mut seen = BTreeMap::new();
            for row in rows_in(tag).into_values() {
                assert!(!row[2].is_empty(), "{tag} {}", row[0]);
                let key = (row[1].clone(), row[2].clone());
                if let Some(other) = seen.insert(key, row[0].clone()) {
                    panic!("{tag}: {other} and {} are both {}", row[0], row[2]);
                }
            }
        }
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

    /// The Aleppo statement's table (`wcc-aleppo-1997`) puts the
    /// astronomical vernal full moon of 2001 on Sunday 8 April, a week
    /// before Easter, and that of 2019 on 21 March, the day of the equinox.
    #[test]
    fn the_paschal_full_moons_of_2001_and_2019_are_the_aleppo_tables() {
        assert_eq!(astronomical_paschal_full_moon(2001), Ok(day(2001, 4, 8)));
        assert_eq!(astronomical_paschal_full_moon(2019), Ok(day(2019, 3, 21)));
        assert_eq!(astronomical_easter(2019), Ok(day(2019, 3, 24)));
        assert_eq!(
            astronomical_paschal_full_moon(2151),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            astronomical_paschal_full_moon(1582),
            Err(Refusal::OutOfRange)
        );
    }

    /// *Spes non confundit*, 6: the Holy Door of St Peter's opened on
    /// 24 December 2024 and closed on 6 January 2026, and the jubilee ran in
    /// the dioceses from 29 December 2024 to 28 December 2025.
    #[test]
    fn the_jubilee_of_2025_is_the_bulls() {
        let line = holy_year_line(day(2025, 6, 1)).expect("carried");
        let row: Vec<&str> = line.trim_end_matches('\n').split('\t').collect();
        assert_eq!(row.len(), HOLY_YEAR_COLUMNS);
        assert_eq!(
            row[..5],
            [
                "within",
                "Ordinary Jubilee of the Year 2025",
                "ordinary",
                "Francis",
                "Spes non confundit"
            ]
        );
        let days = [
            day(2024, 5, 9),
            day(2024, 12, 24),
            day(2026, 1, 6),
            day(2024, 12, 29),
            day(2025, 12, 28),
        ]
        .map(|day| day.to_string());
        assert_eq!(row[5..], days);
        let mercy = holy_year_line(day(2016, 1, 1)).expect("carried");
        assert!(
            mercy.contains("\textraordinary\tFrancis\tMisericordiae vultus\t"),
            "{mercy}"
        );
        assert!(mercy.ends_with("\t\t\n"), "{mercy}");
        assert_eq!(
            holy_year_line(day(2026, 1, 7)).as_deref(),
            Ok("outside\t\t\t\t\t\t\t\t\t\n")
        );
        assert_eq!(holy_year_line(day(1974, 12, 23)), Err(Refusal::NoData));
        assert_eq!(holy_year_line(day(2026, 9, 28)), Err(Refusal::NoData));
    }

    /// Full Fact: St George's Day was kept on Monday 28 April 2025, Easter
    /// being 20 April; Christmas Day is a Principal Feast and Good Friday a
    /// Principal Holy Day.
    #[test]
    fn each_celebration_carries_its_rank() {
        assert_eq!(
            common_worship_lines(day(2025, 4, 28)).as_deref(),
            Ok("George, Martyr, Patron of England\tfestival\tFestival\n")
        );
        assert_eq!(common_worship_lines(day(2025, 4, 23)).as_deref(), Ok(""));
        assert_eq!(
            common_worship_lines(day(2025, 12, 25)).as_deref(),
            Ok("Christmas Day\tprincipal-feast\tPrincipal Feast\n")
        );
        assert_eq!(
            common_worship_lines(day(2025, 4, 18)).as_deref(),
            Ok("Good Friday\tprincipal-holy-day\tPrincipal Holy Day\n")
        );
        assert_eq!(common_worship_lines(i64::MAX), Err(Refusal::OutOfRange));
    }
}
