//! Every region a holiday table scopes a rule to must be a subdivision
//! CLDR 48 knows, and one in current use.
//!
//! `HolidayRule::regions` is a list of strings, so nothing stops a table
//! from scoping a day to a code ISO 3166-2 retired or never had: the day
//! would then answer to no caller who uses the current code. The place
//! names of `hc-i18n` carry CLDR's validity status for every subdivision,
//! and this file holds the tables to it.

#![cfg(all(feature = "holiday", feature = "place-names"))]

use hyper_calendar::hc_holiday::countries;
use hyper_calendar::hc_i18n::place_names::{self, Status};

/// Codes scoped to that are not ISO 3166-2 subdivisions in current use,
/// each for a reason: England and Wales as one jurisdiction, which ISO
/// 3166-2 splits; and Guatemala City's department under the code ISO
/// retired, which Guatemala's table still uses and should replace.
const EXCEPTIONS: &[&str] = &["GB-EAW", "GT-GU"];

#[test]
fn every_region_is_a_regular_subdivision_of_its_country() {
    for table in countries::ALL {
        for code in table.regions() {
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
