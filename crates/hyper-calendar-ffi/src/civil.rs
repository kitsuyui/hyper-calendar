//! The civil calendar, behind the `civil` feature: proleptic Gregorian
//! dates, ISO 8601 text and the TAI–UTC bridge.

use core::ffi::{c_char, c_int};

use hc::civil::Date;
use hc::hc_core::unix::{self, LeapPolicy};

use crate::marshal::{status, write_text};
use crate::{
    HC_ERROR_BUFFER_TOO_SMALL, HC_ERROR_INVALID_DATE, HC_ERROR_NULL_POINTER, HC_ERROR_OVERFLOW,
    HC_OK, HcStatus,
};

/// Format a date into a fixed scratch buffer without allocating.
fn format_into(scratch: &mut [u8; 32], date: Date) -> Option<&str> {
    use core::fmt::Write;

    struct Sink<'a> {
        buffer: &'a mut [u8; 32],
        length: usize,
    }

    impl Write for Sink<'_> {
        fn write_str(&mut self, text: &str) -> core::fmt::Result {
            let end = self.length + text.len();
            if end > self.buffer.len() {
                return Err(core::fmt::Error);
            }
            self.buffer[self.length..end].copy_from_slice(text.as_bytes());
            self.length = end;
            Ok(())
        }
    }

    let mut sink = Sink {
        buffer: scratch,
        length: 0,
    };
    write!(sink, "{date}").ok()?;
    let length = sink.length;
    core::str::from_utf8(&scratch[..length]).ok()
}

/// The fixed day number of a proleptic Gregorian date.
///
/// The fixed day is the Rata Die: `0001-01-01` is day 1. It is also what
/// Python's `date.toordinal()` returns.
///
/// # Safety
///
/// `out_fixed` must be null or writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_gregorian_to_fixed(
    year: i64,
    month: u8,
    day: u8,
    out_fixed: *mut i64,
) -> HcStatus {
    if out_fixed.is_null() {
        return HC_ERROR_NULL_POINTER;
    }
    match Date::new(year, month, day) {
        Ok(date) => {
            // SAFETY: checked non-null immediately above.
            unsafe { *out_fixed = date.to_ordinal() };
            HC_OK
        }
        Err(error) => status(error.into()),
    }
}

/// The proleptic Gregorian date on a fixed day.
///
/// # Safety
///
/// Every non-null out-parameter must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_gregorian_from_fixed(
    fixed: i64,
    out_year: *mut i64,
    out_month: *mut u8,
    out_day: *mut u8,
) -> HcStatus {
    if out_year.is_null() || out_month.is_null() || out_day.is_null() {
        return HC_ERROR_NULL_POINTER;
    }
    match Date::from_ordinal(fixed) {
        Ok(date) => {
            // SAFETY: all three were checked non-null immediately above.
            unsafe {
                *out_year = date.year();
                *out_month = date.month();
                *out_day = date.day();
            }
            HC_OK
        }
        Err(error) => status(error.into()),
    }
}

/// Parse an ISO 8601 date from a NUL-terminated UTF-8 string into its
/// fixed day number.
///
/// Text that is not a date is [`HC_ERROR_INVALID_DATE`], a null `text`
/// [`HC_ERROR_NULL_POINTER`] and bytes that are not UTF-8
/// [`HC_ERROR_NOT_UTF8`](super::HC_ERROR_NOT_UTF8).
///
/// # Safety
///
/// `text` must be null or point to a NUL-terminated string, and
/// `out_fixed` must be null or writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_parse_iso_date(text: *const c_char, out_fixed: *mut i64) -> HcStatus {
    if out_fixed.is_null() {
        return HC_ERROR_NULL_POINTER;
    }
    // SAFETY: forwarded to the caller's contract above.
    let text = match unsafe { crate::marshal::text(text) } {
        Ok(Some(text)) => text,
        Ok(None) => return HC_ERROR_NULL_POINTER,
        Err(status) => return status,
    };
    let Some(fixed) = hc::hc_format::iso8601::parse_date(text)
        .ok()
        .and_then(|parsed| parsed.to_fixed().ok())
    else {
        return HC_ERROR_INVALID_DATE;
    };
    // SAFETY: checked non-null at the top.
    unsafe { *out_fixed = fixed.get() };
    HC_OK
}

/// Render a fixed day as an ISO 8601 date into a caller-owned buffer.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes and `written` must be null
/// or writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_format_iso_date(
    fixed: i64,
    buffer: *mut c_char,
    capacity: usize,
    written: *mut usize,
) -> HcStatus {
    let date = match Date::from_ordinal(fixed) {
        Ok(value) => value,
        Err(error) => return status(error.into()),
    };
    // 17 bytes covers a signed expanded year, the separators and the
    // terminator; `write_text` reports the exact requirement regardless.
    let mut scratch = [0u8; 32];
    let text = match format_into(&mut scratch, date) {
        Some(value) => value,
        None => return HC_ERROR_BUFFER_TOO_SMALL,
    };
    // SAFETY: forwarded to the caller's contract above.
    unsafe { write_text(text, buffer, capacity, written) }
}

/// Convert a POSIX timestamp to a TAI reading in seconds and attoseconds.
///
/// `strict` selects the leap-second policy: non-zero refuses to answer
/// before 1961 and past the announced IERS table, zero extrapolates. The
/// difference matters, which is why it is a parameter rather than a
/// default.
///
/// # Safety
///
/// Both out-parameters must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_tai_from_unix(
    unix_seconds: i64,
    strict: c_int,
    out_tai_seconds: *mut i64,
    out_attoseconds: *mut u64,
) -> HcStatus {
    if out_tai_seconds.is_null() || out_attoseconds.is_null() {
        return HC_ERROR_NULL_POINTER;
    }
    match hc::time_lines::tai_from_unix(unix_seconds, strict != 0) {
        Ok((seconds, attoseconds)) => {
            // SAFETY: both were checked non-null immediately above.
            unsafe {
                *out_tai_seconds = seconds;
                *out_attoseconds = attoseconds;
            }
            HC_OK
        }
        Err(refusal) => status(refusal),
    }
}

/// `TAI - UTC` in whole seconds at a POSIX timestamp.
///
/// Returns [`HC_ERROR_NO_DATA`](super::HC_ERROR_NO_DATA) under the strict policy outside the published
/// leap-second table, which is the honest answer rather than a forecast.
///
/// # Safety
///
/// `out_offset` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_tai_minus_utc(
    unix_seconds: i64,
    strict: c_int,
    out_offset: *mut i64,
) -> HcStatus {
    if out_offset.is_null() {
        return HC_ERROR_NULL_POINTER;
    }
    let policy = if strict != 0 {
        LeapPolicy::Strict
    } else {
        LeapPolicy::Extrapolate
    };
    match unix::tai_minus_utc_at(unix_seconds, policy) {
        Ok(offset) => {
            let seconds = match i64::try_from(offset.whole_seconds()) {
                Ok(value) => value,
                Err(_) => return HC_ERROR_OVERFLOW,
            };
            // SAFETY: checked non-null immediately above.
            unsafe { *out_offset = seconds };
            HC_OK
        }
        Err(error) => status(error.into()),
    }
}

/// Convert a TAI reading back to a UTC label, naming a leap second when the
/// instant falls inside one.
///
/// `out_is_leap_second` receives 1 when the instant is an inserted
/// `23:59:60`, which POSIX time cannot express.
///
/// # Safety
///
/// Both out-parameters must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_utc_from_tai(
    tai_seconds: i64,
    strict: c_int,
    out_unix_seconds: *mut i64,
    out_is_leap_second: *mut c_int,
) -> HcStatus {
    if out_unix_seconds.is_null() || out_is_leap_second.is_null() {
        return HC_ERROR_NULL_POINTER;
    }
    match hc::time_lines::utc_from_tai(tai_seconds, strict != 0) {
        Ok(utc) => {
            // SAFETY: both were checked non-null immediately above.
            unsafe {
                *out_unix_seconds = utc.unix_seconds;
                *out_is_leap_second = c_int::from(utc.leap_second);
            }
            HC_OK
        }
        Err(refusal) => status(refusal),
    }
}

hc::exports!("civil", c_exports);
