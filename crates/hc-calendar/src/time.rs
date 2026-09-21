//! Civil time of day, and a calendar day paired with one.

use core::fmt;

use hc_core::{ATTOS_PER_SEC, Duration};

use crate::error::{CalendarError, CalendarResult};
use crate::fixed::Rd;

/// A time of day on a 24-hour civil clock.
///
/// `second` may be 60: UTC really does have a `23:59:60`, and a type that
/// cannot represent it forces every caller to invent a workaround. Whether a
/// given `23:59:60` exists is a question for the leap-second table, not for
/// this type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CivilTime {
    hour: u8,
    minute: u8,
    second: u8,
    subsec_attos: u64,
}

impl CivilTime {
    /// Midnight.
    pub const MIDNIGHT: Self = Self {
        hour: 0,
        minute: 0,
        second: 0,
        subsec_attos: 0,
    };

    /// Noon.
    pub const NOON: Self = Self {
        hour: 12,
        minute: 0,
        second: 0,
        subsec_attos: 0,
    };

    /// Build a time of day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when any field is outside its
    /// range. `second` may be 60 only when `hour` is 23 and `minute` is 59.
    pub const fn new(hour: u8, minute: u8, second: u8, subsec_attos: u64) -> CalendarResult<Self> {
        if hour > 23 || minute > 59 || subsec_attos >= ATTOS_PER_SEC {
            return Err(CalendarError::DayOutOfRange);
        }
        if second > 59 && !(second == 60 && hour == 23 && minute == 59) {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(Self {
            hour,
            minute,
            second,
            subsec_attos,
        })
    }

    /// Build a time of day from whole hours, minutes and seconds.
    ///
    /// # Errors
    ///
    /// See [`CivilTime::new`].
    pub const fn hms(hour: u8, minute: u8, second: u8) -> CalendarResult<Self> {
        Self::new(hour, minute, second, 0)
    }

    /// The hour, 0 through 23.
    #[must_use]
    pub const fn hour(self) -> u8 {
        self.hour
    }

    /// The minute, 0 through 59.
    #[must_use]
    pub const fn minute(self) -> u8 {
        self.minute
    }

    /// The second, 0 through 60.
    #[must_use]
    pub const fn second(self) -> u8 {
        self.second
    }

    /// The sub-second remainder in attoseconds.
    #[must_use]
    pub const fn subsec_attos(self) -> u64 {
        self.subsec_attos
    }

    /// Whether this names an inserted leap second.
    #[must_use]
    pub const fn is_leap_second(self) -> bool {
        self.second == 60
    }

    /// The elapsed time since midnight.
    #[must_use]
    pub const fn since_midnight(self) -> Duration {
        let seconds = self.hour as i128 * 3_600 + self.minute as i128 * 60 + self.second as i128;
        Duration::from_attos(seconds * ATTOS_PER_SEC as i128 + self.subsec_attos as i128)
    }

    /// Build a time of day from an elapsed span since midnight.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when the span does not fall
    /// within a single 86 401-second day.
    pub fn from_midnight_offset(offset: Duration) -> CalendarResult<Self> {
        let total = offset.whole_seconds();
        if !(0..86_401).contains(&total) {
            return Err(CalendarError::DayOutOfRange);
        }
        let hour = (total / 3_600) as u8;
        let minute = ((total % 3_600) / 60) as u8;
        let second = (total % 60) as u8;
        if total == 86_400 {
            return Self::new(23, 59, 60, offset.subsec_attos());
        }
        Self::new(hour, minute, second, offset.subsec_attos())
    }

    /// The fraction of the day elapsed, in `[0, 1)`.
    #[must_use]
    pub fn day_fraction(self) -> f64 {
        self.since_midnight().as_secs_f64() / 86_400.0
    }

    /// A time of day from a fraction of a day.
    ///
    /// # Errors
    ///
    /// See [`CivilTime::from_midnight_offset`].
    pub fn from_day_fraction(fraction: f64) -> CalendarResult<Self> {
        let offset = Duration::from_secs_f64(fraction * 86_400.0)?;
        Self::from_midnight_offset(offset)
    }
}

impl fmt::Display for CivilTime {
    /// Renders as `HH:MM:SS` with a fractional part only when non-zero.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)?;
        if self.subsec_attos != 0 {
            let mut digits = [0u8; 18];
            let mut remainder = self.subsec_attos;
            for slot in digits.iter_mut().rev() {
                *slot = b'0' + (remainder % 10) as u8;
                remainder /= 10;
            }
            let mut end = digits.len();
            while end > 1 && digits[end - 1] == b'0' {
                end -= 1;
            }
            f.write_str(".")?;
            for digit in &digits[..end] {
                f.write_fmt(format_args!("{}", *digit as char))?;
            }
        }
        Ok(())
    }
}

/// A calendar day paired with a time of day.
///
/// This is *local* civil time: it carries no zone and no scale. Attaching a
/// zone is `hc-tz`'s job, and turning it into a physical instant needs both a
/// zone and a leap-second policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CivilDateTime {
    /// The day.
    pub day: Rd,
    /// The time within that day.
    pub time: CivilTime,
}

impl CivilDateTime {
    /// Pair a day with a time.
    #[must_use]
    pub const fn new(day: Rd, time: CivilTime) -> Self {
        Self { day, time }
    }

    /// Midnight at the start of a day.
    #[must_use]
    pub const fn midnight(day: Rd) -> Self {
        Self {
            day,
            time: CivilTime::MIDNIGHT,
        }
    }

    /// The elapsed time from midnight of `origin` to this moment, counting
    /// every day as 86 400 seconds.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::Overflow`] when the span is unrepresentable.
    pub fn nominal_duration_since(self, origin: Self) -> CalendarResult<Duration> {
        let days = self.day.days_since(origin.day);
        let base = Duration::from_days(days);
        let span = base
            .checked_add(self.time.since_midnight())?
            .checked_sub(origin.time.since_midnight())?;
        Ok(span)
    }
}

impl fmt::Display for CivilDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.day, self.time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midnight_and_noon_are_where_expected() {
        assert_eq!(CivilTime::MIDNIGHT.since_midnight(), Duration::ZERO);
        assert_eq!(
            CivilTime::NOON.since_midnight(),
            Duration::from_secs(43_200)
        );
    }

    #[test]
    fn out_of_range_fields_are_rejected() {
        assert!(CivilTime::hms(24, 0, 0).is_err());
        assert!(CivilTime::hms(0, 60, 0).is_err());
        assert!(CivilTime::hms(12, 0, 60).is_err());
        assert!(CivilTime::new(0, 0, 0, ATTOS_PER_SEC).is_err());
    }

    #[test]
    fn the_leap_second_is_representable_only_where_it_occurs() {
        let leap = CivilTime::hms(23, 59, 60).unwrap();
        assert!(leap.is_leap_second());
        assert_eq!(leap.to_string(), "23:59:60");
        assert!(CivilTime::hms(23, 58, 60).is_err());
    }

    #[test]
    fn times_round_trip_through_midnight_offsets() {
        for (hour, minute, second) in [(0u8, 0u8, 0u8), (12, 34, 56), (23, 59, 59)] {
            let time = CivilTime::hms(hour, minute, second).unwrap();
            let offset = time.since_midnight();
            assert_eq!(CivilTime::from_midnight_offset(offset).unwrap(), time);
        }
    }

    #[test]
    fn the_eighty_six_thousand_four_hundredth_second_is_the_leap_second() {
        let time = CivilTime::from_midnight_offset(Duration::from_secs(86_400)).unwrap();
        assert_eq!(time, CivilTime::hms(23, 59, 60).unwrap());
    }

    #[test]
    fn day_fractions_round_trip() {
        let time = CivilTime::hms(6, 0, 0).unwrap();
        assert!((time.day_fraction() - 0.25).abs() < 1e-12);
        assert_eq!(CivilTime::from_day_fraction(0.25).unwrap(), time);
    }

    #[test]
    fn sub_second_digits_render_without_trailing_zeros() {
        let time = CivilTime::new(1, 2, 3, 500_000_000_000_000_000).unwrap();
        assert_eq!(time.to_string(), "01:02:03.5");
    }

    #[test]
    fn nominal_spans_count_every_day_as_86400_seconds() {
        let start = CivilDateTime::midnight(Rd(1));
        let end = CivilDateTime::new(Rd(3), CivilTime::NOON);
        assert_eq!(
            end.nominal_duration_since(start).unwrap(),
            Duration::from_secs(2 * 86_400 + 43_200)
        );
    }

    #[test]
    fn civil_date_times_order_by_day_then_time() {
        let earlier = CivilDateTime::new(Rd(1), CivilTime::NOON);
        let later = CivilDateTime::midnight(Rd(2));
        assert!(earlier < later);
    }
}
