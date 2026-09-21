//! The Gregorian epact, checked by rebuilding Easter out of it.
//!
//! `hc_calendars_solar::cycles::gregorian_epact` carries the solar
//! equation, the lunar equation and Clavius's two exceptions at 24 and 25.
//! None of those can be checked by staring at them, and a test that
//! asserted the epact against a table of epacts would only be checking the
//! table.
//!
//! So this goes the long way round: take the epact, derive the paschal full
//! moon from it, take the Sunday after, and compare against
//! `hc_holiday::computus`, which computes Easter by Butcher's arrangement
//! and shares no line of code with the epact. If the two agree for every
//! year the computus is defined over, the epact is right — and if either
//! ever stops being right, this fails.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::cycles::{gregorian_epact, gregorian_paschal_moon_march_day};
use hc_calendars_solar::gregorian;
use hc_holiday::computus::{COMPUTUS_LAST_YEAR, GREGORIAN_COMPUTUS_FIRST_YEAR, gregorian_easter};

/// Easter rebuilt from the epact: the Sunday strictly after the paschal
/// full moon, where the moon is given as a day of March that may run past
/// 31 into April.
fn easter_from_epact(year: i64) -> Option<Rd> {
    let march_day = gregorian_paschal_moon_march_day(year);
    let moon = Rd(gregorian::to_fixed(year, 3, 1).ok()?.0 + march_day - 1);
    // Strictly after, so a full moon on a Sunday pushes Easter a week.
    Some(Weekday::Sunday.on_or_after(Rd(moon.0 + 1)))
}

#[test]
fn the_epact_rebuilds_every_easter_the_computus_computes() {
    let mut checked = 0u32;
    for year in GREGORIAN_COMPUTUS_FIRST_YEAR..=COMPUTUS_LAST_YEAR {
        let from_computus = gregorian_easter(year).expect("in range");
        let from_epact = easter_from_epact(year).expect("in range");
        assert_eq!(
            from_epact,
            from_computus,
            "{year}: epact {} gives {:?}, Butcher gives {:?}",
            gregorian_epact(year),
            gregorian::from_fixed(from_epact),
            gregorian::from_fixed(from_computus),
        );
        checked += 1;
    }
    // The whole range the computus claims, not a sample of it.
    assert_eq!(checked, 2_517);
}

#[test]
fn easter_never_falls_outside_the_window_the_reform_fixed() {
    // 22 March to 25 April, which is what the paschal moon bounds imply.
    for year in GREGORIAN_COMPUTUS_FIRST_YEAR..=COMPUTUS_LAST_YEAR {
        let (month, day) = gregorian::from_fixed(gregorian_easter(year).expect("in range"))
            .map(|(_, month, day)| (month, day))
            .expect("in range");
        let ok =
            (month == 3 && (22..=31).contains(&day)) || (month == 4 && (1..=25).contains(&day));
        assert!(ok, "{year} gave {month}-{day}");
    }
}

/// The golden number and the epact are the two halves of the lunar
/// reckoning, and the epact must move with the Metonic cycle.
#[test]
fn the_epact_repeats_with_the_metonic_cycle_inside_a_century() {
    // Within one century, so the solar and lunar equations are constant.
    for year in 1_901..1_981 {
        assert_eq!(
            gregorian_epact(year),
            gregorian_epact(year + 19),
            "{year} and {} share a golden number",
            year + 19
        );
    }
}

/// Across a century where the Gregorian rule drops a leap day, the epact
/// shifts — which is the solar equation doing its job.
#[test]
fn the_solar_equation_shifts_the_epact_at_a_dropped_century() {
    // 1900 is not a Gregorian leap year, so the epact moves relative to the
    // Julian reckoning that would have kept it.
    let before = gregorian_epact(1899);
    let after = gregorian_epact(1899 + 19);
    assert_ne!(
        before, after,
        "the epact should not repeat across the 1900 correction"
    );
}
