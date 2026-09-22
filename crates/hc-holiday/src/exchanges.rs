//! The trading calendars of stock exchanges: the days an exchange is
//! closed, and the days it closes early.
//!
//! An exchange's calendar is a [`RuleSet`] like a country's — a weekend, a
//! list of rules, a substitution policy — keyed by the exchange's ISO 10383
//! Market Identifier Code, `XNYS` for the New York Stock Exchange, and found
//! with [`by_code`]. A closed day is a [`Kind::Public`](crate::rule::Kind::Public) entry, so that
//! [`HolidayCalendar::is_holiday`](crate::engine::HolidayCalendar::is_holiday)
//! and business-day arithmetic count it; an early close is a
//! [`Kind::Observance`](crate::rule::Kind::Observance), a day the exchange notes and trades on, so that they
//! do not. The name of an early close says what it is.
//!
//! # Why not the country's table
//!
//! An exchange keeps its own days. The New York Stock Exchange closes on
//! Good Friday, which no United States statute makes a holiday, and trades
//! on Columbus Day and Veterans Day, which the federal government does not;
//! it moves a Saturday holiday to the Friday before, as the federal rule
//! does, but not a Saturday New Year's Day, whose Friday is the last day of
//! the year; and it closes for a hurricane or a day of mourning that no
//! statute foresaw. The Australian Securities Exchange trades on the
//! states' Monday for a Sunday Anzac Day; the Frankfurt Stock Exchange
//! trades on Ascension Day and Corpus Christi and closes on Christmas Eve
//! and New Year's Eve, which Hesse does not. What the exchange publishes
//! is what is carried, and the
//! unscheduled closures a source records are data in the table, as
//! [ADR 0007](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/adr/0007-sets-the-world-can-extend-are-data.md)
//! has it.
//!
//! # Sources
//!
//! Each table names its own, in `sources`, with the date they were read.

use hc_calendar::Weekday;
use hc_calendars_solar::gregorian;

use crate::computus::offsets::{EASTER_MONDAY, GOOD_FRIDAY};
use crate::rule::{
    Days, HolidayRule, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, SubstituteDirection,
    SubstitutionPolicy,
};

// ─────────────────────────────────────────────────────────────────────────
// New York Stock Exchange
// ─────────────────────────────────────────────────────────────────────────

/// A Saturday holiday closes the Friday before and a Sunday holiday the
/// Monday after — except New Year's Day, which carries only the Sunday, as
/// its rule says.
static XNYS_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Nearest,
    skip_occupied: false,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static XNYS_THANKSGIVING: Rule = Rule::nth(11, 4, Weekday::Thursday);

/// The day, if it is a Monday to Thursday: a Friday would be the observed
/// day of the Saturday holiday that follows, and the exchange is closed.
fn if_monday_to_thursday(year: i64, month: u8, day: u8) -> Days {
    let mut out = Days::new();
    let Ok(day) = gregorian::to_fixed(year, month, day) else {
        return out;
    };
    if matches!(
        Weekday::from_rd(day),
        Weekday::Monday | Weekday::Tuesday | Weekday::Wednesday | Weekday::Thursday
    ) {
        out.push(day);
    }
    out
}

/// The day, if it is a Monday to Friday: a trading day at all.
fn if_weekday(year: i64, month: u8, day: u8) -> Days {
    let mut out = Days::new();
    let Ok(day) = gregorian::to_fixed(year, month, day) else {
        return out;
    };
    if !matches!(Weekday::from_rd(day), Weekday::Saturday | Weekday::Sunday) {
        out.push(day);
    }
    out
}

/// 3 July, when the exchange closes at 1:00 p.m. before Independence Day.
fn xnys_july_third(year: i64) -> Days {
    if_monday_to_thursday(year, 7, 3)
}

/// Christmas Eve, when the exchange closes at 1:00 p.m. before Christmas.
fn xnys_christmas_eve(year: i64) -> Days {
    if_monday_to_thursday(year, 12, 24)
}

/// A day the exchange was closed outside its calendar.
const fn xnys_closed(name: &'static str, month: u8, day: u8, year: i32) -> HolidayRule {
    HolidayRule::fixed_public(name, "", Rule::gregorian(month, day)).years(Some(year), Some(year))
}

static XNYS_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1))
        .substitute_on(&[Weekday::Sunday]),
    HolidayRule::public(
        "Martin Luther King, Jr. Day",
        "",
        Rule::nth(1, 3, Weekday::Monday),
    )
    .years(Some(1998), None),
    HolidayRule::public(
        "Washington's Birthday",
        "",
        Rule::nth(2, 3, Weekday::Monday),
    )
    .years(Some(1971), None),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Memorial Day", "", Rule::last(5, Weekday::Monday)).years(Some(1971), None),
    HolidayRule::public(
        "Juneteenth National Independence Day",
        "",
        Rule::gregorian(6, 19),
    )
    .years(Some(2022), None),
    HolidayRule::public("Independence Day", "", Rule::gregorian(7, 4)),
    HolidayRule::public("Labor Day", "", Rule::nth(9, 1, Weekday::Monday)),
    HolidayRule::public("Thanksgiving Day", "", Rule::nth(11, 4, Weekday::Thursday)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    // Early closes at 1:00 p.m.: trading days, noted and not counted.
    HolidayRule::observance("Early close, 3 July", "", Rule::Computed(xnys_july_third)),
    HolidayRule::observance(
        "Early close, the day after Thanksgiving",
        "",
        Rule::Offset {
            base: &XNYS_THANKSGIVING,
            days: 1,
        },
    ),
    HolidayRule::observance(
        "Early close, Christmas Eve",
        "",
        Rule::Computed(xnys_christmas_eve),
    ),
    // Unscheduled closures a source records.
    xnys_closed("Closed after the September 11 attacks", 9, 11, 2001),
    xnys_closed("Closed after the September 11 attacks", 9, 12, 2001),
    xnys_closed("Closed after the September 11 attacks", 9, 13, 2001),
    xnys_closed("Closed after the September 11 attacks", 9, 14, 2001),
    xnys_closed("Closed for Hurricane Sandy", 10, 29, 2012),
    xnys_closed("Closed for Hurricane Sandy", 10, 30, 2012),
    xnys_closed(
        "National Day of Mourning for President George H. W. Bush",
        12,
        5,
        2018,
    ),
];

/// The New York Stock Exchange.
///
/// The exchange's own calendar, "Holidays & Trading Hours", for 2026 to
/// 2028: nine or ten closed days a year — New Year's Day, Martin Luther
/// King, Jr. Day, Washington's Birthday, Good Friday, Memorial Day,
/// Juneteenth, Independence Day, Labor Day, Thanksgiving and Christmas —
/// with a Sunday holiday closing the Monday and a Saturday one the Friday
/// before, and, as the calendar notes for 2028, no closing for a Saturday
/// New Year's Day. The early closes at 1:00 p.m. it lists are the day
/// after Thanksgiving every year, 3 July when that is a Monday to Thursday
/// (2028, not 2026, when the Friday is the observed holiday, nor 2027, when
/// it is a Saturday), and Christmas Eve on the same terms (2026 only of the
/// three). Juneteenth is carried from 2022, when the exchange first
/// observed it, on Monday 20 June; Martin Luther King, Jr. Day from 1998,
/// the year generally reported, which no page read here states; the Monday
/// forms of Washington's Birthday and Memorial Day from 1971, as the
/// statute that made them so.
/// The closures of 11 to 14 September 2001, 29 and 30 October 2012 and
/// 5 December 2018 are the ones the sources read record; the exchange has
/// closed on other days — the days of mourning for Presidents Nixon,
/// Reagan, Ford and Carter among them — that no page read here gives, and
/// they are not carried.
pub static NEW_YORK_STOCK_EXCHANGE: RuleSet = RuleSet {
    code: "XNYS",
    english_name: "New York Stock Exchange",
    rules: XNYS_RULES,
    substitution: XNYS_SUBSTITUTION,
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "NYSE, \"Holidays & Trading Hours\" (nyse.com/markets/hours-calendars), retrieved \
              2026-09-22, for the closed days and early closings of 2026 to 2028 and the note \
              that no holiday is observed for Saturday, 1 January 2028; Wikipedia, \"New York \
              Stock Exchange\", for the closures after the September 11 attacks and for \
              Hurricane Sandy; Wikipedia, \"Death and state funeral of George H. W. Bush\", \
              for 5 December 2018; Wikipedia, \"Trading day\", for Juneteenth's first \
              observance as a market holiday on 20 June 2022",
};

// ─────────────────────────────────────────────────────────────────────────
// Australian Securities Exchange
// ─────────────────────────────────────────────────────────────────────────

/// "When public holidays fall on weekends, ASX observes substitute days
/// on the following business day" — Anzac Day excepted, which its rule
/// says.
static XASX_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// 24 December, when the market closes at 14:10 Sydney time.
fn xasx_christmas_eve(year: i64) -> Days {
    if_weekday(year, 12, 24)
}

/// 31 December, when the market closes at 14:10 Sydney time.
fn xasx_new_years_eve(year: i64) -> Days {
    if_weekday(year, 12, 31)
}

static XASX_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Australia Day", "", Rule::gregorian(1, 26)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    // Not substituted: in 2027 the market trades on Monday 26 April, the
    // states' substitute for the Sunday.
    HolidayRule::fixed_public("Anzac Day", "", Rule::gregorian(4, 25)),
    HolidayRule::fixed_public("Queen's Birthday", "", Rule::nth(6, 2, Weekday::Monday))
        .years(None, Some(2022)),
    HolidayRule::fixed_public("King's Birthday", "", Rule::nth(6, 2, Weekday::Monday))
        .years(Some(2023), None),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
    HolidayRule::observance(
        "Early close, Christmas Eve",
        "",
        Rule::Computed(xasx_christmas_eve),
    ),
    HolidayRule::observance(
        "Early close, New Year's Eve",
        "",
        Rule::Computed(xasx_new_years_eve),
    ),
];

/// The Australian Securities Exchange.
///
/// The exchange's own trading calendar for 2026 and 2027: New Year's Day,
/// Australia Day, Good Friday, Easter Monday, Anzac Day, the King's
/// Birthday on the second Monday of June, Christmas and Boxing Day, with a
/// weekend holiday observed "on the following business day" — Christmas
/// 2027, a Saturday, on the Monday and Boxing Day on the Tuesday — except
/// Anzac Day, which the calendar shows the market open for on Monday
/// 26 April 2027, the states' substitute for the Sunday, and closed for on
/// Saturday 25 April 2026, a day it would not have traded anyway. The
/// market closes at 14:10 Sydney time on 24 and 31 December when those
/// are trading days. The sovereign's birthday takes the sovereign's name,
/// the Queen's to 2022 and the King's from 2023.
pub static AUSTRALIAN_SECURITIES_EXCHANGE: RuleSet = RuleSet {
    code: "XASX",
    english_name: "Australian Securities Exchange",
    rules: XASX_RULES,
    substitution: XASX_SUBSTITUTION,
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "ASX, \"ASX Trade trading calendar\" (asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar), \
              retrieved 2026-09-23, for the closed days, the early closes and the weekend rule \
              of 2026 and 2027",
};

// ─────────────────────────────────────────────────────────────────────────
// Frankfurt Stock Exchange (Xetra)
// ─────────────────────────────────────────────────────────────────────────

static XETR_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Neujahr", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Good Friday", "Karfreitag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "Ostermontag", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Labour Day", "Tag der Arbeit", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Christmas Eve", "Heiligabend", Rule::gregorian(12, 24)),
    HolidayRule::fixed_public("Christmas Day", "1. Weihnachtstag", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("Boxing Day", "2. Weihnachtstag", Rule::gregorian(12, 26)),
    HolidayRule::fixed_public("New Year's Eve", "Silvester", Rule::gregorian(12, 31)),
];

/// The Frankfurt Stock Exchange, on Xetra.
///
/// Deutsche Börse's trading calendar, published to 2032: no trading on
/// New Year's Day, Good Friday, Easter Monday, 1 May, Christmas Eve,
/// Christmas Day, Boxing Day and New Year's Eve, every year, and no day
/// moved when one of them falls on a weekend; "regular stock exchange
/// trading takes place" on Ascension Day, Whit Monday and Corpus Christi,
/// public holidays in Hesse. No early closes.
pub static FRANKFURT_STOCK_EXCHANGE: RuleSet = RuleSet {
    code: "XETR",
    english_name: "Frankfurt Stock Exchange (Xetra)",
    rules: XETR_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Deutsche Börse, \"Trading calendar and trading hours\" \
              (cashmarket.deutsche-boerse.com/cash-en/trading/trading-calendar-and-trading-hours), \
              retrieved 2026-09-23, for the non-trading days to 2032 and the public holidays \
              traded on",
};

// ─────────────────────────────────────────────────────────────────────────
// Toronto Stock Exchange
// ─────────────────────────────────────────────────────────────────────────

/// A weekend holiday closes the next business day: Boxing Day 2026, a
/// Saturday after a Friday Christmas, closes Monday the 28th "in lieu".
static XTSE_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Christmas Eve, when the exchange closes at 1:00 p.m.
fn xtse_christmas_eve(year: i64) -> Days {
    if_weekday(year, 12, 24)
}

static XTSE_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Family Day", "", Rule::nth(2, 3, Weekday::Monday))
        .years(Some(2008), None),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public(
        "Victoria Day",
        "",
        Rule::WeekdayOnOrBefore {
            month: 5,
            day: 24,
            weekday: Weekday::Monday,
        },
    ),
    HolidayRule::public("Canada Day", "", Rule::gregorian(7, 1)),
    HolidayRule::fixed_public("Civic Holiday", "", Rule::nth(8, 1, Weekday::Monday)),
    HolidayRule::fixed_public("Labour Day", "", Rule::nth(9, 1, Weekday::Monday)),
    HolidayRule::fixed_public("Thanksgiving Day", "", Rule::nth(10, 2, Weekday::Monday)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
    HolidayRule::observance(
        "Early close, Christmas Eve",
        "",
        Rule::Computed(xtse_christmas_eve),
    ),
];

/// The Toronto Stock Exchange.
///
/// The exchange's own calendar for 2025 and 2026: New Year's Day, Family
/// Day on the third Monday of February, Good Friday, Victoria Day on the
/// Monday before 25 May, Canada Day, the Civic Holiday on the first Monday
/// of August, Labour Day, Thanksgiving on the second Monday of October,
/// Christmas and Boxing Day, with a weekend holiday closing the next
/// business day, as Boxing Day 2026 does on Monday the 28th; and the
/// 1:00 p.m. close on Christmas Eve when that is a trading day. Family
/// Day is carried from 2008, Ontario's first. The United States holidays
/// the calendar lists for settlement are trading days and are not here.
pub static TORONTO_STOCK_EXCHANGE: RuleSet = RuleSet {
    code: "XTSE",
    english_name: "Toronto Stock Exchange",
    rules: XTSE_RULES,
    substitution: XTSE_SUBSTITUTION,
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "TMX Group, \"Calendar\" (tsx.com/en/trading/calendars-and-trading-hours/calendar), \
              retrieved 2026-09-23, for the closed days and the Christmas Eve close of 2025 \
              and 2026",
};

/// Every exchange calendar, in Market Identifier Code order.
pub static ALL: &[&RuleSet] = &[
    &AUSTRALIAN_SECURITIES_EXCHANGE,
    &FRANKFURT_STOCK_EXCHANGE,
    &NEW_YORK_STOCK_EXCHANGE,
    &TORONTO_STOCK_EXCHANGE,
];

/// The table for an ISO 10383 Market Identifier Code, case-insensitively.
#[must_use]
pub fn by_code(code: &str) -> Option<&'static RuleSet> {
    ALL.iter().copied().find(|exchange| {
        code.len() == exchange.code.len() && code.eq_ignore_ascii_case(exchange.code)
    })
}

hc_core::catalogue_tests! {
    type: &'static RuleSet,
    id: |exchange| exchange.code,
    sorted_by: |exchange| exchange.code,
    provenance: |exchange| exchange.sources,
    tests: exchange_table_tests,
    all: ALL,
    lookup: by_code,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::Kind;

    #[test]
    fn a_closed_day_stops_work_and_an_early_close_does_not() {
        for rule in ALL.iter().flat_map(|exchange| exchange.rules) {
            let early = rule.name.starts_with("Early close");
            assert_eq!(rule.kind == Kind::Observance, early, "{}", rule.name);
            assert_eq!(rule.kind.is_day_off(), !early, "{}", rule.name);
        }
    }

    #[test]
    fn the_lookup_takes_the_code_in_either_case() {
        assert!(by_code("xnys").is_some());
        assert!(by_code("XNYS").is_some());
        assert!(by_code("xtse").is_some());
        assert!(by_code("XNAS").is_none());
    }
}
