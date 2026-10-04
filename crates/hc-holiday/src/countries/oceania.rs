//! Australia, New Zealand and the Pacific island states.

use hc_calendar::Weekday;

use super::read_all;
use super::weekends;
use crate::computus::offsets::{
    ASCENSION, EASTER_MONDAY, EASTER_SUNDAY, EASTER_TUESDAY, GOOD_FRIDAY, HOLY_SATURDAY,
    WHIT_MONDAY,
};
use crate::rule::{
    HolidayRule, Kind, ListedEntry, Listing, Rule, RuleSet, SourceDate, Subdivisions,
    SubstituteDirection, SubstitutionPolicy,
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
const AU_NSW: &[&str] = &["AU-NSW"];
const AU_EASTER_SATURDAY: &[&str] = &["AU-ACT", "AU-NSW", "AU-NT", "AU-QLD", "AU-SA", "AU-VIC"];
const AU_EASTER_SUNDAY: &[&str] = &["AU-ACT", "AU-NSW", "AU-QLD", "AU-VIC"];
const AU_JUNE_SOVEREIGN: &[&str] = &["AU-ACT", "AU-NSW", "AU-NT", "AU-SA", "AU-TAS", "AU-VIC"];
const AU_OCTOBER_LABOUR: &[&str] = &["AU-ACT", "AU-NSW", "AU-SA"];
const AU_MAY_LABOUR: &[&str] = &["AU-NT", "AU-QLD"];

/// The first year the sources read answer for: the Holidays Act 1983 (Qld)
/// and the Statutory Holidays Act 2000 (Tas), the two Acts read, the later of
/// which began in 2000, for the days every state keeps.
const AU_READ_FROM: i32 = 2000;

/// The first year the sources read answer for, in a day of one state or
/// territory: the NSW Government's list of 2026 and secondary sources, for
/// the days of one state or territory whose own Act was not read.
const AU_SCOPED_READ_FROM: i32 = 2026;

static AU_RULES: &[HolidayRule] = &read_all(
    AU_READ_FROM,
    [
        HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
        HolidayRule::public("Australia Day", "", Rule::gregorian(1, 26)),
        HolidayRule::public("Labour Day", "", Rule::nth(3, 1, Weekday::Monday))
            .in_regions(AU_WA)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Labour Day", "", Rule::nth(3, 2, Weekday::Monday))
            .in_regions(AU_VIC)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Eight Hours Day", "", Rule::nth(3, 2, Weekday::Monday))
            .in_regions(AU_TAS)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Canberra Day", "", Rule::nth(3, 2, Weekday::Monday))
            .in_regions(AU_ACT)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Adelaide Cup Day", "", Rule::nth(3, 2, Weekday::Monday))
            .in_regions(AU_SA)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
        // Easter Saturday and Easter Sunday are holidays where they are holidays
        // at all; no state moves them, because the Monday is already one.
        HolidayRule::fixed_public("Easter Saturday", "", Rule::easter(HOLY_SATURDAY))
            .in_regions(AU_EASTER_SATURDAY)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::fixed_public("Easter Sunday", "", Rule::easter(EASTER_SUNDAY))
            .in_regions(AU_EASTER_SUNDAY)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
        // Anzac Day stays on 25 April when it falls on a weekend. New South
        // Wales adds a day on the Monday after in 2026 and 2027, a trial, by
        // orders under s 5 of its Public Holidays Act 2010; the other states'
        // and territories' rules for a weekend Anzac Day were not read and are
        // not modelled.
        HolidayRule::fixed_public("Anzac Day", "", Rule::gregorian(4, 25)).years(Some(1921), None),
        HolidayRule::fixed_public(
            "Anzac Day additional holiday",
            "",
            Rule::WeekdayOnOrAfter {
                month: 4,
                day: 26,
                weekday: Weekday::Monday,
            },
        )
        .in_regions(AU_NSW)
        .years(Some(2026), Some(2027)),
        HolidayRule::public("Labour Day", "", Rule::nth(5, 1, Weekday::Monday))
            .in_regions(AU_MAY_LABOUR)
            .read_from(AU_SCOPED_READ_FROM),
        // The first Monday on or after 27 May, as the secondary sources state it;
        // the ACT's Holidays Act 1958 was not read.
        HolidayRule::public(
            "Reconciliation Day",
            "",
            Rule::WeekdayOnOrAfter {
                month: 5,
                day: 27,
                weekday: Weekday::Monday,
            },
        )
        .in_regions(AU_ACT)
        .years(Some(2018), None),
        HolidayRule::public(
            "Western Australia Day",
            "",
            Rule::nth(6, 1, Weekday::Monday),
        )
        .in_regions(AU_WA)
        .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Sovereign's Birthday", "", Rule::nth(6, 2, Weekday::Monday))
            .in_regions(AU_JUNE_SOVEREIGN)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Picnic Day", "", Rule::nth(8, 1, Weekday::Monday))
            .in_regions(AU_NT)
            .read_from(AU_SCOPED_READ_FROM),
        // New South Wales's Bank Holiday: retail bank branches close, and it is
        // not a public holiday.
        HolidayRule::fixed_public("Bank Holiday", "", Rule::nth(8, 1, Weekday::Monday))
            .of_kind(Kind::Bank)
            .in_regions(AU_NSW)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public(
            "Sovereign's Birthday",
            "",
            Rule::nth(10, 1, Weekday::Monday),
        )
        .in_regions(AU_QLD)
        .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Labour Day", "", Rule::nth(10, 1, Weekday::Monday))
            .in_regions(AU_OCTOBER_LABOUR)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Melbourne Cup Day", "", Rule::nth(11, 1, Weekday::Tuesday))
            .in_regions(AU_VIC)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
        // South Australia calls 26 December Proclamation Day; everywhere else it
        // is Boxing Day.
        HolidayRule::public("Proclamation Day", "", Rule::gregorian(12, 26))
            .in_regions(AU_SA)
            .read_from(AU_SCOPED_READ_FROM),
        HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
    ],
);

static AU_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    regions: &[],
    avoid: &[],
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
    weekend: weekends::AU,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "Fair Work Act 2009 (Cth), s 115, compilation C2026C00355 of 7 July 2026, its \
              text not read; the Holidays Act 1983 (Qld) and the Statutory Holidays Act 2000 \
              (Tas), read on legislation.qld.gov.au and legislation.tas.gov.au, retrieved \
              2026-09-26; the Public Holidays Act 2010 (NSW), Public Holidays Act 1993 (Vic), \
              Public Holidays Act 2023 (SA), Public and Bank Holidays Act 1972 (WA), Holidays \
              Act 1958 (ACT) and Public Holidays Act 1981 (NT), and the annual gazettals, not \
              read; for New South Wales, the NSW Government's \"NSW public holidays\" \
              (nsw.gov.au/about-nsw/public-holidays), its \"Additional Day\" of Monday \
              27 April 2026 and Monday 26 April 2027 after Anzac Day and its note that the \
              Bank Holiday of the first Monday in August closes retail bank branches under \
              Part 3A of the Retail Trading Act 2008 and \"is not a declared public \
              holiday\", and the Premier's release \"Minns Labor Government announces extra \
              public holiday this year\", which makes the two Anzac Mondays a trial to be \
              reviewed from 2027, both retrieved 2026-09-27 (`nsw-public-holidays`); the \
              orders under s 5 of the NSW Act and the Retail Trading Act not read. Western Australia's Sovereign's Birthday is proclaimed each year and is \
              not modelled; nor are the show days, the Royal Queensland Show's among them, \
              which s 4(5)(a) of the Queensland Act lets the Minister appoint for the City of \
              Brisbane only; nor the part-day holidays of Christmas Eve",
    subdivisions: Subdivisions::Read(&[]),
};

// ─────────────────────────────────────────────────────────────────────────
// New Zealand
// ─────────────────────────────────────────────────────────────────────────

/// Matariki, the Māori new year, as the statute schedules it.
///
/// The Te Kāhui o Matariki Public Holiday Act 2022 does not give a rule; it
/// gives a *table*, because the date is the Friday nearest the Tangaroa
/// nights of the lunar month in which the Pleiades rise, as determined by
/// the Matariki Advisory Group. No rule in the vocabulary expresses that, so
/// this table holds the schedule verbatim.
///
/// The Act schedules dates through 2052. Only the years published in the
/// sources this crate checked are carried here; a year outside them yields
/// nothing rather than an invented Friday. "Nothing" must not read as "no
/// holiday that year", so the rule is a [`Rule::Listed`] with its last
/// year written down, and a calendar built past 2035 reports Matariki as a
/// gap instead of dropping it.
#[rustfmt::skip]
static MATARIKI: Listing = Listing::Dates(&[
    (2022, 6, 24), (2023, 7, 14), (2024, 6, 28), (2025, 6, 20),
    (2026, 7, 10), (2027, 6, 25), (2028, 7, 14), (2029, 7, 6),
    (2030, 6, 21), (2031, 7, 11), (2032, 7, 2), (2033, 6, 24),
    (2034, 7, 7), (2035, 6, 29),
]);

/// The first year the sources read answer for: Employment New Zealand's list
/// of 2010 to 2025, the oldest dated list read; the Acts were not read.
const NZ_READ_FROM: i32 = 2010;

pub(super) static NZ_RULES: &[HolidayRule] = &read_all(
    NZ_READ_FROM,
    [
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
            Rule::listed(MATARIKI.every(), 2022, 2035),
        )
        .years(Some(2022), None),
        HolidayRule::public("Labour Day", "", Rule::nth(10, 4, Weekday::Monday)),
        HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
        HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
    ],
);

static NZ_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

/// New Zealand, with the provincial anniversary days in the regions that
/// bear their provinces' names ([`super::new_zealand`]).
pub static NEW_ZEALAND: RuleSet = RuleSet {
    code: "NZ",
    english_name: "New Zealand",
    rules: &super::new_zealand::RULES,
    substitution: NZ_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: weekends::NZ,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Holidays Act 2003, sections 44, 45 and 45A, and the Holidays (Full Recognition \
              of Waitangi Day and ANZAC Day) Amendment Act 2013; Te Kāhui o Matariki Public \
              Holiday Act 2022, Schedule 1, for the Matariki dates; legislation.govt.nz \
              refused access on 2026-09-26, so the Acts were not read, and the dates were \
              checked against Employment New Zealand's \"Public holidays and anniversary \
              dates\" (employment.govt.nz), retrieved 2026-09-26, and for Matariki 2028 to \
              2035 Wikipedia, \"Matariki\" (secondary); the provincial anniversary days from \
              Employment New Zealand's list for 2026 and 2027 and its \"Previous years: \
              Public holidays and anniversary dates\" for 2010 to 2025, retrieved \
              2026-09-29, as src/countries/new_zealand.rs gives them",
    subdivisions: Subdivisions::Read(&[]),
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
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

/// The first year the sources read answer for: the Code of the Federated
/// States of Micronesia (2014), title 1, chapter 6.
const FM_READ_FROM: i32 = 2014;

static FM_RULES: &[HolidayRule] = &read_all(
    FM_READ_FROM,
    [
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
    ],
);

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
    weekend: weekends::FM,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Code of the Federated States of Micronesia (2014), title 1, chapter 6, \
              sections 601 to 603, and Public Laws 13-38, 16-27 and 21-209, as the FSM \
              Legal Information System publishes them (fsmlaw.org), retrieved 2026-09-23",
    subdivisions: Subdivisions::Read(&[]),
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
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

/// The first year the sources read answer for: the Public Holidays Act 1988.
const MH_READ_FROM: i32 = 1988;

static MH_RULES: &[HolidayRule] = &read_all(
    MH_READ_FROM,
    [
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
    ],
);

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
    weekend: weekends::MH,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act 1988, 1 MIRC Ch. 9, the Nitijela's consolidation \
              (rmiparliament.org, 1988-0016_2.pdf) as the Internet Archive holds it, \
              captured 2025-04-04, the Nitijela's site refusing this session's requests; \
              retrieved 2026-09-23",
    subdivisions: Subdivisions::Read(&[]),
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
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

/// The days the President declared in the gazettes this table read: those
/// of 2023, 2024 and 2026, and three that only the 2026 gazette declared.
#[rustfmt::skip]
static NR_DECLARED: Listing = Listing::Named(&[
    (2023, 3, 8, "women"), (2023, 7, 1, "ronphos"), (2023, 8, 19, "ibumin-earoeni"),
    (2023, 9, 25, "hammer-deroburt"),
    (2024, 3, 8, "women"), (2024, 7, 1, "ronphos"), (2024, 8, 19, "ibumin-earoeni"),
    (2024, 9, 25, "hammer-deroburt"),
    (2026, 3, 8, "women"), (2026, 6, 26, "eigigu"), (2026, 6, 29, "remembrance"),
    (2026, 7, 1, "ronphos"), (2026, 7, 25, "judicial-well-being"), (2026, 8, 19, "ibumin-earoeni"),
    (2026, 9, 25, "hammer-deroburt"),
]);

/// A day declared in the 2023, 2024 and 2026 gazettes, as two tables: 2023
/// to 2024, and 2026 standing in for 2025 onwards, so that 2025, whose
/// gazette was not read, and every year after 2026 are reported as gaps
/// rather than as years without the day.
const fn nr_gazetted(name: &'static str, entry: ListedEntry) -> [HolidayRule; 2] {
    [
        HolidayRule::public(name, "", Rule::listed(entry, 2023, 2024)).years(None, Some(2024)),
        HolidayRule::public(name, "", Rule::listed(entry, 2026, 2026)).years(Some(2025), None),
    ]
}

/// A day the 2026 gazette alone declared, known for 2026 and a gap in
/// every other year.
const fn nr_gazetted_2026(name: &'static str, entry: ListedEntry) -> HolidayRule {
    HolidayRule::public(name, "", Rule::listed(entry, 2026, 2026))
}

const NR_WOMEN_DAY: [HolidayRule; 2] =
    nr_gazetted("International Women's Day", NR_DECLARED.named("women"));
const NR_RONPHOS_HANDOVER: [HolidayRule; 2] =
    nr_gazetted("RONPHOS Handover", NR_DECLARED.named("ronphos"));
const NR_IBUMIN_EAROENI_DAY: [HolidayRule; 2] =
    nr_gazetted("Ibumin Earoeni Day", NR_DECLARED.named("ibumin-earoeni"));
const NR_HAMMER_DEROBURT_DAY: [HolidayRule; 2] = nr_gazetted(
    "Sir Hammer DeRoburt Day",
    NR_DECLARED.named("hammer-deroburt"),
);

/// The first year the sources read answer for: the Government Gazette of 30
/// December 2022, the list for 2023, the oldest dated list read.
const NR_READ_FROM: i32 = 2023;

static NR_RULES: &[HolidayRule] = &read_all(
    NR_READ_FROM,
    [
        HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
        HolidayRule::public("Independence Day", "", Rule::gregorian(1, 31)),
        HolidayRule::public("Day following Independence Day", "", Rule::gregorian(2, 1)),
        NR_WOMEN_DAY[0],
        NR_WOMEN_DAY[1],
        HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
        HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
        HolidayRule::fixed_public("Easter Tuesday", "", Rule::easter(EASTER_TUESDAY)),
        HolidayRule::public("Constitution Day", "", Rule::gregorian(5, 17)),
        nr_gazetted_2026("Eigigu Day", NR_DECLARED.named("eigigu")),
        nr_gazetted_2026("Remembrance Day", NR_DECLARED.named("remembrance")),
        NR_RONPHOS_HANDOVER[0],
        NR_RONPHOS_HANDOVER[1],
        nr_gazetted_2026(
            "International Day for Judicial Well-Being",
            NR_DECLARED.named("judicial-well-being"),
        ),
        NR_IBUMIN_EAROENI_DAY[0],
        NR_IBUMIN_EAROENI_DAY[1],
        NR_HAMMER_DEROBURT_DAY[0],
        NR_HAMMER_DEROBURT_DAY[1],
        HolidayRule::public("Angam Day", "", Rule::gregorian(10, 26)),
        HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
        HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
    ],
);

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
    weekend: weekends::NR,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Service Act 2016, section 81, RONLAW consolidation (service 6); \
              Government Gazette No. 330 of 30 December 2022 \
              (G.N. 1350/2022), No. 14 of 15 January 2024 and No. 7 of 9 January 2026 \
              (G.N. 28/2026), the public-holiday lists for 2023, 2024 and 2026 \
              (ronlaw.gov.nr), retrieved 2026-09-23",
    subdivisions: Subdivisions::Read(&[]),
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
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

/// The first year the sources read answer for: the Palau National Code
/// Annotated, Supp. 12, whose date was not read, retrieved 2026-09-23.
const PW_READ_FROM: i32 = 2026;

static PW_RULES: &[HolidayRule] = &read_all(
    PW_READ_FROM,
    [
        HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
        HolidayRule::public("Youth Day", "", Rule::gregorian(3, 15)),
        HolidayRule::public("Senior Citizens Day", "", Rule::gregorian(5, 5)),
        HolidayRule::public("Constitution Day", "", Rule::gregorian(7, 9)),
        HolidayRule::fixed_public("Labor Day", "", Rule::nth(9, 1, Weekday::Monday)),
        HolidayRule::public("United Nations Day", "", Rule::gregorian(10, 24)),
        HolidayRule::fixed_public("Thanksgiving Day", "", Rule::nth(11, 4, Weekday::Thursday)),
        HolidayRule::fixed_public("Family Day", "", Rule::nth(11, 4, Weekday::Friday)),
        HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    ],
);

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
    weekend: weekends::PW,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Palau National Code Annotated, title 1, sections 701 and 702 (Supp. 12), \
              PacLII's copy (pncgpt1409.pdf) as the Internet Archive holds it, captured \
              2025-12-06, PacLII refusing this session's requests; retrieved 2026-09-23",
    subdivisions: Subdivisions::Read(&[]),
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
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

/// Nothing: no National Gazette was read, so the days it appoints are a
/// gap in every year.
///
/// A day the Head of State appoints each year by notice in the Gazette.
const PG_GAZETTED: Rule = Rule::UNREAD;

/// The first year the sources read answer for: the 1982 revised edition of
/// the Public Holidays Act 1953 (Chapter 321).
const PG_READ_FROM: i32 = 1982;

static PG_RULES: &[HolidayRule] = &read_all(
    PG_READ_FROM,
    [
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
    ],
);

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
    weekend: weekends::PG,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act 1953 (Chapter 321), sections 1 to 5, PacLII's \
              consolidation (pha1953163) and its 1982 revised-edition PDF as the \
              Internet Archive holds them, captured 2025-01-30 and 2025-01-01, PacLII \
              refusing this session's requests; retrieved 2026-09-23. Independence Day \
              and the Sovereign's Birthday are reported as gaps, and the other gazetted days \
              are not modelled",
    subdivisions: Subdivisions::Read(&[]),
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
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

/// The first year the sources read answer for: the 1996 edition of the Public
/// Holidays Act (Cap. 151).
const SB_READ_FROM: i32 = 1996;

pub(super) static SB_RULES: &[HolidayRule] = &read_all(
    SB_READ_FROM,
    [
        HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
        HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
        HolidayRule::fixed_public("Holy Saturday", "", Rule::easter(HOLY_SATURDAY)),
        HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
        HolidayRule::fixed_public("Whit Monday", "", Rule::easter(WHIT_MONDAY)),
        HolidayRule::public("Independence Day", "", Rule::gregorian(7, 7)),
        HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
        HolidayRule::public("National Day of Thanksgiving", "", Rule::gregorian(12, 26)),
    ],
);

/// Solomon Islands.
///
/// The Public Holidays Act, Chapter 151, in the 1996 edition: the
/// Schedule's days and section 2's Sunday rule, with the names the
/// Ministry of Home Affairs' notices use. The Schedule's "day appointed
/// for the celebration of the Anniversary of the Birthday of the
/// Sovereign" is appointed each year — the notices for 2018 and 2020, PDFs
/// that were not read, gave the second Saturday of June as a search
/// engine's extract of the 2018 one quotes it, and the 2026 gazette as
/// reported gave none — and is not carried. The notices also keep a Saturday
/// holiday on the Friday before, which the Act does not say, and that is
/// not modelled. The provincial days are appointed each year under
/// section 6; those of 2026 are carried in their provinces
/// ([`super::solomon_islands`]), and a later year reports them as a gap.
pub static SOLOMON_ISLANDS: RuleSet = RuleSet {
    code: "SB",
    english_name: "Solomon Islands",
    rules: &super::solomon_islands::RULES,
    substitution: SB_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: weekends::SB,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Public Holidays Act (Cap. 151), 1996 edition, PacLII's consolidation \
              (pha163) as the Internet Archive holds it, captured 2024-12-22, PacLII \
              refusing this session's requests; the Ministry of Home Affairs' Public \
              Notices 1/2017 and 1/2019 for 2018 and 2020 (mehrd.gov.sb, solomons.gov.sb) \
              are PDFs, not read, their Sovereign's Birthday as a search engine's extract \
              quotes it; the Island Sun, 13 January 2026, on the 2026 gazette; retrieved 2026-09-23; \
              the same Island Sun report for the provincial days of 2026, retrieved \
              2026-09-29",
    subdivisions: Subdivisions::Read(&["SB-CT"]),
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
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

/// The first year the sources read answer for: the Public Holidays Act,
/// Chapter 8.11, 2020 Revised Edition.
const TO_READ_FROM: i32 = 2020;

static TO_RULES: &[HolidayRule] = &read_all(
    TO_READ_FROM,
    [
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
    ],
);

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
    weekend: weekends::TO,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act, Chapter 8.11, 2020 Revised Edition, section 2 and \
              endnotes (ago.gov.to); Prime Minister's Office media releases \"Tonga Public \
              Holidays for 2024\" (12 October 2023) and \"Tonga Public Holidays for 2026\" \
              (17 November 2025), in English and Tongan (pmo.gov.to), retrieved 2026-09-23",
    subdivisions: Subdivisions::Read(&[]),
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
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

static TV_MAY_SECOND_SUNDAY: Rule = Rule::nth(5, 2, Weekday::Sunday);

/// The first year the sources read answer for: the Public Holidays Act, Cap.
/// 4.50, 2022 Revised Edition.
const TV_READ_FROM: i32 = 2022;

static TV_RULES: &[HolidayRule] = &read_all(
    TV_READ_FROM,
    [
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
    ],
);

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
    weekend: weekends::TV,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act, Cap. 4.50, 2022 Revised Edition, and Cap. 22.10, 2008 \
              Revised Edition, section 2 and Schedule (tuvalu-legislation.tv), retrieved \
              2026-09-23",
    subdivisions: Subdivisions::Read(&[]),
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
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

/// The first year the sources read answer for: the Public Holidays Act [Cap.
/// 114], Consolidated Edition 2006.
const VU_READ_FROM: i32 = 2006;

pub(super) static VU_RULES: &[HolidayRule] = &read_all(
    VU_READ_FROM,
    [
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
    ],
);

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
/// two list are not in the Act; they are carried in their provinces from
/// the Government's list ([`super::vanuatu`]). The days the President
/// declares under section 2 are not yet carried: each is declared by a
/// notice of its own, and no notice was read.
pub static VANUATU: RuleSet = RuleSet {
    code: "VU",
    english_name: "Vanuatu",
    rules: &super::vanuatu::RULES,
    substitution: VU_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: weekends::VU,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Public Holidays Act [Cap. 114], Consolidated Edition 2006, sections 1 to 3 \
              and Schedule (moia.gov.vu; the same text on NATLEX); Government of Vanuatu, \
              \"Holidays\" (gov.vu); Department of Labour, Industrial Relations Unit, \
              \"Public Holiday Brochure\" (dol.gov.vu); retrieved 2026-09-23; the \
              Government's list again for the provincial days, retrieved 2026-09-29",
    subdivisions: Subdivisions::Read(&[]),
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
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

static WS_MAY_SECOND_SUNDAY: Rule = Rule::nth(5, 2, Weekday::Sunday);
static WS_AUGUST_SECOND_SUNDAY: Rule = Rule::nth(8, 2, Weekday::Sunday);
static WS_OCTOBER_SECOND_SUNDAY: Rule = Rule::nth(10, 2, Weekday::Sunday);

/// The first year the sources read answer for: the Public Holidays Act 2008
/// as revised to 31 December 2023.
const WS_READ_FROM: i32 = 2023;

static WS_RULES: &[HolidayRule] = &read_all(
    WS_READ_FROM,
    [
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
    ],
);

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
    weekend: weekends::WS,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act 2008, section 2, revised to 31 December 2023 \
              (ag.gov.ws); Ministry of Commerce, Industry and Labour, \"Public holidays \
              calendar\" for 2025 to 2027 (mcil.gov.ws), retrieved 2026-09-23",
    subdivisions: Subdivisions::Read(&[]),
};

// ─────────────────────────────────────────────────────────────────────────
// The lists read day by day
// ─────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────
// Fiji
// ─────────────────────────────────────────────────────────────────────────

/// The first year of the Ministry of Information's lists carried.
const FJ_FIRST: i64 = 2019;
/// The last.
const FJ_LAST: i64 = 2026;

/// Every day of the lists for 2019 to 2026 but the three of Easter, as the
/// lists date them.
static FJ_LISTED: Listing = Listing::Named(&[
    // 2019
    (2019, 1, 1, "New Year's Day"),
    (2019, 9, 9, "Constitution Day"),
    (2019, 10, 10, "Fiji Day"),
    (2019, 10, 28, "Diwali"),
    (2019, 11, 11, "Prophet Mohammed's Birthday"),
    (2019, 12, 25, "Christmas Day"),
    (2019, 12, 26, "Boxing Day"),
    // 2020
    (2020, 1, 1, "New Year's Day"),
    (2020, 9, 7, "Constitution Day"),
    (2020, 10, 10, "Fiji Day"),
    (2020, 11, 2, "Prophet Mohammed's Birthday"),
    (2020, 11, 16, "Diwali"),
    (2020, 12, 25, "Christmas Day"),
    (2020, 12, 28, "Boxing Day"),
    // 2021
    (2021, 1, 1, "New Year's Day"),
    (2021, 9, 7, "Constitution Day"),
    (2021, 10, 10, "Fiji Day"),
    (2021, 10, 18, "Prophet Mohammed's Birthday"),
    (2021, 11, 4, "Diwali"),
    (2021, 12, 27, "Christmas Day"),
    (2021, 12, 28, "Boxing Day"),
    // 2022
    (2022, 1, 3, "New Year's Day"),
    (2022, 9, 7, "Constitution Day"),
    (2022, 10, 7, "Prophet Mohammed's Birthday"),
    (2022, 10, 10, "Fiji Day"),
    (2022, 10, 25, "Diwali"),
    (2022, 12, 26, "Christmas Day"),
    (2022, 12, 27, "Boxing Day"),
    // 2023
    (2023, 1, 2, "New Year's Day"),
    (2023, 5, 15, "Girmit Day"),
    (2023, 5, 29, "Ratu Sir Lala Sukuna Day"),
    (2023, 10, 2, "Prophet Mohammed's Birthday"),
    (2023, 10, 10, "Fiji Day"),
    (2023, 11, 13, "Diwali"),
    (2023, 12, 25, "Christmas Day"),
    (2023, 12, 26, "Boxing Day"),
    // 2024
    (2024, 1, 1, "New Year's Day"),
    (2024, 5, 13, "Girmit Day"),
    (2024, 5, 31, "Ratu Sir Lala Sukuna Day"),
    (2024, 9, 16, "Prophet Mohammed's Birthday"),
    (2024, 10, 10, "Fiji Day"),
    (2024, 11, 1, "Diwali"),
    (2024, 12, 25, "Christmas Day"),
    (2024, 12, 26, "Boxing Day"),
    // 2025
    (2025, 1, 1, "New Year's Day"),
    (2025, 5, 12, "Girmit Day"),
    (2025, 5, 30, "Ratu Sir Lala Sukuna Day"),
    (2025, 9, 8, "Prophet Mohammed's Birthday"),
    (2025, 10, 10, "Fiji Day"),
    (2025, 10, 21, "Diwali"),
    (2025, 12, 25, "Christmas Day"),
    (2025, 12, 26, "Boxing Day"),
    // 2026
    (2026, 1, 1, "New Year's Day"),
    (2026, 5, 15, "Girmit Day"),
    (2026, 5, 29, "Ratu Sir Lala Sukuna Day"),
    (2026, 8, 24, "Prophet Mohammed's Birthday"),
    (2026, 10, 10, "Fiji Day"),
    (2026, 11, 9, "Diwali"),
    (2026, 12, 25, "Christmas Day"),
    (2026, 12, 28, "Boxing Day"),
]);

/// A day the lists give, for the years they cover.
const fn fj(name: &'static str, entry: ListedEntry) -> HolidayRule {
    HolidayRule::fixed_public(name, "", Rule::listed(entry, FJ_FIRST, FJ_LAST))
}

/// The first year the sources read answer for: the Public Holidays Act (Cap.
/// 101), 1985 edition.
const FJ_READ_FROM: i32 = 1985;

static FJ_RULES: &[HolidayRule] = &read_all(
    FJ_READ_FROM,
    [
        fj("New Year's Day", FJ_LISTED.named("New Year's Day")),
        HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
        HolidayRule::fixed_public("Easter Saturday", "", Rule::easter(HOLY_SATURDAY)),
        HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
        fj("Girmit Day", FJ_LISTED.named("Girmit Day")),
        fj(
            "Ratu Sir Lala Sukuna Day",
            FJ_LISTED.named("Ratu Sir Lala Sukuna Day"),
        ),
        fj("Constitution Day", FJ_LISTED.named("Constitution Day")),
        fj(
            "Prophet Mohammed's Birthday",
            FJ_LISTED.named("Prophet Mohammed's Birthday"),
        ),
        fj("Fiji Day", FJ_LISTED.named("Fiji Day")),
        fj("Diwali", FJ_LISTED.named("Diwali")),
        fj("Christmas Day", FJ_LISTED.named("Christmas Day")),
        fj("Boxing Day", FJ_LISTED.named("Boxing Day")),
    ],
);

/// Fiji — the public holidays as the Government's own yearly lists give
/// them, 2019 to 2026.
///
/// The Public Holidays Act (Cap. 101) keeps the days of its Schedule as
/// "close holidays" and gives the Monday for one on a Sunday "unless
/// otherwise ordered by the Minister by notification in the Gazette";
/// section 6 lets the Minister appoint special days. The Schedule has been
/// amended since the 1985 edition read here — the amendments of 1994 and
/// 2003 were read, any later ones not — and the days actually kept are the
/// ones Cabinet approves and the Ministry of Information publishes each
/// year: Constitution Day in the lists for 2019 to 2022 and not after,
/// Girmit Day and Ratu Sir Lala Sukuna Day from the 2023 list, the
/// Prophet's Birthday and Diwali on the days announced, and a holiday moved
/// off a weekend in some years — New Year's Day 2022 and 2023, Christmas
/// 2021 and 2022, Boxing Day 2020 and 2026 — and left on it in others, Fiji Day on
/// the Saturday of 2020 and 2026 and the Sunday of 2021. No rule reproduces
/// that, so every day but the three of Easter, which the Schedule names
/// and every list keeps where it falls, is taken from the lists, and a year
/// outside 2019–2026 reports them as a gap rather than a guess. Special
/// days the Minister appoints, such as an election day, are not carried.
/// The weekend is Saturday and Sunday, the days the lists move a holiday
/// off when they move one.
///
/// This table is written up with the other twelve whose days are
/// announced year by year in `docs/systems/announced-holidays.md`.
pub static FIJI: RuleSet = RuleSet {
    code: "FJ",
    english_name: "Fiji",
    rules: FJ_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: weekends::FJ,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act (Cap. 101), 1985 edition, sections 2 and 6 and Schedule, and \
              the Public Holidays (Amendment) Acts 1994 (No. 14 of 1995) and 2003 (No. 8 of \
              2003), PacLII's copies as the Internet Archive holds them, captured 2013-07-15 \
              and 2009-01-08; Ministry of Information, \"Public Holidays\" \
              (fiji.gov.fj/About-Fiji/Public-Holidays), the lists for 2019 to 2026 as the \
              Internet Archive holds them, captured 2019-11-19, 2020-11-27, 2022-12-23, \
              2023-05-04, 2024-04-18, 2025-09-09 and 2026-04-11, the 2026 list also at \
              fiji.gov.fj/public-holidays captured 2026-08-12, fiji.gov.fj refusing this \
              session's requests; retrieved 2026-09-23",
    subdivisions: Subdivisions::Read(&[]),
};

// ─────────────────────────────────────────────────────────────────────────
// Kiribati
// ─────────────────────────────────────────────────────────────────────────

/// The first year of the Beretitenti's orders carried.
const KI_FIRST: i64 = 2025;
/// The last.
const KI_LAST: i64 = 2026;

/// Every day of the orders for 2025 (as revised) and 2026 but Good Friday
/// and Easter Monday, under one name for each holiday: the "in honour of"
/// day an order gives beside or instead of the day itself is listed under
/// the holiday's name.
static KI_LISTED: Listing = Listing::Named(&[
    // 2025, the revised order of 18 December 2025
    (2025, 1, 1, "New Year's Day"),
    (2025, 3, 7, "International Women's Day"),
    (2025, 4, 7, "National Health Day"),
    (2025, 4, 25, "Special Day in honour of Pope Francis"),
    (2025, 5, 2, "International Labour Day"),
    (2025, 6, 23, "National Police Day"),
    (2025, 7, 11, "Gospel Day"),
    (2025, 7, 12, "National Day"),
    (2025, 7, 14, "National Day"),
    (2025, 7, 15, "Kiribati Culture and Senior Citizens Day"),
    (2025, 7, 16, "Kiribati Special Day"),
    (2025, 8, 1, "National Youth and Children's Day"),
    (2025, 10, 6, "World Teachers' Day"),
    (2025, 12, 12, "Human Rights Day"),
    (2025, 12, 25, "Christmas Day"),
    (2025, 12, 26, "Boxing Day"),
    (2025, 12, 29, "Kiribati Holiday"),
    (2025, 12, 30, "Kiribati Holiday"),
    (2025, 12, 31, "Kiribati Holiday"),
    // 2026, the order of 18 December 2025
    (2026, 1, 1, "New Year's Day"),
    (2026, 1, 2, "Kiribati Holiday"),
    (2026, 3, 9, "International Women's Day"),
    (2026, 4, 7, "National Health Day"),
    (2026, 5, 1, "International Labour Day"),
    (2026, 6, 22, "National Police Day"),
    (2026, 7, 10, "Gospel Day"),
    (2026, 7, 13, "National Day"),
    (2026, 7, 14, "Kiribati Culture and Senior Citizens Day"),
    (2026, 7, 15, "Kiribati Special Day"),
    (2026, 8, 3, "National Youth and Children's Day"),
    (2026, 10, 5, "World Teachers' Day"),
    (2026, 12, 11, "Human Rights Day"),
    (2026, 12, 25, "Christmas Day"),
    (2026, 12, 28, "Boxing Day"),
    (2026, 12, 29, "Kiribati Holiday"),
    (2026, 12, 30, "Kiribati Holiday"),
    (2026, 12, 31, "Kiribati Holiday"),
]);

/// A day the orders declare, for the years they cover.
const fn ki(name: &'static str, entry: ListedEntry) -> HolidayRule {
    HolidayRule::fixed_public(name, "", Rule::listed(entry, KI_FIRST, KI_LAST))
}

/// The first year the sources read answer for: the Public Holidays Ordinance
/// (Cap. 81), 1977 revised edition.
const KI_READ_FROM: i32 = 1977;

static KI_RULES: &[HolidayRule] = &read_all(
    KI_READ_FROM,
    [
        ki("New Year's Day", KI_LISTED.named("New Year's Day")),
        ki("Kiribati Holiday", KI_LISTED.named("Kiribati Holiday")),
        ki(
            "International Women's Day",
            KI_LISTED.named("International Women's Day"),
        ),
        HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
        HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
        ki(
            "National Health Day",
            KI_LISTED.named("National Health Day"),
        ),
        // Declared by the revised order for 2025 alone.
        ki(
            "Special Day in honour of Pope Francis",
            KI_LISTED.named("Special Day in honour of Pope Francis"),
        )
        .years(Some(2025), Some(2025)),
        ki(
            "International Labour Day",
            KI_LISTED.named("International Labour Day"),
        ),
        ki(
            "National Police Day",
            KI_LISTED.named("National Police Day"),
        ),
        ki("Gospel Day", KI_LISTED.named("Gospel Day")),
        ki("National Day", KI_LISTED.named("National Day")),
        ki(
            "Kiribati Culture and Senior Citizens Day",
            KI_LISTED.named("Kiribati Culture and Senior Citizens Day"),
        ),
        ki(
            "Kiribati Special Day",
            KI_LISTED.named("Kiribati Special Day"),
        ),
        ki(
            "National Youth and Children's Day",
            KI_LISTED.named("National Youth and Children's Day"),
        ),
        ki(
            "World Teachers' Day",
            KI_LISTED.named("World Teachers' Day"),
        ),
        ki("Human Rights Day", KI_LISTED.named("Human Rights Day")),
        ki("Christmas Day", KI_LISTED.named("Christmas Day")),
        ki("Boxing Day", KI_LISTED.named("Boxing Day")),
    ],
);

/// Kiribati — the days the Beretitenti's orders under the Public Holidays
/// Ordinance declare for 2025 and 2026.
///
/// The Ordinance (Cap. 81) keeps the days of its Schedule — the one the
/// Public Holidays (Amendment) Act 2002 substituted — and gives the Monday
/// for one on a Saturday or a Sunday "unless otherwise ordered by the
/// Minister by notice". Section 6 lets the Minister alter the days in any
/// year, and section 9, added in 2005, amend the Schedule by notice. The
/// orders for 2025, as revised on 18 December 2025, and for 2026 list every
/// day of the year's holidays, and they are tabulated as the orders give
/// them, the "in honour of" day standing for a holiday moved off a weekend
/// or kept beside it: Gospel Day on Friday 10 July 2026, the day before a
/// Saturday 11 July; National Day on Saturday 12 July 2025 and on Monday
/// 14 July as well; Boxing Day on Monday 28 December 2026. The names are
/// unified across the two years. Good Friday and Easter Monday, in the
/// Schedule and in both orders, are carried by rule; the rest are reported
/// as a gap in a year outside 2025–2026. Each order also keeps "any day
/// appointed to be a public holiday under section 2" that it does not list;
/// the Schedule as amended by notice was not read, and nothing beyond the
/// orders' lists is carried. The Public Service Office's page for 2025,
/// which puts National Police Day on "Friday, 23rd June" and lacks days the
/// revised order has, was not used.
///
/// This table is written up with the other twelve whose days are
/// announced year by year in `docs/systems/announced-holidays.md`.
pub static KIRIBATI: RuleSet = RuleSet {
    code: "KI",
    english_name: "Kiribati",
    rules: KI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: weekends::KI,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Ordinance (Cap. 81), 1977 revised edition, sections 2 and 6 \
              and Schedule, and the Public Holidays (Amendment) Acts 1992, 2002 and 2005, \
              PacLII's copies as the Internet Archive holds them, captured 2015-08-28 and \
              2024-12-23; the Orders under sections 6 and 9 declaring the public holidays \
              for 2025 (revised) and 2026, dated 18 December 2025 (PH_2025_revised_181225.pdf, \
              PH_2026_181225.pdf, president.gov.ki, Gazettes & Instruments), the 2026 list \
              also from the Public Service Office (pso.gov.ki); retrieved 2026-09-23",
    subdivisions: Subdivisions::Read(&[]),
};
