//! The international observances, against the United Nations' own list.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::international::{self, UNITED_NATIONS};
use hc_holiday::rule::{Confidence, Kind};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// Assert that `name` is observed on the given date.
fn expect(days: &[(i64, u8, u8, &str)]) {
    for (year, month, day, name) in days {
        let calendar = HolidayCalendar::for_year(&UNITED_NATIONS, None, *year);
        let found: Vec<&'static str> = calendar
            .on(ymd(*year, *month, *day))
            .iter()
            .map(|holiday| holiday.name)
            .collect();
        assert!(
            found.contains(name),
            "{year}-{month:02}-{day:02}: expected {name}, found {found:?}"
        );
    }
}

#[test]
fn the_fixed_days_fall_on_their_dates() {
    expect(&[
        (2025, 1, 4, "World Braille Day"),
        (2025, 3, 8, "International Women's Day"),
        (2025, 3, 21, "International Day of Nowruz"),
        (2025, 6, 5, "World Environment Day"),
        (2025, 9, 21, "International Day of Peace"),
        (2025, 10, 24, "United Nations Day"),
        (2025, 12, 10, "Human Rights Day"),
        (2026, 12, 27, "International Day of Epidemic Preparedness"),
    ]);
}

#[test]
fn the_rule_based_days_fall_where_their_pages_put_them() {
    // The dates each day's own United Nations page gives.
    expect(&[
        (2025, 10, 6, "World Habitat Day"),
        (
            2025,
            11,
            16,
            "World Day of Remembrance for Road Traffic Victims",
        ),
        (2025, 11, 20, "World Philosophy Day"),
        (2025, 7, 5, "International Day of Cooperatives"),
        (2026, 7, 4, "International Day of Cooperatives"),
        (2026, 9, 24, "World Maritime Day"),
        (2026, 5, 9, "World Migratory Bird Day"),
        (2026, 10, 10, "World Migratory Bird Day"),
    ]);
}

#[test]
fn every_day_is_an_observance_that_cites_its_instrument_and_gives_nobody_a_day_off() {
    let calendar = HolidayCalendar::for_year(&UNITED_NATIONS, None, 2026);
    assert!(calendar.all().len() > 200, "{}", calendar.all().len());
    assert!(calendar.is_complete());
    for holiday in calendar.all() {
        assert_eq!(holiday.kind, Kind::Observance, "{}", holiday.name);
        assert!(!holiday.is_day_off(), "{}", holiday.name);
        assert!(!holiday.source.is_empty(), "{} cites nothing", holiday.name);
        let expected = if holiday.name.starts_with("Vesak") {
            Confidence::Approximate
        } else {
            Confidence::Exact
        };
        assert_eq!(holiday.confidence, expected, "{}", holiday.name);
    }
}

#[test]
fn vesak_is_the_may_full_moon() {
    // 2025: the full moon of 12 May. 2026 has two May full moons, on the
    // 1st and the 31st; the first is the day, and the United Nations' list
    // shows Vesak on 1 May.
    expect(&[
        (2025, 5, 12, "Vesak, the Day of the Full Moon"),
        (2026, 5, 1, "Vesak, the Day of the Full Moon"),
    ]);
}

#[test]
fn the_set_is_reachable_by_its_code() {
    assert!(international::by_code("un-days").is_some());
    assert!(international::by_code("nonexistent").is_none());
}
