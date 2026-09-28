//! The civil calendar, behind the `civil` feature: proleptic Gregorian
//! dates, ISO 8601 text, POSIX time, TAI − UTC and leap seconds.

use hc::civil::Date;
use hc::hc_calendar::Rd;
use hc::hc_core::unix::{self, LeapPolicy};

use crate::HC_ERR_INVALID_DATE;
use crate::marshal::{emit, sentinel, text};

/// The fixed day number of a proleptic Gregorian date, or an error sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn hc_gregorian_to_fixed(year: i64, month: u32, day: u32) -> i64 {
    let (Ok(month), Ok(day)) = (u8::try_from(month), u8::try_from(day)) else {
        return HC_ERR_INVALID_DATE;
    };
    match Date::new(year, month, day) {
        Ok(date) => date.to_ordinal(),
        Err(error) => sentinel(error.into()),
    }
}

/// The Gregorian year on a fixed day, or an error sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn hc_gregorian_year(fixed: i64) -> i64 {
    match Date::from_ordinal(fixed) {
        Ok(date) => date.year(),
        Err(error) => sentinel(error.into()),
    }
}

/// The Gregorian month on a fixed day, 1 through 12, or an error sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn hc_gregorian_month(fixed: i64) -> i64 {
    match Date::from_ordinal(fixed) {
        Ok(date) => i64::from(date.month()),
        Err(error) => sentinel(error.into()),
    }
}

/// The Gregorian day of the month on a fixed day, or an error sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn hc_gregorian_day(fixed: i64) -> i64 {
    match Date::from_ordinal(fixed) {
        Ok(date) => i64::from(date.day()),
        Err(error) => sentinel(error.into()),
    }
}

/// `TAI - UTC` in whole seconds at a POSIX timestamp.
///
/// `strict` non-zero refuses to answer before 1961 and past the announced
/// leap-second table, returning [`HC_ERR_NO_DATA`](super::HC_ERR_NO_DATA); zero holds the last
/// published value. The difference is a forecast, so it is the caller's
/// choice to make.
#[unsafe(no_mangle)]
pub extern "C" fn hc_tai_minus_utc(unix_seconds: i64, strict: i32) -> i64 {
    let policy = if strict != 0 {
        LeapPolicy::Strict
    } else {
        LeapPolicy::Extrapolate
    };
    match unix::tai_minus_utc_at(unix_seconds, policy) {
        Ok(offset) => offset.whole_seconds() as i64,
        Err(error) => sentinel(error.into()),
    }
}

/// Render a fixed day as an ISO 8601 date, returning the byte length written.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_format_iso_date(fixed: i64, buffer: *mut u8, capacity: usize) -> i64 {
    let date = match Date::from_ordinal(fixed) {
        Ok(date) => date,
        Err(error) => return sentinel(error.into()),
    };
    let text = date.to_string();
    // SAFETY: forwarded to the caller's contract above.
    unsafe { emit(&text, buffer, capacity) }
}

/// Parse an ISO 8601 date from UTF-8, returning its fixed day number.
///
/// Text that is not a date is `HC_ERR_INVALID_DATE`; a null `buffer` with a
/// non-zero `len` is `HC_ERR_NULL_POINTER`, and bytes that are not UTF-8
/// are `HC_ERR_NOT_UTF8`.
///
/// # Safety
///
/// `buffer` must be readable for `len` bytes unless it is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_parse_iso_date(buffer: *const u8, len: usize) -> i64 {
    // SAFETY: forwarded to the caller's contract above.
    let text = match unsafe { text(buffer, len) } {
        Ok(text) => text,
        Err(sentinel) => return sentinel,
    };
    match hc::hc_format::iso8601::parse_date(text) {
        Ok(parsed) => parsed.to_fixed().map_or(HC_ERR_INVALID_DATE, Rd::get),
        Err(_) => HC_ERR_INVALID_DATE,
    }
}

hc::exports!("civil", w_exports);
