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
use hc_astro::riseset::{Location, sunrise, sunset};
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

/// Sunset on a day at a location, in Universal Time, with the same
/// stand-in as [`sunrise_of`] for a place that has none.
#[must_use]
pub fn sunset_of(day: Rd, location: Location) -> Moment {
    sunset(day, location).unwrap_or(Moment(
        day.0 as f64 + 0.75 - location.longitude_degrees / 360.0,
    ))
}

/// When in the day a tithi must be in progress for a festival to fall on
/// that day.
///
/// A civil date carries the tithi at sunrise, but a festival is kept on
/// the day its tithi holds the part of the day the rite belongs to: Rāma
/// Navamī at midday, Vijayā Daśamī in the afternoon, Dīpāvalī in the
/// evening, Janmāṣṭamī and Śivarātri at midnight. Each variant names the
/// instant it probes, in the day's own sunrise-to-sunrise terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Prevalence {
    /// At sunrise: the tithi the civil day carries.
    Sunrise,
    /// At midday, *madhyāhna*: the middle of the daylight.
    Midday,
    /// In the afternoon, *aparāhṇa*: the fourth fifth of the daylight,
    /// probed at its middle — seven tenths of the way from sunrise to
    /// sunset.
    Afternoon,
    /// In the evening, *pradoṣa*: the first two *muhūrta*s after sunset,
    /// probed one hour after it.
    Evening,
    /// At midnight, *niśīta*: the middle of the night that follows the
    /// day.
    Midnight,
}

impl Prevalence {
    /// The instant this prevalence probes on a day at a location, in
    /// Universal Time.
    #[must_use]
    pub fn moment_of(self, day: Rd, location: Location) -> Moment {
        self.moment_between(sunrise_of(day, location), sunset_of(day, location), || {
            sunrise_of(Rd(day.0 + 1), location)
        })
    }

    /// The instant this prevalence probes, from the day's sunrise and
    /// sunset and, asked for only at midnight, the next day's sunrise.
    ///
    /// [`Prevalence::moment_of`] finds the three for one day; a caller that
    /// reads many festivals at one place finds each sunrise once and hands
    /// them in here.
    #[must_use]
    pub fn moment_between(
        self,
        rise: Moment,
        set: Moment,
        next_rise: impl FnOnce() -> Moment,
    ) -> Moment {
        let (rise, set) = (rise.0, set.0);
        Moment(match self {
            Self::Sunrise => rise,
            Self::Midday => rise + (set - rise) / 2.0,
            Self::Afternoon => rise + (set - rise) * 0.7,
            Self::Evening => set + 1.0 / 24.0,
            Self::Midnight => {
                let next_rise = next_rise().0;
                set + (next_rise - set) / 2.0
            }
        })
    }

    /// The tithi in progress at this prevalence's instant on a day.
    #[must_use]
    pub fn tithi_on(self, day: Rd, location: Location) -> u8 {
        tithi_number_at(self.moment_of(day, location))
    }
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
    fn the_prevalences_run_through_the_day_in_order() {
        let day = Rd(738_800.0 as i64);
        let rise = Prevalence::Sunrise.moment_of(day, UJJAIN).0;
        let midday = Prevalence::Midday.moment_of(day, UJJAIN).0;
        let afternoon = Prevalence::Afternoon.moment_of(day, UJJAIN).0;
        let evening = Prevalence::Evening.moment_of(day, UJJAIN).0;
        let midnight = Prevalence::Midnight.moment_of(day, UJJAIN).0;
        assert!(rise < midday && midday < afternoon && afternoon < evening && evening < midnight);
        assert!(midnight < Prevalence::Sunrise.moment_of(Rd(day.0 + 1), UJJAIN).0);
        // Ujjain's midday is about 06:57 UT (12:27 local mean time), so the
        // fraction of the UT day is a little under 0.3.
        let fraction = midday - day.0 as f64;
        assert!((0.25..0.32).contains(&fraction), "{fraction}");
    }

    #[test]
    fn the_fortnights_split_at_the_full_moon() {
        assert_eq!(paksha_of(1), (Paksha::Shukla, 1));
        assert_eq!(paksha_of(15), (Paksha::Shukla, 15));
        assert_eq!(paksha_of(16), (Paksha::Krishna, 1));
        assert_eq!(paksha_of(30), (Paksha::Krishna, 15));
    }
}
