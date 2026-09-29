//! A municipality is a region within its subdivision (ADR 0014): its code
//! is its subdivision's and a local part, it has its subdivision's days
//! and its own, and it is read only where the table says so.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::rule::{
    HolidayRule, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, Subdivisions, UNREAD_SUBDIVISION,
    region_parent, region_within,
};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

static RULES: &[HolidayRule] = &[
    HolidayRule::public("Nationwide", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Not in B", "", Rule::gregorian(5, 1)).except_in(&["XX-B"]),
    HolidayRule::observance("A's own", "", Rule::gregorian(6, 1)).in_regions(&["XX-A"]),
    HolidayRule::observance("A1's own", "", Rule::gregorian(7, 1)).in_regions(&["XX-A-001"]),
    HolidayRule::observance("B1's own", "", Rule::gregorian(8, 1)).in_regions(&["XX-B-001"]),
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
    subdivisions: Subdivisions::Read(&["XX-B", "XX-A-002"]),
};

fn names(region: Option<&str>) -> Vec<&'static str> {
    HolidayCalendar::for_year(&TABLE, region, 2026)
        .all()
        .iter()
        .map(|holiday| holiday.name)
        .collect()
}

fn gaps(region: &str) -> Vec<&'static str> {
    HolidayCalendar::for_year(&TABLE, Some(region), 2026)
        .gaps()
        .iter()
        .map(|gap| gap.name)
        .collect()
}

#[test]
fn a_municipality_s_code_is_under_its_subdivision_s() {
    assert_eq!(region_parent("JP-14-130"), Some("JP-14"));
    assert_eq!(region_parent(" jp-14-130 "), Some("jp-14"));
    assert_eq!(region_parent("JP-14"), None);
    assert_eq!(region_parent("JP"), None);
    assert!(region_within("JP-14-130", "JP-14"));
    assert!(region_within("jp-14-130", "JP-14"));
    assert!(region_within("JP-14-130", "JP-14-130"));
    assert!(!region_within("JP-14", "JP-14-130"));
    // Text is not containment: JP-1 is no prefix region of JP-14-130.
    assert!(!region_within("JP-14-130", "JP-1"));
    assert!(!region_within("JP-14-130", "JP-13"));
}

#[test]
fn a_municipality_has_its_subdivision_s_days_and_its_own() {
    assert_eq!(names(None), ["Nationwide", "Not in B"]);
    assert_eq!(names(Some("XX-A")), ["Nationwide", "Not in B", "A's own"]);
    assert_eq!(
        names(Some("XX-A-001")),
        ["Nationwide", "Not in B", "A's own", "A1's own"]
    );
    assert_eq!(names(Some("xx-a-001")), names(Some("XX-A-001")));
    // A day excepted from the subdivision is excepted from its cities.
    assert_eq!(names(Some("XX-B-001")), ["Nationwide", "B1's own"]);
}

#[test]
fn a_municipality_is_read_only_where_the_table_says_so() {
    // Named by a rule, or listed: read.
    assert!(gaps("XX-A-001").is_empty());
    assert!(gaps("XX-A-002").is_empty());
    // A city of a subdivision read, itself not: the subdivision's days
    // and a gap for its own.
    assert_eq!(gaps("XX-A-003"), [UNREAD_SUBDIVISION]);
    assert_eq!(
        names(Some("XX-A-003")),
        ["Nationwide", "Not in B", "A's own"]
    );
    // Reading a subdivision says nothing of its cities, and a city listed
    // does not make its subdivision read.
    assert!(TABLE.reads_region("XX-A"));
    assert!(!TABLE.reads_region("XX-C-001"));
    assert!(!TABLE.reads_region("XX-C"));
    let calendar = HolidayCalendar::for_year(&TABLE, Some("XX-A-001"), 2026);
    assert!(!calendar.is_holiday(ymd(2026, 7, 1)));
}
