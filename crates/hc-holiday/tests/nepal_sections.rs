//! Nepal's days for one community, faith or group of employees, and the
//! notices' other sections, item by item.
//!
//! Each row is a day the Home Ministry's notices for 2080 to 2083 BS
//! print, with its Bikram Sambat date turned into the Gregorian one; the
//! weekday each notice prints beside it agrees. The rows are the notices',
//! not dates this crate produced.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_holiday::countries::NEPAL;
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::{Confidence, Kind, Scope};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn named(calendar: &HolidayCalendar<'_>, local_name: &str) -> Vec<Holiday> {
    calendar
        .all()
        .iter()
        .filter(|holiday| holiday.local_name == local_name)
        .copied()
        .collect()
}

/// A group, a local name, and a day a notice gives, with the weekday it
/// prints.
type GroupDay = (&'static str, &'static str, (i64, u8, u8), Weekday);

/// Every dated item the notices give one group.
const GROUP_DAYS: &[GroupDay] = &[
    ("newar", "गाईजात्रा", (2023, 8, 31), Weekday::Thursday),
    ("newar", "गाईजात्रा", (2024, 8, 20), Weekday::Tuesday),
    ("newar", "गाईजात्रा", (2025, 8, 10), Weekday::Sunday),
    ("newar", "गाईजात्रा", (2026, 8, 29), Weekday::Saturday),
    (
        "women",
        "हरितालिका (तीज) व्रत",
        (2023, 9, 18),
        Weekday::Monday,
    ),
    (
        "women",
        "हरितालिका (तीज) व्रत",
        (2024, 9, 6),
        Weekday::Friday,
    ),
    (
        "women",
        "हरितालिका (तीज) व्रत",
        (2025, 8, 26),
        Weekday::Tuesday,
    ),
    (
        "women",
        "हरितालिका (तीज) व्रत",
        (2026, 9, 14),
        Weekday::Monday,
    ),
    ("women", "जितिया पर्व", (2023, 10, 7), Weekday::Saturday),
    ("women", "जितिया पर्व", (2024, 9, 25), Weekday::Wednesday),
    ("women", "जितिया पर्व", (2025, 9, 15), Weekday::Monday),
    ("women", "जितिया पर्व", (2026, 10, 4), Weekday::Sunday),
    ("dura", "दुरा म्हैप्रु नकुमा", (2026, 12, 30), Weekday::Wednesday),
    ("kirat", "फाल्गुनन्द जयन्ती", (2023, 11, 11), Weekday::Saturday),
    ("kirat", "फाल्गुनन्द जयन्ती", (2024, 11, 10), Weekday::Sunday),
    ("kirat", "फाल्गुनन्द जयन्ती", (2025, 11, 11), Weekday::Tuesday),
    ("kirat", "फाल्गुनन्द जयन्ती", (2026, 11, 11), Weekday::Wednesday),
    (
        "persons-with-disabilities",
        "अन्तर्राष्ट्रिय अपाङ्गता दिवस",
        (2023, 12, 3),
        Weekday::Sunday,
    ),
    (
        "persons-with-disabilities",
        "अन्तर्राष्ट्रिय अपाङ्गता दिवस",
        (2026, 12, 3),
        Weekday::Thursday,
    ),
    ("sikhs", "गुरु नानक जयन्ती", (2023, 11, 27), Weekday::Monday),
    ("sikhs", "गुरु नानक जयन्ती", (2024, 11, 15), Weekday::Friday),
];

#[test]
fn each_group_has_its_day_and_everyone_does_not() {
    for &(group, local_name, (year, month, day), weekday) in GROUP_DAYS {
        let date = ymd(year, month, day);
        assert_eq!(Weekday::from_rd(date), weekday, "{local_name} {year}");
        let calendar = HolidayCalendar::for_year_scoped(&NEPAL, Scope::group(group), year);
        let own = named(&calendar, local_name);
        assert_eq!(
            own.iter().map(|holiday| holiday.date).collect::<Vec<_>>(),
            [date],
            "{group} {local_name} {year}"
        );
        assert_eq!(own[0].kind, Kind::Public);
        assert_eq!(own[0].groups.len(), 1);
        assert_eq!(own[0].groups[0].id, group);
        let everyone = HolidayCalendar::for_year(&NEPAL, None, year);
        assert!(
            named(&everyone, local_name).is_empty(),
            "{local_name} {year}"
        );
    }
}

#[test]
fn the_days_are_gaps_before_the_first_notice_and_after_the_last() {
    for (group, local_name, name) in [
        ("newar", "गाईजात्रा", "Gai Jatra"),
        ("women", "हरितालिका (तीज) व्रत", "Haritalika Teej"),
        ("women", "जितिया पर्व", "Jitiya"),
    ] {
        let before = HolidayCalendar::for_year_scoped(&NEPAL, Scope::group(group), 2022);
        assert!(named(&before, local_name).is_empty());
        assert!(
            before
                .gaps()
                .iter()
                .any(|gap| gap.name == name && gap.year == 2022),
            "{name}"
        );
        let after = HolidayCalendar::for_year_scoped(&NEPAL, Scope::group(group), 2027);
        assert!(named(&after, local_name).is_empty());
        assert!(
            after
                .gaps()
                .iter()
                .any(|gap| gap.name == name && gap.year == 2027),
            "{name}"
        );
    }
    let kirat = HolidayCalendar::for_year_scoped(&NEPAL, Scope::group("kirat"), 2022);
    assert!(named(&kirat, "फाल्गुनन्द जयन्ती").is_empty());
    assert!(
        kirat
            .gaps()
            .iter()
            .any(|gap| gap.name == "Falgunanda Jayanti")
    );
    // The notices for 2080 to 2082 BS were read and do not list the Dura
    // day: 2025 is without it, and 2022, before them, a gap.
    let before = HolidayCalendar::for_year_scoped(&NEPAL, Scope::group("dura"), 2022);
    assert!(
        before
            .gaps()
            .iter()
            .any(|gap| gap.name == "Dura Mhaipru Nakuma")
    );
    let dura = HolidayCalendar::for_year_scoped(&NEPAL, Scope::group("dura"), 2025);
    assert!(named(&dura, "दुरा म्हैप्रु नकुमा").is_empty());
    assert!(
        !dura
            .gaps()
            .iter()
            .any(|gap| gap.name == "Dura Mhaipru Nakuma")
    );
}

#[test]
fn the_prophet_s_birthday_is_a_prediction_for_muslims() {
    let calendar = HolidayCalendar::for_year_scoped(&NEPAL, Scope::group("muslims"), 2026);
    let own = named(&calendar, "मोहम्मद जयन्ती");
    assert_eq!(own.len(), 1);
    assert_eq!(own[0].confidence, Confidence::Approximate);
}

#[test]
fn basanta_panchami_closes_the_schools_for_everyone() {
    for (year, month, day) in [(2024, 2, 14), (2025, 2, 3), (2026, 1, 23), (2027, 2, 11)] {
        let calendar = HolidayCalendar::for_year(&NEPAL, None, year);
        let own = named(&calendar, "वसन्त पञ्चमी");
        assert_eq!(
            own.iter().map(|holiday| holiday.date).collect::<Vec<_>>(),
            [ymd(year, month, day)]
        );
        assert_eq!(own[0].kind, Kind::School);
        assert!(own[0].groups.is_empty());
    }
    let before = HolidayCalendar::for_year(&NEPAL, None, 2023);
    assert!(named(&before, "वसन्त पञ्चमी").is_empty());
    assert!(
        before
            .gaps()
            .iter()
            .any(|gap| gap.name == "Basanta Panchami")
    );
    let after = HolidayCalendar::for_year(&NEPAL, None, 2028);
    assert!(
        after
            .gaps()
            .iter()
            .any(|gap| gap.name == "Basanta Panchami")
    );
}

#[test]
fn the_section_8_days_are_kept_with_the_offices_open() {
    for (local_name, year, month, day) in [
        ("जातीय भेदभाव तथा छुवाछुत उन्मूलन राष्ट्रिय दिवस", 2023, 6, 4),
        ("जातीय भेदभाव तथा छुवाछुत उन्मूलन राष्ट्रिय दिवस", 2026, 6, 4),
        ("निजामती सेवा दिवस", 2024, 9, 7),
        ("निजामती सेवा दिवस", 2026, 9, 7),
        ("जेनजी सहिद दिवस", 2026, 9, 8),
    ] {
        let calendar = HolidayCalendar::for_year(&NEPAL, None, year);
        let own = named(&calendar, local_name);
        assert_eq!(
            own.iter().map(|holiday| holiday.date).collect::<Vec<_>>(),
            [ymd(year, month, day)]
        );
        assert_eq!(own[0].kind, Kind::Observance);
        assert!(
            calendar.is_business_day(ymd(year, month, day))
                || calendar.is_weekend(ymd(year, month, day))
        );
    }
    let before = HolidayCalendar::for_year(&NEPAL, None, 2025);
    assert!(named(&before, "जेनजी सहिद दिवस").is_empty());
}

#[test]
fn the_table_lists_nepal_s_groups() {
    let ids: Vec<&str> = NEPAL.groups().iter().map(|group| group.id).collect();
    assert_eq!(
        ids,
        [
            "dura",
            "kirat",
            "muslims",
            "newar",
            "persons-with-disabilities",
            "sikhs",
            "women"
        ]
    );
}
