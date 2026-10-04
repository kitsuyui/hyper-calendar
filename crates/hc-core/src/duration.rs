//! An exact span of SI seconds.

use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub, SubAssign};

use crate::error::{TimeError, TimeResult};
use crate::wide::U256;

/// Attoseconds in one second: 10^18.
pub const ATTOS_PER_SEC: u64 = 1_000_000_000_000_000_000;

const ATTOS_PER_SEC_I128: i128 = ATTOS_PER_SEC as i128;

/// Seconds in a nominal day, [`Duration::DAY`] as an integer: 86 400 with
/// no leap second, the day of POSIX time, of TAI and TT, and of every
/// astronomical series that counts days of uniform time.
pub const SECONDS_PER_DAY: i64 = 86_400;

/// [`SECONDS_PER_DAY`] as an `f64`, the divisor of a day fraction.
pub const SECONDS_PER_DAY_F64: f64 = SECONDS_PER_DAY as f64;

/// Split a count of whole seconds since an epoch into whole 86 400-second days
/// (rounded towards negative infinity) and the second of that day, in
/// `0..86 400`.
///
/// This is the one place the split of a POSIX timestamp into its day and its
/// time of day is written: `-1` is the last second of day `-1`, not the 86 399th
/// of day 0 as truncating division gives. It is the inverse of
/// [`seconds_from_days_and_seconds`].
///
/// ```
/// use hc_core::duration::days_and_seconds;
///
/// assert_eq!(days_and_seconds(0), (0, 0));
/// assert_eq!(days_and_seconds(86_399), (0, 86_399));
/// assert_eq!(days_and_seconds(-1), (-1, 86_399));
/// ```
#[inline]
#[must_use]
pub const fn days_and_seconds(seconds: i64) -> (i64, u32) {
    (
        seconds.div_euclid(SECONDS_PER_DAY),
        seconds.rem_euclid(SECONDS_PER_DAY) as u32,
    )
}

/// The seconds of a whole number of 86 400-second days, or `None` when they do
/// not fit an `i64`.
///
/// ```
/// use hc_core::duration::seconds_in_days;
///
/// assert_eq!(seconds_in_days(2), Some(172_800));
/// assert_eq!(seconds_in_days(i64::MAX), None);
/// ```
#[inline]
#[must_use]
pub const fn seconds_in_days(days: i64) -> Option<i64> {
    days.checked_mul(SECONDS_PER_DAY)
}

/// The count of seconds of a day and a second of that day: the inverse of
/// [`days_and_seconds`], or `None` when it does not fit an `i64`. A second
/// of day beyond `86 399` is carried into the following days, so `86 400` is
/// the first second of the next day, and is how a leap second `23:59:60`
/// reads.
///
/// ```
/// use hc_core::duration::seconds_from_days_and_seconds;
///
/// assert_eq!(seconds_from_days_and_seconds(1, 3_600), Some(90_000));
/// assert_eq!(seconds_from_days_and_seconds(-1, 86_399), Some(-1));
/// ```
#[inline]
#[must_use]
pub const fn seconds_from_days_and_seconds(days: i64, second_of_day: u32) -> Option<i64> {
    // The sum can fit where the start of the day does not: the part-day at
    // the bottom of the range starts below `i64::MIN`.
    let total = days as i128 * SECONDS_PER_DAY as i128 + second_of_day as i128;
    if total < i64::MIN as i128 || total > i64::MAX as i128 {
        None
    } else {
        Some(total as i64)
    }
}

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
        secs: SECONDS_PER_DAY as i128,
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
        self.as_secs_f64() / SECONDS_PER_DAY_F64
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
            self.secs.div_euclid(SECONDS_PER_DAY as i128),
            self.secs.rem_euclid(SECONDS_PER_DAY as i128) as u32,
            self.attos,
        )
    }

    /// The whole 86 400-second days of the span, rounded towards negative
    /// infinity, and the whole seconds left in `0..86 400`: the
    /// [`days_and_seconds`] of its floor second.
    #[must_use]
    pub const fn days_and_seconds(self) -> (i128, u32) {
        let (days, seconds, _) = self.days_seconds_attos();
        (days, seconds)
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

    /// The ratio `self / other` as `f64`, correctly rounded.
    ///
    /// Both spans are counted in attoseconds and divided as integers, so the
    /// result is the double nearest the exact quotient, which is what
    /// Python's `timedelta / timedelta` returns for whole microseconds. A
    /// span beyond ±2¹²⁷ attoseconds (about 5 × 10¹² years) has no such
    /// count, and the quotient is taken in `f64` instead.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::DivideByZero`] when `other` is zero.
    pub fn ratio(self, other: Self) -> TimeResult<f64> {
        if other.is_zero() {
            return Err(TimeError::DivideByZero);
        }
        if let (Ok(numerator), Ok(denominator)) = (self.total_attos(), other.total_attos()) {
            return Ok(ratio_of_integers(numerator, denominator));
        }
        Ok(self.as_secs_f64() / other.as_secs_f64())
    }

    /// Scale the span by a real factor, exactly.
    ///
    /// The factor is the rational number its double is, and the product is
    /// rounded once, to the attosecond, half to even. `0.5` of 25 508 964
    /// 008 398 µs is exactly 12 754 482 004 199 µs; the `f64` route through
    /// seconds this replaces did not say so.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::NotFinite`] for NaN or infinity and
    /// [`TimeError::Overflow`] when the product is beyond the range.
    pub fn scale_f64(self, factor: f64) -> TimeResult<Self> {
        self.scale_f64_nearest(factor, 1)
    }

    /// Scale by a real factor and round the product, half to even, to a
    /// multiple of `unit_attos` attoseconds: 1 for an exact span,
    /// 1 000 000 000 000 for Python's `timedelta * float`, which rounds to
    /// the microsecond.
    ///
    /// # Errors
    ///
    /// As [`Duration::scale_f64`]; [`TimeError::OutOfRange`] for a zero
    /// unit.
    pub fn scale_f64_nearest(self, factor: f64, unit_attos: u64) -> TimeResult<Self> {
        let (mantissa, exponent, negative) = decompose(factor)?;
        self.scale_ratio(mantissa, 1, exponent, negative, unit_attos)
    }

    /// Divide by a real divisor and round the quotient, half to even, to a
    /// multiple of `unit_attos` attoseconds: Python's `timedelta / float`
    /// rounds to the microsecond.
    ///
    /// # Errors
    ///
    /// As [`Duration::scale_f64_nearest`], and [`TimeError::DivideByZero`]
    /// for a zero divisor.
    pub fn div_f64_nearest(self, divisor: f64, unit_attos: u64) -> TimeResult<Self> {
        let (mantissa, exponent, negative) = decompose(divisor)?;
        if mantissa == 0 {
            return Err(TimeError::DivideByZero);
        }
        self.scale_ratio(1, mantissa, -exponent, negative, unit_attos)
    }

    /// Divide by an integer and round the quotient, half to even, to a
    /// multiple of `unit_attos` attoseconds: Python's `timedelta / int`
    /// rounds to the microsecond.
    ///
    /// # Errors
    ///
    /// [`TimeError::DivideByZero`] for a zero divisor,
    /// [`TimeError::OutOfRange`] for a zero unit, and
    /// [`TimeError::Overflow`] for a result beyond the range.
    pub fn div_int_nearest(self, divisor: i64, unit_attos: u64) -> TimeResult<Self> {
        if divisor == 0 {
            return Err(TimeError::DivideByZero);
        }
        self.scale_ratio(1, divisor.unsigned_abs(), 0, divisor < 0, unit_attos)
    }

    /// The span's size in attoseconds, as 256-bit magnitude, and its sign.
    fn magnitude_attos(self) -> (U256, bool) {
        let negative = self.secs < 0;
        let seconds = U256::from_u128(self.secs.unsigned_abs());
        // At most 2^127 * 10^18 < 2^187.
        let scaled = seconds.checked_mul_u64(ATTOS_PER_SEC).unwrap_or(U256::ZERO);
        let magnitude = if negative {
            scaled.sub_u64(self.attos)
        } else {
            scaled.checked_add_u64(self.attos).unwrap_or(U256::ZERO)
        };
        (magnitude, negative)
    }

    /// The span of `magnitude` attoseconds with the given sign.
    fn from_magnitude_attos(magnitude: U256, negative: bool) -> TimeResult<Self> {
        let (seconds, attos) = magnitude.divmod_u128(u128::from(ATTOS_PER_SEC));
        let seconds = seconds.to_u128().ok_or(TimeError::Overflow)?;
        let attos = attos as u64;
        if !negative {
            let secs = i128::try_from(seconds).map_err(|_| TimeError::Overflow)?;
            return Ok(Self { secs, attos });
        }
        if attos == 0 {
            // -2^127 is representable.
            let secs = if seconds == 1 << 127 {
                i128::MIN
            } else {
                -i128::try_from(seconds).map_err(|_| TimeError::Overflow)?
            };
            return Ok(Self { secs, attos: 0 });
        }
        let secs = -i128::try_from(seconds).map_err(|_| TimeError::Overflow)? - 1;
        Ok(Self {
            secs,
            attos: ATTOS_PER_SEC - attos,
        })
    }

    /// `self * numerator / denominator * 2^exponent`, rounded half to even
    /// to a multiple of `unit_attos` attoseconds, negated when
    /// `negative_factor`. Exact until the one rounding.
    fn scale_ratio(
        self,
        numerator: u64,
        denominator: u64,
        exponent: i32,
        negative_factor: bool,
        unit_attos: u64,
    ) -> TimeResult<Self> {
        if unit_attos == 0 {
            return Err(TimeError::OutOfRange);
        }
        let (magnitude, negative) = self.magnitude_attos();
        if magnitude.is_zero() || numerator == 0 {
            return Ok(Self::ZERO);
        }
        let mut product = magnitude
            .checked_mul_u64(numerator)
            .ok_or(TimeError::Overflow)?;
        if exponent > 0 {
            product = product
                .checked_shl(exponent.unsigned_abs())
                .ok_or(TimeError::Overflow)?;
        }
        let divisor = u128::from(denominator) * u128::from(unit_attos);
        let (mut quotient, remainder) = product.divmod_u128(divisor);
        let round_up = if exponent >= 0 {
            let twice = remainder * 2;
            twice > divisor || (twice == divisor && quotient.is_odd())
        } else {
            // The exact value is (quotient + remainder / divisor) / 2^shift.
            let shift = exponent.unsigned_abs();
            if shift >= 256 {
                quotient = U256::ZERO;
                false
            } else {
                let low = quotient.low_bits(shift);
                let half = U256::from_u128(1)
                    .checked_shl(shift - 1)
                    .unwrap_or(U256::ZERO);
                let kept = quotient.shr(shift);
                quotient = kept;
                match low.cmp_with(half) {
                    Ordering::Greater => true,
                    Ordering::Equal => remainder > 0 || kept.is_odd(),
                    Ordering::Less => false,
                }
            }
        };
        if round_up {
            quotient = quotient.checked_add_one().ok_or(TimeError::Overflow)?;
        }
        let attos = quotient
            .checked_mul_u64(unit_attos)
            .ok_or(TimeError::Overflow)?;
        Self::from_magnitude_attos(attos, negative != negative_factor)
    }
}

/// A finite double as `(mantissa, exponent, negative)`, the value being
/// `mantissa * 2^exponent`.
fn decompose(value: f64) -> TimeResult<(u64, i32, bool)> {
    if !value.is_finite() {
        return Err(TimeError::NotFinite);
    }
    let bits = value.to_bits();
    let negative = bits >> 63 == 1;
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1 << 52) - 1);
    Ok(if biased == 0 {
        (fraction, -1074, negative)
    } else {
        (fraction | (1 << 52), biased - 1075, negative)
    })
}

/// `numerator / denominator` as the double nearest the exact quotient, ties
/// to even.
fn ratio_of_integers(numerator: i128, denominator: i128) -> f64 {
    let negative = (numerator < 0) != (denominator < 0);
    let n = numerator.unsigned_abs();
    let d = denominator.unsigned_abs();
    let sign = if negative { -1.0 } else { 1.0 };
    if n == 0 {
        return sign * 0.0;
    }
    let power_of_two = |exponent: i32| f64::from_bits(((1023 + i64::from(exponent)) as u64) << 52);
    let quotient = n / d;
    let mut remainder = n % d;
    if quotient >> 53 != 0 {
        let shift = 128 - quotient.leading_zeros() - 53;
        let lost = quotient & ((1u128 << shift) - 1);
        let half = 1u128 << (shift - 1);
        let mut mantissa = (quotient >> shift) as u64;
        if lost > half || (lost == half && (remainder != 0 || mantissa & 1 == 1)) {
            mantissa += 1;
        }
        return sign * mantissa as f64 * power_of_two(shift as i32);
    }
    let mut mantissa = quotient as u64;
    let mut exponent = 0i32;
    while mantissa >> 52 == 0 {
        remainder <<= 1;
        mantissa <<= 1;
        if remainder >= d {
            remainder -= d;
            mantissa |= 1;
        }
        exponent -= 1;
    }
    remainder <<= 1;
    let round_bit = remainder >= d;
    if round_bit {
        remainder -= d;
    }
    if round_bit && (remainder != 0 || mantissa & 1 == 1) {
        mantissa += 1;
    }
    sign * mantissa as f64 * power_of_two(exponent)
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

    /// The split of a POSIX timestamp rounds the day down, so the second of
    /// the day is never negative, and the join undoes it, in range and at its
    /// two ends.
    #[test]
    fn whole_seconds_split_into_days_and_seconds_and_join_again() {
        assert_eq!(days_and_seconds(0), (0, 0));
        assert_eq!(days_and_seconds(86_400), (1, 0));
        assert_eq!(days_and_seconds(-1), (-1, 86_399));
        assert_eq!(days_and_seconds(-86_400), (-1, 0));
        assert_eq!(days_and_seconds(-86_401), (-2, 86_399));
        // 2016-12-31T23:59:59Z, the second before the last leap second.
        assert_eq!(days_and_seconds(1_483_228_799), (17_166, 86_399));
        for seconds in [
            0,
            1,
            -1,
            86_399,
            86_400,
            -86_401,
            1_483_228_800,
            i64::MIN,
            i64::MAX,
        ] {
            let (day, second) = days_and_seconds(seconds);
            assert!(second < 86_400);
            assert_eq!(seconds_from_days_and_seconds(day, second), Some(seconds));
        }
        assert_eq!(seconds_in_days(17_167), Some(1_483_228_800));
        assert_eq!(seconds_in_days(i64::MIN / 86_400 - 1), None);
        // A second of the day past 86 399 carries into the next day.
        assert_eq!(
            seconds_from_days_and_seconds(17_166, 86_400),
            Some(1_483_228_800)
        );
        assert_eq!(
            seconds_from_days_and_seconds(i64::MAX / 86_400, 86_399),
            None
        );
        // The method agrees with the free function on the floor second.
        for duration in [Duration::from_millis(-1), Duration::from_secs(90_061)] {
            let (days, seconds) = duration.days_and_seconds();
            let (floor_days, floor_seconds) = days_and_seconds(duration.floor_seconds() as i64);
            assert_eq!((days, seconds), (i128::from(floor_days), floor_seconds));
        }
        assert_eq!(Duration::from_millis(-1).days_and_seconds(), (-1, 86_399));
    }
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
        // And a mere million years is nowhere near it.
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

    /// Python's `timedelta * float` multiplies by the float's exact ratio and
    /// rounds once, half to even (`datetime` documentation, "rounded to the
    /// nearest multiple of timedelta.resolution using round-half-to-even");
    /// the expected values are what CPython 3.14 answers for them.
    #[test]
    fn scaling_is_exact_and_rounds_once_half_to_even() {
        let micros = |count: i128| Duration::from_micros(count);
        let at_micro = |span: Duration, factor: f64| {
            span.scale_f64_nearest(factor, 1_000_000_000_000).unwrap()
        };
        // 25 508 964 008 398 µs × 0.5, which the `f64` route through seconds
        // left with 7.5e-8 µs of noise.
        assert_eq!(
            micros(25_508_964_008_398).scale_f64(0.5),
            Ok(micros(12_754_482_004_199))
        );
        assert_eq!(at_micro(micros(1), 0.5), micros(0)); // 0.5 → 0
        assert_eq!(at_micro(micros(3), 0.5), micros(2)); // 1.5 → 2
        assert_eq!(at_micro(micros(5), 0.5), micros(2)); // 2.5 → 2
        assert_eq!(at_micro(micros(-5), 0.5), micros(-2));
        assert_eq!(at_micro(micros(1), 0.1), micros(0));
        assert_eq!(at_micro(micros(10), 0.1), micros(1)); // 0.1 is a hair above
        assert_eq!(at_micro(micros(1_000_000), 1.1), micros(1_100_000));
        // At the attosecond the same product keeps its digits.
        assert_eq!(
            micros(1).scale_f64(0.1),
            Ok(Duration::from_attos(100_000_000_000))
        );
        assert_eq!(Duration::ZERO.scale_f64(1e300), Ok(Duration::ZERO));
        assert_eq!(Duration::SECOND.scale_f64(0.0), Ok(Duration::ZERO));
        assert_eq!(
            Duration::SECOND.scale_f64(f64::NAN),
            Err(TimeError::NotFinite)
        );
        assert_eq!(Duration::SECOND.scale_f64(1e300), Err(TimeError::Overflow));
        // Half of MAX is 2^126 − 5×10⁻¹⁹ s, which rounds to 2^126.
        assert_eq!(
            Duration::MAX.scale_f64(0.5),
            Ok(Duration::from_secs(1 << 126))
        );
        // A factor too small to reach half an attosecond rounds to zero.
        assert_eq!(Duration::from_secs(1).scale_f64(1e-300), Ok(Duration::ZERO));
        assert_eq!(Duration::MIN.scale_f64(-1.0), Err(TimeError::Overflow));
        assert_eq!(Duration::MIN.scale_f64(1.0), Ok(Duration::MIN));
    }

    #[test]
    fn division_by_a_real_or_an_integer_rounds_half_to_even() {
        let micros = |count: i128| Duration::from_micros(count);
        let unit = 1_000_000_000_000;
        // `timedelta(microseconds=7) / 2` is 4 µs, `5 / 2` is 2 µs.
        assert_eq!(micros(7).div_int_nearest(2, unit), Ok(micros(4)));
        assert_eq!(micros(5).div_int_nearest(2, unit), Ok(micros(2)));
        assert_eq!(micros(5).div_int_nearest(-2, unit), Ok(micros(-2)));
        assert_eq!(micros(10).div_f64_nearest(0.3, unit), Ok(micros(33)));
        assert_eq!(
            Duration::SECOND.div_int_nearest(0, unit),
            Err(TimeError::DivideByZero)
        );
        assert_eq!(
            Duration::SECOND.div_f64_nearest(0.0, unit),
            Err(TimeError::DivideByZero)
        );
        assert_eq!(
            Duration::SECOND.scale_f64_nearest(1.0, 0),
            Err(TimeError::OutOfRange)
        );
    }

    /// `timedelta / timedelta` is Python's integer true division, correctly
    /// rounded; the values are what CPython 3.14 answers.
    #[test]
    fn the_ratio_of_two_spans_is_the_nearest_double() {
        let micros = |count: i128| Duration::from_micros(count);
        assert_eq!(
            Duration::from_days(1).ratio(micros(3)),
            Ok(28_800_000_000.0)
        );
        assert_eq!(micros(1).ratio(micros(3)), Ok(0.333_333_333_333_333_3));
        assert_eq!(micros(2).ratio(micros(3)), Ok(0.666_666_666_666_666_6));
        assert_eq!(
            micros(1_000_000_000_000_000_000).ratio(micros(3)),
            Ok(3.333_333_333_333_333e17)
        );
        assert_eq!(
            micros(86_400_000_000_000).ratio(micros(7)),
            Ok(12_342_857_142_857.143)
        );
        assert_eq!(micros(-1).ratio(micros(4)), Ok(-0.25));
        assert_eq!(Duration::ZERO.ratio(micros(4)), Ok(0.0));
        assert_eq!(
            micros(1).ratio(Duration::ZERO),
            Err(TimeError::DivideByZero)
        );
    }

    #[test]
    fn rejects_invalid_sub_second_remainders() {
        assert_eq!(Duration::new(0, ATTOS_PER_SEC), Err(TimeError::OutOfRange));
    }
}
