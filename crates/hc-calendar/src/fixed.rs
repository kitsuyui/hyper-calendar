//! Rata Die: the fixed day number every calendar converts through.

use core::fmt;
use core::ops::{Add, Sub};

use hc_core::{Duration, Instant, Tai};

use crate::error::{CalendarError, CalendarResult};

/// A fixed day number. Day 1 is `0001-01-01` in the proleptic Gregorian
/// calendar; day 0 is the day before it.
///
/// The number counts *days*, not seconds, and says nothing about time of day.
/// That is deliberate: a calendar answers "which day is this", and a day is
/// the largest unit every calendar agrees on.
///
/// ```
/// use hc_calendar::Rd;
///
/// // The Julian Day Number of the Rata Die epoch.
/// assert_eq!(Rd(1).to_julian_day_number(), 1_721_426);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Rd(pub i64);

/// The Julian Day Number of `Rd(0)`, i.e. of `0000-12-31` proleptic
/// Gregorian.
pub const JDN_OF_RD_ZERO: i64 = 1_721_425;

/// The Rata Die of the POSIX epoch, `1970-01-01`.
pub const RD_OF_UNIX_EPOCH: i64 = 719_163;

impl Rd {
    /// The Rata Die of `1970-01-01`.
    pub const UNIX_EPOCH: Self = Self(RD_OF_UNIX_EPOCH);

    /// The day count as a plain integer.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Days elapsed between two fixed days.
    #[must_use]
    pub const fn days_since(self, earlier: Self) -> i64 {
        self.0 - earlier.0
    }

    /// Move forward by a whole number of days.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::Overflow`] when the result is
    /// unrepresentable.
    pub const fn checked_add_days(self, days: i64) -> CalendarResult<Self> {
        match self.0.checked_add(days) {
            Some(value) => Ok(Self(value)),
            None => Err(CalendarError::Overflow),
        }
    }

    /// The Julian Day Number of this day.
    #[must_use]
    pub const fn to_julian_day_number(self) -> i64 {
        self.0 + JDN_OF_RD_ZERO
    }

    /// The fixed day for a Julian Day Number.
    #[must_use]
    pub const fn from_julian_day_number(jdn: i64) -> Self {
        Self(jdn - JDN_OF_RD_ZERO)
    }

    /// The Modified Julian Date of this day.
    #[must_use]
    pub const fn to_modified_julian_day(self) -> i64 {
        self.to_julian_day_number() - 2_400_001
    }

    /// The fixed day for a Modified Julian Date.
    #[must_use]
    pub const fn from_modified_julian_day(mjd: i64) -> Self {
        Self::from_julian_day_number(mjd + 2_400_001)
    }

    /// Whole days since the POSIX epoch.
    #[must_use]
    pub const fn to_unix_days(self) -> i64 {
        self.0 - RD_OF_UNIX_EPOCH
    }

    /// The fixed day for a whole-day count since the POSIX epoch.
    #[must_use]
    pub const fn from_unix_days(days: i64) -> Self {
        Self(days + RD_OF_UNIX_EPOCH)
    }
}

impl Add<i64> for Rd {
    type Output = Self;

    fn add(self, days: i64) -> Self {
        Self(self.0 + days)
    }
}

impl Sub<i64> for Rd {
    type Output = Self;

    fn sub(self, days: i64) -> Self {
        Self(self.0 - days)
    }
}

impl Sub for Rd {
    type Output = i64;

    fn sub(self, other: Self) -> i64 {
        self.0 - other.0
    }
}

impl fmt::Display for Rd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RD {}", self.0)
    }
}

/// A "moment": a fixed day plus a fraction of that day, as astronomical
/// algorithms use it.
///
/// Astronomical formulae are stated in fractional days, so lunisolar
/// calendars and solar-term calculations need this bridge. The fraction is
/// measured from local midnight of `rd`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Moment(pub f64);

impl Moment {
    /// The fixed day containing this moment.
    #[must_use]
    pub fn day(self) -> Rd {
        Rd(hc_core::math::floor(self.0) as i64)
    }

    /// The fraction of the day elapsed, in `[0, 1)`.
    #[must_use]
    pub fn day_fraction(self) -> f64 {
        self.0 - hc_core::math::floor(self.0)
    }

    /// This moment as a Julian Date (which starts at noon, not midnight).
    #[must_use]
    pub fn to_julian_date(self) -> f64 {
        self.0 + JDN_OF_RD_ZERO as f64 - 0.5
    }

    /// A moment from a Julian Date.
    #[must_use]
    pub fn from_julian_date(jd: f64) -> Self {
        Self(jd - JDN_OF_RD_ZERO as f64 + 0.5)
    }
}

/// The fixed day containing a moment.
#[must_use]
pub fn moment_to_rd(moment: Moment) -> Rd {
    moment.day()
}

/// A moment at midnight of a fixed day.
#[must_use]
pub fn rd_to_moment(rd: Rd) -> Moment {
    Moment(rd.0 as f64)
}

/// The fixed day and time-of-day offset of a TAI instant, treating the TAI
/// reading as though it were a uniform 86 400-second day count.
///
/// This is the raw, leap-second-free mapping. Civil code should go through
/// [`hc_core::unix`] first so that leap seconds are handled.
#[must_use]
pub fn rd_from_tai_naive(instant: Instant<Tai>) -> (Rd, Duration) {
    let seconds = instant.since_epoch();
    let day = seconds.whole_seconds().div_euclid(86_400);
    let within = seconds.whole_seconds().rem_euclid(86_400);
    (
        Rd(day as i64 + RD_OF_UNIX_EPOCH),
        Duration::from_attos(
            within * hc_core::ATTOS_PER_SEC as i128 + seconds.subsec_attos() as i128,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_unix_epoch_sits_where_the_standard_says() {
        assert_eq!(Rd::UNIX_EPOCH.to_julian_day_number(), 2_440_588);
        assert_eq!(Rd::UNIX_EPOCH.to_modified_julian_day(), 40_587);
        assert_eq!(Rd::UNIX_EPOCH.to_unix_days(), 0);
    }

    #[test]
    fn julian_day_numbers_round_trip() {
        for day in [-100_000i64, -1, 0, 1, 719_163, 1_000_000] {
            let rd = Rd(day);
            assert_eq!(Rd::from_julian_day_number(rd.to_julian_day_number()), rd);
            assert_eq!(
                Rd::from_modified_julian_day(rd.to_modified_julian_day()),
                rd
            );
            assert_eq!(Rd::from_unix_days(rd.to_unix_days()), rd);
        }
    }

    #[test]
    fn day_arithmetic_is_plain_integer_arithmetic() {
        assert_eq!(Rd(10) + 5, Rd(15));
        assert_eq!(Rd(10) - 5, Rd(5));
        assert_eq!(Rd(15) - Rd(10), 5);
        assert_eq!(Rd(10).days_since(Rd(3)), 7);
    }

    #[test]
    fn overflow_is_reported_rather_than_wrapped() {
        assert_eq!(
            Rd(i64::MAX).checked_add_days(1),
            Err(CalendarError::Overflow)
        );
    }

    #[test]
    fn moments_split_into_day_and_fraction() {
        let moment = Moment(719_163.25);
        assert_eq!(moment.day(), Rd::UNIX_EPOCH);
        assert!((moment.day_fraction() - 0.25).abs() < 1e-9);
    }

    #[test]
    fn julian_dates_round_trip_through_moments() {
        let moment = Moment(719_163.25);
        let jd = moment.to_julian_date();
        assert!((Moment::from_julian_date(jd).0 - moment.0).abs() < 1e-9);
        // A Julian Date starts at noon, so 1970-01-01T06:00 UT is JD
        // 2440587.75 and midnight that morning is JD 2440587.5.
        assert!((jd - 2_440_587.75).abs() < 1e-9, "jd was {jd}");
        assert!((Moment(719_163.0).to_julian_date() - 2_440_587.5).abs() < 1e-9);
    }

    #[test]
    fn negative_moments_floor_towards_the_past() {
        let moment = Moment(-0.25);
        assert_eq!(moment.day(), Rd(-1));
        assert!((moment.day_fraction() - 0.75).abs() < 1e-9);
    }

    #[test]
    fn tai_instants_map_onto_fixed_days() {
        let (rd, within) = rd_from_tai_naive(Instant::<Tai>::from_epoch(Duration::from_secs(0)));
        assert_eq!(rd, Rd::UNIX_EPOCH);
        assert_eq!(within, Duration::ZERO);

        let (rd, within) = rd_from_tai_naive(Instant::<Tai>::from_epoch(Duration::from_secs(-1)));
        assert_eq!(rd, Rd(RD_OF_UNIX_EPOCH - 1));
        assert_eq!(within, Duration::from_secs(86_399));
    }
}
