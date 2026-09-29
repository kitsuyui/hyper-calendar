//! Every region a holiday table scopes a rule to must be a subdivision
//! CLDR 48 knows, and one in current use.
//!
//! `HolidayRule::regions` is a list of strings, so nothing stops a table
//! from scoping a day to a code ISO 3166-2 retired or never had: the day
//! would then answer to no caller who uses the current code. The place
//! names of `hc-i18n` carry CLDR's validity status for every subdivision,
//! and this file holds the tables to it. A municipality's code, which
//! CLDR does not name, is held to the subdivision it lies within.

#![cfg(all(feature = "holiday", feature = "place-names"))]

use hyper_calendar::hc_holiday::countries;
use hyper_calendar::hc_holiday::rule::region_parent;
use hyper_calendar::hc_i18n::place_names::{self, Status};

/// Codes scoped to that are not ISO 3166-2 subdivisions in current use,
/// each for a reason: England and Wales as one jurisdiction, which ISO
/// 3166-2 splits.
const EXCEPTIONS: &[&str] = &["GB-EAW"];

#[test]
fn every_region_is_a_regular_subdivision_of_its_country() {
    for table in countries::ALL {
        for region in table.regions() {
            // A municipality's code is held to its subdivision's (ADR 0014).
            let mut code = region;
            while let Some(parent) = region_parent(code) {
                code = parent;
            }
            if EXCEPTIONS.contains(&code) {
                continue;
            }
            let place = place_names::subdivision(code)
                .unwrap_or_else(|| panic!("{}: {code} is not a CLDR 48 subdivision", table.code));
            assert_eq!(
                place.status(),
                Status::Regular,
                "{}: {code} is not in current use",
                table.code
            );
            assert_eq!(place.country(), table.code, "{code}");
        }
    }
}
