//! The minority festivals China's autonomous regions give days off for,
//! notice by notice. The rows are the notices' days, not dates this crate
//! produced.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::CHINA;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::rule::Kind;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn days(region: &str, local_name: &str, year: i64) -> Vec<Rd> {
    HolidayCalendar::for_year(&CHINA, Some(region), year)
        .all()
        .iter()
        .filter(|holiday| holiday.local_name == local_name)
        .map(|holiday| holiday.date)
        .collect()
}

fn span(year: i64, month: u8, first: u8, last: u8) -> Vec<Rd> {
    (first..=last).map(|day| ymd(year, month, day)).collect()
}

#[test]
fn each_region_has_its_notices_days() {
    assert_eq!(days("CN-GX", "壮族三月三", 2024), span(2024, 4, 11, 12));
    assert_eq!(days("CN-GX", "壮族三月三", 2026), span(2026, 4, 17, 20));
    assert_eq!(days("CN-XJ", "肉孜节", 2023), [ymd(2023, 4, 21)]);
    assert_eq!(days("CN-XJ", "肉孜节", 2026), span(2026, 3, 20, 22));
    assert_eq!(days("CN-XJ", "古尔邦节", 2025), span(2025, 6, 6, 10));
    assert_eq!(
        days("CN-NX", "开斋节", 2025),
        [ymd(2025, 3, 31), ymd(2025, 4, 1)]
    );
    assert!(days("CN-NX", "开斋节", 2026).is_empty());
    assert_eq!(days("CN-NX", "古尔邦节", 2026), span(2026, 5, 27, 28));
    let xinjiang = HolidayCalendar::for_year(&CHINA, Some("cn-xj"), 2026);
    assert!(xinjiang.is_holiday(ymd(2026, 5, 27)));
    let kurban = xinjiang
        .all()
        .iter()
        .find(|holiday| holiday.local_name == "古尔邦节")
        .expect("古尔邦节");
    assert_eq!(kurban.kind, Kind::Public);
    assert_eq!(kurban.regions, ["CN-XJ"]);
}

#[test]
fn a_region_s_days_are_its_own() {
    let everyone = HolidayCalendar::for_year(&CHINA, None, 2026);
    assert!(!everyone.is_holiday(ymd(2026, 5, 27)));
    assert!(days("CN-GX", "古尔邦节", 2026).is_empty());
    assert!(days("CN-NX", "肉孜节", 2026).is_empty());
    assert_eq!(CHINA.regions(), ["CN-GX", "CN-NX", "CN-XJ"]);
}

#[test]
fn a_year_without_its_notice_is_a_gap_and_before_the_instrument_absent() {
    // 广西's 令第98号 set the days from 2014: before it they are absent.
    // 新疆's and 宁夏's first instruments were not read: every earlier year
    // is a gap.
    let gap = |region: &str, year: i64, name: &str| {
        HolidayCalendar::for_year(&CHINA, Some(region), year)
            .gaps()
            .iter()
            .any(|gap| gap.name == name && gap.year == year)
    };
    assert!(gap("CN-GX", 2014, "Sanyuesan"));
    assert!(gap("CN-GX", 2025, "Sanyuesan"));
    assert!(gap("CN-GX", 2027, "Sanyuesan"));
    assert!(!gap("CN-GX", 2013, "Sanyuesan"));
    assert!(gap("CN-XJ", 2012, "Eid al-Adha"));
    assert!(gap("CN-XJ", 2022, "Eid al-Fitr"));
    assert!(gap("CN-XJ", 2011, "Eid al-Fitr"));
    assert!(gap("CN-XJ", 2027, "Eid al-Fitr"));
    assert!(gap("CN-NX", 2022, "Eid al-Adha"));
    assert!(gap("CN-NX", 2027, "Eid al-Adha"));
    assert!(!gap("CN-GX", 2024, "Sanyuesan"));
}
