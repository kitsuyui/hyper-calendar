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
    ///
    /// # Panics
    ///
    /// Panics when the difference does not fit in an `i64`. Use
    /// [`Rd::checked_days_since`] to handle it.
    #[must_use]
    pub const fn days_since(self, earlier: Self) -> i64 {
        match self.checked_days_since(earlier) {
            Ok(days) => days,
            Err(_) => panic!("hc-calendar: day difference overflowed"),
        }
    }

    /// Days elapsed between two fixed days, reporting overflow instead of
    /// panicking.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::Overflow`] when the difference does not fit
    /// in an `i64`.
    pub const fn checked_days_since(self, earlier: Self) -> CalendarResult<i64> {
        match self.0.checked_sub(earlier.0) {
            Some(days) => Ok(days),
            None => Err(CalendarError::Overflow),
        }
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

    /// Move back by a whole number of days.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::Overflow`] when the result is
    /// unrepresentable.
    pub const fn checked_sub_days(self, days: i64) -> CalendarResult<Self> {
        match self.0.checked_sub(days) {
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

    /// # Panics
    ///
    /// Panics on overflow, in release builds as well as debug ones, as
    /// [`Duration`]'s operators do. Use [`Rd::checked_add_days`] to handle it.
    fn add(self, days: i64) -> Self {
        match self.checked_add_days(days) {
            Ok(value) => value,
            Err(_) => panic!("hc-calendar: day addition overflowed"),
        }
    }
}

impl Sub<i64> for Rd {
    type Output = Self;

    /// # Panics
    ///
    /// Panics on overflow. Use [`Rd::checked_sub_days`] to handle it.
    fn sub(self, days: i64) -> Self {
        match self.checked_sub_days(days) {
            Ok(value) => value,
            Err(_) => panic!("hc-calendar: day subtraction overflowed"),
        }
    }
}

impl Sub for Rd {
    type Output = i64;

    /// # Panics
    ///
    /// Panics on overflow, as [`Rd::days_since`] does. Use
    /// [`Rd::checked_days_since`] to handle it.
    fn sub(self, other: Self) -> i64 {
        self.days_since(other)
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
    fn day_arithmetic_reports_overflow_where_it_is_checked() {
        assert_eq!(
            Rd(i64::MAX).checked_add_days(1),
            Err(CalendarError::Overflow)
        );
        assert_eq!(
            Rd(i64::MIN).checked_sub_days(1),
            Err(CalendarError::Overflow)
        );
        assert_eq!(
            Rd(i64::MAX).checked_days_since(Rd(-1)),
            Err(CalendarError::Overflow)
        );
        assert_eq!(Rd(10).checked_sub_days(3), Ok(Rd(7)));
        assert_eq!(Rd(10).checked_days_since(Rd(3)), Ok(7));
        assert_eq!(Rd(10) - Rd(3), 7);
        assert_eq!(Rd(10) - 3, Rd(7));
        assert_eq!(Rd(10) + 3, Rd(13));
    }

    #[test]
    #[should_panic(expected = "day addition overflowed")]
    fn the_addition_operator_panics_on_overflow() {
        let _ = Rd(i64::MAX) + 1;
    }

    #[test]
    #[should_panic(expected = "day subtraction overflowed")]
    fn the_subtraction_operator_panics_on_overflow() {
        let _ = Rd(i64::MIN) - 1;
    }

    #[test]
    #[should_panic(expected = "day difference overflowed")]
    fn the_difference_operator_panics_on_overflow() {
        let _ = Rd(i64::MIN) - Rd(1);
    }

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
