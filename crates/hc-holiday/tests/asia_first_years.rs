//! The first year each Asian table answers for (ADR 0013): before the
//! earliest year its sources support, a year is a gap, never a list of days
//! with `exact` or `approximate` confidence.
//!
//! Each row names the country, the first year its sources support, one day
//! the first year must have, and the source that gives that year. The
//! sweep asks every table for years long before it (1500, 1700, 1900) and
//! for the year just before, and finds no day answered and a gap
//! reported. The first year itself must answer the named day.

use hc_calendars_solar::gregorian;
use hc_holiday::countries;
use hc_holiday::engine::HolidayCalendar;

struct First {
    code: &'static str,
    year: i64,
    /// A day the first year has: month, day, name.
    day: (u8, u8, &'static str),
    /// Why that year.
    source: &'static str,
}

const fn row(
    code: &'static str,
    year: i64,
    day: (u8, u8, &'static str),
    source: &'static str,
) -> First {
    First {
        code,
        year,
        day,
        source,
    }
}

static FIRSTS: &[First] = &[
    row(
        "TW",
        2012,
        (10, 10, "National Day"),
        "the 辦法 as of 25 September 2012 and the DGPA calendar of 2012",
    ),
    row(
        "KR",
        1949,
        (7, 17, "Constitution Day"),
        "the 1949 decree 제124호",
    ),
    row(
        "IN",
        2025,
        (1, 26, "Republic Day"),
        "the DoPT O.M. of 9 July 2024, for 2025",
    ),
    row(
        "TH",
        1992,
        (12, 5, "King Bhumibol's Birthday"),
        "the Bank of Thailand's lists from 1992",
    ),
    row(
        "VN",
        2021,
        (9, 2, "National Day"),
        "Labour Code 45/2019, in force 1 January 2021",
    ),
    row("ID", 2020, (8, 17, "Independence Day"), "the SKB for 2020"),
    row(
        "SG",
        2020,
        (8, 9, "National Day"),
        "MOM's public holidays for 2020",
    ),
    row(
        "MY",
        2020,
        (8, 31, "National Day"),
        "the Prime Minister's Department's list for 2020",
    ),
    row(
        "PH",
        2012,
        (6, 12, "Independence Day"),
        "Proclamation 295, for 2012",
    ),
    row(
        "NP",
        2024,
        (5, 1, "Labour Day"),
        "the notice for 2080 BS, which ends in April 2024",
    ),
    row(
        "LK",
        2023,
        (2, 4, "Independence Day"),
        "Gazette No. 2287/4, for 2023",
    ),
    row(
        "PK",
        2026,
        (3, 23, "Pakistan Day"),
        "the Cabinet Division's list for 2026",
    ),
    row(
        "BD",
        2025,
        (3, 26, "Independence and National Day"),
        "the notification for 2025",
    ),
    row(
        "MM",
        2026,
        (1, 4, "Independence Day"),
        "the list read in 2026",
    ),
    row(
        "HK",
        1998,
        (10, 1, "National Day"),
        "Cap. 149 as in force from 18 September 1998",
    ),
    row(
        "MO",
        2001,
        (10, 1, "National Day of the People's Republic of China"),
        "Executive Order 60/2000, in Boletim Oficial 40/2000",
    ),
    row(
        "AM",
        2022,
        (9, 21, "Independence Day"),
        "the law as it stands, whose New Year break changed in 2022",
    ),
    row(
        "AZ",
        2007,
        (5, 28, "Independence Day"),
        "Labour Code art. 105 as amended on 8 December 2006",
    ),
    row(
        "GE",
        2011,
        (5, 26, "Independence Day"),
        "the Labour Code of 17 December 2010",
    ),
    row(
        "KZ",
        2002,
        (12, 16, "Independence Day"),
        "the Law of 13 December 2001",
    ),
    row(
        "UZ",
        2024,
        (9, 1, "Independence Day"),
        "the Labour Code, in force from 30 April 2023",
    ),
    row(
        "KG",
        2005,
        (8, 31, "Independence Day"),
        "the Labour Code of 4 August 2004",
    ),
    row(
        "TJ",
        2012,
        (9, 9, "Independence Day"),
        "the Law on Holidays of 2 August 2011",
    ),
    row(
        "TM",
        2010,
        (12, 12, "Neutrality Day"),
        "the Labour Code of 18 April 2009",
    ),
    row(
        "MN",
        2004,
        (1, 1, "New Year's Day"),
        "the Law of 18 December 2003",
    ),
    row(
        "KH",
        2021,
        (11, 9, "Independence Day"),
        "the sub-decree for 2021",
    ),
    row(
        "LA",
        2018,
        (12, 2, "National Day"),
        "Decree No. 386 of 15 December 2017",
    ),
    row("BN", 2023, (2, 23, "National Day"), "the circular for 2023"),
    row(
        "TL",
        2006,
        (11, 28, "Proclamation of Independence Day"),
        "Law No. 10/2005 of 10 August 2005",
    ),
    row(
        "BT",
        2025,
        (12, 17, "National Day"),
        "the Ministry's list for 2025",
    ),
    row(
        "MV",
        2016,
        (7, 26, "Independence Day"),
        "the Monetary Authority's list for 2016",
    ),
    row(
        "AF",
        2023,
        (8, 15, "Victory Day"),
        "the first Gregorian year wholly under the Emirate's calendars",
    ),
    row(
        "KP",
        2020,
        (9, 9, "Day of the Foundation of the Republic"),
        "the 2020 wall calendar",
    ),
];

fn table(code: &str) -> &'static hc_holiday::rule::RuleSet {
    match countries::by_code(code) {
        Some(table) => table,
        None => panic!("{code} is not a registered country"),
    }
}

/// The years a row's table must not answer for: long before, and the one
/// before its first.
fn earlier_years(first: &First) -> Vec<i64> {
    let mut years = vec![1500, 1700, 1900];
    years.push(first.year - 1);
    years.retain(|&year| year < first.year);
    years
}

#[test]
fn a_year_before_a_tables_first_is_a_gap_and_answers_no_day() {
    for first in FIRSTS {
        // Myanmar's Deepavali notices of 2020 to 2025 are their own rules'
        // years, read before the list's, and Nepal's 2023 is a part year:
        // each has its own test below.
        let skip: &[i64] = match first.code {
            "MM" => &[2020, 2021, 2022, 2023, 2024, 2025],
            "NP" => &[2023],
            _ => &[],
        };
        for year in earlier_years(first) {
            if skip.contains(&year) {
                continue;
            }
            let calendar = HolidayCalendar::for_year(table(first.code), None, year);
            let days = calendar.in_year(year);
            assert!(
                days.is_empty(),
                "{} {year}: {} days answered before {} ({})",
                first.code,
                days.len(),
                first.year,
                first.source
            );
            assert!(
                !calendar.is_complete(),
                "{} {year}: no gap reported before {}",
                first.code,
                first.year
            );
        }
    }
}

#[test]
fn the_first_year_answers_its_named_day() {
    for first in FIRSTS {
        let (month, day, name) = first.day;
        let Ok(date) = gregorian::to_fixed(first.year, month, day) else {
            panic!("{} {}-{month}-{day} is not a date", first.code, first.year);
        };
        let calendar = HolidayCalendar::for_year(table(first.code), None, first.year);
        assert!(
            calendar.on(date).iter().any(|holiday| holiday.name == name),
            "{} {}-{month:02}-{day:02}: no {name} ({})",
            first.code,
            first.year,
            first.source
        );
    }
}

#[test]
fn nepal_answers_from_the_notice_for_2080_bs_and_a_gap_before_it() {
    // 2080 BS began on 14 April 2023: the days of 2023 from then are read,
    // the days of January to the middle of April are a gap, and 2022 has
    // nothing.
    let nepal = table("NP");
    let calendar = HolidayCalendar::for_year(nepal, None, 2023);
    let gap_names: Vec<&str> = calendar.gaps().iter().map(|gap| gap.name).collect();
    for name in [
        "Maghe Sankranti",
        "Prithvi Jayanti",
        "Maha Shivaratri",
        "Gyalpo Lhosar",
    ] {
        assert!(gap_names.contains(&name), "{name}");
    }
    let Ok(january) = gregorian::to_fixed(2023, 1, 15) else {
        panic!("date")
    };
    assert!(calendar.on(january).is_empty());
    // After the new year of 2080 BS the festivals and the civil days stand.
    for (month, day, name) in [(5, 5, "Buddha Jayanti"), (12, 25, "Christmas Day")] {
        let Ok(date) = gregorian::to_fixed(2023, month, day) else {
            panic!("date")
        };
        assert!(
            calendar.on(date).iter().any(|holiday| holiday.name == name),
            "{name}"
        );
    }
    assert!(
        HolidayCalendar::for_year(nepal, None, 2022)
            .in_year(2022)
            .is_empty()
    );
}

#[test]
fn myanmar_answers_its_deepavali_notices_from_2020_and_the_list_from_2026() {
    let myanmar = table("MM");
    for year in 2020..=2025 {
        let calendar = HolidayCalendar::for_year(myanmar, None, year);
        let names: Vec<&str> = calendar
            .in_year(year)
            .iter()
            .map(|holiday| holiday.name)
            .collect();
        assert!(
            names.iter().all(|name| name.starts_with("Deepavali")),
            "{year} {names:?}"
        );
        assert!(!names.is_empty(), "{year}");
        assert!(!calendar.is_complete(), "{year}");
    }
    let before = HolidayCalendar::for_year(myanmar, None, 2019);
    assert!(before.in_year(2019).is_empty());
}

#[test]
fn china_answers_its_statutory_days_from_1999_and_only_commemorations_before() {
    // The 放假办法's text of 1999 was not read, so 1998 and before are a
    // gap for the days off; the 1979 and 1985 decisions that made Arbor Day
    // and Teachers' Day are what the two observances rest on.
    let china = table("CN");
    for year in [1500, 1900, 1998] {
        let calendar = HolidayCalendar::for_year(china, None, year);
        assert!(
            calendar
                .in_year(year)
                .iter()
                .all(|holiday| !holiday.is_day_off()),
            "{year}"
        );
        assert!(!calendar.is_complete(), "{year}");
    }
}

#[test]
fn mongolia_dates_naadam_and_tsagaan_sar_from_the_amendments_that_gave_them() {
    let mongolia = table("MN");
    let on = |year: i64, month: u8, day: u8| {
        let Ok(date) = gregorian::to_fixed(year, month, day) else {
            panic!("date")
        };
        HolidayCalendar::for_year(mongolia, None, year)
            .on(date)
            .iter()
            .map(|holiday| holiday.name)
            .collect::<Vec<_>>()
    };
    // The sixth day, 10 July, was added by the law of 28 June 2022 (the
    // consolidated text's note to article 4.1.1; Ura.mn, 27 June 2022:
    // "the holiday goes from five days to six"); the five days of 11 to 15
    // July stand from the amendment of 1 July 2014.
    assert!(on(2021, 7, 10).is_empty());
    assert_eq!(on(2022, 7, 10), ["Naadam"]);
    assert_eq!(on(2021, 7, 11), ["Naadam"]);
    assert_eq!(on(2014, 7, 15), ["Naadam"]);
    // Before 2014 the days of Naadam are a gap, not five days.
    assert!(on(2013, 7, 12).is_empty());
    assert!(
        HolidayCalendar::for_year(mongolia, None, 2013)
            .gaps()
            .iter()
            .any(|gap| gap.name == "Naadam")
    );
}

#[test]
fn bhutans_thimphu_festivals_are_a_gap_before_the_lists_too() {
    for year in [1990, 2003, 2024] {
        let calendar = HolidayCalendar::for_year(table("BT"), Some("BT-15"), year);
        assert!(calendar.in_year(year).is_empty(), "{year}");
        assert!(
            calendar
                .gaps()
                .iter()
                .any(|gap| gap.name == "Thimphu Tshechu"),
            "{year}"
        );
    }
}
