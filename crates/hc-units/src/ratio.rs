//! An exact rational number of seconds.
//!
//! # Why not `Duration`
//!
//! [`hc_core::Duration`] is exact to the attosecond, which covers every
//! quantity a clock produces. It does not cover every quantity a *unit*
//! produces, because a unit is a division and attoseconds are a decimal
//! grid: one third of a second, one 705 600 000th, and one 1001st all fall
//! between two attoseconds.
//!
//! That matters here specifically, because the units in this crate are the
//! ones chosen *for* their divisibility. A flick exists so that a frame at
//! 24, 25, 30, 48, 50, 60, 90, 100 or 120 fps, the NTSC 1000/1001 pull-down
//! of those frame rates, and a sample at 44.1 or 48 kHz are each a whole
//! number of flicks. Storing a flick as 1 417 233 560 attoseconds throws
//! away the one property it was invented to have.
//!
//! So conversions inside this crate go through `Ratio`, and the step out to
//! `Duration` is explicit: [`Ratio::to_duration`] refuses to round and
//! [`Ratio::to_duration_rounded`] rounds where the caller asked for it.

use core::cmp::Ordering;
use core::fmt;

use hc_core::{ATTOS_PER_SEC, Duration};

use crate::error::{UnitError, UnitResult};

/// Attoseconds in a second, as the `i128` this module does arithmetic in.
const ATTOS: i128 = ATTOS_PER_SEC as i128;

/// An exact rational number of seconds.
///
/// Always normalised: the denominator is strictly positive and shares no
/// factor with the numerator, so two `Ratio`s are equal exactly when they
/// represent the same quantity, and `Eq`/`Hash` agree with `==`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ratio {
    num: i128,
    den: i128,
}

impl Ratio {
    /// Zero seconds.
    pub const ZERO: Self = Self { num: 0, den: 1 };

    /// One second.
    pub const ONE: Self = Self { num: 1, den: 1 };

    /// A ratio from a numerator and denominator, reduced to lowest terms.
    ///
    /// # Errors
    ///
    /// [`UnitError::DivideByZero`] for a zero denominator, and
    /// [`UnitError::Overflow`] when the sign cannot be moved onto the
    /// numerator because it is `i128::MIN`.
    pub const fn new(num: i128, den: i128) -> UnitResult<Self> {
        if den == 0 {
            return Err(UnitError::DivideByZero);
        }
        if num == i128::MIN || den == i128::MIN {
            return Err(UnitError::Overflow);
        }
        let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
        let divisor = gcd(num.unsigned_abs(), den.unsigned_abs()) as i128;
        Ok(Self {
            num: num / divisor,
            den: den / divisor,
        })
    }

    /// A whole number of seconds.
    #[must_use]
    pub const fn from_secs(secs: i128) -> Self {
        Self { num: secs, den: 1 }
    }

    /// A ratio written as a literal, for the unit table.
    ///
    /// [`Ratio::new`] returns a `Result`, which a `const` item cannot
    /// unwrap. This cannot: a bad denominator is a compile error at the
    /// definition site, which is where a typo in the table would be.
    ///
    /// # Panics
    ///
    /// If the denominator is zero, or either side is `i128::MIN`. In a
    /// `const` context that is a compilation failure, not a runtime one.
    #[must_use]
    pub const fn literal(num: i128, den: i128) -> Self {
        assert!(den != 0, "a unit cannot have a zero denominator");
        assert!(
            num != i128::MIN && den != i128::MIN,
            "a unit cannot be written with i128::MIN"
        );
        let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
        let divisor = gcd(num.unsigned_abs(), den.unsigned_abs()) as i128;
        Self {
            num: num / divisor,
            den: den / divisor,
        }
    }

    /// The numerator, in lowest terms.
    #[must_use]
    pub const fn numerator(self) -> i128 {
        self.num
    }

    /// The denominator, in lowest terms and strictly positive.
    #[must_use]
    pub const fn denominator(self) -> i128 {
        self.den
    }

    /// Whether this is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.num == 0
    }

    /// Whether this is strictly negative.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.num < 0
    }

    /// The sum.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the cross-multiplied numerator or the
    /// product of the denominators leaves `i128`.
    pub const fn checked_add(self, other: Self) -> UnitResult<Self> {
        let Some(left) = self.num.checked_mul(other.den) else {
            return Err(UnitError::Overflow);
        };
        let Some(right) = other.num.checked_mul(self.den) else {
            return Err(UnitError::Overflow);
        };
        let Some(num) = left.checked_add(right) else {
            return Err(UnitError::Overflow);
        };
        let Some(den) = self.den.checked_mul(other.den) else {
            return Err(UnitError::Overflow);
        };
        Self::new(num, den)
    }

    /// The difference.
    ///
    /// # Errors
    ///
    /// As [`Ratio::checked_add`].
    pub const fn checked_sub(self, other: Self) -> UnitResult<Self> {
        let Ok(negated) = other.checked_neg() else {
            return Err(UnitError::Overflow);
        };
        self.checked_add(negated)
    }

    /// The negation.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the numerator is `i128::MIN`.
    pub const fn checked_neg(self) -> UnitResult<Self> {
        let Some(num) = self.num.checked_neg() else {
            return Err(UnitError::Overflow);
        };
        Ok(Self { num, den: self.den })
    }

    /// The product.
    ///
    /// Cross-reduces before multiplying, so that chains like "a flick times
    /// 705 600 000" stay inside `i128` instead of overflowing on the way to
    /// an answer that would have fitted.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the reduced product still leaves `i128`.
    pub const fn checked_mul(self, other: Self) -> UnitResult<Self> {
        let left = gcd(self.num.unsigned_abs(), other.den.unsigned_abs()) as i128;
        let right = gcd(other.num.unsigned_abs(), self.den.unsigned_abs()) as i128;
        let Some(num) = (self.num / left).checked_mul(other.num / right) else {
            return Err(UnitError::Overflow);
        };
        let Some(den) = (self.den / right).checked_mul(other.den / left) else {
            return Err(UnitError::Overflow);
        };
        Self::new(num, den)
    }

    /// The product with an integer.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the reduced product leaves `i128`.
    pub const fn checked_mul_int(self, factor: i128) -> UnitResult<Self> {
        self.checked_mul(Self::from_secs(factor))
    }

    /// The quotient.
    ///
    /// This is the conversion operation: `unit_a.seconds() / unit_b.seconds()`
    /// is how many of `b` fit in one `a`, exactly.
    ///
    /// # Errors
    ///
    /// [`UnitError::DivideByZero`] if `other` is zero, and
    /// [`UnitError::Overflow`] as [`Ratio::checked_mul`].
    pub const fn checked_div(self, other: Self) -> UnitResult<Self> {
        let Ok(inverted) = other.checked_recip() else {
            return Err(UnitError::DivideByZero);
        };
        self.checked_mul(inverted)
    }

    /// The reciprocal.
    ///
    /// # Errors
    ///
    /// [`UnitError::DivideByZero`] if this is zero.
    pub const fn checked_recip(self) -> UnitResult<Self> {
        if self.num == 0 {
            return Err(UnitError::DivideByZero);
        }
        Self::new(self.den, self.num)
    }

    /// Whether the value is a whole number.
    #[must_use]
    pub const fn is_integer(self) -> bool {
        self.den == 1
    }

    /// The whole part, truncated toward zero.
    #[must_use]
    pub const fn trunc(self) -> i128 {
        self.num / self.den
    }

    /// The value as `f64`, which is lossy by definition.
    ///
    /// Named rather than an `Into` so that a lossy step never happens by
    /// coercion.
    #[must_use]
    pub fn as_f64(self) -> f64 {
        self.num as f64 / self.den as f64
    }

    /// The exact [`hc_core::Duration`], or an error saying it has none.
    ///
    /// # Errors
    ///
    /// [`UnitError::Inexact`] when the denominator does not divide 10¹⁸ —
    /// a flick, an NTSC frame, a third of a second. [`UnitError::Overflow`]
    /// when the value is too large for `Duration`'s `i128` of seconds.
    pub const fn to_duration(self) -> UnitResult<Duration> {
        let secs = self.num.div_euclid(self.den);
        let remainder = self.num.rem_euclid(self.den);
        let Some(scaled) = remainder.checked_mul(ATTOS) else {
            return Err(UnitError::Overflow);
        };
        if scaled % self.den != 0 {
            return Err(UnitError::Inexact);
        }
        let attos = scaled / self.den;
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "0 <= attos < 10^18 by construction: the remainder is below the \
                      denominator, so the quotient is below ATTOS"
        )]
        let attos = attos as u64;
        match Duration::new(secs, attos) {
            Ok(duration) => Ok(duration),
            Err(inner) => Err(UnitError::from_time(inner)),
        }
    }

    /// The nearest [`hc_core::Duration`], rounding half away from zero.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] when the value is too large for `Duration`.
    pub const fn to_duration_rounded(self) -> UnitResult<Duration> {
        let secs = self.num.div_euclid(self.den);
        let remainder = self.num.rem_euclid(self.den);
        let Some(scaled) = remainder.checked_mul(ATTOS) else {
            return Err(UnitError::Overflow);
        };
        let Some(biased) = scaled.checked_add(self.den / 2) else {
            return Err(UnitError::Overflow);
        };
        let attos = biased / self.den;
        // Rounding up from the last attosecond of a second carries into it.
        let (secs, attos) = if attos >= ATTOS {
            let Some(next) = secs.checked_add(1) else {
                return Err(UnitError::Overflow);
            };
            (next, attos - ATTOS)
        } else {
            (secs, attos)
        };
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "0 <= attos < 10^18 after the carry above"
        )]
        let attos = attos as u64;
        match Duration::new(secs, attos) {
            Ok(duration) => Ok(duration),
            Err(inner) => Err(UnitError::from_time(inner)),
        }
    }

    /// The exact ratio of an existing [`hc_core::Duration`].
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] when the attosecond count leaves `i128`,
    /// which happens above about 1.7×10²⁰ seconds — five thousand times the
    /// age of the universe.
    pub const fn from_duration(duration: Duration) -> UnitResult<Self> {
        let Some(whole) = duration.whole_seconds().checked_mul(ATTOS) else {
            return Err(UnitError::Overflow);
        };
        let Some(total) = whole.checked_add(duration.subsec_attos() as i128) else {
            return Err(UnitError::Overflow);
        };
        Self::new(total, ATTOS)
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ratio {
    /// Compares by cross-multiplication, falling back to `f64` only if that
    /// overflows.
    ///
    /// The fallback is reachable in principle and not by any unit in this
    /// crate: the widest pair here is a quectosecond against a Julian
    /// millennium, whose cross product is 3×10⁴¹ — which does overflow, so
    /// the fallback is exercised by the tests rather than merely asserted.
    fn cmp(&self, other: &Self) -> Ordering {
        match (
            self.num.checked_mul(other.den),
            other.num.checked_mul(self.den),
        ) {
            (Some(left), Some(right)) => left.cmp(&right),
            // Both denominators are positive, so the sign of the difference
            // is the sign of the cross-multiplied difference, and an f64
            // comparison of values this far apart cannot land on the wrong
            // side of it.
            _ => self
                .as_f64()
                .partial_cmp(&other.as_f64())
                .unwrap_or(Ordering::Equal),
        }
    }
}

impl fmt::Display for Ratio {
    /// Prints `3` for an integer and `10/3` otherwise — never a decimal,
    /// because the values that need this type are the ones a decimal cannot
    /// write down.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

/// The greatest common divisor, by the binary algorithm so it stays `const`.
const fn gcd(mut a: u128, mut b: u128) -> u128 {
    if a == 0 {
        return if b == 0 { 1 } else { b };
    }
    if b == 0 {
        return a;
    }
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}
