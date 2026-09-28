//! Andorra's parishes and Bolivia's departments, instrument by instrument.
//!
//! The rows are the dates the comuns' instruments for 2024 to 2026 and the
//! Bolivian decrees and laws give; `docs/systems/andorra-holidays.md` and
//! `docs/systems/bolivia-holidays.md` list what was read.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::{ANDORRA, BOLIVIA};
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::{Kind, RuleSet};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn own_on(table: &RuleSet, region: &str, year: i64, month: u8, day: u8) -> Vec<Holiday> {
    HolidayCalendar::for_year(table, Some(region), year)
        .on(ymd(year, month, day))
        .into_iter()
        .filter(|holiday| !holiday.regions.is_empty() && !holiday.is_substitute())
        .collect()
}

/// `(parish, local name, year, month, day)`.
const PARISH_DAYS: &[(&str, &str, i64, u8, u8)] = &[
    ("AD-02", "Sant Roc", 2024, 8, 16),
    ("AD-02", "Sant Roc", 2026, 8, 16),
    ("AD-04", "Sant Antoni", 2024, 1, 17),
    ("AD-05", "St. Pere", 2025, 6, 29),
    ("AD-06", "Sant Julià, Patró de la Parròquia", 2026, 1, 7),
    (
        "AD-06",
        "Diada de Canòlich, Patrona de la Parròquia",
        2025,
        5,
        31,
    ),
    ("AD-06", "Dilluns de Festa Major", 2026, 7, 27),
    ("AD-06", "Dimarts de Festa Major", 2024, 7, 30),
    ("AD-07", "Festa del Poble (Sant Joan)", 2026, 6, 24),
    ("AD-07", "Festa Major", 2025, 8, 2),
    ("AD-07", "Festa Major", 2025, 8, 4),
    ("AD-08", "Diada de Sant Miquel d'Engolasters", 2026, 5, 8),
    (
        "AD-08",
        "Diada commemorativa de la creació de la parròquia",
        2024,
        6,
        16,
    ),
    ("AD-08", "Sant Jaume (Festa Major)", 2026, 7, 25),
    ("AD-08", "Santa Anna (Festa Major)", 2025, 7, 26),
];

#[test]
fn every_parish_day_falls_where_its_comu_put_it() {
    for &(region, local_name, year, month, day) in PARISH_DAYS {
        let found = own_on(&ANDORRA, region, year, month, day);
        assert_eq!(found.len(), 1, "{region} {local_name} {year}: {found:?}");
        assert_eq!(found[0].local_name, local_name);
        assert_eq!(found[0].kind, Kind::Public);
        assert_eq!(found[0].regions, [region]);
        assert!(found[0].source.contains("BOPA"));
    }
    // Sant Miquel d'Engolasters was not a parish day in 2024.
    assert!(own_on(&ANDORRA, "AD-08", 2024, 5, 8).is_empty());
    // Encamp keeps no day in the whole parish, and none is nationwide.
    assert!(own_on(&ANDORRA, "AD-03", 2026, 8, 16).is_empty());
    assert!(
        HolidayCalendar::for_year(&ANDORRA, None, 2026)
            .on(ymd(2026, 8, 16))
            .is_empty()
    );
}

#[test]
fn the_parish_days_are_a_gap_outside_the_instruments_read() {
    // 2023's and 2027's instruments were not read; 2025's was, and its
    // parish days are all there.
    for region in ["AD-02", "AD-04", "AD-05", "AD-06", "AD-07", "AD-08"] {
        assert!(!HolidayCalendar::for_year(&ANDORRA, Some(region), 2023).is_complete());
        assert!(HolidayCalendar::for_year(&ANDORRA, Some(region), 2025).is_complete());
        assert!(!HolidayCalendar::for_year(&ANDORRA, Some(region), 2027).is_complete());
    }
    assert!(HolidayCalendar::for_year(&ANDORRA, None, 2027).is_complete());
}

#[test]
fn every_departmental_day_falls_where_its_instrument_puts_it() {
    for (region, local_name, year, month, day, kind) in [
        (
            "BO-L",
            "Feriado departamental de La Paz",
            2009,
            7,
            16,
            Kind::Public,
        ),
        (
            "BO-L",
            "Feriado departamental de La Paz",
            2026,
            7,
            16,
            Kind::Public,
        ),
        (
            "BO-O",
            "Efeméride Departamental de Oruro",
            2013,
            2,
            6,
            Kind::Public,
        ),
        (
            "BO-O",
            "Efeméride Departamental de Oruro",
            2014,
            2,
            10,
            Kind::Public,
        ),
        (
            "BO-T",
            "Efeméride del departamento de Tarija",
            2020,
            4,
            15,
            Kind::Public,
        ),
        ("BO-N", "Batalla de Bahía", 2025, 10, 11, Kind::Public),
        (
            "BO-S",
            "Día Departamental de la Autonomía",
            2011,
            5,
            4,
            Kind::Observance,
        ),
        (
            "BO-S",
            "Día Departamental de la Autonomía",
            2026,
            5,
            4,
            Kind::Observance,
        ),
    ] {
        let found = own_on(&BOLIVIA, region, year, month, day);
        assert_eq!(found.len(), 1, "{region} {year}: {found:?}");
        assert_eq!(found[0].local_name, local_name);
        assert_eq!(found[0].kind, kind);
        assert_eq!(found[0].regions, [region]);
    }
    // Before the instruments: no Oruro day on 10 February 2013, no Tarija
    // day in 2019, no Battle of Bahía in 2024.
    assert!(own_on(&BOLIVIA, "BO-O", 2013, 2, 10).is_empty());
    assert!(own_on(&BOLIVIA, "BO-T", 2019, 4, 15).is_empty());
    assert!(own_on(&BOLIVIA, "BO-N", 2024, 10, 11).is_empty());
    // La Paz's is La Paz's alone.
    assert!(own_on(&BOLIVIA, "BO-C", 2026, 7, 16).is_empty());
}

#[test]
fn before_its_instrument_a_departmental_day_is_absent_or_a_gap() {
    let gaps = |region, year| {
        HolidayCalendar::for_year(&BOLIVIA, Some(region), year)
            .gaps()
            .iter()
            .map(|gap| gap.local_name)
            .collect::<Vec<_>>()
    };
    // Pando's law of 2024 sets the Battle of Bahía: 2024 is absent.
    assert!(!gaps("BO-N", 2024).contains(&"Batalla de Bahía"));
    // Tarija's decree of 2020 presupposes the day: 2019 is a gap.
    assert!(gaps("BO-T", 2019).contains(&"Efeméride del departamento de Tarija"));
    assert!(gaps("BO-L", 2008).contains(&"Feriado departamental de La Paz"));
    assert!(gaps("BO-O", 2012).contains(&"Efeméride Departamental de Oruro"));
    assert!(HolidayCalendar::for_year(&BOLIVIA, None, 2008).is_complete());
}

#[test]
fn a_sunday_departmental_day_moves_to_monday_from_2024() {
    // Oruro's 10 February was a Sunday in 2019 and in 2030.
    let before = HolidayCalendar::for_year(&BOLIVIA, Some("BO-O"), 2019);
    assert!(before.on(ymd(2019, 2, 11)).is_empty());
    let after = HolidayCalendar::for_year(&BOLIVIA, Some("BO-O"), 2030);
    assert!(
        after
            .on(ymd(2030, 2, 11))
            .iter()
            .any(Holiday::is_substitute)
    );
}
