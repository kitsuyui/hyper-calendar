//! Gaussian quantities and first-order error propagation.
//!
//! A measurement is a central value and a standard deviation: `3200 ± 50`.
//! This module propagates that pair through arithmetic using the linear
//! (delta-method) approximation, which is what every physics laboratory and
//! every radiocarbon report means by "±".
//!
//! # What the approximation assumes, and when it breaks
//!
//! For `z = f(x, y)` the propagated variance is
//!
//! ```text
//! σ_z² = (∂f/∂x)² σ_x² + (∂f/∂y)² σ_y²
//! ```
//!
//! evaluated at the central values. That is exact for linear `f` and good to
//! O(σ²) otherwise. It assumes the inputs are **independent**: adding a
//! quantity to itself with [`Uncertain::checked_add`] gives `σ√2` rather than
//! the correct `2σ`, because the type carries no covariance. Use
//! [`Uncertain::scaled`] when the correlation is total, and treat any
//! expression in which one variable appears twice with suspicion.
//!
//! The approximation also degrades when the relative uncertainty is large:
//! the distribution of `1/x` for `x = 1 ± 0.5` is not remotely Gaussian, and
//! no first-order formula will say so. This module refuses to divide by a
//! central value of zero, but it cannot detect the softer failure, so the
//! rule of thumb stands: trust the propagated `σ` while `σ/|x|` stays below
//! roughly 0.1.

use core::fmt;

use hc_core::math;

use crate::error::{UncertaintyError, UncertaintyResult};
use crate::sig_figs::{MAX_FIGURES, Significant};

/// A quantity known as a Gaussian: a central value and a 1σ standard
/// deviation.
///
/// ```
/// use hc_uncertainty::Uncertain;
///
/// // A radiocarbon age, 3200 ± 50 years before present.
/// let age = Uncertain::new(3200.0, 50.0).unwrap();
/// assert!(age.contains(3150.0, 1.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Uncertain {
    /// The central value — the best estimate.
    pub value: f64,
    /// The 1σ standard deviation, never negative.
    pub std_dev: f64,
}

impl Uncertain {
    /// Build a quantity from a central value and a standard deviation.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] for NaN or infinity and
    /// [`UncertaintyError::NegativeUncertainty`] for a negative `std_dev`.
    pub fn new(value: f64, std_dev: f64) -> UncertaintyResult<Self> {
        if !value.is_finite() || !std_dev.is_finite() {
            return Err(UncertaintyError::NotFinite);
        }
        if std_dev < 0.0 {
            return Err(UncertaintyError::NegativeUncertainty);
        }
        Ok(Self { value, std_dev })
    }

    /// A quantity with no uncertainty at all.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] for NaN or infinity.
    pub fn exact(value: f64) -> UncertaintyResult<Self> {
        Self::new(value, 0.0)
    }

    /// Build a quantity from a central value and a *relative* uncertainty.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NegativeUncertainty`] for a negative
    /// fraction, and [`UncertaintyError::NotFinite`] for non-finite inputs.
    pub fn from_relative(value: f64, relative: f64) -> UncertaintyResult<Self> {
        if relative < 0.0 {
            return Err(UncertaintyError::NegativeUncertainty);
        }
        Self::new(value, math::abs(value) * relative)
    }

    /// The variance, `σ²`.
    #[must_use]
    pub fn variance(self) -> f64 {
        self.std_dev * self.std_dev
    }

    /// The relative uncertainty `σ/|value|`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::DivideByZero`] when the central value is
    /// zero, where a relative uncertainty has no meaning.
    pub fn relative(self) -> UncertaintyResult<f64> {
        if self.value == 0.0 {
            return Err(UncertaintyError::DivideByZero);
        }
        Ok(self.std_dev / math::abs(self.value))
    }

    /// Whether the quantity carries no uncertainty.
    #[must_use]
    pub fn is_exact(self) -> bool {
        self.std_dev == 0.0
    }

    /// The closed interval `value ± n·σ`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NegativeUncertainty`] for a negative
    /// `n_sigma` and [`UncertaintyError::NotFinite`] if the bounds overflow.
    pub fn within(self, n_sigma: f64) -> UncertaintyResult<(f64, f64)> {
        if n_sigma < 0.0 {
            return Err(UncertaintyError::NegativeUncertainty);
        }
        if !n_sigma.is_finite() {
            return Err(UncertaintyError::NotFinite);
        }
        let half_width = n_sigma * self.std_dev;
        let low = self.value - half_width;
        let high = self.value + half_width;
        if !low.is_finite() || !high.is_finite() {
            return Err(UncertaintyError::NotFinite);
        }
        Ok((low, high))
    }

    /// Whether `probe` lies inside `value ± n·σ`.
    ///
    /// An exact quantity contains only its own value, whatever `n_sigma` is.
    #[must_use]
    pub fn contains(self, probe: f64, n_sigma: f64) -> bool {
        match self.within(n_sigma) {
            Ok((low, high)) => probe >= low && probe <= high,
            Err(_) => false,
        }
    }

    /// Whether the 1σ intervals of the two quantities meet.
    ///
    /// This is the quick "are these two results compatible" test used when
    /// reading two error bars off a plot. [`Uncertain::z_score`] answers the
    /// same question with a number rather than a yes or no.
    #[must_use]
    pub fn overlaps(self, other: Self) -> bool {
        self.overlaps_within(other, 1.0)
    }

    /// Whether the `n`σ intervals of the two quantities meet.
    #[must_use]
    pub fn overlaps_within(self, other: Self, n_sigma: f64) -> bool {
        if n_sigma < 0.0 || !n_sigma.is_finite() {
            return false;
        }
        let separation = math::abs(self.value - other.value);
        separation <= n_sigma * (self.std_dev + other.std_dev)
    }

    /// How many combined standard deviations separate the two central values.
    ///
    /// The denominator is `√(σ₁² + σ₂²)`, the standard deviation of the
    /// difference, so a z-score near 1 means "consistent" and one above 3
    /// means "these are different measurements of different things" — the
    /// usual reading of a tension in the literature.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::DivideByZero`] when both quantities are
    /// exact, where the question has no statistical answer.
    pub fn z_score(self, other: Self) -> UncertaintyResult<f64> {
        let combined = math::sqrt(self.variance() + other.variance());
        if combined == 0.0 {
            return Err(UncertaintyError::DivideByZero);
        }
        Ok(math::abs(self.value - other.value) / combined)
    }

    /// Sum of two independent quantities: `σ² = σ₁² + σ₂²`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the result overflows.
    pub fn checked_add(self, other: Self) -> UncertaintyResult<Self> {
        Self::new(
            self.value + other.value,
            math::sqrt(self.variance() + other.variance()),
        )
    }

    /// Difference of two independent quantities.
    ///
    /// The uncertainties still add in quadrature: subtracting does not cancel
    /// error, it accumulates it.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the result overflows.
    pub fn checked_sub(self, other: Self) -> UncertaintyResult<Self> {
        Self::new(
            self.value - other.value,
            math::sqrt(self.variance() + other.variance()),
        )
    }

    /// Product of two independent quantities.
    ///
    /// `σ_z² = (y σ_x)² + (x σ_y)²`, the delta method applied to `xy`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the result overflows.
    pub fn checked_mul(self, other: Self) -> UncertaintyResult<Self> {
        let value = self.value * other.value;
        let term_a = other.value * self.std_dev;
        let term_b = self.value * other.std_dev;
        Self::new(value, math::sqrt(term_a * term_a + term_b * term_b))
    }

    /// Quotient of two independent quantities.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::DivideByZero`] when the divisor's central
    /// value is zero — the linear approximation has no derivative there — and
    /// [`UncertaintyError::NotFinite`] when the result overflows.
    pub fn checked_div(self, other: Self) -> UncertaintyResult<Self> {
        if other.value == 0.0 {
            return Err(UncertaintyError::DivideByZero);
        }
        let value = self.value / other.value;
        let term_a = self.std_dev / other.value;
        let term_b = self.value * other.std_dev / (other.value * other.value);
        Self::new(value, math::sqrt(term_a * term_a + term_b * term_b))
    }

    /// Multiply by an exactly known constant.
    ///
    /// Unlike [`Uncertain::checked_mul`] with an exact operand this is the
    /// right thing to use when the same variable is being rescaled, because
    /// it multiplies `σ` rather than combining two independent errors.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the result overflows.
    pub fn scaled(self, factor: f64) -> UncertaintyResult<Self> {
        Self::new(self.value * factor, math::abs(factor) * self.std_dev)
    }

    /// Add an exactly known constant, which shifts the value and leaves `σ`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the result overflows.
    pub fn shifted(self, offset: f64) -> UncertaintyResult<Self> {
        Self::new(self.value + offset, self.std_dev)
    }

    /// The quantity with its sign reversed.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the result overflows.
    pub fn negated(self) -> UncertaintyResult<Self> {
        Self::new(-self.value, self.std_dev)
    }

    /// Raise to a real power: `σ_z = |n x^(n-1)| σ_x`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::OutOfDomain`] for a negative base with a
    /// non-integer exponent, and for a zero base with a negative exponent.
    /// Returns [`UncertaintyError::NotFinite`] when the result overflows.
    pub fn powf(self, exponent: f64) -> UncertaintyResult<Self> {
        let integral = math::floor(exponent) == exponent;
        if self.value < 0.0 && !integral {
            return Err(UncertaintyError::OutOfDomain);
        }
        if self.value == 0.0 && exponent < 1.0 {
            return Err(UncertaintyError::OutOfDomain);
        }
        let value = math::powf(self.value, exponent);
        let slope = exponent * math::powf(self.value, exponent - 1.0);
        Self::new(value, math::abs(slope) * self.std_dev)
    }

    /// Natural logarithm: `σ_z = σ_x / |x|`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::OutOfDomain`] for a non-positive central
    /// value.
    pub fn ln(self) -> UncertaintyResult<Self> {
        if self.value <= 0.0 {
            return Err(UncertaintyError::OutOfDomain);
        }
        Self::new(math::ln(self.value), self.std_dev / self.value)
    }

    /// Exponential: `σ_z = e^x σ_x`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the result overflows.
    pub fn exp(self) -> UncertaintyResult<Self> {
        let value = math::exp(self.value);
        Self::new(value, value * self.std_dev)
    }

    /// Combine two independent measurements of the *same* quantity.
    ///
    /// This is the inverse-variance weighted mean: each measurement is
    /// weighted by `1/σ²`, the result's variance is `1/Σw`, and the answer is
    /// always at least as precise as the better of the two inputs. It is the
    /// maximum-likelihood estimator for Gaussian errors and the standard way
    /// a review paper merges published values.
    ///
    /// An exact input wins outright: combining anything with a measurement of
    /// zero uncertainty returns that exact value, because no weighted average
    /// can improve on a quantity that is already known.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::DivideByZero`] when both inputs are exact
    /// but disagree, which is a contradiction rather than a measurement.
    pub fn combine(self, other: Self) -> UncertaintyResult<Self> {
        match (self.is_exact(), other.is_exact()) {
            (true, true) => {
                if self.value == other.value {
                    Ok(self)
                } else {
                    Err(UncertaintyError::DivideByZero)
                }
            }
            (true, false) => Ok(self),
            (false, true) => Ok(other),
            (false, false) => {
                let weight_a = 1.0 / self.variance();
                let weight_b = 1.0 / other.variance();
                let total = weight_a + weight_b;
                if !total.is_finite() || total == 0.0 {
                    return Err(UncertaintyError::DivideByZero);
                }
                Self::new(
                    (weight_a * self.value + weight_b * other.value) / total,
                    math::sqrt(1.0 / total),
                )
            }
        }
    }

    /// The inverse-variance weighted mean of a whole set of measurements.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::EmptyInterval`] for an empty slice, and
    /// the errors of [`Uncertain::combine`] otherwise.
    pub fn weighted_mean(values: &[Self]) -> UncertaintyResult<Self> {
        let (first, rest) = values
            .split_first()
            .ok_or(UncertaintyError::EmptyInterval)?;
        let mut accumulated = *first;
        for value in rest {
            accumulated = accumulated.combine(*value)?;
        }
        Ok(accumulated)
    }

    /// The central value rendered at the precision the error bar justifies.
    ///
    /// The convention here is the one the Particle Data Group uses for
    /// tables: the uncertainty fixes the last significant place of the value,
    /// and `σ` itself is quoted to two digits. `1234.5 ± 12` therefore has
    /// four significant figures, not five.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] if the central value cannot
    /// carry a figure count, and passes through
    /// [`UncertaintyError::InvalidSignificantFigures`].
    pub fn to_significant(self) -> UncertaintyResult<Significant> {
        if self.is_exact() {
            return Significant::exact(self.value);
        }
        let value_exponent = Significant::exact(self.value)?.decimal_exponent();
        let sigma_exponent = Significant::exact(self.std_dev)?.decimal_exponent();
        // Two digits of sigma means the last significant place of the value
        // is one decade below sigma's leading digit.
        let last_place = sigma_exponent - 1;
        let figures = (value_exponent - last_place + 1).clamp(1, i32::from(MAX_FIGURES)) as u8;
        Significant::new(self.value, figures)
    }
}

impl fmt::Display for Uncertain {
    /// Renders as `value ± std_dev`, the notation used in every measurement
    /// report this type is meant to model.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.value, self.std_dev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_radiocarbon_age_keeps_its_error_bar() {
        let age = Uncertain::new(3200.0, 50.0).unwrap();
        assert!((age.value - 3200.0).abs() < 1e-12);
        assert!((age.std_dev - 50.0).abs() < 1e-12);
        assert!((age.variance() - 2500.0).abs() < 1e-9);
    }

    #[test]
    fn a_negative_standard_deviation_is_rejected() {
        assert_eq!(
            Uncertain::new(1.0, -1.0),
            Err(UncertaintyError::NegativeUncertainty)
        );
    }

    #[test]
    fn non_finite_inputs_are_rejected() {
        assert_eq!(
            Uncertain::new(f64::NAN, 1.0),
            Err(UncertaintyError::NotFinite)
        );
        assert_eq!(
            Uncertain::new(1.0, f64::INFINITY),
            Err(UncertaintyError::NotFinite)
        );
    }

    #[test]
    fn addition_combines_errors_in_quadrature() {
        let a = Uncertain::new(10.0, 3.0).unwrap();
        let b = Uncertain::new(20.0, 4.0).unwrap();
        let sum = a.checked_add(b).unwrap();
        assert!((sum.value - 30.0).abs() < 1e-12);
        // 3-4-5 triangle: the quadrature sum is exactly 5.
        assert!((sum.std_dev - 5.0).abs() < 1e-12);
    }

    #[test]
    fn subtraction_accumulates_error_rather_than_cancelling_it() {
        let a = Uncertain::new(10.0, 3.0).unwrap();
        let b = Uncertain::new(10.0, 4.0).unwrap();
        let difference = a.checked_sub(b).unwrap();
        assert!(difference.value.abs() < 1e-12);
        assert!((difference.std_dev - 5.0).abs() < 1e-12);
    }

    #[test]
    fn a_product_adds_relative_errors_in_quadrature() {
        let a = Uncertain::new(2.0, 0.2).unwrap();
        let b = Uncertain::new(5.0, 1.0).unwrap();
        let product = a.checked_mul(b).unwrap();
        assert!((product.value - 10.0).abs() < 1e-12);
        // 0.1 and 0.2 relative: sqrt(0.01 + 0.04) * 10 = 2.2360679...
        let expected = 10.0 * math::sqrt(0.05);
        assert!((product.std_dev - expected).abs() < 1e-12);
    }

    #[test]
    fn a_quotient_adds_relative_errors_in_quadrature() {
        let a = Uncertain::new(10.0, 1.0).unwrap();
        let b = Uncertain::new(2.0, 0.2).unwrap();
        let quotient = a.checked_div(b).unwrap();
        assert!((quotient.value - 5.0).abs() < 1e-12);
        let expected = 5.0 * math::sqrt(0.01 + 0.01);
        assert!((quotient.std_dev - expected).abs() < 1e-12);
    }

    #[test]
    fn division_by_a_zero_central_value_is_refused() {
        let a = Uncertain::new(1.0, 0.1).unwrap();
        let zero = Uncertain::new(0.0, 0.1).unwrap();
        assert_eq!(a.checked_div(zero), Err(UncertaintyError::DivideByZero));
    }

    #[test]
    fn multiplying_by_an_exact_constant_scales_sigma_linearly() {
        let value = Uncertain::new(7.0, 0.5).unwrap();
        let doubled = value.scaled(2.0).unwrap();
        assert!((doubled.value - 14.0).abs() < 1e-12);
        assert!((doubled.std_dev - 1.0).abs() < 1e-12);
    }

    #[test]
    fn scaling_by_a_negative_factor_keeps_sigma_positive() {
        let value = Uncertain::new(7.0, 0.5).unwrap();
        let flipped = value.scaled(-2.0).unwrap();
        assert!((flipped.value + 14.0).abs() < 1e-12);
        assert!((flipped.std_dev - 1.0).abs() < 1e-12);
    }

    #[test]
    fn shifting_by_a_constant_leaves_sigma_alone() {
        let value = Uncertain::new(1950.0, 30.0).unwrap();
        let shifted = value.shifted(-1950.0).unwrap();
        assert!(shifted.value.abs() < 1e-12);
        assert!((shifted.std_dev - 30.0).abs() < 1e-12);
    }

    #[test]
    fn a_square_doubles_the_relative_error() {
        let value = Uncertain::new(4.0, 0.4).unwrap();
        let squared = value.powf(2.0).unwrap();
        assert!((squared.value - 16.0).abs() < 1e-12);
        // d(x^2)/dx = 2x = 8, so sigma = 8 * 0.4 = 3.2, i.e. 20% relative.
        assert!((squared.std_dev - 3.2).abs() < 1e-12);
        assert!((squared.relative().unwrap() - 0.2).abs() < 1e-12);
    }

    #[test]
    fn a_square_root_halves_the_relative_error() {
        let value = Uncertain::new(100.0, 10.0).unwrap();
        let root = value.powf(0.5).unwrap();
        assert!((root.value - 10.0).abs() < 1e-12);
        assert!((root.relative().unwrap() - 0.05).abs() < 1e-12);
    }

    #[test]
    fn a_fractional_power_of_a_negative_value_is_out_of_domain() {
        let value = Uncertain::new(-4.0, 0.1).unwrap();
        assert_eq!(value.powf(0.5), Err(UncertaintyError::OutOfDomain));
    }

    #[test]
    fn the_logarithm_turns_relative_error_into_absolute_error() {
        let value = Uncertain::new(100.0, 5.0).unwrap();
        let logged = value.ln().unwrap();
        assert!((logged.value - math::ln(100.0)).abs() < 1e-12);
        assert!((logged.std_dev - 0.05).abs() < 1e-12);
    }

    #[test]
    fn the_logarithm_of_a_non_positive_value_is_out_of_domain() {
        let zero = Uncertain::new(0.0, 1.0).unwrap();
        let negative = Uncertain::new(-1.0, 1.0).unwrap();
        assert_eq!(zero.ln(), Err(UncertaintyError::OutOfDomain));
        assert_eq!(negative.ln(), Err(UncertaintyError::OutOfDomain));
    }

    #[test]
    fn exp_and_ln_are_inverse_to_first_order() {
        let value = Uncertain::new(2.0, 0.01).unwrap();
        let round_tripped = value.ln().unwrap().exp().unwrap();
        assert!((round_tripped.value - 2.0).abs() < 1e-12);
        assert!((round_tripped.std_dev - 0.01).abs() < 1e-12);
    }

    #[test]
    fn combining_two_equal_measurements_tightens_sigma_by_root_two() {
        let a = Uncertain::new(10.0, 2.0).unwrap();
        let b = Uncertain::new(10.0, 2.0).unwrap();
        let merged = a.combine(b).unwrap();
        assert!((merged.value - 10.0).abs() < 1e-12);
        assert!((merged.std_dev - 2.0 / math::sqrt(2.0)).abs() < 1e-12);
    }

    #[test]
    fn combining_pulls_the_mean_towards_the_sharper_measurement() {
        let loose = Uncertain::new(0.0, 10.0).unwrap();
        let tight = Uncertain::new(100.0, 1.0).unwrap();
        let merged = loose.combine(tight).unwrap();
        assert!(merged.value > 99.0, "got {}", merged.value);
        assert!(merged.std_dev < 1.0);
    }

    #[test]
    fn combining_with_an_exact_value_yields_that_value() {
        let measured = Uncertain::new(9.0, 3.0).unwrap();
        let known = Uncertain::exact(10.0).unwrap();
        assert_eq!(measured.combine(known).unwrap(), known);
        assert_eq!(known.combine(measured).unwrap(), known);
    }

    #[test]
    fn two_contradictory_exact_values_cannot_be_combined() {
        let a = Uncertain::exact(1.0).unwrap();
        let b = Uncertain::exact(2.0).unwrap();
        assert_eq!(a.combine(b), Err(UncertaintyError::DivideByZero));
    }

    #[test]
    fn a_weighted_mean_of_many_matches_repeated_pairwise_combination() {
        let values = [
            Uncertain::new(1.0, 1.0).unwrap(),
            Uncertain::new(2.0, 1.0).unwrap(),
            Uncertain::new(3.0, 1.0).unwrap(),
        ];
        let mean = Uncertain::weighted_mean(&values).unwrap();
        assert!((mean.value - 2.0).abs() < 1e-12);
        assert!((mean.std_dev - 1.0 / math::sqrt(3.0)).abs() < 1e-12);
    }

    #[test]
    fn a_weighted_mean_of_nothing_is_an_error() {
        assert_eq!(
            Uncertain::weighted_mean(&[]),
            Err(UncertaintyError::EmptyInterval)
        );
    }

    #[test]
    fn within_returns_the_n_sigma_band() {
        let value = Uncertain::new(3200.0, 50.0).unwrap();
        let (low, high) = value.within(2.0).unwrap();
        assert!((low - 3100.0).abs() < 1e-12);
        assert!((high - 3300.0).abs() < 1e-12);
    }

    #[test]
    fn within_refuses_a_negative_sigma_count() {
        let value = Uncertain::new(1.0, 1.0).unwrap();
        assert_eq!(
            value.within(-1.0),
            Err(UncertaintyError::NegativeUncertainty)
        );
    }

    #[test]
    fn contains_respects_the_closed_band() {
        let value = Uncertain::new(0.0, 1.0).unwrap();
        assert!(value.contains(1.0, 1.0));
        assert!(value.contains(-1.0, 1.0));
        assert!(!value.contains(1.000_001, 1.0));
    }

    #[test]
    fn an_exact_quantity_contains_only_itself() {
        let value = Uncertain::exact(5.0).unwrap();
        assert!(value.contains(5.0, 3.0));
        assert!(!value.contains(5.000_001, 3.0));
    }

    #[test]
    fn overlapping_error_bars_are_detected() {
        let a = Uncertain::new(10.0, 1.0).unwrap();
        let b = Uncertain::new(11.5, 1.0).unwrap();
        let c = Uncertain::new(20.0, 1.0).unwrap();
        assert!(a.overlaps(b));
        assert!(!a.overlaps(c));
        assert!(a.overlaps_within(c, 5.0));
    }

    #[test]
    fn overlap_is_symmetric() {
        let a = Uncertain::new(3200.0, 50.0).unwrap();
        let b = Uncertain::new(3275.0, 40.0).unwrap();
        assert_eq!(a.overlaps(b), b.overlaps(a));
        assert!(a.overlaps(b));
    }

    #[test]
    fn the_z_score_counts_combined_sigmas() {
        let a = Uncertain::new(0.0, 3.0).unwrap();
        let b = Uncertain::new(5.0, 4.0).unwrap();
        // sqrt(9 + 16) = 5, so the separation is exactly one sigma.
        assert!((a.z_score(b).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn two_exact_quantities_have_no_z_score() {
        let a = Uncertain::exact(1.0).unwrap();
        let b = Uncertain::exact(2.0).unwrap();
        assert_eq!(a.z_score(b), Err(UncertaintyError::DivideByZero));
    }

    #[test]
    fn a_relative_uncertainty_round_trips() {
        let value = Uncertain::from_relative(200.0, 0.05).unwrap();
        assert!((value.std_dev - 10.0).abs() < 1e-12);
        assert!((value.relative().unwrap() - 0.05).abs() < 1e-15);
    }

    #[test]
    fn a_relative_uncertainty_of_zero_is_undefined() {
        let value = Uncertain::new(0.0, 1.0).unwrap();
        assert_eq!(value.relative(), Err(UncertaintyError::DivideByZero));
    }

    #[test]
    fn the_error_bar_fixes_the_significant_figures() {
        let value = Uncertain::new(1234.5, 12.0).unwrap();
        let rendered = value.to_significant().unwrap();
        // Sigma's leading digit is in the tens, so the value is significant
        // down to the units: four figures.
        assert_eq!(rendered.figures(), 4);
    }

    #[test]
    fn an_exact_quantity_claims_every_digit() {
        let value = Uncertain::exact(299_792_458.0).unwrap();
        assert_eq!(value.to_significant().unwrap().figures(), MAX_FIGURES);
    }

    #[test]
    fn adding_a_quantity_to_itself_shows_the_independence_assumption() {
        // Documented sharp edge: the type carries no covariance, so this is
        // sigma*sqrt(2), not 2*sigma. `scaled` is the correct call.
        let value = Uncertain::new(1.0, 1.0).unwrap();
        let doubled_wrongly = value.checked_add(value).unwrap();
        let doubled_rightly = value.scaled(2.0).unwrap();
        assert!((doubled_wrongly.std_dev - math::sqrt(2.0)).abs() < 1e-12);
        assert!((doubled_rightly.std_dev - 2.0).abs() < 1e-12);
    }
}
