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
//! do not. The name of a partial day says what it is — "Early close, …",
//! "Half trading day, …", "Late open, …" — as the exchange's own calendar
//! has it.
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
//! and New Year's Eve, which Hesse does not; and of Euronext's seven
//! markets, six leave a weekend holiday where it fell and Dublin moves
//! it. What the exchange publishes is what is carried, and the
//! unscheduled closures a source records are data in the table, as
//! [ADR 0007](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/adr/0007-sets-the-world-can-extend-are-data.md)
//! has it.
//!
//! # An exchange on its country's calendar
//!
//! Where an exchange closes on every public holiday and adds a few days of
//! its own — Tokyo on 2 and 3 January and 31 December, Seoul on 1 May and
//! the last weekday of the year, Hong Kong on none, with three half days —
//! its table includes the country's through
//! [`RuleSet::includes`] and lists only what is its own, so that the
//! country's substitute and bridge holidays come along under the country's
//! policies and are never copied.
//!
//! # Sources
//!
//! Each table names its own, in `sources`, with the date they were read.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::computus::offsets::{
    ASCENSION, ASH_WEDNESDAY, CORPUS_CHRISTI, EASTER_MONDAY, GOOD_FRIDAY, HOLY_WEDNESDAY,
    MAUNDY_THURSDAY, SHROVE_MONDAY, SHROVE_TUESDAY, WHIT_MONDAY,
};
use crate::countries::europe::GB_ENGLAND_AND_WALES;
use crate::countries::{
    CHINA, HONG_KONG, JAPAN, MEXICO, NEW_ZEALAND, POLAND, SOUTH_AFRICA, SOUTH_KOREA, TAIWAN,
    UNITED_KINGDOM,
};
use crate::rule::{
    CalendarSystem, Days, HolidayRule, Include, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
    SubstituteDirection, SubstitutionPolicy, WeekendPolicy,
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
    includes: &[],
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

/// Nasdaq.
///
/// The same calendar as the New York Stock Exchange, day for day, on the
/// same rules: Nasdaq's own trading calendar for 2026 lists the ten
/// closed days and the two early closes the NYSE's does, and the
/// unscheduled closures — 11 to 14 September 2001, 29 and 30 October
/// 2012, 5 December 2018 — are recorded for Nasdaq by the sources named.
/// The rule slice is shared, so the two cannot drift apart unnoticed; a
/// day on which they differ would need a slice of its own.
pub static NASDAQ: RuleSet = RuleSet {
    code: "XNAS",
    english_name: "Nasdaq",
    rules: XNYS_RULES,
    substitution: XNYS_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Nasdaq Trader, \"Trading Calendar\" (nasdaqtrader.com/trader.aspx?id=calendar), \
              retrieved 2026-09-23, for 2026; Wikipedia, \"Economic effects of the September 11 \
              attacks\", for the closure to 17 September 2001 (\"The Nasdaq also canceled \
              trading\"); Wikipedia, \"Effects of Hurricane Sandy in New York\", for 29 and \
              30 October 2012 (\"U.S. stock trading was suspended\"); Wikipedia, \"Death and \
              state funeral of George H. W. Bush\", for 5 December 2018",
};

// ─────────────────────────────────────────────────────────────────────────
// Australian Securities Exchange
// ─────────────────────────────────────────────────────────────────────────

/// A holiday on a weekend is observed on the next business day, a later
/// one passing a day already taken. The calendar states no rule; this one
/// is read from the substitutes it lists — "Substitute for Saturday 25
/// December" on Monday 27 December 2027 and "Substitute for Sunday 26
/// December" on the Tuesday — and Anzac Day is excepted, as its rule says.
static XASX_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// The last weekday before Christmas Day: 24 December, or the Friday
/// before a weekend one. Sydney's "Last Business day before Christmas
/// Day" and London's "Christmas Holiday half day".
fn last_weekday_before_christmas(year: i64) -> Days {
    last_weekday_on_or_before(year, 12, 24)
}

/// The last weekday of the year: 31 December, or the Friday before a
/// weekend one. Sydney's "Last Business day of the Year" and London's
/// "New Year's Holiday half day".
fn last_weekday_of_the_year(year: i64) -> Days {
    last_weekday_on_or_before(year, 12, 31)
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
        "Early close, the last business day before Christmas Day",
        "",
        Rule::Computed(last_weekday_before_christmas),
    ),
    HolidayRule::observance(
        "Early close, the last business day of the year",
        "",
        Rule::Computed(last_weekday_of_the_year),
    ),
];

/// The Australian Securities Exchange.
///
/// The exchange's own trading calendar for 2026 and 2027: New Year's Day,
/// Australia Day, Good Friday, Easter Monday, Anzac Day, the King's
/// Birthday on the second Monday of June, Christmas and Boxing Day, with a
/// weekend holiday observed on the next business day — Christmas 2027, a
/// Saturday, on the Monday and Boxing Day on the Tuesday — except
/// Anzac Day, which the calendar shows the market open for on Monday
/// 26 April 2027, the states' substitute for the Sunday, and closed for on
/// Saturday 25 April 2026, a day it would not have traded anyway. The
/// market closes at 14:10 Sydney time on the "Last Business day before
/// Christmas Day" and the "Last Business day of the Year", as the calendar
/// names them: 24 and 31 December when those are weekdays, and the Friday
/// before when they are not. The sovereign's birthday takes the sovereign's name,
/// the Queen's to 2022 and the King's from 2023.
pub static AUSTRALIAN_SECURITIES_EXCHANGE: RuleSet = RuleSet {
    code: "XASX",
    english_name: "Australian Securities Exchange",
    rules: XASX_RULES,
    substitution: XASX_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "ASX, \"Trading calendar\" (asx.com.au/markets/market-resources/trading-hours-calendar/cash-market-trading-hours/trading-calendar), \
              retrieved 2026-09-23, for the closed days, the early closes and the weekend rule \
              of 2026 and 2027",
};

// ─────────────────────────────────────────────────────────────────────────
// London Stock Exchange
// ─────────────────────────────────────────────────────────────────────────

static XLON_RULES: &[HolidayRule] = &[
    HolidayRule::observance(
        "Early close, Christmas Holiday half day",
        "",
        Rule::Computed(last_weekday_before_christmas),
    ),
    HolidayRule::observance(
        "Early close, New Year's Holiday half day",
        "",
        Rule::Computed(last_weekday_of_the_year),
    ),
];

/// The London Stock Exchange.
///
/// Its business-days page: the Exchange "generally operates its Trading
/// Services each weekday" and "recognise\[s\] the Public and Bank Holidays
/// of England & Wales", which are the [`UNITED_KINGDOM`] table's days for
/// England and Wales — with their substitutes, Boxing Day 2026 on Monday
/// 28 December — included here and not repeated. Two days a year are half
/// days, on which the "markets closing process commences from 12:30 London
/// time": the "Christmas Holiday half day", the last weekday before
/// Christmas Day, and the "New Year's Holiday half day", the last weekday
/// of the year — Friday 22 and Friday 29 December in 2028, when the 24th
/// and the 31st are Sundays.
pub static LONDON_STOCK_EXCHANGE: RuleSet = RuleSet {
    code: "XLON",
    english_name: "London Stock Exchange",
    rules: XLON_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[Include::in_region(&UNITED_KINGDOM, GB_ENGLAND_AND_WALES)],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "London Stock Exchange, \"Business days\" \
              (londonstockexchange.com/equities-trading/business-days), retrieved 2026-09-23: \
              the statement of which holidays the Exchange recognises, and its table of bank \
              holidays and half days from August 2026 to January 2029",
};

// ─────────────────────────────────────────────────────────────────────────
// SIX Swiss Exchange
// ─────────────────────────────────────────────────────────────────────────

static XSWX_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Neujahr", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Berchtholdstag", "Berchtoldstag", Rule::gregorian(1, 2)),
    HolidayRule::fixed_public("Good Friday", "Karfreitag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "Ostermontag", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Labour Day", "Tag der Arbeit", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Ascension Day", "Auffahrt", Rule::easter(ASCENSION)),
    HolidayRule::fixed_public("Whitmonday", "Pfingstmontag", Rule::easter(WHIT_MONDAY)),
    HolidayRule::fixed_public("Christmas Eve", "Heiligabend", Rule::gregorian(12, 24)),
    HolidayRule::fixed_public("Christmas", "Weihnachten", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("New Year's Eve", "Silvester", Rule::gregorian(12, 31)),
];

/// The SIX Swiss Exchange.
///
/// Its "Market Holidays of the Swiss Stock Exchange" for 2026 and 2027:
/// New Year's Day and Berchtholdstag, Good Friday and Easter Monday,
/// Labour Day, Ascension Day, Whitmonday, Christmas Eve, Christmas and New
/// Year's Eve, each on its day when that is a weekday — 2 January 2027,
/// 1 May 2027 and Christmas 2027, Saturdays, are not listed and move to
/// no other day. No early closes.
///
/// Swiss National Day, 1 August, and St Stephen's Day, 26 December, fall
/// on a Saturday or a Sunday in both years the calendar covers, so it
/// shows neither a closure nor a trading day for them; this table carries
/// neither, and a weekday 1 August or 26 December is answered as a trading
/// day on no evidence either way.
pub static SIX_SWISS_EXCHANGE: RuleSet = RuleSet {
    code: "XSWX",
    english_name: "SIX Swiss Exchange",
    rules: XSWX_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "SIX, \"Trading & Currency Holiday Calendar\" \
              (six-group.com/en/market-data/news-tools/trading-currency-holiday-calendar.html), \
              retrieved 2026-09-23: the market holidays of the Swiss Stock Exchange for 2026 and \
              2027",
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
    includes: &[],
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
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "TMX Group, \"Calendar\" (tsx.com/en/trading/calendars-and-trading-hours/calendar), \
              retrieved 2026-09-23, for the closed days and the Christmas Eve close of 2025 \
              and 2026",
};

// ─────────────────────────────────────────────────────────────────────────
// Euronext: Amsterdam, Brussels, Dublin, Lisbon, Milan, Oslo, Paris
// ─────────────────────────────────────────────────────────────────────────

/// The last weekday on or before a date: the day itself, or the Friday
/// before a Saturday or Sunday.
fn last_weekday_on_or_before(year: i64, month: u8, day: u8) -> Days {
    let mut out = Days::new();
    let Ok(day) = gregorian::to_fixed(year, month, day) else {
        return out;
    };
    let back = match Weekday::from_rd(day) {
        Weekday::Saturday => 1,
        Weekday::Sunday => 2,
        _ => 0,
    };
    out.push(Rd(day.0 - back));
    out
}

/// Christmas Eve, a half trading day when it is a weekday.
fn euronext_christmas_eve(year: i64) -> Days {
    if_weekday(year, 12, 24)
}

/// New Year's Eve, a half trading day when it is a weekday.
fn euronext_new_years_eve(year: i64) -> Days {
    if_weekday(year, 12, 31)
}

/// Dublin's half day before Christmas: Christmas Eve, or the Friday
/// before a weekend one.
fn xdub_before_christmas(year: i64) -> Days {
    last_weekday_on_or_before(year, 12, 24)
}

/// Dublin's half day before the New Year: New Year's Eve, or the Friday
/// before a weekend one.
fn xdub_before_new_year(year: i64) -> Days {
    last_weekday_on_or_before(year, 12, 31)
}

const EURONEXT_NEW_YEARS_DAY: HolidayRule =
    HolidayRule::fixed_public("New Year's Day", "", Rule::gregorian(1, 1));
const EURONEXT_GOOD_FRIDAY: HolidayRule =
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY));
const EURONEXT_EASTER_MONDAY: HolidayRule =
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY));
const EURONEXT_LABOUR_DAY: HolidayRule =
    HolidayRule::fixed_public("Labour Day", "", Rule::gregorian(5, 1));
const EURONEXT_CHRISTMAS: HolidayRule =
    HolidayRule::fixed_public("Christmas Day", "", Rule::gregorian(12, 25));
const EURONEXT_BOXING_DAY: HolidayRule =
    HolidayRule::fixed_public("Boxing Day", "", Rule::gregorian(12, 26));
const EURONEXT_CHRISTMAS_EVE_HALF: HolidayRule = HolidayRule::observance(
    "Half trading day, Christmas Eve",
    "",
    Rule::Computed(euronext_christmas_eve),
);
const EURONEXT_NEW_YEARS_EVE_HALF: HolidayRule = HolidayRule::observance(
    "Half trading day, New Year's Eve",
    "",
    Rule::Computed(euronext_new_years_eve),
);

/// The calendar Amsterdam, Brussels, Lisbon and Paris share: six closed
/// days, none moved off a weekend, and the two half days when they are
/// weekdays.
static EURONEXT_CORE_RULES: &[HolidayRule] = &[
    EURONEXT_NEW_YEARS_DAY,
    EURONEXT_GOOD_FRIDAY,
    EURONEXT_EASTER_MONDAY,
    EURONEXT_LABOUR_DAY,
    EURONEXT_CHRISTMAS,
    EURONEXT_BOXING_DAY,
    EURONEXT_CHRISTMAS_EVE_HALF,
    EURONEXT_NEW_YEARS_EVE_HALF,
];

const EURONEXT_SOURCES: &str = "Euronext, \"Trading hours & holidays\" \
    (euronext.com/en/trade/trading-hours-holidays), retrieved 2026-09-23, the tables for \
    2021 to 2026";

/// A market on the calendar the four share.
const fn euronext_core(code: &'static str, english_name: &'static str) -> RuleSet {
    RuleSet {
        code,
        english_name,
        rules: EURONEXT_CORE_RULES,
        substitution: &[],
        bridges: &[],
        includes: &[],
        weekend: SATURDAY_SUNDAY,
        sources_checked: SourceDate::new(2026, 9, 23),
        sources: EURONEXT_SOURCES,
    }
}

/// Euronext Amsterdam, on the calendar Amsterdam, Brussels, Lisbon and Paris share:
/// New Year's Day, Good Friday, Easter Monday, 1 May, Christmas and Boxing
/// Day, closed when they fall on a weekday and not moved when they do not
/// — Monday 3 January 2022 and Monday 28 December 2026 are full trading
/// days — and half trading days on Christmas Eve and New Year's Eve when
/// those are weekdays, which in 2022 and 2023 they were not.
pub static EURONEXT_AMSTERDAM: RuleSet = euronext_core("XAMS", "Euronext Amsterdam");
/// Euronext Brussels, on the calendar Amsterdam, Brussels, Lisbon and Paris share:
/// New Year's Day, Good Friday, Easter Monday, 1 May, Christmas and Boxing
/// Day, closed when they fall on a weekday and not moved when they do not
/// — Monday 3 January 2022 and Monday 28 December 2026 are full trading
/// days — and half trading days on Christmas Eve and New Year's Eve when
/// those are weekdays, which in 2022 and 2023 they were not.
pub static EURONEXT_BRUSSELS: RuleSet = euronext_core("XBRU", "Euronext Brussels");
/// Euronext Lisbon, on the calendar Amsterdam, Brussels, Lisbon and Paris share:
/// New Year's Day, Good Friday, Easter Monday, 1 May, Christmas and Boxing
/// Day, closed when they fall on a weekday and not moved when they do not
/// — Monday 3 January 2022 and Monday 28 December 2026 are full trading
/// days — and half trading days on Christmas Eve and New Year's Eve when
/// those are weekdays, which in 2022 and 2023 they were not.
pub static EURONEXT_LISBON: RuleSet = euronext_core("XLIS", "Euronext Lisbon");
/// Euronext Paris, on the calendar Amsterdam, Brussels, Lisbon and Paris share:
/// New Year's Day, Good Friday, Easter Monday, 1 May, Christmas and Boxing
/// Day, closed when they fall on a weekday and not moved when they do not
/// — Monday 3 January 2022 and Monday 28 December 2026 are full trading
/// days — and half trading days on Christmas Eve and New Year's Eve when
/// those are weekdays, which in 2022 and 2023 they were not.
pub static EURONEXT_PARIS: RuleSet = euronext_core("XPAR", "Euronext Paris");

/// Dublin alone among the Euronext markets moves a weekend holiday to
/// the next business day: Monday 3 January 2022, Tuesday 27 December 2022
/// after a Monday Boxing Day, Monday 28 December 2026.
static XDUB_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static XDUB_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    EURONEXT_GOOD_FRIDAY,
    EURONEXT_EASTER_MONDAY,
    EURONEXT_LABOUR_DAY,
    HolidayRule::fixed_public(
        "Irish May Bank Holiday",
        "",
        Rule::nth(5, 1, Weekday::Monday),
    ),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("St Stephen's Day", "", Rule::gregorian(12, 26)),
    HolidayRule::observance(
        "Half trading day, before Christmas",
        "",
        Rule::Computed(xdub_before_christmas),
    ),
    HolidayRule::observance(
        "Half trading day, before the New Year",
        "",
        Rule::Computed(xdub_before_new_year),
    ),
];

/// Euronext Dublin: the shared six days and the Irish May Bank Holiday,
/// a weekend holiday moved to the next business day, and the half days
/// on the last trading day before Christmas and before the New Year —
/// Friday 23 and 30 December 2022, Friday 22 and 29 December 2023.
pub static EURONEXT_DUBLIN: RuleSet = RuleSet {
    code: "XDUB",
    english_name: "Euronext Dublin",
    rules: XDUB_RULES,
    substitution: XDUB_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: EURONEXT_SOURCES,
};

static XMIL_RULES: &[HolidayRule] = &[
    EURONEXT_NEW_YEARS_DAY,
    EURONEXT_GOOD_FRIDAY,
    EURONEXT_EASTER_MONDAY,
    EURONEXT_LABOUR_DAY,
    HolidayRule::fixed_public("Ferragosto", "", Rule::gregorian(8, 15)),
    HolidayRule::fixed_public("Christmas Eve", "", Rule::gregorian(12, 24)),
    EURONEXT_CHRISTMAS,
    HolidayRule::fixed_public("St Stephen's Day", "", Rule::gregorian(12, 26)),
    HolidayRule::fixed_public("New Year's Eve", "", Rule::gregorian(12, 31)),
];

/// Euronext Milan, Borsa Italiana, in the tables from 2023: the shared
/// six days and Ferragosto, and Christmas Eve and New Year's Eve closed
/// rather than halved; nothing moved off a weekend, and no half days.
pub static EURONEXT_MILAN: RuleSet = RuleSet {
    code: "XMIL",
    english_name: "Euronext Milan (Borsa Italiana)",
    rules: XMIL_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: EURONEXT_SOURCES,
};

static XOSL_RULES: &[HolidayRule] = &[
    EURONEXT_NEW_YEARS_DAY,
    HolidayRule::observance(
        "Half trading day, the Wednesday before Easter",
        "",
        Rule::easter(HOLY_WEDNESDAY),
    ),
    HolidayRule::fixed_public("Maundy Thursday", "", Rule::easter(MAUNDY_THURSDAY)),
    EURONEXT_GOOD_FRIDAY,
    EURONEXT_EASTER_MONDAY,
    EURONEXT_LABOUR_DAY,
    HolidayRule::fixed_public("Constitution Day", "", Rule::gregorian(5, 17)),
    HolidayRule::fixed_public("Ascension Day", "", Rule::easter(ASCENSION)),
    HolidayRule::fixed_public("Whit Monday", "", Rule::easter(WHIT_MONDAY)),
    HolidayRule::fixed_public("Christmas Eve", "", Rule::gregorian(12, 24)),
    EURONEXT_CHRISTMAS,
    EURONEXT_BOXING_DAY,
    HolidayRule::fixed_public("New Year's Eve", "", Rule::gregorian(12, 31)),
];

/// Euronext Oslo, Oslo Børs: the Norwegian days — Maundy Thursday,
/// Constitution Day on 17 May, Ascension Day and Whit Monday besides the
/// shared six — with Christmas Eve and New Year's Eve closed, nothing
/// moved off a weekend, and the one half day on the Wednesday before
/// Easter.
pub static EURONEXT_OSLO: RuleSet = RuleSet {
    code: "XOSL",
    english_name: "Euronext Oslo (Oslo Børs)",
    rules: XOSL_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: EURONEXT_SOURCES,
};

// ─────────────────────────────────────────────────────────────────────────
// B3, São Paulo
// ─────────────────────────────────────────────────────────────────────────

/// The last weekday of the year, when the banks are closed to the public
/// and the exchange does not trade: 31 December, or the Friday before a
/// weekend one.
fn bvmf_last_business_day(year: i64) -> Days {
    last_weekday_on_or_before(year, 12, 31)
}

static BVMF_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "Confraternização Universal",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public("Carnival Monday", "Carnaval", Rule::easter(SHROVE_MONDAY)),
    HolidayRule::fixed_public("Carnival Tuesday", "Carnaval", Rule::easter(SHROVE_TUESDAY)),
    HolidayRule::observance(
        "Late open, Ash Wednesday (trading from 1:00 p.m.)",
        "Quarta-feira de Cinzas",
        Rule::easter(ASH_WEDNESDAY),
    ),
    HolidayRule::fixed_public(
        "Good Friday",
        "Sexta-feira Santa",
        Rule::easter(GOOD_FRIDAY),
    ),
    HolidayRule::fixed_public("Tiradentes Day", "Tiradentes", Rule::gregorian(4, 21)),
    HolidayRule::fixed_public("Labour Day", "Dia do Trabalho", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Corpus Christi",
        "Corpus Christi",
        Rule::easter(CORPUS_CHRISTI),
    ),
    HolidayRule::fixed_public(
        "Independence Day",
        "Independência do Brasil",
        Rule::gregorian(9, 7),
    ),
    HolidayRule::fixed_public(
        "Our Lady of Aparecida",
        "Nossa Senhora Aparecida",
        Rule::gregorian(10, 12),
    ),
    HolidayRule::fixed_public("All Souls' Day", "Finados", Rule::gregorian(11, 2)),
    HolidayRule::fixed_public(
        "Proclamation of the Republic",
        "Proclamação da República",
        Rule::gregorian(11, 15),
    ),
    HolidayRule::fixed_public(
        "Black Consciousness Day",
        "Dia Nacional de Zumbi e da Consciência Negra",
        Rule::gregorian(11, 20),
    )
    .years(Some(2024), None),
    HolidayRule::fixed_public("Christmas Eve", "Véspera de Natal", Rule::gregorian(12, 24)),
    HolidayRule::fixed_public("Christmas Day", "Natal", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public(
        "Last business day of the year",
        "Último dia útil do ano",
        Rule::Computed(bvmf_last_business_day),
    ),
];

/// B3, the São Paulo exchange.
///
/// B3's own market calendar for 2021 to 2026: the days on which "there
/// will be no trading on the equity, private fixed income, fixed income
/// ETF or listed derivatives markets" — the national holidays, Carnival
/// Monday and Tuesday, Christmas Eve, and the last weekday of the year,
/// Friday 30 December in 2022 and Friday 29 December in 2023 — with
/// Black Consciousness Day from 2024, its first year as a national
/// holiday, and no day moved off a weekend: a Saturday Tiradentes Day
/// or Sunday New Year's Day simply passes. Ash Wednesday opens trading at
/// 1:00 p.m. and is carried as an observance, a trading day whose name
/// says what it is. São Paulo's own days, 25 January and 9 July, are
/// trading days and are not here.
pub static B3: RuleSet = RuleSet {
    code: "BVMF",
    english_name: "B3 (São Paulo)",
    rules: BVMF_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "B3, \"Trading calendar\" (b3.com.br/en_us/solutions/platforms/puma-trading-system/for-members-and-traders/trading-calendar/holidays/), \
              retrieved 2026-09-23, the market calendars for 2021 to 2026",
};

// ─────────────────────────────────────────────────────────────────────────
// Hong Kong Exchanges
// ─────────────────────────────────────────────────────────────────────────

/// Lunar New Year's Eve, a half day when it is a weekday.
fn xhkg_lunar_new_years_eve(year: i64) -> Days {
    let mut out = Days::new();
    for new_year in Rule::in_calendar(CalendarSystem::CHINESE, 1, 1)
        .days_in_year(year)
        .as_slice()
    {
        let eve = Rd(new_year.0 - 1);
        if !matches!(Weekday::from_rd(eve), Weekday::Saturday | Weekday::Sunday) {
            out.push(eve);
        }
    }
    out
}

/// Christmas Eve, a half day when it is a weekday.
fn xhkg_christmas_eve(year: i64) -> Days {
    if_weekday(year, 12, 24)
}

/// New Year's Eve, a half day when it is a weekday.
fn xhkg_new_years_eve(year: i64) -> Days {
    if_weekday(year, 12, 31)
}

static XHKG_RULES: &[HolidayRule] = &[
    HolidayRule::observance(
        "Half trading day, Lunar New Year's Eve",
        "",
        Rule::Computed(xhkg_lunar_new_years_eve),
    ),
    HolidayRule::observance(
        "Half trading day, Christmas Eve",
        "",
        Rule::Computed(xhkg_christmas_eve),
    ),
    HolidayRule::observance(
        "Half trading day, New Year's Eve",
        "",
        Rule::Computed(xhkg_new_years_eve),
    ),
];

/// The Stock Exchange of Hong Kong, of HKEX.
///
/// HKEX's calendar feed, read from October 2025 to October 2027: the
/// market is closed on every general holiday of Hong Kong — the feed's
/// "Hong Kong Market is closed" entries are the [`HONG_KONG`] table's
/// days, which this set includes and does not repeat — and trades a
/// half day, the afternoon session closed, on Lunar New Year's Eve,
/// Christmas Eve and New Year's Eve when those are weekdays.
pub static HONG_KONG_EXCHANGES: RuleSet = RuleSet {
    code: "XHKG",
    english_name: "Stock Exchange of Hong Kong (HKEX)",
    rules: XHKG_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[Include::nationwide(&HONG_KONG)],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "HKEX, \"HKEX Calendar\" (hkex.com.hk/News/HKEX-Calendar), retrieved 2026-09-23: \
              the calendar feed's \"Hong Kong Market is closed\" and \"Half-Day Trading Day\" \
              entries from October 2025 to October 2027",
};

// ─────────────────────────────────────────────────────────────────────────
// Japan Exchange Group
// ─────────────────────────────────────────────────────────────────────────

static XJPX_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("Market holiday", "休業日", Rule::gregorian(1, 2)),
    HolidayRule::fixed_public("Market holiday", "休業日", Rule::gregorian(1, 3)),
    HolidayRule::fixed_public("Market holiday", "休業日", Rule::gregorian(12, 31)),
];

/// The Tokyo Stock Exchange, of the Japan Exchange Group.
///
/// JPX's trading calendar for 2026 and 2027: the exchange is closed on
/// Saturdays, Sundays, the national holidays — the [`JAPAN`] table's
/// days, with its substitute and bridge holidays, which this set includes
/// and does not repeat — and on 2 and 3 January and 31 December, the
/// market holidays that are its own. No early closes.
pub static TOKYO_STOCK_EXCHANGE: RuleSet = RuleSet {
    code: "XJPX",
    english_name: "Tokyo Stock Exchange (JPX)",
    rules: XJPX_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[Include::nationwide(&JAPAN)],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "JPX, \"Trading calendar\" (jpx.co.jp/english/corporate/about-jpx/calendar/index.html), \
              retrieved 2026-09-23, the non-business days of 2026 and 2027",
};

// ─────────────────────────────────────────────────────────────────────────
// Shanghai Stock Exchange
// ─────────────────────────────────────────────────────────────────────────

static XSHG_RULES: &[HolidayRule] = &[
    // The eve of the Spring Festival 2024, a working day in the State
    // Council's arrangement and a day the exchange's notice closes.
    HolidayRule::fixed_public("Chinese New Year's Eve", "除夕", Rule::gregorian(2, 9))
        .years(Some(2024), Some(2024)),
];

/// The Shanghai Stock Exchange.
///
/// The exchange's annual closure notices for 2014 to 2026: it is closed on
/// Saturdays, Sundays and the days off of the State Council's arrangement
/// for each year — the [`CHINA`] table's, which this set includes and does
/// not repeat, with the three later notices that changed 2015, 2019 and
/// 2020 — and not on the weekend days that arrangement makes working days,
/// which the notices list as "周末休市". The one day of its own is
/// 9 February 2024, the eve of the Spring Festival, which the arrangement
/// left a working day. A year the [`CHINA`] table has no arrangement for is
/// a gap here too.
pub static SHANGHAI_STOCK_EXCHANGE: RuleSet = RuleSet {
    code: "XSHG",
    english_name: "Shanghai Stock Exchange",
    rules: XSHG_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[Include::nationwide(&CHINA)],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "上海证券交易所, 关于上海证券交易所2014年 to 2026年全年（部分节假日）休市安排的通知 \
              (sse.com.cn/disclosure/dealinstruc/closed/list/), retrieved 2026-09-23, with \
              上证公告〔2019〕20号 on that year's Labour Day and 上证公告〔2020〕6号 on that \
              year's Spring Festival",
};

// ─────────────────────────────────────────────────────────────────────────
// Taiwan Stock Exchange
// ─────────────────────────────────────────────────────────────────────────

/// The first year the exchange's schedules carried here cover.
const XTAI_FIRST: i64 = 2023;
/// The last.
const XTAI_LAST: i64 = 2026;

/// The days before the Lunar New Year break on which the market does not
/// trade and only settles, as each year's schedule lists them.
static XTAI_SETTLEMENT_ONLY: &[(i64, u8, u8)] = &[
    (2023, 1, 18),
    (2023, 1, 19),
    (2024, 2, 6),
    (2024, 2, 7),
    (2025, 1, 23),
    (2025, 1, 24),
    (2026, 2, 12),
    (2026, 2, 13),
];

fn xtai_settlement_only(year: i64) -> Days {
    let mut out = Days::new();
    for &(y, month, day) in XTAI_SETTLEMENT_ONLY {
        if y == year
            && let Ok(fixed) = gregorian::to_fixed(y, month, day)
        {
            out.push(fixed);
        }
    }
    out
}

static XTAI_RULES: &[HolidayRule] = &[
    // A day off for workers before it was a government holiday in 2026, and
    // one the exchange closed on.
    HolidayRule::fixed_public("Labour Day", "勞動節", Rule::gregorian(5, 1))
        .years(None, Some(2025)),
    HolidayRule::fixed_public(
        "No trading, settlement only",
        "市場無交易，僅辦理結算交割作業",
        Rule::Tabulated {
            function: xtai_settlement_only,
            first_year: XTAI_FIRST,
            last_year: XTAI_LAST,
        },
    ),
];

/// The Taiwan Stock Exchange.
///
/// Its 市場開休市日期 for 2023 to 2026: the market is closed on Saturdays,
/// Sundays and the government's days off — the [`TAIWAN`] table's, which
/// this set includes and does not repeat, and not the Saturdays that table
/// makes working days — on Labour Day before it became a government
/// holiday, and on the two days before the Lunar New Year break when the
/// market only settles, which each year's schedule lists and which are
/// carried as they are listed. A year outside those schedules is a gap.
pub static TAIWAN_STOCK_EXCHANGE: RuleSet = RuleSet {
    code: "XTAI",
    english_name: "Taiwan Stock Exchange",
    rules: XTAI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[Include::nationwide(&TAIWAN)],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "臺灣證券交易所, 市場開休市日期 (twse.com.tw/zh/trading/holiday.html), retrieved \
              2026-09-23, the schedules for 2023 to 2026",
};

// ─────────────────────────────────────────────────────────────────────────
// Korea Exchange
// ─────────────────────────────────────────────────────────────────────────

static XKRX_RULES: &[HolidayRule] = &[
    // 근로자의 날, a paid day off for employees though not a public
    // holiday, until it became the public holiday 노동절 in 2026 and came
    // in through the country's table. Never moved off a weekend.
    HolidayRule::fixed_public("Labour Day", "근로자의 날", Rule::gregorian(5, 1))
        .years(None, Some(2025)),
    HolidayRule::fixed_public(
        "End of Year Holiday",
        "연말 휴장일",
        Rule::Computed(last_weekday_of_the_year),
    ),
];

/// The Korea Exchange, for its securities market.
///
/// KRX's closure lists for 2009 to 2030: the exchange is closed on
/// Saturdays, Sundays and every public holiday — the [`SOUTH_KOREA`]
/// table's days, with its substitute holidays, election days and the days
/// the government designated, which this set includes and does not
/// repeat — and on two days of its own: 1 May, Labour Day, which was a day
/// off for employees before it became a public holiday in 2026, and the
/// last weekday of the year, the "End of Year Holiday" — Friday 29
/// December in 2023, when the 30th and 31st are a weekend. The lists for
/// the years ahead carry only the days already set, so an election or a
/// designated day in those years appears when it is decided.
pub static KOREA_EXCHANGE: RuleSet = RuleSet {
    code: "XKRX",
    english_name: "Korea Exchange",
    rules: XKRX_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[Include::nationwide(&SOUTH_KOREA)],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "KRX, \"Market Closing(Holiday)\" \
              (global.krx.co.kr/contents/GLB/05/0501/0501110000/GLB0501110000.jsp), retrieved \
              2026-09-23, the closed days of each year from 2009 to 2030",
};

// ─────────────────────────────────────────────────────────────────────────
// Nasdaq Nordic: Copenhagen, Stockholm, Helsinki, Iceland
// ─────────────────────────────────────────────────────────────────────────

const NORDIC_SOURCES: &str = "Nasdaq, \"European Markets Trading Hours\" \
    (nasdaqomxnordic.com/tradinghours), retrieved 2026-09-23, the trading calendars for \
    2025 to 2027";

const NORDIC_MAUNDY_THURSDAY: HolidayRule =
    HolidayRule::fixed_public("Maundy Thursday", "", Rule::easter(MAUNDY_THURSDAY));
const NORDIC_ASCENSION: HolidayRule =
    HolidayRule::fixed_public("Ascension Day", "", Rule::easter(ASCENSION));
const NORDIC_WHIT_MONDAY: HolidayRule =
    HolidayRule::fixed_public("Whit Monday", "", Rule::easter(WHIT_MONDAY));
const NORDIC_EPIPHANY: HolidayRule =
    HolidayRule::fixed_public("Epiphany", "", Rule::gregorian(1, 6));
const NORDIC_CHRISTMAS_EVE: HolidayRule =
    HolidayRule::fixed_public("Christmas Eve", "", Rule::gregorian(12, 24));
const NORDIC_NEW_YEARS_EVE: HolidayRule =
    HolidayRule::fixed_public("New Year's Eve", "", Rule::gregorian(12, 31));
/// Midsummer Eve, the Friday between 19 and 25 June, in Sweden and Finland.
const NORDIC_MIDSUMMER_EVE: HolidayRule = HolidayRule::fixed_public(
    "Midsummer Eve",
    "",
    Rule::WeekdayOnOrAfter {
        month: 6,
        day: 19,
        weekday: Weekday::Friday,
    },
);

static XCSE_RULES: &[HolidayRule] = &[
    EURONEXT_NEW_YEARS_DAY,
    NORDIC_MAUNDY_THURSDAY,
    EURONEXT_GOOD_FRIDAY,
    EURONEXT_EASTER_MONDAY,
    NORDIC_ASCENSION,
    HolidayRule::fixed_public("Day after Ascension Day", "", Rule::easter(ASCENSION + 1)),
    HolidayRule::fixed_public("Constitution Day", "Grundlovsdag", Rule::gregorian(6, 5)),
    NORDIC_WHIT_MONDAY,
    NORDIC_CHRISTMAS_EVE,
    EURONEXT_CHRISTMAS,
    EURONEXT_BOXING_DAY,
    NORDIC_NEW_YEARS_EVE,
];

/// Nasdaq Copenhagen.
///
/// The Danish days as Nasdaq's Nordic calendar has them for 2025 to
/// 2027: New Year's Day, Maundy Thursday, Good Friday, Easter Monday,
/// Ascension Day and the Friday after it, Constitution Day on 5 June,
/// Whit Monday, and Christmas Eve to New Year's Eve; no Great Prayer Day
/// since its abolition; nothing moved off a weekend; no half days.
pub static NASDAQ_COPENHAGEN: RuleSet = RuleSet {
    code: "XCSE",
    english_name: "Nasdaq Copenhagen",
    rules: XCSE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: NORDIC_SOURCES,
};

/// 5 January, a half day in Stockholm when it is a weekday.
fn xsto_epiphany_eve(year: i64) -> Days {
    if_weekday(year, 1, 5)
}

/// Walpurgis Night, 30 April, a half day in Stockholm when a weekday.
fn xsto_walpurgis(year: i64) -> Days {
    if_weekday(year, 4, 30)
}

static XSTO_RULES: &[HolidayRule] = &[
    EURONEXT_NEW_YEARS_DAY,
    HolidayRule::observance(
        "Half trading day, Epiphany Eve",
        "",
        Rule::Computed(xsto_epiphany_eve),
    ),
    NORDIC_EPIPHANY,
    HolidayRule::observance(
        "Half trading day, Maundy Thursday",
        "",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    EURONEXT_GOOD_FRIDAY,
    EURONEXT_EASTER_MONDAY,
    HolidayRule::observance(
        "Half trading day, Walpurgis Night",
        "",
        Rule::Computed(xsto_walpurgis),
    ),
    EURONEXT_LABOUR_DAY,
    HolidayRule::observance(
        "Half trading day, the day before Ascension Day",
        "",
        Rule::easter(ASCENSION - 1),
    ),
    NORDIC_ASCENSION,
    HolidayRule::fixed_public(
        "National Day",
        "Sveriges nationaldag",
        Rule::gregorian(6, 6),
    ),
    NORDIC_MIDSUMMER_EVE,
    HolidayRule::observance(
        "Half trading day, All Saints' Eve",
        "",
        Rule::WeekdayOnOrAfter {
            month: 10,
            day: 30,
            weekday: Weekday::Friday,
        },
    ),
    NORDIC_CHRISTMAS_EVE,
    EURONEXT_CHRISTMAS,
    EURONEXT_BOXING_DAY,
    NORDIC_NEW_YEARS_EVE,
];

/// Nasdaq Stockholm.
///
/// The Swedish days as Nasdaq's Nordic calendar has them for 2025 to
/// 2027: New Year's Day, Epiphany, Good Friday, Easter Monday, 1 May,
/// Ascension Day, National Day on 6 June, Midsummer Eve on the Friday
/// between 19 and 25 June, and Christmas Eve to New Year's Eve — and the
/// half days the calendar lists: 5 January, Maundy Thursday, Walpurgis
/// Night on 30 April, the day before Ascension Day, and All Saints' Eve,
/// the Friday between 30 October and 5 November. Nothing moved off a
/// weekend.
pub static NASDAQ_STOCKHOLM: RuleSet = RuleSet {
    code: "XSTO",
    english_name: "Nasdaq Stockholm",
    rules: XSTO_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: NORDIC_SOURCES,
};

static XHEL_RULES: &[HolidayRule] = &[
    EURONEXT_NEW_YEARS_DAY,
    NORDIC_EPIPHANY,
    EURONEXT_GOOD_FRIDAY,
    EURONEXT_EASTER_MONDAY,
    EURONEXT_LABOUR_DAY,
    NORDIC_ASCENSION,
    NORDIC_MIDSUMMER_EVE,
    HolidayRule::fixed_public(
        "Independence Day",
        "Itsenäisyyspäivä",
        Rule::gregorian(12, 6),
    ),
    NORDIC_CHRISTMAS_EVE,
    EURONEXT_CHRISTMAS,
    EURONEXT_BOXING_DAY,
    NORDIC_NEW_YEARS_EVE,
];

/// Nasdaq Helsinki.
///
/// The Finnish days as Nasdaq's Nordic calendar has them for 2025 to
/// 2027: New Year's Day, Epiphany, Good Friday, Easter Monday, 1 May,
/// Ascension Day, Midsummer Eve on the Friday between 19 and 25 June,
/// Independence Day on 6 December, and Christmas Eve to New Year's Eve;
/// nothing moved off a weekend; no half days.
pub static NASDAQ_HELSINKI: RuleSet = RuleSet {
    code: "XHEL",
    english_name: "Nasdaq Helsinki",
    rules: XHEL_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: NORDIC_SOURCES,
};

static XICE_RULES: &[HolidayRule] = &[
    EURONEXT_NEW_YEARS_DAY,
    NORDIC_MAUNDY_THURSDAY,
    EURONEXT_GOOD_FRIDAY,
    EURONEXT_EASTER_MONDAY,
    HolidayRule::fixed_public(
        "First Day of Summer",
        "Sumardagurinn fyrsti",
        Rule::WeekdayOnOrAfter {
            month: 4,
            day: 19,
            weekday: Weekday::Thursday,
        },
    ),
    EURONEXT_LABOUR_DAY,
    NORDIC_ASCENSION,
    NORDIC_WHIT_MONDAY,
    HolidayRule::fixed_public(
        "National Day",
        "Þjóðhátíðardagurinn",
        Rule::gregorian(6, 17),
    ),
    HolidayRule::fixed_public(
        "Commerce Day",
        "Frídagur verslunarmanna",
        Rule::nth(8, 1, Weekday::Monday),
    ),
    NORDIC_CHRISTMAS_EVE,
    EURONEXT_CHRISTMAS,
    EURONEXT_BOXING_DAY,
    NORDIC_NEW_YEARS_EVE,
];

/// Nasdaq Iceland.
///
/// The Icelandic days as Nasdaq's Nordic calendar has them for 2025 to
/// 2027: New Year's Day, Maundy Thursday, Good Friday, Easter Monday,
/// the First Day of Summer on the Thursday between 19 and 25 April, 1 May,
/// Ascension Day, Whit Monday, National Day on 17 June, Commerce Day on
/// the first Monday of August, and Christmas Eve to New Year's Eve;
/// nothing moved off a weekend; no half days.
pub static NASDAQ_ICELAND: RuleSet = RuleSet {
    code: "XICE",
    english_name: "Nasdaq Iceland",
    rules: XICE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: NORDIC_SOURCES,
};

// ─────────────────────────────────────────────────────────────────────────
// Moscow Exchange
// ─────────────────────────────────────────────────────────────────────────

/// The first year the exchange's announcements carried here cover.
const MISX_FIRST: i64 = 2023;
/// The last.
const MISX_LAST: i64 = 2026;

/// The Saturdays the Government's decrees made working days, on which the
/// exchange's announcements say trading "проводятся в обычном режиме".
static MISX_WORKING_SATURDAYS: &[(i64, u8, u8)] =
    &[(2024, 4, 27), (2024, 11, 2), (2024, 12, 28), (2025, 11, 1)];

fn misx_working_saturdays(year: i64) -> Days {
    let mut out = Days::new();
    for &(y, month, day) in MISX_WORKING_SATURDAYS {
        if y == year
            && let Ok(fixed) = gregorian::to_fixed(y, month, day)
        {
            out.push(fixed);
        }
    }
    out
}

/// A day the exchange's announcements for 2023 to 2026 close, in the
/// years given.
const fn misx_closed(
    name: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    from: i32,
    until: i32,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::gregorian(month, day))
        .years(Some(from), Some(until))
}

static MISX_RULES: &[HolidayRule] = &[
    misx_closed("New Year Holidays", "Новогодние каникулы", 1, 1, 2023, 2026),
    misx_closed("New Year Holidays", "Новогодние каникулы", 1, 2, 2023, 2026),
    misx_closed("Orthodox Christmas", "Рождество Христово", 1, 7, 2023, 2026),
    misx_closed(
        "Defender of the Fatherland Day",
        "День защитника Отечества",
        2,
        23,
        2023,
        2025,
    ),
    misx_closed(
        "International Women's Day",
        "Международный женский день",
        3,
        8,
        2023,
        2026,
    ),
    misx_closed(
        "Spring and Labour Day",
        "Праздник Весны и Труда",
        5,
        1,
        2023,
        2025,
    ),
    misx_closed("Victory Day", "День Победы", 5, 9, 2023, 2026),
    misx_closed("Russia Day", "День России", 6, 12, 2023, 2025),
    misx_closed("Unity Day", "День народного единства", 11, 4, 2023, 2025),
    misx_closed(
        "Day off transferred by the Government",
        "Перенесённый выходной день",
        12,
        31,
        2024,
        2026,
    ),
    // 2026: a holiday with only the weekend-day session, whose trades
    // belong to the next trading day. No main session.
    misx_closed(
        "Defender of the Fatherland Day, weekend session only",
        "День защитника Отечества, дополнительная сессия выходного дня",
        2,
        23,
        2026,
        2026,
    ),
    misx_closed(
        "Spring and Labour Day, weekend session only",
        "Праздник Весны и Труда, дополнительная сессия выходного дня",
        5,
        1,
        2026,
        2026,
    ),
    misx_closed(
        "Russia Day, weekend session only",
        "День России, дополнительная сессия выходного дня",
        6,
        12,
        2026,
        2026,
    ),
    misx_closed(
        "Unity Day, weekend session only",
        "День народного единства, дополнительная сессия выходного дня",
        11,
        4,
        2026,
        2026,
    ),
    HolidayRule::workday(
        "Working day, a working Saturday",
        "Рабочая суббота",
        Rule::Tabulated {
            function: misx_working_saturdays,
            first_year: MISX_FIRST,
            last_year: MISX_LAST,
        },
    ),
];

/// The Moscow Exchange, for its equity market.
///
/// The exchange's announcements for 2023 to 2026, and its trading
/// calendar for the equity market for 2025 and 2026: the exchange does
/// not close on every one of Russia's days off. It closes on the holiday dates
/// themselves — 1, 2 and 7 January, 23 February, 8 March, 1 and 9 May,
/// 12 June and 4 November — and, from 2024, on 31 December, a day off the
/// Government transferred there each year; and it trades on the other
/// days off: the New Year holidays of 3 to 6 and 8 January, the days off
/// the decrees transferred and those carried over from a holiday on a
/// weekend — 24 February, 8 May and 6 November 2023, 29 and 30 April, 10 May
/// and 30 December 2024, 2 and 8 May, 13 June and 3 November 2025, 9 January,
/// 9 March and 11 May 2026. It trades too on the Saturdays the decrees
/// made working days, "в обычном режиме", which are carried here as
/// working days, since the [`RUSSIA`](crate::countries::RUSSIA) table's
/// are not lent to an including table and this one includes none.
///
/// From March 2025 the exchange also holds an additional weekend-day
/// session on most Saturdays and Sundays, and in 2026 it holds that session, and no
/// main session, on 23 February, 1 May, 12 June and 4 November; its
/// announcement says those sessions' trades are "частью следующего за
/// выходным торгового дня", part of the next trading day. They are not
/// trading days of their own here: the weekend stays Saturday and Sunday,
/// and the four 2026 holidays are closures whose names say that the
/// weekend session is held. A year outside 2023 to 2026 is a gap: the
/// exchange sets its days each year, and has set them differently in
/// each of the four.
pub static MOSCOW_EXCHANGE: RuleSet = RuleSet {
    code: "MISX",
    english_name: "Moscow Exchange",
    rules: MISX_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Московская биржа, «Расписание торгов на Московской бирже в праздничные дни» \
              for 2023 (moex.com/n51887, 3 October 2022), 2024 (moex.com/n64121, \
              20 September 2023, with moex.com/n69129 of 22 April 2024 for May and \
              moex.com/n75066 of 24 December 2024 for 31 December), 2025 \
              (moex.com/n73701, 2 October 2024, with moex.com/n94472 of 16 October 2025 \
              for 1 November) and 2026 (moex.com/n94172, 6 October 2025, and \
              moex.com/n96571, 30 December 2025, for the holiday weekend sessions); and \
              the exchange's «Торговый календарь» for the equity market \
              (moex.com/ru/tradingcalendar), for 2025 and 2026; all retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Johannesburg Stock Exchange
// ─────────────────────────────────────────────────────────────────────────

/// The days the exchange's December schedules close at 12:00, as each
/// year's schedule lists them.
static XJSE_EARLY_CLOSES: &[(i64, u8, u8)] = &[
    (2023, 12, 22),
    (2023, 12, 29),
    (2024, 12, 24),
    (2024, 12, 31),
    (2025, 12, 24),
    (2025, 12, 31),
];

fn xjse_early_closes(year: i64) -> Days {
    listed_days(XJSE_EARLY_CLOSES, year)
}

static XJSE_RULES: &[HolidayRule] = &[
    // Declared under section 2A of the Public Holidays Act, each for its
    // year, and closed on as the exchange's notices say.
    HolidayRule::fixed_public(
        "Public holiday declared by the President",
        "",
        Rule::gregorian(12, 15),
    )
    .years(Some(2023), Some(2023)),
    HolidayRule::fixed_public("General election day", "", Rule::gregorian(5, 29))
        .years(Some(2024), Some(2024)),
    HolidayRule::fixed_public("Local government election day", "", Rule::gregorian(11, 4))
        .years(Some(2026), Some(2026)),
    HolidayRule::observance(
        "Early close, 12:00",
        "",
        Rule::Tabulated {
            function: xjse_early_closes,
            first_year: 2023,
            last_year: 2025,
        },
    ),
];

/// The Johannesburg Stock Exchange.
///
/// The JSE's markets calendars for 2024, 2025 and 2026 shade the public
/// holidays of South Africa — the [`SOUTH_AFRICA`] table's days, with a
/// Sunday holiday moved to the Monday under section 2(1) of the Public
/// Holidays Act, as Youth Day 2024, Freedom Day 2025 and Women's Day 2026
/// are, and a Saturday one left where it falls, which this set includes
/// and does not repeat — and its notices close "all JSE Markets" on the
/// days the President declared: 15 December 2023, the general election of
/// 29 May 2024 and the local government elections of 4 November 2026,
/// which are carried for their years. It keeps no closed day of its own.
///
/// The early closes, at 12:00, are announced each December in the
/// exchange's trading, clearing and settlement schedule, and are carried
/// as those for 2023 to 2025 list them — 24 and 31 December, or the Friday
/// before when those are Sundays, as in 2023. Any other year's early
/// closes are a gap, 2026's among them until its schedule is published.
pub static JOHANNESBURG_STOCK_EXCHANGE: RuleSet = RuleSet {
    code: "XJSE",
    english_name: "Johannesburg Stock Exchange",
    rules: XJSE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[Include::nationwide(&SOUTH_AFRICA)],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "JSE, markets calendars for 2024 (Market Notice 061/2024, updated), 2025 \
              (Market Notice 305/2024) and 2026 (Market Notice 380/2025) \
              (clientportal.jse.co.za/reports/trading-calendars); JSE Service Hotlines \
              169/2023, 057/2024 and Market Notice 328/2026 for the declared public holidays of \
              15 December 2023, 29 May 2024 and 4 November 2026; JSE Service Hotlines 159/2023, \
              150/2024 and 130/2025, the December schedules, for the early closes; all \
              retrieved 2026-09-23, read in a browser, the site refusing automated access",
};

// ─────────────────────────────────────────────────────────────────────────
// Bolsa Mexicana de Valores
// ─────────────────────────────────────────────────────────────────────────

static XMEX_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "Holy Thursday",
        "Jueves Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Day of the Dead", "Día de Muertos", Rule::gregorian(11, 2)),
    HolidayRule::fixed_public(
        "Bank Employees' Day",
        "Día del Empleado Bancario",
        Rule::gregorian(12, 12),
    ),
];

/// The Bolsa Mexicana de Valores.
///
/// The exchange's "Días no laborables BMV" for 2019 to 2026, and for 2023
/// to 2026 the CNBV's annual list, published in the Diario Oficial de la
/// Federación, of the days on which the stock exchanges among other
/// entities "deberán cerrar sus puertas… y suspender operaciones": the
/// days of rest of the Ley Federal del Trabajo — the [`MEXICO`] table's,
/// with the Mondays for 5 February, 21 March and 20 November and the
/// presidential handover of 1 October 2024, which this set includes and
/// does not repeat — and four more, Holy Thursday, Good Friday, 2 November
/// and 12 December, every year. Nothing moves off a weekend. No early
/// closes. The CNBV may order other closures "por razones de seguridad
/// nacional o de interés público"; none is in the years read.
pub static BOLSA_MEXICANA_DE_VALORES: RuleSet = RuleSet {
    code: "XMEX",
    english_name: "Bolsa Mexicana de Valores",
    rules: XMEX_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[Include::nationwide(&MEXICO)],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "BMV, \"Calendario de días festivos\" \
              (bmv.com.mx/es/grupo-bmv/calendario-de-dias-festivos), the lists for 2019 to 2026, \
              read in web.archive.org copies, the site refusing connections; CNBV, \
              Disposiciones de carácter general que señalan los días del año en que las \
              entidades financieras sujetas a su supervisión deberán cerrar sus puertas, \
              Diario Oficial de la Federación of 28 November 2022, 12 December 2023 and \
              27 December 2024 (web.archive.org copies of dof.gob.mx) and of 10 December 2025 \
              (the copy the Asociación de Bancos de México hosts, abm.org.mx); all retrieved \
              2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Tel Aviv Stock Exchange
// ─────────────────────────────────────────────────────────────────────────

/// Sunday to Thursday until the end of 2025, Monday to Friday from
/// 5 January 2026.
static XTAE_WEEKEND: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        valid_from: None,
        valid_until: Some(2025),
    },
    WeekendPolicy {
        days: &[Weekday::Saturday, Weekday::Sunday],
        valid_from: Some(2026),
        valid_until: None,
    },
];

/// The first year of the exchange's vacation schedules carried here.
const XTAE_FIRST: i64 = 2024;
/// The last.
const XTAE_LAST: i64 = 2027;

/// The exchange's vacation schedules for 2024 to 2027, as (year, month,
/// day, which of the rules below). Clearing-only rows on days the market
/// does not trade anyway are kept as listed.
#[rustfmt::skip]
static XTAE_DAYS: &[(i64, u8, u8, u8)] = &[
    (2024, 3, 24, 0), (2024, 4, 22, 1), (2024, 4, 23, 2), (2024, 4, 28, 3), (2024, 4, 29, 4),
    (2024, 5, 13, 5), (2024, 5, 14, 6), (2024, 6, 11, 7), (2024, 6, 12, 8), (2024, 8, 13, 9),
    (2024, 10, 2, 10), (2024, 10, 3, 11), (2024, 10, 4, 12), (2024, 10, 11, 13), (2024, 10, 16, 15),
    (2024, 10, 17, 16), (2024, 10, 23, 17), (2024, 10, 24, 18),
    (2024, 4, 24, 21), (2024, 4, 25, 21), (2024, 10, 20, 22), (2024, 10, 21, 22), (2024, 10, 22, 22),
    (2025, 3, 14, 0), (2025, 4, 13, 2), (2025, 4, 18, 3), (2025, 4, 30, 5), (2025, 5, 1, 6),
    (2025, 6, 1, 7), (2025, 6, 2, 8), (2025, 8, 3, 9), (2025, 9, 22, 10), (2025, 9, 23, 11),
    (2025, 9, 24, 12), (2025, 10, 1, 13), (2025, 10, 2, 14), (2025, 10, 6, 15), (2025, 10, 7, 16),
    (2025, 10, 13, 17), (2025, 10, 14, 18),
    (2025, 4, 14, 21), (2025, 4, 15, 21), (2025, 4, 16, 21), (2025, 4, 17, 21),
    (2025, 10, 8, 22), (2025, 10, 9, 22), (2025, 10, 12, 22),
    (2026, 1, 2, 23),
    (2026, 3, 3, 0), (2026, 4, 1, 1), (2026, 4, 2, 2), (2026, 4, 7, 3), (2026, 4, 8, 4),
    (2026, 4, 21, 5), (2026, 4, 22, 6), (2026, 5, 21, 7), (2026, 5, 22, 8), (2026, 7, 23, 9),
    (2026, 9, 11, 10), (2026, 9, 13, 12), (2026, 9, 18, 20), (2026, 9, 20, 13), (2026, 9, 21, 14),
    (2026, 9, 25, 15), (2026, 10, 2, 17), (2026, 10, 27, 19),
    (2026, 4, 6, 21), (2026, 9, 28, 22), (2026, 9, 29, 22), (2026, 9, 30, 22), (2026, 10, 1, 22),
    (2027, 3, 23, 0), (2027, 4, 21, 1), (2027, 4, 22, 2), (2027, 4, 27, 3), (2027, 4, 28, 4),
    (2027, 5, 11, 5), (2027, 5, 12, 6), (2027, 6, 10, 7), (2027, 6, 11, 8), (2027, 8, 12, 9),
    (2027, 10, 1, 10), (2027, 10, 3, 12), (2027, 10, 8, 20), (2027, 10, 10, 13), (2027, 10, 11, 14),
    (2027, 10, 15, 15), (2027, 10, 22, 17),
    (2027, 4, 26, 21), (2027, 10, 18, 22), (2027, 10, 19, 22), (2027, 10, 20, 22), (2027, 10, 21, 22),
];

/// The days of one rule in [`XTAE_DAYS`].
fn xtae_days<const RULE: u8>(year: i64) -> Days {
    let mut out = Days::new();
    for &(y, month, day, rule) in XTAE_DAYS {
        if y == year
            && rule == RULE
            && let Ok(fixed) = gregorian::to_fixed(y, month, day)
        {
            out.push(fixed);
        }
    }
    out
}

/// A closure the vacation schedules list.
const fn xtae_closed<const RULE: u8>(name: &'static str, local_name: &'static str) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local_name,
        Rule::Tabulated {
            function: xtae_days::<RULE>,
            first_year: XTAE_FIRST,
            last_year: XTAE_LAST,
        },
    )
}

static XTAE_RULES: &[HolidayRule] = &[
    xtae_closed::<0>("Purim", "פורים"),
    xtae_closed::<1>("Passover Eve", "ערב פסח"),
    xtae_closed::<2>("Passover", "פסח"),
    xtae_closed::<3>("Passover II Eve", "ערב שביעי של פסח"),
    xtae_closed::<4>("Passover II", "שביעי של פסח"),
    xtae_closed::<5>("Memorial Day", "יום הזיכרון"),
    xtae_closed::<6>("Independence Day", "יום העצמאות"),
    xtae_closed::<7>("Shavuot Eve", "ערב שבועות"),
    xtae_closed::<8>("Shavuot", "שבועות"),
    xtae_closed::<9>("Tisha B'Av", "תשעה באב"),
    xtae_closed::<10>("Jewish New Year Eve", "ערב ראש השנה"),
    xtae_closed::<11>("Jewish New Year I", "ראש השנה"),
    xtae_closed::<12>("Jewish New Year II", "ראש השנה"),
    xtae_closed::<13>("Yom Kippur Eve", "ערב יום כיפור"),
    xtae_closed::<14>("Yom Kippur", "יום כיפור"),
    xtae_closed::<15>("Sukkot Eve", "ערב סוכות"),
    xtae_closed::<16>("Sukkot", "סוכות"),
    xtae_closed::<17>("Simchat Torah Eve", "ערב שמחת תורה"),
    xtae_closed::<18>("Simchat Torah", "שמחת תורה"),
    xtae_closed::<19>("Knesset Election Day", "יום הבחירות לכנסת"),
    xtae_closed::<20>("Friday before a holiday or holiday eve on the Sunday", ""),
    HolidayRule::observance(
        "Early close, an interim day of Passover",
        "חול המועד פסח",
        Rule::Tabulated {
            function: xtae_days::<21>,
            first_year: XTAE_FIRST,
            last_year: XTAE_LAST,
        },
    ),
    HolidayRule::observance(
        "Early close, an interim day of Sukkot",
        "חול המועד סוכות",
        Rule::Tabulated {
            function: xtae_days::<22>,
            first_year: XTAE_FIRST,
            last_year: XTAE_LAST,
        },
    ),
    xtae_closed::<23>("Friday before the first Monday-to-Friday week", ""),
];

/// The Tel Aviv Stock Exchange.
///
/// The exchange's vacation schedules for 2024 to 2027. It traded Sunday to
/// Thursday until the end of 2025, and from "the trading week beginning
/// Monday, January 5, 2026" trades Monday to Friday, with no trading on
/// Sunday 4 January; Friday 2 January 2026, the last Friday of the old
/// week, is carried as a closure, which the change implies and no list
/// states. Friday is a shortened day every week under the new schedule,
/// and is not marked.
///
/// The market is closed on each festival and on its eve, on Purim, Memorial
/// Day, Independence Day and Tisha B'Av, on election days, and from 2026 on
/// the Friday before a Sunday that is a festival or its eve — as each
/// year's schedule lists the days, which are carried as listed rather than
/// computed from the Hebrew calendar, since elections and the exchange's
/// arrangements are set each year. The interim days of Passover and Sukkot
/// on which it trades close early, at about 14:30 rather than 17:30, and are
/// early closes here; one that falls on a Friday keeps the Friday hours and
/// is not. The 2028 schedule, which leaves out a Sunday Passover II Eve and
/// its Friday against the exchange's own rule, is not carried; a year
/// outside 2024 to 2027 is a gap.
pub static TEL_AVIV_STOCK_EXCHANGE: RuleSet = RuleSet {
    code: "XTAE",
    english_name: "Tel Aviv Stock Exchange",
    rules: XTAE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: XTAE_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "TASE, \"Trading Vacation Schedule\" \
              (tase.co.il/en/content/knowledge_center/trading_vacation_schedule and the Hebrew \
              page), for 2025 to 2028, and the vacation schedule for 2024 (content.tase.co.il, \
              file_0010_vacation_schedule_2024_heb.pdf, the English file stopping at October); \
              TASE, \"Changing the trading days\" (tase.co.il/en/content/about/tradingdays_change/ \
              and the Hebrew FAQ), for the Monday-to-Friday week from 5 January 2026; all \
              retrieved 2026-09-23, read in a browser",
};

// ─────────────────────────────────────────────────────────────────────────
// Saudi Exchange
// ─────────────────────────────────────────────────────────────────────────

/// Sunday to Thursday, as the exchange's "Trading Days: Sunday to
/// Thursday" has it, since the kingdom's working week moved in June 2013;
/// Saturday to Wednesday before, taken by whole years as the
/// [`SAUDI_ARABIA`](crate::countries::SAUDI_ARABIA) table takes it. Every
/// year before 2023 is a gap here in any case.
static XSAU_WEEKEND: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Thursday, Weekday::Friday],
        valid_from: None,
        valid_until: Some(2012),
    },
    WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        valid_from: Some(2013),
        valid_until: None,
    },
];

/// The trading days the exchange's announcements close, between the last
/// trading day and the day trading resumes, as (year, month, day, which of
/// the rules below).
#[rustfmt::skip]
static XSAU_DAYS: &[(i64, u8, u8, u8)] = &[
    (2023, 2, 22, 0), (2023, 4, 18, 1), (2023, 4, 19, 1), (2023, 4, 20, 1), (2023, 4, 23, 1),
    (2023, 4, 24, 1), (2023, 6, 25, 2), (2023, 6, 26, 2), (2023, 6, 27, 2), (2023, 6, 28, 2),
    (2023, 6, 29, 2), (2023, 9, 24, 3),
    (2024, 2, 22, 0), (2024, 4, 7, 1), (2024, 4, 8, 1), (2024, 4, 9, 1), (2024, 4, 10, 1),
    (2024, 4, 11, 1), (2024, 6, 16, 2), (2024, 6, 17, 2), (2024, 6, 18, 2), (2024, 6, 19, 2),
    (2024, 6, 20, 2), (2024, 9, 23, 3),
    (2025, 2, 23, 0), (2025, 3, 30, 1), (2025, 3, 31, 1), (2025, 4, 1, 1), (2025, 4, 2, 1),
    (2025, 6, 5, 2), (2025, 6, 8, 2), (2025, 6, 9, 2), (2025, 6, 10, 2), (2025, 9, 23, 3),
    (2026, 2, 22, 0), (2026, 3, 17, 1), (2026, 3, 18, 1), (2026, 3, 19, 1), (2026, 3, 22, 1),
    (2026, 3, 23, 1), (2026, 5, 24, 2), (2026, 5, 25, 2), (2026, 5, 26, 2), (2026, 5, 27, 2),
    (2026, 5, 28, 2), (2026, 9, 23, 3),
];

/// The days of one rule in [`XSAU_DAYS`].
fn xsau_days<const RULE: u8>(year: i64) -> Days {
    let mut out = Days::new();
    for &(y, month, day, rule) in XSAU_DAYS {
        if y == year
            && rule == RULE
            && let Ok(fixed) = gregorian::to_fixed(y, month, day)
        {
            out.push(fixed);
        }
    }
    out
}

/// A closure the exchange's announcements list.
const fn xsau_closed<const RULE: u8>(name: &'static str, local_name: &'static str) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local_name,
        Rule::Tabulated {
            function: xsau_days::<RULE>,
            first_year: 2023,
            last_year: 2026,
        },
    )
}

static XSAU_RULES: &[HolidayRule] = &[
    xsau_closed::<0>("Founding Day", "يوم التأسيس"),
    xsau_closed::<1>("Eid al-Fitr holiday", "إجازة عيد الفطر"),
    xsau_closed::<2>("Eid al-Adha holiday", "إجازة عيد الأضحى"),
    xsau_closed::<3>("National Day", "اليوم الوطني"),
];

/// The Saudi Exchange, Tadawul.
///
/// The exchange's holiday announcements for 2023 to 2026: trading
/// "Sunday to Thursday, except official holidays in the kingdom", closed
/// on Founding Day, the Eid al-Fitr and Eid al-Adha holidays and National
/// Day, on the days between the last trading day and the day trading
/// resumes that each announcement gives. The Eid holidays are set each
/// year by the authorities on the moon, and run longer than the country
/// table's statutory days — Eid al-Fitr 2026 closed from Tuesday 17 to
/// Monday 23 March — and Founding Day 2025, a Saturday, closed the Sunday
/// after; so every day is carried as announced, exact, and a year outside
/// 2023 to 2026 is a gap. The exchange's calendar for 2027 to 2029 marks
/// its Eid dates "According to the UMM AL-QURA calendar", a forecast, and is
/// not carried. No early closes; the late open of 19 August 2026 after a
/// technical suspension is not a day the calendar keeps and is not here.
pub static SAUDI_EXCHANGE: RuleSet = RuleSet {
    code: "XSAU",
    english_name: "Saudi Exchange (Tadawul)",
    rules: XSAU_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: XSAU_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Saudi Exchange, \"Trading Cycle and Times\" and \"Saudi Exchange Holiday \
              Calendar\" (saudiexchange.sa), and its holiday announcements for Founding Day, \
              Eid al-Fitr, Eid al-Adha and National Day of 2023 to 2026 (issuer news 7516, \
              7534, 7648, 7856, 8077, 8106, 8220, 8407, 8691, 8703, 8815, 8998, 9259, 9258, 9364 \
              and 9568); all retrieved 2026-09-23, read in a browser, the site refusing \
              automated access",
};

// ─────────────────────────────────────────────────────────────────────────
// Borsa İstanbul
// ─────────────────────────────────────────────────────────────────────────

/// The Bayram days in the exchange's holiday tables for 2019 to 2026, as
/// (year, month, day, which of the rules below): the days closed, and the
/// eves traded to 13:00 when they are weekdays.
#[rustfmt::skip]
static XIST_DAYS: &[(i64, u8, u8, u8)] = &[
    (2019, 6, 4, 0), (2019, 6, 5, 0), (2019, 6, 6, 0), (2019, 6, 3, 2),
    (2019, 8, 11, 1), (2019, 8, 12, 1), (2019, 8, 13, 1), (2019, 8, 14, 1),
    (2020, 5, 24, 0), (2020, 5, 25, 0), (2020, 5, 26, 0),
    (2020, 7, 31, 1), (2020, 8, 1, 1), (2020, 8, 2, 1), (2020, 8, 3, 1), (2020, 7, 30, 3),
    (2021, 5, 13, 0), (2021, 5, 14, 0), (2021, 5, 15, 0), (2021, 5, 12, 2),
    (2021, 7, 20, 1), (2021, 7, 21, 1), (2021, 7, 22, 1), (2021, 7, 23, 1), (2021, 7, 19, 3),
    (2022, 5, 2, 0), (2022, 5, 3, 0), (2022, 5, 4, 0),
    (2022, 7, 9, 1), (2022, 7, 10, 1), (2022, 7, 11, 1), (2022, 7, 12, 1), (2022, 7, 8, 3),
    (2023, 4, 21, 0), (2023, 4, 22, 0), (2023, 4, 23, 0), (2023, 4, 20, 2),
    (2023, 6, 28, 1), (2023, 6, 29, 1), (2023, 6, 30, 1), (2023, 7, 1, 1), (2023, 6, 27, 3),
    (2024, 4, 10, 0), (2024, 4, 11, 0), (2024, 4, 12, 0), (2024, 4, 9, 2),
    (2024, 6, 15, 1), (2024, 6, 16, 1), (2024, 6, 17, 1), (2024, 6, 18, 1), (2024, 6, 19, 1),
    (2025, 3, 29, 0), (2025, 3, 30, 0), (2025, 3, 31, 0), (2025, 4, 1, 0),
    (2025, 6, 6, 1), (2025, 6, 7, 1), (2025, 6, 8, 1), (2025, 6, 9, 1), (2025, 6, 5, 3),
    (2026, 3, 20, 0), (2026, 3, 21, 0), (2026, 3, 22, 0), (2026, 3, 19, 2),
    (2026, 5, 27, 1), (2026, 5, 28, 1), (2026, 5, 29, 1), (2026, 5, 30, 1), (2026, 5, 26, 3),
];

/// The days of one rule in [`XIST_DAYS`].
fn xist_days<const RULE: u8>(year: i64) -> Days {
    let mut out = Days::new();
    for &(y, month, day, rule) in XIST_DAYS {
        if y == year
            && rule == RULE
            && let Ok(fixed) = gregorian::to_fixed(y, month, day)
        {
            out.push(fixed);
        }
    }
    out
}

/// The Bayram days of one rule, over the years the tables carried cover.
const fn xist_bayram<const RULE: u8>() -> Rule {
    Rule::Tabulated {
        function: xist_days::<RULE>,
        first_year: 2019,
        last_year: 2026,
    }
}

/// 28 October, the eve of Republic Day, traded to 13:00 when a weekday.
fn xist_republic_day_eve(year: i64) -> Days {
    if_weekday(year, 10, 28)
}

static XIST_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Yılbaşı", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "National Sovereignty and Children's Day",
        "Ulusal Egemenlik ve Çocuk Bayramı",
        Rule::gregorian(4, 23),
    ),
    HolidayRule::fixed_public(
        "Labour and Solidarity Day",
        "Emek ve Dayanışma Günü",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::fixed_public(
        "Commemoration of Atatürk, Youth and Sports Day",
        "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
        Rule::gregorian(5, 19),
    ),
    HolidayRule::fixed_public(
        "Democracy and National Unity Day",
        "Demokrasi ve Millî Birlik Günü",
        Rule::gregorian(7, 15),
    )
    .years(Some(2017), None),
    HolidayRule::fixed_public("Victory Day", "Zafer Bayramı", Rule::gregorian(8, 30)),
    HolidayRule::observance(
        "Half trading day, the eve of Republic Day (to 13:00)",
        "Cumhuriyet Bayramı arifesi",
        Rule::Computed(xist_republic_day_eve),
    ),
    HolidayRule::fixed_public(
        "Republic Day",
        "Cumhuriyet Bayramı",
        Rule::gregorian(10, 29),
    ),
    HolidayRule::fixed_public("Ramadan Feast", "Ramazan Bayramı", xist_bayram::<0>()),
    HolidayRule::fixed_public(
        "Feast of the Sacrifice",
        "Kurban Bayramı",
        xist_bayram::<1>(),
    ),
    HolidayRule::observance(
        "Half trading day, the eve of the Ramadan Feast (to 13:00)",
        "Ramazan Bayramı arifesi",
        xist_bayram::<2>(),
    ),
    HolidayRule::observance(
        "Half trading day, the eve of the Feast of the Sacrifice (to 13:00)",
        "Kurban Bayramı arifesi",
        xist_bayram::<3>(),
    ),
];

/// Borsa İstanbul, for its equity market.
///
/// The exchange's official holidays for 2019 to 2026: New Year's Day,
/// 23 April, 1 May, 19 May, 15 July from 2017, its first year, 30 August
/// and 29 October, closed when they fall on a weekday and moved nowhere
/// when they do not, with 28 October traded to 13:00; and the Ramadan Feast
/// and the Feast of the Sacrifice on the days each year's table gives,
/// with the eve traded to 13:00 when it is a weekday. The Bayram dates are
/// the Diyanet's, set in advance, and are carried as the tables list them
/// — exact, where the [`TURKEY`](crate::countries::TURKEY) table's
/// computation is approximate — so a year outside 2019 to 2026 is a gap.
/// The days the government adds to a Bayram as administrative leave bind
/// the public sector, and the tables do not close the exchange on them;
/// nor do they record unscheduled suspensions, which are not carried.
pub static BORSA_ISTANBUL: RuleSet = RuleSet {
    code: "XIST",
    english_name: "Borsa İstanbul",
    rules: XIST_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Borsa İstanbul, \"Official Holidays\" (borsaistanbul.com/en/official-holidays, \
              and the Turkish page borsaistanbul.com/resmi-tatil-gunleri), the tables for 2019 to \
              2026, with the \"Equity Market Holiday Schedule\" files for those years; retrieved \
              2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Warsaw Stock Exchange
// ─────────────────────────────────────────────────────────────────────────

static XWAR_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("Good Friday", "Wielki Piątek", Rule::easter(GOOD_FRIDAY)),
    // A public holiday from 2025, and in the country's table from then.
    HolidayRule::fixed_public(
        "Christmas Eve",
        "Wigilia Bożego Narodzenia",
        Rule::gregorian(12, 24),
    )
    .years(None, Some(2024)),
    HolidayRule::fixed_public("New Year's Eve", "Sylwester", Rule::gregorian(12, 31)),
];

/// The Warsaw Stock Exchange, GPW.
///
/// Its "Dni bez sesji" for 2019 to 2027: no session on the public holidays
/// of Poland — the [`POLAND`] table's days, 6 January, Corpus Christi,
/// 15 August, 1 and 11 November among them, which this set includes and
/// does not repeat — and on three days of its own: Good Friday, Christmas
/// Eve, which it kept before the day became a public holiday in 2025, and
/// New Year's Eve. Nothing moves off a weekend. No half days.
pub static WARSAW_STOCK_EXCHANGE: RuleSet = RuleSet {
    code: "XWAR",
    english_name: "Warsaw Stock Exchange (GPW)",
    rules: XWAR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[Include::nationwide(&POLAND)],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "GPW, \"Szczegóły sesji\" — \"Dni bez sesji\" (gpw.pl/szczegoly-sesji, and the \
              English gpw.pl/session-details), for 2025 to 2027, read in a browser; the same \
              pages in web.archive.org copies of 2019 to 2023, for 2019 to 2024; retrieved \
              2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Wiener Börse
// ─────────────────────────────────────────────────────────────────────────

static XWBO_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Neujahr", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Good Friday", "Karfreitag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "Ostermontag", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Labour Day", "Staatsfeiertag", Rule::gregorian(5, 1)),
    // Closed to 2022, traded from 2023.
    HolidayRule::fixed_public("Whit Monday", "Pfingstmontag", Rule::easter(WHIT_MONDAY))
        .years(None, Some(2022)),
    HolidayRule::fixed_public("National Day", "Nationalfeiertag", Rule::gregorian(10, 26)),
    HolidayRule::fixed_public("Christmas Eve", "Heiliger Abend", Rule::gregorian(12, 24)),
    HolidayRule::fixed_public("Christmas Day", "Christtag", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("St Stephen's Day", "Stefanitag", Rule::gregorian(12, 26)),
    HolidayRule::fixed_public("New Year's Eve", "Silvester", Rule::gregorian(12, 31)),
];

/// The Wiener Börse.
///
/// Its "Börsenfeiertage & Feiertagshandel" for 2019 to 2027: the
/// "Handelsfreie Feiertage" — New Year's Day, Good Friday, Easter Monday,
/// 1 May, 26 October, Christmas Eve to St Stephen's Day and New Year's
/// Eve, with Whit Monday until 2022 — each on its day when a weekday and
/// moved nowhere; and the "Zusätzliche Handelstage", the Austrian public
/// holidays it trades on — 6 January, Ascension Day, Corpus Christi,
/// 15 August, 1 November, 8 December, and Whit Monday from 2023 — which
/// this set therefore neither includes nor lists. No early closes. The
/// 2027 list's "Fr, 1. Mai" is a Saturday.
pub static WIENER_BOERSE: RuleSet = RuleSet {
    code: "XWBO",
    english_name: "Wiener Börse",
    rules: XWBO_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Wiener Börse, \"Handelskalender\" \
              (wienerborse.at/handel/handelsinformationen/handelskalender/) and its \
              \"Börsenfeiertage\" files for 2026 and 2027 (wienerborse.at/uploads/u/cms/files/\
              handel/boersenfeiertage-2026-de.pdf and -2027-de.pdf), with web.archive.org copies \
              of the files for 2019 to 2025; retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Bolsa de Madrid
// ─────────────────────────────────────────────────────────────────────────

/// Christmas Eve, traded to 14:00 when a weekday.
fn xmad_christmas_eve(year: i64) -> Days {
    if_weekday(year, 12, 24)
}

/// New Year's Eve, traded to 14:00 when a weekday.
fn xmad_new_years_eve(year: i64) -> Days {
    if_weekday(year, 12, 31)
}

static XMAD_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Lunes de Pascua",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "Fiesta del Trabajo", Rule::gregorian(5, 1)),
    HolidayRule::observance(
        "Early close, Christmas Eve (14:00)",
        "Nochebuena",
        Rule::Computed(xmad_christmas_eve),
    ),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("St Stephen's Day", "San Esteban", Rule::gregorian(12, 26)),
    HolidayRule::observance(
        "Early close, New Year's Eve (14:00)",
        "Nochevieja",
        Rule::Computed(xmad_new_years_eve),
    ),
];

/// The Bolsa de Madrid, on the calendar BME sets for the Spanish equity,
/// fixed-income and derivatives markets.
///
/// BME's session calendars for 2023 to 2026: the days "inhábiles a efectos
/// del funcionamiento" are New Year's Day, Good Friday, Easter Monday,
/// 1 May, Christmas and 26 December, when they fall on a weekday — a
/// weekend one is simply not listed, and nothing moves — and on 24 and
/// 31 December "el mercado permanecerá abierto hasta las 14 horas". The
/// Spanish national holidays of 6 January, 15 August, 12 October,
/// 1 November and 6 and 8 December are in no list and are trading days;
/// the calendar page's FAQ, which names Epiphany among the days the
/// markets close, contradicts every list and is not followed.
pub static BOLSA_DE_MADRID: RuleSet = RuleSet {
    code: "XMAD",
    english_name: "Bolsa de Madrid (BME)",
    rules: XMAD_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "BME, \"Calendario del mercado\" \
              (bolsasymercados.es/es/bme-exchange/negociar/calendario-del-mercado.html), for \
              2026, and its press releases \"Calendario de 2024\", \"Calendario de 2025\" and \
              \"Calendario de 2026 en los mercados financieros españoles\" and \"Calendario 2023 \
              en los mercados de valores españoles\" (bolsasymercados.es/es/sala-de-comunicacion/\
              notas-de-prensa/); retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// NZX
// ─────────────────────────────────────────────────────────────────────────

static XNZE_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "Queen Elizabeth II Memorial Day",
        "",
        Rule::gregorian(9, 26),
    )
    .years(Some(2022), Some(2022)),
    HolidayRule::observance(
        "Early close, the business day before Christmas Day",
        "",
        Rule::Computed(last_weekday_before_christmas),
    ),
    HolidayRule::observance(
        "Early close, the business day before New Year's Day",
        "",
        Rule::Computed(last_weekday_of_the_year),
    ),
];

/// NZX, New Zealand's exchange.
///
/// NZX's memos "Market holidays and abbreviated trading days" for 2021 to
/// the start of 2027: the market is closed on New Zealand's public
/// holidays — the [`NEW_ZEALAND`] table's days, mondayised as the memos
/// say, "As 25 April 2026 falls on a Saturday, the following Monday is
/// observed", and Matariki, which this set includes and does not repeat —
/// and on none of the regional anniversary days, which no memo lists. The
/// one closure outside them is 26 September 2022, the public holiday for
/// Queen Elizabeth II. The "Business Day Prior to Christmas Day" and the
/// "Business Day Prior to New Year's Day" are abbreviated, the main board
/// closing at 12:45 p.m. rather than 4:45 p.m.: 24 and 31 December, or the
/// Friday before when those fall on a weekend, as in 2022.
pub static NZX: RuleSet = RuleSet {
    code: "XNZE",
    english_name: "NZX",
    rules: XNZE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[Include::nationwide(&NEW_ZEALAND)],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "NZX, \"NZX Market Holidays\" memos for 2021/2023 to 2025/2027 \
              (nzx.com/announcements/383874, 403367, 422808, 443000 and 463713), and the \
              announcement of the closure of 26 September 2022 (nzx.com/announcements/398794); \
              NZX, \"Trading hours\" (nzx.com/learning/help-reference/trading-hours), for \
              \"public holidays will be mondayised\"; retrieved 2026-09-23",
};

/// The days of `year` in a table of (year, month, day).
fn listed_days(table: &[(i64, u8, u8)], year: i64) -> Days {
    let mut out = Days::new();
    for &(y, month, day) in table {
        if y == year
            && let Ok(fixed) = gregorian::to_fixed(y, month, day)
        {
            out.push(fixed);
        }
    }
    out
}

/// Every exchange calendar, in Market Identifier Code order.
pub static ALL: &[&RuleSet] = &[
    &B3,
    &MOSCOW_EXCHANGE,
    &EURONEXT_AMSTERDAM,
    &AUSTRALIAN_SECURITIES_EXCHANGE,
    &EURONEXT_BRUSSELS,
    &NASDAQ_COPENHAGEN,
    &EURONEXT_DUBLIN,
    &FRANKFURT_STOCK_EXCHANGE,
    &NASDAQ_HELSINKI,
    &HONG_KONG_EXCHANGES,
    &NASDAQ_ICELAND,
    &BORSA_ISTANBUL,
    &TOKYO_STOCK_EXCHANGE,
    &JOHANNESBURG_STOCK_EXCHANGE,
    &KOREA_EXCHANGE,
    &EURONEXT_LISBON,
    &LONDON_STOCK_EXCHANGE,
    &BOLSA_DE_MADRID,
    &BOLSA_MEXICANA_DE_VALORES,
    &EURONEXT_MILAN,
    &NASDAQ,
    &NEW_YORK_STOCK_EXCHANGE,
    &NZX,
    &EURONEXT_OSLO,
    &EURONEXT_PARIS,
    &SAUDI_EXCHANGE,
    &SHANGHAI_STOCK_EXCHANGE,
    &NASDAQ_STOCKHOLM,
    &SIX_SWISS_EXCHANGE,
    &TEL_AVIV_STOCK_EXCHANGE,
    &TAIWAN_STOCK_EXCHANGE,
    &TORONTO_STOCK_EXCHANGE,
    &WARSAW_STOCK_EXCHANGE,
    &WIENER_BOERSE,
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
            // A weekend day the exchange trades on is neither.
            if rule.kind == Kind::Workday {
                assert!(rule.name.starts_with("Working day"), "{}", rule.name);
                continue;
            }
            let early = rule.name.starts_with("Early close")
                || rule.name.starts_with("Half trading day")
                || rule.name.starts_with("Late open");
            assert_eq!(rule.kind == Kind::Observance, early, "{}", rule.name);
            assert_eq!(rule.kind.is_day_off(), !early, "{}", rule.name);
        }
    }

    #[test]
    fn the_lookup_takes_the_code_in_either_case() {
        assert!(by_code("xnys").is_some());
        assert!(by_code("XNYS").is_some());
        assert!(by_code("xtse").is_some());
        assert!(by_code("XNAS").is_some());
    }
}
