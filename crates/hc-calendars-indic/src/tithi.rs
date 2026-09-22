//! The tithi: the lunar day of the Hindu reckonings.
//!
//! A tithi is the time the Moon takes to gain twelve degrees of longitude
//! on the Sun — a thirtieth of the synodic month, from about 0.9 to 1.1
//! civil days because the Moon's speed varies. Thirty of them make a lunar
//! month: fifteen of the bright fortnight, *śukla pakṣa*, from the new
//! moon, and fifteen of the dark, *kṛṣṇa pakṣa*, from the full moon to the
//! next new moon, the thirtieth being *amāvāsyā* itself.
//!
//! A civil day takes the tithi in progress at its sunrise. Because a tithi
//! and a day are not the same length, a tithi that begins just after one
//! sunrise and ends just before the next never holds a sunrise and is
//! *skipped* (*kṣaya tithi*), and one that holds two sunrises is *repeated*
//! (*adhika tithi*). Both are ordinary and both are carried: a date names
//! the tithi its day began with, and whether it is the second such day.

use hc_astro::lunar::lunar_longitude;
use hc_astro::riseset::{Location, sunrise};
use hc_astro::solar::solar_longitude;
use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::{floor, normalize_degrees};

/// The Moon's gain on the Sun over one tithi, in degrees.
pub const DEGREES_PER_TITHI: f64 = 12.0;

/// The number of tithis in a lunar month.
pub const TITHIS_PER_MONTH: u8 = 30;

/// The two fortnights of a lunar month.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Paksha {
    /// The bright fortnight, new moon to full moon: tithis 1 to 15.
    Shukla,
    /// The dark fortnight, full moon to new moon: tithis 16 to 30.
    Krishna,
}

/// The tithi in progress at a moment, as a real number: the integer part is
/// the tithi's number, 1 to 30, and the fraction how far through it the
/// Moon has run.
#[must_use]
pub fn tithi_at(moment: Moment) -> f64 {
    let elongation = normalize_degrees(lunar_longitude(moment) - solar_longitude(moment));
    elongation / DEGREES_PER_TITHI + 1.0
}

/// The number, 1 to 30, of the tithi in progress at a moment.
#[must_use]
pub fn tithi_number_at(moment: Moment) -> u8 {
    let number = floor(tithi_at(moment)) as u8;
    if number > TITHIS_PER_MONTH {
        TITHIS_PER_MONTH
    } else {
        number
    }
}

/// The fortnight and the day within it, 1 to 15, of a tithi number.
#[must_use]
pub const fn paksha_of(tithi: u8) -> (Paksha, u8) {
    if tithi <= 15 {
        (Paksha::Shukla, tithi)
    } else {
        (Paksha::Krishna, tithi - 15)
    }
}

/// Sunrise on a day at a location, in Universal Time — the instant a day's
/// tithi is read at.
///
/// Every place this crate names sees a sunrise every day; should a caller
/// bring a polar one, six o'clock local mean time stands in, so that a day
/// still has a tithi.
#[must_use]
pub fn sunrise_of(day: Rd, location: Location) -> Moment {
    sunrise(day, location).unwrap_or(Moment(
        day.0 as f64 + 0.25 - location.longitude_degrees / 360.0,
    ))
}

/// The tithi a day carries: the one in progress at its sunrise.
#[must_use]
pub fn tithi_of_day(day: Rd, location: Location) -> u8 {
    tithi_number_at(sunrise_of(day, location))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::places::UJJAIN;
    use hc_astro::lunar::new_moon_at_or_after;

    #[test]
    fn the_tithi_restarts_at_every_new_moon() {
        let mut moon = new_moon_at_or_after(Moment(730_120.0));
        for _ in 0..24 {
            // Just before the conjunction the thirtieth tithi is ending; just
            // after it the first has begun.
            assert_eq!(tithi_number_at(Moment(moon.0 - 0.01)), 30);
            assert_eq!(tithi_number_at(Moment(moon.0 + 0.01)), 1);
            moon = new_moon_at_or_after(Moment(moon.0 + 1.0));
        }
    }

    #[test]
    fn a_month_of_days_runs_through_the_tithis_in_order_with_at_most_one_skip_or_repeat() {
        let moon = new_moon_at_or_after(Moment(738_000.0));
        // The month's first day is the first whose sunrise follows the
        // conjunction.
        let mut start = moon.day();
        if sunrise_of(start, UJJAIN).0 <= moon.0 {
            start = Rd(start.0 + 1);
        }
        let mut previous = tithi_of_day(start, UJJAIN);
        assert_eq!(previous, 1);
        let mut skips = 0;
        let mut repeats = 0;
        for offset in 1..29 {
            let current = tithi_of_day(Rd(start.0 + offset), UJJAIN);
            match current as i16 - previous as i16 {
                0 => repeats += 1,
                1 => {}
                2 => skips += 1,
                gap => panic!("tithi jumped by {gap} on day {offset}"),
            }
            previous = current;
        }
        assert!(skips + repeats <= 2, "{skips} skips and {repeats} repeats");
    }

    #[test]
    fn the_fortnights_split_at_the_full_moon() {
        assert_eq!(paksha_of(1), (Paksha::Shukla, 1));
        assert_eq!(paksha_of(15), (Paksha::Shukla, 15));
        assert_eq!(paksha_of(16), (Paksha::Krishna, 1));
        assert_eq!(paksha_of(30), (Paksha::Krishna, 15));
    }
}
