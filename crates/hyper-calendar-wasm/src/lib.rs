//! A WebAssembly surface for `hyper-calendar`.
//!
//! # Why this is a raw ABI and not `wasm-bindgen`
//!
//! The workspace has no external dependencies ([ADR
//! 0005](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/adr/0005-no-external-dependencies.md)),
//! and that is worth more here than anywhere else. A calendar library is a
//! leaf dependency of a web application; whatever it drags in, the bundle
//! carries. So the exports below are plain `extern "C"` functions over
//! integers and linear memory, which every WebAssembly host can call with no
//! glue at all.
//!
//! The cost is that the JavaScript side does the string marshalling. That is
//! about thirty lines, shown below, and it is thirty lines the caller can
//! read.
//!
//! # Memory
//!
//! The module owns its allocator. [`hc_alloc`] hands out a block, the caller
//! writes into it or reads out of it, and [`hc_free`] takes it back. Every
//! block must be freed with the same length it was allocated with, which is
//! what `Vec::from_raw_parts` requires.
//!
//! Text is UTF-8 and is *not* NUL-terminated: functions that produce it
//! return the byte length, because a length is cheaper and safer than a scan.
//!
//! ```js
//! const { instance } = await WebAssembly.instantiateStreaming(
//!   fetch("hyper_calendar_wasm.wasm"),
//! );
//! const wasm = instance.exports;
//!
//! // 2026-09-21 as a fixed day number.
//! const rd = wasm.hc_gregorian_to_fixed(2026, 9, 21);
//!
//! // Read it back as an ISO 8601 date.
//! const cap = 32;
//! const ptr = wasm.hc_alloc(cap);
//! const len = wasm.hc_format_iso_date(rd, ptr, cap);
//! const bytes = new Uint8Array(wasm.memory.buffer, ptr, len);
//! const text = new TextDecoder().decode(bytes);
//! wasm.hc_free(ptr, cap);
//! ```
//!
//! # Errors
//!
//! Functions returning a day number or a count return a negative sentinel on
//! failure rather than trapping, because a trap tears down the instance and
//! takes any other work in it with it. The sentinels are the `HC_ERR_*`
//! constants, all below [`HC_ERR_FLOOR`], and a day number can never be that
//! negative: [`HC_ERR_FLOOR`] is more than a thousand times the age of the
//! universe in days.

#![allow(unsafe_code)]
#![warn(missing_docs)]

use hc::civil::Date;
use hc::hc_calendar::{Rd, Weekday};
use hc::hc_core::unix::{self, LeapPolicy};

/// Any return value at or below this is an error sentinel, not a result.
///
/// The age of the universe is about 5 × 10¹² days, so no legitimate fixed day
/// number comes anywhere near this.
pub const HC_ERR_FLOOR: i64 = -9_000_000_000_000_000;

/// The date does not exist.
pub const HC_ERR_INVALID_DATE: i64 = -9_000_000_000_000_001;
/// A value was outside the supported range.
pub const HC_ERR_OUT_OF_RANGE: i64 = -9_000_000_000_000_002;
/// The supplied buffer was too small.
pub const HC_ERR_BUFFER_TOO_SMALL: i64 = -9_000_000_000_000_003;
/// The requested model has no data for this value.
pub const HC_ERR_NO_DATA: i64 = -9_000_000_000_000_004;

/// Allocate `len` bytes of linear memory and return a pointer to them.
///
/// Returns null when `len` is zero or the allocation fails. Free it with
/// [`hc_free`], passing the same `len`.
#[unsafe(no_mangle)]
pub extern "C" fn hc_alloc(len: usize) -> *mut u8 {
    if len == 0 {
        return core::ptr::null_mut();
    }
    let mut buffer = Vec::<u8>::new();
    if buffer.try_reserve_exact(len).is_err() {
        return core::ptr::null_mut();
    }
    buffer.resize(len, 0);
    let mut boxed = buffer.into_boxed_slice();
    let pointer = boxed.as_mut_ptr();
    core::mem::forget(boxed);
    pointer
}

/// Return a block from [`hc_alloc`] to the allocator.
///
/// # Safety
///
/// `pointer` must have come from [`hc_alloc`] with the same `len`, and must
/// not have been freed already.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_free(pointer: *mut u8, len: usize) {
    if pointer.is_null() || len == 0 {
        return;
    }
    // SAFETY: the caller guarantees the pointer and length came from a
    // matching `hc_alloc`, which built the block as a boxed slice.
    unsafe {
        let slice = core::ptr::slice_from_raw_parts_mut(pointer, len);
        drop(Box::from_raw(slice));
    }
}

/// Copy `text` into the caller's buffer, returning the byte length written.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes.
unsafe fn emit(text: &str, buffer: *mut u8, capacity: usize) -> i64 {
    if buffer.is_null() || capacity < text.len() {
        return HC_ERR_BUFFER_TOO_SMALL;
    }
    // SAFETY: `capacity >= text.len()`, so the copy stays in the buffer.
    unsafe { core::ptr::copy_nonoverlapping(text.as_ptr(), buffer, text.len()) };
    text.len() as i64
}

/// The library version as UTF-8, returning the byte length written.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_version(buffer: *mut u8, capacity: usize) -> i64 {
    // SAFETY: forwarded to the caller's contract above.
    unsafe { emit(hc::VERSION, buffer, capacity) }
}

/// The fixed day number of a proleptic Gregorian date, or an error sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn hc_gregorian_to_fixed(year: i64, month: u32, day: u32) -> i64 {
    let (Ok(month), Ok(day)) = (u8::try_from(month), u8::try_from(day)) else {
        return HC_ERR_INVALID_DATE;
    };
    match Date::new(year, month, day) {
        Ok(date) => date.to_ordinal(),
        Err(_) => HC_ERR_INVALID_DATE,
    }
}

/// The Gregorian year on a fixed day, or an error sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn hc_gregorian_year(fixed: i64) -> i64 {
    match Date::from_ordinal(fixed) {
        Ok(date) => date.year(),
        Err(_) => HC_ERR_OUT_OF_RANGE,
    }
}

/// The Gregorian month on a fixed day, 1 through 12, or an error sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn hc_gregorian_month(fixed: i64) -> i64 {
    match Date::from_ordinal(fixed) {
        Ok(date) => i64::from(date.month()),
        Err(_) => HC_ERR_OUT_OF_RANGE,
    }
}

/// The Gregorian day of the month on a fixed day, or an error sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn hc_gregorian_day(fixed: i64) -> i64 {
    match Date::from_ordinal(fixed) {
        Ok(date) => i64::from(date.day()),
        Err(_) => HC_ERR_OUT_OF_RANGE,
    }
}

/// The ISO weekday of a fixed day, Monday = 1 through Sunday = 7.
#[unsafe(no_mangle)]
pub extern "C" fn hc_weekday(fixed: i64) -> i64 {
    i64::from(Weekday::from_rd(Rd(fixed)).iso_number())
}

/// The 1-based day of the year on a fixed day, or an error sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn hc_day_of_year(fixed: i64) -> i64 {
    match Date::from_ordinal(fixed) {
        Ok(date) => i64::from(date.day_of_year()),
        Err(_) => HC_ERR_OUT_OF_RANGE,
    }
}

/// Whether the Gregorian year on a fixed day is a leap year: 1, 0, or an
/// error sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn hc_is_leap_year(fixed: i64) -> i64 {
    match Date::from_ordinal(fixed) {
        Ok(date) => i64::from(date.is_leap_year()),
        Err(_) => HC_ERR_OUT_OF_RANGE,
    }
}

/// The fixed day a POSIX timestamp falls on, in UTC.
#[unsafe(no_mangle)]
pub extern "C" fn hc_fixed_from_unix(unix_seconds: i64) -> i64 {
    Rd::from_unix_days(unix_seconds.div_euclid(86_400)).get()
}

/// `TAI - UTC` in whole seconds at a POSIX timestamp.
///
/// `strict` non-zero refuses to answer past the announced leap-second table,
/// returning [`HC_ERR_NO_DATA`]; zero holds the last published value. The
/// difference is a forecast, so it is the caller's choice to make.
#[unsafe(no_mangle)]
pub extern "C" fn hc_tai_minus_utc(unix_seconds: i64, strict: i32) -> i64 {
    let policy = if strict != 0 {
        LeapPolicy::Strict
    } else {
        LeapPolicy::Extrapolate
    };
    match unix::tai_minus_utc_at(unix_seconds, policy) {
        Ok(offset) => offset.whole_seconds() as i64,
        Err(_) => HC_ERR_NO_DATA,
    }
}

/// Whether the UTC day containing a POSIX timestamp ends with an inserted
/// leap second: 1, 0, or an error sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn hc_day_has_leap_second(unix_seconds: i64) -> i64 {
    let day_start = unix_seconds.div_euclid(86_400) * 86_400;
    let policy = LeapPolicy::Extrapolate;
    let (Ok(before), Ok(after)) = (
        unix::tai_minus_utc_at(day_start, policy),
        unix::tai_minus_utc_at(day_start + 86_400, policy),
    ) else {
        return HC_ERR_NO_DATA;
    };
    i64::from(after > before)
}

/// The POSIX timestamp of midnight UTC on a fixed day.
#[unsafe(no_mangle)]
pub extern "C" fn hc_unix_from_fixed(fixed: i64) -> i64 {
    Rd(fixed).to_unix_days() * 86_400
}

/// Render a fixed day as an ISO 8601 date, returning the byte length written.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_format_iso_date(fixed: i64, buffer: *mut u8, capacity: usize) -> i64 {
    let Ok(date) = Date::from_ordinal(fixed) else {
        return HC_ERR_OUT_OF_RANGE;
    };
    let text = date.to_string();
    // SAFETY: forwarded to the caller's contract above.
    unsafe { emit(&text, buffer, capacity) }
}

/// Parse an ISO 8601 date from UTF-8, returning its fixed day number.
///
/// # Safety
///
/// `buffer` must be readable for `len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_parse_iso_date(buffer: *const u8, len: usize) -> i64 {
    if buffer.is_null() {
        return HC_ERR_INVALID_DATE;
    }
    // SAFETY: the caller guarantees `buffer` is readable for `len` bytes.
    let bytes = unsafe { core::slice::from_raw_parts(buffer, len) };
    let Ok(text) = core::str::from_utf8(bytes) else {
        return HC_ERR_INVALID_DATE;
    };
    match hc::hc_format::iso8601::parse_date(text) {
        Ok(parsed) => parsed.to_fixed().map_or(HC_ERR_INVALID_DATE, Rd::get),
        Err(_) => HC_ERR_INVALID_DATE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocation_round_trips() {
        let pointer = hc_alloc(32);
        assert!(!pointer.is_null());
        unsafe { hc_free(pointer, 32) };
        assert!(hc_alloc(0).is_null());
        // Freeing null or zero length is a no-op rather than a fault.
        unsafe { hc_free(core::ptr::null_mut(), 0) };
    }

    #[test]
    fn gregorian_conversion_matches_the_fixed_day() {
        assert_eq!(hc_gregorian_to_fixed(2026, 9, 21), 739_880);
        assert_eq!(hc_gregorian_year(739_880), 2026);
        assert_eq!(hc_gregorian_month(739_880), 9);
        assert_eq!(hc_gregorian_day(739_880), 21);
        assert_eq!(hc_weekday(739_880), 1);
    }

    #[test]
    fn errors_are_sentinels_rather_than_traps() {
        assert!(hc_gregorian_to_fixed(2026, 2, 30) <= HC_ERR_FLOOR);
        assert!(hc_gregorian_to_fixed(2026, 13, 1) <= HC_ERR_FLOOR);
        assert!(hc_gregorian_to_fixed(2026, 1, 300) <= HC_ERR_FLOOR);
        // A legitimate answer is never mistaken for a sentinel.
        assert!(hc_gregorian_to_fixed(-9_000, 1, 1) > HC_ERR_FLOOR);
    }

    #[test]
    fn the_error_floor_is_far_below_any_real_day_number() {
        // The universe is about 5e12 days old; the floor is 9e15, so a fixed
        // day number can never be mistaken for a sentinel.
        let age_of_universe_in_days = -5_000_000_000_000i64;
        assert!(HC_ERR_FLOOR < age_of_universe_in_days);
        for sentinel in [
            HC_ERR_INVALID_DATE,
            HC_ERR_OUT_OF_RANGE,
            HC_ERR_BUFFER_TOO_SMALL,
            HC_ERR_NO_DATA,
        ] {
            assert!(sentinel <= HC_ERR_FLOOR);
        }
    }

    #[test]
    fn text_is_written_without_a_terminator_and_reports_its_length() {
        let mut buffer = [0u8; 32];
        let written = unsafe { hc_format_iso_date(739_880, buffer.as_mut_ptr(), buffer.len()) };
        assert_eq!(written, 10);
        assert_eq!(&buffer[..10], b"2026-09-21");
        // The byte after the text is untouched, not a NUL the caller must skip.
        assert_eq!(buffer[10], 0);
    }

    #[test]
    fn a_short_buffer_is_reported_rather_than_truncated() {
        let mut buffer = [0u8; 4];
        let written = unsafe { hc_format_iso_date(739_880, buffer.as_mut_ptr(), buffer.len()) };
        assert_eq!(written, HC_ERR_BUFFER_TOO_SMALL);
        assert_eq!(buffer, [0u8; 4]);
    }

    #[test]
    fn iso_dates_round_trip_through_the_boundary() {
        let text = "2026-09-21";
        let parsed = unsafe { hc_parse_iso_date(text.as_ptr(), text.len()) };
        assert_eq!(parsed, 739_880);

        let mut buffer = [0u8; 32];
        let written = unsafe { hc_format_iso_date(parsed, buffer.as_mut_ptr(), buffer.len()) };
        assert_eq!(&buffer[..written as usize], text.as_bytes());
    }

    #[test]
    fn malformed_input_is_rejected_without_trapping() {
        for text in ["", "not a date", "2026-13-01", "2026-02-30"] {
            let result = unsafe { hc_parse_iso_date(text.as_ptr(), text.len()) };
            assert!(result <= HC_ERR_FLOOR, "{text} gave {result}");
        }
        assert!(unsafe { hc_parse_iso_date(core::ptr::null(), 0) } <= HC_ERR_FLOOR);
    }

    #[test]
    fn unix_time_maps_onto_fixed_days() {
        assert_eq!(hc_fixed_from_unix(0), 719_163);
        assert_eq!(hc_unix_from_fixed(719_163), 0);
        assert_eq!(hc_fixed_from_unix(-1), 719_162);
    }

    #[test]
    fn the_leap_second_table_is_reachable() {
        assert_eq!(hc_tai_minus_utc(1_700_000_000, 1), 37);
        assert_eq!(hc_day_has_leap_second(1_483_142_400), 1);
        assert_eq!(hc_day_has_leap_second(1_483_228_800), 0);
        // Past the announced table the strict policy refuses.
        assert!(hc_tai_minus_utc(4_000_000_000, 1) <= HC_ERR_FLOOR);
        assert_eq!(hc_tai_minus_utc(4_000_000_000, 0), 37);
    }

    #[test]
    fn day_of_year_and_leap_years_are_exposed() {
        let leap_day = hc_gregorian_to_fixed(2024, 12, 31);
        assert_eq!(hc_day_of_year(leap_day), 366);
        assert_eq!(hc_is_leap_year(leap_day), 1);
        let common = hc_gregorian_to_fixed(2023, 12, 31);
        assert_eq!(hc_day_of_year(common), 365);
        assert_eq!(hc_is_leap_year(common), 0);
    }
}
