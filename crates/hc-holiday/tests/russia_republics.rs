//! The non-working days Russia's republics set, act by act. The rows are
//! the dates the laws, decrees and resolutions read give, not dates this
//! crate produced.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::RUSSIA;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::rule::Kind;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn days(region: &str, local_name: &str, year: i64) -> Vec<Rd> {
    HolidayCalendar::for_year(&RUSSIA, Some(region), year)
        .all()
        .iter()
        .filter(|holiday| holiday.local_name == local_name)
        .map(|holiday| holiday.date)
        .collect()
}

fn has_gap(region: &str, name: &str, year: i64) -> bool {
    HolidayCalendar::for_year(&RUSSIA, Some(region), year)
        .gaps()
        .iter()
        .any(|gap| gap.name == name && gap.year == year)
}

#[test]
fn tatarstan_keeps_its_two_days_and_the_rais_s_bayrams() {
    assert_eq!(
        days("RU-TA", "День Республики Татарстан", 2026),
        [ymd(2026, 8, 30)]
    );
    assert_eq!(
        days("RU-TA", "День Конституции Республики Татарстан", 2004),
        [ymd(2004, 11, 6)]
    );
    // The law of 1992 set the days; its texts before 2003 were not read.
    assert!(days("RU-TA", "День Республики Татарстан", 2003).is_empty());
    assert!(has_gap("RU-TA", "Day of the Republic of Tatarstan", 2003));
    assert!(!has_gap("RU-TA", "Day of the Republic of Tatarstan", 1991));
    assert_eq!(days("RU-TA", "Ураза-байрам", 2026), [ymd(2026, 3, 20)]);
    assert_eq!(days("RU-TA", "Курбан-байрам", 2025), [ymd(2025, 6, 6)]);
    assert!(has_gap("RU-TA", "Kurban Bayram", 2023));
    assert!(has_gap("RU-TA", "Uraza Bayram", 2027));
    assert!(!has_gap("RU-TA", "Uraza Bayram", 2010));
    // 30 August 2015 was a Sunday, and the law then moved the day off; from
    // 2017 it moves nothing, and Sunday 30 August 2020 leaves no gap.
    assert!(has_gap(
        "RU-TA",
        "Day off moved from a holiday on the weekend",
        2015
    ));
    assert!(!has_gap(
        "RU-TA",
        "Day off moved from a holiday on the weekend",
        2020
    ));
    let everyone = HolidayCalendar::for_year(&RUSSIA, None, 2026);
    assert!(everyone.is_business_day(ymd(2026, 3, 20)));
    let tatarstan = HolidayCalendar::for_year(&RUSSIA, Some("ru-ta"), 2026);
    assert!(!tatarstan.is_business_day(ymd(2026, 3, 20)));
}

#[test]
fn bashkortostan_and_adygea_move_a_weekend_day_and_so_have_a_gap() {
    assert_eq!(days("RU-BA", "День Республики", 2026), [ymd(2026, 10, 11)]);
    assert_eq!(days("RU-BA", "Курбан-байрам", 2024), [ymd(2024, 6, 16)]);
    // Sunday 16 June 2024 and Sunday 30 March 2025; Sunday 11 October 2026.
    for year in [2024, 2025, 2026] {
        assert!(
            has_gap("RU-BA", "Day off moved from a holiday on the weekend", year),
            "{year}"
        );
    }
    assert!(!has_gap(
        "RU-BA",
        "Day off moved from a holiday on the weekend",
        2023
    ));
    assert!(has_gap("RU-BA", "Republic Day", 2005));
    assert_eq!(
        days("RU-AD", "День поминовения усопших (Радоница)", 2026),
        [ymd(2026, 4, 21)]
    );
    let transferred: Vec<Rd> = HolidayCalendar::for_year(&RUSSIA, Some("RU-AD"), 2026)
        .all()
        .iter()
        .filter(|holiday| holiday.name == "Day off transferred by the Head")
        .map(|holiday| holiday.date)
        .collect();
    assert_eq!(transferred, [ymd(2026, 4, 20)]);
    let adygea = HolidayCalendar::for_year(&RUSSIA, Some("RU-AD"), 2026);
    assert!(
        adygea.is_business_day(ymd(2026, 4, 25)),
        "Saturday 25 April 2026 worked"
    );
    assert!(!adygea.is_business_day(ymd(2026, 4, 20)));
}

#[test]
fn the_north_caucasus_republics_keep_their_decrees_days() {
    let span = |year, month, first: u8, last: u8| -> Vec<Rd> {
        (first..=last).map(|day| ymd(year, month, day)).collect()
    };
    assert_eq!(days("RU-CE", "Ураза-Байрам", 2026), span(2026, 3, 19, 21));
    assert_eq!(days("RU-CE", "Курбан-Байрам", 2026), span(2026, 5, 27, 29));
    assert_eq!(
        days("RU-CE", "День Конституции Чеченской Республики", 2026),
        [ymd(2026, 3, 23)]
    );
    assert!(days("RU-CE", "Ураза-Байрам", 2023).is_empty());
    assert!(has_gap("RU-CE", "Uraza Bayram", 2023));
    // The decree of 24 March 2003 set 23 March: before it, absent.
    assert!(days("RU-CE", "День Конституции Чеченской Республики", 2003).is_empty());
    assert!(!has_gap(
        "RU-CE",
        "Constitution Day of the Chechen Republic",
        2003
    ));
    assert_eq!(days("RU-DA", "Ураза-Байрам", 2026), span(2026, 3, 19, 20));
    assert_eq!(
        days("RU-DA", "День Конституции Республики Дагестан", 2026),
        [ymd(2026, 7, 26)]
    );
    assert_eq!(days("RU-IN", "Гӏурба", 2026), span(2026, 5, 27, 29));
    assert_eq!(days("RU-KC", "Курбан-Байрам", 2026), [ymd(2026, 5, 27)]);
    assert!(has_gap("RU-KB", "Kurban Bayram", 2025));
}

#[test]
fn sakha_s_days_are_its_government_s() {
    let sakha = HolidayCalendar::for_year(&RUSSIA, Some("RU-SA"), 2026);
    let ysyakh = sakha
        .all()
        .iter()
        .find(|holiday| holiday.local_name == "День национального праздника «Ысыах»")
        .expect("Ysyakh");
    assert_eq!(ysyakh.date, ymd(2026, 6, 21));
    assert_eq!(ysyakh.kind, Kind::Government);
    assert!(days("RU-SA", "День Республики Саха (Якутия)", 2018).is_empty());
}

#[test]
fn a_republic_s_days_are_its_own() {
    assert!(days("RU-BA", "День Республики Татарстан", 2026).is_empty());
    let regions = RUSSIA.regions();
    for code in [
        "RU-AD", "RU-BA", "RU-CE", "RU-DA", "RU-SA", "RU-TA", "RU-TY",
    ] {
        assert!(regions.contains(&code), "{code}");
    }
    assert!(!regions.contains(&"RU-BU"));
}
