//! The sky, behind the `sky` feature: where the Sun and the Moon are at an
//! instant, the solar terms and the moon phases within a span, and the
//! Sun's decan, from `hc-astro`'s series, in Universal Time. The lines are
//! `hyper_calendar::sky_lines`', shared with the WebAssembly module.
//!
//! The Earth's rotation and the Sun's hours, behind the `sky` feature: the
//! Earth Rotation Angle, the Greenwich mean sidereal time by two
//! conventions, UT2 − UT1, and the clocks and named times of day of
//! `hc_astro::solar_time`, the named horizons with sunrise and sunset
//! against each, and the Heliocentric Julian Date in TT and in UTC, from
//! `hyper_calendar::astro_lines`, shared with the WebAssembly module, for
//! the years −1000 to 3000.
//!
//! The religious and traditional hours of a day, behind the `sky` feature:
//! the Islamic prayer times, the Jewish times in temporal hours and the
//! Edo 不定時法. The lines are `hyper_calendar::hours_lines`', shared with
//! the WebAssembly module, and answer for the years −1000 to 3000.

use hc::astro_lines;

use crate::marshal::status;
use crate::{HC_ERROR_NULL_POINTER, HC_OK, HcStatus};

/// One number through an out-parameter.
///
/// # Safety
///
/// `out` must be null or writable.
unsafe fn put(answer: hc::boundary::Answer<f64>, out: *mut f64) -> HcStatus {
    if out.is_null() {
        return HC_ERROR_NULL_POINTER;
    }
    match answer {
        Ok(value) => {
            // SAFETY: checked non-null above.
            unsafe { *out = value };
            HC_OK
        }
        Err(refusal) => status(refusal),
    }
}

/// The Earth Rotation Angle at a UT1 instant, in degrees, 0 to 360.
///
/// By IERS Conventions 2010, equation 5.14. `ut1_unix_seconds` counts
/// UT1 as POSIX time counts UTC, 86 400 seconds a day from 1970-01-01
/// 00:00 UT1, with a fraction. A value that is not finite, or outside
/// the years −1000 to 3000, is `HC_ERROR_OUT_OF_RANGE`.
///
/// # Safety
///
/// `out_degrees` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_earth_rotation_angle(
    ut1_unix_seconds: f64,
    out_degrees: *mut f64,
) -> HcStatus {
    let answer = astro_lines::earth_rotation_angle_degrees(ut1_unix_seconds);
    // SAFETY: forwarded to the caller's contract above.
    unsafe { put(answer, out_degrees) }
}

/// The Greenwich mean sidereal time by the IAU 2006 convention at a UT1
/// instant, in degrees, 0 to 360.
///
/// The Earth Rotation Angle plus the polynomial of IERS Conventions
/// 2010, equation 5.32, in TT taken as UT1 + ΔT. The instant is as for
/// `hc_earth_rotation_angle`, and fails as it does.
///
/// # Safety
///
/// `out_degrees` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_gmst_iau2006(ut1_unix_seconds: f64, out_degrees: *mut f64) -> HcStatus {
    // SAFETY: forwarded to the caller's contract above.
    unsafe {
        put(
            astro_lines::gmst_iau2006_degrees(ut1_unix_seconds),
            out_degrees,
        )
    }
}

/// The Greenwich mean sidereal time by the IAU 1982 convention at a UT1
/// instant, in degrees, 0 to 360.
///
/// Meeus's (12.4), a polynomial in UT1 alone; it differs from
/// `hc_gmst_iau2006` by about 0.14 ms of time in 2006. The instant is as
/// for `hc_earth_rotation_angle`, and fails as it does.
///
/// # Safety
///
/// `out_degrees` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_gmst_iau1982(ut1_unix_seconds: f64, out_degrees: *mut f64) -> HcStatus {
    // SAFETY: forwarded to the caller's contract above.
    unsafe {
        put(
            astro_lines::gmst_iau1982_degrees(ut1_unix_seconds),
            out_degrees,
        )
    }
}

/// UT2 − UT1 at a UT1 instant, in seconds.
///
/// The conventional seasonal variation as the USNO states the formula;
/// its extremes are ±0.031 s. The instant is as for
/// `hc_earth_rotation_angle`, and fails as it does.
///
/// # Safety
///
/// `out_seconds` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_ut2_minus_ut1(
    ut1_unix_seconds: f64,
    out_seconds: *mut f64,
) -> HcStatus {
    // SAFETY: forwarded to the caller's contract above.
    unsafe {
        put(
            astro_lines::ut2_minus_ut1_seconds(ut1_unix_seconds),
            out_seconds,
        )
    }
}

hc::exports!("sky", c_exports);
