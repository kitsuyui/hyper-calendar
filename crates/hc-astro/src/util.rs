//! Small numeric helpers shared by the astronomical series.
//!
//! These exist because `core` has no floating-point `rem_euclid` and because
//! every reference in this crate writes its series as a polynomial with
//! ascending coefficients. Keeping both in one place means the series read
//! the way they are printed in Meeus and in *Calendrical Calculations*.

use hc_core::math::floor;

/// Evaluate a polynomial whose coefficients are given in ascending order.
///
/// `poly(x, &[a, b, c])` is `a + b*x + c*x^2`, computed by Horner's rule so
/// that the high powers do not lose the low ones.
#[must_use]
pub(crate) fn poly(x: f64, coefficients: &[f64]) -> f64 {
    let mut accumulator = 0.0;
    for coefficient in coefficients.iter().rev() {
        accumulator = accumulator * x + coefficient;
    }
    accumulator
}

/// The non-negative remainder of `x` divided by `y`, for `y > 0`.
///
/// `f64::rem_euclid` lives in `std`, and this crate has to work without it.
#[must_use]
pub(crate) fn modulo(x: f64, y: f64) -> f64 {
    x - y * floor(x / y)
}

/// Reduce an angle to `(-180, 180]`, which is what an hour angle or a signed
/// angular difference wants.
#[must_use]
pub(crate) fn signed_degrees(degrees: f64) -> f64 {
    modulo(degrees + 180.0, 360.0) - 180.0
}

/// The larger of two finite numbers, without `std`'s `f64::max`.
#[must_use]
pub(crate) fn max_of(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// Pull a value back into `[low, high]`.
///
/// Every inverse trigonometric call in this crate goes through this, because
/// a cosine that rounds to 1.0000000000000002 turns into a silent `NaN` that
/// then propagates into a date.
#[must_use]
pub(crate) fn clamp(value: f64, low: f64, high: f64) -> f64 {
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horner_evaluates_ascending_coefficients() {
        // 1 + 2x + 3x^2 at x = 2 is 1 + 4 + 12.
        assert!((poly(2.0, &[1.0, 2.0, 3.0]) - 17.0).abs() < 1e-12);
        assert!((poly(5.0, &[]) - 0.0).abs() < 1e-12);
        assert!((poly(5.0, &[7.0]) - 7.0).abs() < 1e-12);
    }

    #[test]
    fn modulo_is_non_negative_for_negative_inputs() {
        assert!((modulo(-1.0, 360.0) - 359.0).abs() < 1e-12);
        assert!((modulo(361.0, 360.0) - 1.0).abs() < 1e-12);
        assert!((modulo(-0.25, 1.0) - 0.75).abs() < 1e-12);
    }

    #[test]
    fn clamping_keeps_rounding_noise_out_of_the_inverse_trigonometry() {
        assert!((clamp(1.000_000_000_000_000_2, -1.0, 1.0) - 1.0).abs() < 1e-18);
        assert!((clamp(-2.0, -1.0, 1.0) + 1.0).abs() < 1e-18);
        assert!((clamp(0.25, -1.0, 1.0) - 0.25).abs() < 1e-18);
        assert!((max_of(3.0, -1.0) - 3.0).abs() < 1e-18);
    }

    #[test]
    fn signed_degrees_centres_an_angle_on_zero() {
        assert!((signed_degrees(350.0) + 10.0).abs() < 1e-12);
        assert!((signed_degrees(10.0) - 10.0).abs() < 1e-12);
        assert!((signed_degrees(190.0) + 170.0).abs() < 1e-12);
    }
}
