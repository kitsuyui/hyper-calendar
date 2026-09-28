//! India's states' holidays under the Negotiable Instruments Act.
//!
//! The rows are the Reserve Bank of India's lists for its regional
//! offices, "Holidays under Negotiable Instruments Act" for 2025 and 2026,
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

/// The thirteen states carried.
const STATES: &[&str] = &[
    "IN-UP", "IN-MH", "IN-BR", "IN-WB", "IN-MP", "IN-TN", "IN-RJ", "IN-KA", "IN-GJ", "IN-AP",
    "IN-OD", "IN-TS", "IN-KL",
];

#[test]
fn a_state_keeps_the_days_the_reserve_bank_lists_for_it() {
    for (region, year, month, day, name) in [
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
    ] {
        let found = own_on(Some(region), year, month, day);
        assert_eq!(found.len(), 1, "{region} {year}-{month}-{day}: {found:?}");
        assert_eq!(found[0].name, name);
        assert_eq!(found[0].kind, Kind::Bank);
        assert_eq!(found[0].regions, [region]);
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

#[test]
fn the_states_are_carried_for_2025_and_2026_and_are_a_gap_after() {
    for region in STATES {
        let before = HolidayCalendar::for_year(&INDIA, Some(region), 2024);
        assert!(
            before
                .in_year(2024)
                .iter()
                .all(|holiday| holiday.regions.is_empty()),
            "{region}"
        );
        for year in [2025, 2026] {
            let calendar = HolidayCalendar::for_year(&INDIA, Some(region), year);
            let own = calendar
                .in_year(year)
                .iter()
                .filter(|holiday| !holiday.regions.is_empty())
                .count();
            assert!(own >= 12, "{region} {year}: {own}");
            assert!(
                !calendar
                    .gaps()
                    .iter()
                    .any(|gap| gap.name == "Holidays under the Negotiable Instruments Act"),
                "{region} {year}"
            );
        }
        let after = HolidayCalendar::for_year(&INDIA, Some(region), 2027);
        let gaps: Vec<&str> = after.gaps().iter().map(|gap| gap.name).collect();
        assert!(
            gaps.contains(&"Holidays under the Negotiable Instruments Act"),
            "{region}: {gaps:?}"
        );
    }
    // The nationwide table has no such gap.
    let nationwide = HolidayCalendar::for_year(&INDIA, None, 2027);
    assert!(
        !nationwide
            .gaps()
            .iter()
            .any(|gap| gap.name == "Holidays under the Negotiable Instruments Act")
    );
}

#[test]
fn no_state_day_falls_on_a_sunday() {
    // The Reserve Bank lists no Sunday, when the banks are closed anyway.
    for region in STATES {
        for year in [2025, 2026] {
            let calendar = HolidayCalendar::for_year(&INDIA, Some(region), year);
            for holiday in calendar.in_year(year) {
                if !holiday.regions.is_empty() {
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
