//! Every feature that compiles a crate in must also give a way to reach it.
//!
//! A missing re-export is not a compile error anywhere, so a feature can
//! declare an optional dependency, appear in `full` and have no `pub use`:
//! enabling it pays the compile cost of a crate the caller has no path to.
//!
//! This file makes it one. Each check below names a real item in its crate, so the
//! test fails to compile if the re-export goes away, and fails to *link* if
//! the item is renamed. That is stronger than listing the module paths,
//! which would pass against an empty re-export.

#![expect(
    clippy::assertions_on_constants,
    reason = "the assertions are compile-time reachability checks; their \
              runtime value is incidental"
)]

#[test]
#[cfg(feature = "units")]
fn the_units_feature_reaches_its_crate() {
    use hyper_calendar::hc_units;
    assert_eq!(
        hc_units::unit::HOUR.seconds,
        hc_units::Ratio::from_secs(3600)
    );
    // And through the prelude, which is the path most callers take.
    use hyper_calendar::prelude::Unit;
    let _: Unit = hc_units::unit::SECOND;
}

#[test]
#[cfg(feature = "almanac")]
fn the_almanac_feature_reaches_its_crate() {
    use hyper_calendar::hc_almanac;
    let _ = hc_almanac::rokuyo::rokuyo(hc_almanac::Rd(738_886), hc_almanac::Meridian::JAPAN);
}

#[test]
#[cfg(feature = "fiscal")]
fn the_fiscal_feature_reaches_its_crate() {
    use hyper_calendar::hc_fiscal;
    // Japan's 年度, the example that put this crate in the workspace.
    assert!(!hc_fiscal::countries::ALL.is_empty());
}

#[test]
#[cfg(feature = "attributes")]
fn the_attributes_feature_reaches_its_crate() {
    use hyper_calendar::hc_attributes;
    assert!(hc_attributes::authority_count() > 0);
}

#[test]
#[cfg(feature = "name-days")]
fn the_name_days_feature_reaches_its_crate() {
    use hyper_calendar::hc_name_days;
    // Latvia's 1 January, the anchor that put this crate in the workspace.
    assert_eq!(
        hc_name_days::names_on(&hc_name_days::latvia::LV_TRADITIONAL_2026, 2026, 1, 1),
        Ok(&["Laimnesis", "Solvita", "Solvija"][..])
    );
}

#[test]
#[cfg(feature = "relativity")]
fn the_relativity_feature_reaches_its_crate() {
    use hyper_calendar::hc_relativity;
    assert!(hc_relativity::lorentz_factor(0.6).is_ok());
}

#[test]
#[cfg(feature = "deep-time")]
fn the_deep_time_feature_reaches_its_crate() {
    use hyper_calendar::hc_deep_time;
    assert!(hc_deep_time::GALACTIC_YEAR.julian_years > 0.0);
}

#[test]
#[cfg(feature = "orbital")]
fn the_orbital_feature_reaches_its_crate() {
    use hyper_calendar::hc_orbital;
    // The Last Glacial Maximum, the epoch the system document works through.
    let lgm = hc_orbital::elements_at(21_000.0).expect("inside the span");
    assert!((lgm.obliquity_degrees.value - 22.949).abs() < 1e-3);
}

/// The two crates that count years before a "present" count from the
/// same one, 1950, so a `Bp` from `hc-deep-time` is the argument
/// `hc-orbital` takes.
#[test]
#[cfg(all(feature = "orbital", feature = "deep-time"))]
fn the_orbital_epoch_is_the_radiocarbon_bp_datum() {
    use hyper_calendar::{hc_deep_time, hc_orbital};
    assert_eq!(
        hc_orbital::EPOCH_YEAR,
        hc_deep_time::archaeology::BP_DATUM_YEAR
    );
}
