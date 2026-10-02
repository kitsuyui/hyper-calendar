//! Bangladesh's optional holidays (ঐচ্ছিক ছুটি), section by section.
//!
//! Each row is a day the Ministry of Public Administration's notifications
//! for 2025 and 2026 list in a faith's or community's section, as the
//! newspapers reproduced the lists and read as HTML; the PDFs on
//! mopa.gov.bd were not opened. The weekday is the one Ekhon TV's table of
//! the 2026 list prints beside the day; the 2025 reproductions print none.
//! The rows are the lists', not dates this crate produced.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_holiday::countries::BANGLADESH;
use hc_holiday::engine::{Holiday, HolidayCalendar};
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

/// A group, an English name, and a day a notification lists, with the
/// weekday Ekhon TV prints for 2026, `None` for 2025.
type Day = (&'static str, &'static str, (i64, u8, u8), Option<Weekday>);

const DAYS: &[Day] = &[
    // The Muslim section.
    (
        "muslims",
        "Shab-e-Meraj",
        (2026, 1, 17),
        Some(Weekday::Saturday),
    ),
    (
        "muslims",
        "Eid-ul-Fitr (third day after)",
        (2025, 4, 3),
        None,
    ),
    (
        "muslims",
        "Eid-ul-Fitr (third day after)",
        (2026, 3, 24),
        Some(Weekday::Tuesday),
    ),
    (
        "muslims",
        "Eid-ul-Azha (fourth day after)",
        (2025, 6, 11),
        None,
    ),
    (
        "muslims",
        "Eid-ul-Azha (fourth day after)",
        (2026, 6, 1),
        Some(Weekday::Monday),
    ),
    (
        "muslims",
        "Akheri Chahar Shomba",
        (2026, 8, 12),
        Some(Weekday::Wednesday),
    ),
    ("muslims", "Fateha-e-Yazdahum", (2025, 10, 4), None),
    (
        "muslims",
        "Fateha-e-Yazdahum",
        (2026, 9, 24),
        Some(Weekday::Thursday),
    ),
    // The Hindu section.
    ("hindus", "Saraswati Puja", (2025, 2, 3), None),
    (
        "hindus",
        "Saraswati Puja",
        (2026, 1, 23),
        Some(Weekday::Friday),
    ),
    ("hindus", "Shivaratri Brata", (2025, 2, 26), None),
    (
        "hindus",
        "Shivaratri Brata",
        (2026, 2, 15),
        Some(Weekday::Sunday),
    ),
    ("hindus", "Dolyatra", (2025, 3, 14), None),
    ("hindus", "Dolyatra", (2026, 3, 3), Some(Weekday::Tuesday)),
    (
        "hindus",
        "Appearance of Harichand Thakur",
        (2025, 3, 27),
        None,
    ),
    (
        "hindus",
        "Appearance of Harichand Thakur",
        (2026, 3, 17),
        Some(Weekday::Tuesday),
    ),
    ("hindus", "Mahalaya", (2025, 9, 21), None),
    (
        "hindus",
        "Mahalaya",
        (2026, 10, 10),
        Some(Weekday::Saturday),
    ),
    (
        "hindus",
        "Durga Puja (Saptami and Ashtami)",
        (2025, 9, 29),
        None,
    ),
    (
        "hindus",
        "Durga Puja (Saptami and Ashtami)",
        (2025, 9, 30),
        None,
    ),
    (
        "hindus",
        "Durga Puja (Saptami and Ashtami)",
        (2026, 10, 18),
        Some(Weekday::Sunday),
    ),
    (
        "hindus",
        "Durga Puja (Saptami and Ashtami)",
        (2026, 10, 19),
        Some(Weekday::Monday),
    ),
    ("hindus", "Lakshmi Puja", (2025, 10, 6), None),
    (
        "hindus",
        "Lakshmi Puja",
        (2026, 10, 25),
        Some(Weekday::Sunday),
    ),
    ("hindus", "Shyama Puja", (2025, 10, 31), None),
    (
        "hindus",
        "Shyama Puja",
        (2026, 11, 8),
        Some(Weekday::Sunday),
    ),
    // The Christian section.
    ("christians", "English New Year's Day", (2025, 1, 1), None),
    (
        "christians",
        "English New Year's Day",
        (2026, 1, 1),
        Some(Weekday::Thursday),
    ),
    ("christians", "Ash Wednesday", (2025, 3, 5), None),
    (
        "christians",
        "Ash Wednesday",
        (2026, 2, 18),
        Some(Weekday::Wednesday),
    ),
    ("christians", "Holy Thursday", (2025, 4, 17), None),
    (
        "christians",
        "Holy Thursday",
        (2026, 4, 2),
        Some(Weekday::Thursday),
    ),
    ("christians", "Good Friday", (2025, 4, 18), None),
    (
        "christians",
        "Good Friday",
        (2026, 4, 3),
        Some(Weekday::Friday),
    ),
    ("christians", "Holy Saturday", (2025, 4, 19), None),
    (
        "christians",
        "Holy Saturday",
        (2026, 4, 4),
        Some(Weekday::Saturday),
    ),
    ("christians", "Easter Sunday", (2025, 4, 20), None),
    (
        "christians",
        "Easter Sunday",
        (2026, 4, 5),
        Some(Weekday::Sunday),
    ),
    (
        "christians",
        "Christmas (the day before and the day after)",
        (2025, 12, 24),
        None,
    ),
    (
        "christians",
        "Christmas (the day before and the day after)",
        (2025, 12, 26),
        None,
    ),
    (
        "christians",
        "Christmas (the day before and the day after)",
        (2026, 12, 24),
        Some(Weekday::Thursday),
    ),
    (
        "christians",
        "Christmas (the day before and the day after)",
        (2026, 12, 26),
        Some(Weekday::Saturday),
    ),
    // The Buddhist section.
    ("buddhists", "Magha Purnima", (2025, 2, 11), None),
    (
        "buddhists",
        "Magha Purnima",
        (2026, 2, 1),
        Some(Weekday::Sunday),
    ),
    ("buddhists", "Chaitra Sankranti", (2025, 4, 13), None),
    (
        "buddhists",
        "Chaitra Sankranti",
        (2026, 4, 13),
        Some(Weekday::Monday),
    ),
    (
        "buddhists",
        "Buddha Purnima (the day before and the day after)",
        (2025, 5, 10),
        None,
    ),
    (
        "buddhists",
        "Buddha Purnima (the day before and the day after)",
        (2025, 5, 12),
        None,
    ),
    (
        "buddhists",
        "Buddha Purnima (the day before and the day after)",
        (2026, 4, 30),
        Some(Weekday::Thursday),
    ),
    (
        "buddhists",
        "Buddha Purnima (the day before and the day after)",
        (2026, 5, 2),
        Some(Weekday::Saturday),
    ),
    ("buddhists", "Asalhi Purnima", (2025, 7, 9), None),
    (
        "buddhists",
        "Asalhi Purnima",
        (2026, 7, 29),
        Some(Weekday::Wednesday),
    ),
    ("buddhists", "Madhu Purnima", (2025, 9, 6), None),
    (
        "buddhists",
        "Madhu Purnima",
        (2026, 9, 26),
        Some(Weekday::Saturday),
    ),
    ("buddhists", "Probarana Purnima", (2025, 10, 5), None),
    (
        "buddhists",
        "Probarana Purnima",
        (2026, 10, 25),
        Some(Weekday::Sunday),
    ),
    // The small ethnic groups' section.
    (
        "small-ethnic-groups",
        "Boisabi and the like",
        (2025, 4, 12),
        None,
    ),
    (
        "small-ethnic-groups",
        "Boisabi and the like",
        (2025, 4, 15),
        None,
    ),
    (
        "small-ethnic-groups",
        "Boisabi and the like",
        (2026, 4, 12),
        Some(Weekday::Sunday),
    ),
    (
        "small-ethnic-groups",
        "Boisabi and the like",
        (2026, 4, 15),
        Some(Weekday::Wednesday),
    ),
];

/// How many days each section lists in a year, as the notifications total
/// them: five, nine, eight, seven and two. Two rows of the table are not
/// carried for 2025; see `the_days_the_reproductions_disagree_on_are_gaps`.
const TOTALS: &[(&str, usize)] = &[
    ("muslims", 5),
    ("hindus", 9),
    ("christians", 8),
    ("buddhists", 7),
    ("small-ethnic-groups", 2),
];

#[test]
fn each_group_has_its_days_and_everyone_does_not() {
    for &(group, name, (year, month, day), weekday) in DAYS {
        let date = ymd(year, month, day);
        if let Some(weekday) = weekday {
            assert_eq!(Weekday::from_rd(date), weekday, "{name} {year}");
        }
        let calendar = HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group(group), year);
        let on = calendar.on(date);
        let own: Vec<_> = on.iter().filter(|holiday| holiday.name == name).collect();
        assert_eq!(own.len(), 1, "{group} {name} {year}-{month}-{day}");
        assert_eq!(own[0].groups.len(), 1);
        assert_eq!(own[0].groups[0].id, group);
        // Leave on application is not a day off.
        assert!(!own[0].kind.is_day_off());
        let coincides = on
            .iter()
            .any(|holiday| holiday.groups.is_empty() && holiday.kind.is_day_off());
        assert!(
            coincides || calendar.is_weekend(date) || calendar.is_business_day(date),
            "{name} {year}-{month}-{day}"
        );
        let everyone = HolidayCalendar::for_year(&BANGLADESH, None, year);
        assert!(
            !everyone.on(date).iter().any(|holiday| holiday.name == name),
            "{name} {year}"
        );
        // Another faith's calendar does not have it.
        let other = if group == "hindus" {
            "christians"
        } else {
            "hindus"
        };
        let neighbour = HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group(other), year);
        assert!(
            !neighbour
                .on(date)
                .iter()
                .any(|holiday| holiday.name == name),
            "{name} {year}"
        );
    }
}

#[test]
fn a_section_lists_the_days_the_notification_totals() {
    for &(group, total) in TOTALS {
        // 2026 carries every day; 2025 all but the two Muslim days its
        // reproductions disagree on.
        let calendar = HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group(group), 2026);
        assert_eq!(own_days(&calendar).len(), total, "{group} 2026");
        let calendar = HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group(group), 2025);
        let expected = if group == "muslims" { total - 2 } else { total };
        assert_eq!(own_days(&calendar).len(), expected, "{group} 2025");
    }
}

#[test]
fn an_optional_holiday_is_a_religious_day_and_the_social_festival_an_observance() {
    let christians =
        HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group("christians"), 2026);
    assert!(
        own_days(&christians)
            .iter()
            .all(|holiday| holiday.kind == Kind::Religious)
    );
    let ethnic =
        HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group("small-ethnic-groups"), 2026);
    assert!(
        own_days(&ethnic)
            .iter()
            .all(|holiday| holiday.kind == Kind::Observance)
    );
    // Easter Sunday 2026 is a weekday of nothing: the day is the weekly
    // holiday of Friday and Saturday, and Sunday a working day the group's
    // calendar still counts as one.
    assert!(christians.is_business_day(ymd(2026, 4, 5)));
}

#[test]
fn the_days_the_reproductions_disagree_on_are_gaps() {
    // Shab-e-Meraj 2025: 28 February in four reproductions, 28 January in
    // two. Akheri Chahar Shomba 2025: 20 September, a Saturday, in all.
    let calendar = HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group("muslims"), 2025);
    for name in ["Shab-e-Meraj", "Akheri Chahar Shomba"] {
        assert!(
            calendar.gaps().iter().any(|gap| gap.name == name),
            "{name}: {:?}",
            calendar.gaps()
        );
    }
    assert!(
        !calendar
            .on(ymd(2025, 2, 28))
            .iter()
            .any(|h| !h.groups.is_empty())
    );
    assert!(
        !calendar
            .on(ymd(2025, 9, 20))
            .iter()
            .any(|h| !h.groups.is_empty())
    );
    assert!(
        !calendar
            .on(ymd(2025, 1, 28))
            .iter()
            .any(|h| !h.groups.is_empty())
    );
}

#[test]
fn the_years_beyond_the_notifications_are_gaps_for_the_group_and_not_for_everyone() {
    for group in [
        "muslims",
        "hindus",
        "christians",
        "buddhists",
        "small-ethnic-groups",
    ] {
        let beyond = HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group(group), 2027);
        assert!(!beyond.is_complete(), "{group}");
        assert!(own_days(&beyond).is_empty(), "{group}");
        let before = HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group(group), 2024);
        assert!(!before.is_complete(), "{group}");
    }
    // Everyone's calendar does not name the group's days among its gaps.
    let everyone = HolidayCalendar::for_year(&BANGLADESH, None, 2027);
    assert!(
        !everyone
            .gaps()
            .iter()
            .any(|gap| gap.name == "Saraswati Puja")
    );
}

#[test]
fn chaitra_sankranti_is_a_buddhist_optional_day_outside_the_hill_districts_from_2026() {
    // 2025: no exception; 2026: "for all districts but the three hill
    // ones", where it is the general holiday.
    let nationwide = |year| {
        HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group("buddhists"), year)
            .on(ymd(year, 4, 13))
            .iter()
            .any(|holiday| holiday.name == "Chaitra Sankranti")
    };
    assert!(nationwide(2025));
    assert!(nationwide(2026));
    for region in ["BD-01", "BD-29", "BD-56"] {
        let scope = |_year| Scope::new(Some(region), Some("buddhists"));
        let in_2025 = HolidayCalendar::for_year_scoped(&BANGLADESH, scope(2025), 2025);
        assert!(
            in_2025
                .on(ymd(2025, 4, 13))
                .iter()
                .any(|holiday| holiday.name == "Chaitra Sankranti"),
            "{region}"
        );
        let in_2026 = HolidayCalendar::for_year_scoped(&BANGLADESH, scope(2026), 2026);
        let on = in_2026.on(ymd(2026, 4, 13));
        // The general holiday of the hill districts, and not the optional
        // day.
        assert_eq!(on.len(), 1, "{region}: {on:?}");
        assert!(on[0].groups.is_empty());
        assert_eq!(on[0].kind, Kind::Public);
    }
    // Another district keeps the optional day, and no general holiday.
    let dhaka = HolidayCalendar::for_year_scoped(
        &BANGLADESH,
        Scope::new(Some("BD-13"), Some("buddhists")),
        2026,
    );
    let on = dhaka.on(ymd(2026, 4, 13));
    assert_eq!(on.len(), 1);
    assert_eq!(on[0].groups[0].id, "buddhists");
}

#[test]
fn the_cap_of_three_is_not_computed_and_the_general_holidays_are_untouched() {
    // A group's calendar lists every day of its section, more than three;
    // which three an employee takes is theirs and their authority's.
    let hindus = HolidayCalendar::for_year_scoped(&BANGLADESH, Scope::group("hindus"), 2026);
    assert!(own_days(&hindus).len() > 3);
    // Everyone's days are the same in a group's calendar.
    let everyone = HolidayCalendar::for_year(&BANGLADESH, None, 2026);
    let common: Vec<_> = hindus
        .all()
        .iter()
        .filter(|holiday| holiday.groups.is_empty())
        .copied()
        .collect();
    assert_eq!(common, everyone.all());
    // The tables name the five groups.
    let ids: Vec<_> = BANGLADESH.groups().iter().map(|group| group.id).collect();
    assert_eq!(
        ids,
        [
            "buddhists",
            "christians",
            "hindus",
            "muslims",
            "small-ethnic-groups"
        ]
    );
}
