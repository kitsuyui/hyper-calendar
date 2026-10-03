//! Floating-point math that works with or without `std`.
//!
//! `core` deliberately omits transcendental functions, so a `no_std` build has
//! to route them somewhere. Enabling `std` (the default) uses the platform
//! implementations; enabling `libm` uses the portable software ones. The rest
//! of the workspace calls these wrappers and never `f64::sin` directly, so a
//! `no_std` port never has to be re-audited function by function.
//!
//! # A `no_std` build must enable `libm`
//!
//! Enabling neither is a **compile error**, and that is the point of the
//! guard below. A run-time `panic!` instead would let such a build compile
//! cleanly and then fail the first time anything asked for a sine, in a
//! configuration no test had exercised.
//!
//! A missing feature is a fact about the build, not about the input, so it
//! belongs at compile time where the person choosing the features will see
//! it.
//!
//! The rounding helpers — [`floor`], [`ceil`], [`trunc`], [`round`] and
//! [`abs`] — are implemented here in plain arithmetic and need neither
//! feature. Only the transcendentals do.

#![allow(missing_docs)]

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
compile_error!(
    "hyper-calendar needs floating-point math: enable the `std` feature (the \
     default) or, for a `no_std` build, the `libm` feature."
);

macro_rules! unary {
    ($($name:ident),* $(,)?) => {
        $(
            #[inline]
            #[must_use]
            pub fn $name(x: f64) -> f64 {
                #[cfg(feature = "std")]
                { f64::$name(x) }
                #[cfg(all(not(feature = "std"), feature = "libm"))]
                { libm::$name(x) }
                #[cfg(all(not(feature = "std"), not(feature = "libm")))]
                { let _ = x; panic!(concat!("hc-core::math::", stringify!($name), " requires the `std` or `libm` feature")) }
            }
        )*
    };
}

unary!(sin, cos, tan, asin, acos, atan, sqrt, exp, cbrt);

/// Round towards negative infinity.
///
/// Plain arithmetic, so it needs neither `std` nor `libm`. Values beyond
/// `i64`'s range are already integral in `f64`, so they are returned as they
/// are.
#[inline]
#[must_use]
pub fn floor(x: f64) -> f64 {
    if !x.is_finite() || x.abs() >= 9.223_372_036_854_776e18 {
        return x;
    }
    let truncated = trunc(x);
    if x < 0.0 && truncated != x {
        truncated - 1.0
    } else {
        truncated
    }
}

/// Round towards positive infinity.
#[inline]
#[must_use]
pub fn ceil(x: f64) -> f64 {
    if !x.is_finite() || x.abs() >= 9.223_372_036_854_776e18 {
        return x;
    }
    let truncated = trunc(x);
    if x > 0.0 && truncated != x {
        truncated + 1.0
    } else {
        truncated
    }
}

/// Discard the fractional part, rounding towards zero.
#[inline]
#[must_use]
pub fn trunc(x: f64) -> f64 {
    if !x.is_finite() || x.abs() >= 9.223_372_036_854_776e18 {
        return x;
    }
    x as i64 as f64
}

#[inline]
#[must_use]
pub fn ln(x: f64) -> f64 {
    #[cfg(feature = "std")]
    {
        f64::ln(x)
    }
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    {
        libm::log(x)
    }
    #[cfg(all(not(feature = "std"), not(feature = "libm")))]
    {
        let _ = x;
        panic!("hc-core::math::ln requires the `std` or `libm` feature")
    }
}

#[inline]
#[must_use]
pub fn log10(x: f64) -> f64 {
    #[cfg(feature = "std")]
    {
        f64::log10(x)
    }
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    {
        libm::log10(x)
    }
    #[cfg(all(not(feature = "std"), not(feature = "libm")))]
    {
        let _ = x;
        panic!("hc-core::math::log10 requires the `std` or `libm` feature")
    }
}

#[inline]
#[must_use]
pub fn atan2(y: f64, x: f64) -> f64 {
    #[cfg(feature = "std")]
    {
        f64::atan2(y, x)
    }
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    {
        libm::atan2(y, x)
    }
    #[cfg(all(not(feature = "std"), not(feature = "libm")))]
    {
        let _ = (y, x);
        panic!("hc-core::math::atan2 requires the `std` or `libm` feature")
    }
}

#[inline]
#[must_use]
pub fn powf(x: f64, y: f64) -> f64 {
    #[cfg(feature = "std")]
    {
        f64::powf(x, y)
    }
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    {
        libm::pow(x, y)
    }
    #[cfg(all(not(feature = "std"), not(feature = "libm")))]
    {
        let _ = (x, y);
        panic!("hc-core::math::powf requires the `std` or `libm` feature")
    }
}

/// Round half away from zero.
///
/// Plain arithmetic, so it needs neither `std` nor `libm`.
#[inline]
#[must_use]
pub fn round(x: f64) -> f64 {
    if !x.is_finite() || x.abs() >= 9.223_372_036_854_776e18 {
        return x;
    }
    if x < 0.0 {
        -floor(-x + 0.5)
    } else {
        floor(x + 0.5)
    }
}

#[inline]
#[must_use]
pub fn abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

/// The remainder of `x` divided by `y`, with the sign of `y`: in `[0, y)`
/// for `y > 0`, as *Calendrical Calculations* defines `mod` for reals.
///
/// `f64::rem_euclid` lives in `std`, and every crate of the workspace has to
/// build without it. This is `x − y·⌊x/y⌋` exactly as written, with no
/// correction afterwards, so rounding can leave the result a hair outside
/// `[0, y)`; [`normalize_degrees`] keeps a rounded-up full turn below 360.
#[inline]
#[must_use]
pub fn modulo(x: f64, y: f64) -> f64 {
    x - y * floor(x / y)
}

/// `x` reduced into `1..=n`, the "adjusted modulo" of *Calendrical
/// Calculations*: `n` where [`i64::rem_euclid`] gives 0.
///
/// This is how a count that runs from 1 — a month, a day of a cycle, a
/// year of the sexagenary cycle — wraps.
#[inline]
#[must_use]
pub const fn amod(x: i64, n: i64) -> i64 {
    (x - 1).rem_euclid(n) + 1
}

/// The fractional part of `x`, in `[0, 1)` for negative `x` as well:
/// `x − ⌊x⌋`.
#[inline]
#[must_use]
pub fn fract(x: f64) -> f64 {
    x - floor(x)
}

/// An angle in degrees reduced to `(−180, 180]`: the signed difference an
/// hour angle or a difference of two longitudes wants.
///
/// The interval is half-open on the negative side, so an angle of exactly
/// ±180° comes back as `+180`, never `−180`. Between the endpoints the
/// result is [`modulo`]`(degrees, 360)`, less 360 above 180, so an angle
/// already in `[0, 180]` comes back unchanged to the last bit.
#[inline]
#[must_use]
pub fn signed_degrees(degrees: f64) -> f64 {
    let wrapped = normalize_degrees(degrees);
    if wrapped > 180.0 {
        wrapped - 360.0
    } else {
        wrapped
    }
}

/// Evaluate a polynomial whose coefficients are given in ascending order.
///
/// `poly(x, &[a, b, c])` is `a + b·x + c·x²`, computed by Horner's rule so
/// that the high powers do not lose the low ones, and the series read the
/// way they are printed in Meeus and in *Calendrical Calculations*. No
/// coefficients is the zero polynomial.
#[inline]
#[must_use]
pub fn poly(x: f64, coefficients: &[f64]) -> f64 {
    let mut accumulator = 0.0;
    for coefficient in coefficients.iter().rev() {
        accumulator = accumulator * x + coefficient;
    }
    accumulator
}

/// The largest `f64` below 360, 360 − 2⁻⁴⁴ ≈ 359.999 999 999 999 94: where
/// an angle a hair below a full turn lands when it must stay below one.
const LAST_BELOW_FULL_TURN: f64 = f64::from_bits(360.0f64.to_bits() - 1);

/// Reduce an angle in degrees to `[0, 360)`.
///
/// [`modulo`] on a tiny negative angle gives `360 − ε`, and when ε is below
/// half the spacing of the doubles under 360 the subtraction rounds up to
/// exactly 360: `-1e-14` came back as `360.0`, outside the interval this
/// function promises and one past the last index of every table a caller
/// divides it into. Such an angle is just *below* a full turn, so it is
/// returned as the largest double below 360 and not as 0: a caller asking
/// how far it still is to a target, `normalize_degrees(target − angle)`, must
/// get "nearly a turn" for an angle that has just passed the target, not
/// "now", and a longitude that has just passed 0 must stay in the last
/// sign and not jump to the first.
///
/// A negative result of [`modulo`] occurs only for a subnormal angle, whose
/// quotient by 360 underflows to −0 so that nothing is subtracted (checked
/// over 6·10⁶ multiples of 360 and five ulps either side of each, which
/// give none); it is lifted by one turn. NaN and the infinities come back as
/// NaN. An angle above about 10¹⁷ degrees has no digit left below the whole
/// turns, and its reduction is not meaningful.
#[inline]
#[must_use]
pub fn normalize_degrees(degrees: f64) -> f64 {
    let reduced = modulo(degrees, 360.0);
    let lifted = if reduced < 0.0 {
        reduced + 360.0
    } else {
        reduced
    };
    if lifted >= 360.0 {
        LAST_BELOW_FULL_TURN
    } else {
        lifted
    }
}

/// Degrees to radians.
pub const DEG_TO_RAD: f64 = core::f64::consts::PI / 180.0;

/// Radians to degrees.
pub const RAD_TO_DEG: f64 = 180.0 / core::f64::consts::PI;

/// Sine of an angle given in degrees.
#[inline]
#[must_use]
pub fn sin_deg(degrees: f64) -> f64 {
    sin(degrees * DEG_TO_RAD)
}

/// Cosine of an angle given in degrees.
#[inline]
#[must_use]
pub fn cos_deg(degrees: f64) -> f64 {
    cos(degrees * DEG_TO_RAD)
}

/// Tangent of an angle given in degrees.
#[inline]
#[must_use]
pub fn tan_deg(degrees: f64) -> f64 {
    tan(degrees * DEG_TO_RAD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_negative_angles() {
        assert!((normalize_degrees(-10.0) - 350.0).abs() < 1e-9);
        assert!((normalize_degrees(370.0) - 10.0).abs() < 1e-9);
        assert!((normalize_degrees(0.0)).abs() < 1e-9);
    }

    /// An angle a hair below zero is a hair below a full turn. In a double
    /// the nearest value below 360 is 5.7·10⁻¹⁴ away, so anything closer
    /// than half of that rounds to 360 itself, which is not in `[0, 360)`.
    #[test]
    fn a_tiny_negative_angle_never_comes_back_as_a_full_turn() {
        for tiny in [-1e-14, -1e-20, -5e-324, -2.8e-14, -f64::MIN_POSITIVE] {
            let reduced = normalize_degrees(tiny);
            assert_eq!(reduced, 359.999_999_999_999_94, "{tiny:e}");
            assert!((0.0..360.0).contains(&reduced), "{tiny:e}");
        }
        // Further than half a spacing from 360 it was already below 360.
        let below = normalize_degrees(-3e-14);
        assert!((359.999_999_999_999..360.0).contains(&below), "{below}");
        // Whole turns, either way, are 0.
        for turns in [-3.0, -1.0, 0.0, 1.0, 2.0, 1000.0] {
            assert_eq!(normalize_degrees(turns * 360.0), 0.0, "{turns}");
        }
        // Every result lies in the half-open interval.
        for degrees in [
            -720.5,
            -360.0,
            -1e-3,
            359.999_999_999_999_9,
            360.0,
            1e6 + 0.25,
        ] {
            let reduced = normalize_degrees(degrees);
            assert!((0.0..360.0).contains(&reduced), "{degrees}: {reduced}");
        }
        assert!(normalize_degrees(f64::NAN).is_nan());
        // The angle that has just passed a target is nearly a turn from it.
        assert!(normalize_degrees(90.0 - (90.0 + 1e-14)) > 359.999);
    }

    #[test]
    fn modulo_takes_the_sign_of_the_divisor() {
        assert!((modulo(-1.0, 360.0) - 359.0).abs() < 1e-12);
        assert!((modulo(361.0, 360.0) - 1.0).abs() < 1e-12);
        assert!((modulo(-0.25, 1.0) - 0.75).abs() < 1e-12);
        assert!((fract(-0.25) - 0.75).abs() < 1e-12);
        assert!((fract(2.5) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn the_adjusted_modulo_runs_from_one_to_n() {
        assert_eq!(amod(1, 12), 1);
        assert_eq!(amod(12, 12), 12);
        assert_eq!(amod(13, 12), 1);
        assert_eq!(amod(0, 12), 12);
        assert_eq!(amod(-1, 12), 11);
        assert_eq!(amod(60, 60), 60);
    }

    #[test]
    fn horner_evaluates_ascending_coefficients() {
        // 1 + 2x + 3x² at x = 2 is 1 + 4 + 12.
        assert!((poly(2.0, &[1.0, 2.0, 3.0]) - 17.0).abs() < 1e-12);
        assert!(poly(5.0, &[]).abs() < 1e-12);
        assert!((poly(5.0, &[7.0]) - 7.0).abs() < 1e-12);
    }

    /// The interval is `(−180, 180]` at both ends. Two copies of this
    /// function once disagreed here: one reduced `d + 180` into `[0, 360)`
    /// and subtracted 180, which puts +180 at −180 against its own
    /// documentation.
    #[test]
    fn signed_degrees_is_half_open_on_the_negative_side() {
        assert_eq!(signed_degrees(180.0), 180.0);
        assert_eq!(signed_degrees(-180.0), 180.0);
        assert_eq!(signed_degrees(540.0), 180.0);
        assert_eq!(signed_degrees(-540.0), 180.0);
        assert!(signed_degrees(180.000_001) < -179.0);
        assert!(signed_degrees(-179.999_999) > -180.0);
        assert!((signed_degrees(350.0) + 10.0).abs() < 1e-12);
        assert!((signed_degrees(190.0) + 170.0).abs() < 1e-12);
        // Inside [0, 180] the angle is returned bit for bit.
        for degrees in [0.0, 1e-300, 10.123_456_789, 179.999_999_999] {
            assert_eq!(signed_degrees(degrees).to_bits(), degrees.to_bits());
        }
    }

    #[test]
    fn degree_trigonometry_matches_radians() {
        assert!((sin_deg(90.0) - 1.0).abs() < 1e-12);
        assert!(cos_deg(90.0).abs() < 1e-12);
    }
}
