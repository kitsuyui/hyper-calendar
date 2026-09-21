//! Floating-point math that works with or without `std`.
//!
//! `core` deliberately omits transcendental functions, so a `no_std` build has
//! to route them somewhere. Enabling `std` (the default) uses the platform
//! implementations; enabling `libm` uses the portable software ones. The rest
//! of the workspace calls these wrappers and never `f64::sin` directly, so a
//! `no_std` port never has to be re-audited function by function.

#![allow(missing_docs)]

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

unary!(
    sin, cos, tan, asin, acos, atan, sqrt, exp, floor, ceil, trunc, cbrt
);

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

#[inline]
#[must_use]
pub fn round(x: f64) -> f64 {
    #[cfg(feature = "std")]
    {
        f64::round(x)
    }
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    {
        libm::round(x)
    }
    #[cfg(all(not(feature = "std"), not(feature = "libm")))]
    {
        let _ = x;
        panic!("hc-core::math::round requires the `std` or `libm` feature")
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
