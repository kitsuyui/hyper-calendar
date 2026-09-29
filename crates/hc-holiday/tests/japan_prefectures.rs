//! Japan's prefectural days, ordinance by ordinance.
//!
//! Each row below is a day a prefecture set for itself, with the date its
//! ordinance gives, the first year the ordinance was in force on that
//! date, and the kind its instruments make it. The rows are the survey in
//! `docs/systems/japan-holidays.md`, not dates this crate produced.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::JAPAN;
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::Kind;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// A prefecture's day: its region, its local name, its date, the first
/// year it was kept and its kind today.
struct Day {
    region: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    first: i64,
    kind: Kind,
}

const fn day(
    region: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    first: i64,
    kind: Kind,
) -> Day {
    Day {
        region,
        local_name,
        month,
        day,
        first,
        kind,
    }
}

/// Every prefectural day the table carries. The first year is the
/// ordinance's first date in force: 施行 before the day that year, or the
/// next year when it came after (栃木 1985-09-30, 福井 1982-03-23 against
/// 7 February, 愛知 2022-12-23, 鹿児島 2018-12-25). Okinawa's row is the
/// 県の休日 of the 休日条例, in force from 26 May 1991.
const DAYS: &[Day] = &[
    day("JP-01", "北海道みんなの日", 7, 17, 2017, Kind::Observance),
    day("JP-07", "福島県民の日", 8, 21, 1997, Kind::Observance),
    day("JP-08", "県民の日", 11, 13, 1968, Kind::School),
    day("JP-09", "県民の日", 6, 15, 1986, Kind::Observance),
    day("JP-10", "群馬県民の日", 10, 28, 1985, Kind::School),
    day("JP-11", "県民の日", 11, 14, 1971, Kind::School),
    day("JP-12", "県民の日", 6, 15, 1984, Kind::School),
    day("JP-13", "都民の日", 10, 1, 1952, Kind::School),
    day("JP-16", "県民ふるさとの日", 5, 9, 2013, Kind::Observance),
    day("JP-18", "ふるさとの日", 2, 7, 1983, Kind::Observance),
    day("JP-19", "県民の日", 11, 20, 1986, Kind::School),
    day("JP-22", "県民の日", 8, 21, 1996, Kind::Observance),
    day("JP-23", "あいち県民の日", 11, 27, 2023, Kind::Observance),
    day("JP-24", "県民の日", 4, 18, 1976, Kind::Observance),
    day("JP-30", "ふるさと誕生日", 11, 22, 1989, Kind::Observance),
    day("JP-31", "とっとり県民の日", 9, 12, 1998, Kind::Observance),
    day("JP-37", "香川県民の日", 12, 3, 2026, Kind::School),
    day("JP-46", "県民の日", 7, 14, 2019, Kind::Observance),
    day("JP-47", "慰霊の日", 6, 23, 1991, Kind::Government),
];

/// The first year each 県民の日 that the prefectural schools close on is
/// `Kind::School`: the first year a school rule read shows the day among
/// its 休業日. Before it the day is the observance its ordinance makes it.
///
/// - 茨城: Article 8 as amended by 平成23年教委規則第8号, in force
///   1 July 2011.
/// - 群馬: the rule as amended by 平成26年教委規則第9号, in force
///   1 April 2014.
/// - 埼玉 and 千葉: the earliest copies read, of 19 January 2017 and
///   6 June 2016; the d1-law text shows no amendment dates.
/// - 東京: Article 5 as amended by 平成14年教委規則第19号, in force
///   1 April 2002; the rule itself is of 1960.
/// - 山梨: Article 3 as amended by 平成14年教委規則第2号, in force
///   1 April 2002.
/// - 香川: the 学則 as amended on 30 March 2026, the first version of it
///   that lists the day, which is also the day's first year.
const SCHOOL_FROM: &[(&str, i64)] = &[
    ("JP-08", 2011),
    ("JP-10", 2014),
    ("JP-11", 2017),
    ("JP-12", 2016),
    ("JP-13", 2002),
    ("JP-19", 2002),
    ("JP-37", 2026),
];

/// The kind a row's day has in `year`.
fn kind_in(row: &Day, year: i64) -> Kind {
    match SCHOOL_FROM.iter().find(|(region, _)| *region == row.region) {
        Some(&(_, from)) if year < from => Kind::Observance,
        _ => row.kind,
    }
}

/// The entries a region's calendar has on a day that are the region's
/// own, not nationwide.
fn own_entries(region: Option<&str>, year: i64, month: u8, day: u8) -> Vec<Holiday> {
    HolidayCalendar::for_year(&JAPAN, region, year)
        .on(ymd(year, month, day))
        .into_iter()
        .filter(|holiday| !holiday.regions.is_empty())
        .collect()
}

#[test]
fn every_prefectural_day_is_kept_from_its_first_year_to_today() {
    for row in DAYS {
        for year in [row.first, 2026] {
            let found = own_entries(Some(row.region), year, row.month, row.day);
            assert_eq!(found.len(), 1, "{} {year}: {found:?}", row.region);
            let entry = found[0];
            assert_eq!(entry.local_name, row.local_name, "{} {year}", row.region);
            assert_eq!(entry.kind, kind_in(row, year), "{} {year}", row.region);
            assert_eq!(entry.regions, [row.region]);
            assert!(!entry.source.is_empty(), "{} cites nothing", row.region);
        }
    }
}

#[test]
fn a_prefectural_day_is_a_school_closure_only_from_the_school_rule_read() {
    for &(region, from) in SCHOOL_FROM {
        let row = DAYS
            .iter()
            .find(|row| row.region == region)
            .unwrap_or_else(|| panic!("{region} has no day"));
        assert_eq!(row.kind, Kind::School, "{region}");
        let at = own_entries(Some(region), from, row.month, row.day);
        assert_eq!(at.len(), 1, "{region} {from}");
        assert_eq!(at[0].kind, Kind::School, "{region} {from}");
        assert!(at[0].source.contains("休業日"), "{region} {from}");
        if from > row.first {
            let before = own_entries(Some(region), from - 1, row.month, row.day);
            assert_eq!(before.len(), 1, "{region} {}", from - 1);
            assert_eq!(before[0].kind, Kind::Observance, "{region} {}", from - 1);
            assert!(
                !before[0].source.contains("休業日"),
                "{region} {}",
                from - 1
            );
        }
    }
    // Tokyo: an observance in 1952 and 2001, a school closure from 2002.
    let tokyo = |year| own_entries(Some("JP-13"), year, 10, 1)[0].kind;
    assert_eq!(tokyo(1952), Kind::Observance);
    assert_eq!(tokyo(2001), Kind::Observance);
    assert_eq!(tokyo(2002), Kind::School);
    // Every row whose kind today is School has a switch year.
    for row in DAYS.iter().filter(|row| row.kind == Kind::School) {
        assert!(
            SCHOOL_FROM.iter().any(|(region, _)| *region == row.region),
            "{}",
            row.region
        );
    }
}

#[test]
fn no_prefectural_day_is_kept_the_year_before_its_ordinance() {
    for row in DAYS.iter().filter(|row| row.region != "JP-47") {
        let before = row.first - 1;
        let calendar = HolidayCalendar::for_year(&JAPAN, Some(row.region), before);
        assert!(
            own_entries(Some(row.region), before, row.month, row.day).is_empty(),
            "{} {before}",
            row.region
        );
        assert!(calendar.is_complete(), "{} {before}", row.region);
    }
}

#[test]
fn a_prefectural_day_belongs_to_its_prefecture_alone() {
    // 都民の日: Tokyo has it, and neither the nationwide calendar nor
    // Kanagawa, next door, does.
    assert_eq!(own_entries(Some("JP-13"), 2026, 10, 1).len(), 1);
    assert!(own_entries(None, 2026, 10, 1).is_empty());
    assert!(own_entries(Some("JP-14"), 2026, 10, 1).is_empty());
    let nationwide = HolidayCalendar::for_year(&JAPAN, None, 2026);
    assert!(nationwide.on(ymd(2026, 10, 1)).is_empty());
    // Chiba's and Tochigi's 県民の日 share 15 June, and each is its own.
    assert_eq!(
        own_entries(Some("JP-12"), 2026, 6, 15)[0].regions,
        ["JP-12"]
    );
    assert_eq!(
        own_entries(Some("JP-09"), 2026, 6, 15)[0].regions,
        ["JP-09"]
    );
    assert!(own_entries(Some("JP-11"), 2026, 6, 15).is_empty());
    // The region is matched as every identifier is, in either case.
    assert_eq!(own_entries(Some("jp-13"), 2026, 10, 1).len(), 1);
    assert_eq!(own_entries(Some(" JP-13 "), 2026, 10, 1).len(), 1);
}

#[test]
fn no_prefectural_day_is_a_day_off_for_business_days() {
    // A school closure and a prefecture's office closure are not public
    // holidays: 1 October 2026 is a Thursday and a business day in Tokyo,
    // 23 June 2026 a Tuesday and one in Okinawa.
    for (region, month, day) in [("JP-13", 10, 1), ("JP-47", 6, 23), ("JP-08", 11, 13)] {
        let calendar = HolidayCalendar::for_year(&JAPAN, Some(region), 2026);
        assert!(!calendar.is_holiday(ymd(2026, month, day)), "{region}");
        assert!(calendar.is_business_day(ymd(2026, month, day)), "{region}");
    }
    for row in DAYS {
        assert!(!row.kind.is_day_off(), "{}", row.region);
    }
}

#[test]
fn okinawa_s_memorial_day_is_a_gap_before_its_holiday_ordinance() {
    // 沖縄県慰霊の日を定める条例 came into force on 21 October 1974, so
    // 1974 has no 慰霊の日; for 1975–1990 no instrument read says what the
    // day was for the prefecture's offices, and each year is a gap.
    let before = HolidayCalendar::for_year(&JAPAN, Some("JP-47"), 1974);
    assert!(before.is_complete());
    assert!(own_entries(Some("JP-47"), 1974, 6, 23).is_empty());
    for year in [1975, 1990] {
        let calendar = HolidayCalendar::for_year(&JAPAN, Some("JP-47"), year);
        assert!(own_entries(Some("JP-47"), year, 6, 23).is_empty(), "{year}");
        let gaps: Vec<_> = calendar
            .gaps()
            .iter()
            .map(|gap| (gap.year, gap.local_name))
            .collect();
        assert_eq!(gaps, [(year, "慰霊の日")], "{year}");
    }
    // The nationwide calendar has neither the day nor the gap.
    assert!(HolidayCalendar::for_year(&JAPAN, None, 1980).is_complete());
    assert_eq!(
        own_entries(Some("JP-47"), 1991, 6, 23)[0].kind,
        Kind::Government
    );
}

#[test]
fn the_table_names_every_prefecture_with_a_day_of_its_own() {
    // The prefectures with a day carried, and Akita and Ehime, whose day's
    // instrument was not found and is a gap.
    let mut regions: Vec<&str> = DAYS
        .iter()
        .chain(OTHER_DAYS)
        .map(|row| row.region)
        .collect();
    regions.extend(["JP-05", "JP-06", "JP-13", "JP-38"]);
    regions.sort_unstable();
    regions.dedup();
    // The municipalities' codes are the other file's.
    let prefectures: Vec<&str> = JAPAN
        .regions()
        .into_iter()
        .filter(|code| hc_holiday::rule::region_parent(code).is_none())
        .collect();
    assert_eq!(prefectures, regions);
}

/// The prefectures' other days, each set by an ordinance or another
/// instrument of the prefecture's own, with the first year it was in force
/// on the day: 施行 before it, or the next year (竹島の日 2005-03-25 against
/// 22 February, みやぎ鎮魂の日 2013-04-01 against 11 March, 新潟 2022-12-27,
/// 飛騨・美濃じまんの日 2007-10-01). None closes anything.
const OTHER_DAYS: &[Day] = &[
    day("JP-32", "竹島の日", 2, 22, 2006, Kind::Observance),
    day("JP-22", "富士山の日", 2, 23, 2010, Kind::Observance),
    day("JP-19", "富士山の日", 2, 23, 2012, Kind::Observance),
    day("JP-13", "東京都平和の日", 3, 10, 1991, Kind::Observance),
    day(
        "JP-03",
        "東日本大震災津波を語り継ぐ日",
        3,
        11,
        2021,
        Kind::Observance,
    ),
    day("JP-04", "みやぎ鎮魂の日", 3, 11, 2014, Kind::Observance),
    day("JP-04", "みやぎ県民防災の日", 6, 12, 2009, Kind::Observance),
    day("JP-03", "平泉世界遺産の日", 6, 29, 2014, Kind::Observance),
    day("JP-25", "びわ湖の日", 7, 1, 1996, Kind::Observance),
    day(
        "JP-21",
        "飛騨・美濃じまんの日",
        8,
        21,
        2008,
        Kind::Observance,
    ),
    day("JP-47", "しまくとぅばの日", 9, 18, 2006, Kind::Observance),
    day("JP-47", "琉球歴史文化の日", 11, 1, 2021, Kind::Observance),
    // The education days, all on 1 November.
    day("JP-03", "いわて教育の日", 11, 1, 2005, Kind::Observance),
    day("JP-04", "みやぎ教育の日", 11, 1, 2005, Kind::Observance),
    day("JP-07", "ふくしま教育の日", 11, 1, 2003, Kind::Observance),
    day("JP-08", "いばらき教育の日", 11, 1, 2004, Kind::Observance),
    day("JP-11", "彩の国教育の日", 11, 1, 2003, Kind::Observance),
    day("JP-15", "新潟県教育の日", 11, 1, 2023, Kind::Observance),
    day("JP-17", "いしかわ教育の日", 11, 1, 2005, Kind::Observance),
    day("JP-25", "滋賀教育の日", 11, 1, 2006, Kind::Observance),
    day("JP-29", "奈良県教育の日", 11, 1, 2003, Kind::Observance),
    day("JP-32", "しまね教育の日", 11, 1, 2002, Kind::Observance),
    day("JP-33", "おかやま教育の日", 11, 1, 2001, Kind::Observance),
    day("JP-34", "ひろしま教育の日", 11, 1, 2001, Kind::Observance),
    day("JP-36", "とくしま教育の日", 11, 1, 2004, Kind::Observance),
    day("JP-44", "おおいた教育の日", 11, 1, 2005, Kind::Observance),
];

/// A region's own entries on a day with a local name.
fn named(region: &str, year: i64, month: u8, day: u8, name: &str) -> Vec<Holiday> {
    own_entries(Some(region), year, month, day)
        .into_iter()
        .filter(|holiday| holiday.local_name == name)
        .collect()
}

#[test]
fn every_other_prefectural_day_is_kept_from_its_first_year_and_not_before() {
    for row in OTHER_DAYS {
        for year in [row.first, 2026] {
            let found = named(row.region, year, row.month, row.day, row.local_name);
            assert_eq!(found.len(), 1, "{} {} {year}", row.region, row.local_name);
            assert_eq!(found[0].kind, row.kind);
            assert_eq!(found[0].regions, [row.region]);
            assert!(!found[0].source.is_empty());
            assert!(!found[0].kind.is_day_off());
        }
        let before = row.first - 1;
        assert!(
            named(row.region, before, row.month, row.day, row.local_name).is_empty(),
            "{} {} {before}",
            row.region,
            row.local_name
        );
    }
    // 竹島の日 is Shimane's alone, and 富士山の日 the two prefectures'.
    assert!(named("JP-31", 2026, 2, 22, "竹島の日").is_empty());
    assert!(own_entries(None, 2026, 2, 22).is_empty());
    assert!(named("JP-14", 2026, 2, 23, "富士山の日").is_empty());
}

#[test]
fn tokyo_s_and_yamagata_s_education_days_are_saturdays() {
    // 東京都教育の日, the first Saturday of November: 7 November 2026, and
    // 1 November 2025 was itself a Saturday.
    assert_eq!(named("JP-13", 2026, 11, 7, "東京都教育の日").len(), 1);
    assert_eq!(named("JP-13", 2025, 11, 1, "東京都教育の日").len(), 1);
    assert!(named("JP-13", 2003, 11, 1, "東京都教育の日").is_empty());
    // やまがた教育の日, the second Saturday, 14 November 2026; the 要綱's
    // year was not read, so 2025 is a gap.
    assert_eq!(named("JP-06", 2026, 11, 14, "やまがた教育の日").len(), 1);
    let gaps: Vec<_> = HolidayCalendar::for_year(&JAPAN, Some("JP-06"), 2025)
        .gaps()
        .iter()
        .map(|gap| gap.local_name)
        .collect();
    assert_eq!(gaps, ["やまがた教育の日"]);
}

#[test]
fn aichi_s_school_holiday_is_a_gap_in_every_year_from_2023() {
    let gaps = |year| {
        HolidayCalendar::for_year(&JAPAN, Some("JP-23"), year)
            .gaps()
            .iter()
            .map(|gap| gap.local_name)
            .collect::<Vec<_>>()
    };
    assert!(gaps(2022).is_empty());
    assert_eq!(gaps(2023), ["あいち県民の日学校ホリデー"]);
    assert_eq!(gaps(2026), ["あいち県民の日学校ホリデー"]);
}

#[test]
fn every_prefecture_is_read_and_two_days_are_gaps() {
    let gaps = |region| {
        HolidayCalendar::for_year(&JAPAN, Some(region), 2026)
            .gaps()
            .iter()
            .map(|gap| gap.local_name)
            .collect::<Vec<_>>()
    };
    // No ordinance found in Aomori or Osaka: complete, the nationwide days.
    for region in ["JP-02", "JP-27", "JP-45"] {
        assert!(gaps(region).is_empty(), "{region}");
    }
    assert_eq!(gaps("JP-05"), ["県の記念日"]);
    assert_eq!(gaps("JP-38"), ["県政発足記念日"]);
    // A code that is no prefecture's is a subdivision not read.
    let unknown = HolidayCalendar::for_year(&JAPAN, Some("JP-48"), 2026);
    assert_eq!(unknown.gaps().len(), 1);
    assert_eq!(unknown.gaps()[0].name, hc_holiday::rule::UNREAD_SUBDIVISION);
}
