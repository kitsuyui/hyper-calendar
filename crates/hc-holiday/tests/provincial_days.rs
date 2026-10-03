//! The provincial and district days of Vanuatu, Solomon Islands and
//! Bhutan, each in its own subdivision.
//!
//! The rows are the sources' days: the Government of Vanuatu's list of
//! holidays, the Island Sun's report of Solomon Islands' notice for 2026,
//! and the Ministry of Home Affairs' lists for 2025 and 2026 as the
//! repository's transcription of them gives them
//! (`hc-calendars-lunar`'s `the_bhutanese_calendar_gives_the_governments_dates`).

use hc_calendar::Rd;
use hc_calendars_lunar::tibetan::TIBETAN_BHUTAN;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::{BHUTAN, SOLOMON_ISLANDS, VANUATU};
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::{Confidence, Kind, RuleSet};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// A region's own entries in a year, not the nationwide ones.
fn own_days(table: &RuleSet, region: &str, year: i64) -> Vec<Holiday> {
    HolidayCalendar::for_year(table, Some(region), year)
        .in_year(year)
        .iter()
        .filter(|holiday| !holiday.regions.is_empty())
        .copied()
        .collect()
}

/// The dates of a region's own entries in a year.
fn own_dates(table: &RuleSet, region: &str, year: i64) -> Vec<Rd> {
    own_days(table, region, year)
        .iter()
        .map(|holiday| holiday.date)
        .collect()
}

/// Vanuatu's six provinces, their days' names and dates.
const VU: &[(&str, &str, u8, u8)] = &[
    ("VU-SEE", "Shefa Day", 6, 18),
    ("VU-PAM", "Penama Day", 9, 16),
    ("VU-SAM", "Sanma Day", 9, 24),
    ("VU-TOB", "Torba Day", 10, 2),
    ("VU-TAE", "Tafea Day", 10, 8),
    ("VU-MAP", "Malampa Day", 10, 10),
];

#[test]
fn vanuatu_keeps_each_provincial_day_in_its_province_from_2020() {
    for &(region, name, month, day) in VU {
        for year in [2020, 2026] {
            let days = own_days(&VANUATU, region, year);
            assert_eq!(days.len(), 1, "{region} {year}");
            assert_eq!(days[0].name, name);
            assert_eq!(days[0].date, ymd(year, month, day));
            assert_eq!(days[0].kind, Kind::Public);
            assert_eq!(days[0].regions, [region]);
        }
        assert!(own_days(&VANUATU, region, 2019).is_empty(), "{region}");
        // No list before 2020's was read: 2019 is a gap, 2020 is not.
        let names = |year| {
            HolidayCalendar::for_year(&VANUATU, Some(region), year)
                .gaps()
                .iter()
                .map(|gap| gap.name)
                .collect::<Vec<_>>()
        };
        assert_eq!(names(2019), [name], "{region}");
        assert!(names(2020).is_empty(), "{region}");
    }
    // Shefa Day 2026 is a Thursday: a day off in Shefa, a working day in
    // Sanma and nationwide.
    let day = ymd(2026, 6, 18);
    assert!(HolidayCalendar::for_year(&VANUATU, Some("VU-SEE"), 2026).is_holiday(day));
    for region in [None, Some("VU-SAM")] {
        let calendar = HolidayCalendar::for_year(&VANUATU, region, 2026);
        assert!(calendar.is_business_day(day), "{region:?}");
    }
    // A provincial day on a Sunday stays there: Penama Day 2029.
    let calendar = HolidayCalendar::for_year(&VANUATU, Some("VU-PAM"), 2029);
    assert!(calendar.is_business_day(ymd(2029, 9, 17)));
}

/// Solomon Islands' provinces and their days of 2026, as observed.
const SB: &[(&str, u8, u8)] = &[
    ("SB-CH", 2, 25),
    ("SB-IS", 6, 2),
    ("SB-TE", 6, 8),
    ("SB-CE", 6, 29),
    ("SB-RB", 7, 20),
    ("SB-GU", 7, 31),
    ("SB-MK", 8, 3),
    ("SB-ML", 8, 14),
    ("SB-WE", 12, 7),
];

#[test]
fn solomon_islands_keeps_the_provincial_days_of_2026() {
    for &(region, month, day) in SB {
        let days = own_days(&SOLOMON_ISLANDS, region, 2026);
        assert_eq!(days.len(), 1, "{region}");
        assert_eq!(days[0].date, ymd(2026, month, day), "{region}");
        assert_eq!(days[0].kind, Kind::Public);
        assert_eq!(days[0].regions, [region]);
        // Before the notice read, nothing, and a gap: the notices before
        // 2026's were not read.
        let before = HolidayCalendar::for_year(&SOLOMON_ISLANDS, Some(region), 2025);
        assert!(own_days(&SOLOMON_ISLANDS, region, 2025).is_empty());
        assert_eq!(before.gaps().len(), 1, "{region}");
        // After it, a gap: the days are appointed each year.
        let after = HolidayCalendar::for_year(&SOLOMON_ISLANDS, Some(region), 2027);
        assert_eq!(after.gaps().len(), 1, "{region}");
    }
    // Guadalcanal's day is kept on Friday 31 July, not Saturday 1 August,
    // and Honiara and the nationwide calendar have neither.
    let guadalcanal = HolidayCalendar::for_year(&SOLOMON_ISLANDS, Some("SB-GU"), 2026);
    assert!(guadalcanal.is_holiday(ymd(2026, 7, 31)));
    assert!(!guadalcanal.is_holiday(ymd(2026, 8, 1)));
    for region in [None, Some("SB-CT")] {
        let calendar = HolidayCalendar::for_year(&SOLOMON_ISLANDS, region, 2026);
        assert!(calendar.is_business_day(ymd(2026, 7, 31)), "{region:?}");
        assert!(calendar.is_complete(), "{region:?}");
    }
}

#[test]
fn bhutan_keeps_thimphu_s_festivals_in_thimphu() {
    for (year, drubchoe, tshechu) in [(2025, (9, 28), (10, 2)), (2026, (9, 17), (9, 21))] {
        let first = ymd(year, tshechu.0, tshechu.1);
        let mut expected = vec![ymd(year, drubchoe.0, drubchoe.1)];
        expected.extend((0..3).map(|offset| Rd(first.0 + offset)));
        assert_eq!(own_dates(&BHUTAN, "BT-15", year), expected, "{year}");
        for holiday in own_days(&BHUTAN, "BT-15", year) {
            assert_eq!(holiday.confidence, Confidence::Exact);
            assert_eq!(holiday.kind, Kind::Public);
            assert_eq!(holiday.regions, ["BT-15"]);
        }
    }
    // Thimphu Drubchoe 2026 is a Thursday, a working day in Paro.
    let day = ymd(2026, 9, 17);
    assert!(HolidayCalendar::for_year(&BHUTAN, Some("BT-15"), 2026).is_holiday(day));
    for region in [None, Some("BT-11")] {
        let calendar = HolidayCalendar::for_year(&BHUTAN, region, 2026);
        assert!(calendar.is_business_day(day), "{region:?}");
    }
}

/// The Ministry's notification of 7 September 2021 moved Thimphu Dromche
/// and Tshechu to dates its page gives only in an image, which was not
/// read: 2021 is a gap for both in Thimphu. Every year before the lists of
/// 2025 is a gap too (ADR 0013), the years after them are predicted, and
/// nothing changes outside Thimphu.
#[test]
fn bhutan_reports_thimphu_s_moved_festivals_of_2021_as_a_gap() {
    for year in [2020, 2021, 2022] {
        let calendar = HolidayCalendar::for_year(&BHUTAN, Some("BT-15"), year);
        let mut gaps: Vec<&str> = calendar.gaps().iter().map(|gap| gap.name).collect();
        gaps.sort_unstable();
        gaps.dedup();
        assert!(gaps.contains(&"Thimphu Drubchoe"), "{year} {gaps:?}");
        assert!(gaps.contains(&"Thimphu Tshechu"), "{year} {gaps:?}");
        assert!(own_days(&BHUTAN, "BT-15", year).is_empty(), "{year}");
    }
    // After the lists the days are the Bhutanese calendar's, approximate.
    let days = own_days(&BHUTAN, "BT-15", 2027);
    assert!(days.len() >= 3, "{days:?}");
    assert!(
        days.iter()
            .all(|holiday| holiday.confidence == Confidence::Approximate)
    );
    let nationwide = HolidayCalendar::for_year(&BHUTAN, None, 2021);
    assert!(
        !nationwide
            .gaps()
            .iter()
            .any(|gap| gap.name.starts_with("Thimphu"))
    );
}

#[test]
fn bhutan_predicts_thimphu_s_festivals_on_the_bhutanese_calendar() {
    // Outside the lists, the 6th and the 10th to 12th of the 8th month,
    // approximate; the rule gives the lists' own days in their years.
    let mut checked = 0;
    for year in [2024, 2027, 2030] {
        let days = own_days(&BHUTAN, "BT-15", year);
        let calendar = HolidayCalendar::for_year(&BHUTAN, Some("BT-15"), year);
        if calendar
            .gaps()
            .iter()
            .any(|gap| gap.name.starts_with("Thimphu"))
        {
            continue;
        }
        assert_eq!(days.len(), 4, "{year}");
        for holiday in &days {
            assert_eq!(holiday.confidence, Confidence::Approximate, "{year}");
            let date = TIBETAN_BHUTAN
                .date_from_fixed(holiday.date)
                .expect("in range");
            assert_eq!(date.month.ordinal, 8, "{year}");
            assert!(!date.month.leap, "{year}");
            let expected: &[u8] = if holiday.name == "Thimphu Drubchoe" {
                &[6]
            } else {
                &[10, 11, 12]
            };
            assert!(expected.contains(&date.day), "{year} {}", holiday.name);
        }
        checked += 1;
    }
    assert!(checked >= 2, "{checked}");
    for &(year, month, day) in &[(2025, 9, 28), (2025, 10, 2), (2026, 9, 17), (2026, 9, 23)] {
        let date = TIBETAN_BHUTAN
            .date_from_fixed(ymd(year, month, day))
            .expect("in range");
        assert_eq!(date.month.ordinal, 8);
    }
}
