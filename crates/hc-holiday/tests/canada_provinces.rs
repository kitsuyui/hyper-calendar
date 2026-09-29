//! Canada's provinces and territories, statute by statute.
//!
//! Each row is a general holiday a province's or territory's law keeps
//! beyond the federal list, with the date its rule gives in its first
//! year and in 2026; `docs/systems/canada-holidays.md` lists what was read.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::CANADA;
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::Kind;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn own_in_year(region: &str, year: i64) -> Vec<Holiday> {
    HolidayCalendar::for_year(&CANADA, Some(region), year)
        .in_year(year)
        .into_iter()
        .filter(|holiday| !holiday.regions.is_empty() && !holiday.is_substitute())
        .collect()
}

/// A month and a day.
type MonthDay = (u8, u8);

/// `(region, name, first year, first date, date in 2026)`.
const DAYS: &[(&str, &str, i64, MonthDay, MonthDay)] = &[
    ("CA-AB", "Family Day", 1990, (2, 19), (2, 16)),
    ("CA-BC", "Family Day", 2013, (2, 11), (2, 16)),
    ("CA-MB", "Louis Riel Day", 2008, (2, 18), (2, 16)),
    ("CA-NB", "Family Day", 2018, (2, 19), (2, 16)),
    ("CA-NS", "Nova Scotia Heritage Day", 2015, (2, 16), (2, 16)),
    ("CA-ON", "Family Day", 2008, (2, 18), (2, 16)),
    ("CA-PE", "Islander Day", 2009, (2, 9), (2, 16)),
    ("CA-SK", "Family Day", 2007, (2, 19), (2, 16)),
    (
        "CA-NT",
        "National Indigenous Peoples Day",
        2001,
        (6, 21),
        (6, 21),
    ),
    (
        "CA-YT",
        "National Indigenous Peoples Day",
        2017,
        (6, 21),
        (6, 21),
    ),
    // Loi sur la fête nationale, 1978, c. 5: carried from 1979.
    ("CA-QC", "Saint-Jean-Baptiste Day", 1979, (6, 24), (6, 24)),
    // Décret 1322-2002: the Monday before 25 May, from 2003.
    ("CA-QC", "National Patriots' Day", 2003, (5, 19), (5, 18)),
    // The provinces and territories that added the federal day by their
    // own acts: Prince Edward Island and the Northwest Territories from
    // 2022, Yukon from 2023.
    (
        "CA-PE",
        "National Day for Truth and Reconciliation",
        2022,
        (9, 30),
        (9, 30),
    ),
    (
        "CA-NT",
        "National Day for Truth and Reconciliation",
        2022,
        (9, 30),
        (9, 30),
    ),
    (
        "CA-YT",
        "National Day for Truth and Reconciliation",
        2023,
        (9, 30),
        (9, 30),
    ),
    ("CA-NU", "Nunavut Day", 2001, (7, 9), (7, 9)),
    ("CA-BC", "British Columbia Day", 2026, (8, 3), (8, 3)),
    ("CA-NB", "New Brunswick Day", 2026, (8, 3), (8, 3)),
    ("CA-NT", "Civic Holiday", 2026, (8, 3), (8, 3)),
    ("CA-NU", "Civic Holiday", 2026, (8, 3), (8, 3)),
    ("CA-SK", "Saskatchewan Day", 2026, (8, 3), (8, 3)),
    ("CA-YT", "Discovery Day", 2026, (8, 17), (8, 17)),
];

#[test]
fn every_provincial_day_falls_where_its_law_puts_it() {
    for &(region, name, first, (month, day), (month_now, day_now)) in DAYS {
        for (year, month, day) in [(first, month, day), (2026, month_now, day_now)] {
            let found: Vec<Holiday> = own_in_year(region, year)
                .into_iter()
                .filter(|holiday| holiday.name == name)
                .collect();
            assert_eq!(found.len(), 1, "{region} {name} {year}: {found:?}");
            assert_eq!(found[0].date, ymd(year, month, day), "{region} {name}");
            assert_eq!(found[0].kind, Kind::Public, "{region} {name}");
            assert_eq!(found[0].regions, [region], "{region} {name}");
            assert!(!found[0].source.is_empty(), "{region} {name}");
        }
        assert!(
            own_in_year(region, first - 1)
                .iter()
                .all(|holiday| holiday.name != name),
            "{region} {name} {}",
            first - 1
        );
    }
}

#[test]
fn before_its_first_year_a_day_is_absent_if_a_source_sets_it_and_a_gap_if_not() {
    let gaps = |region, year| {
        HolidayCalendar::for_year(&CANADA, Some(region), year)
            .gaps()
            .iter()
            .map(|gap| gap.name)
            .collect::<Vec<_>>()
    };
    // Ontario's Family Day was added by 2007, c. 16, in force for 2008:
    // 2007 is absent. Quebec's Fête nationale is carried from 1979, the
    // first year of its Act of 1978, whose first 24 June the text does not
    // give: 1978 is a gap.
    assert!(!gaps("CA-ON", 2007).contains(&"Family Day"));
    assert!(gaps("CA-QC", 1978).contains(&"Saint-Jean-Baptiste Day"));
    assert!(!gaps("CA-QC", 1979).contains(&"Saint-Jean-Baptiste Day"));
    assert!(gaps("CA-YT", 2025).contains(&"Discovery Day"));
    assert!(HolidayCalendar::for_year(&CANADA, None, 2025).is_complete());
}

#[test]
fn british_columbia_moved_family_day_to_the_third_monday_in_2019() {
    let date = |year| {
        own_in_year("CA-BC", year)
            .into_iter()
            .find(|holiday| holiday.name == "Family Day")
            .map(|holiday| holiday.date)
    };
    assert_eq!(date(2018), Some(ymd(2018, 2, 12)));
    assert_eq!(date(2019), Some(ymd(2019, 2, 18)));
}

#[test]
fn ontario_and_manitoba_keep_no_august_holiday() {
    // The Civic Holiday is in neither Ontario's Employment Standards Act
    // nor Manitoba's Code; Terry Fox Day in Manitoba names the day only.
    for region in ["CA-ON", "CA-MB"] {
        let calendar = HolidayCalendar::for_year(&CANADA, Some(region), 2026);
        assert!(calendar.is_business_day(ymd(2026, 8, 3)), "{region}");
    }
    // New Brunswick had no Family Day before 2018.
    let calendar = HolidayCalendar::for_year(&CANADA, Some("CA-NB"), 2017);
    assert!(calendar.is_business_day(ymd(2017, 2, 20)));
}

#[test]
fn prince_edward_island_kept_islander_day_on_the_second_monday_in_2009_only() {
    let date = |year| {
        own_in_year("CA-PE", year)
            .into_iter()
            .find(|holiday| holiday.name == "Islander Day")
            .map(|holiday| holiday.date)
    };
    assert_eq!(date(2008), None);
    assert_eq!(date(2009), Some(ymd(2009, 2, 9)));
    assert_eq!(date(2010), Some(ymd(2010, 2, 15)));
    assert_eq!(date(2011), Some(ymd(2011, 2, 21)));
}

/// The names of the days off a calendar has in a year.
fn days_off(region: Option<&str>, year: i64) -> Vec<&'static str> {
    HolidayCalendar::for_year(&CANADA, region, year)
        .in_year(year)
        .into_iter()
        .filter(|holiday| holiday.is_day_off() && !holiday.is_substitute())
        .map(|holiday| holiday.name)
        .collect()
}

/// The names of a calendar's gaps in a year.
fn gap_names(region: &str, year: i64) -> Vec<&'static str> {
    HolidayCalendar::for_year(&CANADA, Some(region), year)
        .gaps()
        .iter()
        .map(|gap| gap.name)
        .collect()
}

const FEDERAL: [&str; 10] = [
    "New Year's Day",
    "Good Friday",
    "Victoria Day",
    "Canada Day",
    "Labour Day",
    "National Day for Truth and Reconciliation",
    "Thanksgiving",
    "Remembrance Day",
    "Christmas Day",
    "Boxing Day",
];

#[test]
fn asked_for_no_region_the_table_gives_the_federal_days() {
    let days = days_off(None, 2026);
    for name in FEDERAL {
        assert!(days.contains(&name), "{name}");
    }
    assert!(HolidayCalendar::for_year(&CANADA, None, 2026).is_complete());
}

#[test]
fn a_province_keeps_only_the_federal_days_its_text_read_lists() {
    // (region, the federal days its list of 2026 leaves out)
    let left_out: &[(&str, &[&str])] = &[
        (
            "CA-AB",
            &["National Day for Truth and Reconciliation", "Boxing Day"],
        ),
        ("CA-BC", &["Boxing Day"]),
        (
            "CA-MB",
            &[
                "Remembrance Day",
                "Boxing Day",
                "National Day for Truth and Reconciliation",
            ],
        ),
        (
            "CA-NL",
            &[
                "Victoria Day",
                "Canada Day",
                "National Day for Truth and Reconciliation",
                "Thanksgiving",
                "Boxing Day",
            ],
        ),
        (
            "CA-NB",
            &[
                "Victoria Day",
                "National Day for Truth and Reconciliation",
                "Thanksgiving",
                "Boxing Day",
            ],
        ),
        (
            "CA-NS",
            &[
                "Victoria Day",
                "National Day for Truth and Reconciliation",
                "Thanksgiving",
                "Remembrance Day",
                "Boxing Day",
            ],
        ),
        ("CA-PE", &["Victoria Day", "Thanksgiving", "Boxing Day"]),
        (
            "CA-QC",
            &[
                "Good Friday",
                "Victoria Day",
                "National Day for Truth and Reconciliation",
                "Remembrance Day",
                "Boxing Day",
            ],
        ),
        ("CA-NT", &["Boxing Day"]),
        ("CA-YT", &["Boxing Day"]),
        (
            "CA-NU",
            &["National Day for Truth and Reconciliation", "Boxing Day"],
        ),
        (
            "CA-ON",
            &[
                "Remembrance Day",
                "National Day for Truth and Reconciliation",
            ],
        ),
        (
            "CA-SK",
            &["National Day for Truth and Reconciliation", "Boxing Day"],
        ),
    ];
    for &(region, out) in left_out {
        let days = days_off(Some(region), 2026);
        for name in FEDERAL {
            assert_eq!(
                days.contains(&name),
                !out.contains(&name),
                "{region} {name}"
            );
        }
        // The text read is of 2026: the years before are a gap for each day
        // it leaves out, and the Truth and Reconciliation day, first kept in
        // 2021, is no gap in 2020.
        // Yukon's text read is in force from 2023, and Quebec's Monday before
        // 25 May is named from 2003: no gap for those in 2025.
        let gaps = gap_names(region, 2025);
        for name in out.iter().filter(|name| {
            **name != "Canada Day"
                && !(region == "CA-YT" && **name == "Boxing Day")
                && !(region == "CA-QC" && **name == "Victoria Day")
        }) {
            assert!(
                gaps.iter().any(|gap| gap.contains(name)),
                "{region} {name}: {gaps:?}"
            );
        }
        assert!(
            !gap_names(region, 2020).contains(&"National Day for Truth and Reconciliation"),
            "{region}"
        );
    }
    // Newfoundland and Labrador's 1 July is its Memorial Day; British
    // Columbia and Manitoba keep the day of 30 September under their own
    // lists.
    let nl = HolidayCalendar::for_year(&CANADA, Some("CA-NL"), 2026);
    let july = nl.on(ymd(2026, 7, 1));
    assert_eq!(july.len(), 1);
    assert_eq!(july[0].name, "Memorial Day");
    assert!(nl.is_holiday(ymd(2026, 11, 11)));
    for region in ["CA-BC", "CA-MB"] {
        let calendar = HolidayCalendar::for_year(&CANADA, Some(region), 2026);
        assert!(calendar.is_holiday(ymd(2026, 9, 30)), "{region}");
        assert!(
            gap_names(region, 2022)
                .iter()
                .any(|name| name.contains("Truth and Reconciliation"))
        );
        assert!(
            !gap_names(region, 2020)
                .iter()
                .any(|name| name.contains("Truth and Reconciliation"))
        );
    }
    // Ontario keeps Boxing Day.
    assert!(days_off(Some("CA-ON"), 2026).contains(&"Boxing Day"));
    // Quebec's article 60 keeps Victoria Day's Monday as the Journée
    // nationale des patriotes, Monday 18 May 2026, and gives Good Friday or
    // Easter Monday at the employer's choice: both observances, business
    // days in the table. Before 2003 the Monday is a gap.
    let quebec = HolidayCalendar::for_year(&CANADA, Some("CA-QC"), 2026);
    assert!(quebec.is_holiday(ymd(2026, 5, 18)));
    for day in [ymd(2026, 4, 3), ymd(2026, 4, 6)] {
        assert!(quebec.is_business_day(day));
        assert!(
            quebec
                .on(day)
                .iter()
                .any(|holiday| holiday.kind == Kind::Observance
                    && holiday.local_name
                        == "Vendredi saint ou lundi de Pâques, au choix de l'employeur")
        );
    }
    assert!(gap_names("CA-QC", 2002).contains(&"Victoria Day"));
    // Nova Scotia's Remembrance Day rests on an Act not read: a gap every
    // year, and no day off.
    assert!(gap_names("CA-NS", 2026).contains(&"Remembrance Day"));
    // The Northwest Territories kept the day of 30 September for the public
    // service alone in 2021, and Yukon's act came in November 2022.
    assert!(!HolidayCalendar::for_year(&CANADA, Some("CA-NT"), 2021).is_holiday(ymd(2021, 9, 30)));
    assert!(!HolidayCalendar::for_year(&CANADA, Some("CA-YT"), 2022).is_holiday(ymd(2022, 9, 30)));
}
