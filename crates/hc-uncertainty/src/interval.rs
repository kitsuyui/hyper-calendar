//! Closed intervals of [`hc_core::Duration`], with interval arithmetic.
//!
//! An interval here means "the answer is somewhere in `[lo, hi]`, and nothing
//! finer is claimed". Interval arithmetic then gives *guaranteed enclosures*:
//! if `a ∈ A` and `b ∈ B` then `a + b ∈ A + B`, with no probability attached.
//! That is the difference from [`crate::quantity::Uncertain`] — an interval
//! never says a value near the middle is more likely, only that values
//! outside are impossible.
//!
//! # The dependency problem
//!
//! Interval arithmetic is sound but not tight. `A - A` is not zero: for
//! `A = [1, 2]` it is `[-1, 1]`, because the two occurrences of `A` are
//! treated as independent quantities that merely happen to share bounds. Any
//! expression in which one interval appears more than once will be wider than
//! the true range of the expression. This is inherent to the method, not a
//! defect of this implementation, and it is why the type deliberately offers
//! no "simplify" operation that would silently pretend otherwise.

use core::fmt;

use hc_core::Duration;

use crate::error::{UncertaintyError, UncertaintyResult};

/// A closed interval `[lo, hi]` of durations.
///
/// The empty interval is representable and is its own canonical value, so
/// that intersection is total: two disjoint intervals intersect to the empty
/// interval rather than to an error.
///
/// ```
/// use hc_core::Duration;
/// use hc_uncertainty::DurationInterval;
///
/// let a = DurationInterval::new(Duration::from_secs(1), Duration::from_secs(3)).unwrap();
/// let b = DurationInterval::new(Duration::from_secs(2), Duration::from_secs(5)).unwrap();
/// assert!(a.overlaps(b));
/// assert_eq!(a.intersect(b).low(), Some(Duration::from_secs(2)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurationInterval {
    low: Duration,
    high: Duration,
    empty: bool,
}

impl DurationInterval {
    /// The empty interval, which contains nothing.
    pub const EMPTY: Self = Self {
        low: Duration::ZERO,
        high: Duration::ZERO,
        empty: true,
    };

    /// Build `[low, high]`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::EmptyInterval`] when `low > high`. Callers
    /// that want the empty interval should say so with
    /// [`DurationInterval::EMPTY`] rather than encoding it as reversed
    /// bounds, so that a reversed pair stays an obvious mistake.
    pub fn new(low: Duration, high: Duration) -> UncertaintyResult<Self> {
        if low > high {
            return Err(UncertaintyError::EmptyInterval);
        }
        Ok(Self {
            low,
            high,
            empty: false,
        })
    }

    /// The interval containing exactly one duration.
    #[must_use]
    pub const fn degenerate(value: Duration) -> Self {
        Self {
            low: value,
            high: value,
            empty: false,
        }
    }

    /// The interval `centre ± radius`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NegativeUncertainty`] for a negative
    /// radius and [`UncertaintyError::Overflow`] when a bound leaves the
    /// representable range.
    pub fn centred(centre: Duration, radius: Duration) -> UncertaintyResult<Self> {
        if radius.is_negative() {
            return Err(UncertaintyError::NegativeUncertainty);
        }
        let low = centre.checked_sub(radius)?;
        let high = centre.checked_add(radius)?;
        Self::new(low, high)
    }

    /// Whether the interval contains nothing.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.empty
    }

    /// The lower bound, or `None` for the empty interval.
    #[must_use]
    pub const fn low(self) -> Option<Duration> {
        if self.empty { None } else { Some(self.low) }
    }

    /// The upper bound, or `None` for the empty interval.
    #[must_use]
    pub const fn high(self) -> Option<Duration> {
        if self.empty { None } else { Some(self.high) }
    }

    /// Whether the interval contains exactly one value.
    #[must_use]
    pub fn is_degenerate(self) -> bool {
        !self.empty && self.low == self.high
    }

    /// The width `high - low`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::EmptyInterval`] for the empty interval and
    /// [`UncertaintyError::Overflow`] when the width is unrepresentable.
    pub fn width(self) -> UncertaintyResult<Duration> {
        if self.empty {
            return Err(UncertaintyError::EmptyInterval);
        }
        Ok(self.high.checked_sub(self.low)?)
    }

    /// The midpoint, rounded towards negative infinity by one attosecond at
    /// most.
    ///
    /// The width is halved componentwise rather than through
    /// [`Duration::checked_div_int`], which would have to express the span in
    /// attoseconds first and so would overflow beyond about 5.4 years —
    /// uselessly short for a library that has to talk about centuries.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::EmptyInterval`] for the empty interval and
    /// [`UncertaintyError::Overflow`] when the midpoint cannot be computed
    /// without leaving the representable range.
    pub fn midpoint(self) -> UncertaintyResult<Duration> {
        Ok(self.low.checked_add(halved(self.width()?)?)?)
    }

    /// Whether `probe` lies in the closed interval.
    #[must_use]
    pub fn contains(self, probe: Duration) -> bool {
        !self.empty && probe >= self.low && probe <= self.high
    }

    /// Whether every member of `other` is also a member of `self`.
    ///
    /// The empty interval is a subset of everything, including itself.
    #[must_use]
    pub fn contains_interval(self, other: Self) -> bool {
        if other.empty {
            return true;
        }
        !self.empty && other.low >= self.low && other.high <= self.high
    }

    /// Whether the two intervals share at least one value.
    #[must_use]
    pub fn overlaps(self, other: Self) -> bool {
        !self.empty && !other.empty && self.low <= other.high && other.low <= self.high
    }

    /// The largest interval contained in both, possibly empty.
    #[must_use]
    pub fn intersect(self, other: Self) -> Self {
        if !self.overlaps(other) {
            return Self::EMPTY;
        }
        Self {
            low: if self.low > other.low {
                self.low
            } else {
                other.low
            },
            high: if self.high < other.high {
                self.high
            } else {
                other.high
            },
            empty: false,
        }
    }

    /// The smallest interval containing both — the union *hull*.
    ///
    /// The union of two disjoint intervals is not an interval, so the hull
    /// includes the gap between them. That over-approximation is the price of
    /// staying a single interval, and it is always sound.
    #[must_use]
    pub fn hull(self, other: Self) -> Self {
        if self.empty {
            return other;
        }
        if other.empty {
            return self;
        }
        Self {
            low: if self.low < other.low {
                self.low
            } else {
                other.low
            },
            high: if self.high > other.high {
                self.high
            } else {
                other.high
            },
            empty: false,
        }
    }

    /// `[lo₁ + lo₂, hi₁ + hi₂]`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::Overflow`] when a bound leaves the
    /// representable range.
    pub fn checked_add(self, other: Self) -> UncertaintyResult<Self> {
        if self.empty || other.empty {
            return Ok(Self::EMPTY);
        }
        Self::new(
            self.low.checked_add(other.low)?,
            self.high.checked_add(other.high)?,
        )
    }

    /// `[lo₁ - hi₂, hi₁ - lo₂]`.
    ///
    /// Note the crossed bounds: the smallest possible difference pairs the
    /// smallest minuend with the largest subtrahend.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::Overflow`] when a bound leaves the
    /// representable range.
    pub fn checked_sub(self, other: Self) -> UncertaintyResult<Self> {
        if self.empty || other.empty {
            return Ok(Self::EMPTY);
        }
        Self::new(
            self.low.checked_sub(other.high)?,
            self.high.checked_sub(other.low)?,
        )
    }

    /// `[-hi, -lo]`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::Overflow`] for a bound at
    /// [`Duration::MIN`].
    pub fn checked_neg(self) -> UncertaintyResult<Self> {
        if self.empty {
            return Ok(Self::EMPTY);
        }
        Self::new(self.high.checked_neg()?, self.low.checked_neg()?)
    }

    /// Multiply both bounds by an exact integer factor, swapping them when
    /// the factor is negative.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::Overflow`] when a bound leaves the
    /// representable range.
    pub fn scale_int(self, factor: i64) -> UncertaintyResult<Self> {
        if self.empty {
            return Ok(Self::EMPTY);
        }
        let a = self.low.checked_mul_int(factor)?;
        let b = self.high.checked_mul_int(factor)?;
        if factor < 0 {
            Self::new(b, a)
        } else {
            Self::new(a, b)
        }
    }

    /// Multiply both bounds by a real factor, swapping them when the factor
    /// is negative.
    ///
    /// The result is only as good as `f64` allows; use
    /// [`DurationInterval::scale_int`] when the factor is exact.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] for a non-finite factor and
    /// [`UncertaintyError::Overflow`] when a bound leaves the representable
    /// range.
    pub fn scale(self, factor: f64) -> UncertaintyResult<Self> {
        if self.empty {
            return Ok(Self::EMPTY);
        }
        if !factor.is_finite() {
            return Err(UncertaintyError::NotFinite);
        }
        let a = self.low.scale_f64(factor)?;
        let b = self.high.scale_f64(factor)?;
        if factor < 0.0 {
            Self::new(b, a)
        } else {
            Self::new(a, b)
        }
    }
}

/// Half of a span, floored, without going through an attosecond total.
///
/// `Duration` stores a floor decomposition, so halving the whole-second part
/// and folding its remainder into the sub-second part is exact and stays in
/// range for the full `i128` span.
fn halved(span: Duration) -> UncertaintyResult<Duration> {
    let seconds = span.whole_seconds();
    let carry = seconds.rem_euclid(2) * i128::from(hc_core::ATTOS_PER_SEC);
    let attos = (i128::from(span.subsec_attos()) + carry) / 2;
    Ok(Duration::new(seconds.div_euclid(2), attos as u64)?)
}

impl fmt::Display for DurationInterval {
    /// Renders as `[lo, hi]` in seconds, or `[]` when empty.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.empty {
            f.write_str("[]")
        } else {
            write!(f, "[{}, {}]", self.low, self.high)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn secs(value: i128) -> Duration {
        Duration::from_secs(value)
    }

    fn interval(low: i128, high: i128) -> DurationInterval {
        DurationInterval::new(secs(low), secs(high)).unwrap()
    }

    #[test]
    fn reversed_bounds_are_rejected() {
        assert_eq!(
            DurationInterval::new(secs(5), secs(1)),
            Err(UncertaintyError::EmptyInterval)
        );
    }

    #[test]
    fn the_empty_interval_has_no_bounds() {
        let empty = DurationInterval::EMPTY;
        assert!(empty.is_empty());
        assert_eq!(empty.low(), None);
        assert_eq!(empty.high(), None);
        assert_eq!(empty.width(), Err(UncertaintyError::EmptyInterval));
    }

    #[test]
    fn a_degenerate_interval_holds_one_value() {
        let point = DurationInterval::degenerate(secs(7));
        assert!(point.is_degenerate());
        assert!(point.contains(secs(7)));
        assert!(!point.contains(secs(8)));
        assert_eq!(point.width().unwrap(), Duration::ZERO);
    }

    #[test]
    fn a_centred_interval_is_symmetric_about_its_centre() {
        let span = DurationInterval::centred(secs(100), secs(10)).unwrap();
        assert_eq!(span.low(), Some(secs(90)));
        assert_eq!(span.high(), Some(secs(110)));
        assert_eq!(span.midpoint().unwrap(), secs(100));
    }

    #[test]
    fn a_negative_radius_is_rejected() {
        assert_eq!(
            DurationInterval::centred(secs(0), secs(-1)),
            Err(UncertaintyError::NegativeUncertainty)
        );
    }

    #[test]
    fn containment_is_closed_at_both_ends() {
        let span = interval(1, 3);
        assert!(span.contains(secs(1)));
        assert!(span.contains(secs(3)));
        assert!(!span.contains(Duration::from_millis(999)));
        assert!(!span.contains(Duration::from_millis(3001)));
    }

    #[test]
    fn interval_containment_matches_bound_comparison() {
        let outer = interval(0, 10);
        assert!(outer.contains_interval(interval(2, 8)));
        assert!(outer.contains_interval(outer));
        assert!(!outer.contains_interval(interval(2, 11)));
        assert!(outer.contains_interval(DurationInterval::EMPTY));
    }

    #[test]
    fn touching_intervals_overlap_because_the_bounds_are_closed() {
        assert!(interval(0, 5).overlaps(interval(5, 9)));
        assert!(!interval(0, 5).overlaps(interval(6, 9)));
    }

    #[test]
    fn the_empty_interval_overlaps_nothing() {
        assert!(!DurationInterval::EMPTY.overlaps(interval(0, 10)));
        assert!(!interval(0, 10).overlaps(DurationInterval::EMPTY));
    }

    #[test]
    fn intersection_takes_the_tighter_bound_at_each_end() {
        let result = interval(0, 10).intersect(interval(4, 20));
        assert_eq!(result.low(), Some(secs(4)));
        assert_eq!(result.high(), Some(secs(10)));
    }

    #[test]
    fn disjoint_intervals_intersect_to_nothing() {
        assert!(interval(0, 1).intersect(interval(2, 3)).is_empty());
    }

    #[test]
    fn intersection_is_commutative_and_idempotent() {
        let a = interval(-5, 5);
        let b = interval(0, 20);
        assert_eq!(a.intersect(b), b.intersect(a));
        assert_eq!(a.intersect(a), a);
    }

    #[test]
    fn the_hull_spans_the_gap_between_disjoint_intervals() {
        let hull = interval(0, 1).hull(interval(10, 11));
        assert_eq!(hull.low(), Some(secs(0)));
        assert_eq!(hull.high(), Some(secs(11)));
        // The hull is an over-approximation: 5 s is in it but in neither part.
        assert!(hull.contains(secs(5)));
    }

    #[test]
    fn the_hull_of_anything_with_the_empty_interval_is_that_thing() {
        let span = interval(3, 4);
        assert_eq!(span.hull(DurationInterval::EMPTY), span);
        assert_eq!(DurationInterval::EMPTY.hull(span), span);
    }

    #[test]
    fn addition_adds_matching_bounds() {
        let sum = interval(1, 2).checked_add(interval(10, 20)).unwrap();
        assert_eq!(sum.low(), Some(secs(11)));
        assert_eq!(sum.high(), Some(secs(22)));
    }

    #[test]
    fn subtraction_crosses_the_bounds() {
        let difference = interval(1, 2).checked_sub(interval(10, 20)).unwrap();
        assert_eq!(difference.low(), Some(secs(-19)));
        assert_eq!(difference.high(), Some(secs(-8)));
    }

    #[test]
    fn subtracting_an_interval_from_itself_is_not_zero() {
        // The dependency problem, made explicit.
        let span = interval(1, 2);
        let difference = span.checked_sub(span).unwrap();
        assert_eq!(difference.low(), Some(secs(-1)));
        assert_eq!(difference.high(), Some(secs(1)));
    }

    #[test]
    fn negation_reflects_the_interval_about_zero() {
        let negated = interval(1, 3).checked_neg().unwrap();
        assert_eq!(negated.low(), Some(secs(-3)));
        assert_eq!(negated.high(), Some(secs(-1)));
    }

    #[test]
    fn negation_round_trips() {
        let span = interval(-7, 11);
        assert_eq!(span.checked_neg().unwrap().checked_neg().unwrap(), span);
    }

    #[test]
    fn integer_scaling_by_a_negative_factor_swaps_the_bounds() {
        let scaled = interval(1, 3).scale_int(-2).unwrap();
        assert_eq!(scaled.low(), Some(secs(-6)));
        assert_eq!(scaled.high(), Some(secs(-2)));
    }

    #[test]
    fn integer_scaling_by_zero_collapses_to_a_point() {
        let scaled = interval(-4, 9).scale_int(0).unwrap();
        assert!(scaled.is_degenerate());
        assert_eq!(scaled.low(), Some(Duration::ZERO));
    }

    #[test]
    fn real_scaling_matches_integer_scaling_for_whole_factors() {
        let span = interval(2, 6);
        let by_int = span.scale_int(3).unwrap();
        let by_real = span.scale(3.0).unwrap();
        assert_eq!(by_int, by_real);
    }

    #[test]
    fn real_scaling_by_a_negative_factor_swaps_the_bounds() {
        let scaled = interval(2, 6).scale(-0.5).unwrap();
        assert_eq!(scaled.low(), Some(secs(-3)));
        assert_eq!(scaled.high(), Some(secs(-1)));
    }

    #[test]
    fn a_non_finite_scale_factor_is_rejected() {
        assert_eq!(
            interval(0, 1).scale(f64::NAN),
            Err(UncertaintyError::NotFinite)
        );
    }

    #[test]
    fn arithmetic_with_the_empty_interval_stays_empty() {
        let empty = DurationInterval::EMPTY;
        let span = interval(0, 1);
        assert!(empty.checked_add(span).unwrap().is_empty());
        assert!(span.checked_add(empty).unwrap().is_empty());
        assert!(span.checked_sub(empty).unwrap().is_empty());
        assert!(empty.checked_neg().unwrap().is_empty());
        assert!(empty.scale_int(4).unwrap().is_empty());
        assert!(empty.scale(4.0).unwrap().is_empty());
    }

    #[test]
    fn addition_encloses_every_pairwise_sum() {
        // The defining property of interval arithmetic, checked by sampling.
        let a = interval(-3, 7);
        let b = interval(2, 11);
        let sum = a.checked_add(b).unwrap();
        for x in -3..=7i128 {
            for y in 2..=11i128 {
                assert!(sum.contains(secs(x + y)), "{x} + {y} escaped {sum}");
            }
        }
    }

    #[test]
    fn subtraction_encloses_every_pairwise_difference() {
        let a = interval(-3, 7);
        let b = interval(2, 11);
        let difference = a.checked_sub(b).unwrap();
        for x in -3..=7i128 {
            for y in 2..=11i128 {
                assert!(difference.contains(secs(x - y)), "{x} - {y} escaped");
            }
        }
    }

    #[test]
    fn overflow_at_the_representable_edge_is_reported() {
        let top = DurationInterval::degenerate(Duration::MAX);
        assert_eq!(
            top.checked_add(DurationInterval::degenerate(Duration::SECOND)),
            Err(UncertaintyError::Overflow)
        );
    }

    #[test]
    fn the_midpoint_of_an_odd_width_lands_inside() {
        let span = DurationInterval::new(secs(0), secs(3)).unwrap();
        let middle = span.midpoint().unwrap();
        assert!(span.contains(middle));
        assert_eq!(middle, Duration::from_millis(1_500));
    }

    #[test]
    fn the_midpoint_survives_spans_far_longer_than_an_attosecond_count() {
        // A million years is well past the range of `total_attos`, so this
        // would fail if the midpoint went through an attosecond total.
        let million_years = secs(1_000_000 * 365 * 86_400);
        let span = DurationInterval::new(Duration::ZERO, million_years).unwrap();
        let middle = span.midpoint().unwrap();
        assert_eq!(middle.checked_mul_int(2).unwrap(), million_years);
    }

    #[test]
    fn halving_is_exact_for_odd_second_counts() {
        assert_eq!(halved(secs(3)).unwrap(), Duration::from_millis(1_500));
        assert_eq!(halved(secs(-3)).unwrap(), Duration::from_millis(-1_500));
        assert_eq!(halved(Duration::from_attos(1)).unwrap(), Duration::ZERO);
    }

    #[test]
    fn display_shows_the_bounds_in_seconds() {
        #[cfg(feature = "alloc")]
        {
            use alloc::string::ToString as _;
            assert_eq!(interval(1, 3).to_string(), "[1, 3]");
            assert_eq!(DurationInterval::EMPTY.to_string(), "[]");
        }
    }
}
