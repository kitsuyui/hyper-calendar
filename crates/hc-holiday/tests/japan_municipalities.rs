//! Japan's municipal days, city by city (ADR 0014).
//!
//! Each row below is a day a city set by an instrument of its own, scoped
//! to the city's code under its prefecture's, with the date the
//! instrument gives, the first year it was in force on that date and the
//! kind the instruments make it. The rows are the survey of the twenty
//! 政令指定都市 in `docs/systems/japan-holidays.md`, not dates this crate
//! produced.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::JAPAN;
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::{Kind, UNREAD_SUBDIVISION, region_parent};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// A city's day: its code, its local name, its date, the first year the
/// instrument read answers for, whether the day is known to begin then
/// (`true`) or the years before are a gap (`false`), and its kind in that
/// first year and today.
struct Day {
    region: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    first: i64,
    established: bool,
    kind_first: Kind,
    kind_now: Kind,
}

#[allow(clippy::too_many_arguments)]
const fn day(
    region: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    first: i64,
    established: bool,
    kind_first: Kind,
    kind_now: Kind,
) -> Day {
    Day {
        region,
        local_name,
        month,
        day,
        first,
        established,
        kind_first,
        kind_now,
    }
}

use Kind::{Government, Observance, School};

/// Every municipal day on a fixed date the table carries. The first year:
/// さいたま 2021-03-11 before 1 May; 千葉 the 告示 of 1995-12-18, after
/// 18 October; 川崎 the 告示 of 1937-06-25; 静岡 the 告示 of 2010-03-20;
/// 名古屋 2024-04-01; 京都 憲章の日 in force 2011-04-01, after 5 February,
/// and 食の安全安心推進の日's article from 2010-10-01, after 1 August; the
/// 京都市自治記念日 公告 of 1958-09-03; 神戸 1998-01-17, the day itself;
/// 岡山 the mayor's decision of 2012-03-22; 広島 1947-07-31; 長崎
/// 1995-03-23; 熊本 地震の日 in force 2022-10-01, after 16 April, and
/// 市民健康の日 1986-04-01. 横浜, 浜松 and 堺 are read from the
/// first year of their instruments read, and the years before are gaps.
const DAYS: &[Day] = &[
    day(
        "JP-11-100",
        "さいたま市民の日",
        5,
        1,
        2021,
        true,
        School,
        School,
    ),
    day(
        "JP-12-100",
        "千葉市の市民の日",
        10,
        18,
        1996,
        true,
        Observance,
        Observance,
    ),
    day("JP-14-100", "開港記念日", 6, 2, 2021, false, School, School),
    day(
        "JP-14-130",
        "市制記念日",
        7,
        1,
        1937,
        true,
        Observance,
        School,
    ),
    day(
        "JP-22-100",
        "お茶の日",
        11,
        1,
        2010,
        true,
        Observance,
        Observance,
    ),
    day(
        "JP-22-130",
        "市制記念日",
        7,
        1,
        1997,
        false,
        Observance,
        Observance,
    ),
    day(
        "JP-23-100",
        "なごや平和の日",
        5,
        14,
        2024,
        true,
        Observance,
        Observance,
    ),
    day(
        "JP-26-100",
        "憲章の日",
        2,
        5,
        2012,
        true,
        Observance,
        Observance,
    ),
    day(
        "JP-26-100",
        "食の安全安心推進の日",
        8,
        1,
        2011,
        true,
        Observance,
        Observance,
    ),
    day(
        "JP-26-100",
        "京都市自治記念日",
        10,
        15,
        1958,
        true,
        Observance,
        Observance,
    ),
    day(
        "JP-27-140",
        "開庁記念日",
        7,
        26,
        1971,
        false,
        Observance,
        Observance,
    ),
    day(
        "JP-28-100",
        "市民防災の日",
        1,
        17,
        1998,
        true,
        Observance,
        Observance,
    ),
    day(
        "JP-33-100",
        "岡山市民の日",
        6,
        1,
        2012,
        true,
        Observance,
        Observance,
    ),
    day(
        "JP-34-100",
        "平和記念日",
        8,
        6,
        1947,
        true,
        Government,
        Government,
    ),
    day(
        "JP-42-201",
        "ながさき平和の日",
        8,
        9,
        1995,
        true,
        Observance,
        Observance,
    ),
    day(
        "JP-43-100",
        "熊本地震の日",
        4,
        16,
        2023,
        true,
        Observance,
        Observance,
    ),
    day(
        "JP-43-100",
        "市民健康の日",
        10,
        1,
        1986,
        true,
        Observance,
        Observance,
    ),
];

/// The designated cities read that set no day: the table answers for them
/// with their prefecture's days and no gap.
const READ_WITHOUT_A_DAY: &[&str] = &[
    "JP-01-100",
    "JP-04-100",
    "JP-15-100",
    "JP-27-100",
    "JP-40-100",
    "JP-40-130",
];

/// A city's own entries on a day: the ones its prefecture's calendar lacks.
fn own(region: &str, year: i64, month: u8, day: u8) -> Vec<Holiday> {
    let parent = region_parent(region);
    let above = HolidayCalendar::for_year(&JAPAN, parent, year);
    let above = above.on(ymd(year, month, day));
    HolidayCalendar::for_year(&JAPAN, Some(region), year)
        .on(ymd(year, month, day))
        .into_iter()
        .filter(|holiday| !above.contains(holiday))
        .collect()
}

fn gap_names(region: &str, year: i64) -> Vec<&'static str> {
    HolidayCalendar::for_year(&JAPAN, Some(region), year)
        .holiday_gaps()
        .map(|gap| gap.local_name)
        .collect()
}

#[test]
fn every_municipal_day_is_kept_from_its_first_year_to_today() {
    for row in DAYS {
        for (year, kind) in [(row.first, row.kind_first), (2026, row.kind_now)] {
            let found: Vec<_> = own(row.region, year, row.month, row.day)
                .into_iter()
                .filter(|holiday| holiday.local_name == row.local_name)
                .collect();
            assert_eq!(found.len(), 1, "{} {} {year}", row.region, row.local_name);
            assert_eq!(found[0].kind, kind, "{} {year}", row.region);
            assert_eq!(found[0].regions, [row.region]);
            assert!(!found[0].source.is_empty());
            assert!(!found[0].kind.is_day_off());
        }
        let before = row.first - 1;
        assert!(
            own(row.region, before, row.month, row.day).is_empty(),
            "{} {before}",
            row.region
        );
        let gaps = gap_names(row.region, before);
        if row.established {
            assert!(!gaps.contains(&row.local_name), "{} {before}", row.region);
        } else {
            assert!(gaps.contains(&row.local_name), "{} {before}", row.region);
        }
    }
}

#[test]
fn a_city_has_its_prefecture_s_days_and_its_own() {
    // さいたま市 on 14 November 2026: Saitama's 県民の日, the prefecture's
    // own entry, and on 1 May the city's.
    let saitama = HolidayCalendar::for_year(&JAPAN, Some("JP-11-100"), 2026);
    let names = |day| {
        saitama
            .on(day)
            .into_iter()
            .map(|holiday| (holiday.local_name, holiday.regions))
            .collect::<Vec<_>>()
    };
    assert_eq!(names(ymd(2026, 11, 14)), [("県民の日", &["JP-11"][..])]);
    assert_eq!(
        names(ymd(2026, 5, 1)),
        [("さいたま市民の日", &["JP-11-100"][..])]
    );
    // The prefecture asked for alone has none of the city's days.
    let prefecture = HolidayCalendar::for_year(&JAPAN, Some("JP-11"), 2026);
    assert!(prefecture.on(ymd(2026, 5, 1)).is_empty());
    // Nor does a neighbouring city: 開港記念日 is Yokohama's, not
    // Kawasaki's.
    assert_eq!(own("JP-14-100", 2026, 6, 2).len(), 1);
    assert!(own("JP-14-130", 2026, 6, 2).is_empty());
    // The code is matched in either case.
    assert_eq!(own("jp-14-100", 2026, 6, 2).len(), 1);
}

#[test]
fn kyoto_s_traditional_industries_day_is_the_vernal_equinox() {
    // 伝統産業の日 is 春分の日: 20 March 2026 and 21 March 2006.
    for (year, day) in [(2006, 21), (2026, 20)] {
        let found = own("JP-26-100", year, 3, day);
        assert_eq!(found.len(), 1, "{year}");
        assert_eq!(found[0].local_name, "伝統産業の日");
    }
    assert!(own("JP-26-100", 2005, 3, 20).is_empty());
}

#[test]
fn hiroshima_s_peace_memorial_day_closes_the_city_s_offices_alone() {
    // 6 August 2026 is a Thursday: a business day in Hiroshima as
    // everywhere, though the city's offices close.
    let calendar = HolidayCalendar::for_year(&JAPAN, Some("JP-34-100"), 2026);
    assert!(calendar.is_business_day(ymd(2026, 8, 6)));
    assert_eq!(own("JP-34-100", 2026, 8, 6)[0].kind, Kind::Government);
    assert!(own("JP-34", 2026, 8, 6).is_empty());
}

#[test]
fn sagamihara_s_foundation_day_is_a_gap_in_every_year() {
    // 相模原市表彰条例 gives awards on the 市制施行記念日 and no date.
    for year in [1989, 2026] {
        assert_eq!(gap_names("JP-14-150", year), ["市制施行記念日"], "{year}");
    }
    // Before Kanagawa's 休日条例 of 1989 the prefecture is a gap too, and
    // the city's with it.
    assert_eq!(gap_names("JP-14-150", 1960), ["", "市制施行記念日"]);
}

#[test]
fn a_city_read_for_no_day_is_a_gap_before_its_holiday_ordinance() {
    // Each city's 休日条例, as the 条例Webアーカイブ's copy gives it: 札幌市
    // 1990-06-15, 仙台市 1989-09-22, 新潟市 1989-10-09, 大阪市 1991-12-24 (in
    // force 1992-04-01), 北九州市 1991-03-25, 福岡市 1990-12-22. The regime
    // before it was not read, and a city's days are no better known than
    // its prefecture's.
    for (region, first) in [
        ("JP-01-100", 1990),
        ("JP-04-100", 1989),
        ("JP-15-100", 1989),
        ("JP-27-100", 1992),
        ("JP-40-100", 1991),
        ("JP-40-130", 1990),
    ] {
        assert!(!gap_names(region, first - 1).is_empty(), "{region}");
        assert!(gap_names(region, first).is_empty(), "{region} {first}");
        assert!(gap_names(region, 2026).is_empty(), "{region}");
    }
}

#[test]
fn a_city_read_is_answered_and_a_city_not_read_is_a_gap() {
    for &region in READ_WITHOUT_A_DAY {
        assert!(gap_names(region, 2026).is_empty(), "{region}");
    }
    for row in DAYS {
        assert!(JAPAN.reads_region(row.region), "{}", row.region);
    }
    // 鎌倉市, 14204, was not read: Kanagawa's days, and a gap for its own.
    assert_eq!(
        HolidayCalendar::for_year(&JAPAN, Some("JP-14-204"), 2026)
            .holiday_gaps()
            .map(|gap| gap.name)
            .collect::<Vec<_>>(),
        [UNREAD_SUBDIVISION]
    );
    // Every municipal code the table names is under a prefecture's.
    for code in JAPAN.regions() {
        if let Some(parent) = region_parent(code) {
            assert!(parent.starts_with("JP-") && parent.len() == 5, "{code}");
            assert_eq!(code.len(), 9, "{code}");
        }
    }
}

#[test]
fn yokohama_s_port_opening_day_is_the_worked_example() {
    // 2 June 2026, a Tuesday: the city's schools close, nothing else does.
    let june_2 = ymd(2026, 6, 2);
    let yokohama = HolidayCalendar::for_year(&JAPAN, Some("JP-14-100"), 2026);
    let entries = yokohama.on(june_2);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].local_name, "開港記念日");
    assert_eq!(entries[0].kind, Kind::School);
    assert!(entries[0].source.contains("令和7年横浜市条例第21号"));
    assert!(yokohama.is_business_day(june_2));
    for region in ["JP-14", "JP-14-130"] {
        let calendar = HolidayCalendar::for_year(&JAPAN, Some(region), 2026);
        assert!(calendar.on(june_2).is_empty(), "{region}");
        assert!(calendar.is_business_day(june_2), "{region}");
    }
    // 2020 is before the school rule's text read: a gap.
    assert!(own("JP-14-100", 2020, 6, 2).is_empty());
    assert_eq!(gap_names("JP-14-100", 2020), ["開港記念日"]);
}
