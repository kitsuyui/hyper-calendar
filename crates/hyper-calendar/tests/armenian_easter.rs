//! The Armenian Great Era began with the 532-year Easter table: its first
//! year's "Christmas and Easter fell on 6 January and 20 April 553"
//! (Armenian Wikipedia, «Հայոց մեծ թվական», `hywiki-hayots-mets-tvakan`,
//! read 2026-09-29, secondary). The Julian computus `hc-holiday` carries is
//! the Alexandrian table, which repeats every 532 years, so the era's first
//! Easter is its Easter of 553, and the Easter of 1085, the first year after
//! the table's first cycle, falls on the same day. `docs/systems/armenian.md`
//! says why no Armenian computus of its own is registered.

#![cfg(all(feature = "civil", feature = "holiday"))]
#![expect(
    clippy::expect_used,
    reason = "a date that does not convert is a failed test, and the message says which"
)]

use hyper_calendar::hc_calendar::Rd;
use hyper_calendar::hc_calendars_solar::{armenian, julian};
use hyper_calendar::hc_holiday::computus::Computus;

fn julian_day(year: i64, month: u8, day: u8) -> Rd {
    julian::to_fixed(year, month, day).expect("a Julian date")
}

#[test]
fn the_first_easter_of_the_great_era_is_the_julian_computus() {
    let easter = Computus::JULIAN.easter(553).expect("in range");
    assert_eq!(easter, julian_day(553, 4, 20));
    // 20 April 553 is in Armenian year 1, which began on 11 July 552.
    let (year, _, _) = armenian::from_fixed(easter).expect("in range");
    assert_eq!(year, 1);
    assert_eq!(
        Computus::JULIAN.easter(553 + 532),
        Some(julian_day(1085, 4, 20))
    );
    // The table repeats every 532 years.
    for year in 553..553 + 532 {
        let date = |year| {
            let easter = Computus::JULIAN.easter(year).expect("in range");
            julian::from_fixed(easter).expect("a Julian date")
        };
        let ((_, month, day), (_, later_month, later_day)) = (date(year), date(year + 532));
        assert_eq!((month, day), (later_month, later_day), "{year}");
    }
}
