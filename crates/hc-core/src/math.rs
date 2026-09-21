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
//! guard below. It used to be a `panic!` in each function body, which meant a
//! build with neither feature compiled cleanly and then panicked the first
//! time anything asked for a sine — so `cargo build` passed, CI passed, and
//! `cargo test` failed at run time in a configuration nobody had exercised.
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

/// Reduce an angle in degrees to `[0, 360)`.
#[inline]
#[must_use]
pub fn normalize_degrees(degrees: f64) -> f64 {
    let reduced = degrees - 360.0 * floor(degrees / 360.0);
    if reduced < 0.0 {
        reduced + 360.0
    } else {
        reduced
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

    #[test]
    fn degree_trigonometry_matches_radians() {
        assert!((sin_deg(90.0) - 1.0).abs() < 1e-12);
        assert!(cos_deg(90.0).abs() < 1e-12);
    }
}
