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
//! a few lines, shown below, and they are lines the caller can read; written
//! once for every export, they are the binding in `js/hyper-calendar.js`
//! beside this crate, which the README describes.
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
//! constants, all at or below [`HC_ERR_FLOOR`], and the rule is that a
//! value-returning export never returns a number at or below
//! [`HC_ERR_FLOOR`] except as an error. A day number never comes near it:
//! [`HC_ERR_FLOOR`] is more than a thousand times the age of the universe
//! in days. A count of seconds can, about 285 million years back, so the
//! exports that answer in seconds refuse such a day with
//! [`HC_ERR_OUT_OF_RANGE`] rather than return a number every binding would
//! read as a sentinel, and refuse the same way a result that would
//! overflow an `i64`. The README states the range each export answers for.
//!
//! # Lines and cells
//!
//! Every export that answers with more than one value writes UTF-8 lines
//! ending in `\n`, one per entry, with the cells of a line separated by
//! `\t`. The column order of each is fixed, stated on the export and in the
//! README, and only ever grows at the end. A cell that has nothing to say is
//! empty, never a placeholder, and a cell's text never contains a tab or a
//! line break: the few source strings that do have them replaced by a
//! space. Called with a null `buffer`, such an export returns the byte
//! length the text needs, so the caller can allocate exactly and call
//! again.
//!
//! # Layers
//!
//! The exports come in layers that a page can load as it needs them, each a
//! Cargo feature: `civil` (the default), `timestamps`, `calendars`, `holiday`, `seasons`,
//! `deep-time`, `tz`, `sky`, `orbital`, `planetary` and `relativity`, with
//! `full` for all of them.
//! Which feature each export needs is in the README's table.

#![allow(unsafe_code)]
#![warn(missing_docs)]

/// Any return value at or below this is an error sentinel, not a result.
///
/// The age of the universe is about 5 × 10¹² days, so no legitimate fixed day
/// number comes anywhere near this. A count of seconds does, about 285
/// million years back, and an export that answers in seconds returns
/// [`HC_ERR_OUT_OF_RANGE`] for a result that would reach it.
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
/// Data was not in the format the call expects.
pub const HC_ERR_MALFORMED: i64 = -9_000_000_000_000_008;

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
#[allow(dead_code)]
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

/// A computed value as a value-returning export may return it: the value
/// when there is one above [`HC_ERR_FLOOR`], and [`HC_ERR_OUT_OF_RANGE`]
/// when the arithmetic overflowed (`None`) or the value would read as a
/// sentinel.
///
/// This is the one place the rule is kept: no legitimate result is ever at
/// or below [`HC_ERR_FLOOR`].
#[allow(dead_code)]
fn above_floor(value: Option<i64>) -> Result<i64, i64> {
    value
        .filter(|value| *value > HC_ERR_FLOOR)
        .ok_or(HC_ERR_OUT_OF_RANGE)
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

/// Copy `text` into the caller's buffer, or measure it for a null buffer.
///
/// This is the contract of every export that writes lines: a null `buffer`
/// asks for the length the text needs, a buffer that is too small is
/// refused untouched, and otherwise the byte length written comes back.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes unless it is null.
#[allow(dead_code)]
unsafe fn emit_or_measure(text: &str, buffer: *mut u8, capacity: usize) -> i64 {
    if buffer.is_null() {
        return text.len() as i64;
    }
    // SAFETY: forwarded to the caller's contract above.
    unsafe { emit(text, buffer, capacity) }
}

/// Append one cell of a line: `text` with any tab or line break replaced
/// by a space, so the line format survives whatever a source string holds.
#[allow(dead_code)]
fn push_cell(out: &mut String, text: &str) {
    for character in text.chars() {
        out.push(match character {
            '\t' | '\n' | '\r' => ' ',
            other => other,
        });
    }
}

/// The sentinel a refusal of the shared line-makers in `hyper_calendar`
/// is: an overflow is [`HC_ERR_OUT_OF_RANGE`], since the module has no
/// sentinel of its own for it.
#[allow(dead_code)]
const fn sentinel(refusal: hc::boundary::Refusal) -> i64 {
    use hc::boundary::Refusal;
    match refusal {
        Refusal::OutOfRange | Refusal::Overflow => HC_ERR_OUT_OF_RANGE,
        Refusal::NoData => HC_ERR_NO_DATA,
        Refusal::Unknown => HC_ERR_UNKNOWN,
        Refusal::Malformed => HC_ERR_MALFORMED,
        Refusal::InvalidDate => HC_ERR_INVALID_DATE,
    }
}

/// A shared line-maker's answer, measured or copied into the caller's
/// buffer as [`emit_or_measure`] does, or its refusal as a sentinel.
///
/// # Safety
///
/// As [`emit_or_measure`].
#[allow(dead_code)]
unsafe fn emit_answer(
    answer: hc::boundary::Answer<String>,
    buffer: *mut u8,
    capacity: usize,
) -> i64 {
    match answer {
        // SAFETY: forwarded to the caller's contract above.
        Ok(text) => unsafe { emit_or_measure(&text, buffer, capacity) },
        Err(refusal) => sentinel(refusal),
    }
}

/// A shared number as a value-returning export returns it: the number
/// when it is above [`HC_ERR_FLOOR`], else a sentinel.
#[allow(dead_code)]
fn value(answer: hc::boundary::Answer<i64>) -> i64 {
    match answer
        .map_err(sentinel)
        .and_then(|value| above_floor(Some(value)))
    {
        Ok(value) | Err(value) => value,
    }
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

/// The civil calendar, behind the `civil` feature: proleptic Gregorian
/// dates, ISO 8601 text, POSIX time, TAI − UTC and leap seconds.
#[cfg(feature = "civil")]
mod civil {
    use super::{
        HC_ERR_INVALID_DATE, HC_ERR_NO_DATA, HC_ERR_OUT_OF_RANGE, above_floor, emit, text,
    };
    use hc::civil::Date;
    use hc::hc_calendar::fixed::RD_OF_UNIX_EPOCH;
    use hc::hc_calendar::{Rd, Weekday};
    use hc::hc_core::unix::{self, LeapPolicy};

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
    ///
    /// A timestamp in a day whose start or whose end is not an `i64` — the
    /// first and last part-days of the range — is [`HC_ERR_OUT_OF_RANGE`].
    #[unsafe(no_mangle)]
    pub extern "C" fn hc_day_has_leap_second(unix_seconds: i64) -> i64 {
        let Some((day_start, next_day)) = unix_seconds
            .div_euclid(86_400)
            .checked_mul(86_400)
            .and_then(|start| Some((start, start.checked_add(86_400)?)))
        else {
            return HC_ERR_OUT_OF_RANGE;
        };
        let policy = LeapPolicy::Extrapolate;
        let (Ok(before), Ok(after)) = (
            unix::tai_minus_utc_at(day_start, policy),
            unix::tai_minus_utc_at(next_day, policy),
        ) else {
            return HC_ERR_NO_DATA;
        };
        i64::from(after > before)
    }

    /// The POSIX timestamp of midnight UTC on a fixed day.
    ///
    /// A day whose midnight would be at or below [`HC_ERR_FLOOR`] seconds —
    /// before fixed day −104 165 947 503, about 285 million years back — or
    /// would overflow an `i64` — after fixed day 106 751 991 886 463 — is
    /// [`HC_ERR_OUT_OF_RANGE`].
    ///
    /// [`HC_ERR_FLOOR`]: super::HC_ERR_FLOOR
    #[unsafe(no_mangle)]
    pub extern "C" fn hc_unix_from_fixed(fixed: i64) -> i64 {
        let midnight = fixed
            .checked_sub(RD_OF_UNIX_EPOCH)
            .and_then(|days| days.checked_mul(86_400));
        match above_floor(midnight) {
            Ok(seconds) => seconds,
            Err(sentinel) => sentinel,
        }
    }

    /// Render a fixed day as an ISO 8601 date, returning the byte length written.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_format_iso_date(
        fixed: i64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
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
}

#[cfg(feature = "civil")]
pub use civil::{
    hc_day_has_leap_second, hc_day_of_year, hc_fixed_from_unix, hc_format_iso_date,
    hc_gregorian_day, hc_gregorian_month, hc_gregorian_to_fixed, hc_gregorian_year,
    hc_is_leap_year, hc_parse_iso_date, hc_tai_minus_utc, hc_unix_from_fixed, hc_weekday,
};

/// Time scales and day counts, behind the `timestamps` feature: TAI64 labels
/// in both conventions, GNSS weeks, GLONASS dates, OLE Automation dates,
/// Excel 1900 serials, UUID timestamps, NTP eras, FAT date and time words,
/// Swatch Internet Time, Julian and Besselian epochs, and TT(BIPM) from a
/// series the caller supplies. The lines are `hyper_calendar::time_lines`',
/// shared with the C library. A TAI instant
/// is whole seconds from 1970-01-01 00:00:00 TAI and the attoseconds into
/// that second, from 0 to 10¹⁸ − 1; a POSIX instant and a TT instant, from
/// 1970-01-01 00:00:00 TT, cross the same way.
#[cfg(feature = "timestamps")]
mod time_scales {
    use super::{emit_answer, text, value};
    use hc::time_lines;

    /// A TAI instant as a TAI64, TAI64N or TAI64NA label in lower-case
    /// hexadecimal, as one UTF-8 line, returning the byte length written.
    ///
    /// `format` is `tai64` (16 digits: the second), `tai64n` (24: the
    /// nanosecond) or `tai64na` (32: the attosecond), in any case; anything
    /// else is `HC_ERR_UNKNOWN`. TAI64 and TAI64N name the second or the
    /// nanosecond that contains the instant, and TAI64NA the instant
    /// itself. Attoseconds from 10¹⁸, or a second that no TAI64 label can
    /// hold (the labels run below 2⁶³), is `HC_ERR_OUT_OF_RANGE`. A null
    /// `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `format` must be readable for `format_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless
    /// it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tai64_encode(
        tai_seconds: i64,
        attoseconds: u64,
        format: *const u8,
        format_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let format = match unsafe { text(format, format_len) } {
            Ok(format) => format,
            Err(sentinel) => return sentinel,
        };
        let answer = time_lines::tai64_encode_line(tai_seconds, attoseconds, format);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// A TAI64, TAI64N or TAI64NA label in hexadecimal read back, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// Tab-separated: the format (`tai64`, `tai64n` or `tai64na`, by the
    /// label's length), the TAI seconds and the attoseconds of the instant
    /// it names. Text that is not 16, 24 or 32 hexadecimal digits, in
    /// either case, is `HC_ERR_MALFORMED`; a reserved label, from 2⁶³, or a
    /// counter above 999 999 999 is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
    /// returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `hex` must be readable for `hex_len` bytes unless null with a zero
    /// length; `buffer` must be writable for `capacity` bytes unless it is
    /// null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tai64_decode(
        hex: *const u8,
        hex_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let hex = match unsafe { text(hex, hex_len) } {
            Ok(hex) => hex,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(time_lines::tai64_decode_line(hex), buffer, capacity) }
    }

    /// The GNSS week and time of week of a TAI instant, as one UTF-8 line,
    /// returning the byte length written.
    ///
    /// `numbering` names the broadcast week field: `gps-lnav-week` (ten
    /// bits), `gps-cnav-week` (thirteen), `galileo-week` (twelve),
    /// `beidou-week` (thirteen) or `navic-week` (ten), in any case; anything
    /// else is `HC_ERR_UNKNOWN`. Tab-separated: the full week since the
    /// field's week zero, the week as the field broadcasts it (the full
    /// week modulo 2 to the field's bits), and the time of week in whole
    /// seconds and attoseconds. An instant before week zero is
    /// `HC_ERR_NO_DATA`; attoseconds from 10¹⁸, or a week past 2³² − 1, is
    /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `numbering` must be readable for `numbering_len` bytes unless null
    /// with a zero length; `buffer` must be writable for `capacity` bytes
    /// unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gnss_week(
        numbering: *const u8,
        numbering_len: usize,
        tai_seconds: i64,
        attoseconds: u64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let numbering = match unsafe { text(numbering, numbering_len) } {
            Ok(numbering) => numbering,
            Err(sentinel) => return sentinel,
        };
        let answer = time_lines::gnss_week_line(numbering, tai_seconds, attoseconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The TAI instant of a full GNSS week and a time of week, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// `numbering` is as for `hc_gnss_week`. Tab-separated: the TAI seconds
    /// and the attoseconds. A time of week from 604 800 s, or attoseconds
    /// from 10¹⁸, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
    /// length the text needs.
    ///
    /// # Safety
    ///
    /// As `hc_gnss_week`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gnss_to_tai(
        numbering: *const u8,
        numbering_len: usize,
        week: u32,
        tow_seconds: u32,
        tow_attoseconds: u64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let numbering = match unsafe { text(numbering, numbering_len) } {
            Ok(numbering) => numbering,
            Err(sentinel) => return sentinel,
        };
        let answer = time_lines::gnss_to_tai_line(numbering, week, tow_seconds, tow_attoseconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The full GNSS week a broadcast week names, by a rollover rule and a
    /// reference instant, or an error sentinel.
    ///
    /// `numbering` is as for `hc_gnss_week`. `rule` is `not-before`, the
    /// first full week at or after the reference's week — for a date the
    /// receiver knows it is not before, such as its firmware's build date —
    /// or `nearest`, the one within half a rollover period of it, in any
    /// case; anything else is `HC_ERR_UNKNOWN`. `reference_tai_seconds` is
    /// the reference as whole TAI seconds; one before week zero counts as
    /// week zero. A broadcast week that does not fit the field, or an
    /// answer past week 2³² − 1, is `HC_ERR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `numbering` must be readable for `numbering_len` bytes and `rule`
    /// for `rule_len`, unless null with a zero length.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gnss_resolve_week(
        numbering: *const u8,
        numbering_len: usize,
        broadcast: u32,
        rule: *const u8,
        rule_len: usize,
        reference_tai_seconds: i64,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let numbering = match unsafe { text(numbering, numbering_len) } {
            Ok(numbering) => numbering,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        let rule = match unsafe { text(rule, rule_len) } {
            Ok(rule) => rule,
            Err(sentinel) => return sentinel,
        };
        value(
            time_lines::gnss_resolve_week(numbering, broadcast, rule, reference_tai_seconds)
                .map(i64::from),
        )
    }

    /// GLONASS's four-year interval N4 and day N_T at a TAI instant, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// Tab-separated: N4, 1 for 1996–1999, and N_T, 1 on 1 January of the
    /// interval's leap year, of GLONASS time, UTC(SU) + 3 h. The instant
    /// goes to UTC through the leap-second table: `strict` non-zero refuses
    /// outside it with `HC_ERR_NO_DATA`, zero holds the last published
    /// offset. An instant before 1996 or from 2100, where the intervals are
    /// not counted, or attoseconds from 10¹⁸, is `HC_ERR_OUT_OF_RANGE`. A
    /// null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_glonass_date(
        tai_seconds: i64,
        attoseconds: u64,
        strict: i32,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = time_lines::glonass_date_line(tai_seconds, attoseconds, strict != 0);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The fixed day and the time of day of an OLE Automation date, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// Tab-separated: the fixed day and the seconds into it. The integer
    /// part counts days from 30 December 1899 and the fraction is the time
    /// of day, read as a magnitude when the value is negative, as
    /// `DateTime.ToOADate` writes it: −1.25 is 06:00 on 29 December 1899. A
    /// value that is not finite, or outside 1 January 100 to 31 December
    /// 9999, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length
    /// the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_fixed_from_ole_automation(
        value: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = time_lines::fixed_from_ole_automation_line(value);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The OLE Automation date of a fixed day and a time of day, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// The one cell is the value, in plain decimal notation; before
    /// 30 December 1899 the time is subtracted, so 06:00 on 29 December 1899
    /// is −1.25. A time of day that is not finite, negative or not below
    /// 86 400 s, or a day outside 1 January 100 to 31 December 9999, is
    /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_ole_automation_from_fixed(
        fixed: i64,
        seconds_of_day: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = time_lines::ole_automation_from_fixed_line(fixed, seconds_of_day);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// What an Excel 1900 serial names, as one UTF-8 line, returning the
    /// byte length written.
    ///
    /// Tab-separated: the fixed day, and `1` for serial 60, which Excel
    /// counts as 29 February 1900, a day that never was, else `0`. Serial 60
    /// has an empty first cell: it is named, not given a date. A serial
    /// below 1 or above 2 958 465, 31 December 9999, is
    /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_excel_1900_day(
        serial: i64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(time_lines::excel_1900_day_line(serial), buffer, capacity) }
    }

    /// A POSIX timestamp as a TAI reading, as one UTF-8 line, returning the
    /// byte length written.
    ///
    /// The C library's entry point of the same name, in this module's
    /// `timestamps` layer rather than `civil`, as its 128-bit arithmetic
    /// is: here it would grow `civil` by a sixth.
    ///
    /// Tab-separated: the whole TAI seconds from 1970-01-01 00:00:00 TAI and
    /// the attoseconds into that second, the TAI instant the other exports
    /// of this layer take. `strict` non-zero refuses before
    /// 1961 and past the announced leap-second table with `HC_ERR_NO_DATA`;
    /// zero holds the last published offset, and before 1961 treats UTC as
    /// TAI. A timestamp after `i64::MAX − 37`, whose TAI seconds no `i64`
    /// holds, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length
    /// the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tai_from_unix(
        unix_seconds: i64,
        strict: i32,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = time_lines::tai_from_unix_line(unix_seconds, strict != 0);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// A whole TAI second as a UTC label, as one UTF-8 line, returning the
    /// byte length written.
    ///
    /// Tab-separated: the POSIX second, and `1` when the TAI second is an
    /// inserted leap second, `23:59:60`, which POSIX time cannot express
    /// and names by the second after it, else `0`. `tai_seconds` counts
    /// from 1970-01-01 00:00:00 TAI; `strict` is as for `hc_tai_from_unix`.
    /// A null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_utc_from_tai(
        tai_seconds: i64,
        strict: i32,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = time_lines::utc_from_tai_line(tai_seconds, strict != 0);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// A POSIX instant as a TAI64 or TAI64N label in the
    /// `tai64-posix-plus-10` convention, in lower-case hexadecimal, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// The convention is the label daemontools' `tai64n` writes on a clock
    /// that keeps POSIX time: 2⁶² + 10 + the POSIX seconds, with no
    /// leap-second table, so POSIX 0 is `400000000000000a`. It is not
    /// `hc_tai64_encode`'s true TAI, and nothing in the bytes says which
    /// wrote them. `format` is `tai64` (16 digits, the second) or `tai64n`
    /// (24, the nanosecond), in any case; `tai64na`, which the convention
    /// does not write, and anything else is `HC_ERR_UNKNOWN`. Attoseconds
    /// from 10¹⁸, or a second whose label would fall outside 0 to 2⁶³ − 1,
    /// is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the
    /// text needs.
    ///
    /// # Safety
    ///
    /// As `hc_tai64_encode`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tai64_posix_plus_10_encode(
        unix_seconds: i64,
        attoseconds: u64,
        format: *const u8,
        format_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let format = match unsafe { text(format, format_len) } {
            Ok(format) => format,
            Err(sentinel) => return sentinel,
        };
        let answer = time_lines::tai64_posix_plus_10_encode_line(unix_seconds, attoseconds, format);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// A TAI64 or TAI64N label in the `tai64-posix-plus-10` convention read
    /// back, as one UTF-8 line, returning the byte length written.
    ///
    /// Tab-separated: the format (`tai64` or `tai64n`, by the label's
    /// length), the POSIX seconds and the attoseconds of the instant it
    /// names. Text that is not 16 or 24 hexadecimal digits, in either case,
    /// is `HC_ERR_MALFORMED`; a reserved label, from 2⁶³, or a nanosecond
    /// count above 999 999 999 is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
    /// returns the length the text needs.
    ///
    /// # Safety
    ///
    /// As `hc_tai64_decode`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tai64_posix_plus_10_decode(
        hex: *const u8,
        hex_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let hex = match unsafe { text(hex, hex_len) } {
            Ok(hex) => hex,
            Err(sentinel) => return sentinel,
        };
        let answer = time_lines::tai64_posix_plus_10_decode_line(hex);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The timestamp of a version 1 or version 6 UUID, as one UTF-8 line,
    /// returning the byte length written.
    ///
    /// `uuid` is RFC 9562's string form: 32 hexadecimal digits in either
    /// case, bare or hyphenated 8-4-4-4-12, optionally after `urn:uuid:`;
    /// any other text is `HC_ERR_MALFORMED`. Tab-separated: the version,
    /// `1` or `6`; the 60-bit timestamp, 100-nanosecond intervals from
    /// 1582-10-15 00:00 UTC; and the POSIX seconds and attoseconds of the
    /// start of that interval. A UUID of another version or variant, which
    /// carries no timestamp, is `HC_ERR_NO_DATA`. The count is what the
    /// generator wrote, as POSIX time counts, with no leap second. A null
    /// `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `uuid` must be readable for `uuid_len` bytes unless null with a zero
    /// length; `buffer` must be writable for `capacity` bytes unless it is
    /// null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_uuid_timestamp(
        uuid: *const u8,
        uuid_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let uuid = match unsafe { text(uuid, uuid_len) } {
            Ok(uuid) => uuid,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(time_lines::uuid_timestamp_line(uuid), buffer, capacity) }
    }

    /// A 64-bit NTP timestamp placed in its era by a reference time, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// `seconds` and `fraction` are the timestamp's two 32-bit halves, the
    /// seconds of an era from 1900-01-01 00:00 UTC and 2⁻³² s units of the
    /// second; the timestamp names no era, and the era is the one that puts
    /// it within 2³¹ s, some 68 years, of `reference_unix`, from 2³¹ s
    /// before it, included, to 2³¹ s after it, excluded, as RFC 5905 §6
    /// says a client set within 68 years of the server reads it.
    /// Tab-separated: the era, 0 for 1900 to 2036; the era offset; the
    /// fraction in 2⁻⁶⁴ s units; and the POSIX seconds and attoseconds. The
    /// zero timestamp, which RFC 5905 reserves for unknown or
    /// unsynchronised time, is `HC_ERR_NO_DATA`; a reference so far off
    /// that the era or the second leaves an `i64` is `HC_ERR_OUT_OF_RANGE`.
    /// A null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_ntp_resolve(
        seconds: u32,
        fraction: u32,
        reference_unix: i64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = time_lines::ntp_resolve_line(seconds, fraction, reference_unix);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The 60-bit UUID timestamp of a POSIX instant, and the time fields a
    /// version 1 and a version 6 UUID write it in, as one UTF-8 line,
    /// returning the byte length written.
    ///
    /// Tab-separated: the timestamp, the 100-nanosecond interval that
    /// contains the instant counted from 1582-10-15 00:00 UTC, the
    /// fraction below 100 ns dropped; then the first three groups of a
    /// version 1 UUID that carries it and of a version 6 one, RFC 9562's
    /// hex-and-dash form in lower case, such as `c232ab00-9414-11ec` and
    /// `1ec9414c-232a-6b00`, for the caller's clock sequence and node to
    /// follow. The instant is counted as POSIX time counts, with no leap
    /// second. Attoseconds from 10¹⁸, or an instant before 1582-10-15 or
    /// after the field's last interval on 5236-03-31, is
    /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_uuid_timestamp_encode(
        unix_seconds: i64,
        attoseconds: u64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = time_lines::uuid_timestamp_encode_line(unix_seconds, attoseconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The NTP date and timestamp of a POSIX instant, as one UTF-8 line,
    /// returning the byte length written.
    ///
    /// Tab-separated: the era, 0 for 1900 to 2036; the era offset; the
    /// fraction in 2⁻⁶⁴ s units, floored; the 128-bit date in RFC 5905's
    /// Figure 3 layout, era, offset and fraction, as 32 lower-case
    /// hexadecimal digits; and the 64-bit timestamp of the packet headers,
    /// the offset and the top 32 bits of the fraction with the era
    /// dropped, as 16. The seconds count whole 86 400-second days from
    /// 1900, as RFC 5905 §6's table does, with no leap second. Attoseconds
    /// from 10¹⁸, or a second so late that its count from 1900 leaves an
    /// `i64`, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length
    /// the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_ntp_encode(
        unix_seconds: i64,
        attoseconds: u64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = time_lines::ntp_encode_line(unix_seconds, attoseconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The local reading a FAT date word and time word name, as one UTF-8
    /// line, returning the byte length written.
    ///
    /// Tab-separated: the fixed day and the seconds into it, always even.
    /// The words are a wall-clock reading in a zone they do not record, so
    /// the day is a local one and no instant is claimed. A word above
    /// 65 535 is `HC_ERR_OUT_OF_RANGE`, and one whose fields name no day or
    /// no time — month 0 or above 12, day 0 or one the month lacks, hour
    /// above 23, minute above 59, halved second above 29 — is
    /// `HC_ERR_INVALID_DATE`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_fat_decode(
        date: u32,
        time: u32,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(time_lines::fat_decode_line(date, time), buffer, capacity) }
    }

    /// The FAT date and time words of a fixed day and a time of day, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// Tab-separated: the date word and the time word, the second rounded
    /// down to an even one as the word carries it. A time of day from
    /// 86 400 s, or a day outside 1980 to 2107, the years the date word
    /// holds, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length
    /// the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_fat_encode(
        fixed: i64,
        seconds_of_day: u32,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = time_lines::fat_encode_line(fixed, seconds_of_day);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The Swatch Internet Time at a POSIX instant, 0 through 999, or an
    /// error sentinel.
    ///
    /// The day of Biel Mean Time, UTC+1 all year, in a thousand beats of
    /// 86.4 s: @000 begins at 23:00 UTC. Attoseconds from 10¹⁸ are
    /// `HC_ERR_OUT_OF_RANGE`.
    #[unsafe(no_mangle)]
    pub extern "C" fn hc_swatch_beat(unix_seconds: i64, attoseconds: u64) -> i64 {
        value(time_lines::swatch_beat(unix_seconds, attoseconds).map(i64::from))
    }

    /// The Julian or Besselian epoch of a TT instant, as one UTF-8 line,
    /// returning the byte length written.
    ///
    /// `notation` is `J` or `julian-epoch`, for Julian years of 365.25 days
    /// of TT from J2000.0 = 2000-01-01T12:00:00 TT, or `B` or
    /// `besselian-epoch`, for Besselian years of 365.242 198 781 days from
    /// B1900.0 = JD 2415020.31352, in any case; anything else is
    /// `HC_ERR_UNKNOWN`. The instant is whole seconds from
    /// 1970-01-01 00:00:00 TT and attoseconds; TT is TAI + 32.184 s.
    /// Tab-separated: the notation's letter and the epoch, `2000` for
    /// J2000.0. Attoseconds from 10¹⁸ are `HC_ERR_OUT_OF_RANGE`. A null
    /// `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `notation` must be readable for `notation_len` bytes unless null
    /// with a zero length; `buffer` must be writable for `capacity` bytes
    /// unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_epoch_from_tt(
        notation: *const u8,
        notation_len: usize,
        tt_seconds: i64,
        attoseconds: u64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let notation = match unsafe { text(notation, notation_len) } {
            Ok(notation) => notation,
            Err(sentinel) => return sentinel,
        };
        let answer = time_lines::epoch_from_tt_line(notation, tt_seconds, attoseconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The TT instant of a Julian or Besselian epoch, as one UTF-8 line,
    /// returning the byte length written.
    ///
    /// `notation` is as for `hc_epoch_from_tt`, or empty for an epoch
    /// written without a letter, which SOFA reads as Besselian before
    /// 1984.0 and Julian from it. Tab-separated: the letter the year was
    /// read in, and the whole seconds from 1970-01-01 00:00:00 TT and the
    /// attoseconds. A year that is not finite, or whose instant leaves an
    /// `i64` of seconds, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns
    /// the length the text needs.
    ///
    /// # Safety
    ///
    /// As `hc_epoch_from_tt`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tt_from_epoch(
        notation: *const u8,
        notation_len: usize,
        year: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let notation = match unsafe { text(notation, notation_len) } {
            Ok(notation) => notation,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe {
            emit_answer(
                time_lines::tt_from_epoch_line(notation, year),
                buffer,
                capacity,
            )
        }
    }

    /// TT(BIPM) at a TAI instant, read from a realisation the caller
    /// supplies, as one UTF-8 line, returning the byte length written.
    ///
    /// `series` is the realisation as text, one line per sample: the
    /// Modified Julian Date at 0 h UTC and TT(BIPMxx) − TAI − 32.184 s there
    /// in microseconds, separated by a tab, the dates ascending, as the
    /// first and third columns of the BIPM's `TTBIPM` files give them.
    /// Blank lines are skipped, and text in any other shape is
    /// `HC_ERR_MALFORMED`. The instant is whole seconds from
    /// 1970-01-01 00:00:00 TAI and attoseconds. Tab-separated: TT(BIPMxx) −
    /// TT(TAI) in seconds, interpolated linearly in TAI between the
    /// samples; TT(BIPMxx) − TAI as whole seconds and attoseconds; and the
    /// TT(BIPMxx) reading of the instant as whole seconds from
    /// 1970-01-01 00:00:00 of that scale and attoseconds. An instant before
    /// the first sample or after the last, or an empty series, is
    /// `HC_ERR_NO_DATA`: the series is never extrapolated. `strict`
    /// non-zero places the samples by the leap-second table and refuses one
    /// outside it with `HC_ERR_NO_DATA`; zero holds the table's ends.
    /// Attoseconds from 10¹⁸ are `HC_ERR_OUT_OF_RANGE`. A null `buffer`
    /// returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `series` must be readable for `series_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless
    /// it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tt_bipm(
        series: *const u8,
        series_len: usize,
        tai_seconds: i64,
        attoseconds: u64,
        strict: i32,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let series = match unsafe { text(series, series_len) } {
            Ok(series) => series,
            Err(sentinel) => return sentinel,
        };
        let answer = time_lines::tt_bipm_line(series, tai_seconds, attoseconds, strict != 0);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }
}

#[cfg(feature = "timestamps")]
pub use time_scales::{
    hc_epoch_from_tt, hc_excel_1900_day, hc_fat_decode, hc_fat_encode,
    hc_fixed_from_ole_automation, hc_glonass_date, hc_gnss_resolve_week, hc_gnss_to_tai,
    hc_gnss_week, hc_ntp_encode, hc_ntp_resolve, hc_ole_automation_from_fixed, hc_swatch_beat,
    hc_tai_from_unix, hc_tai64_decode, hc_tai64_encode, hc_tai64_posix_plus_10_decode,
    hc_tai64_posix_plus_10_encode, hc_tt_bipm, hc_tt_from_epoch, hc_utc_from_tai,
    hc_uuid_timestamp, hc_uuid_timestamp_encode,
};

/// Every calendar, behind the `calendars` feature: the registry the facade
/// populates, described for one day, walked as eras, years, months and
/// days, and listed, in the vocabulary of a locale; the locales
/// themselves; when each country adopted the Gregorian calendar; and the
/// month and weekday names a government decreed for a period. The lines
/// are `hyper_calendar::lines`', shared with the C library.
#[cfg(feature = "calendars")]
mod calendars {
    use super::{HC_ERR_UNKNOWN, emit_answer, emit_or_measure, text};
    use hc::hc_calendar::Rd;
    use hc::hc_calendar::units::Unit;
    use hc::lines;

    /// One fixed day in every registered calendar, as UTF-8 lines, returning
    /// the byte length written.
    ///
    /// One line per calendar, in registry order, tab-separated: the calendar
    /// identifier, its English name, the era code, the era's name in the
    /// locale (or the calendar's own name for it), the year, the month
    /// ordinal, `1` for a leap month, the month's name in the locale (or the
    /// calendar's own name for it), the day, `1` for a leap day, the
    /// calendar's extra fields as `name=value` pairs joined by `;`, the error
    /// code, the error name, the standing (`in-use`, `proleptic`, `extended`
    /// or `unrecorded`), where the calendar's day begins (`midnight`, `noon`,
    /// `sunset`, `sunrise` or `local-time HH:MM:SS`), the date as the locale
    /// writes it (令和8年9月21日, 癸卯年闰二月初一), the locale used, and
    /// which civil day names a day that does not begin at midnight (`start`
    /// for the one it begins on, `end` for the one it ends on, empty for
    /// midnight). A
    /// calendar that refuses the day is still a line: its date columns,
    /// standing and formatted date are empty and the error code and name say
    /// why. `locale` is a BCP 47 tag, or `native` for each calendar's own
    /// language; a calendar the tag's data does not name is rendered in
    /// English, else in the tag with the calendar's own names, never in the
    /// calendar's own language, and the last column says which. A tag that does not parse is the root locale
    /// `und`, whose month names are CLDR's `M01`..`M12` — ask for `en` for
    /// English. A null `buffer` returns the length the text needs. The locale
    /// argument fails as `hc_parse_iso_date` does.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_describe_day(
        fixed: i64,
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let text = lines::describe_day(&hc::registry(), Rd(fixed), tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// The days from `from_fixed` up to but not including `to_fixed` as one
    /// calendar's eras, years, months or days, as UTF-8 lines, returning the
    /// byte length written.
    ///
    /// `id` is a registry identifier — `gregory`, `chinese`, `japanese` —
    /// and `unit` is `0` for eras, `1` for years, `2` for months and `3` for
    /// days; an identifier or unit the module does not know is
    /// `HC_ERR_UNKNOWN`. One line per span, in order and touching end to
    /// start, tab-separated: the first day of the span, the day after its
    /// last, its label in the locale (令和元年, `Adar I`, 閏二月, 初四), `1`
    /// for an intercalary unit, the standing of its first day, the error
    /// code, the error name and the locale used. The first and last spans
    /// are whole units and may reach outside the range asked for. A span the
    /// calendar refuses — days before its epoch or past its table, a unit it
    /// does not have — has an empty label, leap flag and standing and
    /// carries the refusal's code and name. An empty range writes nothing,
    /// and a range of more than 100 000 spans, `hyper_calendar::lines`'
    /// `MAX_CALENDAR_UNITS`, is `HC_ERR_OUT_OF_RANGE`: the text would grow
    /// without bound, one line a unit, and a caller that wants more asks in
    /// pieces. `locale` is as for `hc_describe_day`, `native` included. A
    /// null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `id` must be readable for `id_len` bytes and `locale` for
    /// `locale_len` bytes, unless null with a zero length; `buffer` must be
    /// writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_calendar_units(
        id: *const u8,
        id_len: usize,
        unit: u32,
        from_fixed: i64,
        to_fixed: i64,
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let id = match unsafe { text(id, id_len) } {
            Ok(id) => id,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let Some(unit) = Unit::from_index(unit) else {
            return HC_ERR_UNKNOWN;
        };
        let registry = hc::registry();
        let Some(calendar) = registry.get_by_name(id) else {
            return HC_ERR_UNKNOWN;
        };
        let answer = lines::calendar_units(calendar, unit, Rd(from_fixed), Rd(to_fixed), tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// Every registered calendar, as UTF-8 lines, returning the byte length
    /// written.
    ///
    /// One line per calendar, in registry order, tab-separated: the
    /// identifier, what the locale calls the calendar (和暦, or empty where
    /// it has no name), its English name, the earliest and latest fixed days
    /// it converts (empty where unbounded), whether it has eras, years,
    /// months and days as `1` or `0` each, the languages its sources are
    /// written in as BCP 47 tags joined by `;` (empty for a day count, a
    /// proposal or the Gregorian family), and its standing on `today`
    /// (`in-use`, `proleptic`, `extended` or `unrecorded`). `locale` is as
    /// for `hc_describe_day`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_calendars(
        today: i64,
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let text = lines::calendars(&hc::registry(), Rd(today), tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// Every registered calendar by name alone, as UTF-8 lines, returning
    /// the byte length written.
    ///
    /// One line per calendar, in registry order, tab-separated: the
    /// identifier, what the locale calls the calendar (和暦, or empty where
    /// it has no name), its English name, the locale used (the tag of the
    /// data entry the name came from, empty where the name is), the crate
    /// that registers it (`hc-calendars-solar`, `hc-calendars-lunar`,
    /// `hc-calendars-equinox`, `hc-calendars-indic` or
    /// `hc-calendars-regional`), and the languages of its sources as
    /// `hc_calendars` gives them, BCP 47 tags joined by `;` or empty, so
    /// that a menu can list a reader's own calendars first. The names are
    /// `hc_calendars`', without the
    /// range, the units and the standing on a day, so nothing is converted:
    /// for a menu of calendars, which a page asks for far more often than it
    /// describes a day. `locale` is as for `hc_describe_day`. A null
    /// `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_calendar_list(
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let text = lines::calendar_list(&hc::registry(), tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// Every locale the module carries, as UTF-8 lines, returning the byte
    /// length written.
    ///
    /// One line per locale, in tag order, tab-separated: the BCP 47 tag, the
    /// language's name in English and in itself, whether the locale's own
    /// data names the Gregorian months, the weekdays and the Gregorian eras
    /// as `1` or `0` each, and the identifiers of the calendars it has
    /// vocabulary of its own for beyond the shared Gregorian months, joined
    /// by `;`. A null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_locales(buffer: *mut u8, capacity: usize) -> i64 {
        let text = lines::locales();
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// The ISO weekday of the first day of the week in a locale, Monday = 1
    /// through Sunday = 7, or an error sentinel.
    ///
    /// `locale` is a BCP 47 tag, read as `hc-i18n` reads CLDR 48's week
    /// data: a `-u-fw-` key first, then the tag's region (`en-US` is 7,
    /// `en-GB` 1), then, for a tag without a region, the region its
    /// language's likely subtags give (`ja` is 7, `fr` 1). A tag that does
    /// not parse, and `native`, are the root locale `und`, whose week
    /// begins on the world's Monday. The locale argument fails as
    /// `hc_parse_iso_date` does; the answer is an `i64`, as every export
    /// that can return a sentinel is.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_first_day_of_week(locale: *const u8, locale_len: usize) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        match unsafe { text(locale, locale_len) } {
            Ok(tag) => i64::from(lines::first_day_of_week(tag)),
            Err(sentinel) => sentinel,
        }
    }

    /// The steps by which a country adopted the Gregorian calendar, as
    /// UTF-8 lines, returning the byte length written.
    ///
    /// `region` is an ISO 3166-1 alpha-2 code, in either case. One line per
    /// step, oldest first, tab-separated: the last day of the old reckoning
    /// and the first day of the new as fixed days, the old calendar's
    /// registry identifier (`julian`, `japanese-tenpo`, `dangi`, `chinese`,
    /// `islamic-umalqura`, `rumi`, `swedish-1700`), the scope (`civil` for
    /// the civil calendar of the whole polity as it then was, `partial` for
    /// part of the country, some purposes or part of the calendar, and
    /// `ecclesiastical` for a church's calendar alone), the instrument
    /// behind the step with its date and whether it was read, the new
    /// calendar's identifier (`gregory`, except for Sweden's steps of 1700
    /// to `swedish-1700` and of 1712 back to `julian`), and who took the
    /// step, in English. A staged adoption is several lines — China in 1912
    /// and 1929, Sweden in 1700, 1712 and 1753, the Dutch provinces — and a
    /// code the table does not know writes nothing, which is not a claim
    /// that the country never adopted the calendar. The region argument
    /// fails as `hc_parse_iso_date` does. A null `buffer` returns the
    /// length the text needs.
    ///
    /// # Safety
    ///
    /// `region` must be readable for `region_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gregorian_adoption(
        region: *const u8,
        region_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let region = match unsafe { text(region, region_len) } {
            Ok(region) => region,
            Err(sentinel) => return sentinel,
        };
        let text = lines::gregorian_adoption(region);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// Which month and weekday names a locale writes for a calendar on a
    /// fixed day, where a government renamed them for a period, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// `calendar` is a registry identifier, as `hc_calendar_units` takes
    /// it; one the registry does not carry is `HC_ERR_UNKNOWN`. `locale` is a BCP 47 tag, read
    /// as `hc_describe_day` reads it; `native` names no one language and
    /// takes no period. Tab-separated: `in-force` when a period's names
    /// were in force on the day, `undecided` when a period applies and no
    /// source read says whether it was yet in force, or `ordinary` when
    /// none applies and the locale's own names hold; then, for a period,
    /// its identifier, its name for the day's month and for the day's
    /// weekday, the English meaning of that weekday name, the first day the
    /// names can have been in force, the first day by which every source
    /// read has them in force and the first day the old names were back, as
    /// fixed days, and its sources. For `ordinary` the other eight cells are
    /// empty. The one period carried is Turkmenistan's, `turkmen-2002`, for
    /// the Gregorian calendar in Turkmen. The text arguments fail as
    /// `hc_parse_iso_date`'s does. A null `buffer` returns the length the
    /// text needs.
    ///
    /// # Safety
    ///
    /// `calendar` must be readable for `calendar_len` bytes and `locale`
    /// for `locale_len`, unless null with a zero length; `buffer` must be
    /// writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_naming_period_on(
        calendar: *const u8,
        calendar_len: usize,
        fixed: i64,
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let calendar = match unsafe { text(calendar, calendar_len) } {
            Ok(calendar) => calendar,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let answer = lines::naming_period_line(&hc::registry(), tag, calendar, fixed);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }
}

#[cfg(feature = "calendars")]
pub use calendars::{
    hc_calendar_list, hc_calendar_units, hc_calendars, hc_describe_day, hc_first_day_of_week,
    hc_gregorian_adoption, hc_locales, hc_naming_period_on,
};

/// Days in the calendars, behind the `calendars` feature: the pañcāṅga's
/// yoga and karaṇa, the Hindu lunisolar date at a place, the *Sūrya
/// Siddhānta*'s sky and sunrise, the young crescent by a named criterion,
/// the modern Olympiad of a year, the Hebrew anniversaries and sabbatical
/// cycle, and the days of the Asian calendar as it writes them. The lines
/// and numbers are `hyper_calendar`'s `panchanga_lines`, `hindu_lines`,
/// `crescent_lines` and `calendar_values`, shared with the C library.
#[cfg(feature = "calendars")]
mod calendar_days {
    use super::{emit_answer, sentinel, text, value};
    use hc::{astro_lines, calendar_values, crescent_lines, hindu_lines, panchanga_lines};

    /// The yoga and the karaṇa in progress at a POSIX timestamp, as two
    /// UTF-8 lines, returning the byte length written.
    ///
    /// The yoga's line first, then the karaṇa's, tab-separated alike: the
    /// limb (`yoga` or `karana`), its number (the yoga 1 for Viṣkambha
    /// through 27, the karaṇa the half-tithi 1 through 60), its name as
    /// Drik Panchang spells it in English and in Devanagari, the instants
    /// it began and ends and the instant it was read at as whole POSIX
    /// seconds, rounded down, in Universal Time, and the ayanamsa the yoga
    /// was reckoned with (empty for the karaṇa, which needs none).
    /// `ayanamsa` is `Lahiri (Chitrapaksha)`, `Raman`, `Krishnamurti` or
    /// `Fagan-Bradley`, or the first word of one, in any case; anything
    /// else, the empty string included, is `HC_ERR_UNKNOWN`. An instant
    /// outside the years −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null
    /// `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `ayanamsa` must be readable for `ayanamsa_len` bytes unless null with
    /// a zero length; `buffer` must be writable for `capacity` bytes unless
    /// it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_panchanga_at(
        unix_seconds: i64,
        ayanamsa: *const u8,
        ayanamsa_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let ayanamsa = match unsafe { text(ayanamsa, ayanamsa_len) } {
            Ok(ayanamsa) => ayanamsa,
            Err(sentinel) => return sentinel,
        };
        let answer = panchanga_lines::panchanga_at_lines(unix_seconds, ayanamsa);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The yoga and the karaṇa a fixed day carries at a place, the ones in
    /// progress at its sunrise, as two UTF-8 lines, returning the byte
    /// length written.
    ///
    /// The lines of `hc_panchanga_at`, read at the day's sunrise at the
    /// latitude and longitude in degrees, north and east positive, and the
    /// elevation in metres. A day on which the Sun does not rise there is
    /// `HC_ERR_NO_DATA`: no other moment is put in its place. A place off
    /// the globe, or a day outside the years −1000 to 3000, is
    /// `HC_ERR_OUT_OF_RANGE`; `ayanamsa` is as for `hc_panchanga_at`. A null
    /// `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// As `hc_panchanga_at`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_panchanga_of_day(
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        ayanamsa: *const u8,
        ayanamsa_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let ayanamsa = match unsafe { text(ayanamsa, ayanamsa_len) } {
            Ok(ayanamsa) => ayanamsa,
            Err(sentinel) => return sentinel,
        };
        let place = match astro_lines::location(latitude, longitude, elevation) {
            Ok(place) => place,
            Err(refusal) => return sentinel(refusal),
        };
        let answer = panchanga_lines::panchanga_of_day_lines(fixed, place, ayanamsa);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The Hindu lunisolar date of a fixed day at a place, as one UTF-8
    /// line, returning the byte length written.
    ///
    /// The amānta date read at the place's sunrise, tab-separated: the Śaka
    /// year, the Vikrama year, the month (1 for Chaitra through 12 for
    /// Phālguna), 1 if it is the intercalary month and 0 if not, the tithi
    /// (1 through 30), 1 if the day is the second to carry it and 0 if not,
    /// and the sunrise it was read at as whole POSIX seconds of Universal
    /// Time, rounded down; then, in the locale, the month's name (`Bhadra`,
    /// भाद्रपद under `hi`), with the locale's word for an intercalary month
    /// before it where it is one (`Adhika Sravana`); that word alone for an
    /// intercalary month, else empty; the Śaka era's name; the Vikrama
    /// Saṃvat's; and the tag of the data that answered. The names are
    /// `hc_describe_day`'s for `hindu-lunar` and resolve the locale as it
    /// does, `native` asking for Sanskrit; a name the locale's data does
    /// not have, such as either era in Sanskrit, is an empty cell. The
    /// locale argument fails as `hc_parse_iso_date` does. `sky` is an
    /// ayanamsa `hc_panchanga_at` names, for
    /// the true Sun and Moon in its zodiac, as `hindu-lunar` reads them with
    /// Lahiri's at the Central Station; or `surya-siddhanta`, for the
    /// *Sūrya Siddhānta*'s Sun and Moon at its own sunrise, as
    /// `hindu-lunar-surya-siddhanta` reads them at Ujjain; in any case, and
    /// anything else, the empty string included, is `HC_ERR_UNKNOWN`. The
    /// place is the latitude and longitude in degrees, north and east
    /// positive, and the elevation in metres. A place beyond 65° of
    /// latitude, where some day of the year has no sunrise, is
    /// `HC_ERR_OUT_OF_RANGE` on either sky, as is a day outside Śaka 1622
    /// through 2221 on the true sky, Chaitra śukla 1 in March 1700 to the
    /// eve of the one in March 2300, and outside Kali Yuga 1 to 10 000 on
    /// the Siddhānta's. A place off the globe is
    /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `sky` must be readable for `sky_len` bytes and `locale` for
    /// `locale_len`, unless null with a zero length; `buffer` must be
    /// writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_hindu_lunar_date(
        sky: *const u8,
        sky_len: usize,
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let (sky, tag) = match unsafe { (text(sky, sky_len), text(locale, locale_len)) } {
            (Ok(sky), Ok(tag)) => (sky, tag),
            (Err(sentinel), _) | (_, Err(sentinel)) => return sentinel,
        };
        let place = match astro_lines::location(latitude, longitude, elevation) {
            Ok(place) => place,
            Err(refusal) => return sentinel(refusal),
        };
        let answer = hindu_lines::hindu_lunar_date_line(sky, fixed, place, tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The *Sūrya Siddhānta*'s Sun and Moon at a POSIX timestamp, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// Tab-separated: the Sun's and the Moon's sidereal longitudes in
    /// degrees, the Moon's elongation from the Sun in degrees, 0 to 360,
    /// the tithi in progress (1 through 30) and the sign the Sun is in (1
    /// for Meṣa through 12 for Mīna). The instant is read as Universal
    /// Time. An instant outside the days of Kali Yuga 1 to 10 000, 3101 BCE
    /// to 6900 CE, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
    /// length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_surya_siddhanta_at(
        unix_seconds: i64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = hindu_lines::surya_siddhanta_line(unix_seconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The *Sūrya Siddhānta*'s sunrise on a fixed day at a place, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// The one cell is the instant as whole POSIX seconds of Universal
    /// Time, rounded down. The Siddhānta reads the latitude and the
    /// longitude, in degrees, north and east positive, and no height. A
    /// day outside Kali Yuga 1 to 10 000, a place beyond 65° of latitude,
    /// or one off the globe, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
    /// returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_surya_siddhanta_sunrise(
        fixed: i64,
        latitude: f64,
        longitude: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = astro_lines::location(latitude, longitude, 0.0)
            .and_then(|place| hindu_lines::surya_siddhanta_sunrise_line(fixed, place));
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// Whether the young crescent should have been visible on the evening
    /// that begins a fixed day, from a place, by a named criterion, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// `criterion` is `shaukat`, `yallop`, `saudi-rule`, `odeh`,
    /// `istanbul-2016`, `khgt`, `mabims-2021-topocentric` or
    /// `mabims-2021-geocentric-elongation`, in any case;
    /// anything else is `HC_ERR_UNKNOWN`. The evening is the one before
    /// `fixed`, since the day begins at sunset. Tab-separated: 1 if the
    /// crescent passes and 0 if not; the moment the criterion judges the
    /// evening at, as whole POSIX seconds of Universal Time, rounded down;
    /// and at that moment the Moon's longitude less the Sun's, 0 to 360,
    /// its arc of light, its geocentric altitude and the arc of vision, in
    /// degrees, and the
    /// crescent's topocentric width in arcminutes. Where there is no such
    /// moment the first cell is 0 and the rest are empty. The place is as
    /// for `hc_hindu_lunar_date`. A place off the globe, or a day outside
    /// the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
    /// returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `criterion` must be readable for `criterion_len` bytes unless null
    /// with a zero length; `buffer` must be writable for `capacity` bytes
    /// unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_crescent_visible(
        criterion: *const u8,
        criterion_len: usize,
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let criterion = match unsafe { text(criterion, criterion_len) } {
            Ok(criterion) => criterion,
            Err(sentinel) => return sentinel,
        };
        let place = match astro_lines::location(latitude, longitude, elevation) {
            Ok(place) => place,
            Err(refusal) => return sentinel(refusal),
        };
        let answer = crescent_lines::crescent_line(criterion, fixed, place);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The number of the modern Olympiad a Gregorian year belongs to, or an
    /// error sentinel.
    ///
    /// 1 for 1896–1899, under the Olympic Charter's definition, whether or
    /// not its Games were held; a year before 1896 is
    /// `HC_ERR_OUT_OF_RANGE`.
    #[unsafe(no_mangle)]
    pub extern "C" fn hc_ioc_olympiad(gregorian_year: i64) -> i64 {
        value(calendar_values::ioc_olympiad(gregorian_year))
    }

    /// The fixed day of the yahrzeit in a Hebrew year of a death on the
    /// Hebrew date a fixed day names, or an error sentinel.
    ///
    /// `death_fixed` is the fixed day whose daylight carries the Hebrew
    /// date of the death — a death after sunset is the next fixed day — and
    /// the rules for the dates a later year may lack (30 Ḥeshvan, 30 Kislev,
    /// Adar) are Reingold and Dershowitz's. A day or a year outside the
    /// Hebrew years 1 to 9999 is `HC_ERR_OUT_OF_RANGE`.
    #[unsafe(no_mangle)]
    pub extern "C" fn hc_hebrew_yahrzeit(death_fixed: i64, hebrew_year: i64) -> i64 {
        value(calendar_values::hebrew_yahrzeit(death_fixed, hebrew_year))
    }

    /// The fixed day of the birthday in a Hebrew year of a birth on the
    /// Hebrew date a fixed day names, or an error sentinel.
    ///
    /// As `hc_hebrew_yahrzeit`, by Reingold and Dershowitz's
    /// `hebrew-birthday`: a birth in the last month of a year is kept in the
    /// last month of the later one.
    #[unsafe(no_mangle)]
    pub extern "C" fn hc_hebrew_birthday(birth_fixed: i64, hebrew_year: i64) -> i64 {
        value(calendar_values::hebrew_birthday(birth_fixed, hebrew_year))
    }

    /// A person's age as the Chinese count reckons it on a fixed day, or an
    /// error sentinel.
    ///
    /// One at birth and one more at each Chinese New Year after, whatever
    /// the day of birth, as Reingold and Dershowitz's `chinese-age` counts
    /// it and Wikipedia gives the pre-modern *suì* of China; nothing here
    /// says how any other country counts. The birth crosses as its fixed
    /// day. A day before the birth has no age and is `HC_ERR_NO_DATA`; a
    /// day outside the Chinese calendar's range is `HC_ERR_OUT_OF_RANGE`.
    #[unsafe(no_mangle)]
    pub extern "C" fn hc_chinese_reckoned_age(birth_fixed: i64, on_fixed: i64) -> i64 {
        value(calendar_values::chinese_reckoned_age(birth_fixed, on_fixed).map(i64::from))
    }

    /// The marriage augury of a Chinese year, as one UTF-8 line, returning
    /// the byte length written.
    ///
    /// Tab-separated: the augury by the published code's names — `widow`
    /// for a year without 立春, `blind` for one only near its end, `bright`
    /// for one only near its start, `double-bright` for both — then `1` or
    /// `0` for whether 立春 falls after the year's New Year and whether
    /// another falls before the next. `chinese_year` is the year as the
    /// Chinese calendar counts it, 4661 for the one that began on
    /// 10 February 2024. A year that begins, or whose next begins, outside
    /// the calendar's range is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
    /// returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_chinese_marriage_augury(
        chinese_year: i64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = calendar_values::chinese_marriage_augury_line(chinese_year);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The place of a Hebrew year in the seven-year sabbatical cycle, 1
    /// through 7, or an error sentinel.
    ///
    /// 7 is the sabbatical year, *shemittah*, counted from Rosh Hashanah as
    /// the years published today count it: 5782 (2021–22) and 5789
    /// (2028–29) are sabbatical years. A year outside the Hebrew years 1 to
    /// 9999 is `HC_ERR_OUT_OF_RANGE`.
    #[unsafe(no_mangle)]
    pub extern "C" fn hc_hebrew_sabbatical_cycle_year(hebrew_year: i64) -> i64 {
        value(calendar_values::hebrew_sabbatical_cycle_year(hebrew_year))
    }

    /// A fixed day in the calendar of the Roman province of Asia as the
    /// calendar writes it, unnumbered days included, as one UTF-8 line,
    /// returning the byte length written.
    ///
    /// Tab-separated: the Julian year, AD, in which the Asian year began;
    /// the month, 1 for Kaisar through 12 for Hyperberetaios; the month's
    /// name; `unnumbered` for a day before day 1 — Sebaste, which opens a
    /// 31-day month, and in a leap Xandikos Sebaste and the intercalary
    /// day, whose order the sources read do not settle — or `numbered`; and
    /// the day's number, 1 to 30, or for an unnumbered day its place among
    /// them, 1 or 2. A day outside 23 September AD 4 to the end of the
    /// Asian year 9999 is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns
    /// the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_asian_day(fixed: i64, buffer: *mut u8, capacity: usize) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(calendar_values::asian_day_line(fixed), buffer, capacity) }
    }
}

#[cfg(feature = "calendars")]
pub use calendar_days::{
    hc_asian_day, hc_chinese_marriage_augury, hc_chinese_reckoned_age, hc_crescent_visible,
    hc_hebrew_birthday, hc_hebrew_sabbatical_cycle_year, hc_hebrew_yahrzeit, hc_hindu_lunar_date,
    hc_ioc_olympiad, hc_panchanga_at, hc_panchanga_of_day, hc_surya_siddhanta_at,
    hc_surya_siddhanta_sunrise,
};

/// The holiday tables, behind the `holiday` feature: every country,
/// exchange, tradition and international set of `hc-holiday`, looked up by
/// identifier and rendered as tab-separated lines.
#[cfg(feature = "holiday")]
mod holiday {
    use super::{
        HC_ERR_BUFFER_TOO_SMALL, HC_ERR_OUT_OF_RANGE, HC_ERR_UNKNOWN, emit, emit_or_measure,
        push_cell, text,
    };
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

    /// Every table, in the order [`hc_holiday_codes`] lists them: the
    /// countries, then the exchanges, the traditions and the international
    /// sets.
    pub(super) fn tables() -> impl Iterator<Item = &'static RuleSet> {
        hc::holiday_lines::tables()
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
        for set in tables() {
            out.push_str(set.code);
            out.push('\n');
        }
        out
    }

    /// The word a [`Kind`] is written as.
    const fn kind_name(kind: Kind) -> &'static str {
        match kind {
            Kind::Public => "public",
            Kind::Bank => "bank",
            Kind::Religious => "religious",
            Kind::Observance => "observance",
            Kind::School => "school",
            Kind::Workday => "workday",
        }
    }

    /// The word a [`Confidence`] is written as.
    const fn confidence_name(confidence: Confidence) -> &'static str {
        match confidence {
            Confidence::Exact => "exact",
            Confidence::Approximate => "approximate",
        }
    }

    /// The holidays of a year as lines: the ISO date, the name, the local
    /// name, the kind, the confidence, `1` for a substitute day and the date
    /// it stands in for, tab-separated.
    pub(super) fn lines(table: &RuleSet, region: Option<&str>, year: i64) -> String {
        use core::fmt::Write;
        let calendar = HolidayCalendar::for_year(table, region, year);
        let mut out = String::new();
        for holiday in calendar.all() {
            let _ = write!(
                out,
                "{}\t{}\t{}\t{}\t{}\t{}\t",
                iso(holiday.date),
                holiday.name,
                holiday.local_name,
                kind_name(holiday.kind),
                confidence_name(holiday.confidence),
                u8::from(holiday.is_substitute())
            );
            if let Some(day) = holiday.observed_for {
                out.push_str(&iso(day));
            }
            out.push('\n');
        }
        out
    }

    /// Every entry on one day across every table, one line each; see
    /// [`hc_holidays_on`] for the columns.
    ///
    /// # Errors
    ///
    /// [`HC_ERR_OUT_OF_RANGE`] for a day with no Gregorian year.
    pub(super) fn lines_on(fixed: i64) -> Result<String, i64> {
        let day = Rd(fixed);
        gregorian::year_from_fixed(day).map_err(|_| HC_ERR_OUT_OF_RANGE)?;
        let mut out = String::new();
        // One memo for every table: the astronomy the tables share — the
        // same tithis, the same new moons — is done once.
        let mut context = hc::hc_holiday::EvaluationContext::new();
        for table in tables() {
            let calendar = HolidayCalendar::for_day_with(table, None, day, &mut context);
            push_lines_on(&mut out, table, &calendar, day);
        }
        Ok(out)
    }

    /// One table's lines for one day, as [`lines_on`] writes them: the
    /// entries `calendar` has on the day, then the gaps it reports.
    pub(super) fn push_lines_on(
        out: &mut String,
        table: &RuleSet,
        calendar: &HolidayCalendar<'_>,
        day: Rd,
    ) {
        use core::fmt::Write;
        for holiday in calendar.on(day) {
            push_cell(out, table.code);
            out.push('\t');
            push_cell(out, table.english_name);
            out.push('\t');
            push_cell(out, holiday.name);
            out.push('\t');
            push_cell(out, holiday.local_name);
            let _ = write!(
                out,
                "\t{}\t{}\t",
                kind_name(holiday.kind),
                confidence_name(holiday.confidence)
            );
            push_cell(out, holiday.source);
            let _ = write!(out, "\t{}\t", u8::from(holiday.is_substitute()));
            if let Some(observed_for) = holiday.observed_for {
                let _ = write!(out, "{}", observed_for.0);
            }
            out.push('\n');
        }
        // A gap is a holiday the table could not place this year — its
        // calendar's range ended, or no announcement was read — and it is
        // reported rather than left out, so that a page can say so.
        for gap in calendar.gaps() {
            push_cell(out, table.code);
            out.push('\t');
            push_cell(out, table.english_name);
            out.push('\t');
            push_cell(out, gap.name);
            out.push('\t');
            push_cell(out, gap.local_name);
            out.push_str("\tgap\t\t\t0\t\n");
        }
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
        if gregorian::year_from_fixed(day).is_err() {
            return HC_ERR_OUT_OF_RANGE;
        }
        i64::from(HolidayCalendar::for_day(table, region, day).is_holiday(day))
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

    /// Every holiday on one fixed day across every table, as UTF-8 lines,
    /// returning the byte length written.
    ///
    /// The tables are the ones `hc_holiday_codes` lists, in that order, each
    /// evaluated nationwide. One line per (table, entry), tab-separated: the
    /// table's identifier, its English name, the holiday's English name, its
    /// local name, the kind (`public`, `bank`, `religious`, `observance`,
    /// `school`, `workday`, or `gap`), the confidence (`exact` or
    /// `approximate`), the instrument the rule cites or nothing, `1` for a
    /// substitute day and `0` otherwise, and the fixed day a substitute
    /// stands in for or nothing. A `gap` line is a holiday the table could
    /// not place in the day's year — its calendar's range ended, or no
    /// announcement was read — with the confidence and source empty, so a
    /// page can say the year is unanswered rather than show nothing. A day
    /// with no Gregorian year is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
    /// returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holidays_on(fixed: i64, buffer: *mut u8, capacity: usize) -> i64 {
        let text = match lines_on(fixed) {
            Ok(text) => text,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }
}

#[cfg(feature = "holiday")]
pub use holiday::{hc_holiday_codes, hc_holiday_is_day_off, hc_holidays_in_year, hc_holidays_on};

/// The tables and the liturgical year, behind the `holiday` feature: every
/// holiday table described, the lectionary cycles of a day, the
/// astronomical Easter, the Holy Year a day falls in and the ranks of the
/// *Common Worship* celebrations of a day. The lines are
/// `hyper_calendar::holiday_lines`', shared with the C library.
#[cfg(feature = "holiday")]
mod observances {
    use super::{emit_answer, emit_or_measure, text, value};
    use hc::holiday_lines;

    /// Every holiday table with its kind, names and sources, as UTF-8
    /// lines, returning the byte length written.
    ///
    /// One line per table, in the order `hc_holiday_codes` lists them,
    /// tab-separated: the code; the kind (`country`, `subdivision`,
    /// `exchange`, `tradition` or `observance`), read from the list the
    /// table is in; the name in the locale; the English name; the locale
    /// that answered; the sources the table names; and, for a subdivision
    /// or an exchange, the ISO 3166-1 country it belongs to as its table
    /// records it, else empty; and the short name in the locale, else
    /// empty. A country is named by its CLDR 48 territory
    /// name in the locale where `hc-i18n` carries one, a country the locale
    /// has no name for by CLDR's English one, and every other table by its
    /// English name; column 5 is the tag that answered. Column 8 is CLDR
    /// 48's `alt="short"` name of a country, from the data that named it —
    /// `Hong Kong` for `HK` under `en`, 香港 under `ja` — and empty where
    /// the data has none and for every table that is not a country. The locale argument fails as `hc_parse_iso_date`
    /// does. A null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless
    /// it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holiday_tables(
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let text = holiday_lines::holiday_tables(tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// The lectionary cycles of a fixed day, as one UTF-8 line, returning
    /// the byte length written.
    ///
    /// Tab-separated: the liturgical year, named by the civil year of its
    /// Easter and begun on the First Sunday of Advent before it; the Sunday
    /// cycle of the Roman Lectionary and the Revised Common Lectionary, `A`,
    /// `B` or `C`; the Roman weekday cycle, `I` or `II`; and the RCL's
    /// numbered Proper, 3 to 29, for a Sunday after Trinity Sunday, else
    /// empty. A day outside the liturgical years 1583 to 4099 is
    /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_lectionary(fixed: i64, buffer: *mut u8, capacity: usize) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(holiday_lines::lectionary_line(fixed), buffer, capacity) }
    }

    /// The fixed day of Easter Sunday of a Gregorian year by the
    /// astronomical reckoning at the meridian of Jerusalem, or an error
    /// sentinel.
    ///
    /// The first Sunday after the day, by apparent solar time at
    /// Jerusalem, of the first full moon at or after the March equinox, as
    /// the World Council of Churches' Aleppo statement of 1997 proposed. A
    /// year outside 1583 to 2150 is `HC_ERR_OUT_OF_RANGE`.
    #[unsafe(no_mangle)]
    pub extern "C" fn hc_astronomical_easter(year: i64) -> i64 {
        value(holiday_lines::astronomical_easter(year))
    }

    /// The fixed day of the paschal full moon of a Gregorian year by the
    /// astronomical reckoning at the meridian of Jerusalem, or an error
    /// sentinel.
    ///
    /// The day, by apparent solar time at Jerusalem, of the first full moon
    /// at or after the March equinox: the day `hc_astronomical_easter` is
    /// the first Sunday after, so a full moon on a Sunday puts Easter a
    /// week later. A year outside 1583 to 2150 is `HC_ERR_OUT_OF_RANGE`.
    #[unsafe(no_mangle)]
    pub extern "C" fn hc_astronomical_paschal_full_moon(year: i64) -> i64 {
        value(holiday_lines::astronomical_paschal_full_moon(year))
    }

    /// The Holy Year of the Catholic Church a fixed day falls in, if any,
    /// as one UTF-8 line, returning the byte length written.
    ///
    /// Tab-separated: `within` when the day falls in a jubilee in Rome,
    /// from the opening of the Holy Door of St Peter's to its closing, else
    /// `outside`; then, within one, the jubilee's title, `ordinary` or
    /// `extraordinary`, the Pope who proclaimed it, the bull of indiction
    /// by its opening words, the day the bull was given and the jubilee's
    /// first and last days in Rome, as fixed days, and its first and last
    /// days in the dioceses where the bull dates them, else empty. Outside
    /// one the nine cells after the first are empty. The table holds the
    /// jubilees from 1975 to 2025; a day before 24 December 1974 or after
    /// 27 September 2026, the day its sources were checked, is
    /// `HC_ERR_NO_DATA`. A null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holy_year_on(fixed: i64, buffer: *mut u8, capacity: usize) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(holiday_lines::holy_year_line(fixed), buffer, capacity) }
    }

    /// The rank of every *Common Worship* celebration kept on a fixed day,
    /// as UTF-8 lines, returning the byte length written.
    ///
    /// One line per Principal Feast, Principal Holy Day or Festival of the
    /// Church of England's calendar kept on the day after the transfers its
    /// Rules require, tab-separated: the title as the Rules print it, which
    /// is the name `hc_holidays_on` gives it in the `common-worship` table;
    /// the rank, `principal-feast`, `principal-holy-day` or `festival`; and
    /// the rank's English name. A day that keeps none writes nothing. The
    /// years the Rules leave a Festival without a day are `gap` lines of
    /// `hc_holidays_on`. A day with no Gregorian year is
    /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_common_worship_on(
        fixed: i64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(holiday_lines::common_worship_lines(fixed), buffer, capacity) }
    }
}

#[cfg(feature = "holiday")]
pub use observances::{
    hc_astronomical_easter, hc_astronomical_paschal_full_moon, hc_common_worship_on,
    hc_holiday_tables, hc_holy_year_on, hc_lectionary,
};

/// The almanac, behind the `seasons` feature: the 24 solar terms and the 72
/// pentads of `hc-seasons`, judged at a named meridian, and 寒食 under each
/// of its reckonings. The lines are `hyper_calendar::season_lines`', shared
/// with the C library.
#[cfg(feature = "seasons")]
mod seasons {
    use super::{emit_answer, text, value};
    use hc::season_lines;

    /// The solar term in effect on a fixed day at a meridian, as one UTF-8
    /// line, returning the byte length written.
    ///
    /// Tab-separated: the term's index from 春分 at 0 through 驚蟄 at 23
    /// (the longitude divided by 15°), its name in traditional Chinese, its
    /// name in Japanese, the fixed day the term began at that meridian, the
    /// last fixed day before the next term begins, the authority for the
    /// Chinese names and the authority for the Japanese names. `meridian`
    /// is a name — `universal`, `japan`, `china`, `korea`, `india` or
    /// `china-before-1929`, in any case, or empty for `universal` — or a
    /// longitude in decimal degrees east of Greenwich, read as local mean
    /// solar time; anything else is `HC_ERR_UNKNOWN`, and the pointer and
    /// bytes fail as for `hc_parse_iso_date`. A day outside the years −1000
    /// to 3000, the era `hc_sky_at` answers for, is `HC_ERR_OUT_OF_RANGE`.
    /// A null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `meridian` must be readable for `meridian_len` bytes unless null with
    /// a zero length; `buffer` must be writable for `capacity` bytes unless
    /// it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_term_in_effect(
        fixed: i64,
        meridian: *const u8,
        meridian_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { text(meridian, meridian_len) } {
            Ok(name) => name,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(season_lines::term_line(fixed, name), buffer, capacity) }
    }

    /// The pentad (候) in effect on a fixed day at a meridian, as one UTF-8
    /// line, returning the byte length written.
    ///
    /// Tab-separated: the pentad's index from the first pentad of 春分 at 0
    /// through 71 (the longitude divided by 5°), its name in the Chinese
    /// tradition, its name in the Japanese tradition, the fixed day the
    /// pentad began at that meridian, the last fixed day before the next
    /// pentad begins, the text the Chinese names come from and the text the
    /// Japanese names come from. `meridian` and the day are as for
    /// `hc_term_in_effect`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// As `hc_term_in_effect`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_pentad_in_effect(
        fixed: i64,
        meridian: *const u8,
        meridian_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { text(meridian, meridian_len) } {
            Ok(name) => name,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(season_lines::pentad_line(fixed, name), buffer, capacity) }
    }

    /// The fixed day of 寒食, the Cold Food Day, of a Gregorian year under
    /// a named reckoning, or an error sentinel.
    ///
    /// `convention` is `hanshi-solstice-105`, 105 days after the winter
    /// solstice at the Chinese meridian, the reckoning before 1645;
    /// `hanshi-eve-of-qingming`, the day before 清明 at the Chinese
    /// meridian, as kept after the 時憲曆 of 1645; or `hansik`, Korea's
    /// 한식, 105 days after 동지 at the Korean meridian; in any case. Any
    /// other name is `HC_ERR_UNKNOWN`, and the pointer and bytes fail as
    /// for `hc_parse_iso_date`. The solstice reckonings count from the
    /// solstice of the year before, so a year outside −999 to 3000 is
    /// `HC_ERR_OUT_OF_RANGE` under every reckoning.
    ///
    /// # Safety
    ///
    /// `convention` must be readable for `convention_len` bytes unless null
    /// with a zero length.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_cold_food_day(
        convention: *const u8,
        convention_len: usize,
        year: i64,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let id = match unsafe { text(convention, convention_len) } {
            Ok(id) => id,
            Err(sentinel) => return sentinel,
        };
        value(season_lines::cold_food_day(id, year))
    }
}

#[cfg(feature = "seasons")]
pub use seasons::{hc_cold_food_day, hc_pentad_in_effect, hc_term_in_effect};

/// Deep time, behind the `deep-time` feature: the cosmic, geologic and
/// archaeological chronologies of `hc-deep-time`, and a moment placed in
/// all of them at once.
#[cfg(feature = "deep-time")]
mod deep_time {
    use super::{HC_ERR_OUT_OF_RANGE, HC_ERR_UNKNOWN, emit_or_measure, text};
    use hc::deep_time_lines;

    /// A moment some years before the present, placed in every chronology
    /// at once, as UTF-8 lines, returning the byte length written.
    ///
    /// One line per entry, tab-separated, every deep-time export alike: the
    /// kind (`moment`, `cosmic-epoch`, `cosmic-event`,
    /// `earliest-evidence`, `future-era`, a geologic rank `eon`, `era`,
    /// `period`, `epoch` or `age`, or `archaeological`), the entry's stable
    /// lower-case kebab identifier, the name, the scope (the identifier of
    /// the interval one rank up for a geologic interval, the region for an
    /// archaeological period, the landmark for an earliest-evidence claim),
    /// the older bound's value, standard uncertainty, significant figures
    /// and `1` where the table marks it approximate, the same four for the
    /// younger bound, the unit the values are in, the description, the
    /// source, and the name in the locale. A point in time has the same
    /// start and end; a minimum age has no start, and a standard
    /// uncertainty the source does not state is empty. The units are what
    /// each table counts in: `seconds-since-big-bang` for the cosmic rows,
    /// `megayears-before-present` for the geologic chart's, the
    /// `years-before-1950` of the BP convention for the archaeological and
    /// earliest-evidence rows, `log10-years-from-now` for a future era. The lines are the
    /// moment itself as `since-big-bang` and `before-present`, its cosmic
    /// epoch and the last dated cosmic event before it, its future era if
    /// it lies ahead, its geologic chain from eon down to age, and its
    /// archaeological period, each present only where that chronology
    /// reaches. `years_ago` counts back from the present as the crate
    /// defines it — the Planck 2018 age of the universe — not from 1950 and
    /// not from the caller's clock; the crate ignores the difference
    /// between the three, which lies below the smallest uncertainty in any
    /// of its tables. Negative years are the future: a moment up to a
    /// century ahead is still in the present intervals — the chart's
    /// youngest chain, the Modern period, the last cosmic epoch — as well
    /// as in its future era, and beyond a century it is in its future era
    /// alone. `locale` is a BCP 47 tag; the last column is the geologic
    /// chart's own name for an interval in that language, from the ICS's
    /// translations, or for a cosmic, archaeological or earliest-evidence
    /// row the established term `hc_deep_time::names` carries for it, and
    /// empty where neither has one. A value the crate refuses — not finite,
    /// beyond its range — is `HC_ERR_OUT_OF_RANGE`; the locale argument
    /// fails as `hc_parse_iso_date` does. A null `buffer` returns the
    /// length the text needs.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_place_years_ago(
        years_ago: f64,
        std_dev_years: f64,
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let Ok(text) = deep_time_lines::placement(years_ago, std_dev_years, tag) else {
            return HC_ERR_OUT_OF_RANGE;
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// Every cosmic epoch and every dated cosmic event, as UTF-8 lines,
    /// returning the byte length written.
    ///
    /// The epochs first, Big Bang to the present, then the events, oldest
    /// first, each a line of the columns `hc_place_years_ago` writes, in
    /// `seconds-since-big-bang`. `locale` is as for `hc_place_years_ago`,
    /// and names an entry where an established term is carried. A null
    /// `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_cosmic_events(
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let text = deep_time_lines::cosmic(tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// Every claim to the earliest evidence of life, of *Homo sapiens* and of
    /// writing, as UTF-8 lines, returning the byte length written.
    ///
    /// The claims to the earliest evidence of life, of *Homo sapiens* and of
    /// writing, grouped by landmark and oldest first, each a line of the
    /// columns `hc_place_years_ago` writes, in `years-before-1950`, with the
    /// landmark as the scope. Each keeps the shape of its source's date: an
    /// age has the same start and end, a minimum age no start, a range two
    /// different bounds; a standard uncertainty the source does not state
    /// is an empty cell, and a disputed claim names the rebuttal in its
    /// description. `locale` is as for `hc_place_years_ago`, and names a
    /// claim where an established term is carried.
    /// A null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_earliest_evidence(
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let text = deep_time_lines::earliest_evidence(tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// Every conventional archaeological period, as UTF-8 lines, returning
    /// the byte length written.
    ///
    /// The conventional Southwest Asian and European sequence, youngest
    /// first, each a line of the columns `hc_place_years_ago` writes, in
    /// `years-before-1950`, with the region as the scope. `locale` is as for
    /// `hc_place_years_ago`, and names a period where an established term is
    /// carried.
    /// A null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_archaeological_periods(
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let text = deep_time_lines::archaeological_periods(tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// Every dated event of the far future, as UTF-8 lines, returning the
    /// byte length written.
    ///
    /// The dated events of the far future, soonest first, each a line of
    /// the columns `hc_place_years_ago` writes, in `years-from-now`, with
    /// the kind of prediction as the scope: `modelled`,
    /// `experimental-bound` or `order-of-magnitude`. An experimental bound
    /// has a start and no end, because the event, if it happens, is no
    /// sooner. `locale` is as for `hc_place_years_ago`; no future event has
    /// a name in another language, so the last column is empty.
    /// A null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_future_events(
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let text = deep_time_lines::future_events(tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// Every interval of one rank of the geologic time scale, as UTF-8
    /// lines, returning the byte length written.
    ///
    /// `rank` is 0 for the eons, 1 for the eras, 2 for the periods, 3 for
    /// the epochs and 4 for the ages; anything else is `HC_ERR_UNKNOWN`.
    /// The intervals come youngest first, each a line of the columns
    /// `hc_place_years_ago` writes, in `megayears-before-present` with the
    /// chart's own figures and uncertainties, the chart as the source and
    /// the chart's name in `locale` last. A null `buffer` returns the
    /// length the text needs.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_geologic_intervals(
        rank: u32,
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let Some(rank) = deep_time_lines::rank(rank) else {
            return HC_ERR_UNKNOWN;
        };
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        let text = deep_time_lines::intervals(rank, tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }
}

#[cfg(feature = "deep-time")]
pub use deep_time::{
    hc_archaeological_periods, hc_cosmic_events, hc_earliest_evidence, hc_future_events,
    hc_geologic_intervals, hc_place_years_ago,
};

/// Time zones, behind the `tz` feature: the day an instant falls on, and
/// the instant a day begins, by the wall clock of an IANA zone.
#[cfg(feature = "tz")]
mod tz {
    use std::sync::{Mutex, PoisonError};

    use super::{HC_ERR_MALFORMED, HC_ERR_OUT_OF_RANGE, HC_ERR_UNKNOWN, above_floor, text};
    use hc::hc_calendar::fixed::RD_OF_UNIX_EPOCH;
    use hc::hc_calendar::{CivilDateTime, Rd};
    use hc::hc_core::UnixTime;
    use hc::hc_tz::{LocalResolution, TimeZone, TzifTimeZone, builtin};

    /// The zones a page has handed the module as TZif bytes, by name.
    ///
    /// Behind a mutex because a `static` has to be, not because the module
    /// is ever called from two threads: a WebAssembly instance is
    /// single-threaded. The bytes are kept and parsed again on every call,
    /// which is a few microseconds against the cost of owning a borrowed
    /// parse across the boundary.
    static LOADED: Mutex<Vec<(String, Vec<u8>)>> = Mutex::new(Vec::new());

    /// The zone a name selects — one loaded through [`hc_zone_load`] first,
    /// because a page that supplied the IANA data for a name wants that and
    /// not the built-in approximation, then the built-in table — handed to
    /// `answer`.
    ///
    /// # Errors
    ///
    /// [`HC_ERR_UNKNOWN`] for a name neither knows.
    fn with_zone<R>(name: &str, answer: impl FnOnce(&dyn TimeZone) -> R) -> Result<R, i64> {
        let loaded = LOADED.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some((known, bytes)) = loaded
            .iter()
            .find(|(known, _)| known.eq_ignore_ascii_case(name))
        {
            // Validated when it was loaded; a failure here is impossible,
            // and would be reported as unknown rather than trapped on.
            let zone = TzifTimeZone::parse(known, bytes).map_err(|_| HC_ERR_UNKNOWN)?;
            return Ok(answer(&zone));
        }
        drop(loaded);
        let zone = builtin::zone(name).map_err(|_| HC_ERR_UNKNOWN)?;
        Ok(answer(&zone))
    }

    /// The fixed day an instant falls on by a zone's wall clock.
    pub(super) fn day_in_zone(unix: i64, name: &str) -> Result<i64, i64> {
        with_zone(name, |zone| {
            zone.local_at(UnixTime::from_seconds(unix))
                .map(|local| local.day.0)
                .map_err(|_| HC_ERR_OUT_OF_RANGE)
        })?
    }

    /// The instant a day begins by a zone's wall clock: its midnight, or
    /// the first instant after a gap that swallows it, or the earlier of
    /// two midnights when the clocks fall back across it.
    ///
    /// # Errors
    ///
    /// [`HC_ERR_UNKNOWN`] for a name neither table knows, and
    /// [`HC_ERR_OUT_OF_RANGE`] for a day whose start would overflow an
    /// `i64` or would be at or below [`HC_ERR_FLOOR`].
    ///
    /// [`HC_ERR_FLOOR`]: super::HC_ERR_FLOOR
    pub(super) fn start_in_zone(fixed: i64, name: &str) -> Result<i64, i64> {
        with_zone(name, |zone| {
            let instant = match zone.resolve_local(CivilDateTime::midnight(Rd(fixed))) {
                LocalResolution::Unambiguous(instant) => instant,
                LocalResolution::Ambiguous { earlier, .. } => earlier,
                LocalResolution::Nonexistent { after_gap, .. } => after_gap,
            };
            above_floor(unsaturated(fixed, instant.seconds(), zone))
        })?
    }

    /// The start of a day as `resolve_local` gave it, or `None` when that
    /// was an `i64` bound standing in for an instant beyond it.
    ///
    /// `resolve_local` saturates rather than fails, so `i64::MIN` and
    /// `i64::MAX` are its answer for every day whose start is out of reach.
    /// Either is kept only when the day's midnight less the zone's offset
    /// there really is that instant.
    fn unsaturated(fixed: i64, seconds: i64, zone: &dyn TimeZone) -> Option<i64> {
        if seconds != i64::MIN && seconds != i64::MAX {
            return Some(seconds);
        }
        let midnight = (i128::from(fixed) - i128::from(RD_OF_UNIX_EPOCH)) * 86_400;
        let offset = zone.offset_at(UnixTime::from_seconds(seconds)).seconds();
        (midnight - i128::from(offset) == i128::from(seconds)).then_some(seconds)
    }

    /// Keep TZif bytes under a name, replacing any already there.
    ///
    /// # Errors
    ///
    /// [`HC_ERR_MALFORMED`] for bytes that are not a TZif file.
    pub(super) fn load(name: &str, bytes: &[u8]) -> Result<(), i64> {
        TzifTimeZone::parse(name, bytes).map_err(|_| HC_ERR_MALFORMED)?;
        let mut loaded = LOADED.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(slot) = loaded
            .iter_mut()
            .find(|(known, _)| known.eq_ignore_ascii_case(name))
        {
            slot.1 = bytes.to_vec();
        } else {
            loaded.push((name.to_owned(), bytes.to_vec()));
        }
        Ok(())
    }

    /// The fixed day a POSIX timestamp falls on by the wall clock of a
    /// zone, or an error sentinel.
    ///
    /// `zone` is an IANA name, `Asia/Tokyo`, in any case: one a page has
    /// loaded through `hc_zone_load`, or else one of the seventeen the
    /// module carries with their current rules. A name neither knows is
    /// `HC_ERR_UNKNOWN`, an instant whose local day leaves the range of a
    /// day number `HC_ERR_OUT_OF_RANGE`, and the name's pointer and bytes
    /// fail as for `hc_parse_iso_date`.
    ///
    /// # Safety
    ///
    /// `zone` must be readable for `zone_len` bytes unless null with a zero
    /// length.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_fixed_from_unix_in_zone(
        unix_seconds: i64,
        zone: *const u8,
        zone_len: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { text(zone, zone_len) } {
            Ok(name) => name,
            Err(sentinel) => return sentinel,
        };
        match day_in_zone(unix_seconds, name) {
            Ok(day) => day,
            Err(sentinel) => sentinel,
        }
    }

    /// The POSIX timestamp at which a fixed day begins by the wall clock of
    /// a zone, or an error sentinel.
    ///
    /// The day begins at its local midnight. When the clocks go forward
    /// across that midnight, so that it does not exist, the day begins at
    /// the first instant after the gap — Cairo's first day of summer time
    /// begins at 01:00 EEST; when they go back across it, at the earlier of
    /// the two midnights. `zone` is as for `hc_fixed_from_unix_in_zone`,
    /// and fails the same way.
    ///
    /// A day whose start would be at or below `HC_ERR_FLOOR` seconds or
    /// would overflow an `i64` is `HC_ERR_OUT_OF_RANGE`: by UTC, a day
    /// before fixed day −104 165 947 503 or after 106 751 991 886 463, and
    /// a zone's offset moves each end by at most a day.
    ///
    /// # Safety
    ///
    /// As `hc_fixed_from_unix_in_zone`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_unix_from_fixed_in_zone(
        fixed: i64,
        zone: *const u8,
        zone_len: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { text(zone, zone_len) } {
            Ok(name) => name,
            Err(sentinel) => return sentinel,
        };
        match start_in_zone(fixed, name) {
            Ok(instant) => instant,
            Err(sentinel) => sentinel,
        }
    }

    /// Give the module a zone's TZif data under an IANA name, returning 0.
    ///
    /// The built-in table carries seventeen zones and only their current
    /// rules; a page that wants another zone, or a zone's history, fetches
    /// the IANA file (`/usr/share/zoneinfo/Europe/Rome` on most systems)
    /// and hands its bytes here once, after which the two `_in_zone`
    /// exports answer for that name — a loaded zone takes precedence over a
    /// built-in one of the same name. The bytes are copied, so the caller
    /// may free them. Bytes that are not a TZif file are `HC_ERR_MALFORMED`
    /// and nothing is kept; the name's pointer and bytes fail as for
    /// `hc_parse_iso_date`.
    ///
    /// # Safety
    ///
    /// `name` must be readable for `name_len` bytes and `tzif` for
    /// `tzif_len`, unless null with a zero length.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_zone_load(
        name: *const u8,
        name_len: usize,
        tzif: *const u8,
        tzif_len: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { text(name, name_len) } {
            Ok(name) => name,
            Err(sentinel) => return sentinel,
        };
        if name.is_empty() {
            return HC_ERR_UNKNOWN;
        }
        let bytes: &[u8] = if tzif.is_null() {
            &[]
        } else {
            // SAFETY: the caller guarantees `tzif` is readable for `tzif_len`.
            unsafe { core::slice::from_raw_parts(tzif, tzif_len) }
        };
        match load(name, bytes) {
            Ok(()) => 0,
            Err(sentinel) => sentinel,
        }
    }
}

#[cfg(feature = "tz")]
pub use tz::{hc_fixed_from_unix_in_zone, hc_unix_from_fixed_in_zone, hc_zone_load};

/// Where each zone is, behind the `tz` feature: the principal location
/// the IANA database gives a zone, its countries and its CLDR exemplar
/// city. The lines are `hyper_calendar::zone_lines`', shared with the C
/// library.
#[cfg(feature = "tz")]
mod zones {
    use super::{emit_answer, emit_or_measure, text};
    use hc::zone_lines;

    /// Every zone of the IANA database's `zone1970.tab` with its principal
    /// location, as UTF-8 lines, returning the byte length written.
    ///
    /// One line per zone, in the table's order, tab-separated: the zone;
    /// the latitude of its principal location, decimal degrees north; the
    /// longitude, decimal degrees east; the ISO 3166-1 codes of the
    /// countries it overlaps, `;`-separated, the country of the location
    /// first; the one country `zone.tab` lists the zone under, `JP` for
    /// `Asia/Tokyo` where column 4 is `JP;AU`, empty for a zone it has no
    /// row for, which no zone of release 2026c is; the table's comment,
    /// which tells apart a country's zones and is empty where the country
    /// has one; the zone's CLDR 48 exemplar city in the locale; and the tag
    /// of the data that named the city. Each
    /// coordinate is the table's whole arcseconds written in decimal degrees
    /// to six places, `arcseconds × 10⁶ / 3600` millionths rounded half away
    /// from zero, so that multiplying by 3600 and rounding gives the
    /// arcseconds back. A build
    /// without the `calendars` feature, and a locale with no city for the
    /// zone, names the city in English, with `en` in column 8. The locale
    /// argument fails as `hc_parse_iso_date` does. A null `buffer` returns
    /// the length the text needs.
    ///
    /// # Safety
    ///
    /// `locale` must be readable for `locale_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless
    /// it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_zones(
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale, locale_len) } {
            Ok(tag) => tag,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&zone_lines::zones(tag), buffer, capacity) }
    }

    /// Where one zone is, as the UTF-8 line `hc_zones` writes for it,
    /// returning the byte length written.
    ///
    /// `zone` is an IANA name in any case: a zone of `zone1970.tab`; a link
    /// `zone.tab` gives a place of its own, such as `Europe/Oslo`, which
    /// answers with Oslo and not with the zone it links to; or another
    /// link of the database's `backward` file, such as `Asia/Calcutta`,
    /// which answers with the line of the name it leads to, so that column
    /// 1 is then `Asia/Kolkata`. A name that places nothing, such as `UTC`,
    /// is `HC_ERR_UNKNOWN`. Both text arguments fail as for
    /// `hc_parse_iso_date`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `zone` must be readable for `zone_len` bytes and `locale` for
    /// `locale_len`, unless null with a zero length; `buffer` must be
    /// writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_zone_location(
        zone: *const u8,
        zone_len: usize,
        locale: *const u8,
        locale_len: usize,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let (zone, tag) = match unsafe { (text(zone, zone_len), text(locale, locale_len)) } {
            (Ok(zone), Ok(tag)) => (zone, tag),
            (Err(sentinel), _) | (_, Err(sentinel)) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(zone_lines::zone_location(zone, tag), buffer, capacity) }
    }
}

#[cfg(feature = "tz")]
pub use zones::{hc_zone_location, hc_zones};

/// The sky, behind the `sky` feature: where the Sun and the Moon are at an
/// instant, the solar terms and the moon phases within a span, and the
/// Sun's decan, from `hc-astro`'s series, in Universal Time. The lines are
/// `hyper_calendar::sky_lines`', shared with the C library.
#[cfg(feature = "sky")]
mod sky {
    use super::emit_answer;
    use hc::sky_lines;

    /// The Sun and the Moon at a POSIX timestamp, as one UTF-8 line,
    /// returning the byte length written.
    ///
    /// Tab-separated: the Sun's apparent longitude in degrees, the
    /// Earth–Sun distance in astronomical units, the Moon's apparent
    /// longitude and latitude in degrees and its distance in kilometres,
    /// the Moon's elongation from the Sun in degrees (0 at new moon, 180 at
    /// full), the illuminated fraction of its disc, the last new moon
    /// before the instant and the first at or after it as POSIX seconds,
    /// ΔT in seconds, the regime ΔT was answered from (`observed`,
    /// `predicted`, `fitted` or `extrapolated`) and a `source` naming the
    /// series each figure came from. The instant is read as Universal
    /// Time. An instant outside the years −1000 to 3000, the era over
    /// which `hc-astro` states its series valid, is `HC_ERR_OUT_OF_RANGE`.
    /// A null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_sky_at(unix_seconds: i64, buffer: *mut u8, capacity: usize) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(sky_lines::sky_line(unix_seconds), buffer, capacity) }
    }

    /// Every solar term whose instant falls in `[from_unix, to_unix)`, as
    /// UTF-8 lines, returning the byte length written.
    ///
    /// One line per term, in time order, tab-separated: the Sun's
    /// apparent longitude that defines the term in degrees (0 for 春分
    /// through 345), the instant as whole POSIX seconds, rounded down, in
    /// Universal Time, the term's name in traditional Chinese and its name
    /// in Japanese. The span is half-open and is refused with
    /// `HC_ERR_OUT_OF_RANGE` when either end lies outside the years −1000
    /// to 3000 or when it is longer than 400 years; `to_unix` at or before
    /// `from_unix` is an empty answer. A null `buffer` returns the length
    /// the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_solar_terms_between(
        from_unix: i64,
        to_unix: i64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(sky_lines::term_lines(from_unix, to_unix), buffer, capacity) }
    }

    /// Every new moon, first quarter, full moon and last quarter whose
    /// instant falls in `[from_unix, to_unix)`, as UTF-8 lines, returning
    /// the byte length written.
    ///
    /// One line per phase, in time order, tab-separated: the Moon's
    /// elongation from the Sun that defines the phase in degrees (0, 90,
    /// 180 or 270), the instant as whole POSIX seconds, rounded down, in
    /// Universal Time, the phase's name (`new`, `first-quarter`, `full` or
    /// `last-quarter`) and an empty fourth column, so the lines have the
    /// shape of `hc_solar_terms_between`'s. The instants are the phase
    /// series of Meeus chapter 49, the same series that dates the new
    /// moons of `hc_sky_at`. The span fails as for
    /// `hc_solar_terms_between`. A null `buffer` returns the length the
    /// text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_moon_phases_between(
        from_unix: i64,
        to_unix: i64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(sky_lines::phase_lines(from_unix, to_unix), buffer, capacity) }
    }

    /// The decan the Sun is in at a POSIX timestamp, as one UTF-8 line,
    /// returning the byte length written.
    ///
    /// Tab-separated: the tropical sign, 1 for Aries through 12 for Pisces,
    /// and its English name; which of the sign's three 10° decans, or
    /// faces, the Sun is in, 1 to 3; the decan's ruler by al-Bīrūnī's
    /// table, the Chaldean order from Mars at the first face of Aries, as
    /// its identifier (`saturn`, `jupiter`, `mars`, `sun`, `venus`,
    /// `mercury`, `moon`) and its English name; and how far into the decan
    /// the Sun is, in degrees from 0 up to 10. The sign and the decan are
    /// read from the Sun's apparent longitude, so they turn at the solar
    /// terms. The instant is read as Universal Time; one outside the years
    /// −1000 to 3000 is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
    /// length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_decan_at(
        unix_seconds: i64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(sky_lines::decan_line(unix_seconds), buffer, capacity) }
    }
}

#[cfg(feature = "sky")]
pub use sky::{hc_decan_at, hc_moon_phases_between, hc_sky_at, hc_solar_terms_between};

/// The Earth's rotation and the Sun's hours, behind the `sky` feature: the
/// Earth Rotation Angle, the Greenwich mean sidereal time by two
/// conventions, UT2 − UT1, the clocks and named times of day of
/// `hc_astro::solar_time`, the named horizons with sunrise and sunset
/// against each, and the Heliocentric Julian Date in TT and in UTC. The
/// lines are `hyper_calendar::astro_lines`', shared with the C library,
/// and answer for the years −1000 to 3000.
#[cfg(feature = "sky")]
mod earth_and_sun {
    use super::{emit_answer, sentinel, text};
    use hc::astro_lines;

    /// Every named horizon a rising or a setting can be measured against,
    /// as UTF-8 lines, returning the byte length written.
    ///
    /// One line per horizon of `hc_astro::horizon::HORIZONS`,
    /// tab-separated: its identifier (`geometric-dip`, the default of the
    /// other exports; `usno`; `calendrical-calculations`), its English
    /// name, what it takes the visible horizon to be, its source, and a
    /// short English name for a label (`geometric dip`, `USNO`,
    /// `Calendrical Calculations`). A null `buffer` returns the length the
    /// text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_horizons(buffer: *mut u8, capacity: usize) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(Ok(astro_lines::horizons_lines()), buffer, capacity) }
    }

    /// A crossing's export: the horizon's name read, the place checked,
    /// and the line written.
    ///
    /// # Safety
    ///
    /// As `hc_sunrise`.
    #[allow(clippy::too_many_arguments)]
    unsafe fn crossing(
        line: fn(&str, i64, hc::hc_astro::Location) -> hc::boundary::Answer<String>,
        horizon: *const u8,
        horizon_len: usize,
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let horizon = match unsafe { text(horizon, horizon_len) } {
            Ok(horizon) => horizon,
            Err(sentinel) => return sentinel,
        };
        let answer = astro_lines::location(latitude, longitude, elevation)
            .and_then(|place| line(horizon, fixed, place));
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// Sunrise on a fixed day at a place against a named horizon, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// `horizon` is an identifier `hc_horizons` lists, in any case; anything
    /// else, the empty string included, is `HC_ERR_UNKNOWN`. The day is the
    /// local one, from local mean midnight at the longitude. The place is
    /// the latitude and longitude in degrees, north and east positive, and
    /// the elevation in metres, which the horizon may or may not take into
    /// account. Tab-separated: the instant the Sun's upper limb rises over
    /// the horizon as whole POSIX seconds of Universal Time, rounded down;
    /// the three cells of `hc_solar_event` naming a missing solar event,
    /// `sunrise` and the day, which are empty when the Sun rises, the first
    /// cell being empty instead when it does not; and the altitude of the
    /// Sun's centre at the crossing in degrees, which the horizon and the
    /// elevation fix. A place off the globe, or a day outside the years
    /// −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
    /// length the text needs.
    ///
    /// # Safety
    ///
    /// `horizon` must be readable for `horizon_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless
    /// it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_sunrise(
        horizon: *const u8,
        horizon_len: usize,
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe {
            crossing(
                astro_lines::sunrise_line,
                horizon,
                horizon_len,
                fixed,
                latitude,
                longitude,
                elevation,
                buffer,
                capacity,
            )
        }
    }

    /// Sunset on a fixed day at a place against a named horizon, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// As `hc_sunrise`, for the upper limb's setting, with `sunset` as the
    /// missing event.
    ///
    /// # Safety
    ///
    /// As `hc_sunrise`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_sunset(
        horizon: *const u8,
        horizon_len: usize,
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe {
            crossing(
                astro_lines::sunset_line,
                horizon,
                horizon_len,
                fixed,
                latitude,
                longitude,
                elevation,
                buffer,
                capacity,
            )
        }
    }

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

    /// A local clock's reading at a POSIX timestamp and a place, as one
    /// UTF-8 line, returning the byte length written.
    ///
    /// `clock` is `local-mean`, `local-apparent` (the sundial), `temporal`
    /// (unequal hours: 6 at sunrise, 18 at sunset) or `italian` (hours since
    /// the zero hour, half an hour after the sunset of the evening before),
    /// in any case; anything else is `HC_ERR_UNKNOWN`. The place is the
    /// latitude and longitude in degrees, north and east positive, and the
    /// elevation in metres. Tab-separated: the fixed day of the local date
    /// the reading belongs to, the hours into it, and three cells naming a
    /// solar event the reading needs that does not happen — `sunrise`,
    /// `sunset` or `depression`, the local day it is missing on, and the
    /// depression sought in arcminutes — which are empty when the reading
    /// exists; when it does not, the first two cells are empty instead. The
    /// timestamp is read as Universal Time. A place off the globe, or an
    /// instant outside the years −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A
    /// null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `clock` must be readable for `clock_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless
    /// it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_solar_time(
        clock: *const u8,
        clock_len: usize,
        unix_seconds: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let clock = match unsafe { text(clock, clock_len) } {
            Ok(clock) => clock,
            Err(sentinel) => return sentinel,
        };
        let place = match astro_lines::location(latitude, longitude, elevation) {
            Ok(place) => place,
            Err(refusal) => return sentinel(refusal),
        };
        let answer = astro_lines::solar_time_line(clock, unix_seconds, place);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// A named time of day on a fixed day at a place, as one UTF-8 line,
    /// returning the byte length written.
    ///
    /// `event` is `asr-shafii` or `asr-hanafi`, the afternoon prayer when a
    /// shadow is its noon length plus once or twice the object's height;
    /// `jewish-dusk-vilna-gaon`, the Sun 4°40′ below the horizon;
    /// `jewish-sabbath-ends-cohn`, 7°5′; or `italian-zero-hour`, half an
    /// hour after the Sun's centre is 16′ down; in any case, and anything
    /// else is `HC_ERR_UNKNOWN`. The place is as for `hc_solar_time`.
    /// Tab-separated: the instant as whole POSIX seconds of Universal Time,
    /// rounded down, and the three cells of `hc_solar_time` naming a
    /// missing solar event (`sunset`, `depression` or `no-noon-shadow`),
    /// empty when the time exists; when it does not, the first cell is
    /// empty instead. A place off the globe, or a day outside the years
    /// −1000 to 3000, is `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the
    /// length the text needs.
    ///
    /// # Safety
    ///
    /// `event` must be readable for `event_len` bytes unless null with a
    /// zero length; `buffer` must be writable for `capacity` bytes unless
    /// it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_solar_event(
        event: *const u8,
        event_len: usize,
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let event = match unsafe { text(event, event_len) } {
            Ok(event) => event,
            Err(sentinel) => return sentinel,
        };
        let place = match astro_lines::location(latitude, longitude, elevation) {
            Ok(place) => place,
            Err(refusal) => return sentinel(refusal),
        };
        let answer = astro_lines::solar_event_line(event, fixed, place);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The Heliocentric Julian Date in TT, HJD_TT, of a Julian Date of TT
    /// for a target, as one UTF-8 line, returning the byte length written.
    ///
    /// The target's direction is its right ascension, 0 to 360 degrees, and
    /// its declination, −90 to 90 degrees, on the mean equator and equinox
    /// of J2000; it is taken to be infinitely far away. Tab-separated: the
    /// HJD_TT, and the light-time correction added to the date, in seconds,
    /// negative when the light reaches the Sun before the Earth. The HJD
    /// is good to about 8 s, the Sun's own motion about the barycentre. A
    /// date outside the years −1000 to 3000 or not finite, or a direction
    /// outside those ranges, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
    /// returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_hjd_tt(
        tt_julian_date: f64,
        right_ascension: f64,
        declination: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = astro_lines::hjd_tt_line(tt_julian_date, right_ascension, declination);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The Heliocentric Julian Date in UTC, HJD_UTC, of a Julian Date of
    /// UTC for a target, as one UTF-8 line, returning the byte length
    /// written.
    ///
    /// As `hc_hjd_tt`, with the Earth's position taken at the TT instant of
    /// the UTC date: TT − UTC is 32.184 s plus TAI − UTC from the
    /// leap-second table. Tab-separated: the HJD_UTC, the correction added
    /// to the date in seconds, and the TT − UTC it used, in seconds.
    /// `strict` non-zero refuses a date outside the leap-second table,
    /// before 1961 or past its announced end, with `HC_ERR_NO_DATA`; zero
    /// holds the table's ends and takes TAI − UTC as 0 before 1961. A null
    /// `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_hjd_utc(
        utc_julian_date: f64,
        right_ascension: f64,
        declination: f64,
        strict: i32,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer =
            astro_lines::hjd_utc_line(utc_julian_date, right_ascension, declination, strict != 0);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }
}

#[cfg(feature = "sky")]
pub use earth_and_sun::{
    hc_earth_rotation_angle, hc_gmst_iau1982, hc_gmst_iau2006, hc_hjd_tt, hc_hjd_utc, hc_horizons,
    hc_solar_event, hc_solar_time, hc_sunrise, hc_sunset, hc_ut2_minus_ut1,
};

/// The orbit, behind the `orbital` feature: Earth's orbital elements and
/// the June insolation at 65° N they set, a million years either side of
/// 1950, from Berger's 1978 series in `hc-orbital`.
#[cfg(feature = "orbital")]
mod orbital {
    use super::{HC_ERR_OUT_OF_RANGE, emit_or_measure, push_cell};
    use hc::hc_core::math::floor;
    use hc::hc_orbital::{
        LATITUDE_65N, MID_JUNE_SOLAR_LONGITUDE, SOLAR_CONSTANT_BERGER_LOUTRE_1991, SOURCE,
        VALID_SPAN, daily_insolation, elements_at,
    };

    /// The solar constant every insolation cell is computed with: the
    /// 1360 W m⁻² of Berger & Loutre's 1991 tables, `hc-orbital`'s
    /// `SOLAR_CONSTANT_BERGER_LOUTRE_1991`, so a figure here compares with
    /// the NOAA `orbit91` table exactly. The value is written in its own
    /// column and the constant's name in the source; a page that wants
    /// another value — 1361 W m⁻² for a modern paper — scales the cell,
    /// since the insolation is proportional to the constant.
    pub(super) const SOLAR_CONSTANT: f64 = SOLAR_CONSTANT_BERGER_LOUTRE_1991;

    /// The name the source cell gives the constant.
    const SOLAR_CONSTANT_NAME: &str = "SOLAR_CONSTANT_BERGER_LOUTRE_1991";

    /// The most samples one call of [`hc_orbit_series`] writes.
    ///
    /// Ten thousand lines are about five megabytes of text, most of it the
    /// source cell repeated, and some forty milliseconds; a page draws far
    /// fewer columns than that, and a caller that wants more asks in
    /// pieces.
    pub(super) const MAX_SERIES_SAMPLES: usize = 10_000;

    /// Append the eleven cells of one epoch and the line break; see
    /// [`hc_orbit_at`] for the columns.
    ///
    /// # Errors
    ///
    /// [`HC_ERR_OUT_OF_RANGE`] for an epoch the crate refuses: not finite,
    /// or beyond a million years either side of 1950.
    fn push_epoch(out: &mut String, years_before_present: f64) -> Result<(), i64> {
        use core::fmt::Write;
        let elements = elements_at(years_before_present).map_err(|_| HC_ERR_OUT_OF_RANGE)?;
        let insolation = daily_insolation(
            &elements,
            LATITUDE_65N,
            MID_JUNE_SOLAR_LONGITUDE,
            SOLAR_CONSTANT,
        )
        .map_err(|_| HC_ERR_OUT_OF_RANGE)?;
        let _ = write!(
            out,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{insolation}\t{SOLAR_CONSTANT}\t",
            elements.eccentricity.value,
            elements.eccentricity.std_dev,
            elements.obliquity_degrees.value,
            elements.obliquity_degrees.std_dev,
            elements.longitude_of_perihelion_degrees.value,
            elements.longitude_of_perihelion_degrees.std_dev,
            elements.climatic_precession.value,
            elements.climatic_precession.std_dev,
        );
        push_cell(out, SOURCE);
        out.push_str(
            "; insolation: Berger (1978) daily mean at 65N for solar longitude 90, solar constant ",
        );
        out.push_str(SOLAR_CONSTANT_NAME);
        out.push('\n');
        Ok(())
    }

    /// The elements and the insolation at one epoch as one line; see
    /// [`hc_orbit_at`] for the columns.
    ///
    /// # Errors
    ///
    /// As [`push_epoch`].
    pub(super) fn orbit_line(years_before_present: f64) -> Result<String, i64> {
        let mut out = String::new();
        push_epoch(&mut out, years_before_present)?;
        Ok(out)
    }

    /// One line per sample from `from` to `to` in steps of `step`, each
    /// with the epoch first; see [`hc_orbit_series`].
    ///
    /// # Errors
    ///
    /// [`HC_ERR_OUT_OF_RANGE`] for a step that is not finite and positive,
    /// an end outside the span, or more than [`MAX_SERIES_SAMPLES`]
    /// samples.
    pub(super) fn series_lines(from: f64, to: f64, step: f64) -> Result<String, i64> {
        use core::fmt::Write;
        if !(from.is_finite() && to.is_finite() && step.is_finite()) || step <= 0.0 {
            return Err(HC_ERR_OUT_OF_RANGE);
        }
        if !(VALID_SPAN.contains(&from) && VALID_SPAN.contains(&to)) {
            return Err(HC_ERR_OUT_OF_RANGE);
        }
        let mut out = String::new();
        if to < from {
            return Ok(out);
        }
        // A step too small for the span divides to a count the cast
        // saturates, which the cap then refuses; the cap is checked before
        // the first sample is counted so that nothing overflows.
        let intervals = floor((to - from) / step) as usize;
        if intervals >= MAX_SERIES_SAMPLES {
            return Err(HC_ERR_OUT_OF_RANGE);
        }
        for sample in 0..=intervals {
            // Each sample is computed from the start rather than
            // accumulated, so the error does not grow along the series, and
            // the last one is held to `to` against rounding.
            let epoch = (from + sample as f64 * step).min(to);
            let _ = write!(out, "{epoch}\t");
            push_epoch(&mut out, epoch)?;
        }
        Ok(out)
    }

    /// Earth's orbital elements and the June insolation at 65° N at an
    /// epoch, as one UTF-8 line, returning the byte length written.
    ///
    /// The epoch is in years before 1950, negative for the future, as
    /// `hc-orbital` counts. Tab-separated: the eccentricity and its spread,
    /// the obliquity in degrees and its spread, the longitude of perihelion
    /// from the moving equinox in degrees (heliocentric, about 102° at
    /// present) and its spread, the climatic precession *e* sin ϖ and its
    /// spread, the daily mean insolation at 65° N at the June solstice in
    /// W m⁻², the solar constant that insolation was computed with in W m⁻²
    /// and the source, which names the series and the constant. Each
    /// spread is the measured disagreement between this series and Berger
    /// & Loutre's 1991 solution for the tier of the span the epoch falls
    /// in, not a Gaussian width; the perihelion's is 180° where the
    /// eccentricity is smaller than the precession's spread. An epoch that
    /// is not finite or lies beyond a million years either side of 1950 is
    /// `HC_ERR_OUT_OF_RANGE`: the series would return numbers there, and
    /// they would be fiction. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_orbit_at(
        years_before_1950: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let text = match orbit_line(years_before_1950) {
            Ok(text) => text,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }

    /// The line of `hc_orbit_at` at every epoch from `from_years_before_1950`
    /// to `to_years_before_1950` in steps of `step_years`, each with the
    /// epoch as a first column, as UTF-8 lines, returning the byte length
    /// written.
    ///
    /// One line per sample, tab-separated: the epoch in years before 1950,
    /// then the eleven columns of `hc_orbit_at`. The samples are `from`,
    /// `from + step`, `from + 2 step` and so on, every one at or before
    /// `to`, so `from` and `to` themselves are samples when `to - from` is
    /// a multiple of `step`, and a page drawing across the span asks once
    /// rather than once per column. Both ends have to lie within a million
    /// years either side of 1950 and `step` has to be finite and positive,
    /// else `HC_ERR_OUT_OF_RANGE`; more than 10 000 samples is
    /// `HC_ERR_OUT_OF_RANGE` too, and a caller who wants more asks in
    /// pieces. A `to` before `from` is an empty answer of zero bytes, not an
    /// error. A null `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_orbit_series(
        from_years_before_1950: f64,
        to_years_before_1950: f64,
        step_years: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let text = match series_lines(from_years_before_1950, to_years_before_1950, step_years) {
            Ok(text) => text,
            Err(sentinel) => return sentinel,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_or_measure(&text, buffer, capacity) }
    }
}

#[cfg(feature = "orbital")]
pub use orbital::{hc_orbit_at, hc_orbit_series};

/// Time on other bodies, behind the `planetary` feature: Mars time, the
/// surface missions' sol counts, and the solar day and local mean solar
/// time of every body in `hc-planetary`'s table.
///
/// Titan's and the Galilean moons' circad calendars and the Martiana
/// calendar are [`hc_circad_date`], one export with its own line beside
/// [`hc_mars_time`], not new columns of an existing one.
#[cfg(feature = "planetary")]
mod planetary {
    use super::{emit_answer, text, value};
    use hc::planetary_lines;

    /// Mars at a POSIX instant and an east longitude, as one UTF-8 line,
    /// returning the byte length written.
    ///
    /// Tab-separated: the Mars Sol Date; Coordinated Mars Time, local mean
    /// solar time and local true solar time at the longitude, each as
    /// `HH:MM:SS` on the 24-hour Martian clock (truncated) and in decimal
    /// Martian hours; the equation of time in Martian minutes; the
    /// areocentric solar longitude `Ls` in degrees; the Mars year under the
    /// Clancy convention; the Darian year, month, sol of the month, month
    /// name and sol-of-week name at Airy-0; and the source. `unix_seconds`
    /// is POSIX time with a fraction, read through the leap-second table
    /// with the last offset held; `east_longitude_degrees` is planetocentric,
    /// east-positive, and wraps. An instant more than 100 Julian years from
    /// J2000.0, where Allison and McEwen's series is an extrapolation, or a
    /// value that is not finite, is `HC_ERR_OUT_OF_RANGE`. A null `buffer`
    /// returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_mars_time(
        unix_seconds: f64,
        east_longitude_degrees: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer = planetary_lines::mars_time_line(unix_seconds, east_longitude_degrees);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// Every surface mission on Mars and the rules of its sol count, as
    /// UTF-8 lines, returning the byte length written.
    ///
    /// One line per mission, in landing order, tab-separated: the
    /// identifier, the name, the landing instant as UTC text and as a POSIX
    /// timestamp, the number of the landing sol (0 or 1), the clock's
    /// midnight (`local-mean-solar-time`, or
    /// `local-true-solar-time-at-landing`), the clock meridian's east
    /// longitude, the achieved site's east longitude, `1` where the
    /// operators published the convention and `0` otherwise, the note and
    /// the source. Where no convention was published the landing sol, the
    /// clock and its meridian are empty. A null `buffer` returns the
    /// length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_missions(buffer: *mut u8, capacity: usize) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(Ok(planetary_lines::missions_lines()), buffer, capacity) }
    }

    /// The sol number of a Mars surface mission at a POSIX instant, by the
    /// mission's own clock, or an error sentinel.
    ///
    /// `mission` is an identifier or a name `hc_missions` lists, in any
    /// ASCII case. The sol is counted as the mission counted it: from the
    /// midnight, mean or true, on the mission's clock meridian that began
    /// the landing sol, which is sol 0 or sol 1 as the operators numbered
    /// it. A mission the table does not carry is `HC_ERR_UNKNOWN`; one
    /// whose operators published no sol numbering is `HC_ERR_NO_DATA`; an
    /// instant before the landing sol began, or outside `hc_mars_time`'s
    /// span, is `HC_ERR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `mission` must be readable for `mission_len` bytes unless null with
    /// a zero length.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_mission_sol(
        mission: *const u8,
        mission_len: usize,
        unix_seconds: f64,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { text(mission, mission_len) } {
            Ok(name) => name,
            Err(sentinel) => return sentinel,
        };
        value(planetary_lines::mission_sol(name, unix_seconds))
    }

    /// Every body `hc-planetary` carries, with its solar day, as UTF-8
    /// lines, returning the byte length written.
    ///
    /// One line per body, outward from the Sun with each planet's moons
    /// after it, tab-separated: the identifier, the name, the kind
    /// (`star`, `planet`, `dwarf-planet` or `moon`), the identifier of the
    /// body it orbits, the sidereal rotation period in hours (negative for
    /// a retrograde rotator), the solar day in SI seconds, `measured` or
    /// `derived`, the year in local solar days, whether the clock's zero
    /// point is a `standard` or a `convention` this library declares, what
    /// the zero point is, the source, and the status of a standard still
    /// being drawn up (the Moon's Coordinated Lunar Time). The Sun has no
    /// solar day, and its three day cells are empty. A null `buffer`
    /// returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_bodies(buffer: *mut u8, capacity: usize) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(Ok(planetary_lines::bodies_lines()), buffer, capacity) }
    }

    /// Local mean solar time on a body at a POSIX instant and an east
    /// longitude, as one UTF-8 line, returning the byte length written.
    ///
    /// Tab-separated: the local day number, the fraction of it elapsed,
    /// the reading as `HH:MM:SS` (truncated) and in decimal local hours on
    /// a 24-hour face, the solar day and the local hour in SI seconds,
    /// whether the zero point is a `standard` or a `convention`, and what
    /// it is. `body` is an identifier or a name `hc_bodies` lists, in any
    /// ASCII case; a body it does not list is `HC_ERR_UNKNOWN`, and the
    /// Sun, which has no solar day, is `HC_ERR_NO_DATA`. The instant and
    /// the longitude fail as for `hc_mars_time`. A null `buffer` returns
    /// the length the text needs.
    ///
    /// # Safety
    ///
    /// `body` must be readable for `body_len` bytes unless null with a zero
    /// length, and `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_body_time(
        body: *const u8,
        body_len: usize,
        unix_seconds: f64,
        east_longitude_degrees: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { text(body, body_len) } {
            Ok(name) => name,
            Err(sentinel) => return sentinel,
        };
        let answer = planetary_lines::body_time_line(name, unix_seconds, east_longitude_degrees);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// The date at a POSIX instant in a calendar of another body's days, as
    /// one UTF-8 line, returning the byte length written.
    ///
    /// `calendar` is `darian-titan`, `gregorian-io`, `gregorian-europa`,
    /// `gregorian-ganymede`, `gregorian-callisto` or `martiana`, in any
    /// ASCII case; another is `HC_ERR_UNKNOWN`. Tab-separated: the
    /// calendar, the year, the month, the day of the month, the month's
    /// name, the name of the day's place in the week, the day count the
    /// date is numbered by, the fraction of that day elapsed, `1` for a
    /// leap year, and the source. Titan's and the Galilean moons' days are
    /// *circads*, fixed fractions of the moon's solar day in weeks of
    /// eight, counted from the calendar's epoch by Gangale's calibration;
    /// Martiana's are sols at Airy-0 in weeks of seven, counted as the
    /// Darian sol number, with an empty week cell on the epagomenal sol of
    /// every tenth year. The instant is read as for `hc_mars_time`, and one
    /// more than 100 Julian years from J2000.0, or not finite, is
    /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `calendar` must be readable for `calendar_len` bytes unless null with
    /// a zero length, and `buffer` must be writable for `capacity` bytes
    /// unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_circad_date(
        calendar: *const u8,
        calendar_len: usize,
        unix_seconds: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { text(calendar, calendar_len) } {
            Ok(name) => name,
            Err(sentinel) => return sentinel,
        };
        let answer = planetary_lines::circad_date_line(name, unix_seconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }
}

#[cfg(feature = "planetary")]
pub use planetary::{
    hc_bodies, hc_body_time, hc_circad_date, hc_mars_time, hc_mission_sol, hc_missions,
};

/// Relativistic time dilation, behind the `relativity` feature: a clock
/// at a constant velocity, and a clock held still at a radius from a
/// mass, from `hc-relativity`'s Schwarzschild formulas and named
/// constants.
#[cfg(feature = "relativity")]
mod relativity {
    use super::{emit_answer, text};
    use hc::relativity_lines;

    /// A clock moving at a constant speed while some coordinate time
    /// passes, as one UTF-8 line, returning the byte length written.
    ///
    /// Tab-separated: β, the Lorentz factor γ, the proper time the moving
    /// clock records in seconds, its rate dτ/dt = 1/γ, that rate's offset
    /// from 1 in microseconds per 86 400-second day (negative, computed
    /// without cancellation), the `hc-relativity` constant used
    /// (`SPEED_OF_LIGHT`), and the source. A speed at or beyond the speed
    /// of light either way, or a value that is not finite, is
    /// `HC_ERR_OUT_OF_RANGE`. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_proper_time(
        speed_metres_per_second: f64,
        coordinate_seconds: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        let answer =
            relativity_lines::proper_time_line(speed_metres_per_second, coordinate_seconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// A clock held still at a radius from a body's centre, against one far
    /// from every mass, as one UTF-8 line, returning the byte length
    /// written.
    ///
    /// Tab-separated: the body's identifier, its GM in m³ s⁻², the name of
    /// the `hc-relativity` constant that holds it, the Schwarzschild radius
    /// in metres, the static dilation factor dτ/dt = √(1 − r_s/r), that
    /// factor's offset from 1 in microseconds per 86 400-second day
    /// (negative, computed without cancellation), the constants used,
    /// separated by `;`, and the body's source. `body` is an identifier or a name
    /// `hc_gravitating_bodies` lists, in any ASCII case; another is
    /// `HC_ERR_UNKNOWN`. A radius that is not finite, not positive, or at
    /// or inside the Schwarzschild radius is `HC_ERR_OUT_OF_RANGE`. A null
    /// `buffer` returns the length the text needs.
    ///
    /// # Safety
    ///
    /// `body` must be readable for `body_len` bytes unless null with a zero
    /// length, and `buffer` must be writable for `capacity` bytes unless it
    /// is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gravitational_dilation(
        body: *const u8,
        body_len: usize,
        radius_metres: f64,
        buffer: *mut u8,
        capacity: usize,
    ) -> i64 {
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { text(body, body_len) } {
            Ok(name) => name,
            Err(sentinel) => return sentinel,
        };
        let answer = relativity_lines::gravitational_dilation_line(name, radius_metres);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(answer, buffer, capacity) }
    }

    /// Every body `hc-relativity` carries a gravitational parameter for, as
    /// UTF-8 lines, returning the byte length written.
    ///
    /// One line per body, tab-separated: the identifier, the English name,
    /// GM in m³ s⁻², the name of the `hc-relativity` constant that holds
    /// it, and the source. A null `buffer` returns the length the text
    /// needs.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes unless it is null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gravitating_bodies(buffer: *mut u8, capacity: usize) -> i64 {
        let text = relativity_lines::gravitating_bodies_lines();
        // SAFETY: forwarded to the caller's contract above.
        unsafe { emit_answer(Ok(text), buffer, capacity) }
    }
}

#[cfg(feature = "relativity")]
pub use relativity::{hc_gravitating_bodies, hc_gravitational_dilation, hc_proper_time};

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
            HC_ERR_MALFORMED,
        ] {
            assert!(sentinel <= HC_ERR_FLOOR);
        }
    }

    #[test]
    fn a_cell_never_carries_a_separator() {
        let mut out = String::new();
        push_cell(&mut out, "a\tb\nc\r\nd");
        assert_eq!(out, "a b c  d");
    }

    /// Call a line-writing export the way a page does: measure with a null
    /// buffer, refuse a buffer that is too small, then read the text.
    #[cfg(any(
        feature = "timestamps",
        feature = "calendars",
        feature = "holiday",
        feature = "seasons",
        feature = "deep-time",
        feature = "tz",
        feature = "sky",
        feature = "orbital",
        feature = "planetary",
        feature = "relativity"
    ))]
    fn read_lines(call: impl Fn(*mut u8, usize) -> i64) -> String {
        let needed = call(core::ptr::null_mut(), 0);
        assert!(needed > 0, "measured {needed}");
        let capacity = needed as usize;
        let mut small = [7u8; 1];
        assert_eq!(
            call(small.as_mut_ptr(), small.len()),
            HC_ERR_BUFFER_TOO_SMALL
        );
        assert_eq!(small, [7u8], "a refused buffer is left untouched");
        let pointer = hc_alloc(capacity);
        assert_eq!(call(pointer, capacity), needed);
        let text = unsafe { core::str::from_utf8(core::slice::from_raw_parts(pointer, capacity)) }
            .expect("UTF-8")
            .to_owned();
        unsafe { hc_free(pointer, capacity) };
        assert!(text.ends_with('\n'), "{text:?}");
        text
    }

    #[cfg(feature = "civil")]
    mod civil {
        use super::super::*;

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
        fn the_gregorian_range_is_the_readmes() {
            let earliest = -3_652_424_999;
            let latest = 3_652_424_634;
            assert_eq!(hc_gregorian_to_fixed(-9_999_999, 1, 1), earliest);
            assert_eq!(hc_gregorian_to_fixed(9_999_999, 12, 31), latest);
            assert_eq!(
                hc_gregorian_to_fixed(-10_000_000, 12, 31),
                HC_ERR_INVALID_DATE
            );
            assert_eq!(hc_gregorian_to_fixed(10_000_000, 1, 1), HC_ERR_INVALID_DATE);
            assert_eq!(hc_gregorian_year(earliest), -9_999_999);
            assert_eq!(hc_gregorian_year(latest), 9_999_999);
            assert_eq!(hc_gregorian_year(earliest - 1), HC_ERR_OUT_OF_RANGE);
            assert_eq!(hc_gregorian_year(latest + 1), HC_ERR_OUT_OF_RANGE);
            assert_eq!(hc_gregorian_year(i64::MIN), HC_ERR_OUT_OF_RANGE);
            assert_eq!(hc_gregorian_to_fixed(i64::MIN, 1, 1), HC_ERR_INVALID_DATE);
            assert_eq!(hc_gregorian_to_fixed(i64::MAX, 12, 31), HC_ERR_INVALID_DATE);
            for text in [
                "-9999999-01-01",
                "+9999999-12-31",
                "-10000000-12-31",
                "+99999999999999999999-01-01",
                "-99999999999999999999-01-01",
            ] {
                let parsed = unsafe { hc_parse_iso_date(text.as_ptr(), text.len()) };
                assert!(
                    parsed == HC_ERR_INVALID_DATE || (earliest..=latest).contains(&parsed),
                    "{text}: {parsed}"
                );
            }
            // Every day has a weekday, to both ends of an i64.
            assert_eq!(hc_weekday(i64::MIN), 6);
            assert_eq!(hc_weekday(i64::MAX), 7);
        }

        /// The first fixed day `hc_unix_from_fixed` answers for, and the
        /// last: the README's range.
        const UNIX_FROM_FIXED_FIRST: i64 = -104_165_947_503;
        const UNIX_FROM_FIXED_LAST: i64 = 106_751_991_886_463;

        #[test]
        fn midnight_in_seconds_never_reads_as_a_sentinel() {
            // The last day whose midnight is above the floor, and the first
            // whose midnight would reach it.
            let first = hc_unix_from_fixed(UNIX_FROM_FIXED_FIRST);
            assert_eq!(first, -8_999_999_999_942_400);
            assert!(first > HC_ERR_FLOOR);
            assert_eq!(
                hc_unix_from_fixed(UNIX_FROM_FIXED_FIRST - 1),
                HC_ERR_OUT_OF_RANGE
            );
            // The day the report was about, some 54 billion years back.
            assert_eq!(hc_unix_from_fixed(-19_723_095_000_000), HC_ERR_OUT_OF_RANGE);
        }

        #[test]
        fn midnight_in_seconds_refuses_rather_than_overflows() {
            // The last day whose midnight fits an i64, and the first that
            // would not.
            assert_eq!(
                hc_unix_from_fixed(UNIX_FROM_FIXED_LAST),
                9_223_372_036_854_720_000
            );
            assert_eq!(
                hc_unix_from_fixed(UNIX_FROM_FIXED_LAST + 1),
                HC_ERR_OUT_OF_RANGE
            );
            // 2^53 days, whose midnight a wrapping product would put at
            // 3458764451684857728.
            assert_eq!(hc_unix_from_fixed(1 << 53), HC_ERR_OUT_OF_RANGE);
            assert_eq!(hc_unix_from_fixed(i64::MAX), HC_ERR_OUT_OF_RANGE);
            // Overflow downwards too: the subtraction, then the product.
            assert_eq!(hc_unix_from_fixed(i64::MIN), HC_ERR_OUT_OF_RANGE);
            assert_eq!(hc_unix_from_fixed(i64::MIN / 86_400), HC_ERR_OUT_OF_RANGE);
        }

        #[test]
        fn every_timestamp_has_a_fixed_day_above_the_floor() {
            assert_eq!(hc_fixed_from_unix(i64::MIN), -106_751_990_448_138);
            assert_eq!(hc_fixed_from_unix(i64::MAX), UNIX_FROM_FIXED_LAST);
            assert_eq!(
                hc_unix_from_fixed(hc_fixed_from_unix(i64::MAX)),
                9_223_372_036_854_720_000
            );
        }

        #[test]
        fn the_leap_second_question_refuses_a_day_with_no_i64_bounds() {
            // The first whole day of the range, and the part-day before it.
            let first_whole = -9_223_372_036_854_720_000;
            assert_eq!(hc_day_has_leap_second(first_whole), 0);
            assert_eq!(hc_day_has_leap_second(first_whole - 1), HC_ERR_OUT_OF_RANGE);
            assert_eq!(hc_day_has_leap_second(i64::MIN), HC_ERR_OUT_OF_RANGE);
            // The last day whose end is an i64, and the part-day after it.
            let last_end = 9_223_372_036_854_720_000;
            assert_eq!(hc_day_has_leap_second(last_end - 1), 0);
            assert_eq!(hc_day_has_leap_second(last_end), HC_ERR_OUT_OF_RANGE);
            assert_eq!(hc_day_has_leap_second(i64::MAX), HC_ERR_OUT_OF_RANGE);
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

    #[cfg(feature = "calendars")]
    mod calendars {
        use super::super::*;
        use super::read_lines;

        /// 21 March 2005, a Monday of March, is Başgün of Nowruz in Turkmen
        /// (Wikipedia's names of 2002 to 2008); in Russian it is ordinary.
        #[test]
        fn a_turkmen_day_of_2005_is_named_by_the_period() {
            let day = hc_gregorian_to_fixed(2005, 3, 21);
            let read = |calendar: &str, locale: &str| {
                read_lines(|buffer, capacity| unsafe {
                    hc_naming_period_on(
                        calendar.as_ptr(),
                        calendar.len(),
                        day,
                        locale.as_ptr(),
                        locale.len(),
                        buffer,
                        capacity,
                    )
                })
            };
            let text = read("gregory", "tk-TM");
            assert!(
                text.starts_with("in-force\tturkmen-2002\tNowruz\tBaşgün\tFirst day\t"),
                "{text}"
            );
            assert_eq!(text.trim_end_matches('\n').split('\t').count(), 9);
            assert_eq!(read("gregory", "ru"), "ordinary\t\t\t\t\t\t\t\t\n");
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_naming_period_on("maya".as_ptr(), 4, day, "tk".as_ptr(), 2, null, 0) },
                HC_ERR_UNKNOWN
            );
        }

        /// 2026-09-21, described for a locale.
        fn describe(locale: &str) -> Vec<Vec<String>> {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_describe_day(739_880, locale.as_ptr(), locale.len(), buffer, capacity)
            });
            text.lines()
                .map(|line| line.split('\t').map(str::to_owned).collect())
                .collect()
        }

        fn row<'a>(rows: &'a [Vec<String>], id: &str) -> &'a [String] {
            rows.iter()
                .find(|row| row[0] == id)
                .unwrap_or_else(|| panic!("no row for {id}"))
        }

        fn first_day(locale: &str) -> i64 {
            unsafe { hc_first_day_of_week(locale.as_ptr(), locale.len()) }
        }

        #[test]
        fn the_first_day_of_the_week_follows_the_locale() {
            // By region, by the language's likely region, and by `-u-fw-`.
            for (tag, day) in [
                ("en-US", 7),
                ("en-GB", 1),
                ("en", 7),
                ("ja", 7),
                ("zh-Hans", 1),
                ("ar-SA", 7),
                ("ar-EG", 6),
                ("fr", 1),
                ("pt", 7),
                ("pt-PT", 7),
                ("de-u-fw-sun", 7),
                ("und", 1),
                ("not a tag", 1),
                ("", 1),
            ] {
                assert_eq!(first_day(tag), day, "{tag}");
            }
            assert_eq!(
                unsafe { hc_first_day_of_week(core::ptr::null(), 0) },
                1,
                "a null empty tag is the root locale"
            );
            assert_eq!(
                unsafe { hc_first_day_of_week(core::ptr::null(), 2) },
                HC_ERR_NULL_POINTER
            );
            let bad = [0xff_u8, 0xfe];
            assert_eq!(
                unsafe { hc_first_day_of_week(bad.as_ptr(), bad.len()) },
                HC_ERR_NOT_UTF8
            );
        }

        #[test]
        fn every_registered_calendar_is_a_line_with_eighteen_columns() {
            let rows = describe("en");
            assert_eq!(rows.len(), hc::registry().len());
            let ids: Vec<&str> = hc::registry().metas().map(|meta| meta.id.0).collect();
            let listed: Vec<&str> = rows.iter().map(|row| row[0].as_str()).collect();
            assert_eq!(listed, ids, "registry order");
            for row in &rows {
                assert_eq!(row.len(), 18, "{row:?}");
                // A midnight start needs no naming; every other one has it.
                assert_eq!(row[17].is_empty(), row[14] == "midnight", "{row:?}");
                assert!(["", "start", "end"].contains(&row[17].as_str()), "{row:?}");
                // Every calendar states where its day begins and which
                // locale answered.
                assert!(!row[14].is_empty(), "{row:?}");
                assert!(!row[16].is_empty(), "{row:?}");
                // A converted day is formatted; a refused one is not.
                assert_eq!(row[15].is_empty(), !row[11].is_empty(), "{row:?}");
            }
        }

        #[test]
        fn a_converted_day_decodes_column_by_column() {
            let rows = describe("ja-JP");
            let japanese = row(&rows, "japanese");
            assert_eq!(
                japanese,
                [
                    "japanese",
                    "Japanese (imperial eras)",
                    "reiwa",
                    "令和",
                    "8",
                    "9",
                    "0",
                    "9月",
                    "21",
                    "0",
                    "",
                    "",
                    "",
                    // The era calendar has been in use since 862, and never
                    // abandoned.
                    "in-use",
                    "midnight",
                    "令和8年9月21日",
                    "ja",
                    ""
                ]
            );
            let gregorian = row(&rows, "gregory");
            assert_eq!(gregorian[0..2], ["gregory", "Gregorian"]);
            assert_eq!(gregorian[4..10], ["2026", "9", "0", "9月", "21", "0"]);
            assert_eq!(gregorian[11..14], ["", "", "in-use"]);
            assert_eq!(gregorian[15..17], ["2026年9月21日", "ja"]);
            // Japanese has no words for the Hebrew months, so the Hebrew
            // calendar answers in English, not Hebrew, and says so.
            let hebrew = row(&rows, "hebrew");
            assert_eq!(hebrew[16], "en");
            assert!(hebrew[7].is_ascii(), "{hebrew:?}");
            // The last column names the civil day a day is named after: the
            // Hebrew day that begins at sunset by the one it ends on, the
            // Julian Day that begins at noon by the one it begins on.
            assert_eq!(
                (hebrew[14].as_str(), hebrew[17].as_str()),
                ("sunset", "end")
            );
            let julian_day = row(&rows, "julian-day");
            assert_eq!(
                (julian_day[14].as_str(), julian_day[17].as_str()),
                ("noon", "start")
            );
            let tibetan = row(&rows, "tibetan");
            assert_eq!(
                (tibetan[14].as_str(), tibetan[17].as_str()),
                ("local-time 05:00:00", "start")
            );
            let rows = describe("en");
            assert_eq!(row(&rows, "gregory")[7], "September");
            assert_eq!(row(&rows, "gregory")[15..17], ["September 21, 2026", "en"]);
            assert_eq!(row(&rows, "hebrew")[16], "en");
            // A tag with no data falls back to the root locale, whose month
            // names are CLDR's `M01`..`M12` rather than English.
            let rows = describe("tlh");
            assert_eq!(row(&rows, "gregory")[7], "M09");
            assert_eq!(row(&rows, "gregory")[16], "und");
            // A tag that does not parse at all falls back the same way.
            let rows = describe("!!");
            assert_eq!(row(&rows, "gregory")[7], "M09");
            // And `native` renders each calendar in its own language.
            let rows = describe("native");
            assert_eq!(row(&rows, "gregory")[16], "en");
            assert_eq!(row(&rows, "japanese")[15..17], ["令和8年9月21日", "ja"]);
            assert_eq!(row(&rows, "hebrew")[16], "he");
            assert_eq!(row(&rows, "chinese")[16], "zh-Hans");
        }

        #[test]
        fn a_leap_month_and_the_extra_fields_are_carried() {
            // 2023-03-22 was 閏二月初一 in the Chinese calendar.
            let day = hc_gregorian_to_fixed(2023, 3, 22);
            let text = read_lines(|buffer, capacity| unsafe {
                hc_describe_day(day, "zh-Hans".as_ptr(), 7, buffer, capacity)
            });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            let chinese = rows
                .iter()
                .find(|row| row[0] == "chinese")
                .expect("chinese");
            assert_eq!(chinese[5..10], ["2", "1", "闰二月", "1", "0"]);
            assert!(chinese[10].contains("cycle="), "{chinese:?}");
            assert!(chinese[10].contains(';'), "{chinese:?}");
            assert_eq!(chinese[15..17], ["癸卯年闰二月初一", "zh-Hans"]);
        }

        #[test]
        fn a_refusal_is_a_line_with_its_code_and_name() {
            let rows = describe("en");
            // The Rumi calendar was kept only from 1840 to 1925.
            let rumi = row(&rows, "rumi");
            assert_eq!(rumi[0..2], ["rumi", "Rumi"]);
            assert!(rumi[2..11].iter().all(String::is_empty), "{rumi:?}");
            assert_eq!(rumi[11..14], ["7", "after-supported-range", ""]);
            assert_eq!(rumi[14..17], ["midnight", "", "en"]);
        }

        /// The units of one calendar over a range, decoded.
        fn walk(id: &str, unit: u32, from: i64, to: i64, locale: &str) -> Vec<Vec<String>> {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_calendar_units(
                    id.as_ptr(),
                    id.len(),
                    unit,
                    from,
                    to,
                    locale.as_ptr(),
                    locale.len(),
                    buffer,
                    capacity,
                )
            });
            text.lines()
                .map(|line| line.split('\t').map(str::to_owned).collect())
                .collect()
        }

        #[test]
        fn the_units_of_a_calendar_are_labelled_spans() {
            // The Japanese eras from the Meiji Restoration to Reiwa.
            let eras = walk(
                "japanese",
                0,
                hc_gregorian_to_fixed(1868, 1, 1),
                hc_gregorian_to_fixed(2019, 12, 31),
                "ja",
            );
            let labels: Vec<&str> = eras.iter().map(|row| row[2].as_str()).collect();
            assert!(
                labels.ends_with(&["明治", "大正", "昭和", "平成", "令和"]),
                "{labels:?}"
            );
            let reiwa = eras.last().expect("reiwa");
            assert_eq!(reiwa.len(), 8);
            assert_eq!(reiwa[0], hc_gregorian_to_fixed(2019, 5, 1).to_string());
            // The imperial-era calendar records no period of use.
            assert_eq!(reiwa[3..8], ["0", "in-use", "", "", "ja"]);
            for pair in eras.windows(2) {
                assert_eq!(pair[0][1], pair[1][0], "spans touch");
            }
            // The same eras in English, from the calendar's own romanisation
            // where the data lists none.
            let eras = walk(
                "japanese",
                0,
                hc_gregorian_to_fixed(1850, 1, 1),
                hc_gregorian_to_fixed(1870, 1, 1),
                "en",
            );
            let labels: Vec<&str> = eras.iter().map(|row| row[2].as_str()).collect();
            assert!(
                labels.contains(&"Kaei") && labels.contains(&"Meiji"),
                "{labels:?}"
            );
            // A first year is 元年 and the years after it are numbered.
            let years = walk(
                "japanese",
                1,
                hc_gregorian_to_fixed(2019, 5, 1),
                hc_gregorian_to_fixed(2020, 1, 2),
                "ja",
            );
            let labels: Vec<&str> = years.iter().map(|row| row[2].as_str()).collect();
            assert_eq!(labels, ["令和元年", "令和2年"]);
            // The Chinese months of 2023 carry the leap second month.
            let months = walk(
                "chinese",
                2,
                hc_gregorian_to_fixed(2023, 1, 22),
                hc_gregorian_to_fixed(2024, 2, 10),
                "zh-Hans",
            );
            assert_eq!(months.len(), 13, "{months:?}");
            let leap = months
                .iter()
                .find(|row| row[3] == "1")
                .expect("a leap month");
            assert_eq!(leap[2], "闰二月");
            assert_eq!(leap[0], hc_gregorian_to_fixed(2023, 3, 22).to_string());
            assert_eq!(months[0][2], "正月");
            // A calendar's edges are refusals with the calendar's own error.
            let years = walk(
                "rumi",
                1,
                hc_gregorian_to_fixed(1925, 1, 1),
                hc_gregorian_to_fixed(1927, 1, 1),
                "en",
            );
            let last = years.last().expect("a span");
            assert_eq!(last[2..7], ["", "", "", "7", "after-supported-range"]);
            assert_eq!(last[7], "en");
            // A unit the calendar does not have is one refusal.
            let months = walk("maya-longcount", 2, 0, 1_000, "en");
            assert_eq!(months.len(), 1);
            assert_eq!(months[0][5..7], ["5", "unsupported-field"]);
            let days = walk("maya-longcount", 3, 0, 3, "en");
            assert_eq!(days.len(), 3);
            assert!(!days[0][2].is_empty(), "{days:?}");
        }

        #[test]
        fn thirty_years_of_chinese_months_are_walked_by_month_not_by_day() {
            let started = std::time::Instant::now();
            let months = walk(
                "chinese",
                2,
                hc_gregorian_to_fixed(1996, 1, 1),
                hc_gregorian_to_fixed(2026, 1, 1),
                "zh-Hans",
            );
            let elapsed = started.elapsed();
            assert!(months.len() >= 370, "{} months", months.len());
            // About a third of a second in a release build on a developer
            // machine; a shared CI runner, with the suite running in
            // parallel, has taken one and a half. The bound is a regression
            // guard against a day-by-day walk, which takes minutes, not a
            // benchmark. A debug build is too slow and too variable for any
            // bound, so the time is only reported there.
            if !cfg!(debug_assertions) {
                assert!(
                    elapsed.as_secs_f64() < 5.0,
                    "{} months took {elapsed:?}",
                    months.len()
                );
            }
            eprintln!("{} Chinese months walked in {elapsed:?}", months.len());
        }

        #[test]
        fn units_refuse_what_they_do_not_know() {
            let id = "no-such-calendar";
            assert_eq!(
                unsafe {
                    hc_calendar_units(
                        id.as_ptr(),
                        id.len(),
                        1,
                        0,
                        10,
                        "en".as_ptr(),
                        2,
                        core::ptr::null_mut(),
                        0,
                    )
                },
                HC_ERR_UNKNOWN
            );
            let id = "gregory";
            assert_eq!(
                unsafe {
                    hc_calendar_units(
                        id.as_ptr(),
                        id.len(),
                        4,
                        0,
                        10,
                        "en".as_ptr(),
                        2,
                        core::ptr::null_mut(),
                        0,
                    )
                },
                HC_ERR_UNKNOWN
            );
            // An empty range is no text at all.
            assert_eq!(
                unsafe {
                    hc_calendar_units(
                        id.as_ptr(),
                        id.len(),
                        1,
                        10,
                        10,
                        "en".as_ptr(),
                        2,
                        core::ptr::null_mut(),
                        0,
                    )
                },
                0
            );
            // At the cap, 100 000 days as days, the text is measured; one
            // day more, or a trillion, is refused without being built.
            let units_of = |unit: u32, from: i64, to: i64| unsafe {
                hc_calendar_units(
                    id.as_ptr(),
                    id.len(),
                    unit,
                    from,
                    to,
                    "en".as_ptr(),
                    2,
                    core::ptr::null_mut(),
                    0,
                )
            };
            let cap = hc::lines::MAX_CALENDAR_UNITS as i64;
            assert_eq!(cap, 100_000);
            assert!(units_of(3, 700_000, 700_000 + cap) > 0);
            assert_eq!(units_of(3, 700_000, 700_000 + cap + 1), HC_ERR_OUT_OF_RANGE);
            assert_eq!(units_of(3, 0, 1_000_000_000_000), HC_ERR_OUT_OF_RANGE);
            // The cap counts lines, not days: a millennium of years is a
            // thousand lines.
            assert!(units_of(1, 700_000, 700_000 + 365_243) > 0);
            let not_utf8 = [0xffu8];
            assert_eq!(
                unsafe {
                    hc_calendar_units(
                        not_utf8.as_ptr(),
                        1,
                        1,
                        0,
                        10,
                        "en".as_ptr(),
                        2,
                        core::ptr::null_mut(),
                        0,
                    )
                },
                HC_ERR_NOT_UTF8
            );
        }

        #[test]
        fn the_gregorian_adoption_is_one_line_per_step() {
            let region = |code: &str| {
                read_lines(|buffer, capacity| unsafe {
                    hc_gregorian_adoption(code.as_ptr(), code.len(), buffer, capacity)
                })
            };
            let japan = region("JP");
            let rows: Vec<Vec<&str>> = japan
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].len(), 7);
            // 1872-12-31 was the Tenpō calendar's last day.
            assert_eq!(
                rows[0][..4],
                ["683734", "683735", "japanese-tenpo", "civil"]
            );
            assert!(rows[0][4].contains("337"), "{rows:?}");
            assert_eq!(rows[0][5..], ["gregory", "Japan"]);
            assert_eq!(region("se").lines().count(), 3);
            assert_eq!(
                unsafe { hc_gregorian_adoption("ZZ".as_ptr(), 2, core::ptr::null_mut(), 0) },
                0,
                "an unknown region writes nothing"
            );
        }

        #[test]
        fn every_calendar_and_every_locale_is_listed() {
            let today = 739_880;
            let text = read_lines(|buffer, capacity| unsafe {
                hc_calendars(today, "ja".as_ptr(), 2, buffer, capacity)
            });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert_eq!(rows.len(), hc::registry().len());
            assert!(rows.iter().all(|row| row.len() == 11), "{rows:?}");
            let gregorian = rows
                .iter()
                .find(|row| row[0] == "gregory")
                .expect("gregory");
            assert_eq!(gregorian[1..3], ["西暦(グレゴリオ暦)", "Gregorian"]);
            assert!(
                !gregorian[3].is_empty() && !gregorian[4].is_empty(),
                "{gregorian:?}"
            );
            assert_eq!(gregorian[5..11], ["0", "1", "1", "1", "", "in-use"]);
            let japanese = rows
                .iter()
                .find(|row| row[0] == "japanese")
                .expect("japanese");
            assert_eq!(japanese[1], "和暦");
            assert_eq!(japanese[5..10], ["1", "1", "1", "1", "ja"]);
            let chinese = rows
                .iter()
                .find(|row| row[0] == "chinese")
                .expect("chinese");
            assert_eq!(chinese[9], "zh-Hans;zh-Hant");
            let long_count = rows
                .iter()
                .find(|row| row[0] == "maya-longcount")
                .expect("long count");
            assert_eq!(long_count[5..10], ["0", "0", "0", "0", "yua"]);
            let rumi = rows.iter().find(|row| row[0] == "rumi").expect("rumi");
            assert!(
                ["in-use", "proleptic", "extended", "unrecorded"].contains(&rumi[10]),
                "{rumi:?}"
            );

            let text = read_lines(|buffer, capacity| unsafe { hc_locales(buffer, capacity) });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert_eq!(rows.len(), hc::hc_i18n::data::LOCALES.len());
            assert!(rows.iter().all(|row| row.len() == 7), "{rows:?}");
            let ja = rows.iter().find(|row| row[0] == "ja").expect("ja");
            assert_eq!(ja[1..6], ["Japanese", "日本語", "1", "1", "1"]);
            let named: Vec<&str> = ja[6].split(';').collect();
            assert!(
                named.contains(&"japanese") && named.contains(&"chinese"),
                "{named:?}"
            );
            assert!(!named.contains(&"gregory"), "{named:?}");
            let coptic = rows.iter().find(|row| row[0] == "cop").expect("cop");
            assert_eq!(coptic[3..6], ["0", "0", "0"]);
            assert_eq!(coptic[6], "coptic");
        }

        #[test]
        fn the_calendar_list_is_the_calendars_names_without_the_day() {
            let split = |text: &str| -> Vec<Vec<String>> {
                text.lines()
                    .map(|line| line.split('\t').map(str::to_owned).collect())
                    .collect()
            };
            for locale in ["ja", "en", "he", "und", "native"] {
                let list = split(&read_lines(|buffer, capacity| unsafe {
                    hc_calendar_list(locale.as_ptr(), locale.len(), buffer, capacity)
                }));
                let calendars = split(&read_lines(|buffer, capacity| unsafe {
                    hc_calendars(739_880, locale.as_ptr(), locale.len(), buffer, capacity)
                }));
                assert_eq!(list.len(), hc::registry().len(), "{locale}");
                assert!(list.iter().all(|row| row.len() == 6), "{list:?}");
                for (row, full) in list.iter().zip(&calendars) {
                    assert_eq!(row[..3], full[..3], "{locale}");
                    assert_eq!(row[5], full[9], "the native locales, {row:?}");
                    assert_eq!(row[1].is_empty(), row[3].is_empty(), "{row:?}");
                }
            }
            let list = split(&read_lines(|buffer, capacity| unsafe {
                hc_calendar_list("ja".as_ptr(), 2, buffer, capacity)
            }));
            let row = |id: &str| list.iter().find(|row| row[0] == id).expect(id).clone();
            assert_eq!(
                row("japanese")[1..],
                [
                    "和暦",
                    "Japanese (imperial eras)",
                    "ja",
                    "hc-calendars-regional",
                    "ja"
                ]
            );
            assert_eq!(row("gregory")[3..], ["ja", "hc-calendars-solar", ""]);
            assert_eq!(row("chinese")[5], "zh-Hans;zh-Hant");
            assert_eq!(row("chinese")[4], "hc-calendars-lunar");
            assert_eq!(row("persian")[4], "hc-calendars-equinox");
            assert_eq!(row("hindu-lunar")[4], "hc-calendars-indic");
            let native = split(&read_lines(|buffer, capacity| unsafe {
                hc_calendar_list("native".as_ptr(), 6, buffer, capacity)
            }));
            let hebrew = native
                .iter()
                .find(|row| row[0] == "hebrew")
                .expect("hebrew");
            assert_eq!(hebrew[1], "לוח השנה העברי");
            assert_eq!(hebrew[3], "he");
            let not_utf8 = [0xffu8];
            assert_eq!(
                unsafe { hc_calendar_list(not_utf8.as_ptr(), 1, core::ptr::null_mut(), 0) },
                HC_ERR_NOT_UTF8
            );
        }

        #[test]
        fn the_locale_argument_fails_as_text_does() {
            let not_utf8 = [0xffu8];
            assert_eq!(
                unsafe { hc_describe_day(739_880, not_utf8.as_ptr(), 1, core::ptr::null_mut(), 0) },
                HC_ERR_NOT_UTF8
            );
            assert_eq!(
                unsafe { hc_describe_day(739_880, core::ptr::null(), 3, core::ptr::null_mut(), 0) },
                HC_ERR_NULL_POINTER
            );
            // A null locale with no length is `und`.
            let needed =
                unsafe { hc_describe_day(739_880, core::ptr::null(), 0, core::ptr::null_mut(), 0) };
            assert!(needed > 0);
        }
    }

    #[cfg(feature = "holiday")]
    mod holiday {
        use super::super::*;
        use super::read_lines;

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
                unsafe {
                    hc_holiday_is_day_off(xnys.as_ptr(), 4, core::ptr::null(), 0, good_friday)
                },
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
            let text =
                unsafe { core::str::from_utf8(core::slice::from_raw_parts(pointer, capacity)) }
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
            let text =
                unsafe { core::str::from_utf8(core::slice::from_raw_parts(pointer, capacity)) }
                    .expect("UTF-8");
            assert!(
                text.contains("\nJP\n") && text.contains("\nXNYS\n") && text.contains("un-days\n")
            );
            unsafe { hc_free(pointer, capacity) };
        }

        #[test]
        fn one_day_across_every_table_decodes_column_by_column() {
            // 2026-05-06 is Japan's substitute for Constitution Memorial Day
            // (3 May, a Sunday), and Greenery Day is 4 May.
            let day = hc_gregorian_to_fixed(2026, 5, 6);
            let text =
                read_lines(|buffer, capacity| unsafe { hc_holidays_on(day, buffer, capacity) });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert!(rows.iter().all(|row| row.len() == 9), "{rows:?}");
            let japan: Vec<&Vec<&str>> = rows.iter().filter(|row| row[0] == "JP").collect();
            let substitute = japan
                .iter()
                .find(|row| row[7] == "1")
                .unwrap_or_else(|| panic!("no substitute in {japan:?}"));
            assert_eq!(substitute[1], "Japan");
            assert_eq!(substitute[2], "Constitution Memorial Day");
            assert_eq!(substitute[3], "憲法記念日");
            assert_eq!(substitute[4], "public");
            assert_eq!(substitute[5], "exact");
            assert_eq!(substitute[8], hc_gregorian_to_fixed(2026, 5, 3).to_string());
            // The tables come in the order `hc_holiday_codes` lists them:
            // an exchange's rows follow every country's.
            let codes: Vec<&str> = rows.iter().map(|row| row[0]).collect();
            let first_exchange = codes
                .iter()
                .position(|code| code.starts_with('X'))
                .expect("x");
            assert!(
                codes[..first_exchange].iter().all(|code| code.len() == 2),
                "{codes:?}"
            );
            // A gap on an ordinary day is a table whose announcement for
            // the year has not been read, and it is reported as such.
            let gap = rows.iter().find(|row| row[4] == "gap").expect("a gap");
            assert_eq!(gap[5..9], ["", "", "0", ""], "{gap:?}");
        }

        #[test]
        fn a_rule_that_cites_its_instrument_carries_it() {
            // World Braille Day, set by General Assembly resolution 73/161.
            let day = hc_gregorian_to_fixed(2026, 1, 4);
            let text =
                read_lines(|buffer, capacity| unsafe { hc_holidays_on(day, buffer, capacity) });
            let braille = text
                .lines()
                .map(|line| line.split('\t').collect::<Vec<&str>>())
                .find(|row| row[2] == "World Braille Day")
                .expect("the UN days are a table");
            assert_eq!(
                braille,
                [
                    "un-days",
                    "United Nations international days",
                    "World Braille Day",
                    "",
                    "observance",
                    "exact",
                    "A/RES/73/161",
                    "0",
                    ""
                ]
            );
        }

        #[test]
        fn a_year_a_table_cannot_answer_is_reported_as_gaps() {
            // 2150 is past the Chinese calendar's range, so the tables dated
            // in it report the lunisolar holidays as gaps rather than
            // showing an empty day.
            let day = hc_gregorian_to_fixed(2150, 2, 1);
            let text =
                read_lines(|buffer, capacity| unsafe { hc_holidays_on(day, buffer, capacity) });
            let gaps: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect::<Vec<&str>>())
                .filter(|row| row[4] == "gap")
                .collect();
            let chinese = gaps
                .iter()
                .find(|row| row[0] == "CN" && row[2] == "Spring Festival")
                .unwrap_or_else(|| panic!("no Chinese gap in {gaps:?}"));
            assert_eq!(
                chinese[..9],
                [
                    "CN",
                    "China",
                    "Spring Festival",
                    "春节",
                    "gap",
                    "",
                    "",
                    "0",
                    ""
                ]
            );
        }

        #[test]
        fn a_day_with_no_year_is_refused() {
            assert_eq!(
                unsafe { hc_holidays_on(i64::MAX, core::ptr::null_mut(), 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        #[test]
        fn one_day_across_every_table_is_what_the_years_say() {
            // The call evaluates each table for the day alone, through one
            // memo for all of them; the lines must be the ones the tables'
            // whole years give, each evaluated on its own.
            for (month, day) in [(1, 1), (2, 17), (9, 25), (11, 8)] {
                let fixed = hc_gregorian_to_fixed(2026, month, day);
                let text = read_lines(|buffer, capacity| unsafe {
                    hc_holidays_on(fixed, buffer, capacity)
                });
                let mut expected = String::new();
                for table in super::super::holiday::tables() {
                    let year = hc::hc_holiday::HolidayCalendar::for_year(table, None, 2026);
                    super::super::holiday::push_lines_on(
                        &mut expected,
                        table,
                        &year,
                        hc::hc_holiday::Rd(fixed),
                    );
                }
                assert_eq!(text, expected, "2026-{month:02}-{day:02}");
            }
        }

        /// A measurement, not a gate: one day across every table, as a page
        /// asks for it. Run with `cargo test --release -p hyper-calendar-wasm
        /// --features holiday -- --nocapture` to read the times; the
        /// README's figures are the `release-compact` profile's.
        #[test]
        fn one_day_across_every_table_is_timed() {
            let tables = super::super::holiday::codes().lines().count();
            for (month, day) in [(1, 1), (2, 17), (9, 25)] {
                let fixed = hc_gregorian_to_fixed(2026, month, day);
                let start = std::time::Instant::now();
                let needed = unsafe { hc_holidays_on(fixed, core::ptr::null_mut(), 0) };
                let elapsed = start.elapsed();
                assert!(needed > 0);
                eprintln!(
                    "hc_holidays_on for 2026-{month:02}-{day:02} across {tables} tables: {elapsed:?}"
                );
            }
        }
    }

    #[cfg(feature = "seasons")]
    mod seasons {
        use super::super::*;
        use super::read_lines;

        fn columns(text: &str) -> Vec<&str> {
            let line = text.strip_suffix('\n').expect("one line");
            assert!(!line.contains('\n'), "{text:?}");
            line.split('\t').collect()
        }

        #[test]
        fn the_term_in_effect_decodes_column_by_column() {
            // 2024-02-04 was 立春 in Japan; the term runs to 18 February.
            let day = hc_gregorian_to_fixed(2024, 2, 10);
            let text = read_lines(|buffer, capacity| unsafe {
                hc_term_in_effect(day, "japan".as_ptr(), 5, buffer, capacity)
            });
            let columns = columns(&text);
            assert_eq!(columns.len(), 7, "{columns:?}");
            assert_eq!(columns[..3], ["21", "立春", "立春"]);
            assert_eq!(columns[3], hc_gregorian_to_fixed(2024, 2, 4).to_string());
            assert_eq!(columns[4], hc_gregorian_to_fixed(2024, 2, 18).to_string());
            assert!(
                !columns[5].is_empty() && !columns[6].is_empty(),
                "{columns:?}"
            );
        }

        #[test]
        fn the_pentad_in_effect_decodes_column_by_column() {
            let day = hc_gregorian_to_fixed(2024, 2, 10);
            let text = read_lines(|buffer, capacity| unsafe {
                hc_pentad_in_effect(day, "japan".as_ptr(), 5, buffer, capacity)
            });
            let columns = columns(&text);
            assert_eq!(columns.len(), 7, "{columns:?}");
            // 立春 is pentads 63, 64 and 65 from 春分; 10 February is in the
            // second of them, 黄鶯睍睆 in Japan and 蟄蟲始振 in China.
            assert_eq!(columns[..3], ["64", "蟄蟲始振", "黄鶯睍睆"]);
            assert_eq!(columns[3], hc_gregorian_to_fixed(2024, 2, 9).to_string());
            assert_eq!(columns[4], hc_gregorian_to_fixed(2024, 2, 13).to_string());
            assert!(
                !columns[5].is_empty() && !columns[6].is_empty(),
                "{columns:?}"
            );
        }

        #[test]
        fn a_meridian_is_a_name_or_a_longitude() {
            let day = hc_gregorian_to_fixed(2024, 2, 4);
            let at = |name: &str| {
                columns(&read_lines(|buffer, capacity| unsafe {
                    hc_term_in_effect(day, name.as_ptr(), name.len(), buffer, capacity)
                }))[3]
                    .to_owned()
            };
            // 立春 2024 began at 08:27 UT on 4 February by this model: the
            // 4th from 120°W east to Tokyo, still the 3rd at 180°W.
            let fourth = day.to_string();
            let third = (day - 1).to_string();
            assert_eq!(at("japan"), fourth);
            assert_eq!(at("JAPAN"), fourth);
            assert_eq!(at("china"), fourth);
            assert_eq!(at("135"), fourth);
            assert_eq!(at("135.0"), fourth);
            assert_eq!(at("universal"), fourth);
            assert_eq!(at(""), fourth);
            assert_eq!(at("0"), fourth);
            assert_eq!(at("-120"), fourth);
            assert_eq!(at("-180"), third);
            for bad in ["mars", "181", "nan", "1e400"] {
                assert_eq!(
                    unsafe {
                        hc_term_in_effect(day, bad.as_ptr(), bad.len(), core::ptr::null_mut(), 0)
                    },
                    HC_ERR_UNKNOWN,
                    "{bad}"
                );
            }
            let not_utf8 = [0xffu8];
            assert_eq!(
                unsafe { hc_pentad_in_effect(day, not_utf8.as_ptr(), 1, core::ptr::null_mut(), 0) },
                HC_ERR_NOT_UTF8
            );
        }

        /// KASI's 월력요항: 한식 on 5 April 2024 and 6 April 2026.
        #[test]
        fn the_cold_food_day_crosses_the_boundary() {
            let day =
                |id: &str, year: i64| unsafe { hc_cold_food_day(id.as_ptr(), id.len(), year) };
            assert_eq!(day("hansik", 2024), hc_gregorian_to_fixed(2024, 4, 5));
            assert_eq!(day("Hansik", 2026), hc_gregorian_to_fixed(2026, 4, 6));
            let eve = day("hanshi-eve-of-qingming", 2026);
            let older = day("hanshi-solstice-105", 2026);
            assert!(matches!(older - eve, 1 | 2), "{eve} {older}");
            assert_eq!(day("hanshi", 2026), HC_ERR_UNKNOWN);
            assert_eq!(day("hansik", -1000), HC_ERR_OUT_OF_RANGE);
            assert_eq!(day("hansik", 3001), HC_ERR_OUT_OF_RANGE);
            assert!(day("hansik", -999) > HC_ERR_FLOOR);
            let not_utf8 = [0xffu8];
            assert_eq!(
                unsafe { hc_cold_food_day(not_utf8.as_ptr(), 1, 2026) },
                HC_ERR_NOT_UTF8
            );
        }

        /// The days of the years −1000 to 3000 answer, as `hc_sky_at`'s do;
        /// the days either side of them are refused rather than computed
        /// from a series stated for that era only.
        #[test]
        fn the_term_and_pentad_refuse_days_outside_the_era() {
            let first = hc_gregorian_to_fixed(-1000, 1, 1);
            let last = hc_gregorian_to_fixed(3000, 12, 31);
            let null = core::ptr::null_mut();
            for day in [first, last] {
                assert!(unsafe { hc_term_in_effect(day, "".as_ptr(), 0, null, 0) } > 0);
                assert!(unsafe { hc_pentad_in_effect(day, "".as_ptr(), 0, null, 0) } > 0);
            }
            for day in [first - 1, last + 1, i64::MIN, i64::MAX] {
                assert_eq!(
                    unsafe { hc_term_in_effect(day, "".as_ptr(), 0, null, 0) },
                    HC_ERR_OUT_OF_RANGE,
                    "{day}"
                );
                assert_eq!(
                    unsafe { hc_pentad_in_effect(day, "".as_ptr(), 0, null, 0) },
                    HC_ERR_OUT_OF_RANGE,
                    "{day}"
                );
            }
        }
    }

    #[cfg(feature = "tz")]
    mod tz {
        use super::super::*;

        const TZIF_V2_EASTERN: &[u8] = &[
            0x54, 0x5a, 0x69, 0x66, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
            0x00, 0x08, 0x65, 0xed, 0x5a, 0x70, 0x67, 0x27, 0x11, 0x60, 0x67, 0xcd, 0x3c, 0x70,
            0x69, 0x06, 0xf3, 0x60, 0x01, 0x00, 0x01, 0x00, 0xff, 0xff, 0xb9, 0xb0, 0x00, 0x00,
            0xff, 0xff, 0xc7, 0xc0, 0x01, 0x04, 0x45, 0x53, 0x54, 0x00, 0x45, 0x44, 0x54, 0x00,
            0x58, 0x68, 0x46, 0x80, 0x00, 0x00, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x00, 0x54, 0x5a,
            0x69, 0x66, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08,
            0x00, 0x00, 0x00, 0x00, 0x65, 0xed, 0x5a, 0x70, 0x00, 0x00, 0x00, 0x00, 0x67, 0x27,
            0x11, 0x60, 0x00, 0x00, 0x00, 0x00, 0x67, 0xcd, 0x3c, 0x70, 0x00, 0x00, 0x00, 0x00,
            0x69, 0x06, 0xf3, 0x60, 0x01, 0x00, 0x01, 0x00, 0xff, 0xff, 0xb9, 0xb0, 0x00, 0x00,
            0xff, 0xff, 0xc7, 0xc0, 0x01, 0x04, 0x45, 0x53, 0x54, 0x00, 0x45, 0x44, 0x54, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x58, 0x68, 0x46, 0x80, 0x00, 0x00, 0x00, 0x1b, 0x00, 0x00,
            0x00, 0x00, 0x0a, 0x45, 0x53, 0x54, 0x35, 0x45, 0x44, 0x54, 0x2c, 0x4d, 0x33, 0x2e,
            0x32, 0x2e, 0x30, 0x2c, 0x4d, 0x31, 0x31, 0x2e, 0x31, 0x2e, 0x30, 0x0a,
        ];

        fn in_zone(unix: i64, zone: &str) -> i64 {
            unsafe { hc_fixed_from_unix_in_zone(unix, zone.as_ptr(), zone.len()) }
        }

        fn starts(fixed: i64, zone: &str) -> i64 {
            unsafe { hc_unix_from_fixed_in_zone(fixed, zone.as_ptr(), zone.len()) }
        }

        #[test]
        fn a_days_start_never_reads_as_a_sentinel() {
            // By UTC the range is hc_unix_from_fixed's.
            let first = -104_165_947_503;
            let midnight = -8_999_999_999_942_400;
            assert_eq!(starts(first, "UTC"), midnight);
            assert_eq!(starts(first - 1, "UTC"), HC_ERR_OUT_OF_RANGE);
            // An offset moves the start, not far enough here to move the
            // first day.
            assert_eq!(starts(first, "Asia/Tokyo"), midnight - 9 * 3_600);
            assert_eq!(starts(first - 1, "Asia/Tokyo"), HC_ERR_OUT_OF_RANGE);
            assert_eq!(starts(first, "America/Sao_Paulo"), midnight + 3 * 3_600);
            assert_eq!(starts(first - 1, "America/Sao_Paulo"), HC_ERR_OUT_OF_RANGE);
            // Some 54 billion years back.
            assert_eq!(
                starts(-19_723_095_000_000, "Asia/Tokyo"),
                HC_ERR_OUT_OF_RANGE
            );
            // Overflow downwards.
            assert_eq!(starts(i64::MIN, "UTC"), HC_ERR_OUT_OF_RANGE);
            assert_eq!(starts(i64::MIN, "America/New_York"), HC_ERR_OUT_OF_RANGE);
        }

        #[test]
        fn a_days_start_refuses_rather_than_saturates() {
            let last = 106_751_991_886_463;
            let midnight = 9_223_372_036_854_720_000;
            assert_eq!(starts(last, "UTC"), midnight);
            assert_eq!(starts(last + 1, "UTC"), HC_ERR_OUT_OF_RANGE);
            // West of Greenwich the day begins later, and UTC's last day
            // is the last; Tokyo's day after it begins nine hours before
            // UTC's, which still fits.
            assert_eq!(starts(last, "America/Sao_Paulo"), midnight + 3 * 3_600);
            assert_eq!(starts(last + 1, "America/Sao_Paulo"), HC_ERR_OUT_OF_RANGE);
            assert_eq!(
                starts(last + 1, "Asia/Tokyo"),
                midnight - 9 * 3_600 + 86_400
            );
            assert_eq!(starts(last + 2, "Asia/Tokyo"), HC_ERR_OUT_OF_RANGE);
            // 2^53 days, whose start a saturating product would clamp to
            // i64::MAX.
            assert_eq!(starts(1 << 53, "Asia/Tokyo"), HC_ERR_OUT_OF_RANGE);
            assert_eq!(starts(i64::MAX, "UTC"), HC_ERR_OUT_OF_RANGE);
            assert_eq!(starts(i64::MAX, "America/New_York"), HC_ERR_OUT_OF_RANGE);
        }

        #[test]
        fn every_timestamp_has_a_zoned_day_above_the_floor() {
            for zone in ["UTC", "Asia/Tokyo", "America/New_York"] {
                assert!(in_zone(i64::MIN, zone) > HC_ERR_FLOOR, "{zone}");
                assert!(in_zone(i64::MAX, zone) > HC_ERR_FLOOR, "{zone}");
            }
        }

        #[test]
        fn the_readers_day_is_the_zones_day_not_utcs() {
            // 08:00 on 25 September 2026 in Tokyo is 23:00 UTC on the 24th.
            let september_24 = hc_gregorian_to_fixed(2026, 9, 24);
            let instant = hc_unix_from_fixed(september_24) + 23 * 3_600;
            assert_eq!(hc_fixed_from_unix(instant), september_24);
            assert_eq!(in_zone(instant, "Asia/Tokyo"), september_24 + 1);
            assert_eq!(in_zone(instant, "asia/tokyo"), september_24 + 1);
            assert_eq!(in_zone(instant, "UTC"), september_24);
            // And the Tokyo day begins nine hours before the UTC one.
            assert_eq!(
                starts(september_24 + 1, "Asia/Tokyo"),
                hc_unix_from_fixed(september_24 + 1) - 9 * 3_600
            );
        }

        #[test]
        fn a_day_that_begins_in_a_gap_begins_after_it() {
            // New York's clocks go forward at 02:00 on 8 March 2026: the
            // day begins at its ordinary midnight, 05:00 UTC, and 03:00 EDT
            // is 07:00 UTC, still the 8th.
            let march_8 = hc_gregorian_to_fixed(2026, 3, 8);
            let midnight_utc = hc_unix_from_fixed(march_8);
            assert_eq!(
                starts(march_8, "America/New_York"),
                midnight_utc + 5 * 3_600
            );
            assert_eq!(
                in_zone(midnight_utc + 7 * 3_600, "America/New_York"),
                march_8
            );
            assert_eq!(
                in_zone(midnight_utc + 4 * 3_600, "America/New_York"),
                march_8 - 1
            );
            // Cairo's clocks go forward at 00:00 on the last Friday of
            // April, so 24 April 2026 has no midnight: it begins at 01:00
            // EEST, which is 22:00 UTC on the 23rd.
            let april_24 = hc_gregorian_to_fixed(2026, 4, 24);
            assert_eq!(
                starts(april_24, "Africa/Cairo"),
                hc_unix_from_fixed(april_24) - 2 * 3_600
            );
            assert_eq!(
                starts(april_24 + 1, "Africa/Cairo"),
                hc_unix_from_fixed(april_24 + 1) - 3 * 3_600
            );
        }

        #[test]
        fn unknown_zones_and_bad_names_are_sentinels() {
            assert_eq!(in_zone(0, "Mars/Olympus"), HC_ERR_UNKNOWN);
            assert_eq!(starts(0, ""), HC_ERR_UNKNOWN);
            let not_utf8 = [0xffu8];
            assert_eq!(
                unsafe { hc_fixed_from_unix_in_zone(0, not_utf8.as_ptr(), 1) },
                HC_ERR_NOT_UTF8
            );
            assert_eq!(
                unsafe { hc_unix_from_fixed_in_zone(0, core::ptr::null(), 3) },
                HC_ERR_NULL_POINTER
            );
        }

        #[test]
        fn a_loaded_zone_answers_by_name_and_outranks_the_builtin() {
            let name = "Test/Eastern";
            assert_eq!(in_zone(0, name), HC_ERR_UNKNOWN);
            assert_eq!(
                unsafe {
                    hc_zone_load(
                        name.as_ptr(),
                        name.len(),
                        TZIF_V2_EASTERN.as_ptr(),
                        TZIF_V2_EASTERN.len(),
                    )
                },
                0
            );
            // 2025-03-09 07:00 UTC is 03:00 EDT, just after the gap.
            let march_9 = hc_gregorian_to_fixed(2025, 3, 9);
            let midnight_utc = hc_unix_from_fixed(march_9);
            assert_eq!(in_zone(midnight_utc + 7 * 3_600, name), march_9);
            assert_eq!(in_zone(midnight_utc + 4 * 3_600, name), march_9 - 1);
            assert_eq!(starts(march_9, name), midnight_utc + 5 * 3_600);
            // The same bytes under a built-in name take precedence over it.
            let builtin = "America/Los_Angeles";
            assert_eq!(starts(march_9, builtin), midnight_utc + 8 * 3_600);
            assert_eq!(
                unsafe {
                    hc_zone_load(
                        builtin.as_ptr(),
                        builtin.len(),
                        TZIF_V2_EASTERN.as_ptr(),
                        TZIF_V2_EASTERN.len(),
                    )
                },
                0
            );
            assert_eq!(starts(march_9, builtin), midnight_utc + 5 * 3_600);
            // Bytes that are not TZif are refused and nothing is kept.
            let junk = b"not a zone";
            let other = "Test/Junk";
            assert_eq!(
                unsafe { hc_zone_load(other.as_ptr(), other.len(), junk.as_ptr(), junk.len()) },
                HC_ERR_MALFORMED
            );
            assert_eq!(in_zone(0, other), HC_ERR_UNKNOWN);
            assert_eq!(
                unsafe { hc_zone_load(core::ptr::null(), 0, junk.as_ptr(), junk.len()) },
                HC_ERR_UNKNOWN
            );
        }
    }

    #[cfg(feature = "tz")]
    mod zones {
        use super::super::*;
        use super::read_lines;

        fn location(zone: &str, locale: &str) -> i64 {
            unsafe {
                hc_zone_location(
                    zone.as_ptr(),
                    zone.len(),
                    locale.as_ptr(),
                    locale.len(),
                    core::ptr::null_mut(),
                    0,
                )
            }
        }

        fn line(zone: &str, locale: &str) -> String {
            read_lines(|buffer, capacity| unsafe {
                hc_zone_location(
                    zone.as_ptr(),
                    zone.len(),
                    locale.as_ptr(),
                    locale.len(),
                    buffer,
                    capacity,
                )
            })
        }

        /// `zone1970.tab` 2026c: `JP,AU +353916+1394441 Asia/Tokyo Eyre
        /// Bird Observatory`, 128 356″ and 503 081″.
        #[test]
        fn every_zone_has_a_line_and_tokyo_is_its_row() {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_zones("en".as_ptr(), 2, buffer, capacity)
            });
            assert_eq!(text.lines().count(), 312);
            assert!(text.lines().all(|line| line.split('\t').count() == 8));
            let tokyo = text
                .lines()
                .find(|line| line.starts_with("Asia/Tokyo\t"))
                .expect("Tokyo");
            assert_eq!(
                tokyo,
                "Asia/Tokyo\t35.654444\t139.744722\tJP;AU\tJP\tEyre Bird Observatory\tTokyo\ten"
            );
            assert_eq!(line("asia/tokyo", "en"), format!("{tokyo}\n"));
        }

        /// `zone.tab`: `NO +5955+01045 Europe/Oslo`; `backward`: `Link
        /// Asia/Kolkata Asia/Calcutta` and `Link Etc/UTC UTC`.
        #[test]
        fn links_answer_with_their_rows_and_utc_is_unknown() {
            assert!(
                line("Europe/Oslo", "en")
                    .starts_with("Europe/Oslo\t59.916667\t10.750000\tNO\tNO\t\t")
            );
            assert!(line("Asia/Calcutta", "en").starts_with("Asia/Kolkata\t"));
            assert_eq!(location("UTC", "en"), HC_ERR_UNKNOWN);
            let not_utf8 = [0xffu8];
            assert_eq!(
                unsafe {
                    hc_zone_location(
                        not_utf8.as_ptr(),
                        1,
                        "en".as_ptr(),
                        2,
                        core::ptr::null_mut(),
                        0,
                    )
                },
                HC_ERR_NOT_UTF8
            );
            assert_eq!(
                unsafe { hc_zones(core::ptr::null(), 1, core::ptr::null_mut(), 0) },
                HC_ERR_NULL_POINTER
            );
        }

        /// CLDR 48 `ja.xml`: `Asia/Tokyo` 東京. The city is localised only
        /// in a build that carries `calendars` too.
        #[test]
        fn the_city_is_in_the_locale_where_the_build_carries_it() {
            let tokyo = line("Asia/Tokyo", "ja-JP");
            let cells: Vec<&str> = tokyo.trim_end().split('\t').collect();
            if cfg!(feature = "calendars") {
                assert_eq!(cells[6..], ["東京", "ja"]);
            } else {
                assert_eq!(cells[6..], ["Tokyo", "en"]);
            }
        }
    }
    #[cfg(feature = "deep-time")]
    mod deep_time {
        use super::super::*;
        use super::read_lines;

        fn cells(text: &str) -> Vec<Vec<&str>> {
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert!(rows.iter().all(|row| row.len() == 16), "{rows:?}");
            rows
        }

        #[test]
        fn a_moment_is_placed_in_every_chronology() {
            // The end-Cretaceous extinction, 66 million years ago.
            let text = read_lines(|buffer, capacity| unsafe {
                hc_place_years_ago(66.0e6, 0.0, "ja".as_ptr(), 2, buffer, capacity)
            });
            let rows = cells(&text);
            let kinds: Vec<&str> = rows.iter().map(|row| row[0]).collect();
            assert_eq!(
                kinds,
                [
                    "moment",
                    "moment",
                    "cosmic-epoch",
                    "cosmic-event",
                    "eon",
                    "era",
                    "period",
                    "epoch",
                    "age"
                ]
            );
            assert_eq!(rows[0][2], "since-big-bang");
            assert_eq!(rows[0][12], "seconds-since-big-bang");
            assert_eq!(rows[1][2], "before-present");
            assert_eq!(rows[1][12], "seconds-before-present");
            assert_eq!(rows[2][1..3], ["era-of-galaxies", "Era of galaxies"]);
            let age = &rows[8];
            assert_eq!(
                age[..4],
                ["age", "maastrichtian", "Maastrichtian", "upper-cretaceous"]
            );
            // The older bound, then the younger, each with its own figures.
            assert_eq!(age[4..8], ["72.2", "0.2", "3", "0"]);
            assert_eq!(age[8..12], ["66", "0", "4", "0"]);
            assert_eq!(age[12], "megayears-before-present");
            assert!(age[14].contains("v2026/06"), "{age:?}");
            // The chart's Japanese name, from the Geological Society of
            // Japan's translation; the cosmic rows have none.
            assert_eq!(age[15], "マーストリヒチアン");
            assert_eq!(rows[6][15], "白亜系／紀");
            assert_eq!(rows[2][15], "");
        }

        #[test]
        fn the_present_and_the_future_are_placed_too() {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_place_years_ago(0.0, 0.0, core::ptr::null(), 0, buffer, capacity)
            });
            let rows = cells(&text);
            let archaeological = rows
                .iter()
                .find(|row| row[0] == "archaeological")
                .expect("now");
            assert_eq!(archaeological[2], "Modern period");
            assert_eq!(archaeological[12], "years-before-1950");
            assert!(!archaeological[14].is_empty());
            // Eight billion years ahead the Sun is a red giant and the
            // cosmic history tables have ended.
            let text = read_lines(|buffer, capacity| unsafe {
                hc_place_years_ago(-8.0e9, 0.0, core::ptr::null(), 0, buffer, capacity)
            });
            let rows = cells(&text);
            let kinds: Vec<&str> = rows.iter().map(|row| row[0]).collect();
            // The present day is the last dated event before any future
            // moment, so it is still listed.
            assert_eq!(kinds, ["moment", "moment", "cosmic-event", "future-era"]);
            assert_eq!(rows[2][2], "The present");
            assert_eq!(rows[3][2], "Stelliferous Era");
            assert_eq!(rows[3][12], "log10-years-from-now");
        }

        #[test]
        fn the_near_future_keeps_the_whole_chain() {
            // Six hours and three years ahead: still the Meghalayan, the
            // Modern period and the era of galaxies, and the future era too.
            for years_ago in [-6.0 / (24.0 * 365.25), -3.0] {
                let text = read_lines(|buffer, capacity| unsafe {
                    hc_place_years_ago(years_ago, 0.0, "en".as_ptr(), 2, buffer, capacity)
                });
                let rows = cells(&text);
                let kinds: Vec<&str> = rows.iter().map(|row| row[0]).collect();
                assert_eq!(
                    kinds,
                    [
                        "moment",
                        "moment",
                        "cosmic-epoch",
                        "cosmic-event",
                        "future-era",
                        "eon",
                        "era",
                        "period",
                        "epoch",
                        "age",
                        "archaeological"
                    ],
                    "{years_ago}"
                );
                assert_eq!(rows[2][2], "Era of galaxies");
                assert_eq!(rows[9][2], "Meghalayan");
                assert_eq!(rows[10][2], "Modern period");
            }
            // Beyond a century ahead the future begins.
            let text = read_lines(|buffer, capacity| unsafe {
                hc_place_years_ago(-101.0, 0.0, core::ptr::null(), 0, buffer, capacity)
            });
            let kinds: Vec<Vec<&str>> = cells(&text);
            let kinds: Vec<&str> = kinds.iter().map(|row| row[0]).collect();
            assert_eq!(kinds, ["moment", "moment", "cosmic-event", "future-era"]);
        }

        #[test]
        fn a_value_the_crate_refuses_is_a_sentinel() {
            assert_eq!(
                unsafe {
                    hc_place_years_ago(
                        f64::NAN,
                        0.0,
                        core::ptr::null(),
                        0,
                        core::ptr::null_mut(),
                        0,
                    )
                },
                HC_ERR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe {
                    hc_place_years_ago(1.0, -1.0, core::ptr::null(), 0, core::ptr::null_mut(), 0)
                },
                HC_ERR_OUT_OF_RANGE
            );
            let not_utf8 = [0xff_u8];
            assert_eq!(
                unsafe {
                    hc_place_years_ago(0.0, 0.0, not_utf8.as_ptr(), 1, core::ptr::null_mut(), 0)
                },
                HC_ERR_NOT_UTF8
            );
        }

        #[test]
        fn the_cosmic_tables_are_listed_with_their_sources() {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_cosmic_events("ja".as_ptr(), 2, buffer, capacity)
            });
            let rows = cells(&text);
            let epochs = rows.iter().filter(|row| row[0] == "cosmic-epoch").count();
            let events = rows.iter().filter(|row| row[0] == "cosmic-event").count();
            assert_eq!(epochs, hc::hc_deep_time::universe::EPOCHS.len());
            assert_eq!(events, hc::hc_deep_time::universe::EVENTS.len());
            assert!(
                rows.iter().all(|row| !row[14].is_empty()),
                "every row cites"
            );
            assert_eq!(rows.len(), epochs + events);
            // Named in Japanese where an established term was read.
            let named = |id: &str| rows.iter().find(|row| row[1] == id).map(|row| row[15]);
            assert_eq!(named("planck-epoch"), Some("プランク時代"));
            assert_eq!(named("era-of-galaxies"), Some(""));
            // A minimum age has no older bound and a stated sigma.
            let planck = &rows[0];
            assert_eq!(planck[2], "Planck epoch");
            assert_eq!(planck[4], "0");
            // Values are written in plain decimal notation, however small.
            let end: f64 = planck[8].parse().expect("a number");
            assert!(end > 0.0 && end < 1e-40, "{planck:?}");
            assert!(!planck[8].contains('e'), "{planck:?}");
        }

        #[test]
        fn the_earliest_evidence_the_periods_and_the_future_have_lists_of_their_own() {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_earliest_evidence("ja".as_ptr(), 2, buffer, capacity)
            });
            let rows = cells(&text);
            assert_eq!(rows.len(), hc::hc_deep_time::evidence::EVIDENCE.len());
            assert!(rows.iter().all(|row| row[0] == "earliest-evidence"));
            let named = |id: &str| rows.iter().find(|row| row[1] == id).map(|row| row[15]);
            assert_eq!(named("earliest-writing-uruk-iv"), Some("原楔形文字"));
            // A minimum age has no older bound and a stated sigma.
            let omo = rows
                .iter()
                .find(|row| row[1] == "earliest-homo-sapiens-omo-kibish")
                .expect("listed");
            assert_eq!(omo[3], "earliest-homo-sapiens");
            assert_eq!(omo[4..9], ["", "", "", "", "233000"]);
            assert_eq!(omo[9], "11000");
            let text = read_lines(|buffer, capacity| unsafe {
                hc_archaeological_periods(core::ptr::null(), 0, buffer, capacity)
            });
            let rows = cells(&text);
            assert_eq!(rows.len(), hc::hc_deep_time::archaeology::PERIODS.len());
            assert_eq!(
                rows[0][..3],
                ["archaeological", "modern-period", "Modern period"]
            );
            let text = read_lines(|buffer, capacity| unsafe {
                hc_future_events(core::ptr::null(), 0, buffer, capacity)
            });
            let rows = cells(&text);
            assert_eq!(rows.len(), hc::hc_deep_time::future::EVENTS.len());
            let proton = rows
                .iter()
                .find(|row| row[1] == "proton-decay-lower-bound")
                .expect("listed");
            assert_eq!(proton[3], "experimental-bound");
            assert_eq!(proton[8], "", "a bound has no end");
            assert_eq!(proton[12], "years-from-now");
            let not_utf8 = [0xff_u8];
            for export in [
                hc_earliest_evidence,
                hc_archaeological_periods,
                hc_future_events,
            ] {
                assert_eq!(
                    unsafe { export(not_utf8.as_ptr(), 1, core::ptr::null_mut(), 0) },
                    HC_ERR_NOT_UTF8
                );
            }
        }

        #[test]
        fn the_geologic_ranks_are_numbered_coarsest_first() {
            for (number, expected) in [
                (0, "eon"),
                (1, "era"),
                (2, "period"),
                (3, "epoch"),
                (4, "age"),
            ] {
                let text = read_lines(|buffer, capacity| unsafe {
                    hc_geologic_intervals(number, "zh-Hans".as_ptr(), 7, buffer, capacity)
                });
                let rows = cells(&text);
                assert!(
                    rows.iter().all(|row| row[0] == expected),
                    "{number}: {rows:?}"
                );
            }
            let eons = read_lines(|buffer, capacity| unsafe {
                hc_geologic_intervals(0, core::ptr::null(), 0, buffer, capacity)
            });
            assert!(
                eons.starts_with("eon\tphanerozoic\tPhanerozoic\t\t538.8\t0.6\t4\t0\t0\t0\t1\t0\t"),
                "{eons}"
            );
            assert!(cells(&eons).iter().all(|row| row[15].is_empty()));
            let eons = read_lines(|buffer, capacity| unsafe {
                hc_geologic_intervals(0, "zh-Hans".as_ptr(), 7, buffer, capacity)
            });
            assert_eq!(cells(&eons)[0][15], "显生宇");
            assert_eq!(
                unsafe { hc_geologic_intervals(5, core::ptr::null(), 0, core::ptr::null_mut(), 0) },
                HC_ERR_UNKNOWN
            );
        }
    }

    #[cfg(feature = "sky")]
    mod sky {
        use super::super::*;
        use super::read_lines;

        /// At the NAOJ's 秋分 of 2026, 23 September 00:05 UTC, the Sun enters
        /// Libra, whose first face al-Bīrūnī gives to the Moon.
        #[test]
        fn the_sun_enters_the_moons_face_of_libra_at_the_equinox() {
            let instant = at(2026, 9, 23, 1, 5);
            let text =
                read_lines(|buffer, capacity| unsafe { hc_decan_at(instant, buffer, capacity) });
            assert!(text.starts_with("7\tLibra\t1\tmoon\tMoon\t0.0"), "{text}");
            assert_eq!(
                unsafe { hc_decan_at(i64::MIN, core::ptr::null_mut(), 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        /// The POSIX timestamp of a UTC date and time.
        fn at(year: i64, month: u32, day: u32, hour: i64, minute: i64) -> i64 {
            hc_unix_from_fixed(hc_gregorian_to_fixed(year, month, day)) + hour * 3_600 + minute * 60
        }

        fn sky(unix: i64) -> Vec<String> {
            let text = read_lines(|buffer, capacity| unsafe { hc_sky_at(unix, buffer, capacity) });
            let line = text.strip_suffix('\n').expect("one line");
            assert!(!line.contains('\n'), "{text:?}");
            line.split('\t').map(str::to_owned).collect()
        }

        fn rows(text: &str) -> Vec<Vec<String>> {
            text.lines()
                .map(|line| line.split('\t').map(str::to_owned).collect())
                .collect()
        }

        fn terms(from: i64, to: i64) -> Vec<Vec<String>> {
            rows(&read_lines(|buffer, capacity| unsafe {
                hc_solar_terms_between(from, to, buffer, capacity)
            }))
        }

        fn phases(from: i64, to: i64) -> Vec<Vec<String>> {
            rows(&read_lines(|buffer, capacity| unsafe {
                hc_moon_phases_between(from, to, buffer, capacity)
            }))
        }

        fn number(cell: &str) -> f64 {
            cell.parse()
                .unwrap_or_else(|_| panic!("not a number: {cell}"))
        }

        /// The 暦要項 of the National Astronomical Observatory of Japan for
        /// 2026 puts the new moon (朔) of September at 11 September 12:27
        /// JST, which is 03:27 UTC, and the equinox (秋分) at 23 September
        /// 09:05 JST, 00:05 UTC; both are printed to the minute, and the
        /// Hong Kong Observatory's tables round the same conjunction to
        /// 03:26 UTC.
        #[test]
        fn the_sky_before_the_new_moon_of_september_2026_decodes_column_by_column() {
            let columns = sky(at(2026, 9, 11, 3, 0));
            assert_eq!(columns.len(), 12, "{columns:?}");
            // Twelve days before the equinox the Sun is about 168° along.
            let sun = number(&columns[0]);
            assert!((167.0..170.0).contains(&sun), "{sun}");
            let distance = number(&columns[1]);
            assert!((1.0..1.02).contains(&distance), "{distance} au");
            // Half an hour before conjunction the Moon is just behind the
            // Sun, within six degrees of the ecliptic, and dark.
            let moon = number(&columns[2]);
            assert!((sun - moon).abs() < 1.0, "sun {sun}, moon {moon}");
            assert!(number(&columns[3]).abs() < 5.5, "{columns:?}");
            let kilometres = number(&columns[4]);
            assert!((355_000.0..407_000.0).contains(&kilometres), "{kilometres}");
            let elongation = number(&columns[5]);
            assert!(elongation > 359.0, "{elongation}");
            assert!(number(&columns[6]) < 0.001, "{columns:?}");
            let previous: i64 = columns[7].parse().expect("a timestamp");
            let next: i64 = columns[8].parse().expect("a timestamp");
            let published = at(2026, 9, 11, 3, 27);
            assert!((next - published).abs() <= 90, "next new moon {next}");
            assert!(
                (29..30).contains(&((next - previous) / 86_400)),
                "{columns:?}"
            );
            // ΔT in September 2026 comes from the USNO's predictions.
            let delta_t = number(&columns[9]);
            assert!((69.0..69.5).contains(&delta_t), "{delta_t}");
            assert_eq!(columns[10], "predicted");
            assert!(columns[11].contains("VSOP87"), "{}", columns[11]);
            assert!(columns[11].contains("deltat.preds"), "{}", columns[11]);
        }

        #[test]
        fn the_terms_and_phases_of_september_2026_are_where_the_almanacs_put_them() {
            let (from, to) = (at(2026, 9, 1, 0, 0), at(2026, 10, 1, 0, 0));
            let terms = terms(from, to);
            assert_eq!(terms.len(), 2, "{terms:?}");
            assert!(terms.iter().all(|row| row.len() == 4), "{terms:?}");
            assert_eq!(terms[0][0], "165");
            assert_eq!(terms[0][2..], ["白露", "白露"]);
            let equinox = &terms[1];
            assert_eq!(equinox[0], "180");
            assert_eq!(equinox[2..], ["秋分", "秋分"]);
            let instant: i64 = equinox[1].parse().expect("a timestamp");
            let published = at(2026, 9, 23, 0, 5);
            assert!((instant - published).abs() <= 90, "equinox at {instant}");

            let phases = phases(from, to);
            assert_eq!(phases.len(), 4, "{phases:?}");
            assert!(phases.iter().all(|row| row.len() == 4), "{phases:?}");
            let kinds: Vec<(&str, &str)> = phases
                .iter()
                .map(|row| (row[0].as_str(), row[2].as_str()))
                .collect();
            // 下弦 on the 4th, 朔 on the 11th, 上弦 on the 19th (JST) and
            // 望 on the 27th.
            assert_eq!(
                kinds,
                [
                    ("270", "last-quarter"),
                    ("0", "new"),
                    ("90", "first-quarter"),
                    ("180", "full")
                ]
            );
            assert!(phases.iter().all(|row| row[3].is_empty()), "{phases:?}");
            let new_moon: i64 = phases[1][1].parse().expect("a timestamp");
            assert!(
                (new_moon - at(2026, 9, 11, 3, 27)).abs() <= 90,
                "{new_moon}"
            );
            // 望 at 27 September 01:49 JST, 16:49 UTC on the 26th.
            let full_moon: i64 = phases[3][1].parse().expect("a timestamp");
            assert!(
                (full_moon - at(2026, 9, 26, 16, 49)).abs() <= 90,
                "{full_moon}"
            );
            // The new moons the list gives are the ones `hc_sky_at` gives.
            let before = sky(new_moon - 60);
            assert_eq!(before[8], phases[1][1]);
            let after = sky(new_moon + 60);
            assert_eq!(after[7], phases[1][1]);
        }

        /// The Chinese calendar of 2023 had a leap second month (閏二月)
        /// because the lunation that began with the new moon of 21 March
        /// 2023 17:23 UTC and ended with that of 20 April 04:12 UTC held no
        /// principal term (中気): 春分 fell earlier on the 21st and 穀雨
        /// later on the 20th, leaving only the sectional 清明.
        #[test]
        fn the_lunation_of_march_2023_holds_no_principal_term() {
            let first = sky(at(2023, 3, 21, 0, 0));
            let new_moon: i64 = first[8].parse().expect("a timestamp");
            assert!(
                (new_moon - at(2023, 3, 21, 17, 23)).abs() <= 90,
                "{new_moon}"
            );
            let second = sky(new_moon + 1);
            let next_new_moon: i64 = second[8].parse().expect("a timestamp");
            assert!(
                (next_new_moon - at(2023, 4, 20, 4, 12)).abs() <= 90,
                "{next_new_moon}"
            );
            let terms = terms(new_moon, next_new_moon);
            assert_eq!(terms.len(), 1, "{terms:?}");
            assert_eq!(terms[0][0], "15");
            assert_eq!(terms[0][2..], ["清明", "清明"]);
            assert!(
                terms.iter().all(|row| number(&row[0]) % 30.0 != 0.0),
                "a principal term in {terms:?}"
            );
        }

        #[test]
        fn the_moon_is_lit_at_a_full_moon() {
            let phases = phases(at(2026, 1, 1, 0, 0), at(2027, 1, 1, 0, 0));
            let full: Vec<i64> = phases
                .iter()
                .filter(|row| row[2] == "full")
                .map(|row| row[1].parse().expect("a timestamp"))
                .collect();
            // 2026 has thirteen full moons: two in May.
            assert_eq!(full.len(), 13, "{phases:?}");
            for instant in full {
                let columns = sky(instant);
                let fraction = number(&columns[6]);
                assert!(fraction > 0.99, "{fraction} at {instant}");
                let elongation = number(&columns[5]);
                assert!((elongation - 180.0).abs() < 0.5, "{elongation}");
            }
        }

        #[test]
        fn instants_outside_the_era_and_spans_too_long_are_refused() {
            let null = core::ptr::null_mut();
            assert!(unsafe { hc_sky_at(at(3000, 12, 31, 23, 59), null, 0) } > 0);
            assert!(unsafe { hc_sky_at(at(-1000, 1, 1, 0, 0), null, 0) } > 0);
            for instant in [
                at(3001, 1, 1, 0, 0),
                at(-1000, 1, 1, 0, 0) - 1,
                i64::MAX,
                i64::MIN,
            ] {
                assert_eq!(
                    unsafe { hc_sky_at(instant, null, 0) },
                    HC_ERR_OUT_OF_RANGE,
                    "{instant}"
                );
                assert_eq!(
                    unsafe { hc_solar_terms_between(instant, instant.saturating_add(1), null, 0) },
                    HC_ERR_OUT_OF_RANGE
                );
            }
            for instant in [at(3001, 1, 1, 0, 1), i64::MAX] {
                assert_eq!(
                    unsafe { hc_moon_phases_between(at(2000, 1, 1, 0, 0), instant, null, 0) },
                    HC_ERR_OUT_OF_RANGE
                );
            }
            // A span may end at the era's end, but not run past it.
            let end = at(3001, 1, 1, 0, 0);
            assert!(unsafe { hc_solar_terms_between(end - 86_400 * 40, end, null, 0) } > 0);
            assert_eq!(
                unsafe { hc_solar_terms_between(end - 86_400 * 40, end + 1, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            // Four hundred years is the most a call answers for.
            let from = at(1600, 1, 1, 0, 0);
            assert_eq!(
                unsafe { hc_moon_phases_between(from, at(2001, 1, 1, 0, 0), null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_solar_terms_between(from, at(2001, 1, 1, 0, 0), null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            // An empty or inverted span is an empty answer, not an error.
            let instant = at(2026, 9, 11, 3, 0);
            assert_eq!(
                unsafe { hc_solar_terms_between(instant, instant, null, 0) },
                0
            );
            assert_eq!(
                unsafe { hc_moon_phases_between(instant, instant - 1, null, 0) },
                0
            );
        }

        #[test]
        fn a_decade_of_terms_and_phases_is_complete_and_ordered() {
            let (from, to) = (at(2000, 1, 1, 0, 0), at(2010, 1, 1, 0, 0));
            let terms = terms(from, to);
            assert_eq!(terms.len(), 240, "{}", terms.len());
            let phases = phases(from, to);
            // 123 or 124 lunations of four phases each.
            assert!((492..=496).contains(&phases.len()), "{}", phases.len());
            for rows in [&terms, &phases] {
                let instants: Vec<i64> = rows
                    .iter()
                    .map(|row| row[1].parse().expect("a timestamp"))
                    .collect();
                assert!(instants.windows(2).all(|pair| pair[0] < pair[1]));
                assert!(
                    instants
                        .iter()
                        .all(|&instant| from <= instant && instant < to)
                );
            }
            // Term angles step by 15° and phase angles by 90°, wrapping.
            for pair in terms.windows(2) {
                let step = (number(&pair[1][0]) - number(&pair[0][0])).rem_euclid(360.0);
                assert!((step - 15.0).abs() < 1e-9, "{pair:?}");
            }
            for pair in phases.windows(2) {
                let step = (number(&pair[1][0]) - number(&pair[0][0])).rem_euclid(360.0);
                assert!((step - 90.0).abs() < 1e-9, "{pair:?}");
            }
        }

        #[test]
        fn the_sky_measures_and_refuses_a_short_buffer_like_every_line_export() {
            let instant = at(2026, 9, 11, 3, 0);
            let needed = unsafe { hc_sky_at(instant, core::ptr::null_mut(), 0) };
            assert!(needed > 0);
            let mut small = [7u8; 8];
            assert_eq!(
                unsafe { hc_sky_at(instant, small.as_mut_ptr(), small.len()) },
                HC_ERR_BUFFER_TOO_SMALL
            );
            assert_eq!(small, [7u8; 8]);
            let capacity = needed as usize;
            let pointer = hc_alloc(capacity);
            assert_eq!(unsafe { hc_sky_at(instant, pointer, capacity) }, needed);
            unsafe { hc_free(pointer, capacity) };
        }
    }

    #[cfg(feature = "orbital")]
    mod orbital {
        use super::super::*;
        use super::read_lines;

        fn rows(text: &str) -> Vec<Vec<String>> {
            text.lines()
                .map(|line| line.split('\t').map(str::to_owned).collect())
                .collect()
        }

        fn orbit(years_before_1950: f64) -> Vec<String> {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_orbit_at(years_before_1950, buffer, capacity)
            });
            let lines = rows(&text);
            assert_eq!(lines.len(), 1, "{text:?}");
            lines.into_iter().next().unwrap_or_default()
        }

        fn number(cell: &str) -> f64 {
            cell.parse()
                .unwrap_or_else(|_| panic!("not a number: {cell}"))
        }

        /// The author's own table (`bein1.dat`) and the PMIP experiments
        /// put the Last Glacial Maximum, 21 000 years before 1950, at
        /// e = 0.018994, ϖ = 114.42°, ε = 22.949°, e sin ϖ = 0.01729; the
        /// spreads are the nearest tier's; and at 1950 the June insolation
        /// at 65° N is 477.6 W m⁻² for 1360 W m⁻².
        #[test]
        fn the_last_glacial_maximum_decodes_column_by_column() {
            let columns = orbit(21_000.0);
            assert_eq!(columns.len(), 11, "{columns:?}");
            assert!(
                (number(&columns[0]) - 0.018_994).abs() < 1e-6,
                "{columns:?}"
            );
            assert_eq!(columns[1], "0.002");
            assert!((number(&columns[2]) - 22.949).abs() < 1e-3, "{columns:?}");
            assert_eq!(columns[3], "0.05");
            assert!((number(&columns[4]) - 114.42).abs() < 0.01, "{columns:?}");
            // asin(0.0025 / 0.018994), in degrees.
            assert!((number(&columns[5]) - 7.56).abs() < 0.01, "{columns:?}");
            assert!((number(&columns[6]) - 0.017_29).abs() < 5e-6, "{columns:?}");
            assert_eq!(columns[7], "0.0025");
            let lgm_june = number(&columns[8]);
            assert_eq!(columns[9], "1360");
            assert!(columns[10].contains("Berger, A. (1978)"), "{}", columns[10]);
            assert!(
                columns[10].contains("SOLAR_CONSTANT_BERGER_LOUTRE_1991"),
                "{}",
                columns[10]
            );
            assert!(!columns[10].contains('\t'));

            let now = orbit(0.0);
            assert!((number(&now[8]) - 477.6).abs() < 0.1, "{now:?}");
            assert!(lgm_june < number(&now[8]));
            let early_holocene = orbit(11_000.0);
            assert!(number(&early_holocene[8]) - number(&now[8]) > 45.0);
            // Numbers are written in plain decimal notation.
            assert!(
                now.iter().take(10).all(|cell| !cell.contains('e')),
                "{now:?}"
            );
        }

        #[test]
        fn a_million_years_is_answered_and_a_year_more_is_refused() {
            let null = core::ptr::null_mut();
            assert!(unsafe { hc_orbit_at(1_000_000.0, null, 0) } > 0);
            assert!(unsafe { hc_orbit_at(-1_000_000.0, null, 0) } > 0);
            for epoch in [
                1_000_001.0,
                -1_000_001.0,
                f64::NAN,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ] {
                assert_eq!(
                    unsafe { hc_orbit_at(epoch, null, 0) },
                    HC_ERR_OUT_OF_RANGE,
                    "{epoch}"
                );
            }
        }

        #[test]
        fn a_series_is_the_single_lines_with_the_epoch_first() {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_orbit_series(0.0, 21_000.0, 1_000.0, buffer, capacity)
            });
            let lines = rows(&text);
            assert_eq!(lines.len(), 22, "{}", lines.len());
            assert!(lines.iter().all(|line| line.len() == 12), "{lines:?}");
            let epochs: Vec<f64> = lines.iter().map(|line| number(&line[0])).collect();
            let expected: Vec<f64> = (0..22).map(|kyr| f64::from(kyr) * 1_000.0).collect();
            assert_eq!(epochs, expected);
            assert_eq!(lines[0][1..], orbit(0.0)[..]);
            assert_eq!(lines[21][1..], orbit(21_000.0)[..]);
            // A step that does not divide the span stops before `to`.
            let text = read_lines(|buffer, capacity| unsafe {
                hc_orbit_series(0.0, 1_000.0, 300.0, buffer, capacity)
            });
            let epochs: Vec<String> = rows(&text).iter().map(|line| line[0].clone()).collect();
            assert_eq!(epochs, ["0", "300", "600", "900"]);
            // The whole span at the widest step that fits the cap.
            let text = read_lines(|buffer, capacity| unsafe {
                hc_orbit_series(-1_000_000.0, 1_000_000.0, 200.01, buffer, capacity)
            });
            let lines = rows(&text);
            assert_eq!(lines.len(), 10_000);
            assert!(number(&lines[9_999][0]) <= 1_000_000.0);
        }

        #[test]
        fn a_series_that_is_empty_too_long_or_off_the_span_is_refused_or_empty() {
            let null = core::ptr::null_mut();
            // A `to` before `from` is an empty answer.
            assert_eq!(unsafe { hc_orbit_series(1_000.0, 0.0, 100.0, null, 0) }, 0);
            // One sample when they coincide.
            assert!(unsafe { hc_orbit_series(0.0, 0.0, 100.0, null, 0) } > 0);
            for (from, to, step) in [
                (0.0, 1_000_001.0, 100.0),
                (-1_000_001.0, 0.0, 100.0),
                (f64::NAN, 0.0, 100.0),
                (0.0, f64::INFINITY, 100.0),
                (0.0, 1_000.0, 0.0),
                (0.0, 1_000.0, -100.0),
                (0.0, 1_000.0, f64::NAN),
                (0.0, 1_000.0, f64::MIN_POSITIVE),
                // 10 001 samples.
                (0.0, 10_000.0, 1.0),
                (-1_000_000.0, 1_000_000.0, 200.0),
            ] {
                assert_eq!(
                    unsafe { hc_orbit_series(from, to, step, null, 0) },
                    HC_ERR_OUT_OF_RANGE,
                    "{from} to {to} by {step}"
                );
            }
            // 10 000 samples is the most.
            assert!(unsafe { hc_orbit_series(0.0, 9_999.0, 1.0, null, 0) } > 0);
        }
    }

    #[cfg(feature = "timestamps")]
    mod time_scales {
        use super::super::*;
        use super::read_lines;

        /// `TTBIPM.2025`'s 27.6740 µs on MJD 58 479, 2018-12-22, when TAI −
        /// UTC was 37 s, and 27.6745 µs ten days later.
        #[test]
        fn tt_bipm_reads_the_callers_series() {
            let series = "58479\t27.6740\n58489\t27.6745\n";
            let tai = (58_479 - 40_587) * 86_400 + 37;
            let cells = line(|buffer, capacity| unsafe {
                hc_tt_bipm(series.as_ptr(), series.len(), tai, 0, 1, buffer, capacity)
            });
            assert_eq!(cells.len(), 5);
            let offset: f64 = cells[0].parse().expect("seconds");
            assert!((offset - 27.674e-6).abs() < 1e-15, "{offset}");
            assert_eq!(cells[1], "32");
            assert_eq!(cells[3], (tai + 32).to_string());
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_tt_bipm(series.as_ptr(), series.len(), tai - 1, 0, 1, null, 0) },
                HC_ERR_NO_DATA
            );
            assert_eq!(
                unsafe { hc_tt_bipm("58479 27.6".as_ptr(), 10, tai, 0, 1, null, 0) },
                HC_ERR_MALFORMED
            );
            assert_eq!(
                unsafe {
                    hc_tt_bipm(
                        series.as_ptr(),
                        series.len(),
                        tai,
                        1_000_000_000_000_000_000,
                        1,
                        null,
                        0,
                    )
                },
                HC_ERR_OUT_OF_RANGE
            );
        }

        fn line(call: impl Fn(*mut u8, usize) -> i64) -> Vec<String> {
            let text = read_lines(call);
            let line = text.strip_suffix('\n').expect("one line");
            assert!(!line.contains('\n'), "{text:?}");
            line.split('\t').map(str::to_owned).collect()
        }

        #[test]
        fn a_tai64_label_crosses_both_ways_and_refuses_by_name() {
            let label = line(|buffer, capacity| unsafe {
                hc_tai64_encode(0, 0, "tai64n".as_ptr(), 6, buffer, capacity)
            });
            assert_eq!(label, ["400000000000000000000000"]);
            let hex = "3FFFFFFFFFFFFFFF3B9AC9FF3B9AC9FF";
            let decoded = line(|buffer, capacity| unsafe {
                hc_tai64_decode(hex.as_ptr(), hex.len(), buffer, capacity)
            });
            assert_eq!(decoded, ["tai64na", "-1", "999999999999999999"]);
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_tai64_encode(0, 0, "tai32".as_ptr(), 5, null, 0) },
                HC_ERR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_tai64_encode(0, 1_000_000_000_000_000_000, "tai64".as_ptr(), 5, null, 0)
                },
                HC_ERR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_tai64_decode("40".as_ptr(), 2, null, 0) },
                HC_ERR_MALFORMED
            );
            assert_eq!(
                unsafe { hc_tai64_decode(core::ptr::null(), 1, null, 0) },
                HC_ERR_NULL_POINTER
            );
            assert_eq!(
                unsafe { hc_tai64_decode([0xffu8; 16].as_ptr(), 16, null, 0) },
                HC_ERR_NOT_UTF8
            );
        }

        /// The leap second at the end of 2016 crosses as a line: 23:59:59
        /// UTC was TAI + 36 s, so TAI second 1 483 228 836 is 23:59:60,
        /// named by the POSIX second after it, and 00:00:00 is TAI + 37 s.
        #[test]
        fn the_tai_bridge_writes_one_line_each_way() {
            let read = |call: &dyn Fn(*mut u8, usize) -> i64| {
                let measured = call(core::ptr::null_mut(), 0);
                let mut buffer = [0u8; 64];
                let len = call(buffer.as_mut_ptr(), buffer.len());
                assert_eq!(len, measured);
                String::from_utf8(buffer[..len as usize].to_vec()).expect("UTF-8")
            };
            let new_year = 1_483_228_800;
            assert_eq!(
                read(&|buffer, capacity| unsafe {
                    hc_tai_from_unix(new_year, 1, buffer, capacity)
                }),
                "1483228837\t0\n"
            );
            assert_eq!(
                read(&|buffer, capacity| unsafe {
                    hc_utc_from_tai(new_year + 36, 1, buffer, capacity)
                }),
                "1483228800\t1\n"
            );
            assert_eq!(
                read(&|buffer, capacity| unsafe {
                    hc_utc_from_tai(new_year + 37, 0, buffer, capacity)
                }),
                "1483228800\t0\n"
            );
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_tai_from_unix(-400_000_000, 1, null, 0) },
                HC_ERR_NO_DATA
            );
            assert_eq!(
                unsafe { hc_tai_from_unix(i64::MAX, 0, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            let mut small = [0u8; 4];
            assert_eq!(
                unsafe { hc_tai_from_unix(new_year, 1, small.as_mut_ptr(), small.len()) },
                HC_ERR_BUFFER_TOO_SMALL
            );
        }

        /// POSIX 0 is `@400000000000000a` on daemontools' ordinary clock,
        /// which the true-TAI decoder reads as ten seconds after 1970 TAI.
        #[test]
        fn the_posix_plus_10_labels_cross_both_ways() {
            let label = line(|buffer, capacity| unsafe {
                hc_tai64_posix_plus_10_encode(0, 0, "TAI64".as_ptr(), 5, buffer, capacity)
            });
            assert_eq!(label, ["400000000000000a"]);
            let hex = "400000000000000b1dcd6500";
            let decoded = line(|buffer, capacity| unsafe {
                hc_tai64_posix_plus_10_decode(hex.as_ptr(), hex.len(), buffer, capacity)
            });
            assert_eq!(decoded, ["tai64n", "1", "500000000000000000"]);
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_tai64_posix_plus_10_encode(0, 0, "tai64na".as_ptr(), 7, null, 0) },
                HC_ERR_UNKNOWN
            );
            let na = "3fffffffffffffff3b9ac9ff3b9ac9ff";
            assert_eq!(
                unsafe { hc_tai64_posix_plus_10_decode(na.as_ptr(), na.len(), null, 0) },
                HC_ERR_MALFORMED
            );
        }

        /// RFC 9562's version 1 and version 6 vectors, Appendix A: both
        /// 0x1EC9414C232AB00, POSIX 1 645 557 742.
        #[test]
        fn a_uuid_timestamp_crosses_the_boundary() {
            for (uuid, version) in [
                ("C232AB00-9414-11EC-B3C8-9F6BDECED846", "1"),
                ("1EC9414C-232A-6B00-B3C8-9F6BDECED846", "6"),
            ] {
                let cells = line(|buffer, capacity| unsafe {
                    hc_uuid_timestamp(uuid.as_ptr(), uuid.len(), buffer, capacity)
                });
                assert_eq!(cells, [version, "138648505420000000", "1645557742", "0"]);
            }
            let v4 = "919108f7-52d1-4320-9bac-f847db4148a8";
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_uuid_timestamp(v4.as_ptr(), v4.len(), null, 0) },
                HC_ERR_NO_DATA
            );
            assert_eq!(
                unsafe { hc_uuid_timestamp("C232AB00".as_ptr(), 8, null, 0) },
                HC_ERR_MALFORMED
            );
        }

        /// RFC 5905's Figure 4: 8 February 2036 is era 1, offset 63 104,
        /// read against a clock in 2030; the zero timestamp is unknown.
        #[test]
        fn an_ntp_timestamp_crosses_with_its_era() {
            let cells = line(|buffer, capacity| unsafe {
                hc_ntp_resolve(63_104, 0, 1_893_456_000, buffer, capacity)
            });
            assert_eq!(cells, ["1", "63104", "0", "2086041600", "0"]);
            assert_eq!(
                unsafe { hc_ntp_resolve(0, 0, 0, core::ptr::null_mut(), 0) },
                HC_ERR_NO_DATA
            );
        }

        /// RFC 9562's vectors the other way: POSIX 1 645 557 742 is the
        /// timestamp 0x1EC9414C232AB00, `C232AB00-9414-11EC` in version 1
        /// and `1EC9414C-232A-6B00` in version 6.
        #[test]
        fn a_posix_instant_crosses_as_a_uuid_timestamp() {
            let cells = line(|buffer, capacity| unsafe {
                hc_uuid_timestamp_encode(1_645_557_742, 0, buffer, capacity)
            });
            assert_eq!(
                cells,
                [
                    "138648505420000000",
                    "c232ab00-9414-11ec",
                    "1ec9414c-232a-6b00"
                ]
            );
            let null = core::ptr::null_mut();
            for (seconds, attoseconds) in [
                (-12_219_292_801, 0),
                (103_072_857_661, 0),
                (0, 1_000_000_000_000_000_000),
            ] {
                assert_eq!(
                    unsafe { hc_uuid_timestamp_encode(seconds, attoseconds, null, 0) },
                    HC_ERR_OUT_OF_RANGE,
                    "{seconds}"
                );
            }
        }

        /// RFC 5905's Figure 4: 1 January 1970 is era 0, offset
        /// 2 208 988 800, and 8 February 2036 is era 1, offset 63 104.
        #[test]
        fn a_posix_instant_crosses_as_an_ntp_date() {
            let cells = line(|buffer, capacity| unsafe { hc_ntp_encode(0, 0, buffer, capacity) });
            assert_eq!(
                cells,
                [
                    "0",
                    "2208988800",
                    "0",
                    "0000000083aa7e800000000000000000",
                    "83aa7e8000000000"
                ]
            );
            let cells = line(|buffer, capacity| unsafe {
                hc_ntp_encode(2_086_041_600, 0, buffer, capacity)
            });
            assert_eq!(cells[..2], ["1", "63104"]);
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_ntp_encode(i64::MAX, 0, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_ntp_encode(0, 1_000_000_000_000_000_000, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        /// The worked example of `docs/systems/binary-timestamps.md`:
        /// 2026-09-26 23:59:58 is the words 23 866 and 49 021.
        #[test]
        fn the_fat_words_cross_both_ways() {
            let day = hc_gregorian_to_fixed(2026, 9, 26);
            let cells =
                line(|buffer, capacity| unsafe { hc_fat_decode(23_866, 49_021, buffer, capacity) });
            assert_eq!(cells, [day.to_string(), "86398".to_owned()]);
            let cells =
                line(|buffer, capacity| unsafe { hc_fat_encode(day, 86_399, buffer, capacity) });
            assert_eq!(cells, ["23866", "49021"]);
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_fat_decode(65_536, 0, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_fat_decode(46 << 9 | 1, 0, null, 0) },
                HC_ERR_INVALID_DATE
            );
            assert_eq!(
                unsafe { hc_fat_encode(hc_gregorian_to_fixed(1979, 12, 31), 0, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        /// Wikipedia's example, @248 at 04:57:07.2 UTC; the POSIX epoch is
        /// @041.
        #[test]
        fn a_swatch_beat_is_a_value() {
            let at_248 = (4 * 60 + 57) * 60 + 7;
            assert_eq!(hc_swatch_beat(at_248, 200_000_000_000_000_000), 248);
            assert_eq!(hc_swatch_beat(0, 0), 41);
            assert_eq!(
                hc_swatch_beat(0, 1_000_000_000_000_000_000),
                HC_ERR_OUT_OF_RANGE
            );
        }

        /// SOFA's §2.4 example, JD 2457073.05631 TT: J2015.1349933196 and
        /// B2015.1365941021; and J2000.0 is 946 728 000 TT seconds.
        #[test]
        fn the_epochs_cross_both_ways() {
            let epoch = |notation: &str, seconds: i64, attos: u64| {
                line(|buffer, capacity| unsafe {
                    hc_epoch_from_tt(
                        notation.as_ptr(),
                        notation.len(),
                        seconds,
                        attos,
                        buffer,
                        capacity,
                    )
                })
            };
            let julian = epoch("J", 1_424_352_065, 184_000_000_000_000_000);
            assert_eq!(julian[0], "J");
            let year: f64 = julian[1].parse().expect("a number");
            assert!((year - 2_015.134_993_319_6).abs() < 1e-10, "{year}");
            let besselian = epoch("B", 1_424_352_065, 184_000_000_000_000_000);
            let year: f64 = besselian[1].parse().expect("a number");
            assert!((year - 2_015.136_594_102_1).abs() < 1e-10, "{year}");
            assert_eq!(epoch("julian-epoch", 946_728_000, 0), ["J", "2000"]);
            let back = line(|buffer, capacity| unsafe {
                hc_tt_from_epoch("".as_ptr(), 0, 2000.0, buffer, capacity)
            });
            assert_eq!(back, ["J", "946728000", "0"]);
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_epoch_from_tt("X".as_ptr(), 1, 0, 0, null, 0) },
                HC_ERR_UNKNOWN
            );
            assert_eq!(
                unsafe { hc_tt_from_epoch("J".as_ptr(), 1, f64::NAN, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        /// GPS week 2048 began at 2019-04-06 23:59:42 UTC, TAI second
        /// 1 554 595 219.
        #[test]
        fn the_april_2019_rollover_crosses_the_boundary() {
            let tai = 1_554_595_219;
            let id = "gps-lnav-week";
            let week = line(|buffer, capacity| unsafe {
                hc_gnss_week(id.as_ptr(), id.len(), tai, 0, buffer, capacity)
            });
            assert_eq!(week, ["2048", "0", "0", "0"]);
            let back = line(|buffer, capacity| unsafe {
                hc_gnss_to_tai(id.as_ptr(), id.len(), 2048, 0, 0, buffer, capacity)
            });
            assert_eq!(back, [tai.to_string(), "0".to_owned()]);
            let resolve = |broadcast: u32, rule: &str| unsafe {
                hc_gnss_resolve_week(
                    id.as_ptr(),
                    id.len(),
                    broadcast,
                    rule.as_ptr(),
                    rule.len(),
                    tai,
                )
            };
            assert_eq!(resolve(0, "not-before"), 2048);
            assert_eq!(resolve(1023, "nearest"), 2047);
            assert_eq!(resolve(1024, "nearest"), HC_ERR_OUT_OF_RANGE);
            assert_eq!(resolve(0, "latest"), HC_ERR_UNKNOWN);
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_gnss_week(id.as_ptr(), id.len(), 0, 0, null, 0) },
                HC_ERR_NO_DATA
            );
            assert_eq!(
                unsafe { hc_gnss_to_tai(id.as_ptr(), id.len(), 0, 604_800, 0, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        #[test]
        fn glonass_ole_and_excel_dates_cross_the_boundary() {
            // 2024-01-01 00:00 UTC: N4 = 8, N_T = 1.
            let tai = 1_704_067_200 + 37;
            let date =
                line(|buffer, capacity| unsafe { hc_glonass_date(tai, 0, 1, buffer, capacity) });
            assert_eq!(date, ["8", "1"]);
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_glonass_date(0, 0, 0, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            let new_year_1900 = hc_gregorian_to_fixed(1900, 1, 1);
            let ole = line(|buffer, capacity| unsafe {
                hc_fixed_from_ole_automation(-1.25, buffer, capacity)
            });
            assert_eq!(ole, [(new_year_1900 - 3).to_string(), "21600".to_owned()]);
            let value = line(|buffer, capacity| unsafe {
                hc_ole_automation_from_fixed(new_year_1900 - 3, 21_600.0, buffer, capacity)
            });
            assert_eq!(value, ["-1.25"]);
            assert_eq!(
                unsafe { hc_fixed_from_ole_automation(f64::INFINITY, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            let phantom =
                line(|buffer, capacity| unsafe { hc_excel_1900_day(60, buffer, capacity) });
            assert_eq!(phantom, ["", "1"]);
            let march = line(|buffer, capacity| unsafe { hc_excel_1900_day(61, buffer, capacity) });
            assert_eq!(
                march,
                [
                    hc_gregorian_to_fixed(1900, 3, 1).to_string(),
                    "0".to_owned()
                ]
            );
            assert_eq!(
                unsafe { hc_excel_1900_day(0, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }
    }

    #[cfg(feature = "calendars")]
    mod calendar_days {
        use super::super::*;
        use super::read_lines;

        /// 5782 (2021–22) and 5789 (2028–29) are sabbatical years, as
        /// Wikipedia and Chabad.org list them.
        #[test]
        fn the_published_sabbatical_years_are_the_seventh() {
            assert_eq!(hc_hebrew_sabbatical_cycle_year(5_782), 7);
            assert_eq!(hc_hebrew_sabbatical_cycle_year(5_789), 7);
            assert_eq!(hc_hebrew_sabbatical_cycle_year(5_786), 4);
            assert_eq!(hc_hebrew_sabbatical_cycle_year(0), HC_ERR_OUT_OF_RANGE);
        }

        /// The first day carried, 23 September AD 4, is Sebaste of Kaisar,
        /// and 7 October is day 14, as the Metropolis *hemerologion* has it.
        #[test]
        fn sebaste_is_unnumbered_and_the_days_after_it_are_counted() {
            let read = |fixed: i64| {
                read_lines(|buffer, capacity| unsafe { hc_asian_day(fixed, buffer, capacity) })
            };
            assert_eq!(read(1_360), "4\t1\tKaisar\tunnumbered\t1\n");
            assert_eq!(read(1_374), "4\t1\tKaisar\tnumbered\t14\n");
            assert_eq!(
                unsafe { hc_asian_day(1_359, core::ptr::null_mut(), 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        /// Chaitra śukla 1 of Śaka 1947, Vikrama 2082, on 30 March 2025, by
        /// the true sky at the Central Station and by the Siddhānta's at
        /// Ujjain (`docs/systems/hindu-calendars.md`), with the Siddhānta's
        /// Sun in Mīna and its first tithi at its sunrise.
        #[test]
        fn the_hindu_new_year_of_saka_1947_on_both_skies() {
            let day = hc_gregorian_to_fixed(2025, 3, 30);
            for (sky, latitude, longitude) in [
                ("lahiri", 23.183_333, 82.5),
                ("surya-siddhanta", 23.15, 75.768_333),
            ] {
                let text = read_lines(|buffer, capacity| unsafe {
                    hc_hindu_lunar_date(
                        sky.as_ptr(),
                        sky.len(),
                        day,
                        latitude,
                        longitude,
                        0.0,
                        "hi".as_ptr(),
                        2,
                        buffer,
                        capacity,
                    )
                });
                let cells: Vec<&str> = text.trim_end_matches('\n').split('\t').collect();
                assert_eq!(cells[..6], ["1947", "2082", "1", "0", "1", "0"], "{sky}");
                // Chaitra, in Hindi, with CLDR's शक for the era.
                assert_eq!(cells[7..], ["चैत्र", "", "शक", "", "hi"], "{sky}");
            }
            let text = read_lines(|buffer, capacity| unsafe {
                hc_surya_siddhanta_sunrise(day, 23.15, 75.768_333, buffer, capacity)
            });
            let sunrise: i64 = text.trim_end().parse().expect("an instant");
            let text = read_lines(|buffer, capacity| unsafe {
                hc_surya_siddhanta_at(sunrise, buffer, capacity)
            });
            let cells: Vec<&str> = text.trim_end_matches('\n').split('\t').collect();
            assert_eq!(cells[3..], ["1", "12"]);
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe {
                    hc_hindu_lunar_date(
                        "".as_ptr(),
                        0,
                        day,
                        0.0,
                        0.0,
                        0.0,
                        "en".as_ptr(),
                        2,
                        null,
                        0,
                    )
                },
                HC_ERR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_hindu_lunar_date(
                        "lahiri".as_ptr(),
                        6,
                        day,
                        23.18,
                        82.5,
                        0.0,
                        [0xffu8].as_ptr(),
                        1,
                        null,
                        0,
                    )
                },
                HC_ERR_NOT_UTF8
            );
            assert_eq!(
                unsafe { hc_surya_siddhanta_sunrise(day, 80.0, 0.0, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_surya_siddhanta_at(i64::MIN, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        /// The first of Ramadan 1445 at Mecca by `islamic-rgsa`, the
        /// calendar the criterion begins months for: its eve carries a
        /// crescent by Shaukat's criterion, and the eve before none.
        #[test]
        fn the_crescent_that_began_ramadan_1445_at_mecca() {
            let criterion = "shaukat";
            let visible = |day: i64| {
                let text = read_lines(|buffer, capacity| unsafe {
                    hc_crescent_visible(
                        criterion.as_ptr(),
                        criterion.len(),
                        day,
                        21.423_333,
                        39.823_333,
                        298.0,
                        buffer,
                        capacity,
                    )
                });
                let cells: Vec<String> = text
                    .trim_end_matches('\n')
                    .split('\t')
                    .map(str::to_owned)
                    .collect();
                assert_eq!(cells.len(), 7, "{text}");
                cells[0] == "1"
            };
            let first =
                hc::hc_calendars_lunar::islamic_observational::IslamicObservationalCalendar::MECCA
                    .compose(1_445, 9, 1)
                    .expect("in range")
                    .0;
            assert!(visible(first));
            assert!(!visible(first - 1));
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_crescent_visible("danjon".as_ptr(), 6, first, 0.0, 0.0, 0.0, null, 0) },
                HC_ERR_UNKNOWN
            );
        }

        /// Drik Panchang for 1 January 2025, read at sunrise: Vyaghata and
        /// Balava.
        #[test]
        fn the_panchanga_of_the_first_of_january_2025_decodes_column_by_column() {
            let day = hc_gregorian_to_fixed(2025, 1, 1);
            let text = read_lines(|buffer, capacity| unsafe {
                hc_panchanga_of_day(
                    day,
                    23.183_333,
                    82.5,
                    0.0,
                    "Lahiri".as_ptr(),
                    6,
                    buffer,
                    capacity,
                )
            });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert_eq!(rows.len(), 2);
            assert!(rows.iter().all(|row| row.len() == 8), "{rows:?}");
            assert_eq!(rows[0][..4], ["yoga", "13", "Vyaghata", "व्याघात"]);
            assert_eq!(rows[1][2..4], ["Balava", "बालव"]);
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_panchanga_of_day(day, 89.0, 0.0, 0.0, "Lahiri".as_ptr(), 6, null, 0) },
                HC_ERR_NO_DATA
            );
            assert_eq!(
                unsafe {
                    hc_panchanga_of_day(day, f64::NAN, 0.0, 0.0, "Lahiri".as_ptr(), 6, null, 0)
                },
                HC_ERR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_panchanga_at(0, core::ptr::null(), 0, null, 0) },
                HC_ERR_UNKNOWN
            );
            assert_eq!(
                unsafe { hc_panchanga_at(i64::MAX, "Raman".as_ptr(), 5, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        #[test]
        fn the_olympiads_and_the_hebrew_anniversaries_are_numbers_above_the_floor() {
            assert_eq!(hc_ioc_olympiad(2021), 32);
            assert_eq!(hc_ioc_olympiad(i64::MAX), 2_305_843_009_213_693_478);
            assert_eq!(hc_ioc_olympiad(1895), HC_ERR_OUT_OF_RANGE);
            // 10 Tevet 5780 was 7 January 2020; 10 Tevet 5781, 25 December.
            let death = hc_gregorian_to_fixed(2020, 1, 7);
            assert_eq!(
                hc_hebrew_yahrzeit(death, 5781),
                hc_gregorian_to_fixed(2020, 12, 25)
            );
            assert_eq!(
                hc_hebrew_birthday(death, 5781),
                hc_gregorian_to_fixed(2020, 12, 25)
            );
            assert_eq!(hc_hebrew_yahrzeit(death, 0), HC_ERR_OUT_OF_RANGE);
            assert_eq!(hc_hebrew_birthday(i64::MIN, 5781), HC_ERR_OUT_OF_RANGE);
        }

        /// Wikipedia's child born in June 2000, 13 *suì* from the lunar new
        /// year of 2012; the South China Morning Post's widow year of 2024.
        #[test]
        fn the_chinese_reckonings_cross_the_boundary() {
            let birth = hc_gregorian_to_fixed(2000, 6, 15);
            assert_eq!(
                hc_chinese_reckoned_age(birth, hc_gregorian_to_fixed(2012, 1, 23)),
                13
            );
            assert_eq!(hc_chinese_reckoned_age(birth, birth - 1), HC_ERR_NO_DATA);
            assert_eq!(
                hc_chinese_reckoned_age(i64::MIN, birth),
                HC_ERR_OUT_OF_RANGE
            );
            let text = read_lines(|buffer, capacity| unsafe {
                hc_chinese_marriage_augury(4_661, buffer, capacity)
            });
            assert_eq!(text, "widow\t0\t0\n");
            assert_eq!(
                unsafe { hc_chinese_marriage_augury(i64::MAX, core::ptr::null_mut(), 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }
    }

    #[cfg(feature = "holiday")]
    mod observances {
        use super::super::*;
        use super::read_lines;

        /// *Spes non confundit*, 6: the jubilee of 2025 in Rome from
        /// 24 December 2024 to 6 January 2026.
        #[test]
        fn the_jubilee_of_2025_is_the_bulls() {
            let day = hc_gregorian_to_fixed(2025, 6, 1);
            let text =
                read_lines(|buffer, capacity| unsafe { hc_holy_year_on(day, buffer, capacity) });
            let cells: Vec<&str> = text.trim_end_matches('\n').split('\t').collect();
            assert_eq!(cells.len(), 10);
            assert_eq!(
                cells[..3],
                ["within", "Ordinary Jubilee of the Year 2025", "ordinary"]
            );
            assert_eq!(cells[6], hc_gregorian_to_fixed(2024, 12, 24).to_string());
            assert_eq!(cells[7], hc_gregorian_to_fixed(2026, 1, 6).to_string());
            let after = hc_gregorian_to_fixed(2026, 1, 7);
            let text =
                read_lines(|buffer, capacity| unsafe { hc_holy_year_on(after, buffer, capacity) });
            assert_eq!(text, "outside\t\t\t\t\t\t\t\t\t\n");
            let null = core::ptr::null_mut();
            assert_eq!(unsafe { hc_holy_year_on(0, null, 0) }, HC_ERR_NO_DATA);
        }

        /// St George's Day on Monday 28 April 2025 (Full Fact), a Festival;
        /// and in 2011, Easter being 24 April, Philip and James has no day,
        /// which `hc_holidays_on` reports as a gap of `common-worship`.
        #[test]
        fn a_celebration_carries_its_rank_and_an_unsettled_one_is_a_gap() {
            let day = hc_gregorian_to_fixed(2025, 4, 28);
            let text = read_lines(|buffer, capacity| unsafe {
                hc_common_worship_on(day, buffer, capacity)
            });
            assert_eq!(
                text,
                "George, Martyr, Patron of England\tfestival\tFestival\n"
            );
            let null = core::ptr::null_mut();
            let ordinary = hc_gregorian_to_fixed(2025, 4, 23);
            assert_eq!(unsafe { hc_common_worship_on(ordinary, null, 0) }, 0);
            assert_eq!(
                unsafe { hc_common_worship_on(i64::MAX, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            let in_2011 = hc_gregorian_to_fixed(2011, 5, 1);
            let text =
                read_lines(|buffer, capacity| unsafe { hc_holidays_on(in_2011, buffer, capacity) });
            assert!(
                text.lines().any(|line| line.starts_with("common-worship\t")
                    && line.contains("\tPhilip and James, Apostles\t\tgap\t")),
                "no gap in {text}"
            );
        }

        #[test]
        fn every_table_is_described_in_the_order_of_the_codes() {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_holiday_tables("en".as_ptr(), 2, buffer, capacity)
            });
            let codes =
                read_lines(|buffer, capacity| unsafe { hc_holiday_codes(buffer, capacity) });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert!(rows.iter().all(|row| row.len() == 8));
            assert_eq!(
                rows.iter().map(|row| row[0]).collect::<Vec<_>>(),
                codes.lines().collect::<Vec<_>>()
            );
            let xhkg = rows
                .iter()
                .find(|row| row[0] == "XHKG")
                .expect("Hong Kong's exchange");
            assert_eq!(xhkg[1], "exchange");
            assert_eq!(xhkg[6], "HK");
            assert_eq!(xhkg[7], "");
            // CLDR 48's short name for Hong Kong, in English and in
            // Japanese.
            let hong_kong = rows.iter().find(|row| row[0] == "HK").expect("HK");
            assert_eq!(
                hong_kong[2..],
                [
                    "Hong Kong SAR China",
                    "Hong Kong",
                    "en",
                    hong_kong[5],
                    "",
                    "Hong Kong"
                ]
            );
            let japanese = read_lines(|buffer, capacity| unsafe {
                hc_holiday_tables("ja".as_ptr(), 2, buffer, capacity)
            });
            let hong_kong = japanese
                .lines()
                .find(|line| line.starts_with("HK\t"))
                .expect("HK");
            assert!(hong_kong.ends_with("\t\t香港"), "{hong_kong}");
            assert_eq!(
                unsafe { hc_holiday_tables(core::ptr::null(), 1, core::ptr::null_mut(), 0) },
                HC_ERR_NULL_POINTER
            );
        }

        #[test]
        fn the_lectionary_and_the_astronomical_easter_cross_the_boundary() {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_lectionary(hc_gregorian_to_fixed(2026, 11, 22), buffer, capacity)
            });
            assert_eq!(text, "2026\tA\tII\t29\n");
            assert_eq!(
                unsafe {
                    hc_lectionary(hc_gregorian_to_fixed(4100, 1, 1), core::ptr::null_mut(), 0)
                },
                HC_ERR_OUT_OF_RANGE
            );
            assert_eq!(
                hc_astronomical_easter(2001),
                hc_gregorian_to_fixed(2001, 4, 15)
            );
            assert_eq!(hc_astronomical_easter(2151), HC_ERR_OUT_OF_RANGE);
        }

        /// The Aleppo statement's table: the vernal full moon of 2001 on
        /// Sunday 8 April, a week before its Easter, and that of 2019 on
        /// 21 March.
        #[test]
        fn the_astronomical_paschal_full_moon_crosses_the_boundary() {
            assert_eq!(
                hc_astronomical_paschal_full_moon(2001),
                hc_gregorian_to_fixed(2001, 4, 8)
            );
            assert_eq!(
                hc_astronomical_easter(2001) - hc_astronomical_paschal_full_moon(2001),
                7
            );
            assert_eq!(
                hc_astronomical_paschal_full_moon(2019),
                hc_gregorian_to_fixed(2019, 3, 21)
            );
            assert_eq!(hc_astronomical_paschal_full_moon(1582), HC_ERR_OUT_OF_RANGE);
            assert_eq!(hc_astronomical_paschal_full_moon(2151), HC_ERR_OUT_OF_RANGE);
        }
    }

    #[cfg(feature = "sky")]
    mod earth_and_sun {
        use super::super::*;
        use super::read_lines;

        /// Warren's row for 29 February 1992, 03:15:56.2, and 12h 56m 27.4s,
        /// +42° 10′ 17″ (J2000), in the IDL Astronomy Library's `helio_jd`:
        /// HJD − JD is 350.9 s.
        #[test]
        fn hjd_tt_is_the_idl_tables_correction() {
            let date = hc_gregorian_to_fixed(1992, 2, 29) as f64
                + 1_721_424.5
                + (3.0 * 3_600.0 + 15.0 * 60.0 + 56.2) / 86_400.0;
            let alpha = 15.0 * (12.0 + 56.0 / 60.0 + 27.4 / 3_600.0);
            let delta = 42.0 + 10.0 / 60.0 + 17.0 / 3_600.0;
            let text = read_lines(|buffer, capacity| unsafe {
                hc_hjd_tt(date, alpha, delta, buffer, capacity)
            });
            let cells: Vec<f64> = text
                .trim_end()
                .split('\t')
                .map(|cell| cell.parse().expect("a number"))
                .collect();
            assert_eq!(cells.len(), 2);
            assert!((cells[1] - 350.9).abs() < 0.1, "{text}");
            let text = read_lines(|buffer, capacity| unsafe {
                hc_hjd_utc(date, alpha, delta, 1, buffer, capacity)
            });
            // TAI − UTC was 26 s in February 1992.
            assert!(text.trim_end().ends_with("\t58.184"), "{text}");
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_hjd_tt(f64::INFINITY, alpha, delta, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_hjd_utc(2_433_282.5, alpha, delta, 1, null, 0) },
                HC_ERR_NO_DATA
            );
        }

        fn number(call: impl Fn(*mut u8, usize) -> i64) -> f64 {
            read_lines(call).trim_end().parse().expect("one number")
        }

        /// ERFA's test values, which `hc-astro`'s own tests read:
        /// `eraEra00` at JD 2 454 388.5 UT1 and `eraGmst82` at MJD 53 736.
        #[test]
        fn the_rotation_is_erfas_and_refuses_off_the_era() {
            let era = number(|buffer, capacity| unsafe {
                hc_earth_rotation_angle(1_192_406_400.0, buffer, capacity)
            });
            assert!(
                (era - 0.402_283_724_002_815_8_f64.to_degrees()).abs() < 1e-8,
                "{era}"
            );
            let gmst = number(|buffer, capacity| unsafe {
                hc_gmst_iau1982(1_136_073_600.0, buffer, capacity)
            });
            assert!(
                (gmst - 1.754_174_981_860_675_f64.to_degrees()).abs() < 1e-8,
                "{gmst}"
            );
            let gmst06 = number(|buffer, capacity| unsafe {
                hc_gmst_iau2006(1_136_073_600.0, buffer, capacity)
            });
            assert!(
                (gmst06 - 1.754_174_971_870_091_2_f64.to_degrees()).abs() < 1e-7,
                "{gmst06}"
            );
            let ut2 = number(|buffer, capacity| unsafe {
                hc_ut2_minus_ut1(946_684_800.0 + 0.03 * 86_400.0, buffer, capacity)
            });
            assert!((ut2 + 0.005).abs() < 1e-6, "{ut2}");
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_earth_rotation_angle(f64::NAN, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_gmst_iau2006(1e15, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        /// NAOJ: sunrise in Tokyo on 2024-01-01 at 06:50 JST, the temporal
        /// hour 6; and Tromsø's polar night, which has no temporal hour.
        #[test]
        fn a_clock_reads_or_names_what_it_misses() {
            let clock = "temporal";
            let sunrise =
                hc_unix_from_fixed(hc_gregorian_to_fixed(2023, 12, 31)) + 21 * 3_600 + 50 * 60;
            let text = read_lines(|buffer, capacity| unsafe {
                hc_solar_time(
                    clock.as_ptr(),
                    clock.len(),
                    sunrise,
                    35.6581,
                    139.7414,
                    0.0,
                    buffer,
                    capacity,
                )
            });
            let cells: Vec<&str> = text.trim_end_matches('\n').split('\t').collect();
            assert_eq!(cells.len(), 5);
            assert_eq!(cells[0], hc_gregorian_to_fixed(2024, 1, 1).to_string());
            let hours: f64 = cells[1].parse().expect("hours");
            assert!((hours - 6.0).abs() < 0.03, "{hours}");
            assert_eq!(cells[2..], ["", "", ""]);
            let midwinter = hc_gregorian_to_fixed(2024, 12, 21);
            let event = "asr-hanafi";
            let text = read_lines(|buffer, capacity| unsafe {
                hc_solar_event(
                    event.as_ptr(),
                    event.len(),
                    midwinter,
                    69.6496,
                    18.956,
                    0.0,
                    buffer,
                    capacity,
                )
            });
            assert_eq!(text, format!("\tno-noon-shadow\t{midwinter}\t\n"));
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_solar_event("dhuhr".as_ptr(), 5, midwinter, 0.0, 0.0, 0.0, null, 0) },
                HC_ERR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_solar_time(
                        clock.as_ptr(),
                        clock.len(),
                        sunrise,
                        0.0,
                        181.0,
                        0.0,
                        null,
                        0,
                    )
                },
                HC_ERR_OUT_OF_RANGE
            );
        }
        /// The USNO's minutes for Jerusalem on 2024-01-01, sunrise 06:39 and
        /// sunset 16:46 at UT+2 (`usno-api-rstt`), under its own horizon;
        /// the horizons listed; and a missing sunrise named.
        #[test]
        fn the_horizons_are_listed_and_each_rises_and_sets() {
            let text = read_lines(|buffer, capacity| unsafe { hc_horizons(buffer, capacity) });
            let ids: Vec<&str> = text
                .lines()
                .map(|line| line.split('\t').next().expect("an id"))
                .collect();
            assert_eq!(ids, ["geometric-dip", "usno", "calendrical-calculations"]);
            assert!(text.lines().all(|line| line.split('\t').count() == 5));
            assert!(text.lines().any(|line| line.ends_with("\tUSNO")));
            let day = hc_gregorian_to_fixed(2024, 1, 1);
            let midnight = hc_unix_from_fixed(day);
            let horizon = "USNO";
            for (export, published) in [
                (
                    hc_sunrise as unsafe extern "C" fn(_, _, _, _, _, _, _, _) -> i64,
                    4 * 3_600 + 39 * 60,
                ),
                (hc_sunset, 14 * 3_600 + 46 * 60),
            ] {
                let text = read_lines(|buffer, capacity| unsafe {
                    export(
                        horizon.as_ptr(),
                        horizon.len(),
                        day,
                        31.78,
                        35.24,
                        740.0,
                        buffer,
                        capacity,
                    )
                });
                let cells: Vec<&str> = text.trim_end_matches('\n').split('\t').collect();
                assert_eq!(cells.len(), 5);
                let instant: i64 = cells[0].parse().expect("an instant");
                assert!((instant - midnight - published).abs() <= 31, "{text}");
            }
            let midwinter = hc_gregorian_to_fixed(2024, 12, 21);
            let horizon = "geometric-dip";
            let text = read_lines(|buffer, capacity| unsafe {
                hc_sunrise(
                    horizon.as_ptr(),
                    horizon.len(),
                    midwinter,
                    69.6496,
                    18.956,
                    0.0,
                    buffer,
                    capacity,
                )
            });
            assert!(
                text.starts_with(&format!("\tsunrise\t{midwinter}\t\t")),
                "{text}"
            );
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_sunset("naoj".as_ptr(), 4, day, 0.0, 0.0, 0.0, null, 0) },
                HC_ERR_UNKNOWN
            );
            assert_eq!(
                unsafe { hc_sunset(core::ptr::null(), 0, day, 0.0, 0.0, 0.0, null, 0) },
                HC_ERR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_sunrise(
                        horizon.as_ptr(),
                        horizon.len(),
                        day,
                        91.0,
                        0.0,
                        0.0,
                        null,
                        0,
                    )
                },
                HC_ERR_OUT_OF_RANGE
            );
        }
    }

    #[cfg(feature = "planetary")]
    mod planetary {
        use super::super::*;
        use super::read_lines;

        fn cells(text: &str) -> Vec<Vec<String>> {
            text.lines()
                .map(|line| line.split('\t').map(str::to_owned).collect())
                .collect()
        }

        /// Gangale's Titan calibration: 2002-12-18 10:42 UTC, POSIX
        /// 1 040 208 120, was 209 Aries 13, Julian Circad 144 096, Solis.
        #[test]
        fn a_circad_date_crosses_the_boundary() {
            let id = "darian-titan";
            let text = read_lines(|buffer, capacity| unsafe {
                hc_circad_date(id.as_ptr(), id.len(), 1_040_208_120.0, buffer, capacity)
            });
            let rows = cells(&text);
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].len(), hc::planetary_lines::CIRCAD_DATE_COLUMNS);
            assert_eq!(
                rows[0][..7],
                ["darian-titan", "209", "9", "13", "Aries", "Solis", "144096"]
            );
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_circad_date("titan".as_ptr(), 5, 0.0, null, 0) },
                HC_ERR_UNKNOWN
            );
            assert_eq!(
                unsafe { hc_circad_date("martiana".as_ptr(), 8, 1e10, null, 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        fn number(cell: &str) -> f64 {
            cell.parse()
                .unwrap_or_else(|_| panic!("not a number: {cell}"))
        }

        /// Mars24's worked example A: 2000-01-06T00:00:00Z at the prime
        /// meridian is MSD 44795.99976, MTC 23:59:39, Ls 277.18758°, EOT
        /// −5.18774° and LTST 23:38:54.
        #[test]
        fn the_first_mars24_worked_example_decodes_column_by_column() {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_mars_time(947_116_800.0, 0.0, buffer, capacity)
            });
            let rows = cells(&text);
            assert_eq!(rows.len(), 1);
            let row = &rows[0];
            assert_eq!(row.len(), 16, "{row:?}");
            assert!((number(&row[0]) - 44_795.999_760_4).abs() < 1e-6, "{row:?}");
            assert_eq!(row[1], "23:59:39");
            assert_eq!(row[3], "23:59:39");
            assert_eq!(row[5], "23:38:54");
            assert!((number(&row[7]) - -5.187_74 * 4.0).abs() < 1e-3, "{row:?}");
            assert!((number(&row[8]) - 277.187_58).abs() < 1e-5, "{row:?}");
            assert_eq!(row[9], "24");
            assert_eq!(row[10], "207");
            assert!(row[15].contains("Mars24"), "{}", row[15]);
            assert!(
                row.iter()
                    .all(|cell| !cell.contains('e') || cell.contains(' '))
            );
        }

        #[test]
        fn a_mission_sol_is_the_missions_own_and_an_unpublished_one_is_refused() {
            let curiosity = "curiosity";
            let sol =
                |name: &str, unix: f64| unsafe { hc_mission_sol(name.as_ptr(), name.len(), unix) };
            assert_eq!(sol(curiosity, 1_344_230_277.0), 0);
            assert_eq!(sol("Mars Pathfinder", 868_035_415.0), 1);
            assert_eq!(sol("zhurong", 1_700_000_000.0), HC_ERR_NO_DATA);
            assert_eq!(sol("beagle-2", 1_700_000_000.0), HC_ERR_UNKNOWN);
            assert_eq!(
                sol(curiosity, 1_344_230_277.0 - 86_400.0),
                HC_ERR_OUT_OF_RANGE
            );
            assert_eq!(sol(curiosity, 5e9), HC_ERR_OUT_OF_RANGE);
            assert_eq!(
                unsafe { hc_mission_sol(core::ptr::null(), 3, 0.0) },
                HC_ERR_NULL_POINTER
            );
            let listed = cells(&read_lines(|buffer, capacity| unsafe {
                hc_missions(buffer, capacity)
            }));
            assert_eq!(listed.len(), 10);
            assert!(listed.iter().all(|row| row.len() == 11), "{listed:?}");
            let zhurong = listed.iter().find(|row| row[0] == "zhurong");
            assert_eq!(
                zhurong.map(|row| &row[4..7]),
                Some(&[String::new(), String::new(), String::new()][..])
            );
        }

        #[test]
        fn every_body_lists_and_titans_clock_reads() {
            let bodies = cells(&read_lines(|buffer, capacity| unsafe {
                hc_bodies(buffer, capacity)
            }));
            assert_eq!(bodies.len(), 22);
            assert!(bodies.iter().all(|row| row.len() == 12), "{bodies:?}");
            let titan = "titan";
            let text = read_lines(|buffer, capacity| unsafe {
                hc_body_time(
                    titan.as_ptr(),
                    titan.len(),
                    947_116_800.0,
                    0.0,
                    buffer,
                    capacity,
                )
            });
            let row = &cells(&text)[0];
            assert_eq!(row.len(), 8, "{row:?}");
            // A Titan hour is 15.97 Earth hours.
            assert!((number(&row[5]) / 3_600.0 - 15.97).abs() < 0.01, "{row:?}");
            assert_eq!(row[6], "convention");
            let sun = "Sun";
            assert_eq!(
                unsafe {
                    hc_body_time(sun.as_ptr(), sun.len(), 0.0, 0.0, core::ptr::null_mut(), 0)
                },
                HC_ERR_NO_DATA
            );
        }
    }

    #[cfg(feature = "relativity")]
    mod relativity {
        use super::super::*;
        use super::read_lines;

        fn row(text: &str) -> Vec<String> {
            text.trim_end().split('\t').map(str::to_owned).collect()
        }

        fn number(cell: &str) -> f64 {
            cell.parse()
                .unwrap_or_else(|_| panic!("not a number: {cell}"))
        }

        #[test]
        fn six_tenths_of_c_is_five_quarters() {
            let text = read_lines(|buffer, capacity| unsafe {
                hc_proper_time(0.6 * 299_792_458.0, 10.0, buffer, capacity)
            });
            let cells = row(&text);
            assert_eq!(cells.len(), 7, "{cells:?}");
            assert!((number(&cells[1]) - 1.25).abs() < 1e-12, "{cells:?}");
            assert!((number(&cells[2]) - 8.0).abs() < 1e-9, "{cells:?}");
            assert_eq!(cells[5], "SPEED_OF_LIGHT");
            assert_eq!(
                unsafe { hc_proper_time(299_792_458.0, 1.0, core::ptr::null_mut(), 0) },
                HC_ERR_OUT_OF_RANGE
            );
        }

        #[test]
        fn a_clock_at_a_radius_names_the_constant_it_used() {
            let earth = "earth";
            let text = read_lines(|buffer, capacity| unsafe {
                hc_gravitational_dilation(
                    earth.as_ptr(),
                    earth.len(),
                    6_378_137.0,
                    buffer,
                    capacity,
                )
            });
            let cells = row(&text);
            assert_eq!(cells.len(), 8, "{cells:?}");
            assert_eq!(cells[0], "earth");
            assert_eq!(cells[2], "GM_EARTH");
            assert!(number(&cells[4]) < 1.0);
            assert!(number(&cells[5]) < 0.0);
            let listed =
                read_lines(|buffer, capacity| unsafe { hc_gravitating_bodies(buffer, capacity) });
            assert_eq!(listed.lines().count(), 6);
            let sun = "sun";
            assert_eq!(
                unsafe {
                    hc_gravitational_dilation(
                        sun.as_ptr(),
                        sun.len(),
                        1_000.0,
                        core::ptr::null_mut(),
                        0,
                    )
                },
                HC_ERR_OUT_OF_RANGE
            );
            let vulcan = "vulcan";
            assert_eq!(
                unsafe {
                    hc_gravitational_dilation(
                        vulcan.as_ptr(),
                        vulcan.len(),
                        1e7,
                        core::ptr::null_mut(),
                        0,
                    )
                },
                HC_ERR_UNKNOWN
            );
        }
    }
}
