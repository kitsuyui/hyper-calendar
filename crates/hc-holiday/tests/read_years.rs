//! The years before a rule's first, and the subdivisions a table was read
//! for (ADR 0013): the engine reports what the sources do not answer as a
//! gap, and leaves out only what a source says was not kept.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::rule::{
    HolidayRule, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, Subdivisions, UNREAD_SUBDIVISION,
};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn gap_names(table: &RuleSet, region: Option<&str>, year: i64) -> Vec<&'static str> {
    HolidayCalendar::for_year(table, region, year)
        .gaps()
        .iter()
        .map(|gap| gap.name)
        .collect()
}

static RULES: &[HolidayRule] = &[
    // Established in 1990 and read from 2000: absent before 1990, a gap in
    // 1990–1999.
    HolidayRule::public("Established", "", Rule::gregorian(3, 1))
        .years(Some(1990), None)
        .read_from(2000)
        .cited("an act of 1990, read in its text of 2000"),
    // No establishment known, read from 2000: a gap in every earlier year.
    HolidayRule::public("Unknown before", "", Rule::gregorian(4, 1)).read_from(2000),
    // Kept everywhere but XX-B.
    HolidayRule::public("Not in B", "", Rule::gregorian(5, 1)).except_in(&["XX-B"]),
    HolidayRule::public("A's own", "", Rule::gregorian(6, 1)).in_regions(&["XX-A"]),
];

static TABLE: RuleSet = RuleSet {
    code: "XX",
    english_name: "A test country",
    rules: RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "invented for the test",
    subdivisions: Subdivisions::Read(&["XX-C"]),
};

#[test]
fn a_rule_is_absent_before_its_establishment_and_a_gap_before_its_first_year_read() {
    assert!(gap_names(&TABLE, None, 1989) == ["Unknown before"]);
    assert_eq!(
        gap_names(&TABLE, None, 1990),
        ["Established", "Unknown before"]
    );
    assert_eq!(
        gap_names(&TABLE, None, 1999),
        ["Established", "Unknown before"]
    );
    assert!(gap_names(&TABLE, None, 2000).is_empty());
    let calendar = HolidayCalendar::for_year(&TABLE, None, 1995);
    assert!(!calendar.is_holiday(ymd(1995, 3, 1)));
    assert!(HolidayCalendar::for_year(&TABLE, None, 2000).is_holiday(ymd(2000, 3, 1)));
    // The gap carries the rule's source, which says what was read.
    let gap = HolidayCalendar::for_year(&TABLE, None, 1995).gaps()[0];
    assert_eq!(gap.source, "an act of 1990, read in its text of 2000");
}

#[test]
fn an_excepted_region_does_not_keep_a_nationwide_day() {
    let may = ymd(2026, 5, 1);
    assert!(HolidayCalendar::for_year(&TABLE, None, 2026).is_holiday(may));
    assert!(HolidayCalendar::for_year(&TABLE, Some("XX-A"), 2026).is_holiday(may));
    assert!(!HolidayCalendar::for_year(&TABLE, Some("XX-B"), 2026).is_holiday(may));
    assert!(!HolidayCalendar::for_year(&TABLE, Some(" xx-b "), 2026).is_holiday(may));
    // XX-B is a region the table answers for, so it is listed.
    assert_eq!(TABLE.regions(), ["XX-A", "XX-B"]);
}

#[test]
fn a_subdivision_not_read_is_a_gap_and_keeps_the_nationwide_days() {
    // Read with days of its own, read with none, and excepted: no gap.
    for region in ["XX-A", "XX-B", "XX-C", "xx-c"] {
        assert!(gap_names(&TABLE, Some(region), 2026).is_empty(), "{region}");
    }
    // Not read: the nationwide days, and one gap a year.
    let calendar = HolidayCalendar::new(&TABLE, Some("XX-D"), 2025, 2026);
    assert!(calendar.is_holiday(ymd(2026, 5, 1)));
    let gaps: Vec<(i64, &str)> = calendar
        .gaps()
        .iter()
        .map(|gap| (gap.year, gap.name))
        .collect();
    assert_eq!(
        gaps,
        [(2025, UNREAD_SUBDIVISION), (2026, UNREAD_SUBDIVISION)]
    );
    // The nationwide calendar has no such gap.
    assert!(gap_names(&TABLE, None, 2026).is_empty());
    // A table with no subdivisions has no such gap either.
    assert!(
        gap_names(hc_holiday::traditions::ALL[0], Some("XX-D"), 2026)
            .iter()
            .all(|name| *name != UNREAD_SUBDIVISION)
    );
}

/// The audit's unread subdivisions: each now reports its own days as a gap.
#[test]
fn the_subdivisions_no_source_was_read_for_are_gaps() {
    for (table, region) in [
        (&countries::MEXICO, "MX-CMX"),
        (&countries::MEXICO, "MX-NLE"),
        (&countries::NEW_ZEALAND, "NZ-NTL"),
        (&countries::RUSSIA, "RU-MO"),
        (&countries::NEPAL, "NP-P1"),
        (&countries::BOLIVIA, "BO-X"),
        (&countries::CHINA, "CN-BJ"),
        (&countries::INDIA, "IN-PB"),
        (&countries::FRANCE, "FR-75"),
    ] {
        assert!(
            gap_names(table, Some(region), 2026).contains(&UNREAD_SUBDIVISION),
            "{region}"
        );
        assert!(
            !gap_names(table, None, 2026).contains(&UNREAD_SUBDIVISION),
            "{region}"
        );
    }
}

/// India's state lists: a gap before 2019 back to the state's formation, or
/// to the Negotiable Instruments Act's commencement in 1882, and absent
/// before it.
#[test]
fn india_s_state_gaps_stop_at_the_act_that_formed_the_state() {
    const LISTS: &str = "Holidays under the Negotiable Instruments Act";
    let gap = |region, year| gap_names(&countries::INDIA, Some(region), year).contains(&LISTS);
    for (region, formed, first_read) in [
        ("IN-TS", 2014, 2019),
        ("IN-GJ", 1960, 2019),
        ("IN-MH", 1960, 2019),
        ("IN-KL", 1956, 2019),
        ("IN-AP", 1956, 2023),
        ("IN-UP", 1882, 2019),
        ("IN-TN", 1882, 2019),
    ] {
        assert!(!gap(region, formed - 1), "{region} {}", formed - 1);
        assert!(gap(region, formed), "{region} {formed}");
        assert!(gap(region, first_read - 1), "{region} {}", first_read - 1);
        assert!(!gap(region, first_read), "{region} {first_read}");
        assert!(gap(region, 2027), "{region} 2027");
    }
    // Years the sources read: IN-TS in 2010 and 1900, IN-GJ in 1950 and
    // IN-KL in 1880 are not gaps.
    assert!(!gap("IN-TS", 2010));
    assert!(!gap("IN-TS", 1900));
    assert!(!gap("IN-GJ", 1950));
    assert!(!gap("IN-KL", 1880));
}

static FROM_TABLE: RuleSet = RuleSet {
    code: "XY",
    english_name: "A test country whose subdivisions were read from a year",
    rules: &[HolidayRule::public("Everywhere", "", Rule::gregorian(1, 1))],
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 10, 3),
    sources: "invented for the test",
    subdivisions: Subdivisions::ReadFrom(&[("XY-A", 2000), ("XY-A-001", 2005)]),
};

#[test]
fn a_subdivision_read_from_a_year_is_a_gap_before_it() {
    // Audit 10 a4: Japan's prefectures whose ordinances in force now give no
    // day of their own say nothing of the years before the 休日条例.
    let gaps = |region: &str, year: i64| -> Vec<&'static str> {
        gap_names(&FROM_TABLE, Some(region), year)
    };
    assert_eq!(gaps("XY-A", 1999), [UNREAD_SUBDIVISION]);
    assert!(gaps("XY-A", 2000).is_empty());
    assert!(gaps("XY-A", 2026).is_empty());
    // A municipality is read from its own year, and not before its
    // subdivision's: 2004 is its subdivision's year but not its own.
    assert_eq!(gaps("XY-A-001", 2004), [UNREAD_SUBDIVISION]);
    assert!(gaps("XY-A-001", 2005).is_empty());
    assert_eq!(gaps("XY-A-001", 1990), [UNREAD_SUBDIVISION]);
    // A subdivision not listed at all is a gap in every year, and the
    // nationwide table has none.
    assert_eq!(gaps("XY-B", 2026), [UNREAD_SUBDIVISION]);
    assert!(gap_names(&FROM_TABLE, None, 1990).is_empty());
    // Which years each is read for is `reads_region_in`; `reads_region` is
    // true for any year.
    assert!(FROM_TABLE.reads_region("XY-A"));
    assert!(!FROM_TABLE.reads_region_in("XY-A", 1999));
    assert!(FROM_TABLE.reads_region_in("xy-a", 2000));
    assert_eq!(FROM_TABLE.read_subdivisions(), ["XY-A", "XY-A-001"]);
}
