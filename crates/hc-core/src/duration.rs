//! An exact span of SI seconds.

use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub, SubAssign};

use crate::error::{TimeError, TimeResult};

/// Attoseconds in one second: 10^18.
pub const ATTOS_PER_SEC: u64 = 1_000_000_000_000_000_000;

const ATTOS_PER_SEC_I128: i128 = ATTOS_PER_SEC as i128;

/// An exact, signed span of SI seconds with attosecond resolution.
///
/// The representation is a *floor* decomposition: `value = secs + attos·10⁻¹⁸`
/// with `0 ≤ attos < 10¹⁸`, so `-0.5 s` is `secs = -1, attos = 5·10¹⁷`. That
/// choice makes comparison a plain lexicographic tuple comparison and keeps
/// `Ord` consistent with arithmetic.
///
/// # Why attoseconds, and why `i128`
///
/// An `i128` count of seconds spans roughly 4·10²⁰ times the age of the
/// universe, so no calendar or cosmological question runs out of range, while
/// 10⁻¹⁸ s resolves anything an optical clock can measure. Spans shorter than
/// an attosecond — Planck time, for instance — are *not* exact quantities in
/// any physical sense; they live in
/// [`hc-deep-time`](https://docs.rs/hc-deep-time) as floating-point
/// magnitudes with explicit uncertainty instead of being forced into this
/// type.
///
/// ```
/// use hc_core::Duration;
///
/// let a = Duration::from_secs(3);
/// let b = Duration::from_millis(-500);
/// assert_eq!((a + b).to_string(), "2.5");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Duration {
    secs: i128,
    attos: u64,
}

impl Duration {
    /// The zero span.
    pub const ZERO: Self = Self { secs: 0, attos: 0 };

    /// One second.
    pub const SECOND: Self = Self { secs: 1, attos: 0 };

    /// One minute (60 SI seconds).
    pub const MINUTE: Self = Self { secs: 60, attos: 0 };

    /// One hour (3600 SI seconds).
    pub const HOUR: Self = Self {
        secs: 3_600,
        attos: 0,
    };

    /// One mean solar day as used by civil calendars (86 400 SI seconds).
    ///
    /// This is the *nominal* day of the SI-second world, not the varying
    /// rotation period of the Earth, which UT1 follows (see `hc-astro`).
    pub const DAY: Self = Self {
        secs: 86_400,
        attos: 0,
    };

    /// The largest representable span.
    pub const MAX: Self = Self {
        secs: i128::MAX,
        attos: ATTOS_PER_SEC - 1,
    };

    /// The smallest (most negative) representable span.
    pub const MIN: Self = Self {
        secs: i128::MIN,
        attos: 0,
    };

    /// Build a span from a whole number of seconds plus a sub-second part.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::OutOfRange`] when `attos` is not a valid
    /// sub-second remainder.
    pub const fn new(secs: i128, attos: u64) -> TimeResult<Self> {
        if attos >= ATTOS_PER_SEC {
            return Err(TimeError::OutOfRange);
        }
        Ok(Self { secs, attos })
    }

    /// The whole-second floor of the span.
    #[must_use]
    pub const fn whole_seconds(self) -> i128 {
        self.secs
    }

    /// The sub-second remainder, always in `[0, 10¹⁸)`.
    #[must_use]
    pub const fn subsec_attos(self) -> u64 {
        self.attos
    }

    /// A span of whole seconds.
    #[must_use]
    pub const fn from_secs(secs: i128) -> Self {
        Self { secs, attos: 0 }
    }

    /// A span of whole minutes.
    #[must_use]
    pub const fn from_minutes(minutes: i64) -> Self {
        Self::from_secs(minutes as i128 * 60)
    }

    /// A span of whole hours.
    #[must_use]
    pub const fn from_hours(hours: i64) -> Self {
        Self::from_secs(hours as i128 * 3_600)
    }

    /// A span of whole 86 400-second days.
    #[must_use]
    pub const fn from_days(days: i64) -> Self {
        Self::from_secs(days as i128 * 86_400)
    }

    /// A span of whole seven-day weeks, each day 86 400 seconds.
    #[must_use]
    pub const fn from_weeks(weeks: i64) -> Self {
        Self::from_secs(weeks as i128 * 604_800)
    }

    /// A span given in attoseconds.
    #[must_use]
    pub const fn from_attos(attos: i128) -> Self {
        let secs = attos.div_euclid(ATTOS_PER_SEC_I128);
        let rem = attos.rem_euclid(ATTOS_PER_SEC_I128);
        Self {
            secs,
            attos: rem as u64,
        }
    }

    /// A span given in nanoseconds.
    #[must_use]
    pub const fn from_nanos(nanos: i128) -> Self {
        Self::from_attos(nanos * 1_000_000_000)
    }

    /// A span given in microseconds.
    #[must_use]
    pub const fn from_micros(micros: i128) -> Self {
        Self::from_attos(micros * 1_000_000_000_000)
    }

    /// A span given in milliseconds.
    #[must_use]
    pub const fn from_millis(millis: i128) -> Self {
        Self::from_attos(millis * 1_000_000_000_000_000)
    }

    /// The total span in attoseconds.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::Overflow`] when the span exceeds ±2¹²⁷ as, which
    /// is about ±5.4 × 10¹² years — five trillion, roughly four hundred
    /// times the age of the universe.
    pub const fn total_attos(self) -> TimeResult<i128> {
        match self.secs.checked_mul(ATTOS_PER_SEC_I128) {
            Some(scaled) => match scaled.checked_add(self.attos as i128) {
                Some(total) => Ok(total),
                None => Err(TimeError::Overflow),
            },
            None => Err(TimeError::Overflow),
        }
    }

    /// The span as a count of seconds, rounding towards negative infinity for
    /// the sub-second part.
    #[must_use]
    pub const fn floor_seconds(self) -> i128 {
        self.secs
    }

    /// Approximate the span as seconds in `f64`.
    ///
    /// Precision degrades for very large spans; this is a convenience for
    /// astronomy and physics code, not a lossless conversion.
    #[must_use]
    pub fn as_secs_f64(self) -> f64 {
        self.secs as f64 + (self.attos as f64) / (ATTOS_PER_SEC as f64)
    }

    /// Approximate the span as 86 400-second days in `f64`.
    #[must_use]
    pub fn as_days_f64(self) -> f64 {
        self.as_secs_f64() / 86_400.0
    }

    /// Build a span from a `f64` count of seconds.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::NotFinite`] for NaN or infinity and
    /// [`TimeError::Overflow`] when the value does not fit.
    pub fn from_secs_f64(seconds: f64) -> TimeResult<Self> {
        if !seconds.is_finite() {
            return Err(TimeError::NotFinite);
        }
        let whole = crate::math::floor(seconds);
        if whole.abs() >= 1.7e38 {
            return Err(TimeError::Overflow);
        }
        let frac = seconds - whole;
        let attos = (frac * ATTOS_PER_SEC as f64) as u64;
        let attos = if attos >= ATTOS_PER_SEC {
            ATTOS_PER_SEC - 1
        } else {
            attos
        };
        Ok(Self {
            secs: whole as i128,
            attos,
        })
    }

    /// Build a span from a `f64` count of 86 400-second days.
    ///
    /// # Errors
    ///
    /// See [`Duration::from_secs_f64`].
    pub fn from_days_f64(days: f64) -> TimeResult<Self> {
        Self::from_secs_f64(days * 86_400.0)
    }

    /// Whether the span is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.secs == 0 && self.attos == 0
    }

    /// Whether the span points backwards in time.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.secs < 0
    }

    /// Addition that reports overflow instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::Overflow`] when the result is unrepresentable.
    pub const fn checked_add(self, other: Self) -> TimeResult<Self> {
        let mut attos = self.attos + other.attos;
        let mut carry = 0i128;
        if attos >= ATTOS_PER_SEC {
            attos -= ATTOS_PER_SEC;
            carry = 1;
        }
        match self.secs.checked_add(other.secs) {
            Some(secs) => match secs.checked_add(carry) {
                Some(secs) => Ok(Self { secs, attos }),
                None => Err(TimeError::Overflow),
            },
            None => Err(TimeError::Overflow),
        }
    }

    /// Subtraction that reports overflow instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::Overflow`] when the result is unrepresentable.
    pub const fn checked_sub(self, other: Self) -> TimeResult<Self> {
        match other.checked_neg() {
            Ok(negated) => self.checked_add(negated),
            Err(error) => Err(error),
        }
    }

    /// Negation that reports overflow instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::Overflow`] for [`Duration::MIN`].
    pub const fn checked_neg(self) -> TimeResult<Self> {
        if self.attos == 0 {
            match self.secs.checked_neg() {
                Some(secs) => Ok(Self { secs, attos: 0 }),
                None => Err(TimeError::Overflow),
            }
        } else {
            match self.secs.checked_neg() {
                Some(secs) => match secs.checked_sub(1) {
                    Some(secs) => Ok(Self {
                        secs,
                        attos: ATTOS_PER_SEC - self.attos,
                    }),
                    None => Err(TimeError::Overflow),
                },
                None => Err(TimeError::Overflow),
            }
        }
    }

    /// Multiplication by an integer factor.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::Overflow`] when the result is unrepresentable.
    pub const fn checked_mul_int(self, factor: i64) -> TimeResult<Self> {
        let factor = factor as i128;
        let attos_product = self.attos as i128 * factor;
        let carry = attos_product.div_euclid(ATTOS_PER_SEC_I128);
        let attos = attos_product.rem_euclid(ATTOS_PER_SEC_I128) as u64;
        match self.secs.checked_mul(factor) {
            Some(secs) => match secs.checked_add(carry) {
                Some(secs) => Ok(Self { secs, attos }),
                None => Err(TimeError::Overflow),
            },
            None => Err(TimeError::Overflow),
        }
    }

    /// Division by an integer divisor, truncating towards negative infinity.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::DivideByZero`] for a zero divisor and
    /// [`TimeError::Overflow`] when the span is too long to express in
    /// attoseconds.
    pub const fn checked_div_int(self, divisor: i64) -> TimeResult<Self> {
        if divisor == 0 {
            return Err(TimeError::DivideByZero);
        }
        match self.total_attos() {
            Ok(total) => Ok(Self::from_attos(total.div_euclid(divisor as i128))),
            Err(error) => Err(error),
        }
    }

    /// How many whole times `divisor` fits into the span, rounding towards
    /// negative infinity: Python's `timedelta // timedelta`.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::DivideByZero`] for a zero divisor and
    /// [`TimeError::Overflow`] when either span is too long to express in
    /// attoseconds.
    pub const fn checked_div_floor(self, divisor: Self) -> TimeResult<i128> {
        match self.checked_div_rem(divisor) {
            Ok((quotient, _)) => Ok(quotient),
            Err(error) => Err(error),
        }
    }

    /// What is left after taking whole `divisor`s out of the span, with the
    /// sign of the divisor: Python's `timedelta % timedelta`.
    ///
    /// # Errors
    ///
    /// As [`Duration::checked_div_floor`].
    pub const fn checked_rem(self, divisor: Self) -> TimeResult<Self> {
        match self.checked_div_rem(divisor) {
            Ok((_, remainder)) => Ok(remainder),
            Err(error) => Err(error),
        }
    }

    /// The floor quotient and the remainder together: Python's
    /// `divmod(timedelta, timedelta)`.
    ///
    /// The remainder has the sign of the divisor and is smaller than it in
    /// magnitude, so `quotient × divisor + remainder` is the span exactly.
    ///
    /// # Errors
    ///
    /// As [`Duration::checked_div_floor`].
    pub const fn checked_div_rem(self, divisor: Self) -> TimeResult<(i128, Self)> {
        if divisor.is_zero() {
            return Err(TimeError::DivideByZero);
        }
        let numerator = match self.total_attos() {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let denominator = match divisor.total_attos() {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        // `div_euclid` keeps the remainder non-negative. Python's floor
        // division keeps it with the sign of the divisor, which for a negative
        // divisor and an inexact division is one quotient step further down.
        let mut quotient = match numerator.checked_div_euclid(denominator) {
            Some(value) => value,
            None => return Err(TimeError::Overflow),
        };
        let mut remainder = numerator.rem_euclid(denominator);
        if denominator < 0 && remainder != 0 {
            quotient -= 1;
            remainder += denominator;
        }
        Ok((quotient, Self::from_attos(remainder)))
    }

    /// Split the span into Python's normalised `timedelta` fields: whole
    /// 86 400-second days (rounded towards negative infinity), the seconds
    /// left in `[0, 86 400)`, and the attoseconds left in `[0, 10¹⁸)`.
    ///
    /// `timedelta(microseconds=-1)` is `(-1, 86 399, 999 999 µs)`, and so is
    /// this.
    #[must_use]
    pub const fn days_seconds_attos(self) -> (i128, u32, u64) {
        (
            self.secs.div_euclid(86_400),
            self.secs.rem_euclid(86_400) as u32,
            self.attos,
        )
    }

    /// A [`fmt::Display`] view of the span as Python writes a `timedelta`:
    /// `[D day[s], ][H]H:MM:SS[.UUUUUU]`.
    ///
    /// The day count is the floor, so the clock part is never negative:
    /// minus five hours is `-1 day, 19:00:00`. A fraction is written with six
    /// digits, as Python writes microseconds, and with more only when the
    /// span has a part finer than a microsecond, which a Python `timedelta`
    /// cannot.
    ///
    /// ```
    /// use hc_core::Duration;
    ///
    /// assert_eq!(Duration::from_hours(-5).days_and_clock().to_string(), "-1 day, 19:00:00");
    /// ```
    #[must_use]
    pub const fn days_and_clock(self) -> DaysAndClock {
        DaysAndClock(self)
    }

    /// The absolute value of the span.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::Overflow`] for [`Duration::MIN`].
    pub const fn checked_abs(self) -> TimeResult<Self> {
        if self.is_negative() {
            self.checked_neg()
        } else {
            Ok(self)
        }
    }

    /// The ratio `self / other` as `f64`.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::DivideByZero`] when `other` is zero.
    pub fn ratio(self, other: Self) -> TimeResult<f64> {
        if other.is_zero() {
            return Err(TimeError::DivideByZero);
        }
        Ok(self.as_secs_f64() / other.as_secs_f64())
    }

    /// Scale the span by a real factor.
    ///
    /// The result is only as accurate as `f64` allows; use
    /// [`Duration::checked_mul_int`] when the factor is exact.
    ///
    /// # Errors
    ///
    /// See [`Duration::from_secs_f64`].
    pub fn scale_f64(self, factor: f64) -> TimeResult<Self> {
        Self::from_secs_f64(self.as_secs_f64() * factor)
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.secs
            .cmp(&other.secs)
            .then_with(|| self.attos.cmp(&other.attos))
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Duration {
    type Output = Self;

    /// # Panics
    ///
    /// Panics on overflow. Use [`Duration::checked_add`] to handle it.
    fn add(self, other: Self) -> Self {
        match self.checked_add(other) {
            Ok(value) => value,
            Err(_) => panic!("hc-core: duration addition overflowed"),
        }
    }
}

impl Sub for Duration {
    type Output = Self;

    /// # Panics
    ///
    /// Panics on overflow. Use [`Duration::checked_sub`] to handle it.
    fn sub(self, other: Self) -> Self {
        match self.checked_sub(other) {
            Ok(value) => value,
            Err(_) => panic!("hc-core: duration subtraction overflowed"),
        }
    }
}

impl Neg for Duration {
    type Output = Self;

    /// # Panics
    ///
    /// Panics for [`Duration::MIN`]. Use [`Duration::checked_neg`].
    fn neg(self) -> Self {
        match self.checked_neg() {
            Ok(value) => value,
            Err(_) => panic!("hc-core: duration negation overflowed"),
        }
    }
}

impl Mul<i64> for Duration {
    type Output = Self;

    /// # Panics
    ///
    /// Panics on overflow. Use [`Duration::checked_mul_int`] to handle it.
    fn mul(self, factor: i64) -> Self {
        match self.checked_mul_int(factor) {
            Ok(value) => value,
            Err(_) => panic!("hc-core: duration multiplication overflowed"),
        }
    }
}

impl Mul<Duration> for i64 {
    type Output = Duration;

    /// # Panics
    ///
    /// Panics on overflow. Use [`Duration::checked_mul_int`] to handle it.
    fn mul(self, span: Duration) -> Duration {
        span * self
    }
}

impl Div<i64> for Duration {
    type Output = Self;

    /// Division rounding towards negative infinity at the attosecond.
    ///
    /// # Panics
    ///
    /// Panics on a zero divisor, as integer division does, and when the span
    /// is too long to express in attoseconds. Use
    /// [`Duration::checked_div_int`] to handle either.
    fn div(self, divisor: i64) -> Self {
        match self.checked_div_int(divisor) {
            Ok(value) => value,
            Err(TimeError::DivideByZero) => panic!("hc-core: duration divided by zero"),
            Err(_) => panic!("hc-core: duration division overflowed"),
        }
    }
}

impl Rem for Duration {
    type Output = Self;

    /// The remainder with the sign of the divisor, as Python's `%` on two
    /// `timedelta`s.
    ///
    /// # Panics
    ///
    /// Panics on a zero divisor and when either span is too long to express
    /// in attoseconds. Use [`Duration::checked_rem`] to handle either.
    fn rem(self, divisor: Self) -> Self {
        match self.checked_rem(divisor) {
            Ok(value) => value,
            Err(TimeError::DivideByZero) => panic!("hc-core: duration divided by zero"),
            Err(_) => panic!("hc-core: duration remainder overflowed"),
        }
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl fmt::Display for Duration {
    /// Renders the span as a decimal count of seconds with no trailing zeros.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let negative = self.is_negative();
        let (whole, attos) = if negative {
            if self.attos == 0 {
                (self.secs.unsigned_abs(), 0)
            } else {
                ((self.secs + 1).unsigned_abs(), ATTOS_PER_SEC - self.attos)
            }
        } else {
            (self.secs.unsigned_abs(), self.attos)
        };
        if negative {
            f.write_str("-")?;
        }
        write!(f, "{whole}")?;
        if attos != 0 {
            let mut digits = [0u8; 18];
            let mut remainder = attos;
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

/// A span written as Python writes a `timedelta`; see
/// [`Duration::days_and_clock`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DaysAndClock(Duration);

impl fmt::Display for DaysAndClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (days, seconds, attos) = self.0.days_seconds_attos();
        if days != 0 {
            let unit = if days == 1 || days == -1 {
                "day"
            } else {
                "days"
            };
            write!(f, "{days} {unit}, ")?;
        }
        write!(
            f,
            "{}:{:02}:{:02}",
            seconds / 3_600,
            seconds % 3_600 / 60,
            seconds % 60
        )?;
        if attos != 0 {
            // Six digits always, as Python's microseconds; more only for a
            // part finer than a microsecond.
            let mut digits = [0u8; 18];
            let mut remainder = attos;
            for slot in digits.iter_mut().rev() {
                *slot = b'0' + (remainder % 10) as u8;
                remainder /= 10;
            }
            let mut end = digits.len();
            while end > 6 && digits[end - 1] == b'0' {
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

#[cfg(test)]
mod tests {
    use super::*;
    /// The range claims in this module's doc comment, pinned.
    ///
    /// Both limits are easy to misstate by many orders of magnitude, and
    /// other crates rely on them when deciding whether `total_attos` is safe
    /// to call. A number that justifies a decision should be a test.
    #[test]
    fn the_attosecond_total_reaches_trillions_of_years_not_units_of_them() {
        // The Julian year astronomy counts in.
        const YEAR: i128 = 31_557_600;
        // 2^127 attoseconds is 1.70×10^20 s, which is 5.39×10^12 years — so
        // five trillion fits and six does not.
        assert!(
            Duration::from_secs(5_000_000_000_000 * YEAR)
                .total_attos()
                .is_ok(),
            "five trillion years should fit in an attosecond total"
        );
        assert!(
            Duration::from_secs(6_000_000_000_000 * YEAR)
                .total_attos()
                .is_err(),
            "six trillion years should not"
        );
        // And a mere million years, which an earlier test called "well past
        // the range", is nowhere near it.
        assert!(Duration::from_secs(1_000_000 * YEAR).total_attos().is_ok());
    }

    /// `i128` seconds against the age of the universe, about 4.35×10^17 s.
    #[test]
    fn the_second_count_spans_about_four_hundred_quintillion_universe_ages() {
        const UNIVERSE_AGE_SECS: i128 = 435_084_600_000_000_000;
        let ratio = i128::MAX / UNIVERSE_AGE_SECS;
        assert!(
            (3 * 10_i128.pow(20)..5 * 10_i128.pow(20)).contains(&ratio),
            "expected about 4e20, got {ratio}"
        );
    }

    #[cfg(not(feature = "std"))]
    use alloc::string::ToString as _;

    #[test]
    fn normalizes_negative_sub_second_values() {
        let half_back = Duration::from_millis(-500);
        assert_eq!(half_back.whole_seconds(), -1);
        assert_eq!(half_back.subsec_attos(), ATTOS_PER_SEC / 2);
    }

    #[test]
    fn addition_carries_across_the_second_boundary() {
        let a = Duration::from_millis(600);
        let b = Duration::from_millis(700);
        let sum = a + b;
        assert_eq!(sum.whole_seconds(), 1);
        assert_eq!(sum.subsec_attos(), 300_000_000_000_000_000);
    }

    #[test]
    fn negation_round_trips() {
        for millis in [-1_500i128, -1, 0, 1, 999, 86_400_000] {
            let value = Duration::from_millis(millis);
            assert_eq!(-(-value), value);
        }
    }

    #[test]
    fn ordering_matches_numeric_order() {
        let mut values = [
            Duration::from_millis(1),
            Duration::from_millis(-1),
            Duration::ZERO,
            Duration::from_secs(2),
        ];
        values.sort();
        assert_eq!(values[0], Duration::from_millis(-1));
        assert_eq!(values[1], Duration::ZERO);
        assert_eq!(values[2], Duration::from_millis(1));
        assert_eq!(values[3], Duration::from_secs(2));
    }

    #[test]
    fn display_trims_trailing_zeros() {
        assert_eq!(Duration::from_secs(5).to_string(), "5");
        assert_eq!(Duration::from_millis(1_500).to_string(), "1.5");
        assert_eq!(Duration::from_millis(-1_500).to_string(), "-1.5");
        assert_eq!(Duration::from_attos(1).to_string(), "0.000000000000000001");
        assert_eq!(Duration::from_millis(-500).to_string(), "-0.5");
    }

    #[test]
    fn multiplication_and_division_are_inverse_for_exact_factors() {
        let value = Duration::from_millis(1_234);
        let scaled = value.checked_mul_int(7).unwrap();
        assert_eq!(scaled.checked_div_int(7).unwrap(), value);
    }

    #[test]
    fn division_by_zero_is_reported() {
        assert_eq!(
            Duration::SECOND.checked_div_int(0),
            Err(TimeError::DivideByZero)
        );
    }

    #[test]
    fn float_round_trip_is_stable_for_typical_values() {
        let value = Duration::from_secs_f64(1234.5).unwrap();
        assert!((value.as_secs_f64() - 1234.5).abs() < 1e-9);
    }

    /// Python's documentation, `timedelta`:
    ///
    /// ```text
    /// >>> d = dt.timedelta(microseconds=-1)
    /// >>> (d.days, d.seconds, d.microseconds)
    /// (-1, 86399, 999999)
    /// ```
    #[test]
    fn the_normalised_fields_match_python_for_a_negative_microsecond() {
        let (days, seconds, attos) = Duration::from_micros(-1).days_seconds_attos();
        assert_eq!(
            (days, seconds, attos / 1_000_000_000_000),
            (-1, 86_399, 999_999)
        );
    }

    /// Python's documentation, the "common bug" note:
    ///
    /// ```text
    /// >>> duration = dt.timedelta(seconds=11235813)
    /// >>> duration.days, duration.seconds
    /// (130, 3813)
    /// ```
    #[test]
    fn the_seconds_field_is_the_remainder_after_whole_days() {
        let (days, seconds, _) = Duration::from_secs(11_235_813).days_seconds_attos();
        assert_eq!((days, seconds), (130, 3_813));
    }

    /// Python's documentation:
    ///
    /// ```text
    /// >>> timedelta(hours=-5)
    /// datetime.timedelta(days=-1, seconds=68400)
    /// >>> print(_)
    /// -1 day, 19:00:00
    /// ```
    #[test]
    fn the_python_string_form_floors_the_days() {
        let render = |span: Duration| span.days_and_clock().to_string();
        assert_eq!(render(Duration::from_hours(-5)), "-1 day, 19:00:00");
        assert_eq!(render(Duration::ZERO), "0:00:00");
        assert_eq!(render(Duration::DAY), "1 day, 0:00:00");
        assert_eq!(render(Duration::from_days(-2)), "-2 days, 0:00:00");
        assert_eq!(
            render(
                Duration::from_days(64) + Duration::from_secs(29_156) + Duration::from_micros(10)
            ),
            "64 days, 8:05:56.000010"
        );
        // Finer than a microsecond, which Python cannot hold, is written out
        // rather than dropped.
        assert_eq!(render(Duration::from_nanos(1_500)), "0:00:00.0000015");
    }

    /// Python's documentation:
    ///
    /// ```text
    /// >>> year = dt.timedelta(days=365)
    /// >>> ten_years = 10 * year
    /// >>> ten_years
    /// datetime.timedelta(days=3650)
    /// >>> nine_years = ten_years - year
    /// >>> three_years = nine_years // 3
    /// >>> three_years, three_years.days // 365
    /// (datetime.timedelta(days=1095), 3)
    /// ```
    #[test]
    fn integer_multiplication_and_division_follow_the_python_example() {
        let year = Duration::from_days(365);
        let ten_years = 10 * year;
        assert_eq!(ten_years, Duration::from_days(3_650));
        let nine_years = ten_years - year;
        assert_eq!(nine_years, Duration::from_days(3_285));
        let three_years = nine_years / 3;
        assert_eq!(three_years, Duration::from_days(1_095));
        assert_eq!(three_years.checked_div_floor(year), Ok(3));
        assert_eq!(year * 2, Duration::from_days(730));
    }

    /// `divmod(timedelta(days=1), timedelta(hours=1))` is `(24, timedelta(0))`,
    /// and floor division by a negative divisor leaves a remainder with the
    /// divisor's sign, as Python's `//` and `%` do: `7 s // -2 s` is `-4` and
    /// `7 s % -2 s` is `-1 s`.
    #[test]
    fn floor_division_and_remainder_follow_python_signs() {
        assert_eq!(
            Duration::DAY.checked_div_rem(Duration::HOUR),
            Ok((24, Duration::ZERO))
        );
        let seven = Duration::from_secs(7);
        let two = Duration::from_secs(2);
        assert_eq!(
            seven.checked_div_rem(-two),
            Ok((-4, Duration::from_secs(-1)))
        );
        assert_eq!((-seven).checked_div_rem(two), Ok((-4, Duration::SECOND)));
        assert_eq!(
            (-seven).checked_div_rem(-two),
            Ok((3, Duration::from_secs(-1)))
        );
        assert_eq!(seven.checked_div_rem(two), Ok((3, Duration::SECOND)));
        assert_eq!(seven % -two, Duration::from_secs(-1));
        assert_eq!(
            Duration::from_secs(6).checked_div_rem(-two),
            Ok((-3, Duration::ZERO))
        );
        assert_eq!(
            seven.checked_rem(Duration::ZERO),
            Err(TimeError::DivideByZero)
        );
        assert_eq!(
            seven.checked_div_floor(Duration::ZERO),
            Err(TimeError::DivideByZero)
        );
    }

    #[test]
    fn a_week_is_seven_days() {
        assert_eq!(Duration::from_weeks(2), Duration::from_days(14));
    }

    #[test]
    #[should_panic(expected = "duration divided by zero")]
    fn the_division_operator_panics_on_a_zero_divisor() {
        let _ = Duration::SECOND / 0;
    }

    #[test]
    #[should_panic(expected = "duration multiplication overflowed")]
    fn the_multiplication_operator_panics_on_overflow() {
        let _ = Duration::MAX * 2;
    }

    #[test]
    fn rejects_invalid_sub_second_remainders() {
        assert_eq!(Duration::new(0, ATTOS_PER_SEC), Err(TimeError::OutOfRange));
    }
}
