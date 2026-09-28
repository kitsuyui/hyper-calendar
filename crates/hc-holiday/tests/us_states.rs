//! The states' own days, code by code.
//!
//! Each anchor is a day a state's code lists beyond the federal holidays,
//! in the first year the table carries it and in 2026, with the date its
//! rule gives: the rules are the codes' as `docs/systems/us-state-holidays.md`
//! quotes them, and the dates were worked out from them by hand and by a
//! script apart from this crate, not taken from its output.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::UNITED_STATES;
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::Kind;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// A state's own entries in a year, substitutes left out.
fn own_in_year(region: &str, year: i64) -> Vec<Holiday> {
    HolidayCalendar::for_year(&UNITED_STATES, Some(region), year)
        .in_year(year)
        .into_iter()
        .filter(|holiday| !holiday.regions.is_empty() && !holiday.is_substitute())
        .collect()
}

/// `(region, name, year, month, day, kind)`.
const ANCHORS: &[(&str, &str, i64, u8, u8, Kind)] = &[
    (
        "US-AL",
        "Confederate Memorial Day",
        2025,
        4,
        28,
        Kind::Government,
    ),
    (
        "US-AL",
        "Confederate Memorial Day",
        2026,
        4,
        27,
        Kind::Government,
    ),
    (
        "US-AL",
        "Jefferson Davis' Birthday",
        2025,
        6,
        2,
        Kind::Government,
    ),
    (
        "US-AL",
        "Jefferson Davis' Birthday",
        2026,
        6,
        1,
        Kind::Government,
    ),
    ("US-AK", "Seward's Day", 2025, 3, 31, Kind::Government),
    ("US-AK", "Seward's Day", 2026, 3, 30, Kind::Government),
    ("US-AK", "Alaska Day", 2025, 10, 18, Kind::Government),
    ("US-AK", "Alaska Day", 2026, 10, 18, Kind::Government),
    ("US-AZ", "Mothers' Day", 2026, 5, 10, Kind::Government),
    ("US-AZ", "Native American Day", 2026, 6, 7, Kind::Government),
    ("US-AZ", "Fathers' Day", 2026, 6, 21, Kind::Government),
    ("US-AZ", "American Family Day", 2026, 8, 2, Kind::Government),
    (
        "US-AZ",
        "Constitution Commemoration Day",
        2026,
        9,
        13,
        Kind::Government,
    ),
    ("US-AR", "Christmas Eve", 2024, 12, 24, Kind::Government),
    ("US-AR", "Christmas Eve", 2026, 12, 24, Kind::Government),
    ("US-CA", "Farmworkers Day", 2026, 3, 31, Kind::Government),
    ("US-CA", "Lincoln Day", 2026, 2, 12, Kind::Observance),
    (
        "US-CA",
        "Genocide Remembrance Day",
        2023,
        4,
        24,
        Kind::Observance,
    ),
    (
        "US-CA",
        "Genocide Remembrance Day",
        2026,
        4,
        24,
        Kind::Observance,
    ),
    ("US-CA", "Admission Day", 2026, 9, 9, Kind::Observance),
    (
        "US-CA",
        "Native American Day",
        1999,
        9,
        24,
        Kind::Observance,
    ),
    (
        "US-CA",
        "Native American Day",
        2026,
        9,
        25,
        Kind::Observance,
    ),
    (
        "US-CA",
        "Day after Thanksgiving",
        2026,
        11,
        27,
        Kind::Government,
    ),
    (
        "US-CO",
        "Frances Xavier Cabrini Day",
        2020,
        10,
        5,
        Kind::Government,
    ),
    (
        "US-CO",
        "Frances Xavier Cabrini Day",
        2026,
        10,
        5,
        Kind::Government,
    ),
    ("US-CT", "Lincoln Day", 2026, 2, 12, Kind::Government),
    ("US-DE", "Good Friday", 2026, 4, 3, Kind::Government),
    (
        "US-DE",
        "Friday after Thanksgiving",
        2026,
        11,
        27,
        Kind::Government,
    ),
    (
        "US-DE",
        "General Election Day",
        2026,
        11,
        3,
        Kind::Government,
    ),
    (
        "US-DC",
        "District of Columbia Emancipation Day",
        2005,
        4,
        16,
        Kind::Government,
    ),
    (
        "US-DC",
        "District of Columbia Emancipation Day",
        2026,
        4,
        16,
        Kind::Government,
    ),
    (
        "US-FL",
        "Birthday of Martin Luther King, Jr.",
        2026,
        1,
        15,
        Kind::Observance,
    ),
    (
        "US-FL",
        "Birthday of Robert E. Lee",
        2026,
        1,
        19,
        Kind::Observance,
    ),
    ("US-FL", "Lincoln's Birthday", 2026, 2, 12, Kind::Observance),
    (
        "US-FL",
        "Susan B. Anthony's Birthday",
        2026,
        2,
        15,
        Kind::Observance,
    ),
    (
        "US-FL",
        "Tuskegee Airmen Commemoration Day",
        2026,
        3,
        26,
        Kind::Observance,
    ),
    ("US-FL", "Good Friday", 2026, 4, 3, Kind::Observance),
    ("US-FL", "Pascua Florida Day", 2026, 4, 2, Kind::Observance),
    (
        "US-FL",
        "Confederate Memorial Day",
        2026,
        4,
        26,
        Kind::Observance,
    ),
    (
        "US-FL",
        "Birthday of Jefferson Davis",
        2026,
        6,
        3,
        Kind::Observance,
    ),
    ("US-FL", "Flag Day", 2026, 6, 14, Kind::Observance),
    (
        "US-FL",
        "General Election Day",
        2026,
        11,
        3,
        Kind::Observance,
    ),
    (
        "US-FL",
        "Friday after Thanksgiving",
        2022,
        11,
        25,
        Kind::Government,
    ),
    (
        "US-FL",
        "Friday after Thanksgiving",
        2026,
        11,
        27,
        Kind::Government,
    ),
    (
        "US-HI",
        "Prince Jonah Kuhio Kalanianaole Day",
        2001,
        3,
        26,
        Kind::Government,
    ),
    (
        "US-HI",
        "Prince Jonah Kuhio Kalanianaole Day",
        2026,
        3,
        26,
        Kind::Government,
    ),
    ("US-HI", "Good Friday", 2001, 4, 13, Kind::Government),
    ("US-HI", "Good Friday", 2026, 4, 3, Kind::Government),
    (
        "US-HI",
        "King Kamehameha I Day",
        2001,
        6,
        11,
        Kind::Government,
    ),
    (
        "US-HI",
        "King Kamehameha I Day",
        2026,
        6,
        11,
        Kind::Government,
    ),
    ("US-HI", "Statehood Day", 2001, 8, 17, Kind::Government),
    ("US-HI", "Statehood Day", 2026, 8, 21, Kind::Government),
    (
        "US-HI",
        "General Election Day",
        2002,
        11,
        5,
        Kind::Government,
    ),
    (
        "US-HI",
        "General Election Day",
        2026,
        11,
        3,
        Kind::Government,
    ),
    (
        "US-ID",
        "Constitutional Commemorative Day",
        1989,
        9,
        17,
        Kind::Observance,
    ),
    (
        "US-ID",
        "Constitutional Commemorative Day",
        2026,
        9,
        17,
        Kind::Observance,
    ),
    ("US-ID", "Children's Day", 2003, 4, 30, Kind::Observance),
    ("US-ID", "Children's Day", 2026, 4, 30, Kind::Observance),
    ("US-ID", "Idaho Day", 2014, 3, 4, Kind::Observance),
    ("US-ID", "Idaho Day", 2026, 3, 4, Kind::Observance),
    ("US-IL", "Lincoln's Birthday", 2022, 2, 12, Kind::Observance),
    ("US-IL", "Lincoln's Birthday", 2026, 2, 12, Kind::Observance),
    (
        "US-IL",
        "Casimir Pulaski's Birthday",
        2022,
        3,
        7,
        Kind::Observance,
    ),
    (
        "US-IL",
        "Casimir Pulaski's Birthday",
        2026,
        3,
        2,
        Kind::Observance,
    ),
    ("US-IL", "Good Friday", 2022, 4, 15, Kind::Observance),
    ("US-IL", "Good Friday", 2026, 4, 3, Kind::Observance),
    (
        "US-IL",
        "General Election Day",
        2022,
        11,
        8,
        Kind::Observance,
    ),
    (
        "US-IL",
        "General Election Day",
        2026,
        11,
        3,
        Kind::Observance,
    ),
    ("US-IN", "Lincoln's Birthday", 2026, 2, 12, Kind::Government),
    ("US-IN", "Good Friday", 2026, 4, 3, Kind::Government),
    ("US-IN", "Election Day", 2026, 11, 3, Kind::Government),
    ("US-IA", "Lincoln's Birthday", 1993, 2, 12, Kind::Observance),
    ("US-IA", "Lincoln's Birthday", 2026, 2, 12, Kind::Observance),
    (
        "US-IA",
        "Friday after Thanksgiving",
        2008,
        11,
        28,
        Kind::Government,
    ),
    (
        "US-IA",
        "Friday after Thanksgiving",
        2026,
        11,
        27,
        Kind::Government,
    ),
    (
        "US-KS",
        "General Pulaski's Memorial Day",
        1935,
        10,
        11,
        Kind::Observance,
    ),
    (
        "US-KS",
        "General Pulaski's Memorial Day",
        2026,
        10,
        11,
        Kind::Observance,
    ),
    ("US-KS", "Family Day", 1971, 11, 28, Kind::Observance),
    ("US-KS", "Family Day", 2026, 11, 29, Kind::Observance),
    (
        "US-KS",
        "Pearl Harbor Remembrance Day",
        1988,
        12,
        7,
        Kind::Observance,
    ),
    (
        "US-KS",
        "Pearl Harbor Remembrance Day",
        2026,
        12,
        7,
        Kind::Observance,
    ),
    (
        "US-KS",
        "Dwight D. Eisenhower Day",
        1999,
        10,
        14,
        Kind::Observance,
    ),
    (
        "US-KS",
        "Dwight D. Eisenhower Day",
        2026,
        10,
        14,
        Kind::Observance,
    ),
    (
        "US-KS",
        "Native American Day",
        2013,
        9,
        28,
        Kind::Observance,
    ),
    (
        "US-KS",
        "Native American Day",
        2026,
        9,
        26,
        Kind::Observance,
    ),
    (
        "US-KS",
        "National Day of the Cowboy",
        2014,
        7,
        26,
        Kind::Observance,
    ),
    (
        "US-KS",
        "National Day of the Cowboy",
        2026,
        7,
        25,
        Kind::Observance,
    ),
    ("US-KY", "Robert E. Lee Day", 2025, 1, 19, Kind::Observance),
    ("US-KY", "Robert E. Lee Day", 2026, 1, 19, Kind::Observance),
    (
        "US-KY",
        "Franklin D. Roosevelt Day",
        2025,
        1,
        30,
        Kind::Observance,
    ),
    (
        "US-KY",
        "Franklin D. Roosevelt Day",
        2026,
        1,
        30,
        Kind::Observance,
    ),
    ("US-KY", "Lincoln's Birthday", 2025, 2, 12, Kind::Observance),
    ("US-KY", "Lincoln's Birthday", 2026, 2, 12, Kind::Observance),
    (
        "US-KY",
        "Confederate Memorial Day and Jefferson Davis Day",
        2025,
        6,
        3,
        Kind::Observance,
    ),
    (
        "US-KY",
        "Confederate Memorial Day and Jefferson Davis Day",
        2026,
        6,
        3,
        Kind::Observance,
    ),
    (
        "US-KY",
        "Presidential Election Day",
        2028,
        11,
        7,
        Kind::Government,
    ),
    (
        "US-LA",
        "Battle of New Orleans",
        2026,
        1,
        8,
        Kind::Observance,
    ),
    ("US-LA", "Mardi Gras", 2026, 2, 17, Kind::Government),
    ("US-LA", "Good Friday", 2026, 4, 3, Kind::Government),
    ("US-LA", "Huey P. Long Day", 2026, 8, 30, Kind::Observance),
    ("US-LA", "All Saints' Day", 2026, 11, 1, Kind::Observance),
    ("US-ME", "Patriot's Day", 2026, 4, 20, Kind::Government),
    ("US-MD", "Lincoln's Birthday", 2026, 2, 12, Kind::Observance),
    ("US-MD", "Maryland Day", 2026, 3, 25, Kind::Observance),
    ("US-MD", "Good Friday", 2026, 4, 3, Kind::Observance),
    ("US-MD", "Defenders' Day", 2026, 9, 12, Kind::Observance),
    (
        "US-MD",
        "American Indian Heritage Day",
        2026,
        11,
        27,
        Kind::Government,
    ),
    (
        "US-MD",
        "General Election Day",
        2026,
        11,
        3,
        Kind::Government,
    ),
    ("US-MA", "Patriots' Day", 2025, 4, 21, Kind::Government),
    ("US-MA", "Patriots' Day", 2026, 4, 20, Kind::Government),
    ("US-MI", "Lincoln's Birthday", 2025, 2, 12, Kind::Observance),
    ("US-MI", "Lincoln's Birthday", 2026, 2, 12, Kind::Observance),
    (
        "US-MN",
        "Friday after Thanksgiving",
        2025,
        11,
        28,
        Kind::Government,
    ),
    (
        "US-MN",
        "Friday after Thanksgiving",
        2026,
        11,
        27,
        Kind::Government,
    ),
    (
        "US-MS",
        "Confederate Memorial Day",
        2025,
        4,
        28,
        Kind::Government,
    ),
    (
        "US-MS",
        "Confederate Memorial Day",
        2026,
        4,
        27,
        Kind::Government,
    ),
    ("US-MO", "Lincoln Day", 2022, 2, 12, Kind::Government),
    ("US-MO", "Lincoln Day", 2026, 2, 12, Kind::Government),
    ("US-MO", "Truman Day", 2022, 5, 8, Kind::Government),
    ("US-MO", "Truman Day", 2026, 5, 8, Kind::Government),
    (
        "US-MT",
        "General Election Day",
        2026,
        11,
        3,
        Kind::Government,
    ),
    ("US-NE", "Arbor Day", 2024, 4, 26, Kind::Government),
    ("US-NE", "Arbor Day", 2026, 4, 24, Kind::Government),
    (
        "US-NE",
        "Day after Thanksgiving",
        2024,
        11,
        29,
        Kind::Government,
    ),
    (
        "US-NE",
        "Day after Thanksgiving",
        2026,
        11,
        27,
        Kind::Government,
    ),
    ("US-NV", "Nevada Day", 2025, 10, 31, Kind::Government),
    ("US-NV", "Nevada Day", 2026, 10, 30, Kind::Government),
    ("US-NV", "Family Day", 2025, 11, 28, Kind::Government),
    ("US-NV", "Family Day", 2026, 11, 27, Kind::Government),
    ("US-NJ", "Lincoln's Birthday", 2024, 2, 12, Kind::Observance),
    ("US-NJ", "Lincoln's Birthday", 2026, 2, 12, Kind::Observance),
    ("US-NJ", "Good Friday", 2024, 3, 29, Kind::Government),
    ("US-NJ", "Good Friday", 2026, 4, 3, Kind::Government),
    ("US-NJ", "Juneteenth Day", 2024, 6, 21, Kind::Government),
    ("US-NJ", "Juneteenth Day", 2026, 6, 19, Kind::Government),
    (
        "US-NJ",
        "General Election Day",
        2024,
        11,
        5,
        Kind::Government,
    ),
    (
        "US-NJ",
        "General Election Day",
        2026,
        11,
        3,
        Kind::Government,
    ),
    ("US-NM", "American Indian Day", 2024, 2, 2, Kind::Observance),
    ("US-NM", "American Indian Day", 2026, 2, 6, Kind::Observance),
    (
        "US-NM",
        "Guadalupe Hidalgo Treaty Day",
        2024,
        2,
        2,
        Kind::Observance,
    ),
    (
        "US-NM",
        "Guadalupe Hidalgo Treaty Day",
        2026,
        2,
        2,
        Kind::Observance,
    ),
    (
        "US-NM",
        "African-American Day",
        2024,
        2,
        9,
        Kind::Observance,
    ),
    (
        "US-NM",
        "African-American Day",
        2026,
        2,
        13,
        Kind::Observance,
    ),
    ("US-NM", "Arbor Day", 2024, 3, 8, Kind::Observance),
    ("US-NM", "Arbor Day", 2026, 3, 13, Kind::Observance),
    ("US-NM", "Bataan Day", 2024, 4, 9, Kind::Observance),
    ("US-NM", "Bataan Day", 2026, 4, 9, Kind::Observance),
    ("US-NM", "Ernie Pyle Day", 2024, 8, 3, Kind::Observance),
    ("US-NM", "Ernie Pyle Day", 2026, 8, 3, Kind::Observance),
    ("US-NY", "Lincoln's Birthday", 2020, 2, 12, Kind::Government),
    ("US-NY", "Lincoln's Birthday", 2026, 2, 12, Kind::Government),
    ("US-NY", "Flag Day", 2020, 6, 14, Kind::Government),
    ("US-NY", "Flag Day", 2026, 6, 14, Kind::Government),
    (
        "US-NY",
        "General Election Day",
        2020,
        11,
        3,
        Kind::Government,
    ),
    (
        "US-NY",
        "General Election Day",
        2026,
        11,
        3,
        Kind::Government,
    ),
    (
        "US-NC",
        "Robert E. Lee's Birthday",
        2023,
        1,
        19,
        Kind::Observance,
    ),
    (
        "US-NC",
        "Robert E. Lee's Birthday",
        2026,
        1,
        19,
        Kind::Observance,
    ),
    (
        "US-NC",
        "Greek Independence Day",
        2023,
        3,
        25,
        Kind::Observance,
    ),
    (
        "US-NC",
        "Greek Independence Day",
        2026,
        3,
        25,
        Kind::Observance,
    ),
    (
        "US-NC",
        "Anniversary of the Halifax Resolves",
        2023,
        4,
        12,
        Kind::Observance,
    ),
    (
        "US-NC",
        "Anniversary of the Halifax Resolves",
        2026,
        4,
        12,
        Kind::Observance,
    ),
    (
        "US-NC",
        "Confederate Memorial Day",
        2023,
        5,
        10,
        Kind::Observance,
    ),
    (
        "US-NC",
        "Confederate Memorial Day",
        2026,
        5,
        10,
        Kind::Observance,
    ),
    (
        "US-NC",
        "Anniversary of the Mecklenburg Declaration of Independence",
        2023,
        5,
        20,
        Kind::Observance,
    ),
    (
        "US-NC",
        "Anniversary of the Mecklenburg Declaration of Independence",
        2026,
        5,
        20,
        Kind::Observance,
    ),
    ("US-NC", "Good Friday", 2023, 4, 7, Kind::Observance),
    ("US-NC", "Good Friday", 2026, 4, 3, Kind::Observance),
    (
        "US-NC",
        "First Responders Day",
        2023,
        9,
        11,
        Kind::Observance,
    ),
    (
        "US-NC",
        "First Responders Day",
        2026,
        9,
        11,
        Kind::Observance,
    ),
    ("US-NC", "Yom Kippur", 2023, 9, 25, Kind::Observance),
    ("US-NC", "Yom Kippur", 2026, 9, 21, Kind::Observance),
    ("US-NC", "Election Day", 2024, 11, 5, Kind::Observance),
    ("US-NC", "Election Day", 2026, 11, 3, Kind::Observance),
    ("US-ND", "Good Friday", 2024, 3, 29, Kind::Government),
    ("US-ND", "Good Friday", 2026, 4, 3, Kind::Government),
    (
        "US-OR",
        "Oregon Statehood Day",
        2015,
        2,
        14,
        Kind::Observance,
    ),
    (
        "US-OR",
        "Oregon Statehood Day",
        2026,
        2,
        14,
        Kind::Observance,
    ),
    ("US-PA", "Good Friday", 2026, 4, 3, Kind::Observance),
    ("US-PA", "Flag Day", 2026, 6, 14, Kind::Observance),
    ("US-PA", "Election Day", 2026, 11, 3, Kind::Observance),
    (
        "US-RI",
        "Rhode Island Independence Day",
        2026,
        5,
        4,
        Kind::Government,
    ),
    ("US-RI", "Victory Day", 2026, 8, 10, Kind::Government),
    ("US-RI", "Election Day", 2026, 11, 3, Kind::Government),
    (
        "US-SC",
        "Confederate Memorial Day",
        2010,
        5,
        10,
        Kind::Government,
    ),
    (
        "US-SC",
        "Confederate Memorial Day",
        2026,
        5,
        10,
        Kind::Government,
    ),
    (
        "US-SC",
        "Day after Thanksgiving",
        2009,
        11,
        27,
        Kind::Government,
    ),
    (
        "US-SC",
        "Day after Thanksgiving",
        2026,
        11,
        27,
        Kind::Government,
    ),
    ("US-SC", "Christmas Eve", 2009, 12, 24, Kind::Government),
    ("US-SC", "Christmas Eve", 2026, 12, 24, Kind::Government),
    ("US-SC", "26 December", 2009, 12, 26, Kind::Government),
    ("US-SC", "26 December", 2026, 12, 26, Kind::Government),
    (
        "US-SD",
        "South Dakota Statehood Day",
        2001,
        11,
        2,
        Kind::Observance,
    ),
    (
        "US-SD",
        "South Dakota Statehood Day",
        2026,
        11,
        2,
        Kind::Observance,
    ),
    (
        "US-SD",
        "Little Big Horn Recognition Day",
        1994,
        6,
        25,
        Kind::Observance,
    ),
    (
        "US-SD",
        "Little Big Horn Recognition Day",
        2026,
        6,
        25,
        Kind::Observance,
    ),
    ("US-SD", "Wounded Knee Day", 1994, 12, 29, Kind::Observance),
    ("US-SD", "Wounded Knee Day", 2026, 12, 29, Kind::Observance),
    (
        "US-SD",
        "Bill of Rights Day",
        1998,
        12,
        15,
        Kind::Observance,
    ),
    (
        "US-SD",
        "Bill of Rights Day",
        2026,
        12,
        15,
        Kind::Observance,
    ),
    ("US-SD", "Joe Foss Day", 2004, 4, 17, Kind::Observance),
    ("US-SD", "Joe Foss Day", 2026, 4, 17, Kind::Observance),
    (
        "US-SD",
        "Purple Heart Recognition Day",
        2013,
        8,
        7,
        Kind::Observance,
    ),
    (
        "US-SD",
        "Purple Heart Recognition Day",
        2026,
        8,
        7,
        Kind::Observance,
    ),
    (
        "US-SD",
        "Welcome Home Vietnam Veterans Day",
        2013,
        3,
        30,
        Kind::Observance,
    ),
    (
        "US-SD",
        "Welcome Home Vietnam Veterans Day",
        2026,
        3,
        30,
        Kind::Observance,
    ),
    (
        "US-SD",
        "Day of the American Cowboy",
        2014,
        7,
        26,
        Kind::Observance,
    ),
    (
        "US-SD",
        "Day of the American Cowboy",
        2026,
        7,
        25,
        Kind::Observance,
    ),
    ("US-SD", "Peter Norbeck Day", 2018, 8, 27, Kind::Observance),
    ("US-SD", "Peter Norbeck Day", 2026, 8, 27, Kind::Observance),
    (
        "US-SD",
        "Medal of Honor Recognition Day",
        2024,
        3,
        25,
        Kind::Observance,
    ),
    (
        "US-SD",
        "Medal of Honor Recognition Day",
        2026,
        3,
        25,
        Kind::Observance,
    ),
    ("US-TN", "Good Friday", 2024, 3, 29, Kind::Government),
    ("US-TN", "Good Friday", 2026, 4, 3, Kind::Government),
    (
        "US-TX",
        "Confederate Heroes Day",
        2025,
        1,
        19,
        Kind::Government,
    ),
    (
        "US-TX",
        "Confederate Heroes Day",
        2026,
        1,
        19,
        Kind::Government,
    ),
    (
        "US-TX",
        "Texas Independence Day",
        2025,
        3,
        2,
        Kind::Government,
    ),
    (
        "US-TX",
        "Texas Independence Day",
        2026,
        3,
        2,
        Kind::Government,
    ),
    ("US-TX", "San Jacinto Day", 2025, 4, 21, Kind::Government),
    ("US-TX", "San Jacinto Day", 2026, 4, 21, Kind::Government),
    (
        "US-TX",
        "Lyndon Baines Johnson Day",
        2025,
        8,
        27,
        Kind::Government,
    ),
    (
        "US-TX",
        "Lyndon Baines Johnson Day",
        2026,
        8,
        27,
        Kind::Government,
    ),
    (
        "US-TX",
        "Friday after Thanksgiving",
        2025,
        11,
        28,
        Kind::Government,
    ),
    (
        "US-TX",
        "Friday after Thanksgiving",
        2026,
        11,
        27,
        Kind::Government,
    ),
    ("US-TX", "24 December", 2025, 12, 24, Kind::Government),
    ("US-TX", "24 December", 2026, 12, 24, Kind::Government),
    ("US-TX", "26 December", 2025, 12, 26, Kind::Government),
    ("US-TX", "26 December", 2026, 12, 26, Kind::Government),
    ("US-UT", "Pioneer Day", 2025, 7, 24, Kind::Government),
    ("US-UT", "Pioneer Day", 2026, 7, 24, Kind::Government),
    (
        "US-UT",
        "Juneteenth National Freedom Day",
        2025,
        6,
        16,
        Kind::Government,
    ),
    (
        "US-UT",
        "Juneteenth National Freedom Day",
        2026,
        6,
        15,
        Kind::Government,
    ),
    ("US-VT", "Town Meeting Day", 2024, 3, 5, Kind::Government),
    ("US-VT", "Town Meeting Day", 2026, 3, 3, Kind::Government),
    (
        "US-VT",
        "Bennington Battle Day",
        2024,
        8,
        16,
        Kind::Government,
    ),
    (
        "US-VT",
        "Bennington Battle Day",
        2026,
        8,
        16,
        Kind::Government,
    ),
    ("US-VA", "Election Day", 2020, 11, 3, Kind::Government),
    ("US-VA", "Election Day", 2026, 11, 3, Kind::Government),
    (
        "US-VA",
        "Day after Thanksgiving",
        2020,
        11,
        27,
        Kind::Government,
    ),
    (
        "US-VA",
        "Day after Thanksgiving",
        2026,
        11,
        27,
        Kind::Government,
    ),
    (
        "US-WA",
        "Native American Heritage Day",
        2014,
        11,
        28,
        Kind::Government,
    ),
    (
        "US-WA",
        "Native American Heritage Day",
        2026,
        11,
        27,
        Kind::Government,
    ),
    ("US-WV", "West Virginia Day", 2026, 6, 20, Kind::Government),
    ("US-WV", "Lincoln's Day", 2026, 11, 27, Kind::Government),
    (
        "US-WI",
        "General Election Day",
        2026,
        11,
        3,
        Kind::Government,
    ),
];

/// `(region, name, first year carried, whether a source gives that year
/// as the day's establishment)`: before an established year the day is
/// absent, before any other it is a gap.
const FIRST_YEARS: &[(&str, &str, i64, bool)] = &[
    ("US-AL", "Confederate Memorial Day", 2025, false),
    ("US-AL", "Jefferson Davis' Birthday", 2025, false),
    ("US-AK", "Seward's Day", 2025, false),
    ("US-AK", "Alaska Day", 2025, false),
    ("US-AZ", "Mothers' Day", 2026, false),
    ("US-AZ", "Native American Day", 2026, false),
    ("US-AZ", "Fathers' Day", 2026, false),
    ("US-AZ", "American Family Day", 2026, false),
    ("US-AZ", "Constitution Commemoration Day", 2026, false),
    ("US-AR", "Christmas Eve", 2024, false),
    ("US-CA", "Farmworkers Day", 2026, false),
    ("US-CA", "Lincoln Day", 2026, false),
    ("US-CA", "Genocide Remembrance Day", 2023, true),
    ("US-CA", "Admission Day", 2026, false),
    ("US-CA", "Native American Day", 1999, false),
    ("US-CA", "Day after Thanksgiving", 2026, false),
    ("US-CO", "Frances Xavier Cabrini Day", 2020, true),
    ("US-CT", "Lincoln Day", 2026, false),
    ("US-DE", "Good Friday", 2026, false),
    ("US-DE", "Friday after Thanksgiving", 2026, false),
    ("US-DE", "General Election Day", 2026, false),
    ("US-DC", "District of Columbia Emancipation Day", 2005, true),
    ("US-FL", "Birthday of Martin Luther King, Jr.", 2026, false),
    ("US-FL", "Birthday of Robert E. Lee", 2026, false),
    ("US-FL", "Lincoln's Birthday", 2026, false),
    ("US-FL", "Susan B. Anthony's Birthday", 2026, false),
    ("US-FL", "Tuskegee Airmen Commemoration Day", 2026, false),
    ("US-FL", "Good Friday", 2026, false),
    ("US-FL", "Pascua Florida Day", 1953, true),
    ("US-FL", "Confederate Memorial Day", 2026, false),
    ("US-FL", "Birthday of Jefferson Davis", 2026, false),
    ("US-FL", "Flag Day", 2026, false),
    ("US-FL", "General Election Day", 2026, false),
    ("US-FL", "Friday after Thanksgiving", 2022, false),
    ("US-HI", "Prince Jonah Kuhio Kalanianaole Day", 2001, false),
    ("US-HI", "Good Friday", 2001, false),
    ("US-HI", "King Kamehameha I Day", 2001, false),
    ("US-HI", "Statehood Day", 2001, false),
    ("US-HI", "General Election Day", 2001, false),
    ("US-ID", "Constitutional Commemorative Day", 1989, true),
    ("US-ID", "Children's Day", 2003, true),
    ("US-ID", "Idaho Day", 2014, true),
    ("US-IL", "Lincoln's Birthday", 2022, false),
    ("US-IL", "Casimir Pulaski's Birthday", 2022, false),
    ("US-IL", "Good Friday", 2022, false),
    ("US-IL", "General Election Day", 2022, false),
    ("US-IN", "Lincoln's Birthday", 2026, false),
    ("US-IN", "Good Friday", 2026, false),
    ("US-IN", "Election Day", 2026, false),
    ("US-IA", "Lincoln's Birthday", 1993, false),
    ("US-IA", "Friday after Thanksgiving", 2008, false),
    ("US-KS", "General Pulaski's Memorial Day", 1935, true),
    ("US-KS", "Family Day", 1971, true),
    ("US-KS", "Pearl Harbor Remembrance Day", 1988, true),
    ("US-KS", "Dwight D. Eisenhower Day", 1999, true),
    ("US-KS", "Native American Day", 2013, false),
    ("US-KS", "National Day of the Cowboy", 2014, true),
    ("US-KY", "Robert E. Lee Day", 2025, false),
    ("US-KY", "Franklin D. Roosevelt Day", 2025, false),
    ("US-KY", "Lincoln's Birthday", 2025, false),
    (
        "US-KY",
        "Confederate Memorial Day and Jefferson Davis Day",
        2025,
        false,
    ),
    ("US-KY", "Presidential Election Day", 2025, false),
    ("US-LA", "Battle of New Orleans", 2026, false),
    ("US-LA", "Mardi Gras", 2026, false),
    ("US-LA", "Good Friday", 2026, false),
    ("US-LA", "Huey P. Long Day", 2026, false),
    ("US-LA", "All Saints' Day", 2026, false),
    ("US-ME", "Patriot's Day", 2026, false),
    ("US-MD", "Lincoln's Birthday", 2026, false),
    ("US-MD", "Maryland Day", 2026, false),
    ("US-MD", "Good Friday", 2026, false),
    ("US-MD", "Defenders' Day", 2026, false),
    ("US-MD", "American Indian Heritage Day", 2026, false),
    ("US-MD", "General Election Day", 2026, false),
    ("US-MA", "Patriots' Day", 2025, false),
    ("US-MI", "Lincoln's Birthday", 2025, false),
    ("US-MN", "Friday after Thanksgiving", 2025, false),
    ("US-MS", "Confederate Memorial Day", 2025, false),
    ("US-MO", "Lincoln Day", 2022, false),
    ("US-MO", "Truman Day", 2022, false),
    ("US-MT", "General Election Day", 2025, false),
    ("US-NE", "Arbor Day", 2024, false),
    ("US-NE", "Day after Thanksgiving", 2024, false),
    ("US-NV", "Nevada Day", 2025, false),
    ("US-NV", "Family Day", 2025, false),
    ("US-NJ", "Lincoln's Birthday", 2024, false),
    ("US-NJ", "Good Friday", 2024, false),
    ("US-NJ", "Juneteenth Day", 2024, false),
    ("US-NJ", "General Election Day", 2024, false),
    ("US-NM", "American Indian Day", 2024, false),
    ("US-NM", "Guadalupe Hidalgo Treaty Day", 2024, false),
    ("US-NM", "African-American Day", 2024, false),
    ("US-NM", "Arbor Day", 2024, false),
    ("US-NM", "Bataan Day", 2024, false),
    ("US-NM", "Ernie Pyle Day", 2024, false),
    ("US-NY", "Lincoln's Birthday", 2020, false),
    ("US-NY", "Flag Day", 2020, false),
    ("US-NY", "General Election Day", 2020, false),
    ("US-NC", "Robert E. Lee's Birthday", 2023, false),
    ("US-NC", "Greek Independence Day", 2023, false),
    ("US-NC", "Anniversary of the Halifax Resolves", 2023, false),
    ("US-NC", "Confederate Memorial Day", 2023, false),
    (
        "US-NC",
        "Anniversary of the Mecklenburg Declaration of Independence",
        2023,
        false,
    ),
    ("US-NC", "Good Friday", 2023, false),
    ("US-NC", "First Responders Day", 2023, false),
    ("US-NC", "Yom Kippur", 2023, false),
    ("US-NC", "Election Day", 2023, false),
    ("US-ND", "Good Friday", 2024, false),
    ("US-OR", "Oregon Statehood Day", 2015, true),
    ("US-PA", "Good Friday", 2026, false),
    ("US-PA", "Flag Day", 2026, false),
    ("US-PA", "Election Day", 2026, false),
    ("US-RI", "Rhode Island Independence Day", 2026, false),
    ("US-RI", "Victory Day", 2026, false),
    ("US-RI", "Election Day", 2026, false),
    ("US-SC", "Confederate Memorial Day", 2010, false),
    ("US-SC", "Day after Thanksgiving", 2009, false),
    ("US-SC", "Christmas Eve", 2009, false),
    ("US-SC", "26 December", 2009, false),
    ("US-SD", "South Dakota Statehood Day", 2001, true),
    ("US-SD", "Little Big Horn Recognition Day", 1994, true),
    ("US-SD", "Wounded Knee Day", 1994, true),
    ("US-SD", "Bill of Rights Day", 1998, true),
    ("US-SD", "Joe Foss Day", 2004, true),
    ("US-SD", "Purple Heart Recognition Day", 2013, true),
    ("US-SD", "Welcome Home Vietnam Veterans Day", 2013, true),
    ("US-SD", "Day of the American Cowboy", 2014, true),
    ("US-SD", "Peter Norbeck Day", 2018, true),
    ("US-SD", "Medal of Honor Recognition Day", 2024, true),
    ("US-TN", "Good Friday", 2024, false),
    ("US-TX", "Confederate Heroes Day", 2025, false),
    ("US-TX", "Texas Independence Day", 2025, false),
    ("US-TX", "San Jacinto Day", 2025, false),
    ("US-TX", "Lyndon Baines Johnson Day", 2025, false),
    ("US-TX", "Friday after Thanksgiving", 2025, false),
    ("US-TX", "24 December", 2025, false),
    ("US-TX", "26 December", 2025, false),
    ("US-UT", "Pioneer Day", 2025, false),
    ("US-UT", "Juneteenth National Freedom Day", 2025, false),
    ("US-VT", "Town Meeting Day", 2024, false),
    ("US-VT", "Bennington Battle Day", 2024, false),
    ("US-VA", "Election Day", 2020, true),
    ("US-VA", "Day after Thanksgiving", 2020, false),
    ("US-WA", "Native American Heritage Day", 2014, false),
    ("US-WV", "West Virginia Day", 2026, false),
    ("US-WV", "Lincoln's Day", 2026, false),
    ("US-WI", "General Election Day", 2025, false),
];

#[test]
fn every_state_day_falls_where_its_code_puts_it() {
    for &(region, name, year, month, day, kind) in ANCHORS {
        let found: Vec<Holiday> = own_in_year(region, year)
            .into_iter()
            .filter(|holiday| holiday.name == name)
            .collect();
        assert_eq!(found.len(), 1, "{region} {name} {year}: {found:?}");
        let entry = found[0];
        assert_eq!(entry.date, ymd(year, month, day), "{region} {name} {year}");
        assert_eq!(entry.kind, kind, "{region} {name}");
        assert_eq!(entry.regions, [region], "{region} {name}");
        assert!(!entry.source.is_empty(), "{region} {name} cites nothing");
    }
}

#[test]
fn before_its_first_year_a_state_day_is_absent_or_a_gap() {
    for &(region, name, first, established) in FIRST_YEARS {
        let before = first - 1;
        assert!(
            own_in_year(region, before)
                .iter()
                .all(|holiday| holiday.name != name),
            "{region} {name} {before}"
        );
        let calendar = HolidayCalendar::for_year(&UNITED_STATES, Some(region), before);
        let gap = calendar
            .gaps()
            .iter()
            .any(|gap| gap.name == name && gap.year == before);
        assert_eq!(gap, !established, "{region} {name} {before}");
    }
}

#[test]
fn a_session_law_makes_the_years_before_absent_and_a_copy_makes_them_a_gap() {
    // Colorado's Cabrini Day was created by HB20-1031, in force 2020: 2019
    // has no day and no gap. Texas Independence Day is carried from the
    // copy read, of 2025: 2024 is a gap, not a year without the day.
    let gaps = |region, year| {
        HolidayCalendar::for_year(&UNITED_STATES, Some(region), year)
            .gaps()
            .iter()
            .map(|gap| gap.name)
            .collect::<Vec<_>>()
    };
    assert!(!gaps("US-CO", 2019).contains(&"Frances Xavier Cabrini Day"));
    assert!(gaps("US-TX", 2024).contains(&"Texas Independence Day"));
    // Kansas's Native American Day: set in 1945, carried under its name
    // from 2013; 1944 absent, 1945 to 2012 a gap.
    assert!(!gaps("US-KS", 1944).contains(&"Native American Day"));
    assert!(gaps("US-KS", 1945).contains(&"Native American Day"));
    assert!(gaps("US-KS", 2012).contains(&"Native American Day"));
    // The nationwide calendar has no state's gap.
    assert!(HolidayCalendar::for_year(&UNITED_STATES, None, 2020).is_complete());
}

#[test]
fn a_state_day_belongs_to_its_state_and_is_no_day_off() {
    // Texas Independence Day, Monday 2 March 2026: in Texas, not in
    // Oklahoma next door nor nationwide, and a business day in Texas.
    let texas = HolidayCalendar::for_year(&UNITED_STATES, Some("US-TX"), 2026);
    let day = ymd(2026, 3, 2);
    assert!(
        texas
            .on(day)
            .iter()
            .any(|holiday| holiday.name == "Texas Independence Day")
    );
    assert!(texas.is_business_day(day));
    assert!(
        HolidayCalendar::for_year(&UNITED_STATES, Some("US-OK"), 2026)
            .on(day)
            .is_empty()
    );
    assert!(
        HolidayCalendar::for_year(&UNITED_STATES, None, 2026)
            .on(day)
            .is_empty()
    );
    // The region is matched as every identifier is.
    assert_eq!(
        own_in_year("us-tx", 2026).len(),
        own_in_year("US-TX", 2026).len()
    );
    for &(_, _, _, _, _, kind) in ANCHORS {
        assert!(!kind.is_day_off());
    }
}

#[test]
fn the_election_days_fall_in_the_years_their_codes_name() {
    // Kentucky's presidential election day: 7 November 2028, not 2026.
    assert!(
        own_in_year("US-KY", 2026)
            .iter()
            .all(|holiday| holiday.name != "Presidential Election Day")
    );
    // Delaware's general election: even years only.
    assert!(
        own_in_year("US-DE", 2027)
            .iter()
            .all(|holiday| holiday.name != "General Election Day")
    );
    // New York's, every year: 2 November 2027.
    assert!(
        own_in_year("US-NY", 2027)
            .iter()
            .any(|holiday| holiday.name == "General Election Day"
                && holiday.date == ymd(2027, 11, 2))
    );
}

#[test]
fn utah_keeps_juneteenth_on_a_monday() {
    // Friday 19 June 2026 → Monday 15 June; Saturday 19 June 2027 →
    // Monday 21 June.
    for (year, day) in [(2026, 15), (2027, 21)] {
        let found: Vec<Holiday> = own_in_year("US-UT", year)
            .into_iter()
            .filter(|holiday| holiday.name == "Juneteenth National Freedom Day")
            .collect();
        assert_eq!(found.len(), 1, "{year}");
        assert_eq!(found[0].date, ymd(year, 6, day), "{year}");
    }
}
