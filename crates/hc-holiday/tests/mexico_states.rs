//! Mexico's states' own days of rest, law by law.
//!
//! Jalisco's is the only state law read; `docs/systems/mexico-holidays.md`
//! says what was searched for the others.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::MEXICO;
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::Kind;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn own_entries(region: Option<&str>, year: i64, month: u8, day: u8) -> Vec<Holiday> {
    HolidayCalendar::for_year(&MEXICO, region, year)
        .on(ymd(year, month, day))
        .into_iter()
        .filter(|holiday| !holiday.regions.is_empty())
        .collect()
}

/// Article 38's four days beyond the federal list.
const JALISCO: &[(u8, u8, &str)] = &[
    (5, 5, "5 de mayo"),
    (9, 28, "28 de septiembre"),
    (10, 12, "12 de octubre"),
    (11, 2, "2 de noviembre"),
];

#[test]
fn jalisco_keeps_its_four_days_from_the_text_read() {
    for &(month, day, local_name) in JALISCO {
        for year in [2007, 2026] {
            let found = own_entries(Some("MX-JAL"), year, month, day);
            assert_eq!(found.len(), 1, "{year}-{month}-{day}: {found:?}");
            assert_eq!(found[0].local_name, local_name);
            assert_eq!(found[0].kind, Kind::Government);
            assert_eq!(found[0].regions, ["MX-JAL"]);
            assert!(found[0].source.contains("artículo 38"));
        }
        // The text read is the article as reformed in December 2006, and
        // the law before it was not read: 2006 has no day, and a gap.
        assert!(own_entries(Some("MX-JAL"), 2006, month, day).is_empty());
        assert!(
            HolidayCalendar::for_year(&MEXICO, Some("MX-JAL"), 2006)
                .gaps()
                .iter()
                .any(|gap| gap.local_name == local_name)
        );
    }
}

#[test]
fn jalisco_s_days_are_its_own_and_not_a_day_off_for_business_days() {
    assert!(own_entries(None, 2026, 11, 2).is_empty());
    assert!(own_entries(Some("MX-CMX"), 2026, 11, 2).is_empty());
    // Monday 2 November 2026: the state's offices close, and it is still a
    // business day, since the federal list does not have it.
    let calendar = HolidayCalendar::for_year(&MEXICO, Some("MX-JAL"), 2026);
    assert!(calendar.is_business_day(ymd(2026, 11, 2)));
    assert!(calendar.is_complete());
    assert_eq!(MEXICO.regions(), ["MX-JAL"]);
}
