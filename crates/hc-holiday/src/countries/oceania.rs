//! Australia and New Zealand.

use hc_calendar::Weekday;
use hc_calendars_solar::gregorian;

use crate::computus::offsets::{EASTER_MONDAY, EASTER_SUNDAY, GOOD_FRIDAY, HOLY_SATURDAY};
use crate::rule::{
    Days, HolidayRule, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, SubstituteDirection,
    SubstitutionPolicy,
};

// ─────────────────────────────────────────────────────────────────────────
// Australia
// ─────────────────────────────────────────────────────────────────────────

const AU_VIC: &[&str] = &["AU-VIC"];
const AU_QLD: &[&str] = &["AU-QLD"];
const AU_SA: &[&str] = &["AU-SA"];
const AU_WA: &[&str] = &["AU-WA"];
const AU_TAS: &[&str] = &["AU-TAS"];
const AU_NT: &[&str] = &["AU-NT"];
const AU_ACT: &[&str] = &["AU-ACT"];
const AU_EASTER_SATURDAY: &[&str] = &["AU-ACT", "AU-NSW", "AU-NT", "AU-QLD", "AU-SA", "AU-VIC"];
const AU_EASTER_SUNDAY: &[&str] = &["AU-ACT", "AU-NSW", "AU-QLD", "AU-VIC"];
const AU_JUNE_SOVEREIGN: &[&str] = &["AU-ACT", "AU-NSW", "AU-NT", "AU-SA", "AU-TAS", "AU-VIC"];
const AU_OCTOBER_LABOUR: &[&str] = &["AU-ACT", "AU-NSW", "AU-SA"];
const AU_MAY_LABOUR: &[&str] = &["AU-NT", "AU-QLD"];

static AU_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Australia Day", "", Rule::gregorian(1, 26)),
    HolidayRule::public("Labour Day", "", Rule::nth(3, 1, Weekday::Monday)).in_regions(AU_WA),
    HolidayRule::public("Labour Day", "", Rule::nth(3, 2, Weekday::Monday)).in_regions(AU_VIC),
    HolidayRule::public("Eight Hours Day", "", Rule::nth(3, 2, Weekday::Monday)).in_regions(AU_TAS),
    HolidayRule::public("Canberra Day", "", Rule::nth(3, 2, Weekday::Monday)).in_regions(AU_ACT),
    HolidayRule::public("Adelaide Cup Day", "", Rule::nth(3, 2, Weekday::Monday)).in_regions(AU_SA),
    HolidayRule::public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    // Easter Saturday and Easter Sunday are holidays where they are holidays
    // at all; no state moves them, because the Monday is already one.
    HolidayRule::fixed_public("Easter Saturday", "", Rule::easter(HOLY_SATURDAY))
        .in_regions(AU_EASTER_SATURDAY),
    HolidayRule::fixed_public("Easter Sunday", "", Rule::easter(EASTER_SUNDAY))
        .in_regions(AU_EASTER_SUNDAY),
    HolidayRule::public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    // Anzac Day is not mondayised in New South Wales, Victoria, Tasmania or
    // the Northern Territory, and is in Western Australia, Queensland, South
    // Australia and the ACT. Nationally the crate takes the stricter of the
    // two and leaves it where it falls.
    HolidayRule::fixed_public("Anzac Day", "", Rule::gregorian(4, 25)).years(Some(1921), None),
    HolidayRule::public("Labour Day", "", Rule::nth(5, 1, Weekday::Monday))
        .in_regions(AU_MAY_LABOUR),
    HolidayRule::public("Reconciliation Day", "", Rule::nth(5, 4, Weekday::Monday))
        .in_regions(AU_ACT)
        .years(Some(2018), None),
    HolidayRule::public(
        "Western Australia Day",
        "",
        Rule::nth(6, 1, Weekday::Monday),
    )
    .in_regions(AU_WA),
    HolidayRule::public("Sovereign's Birthday", "", Rule::nth(6, 2, Weekday::Monday))
        .in_regions(AU_JUNE_SOVEREIGN),
    HolidayRule::public("Picnic Day", "", Rule::nth(8, 1, Weekday::Monday)).in_regions(AU_NT),
    HolidayRule::public(
        "Royal Queensland Show",
        "",
        Rule::nth(8, 2, Weekday::Wednesday),
    )
    .in_regions(AU_QLD),
    HolidayRule::public(
        "Sovereign's Birthday",
        "",
        Rule::nth(10, 1, Weekday::Monday),
    )
    .in_regions(AU_QLD),
    HolidayRule::public("Labour Day", "", Rule::nth(10, 1, Weekday::Monday))
        .in_regions(AU_OCTOBER_LABOUR),
    HolidayRule::public("Melbourne Cup Day", "", Rule::nth(11, 1, Weekday::Tuesday))
        .in_regions(AU_VIC),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    // South Australia calls 26 December Proclamation Day; everywhere else it
    // is Boxing Day.
    HolidayRule::public("Proclamation Day", "", Rule::gregorian(12, 26)).in_regions(AU_SA),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

static AU_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Australia, with the six states and two territories as regions.
pub static AUSTRALIA: RuleSet = RuleSet {
    code: "AU",
    english_name: "Australia",
    rules: AU_RULES,
    substitution: AU_SUBSTITUTION,
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "State and territory public holiday acts and the annual \
              gazettals; Fair Work Ombudsman's published list. Western \
              Australia's Sovereign's Birthday is proclaimed each year and \
              is not modelled; nor are regional show days other than \
              Queensland's",
};

// ─────────────────────────────────────────────────────────────────────────
// New Zealand
// ─────────────────────────────────────────────────────────────────────────

/// Matariki, the Māori new year, as the statute schedules it.
///
/// The Te Kāhui o Matariki Public Holiday Act 2022 does not give a rule; it
/// gives a *table*, because the date is the Friday nearest the Tangaroa
/// nights of the lunar month in which the Pleiades rise, as determined by
/// the Matariki Advisory Group. A table is the one thing the rule
/// vocabulary genuinely cannot express, so this is a `Computed` rule holding
/// the schedule verbatim.
///
/// The Act schedules dates through 2052. Only the years published in the
/// sources this crate checked are carried here; a year outside them yields
/// nothing rather than an invented Friday.
fn matariki(year: i64) -> Days {
    let (month, day) = match year {
        2022 => (6u8, 24u8),
        2023 => (7, 14),
        2024 => (6, 28),
        2025 => (6, 20),
        2026 => (7, 10),
        2027 => (6, 25),
        2028 => (7, 14),
        2029 => (7, 6),
        2030 => (6, 21),
        2031 => (7, 11),
        2032 => (7, 2),
        2033 => (6, 24),
        2034 => (7, 7),
        2035 => (6, 29),
        _ => return Days::new(),
    };
    gregorian::to_fixed(year, month, day).map_or_else(|_| Days::new(), Days::one)
}

static NZ_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Day after New Year's Day", "", Rule::gregorian(1, 2)),
    // Waitangi Day and Anzac Day were mondayised only from 2014.
    HolidayRule::fixed_public("Waitangi Day", "Te Rā o Waitangi", Rule::gregorian(2, 6))
        .years(Some(1974), Some(2013)),
    HolidayRule::public("Waitangi Day", "Te Rā o Waitangi", Rule::gregorian(2, 6))
        .years(Some(2014), None),
    HolidayRule::public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Anzac Day", "", Rule::gregorian(4, 25))
        .years(Some(1921), Some(2013)),
    HolidayRule::public("Anzac Day", "", Rule::gregorian(4, 25)).years(Some(2014), None),
    HolidayRule::public("Sovereign's Birthday", "", Rule::nth(6, 1, Weekday::Monday)),
    HolidayRule::fixed_public("Matariki", "Matariki", Rule::Computed(matariki))
        .years(Some(2022), None),
    HolidayRule::public("Labour Day", "", Rule::nth(10, 4, Weekday::Monday)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

static NZ_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// New Zealand.
pub static NEW_ZEALAND: RuleSet = RuleSet {
    code: "NZ",
    english_name: "New Zealand",
    rules: NZ_RULES,
    substitution: NZ_SUBSTITUTION,
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Holidays Act 2003, sections 44 and 45 for mondayisation; \
              Holidays (Full Recognition of Waitangi Day and ANZAC Day) \
              Amendment Act 2013; Te Kāhui o Matariki Public Holiday Act \
              2022 for the Matariki schedule. Regional anniversary days are \
              set by provincial custom and are not modelled",
};
