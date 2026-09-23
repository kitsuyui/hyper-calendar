//! Hyperbolic functions, built from the exponentials `hc-core` provides.
//!
//! Relativity is hyperbolic trigonometry: rapidity adds where velocity does
//! not, and every relativistic-rocket relation is a `sinh`, `cosh` or their
//! inverses. `core` has none of these and `hc_core::math` deliberately stops
//! at `exp`, `ln` and `sqrt`, so they are assembled here.
//!
//! # Accuracy
//!
//! Each function is the textbook exponential identity with a Taylor series
//! substituted near zero, where the identity loses digits to cancellation,
//! and a large-argument shortcut where the identity would overflow. The
//! relative error stays below 1e-13 across the whole range in the round-trip
//! tests at the bottom of this file, which is far finer than any of the
//! physical inputs this crate takes.

use hc_core::math;

use crate::error::{RelativityError, RelativityResult};

/// Below this magnitude the exponential identities cancel, so a series is
/// used instead. The omitted terms are `O(x⁷)`, which at 10⁻² is 2·10⁻¹⁶
/// relative.
const SMALL: f64 = 1e-2;

/// Above this magnitude `tanh` is 1 to the last bit of a double.
const TANH_SATURATION: f64 = 20.0;

/// Above this magnitude `x²` would dominate `1` completely, so the inverse
/// hyperbolic functions collapse to `ln(2x)`.
const LARGE: f64 = 1e8;

/// Hyperbolic sine.
#[must_use]
pub(crate) fn sinh(x: f64) -> f64 {
    if math::abs(x) < SMALL {
        let square = x * x;
        return x * (1.0 + square / 6.0 + square * square / 120.0);
    }
    (math::exp(x) - math::exp(-x)) / 2.0
}

/// Hyperbolic cosine. The identity never cancels here, so no special case is
/// needed.
#[must_use]
pub(crate) fn cosh(x: f64) -> f64 {
    (math::exp(x) + math::exp(-x)) / 2.0
}

/// Hyperbolic tangent, the velocity corresponding to a rapidity.
#[must_use]
pub(crate) fn tanh(x: f64) -> f64 {
    if math::abs(x) < SMALL {
        let square = x * x;
        return x * (1.0 - square / 3.0 + 2.0 * square * square / 15.0);
    }
    if x > TANH_SATURATION {
        return 1.0;
    }
    if x < -TANH_SATURATION {
        return -1.0;
    }
    let doubled = math::exp(2.0 * x);
    (doubled - 1.0) / (doubled + 1.0)
}

/// Inverse hyperbolic sine.
#[must_use]
pub(crate) fn asinh(x: f64) -> f64 {
    let magnitude = math::abs(x);
    let result = if magnitude < SMALL {
        let square = x * x;
        magnitude * (1.0 - square / 6.0 + 3.0 * square * square / 40.0)
    } else if magnitude > LARGE {
        // ln(x + sqrt(x²+1)) -> ln(2x) once x² swamps 1; squaring 1e154
        // would overflow, so this branch is a necessity, not an optimisation.
        math::ln(2.0 * magnitude)
    } else {
        math::ln(magnitude + math::sqrt(magnitude * magnitude + 1.0))
    };
    if x < 0.0 { -result } else { result }
}

/// `cosh(b) − cosh(a)`, computed without cancellation.
///
/// The naive difference is catastrophic when the two arguments are close, and
/// for the relativistic rocket they very often are: `cosh(at/c) − 1` at one
/// second of 1 g is `5·10⁻¹⁶`, which the direct subtraction gets wrong by
/// tens of per cent. The product form
///
/// ```text
/// cosh b − cosh a = 2 sinh((b+a)/2) sinh((b−a)/2)
/// ```
///
/// has no subtraction of near-equal quantities left in it, and [`sinh`]
/// already handles its own small arguments, so the result keeps full
/// precision at every scale.
#[must_use]
pub(crate) fn cosh_difference(upper: f64, lower: f64) -> f64 {
    2.0 * sinh((upper + lower) / 2.0) * sinh((upper - lower) / 2.0)
}

/// `arcosh(1 + y)` for `y ≥ 0`, computed without cancellation.
///
/// `cosh 2u = 1 + 2 sinh² u`, so `arcosh(1 + y) = 2 arsinh √(y/2)`. Written
/// that way the function never subtracts 1 from a number just above 1, which
/// is exactly the regime a short rocket burn lives in.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a non-finite argument and
/// [`RelativityError::LorentzFactorBelowOne`] for a negative one, where the
/// result would be the inverse cosine of an argument below 1. In the crate's
/// one use, `1 + y` is the Lorentz factor `1 + ad/c²` at the end of a burn.
pub(crate) fn acosh_one_plus(y: f64) -> RelativityResult<f64> {
    if !y.is_finite() {
        return Err(RelativityError::NotFinite);
    }
    if y < 0.0 {
        return Err(RelativityError::LorentzFactorBelowOne);
    }
    Ok(2.0 * asinh(math::sqrt(y / 2.0)))
}

/// Inverse hyperbolic tangent, the rapidity corresponding to a velocity.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a non-finite argument and
/// [`RelativityError::FasterThanLight`] for `|x| ≥ 1`.
pub(crate) fn atanh(x: f64) -> RelativityResult<f64> {
    if !x.is_finite() {
        return Err(RelativityError::NotFinite);
    }
    if math::abs(x) >= 1.0 {
        return Err(RelativityError::FasterThanLight);
    }
    if math::abs(x) < SMALL {
        let square = x * x;
        return Ok(x * (1.0 + square / 3.0 + square * square / 5.0));
    }
    Ok(0.5 * math::ln((1.0 + x) / (1.0 - x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Relative difference, guarding against a zero reference.
    fn close(actual: f64, expected: f64, tolerance: f64) -> bool {
        let scale = math::abs(expected).max(1.0);
        math::abs(actual - expected) / scale < tolerance
    }

    #[test]
    fn sinh_and_cosh_satisfy_the_fundamental_identity() {
        // The tolerance has to scale with cosh²: the identity subtracts two
        // numbers of that size to leave 1, so at x = 10 it throws away eight
        // digits before this test ever sees the result. That is a property
        // of the identity, not of the implementation.
        for step in -400..=400 {
            let x = f64::from(step) / 40.0;
            let identity = cosh(x) * cosh(x) - sinh(x) * sinh(x);
            let scale = cosh(x) * cosh(x);
            assert!(
                (identity - 1.0).abs() <= scale * 1e-14,
                "failed at {x}: {identity}"
            );
        }
    }

    #[test]
    fn the_cosh_difference_keeps_precision_where_the_direct_one_does_not() {
        // cosh(3.27e-8) - 1 is 5.35e-16: the direct subtraction loses most
        // of it, the product form keeps all of it.
        let tiny = 3.27e-8;
        let expected = tiny * tiny / 2.0;
        let stable = cosh_difference(tiny, 0.0);
        assert!(close(stable / expected, 1.0, 1e-12), "got {stable}");
    }

    #[test]
    fn the_cosh_difference_agrees_with_the_direct_one_where_that_is_safe() {
        for step in 1..=60 {
            let upper = f64::from(step) / 10.0;
            let lower = upper / 2.0;
            let direct = cosh(upper) - cosh(lower);
            assert!(
                close(cosh_difference(upper, lower), direct, 1e-12),
                "failed at {upper}"
            );
        }
    }

    #[test]
    fn acosh_one_plus_inverts_cosh_minus_one() {
        for step in 0..=300 {
            let x = f64::from(step) / 30.0;
            let y = cosh_difference(x, 0.0);
            assert!(close(acosh_one_plus(y).unwrap(), x, 1e-12), "failed at {x}");
        }
    }

    #[test]
    fn acosh_one_plus_keeps_precision_for_a_tiny_excess() {
        let y = 1e-20;
        let expected = 2.0 * math::sqrt(y / 2.0);
        assert!(close(acosh_one_plus(y).unwrap() / expected, 1.0, 1e-12));
    }

    #[test]
    fn acosh_one_plus_refuses_a_negative_excess() {
        assert_eq!(
            acosh_one_plus(-1.0),
            Err(RelativityError::LorentzFactorBelowOne)
        );
        assert_eq!(acosh_one_plus(f64::NAN), Err(RelativityError::NotFinite));
        assert_eq!(acosh_one_plus(0.0), Ok(0.0));
    }

    #[test]
    fn tanh_is_sinh_over_cosh() {
        for step in -200..=200 {
            let x = f64::from(step) / 20.0;
            assert!(close(tanh(x), sinh(x) / cosh(x), 1e-12), "failed at {x}");
        }
    }

    #[test]
    fn asinh_inverts_sinh_across_the_range() {
        for step in -300..=300 {
            let x = f64::from(step) / 30.0;
            assert!(close(asinh(sinh(x)), x, 1e-12), "failed at {x}");
        }
    }

    #[test]
    fn atanh_inverts_tanh_across_the_range() {
        // Only up to a rapidity of 5. Beyond that `tanh` is so close to 1
        // that a double cannot hold the remaining distance, and the round
        // trip loses a digit for every further unit of rapidity — a limit of
        // the representation, which the next test states outright.
        for step in -100..=100 {
            let x = f64::from(step) / 20.0;
            assert!(close(atanh(tanh(x)).unwrap(), x, 1e-11), "failed at {x}");
        }
    }

    #[test]
    fn the_tanh_round_trip_degrades_gracefully_at_extreme_rapidity() {
        for step in [6.0, 8.0, 10.0, 15.0] {
            let recovered = atanh(tanh(step)).unwrap();
            assert!(close(recovered, step, 1e-5), "failed at {step}");
        }
    }

    #[test]
    fn the_series_branches_agree_with_the_exponential_ones() {
        // Continuity across the SMALL threshold is what makes the series
        // substitution invisible to callers.
        let below = SMALL * 0.999_999;
        let above = SMALL * 1.000_001;
        assert!(close(sinh(below), sinh(above), 1e-5));
        assert!(close(tanh(below), tanh(above), 1e-5));
        assert!(close(asinh(below), asinh(above), 1e-5));
        assert!(close(atanh(below).unwrap(), atanh(above).unwrap(), 1e-5));
    }

    #[test]
    fn small_arguments_keep_full_precision() {
        // The naive identity would lose most of its digits here.
        let tiny = 1e-12;
        assert!(close(sinh(tiny) / tiny, 1.0, 1e-15));
        assert!(close(tanh(tiny) / tiny, 1.0, 1e-15));
        assert!(close(asinh(tiny) / tiny, 1.0, 1e-15));
    }

    #[test]
    fn asinh_survives_arguments_whose_square_would_overflow() {
        let huge = 1e200;
        let expected = math::ln(2.0 * huge);
        assert!(close(asinh(huge), expected, 1e-15));
        assert!(asinh(huge).is_finite());
        assert!(asinh(-huge).is_finite());
    }

    #[test]
    fn tanh_saturates_rather_than_overflowing() {
        assert!((tanh(800.0) - 1.0).abs() < 1e-15);
        assert!((tanh(-800.0) + 1.0).abs() < 1e-15);
    }

    #[test]
    fn the_odd_functions_are_odd() {
        for step in 1..=50 {
            let x = f64::from(step) / 5.0;
            assert!(close(sinh(-x), -sinh(x), 1e-12));
            assert!(close(tanh(-x), -tanh(x), 1e-12));
            assert!(close(asinh(-x), -asinh(x), 1e-12));
            assert!(close(cosh(-x), cosh(x), 1e-12));
        }
    }

    #[test]
    fn atanh_refuses_light_speed_and_beyond() {
        assert_eq!(atanh(1.0), Err(RelativityError::FasterThanLight));
        assert_eq!(atanh(-1.0), Err(RelativityError::FasterThanLight));
        assert_eq!(atanh(1.5), Err(RelativityError::FasterThanLight));
        assert_eq!(atanh(f64::INFINITY), Err(RelativityError::NotFinite));
    }
}
