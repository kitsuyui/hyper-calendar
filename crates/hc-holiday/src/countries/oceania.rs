//! Australia, New Zealand and the Pacific island states.

use hc_calendar::Weekday;
use hc_calendars_solar::gregorian;

use crate::computus::offsets::{
    ASCENSION, EASTER_MONDAY, EASTER_SUNDAY, EASTER_TUESDAY, GOOD_FRIDAY, HOLY_SATURDAY,
    WHIT_MONDAY,
};
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
    includes: &[],
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
/// nothing rather than an invented Friday — and, because "nothing" and "no
/// holiday that year" used to be the same answer, the rule is a
/// [`Rule::Tabulated`] with its last year written down, so a calendar built
/// past 2035 reports Matariki as a gap instead of dropping it.
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
    HolidayRule::fixed_public(
        "Matariki",
        "Matariki",
        Rule::Tabulated {
            function: matariki,
            first_year: 2022,
            last_year: 2035,
        },
    )
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
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Holidays Act 2003, sections 44 and 45 for mondayisation; \
              Holidays (Full Recognition of Waitangi Day and ANZAC Day) \
              Amendment Act 2013; Te Kāhui o Matariki Public Holiday Act \
              2022 for the Matariki schedule. Regional anniversary days are \
              set by provincial custom and are not modelled",
};

// ─────────────────────────────────────────────────────────────────────────
// Micronesia
// ─────────────────────────────────────────────────────────────────────────

/// Title 1, section 602: a Saturday holiday is observed on the Friday
/// before, a Sunday one on the Monday after.
static FM_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Nearest,
    skip_occupied: false,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static FM_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Micronesian Culture and Tradition Day",
        "",
        Rule::gregorian(3, 31),
    )
    .years(Some(2010), None),
    HolidayRule::public(
        "Federated States of Micronesia Day",
        "",
        Rule::gregorian(5, 10),
    ),
    HolidayRule::public("United Nations Day", "", Rule::gregorian(10, 24)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(11, 3)),
    HolidayRule::public(
        "FSM Veterans of Foreign Wars Day",
        "",
        Rule::gregorian(11, 11),
    )
    .years(Some(2004), Some(2020)),
    HolidayRule::public("Veterans Day", "", Rule::gregorian(11, 11)).years(Some(2021), None),
    HolidayRule::public("Presidents Day", "", Rule::gregorian(11, 23)).years(Some(2021), None),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
];

/// The Federated States of Micronesia: the national holidays.
///
/// Title 1, chapter 6 of the Code: section 601's days, section 602's
/// Friday-before and Monday-after rule, and section 603, which forbids
/// the National Government any other. Public Law 13-38, approved on 23
/// June 2004, added FSM Veterans of Foreign Wars Day; Public Law 16-27,
/// approved on 17 March 2010, Micronesian Culture and Tradition Day; and
/// Public Law 21-209, approved on 24 November 2020, renamed 11 November
/// Veterans Day and added Presidents Day on 23 November, so both are
/// carried from 2021. The four states' own holidays are not modelled.
pub static MICRONESIA: RuleSet = RuleSet {
    code: "FM",
    english_name: "Micronesia",
    rules: FM_RULES,
    substitution: FM_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Code of the Federated States of Micronesia (2014), title 1, chapter 6, \
              sections 601 to 603, and Public Laws 13-38, 16-27 and 21-209, as the FSM \
              Legal Information System publishes them (fsmlaw.org), retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Marshall Islands
// ─────────────────────────────────────────────────────────────────────────

/// Section 902: a Saturday holiday is the Friday before, a Sunday one the
/// Monday after, "unless otherwise directed by the Public Service
/// Commission".
static MH_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Nearest,
    skip_occupied: false,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static MH_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Nuclear Victims Remembrance Day", "", Rule::gregorian(3, 1)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Constitution Day", "", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Fisherman's Day", "", Rule::nth(7, 1, Weekday::Friday)),
    HolidayRule::fixed_public("Dri-jerbal Day", "", Rule::nth(9, 1, Weekday::Friday)),
    HolidayRule::fixed_public("Manit Day", "", Rule::last(9, Weekday::Friday)),
    HolidayRule::public("President's Day", "", Rule::gregorian(11, 17)),
    HolidayRule::fixed_public("Gospel Day", "", Rule::nth(12, 1, Weekday::Friday)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
];

/// The Marshall Islands.
///
/// The Public Holidays Act 1988, 1 MIRC chapter 9, as amended to P.L.
/// 2015-38: the Schedule's days, and section 902's Friday before a
/// Saturday holiday and Monday after a Sunday one. The Schedule's General
/// Election Day, the third Monday of November "of the election year", is
/// not carried: the Act does not say which years those are. The Friday
/// holidays cannot reach a weekend and are marked as never moving.
pub static MARSHALL_ISLANDS: RuleSet = RuleSet {
    code: "MH",
    english_name: "Marshall Islands",
    rules: MH_RULES,
    substitution: MH_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act 1988, 1 MIRC Ch. 9, the Nitijela's consolidation \
              (rmiparliament.org, 1988-0016_2.pdf) as the Internet Archive holds it, \
              captured 2025-04-04, the Nitijela's site refusing this session's requests; \
              retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Nauru
// ─────────────────────────────────────────────────────────────────────────

/// Section 81(2) of the Public Service Act 2016: a holiday on a Saturday
/// or Sunday goes to the following Monday; Independence Day on a weekend
/// gives the Monday and the Tuesday, and so does a Saturday Christmas, for
/// Christmas and Boxing Day both; a Sunday Christmas gives the Tuesday.
///
/// All of that is the next free weekday, taken in date order: a weekend
/// Independence Day and the 1 February beside it land on the Monday and
/// the Tuesday, and a Christmas and Boxing Day pair on the Monday and the
/// Tuesday or, for a Sunday Christmas, on the Tuesday after Boxing Day.
static NR_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// A day the President declared in the gazettes this table read, for the
/// years they covered.
fn nr_declared(year: i64, month: u8, day: u8, years: &[i64]) -> Days {
    if !years.contains(&year) {
        return Days::new();
    }
    gregorian::to_fixed(year, month, day).map_or_else(|_| Days::new(), Days::one)
}

/// The years whose public-holiday gazettes were read: 2023, 2024 and 2026.
const NR_GAZETTED: &[i64] = &[2023, 2024, 2026];

/// The day only the 2026 gazette declared.
const NR_GAZETTED_2026: &[i64] = &[2026];

fn nr_women_day(year: i64) -> Days {
    nr_declared(year, 3, 8, NR_GAZETTED)
}

fn nr_eigigu_day(year: i64) -> Days {
    nr_declared(year, 6, 26, NR_GAZETTED_2026)
}

fn nr_remembrance_day(year: i64) -> Days {
    nr_declared(year, 6, 29, NR_GAZETTED_2026)
}

fn nr_ronphos_handover(year: i64) -> Days {
    nr_declared(year, 7, 1, NR_GAZETTED)
}

fn nr_judicial_well_being(year: i64) -> Days {
    nr_declared(year, 7, 25, NR_GAZETTED_2026)
}

fn nr_ibumin_earoeni_day(year: i64) -> Days {
    nr_declared(year, 8, 19, NR_GAZETTED)
}

fn nr_hammer_deroburt_day(year: i64) -> Days {
    nr_declared(year, 9, 25, NR_GAZETTED)
}

/// A day declared in the 2023, 2024 and 2026 gazettes, as two tables: 2023
/// to 2024, and 2026 standing in for 2025 onwards, so that 2025, whose
/// gazette was not read, and every year after 2026 are reported as gaps
/// rather than as years without the day.
const fn nr_gazetted(name: &'static str, function: fn(i64) -> Days) -> [HolidayRule; 2] {
    [
        HolidayRule::public(
            name,
            "",
            Rule::Tabulated {
                function,
                first_year: 2023,
                last_year: 2024,
            },
        )
        .years(None, Some(2024)),
        HolidayRule::public(
            name,
            "",
            Rule::Tabulated {
                function,
                first_year: 2026,
                last_year: 2026,
            },
        )
        .years(Some(2025), None),
    ]
}

/// A day the 2026 gazette alone declared, known for 2026 and a gap in
/// every other year.
const fn nr_gazetted_2026(name: &'static str, function: fn(i64) -> Days) -> HolidayRule {
    HolidayRule::public(
        name,
        "",
        Rule::Tabulated {
            function,
            first_year: 2026,
            last_year: 2026,
        },
    )
}

const NR_WOMEN_DAY: [HolidayRule; 2] = nr_gazetted("International Women's Day", nr_women_day);
const NR_RONPHOS_HANDOVER: [HolidayRule; 2] = nr_gazetted("RONPHOS Handover", nr_ronphos_handover);
const NR_IBUMIN_EAROENI_DAY: [HolidayRule; 2] =
    nr_gazetted("Ibumin Earoeni Day", nr_ibumin_earoeni_day);
const NR_HAMMER_DEROBURT_DAY: [HolidayRule; 2] =
    nr_gazetted("Sir Hammer DeRoburt Day", nr_hammer_deroburt_day);

static NR_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(1, 31)),
    HolidayRule::public("Day following Independence Day", "", Rule::gregorian(2, 1)),
    NR_WOMEN_DAY[0],
    NR_WOMEN_DAY[1],
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Easter Tuesday", "", Rule::easter(EASTER_TUESDAY)),
    HolidayRule::public("Constitution Day", "", Rule::gregorian(5, 17)),
    nr_gazetted_2026("Eigigu Day", nr_eigigu_day),
    nr_gazetted_2026("Remembrance Day", nr_remembrance_day),
    NR_RONPHOS_HANDOVER[0],
    NR_RONPHOS_HANDOVER[1],
    nr_gazetted_2026(
        "International Day for Judicial Well-Being",
        nr_judicial_well_being,
    ),
    NR_IBUMIN_EAROENI_DAY[0],
    NR_IBUMIN_EAROENI_DAY[1],
    NR_HAMMER_DEROBURT_DAY[0],
    NR_HAMMER_DEROBURT_DAY[1],
    HolidayRule::public("Angam Day", "", Rule::gregorian(10, 26)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Nauru.
///
/// Section 81 of the Public Service Act 2016, as RONLAW's consolidation
/// prints it in 2026: the ten days of subsection (1)(a) to (j), and
/// subsection (2)'s weekend rule. The days the President declares under
/// (1)(k) are declared year by year; the gazettes for 2023, 2024 and 2026
/// each declare International Women's Day, RONPHOS Handover, Ibumin
/// Earoeni Day and Sir Hammer DeRoburt Day, and 2026's adds Eigigu Day,
/// Remembrance Day and the International Day for Judicial Well-Being.
/// Those are tabulated for the years read, and every other year — 2025,
/// whose gazette was not found, among them — is reported as a gap. The
/// declared days are public holidays under subsection (1) and move off a
/// weekend with the rest. The Act speaks for the public service; the
/// gazettes are the national list.
pub static NAURU: RuleSet = RuleSet {
    code: "NR",
    english_name: "Nauru",
    rules: NR_RULES,
    substitution: NR_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Service Act 2016, section 81, RONLAW consolidation (service 6); \
              Government Gazette No. 330 of 30 December 2022 \
              (G.N. 1350/2022), No. 14 of 15 January 2024 and No. 7 of 9 January 2026 \
              (G.N. 28/2026), the public-holiday lists for 2023, 2024 and 2026 \
              (ronlaw.gov.nr), retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Palau
// ─────────────────────────────────────────────────────────────────────────

/// 1 PNCA § 702(a): a Sunday holiday is observed on the following Monday,
/// a Saturday one on the preceding Friday.
static PW_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Nearest,
    skip_occupied: false,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static PW_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Youth Day", "", Rule::gregorian(3, 15)),
    HolidayRule::public("Senior Citizens Day", "", Rule::gregorian(5, 5)),
    HolidayRule::public("Constitution Day", "", Rule::gregorian(7, 9)),
    HolidayRule::fixed_public("Labor Day", "", Rule::nth(9, 1, Weekday::Monday)),
    HolidayRule::public("United Nations Day", "", Rule::gregorian(10, 24)),
    HolidayRule::fixed_public("Thanksgiving Day", "", Rule::nth(11, 4, Weekday::Thursday)),
    HolidayRule::fixed_public("Family Day", "", Rule::nth(11, 4, Weekday::Friday)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
];

/// Palau.
///
/// Title 1, chapter 7 of the Palau National Code Annotated, as its
/// Supplement 12 prints it: section 701's nine legal holidays, RPPL 2-15
/// as amended by RPPL 10-15, and section 702's Friday-before and
/// Monday-after rule. Family Day is the fourth Friday of November, which
/// the Code says, and not the day after Thanksgiving, which it is in most
/// years and not in one where November begins on a Friday. Section 701
/// also lets the President declare other days; those, and any holiday on
/// another list that section 701 does not name, are not carried.
pub static PALAU: RuleSet = RuleSet {
    code: "PW",
    english_name: "Palau",
    rules: PW_RULES,
    substitution: PW_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Palau National Code Annotated, title 1, sections 701 and 702 (Supp. 12), \
              PacLII's copy (pncgpt1409.pdf) as the Internet Archive holds it, captured \
              2025-12-06, PacLII refusing this session's requests; retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Papua New Guinea
// ─────────────────────────────────────────────────────────────────────────

/// Section 1(2) and (3): a Sunday holiday, Christmas Day aside, is kept on
/// the next Monday, and a Sunday Christmas Day gives the Tuesday too —
/// the next free weekday in both cases, Boxing Day holding the Monday.
static PG_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Nothing: no National Gazette was read, so the days it appoints are a
/// gap in every year.
fn pg_gazetted(_: i64) -> Days {
    Days::new()
}

/// A day the Head of State appoints each year by notice in the Gazette.
const PG_GAZETTED: Rule = Rule::Tabulated {
    function: pg_gazetted,
    first_year: 1,
    last_year: 0,
};

static PG_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Saturday", "", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public(
        "Papua New Guinea Remembrance Day",
        "",
        Rule::gregorian(7, 23),
    ),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
    HolidayRule::fixed_public("Independence Day", "", PG_GAZETTED),
    HolidayRule::fixed_public("Sovereign's Birthday", "", PG_GAZETTED),
];

/// Papua New Guinea: the days the Act itself fixes, and no more.
///
/// The Public Holidays Act 1953, Chapter 321, section 1: 1 January, Good
/// Friday and the Saturday and Monday after it, 23 July, Christmas Day
/// and the day after, with subsection (2)'s Sunday rule and (3)'s Tuesday
/// after a Sunday Christmas. Independence Day and the Sovereign's
/// Birthday are whatever the Head of State appoints by notice in the
/// National Gazette under sections 2 and 3, and other days are added
/// under section 5; no gazette was read, so the first two are reported as
/// a gap in every year, and the table is a floor, not the year's list.
pub static PAPUA_NEW_GUINEA: RuleSet = RuleSet {
    code: "PG",
    english_name: "Papua New Guinea",
    rules: PG_RULES,
    substitution: PG_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act 1953 (Chapter 321), sections 1 to 5, PacLII's \
              consolidation (pha1953163) and its 1982 revised-edition PDF as the \
              Internet Archive holds them, captured 2025-01-30 and 2025-01-01, PacLII \
              refusing this session's requests; retrieved 2026-09-23. Independence Day \
              and the Sovereign's Birthday are reported as gaps, and the other gazetted days \
              are not modelled",
};

// ─────────────────────────────────────────────────────────────────────────
// Solomon Islands
// ─────────────────────────────────────────────────────────────────────────

/// Section 2: a Sunday holiday gives "the Monday next following", and a
/// Monday 26 December gives the Tuesday — the next free weekday, which is
/// what a Sunday Christmas produces.
static SB_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static SB_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Holy Saturday", "", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Whit Monday", "", Rule::easter(WHIT_MONDAY)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(7, 7)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("National Day of Thanksgiving", "", Rule::gregorian(12, 26)),
];

/// Solomon Islands.
///
/// The Public Holidays Act, Chapter 151, in the 1996 edition: the
/// Schedule's days and section 2's Sunday rule, with the names the
/// Ministry of Home Affairs' notices use. The Schedule's "day appointed
/// for the celebration of the Anniversary of the Birthday of the
/// Sovereign" is appointed each year — the notices for 2018 and 2020
/// gave the second Saturday of June, and the 2026 gazette as reported
/// gave none — and is not carried. The notices also keep a Saturday
/// holiday on the Friday before, which the Act does not say, and the
/// provincial days are appointed each year under section 6; neither is
/// modelled.
pub static SOLOMON_ISLANDS: RuleSet = RuleSet {
    code: "SB",
    english_name: "Solomon Islands",
    rules: SB_RULES,
    substitution: SB_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act (Cap. 151), 1996 edition, PacLII's consolidation \
              (pha163) as the Internet Archive holds it, captured 2024-12-22, PacLII \
              refusing this session's requests; Ministry of Home Affairs, Public Notices \
              1/2017 and 1/2019 for 2018 and 2020 (mehrd.gov.sb, solomons.gov.sb); the \
              Island Sun, 13 January 2026, on the 2026 gazette; retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Tonga
// ─────────────────────────────────────────────────────────────────────────

/// Section 2's first proviso: Emancipation Day, Constitution Day and the
/// Anniversary of the Coronation of King George Tupou I are kept on the
/// Monday after from a Thursday, Friday, Saturday or Sunday and on the
/// Monday before from a Tuesday or Wednesday.
const TO_TO_MONDAY: &[(Weekday, i16)] = &[
    (Weekday::Tuesday, -1),
    (Weekday::Wednesday, -2),
    (Weekday::Thursday, 4),
    (Weekday::Friday, 3),
    (Weekday::Saturday, 2),
    (Weekday::Sunday, 1),
];

static TO_JUNE_4: Rule = Rule::gregorian(6, 4);
static TO_NOVEMBER_4: Rule = Rule::gregorian(11, 4);
static TO_DECEMBER_4: Rule = Rule::gregorian(12, 4);

/// Section 2's second proviso, inserted by Act 5 of 2013: the two royal
/// birthdays are kept on the day, or on the Monday when the day is a
/// Sunday. Nothing else moves off a weekend.
static TO_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static TO_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "ʻUluaki ʻaho ʻo e taʻu foʻou",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public("Good Friday", "Falaite Lelei", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Monite Toetuʻu",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("Anzac Day", "ʻAho ANZAC", Rule::gregorian(4, 25)),
    HolidayRule::fixed_public(
        "Emancipation Day",
        "ʻAho ʻo e Tauʻatāina",
        Rule::moved_by_weekday(&TO_JUNE_4, TO_TO_MONDAY),
    ),
    HolidayRule::public(
        "Birthday of the Reigning Sovereign",
        "ʻAho ʻaloʻi ʻo ʻEne ʻAfio ko e Tuʻi ʻo Tonga, ʻoku lolotonga Pule",
        Rule::gregorian(7, 4),
    ),
    HolidayRule::public(
        "Birthday of the Heir to the Crown",
        "ʻAho ʻaloʻi ʻo e ʻEa ki he Kalauni ʻo Tonga",
        Rule::gregorian(9, 17),
    ),
    HolidayRule::fixed_public(
        "Constitution Day",
        "ʻAho Konisitutone",
        Rule::moved_by_weekday(&TO_NOVEMBER_4, TO_TO_MONDAY),
    ),
    HolidayRule::fixed_public(
        "Anniversary of the Coronation of King George Tupou I",
        "ʻAho Fakamanatu ʻo e Hilifaki kalauni ʻo ʻEne ʻAfio ko Siaosi Tupou I",
        Rule::moved_by_weekday(&TO_DECEMBER_4, TO_TO_MONDAY),
    ),
    HolidayRule::fixed_public("Christmas Day", "ʻAho Kilisimasi", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Tonga.
///
/// The Public Holidays Act, Chapter 8.11 of the 2020 Revised Edition,
/// section 2, as the Prime Minister's Office applies it in its lists for
/// 2024 and 2026, whose Tongan names these are. The three days of the
/// first proviso move to a Monday by the weekday they fall on, and the
/// original date is not a holiday; the royal birthdays move off a Sunday
/// only; New Year's Day, Good Friday, Easter Monday, Anzac Day, Christmas
/// and Boxing Day stay where they fall, as both lists say.
///
/// The royal days are the reign's: the Sovereign's birthday is kept on 4
/// July by the Cabinet decision of 6 July 2012 the Act's endnote cites,
/// and the Heir's on 17 September, Crown Prince Tupoutoʻa-ʻUlukalala's
/// birthday, as the lists give it. Both change with the monarch and the
/// heir, and no years are claimed for them. The Act's Anniversary of the
/// Coronation Day of the reigning Sovereign is not on either list and is
/// not carried. The forms of the weekday rule before the amendments of
/// 2010 and 2013 are not modelled.
pub static TONGA: RuleSet = RuleSet {
    code: "TO",
    english_name: "Tonga",
    rules: TO_RULES,
    substitution: TO_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act, Chapter 8.11, 2020 Revised Edition, section 2 and \
              endnotes (ago.gov.to); Prime Minister's Office media releases \"Tonga Public \
              Holidays for 2024\" (12 October 2023) and \"Tonga Public Holidays for 2026\" \
              (17 November 2025), in English and Tongan (pmo.gov.to), retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Tuvalu
// ─────────────────────────────────────────────────────────────────────────

/// Section 2(1) and (3): a Saturday or Sunday holiday gives the Monday
/// next following, and a Sunday or Monday 2 October or 26 December gives
/// the Tuesday as well — the next free weekday, taken in date order, which
/// is what the Tuvalu Day and Christmas pairs produce.
static TV_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static TV_MAY_SECOND_SUNDAY: Rule = Rule::nth(5, 2, Weekday::Sunday);

static TV_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public(
        "Gospel Day",
        "Te Aso o te Tala Lei",
        Rule::Offset {
            base: &TV_MAY_SECOND_SUNDAY,
            days: 1,
        },
    ),
    HolidayRule::public(
        "Sovereign's Birthday",
        "",
        Rule::nth(6, 2, Weekday::Saturday),
    ),
    HolidayRule::fixed_public("National Youth Day", "", Rule::nth(8, 1, Weekday::Monday)),
    HolidayRule::public("Tuvalu Day", "", Rule::gregorian(10, 1)),
    HolidayRule::public("Tuvalu Day", "", Rule::gregorian(10, 2)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Tuvalu.
///
/// The Public Holidays Act, Cap. 4.50 of the 2022 Revised Edition: the
/// Schedule as amended by Acts 9 of 2018 and 13 of 2020, and section 2's
/// weekend rule. The Sovereign's birthday is on the second Saturday of
/// June "or as appointed", so the Saturday and, by section 2(1), the
/// Monday after it. National Children's Day is on the "1st Monday after
/// White Sunday", and neither the Act nor any source read dates White
/// Sunday in Tuvalu, so it is not carried. The 2008 edition's Commonwealth
/// Day and its name National Children's Day for the first Monday of August
/// are gone from the 2022 Schedule, but the Act does not say which of the
/// two amendments removed them, and no years are claimed. The Minister's
/// notices, which can move any of these, were not read.
pub static TUVALU: RuleSet = RuleSet {
    code: "TV",
    english_name: "Tuvalu",
    rules: TV_RULES,
    substitution: TV_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act, Cap. 4.50, 2022 Revised Edition, and Cap. 22.10, 2008 \
              Revised Edition, section 2 and Schedule (tuvalu-legislation.tv), retrieved \
              2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Vanuatu
// ─────────────────────────────────────────────────────────────────────────

/// Section 3: a Sunday holiday gives the following Monday, and a Monday
/// Family Day the Tuesday — the next free weekday, which is what a Sunday
/// Christmas produces.
static VU_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static VU_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Lini Day", "", Rule::gregorian(2, 21)),
    HolidayRule::public("Custom Chief's Day", "", Rule::gregorian(3, 5)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Ascension Day", "", Rule::easter(ASCENSION)),
    HolidayRule::public("Children's National Day", "", Rule::gregorian(7, 24)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(7, 30)),
    HolidayRule::public("Assumption Day", "", Rule::gregorian(8, 15)),
    HolidayRule::public("Constitution Day", "", Rule::gregorian(10, 5)),
    HolidayRule::public("National Unity Day", "", Rule::gregorian(11, 29)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Family Day", "", Rule::gregorian(12, 26)),
];

/// Vanuatu.
///
/// The Public Holidays Act, Chapter 114, in the 2006 consolidation: the
/// Schedule's fourteen days and section 3's Sunday rule. The Schedule
/// does not date Independence Day, Assumption Day, Christmas or Family
/// Day, and puts National Unity Day on 5 October and Constitution Day on
/// 29 November; the Government's own holiday list and the Department of
/// Labour's brochure both give Constitution Day on 5 October and Unity
/// Day on 29 November, and date the rest, and the table follows them —
/// the days off are the same either way. The six provincial days those
/// two list are not in the Act and are not carried, nor are days the
/// President declares under section 2.
pub static VANUATU: RuleSet = RuleSet {
    code: "VU",
    english_name: "Vanuatu",
    rules: VU_RULES,
    substitution: VU_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act [Cap. 114], Consolidated Edition 2006, sections 1 to 3 \
              and Schedule (moia.gov.vu; the same text on NATLEX); Government of Vanuatu, \
              \"Holidays\" (gov.vu); Department of Labour, Industrial Relations Unit, \
              \"Public Holiday Brochure\" (dol.gov.vu); retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Samoa
// ─────────────────────────────────────────────────────────────────────────

/// Section 2's paragraphs (l) and (m): the Monday after a Sunday Christmas
/// Day, Boxing Day, New Year's Day or day after it, and the Tuesday after
/// a Sunday Christmas Day or New Year's Day — the next free weekday in
/// every case. Paragraph (l) names Mothers' Day, which is always a
/// Monday; the Ministry kept Independence Day, a Sunday in 2025, on
/// Monday 2 June, and the table follows the Ministry.
static WS_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static WS_MAY_SECOND_SUNDAY: Rule = Rule::nth(5, 2, Weekday::Sunday);
static WS_AUGUST_SECOND_SUNDAY: Rule = Rule::nth(8, 2, Weekday::Sunday);
static WS_OCTOBER_SECOND_SUNDAY: Rule = Rule::nth(10, 2, Weekday::Sunday);

static WS_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Day after New Year's Day", "", Rule::gregorian(1, 2)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public(
        "Saturday after Good Friday",
        "",
        Rule::easter(HOLY_SATURDAY),
    ),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public(
        "Mothers' Day",
        "",
        Rule::Offset {
            base: &WS_MAY_SECOND_SUNDAY,
            days: 1,
        },
    ),
    HolidayRule::public("Independence Day", "Aso Tutoʻatasi", Rule::gregorian(6, 1)),
    HolidayRule::fixed_public(
        "Fathers' Day",
        "",
        Rule::Offset {
            base: &WS_AUGUST_SECOND_SUNDAY,
            days: 1,
        },
    ),
    HolidayRule::fixed_public(
        "White Sunday Holiday",
        "Lotu a Tamaiti",
        Rule::Offset {
            base: &WS_OCTOBER_SECOND_SUNDAY,
            days: 1,
        },
    ),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Samoa.
///
/// The Public Holidays Act 2008, as the Attorney General's revision of
/// 31 December 2023 prints it: section 2's definition of a public holiday
/// and its Monday and Tuesday for a Sunday, with the Ministry of Commerce,
/// Industry and Labour's calendar for 2025 to 2027 for Independence Day's
/// date, 1 June, which the Act does not give. Mothers' Day, Fathers' Day
/// and the White Sunday holiday are the Mondays after the second Sundays
/// of May, August and October. Polling day and the day before it in a
/// general election, and days the Head of State declares under section 7,
/// are announced and not carried.
pub static SAMOA: RuleSet = RuleSet {
    code: "WS",
    english_name: "Samoa",
    rules: WS_RULES,
    substitution: WS_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act 2008, section 2, revised to 31 December 2023 \
              (ag.gov.ws); Ministry of Commerce, Industry and Labour, \"Public holidays \
              calendar\" for 2025 to 2027 (mcil.gov.ws), retrieved 2026-09-23",
};
