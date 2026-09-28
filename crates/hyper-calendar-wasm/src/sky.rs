//! The sky, behind the `sky` feature: where the Sun and the Moon are at an
//! instant, the solar terms and the moon phases within a span, and the
//! Sun's decan, from `hc-astro`'s series, in Universal Time. The lines are
//! `hyper_calendar::sky_lines`', shared with the C library.
//!
//! The Earth's rotation and the Sun's hours, behind the `sky` feature: the
//! Earth Rotation Angle, the Greenwich mean sidereal time by two
//! conventions, UT2 − UT1, the clocks and named times of day of
//! `hc_astro::solar_time`, the named horizons with sunrise and sunset
//! against each, and the Heliocentric Julian Date in TT and in UTC. The
//! lines are `hyper_calendar::astro_lines`', shared with the C library,
//! and answer for the years −1000 to 3000.
//!
//! The religious and traditional hours of a day, behind the `sky` feature:
//! the Islamic prayer times by a named method, the Jewish times in
//! temporal hours with the dawns and nightfalls, and the Edo 不定時法. The
//! lines are `hyper_calendar::hours_lines`', shared with the C library,
//! and answer for the years −1000 to 3000.

use hc::astro_lines;

use crate::marshal::emit_answer;

/// The Earth Rotation Angle at a UT1 instant, as one UTF-8 line,
/// returning the byte length written.
///
/// The one cell is the angle in degrees, 0 to 360, by IERS Conventions
/// 2010, equation 5.14. `ut1_unix_seconds` counts UT1 as POSIX time
/// counts UTC, 86 400 seconds a day from 1970-01-01 00:00 UT1, with a
/// fraction. A value that is not finite, or outside the years −1000 to
/// 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length
/// the text needs.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes unless it is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_earth_rotation_angle(
    ut1_unix_seconds: f64,
    buffer: *mut u8,
    capacity: usize,
) -> i64 {
    let answer = astro_lines::earth_rotation_angle_line(ut1_unix_seconds);
    // SAFETY: forwarded to the caller's contract above.
    unsafe { emit_answer(answer, buffer, capacity) }
}

/// The Greenwich mean sidereal time by the IAU 2006 convention at a UT1
/// instant, as one UTF-8 line, returning the byte length written.
///
/// The one cell is the angle in degrees, 0 to 360: the Earth Rotation
/// Angle plus the polynomial of IERS Conventions 2010, equation 5.32,
/// in TT taken as UT1 + ΔT. The instant is as for
/// `hc_earth_rotation_angle`, and fails as it does. A null `buffer`
/// returns the length the text needs.
///
/// # Safety
///
/// As `hc_earth_rotation_angle`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_gmst_iau2006(
    ut1_unix_seconds: f64,
    buffer: *mut u8,
    capacity: usize,
) -> i64 {
    // SAFETY: forwarded to the caller's contract above.
    unsafe {
        emit_answer(
            astro_lines::gmst_iau2006_line(ut1_unix_seconds),
            buffer,
            capacity,
        )
    }
}

/// The Greenwich mean sidereal time by the IAU 1982 convention at a UT1
/// instant, as one UTF-8 line, returning the byte length written.
///
/// The one cell is the angle in degrees, 0 to 360, by Meeus's (12.4),
/// a polynomial in UT1 alone; it differs from `hc_gmst_iau2006` by
/// about 0.14 ms of time in 2006. The instant is as for
/// `hc_earth_rotation_angle`, and fails as it does. A null `buffer`
/// returns the length the text needs.
///
/// # Safety
///
/// As `hc_earth_rotation_angle`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_gmst_iau1982(
    ut1_unix_seconds: f64,
    buffer: *mut u8,
    capacity: usize,
) -> i64 {
    // SAFETY: forwarded to the caller's contract above.
    unsafe {
        emit_answer(
            astro_lines::gmst_iau1982_line(ut1_unix_seconds),
            buffer,
            capacity,
        )
    }
}

/// UT2 − UT1 at a UT1 instant, as one UTF-8 line, returning the byte
/// length written.
///
/// The one cell is the conventional seasonal variation in seconds,
/// 0.022 sin 2πT − 0.012 cos 2πT − 0.006 sin 4πT + 0.007 cos 4πT with T
/// the Besselian year, as the USNO states the formula; its extremes are
/// ±0.031 s. The instant is as for `hc_earth_rotation_angle`, and fails
/// as it does. A null `buffer` returns the length the text needs.
///
/// # Safety
///
/// As `hc_earth_rotation_angle`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_ut2_minus_ut1(
    ut1_unix_seconds: f64,
    buffer: *mut u8,
    capacity: usize,
) -> i64 {
    // SAFETY: forwarded to the caller's contract above.
    unsafe {
        emit_answer(
            astro_lines::ut2_minus_ut1_line(ut1_unix_seconds),
            buffer,
            capacity,
        )
    }
}

hc::exports!("sky", w_exports);
