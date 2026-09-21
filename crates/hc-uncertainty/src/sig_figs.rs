//! Significant figures: how many digits of a number are actually claimed.
//!
//! "The universe is 13.8 billion years old" is a statement with three
//! significant figures. Storing it as `13_800_000_000.0` and printing it back
//! silently upgrades it to eleven, which is a claim no cosmologist made. This
//! module keeps the digit count alongside the value, propagates it through
//! arithmetic using the ordinary laboratory rules, and renders the number so
//! that what is printed is exactly what is known.
//!
//! # The rules, and what they are worth
//!
//! * Multiplication and division: the result carries the smaller of the two
//!   operands' figure counts.
//! * Addition and subtraction: the result is significant down to the coarser
//!   of the two operands' last significant decimal places, and the figure
//!   count follows from that place and the magnitude of the answer.
//!
//! These are the rules taught for hand computation, and they are a heuristic,
//! not a probability calculus: they systematically over- and under-state the
//! true error in different regimes, and they say nothing about correlated
//! inputs. When the question is "how wrong might this be", use
//! [`crate::quantity::Uncertain`], which propagates an actual variance. This
//! type answers a narrower question: "how many digits am I entitled to show".

use core::fmt;

use hc_core::math;

use crate::error::{UncertaintyError, UncertaintyResult};

/// The most decimal digits an IEEE-754 binary double can distinguish.
///
/// A `f64` has a 53-bit significand, so 2^53 ≈ 9.007·10^15: 15 digits always
/// round-trip and 17 always suffice to identify the value. Claiming more than
/// 17 would be claiming precision the representation does not carry.
pub const MAX_FIGURES: u8 = 17;

/// A real number together with the number of digits that are actually known.
///
/// ```
/// use hc_uncertainty::Significant;
///
/// let age = Significant::new(13.8e9, 3).unwrap();
/// assert_eq!(age.to_string(), "1.38e10");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Significant {
    value: f64,
    figures: u8,
}

impl Significant {
    /// Attach a figure count to a value.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] for NaN or infinity and
    /// [`UncertaintyError::InvalidSignificantFigures`] when `figures` is zero
    /// or above [`MAX_FIGURES`].
    pub fn new(value: f64, figures: u8) -> UncertaintyResult<Self> {
        if !value.is_finite() {
            return Err(UncertaintyError::NotFinite);
        }
        if figures == 0 || figures > MAX_FIGURES {
            return Err(UncertaintyError::InvalidSignificantFigures);
        }
        Ok(Self { value, figures })
    }

    /// A value whose every representable digit is claimed.
    ///
    /// Use this for counts and definitions — the 86 400 seconds of a nominal
    /// day, the defined speed of light — not for measurements.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] for NaN or infinity.
    pub fn exact(value: f64) -> UncertaintyResult<Self> {
        Self::new(value, MAX_FIGURES)
    }

    /// The stored value, unrounded.
    ///
    /// This is the number as it was supplied or as arithmetic produced it. It
    /// may carry digits beyond the significant ones; [`Significant::rounded`]
    /// discards those.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }

    /// How many digits of the value are claimed.
    #[must_use]
    pub const fn figures(self) -> u8 {
        self.figures
    }

    /// The base-ten exponent of the leading digit.
    ///
    /// `1.38e10` has exponent 10, `0.00123` has exponent −3. Zero has no
    /// leading digit; it reports 0 by convention so that callers do not have
    /// to special-case it.
    #[must_use]
    pub fn decimal_exponent(self) -> i32 {
        decimal_exponent(self.value)
    }

    /// The decimal place of the last significant digit, as a power of ten.
    ///
    /// `13.8` with three figures is significant down to 10^−1; `1.38e10` with
    /// three figures only down to 10^8, which is why it must not be printed
    /// as a plain integer.
    #[must_use]
    pub fn last_significant_place(self) -> i32 {
        self.decimal_exponent() - i32::from(self.figures) + 1
    }

    /// The value rounded to its significant digits.
    ///
    /// Rounding is half-away-from-zero, matching `f64::round` and the usual
    /// convention in physical-science tables; bankers' rounding would be
    /// wrong here because these values are reported numbers, not sums.
    #[must_use]
    pub fn rounded(self) -> f64 {
        round_to_figures(self.value, self.figures)
    }

    /// The same value with a different, explicitly stated figure count.
    ///
    /// # Errors
    ///
    /// See [`Significant::new`].
    pub fn with_figures(self, figures: u8) -> UncertaintyResult<Self> {
        Self::new(self.value, figures)
    }

    /// The value with its sign reversed. Negation loses no digits.
    #[must_use]
    pub const fn negated(self) -> Self {
        Self {
            value: -self.value,
            figures: self.figures,
        }
    }

    /// Sum, significant down to the coarser of the two last places.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the sum overflows `f64`.
    pub fn checked_add(self, other: Self) -> UncertaintyResult<Self> {
        self.additive(other, self.value + other.value)
    }

    /// Difference, significant down to the coarser of the two last places.
    ///
    /// Subtracting two close numbers destroys figures — `1.0000` minus
    /// `0.9999` keeps one — and this is exactly the catastrophic
    /// cancellation the rule is meant to make visible.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the difference overflows
    /// `f64`.
    pub fn checked_sub(self, other: Self) -> UncertaintyResult<Self> {
        self.additive(other, self.value - other.value)
    }

    /// Product, carrying the smaller figure count.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the product overflows
    /// `f64`.
    pub fn checked_mul(self, other: Self) -> UncertaintyResult<Self> {
        Self::new(self.value * other.value, self.figures.min(other.figures))
    }

    /// Quotient, carrying the smaller figure count.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::DivideByZero`] for a zero divisor and
    /// [`UncertaintyError::NotFinite`] when the quotient overflows `f64`.
    pub fn checked_div(self, other: Self) -> UncertaintyResult<Self> {
        if other.value == 0.0 {
            return Err(UncertaintyError::DivideByZero);
        }
        Self::new(self.value / other.value, self.figures.min(other.figures))
    }

    /// Raise to an integer power, keeping this value's figure count.
    ///
    /// Repeated multiplication of a number by itself does not lose figures
    /// under the laboratory rule, because both operands carry the same count.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the power overflows
    /// `f64`.
    pub fn checked_powi(self, exponent: i32) -> UncertaintyResult<Self> {
        Self::new(math::powf(self.value, f64::from(exponent)), self.figures)
    }

    /// Shared tail of addition and subtraction.
    ///
    /// The figure count of the answer is recovered from the coarser of the
    /// two operands' last significant places rather than from their figure
    /// counts directly, which is what makes `100.0 + 0.001` keep four rather
    /// than seven.
    fn additive(self, other: Self, result: f64) -> UncertaintyResult<Self> {
        if !result.is_finite() {
            return Err(UncertaintyError::NotFinite);
        }
        let place = self
            .last_significant_place()
            .max(other.last_significant_place());
        let figures = figures_for(result, place);
        Self::new(result, figures)
    }
}

/// The base-ten exponent of the leading digit of `value`.
///
/// `log10` is used for the first guess and then corrected, because the
/// library function is only correctly rounded to within an ulp and a value
/// such as 1000.0 can otherwise report an exponent of 2.
fn decimal_exponent(value: f64) -> i32 {
    let magnitude = math::abs(value);
    if magnitude == 0.0 {
        return 0;
    }
    let mut exponent = math::floor(math::log10(magnitude)) as i32;
    if math::powf(10.0, f64::from(exponent + 1)) <= magnitude {
        exponent += 1;
    } else if math::powf(10.0, f64::from(exponent)) > magnitude {
        exponent -= 1;
    }
    exponent
}

/// How many figures a value has when it is significant down to `place`.
///
/// Clamped to the range `Significant` accepts: a result whose last
/// significant place lies above its own leading digit still gets one figure,
/// because reporting a bare "no digits are meaningful" is less useful to a
/// caller than reporting the order of magnitude.
fn figures_for(value: f64, place: i32) -> u8 {
    let figures = decimal_exponent(value) - place + 1;
    figures.clamp(1, i32::from(MAX_FIGURES)) as u8
}

/// Round `value` so that only `figures` leading digits survive.
fn round_to_figures(value: f64, figures: u8) -> f64 {
    if value == 0.0 {
        return 0.0;
    }
    let shift = i32::from(figures) - 1 - decimal_exponent(value);
    let scale = math::powf(10.0, f64::from(shift));
    if !scale.is_finite() || scale == 0.0 {
        return value;
    }
    math::round(value * scale) / scale
}

impl fmt::Display for Significant {
    /// Render the value at its true precision, and no better.
    ///
    /// Plain decimal notation is used when every digit it shows is either
    /// significant or a placeholder zero after the point. When the value is
    /// large enough that plain notation would need non-significant zeros
    /// before the point — 13.8 billion, whose `13800000000` claims eleven
    /// digits — or small enough that it would need more than four leading
    /// zeros, scientific notation is used instead. This is the convention of
    /// the SI Brochure and of *Physical Review*'s style guide, and it is the
    /// only notation in which "three significant figures" is unambiguous.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let figures = usize::from(self.figures);
        if self.value == 0.0 {
            return write!(f, "{:.*}", figures - 1, 0.0);
        }
        let exponent = self.decimal_exponent();
        if exponent >= i32::from(self.figures) || exponent < -4 {
            write!(f, "{:.*e}", figures - 1, self.value)
        } else {
            let decimals = (i32::from(self.figures) - 1 - exponent).max(0) as usize;
            write!(f, "{:.*}", decimals, self.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::string::ToString as _;

    #[test]
    fn thirteen_point_eight_billion_years_keeps_three_figures() {
        let age = Significant::new(13.8e9, 3).unwrap();
        assert_eq!(age.figures(), 3);
        #[cfg(feature = "alloc")]
        assert_eq!(age.to_string(), "1.38e10");
    }

    #[test]
    fn a_three_figure_value_never_renders_as_eleven_digits() {
        let age = Significant::new(13.8e9, 3).unwrap();
        #[cfg(feature = "alloc")]
        assert_ne!(age.to_string(), "13800000000");
        assert_eq!(age.last_significant_place(), 8);
    }

    #[test]
    fn small_magnitudes_render_in_plain_decimal_until_they_get_silly() {
        let a = Significant::new(0.00123, 3).unwrap();
        let b = Significant::new(0.000_012_3, 3).unwrap();
        #[cfg(feature = "alloc")]
        {
            assert_eq!(a.to_string(), "0.00123");
            assert_eq!(b.to_string(), "1.23e-5");
        }
        let _ = (a, b);
    }

    #[test]
    fn modest_magnitudes_render_in_plain_decimal() {
        #[cfg(feature = "alloc")]
        {
            assert_eq!(Significant::new(13.8, 3).unwrap().to_string(), "13.8");
            assert_eq!(Significant::new(1.0, 4).unwrap().to_string(), "1.000");
            assert_eq!(Significant::new(-2.5, 2).unwrap().to_string(), "-2.5");
            assert_eq!(Significant::new(100.0, 3).unwrap().to_string(), "100");
        }
    }

    #[test]
    fn zero_renders_with_its_claimed_decimals() {
        #[cfg(feature = "alloc")]
        assert_eq!(Significant::new(0.0, 3).unwrap().to_string(), "0.00");
        assert_eq!(Significant::new(0.0, 3).unwrap().decimal_exponent(), 0);
    }

    #[test]
    fn the_decimal_exponent_is_exact_at_powers_of_ten() {
        for exponent in -12..=12 {
            let value = math::powf(10.0, f64::from(exponent));
            assert_eq!(decimal_exponent(value), exponent, "at 10^{exponent}");
            assert_eq!(decimal_exponent(-value), exponent, "at -10^{exponent}");
        }
    }

    #[test]
    fn the_decimal_exponent_brackets_every_magnitude() {
        let samples = [1.0, 9.999, 10.0, 99.999, 1e-7, 6.02e23, 3.5e-18];
        for value in samples {
            let exponent = decimal_exponent(value);
            let low = math::powf(10.0, f64::from(exponent));
            let high = math::powf(10.0, f64::from(exponent + 1));
            assert!(low <= value && value < high, "{value} not in decade");
        }
    }

    #[test]
    fn rounding_discards_digits_beyond_the_claim() {
        assert!((Significant::new(1.23456, 3).unwrap().rounded() - 1.23).abs() < 1e-12);
        assert!((Significant::new(9.87654, 2).unwrap().rounded() - 9.9).abs() < 1e-12);
        assert!((Significant::new(123_456.0, 2).unwrap().rounded() - 120_000.0).abs() < 1e-6);
    }

    #[test]
    fn rounding_is_half_away_from_zero() {
        assert!((Significant::new(2.5, 1).unwrap().rounded() - 3.0).abs() < 1e-12);
        assert!((Significant::new(-2.5, 1).unwrap().rounded() + 3.0).abs() < 1e-12);
    }

    #[test]
    fn rounding_a_value_already_at_precision_changes_nothing() {
        for figures in 1..=9u8 {
            let value = 1.234_567_89_f64;
            let once = round_to_figures(value, figures);
            let twice = round_to_figures(once, figures);
            assert!((once - twice).abs() < 1e-15, "unstable at {figures}");
        }
    }

    #[test]
    fn multiplication_keeps_the_weaker_operand() {
        let a = Significant::new(2.0, 1).unwrap();
        let b = Significant::new(3.000, 4).unwrap();
        let product = a.checked_mul(b).unwrap();
        assert_eq!(product.figures(), 1);
        assert!((product.value() - 6.0).abs() < 1e-12);
    }

    #[test]
    fn division_keeps_the_weaker_operand() {
        let a = Significant::new(1.0, 2).unwrap();
        let b = Significant::new(3.0, 5).unwrap();
        let quotient = a.checked_div(b).unwrap();
        assert_eq!(quotient.figures(), 2);
        #[cfg(feature = "alloc")]
        assert_eq!(quotient.to_string(), "0.33");
    }

    #[test]
    fn division_by_zero_is_reported_rather_than_returning_infinity() {
        let a = Significant::new(1.0, 3).unwrap();
        let zero = Significant::new(0.0, 3).unwrap();
        assert_eq!(a.checked_div(zero), Err(UncertaintyError::DivideByZero));
    }

    #[test]
    fn addition_is_limited_by_the_coarser_decimal_place() {
        // 100.0 knows nothing past the tenths; 0.001 cannot rescue it.
        let coarse = Significant::new(100.0, 4).unwrap();
        let fine = Significant::new(0.001, 1).unwrap();
        let sum = coarse.checked_add(fine).unwrap();
        assert_eq!(sum.figures(), 4);
        #[cfg(feature = "alloc")]
        assert_eq!(sum.to_string(), "100.0");
    }

    #[test]
    fn subtraction_of_near_equals_destroys_figures() {
        let a = Significant::new(1.0000, 5).unwrap();
        let b = Significant::new(0.9999, 4).unwrap();
        let difference = a.checked_sub(b).unwrap();
        assert_eq!(difference.figures(), 1);
    }

    #[test]
    fn negation_preserves_the_figure_count() {
        let value = Significant::new(6.674e-11, 4).unwrap();
        let negated = value.negated();
        assert_eq!(negated.figures(), 4);
        assert!((negated.value() + 6.674e-11).abs() < 1e-25);
    }

    #[test]
    fn integer_powers_keep_the_figure_count() {
        let value = Significant::new(2.50, 3).unwrap();
        let squared = value.checked_powi(2).unwrap();
        assert_eq!(squared.figures(), 3);
        assert!((squared.value() - 6.25).abs() < 1e-12);
    }

    #[test]
    fn a_figure_count_of_zero_or_eighteen_is_rejected() {
        assert_eq!(
            Significant::new(1.0, 0),
            Err(UncertaintyError::InvalidSignificantFigures)
        );
        assert_eq!(
            Significant::new(1.0, 18),
            Err(UncertaintyError::InvalidSignificantFigures)
        );
    }

    #[test]
    fn non_finite_values_are_rejected() {
        assert_eq!(
            Significant::new(f64::NAN, 3),
            Err(UncertaintyError::NotFinite)
        );
        assert_eq!(
            Significant::new(f64::INFINITY, 3),
            Err(UncertaintyError::NotFinite)
        );
    }

    #[test]
    fn overflow_in_a_product_is_reported_as_non_finite() {
        let big = Significant::new(1e300, 3).unwrap();
        assert_eq!(big.checked_mul(big), Err(UncertaintyError::NotFinite));
    }

    #[test]
    fn an_exact_value_claims_every_representable_digit() {
        let day = Significant::exact(86_400.0).unwrap();
        assert_eq!(day.figures(), MAX_FIGURES);
        assert!((day.rounded() - 86_400.0).abs() < 1e-9);
    }

    #[test]
    fn with_figures_restates_the_claim_without_moving_the_value() {
        let value = Significant::new(1.23456, 6).unwrap();
        let coarser = value.with_figures(3).unwrap();
        assert!((coarser.value() - 1.23456).abs() < 1e-15);
        assert!((coarser.rounded() - 1.23).abs() < 1e-12);
    }

    #[test]
    fn rendering_round_trips_through_a_reparsed_magnitude() {
        // Whatever notation is chosen, the rendered digits must round-trip to
        // the rounded value; that is the whole point of the type.
        #[cfg(feature = "alloc")]
        for (value, figures) in [(13.8e9, 3u8), (0.00123, 3), (1.0, 1), (-45.67, 4)] {
            let rendered = Significant::new(value, figures).unwrap().to_string();
            let parsed: f64 = rendered.parse().unwrap();
            let expected = round_to_figures(value, figures);
            assert!(
                (parsed - expected).abs() <= math::abs(expected) * 1e-12,
                "{rendered} != {expected}"
            );
        }
    }
}
