//! New Zealand's provincial anniversary days.
//!
//! The rows are Employment New Zealand's list of observed anniversary
//! days, "Public holidays and anniversary dates" for 2026 and 2027 and
//! "Previous years" for 2010 to 2025, retrieved 2026-09-29, not dates this
//! crate produced; South Canterbury's Dominion Day, which has no region
//! code of its own, is left out.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::NEW_ZEALAND;
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::Kind;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// Every observed day of the list, by year and region.
#[rustfmt::skip]
const LISTED: &[(i64, u8, u8, &str)] = &[
    (2010, 2, 1, "NZ-AUK"),
    (2010, 3, 8, "NZ-TKI"),
    (2010, 10, 22, "NZ-HKB"),
    (2010, 1, 25, "NZ-WGN"),
    (2010, 11, 1, "NZ-MBH"),
    (2010, 2, 1, "NZ-NSN"),
    (2010, 11, 12, "NZ-CAN"),
    (2010, 11, 29, "NZ-WTC"),
    (2010, 3, 22, "NZ-OTA"),
    (2010, 1, 18, "NZ-STL"),
    (2010, 11, 29, "NZ-CIT"),
    (2011, 1, 31, "NZ-AUK"),
    (2011, 3, 14, "NZ-TKI"),
    (2011, 10, 21, "NZ-HKB"),
    (2011, 1, 24, "NZ-WGN"),
    (2011, 10, 31, "NZ-MBH"),
    (2011, 1, 31, "NZ-NSN"),
    (2011, 11, 11, "NZ-CAN"),
    (2011, 11, 28, "NZ-WTC"),
    (2011, 3, 21, "NZ-OTA"),
    (2011, 1, 17, "NZ-STL"),
    (2011, 11, 28, "NZ-CIT"),
    (2012, 1, 30, "NZ-AUK"),
    (2012, 3, 12, "NZ-TKI"),
    (2012, 10, 19, "NZ-HKB"),
    (2012, 1, 23, "NZ-WGN"),
    (2012, 10, 29, "NZ-MBH"),
    (2012, 1, 30, "NZ-NSN"),
    (2012, 11, 16, "NZ-CAN"),
    (2012, 12, 3, "NZ-WTC"),
    (2012, 3, 26, "NZ-OTA"),
    (2012, 4, 10, "NZ-STL"),
    (2012, 12, 3, "NZ-CIT"),
    (2013, 1, 28, "NZ-AUK"),
    (2013, 3, 11, "NZ-TKI"),
    (2013, 10, 25, "NZ-HKB"),
    (2013, 1, 21, "NZ-WGN"),
    (2013, 11, 4, "NZ-MBH"),
    (2013, 2, 4, "NZ-NSN"),
    (2013, 11, 15, "NZ-CAN"),
    (2013, 12, 2, "NZ-WTC"),
    (2013, 3, 25, "NZ-OTA"),
    (2013, 4, 2, "NZ-STL"),
    (2013, 12, 2, "NZ-CIT"),
    (2014, 1, 27, "NZ-AUK"),
    (2014, 3, 10, "NZ-TKI"),
    (2014, 10, 24, "NZ-HKB"),
    (2014, 1, 20, "NZ-WGN"),
    (2014, 11, 3, "NZ-MBH"),
    (2014, 2, 3, "NZ-NSN"),
    (2014, 11, 14, "NZ-CAN"),
    (2014, 12, 1, "NZ-WTC"),
    (2014, 3, 24, "NZ-OTA"),
    (2014, 4, 22, "NZ-STL"),
    (2014, 12, 1, "NZ-CIT"),
    (2015, 1, 26, "NZ-AUK"),
    (2015, 3, 9, "NZ-TKI"),
    (2015, 10, 23, "NZ-HKB"),
    (2015, 1, 19, "NZ-WGN"),
    (2015, 11, 2, "NZ-MBH"),
    (2015, 2, 2, "NZ-NSN"),
    (2015, 11, 13, "NZ-CAN"),
    (2015, 11, 30, "NZ-WTC"),
    (2015, 3, 23, "NZ-OTA"),
    (2015, 4, 7, "NZ-STL"),
    (2015, 11, 30, "NZ-CIT"),
    (2016, 2, 1, "NZ-AUK"),
    (2016, 3, 14, "NZ-TKI"),
    (2016, 10, 21, "NZ-HKB"),
    (2016, 1, 25, "NZ-WGN"),
    (2016, 10, 31, "NZ-MBH"),
    (2016, 2, 1, "NZ-NSN"),
    (2016, 11, 11, "NZ-CAN"),
    (2016, 11, 28, "NZ-WTC"),
    (2016, 3, 21, "NZ-OTA"),
    (2016, 3, 29, "NZ-STL"),
    (2016, 11, 28, "NZ-CIT"),
    (2017, 1, 30, "NZ-AUK"),
    (2017, 3, 13, "NZ-TKI"),
    (2017, 10, 20, "NZ-HKB"),
    (2017, 1, 23, "NZ-WGN"),
    (2017, 10, 30, "NZ-MBH"),
    (2017, 1, 30, "NZ-NSN"),
    (2017, 11, 17, "NZ-CAN"),
    (2017, 12, 4, "NZ-WTC"),
    (2017, 3, 20, "NZ-OTA"),
    (2017, 4, 18, "NZ-STL"),
    (2017, 11, 27, "NZ-CIT"),
    (2018, 1, 29, "NZ-AUK"),
    (2018, 3, 12, "NZ-TKI"),
    (2018, 10, 19, "NZ-HKB"),
    (2018, 1, 22, "NZ-WGN"),
    (2018, 10, 29, "NZ-MBH"),
    (2018, 1, 29, "NZ-NSN"),
    (2018, 11, 16, "NZ-CAN"),
    (2018, 12, 3, "NZ-WTC"),
    (2018, 3, 26, "NZ-OTA"),
    (2018, 4, 3, "NZ-STL"),
    (2018, 12, 3, "NZ-CIT"),
    (2019, 1, 28, "NZ-AUK"),
    (2019, 3, 11, "NZ-TKI"),
    (2019, 10, 25, "NZ-HKB"),
    (2019, 1, 21, "NZ-WGN"),
    (2019, 11, 4, "NZ-MBH"),
    (2019, 2, 4, "NZ-NSN"),
    (2019, 11, 15, "NZ-CAN"),
    (2019, 12, 2, "NZ-WTC"),
    (2019, 3, 25, "NZ-OTA"),
    (2019, 4, 23, "NZ-STL"),
    (2019, 12, 2, "NZ-CIT"),
    (2020, 1, 27, "NZ-AUK"),
    (2020, 3, 9, "NZ-TKI"),
    (2020, 10, 23, "NZ-HKB"),
    (2020, 1, 20, "NZ-WGN"),
    (2020, 11, 2, "NZ-MBH"),
    (2020, 2, 3, "NZ-NSN"),
    (2020, 11, 13, "NZ-CAN"),
    (2020, 11, 30, "NZ-WTC"),
    (2020, 3, 23, "NZ-OTA"),
    (2020, 4, 14, "NZ-STL"),
    (2020, 11, 30, "NZ-CIT"),
    (2021, 2, 1, "NZ-AUK"),
    (2021, 3, 8, "NZ-TKI"),
    (2021, 10, 22, "NZ-HKB"),
    (2021, 1, 25, "NZ-WGN"),
    (2021, 11, 1, "NZ-MBH"),
    (2021, 2, 1, "NZ-NSN"),
    (2021, 11, 12, "NZ-CAN"),
    (2021, 11, 29, "NZ-WTC"),
    (2021, 3, 22, "NZ-OTA"),
    (2021, 4, 6, "NZ-STL"),
    (2021, 11, 29, "NZ-CIT"),
    (2022, 1, 31, "NZ-AUK"),
    (2022, 3, 14, "NZ-TKI"),
    (2022, 10, 21, "NZ-HKB"),
    (2022, 1, 24, "NZ-WGN"),
    (2022, 10, 31, "NZ-MBH"),
    (2022, 1, 31, "NZ-NSN"),
    (2022, 11, 11, "NZ-CAN"),
    (2022, 11, 28, "NZ-WTC"),
    (2022, 3, 21, "NZ-OTA"),
    (2022, 4, 19, "NZ-STL"),
    (2022, 11, 28, "NZ-CIT"),
    (2023, 1, 30, "NZ-AUK"),
    (2023, 3, 13, "NZ-TKI"),
    (2023, 10, 20, "NZ-HKB"),
    (2023, 1, 23, "NZ-WGN"),
    (2023, 10, 30, "NZ-MBH"),
    (2023, 1, 30, "NZ-NSN"),
    (2023, 11, 17, "NZ-CAN"),
    (2023, 12, 4, "NZ-WTC"),
    (2023, 3, 20, "NZ-OTA"),
    (2023, 4, 11, "NZ-STL"),
    (2023, 11, 27, "NZ-CIT"),
    (2024, 1, 29, "NZ-AUK"),
    (2024, 3, 11, "NZ-TKI"),
    (2024, 10, 25, "NZ-HKB"),
    (2024, 1, 22, "NZ-WGN"),
    (2024, 11, 4, "NZ-MBH"),
    (2024, 1, 29, "NZ-NSN"),
    (2024, 11, 15, "NZ-CAN"),
    (2024, 12, 2, "NZ-WTC"),
    (2024, 3, 25, "NZ-OTA"),
    (2024, 4, 2, "NZ-STL"),
    (2024, 12, 2, "NZ-CIT"),
    (2025, 1, 27, "NZ-AUK"),
    (2025, 3, 10, "NZ-TKI"),
    (2025, 10, 24, "NZ-HKB"),
    (2025, 1, 20, "NZ-WGN"),
    (2025, 11, 3, "NZ-MBH"),
    (2025, 2, 3, "NZ-NSN"),
    (2025, 11, 14, "NZ-CAN"),
    (2025, 12, 1, "NZ-WTC"),
    (2025, 3, 24, "NZ-OTA"),
    (2025, 4, 22, "NZ-STL"),
    (2025, 12, 1, "NZ-CIT"),
    (2026, 1, 26, "NZ-AUK"),
    (2026, 3, 9, "NZ-TKI"),
    (2026, 10, 23, "NZ-HKB"),
    (2026, 1, 19, "NZ-WGN"),
    (2026, 11, 2, "NZ-MBH"),
    (2026, 2, 2, "NZ-NSN"),
    (2026, 11, 13, "NZ-CAN"),
    (2026, 11, 30, "NZ-WTC"),
    (2026, 3, 23, "NZ-OTA"),
    (2026, 4, 7, "NZ-STL"),
    (2026, 11, 30, "NZ-CIT"),
    (2027, 2, 1, "NZ-AUK"),
    (2027, 3, 8, "NZ-TKI"),
    (2027, 10, 22, "NZ-HKB"),
    (2027, 1, 25, "NZ-WGN"),
    (2027, 11, 1, "NZ-MBH"),
    (2027, 2, 1, "NZ-NSN"),
    (2027, 11, 12, "NZ-CAN"),
    (2027, 11, 29, "NZ-WTC"),
    (2027, 3, 22, "NZ-OTA"),
    (2027, 3, 30, "NZ-STL"),
    (2027, 11, 29, "NZ-CIT"),
];

/// The regions that have an anniversary day, and their days in a year.
fn own_days(region: &str, year: i64) -> Vec<Holiday> {
    HolidayCalendar::for_year(&NEW_ZEALAND, Some(region), year)
        .in_year(year)
        .iter()
        .filter(|holiday| !holiday.regions.is_empty())
        .copied()
        .collect()
}

#[test]
fn every_anniversary_day_of_the_list_is_reproduced() {
    for &(year, month, day, region) in LISTED {
        let days = own_days(region, year);
        assert_eq!(days.len(), 1, "{region} {year}: {days:?}");
        assert_eq!(days[0].date, ymd(year, month, day), "{region} {year}");
        assert_eq!(days[0].regions, [region]);
        assert_eq!(days[0].kind, Kind::Public, "{region} {year}");
    }
    assert_eq!(LISTED.len(), 18 * 11);
}

#[test]
fn before_the_list_begins_an_anniversary_day_is_a_gap() {
    for region in [
        "NZ-AUK", "NZ-TKI", "NZ-HKB", "NZ-WGN", "NZ-MBH", "NZ-NSN", "NZ-CAN", "NZ-WTC", "NZ-OTA",
        "NZ-STL", "NZ-CIT",
    ] {
        assert!(own_days(region, 2009).is_empty(), "{region}");
        let gaps = HolidayCalendar::for_year(&NEW_ZEALAND, Some(region), 2009);
        assert_eq!(gaps.gaps().len(), 1, "{region}");
        assert!(
            gaps.gaps()[0].name.ends_with("Anniversary Day")
                || gaps.gaps()[0].name.contains("Show Day")
        );
        assert_eq!(own_days(region, 2010).len(), 1, "{region}");
        assert!(HolidayCalendar::for_year(&NEW_ZEALAND, Some(region), 2010).is_complete());
        assert_eq!(own_days(region, 2040).len(), 1, "{region}");
    }
    // Nationwide, no anniversary day, and no gap for one.
    assert!(HolidayCalendar::for_year(&NEW_ZEALAND, None, 2009).is_complete());
}

#[test]
fn a_region_that_bears_no_province_s_name_is_a_gap() {
    // No source read says which province's day these regions keep.
    for region in ["NZ-NTL", "NZ-WKO", "NZ-BOP", "NZ-GIS", "NZ-MWT", "NZ-TAS"] {
        let calendar = HolidayCalendar::for_year(&NEW_ZEALAND, Some(region), 2026);
        let names: Vec<&str> = calendar.gaps().iter().map(|gap| gap.name).collect();
        assert_eq!(names, [hc_holiday::rule::UNREAD_SUBDIVISION], "{region}");
        // The nationwide days are still there.
        assert!(calendar.is_holiday(ymd(2026, 12, 25)), "{region}");
    }
}

#[test]
fn an_anniversary_day_is_its_region_s_alone() {
    // Wellington Anniversary Day 2026 is Monday 19 January: a day off in
    // Wellington, a working day in Auckland and nationwide.
    let day = ymd(2026, 1, 19);
    let wellington = HolidayCalendar::for_year(&NEW_ZEALAND, Some("NZ-WGN"), 2026);
    assert!(wellington.is_holiday(day));
    assert!(!wellington.is_business_day(day));
    assert_eq!(wellington.name_on(day), Some("Wellington Anniversary Day"));
    for region in [None, Some("NZ-AUK")] {
        let calendar = HolidayCalendar::for_year(&NEW_ZEALAND, region, 2026);
        assert!(!calendar.is_holiday(day), "{region:?}");
        assert!(calendar.is_business_day(day), "{region:?}");
    }
    // The regions that bear no province's name have no anniversary day.
    for region in ["NZ-NTL", "NZ-WKO", "NZ-BOP", "NZ-GIS", "NZ-MWT", "NZ-TAS"] {
        assert!(own_days(region, 2026).is_empty(), "{region}");
    }
    // The region is matched as every identifier is.
    assert_eq!(own_days("nz-wgn", 2026).len(), 1);
}

#[test]
fn southland_moved_to_easter_tuesday_in_2012() {
    // Monday nearest 17 January in 2011; Easter Tuesday, 10 April, in 2012.
    assert_eq!(own_days("NZ-STL", 2011)[0].date, ymd(2011, 1, 17));
    assert_eq!(own_days("NZ-STL", 2012)[0].date, ymd(2012, 4, 10));
    let calendar = HolidayCalendar::for_year(&NEW_ZEALAND, Some("NZ-STL"), 2012);
    assert!(!calendar.is_holiday(ymd(2012, 1, 16)));
}
