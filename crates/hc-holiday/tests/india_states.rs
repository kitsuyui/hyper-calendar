//! India's states' holidays under the Negotiable Instruments Act.
//!
//! The rows are the Reserve Bank of India's lists for its regional
//! offices, "Holidays under Negotiable Instruments Act" for 2019 to 2026,
//! retrieved 2026-09-29, not dates this crate produced.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::INDIA;
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::Kind;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// A state's own entries on a day.
fn own_on(region: Option<&str>, year: i64, month: u8, day: u8) -> Vec<Holiday> {
    HolidayCalendar::for_year(&INDIA, region, year)
        .on(ymd(year, month, day))
        .into_iter()
        .filter(|holiday| !holiday.regions.is_empty())
        .collect()
}

/// The years from `first` to 2026 that a test over every state and year
/// reads: all of them, and in a build instrumented for coverage every third
/// and the last, 2026, each of which builds a state's whole table anew
/// (docs/policy.md §7).
fn listed_years(first: i64) -> impl Iterator<Item = i64> {
    (first..=2026).filter(move |year| {
        !hc_core::sweep::INSTRUMENTED || (year - first) % 3 == 0 || *year == 2026
    })
}

/// The twenty-eight states and union territories carried.
const STATES: &[&str] = &[
    "IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP",
    "IN-OD", "IN-TS", "IN-KL", "IN-AS", "IN-JH", "IN-DL", "IN-JK", "IN-UK", "IN-CG", "IN-HP",
    "IN-TR", "IN-ML", "IN-MN", "IN-NL", "IN-GA", "IN-AR", "IN-MZ", "IN-SK",
];

/// The states whose offices' lists begin in 2023: Vijayawada, Itanagar
/// and Kohima.
const FROM_2023: &[&str] = &["IN-AP", "IN-AR", "IN-NL"];

#[test]
fn a_state_keeps_the_days_the_reserve_bank_lists_for_it() {
    for (region, year, month, day, name) in [
        (
            "IN-MH",
            2019,
            2,
            19,
            "Chhatrapati Shivaji Maharaj Jayanti/Guru Ravidas’s Birthday",
        ),
        ("IN-MH", 2020, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti"),
        ("IN-WB", 2022, 1, 12, "Birthday of Swami Vivekananda"),
        (
            "IN-MH",
            2024,
            9,
            7,
            "Ganesh Chaturthi/Samvatsari (Chaturthi Paksha)/Varasiddhi Vinayaka Vrata/Vinayakar Chathurthi",
        ),
        ("IN-MH", 2025, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti"),
        ("IN-MH", 2026, 2, 19, "Chhatrapati Shivaji Maharaj Jayanti"),
        ("IN-TN", 2025, 1, 16, "Uzhavar Thirunal"),
        ("IN-TN", 2026, 1, 16, "Thiruvalluvar Day/Kanuma"),
        ("IN-WB", 2026, 1, 12, "Birth Day of Swami Vivekananda"),
        ("IN-KA", 2025, 11, 1, "Kannada Rajyothsava/Igas-Bagwal"),
        (
            "IN-BR",
            2026,
            11,
            16,
            "Chhath Puja/Surya Shashti Dala Chhath (Prath Arghya)",
        ),
        ("IN-MH", 2026, 5, 1, "Maharashtra Din/Buddha Pournima"),
        (
            "IN-AS",
            2022,
            4,
            15,
            "Bengali New Year’s Day (Nababarsha)/Himachal Day/Vishu/Bohag Bihu",
        ),
        ("IN-SK", 2026, 12, 10, "Losoong / Namsoong"),
        ("IN-GA", 2025, 12, 3, "Feast of St. Francis Xavier"),
        ("IN-MZ", 2025, 2, 20, "Statehood Day/State Day"),
        ("IN-JK", 2026, 3, 13, "Jumat-ul-Vida"),
        (
            "IN-NL",
            2025,
            12,
            1,
            "State Inauguration Day/Indigenous Faith Day",
        ),
    ] {
        let found = own_on(Some(region), year, month, day);
        assert_eq!(found.len(), 1, "{region} {year}-{month}-{day}: {found:?}");
        assert_eq!(found[0].name, name);
        assert_eq!(found[0].kind, Kind::Bank);
        assert!(found[0].regions.contains(&region), "{region}");
        assert!(found[0].source.contains("Reserve Bank of India"));
        let calendar = HolidayCalendar::for_year(&INDIA, Some(region), year);
        assert!(!calendar.is_business_day(ymd(year, month, day)), "{region}");
    }
}

#[test]
fn a_state_s_day_is_its_own() {
    // Shivaji Jayanti, a Thursday in 2026: a bank holiday in Maharashtra, a
    // business day in Uttar Pradesh and for the nationwide table.
    let day = ymd(2026, 2, 19);
    for region in [None, Some("IN-UP")] {
        let calendar = HolidayCalendar::for_year(&INDIA, region, 2026);
        assert!(calendar.is_business_day(day), "{region:?}");
        assert!(own_on(region, 2026, 2, 19).is_empty(), "{region:?}");
    }
    // A day at one of Maharashtra's three offices is not the state's:
    // Mumbai's Id-E-Milad of 8 September 2025.
    assert!(own_on(Some("IN-MH"), 2025, 9, 8).is_empty());
    // Nor is an election at a state's one office.
    assert!(own_on(Some("IN-TN"), 2026, 4, 23).is_empty());
}

/// The gap every state carried reports for a year whose list is not read.
const GAP: &str = "Holidays under the Negotiable Instruments Act";

#[test]
fn the_states_are_carried_for_2019_to_2026_and_are_a_gap_before_and_after() {
    for region in STATES {
        let first = if FROM_2023.contains(region) {
            2023
        } else {
            2019
        };
        let before = HolidayCalendar::for_year(&INDIA, Some(region), first - 1);
        assert!(
            before
                .in_year(first - 1)
                .iter()
                .all(|holiday| holiday.regions.is_empty()),
            "{region}"
        );
        let gaps: Vec<&str> = before.gaps().iter().map(|gap| gap.name).collect();
        assert!(gaps.contains(&GAP), "{region} {}: {gaps:?}", first - 1);
        for year in listed_years(first) {
            let calendar = HolidayCalendar::for_year(&INDIA, Some(region), year);
            let own = calendar
                .in_year(year)
                .iter()
                .filter(|holiday| !holiday.regions.is_empty())
                .count();
            // Delhi's 2022 list has twelve days, some of them nationwide
            // ones of the same name.
            assert!(own >= 9, "{region} {year}: {own}");
            assert!(
                !calendar.gaps().iter().any(|gap| gap.name == GAP),
                "{region} {year}"
            );
        }
        let after = HolidayCalendar::for_year(&INDIA, Some(region), 2027);
        let gaps: Vec<&str> = after.gaps().iter().map(|gap| gap.name).collect();
        assert!(gaps.contains(&GAP), "{region}: {gaps:?}");
    }
    // The nationwide table has no such gap.
    for year in [2018, 2027] {
        let nationwide = HolidayCalendar::for_year(&INDIA, None, year);
        assert!(
            !nationwide.gaps().iter().any(|gap| gap.name == GAP),
            "{year}"
        );
    }
}

#[test]
fn no_state_day_falls_on_a_sunday_but_the_one_the_list_has() {
    // The Reserve Bank lists no Sunday, when the banks are closed anyway,
    // but for Shillong's Beh Dienkhlam of 14 July 2019.
    let listed = ymd(2019, 7, 14);
    assert_eq!(
        own_on(Some("IN-ML"), 2019, 7, 14)
            .iter()
            .map(|holiday| holiday.name)
            .collect::<Vec<_>>(),
        ["Beh Dienkhlam"]
    );
    for region in STATES {
        for year in listed_years(2019) {
            let calendar = HolidayCalendar::for_year(&INDIA, Some(region), year);
            for holiday in calendar.in_year(year) {
                if !holiday.regions.is_empty() && holiday.date != listed {
                    assert_ne!(
                        hc_calendar::Weekday::from_rd(holiday.date),
                        hc_calendar::Weekday::Sunday,
                        "{region} {:?}",
                        holiday.name
                    );
                }
            }
        }
    }
}

#[test]
fn a_name_keeps_the_parts_the_list_does_not_show_another_state_s() {
    // One description for all the offices, split at its slashes: the
    // parts the list shows a state not keeping are not its name.
    let name = |region: &str, year: i64, month: u8, day: u8| -> String {
        let found = own_on(Some(region), year, month, day);
        assert_eq!(found.len(), 1, "{region} {year}-{month}-{day}: {found:?}");
        found[0].name.to_string()
    };
    // Guwahati lists no Good Friday but 2022's, whose description Assam's
    // Bohag Bihu shares.
    assert!(!name("IN-AS", 2022, 4, 15).contains("Good Friday"));
    assert!(name("IN-MP", 2022, 4, 15).starts_with("Good Friday/"));
    // Maharashtra's own list for 2026 names Maharashtra Din and Buddha
    // Pournima on 1 May, not the other states' May Day and Raghunath Murmu.
    assert!(!name("IN-MH", 2026, 5, 1).contains("Murmu"));
    // A part the list does not show as another's stays: Karnataka's
    // Kannada Rajyothsava shares its description with Uttarakhand's
    // Igas-Bagwal, and so does the rule.
    let found = own_on(Some("IN-UK"), 2025, 11, 1);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].name, "Kannada Rajyothsava/Igas-Bagwal");
    assert!(
        found[0].regions.contains(&"IN-KA"),
        "{:?}",
        found[0].regions
    );
}
