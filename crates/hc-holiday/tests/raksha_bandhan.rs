//! Raksha Bandhan is not done in Bhadra (audit 10 a5).
//!
//! The full moon of Śrāvaṇa is the day, but the first half of the
//! full-moon tithi is the karaṇa Viṣṭi, Bhadra, and the rite waits until it
//! is over. The rule used to take the day the tithi holds in the afternoon
//! and nothing more: 2025-08-08 and 2026-08-27, a day early, where the
//! calendars give the 9th and the 28th.
//!
//! The days below are Drik Panchang's pages "Raksha Bandhan date and
//! auspicious time" for New Delhi, one per year, read 2026-10-03
//! (`drik-raksha-bandhan`): the first line of each, "Raksha Bandhan on
//! ...". The central government's lists agree for the years the India table
//! reads (2025: 9 August, 2026: 28 August).

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::hindu::RAKSHA_BANDHAN;
use hc_holiday::traditions::HINDU;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn raksha_bandhan(year: i64) -> Vec<Rd> {
    RAKSHA_BANDHAN.days_in_year(year).as_slice().to_vec()
}

/// Drik Panchang's day for every year it was read for, 1995 to 2070,
/// except 2036, which parts from it (below).
const DRIK: [(i64, u8, u8); 75] = [
    (1995, 8, 10),
    (1996, 8, 28),
    (1997, 8, 18),
    (1998, 8, 7),
    (1999, 8, 26),
    (2000, 8, 15),
    (2001, 8, 4),
    (2002, 8, 22),
    (2003, 8, 12),
    (2004, 8, 29),
    (2005, 8, 19),
    (2006, 8, 9),
    (2007, 8, 28),
    (2008, 8, 16),
    (2009, 8, 5),
    (2010, 8, 24),
    (2011, 8, 13),
    (2012, 8, 2),
    (2013, 8, 20),
    (2014, 8, 10),
    (2015, 8, 29),
    (2016, 8, 18),
    (2017, 8, 7),
    (2018, 8, 26),
    (2019, 8, 15),
    (2020, 8, 3),
    (2021, 8, 22),
    (2022, 8, 11),
    (2023, 8, 30),
    (2024, 8, 19),
    (2025, 8, 9),
    (2026, 8, 28),
    (2027, 8, 17),
    (2028, 8, 5),
    (2029, 8, 23),
    (2030, 8, 13),
    (2031, 8, 2),
    (2032, 8, 20),
    (2033, 8, 10),
    (2034, 8, 29),
    (2035, 8, 18),
    (2037, 8, 25),
    (2038, 8, 14),
    (2039, 8, 4),
    (2040, 8, 22),
    (2041, 8, 11),
    (2042, 8, 30),
    (2043, 8, 20),
    (2044, 8, 8),
    (2045, 8, 27),
    (2046, 8, 16),
    (2047, 8, 5),
    (2048, 8, 23),
    (2049, 8, 13),
    (2050, 8, 2),
    (2051, 8, 21),
    (2052, 8, 10),
    (2053, 8, 29),
    (2054, 8, 18),
    (2055, 8, 7),
    (2056, 8, 25),
    (2057, 8, 14),
    (2058, 8, 4),
    (2059, 8, 23),
    (2060, 8, 11),
    (2061, 8, 30),
    (2062, 8, 20),
    (2063, 8, 9),
    (2064, 8, 26),
    (2065, 8, 16),
    (2066, 8, 5),
    (2067, 8, 24),
    (2068, 8, 13),
    (2069, 9, 1),
    (2070, 8, 21),
];

#[test]
fn raksha_bandhan_is_the_day_drik_panchang_gives_for_seventy_five_years() {
    let mut mismatches = Vec::new();
    for (year, month, day) in DRIK {
        let found = raksha_bandhan(year);
        if found != [ymd(year, month, day)] {
            let placed: Vec<_> = found
                .iter()
                .filter_map(|day| gregorian::from_fixed(*day).ok())
                .collect();
            mismatches.push(format!("{year}: Drik {month}-{day}, the rule {placed:?}"));
        }
    }
    assert!(mismatches.is_empty(), "{mismatches:#?}");
}

#[test]
fn bhadra_that_ends_in_the_evening_puts_the_day_on_the_evening() {
    // 2023: the full moon holds the afternoon of 30 August, Bhadra ends at
    // 21:01, and the tithi ends at 07:05 on the 31st, 1 hour 24 minutes
    // after that sunrise, less than six ghaṭikās: "Thread Ceremony Time -
    // after 09:01 PM" on 30 August. The same in 2022, 2029, 2031 and 2032.
    for (year, month, day) in [
        (2022, 8, 11),
        (2023, 8, 30),
        (2029, 8, 23),
        (2031, 8, 2),
        (2032, 8, 20),
    ] {
        assert_eq!(raksha_bandhan(year), [ymd(year, month, day)], "{year}");
    }
}

#[test]
fn bhadra_that_ends_before_a_long_morning_of_the_tithi_puts_the_day_on_the_morning() {
    // 2025, 2026, 2027 and 2028: Bhadra ends in the night, and the tithi
    // runs on for 4, 4, 7 and 8 hours of the next day's daylight, so the
    // day is the next one, when the afternoon has no full moon at all
    // ("Bhadra got over before Sunrise").
    for (year, month, day) in [(2025, 8, 9), (2026, 8, 28), (2027, 8, 17), (2028, 8, 5)] {
        assert_eq!(raksha_bandhan(year), [ymd(year, month, day)], "{year}");
    }
}

#[test]
fn raksha_bandhan_2036_parts_from_drik_panchang_where_the_place_decides() {
    // 2036: Bhadra ends at 19:00 on 6 August, the tithi at 08:18 on the
    // 7th. That is 2 hours 35 minutes after the sunrise of New Delhi, where
    // Drik Panchang's page gives 6 August, and 2 hours 45 after the Central
    // Station's, which the national almanac and this rule read: past six
    // ghaṭikās, the 7th. The rule parts from the page in this one year of
    // 76, for the place alone.
    assert_eq!(raksha_bandhan(2036), [ymd(2036, 8, 7)]);
}

#[test]
fn the_hindu_table_carries_the_rule() {
    let calendar = HolidayCalendar::for_year(&HINDU, None, 2025);
    let days: Vec<_> = calendar
        .all()
        .iter()
        .filter(|holiday| holiday.name == "Raksha Bandhan")
        .map(|holiday| holiday.date)
        .collect();
    assert_eq!(days, [ymd(2025, 8, 9)]);
}
