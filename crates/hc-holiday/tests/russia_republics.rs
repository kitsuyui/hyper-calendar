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
    assert!(has_gap("RU-TA", "Kurban Bayram", 2014));
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
    assert!(days("RU-CE", "Ураза-Байрам", 2021).is_empty());
    assert!(has_gap("RU-CE", "Uraza Bayram", 2021));
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
    assert!(has_gap("RU-KB", "Kurban Bayram", 2014));
    assert!(has_gap("RU-KB", "Uraza Bayram", 2022));
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
    assert!(regions.contains(&"RU-BU"));
}

const SAGAALGAN: &str = "Праздник Белого месяца «Сагаалган»";
const MOVED: &str = "Day off moved from a holiday on the weekend";

fn transferred(region: &str, year: i64) -> Vec<Rd> {
    HolidayCalendar::for_year(&RUSSIA, Some(region), year)
        .all()
        .iter()
        .filter(|holiday| holiday.name == "Day off transferred by the Head")
        .map(|holiday| holiday.date)
        .collect()
}

/// The Head's decrees «О празднике Белого месяца "Сагаалган"»: 21 February
/// 2023 (No. 255, text read), 12 February 2021 (No. 263, text read) and
/// 19 February 2015 (No. 216, text read) are declared non-working; 2024 and
/// 2025 fall on a Saturday and the decrees move the day off to the Monday.
#[test]
fn buryatia_keeps_sagaalgan_by_the_heads_decree_each_year() {
    for (year, month, day) in [
        (2012, 2, 22),
        (2015, 2, 19),
        (2018, 2, 16),
        (2019, 2, 5),
        (2020, 2, 24),
        (2021, 2, 12),
        (2023, 2, 21),
        (2024, 2, 10),
        (2025, 3, 1),
        (2026, 2, 18),
    ] {
        assert_eq!(
            days("RU-BU", SAGAALGAN, year),
            [ymd(year, month, day)],
            "{year}"
        );
        // Not the nationwide calendar's.
        assert!(
            HolidayCalendar::for_year(&RUSSIA, None, year)
                .all()
                .iter()
                .all(|holiday| holiday.local_name != SAGAALGAN),
            "{year}"
        );
    }
    assert!(
        !HolidayCalendar::for_year(&RUSSIA, Some("RU-BU"), 2023).is_business_day(ymd(2023, 2, 21))
    );
    // Saturday 10 February 2024 and Saturday 1 March 2025: the day off
    // moves to the Monday, which the decree names, so there is no gap.
    assert_eq!(transferred("RU-BU", 2024), [ymd(2024, 2, 12)]);
    assert_eq!(transferred("RU-BU", 2025), [ymd(2025, 3, 3)]);
    assert!(!has_gap("RU-BU", MOVED, 2024));
    assert!(!has_gap("RU-BU", MOVED, 2025));
    // Monday 24 February 2020 was already a day off (the carry-over of
    // Sunday 23 February), and the decree moves the day off to the Tuesday.
    assert_eq!(transferred("RU-BU", 2020), [ymd(2020, 2, 25)]);
    // Decrees not read: 2013, 2014, 2016, 2017 and 2022, and every year
    // from 2009 to 2011 and from 2027.
    for year in [2009, 2011, 2013, 2014, 2016, 2017, 2022, 2027] {
        assert!(has_gap("RU-BU", "Sagaalgan", year), "{year}");
        assert!(days("RU-BU", SAGAALGAN, year).is_empty(), "{year}");
    }
    assert!(!has_gap("RU-BU", "Sagaalgan", 2008));
    assert!(!has_gap("RU-BU", "Sagaalgan", 2012));
    assert!(!has_gap("RU-BU", "Sagaalgan", 2026));
}

/// Mari El's Law № 21-З of 5 July 2022 names 4 November (the Day of the
/// Republic, which is also the federal Unity Day) and fixes Peledysh Payrem
/// on Saturdays: no day off of its own, so the republic is read and has no
/// day beyond the nationwide ones.
#[test]
fn mari_el_keeps_no_day_off_of_its_own() {
    let own = HolidayCalendar::for_year(&RUSSIA, Some("RU-ME"), 2026);
    let nationwide = HolidayCalendar::for_year(&RUSSIA, None, 2026);
    assert_eq!(own.all().len(), nationwide.all().len());
    assert!(
        own.gaps()
            .iter()
            .all(|gap| gap.name != hc_holiday::rule::UNREAD_SUBDIVISION)
    );
    assert!(RUSSIA.reads_region("RU-ME"));
    assert!(!RUSSIA.reads_region("RU-MO"));
}

/// The four republics read and found to keep no day off of their own are a
/// gap before the first year of the law read, not "no day" for every year
/// back to 1992 (audit 10, a4): Karelia from 1999 (Закон РК № 346-ЗРК),
/// Khakassia from 2005 (the law of 1992 as restated), Mari El from 2023
/// (Закон РМЭ № 21-З of 5 July 2022, in force on publication) and Udmurtia
/// from 2020 (Закон УР № 81-РЗ).
#[test]
fn the_republics_that_keep_no_day_are_a_gap_before_their_law() {
    for (region, first) in [
        ("RU-KR", 1999),
        ("RU-KK", 2005),
        ("RU-ME", 2023),
        ("RU-UD", 2020),
    ] {
        for year in [1995, first - 1] {
            if year >= first {
                continue;
            }
            let calendar = HolidayCalendar::for_year(&RUSSIA, Some(region), year);
            assert!(
                calendar.gaps().iter().any(|gap| !gap.name.is_empty()
                    && gap.name != hc_holiday::rule::UNREAD_SUBDIVISION
                    && gap.source.contains("Закон")),
                "{region} {year}"
            );
        }
        let calendar = HolidayCalendar::for_year(&RUSSIA, Some(region), first);
        let nationwide = HolidayCalendar::for_year(&RUSSIA, None, first);
        assert_eq!(calendar.all().len(), nationwide.all().len(), "{region}");
        assert!(
            calendar
                .gaps()
                .iter()
                .all(|gap| !gap.source.contains("Закон")),
            "{region} {first}"
        );
        assert!(RUSSIA.reads_region(region));
    }
}

/// The acts of 2015 to 2023 that were read: the dates their texts, or their
/// titles on the portal of official publication, give.
#[test]
fn the_acts_before_2024_that_were_read_give_their_dates() {
    // Tatarstan: Указ № УП-401 of 19.05.2017 names 25 June (a Sunday) and
    // 1 September; № 189 of 25.03.2023 names 21 April and 28 June.
    assert_eq!(days("RU-TA", "Ураза-байрам", 2017), [ymd(2017, 6, 25)]);
    assert_eq!(days("RU-TA", "Курбан-байрам", 2017), [ymd(2017, 9, 1)]);
    assert_eq!(days("RU-TA", "Ураза-байрам", 2023), [ymd(2023, 4, 21)]);
    assert_eq!(days("RU-TA", "Курбан-байрам", 2022), [ymd(2022, 7, 9)]);
    assert_eq!(days("RU-TA", "Курбан-байрам", 2015), [ymd(2015, 9, 24)]);
    assert!(has_gap("RU-TA", "Uraza Bayram", 2014));
    // Bashkortostan: Постановление № 431 of 02.09.2021 (2 May, 9 July) and
    // № 567 of 26.09.2022 (21 April, 28 June); the resolutions for 2018 to
    // 2020 were not read.
    assert_eq!(days("RU-BA", "Ураза-байрам", 2022), [ymd(2022, 5, 2)]);
    assert_eq!(days("RU-BA", "Курбан-байрам", 2023), [ymd(2023, 6, 28)]);
    assert_eq!(days("RU-BA", "Ураза-байрам", 2016), [ymd(2016, 7, 5)]);
    for year in 2018..=2020 {
        assert!(has_gap("RU-BA", "Uraza Bayram", year), "{year}");
        assert!(has_gap("RU-BA", "Kurban Bayram", year), "{year}");
    }
    assert!(!has_gap("RU-BA", "Uraza Bayram", 2021));
    // Adygea: Указ № 113 of 03.10.2022 gives the three days of 2023 and moves
    // Saturday 15 April to Monday 24 April and Sunday 1 October to Friday
    // 6 October.
    assert_eq!(days("RU-AD", "Ураза-Байрам", 2023), [ymd(2023, 4, 21)]);
    assert_eq!(
        days("RU-AD", "День поминовения усопших (Радоница)", 2023),
        [ymd(2023, 4, 25)]
    );
    assert_eq!(days("RU-AD", "Курбан-Байрам", 2023), [ymd(2023, 6, 28)]);
    assert_eq!(
        transferred("RU-AD", 2023),
        [ymd(2023, 4, 24), ymd(2023, 10, 6)]
    );
    let adygea = HolidayCalendar::for_year(&RUSSIA, Some("RU-AD"), 2023);
    assert!(adygea.is_business_day(ymd(2023, 4, 15)));
    assert!(adygea.is_business_day(ymd(2023, 10, 1)));
    assert!(!adygea.is_business_day(ymd(2023, 10, 6)));
    assert!(has_gap("RU-AD", "Kurban Bayram", 2022));
    // The decrees for 2024 and 2025 move a day off, by their titles; the
    // texts were not read.
    assert!(has_gap("RU-AD", "Day off transferred by the Head", 2024));
    assert!(has_gap("RU-AD", "Day off transferred by the Head", 2025));
    // Kabardino-Balkaria: the titles on the portal give the day: Kurban
    // Bayram 2022 on Monday 11 July (the festival fell on the Saturday), and
    // Uraza Bayram 2017 on Monday 26 June.
    assert_eq!(days("RU-KB", "Курбан-Байрам", 2022), [ymd(2022, 7, 11)]);
    assert_eq!(days("RU-KB", "Ураза-Байрам", 2017), [ymd(2017, 6, 26)]);
    assert_eq!(days("RU-KB", "Ураза-Байрам", 2020), [ymd(2020, 5, 25)]);
    assert_eq!(days("RU-KB", "Курбан-Байрам", 2021), [ymd(2021, 7, 20)]);
    assert_eq!(days("RU-KB", "Курбан-Байрам", 2025), [ymd(2025, 6, 6)]);
    assert_eq!(days("RU-KB", "Радоница", 2023), [ymd(2023, 4, 25)]);
    assert_eq!(days("RU-KB", "Радоница", 2025), [ymd(2025, 4, 29)]);
    assert!(has_gap("RU-KB", "Radonitsa", 2022));
    // Kalmykia: the titles of the Head's decrees give the day, e.g. Указ
    // № 10 of 14.02.2022 «Об объявлении 3 марта 2022 года ...»; 2017 for the Buddha's birthday and 2016 and 2018 for
    // Zul were not found.
    assert_eq!(days("RU-KL", "Цаган Сар", 2022), [ymd(2022, 3, 3)]);
    assert_eq!(
        days("RU-KL", "День рождения Будды Шакьямуни", 2016),
        [ymd(2016, 5, 21)]
    );
    assert_eq!(days("RU-KL", "Зул", 2014), [ymd(2014, 12, 16)]);
    assert_eq!(days("RU-KL", "Зул", 2023), [ymd(2023, 12, 7)]);
    assert!(has_gap("RU-KL", "Tsagan Sar", 2014));
    assert_eq!(days("RU-KL", "Цаган Сар", 2017), [ymd(2017, 2, 27)]);
    assert!(has_gap("RU-KL", "Buddha Shakyamuni's Birthday", 2017));
    assert!(has_gap("RU-KL", "Zul", 2016));
    assert!(has_gap("RU-KL", "Zul", 2018));
    assert!(!has_gap("RU-KL", "Zul", 2017));
    // The Altai Republic: Указ № 20-у of 22.01.2018 sets 17 February and
    // Garant's note of № 21-у of 20.01.2014 gives 2 February.
    assert_eq!(days("RU-AL", "Чага-Байрам", 2018), [ymd(2018, 2, 17)]);
    assert_eq!(days("RU-AL", "Чага-Байрам", 2014), [ymd(2014, 2, 2)]);
    assert!(has_gap("RU-AL", "Chaga Bayram", 2013));
    assert!(has_gap("RU-AL", "Chaga Bayram", 2016));
    assert!(has_gap("RU-AL", "Chaga Bayram", 2023));
    // North Ossetia–Alania: 17 November 2025 by Указ № 453 and 24 November
    // by the law, as the list gives them.
    assert_eq!(
        days("RU-SE", "Уастырджи (Джеоргуыба)", 2025),
        [ymd(2025, 11, 17), ymd(2025, 11, 24)]
    );
    assert!(has_gap("RU-SE", "Uastyrdzhi", 2023));
}

/// The North Caucasus, Tuva and Kalmykia before 2024: the texts, or the
/// governments' own announcements of them, that were read.
#[test]
fn the_other_republics_acts_before_2024_give_their_dates() {
    let span = |year, month, first: u8, last: u8| -> Vec<Rd> {
        (first..=last).map(|day| ymd(year, month, day)).collect()
    };
    // Chechnya: Указ № 146 of 29.08.2017 (31 August, 1 and 2 September),
    // № 93 of 30.05.2018 (15 to 18 June) and № 53 of 08.04.2023.
    assert_eq!(
        days("RU-CE", "Курбан-Байрам", 2017),
        [ymd(2017, 8, 31), ymd(2017, 9, 1), ymd(2017, 9, 2)]
    );
    assert_eq!(days("RU-CE", "Ураза-Байрам", 2018), span(2018, 6, 15, 18));
    assert_eq!(days("RU-CE", "Ураза-Байрам", 2020), span(2020, 5, 23, 26));
    assert_eq!(days("RU-CE", "Ураза-Байрам", 2023), span(2023, 4, 20, 22));
    assert_eq!(days("RU-CE", "Курбан-Байрам", 2016), span(2016, 9, 12, 14));
    assert!(has_gap("RU-CE", "Kurban Bayram", 2019));
    assert!(has_gap("RU-CE", "Uraza Bayram", 2015));
    // Dagestan: Постановление № 215 of 15.07.2015 declares Saturday 18 July
    // and the day off moves to Monday 20 July; № 180 of 14.07.2021 and
    // № 216 of 06.07.2022 do the same for Kurban Bayram; № 127 of 12.09.2018
    // for the Day of Unity (Saturday 15 September, Monday 17).
    assert_eq!(days("RU-DA", "Ураза-Байрам", 2015), [ymd(2015, 7, 18)]);
    assert_eq!(
        HolidayCalendar::for_year(&RUSSIA, Some("RU-DA"), 2015)
            .all()
            .iter()
            .filter(|holiday| holiday.name == "Day off transferred by the Government of Dagestan")
            .map(|holiday| holiday.date)
            .collect::<Vec<_>>(),
        [ymd(2015, 7, 20)]
    );
    assert!(!has_gap("RU-DA", MOVED, 2015));
    assert_eq!(days("RU-DA", "Курбан-Байрам", 2022), [ymd(2022, 7, 9)]);
    assert!(!has_gap("RU-DA", MOVED, 2022));
    assert_eq!(
        days("RU-DA", "День единства народов Дагестана", 2018),
        [ymd(2018, 9, 15)]
    );
    assert_eq!(
        days("RU-DA", "День единства народов Дагестана", 2024),
        [ymd(2024, 9, 15)]
    );
    assert_eq!(days("RU-DA", "Ураза-Байрам", 2019), span(2019, 6, 5, 7));
    assert_eq!(days("RU-DA", "Ураза-Байрам", 2023), span(2023, 4, 20, 21));
    assert!(has_gap("RU-DA", "Uraza Bayram", 2021));
    assert!(has_gap("RU-DA", "Kurban Bayram", 2023));
    // Ingushetia: the Head's decrees: 24 September 2015 (No. 194), 26 and
    // 27 June 2017, 11 July 2022, and 20 to 22 April 2023.
    assert_eq!(days("RU-IN", "Гӏурба", 2015), [ymd(2015, 9, 24)]);
    assert_eq!(days("RU-IN", "Мархаш", 2017), span(2017, 6, 26, 27));
    assert_eq!(days("RU-IN", "Мархаш", 2016), span(2016, 7, 6, 8));
    assert_eq!(days("RU-IN", "Гӏурба", 2022), [ymd(2022, 7, 11)]);
    assert_eq!(days("RU-IN", "Мархаш", 2023), span(2023, 4, 20, 22));
    assert!(has_gap("RU-IN", "Eid al-Fitr", 2019));
    // Tuva: ПВХ-III № 1742 of 14.12.2022 sets Shagaa on 21 February 2023;
    // № 2034 ПВХ-II sets 5 February 2019 and moves Saturday 2 February to
    // Monday 4 February; Naadym 2018 is 14 and 15 July (Saturday and Sunday),
    // moved to Monday 16 and Tuesday 17 July (Постановление № 171).
    assert_eq!(days("RU-TY", "Шагаа", 2023), [ymd(2023, 2, 21)]);
    assert_eq!(days("RU-TY", "Шагаа", 2019), [ymd(2019, 2, 5)]);
    assert_eq!(days("RU-TY", "Наадым", 2018), span(2018, 7, 14, 15));
    assert_eq!(days("RU-TY", "Наадым", 2021), [ymd(2021, 9, 24)]);
    let tuva = HolidayCalendar::for_year(&RUSSIA, Some("RU-TY"), 2018);
    assert!(!tuva.is_business_day(ymd(2018, 7, 16)));
    assert!(!tuva.is_business_day(ymd(2018, 7, 17)));
    let tuva_2019 = HolidayCalendar::for_year(&RUSSIA, Some("RU-TY"), 2019);
    assert!(tuva_2019.is_business_day(ymd(2019, 2, 2)));
    assert!(!tuva_2019.is_business_day(ymd(2019, 2, 4)));
    assert!(has_gap("RU-TY", "Shagaa", 2018));
    assert!(has_gap("RU-TY", "Naadym", 2019));
    assert!(has_gap("RU-TY", "Naadym", 2023));
    // Constitution Day 2021 moved from Thursday 6 May to Friday 7 May.
    assert_eq!(days("RU-TY", "День Конституции", 2021), [ymd(2021, 5, 7)]);
    assert_eq!(days("RU-TY", "День Конституции", 2022), [ymd(2022, 5, 6)]);
    // 2018: 6 May was a Sunday, and no resolution read moves it.
    assert!(has_gap("RU-TY", MOVED, 2018));
    // Kalmykia: Указ № 37 of 17.02.2020 moves the day off from Monday 24
    // February to Tuesday 25 February.
    assert_eq!(days("RU-KL", "Цаган Сар", 2020), [ymd(2020, 2, 24)]);
    assert_eq!(transferred("RU-KL", 2020), [ymd(2020, 2, 25)]);
}
