//! Spain's autonomous communities, resolution by resolution.
//!
//! Each row below is a day the Dirección General's resolution for that
//! year lists for one community and not nationwide, with its date as the
//! annex gives it. The rows are read from the BOE, not dates this crate
//! produced; `docs/systems/spain-holidays.md` lists the resolutions.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::SPAIN;
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::Kind;

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// The entries a region's calendar has on a day that are the region's
/// own, not nationwide.
fn own_entries(region: Option<&str>, year: i64, month: u8, day: u8) -> Vec<Holiday> {
    HolidayCalendar::for_year(&SPAIN, region, year)
        .on(ymd(year, month, day))
        .into_iter()
        .filter(|holiday| !holiday.regions.is_empty())
        .collect()
}

/// One day of each community in the first resolution read, for 2013,
/// and in the last, for 2026: `(region, year, month, day, local name)`.
const ANCHORS: &[(&str, i64, u8, u8, &str)] = &[
    ("ES-AN", 2013, 2, 28, "Día de Andalucía"),
    ("ES-AN", 2026, 2, 28, "Día de Andalucía"),
    ("ES-AR", 2013, 4, 23, "San Jorge/Día de Aragón"),
    ("ES-AR", 2026, 4, 23, "San Jorge/Día de Aragón"),
    ("ES-AS", 2013, 9, 9, "Lunes siguiente al Día de Asturias"),
    ("ES-AS", 2026, 9, 8, "Día de Asturias"),
    ("ES-CB", 2013, 4, 1, "Lunes de Pascua"),
    (
        "ES-CB",
        2026,
        7,
        28,
        "Día de las Instituciones de Cantabria",
    ),
    ("ES-CE", 2013, 10, 15, "Fiesta del Sacrificio-Eidul Adha"),
    ("ES-CE", 2026, 9, 2, "Día de Ceuta"),
    ("ES-CL", 2013, 4, 23, "Fiesta de Castilla y León"),
    ("ES-CL", 2026, 4, 23, "Fiesta de Castilla y León"),
    ("ES-CM", 2013, 5, 31, "Día de Castilla-La Mancha"),
    ("ES-CM", 2026, 4, 6, "Lunes de Pascua"),
    ("ES-CN", 2013, 5, 30, "Día de Canarias"),
    ("ES-CN", 2026, 5, 30, "Día de Canarias"),
    ("ES-CT", 2013, 9, 11, "Fiesta Nacional de Cataluña"),
    ("ES-CT", 2026, 9, 11, "Fiesta Nacional de Cataluña"),
    ("ES-EX", 2013, 9, 9, "Lunes siguiente al Día de Extremadura"),
    ("ES-EX", 2026, 9, 8, "Día de Extremadura"),
    ("ES-GA", 2013, 5, 17, "Día de las Letras Gallegas"),
    (
        "ES-GA",
        2026,
        7,
        25,
        "Santiago Apóstol/Día Nacional de Galicia",
    ),
    ("ES-IB", 2013, 3, 1, "Día de les Illes Balears"),
    (
        "ES-IB",
        2026,
        3,
        2,
        "Lunes siguiente al Día de les Illes Balears",
    ),
    ("ES-MC", 2013, 3, 19, "San José"),
    ("ES-MC", 2026, 6, 9, "Día de la Región de Murcia"),
    ("ES-MD", 2013, 5, 2, "Fiesta de la Comunidad de Madrid"),
    ("ES-MD", 2026, 5, 2, "Fiesta de la Comunidad de Madrid"),
    ("ES-ML", 2013, 10, 15, "Fiesta del Sacrificio-Aid Al Adha"),
    ("ES-ML", 2026, 3, 20, "Fiesta del Eid Fitr"),
    ("ES-NC", 2013, 4, 1, "Lunes de Pascua"),
    ("ES-NC", 2026, 4, 6, "Lunes de Pascua"),
    ("ES-PV", 2013, 10, 25, "Día del País Vasco-Euskadiko Eguna"),
    ("ES-PV", 2026, 4, 6, "Lunes de Pascua"),
    ("ES-RI", 2013, 6, 10, "Día de La Rioja"),
    ("ES-RI", 2026, 6, 9, "Día de La Rioja"),
    ("ES-VC", 2013, 10, 9, "Día de la Comunitat Valenciana"),
    ("ES-VC", 2026, 10, 9, "Día de la Comunitat Valenciana"),
];

/// The nineteen: seventeen communities and two cities.
const COMMUNITIES: &[&str] = &[
    "ES-AN", "ES-AR", "ES-AS", "ES-CB", "ES-CE", "ES-CL", "ES-CM", "ES-CN", "ES-CT", "ES-EX",
    "ES-GA", "ES-IB", "ES-MC", "ES-MD", "ES-ML", "ES-NC", "ES-PV", "ES-RI", "ES-VC",
];

#[test]
fn every_community_keeps_its_days_in_the_first_and_last_resolution_read() {
    for &(region, year, month, day, local_name) in ANCHORS {
        let found = own_entries(Some(region), year, month, day);
        assert_eq!(found.len(), 1, "{region} {year}: {found:?}");
        let entry = found[0];
        assert_eq!(entry.local_name, local_name, "{region} {year}");
        assert_eq!(entry.kind, Kind::Public, "{region} {year}");
        assert_eq!(entry.regions, [region], "{region} {year}");
        let resolution = if year < 2016 {
            "BOE-A-2012-13644"
        } else {
            "BOE-A-2025-21667"
        };
        assert!(entry.source.contains(resolution), "{region} {year}");
    }
}

#[test]
fn the_years_whose_resolution_was_not_read_are_gaps() {
    // 2016 and 2017 publish the annex as an image; 2027 is not published;
    // the resolutions before 2013 were not read.
    for &region in COMMUNITIES {
        for year in [2012, 2016, 2017, 2027] {
            let calendar = HolidayCalendar::for_year(&SPAIN, Some(region), year);
            assert!(!calendar.is_complete(), "{region} {year}");
            assert!(
                calendar
                    .in_year(year)
                    .iter()
                    .all(|holiday| holiday.regions.is_empty()),
                "{region} {year}: {:?}",
                calendar.in_year(year)
            );
        }
        for year in [2013, 2015, 2018, 2026] {
            let calendar = HolidayCalendar::for_year(&SPAIN, Some(region), year);
            assert!(calendar.is_complete(), "{region} {year}");
        }
    }
    // A day a read resolution does not list is absent, not a gap: Ceuta's
    // own day first appears in 2019, and 2015 has neither the day nor a
    // gap for it; 2012 has the gap.
    let ceuta_gap = |year| {
        HolidayCalendar::for_year(&SPAIN, Some("ES-CE"), year)
            .gaps()
            .iter()
            .any(|gap| gap.local_name == "Día de Ceuta")
    };
    assert!(!ceuta_gap(2015));
    assert!(ceuta_gap(2012));
    // The nationwide days are rules and have no gap.
    assert!(HolidayCalendar::for_year(&SPAIN, None, 2012).is_complete());
    assert!(HolidayCalendar::for_year(&SPAIN, None, 2016).is_complete());
    assert!(HolidayCalendar::for_year(&SPAIN, None, 2030).is_complete());
}

#[test]
fn a_community_day_belongs_to_its_community_alone() {
    // Andalusia's 28 February is neither Madrid's nor nationwide.
    assert_eq!(own_entries(Some("ES-AN"), 2026, 2, 28).len(), 1);
    assert!(own_entries(Some("ES-MD"), 2026, 2, 28).is_empty());
    assert!(own_entries(None, 2026, 2, 28).is_empty());
    // Maundy Thursday, 2 April 2026, is kept by every community but
    // Catalonia and the Valencian Community, which replaced it.
    for &region in COMMUNITIES {
        let kept = !own_entries(Some(region), 2026, 4, 2).is_empty();
        assert_eq!(kept, !["ES-CT", "ES-VC"].contains(&region), "{region}");
    }
    // The region is matched as every identifier is.
    assert_eq!(own_entries(Some("es-md"), 2026, 5, 2).len(), 1);
}

#[test]
fn a_community_day_is_a_day_off_for_business_days() {
    let madrid = HolidayCalendar::for_year(&SPAIN, Some("ES-MD"), 2026);
    let catalonia = HolidayCalendar::for_year(&SPAIN, Some("ES-CT"), 2026);
    // Thursday 2 April 2026: Maundy Thursday in Madrid, a working day in
    // Catalonia; Monday 6 April the reverse.
    assert!(!madrid.is_business_day(ymd(2026, 4, 2)));
    assert!(catalonia.is_business_day(ymd(2026, 4, 2)));
    assert!(madrid.is_business_day(ymd(2026, 4, 6)));
    assert!(!catalonia.is_business_day(ymd(2026, 4, 6)));
}

#[test]
fn the_monday_rest_of_a_sunday_holiday_is_each_community_s_choice() {
    // 6 December 2026 is a Sunday. Andalusia rests on Monday the 7th;
    // Catalonia does not.
    assert_eq!(
        own_entries(Some("ES-AN"), 2026, 12, 7)[0].local_name,
        "Lunes siguiente al Día de la Constitución Española"
    );
    assert!(own_entries(Some("ES-CT"), 2026, 12, 7).is_empty());
}

#[test]
fn the_table_names_every_community() {
    assert_eq!(SPAIN.regions(), COMMUNITIES);
}
