//! New Zealand's provincial anniversary days, each scoped to the ISO 3166-2
//! region that bears its province's name.
//!
//! The Holidays Act 2003 makes the anniversary day of each province a
//! public holiday but does not date it: the provinces are the nineteenth
//! century's, abolished in 1876, and each day is "observed locally by
//! custom and practice and … generally prescribed by regional or city
//! councils", as Employment New Zealand puts it. Its list of "Public
//! holidays and anniversary dates" gives the observed day of every
//! province for 2010 to 2027 and the rule behind each, and this file
//! carries the rules, from 2010, the first year of the list:
//!
//! * the Monday nearest the anniversary for Auckland (29 January),
//!   Wellington (22 January), Nelson (1 February), Otago (23 March),
//!   Westland (1 December) and the Chatham Islands (30 November), and for
//!   Southland (17 January) in 2010 and 2011;
//! * Taranaki's second Monday of March, which keeps it clear of Easter;
//! * Hawke's Bay's Friday before Labour Day, and Marlborough's first Monday
//!   after it;
//! * Canterbury's Christchurch Show Day, "the second Friday after the first
//!   Tuesday in November", which northern and central Canterbury observe;
//! * Southland's Easter Tuesday from 2012.
//!
//! Each rule gives every day of the list, 2010 to 2027, and the years
//! before 2010 are not carried: no list or council's resolution for them
//! was read. The days are public holidays and do not move off a weekend,
//! the Monday and Friday rules keeping them off it in any case.
//!
//! Two things the list gives are not carried. South Canterbury observes
//! Dominion Day, the fourth Monday of September, and has no ISO 3166-2
//! code of its own: `NZ-CAN` is the whole region, and it is given the Show
//! Day. And the regions that bear no province's name — Northland, Waikato,
//! Bay of Plenty, Gisborne, Manawatū-Whanganui and Tasman — are given no
//! anniversary day, since the list names provinces, which "are not
//! determined by present-day districts or regions", and no source read
//! says which province's day each region keeps. Employment New Zealand also
//! says that Westland's day "varies throughout Westland, but Greymouth
//! observes the official day", and that for Otago "there is no easily
//! determined single day of local observance"; the rules are the days it
//! lists. The English names are the crate's.

use hc_calendar::Weekday;

use crate::computus::offsets::EASTER_TUESDAY;
use crate::rule::{HolidayRule, Rule};

use super::oceania::NZ_RULES;
use crate::rule::joined;

/// The Monday nearest a date: the three days after a Monday back to it, and
/// the three days before one forward.
const NEAREST_MONDAY: &[(Weekday, i16)] = &[
    (Weekday::Tuesday, -1),
    (Weekday::Wednesday, -2),
    (Weekday::Thursday, -3),
    (Weekday::Friday, 3),
    (Weekday::Saturday, 2),
    (Weekday::Sunday, 1),
];

/// The first year of Employment New Zealand's list.
const FIRST: i32 = 2010;

static AUCKLAND: Rule = Rule::gregorian(1, 29);
static WELLINGTON: Rule = Rule::gregorian(1, 22);
static NELSON: Rule = Rule::gregorian(2, 1);
static OTAGO: Rule = Rule::gregorian(3, 23);
static WESTLAND: Rule = Rule::gregorian(12, 1);
static CHATHAM_ISLANDS: Rule = Rule::gregorian(11, 30);
static SOUTHLAND: Rule = Rule::gregorian(1, 17);
static LABOUR_DAY: Rule = Rule::nth(10, 4, Weekday::Monday);
static FIRST_TUESDAY_OF_NOVEMBER: Rule = Rule::nth(11, 1, Weekday::Tuesday);

/// An anniversary day, from the first year of the list, in `region`.
const fn anniversary(
    name: &'static str,
    rule: Rule,
    region: &'static [&'static str],
) -> HolidayRule {
    HolidayRule::fixed_public(name, "", rule)
        .years(Some(FIRST), None)
        .in_regions(region)
}

/// Every anniversary day carried, by the region that bears its
/// province's name.
pub static ANNIVERSARY_DAYS: &[HolidayRule] = &[
    anniversary(
        "Wellington Anniversary Day",
        Rule::moved_by_weekday(&WELLINGTON, NEAREST_MONDAY),
        &["NZ-WGN"],
    ),
    anniversary(
        "Auckland Anniversary Day",
        Rule::moved_by_weekday(&AUCKLAND, NEAREST_MONDAY),
        &["NZ-AUK"],
    ),
    anniversary(
        "Nelson Anniversary Day",
        Rule::moved_by_weekday(&NELSON, NEAREST_MONDAY),
        &["NZ-NSN"],
    ),
    anniversary(
        "Taranaki Anniversary Day",
        Rule::nth(3, 2, Weekday::Monday),
        &["NZ-TKI"],
    ),
    anniversary(
        "Otago Anniversary Day",
        Rule::moved_by_weekday(&OTAGO, NEAREST_MONDAY),
        &["NZ-OTA"],
    ),
    anniversary(
        "Southland Anniversary Day",
        Rule::moved_by_weekday(&SOUTHLAND, NEAREST_MONDAY),
        &["NZ-STL"],
    )
    .years(Some(FIRST), Some(2011)),
    anniversary(
        "Southland Anniversary Day",
        Rule::easter(EASTER_TUESDAY),
        &["NZ-STL"],
    )
    .years(Some(2012), None),
    anniversary(
        "Hawke's Bay Anniversary Day",
        Rule::Offset {
            base: &LABOUR_DAY,
            days: -3,
        },
        &["NZ-HKB"],
    ),
    anniversary(
        "Marlborough Anniversary Day",
        Rule::Offset {
            base: &LABOUR_DAY,
            days: 7,
        },
        &["NZ-MBH"],
    ),
    // The second Friday after the first Tuesday: ten days on.
    anniversary(
        "Canterbury Anniversary Day",
        Rule::Offset {
            base: &FIRST_TUESDAY_OF_NOVEMBER,
            days: 10,
        },
        &["NZ-CAN"],
    ),
    anniversary(
        "Chatham Islands Anniversary Day",
        Rule::moved_by_weekday(&CHATHAM_ISLANDS, NEAREST_MONDAY),
        &["NZ-CIT"],
    ),
    anniversary(
        "Westland Anniversary Day",
        Rule::moved_by_weekday(&WESTLAND, NEAREST_MONDAY),
        &["NZ-WTC"],
    ),
];

/// New Zealand's nationwide rules and its anniversary days, the table
/// [`super::NEW_ZEALAND`] evaluates.
pub(super) static RULES: [HolidayRule; NZ_RULES.len() + ANNIVERSARY_DAYS.len()] =
    joined(&[NZ_RULES, ANNIVERSARY_DAYS]);
