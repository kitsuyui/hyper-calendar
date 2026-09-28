//! The days a statute gives to one group of people alone.
//!
//! Each row is a day a statute read gives to a group, with the date and
//! the effect its text states, not dates this crate produced: China's
//! Article 3 of the 全国年节及纪念日放假办法, and Taiwan's Article 6 of the
//! 紀念日及節日實施條例, whose services' days are gaps because the
//! authorities' rules were not read.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::{CHINA, JAPAN, TAIWAN};
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::group::{self, Group};
use hc_holiday::rule::{Kind, Scope};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn own_days(calendar: &HolidayCalendar<'_>) -> Vec<Holiday> {
    calendar
        .all()
        .iter()
        .filter(|holiday| !holiday.groups.is_empty())
        .copied()
        .collect()
}

/// China's Article 3: the day, the group, the kind.
const CHINA_DAYS: &[(&str, u8, u8, Group, Kind)] = &[
    ("妇女节", 3, 8, group::WOMEN, Kind::HalfDay),
    ("青年节", 5, 4, group::YOUTH, Kind::HalfDay),
    ("儿童节", 6, 1, group::CHILDREN, Kind::Public),
    (
        "中国人民解放军建军纪念日",
        8,
        1,
        group::MILITARY,
        Kind::HalfDay,
    ),
];

#[test]
fn china_gives_each_article_3_day_to_its_group_alone() {
    for year in [1999, 2008, 2026] {
        let everyone = HolidayCalendar::for_year(&CHINA, None, year);
        for &(local_name, month, day, group, kind) in CHINA_DAYS {
            let calendar = HolidayCalendar::for_year_scoped(&CHINA, Scope::group(group.id), year);
            let own = own_days(&calendar);
            assert_eq!(own.len(), 1, "{year} {}: {own:?}", group.id);
            assert_eq!(own[0].date, ymd(year, month, day));
            assert_eq!(own[0].local_name, local_name);
            assert_eq!(own[0].kind, kind);
            assert_eq!(own[0].groups, &[group]);
            assert!(
                own[0]
                    .source
                    .starts_with("全国年节及纪念日放假办法, 第三条")
            );
            assert!(
                !everyone
                    .all()
                    .iter()
                    .any(|holiday| holiday.local_name == local_name),
                "{year}: {local_name} is not everyone's"
            );
        }
    }
}

#[test]
fn a_half_day_is_a_business_day_and_the_children_s_day_is_not() {
    let women = HolidayCalendar::for_year_scoped(&CHINA, Scope::group("women"), 2026);
    // Sunday 8 March 2026 is the weekend anyway; 2027's is a Monday.
    let women_2027 = HolidayCalendar::for_year_scoped(&CHINA, Scope::group("women"), 2027);
    assert!(!Kind::HalfDay.is_day_off());
    assert!(women_2027.is_business_day(ymd(2027, 3, 8)));
    assert!(!women.is_holiday(ymd(2026, 3, 8)));
    let children = HolidayCalendar::for_year_scoped(&CHINA, Scope::group("CHILDREN"), 2026);
    // Monday 1 June 2026.
    assert!(children.is_holiday(ymd(2026, 6, 1)));
    assert!(!children.is_business_day(ymd(2026, 6, 1)));
    let everyone = HolidayCalendar::for_year(&CHINA, None, 2026);
    assert!(everyone.is_business_day(ymd(2026, 6, 1)));
}

/// The texts before 1999 were not read: 1998 is a gap for each group's
/// day, not a year without it.
#[test]
fn china_s_group_days_are_gaps_before_1999() {
    for (name, group) in [
        ("Women's Day", "women"),
        ("Youth Day", "youth"),
        ("Children's Day", "children"),
        ("Army Day", "military"),
    ] {
        let calendar = HolidayCalendar::for_year_scoped(&CHINA, Scope::group(group), 1998);
        assert!(own_days(&calendar).is_empty(), "{group}");
        assert!(
            calendar
                .gaps()
                .iter()
                .any(|gap| gap.name == name && gap.year == 1998),
            "{group}"
        );
    }
    let everyone = HolidayCalendar::for_year(&CHINA, None, 1998);
    assert!(!everyone.gaps().iter().any(|gap| gap.name == "Women's Day"));
}

#[test]
fn a_group_and_a_region_are_independent() {
    // A region adds nothing for a group, and a group nothing for a region
    // it does not name; a group China does not name adds nothing.
    let both =
        HolidayCalendar::for_year_scoped(&CHINA, Scope::new(Some("CN-XX"), Some("women")), 2026);
    let women = HolidayCalendar::for_year_scoped(&CHINA, Scope::group("women"), 2026);
    assert_eq!(own_days(&both), own_days(&women));
    let police = HolidayCalendar::for_year_scoped(&CHINA, Scope::group("police"), 2026);
    assert!(own_days(&police).is_empty());
    // Japan names no group: a group changes nothing there.
    let japan = HolidayCalendar::for_year(&JAPAN, None, 2026);
    let japan_women = HolidayCalendar::for_year_scoped(&JAPAN, Scope::group("women"), 2026);
    assert_eq!(japan.all(), japan_women.all());
    assert_eq!(japan_women.group(), Some("women"));
}

#[test]
fn the_tables_list_the_groups_they_name() {
    let ids = |groups: Vec<Group>| groups.iter().map(|group| group.id).collect::<Vec<_>>();
    assert_eq!(
        ids(CHINA.groups()),
        ["children", "military", "women", "youth"]
    );
    assert_eq!(
        ids(TAIWAN.groups()),
        [
            "coast-guard",
            "firefighters",
            "indigenous-peoples",
            "military",
            "police"
        ]
    );
    assert!(JAPAN.groups().is_empty());
    assert!(CHINA.region_groups().is_empty());
}

#[test]
fn taiwan_s_services_days_are_gaps_from_the_statute() {
    for (group, name, first) in [
        ("police", "Police Day", 2025),
        ("military", "Armed Forces Day", 2025),
        ("coast-guard", "Coast Guard Day", 2025),
        ("firefighters", "Fire Fighters' Day", 2026),
        ("indigenous-peoples", "Indigenous ceremonies", 2025),
    ] {
        let calendar = HolidayCalendar::for_year_scoped(&TAIWAN, Scope::group(group), first);
        assert!(
            calendar
                .gaps()
                .iter()
                .any(|gap| gap.name == name && gap.year == first),
            "{group} {first}: {:?}",
            calendar.gaps()
        );
        assert!(own_days(&calendar).is_empty());
        // The 辦法 before the 條例 was not read for these days: a gap too.
        let before = HolidayCalendar::for_year_scoped(&TAIWAN, Scope::group(group), first - 1);
        assert!(before.gaps().iter().any(|gap| gap.name == name), "{group}");
        let everyone = HolidayCalendar::for_year(&TAIWAN, None, first);
        assert!(
            !everyone.gaps().iter().any(|gap| gap.name == name),
            "{group}"
        );
    }
}
