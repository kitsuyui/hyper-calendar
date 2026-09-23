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
//! a few lines, shown below, and they are lines the caller can read.
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
//! // 2026-09-21 as a fixed day number. i64 arrives as a BigInt.
//! const rd = wasm.hc_gregorian_to_fixed(2026n, 9, 21);
//!
//! // Read it back as an ISO 8601 date.
//! const cap = 32;
//! const ptr = wasm.hc_alloc(cap);
//! const len = Number(wasm.hc_format_iso_date(rd, ptr, cap));
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
//! constants, all at or below [`HC_ERR_FLOOR`], and a day number can never
//! be that negative: [`HC_ERR_FLOOR`] is more than a thousand times the age
//! of the universe in days.

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
/// A pointer was null with a non-zero length.
pub const HC_ERR_NULL_POINTER: i64 = -9_000_000_000_000_005;
/// The requested table or identifier is not known.
pub const HC_ERR_UNKNOWN: i64 = -9_000_000_000_000_006;
/// Text was not valid UTF-8.
pub const HC_ERR_NOT_UTF8: i64 = -9_000_000_000_000_007;

/// UTF-8 text from linear memory. A null pointer with a zero length is the
/// empty string.
///
/// # Errors
///
/// [`HC_ERR_NULL_POINTER`] for a null pointer with a non-zero length and
/// [`HC_ERR_NOT_UTF8`] for bytes that are not UTF-8.
///
/// # Safety
///
/// `pointer` must be readable for `len` bytes unless it is null.
unsafe fn text<'a>(pointer: *const u8, len: usize) -> Result<&'a str, i64> {
    if pointer.is_null() {
        return if len == 0 {
            Ok("")
        } else {
            Err(HC_ERR_NULL_POINTER)
        };
    }
    // SAFETY: the caller guarantees `pointer` is readable for `len` bytes.
    let bytes = unsafe { core::slice::from_raw_parts(pointer, len) };
    core::str::from_utf8(bytes).map_err(|_| HC_ERR_NOT_UTF8)
}

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
/// `strict` non-zero refuses to answer before 1961 and past the announced
/// leap-second table, returning [`HC_ERR_NO_DATA`]; zero holds the last
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

/// The holiday tables, behind the `holiday` feature: every country,
/// exchange, tradition and international set of `hc-holiday`, looked up by
/// identifier and rendered as tab-separated lines.
#[cfg(feature = "holiday")]
mod holiday {
    use super::{HC_ERR_BUFFER_TOO_SMALL, HC_ERR_OUT_OF_RANGE, HC_ERR_UNKNOWN, emit, text};
    use hc::hc_calendar::Rd;
    use hc::hc_holiday::engine::HolidayCalendar;
    use hc::hc_holiday::hc_calendars_solar::gregorian;
    use hc::hc_holiday::rule::{Confidence, Kind, RuleSet};
    use hc::hc_holiday::{countries, exchanges, international, traditions};

    /// The table an identifier names: a country's ISO 3166-1 alpha-2 code,
    /// an exchange's ISO 10383 Market Identifier Code, a tradition's slug or
    /// `un-days`, tried in that order.
    fn table(code: &str) -> Option<&'static RuleSet> {
        countries::by_code(code)
            .or_else(|| exchanges::by_code(code))
            .or_else(|| traditions::by_code(code))
            .or_else(|| international::by_code(code))
    }

    /// The table and region two strings in linear memory name.
    ///
    /// # Errors
    ///
    /// As [`text`] for either string, and [`HC_ERR_UNKNOWN`] for a code that
    /// names no table. An empty region is no region.
    ///
    /// # Safety
    ///
    /// As [`text`], for each pointer and its length.
    unsafe fn table_and_region<'a>(
        code: *const u8,
        code_len: usize,
        region: *const u8,
        region_len: usize,
    ) -> Result<(&'static RuleSet, Option<&'a str>), i64> {
        // SAFETY: forwarded to the caller's contract above.
        let code = unsafe { text(code, code_len) }?;
        // SAFETY: forwarded to the caller's contract above.
        let region = unsafe { text(region, region_len) }?;
        let table = table(code).ok_or(HC_ERR_UNKNOWN)?;
        Ok((table, (!region.is_empty()).then_some(region)))
    }

    /// Every table's identifier, one per line.
    pub(super) fn codes() -> String {
        let mut out = String::new();
        for set in countries::ALL
            .iter()
            .chain(exchanges::ALL)
            .chain(traditions::ALL)
            .chain(international::ALL)
        {
            out.push_str(set.code);
            out.push('\n');
        }
        out
    }

    /// The holidays of a year as lines: the ISO date, the name, the local
    /// name, the kind, the confidence, `1` for a substitute day and the date
    /// it stands in for, tab-separated.
    pub(super) fn lines(table: &RuleSet, region: Option<&str>, year: i64) -> String {
        use core::fmt::Write;
        let calendar = HolidayCalendar::for_year(table, region, year);
        let mut out = String::new();
        for holiday in calendar.all() {
            let kind = match holiday.kind {
                Kind::Public => "public",
                Kind::Bank => "bank",
                Kind::Religious => "religious",
                Kind::Observance => "observance",
                Kind::School => "school",
                Kind::Workday => "workday",
            };
            let confidence = match holiday.confidence {
                Confidence::Exact => "exact",
                Confidence::Approximate => "approximate",
            };
            let _ = write!(
                out,
                "{}\t{}\t{}\t{kind}\t{confidence}\t{}\t",
                iso(holiday.date),
                holiday.name,
                holiday.local_name,
                u8::from(holiday.is_substitute())
            );
            if let Some(day) = holiday.observed_for {
                out.push_str(&iso(day));
            }
            out.push('\n');
        }
        out
    }

    fn iso(day: Rd) -> String {
        hc::civil::Date::from_ordinal(day.0)
            .map_or_else(|_| String::from("?"), |date| date.to_string())
    }

    /// Whether a fixed day is a day off in a holiday table: 1, 0, or an
    /// error sentinel.
    ///
    /// `code` names the table — a country's ISO 3166-1 alpha-2 code, an
    /// exchange's ISO 10383 Market Identifier Code, a tradition's slug or
    /// `un-days` — and `region`, which may be empty, a subdivision's ISO
    /// 3166-2 code. A null pointer with a non-zero length is
    /// `HC_ERR_NULL_POINTER`, text that is not UTF-8 `HC_ERR_NOT_UTF8`, and a
    /// code that names no table `HC_ERR_UNKNOWN`.
    ///
    /// # Safety
    ///
    /// `code` must be readable for `code_len` bytes and `region` for
    /// `region_len`, unless null with a zero length.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holiday_is_day_off(
        code: *const u8,
        code_len: usize,
        region: *const u8,
        region_len: usize,
        fixed: i64,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let (table, region) = match unsafe { table_and_region(code, code_len, region, region_len) }
        {
            Ok(found) => found,
            Err(sentinel) => return sentinel,
        };
        let day = Rd(fixed);
        let Ok(year) = gregorian::year_from_fixed(day) else {
            return HC_ERR_OUT_OF_RANGE;
        };
        i64::from(HolidayCalendar::for_year(table, region, year).is_holiday(day))
    }

    /// The holidays of a Gregorian year in a table, as UTF-8 lines,
    /// returning the byte length written.
    ///
    /// One line per entry, tab-separated: the ISO 8601 date, the name, the
    /// local name, the kind (`public`, `bank`, `religious`, `observance`,
    /// `school` or `workday`), the confidence (`exact` or `approximate`), `1` for a
    /// substitute day and `0` otherwise, and the date the substitute
    /// stands in for or nothing. A null `buffer` returns the length the
    /// text needs, so a caller can allocate exactly. The string arguments
    /// fail as for `hc_holiday_is_day_off`.
    ///
    /// # Safety
    ///
    /// `code` and `region` as for `hc_holiday_is_day_off`; `buffer` must be
    /// writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holidays_in_year(
        code: *const u8,
        code_len: usize,
        region: *const u8,
        region_len: usize,
        year: i64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let (table, region) = match unsafe { table_and_region(code, code_len, region, region_len) }
        {
            Ok(found) => found,
            Err(sentinel) => return sentinel,
        };
        let text = lines(table, region, year);
        if buffer.is_null() {
            return text.len() as i64;
        }
        if capacity < text.len() {
            return HC_ERR_BUFFER_TOO_SMALL;
        }
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit(&text, buffer, capacity) }
    }

    /// The identifier of every holiday table, one per line, returning the
    /// byte length written.
    ///
    /// Countries first, then exchanges, traditions and the international
    /// sets, each as `hc_holiday_is_day_off` accepts it. A null `buffer`
    /// returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holiday_codes(buffer: *mut u8, capacity: usize) -> i64 {
        let text = codes();
        if buffer.is_null() {
            return text.len() as i64;
        }
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit(&text, buffer, capacity) }
    }
}

#[cfg(feature = "holiday")]
pub use holiday::{hc_holiday_codes, hc_holiday_is_day_off, hc_holidays_in_year};

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
            HC_ERR_NULL_POINTER,
            HC_ERR_UNKNOWN,
            HC_ERR_NOT_UTF8,
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
        assert_eq!(
            unsafe { hc_parse_iso_date(core::ptr::null(), 0) },
            HC_ERR_INVALID_DATE
        );
        assert_eq!(
            unsafe { hc_parse_iso_date(core::ptr::null(), 10) },
            HC_ERR_NULL_POINTER
        );
        let not_utf8 = [0xffu8, b'-', b'0'];
        assert_eq!(
            unsafe { hc_parse_iso_date(not_utf8.as_ptr(), not_utf8.len()) },
            HC_ERR_NOT_UTF8
        );
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

    #[cfg(feature = "holiday")]
    #[test]
    fn holiday_tables_answer_by_identifier() {
        let jp = b"JP";
        let day = hc_gregorian_to_fixed(2026, 1, 1);
        assert_eq!(
            unsafe { hc_holiday_is_day_off(jp.as_ptr(), 2, core::ptr::null(), 0, day) },
            1
        );
        let xnys = b"XNYS";
        let good_friday = hc_gregorian_to_fixed(2026, 4, 3);
        assert_eq!(
            unsafe { hc_holiday_is_day_off(xnys.as_ptr(), 4, core::ptr::null(), 0, good_friday) },
            1
        );
        let us = b"US";
        assert_eq!(
            unsafe { hc_holiday_is_day_off(us.as_ptr(), 2, core::ptr::null(), 0, good_friday) },
            0
        );
        let zz = b"ZZ";
        assert_eq!(
            unsafe { hc_holiday_is_day_off(zz.as_ptr(), 2, core::ptr::null(), 0, day) },
            HC_ERR_UNKNOWN
        );
        let not_utf8 = [0xffu8];
        assert_eq!(
            unsafe { hc_holiday_is_day_off(not_utf8.as_ptr(), 1, core::ptr::null(), 0, day) },
            HC_ERR_NOT_UTF8
        );
        assert_eq!(
            unsafe { hc_holiday_is_day_off(jp.as_ptr(), 2, not_utf8.as_ptr(), 1, day) },
            HC_ERR_NOT_UTF8
        );
        assert_eq!(
            unsafe { hc_holiday_is_day_off(core::ptr::null(), 2, core::ptr::null(), 0, day) },
            HC_ERR_NULL_POINTER
        );
        assert_eq!(
            unsafe {
                hc_holidays_in_year(
                    zz.as_ptr(),
                    2,
                    core::ptr::null(),
                    0,
                    2026,
                    core::ptr::null_mut(),
                    0,
                )
            },
            HC_ERR_UNKNOWN
        );
        // A null buffer measures; a sized one receives the lines.
        let needed = unsafe {
            hc_holidays_in_year(
                jp.as_ptr(),
                2,
                core::ptr::null(),
                0,
                2026,
                core::ptr::null_mut(),
                0,
            )
        };
        assert!(needed > 0);
        let capacity = needed as usize;
        let pointer = hc_alloc(capacity);
        let written = unsafe {
            hc_holidays_in_year(
                jp.as_ptr(),
                2,
                core::ptr::null(),
                0,
                2026,
                pointer,
                capacity,
            )
        };
        assert_eq!(written, needed);
        let text = unsafe { core::str::from_utf8(core::slice::from_raw_parts(pointer, capacity)) }
            .expect("UTF-8");
        assert!(
            text.starts_with("2026-01-01\tNew Year's Day\t元日\tpublic\texact\t0\t\n"),
            "{text}"
        );
        assert!(text.contains("\t1\t2026-05-03\n"), "{text}");
        unsafe { hc_free(pointer, capacity) };
        let codes_len = unsafe { hc_holiday_codes(core::ptr::null_mut(), 0) };
        assert!(codes_len > 0);
        let capacity = codes_len as usize;
        let pointer = hc_alloc(capacity);
        assert_eq!(unsafe { hc_holiday_codes(pointer, capacity) }, codes_len);
        let text = unsafe { core::str::from_utf8(core::slice::from_raw_parts(pointer, capacity)) }
            .expect("UTF-8");
        assert!(text.contains("\nJP\n") && text.contains("\nXNYS\n") && text.contains("un-days\n"));
        unsafe { hc_free(pointer, capacity) };
    }
}
