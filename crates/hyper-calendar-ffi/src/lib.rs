//! A C ABI for `hyper-calendar`.
//!
//! # The shape of this interface, and why
//!
//! A C boundary has three ways to go wrong, and each is closed deliberately
//! here.
//!
//! **Panics cannot cross it.** Unwinding across an `extern "C"` frame is
//! undefined behaviour. The workspace already denies `unwrap` and `expect`
//! outside tests, so the Rust side does not panic; every entry point here
//! additionally returns a status code rather than a value, so a failure is a
//! value the caller can see rather than a trap.
//!
//! **Nothing is allocated that the caller cannot free.** There are no
//! returned pointers and no opaque handles. Every function writes into
//! storage the caller owns: integers through out-parameters, text into a
//! caller-supplied buffer. That removes the whole class of "who frees this"
//! bugs, and it means the library has no global state to tear down.
//!
//! **Truncation is reported, not silent.** A text function that does not fit
//! returns [`HC_ERROR_BUFFER_TOO_SMALL`] and writes the required length, so
//! the caller can retry with a bigger buffer. It never writes a partial
//! answer and calls it success.
//!
//! # Status codes
//!
//! Every function returns [`HcStatus`]: zero for success, negative for
//! failure. The codes are stable and are the only error channel.
//!
//! ```c
//! int64_t rd;
//! if (hc_gregorian_to_fixed(2026, 9, 21, &rd) == HC_OK) {
//!     // rd == 739880
//! }
//! ```
//!
//! # Lines and cells
//!
//! Every entry point that answers with more than one value writes UTF-8
//! lines ending in `\n`, one per entry, with the cells of a line separated
//! by `\t`, NUL-terminated as a whole. The column order of each is fixed,
//! stated on the entry point and in the README, and only ever grows at the
//! end; a cell with nothing to say is empty, and no cell contains a tab or
//! a line break. The lines are the same lines the WebAssembly module
//! writes.
//!
//! # Layers
//!
//! The entry points come in layers, each a Cargo feature: `civil` (the
//! default), `timestamps`, `calendars`, `holiday`, `seasons`, `deep-time`,
//! `tz`, `sky`, `orbital`, `planetary` and `relativity`, with `full` for
//! all of them. Which feature each needs is in the README's table.

#![allow(unsafe_code)]
#![warn(missing_docs)]

use core::ffi::{c_char, c_int};

use hc::hc_core::TimeError;

/// The status returned by every entry point. Zero is success.
pub type HcStatus = c_int;

/// The call succeeded.
pub const HC_OK: HcStatus = 0;
/// A required pointer was null.
pub const HC_ERROR_NULL_POINTER: HcStatus = -1;
/// A field was outside its valid range.
pub const HC_ERROR_OUT_OF_RANGE: HcStatus = -2;
/// The date does not exist in the requested calendar.
pub const HC_ERROR_INVALID_DATE: HcStatus = -3;
/// Arithmetic left the representable range.
pub const HC_ERROR_OVERFLOW: HcStatus = -4;
/// The supplied buffer was too small; the required length, including the
/// terminating NUL, was written out.
pub const HC_ERROR_BUFFER_TOO_SMALL: HcStatus = -5;
/// The value lies outside the range where the requested model has data.
pub const HC_ERROR_NO_DATA: HcStatus = -6;
/// The requested calendar, table or identifier is not known.
pub const HC_ERROR_UNKNOWN: HcStatus = -7;
/// A string argument was not valid UTF-8.
pub const HC_ERROR_NOT_UTF8: HcStatus = -8;
/// Data was not in the format the call expects.
pub const HC_ERROR_MALFORMED: HcStatus = -9;

#[allow(dead_code)]
fn status_from_time(error: TimeError) -> HcStatus {
    match error {
        TimeError::Overflow => HC_ERROR_OVERFLOW,
        TimeError::OutOfRange => HC_ERROR_OUT_OF_RANGE,
        TimeError::BeforeModelStart | TimeError::AfterModelEnd => HC_ERROR_NO_DATA,
        _ => HC_ERROR_OUT_OF_RANGE,
    }
}

/// Write a string into a caller-owned buffer, NUL-terminated.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes, or `capacity` must be zero.
unsafe fn write_text(
    text: &str,
    buffer: *mut c_char,
    capacity: usize,
    written: *mut usize,
) -> HcStatus {
    let needed = text.len() + 1;
    if !written.is_null() {
        // SAFETY: the caller guarantees `written` is either null or writable,
        // and it was checked non-null just above.
        unsafe { *written = needed };
    }
    if buffer.is_null() || capacity < needed {
        return HC_ERROR_BUFFER_TOO_SMALL;
    }
    // SAFETY: `capacity >= needed == text.len() + 1`, so the copy and the
    // terminator both stay inside the caller's buffer.
    unsafe {
        core::ptr::copy_nonoverlapping(text.as_ptr().cast::<c_char>(), buffer, text.len());
        *buffer.add(text.len()) = 0;
    }
    HC_OK
}

/// A NUL-terminated string: `Ok(None)` for a null pointer and
/// [`HC_ERROR_NOT_UTF8`] for bytes that are not UTF-8.
///
/// # Safety
///
/// `pointer` must be null or point to a NUL-terminated string.
#[allow(dead_code)]
unsafe fn text<'a>(pointer: *const c_char) -> Result<Option<&'a str>, HcStatus> {
    if pointer.is_null() {
        return Ok(None);
    }
    // SAFETY: the caller guarantees a NUL-terminated string.
    unsafe { core::ffi::CStr::from_ptr(pointer) }
        .to_str()
        .map(Some)
        .map_err(|_| HC_ERROR_NOT_UTF8)
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

/// The status a refusal of the shared line-makers in `hyper_calendar` is.
#[allow(dead_code)]
const fn status(refusal: hc::boundary::Refusal) -> HcStatus {
    use hc::boundary::Refusal;
    match refusal {
        Refusal::OutOfRange => HC_ERROR_OUT_OF_RANGE,
        Refusal::Overflow => HC_ERROR_OVERFLOW,
        Refusal::NoData => HC_ERROR_NO_DATA,
        Refusal::Unknown => HC_ERROR_UNKNOWN,
        Refusal::Malformed => HC_ERROR_MALFORMED,
        Refusal::InvalidDate => HC_ERROR_INVALID_DATE,
    }
}

/// A shared line-maker's answer, written into a caller-owned buffer as
/// [`write_text`] does, or its refusal as a status.
///
/// # Safety
///
/// As [`write_text`].
#[allow(dead_code)]
unsafe fn write_answer(
    answer: hc::boundary::Answer<String>,
    buffer: *mut c_char,
    capacity: usize,
    written: *mut usize,
) -> HcStatus {
    match answer {
        // SAFETY: forwarded to the caller's contract above.
        Ok(text) => unsafe { write_text(&text, buffer, capacity, written) },
        Err(refusal) => status(refusal),
    }
}

/// A NUL-terminated name the call needs: [`HC_ERROR_NULL_POINTER`] for a
/// null pointer and [`HC_ERROR_NOT_UTF8`] for bytes that are not UTF-8.
///
/// # Safety
///
/// As [`text`].
#[allow(dead_code)]
unsafe fn name<'a>(pointer: *const c_char) -> Result<&'a str, HcStatus> {
    // SAFETY: forwarded to the caller's contract above.
    unsafe { text(pointer) }?.ok_or(HC_ERROR_NULL_POINTER)
}

/// The library version, as a NUL-terminated string.
///
/// Writes the required length, including the terminator, into `written`.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes and `written` must be null
/// or writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_version(
    buffer: *mut c_char,
    capacity: usize,
    written: *mut usize,
) -> HcStatus {
    // SAFETY: forwarded to the caller's contract above.
    unsafe { write_text(hc::VERSION, buffer, capacity, written) }
}

/// The civil calendar, behind the `civil` feature: proleptic Gregorian
/// dates, ISO 8601 text and the TAI–UTC bridge.
#[cfg(feature = "civil")]
mod civil {
    use core::ffi::{c_char, c_int};

    use hc::civil::Date;
    use hc::hc_calendar::fixed::RD_OF_UNIX_EPOCH;
    use hc::hc_calendar::{CalendarError, Rd, Weekday};
    use hc::hc_core::unix::{self, LeapPolicy};

    use super::{
        HC_ERROR_BUFFER_TOO_SMALL, HC_ERROR_INVALID_DATE, HC_ERROR_NO_DATA, HC_ERROR_NULL_POINTER,
        HC_ERROR_OUT_OF_RANGE, HC_ERROR_OVERFLOW, HC_ERROR_UNKNOWN, HC_OK, HcStatus,
        status_from_time, write_text,
    };

    fn status_from_calendar(error: CalendarError) -> HcStatus {
        match error {
            CalendarError::MonthOutOfRange
            | CalendarError::DayOutOfRange
            | CalendarError::YearOutOfRange => HC_ERROR_INVALID_DATE,
            CalendarError::Overflow => HC_ERROR_OVERFLOW,
            CalendarError::BeforeEpoch | CalendarError::AfterSupportedRange => HC_ERROR_NO_DATA,
            CalendarError::UnknownCalendar | CalendarError::UnknownEra => HC_ERROR_UNKNOWN,
            _ => HC_ERROR_OUT_OF_RANGE,
        }
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
            Err(error) => status_from_calendar(error),
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
            Err(error) => status_from_calendar(error),
        }
    }

    /// The ISO 8601 weekday of a fixed day, Monday = 1 through Sunday = 7.
    ///
    /// # Safety
    ///
    /// `out_weekday` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_weekday(fixed: i64, out_weekday: *mut u8) -> HcStatus {
        if out_weekday.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        let weekday = Weekday::from_rd(Rd(fixed));
        // SAFETY: checked non-null immediately above.
        unsafe { *out_weekday = weekday.iso_number() };
        HC_OK
    }

    /// The 1-based day of the Gregorian year on a fixed day.
    ///
    /// # Safety
    ///
    /// `out_day_of_year` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_day_of_year(fixed: i64, out_day_of_year: *mut u32) -> HcStatus {
        if out_day_of_year.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match Date::from_ordinal(fixed) {
            Ok(date) => {
                // SAFETY: checked non-null immediately above.
                unsafe { *out_day_of_year = u32::from(date.day_of_year()) };
                HC_OK
            }
            Err(error) => status_from_calendar(error),
        }
    }

    /// Whether the Gregorian year on a fixed day is a leap year: writes 1
    /// or 0.
    ///
    /// # Safety
    ///
    /// `out_is_leap` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_is_leap_year(fixed: i64, out_is_leap: *mut c_int) -> HcStatus {
        if out_is_leap.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match Date::from_ordinal(fixed) {
            Ok(date) => {
                // SAFETY: checked non-null immediately above.
                unsafe { *out_is_leap = c_int::from(date.is_leap_year()) };
                HC_OK
            }
            Err(error) => status_from_calendar(error),
        }
    }

    /// The fixed day a POSIX timestamp falls on, in UTC.
    ///
    /// # Safety
    ///
    /// `out_fixed` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_fixed_from_unix(
        unix_seconds: i64,
        out_fixed: *mut i64,
    ) -> HcStatus {
        if out_fixed.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        let fixed = Rd::from_unix_days(unix_seconds.div_euclid(86_400)).get();
        // SAFETY: checked non-null immediately above.
        unsafe { *out_fixed = fixed };
        HC_OK
    }

    /// The POSIX timestamp of midnight UTC on a fixed day.
    ///
    /// A day whose midnight does not fit an `int64_t` is
    /// [`HC_ERROR_OUT_OF_RANGE`]. There is no floor, as the WebAssembly
    /// module has; see the README.
    ///
    /// # Safety
    ///
    /// `out_unix_seconds` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_unix_from_fixed(
        fixed: i64,
        out_unix_seconds: *mut i64,
    ) -> HcStatus {
        if out_unix_seconds.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        let Some(midnight) = fixed
            .checked_sub(RD_OF_UNIX_EPOCH)
            .and_then(|days| days.checked_mul(86_400))
        else {
            return HC_ERROR_OUT_OF_RANGE;
        };
        // SAFETY: checked non-null immediately above.
        unsafe { *out_unix_seconds = midnight };
        HC_OK
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
    pub unsafe extern "C" fn hc_parse_iso_date(
        text: *const c_char,
        out_fixed: *mut i64,
    ) -> HcStatus {
        if out_fixed.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let text = match unsafe { super::text(text) } {
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
            Err(error) => return status_from_calendar(error),
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
            Err(refusal) => super::status(refusal),
        }
    }

    /// `TAI - UTC` in whole seconds at a POSIX timestamp.
    ///
    /// Returns [`HC_ERROR_NO_DATA`] under the strict policy outside the published
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
            Err(error) => status_from_time(error),
        }
    }

    /// Whether a POSIX timestamp names a day that ends with an inserted leap
    /// second.
    ///
    /// A timestamp in a day whose start or whose end is not an `int64_t` —
    /// the first and last part-days of the range — is
    /// [`HC_ERROR_OUT_OF_RANGE`].
    ///
    /// # Safety
    ///
    /// `out_has_leap` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_day_has_leap_second(
        unix_seconds: i64,
        out_has_leap: *mut c_int,
    ) -> HcStatus {
        if out_has_leap.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        let Some((day_start, next_day)) = unix_seconds
            .div_euclid(86_400)
            .checked_mul(86_400)
            .and_then(|start| Some((start, start.checked_add(86_400)?)))
        else {
            return HC_ERROR_OUT_OF_RANGE;
        };
        let policy = LeapPolicy::Extrapolate;
        let before = match unix::tai_minus_utc_at(day_start, policy) {
            Ok(value) => value,
            Err(error) => return status_from_time(error),
        };
        let after = match unix::tai_minus_utc_at(next_day, policy) {
            Ok(value) => value,
            Err(error) => return status_from_time(error),
        };
        let has_leap = c_int::from(after > before);
        // SAFETY: checked non-null immediately above.
        unsafe { *out_has_leap = has_leap };
        HC_OK
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
            Err(refusal) => super::status(refusal),
        }
    }
}

#[cfg(feature = "civil")]
pub use civil::{
    hc_day_has_leap_second, hc_day_of_year, hc_fixed_from_unix, hc_format_iso_date,
    hc_gregorian_from_fixed, hc_gregorian_to_fixed, hc_is_leap_year, hc_parse_iso_date,
    hc_tai_from_unix, hc_tai_minus_utc, hc_unix_from_fixed, hc_utc_from_tai, hc_weekday,
};

/// Time scales and day counts, behind the `timestamps` feature: TAI64 labels
/// in both conventions, GNSS weeks, GLONASS dates, OLE Automation dates,
/// Excel 1900 serials, UUID timestamps, NTP eras, FAT date and time words,
/// Swatch Internet Time, Julian and Besselian epochs, and TT(BIPM) from a
/// series the caller supplies, from `hyper_calendar::time_lines`, shared
/// with the WebAssembly module. A TAI
/// instant is whole seconds from 1970-01-01 00:00:00 TAI and the
/// attoseconds into that second, from 0 to 10¹⁸ − 1; a POSIX instant and a
/// TT instant, from 1970-01-01 00:00:00 TT, cross the same way.
#[cfg(feature = "timestamps")]
mod time_scales {
    use core::ffi::{c_char, c_int};

    use hc::hc_calendars_solar::spreadsheet::Excel1900Day;
    use hc::time_lines;

    use super::{HC_ERROR_NULL_POINTER, HC_OK, HcStatus, name, status, text, write_answer};

    /// A TAI instant as a TAI64, TAI64N or TAI64NA label in lower-case
    /// hexadecimal, as one NUL-terminated UTF-8 line in a caller-owned
    /// buffer.
    ///
    /// `format` is `tai64`, `tai64n` or `tai64na`, in any case; anything
    /// else is `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`.
    /// Attoseconds from 10¹⁸, or a second that no TAI64 label can hold (the
    /// labels run below 2⁶³), is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `format` must be null or NUL-terminated; `buffer` must be writable
    /// for `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tai64_encode(
        tai_seconds: i64,
        attoseconds: u64,
        format: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let format = match unsafe { name(format) } {
            Ok(format) => format,
            Err(status) => return status,
        };
        let answer = time_lines::tai64_encode_line(tai_seconds, attoseconds, format);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// A TAI64, TAI64N or TAI64NA label in hexadecimal read back into the
    /// TAI seconds and attoseconds of the instant it names.
    ///
    /// Text that is not 16, 24 or 32 hexadecimal digits, in either case, is
    /// `HC_ERROR_MALFORMED`; a reserved label, from 2⁶³, or a counter above
    /// 999 999 999 is `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `hex` must be null or NUL-terminated, and both out-parameters must be
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tai64_decode(
        hex: *const c_char,
        out_tai_seconds: *mut i64,
        out_attoseconds: *mut u64,
    ) -> HcStatus {
        if out_tai_seconds.is_null() || out_attoseconds.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let hex = match unsafe { name(hex) } {
            Ok(hex) => hex,
            Err(status) => return status,
        };
        match time_lines::tai64_decode(hex).and_then(|(_, instant)| time_lines::tai_parts(instant))
        {
            Ok((seconds, attoseconds)) => {
                // SAFETY: both were checked non-null above.
                unsafe {
                    *out_tai_seconds = seconds;
                    *out_attoseconds = attoseconds;
                }
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The GNSS week and time of week of a TAI instant.
    ///
    /// `numbering` names the broadcast week field: `gps-lnav-week`,
    /// `gps-cnav-week`, `galileo-week`, `beidou-week` or `navic-week`, in
    /// any case; anything else is `HC_ERROR_UNKNOWN`. Writes the full week
    /// since the field's week zero, the week as the field broadcasts it,
    /// and the time of week in whole seconds and attoseconds. An instant
    /// before week zero is `HC_ERROR_NO_DATA`; attoseconds from 10¹⁸ are
    /// `HC_ERROR_OUT_OF_RANGE`, and a week past 2³² − 1 `HC_ERROR_OVERFLOW`.
    ///
    /// # Safety
    ///
    /// `numbering` must be null or NUL-terminated, and every out-parameter
    /// must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gnss_week(
        numbering: *const c_char,
        tai_seconds: i64,
        attoseconds: u64,
        out_week: *mut u32,
        out_broadcast: *mut u32,
        out_tow_seconds: *mut u32,
        out_tow_attoseconds: *mut u64,
    ) -> HcStatus {
        if out_week.is_null()
            || out_broadcast.is_null()
            || out_tow_seconds.is_null()
            || out_tow_attoseconds.is_null()
        {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let numbering = match unsafe { name(numbering) } {
            Ok(numbering) => numbering,
            Err(status) => return status,
        };
        let answer = time_lines::gnss_week(numbering, tai_seconds, attoseconds)
            .and_then(|(field, time)| Ok((field, time, time_lines::time_of_week_parts(time)?)));
        match answer {
            Ok((field, time, (seconds, attoseconds))) => {
                // SAFETY: all four were checked non-null above.
                unsafe {
                    *out_week = time.week;
                    *out_broadcast = field.broadcast(time.week);
                    *out_tow_seconds = seconds;
                    *out_tow_attoseconds = attoseconds;
                }
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The TAI instant of a full GNSS week and a time of week.
    ///
    /// `numbering` is as for `hc_gnss_week`. A time of week from 604 800 s,
    /// or attoseconds from 10¹⁸, is `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `numbering` must be null or NUL-terminated, and both out-parameters
    /// must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gnss_to_tai(
        numbering: *const c_char,
        week: u32,
        tow_seconds: u32,
        tow_attoseconds: u64,
        out_tai_seconds: *mut i64,
        out_attoseconds: *mut u64,
    ) -> HcStatus {
        if out_tai_seconds.is_null() || out_attoseconds.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let numbering = match unsafe { name(numbering) } {
            Ok(numbering) => numbering,
            Err(status) => return status,
        };
        match time_lines::gnss_to_tai(numbering, week, tow_seconds, tow_attoseconds)
            .and_then(time_lines::tai_parts)
        {
            Ok((seconds, attoseconds)) => {
                // SAFETY: both were checked non-null above.
                unsafe {
                    *out_tai_seconds = seconds;
                    *out_attoseconds = attoseconds;
                }
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The full GNSS week a broadcast week names, by a rollover rule and a
    /// reference instant.
    ///
    /// `numbering` is as for `hc_gnss_week`; `rule` is `not-before`, the
    /// first full week at or after the reference's week, or `nearest`, the
    /// one within half a rollover period of it, in any case, and anything
    /// else is `HC_ERROR_UNKNOWN`. `reference_tai_seconds` is the reference
    /// as whole TAI seconds; one before week zero counts as week zero. A
    /// broadcast week that does not fit the field, or an answer past week
    /// 2³² − 1, is `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `numbering` and `rule` must be null or NUL-terminated, and
    /// `out_week` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gnss_resolve_week(
        numbering: *const c_char,
        broadcast: u32,
        rule: *const c_char,
        reference_tai_seconds: i64,
        out_week: *mut u32,
    ) -> HcStatus {
        if out_week.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let (numbering, rule) = match unsafe { (name(numbering), name(rule)) } {
            (Ok(numbering), Ok(rule)) => (numbering, rule),
            (Err(status), _) | (_, Err(status)) => return status,
        };
        match time_lines::gnss_resolve_week(numbering, broadcast, rule, reference_tai_seconds) {
            Ok(week) => {
                // SAFETY: checked non-null above.
                unsafe { *out_week = week };
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// GLONASS's four-year interval N4 and day N_T at a TAI instant.
    ///
    /// N4 is 1 for 1996–1999 and N_T is 1 on 1 January of the interval's
    /// leap year, of GLONASS time, UTC(SU) + 3 h. The instant goes to UTC
    /// through the leap-second table: `strict` non-zero refuses outside it
    /// with `HC_ERROR_NO_DATA`, zero holds the last published offset. An
    /// instant before 1996 or from 2100, or attoseconds from 10¹⁸, is
    /// `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// Both out-parameters must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_glonass_date(
        tai_seconds: i64,
        attoseconds: u64,
        strict: c_int,
        out_four_year_interval: *mut u32,
        out_day: *mut u32,
    ) -> HcStatus {
        if out_four_year_interval.is_null() || out_day.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match time_lines::glonass_date(tai_seconds, attoseconds, strict != 0) {
            Ok(date) => {
                // SAFETY: both were checked non-null above.
                unsafe {
                    *out_four_year_interval = date.four_year_interval;
                    *out_day = date.day;
                }
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The fixed day and the time of day, in seconds, of an OLE Automation
    /// date.
    ///
    /// The integer part counts days from 30 December 1899 and the fraction
    /// is the time of day, read as a magnitude when the value is negative:
    /// −1.25 is 06:00 on 29 December 1899. A value that is not finite, or
    /// outside 1 January 100 to 31 December 9999, is
    /// `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// Both out-parameters must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_fixed_from_ole_automation(
        value: f64,
        out_fixed: *mut i64,
        out_seconds_of_day: *mut f64,
    ) -> HcStatus {
        if out_fixed.is_null() || out_seconds_of_day.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match time_lines::fixed_from_ole_automation(value) {
            Ok((day, seconds)) => {
                // SAFETY: both were checked non-null above.
                unsafe {
                    *out_fixed = day.0;
                    *out_seconds_of_day = seconds;
                }
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The OLE Automation date of a fixed day and a time of day in seconds.
    ///
    /// Before 30 December 1899 the time is subtracted, so 06:00 on
    /// 29 December 1899 is −1.25. A time of day that is not finite, negative
    /// or not below 86 400 s, or a day outside 1 January 100 to 31 December
    /// 9999, is `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `out_value` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_ole_automation_from_fixed(
        fixed: i64,
        seconds_of_day: f64,
        out_value: *mut f64,
    ) -> HcStatus {
        if out_value.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match time_lines::ole_automation_from_fixed(fixed, seconds_of_day) {
            Ok(value) => {
                // SAFETY: checked non-null above.
                unsafe { *out_value = value };
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// What an Excel 1900 serial names.
    ///
    /// `out_phantom` receives 1 for serial 60, which Excel counts as
    /// 29 February 1900, a day that never was, and `out_fixed` is then left
    /// as it was: the serial is named, not given a date. Any other serial
    /// writes its fixed day and 0. A serial below 1 or above 2 958 465,
    /// 31 December 9999, is `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// Both out-parameters must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_excel_1900_day(
        serial: i64,
        out_fixed: *mut i64,
        out_phantom: *mut c_int,
    ) -> HcStatus {
        if out_fixed.is_null() || out_phantom.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match time_lines::excel_1900_day(serial) {
            Ok(Excel1900Day::Date(day)) => {
                // SAFETY: both were checked non-null above.
                unsafe {
                    *out_fixed = day.0;
                    *out_phantom = 0;
                }
                HC_OK
            }
            Ok(Excel1900Day::Phantom29February1900) => {
                // SAFETY: checked non-null above.
                unsafe { *out_phantom = 1 };
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// A POSIX instant as a TAI64 or TAI64N label in the
    /// `tai64-posix-plus-10` convention, in lower-case hexadecimal, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// The convention is daemontools' on a clock that keeps POSIX time:
    /// 2⁶² + 10 + the POSIX seconds, with no leap-second table, so POSIX 0
    /// is `400000000000000a`; it is not `hc_tai64_encode`'s true TAI.
    /// `format` is `tai64` or `tai64n`, in any case; `tai64na`, which the
    /// convention does not write, and anything else is `HC_ERROR_UNKNOWN`,
    /// and null `HC_ERROR_NULL_POINTER`. Attoseconds from 10¹⁸, or a second
    /// whose label would fall outside 0 to 2⁶³ − 1, is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// As `hc_tai64_encode`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tai64_posix_plus_10_encode(
        unix_seconds: i64,
        attoseconds: u64,
        format: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let format = match unsafe { name(format) } {
            Ok(format) => format,
            Err(status) => return status,
        };
        let answer = time_lines::tai64_posix_plus_10_encode_line(unix_seconds, attoseconds, format);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// A TAI64 or TAI64N label in the `tai64-posix-plus-10` convention read
    /// back into the POSIX seconds and attoseconds of the instant it names.
    ///
    /// Text that is not 16 or 24 hexadecimal digits, in either case, is
    /// `HC_ERROR_MALFORMED`; a reserved label, from 2⁶³, or a nanosecond
    /// count above 999 999 999 is `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `hex` must be null or NUL-terminated, and both out-parameters must be
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tai64_posix_plus_10_decode(
        hex: *const c_char,
        out_unix_seconds: *mut i64,
        out_attoseconds: *mut u64,
    ) -> HcStatus {
        if out_unix_seconds.is_null() || out_attoseconds.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let hex = match unsafe { name(hex) } {
            Ok(hex) => hex,
            Err(status) => return status,
        };
        match time_lines::tai64_posix_plus_10_decode(hex) {
            Ok((_, unix)) => {
                // SAFETY: both were checked non-null above.
                unsafe {
                    *out_unix_seconds = unix.seconds();
                    *out_attoseconds = unix.subsec_attos();
                }
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The version, the 60-bit timestamp and the POSIX instant of a
    /// version 1 or version 6 UUID.
    ///
    /// `uuid` is RFC 9562's string form, NUL-terminated: 32 hexadecimal
    /// digits in either case, bare or hyphenated 8-4-4-4-12, optionally
    /// after `urn:uuid:`; any other text is `HC_ERROR_MALFORMED`. Writes the
    /// version, 1 or 6; the timestamp, 100-nanosecond intervals from
    /// 1582-10-15 00:00 UTC; and the POSIX seconds and attoseconds of the
    /// start of that interval. A UUID of another version or variant, which
    /// carries no timestamp, is `HC_ERROR_NO_DATA`.
    ///
    /// # Safety
    ///
    /// `uuid` must be null or NUL-terminated, and every out-parameter must
    /// be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_uuid_timestamp(
        uuid: *const c_char,
        out_version: *mut c_int,
        out_timestamp: *mut u64,
        out_unix_seconds: *mut i64,
        out_attoseconds: *mut u64,
    ) -> HcStatus {
        if out_version.is_null()
            || out_timestamp.is_null()
            || out_unix_seconds.is_null()
            || out_attoseconds.is_null()
        {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let uuid = match unsafe { name(uuid) } {
            Ok(uuid) => uuid,
            Err(status) => return status,
        };
        match time_lines::uuid_timestamp(uuid) {
            Ok((version, timestamp, unix)) => {
                // SAFETY: all four were checked non-null above.
                unsafe {
                    *out_version = c_int::from(version.number());
                    *out_timestamp = timestamp;
                    *out_unix_seconds = unix.seconds();
                    *out_attoseconds = unix.subsec_attos();
                }
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// A 64-bit NTP timestamp placed in its era by a reference POSIX
    /// second: the era, the era offset, the fraction in 2⁻⁶⁴ s units, and
    /// the POSIX seconds and attoseconds.
    ///
    /// The era is the one that puts the timestamp within 2³¹ s of
    /// `reference_unix`, from 2³¹ s before it, included, to 2³¹ s after it,
    /// excluded, as RFC 5905 §6 reads it. The zero timestamp, reserved for
    /// unknown or unsynchronised time, is `HC_ERROR_NO_DATA`; a reference so
    /// far off that the era or the second does not fit is
    /// `HC_ERROR_OVERFLOW` or `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// Every out-parameter must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_ntp_resolve(
        seconds: u32,
        fraction: u32,
        reference_unix: i64,
        out_era: *mut i32,
        out_offset: *mut u32,
        out_fraction: *mut u64,
        out_unix_seconds: *mut i64,
        out_attoseconds: *mut u64,
    ) -> HcStatus {
        if out_era.is_null()
            || out_offset.is_null()
            || out_fraction.is_null()
            || out_unix_seconds.is_null()
            || out_attoseconds.is_null()
        {
            return HC_ERROR_NULL_POINTER;
        }
        match time_lines::ntp_resolve(seconds, fraction, reference_unix) {
            Ok((date, unix)) => {
                // SAFETY: all five were checked non-null above.
                unsafe {
                    *out_era = date.era;
                    *out_offset = date.offset;
                    *out_fraction = date.fraction;
                    *out_unix_seconds = unix.seconds();
                    *out_attoseconds = unix.subsec_attos();
                }
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The 60-bit UUID timestamp of a POSIX instant, and the time fields a
    /// version 1 and a version 6 UUID write it in, as one NUL-terminated
    /// UTF-8 line in a caller-owned buffer.
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
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_uuid_timestamp_encode(
        unix_seconds: i64,
        attoseconds: u64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = time_lines::uuid_timestamp_encode_line(unix_seconds, attoseconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The NTP date and timestamp of a POSIX instant, as one NUL-terminated
    /// UTF-8 line in a caller-owned buffer.
    ///
    /// Tab-separated: the era, 0 for 1900 to 2036; the era offset; the
    /// fraction in 2⁻⁶⁴ s units, floored; the 128-bit date in RFC 5905's
    /// Figure 3 layout, era, offset and fraction, as 32 lower-case
    /// hexadecimal digits; and the 64-bit timestamp of the packet headers,
    /// the offset and the top 32 bits of the fraction with the era
    /// dropped, as 16. The seconds count whole 86 400-second days from
    /// 1900, as RFC 5905 §6's table does, with no leap second. Attoseconds
    /// from 10¹⁸ is `HC_ERROR_OUT_OF_RANGE`, and a second so late that its
    /// count from 1900 leaves an `int64_t` is `HC_ERROR_OVERFLOW`. Writes
    /// the required length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// As `hc_uuid_timestamp_encode`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_ntp_encode(
        unix_seconds: i64,
        attoseconds: u64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = time_lines::ntp_encode_line(unix_seconds, attoseconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The local reading a FAT date word and time word name: its fixed day
    /// and the seconds into it, always even.
    ///
    /// The words are a wall-clock reading in a zone they do not record, so
    /// the day is a local one. A word whose fields name no day or no time —
    /// month 0 or above 12, day 0 or one the month lacks, hour above 23,
    /// minute above 59, halved second above 29 — is
    /// `HC_ERROR_INVALID_DATE`.
    ///
    /// # Safety
    ///
    /// Both out-parameters must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_fat_decode(
        date: u16,
        time: u16,
        out_fixed: *mut i64,
        out_seconds_of_day: *mut u32,
    ) -> HcStatus {
        if out_fixed.is_null() || out_seconds_of_day.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match time_lines::fat_decode(u32::from(date), u32::from(time)) {
            Ok((day, seconds)) => {
                // SAFETY: both were checked non-null above.
                unsafe {
                    *out_fixed = day.0;
                    *out_seconds_of_day = seconds;
                }
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The FAT date and time words of a fixed day and a time of day in
    /// whole seconds, the second rounded down to an even one.
    ///
    /// A time of day from 86 400 s, or a day outside 1980 to 2107, the
    /// years the date word holds, is `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// Both out-parameters must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_fat_encode(
        fixed: i64,
        seconds_of_day: u32,
        out_date: *mut u16,
        out_time: *mut u16,
    ) -> HcStatus {
        if out_date.is_null() || out_time.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match time_lines::fat_encode(fixed, seconds_of_day) {
            Ok((date, time)) => {
                // SAFETY: both were checked non-null above.
                unsafe {
                    *out_date = date;
                    *out_time = time;
                }
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The Swatch Internet Time at a POSIX instant, 0 through 999: the
    /// thousandth of the day of Biel Mean Time, UTC+1 all year, that it
    /// falls in, so @000 begins at 23:00 UTC.
    ///
    /// Attoseconds from 10¹⁸ are `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `out_beat` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_swatch_beat(
        unix_seconds: i64,
        attoseconds: u64,
        out_beat: *mut u16,
    ) -> HcStatus {
        if out_beat.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match time_lines::swatch_beat(unix_seconds, attoseconds) {
            Ok(beat) => {
                // SAFETY: checked non-null above.
                unsafe { *out_beat = beat };
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The Julian or Besselian epoch of a TT instant, as a year with a
    /// fraction.
    ///
    /// `notation` is `J` or `julian-epoch`, or `B` or `besselian-epoch`, in
    /// any case; anything else is `HC_ERROR_UNKNOWN`, and null
    /// `HC_ERROR_NULL_POINTER`. The instant is whole seconds from
    /// 1970-01-01 00:00:00 TT and attoseconds; attoseconds from 10¹⁸ are
    /// `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `notation` must be null or NUL-terminated, and `out_year` must be
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_epoch_from_tt(
        notation: *const c_char,
        tt_seconds: i64,
        attoseconds: u64,
        out_year: *mut f64,
    ) -> HcStatus {
        if out_year.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let notation = match unsafe { name(notation) } {
            Ok(notation) => notation,
            Err(status) => return status,
        };
        match time_lines::epoch_from_tt(notation, tt_seconds, attoseconds) {
            Ok((_, year)) => {
                // SAFETY: checked non-null above.
                unsafe { *out_year = year };
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The TT instant of a Julian or Besselian epoch, as whole seconds from
    /// 1970-01-01 00:00:00 TT and attoseconds.
    ///
    /// `notation` is as for `hc_epoch_from_tt`, or null or empty for an
    /// epoch written without a letter, which SOFA reads as Besselian before
    /// 1984.0 and Julian from it. A year that is not finite is
    /// `HC_ERROR_OUT_OF_RANGE`, and one whose instant leaves an `int64_t` of
    /// seconds `HC_ERROR_OVERFLOW`.
    ///
    /// # Safety
    ///
    /// `notation` must be null or NUL-terminated, and both out-parameters
    /// must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tt_from_epoch(
        notation: *const c_char,
        year: f64,
        out_tt_seconds: *mut i64,
        out_attoseconds: *mut u64,
    ) -> HcStatus {
        if out_tt_seconds.is_null() || out_attoseconds.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let notation = match unsafe { text(notation) } {
            Ok(notation) => notation.unwrap_or(""),
            Err(status) => return status,
        };
        match time_lines::tt_from_epoch(notation, year) {
            Ok((_, seconds, attoseconds)) => {
                // SAFETY: both were checked non-null above.
                unsafe {
                    *out_tt_seconds = seconds;
                    *out_attoseconds = attoseconds;
                }
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// TT(BIPM) at a TAI instant, read from a realisation the caller
    /// supplies, as one NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// `series` is the realisation as NUL-terminated text, one line per
    /// sample: the Modified Julian Date at 0 h UTC and TT(BIPMxx) − TAI −
    /// 32.184 s there in microseconds, separated by a tab, the dates
    /// ascending, as the first and third columns of the BIPM's `TTBIPM`
    /// files give them; text in any other shape is `HC_ERROR_MALFORMED`,
    /// and null `HC_ERROR_NULL_POINTER`. The line is the WebAssembly
    /// module's: TT(BIPMxx) − TT(TAI) in seconds, TT(BIPMxx) − TAI as whole
    /// seconds and attoseconds, and the TT(BIPMxx) reading of the instant
    /// as whole seconds and attoseconds. An instant outside the series, or
    /// an empty series, is `HC_ERROR_NO_DATA`; `strict` non-zero refuses a
    /// sample outside the leap-second table the same way. Attoseconds from
    /// 10¹⁸ are `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
    /// including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `series` must be null or NUL-terminated; `buffer` must be writable
    /// for `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_tt_bipm(
        series: *const c_char,
        tai_seconds: i64,
        attoseconds: u64,
        strict: c_int,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let series = match unsafe { name(series) } {
            Ok(series) => series,
            Err(status) => return status,
        };
        let answer = time_lines::tt_bipm_line(series, tai_seconds, attoseconds, strict != 0);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }
}

#[cfg(feature = "timestamps")]
pub use time_scales::{
    hc_epoch_from_tt, hc_excel_1900_day, hc_fat_decode, hc_fat_encode,
    hc_fixed_from_ole_automation, hc_glonass_date, hc_gnss_resolve_week, hc_gnss_to_tai,
    hc_gnss_week, hc_ntp_encode, hc_ntp_resolve, hc_ole_automation_from_fixed, hc_swatch_beat,
    hc_tai64_decode, hc_tai64_encode, hc_tai64_posix_plus_10_decode, hc_tai64_posix_plus_10_encode,
    hc_tt_bipm, hc_tt_from_epoch, hc_uuid_timestamp, hc_uuid_timestamp_encode,
};

/// Every calendar, behind the `calendars` feature: the registry the facade
/// populates, described for one day, walked as eras, years, months and
/// days, and listed, in the vocabulary of a locale; the locales
/// themselves; when each country adopted the Gregorian calendar; and the
/// month and weekday names a government decreed for a period. The lines
/// are `hyper_calendar::lines`', shared with the WebAssembly module.
#[cfg(feature = "calendars")]
mod calendars {
    use core::ffi::c_char;

    use hc::hc_calendar::Rd;
    use hc::hc_calendar::units::Unit;
    use hc::lines;

    use super::{
        HC_ERROR_NULL_POINTER, HC_ERROR_UNKNOWN, HC_OK, HcStatus, text, write_answer, write_text,
    };

    /// One fixed day in every registered calendar, as NUL-terminated UTF-8
    /// lines in a caller-owned buffer.
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
    /// why. `locale` is a NUL-terminated BCP 47 tag, `native` for each
    /// calendar's own language, or null; a calendar the tag's data does not
    /// name is rendered in English, else in the tag with the calendar's own
    /// names, never in the calendar's own language, and the last column
    /// says which. A tag that does not
    /// parse is the root locale `und`, whose month names are CLDR's
    /// `M01`..`M12` — ask for `en` for English. A `locale` that is not UTF-8
    /// is `HC_ERROR_NOT_UTF8`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `locale` must be null or point to a NUL-terminated string; `buffer`
    /// must be writable for `capacity` bytes and `written` must be null or
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_describe_day(
        fixed: i64,
        locale: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        let text = lines::describe_day(&hc::registry(), Rd(fixed), tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }

    /// The days from `from_fixed` up to but not including `to_fixed` as one
    /// calendar's eras, years, months or days, as NUL-terminated UTF-8 lines
    /// in a caller-owned buffer.
    ///
    /// `id` is a NUL-terminated registry identifier — `gregory`, `chinese`,
    /// `japanese` — and `unit` is `0` for eras, `1` for years, `2` for
    /// months and `3` for days; a null `id` is `HC_ERROR_NULL_POINTER` and an
    /// identifier or unit the library does not know is `HC_ERROR_UNKNOWN`.
    /// One line per span, in order and touching end to start, tab-separated:
    /// the first day of the span, the day after its last, its label in the
    /// locale (令和元年, `Adar I`, 閏二月, 初四), `1` for an intercalary unit,
    /// the standing of its first day, the error code, the error name and the
    /// locale used. The first and last spans are whole units and may reach
    /// outside the range asked for. A span the calendar refuses — days before
    /// its epoch or past its table, a unit it does not have — has an empty
    /// label, leap flag and standing and carries the refusal's code and name.
    /// An empty range writes an empty string, and a range of more than
    /// 100 000 spans, `hyper_calendar::lines`' `MAX_CALENDAR_UNITS`, is
    /// `HC_ERROR_OUT_OF_RANGE` with nothing written: the text would grow
    /// without bound, one line a unit, and a caller that wants more asks in
    /// pieces. `locale` is as for `hc_describe_day`, `native` included.
    /// Writes the required length, including the terminator, into
    /// `written`.
    ///
    /// # Safety
    ///
    /// `id` and `locale` must be null or point to NUL-terminated strings;
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_calendar_units(
        id: *const c_char,
        unit: u32,
        from_fixed: i64,
        to_fixed: i64,
        locale: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let id = match unsafe { text(id) } {
            Ok(Some(id)) => id,
            Ok(None) => return HC_ERROR_NULL_POINTER,
            Err(status) => return status,
        };
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        let Some(unit) = Unit::from_index(unit) else {
            return HC_ERROR_UNKNOWN;
        };
        let registry = hc::registry();
        let Some(calendar) = registry.get_by_name(id) else {
            return HC_ERROR_UNKNOWN;
        };
        let answer = lines::calendar_units(calendar, unit, Rd(from_fixed), Rd(to_fixed), tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// Every registered calendar, as NUL-terminated UTF-8 lines in a
    /// caller-owned buffer.
    ///
    /// One line per calendar, in registry order, tab-separated: the
    /// identifier, what the locale calls the calendar (和暦, or empty where
    /// it has no name), its English name, the earliest and latest fixed days
    /// it converts (empty where unbounded), whether it has eras, years,
    /// months and days as `1` or `0` each, the languages its sources are
    /// written in as BCP 47 tags joined by `;` (empty for a day count, a
    /// proposal or the Gregorian family), and its standing on `today`
    /// (`in-use`, `proleptic`, `extended` or `unrecorded`). `locale` is as
    /// for `hc_describe_day`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `locale` must be null or point to a NUL-terminated string; `buffer`
    /// must be writable for `capacity` bytes and `written` must be null or
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_calendars(
        today: i64,
        locale: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        let text = lines::calendars(&hc::registry(), Rd(today), tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }

    /// Every registered calendar by name alone, as NUL-terminated UTF-8
    /// lines in a caller-owned buffer.
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
    /// for a menu of calendars, which is asked for far more often than a
    /// day is described. `locale` is as for `hc_describe_day`. Writes the
    /// required length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `locale` must be null or point to a NUL-terminated string; `buffer`
    /// must be writable for `capacity` bytes and `written` must be null or
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_calendar_list(
        locale: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        let text = lines::calendar_list(&hc::registry(), tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }

    /// Every locale the library carries, as NUL-terminated UTF-8 lines in a
    /// caller-owned buffer.
    ///
    /// One line per locale, in tag order, tab-separated: the BCP 47 tag, the
    /// language's name in English and in itself, whether the locale's own
    /// data names the Gregorian months, the weekdays and the Gregorian eras
    /// as `1` or `0` each, and the identifiers of the calendars it has
    /// vocabulary of its own for beyond the shared Gregorian months, joined
    /// by `;`. Writes the required length, including the terminator, into
    /// `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_locales(
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let text = lines::locales();
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }

    /// The ISO 8601 weekday of the first day of the week in a locale, Monday
    /// = 1 through Sunday = 7.
    ///
    /// `locale` is a NUL-terminated BCP 47 tag or null, read as `hc-i18n`
    /// reads CLDR 48's week data: a `-u-fw-` key first, then the tag's
    /// region (`en-US` is 7, `en-GB` 1), then, for a tag without a region,
    /// the region its language's likely subtags give (`ja` is 7, `fr` 1).
    /// A null `locale`, a tag that does not parse, and `native` are the
    /// root locale `und`, whose week begins on the world's Monday. A
    /// `locale` that is not UTF-8 is `HC_ERROR_NOT_UTF8`.
    ///
    /// # Safety
    ///
    /// `locale` must be null or point to a NUL-terminated string, and
    /// `out_weekday` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_first_day_of_week(
        locale: *const c_char,
        out_weekday: *mut u8,
    ) -> HcStatus {
        if out_weekday.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        // SAFETY: checked non-null above.
        unsafe { *out_weekday = lines::first_day_of_week(tag) };
        HC_OK
    }

    /// The steps by which a country adopted the Gregorian calendar, as
    /// NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// `region` is a NUL-terminated ISO 3166-1 alpha-2 code, in either
    /// case; a null `region` is `HC_ERROR_NULL_POINTER` and one that is not
    /// UTF-8 is `HC_ERROR_NOT_UTF8`. One line per step, oldest first,
    /// tab-separated: the last day of the old reckoning and the first day
    /// of the new as fixed days, the old calendar's registry identifier
    /// (`julian`, `japanese-tenpo`, `dangi`, `chinese`, `islamic-umalqura`,
    /// `rumi`, `swedish-1700`), the scope (`civil` for the civil calendar
    /// of the whole polity as it then was, `partial` for part of the
    /// country, some purposes or part of the calendar, and `ecclesiastical`
    /// for a church's calendar alone), the instrument behind the step with
    /// its date and whether it was read, the new calendar's identifier
    /// (`gregory`, except for Sweden's steps of 1700 to `swedish-1700` and
    /// of 1712 back to `julian`), and who took the step, in English. A
    /// staged adoption is several lines — China in 1912 and 1929, Sweden in
    /// 1700, 1712 and 1753, the Dutch provinces — and a code the library
    /// does not know writes an empty string, which is not a claim that the
    /// country never adopted the calendar. Writes the required length,
    /// including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `region` must be null or point to a NUL-terminated string; `buffer`
    /// must be writable for `capacity` bytes and `written` must be null or
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gregorian_adoption(
        region: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let region = match unsafe { text(region) } {
            Ok(Some(region)) => region,
            Ok(None) => return HC_ERROR_NULL_POINTER,
            Err(status) => return status,
        };
        let text = lines::gregorian_adoption(region);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }

    /// Which month and weekday names a locale writes for a calendar on a
    /// fixed day, where a government renamed them for a period, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// `calendar` is a NUL-terminated registry identifier; null is
    /// `HC_ERROR_NULL_POINTER` and one the registry does not carry
    /// `HC_ERROR_UNKNOWN`. `locale` is as for `hc_describe_day`. The line is
    /// the WebAssembly module's: `in-force`, `undecided` or `ordinary`, then
    /// for a period its identifier, its names for the day's month and
    /// weekday, the weekday name's English meaning, the three fixed days
    /// that bound it, and its sources. Writes the required length,
    /// including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `calendar` and `locale` must be null or point to NUL-terminated
    /// strings; `buffer` must be writable for `capacity` bytes and
    /// `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_naming_period_on(
        calendar: *const c_char,
        fixed: i64,
        locale: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let calendar = match unsafe { text(calendar) } {
            Ok(Some(calendar)) => calendar,
            Ok(None) => return HC_ERROR_NULL_POINTER,
            Err(status) => return status,
        };
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        let answer = lines::naming_period_line(&hc::registry(), tag, calendar, fixed);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
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
/// cycle, and the days of the Asian calendar as it writes them, from
/// `hyper_calendar`'s `panchanga_lines`, `hindu_lines`, `crescent_lines`
/// and `calendar_values`, shared with the WebAssembly module.
#[cfg(feature = "calendars")]
mod calendar_days {
    use core::ffi::c_char;

    use hc::{astro_lines, calendar_values, crescent_lines, hindu_lines, panchanga_lines};

    use super::{HC_ERROR_NULL_POINTER, HC_OK, HcStatus, name, status, write_answer};

    /// One number through an out-parameter.
    ///
    /// # Safety
    ///
    /// `out` must be null or writable.
    unsafe fn put(answer: hc::boundary::Answer<i64>, out: *mut i64) -> HcStatus {
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

    /// The yoga and the karaṇa in progress at a POSIX timestamp, as two
    /// NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// The lines are the WebAssembly module's: the limb, its number, its
    /// English and Devanagari names, the instants it began and ends and the
    /// instant it was read at, and the ayanamsa of the yoga. `ayanamsa` is
    /// `Lahiri (Chitrapaksha)`, `Raman`, `Krishnamurti` or `Fagan-Bradley`,
    /// or the first word of one, in any case; anything else is
    /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. An instant
    /// outside the years −1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes
    /// the required length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `ayanamsa` must be null or NUL-terminated; `buffer` must be writable
    /// for `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_panchanga_at(
        unix_seconds: i64,
        ayanamsa: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let ayanamsa = match unsafe { name(ayanamsa) } {
            Ok(ayanamsa) => ayanamsa,
            Err(status) => return status,
        };
        let answer = panchanga_lines::panchanga_at_lines(unix_seconds, ayanamsa);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The yoga and the karaṇa a fixed day carries at a place, the ones in
    /// progress at its sunrise, as two NUL-terminated UTF-8 lines in a
    /// caller-owned buffer.
    ///
    /// The lines of `hc_panchanga_at`, read at the day's sunrise at the
    /// latitude and longitude in degrees and the elevation in metres. A day
    /// on which the Sun does not rise there is `HC_ERROR_NO_DATA`; a place
    /// off the globe, or a day outside the years −1000 to 3000, is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
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
        ayanamsa: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let ayanamsa = match unsafe { name(ayanamsa) } {
            Ok(ayanamsa) => ayanamsa,
            Err(status) => return status,
        };
        let answer = astro_lines::location(latitude, longitude, elevation)
            .and_then(|place| panchanga_lines::panchanga_of_day_lines(fixed, place, ayanamsa));
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The Hindu lunisolar date of a fixed day at a place, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// The line is the WebAssembly module's: the Śaka year, the Vikrama
    /// year, the month, 1 or 0 for the intercalary month, the tithi, 1 or 0
    /// for a repeated tithi, and the sunrise the day was read at as POSIX
    /// seconds. `sky` is an ayanamsa `hc_panchanga_at` names, for the true
    /// Sun and Moon, or `surya-siddhanta`, for the *Sūrya Siddhānta*'s, in
    /// any case; anything else is `HC_ERROR_UNKNOWN`, and null
    /// `HC_ERROR_NULL_POINTER`. A place beyond 65° of latitude, where
    /// some day of the year has no sunrise, is `HC_ERROR_OUT_OF_RANGE` on
    /// either sky, as is a place off the globe, a day outside Śaka 1622
    /// through 2221 on the true sky, Chaitra śukla 1 in March 1700 to the
    /// eve of the one in March 2300, and a day outside Kali Yuga 1 to
    /// 10 000 on the Siddhānta's. Writes the
    /// required length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `sky` must be null or NUL-terminated; `buffer` must be writable for
    /// `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_hindu_lunar_date(
        sky: *const c_char,
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let sky = match unsafe { name(sky) } {
            Ok(sky) => sky,
            Err(status) => return status,
        };
        let answer = astro_lines::location(latitude, longitude, elevation)
            .and_then(|place| hindu_lines::hindu_lunar_date_line(sky, fixed, place));
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The *Sūrya Siddhānta*'s Sun and Moon at a POSIX timestamp, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// The line is the WebAssembly module's: the Sun's and the Moon's
    /// sidereal longitudes, the elongation, the tithi and the Sun's sign.
    /// An instant outside the days of Kali Yuga 1 to 10 000 is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_surya_siddhanta_at(
        unix_seconds: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = hindu_lines::surya_siddhanta_line(unix_seconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The *Sūrya Siddhānta*'s sunrise on a fixed day at a place, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// The one cell is the instant as POSIX seconds, rounded down. A day
    /// outside Kali Yuga 1 to 10 000, a place beyond 65° of latitude, or one
    /// off the globe, is `HC_ERROR_OUT_OF_RANGE`. Writes the required
    /// length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// As `hc_surya_siddhanta_at`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_surya_siddhanta_sunrise(
        fixed: i64,
        latitude: f64,
        longitude: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = astro_lines::location(latitude, longitude, 0.0)
            .and_then(|place| hindu_lines::surya_siddhanta_sunrise_line(fixed, place));
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// Whether the young crescent should have been visible on the evening
    /// that begins a fixed day, from a place, by a named criterion, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// The line is the WebAssembly module's: 1 or 0, the moment the evening
    /// is judged at as POSIX seconds, and the Moon's longitude less the
    /// Sun's (0 to 360), arc of light, altitude and arc of vision and the crescent's width there,
    /// empty where there is no such moment. `criterion` is `shaukat`,
    /// `yallop` or `saudi-rule`, in any case; anything else is
    /// `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. A place off the
    /// globe, or a day outside the years −1000 to 3000, is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `criterion` must be null or NUL-terminated; `buffer` must be writable
    /// for `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_crescent_visible(
        criterion: *const c_char,
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let criterion = match unsafe { name(criterion) } {
            Ok(criterion) => criterion,
            Err(status) => return status,
        };
        let answer = astro_lines::location(latitude, longitude, elevation)
            .and_then(|place| crescent_lines::crescent_line(criterion, fixed, place));
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The number of the modern Olympiad a Gregorian year belongs to.
    ///
    /// 1 for 1896–1899, under the Olympic Charter's definition, whether or
    /// not its Games were held; a year before 1896 is
    /// `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `out_olympiad` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_ioc_olympiad(
        gregorian_year: i64,
        out_olympiad: *mut i64,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { put(calendar_values::ioc_olympiad(gregorian_year), out_olympiad) }
    }

    /// The fixed day of the yahrzeit in a Hebrew year of a death on the
    /// Hebrew date a fixed day names.
    ///
    /// `death_fixed` is the fixed day whose daylight carries the Hebrew
    /// date of the death — a death after sunset is the next fixed day —
    /// and the rules for the dates a later year may lack are Reingold and
    /// Dershowitz's. A day or a year outside the Hebrew years 1 to 9999 is
    /// `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `out_fixed` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_hebrew_yahrzeit(
        death_fixed: i64,
        hebrew_year: i64,
        out_fixed: *mut i64,
    ) -> HcStatus {
        let answer = calendar_values::hebrew_yahrzeit(death_fixed, hebrew_year);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { put(answer, out_fixed) }
    }

    /// The fixed day of the birthday in a Hebrew year of a birth on the
    /// Hebrew date a fixed day names.
    ///
    /// As `hc_hebrew_yahrzeit`, by Reingold and Dershowitz's
    /// `hebrew-birthday`: a birth in the last month of a year is kept in the
    /// last month of the later one.
    ///
    /// # Safety
    ///
    /// `out_fixed` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_hebrew_birthday(
        birth_fixed: i64,
        hebrew_year: i64,
        out_fixed: *mut i64,
    ) -> HcStatus {
        let answer = calendar_values::hebrew_birthday(birth_fixed, hebrew_year);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { put(answer, out_fixed) }
    }

    /// A person's age as the Chinese count reckons it on a fixed day.
    ///
    /// One at birth and one more at each Chinese New Year after, as
    /// Reingold and Dershowitz's `chinese-age` counts it. The birth crosses
    /// as its fixed day. A day before the birth has no age and is
    /// `HC_ERROR_NO_DATA`; a day outside the Chinese calendar's range is
    /// `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `out_age` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_chinese_reckoned_age(
        birth_fixed: i64,
        on_fixed: i64,
        out_age: *mut u32,
    ) -> HcStatus {
        if out_age.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match calendar_values::chinese_reckoned_age(birth_fixed, on_fixed) {
            Ok(age) => {
                // SAFETY: checked non-null above.
                unsafe { *out_age = age };
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The marriage augury of a Chinese year, as one NUL-terminated UTF-8
    /// line in a caller-owned buffer.
    ///
    /// The line is the WebAssembly module's: `widow`, `blind`, `bright` or
    /// `double-bright`, then `1` or `0` for whether 立春 falls after the
    /// year's New Year and whether another falls before the next.
    /// `chinese_year` is the Chinese calendar's own count, 4661 for the year
    /// that began on 10 February 2024; outside its range is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_chinese_marriage_augury(
        chinese_year: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = calendar_values::chinese_marriage_augury_line(chinese_year);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The place of a Hebrew year in the seven-year sabbatical cycle, 1
    /// through 7.
    ///
    /// 7 is the sabbatical year, *shemittah*, as the years published today
    /// count it: 5782 and 5789 are sabbatical years. A year outside the
    /// Hebrew years 1 to 9999 is `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `out_place` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_hebrew_sabbatical_cycle_year(
        hebrew_year: i64,
        out_place: *mut i64,
    ) -> HcStatus {
        let answer = calendar_values::hebrew_sabbatical_cycle_year(hebrew_year);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { put(answer, out_place) }
    }

    /// A fixed day in the calendar of the Roman province of Asia as the
    /// calendar writes it, unnumbered days included, as one NUL-terminated
    /// UTF-8 line in a caller-owned buffer.
    ///
    /// The line is the WebAssembly module's: the Julian year in which the
    /// Asian year began, the month, its name, `unnumbered` or `numbered`,
    /// and the day's number or its place among the unnumbered days. A day
    /// outside 23 September AD 4 to the end of the Asian year 9999 is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_asian_day(
        fixed: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = calendar_values::asian_day_line(fixed);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
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
    use core::ffi::{c_char, c_int};

    use hc::hc_calendar::Rd;
    use hc::hc_holiday::engine::HolidayCalendar;
    use hc::hc_holiday::hc_calendars_solar::gregorian;
    use hc::hc_holiday::rule::{Confidence, Kind, RuleSet};
    use hc::hc_holiday::{countries, exchanges, international, traditions};

    use super::{
        HC_ERROR_NULL_POINTER, HC_ERROR_OUT_OF_RANGE, HC_ERROR_UNKNOWN, HC_OK, HcStatus, push_cell,
        text, write_text,
    };

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
    fn tables() -> impl Iterator<Item = &'static RuleSet> {
        hc::holiday_lines::tables()
    }

    /// The table and region two string arguments name.
    ///
    /// # Errors
    ///
    /// [`HC_ERROR_NULL_POINTER`] for a null `code`, [`HC_ERROR_NOT_UTF8`] for
    /// either argument not being UTF-8, and [`HC_ERROR_UNKNOWN`] for a code
    /// that names no table. A null or empty `region` is no region.
    ///
    /// # Safety
    ///
    /// Each pointer must be null or point to a NUL-terminated string.
    unsafe fn table_and_region<'a>(
        code: *const c_char,
        region: *const c_char,
    ) -> Result<(&'static RuleSet, Option<&'a str>), HcStatus> {
        // SAFETY: forwarded to the caller's contract above.
        let code = unsafe { text(code) }?.ok_or(HC_ERROR_NULL_POINTER)?;
        // SAFETY: forwarded to the caller's contract above.
        let region = unsafe { text(region) }?.filter(|region| !region.is_empty());
        let table = table(code).ok_or(HC_ERROR_UNKNOWN)?;
        Ok((table, region))
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
    /// [`HC_ERROR_OUT_OF_RANGE`] for a day with no Gregorian year.
    pub(super) fn lines_on(fixed: i64) -> Result<String, HcStatus> {
        use core::fmt::Write;
        let day = Rd(fixed);
        gregorian::year_from_fixed(day).map_err(|_| HC_ERROR_OUT_OF_RANGE)?;
        let mut out = String::new();
        for table in tables() {
            let calendar = HolidayCalendar::for_day(table, None, day);
            for holiday in calendar.on(day) {
                push_cell(&mut out, table.code);
                out.push('\t');
                push_cell(&mut out, table.english_name);
                out.push('\t');
                push_cell(&mut out, holiday.name);
                out.push('\t');
                push_cell(&mut out, holiday.local_name);
                let _ = write!(
                    out,
                    "\t{}\t{}\t",
                    kind_name(holiday.kind),
                    confidence_name(holiday.confidence)
                );
                push_cell(&mut out, holiday.source);
                let _ = write!(out, "\t{}\t", u8::from(holiday.is_substitute()));
                if let Some(observed_for) = holiday.observed_for {
                    let _ = write!(out, "{}", observed_for.0);
                }
                out.push('\n');
            }
            // A gap is a holiday the table could not place this year — its
            // calendar's range ended, or no announcement was read — and it
            // is reported rather than left out, so that a caller can say so.
            for gap in calendar.gaps() {
                push_cell(&mut out, table.code);
                out.push('\t');
                push_cell(&mut out, table.english_name);
                out.push('\t');
                push_cell(&mut out, gap.name);
                out.push('\t');
                push_cell(&mut out, gap.local_name);
                out.push_str("\tgap\t\t\t0\t\n");
            }
        }
        Ok(out)
    }

    fn iso(day: Rd) -> String {
        hc::civil::Date::from_ordinal(day.0)
            .map_or_else(|_| String::from("?"), |date| date.to_string())
    }

    /// Whether a fixed day is a day off in a holiday table.
    ///
    /// `code` is a NUL-terminated table identifier — a country's ISO 3166-1
    /// alpha-2 code, an exchange's ISO 10383 Market Identifier Code, a
    /// tradition's slug or `un-days` — and `region`, which may be null, a
    /// subdivision's ISO 3166-2 code. Writes 1 or 0 to `out_is_day_off`.
    /// A null `code` or `out_is_day_off` is `HC_ERROR_NULL_POINTER`, a
    /// string that is not UTF-8 `HC_ERROR_NOT_UTF8`, and a code that names
    /// no table `HC_ERROR_UNKNOWN`.
    ///
    /// # Safety
    ///
    /// `code` must point to a NUL-terminated string, `region` must be null
    /// or do the same, and `out_is_day_off` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holiday_is_day_off(
        code: *const c_char,
        region: *const c_char,
        fixed: i64,
        out_is_day_off: *mut c_int,
    ) -> HcStatus {
        if out_is_day_off.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let (table, region) = match unsafe { table_and_region(code, region) } {
            Ok(found) => found,
            Err(status) => return status,
        };
        let day = Rd(fixed);
        if gregorian::year_from_fixed(day).is_err() {
            return HC_ERROR_OUT_OF_RANGE;
        }
        let answer = HolidayCalendar::for_day(table, region, day).is_holiday(day);
        // SAFETY: checked non-null above; the caller guarantees it is writable.
        unsafe { *out_is_day_off = c_int::from(answer) };
        HC_OK
    }

    /// The holidays of a Gregorian year in a table, as NUL-terminated UTF-8
    /// lines in a caller-owned buffer.
    ///
    /// One line per entry, tab-separated: the ISO 8601 date, the name, the
    /// local name, the kind (`public`, `bank`, `religious`, `observance`,
    /// `school` or `workday`), the confidence (`exact` or `approximate`), `1` for a
    /// substitute day and `0` otherwise, and the date the substitute
    /// stands in for or nothing. Writes the required length, including the
    /// terminator, into `written`. The string arguments fail as for
    /// `hc_holiday_is_day_off`.
    ///
    /// # Safety
    ///
    /// `code` and `region` as for `hc_holiday_is_day_off`; `buffer` must be
    /// writable for `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holidays_in_year(
        code: *const c_char,
        region: *const c_char,
        year: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let (table, region) = match unsafe { table_and_region(code, region) } {
            Ok(found) => found,
            Err(status) => return status,
        };
        let text = lines(table, region, year);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }

    /// The identifier of every holiday table, one per line, NUL-terminated.
    ///
    /// Countries first, then exchanges, traditions and the international
    /// sets, each as `hc_holiday_is_day_off` accepts it. Writes the
    /// required length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holiday_codes(
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let text = codes();
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }

    /// Every holiday on one fixed day across every table, as NUL-terminated
    /// UTF-8 lines in a caller-owned buffer.
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
    /// caller can say the year is unanswered rather than show nothing. A
    /// day with no Gregorian year is `HC_ERROR_OUT_OF_RANGE`. Writes the
    /// required length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holidays_on(
        fixed: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let text = match lines_on(fixed) {
            Ok(text) => text,
            Err(status) => return status,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }
}

#[cfg(feature = "holiday")]
pub use holiday::{hc_holiday_codes, hc_holiday_is_day_off, hc_holidays_in_year, hc_holidays_on};

/// The tables and the liturgical year, behind the `holiday` feature: every
/// holiday table described, the lectionary cycles of a day, the
/// astronomical Easter, the Holy Year a day falls in and the ranks of the
/// *Common Worship* celebrations of a day, from
/// `hyper_calendar::holiday_lines`, shared with the WebAssembly module.
#[cfg(feature = "holiday")]
mod observances {
    use core::ffi::c_char;

    use hc::holiday_lines;

    use super::{HC_ERROR_NULL_POINTER, HC_OK, HcStatus, status, text, write_answer, write_text};

    /// Every holiday table with its kind, names and sources, as
    /// NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// The lines are the WebAssembly module's: one per table in
    /// `hc_holiday_codes` order, with the code, the kind (`country`,
    /// `subdivision`, `exchange`, `tradition` or `observance`), the name in
    /// the locale, the English name, the locale that answered, the sources,
    /// the ISO 3166-1 country of a subdivision or an exchange where its
    /// table records one, and the short name in the locale. A country is
    /// named by its CLDR 48 territory name
    /// in the `locale` where `hc-i18n` carries one, and else, as for a null
    /// `locale`, by CLDR's English one; every other table by its English
    /// name; the tag that answered is in column 5. Column 8 is CLDR 48's
    /// `alt="short"` name of a country, from the data that named it —
    /// `Hong Kong` for `HK` under `en`, 香港 under `ja` — and empty where
    /// the data has none and for every table that is not a country. Writes the required
    /// length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `locale` must be null or NUL-terminated; `buffer` must be writable
    /// for `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holiday_tables(
        locale: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        let lines = holiday_lines::holiday_tables(tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&lines, buffer, capacity, written) }
    }

    /// The lectionary cycles of a fixed day, as one NUL-terminated UTF-8
    /// line in a caller-owned buffer.
    ///
    /// The line is the WebAssembly module's: the liturgical year, the
    /// Sunday cycle `A`, `B` or `C`, the Roman weekday cycle `I` or `II`,
    /// and the RCL's Proper for a Sunday after Trinity Sunday, else empty.
    /// A day outside the liturgical years 1583 to 4099 is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_lectionary(
        fixed: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = holiday_lines::lectionary_line(fixed);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The fixed day of Easter Sunday of a Gregorian year by the
    /// astronomical reckoning at the meridian of Jerusalem.
    ///
    /// The first Sunday after the day, by apparent solar time at Jerusalem,
    /// of the first full moon at or after the March equinox, as the World
    /// Council of Churches' Aleppo statement of 1997 proposed. A year
    /// outside 1583 to 2150 is `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `out_fixed` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_astronomical_easter(year: i64, out_fixed: *mut i64) -> HcStatus {
        if out_fixed.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match holiday_lines::astronomical_easter(year) {
            Ok(day) => {
                // SAFETY: checked non-null above.
                unsafe { *out_fixed = day };
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The fixed day of the paschal full moon of a Gregorian year by the
    /// astronomical reckoning at the meridian of Jerusalem.
    ///
    /// The day, by apparent solar time at Jerusalem, of the first full moon
    /// at or after the March equinox: the day `hc_astronomical_easter` is
    /// the first Sunday after, so a full moon on a Sunday puts Easter a
    /// week later. A year outside 1583 to 2150 is `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `out_fixed` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_astronomical_paschal_full_moon(
        year: i64,
        out_fixed: *mut i64,
    ) -> HcStatus {
        if out_fixed.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        match holiday_lines::astronomical_paschal_full_moon(year) {
            Ok(day) => {
                // SAFETY: checked non-null above.
                unsafe { *out_fixed = day };
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// The Holy Year of the Catholic Church a fixed day falls in, if any,
    /// as one NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// The line is the WebAssembly module's: `within` or `outside`, then
    /// within a jubilee its title, kind, Pope and bull, the day the bull was
    /// given, its first and last days in Rome and in the dioceses. A day
    /// before 24 December 1974 or after 27 September 2026, the day the
    /// table's sources were checked, is `HC_ERROR_NO_DATA`. Writes the
    /// required length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_holy_year_on(
        fixed: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = holiday_lines::holy_year_line(fixed);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The rank of every *Common Worship* celebration kept on a fixed day,
    /// as NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// The lines are the WebAssembly module's, one per celebration: its
    /// title, which is its name in `hc_holidays_on`'s `common-worship`
    /// table, the rank's identifier and the rank's English name. A day that
    /// keeps none writes an empty string, and a day with no Gregorian year
    /// is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
    /// the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_common_worship_on(
        fixed: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = holiday_lines::common_worship_lines(fixed);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
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
/// with the WebAssembly module.
#[cfg(feature = "seasons")]
mod seasons {
    use core::ffi::c_char;

    use hc::season_lines;

    use super::{HC_ERROR_NULL_POINTER, HC_OK, HcStatus, name, status, text, write_answer};

    /// The solar term in effect on a fixed day at a meridian, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// Tab-separated: the term's index from 春分 at 0 through 驚蟄 at 23
    /// (the longitude divided by 15°), its name in traditional Chinese, its
    /// name in Japanese, the fixed day the term began at that meridian, the
    /// last fixed day before the next term begins, the authority for the
    /// Chinese names and the authority for the Japanese names. `meridian`
    /// is a NUL-terminated name — `universal`, `japan`, `china`, `korea`,
    /// `india` or `china-before-1929`, in any case — or a longitude in
    /// decimal degrees east of Greenwich, read as local mean solar time;
    /// null or empty is `universal`, anything else `HC_ERROR_UNKNOWN`, and
    /// text that is not UTF-8 `HC_ERROR_NOT_UTF8`. A day outside the years
    /// −1000 to 3000, the era `hc_sky_at` answers for, is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `meridian` must be null or point to a NUL-terminated string; `buffer`
    /// must be writable for `capacity` bytes and `written` must be null or
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_term_in_effect(
        fixed: i64,
        meridian: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above; null is
        // `universal`, as the empty string is.
        let name = match unsafe { text(meridian) } {
            Ok(name) => name.unwrap_or(""),
            Err(status) => return status,
        };
        let answer = season_lines::term_line(fixed, name);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The pentad (候) in effect on a fixed day at a meridian, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// Tab-separated: the pentad's index from the first pentad of 春分 at 0
    /// through 71 (the longitude divided by 5°), its name in the Chinese
    /// tradition, its name in the Japanese tradition, the fixed day the
    /// pentad began at that meridian, the last fixed day before the next
    /// pentad begins, the text the Chinese names come from and the text the
    /// Japanese names come from. `meridian` and the day are as for
    /// `hc_term_in_effect`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// As `hc_term_in_effect`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_pentad_in_effect(
        fixed: i64,
        meridian: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above; null is
        // `universal`, as the empty string is.
        let name = match unsafe { text(meridian) } {
            Ok(name) => name.unwrap_or(""),
            Err(status) => return status,
        };
        let answer = season_lines::pentad_line(fixed, name);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The fixed day of 寒食, the Cold Food Day, of a Gregorian year under
    /// a named reckoning.
    ///
    /// `convention` is a NUL-terminated name: `hanshi-solstice-105`, 105
    /// days after the winter solstice at the Chinese meridian, the
    /// reckoning before 1645; `hanshi-eve-of-qingming`, the day before 清明
    /// at the Chinese meridian, as kept after the 時憲曆 of 1645; or
    /// `hansik`, Korea's 한식, 105 days after 동지 at the Korean meridian;
    /// in any case. Any other name is `HC_ERROR_UNKNOWN`, null
    /// `HC_ERROR_NULL_POINTER`, and text that is not UTF-8
    /// `HC_ERROR_NOT_UTF8`. The solstice reckonings count from the solstice
    /// of the year before, so a year outside −999 to 3000 is
    /// `HC_ERROR_OUT_OF_RANGE` under every reckoning.
    ///
    /// # Safety
    ///
    /// `convention` must be null or point to a NUL-terminated string, and
    /// `out_fixed` must be writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_cold_food_day(
        convention: *const c_char,
        year: i64,
        out_fixed: *mut i64,
    ) -> HcStatus {
        if out_fixed.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let id = match unsafe { name(convention) } {
            Ok(id) => id,
            Err(status) => return status,
        };
        match season_lines::cold_food_day(id, year) {
            Ok(day) => {
                // SAFETY: checked non-null above.
                unsafe { *out_fixed = day };
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }
}

#[cfg(feature = "seasons")]
pub use seasons::{hc_cold_food_day, hc_pentad_in_effect, hc_term_in_effect};

/// Deep time, behind the `deep-time` feature: the cosmic, geologic and
/// archaeological chronologies of `hc-deep-time`, and a moment placed in
/// all of them at once.
#[cfg(feature = "deep-time")]
mod deep_time {
    use core::ffi::c_char;

    use hc::deep_time_lines;

    use super::{HC_ERROR_OUT_OF_RANGE, HC_ERROR_UNKNOWN, HcStatus, text, write_text};

    /// A moment some years before the present, placed in every chronology
    /// at once, as NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// One line per entry, tab-separated, every deep-time entry point alike: the
    /// kind (`moment`, `cosmic-epoch`, `cosmic-event`, `future-era`, a
    /// geologic rank `eon`, `era`, `period`, `epoch` or `age`, or
    /// `archaeological`), the name, the scope (the interval one rank up for
    /// a geologic interval, the region for an archaeological period), the
    /// older bound's value, standard uncertainty, significant figures and
    /// `1` where the chart marks it approximate, the same four for the
    /// younger bound, the unit the values are in, the description, the
    /// source, and the name in the locale. A point in time has the same
    /// start and end. The units are what each table counts in:
    /// `seconds-since-big-bang` for the cosmic rows,
    /// `megayears-before-present` for the geologic chart's, the
    /// `years-before-1950` of the BP convention for the archaeological
    /// rows, `log10-years-from-now` for a future era. The lines are the
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
    /// alone. `locale` is a NUL-terminated BCP 47 tag, or null for none; the last column is the geologic
    /// chart's own name for an interval in that language, from the ICS's
    /// translations, and empty for every other row and for a language the
    /// chart has no names in. A value the crate refuses — not finite,
    /// beyond its range — is `HC_ERROR_OUT_OF_RANGE`; a `locale` that is not
    /// UTF-8 is `HC_ERROR_NOT_UTF8`. Writes the required length, including
    /// the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `locale` must be null or point to a NUL-terminated string; `buffer`
    /// must be writable for `capacity` bytes and `written` must be null or
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_place_years_ago(
        years_ago: f64,
        std_dev_years: f64,
        locale: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        let Ok(text) = deep_time_lines::placement(years_ago, std_dev_years, tag) else {
            return HC_ERROR_OUT_OF_RANGE;
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }

    /// Every cosmic epoch and every dated cosmic event, as NUL-terminated
    /// UTF-8 lines in a caller-owned buffer.
    ///
    /// The epochs first, Big Bang to the present, then the events, oldest
    /// first, each a line of the columns `hc_place_years_ago` writes, in
    /// `seconds-since-big-bang`. `locale` is as for `hc_place_years_ago`;
    /// no cosmic name has a translation this module carries, so the last
    /// column is empty. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `locale` must be null or point to a NUL-terminated string; `buffer`
    /// must be writable for `capacity` bytes and `written` must be null or
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_cosmic_events(
        locale: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        let text = deep_time_lines::cosmic(tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }

    /// Every interval of one rank of the geologic time scale, as
    /// NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// `rank` is 0 for the eons, 1 for the eras, 2 for the periods, 3 for
    /// the epochs and 4 for the ages; anything else is `HC_ERROR_UNKNOWN`.
    /// The intervals come youngest first, each a line of the columns
    /// `hc_place_years_ago` writes, in `megayears-before-present` with the
    /// chart's own figures and uncertainties, the chart as the source and
    /// the chart's name in `locale` last. Writes the required length,
    /// including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `locale` must be null or point to a NUL-terminated string; `buffer`
    /// must be writable for `capacity` bytes and `written` must be null or
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_geologic_intervals(
        rank: u32,
        locale: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let Some(rank) = deep_time_lines::rank(rank) else {
            return HC_ERROR_UNKNOWN;
        };
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        let text = deep_time_lines::intervals(rank, tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }
}

#[cfg(feature = "deep-time")]
pub use deep_time::{hc_cosmic_events, hc_geologic_intervals, hc_place_years_ago};

/// Time zones, behind the `tz` feature: the day an instant falls on, and
/// the instant a day begins, by the wall clock of an IANA zone.
#[cfg(feature = "tz")]
mod tz {
    use core::ffi::c_char;
    use std::sync::{Mutex, PoisonError};

    use hc::hc_calendar::fixed::RD_OF_UNIX_EPOCH;
    use hc::hc_calendar::{CivilDateTime, Rd};
    use hc::hc_core::UnixTime;
    use hc::hc_tz::{LocalResolution, TimeZone, TzifTimeZone, builtin};

    use super::{
        HC_ERROR_MALFORMED, HC_ERROR_NULL_POINTER, HC_ERROR_OUT_OF_RANGE, HC_ERROR_UNKNOWN, HC_OK,
        HcStatus, text,
    };

    /// The zones a caller has handed the library as TZif bytes, by name.
    static LOADED: Mutex<Vec<(String, Vec<u8>)>> = Mutex::new(Vec::new());

    /// The zone a name selects — one loaded through [`hc_zone_load`] first,
    /// then the built-in table — handed to `answer`.
    ///
    /// # Errors
    ///
    /// [`HC_ERROR_UNKNOWN`] for a name neither knows.
    fn with_zone<R>(name: &str, answer: impl FnOnce(&dyn TimeZone) -> R) -> Result<R, HcStatus> {
        let loaded = LOADED.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some((known, bytes)) = loaded
            .iter()
            .find(|(known, _)| known.eq_ignore_ascii_case(name))
        {
            let zone = TzifTimeZone::parse(known, bytes).map_err(|_| HC_ERROR_UNKNOWN)?;
            return Ok(answer(&zone));
        }
        drop(loaded);
        let zone = builtin::zone(name).map_err(|_| HC_ERROR_UNKNOWN)?;
        Ok(answer(&zone))
    }

    /// The fixed day an instant falls on by a zone's wall clock.
    pub(super) fn day_in_zone(unix: i64, name: &str) -> Result<i64, HcStatus> {
        with_zone(name, |zone| {
            zone.local_at(UnixTime::from_seconds(unix))
                .map(|local| local.day.0)
                .map_err(|_| HC_ERROR_OUT_OF_RANGE)
        })?
    }

    /// The instant a day begins by a zone's wall clock: its midnight, or
    /// the first instant after a gap that swallows it, or the earlier of
    /// two midnights when the clocks fall back across it.
    ///
    /// # Errors
    ///
    /// [`HC_ERROR_UNKNOWN`] for a name neither table knows, and
    /// [`HC_ERROR_OUT_OF_RANGE`] for a day whose start would overflow an
    /// `i64`.
    pub(super) fn start_in_zone(fixed: i64, name: &str) -> Result<i64, HcStatus> {
        with_zone(name, |zone| {
            let instant = match zone.resolve_local(CivilDateTime::midnight(Rd(fixed))) {
                LocalResolution::Unambiguous(instant) => instant,
                LocalResolution::Ambiguous { earlier, .. } => earlier,
                LocalResolution::Nonexistent { after_gap, .. } => after_gap,
            };
            unsaturated(fixed, instant.seconds(), zone).ok_or(HC_ERROR_OUT_OF_RANGE)
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

    /// The zone name a NUL-terminated argument carries.
    ///
    /// # Safety
    ///
    /// `zone` must be null or point to a NUL-terminated string.
    unsafe fn zone_argument<'a>(zone: *const c_char) -> Result<&'a str, HcStatus> {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { text(zone) }?.ok_or(HC_ERROR_NULL_POINTER)
    }

    /// The fixed day a POSIX timestamp falls on by the wall clock of a zone.
    ///
    /// `zone` is a NUL-terminated IANA name, `Asia/Tokyo`, in any case: one
    /// a caller has loaded through `hc_zone_load`, or else one of the
    /// seventeen the library carries with their current rules. A null
    /// `zone` or `out_fixed` is `HC_ERROR_NULL_POINTER`, a name neither
    /// knows `HC_ERROR_UNKNOWN`, an instant whose local day leaves the range
    /// of a day number `HC_ERROR_OUT_OF_RANGE`, and a name that is not UTF-8
    /// `HC_ERROR_NOT_UTF8`.
    ///
    /// # Safety
    ///
    /// `zone` must be null or point to a NUL-terminated string, and
    /// `out_fixed` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_fixed_from_unix_in_zone(
        unix_seconds: i64,
        zone: *const c_char,
        out_fixed: *mut i64,
    ) -> HcStatus {
        if out_fixed.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { zone_argument(zone) } {
            Ok(name) => name,
            Err(status) => return status,
        };
        match day_in_zone(unix_seconds, name) {
            Ok(day) => {
                // SAFETY: checked non-null immediately above.
                unsafe { *out_fixed = day };
                HC_OK
            }
            Err(status) => status,
        }
    }

    /// The POSIX timestamp at which a fixed day begins by the wall clock of
    /// a zone.
    ///
    /// The day begins at its local midnight. When the clocks go forward
    /// across that midnight, so that it does not exist, the day begins at
    /// the first instant after the gap; when they go back across it, at the
    /// earlier of the two midnights. `zone` is as for
    /// `hc_fixed_from_unix_in_zone`, and fails the same way.
    ///
    /// A day whose start would overflow an `int64_t` is
    /// `HC_ERROR_OUT_OF_RANGE`, never a clamped or wrapped number: by UTC, a
    /// day before fixed day −106 751 990 448 137 or after
    /// 106 751 991 886 463, and a zone's offset moves each end by at most a
    /// day.
    ///
    /// # Safety
    ///
    /// `zone` must be null or point to a NUL-terminated string, and
    /// `out_unix_seconds` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_unix_from_fixed_in_zone(
        fixed: i64,
        zone: *const c_char,
        out_unix_seconds: *mut i64,
    ) -> HcStatus {
        if out_unix_seconds.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { zone_argument(zone) } {
            Ok(name) => name,
            Err(status) => return status,
        };
        match start_in_zone(fixed, name) {
            Ok(instant) => {
                // SAFETY: checked non-null immediately above.
                unsafe { *out_unix_seconds = instant };
                HC_OK
            }
            Err(status) => status,
        }
    }

    /// Give the library a zone's TZif data under an IANA name.
    ///
    /// The built-in table carries seventeen zones and only their current
    /// rules; a caller that wants another zone, or a zone's history, reads
    /// the IANA file and hands its bytes here once, after which the two
    /// `_in_zone` entry points answer for that name — a loaded zone takes
    /// precedence over a built-in one of the same name. The bytes are
    /// copied. A null or empty `name` is `HC_ERROR_NULL_POINTER`, bytes
    /// that are not a TZif file `HC_ERROR_MALFORMED`, and nothing is kept
    /// on failure.
    ///
    /// # Safety
    ///
    /// `name` must be null or point to a NUL-terminated string, and `tzif`
    /// must be readable for `tzif_len` bytes unless `tzif_len` is zero.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_zone_load(
        name: *const c_char,
        tzif: *const u8,
        tzif_len: usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let name = match unsafe { zone_argument(name) } {
            Ok(name) if !name.is_empty() => name,
            Ok(_) => return HC_ERROR_NULL_POINTER,
            Err(status) => return status,
        };
        let bytes: &[u8] = if tzif.is_null() || tzif_len == 0 {
            &[]
        } else {
            // SAFETY: the caller guarantees `tzif` is readable for `tzif_len`.
            unsafe { core::slice::from_raw_parts(tzif, tzif_len) }
        };
        if TzifTimeZone::parse(name, bytes).is_err() {
            return HC_ERROR_MALFORMED;
        }
        let mut loaded = LOADED.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(slot) = loaded
            .iter_mut()
            .find(|(known, _)| known.eq_ignore_ascii_case(name))
        {
            slot.1 = bytes.to_vec();
        } else {
            loaded.push((name.to_owned(), bytes.to_vec()));
        }
        HC_OK
    }
}

#[cfg(feature = "tz")]
pub use tz::{hc_fixed_from_unix_in_zone, hc_unix_from_fixed_in_zone, hc_zone_load};

/// Where each zone is, behind the `tz` feature: the principal location
/// the IANA database gives a zone, its countries and its CLDR exemplar
/// city. The lines are `hyper_calendar::zone_lines`', shared with the
/// WebAssembly module.
#[cfg(feature = "tz")]
mod zones {
    use core::ffi::c_char;

    use hc::zone_lines;

    use super::{HcStatus, name, text, write_answer, write_text};

    /// Every zone of the IANA database's `zone1970.tab` with its principal
    /// location, as NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// The lines are the WebAssembly module's: one per zone, in the table's
    /// order, with the zone, the latitude in decimal degrees north, the
    /// longitude in decimal degrees east, each the table's whole arcseconds
    /// to six places, the ISO 3166-1 codes of the
    /// countries it overlaps `;`-separated, the table's comment, the zone's
    /// CLDR 48 exemplar city in the `locale`, and the tag of the data that
    /// named the city. A build without the `calendars` feature, a locale
    /// with no city for the zone, and a null `locale` name the city in
    /// English, with `en` in column 7. Writes the required length,
    /// including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `locale` must be null or NUL-terminated; `buffer` must be writable
    /// for `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_zones(
        locale: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&zone_lines::zones(tag), buffer, capacity, written) }
    }

    /// Where one zone is, as the NUL-terminated UTF-8 line `hc_zones`
    /// writes for it, in a caller-owned buffer.
    ///
    /// `zone` is an IANA name in any case: a zone of `zone1970.tab`; a link
    /// `zone.tab` gives a place of its own, such as `Europe/Oslo`; or
    /// another link of the database's `backward` file, such as
    /// `Asia/Calcutta`, which answers with the line of the name it leads
    /// to, so that column 1 is then `Asia/Kolkata`. A name that places
    /// nothing, such as `UTC`, is `HC_ERROR_UNKNOWN`, and a null `zone`
    /// `HC_ERROR_NULL_POINTER`. `locale` is as for `hc_zones`. Writes the
    /// required length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `zone` must be null or NUL-terminated, and so must `locale`;
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_zone_location(
        zone: *const c_char,
        locale: *const c_char,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let zone = match unsafe { name(zone) } {
            Ok(zone) => zone,
            Err(status) => return status,
        };
        // SAFETY: forwarded to the caller's contract above.
        let tag = match unsafe { text(locale) } {
            Ok(tag) => tag.unwrap_or(""),
            Err(status) => return status,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe {
            write_answer(
                zone_lines::zone_location(zone, tag),
                buffer,
                capacity,
                written,
            )
        }
    }
}

#[cfg(feature = "tz")]
pub use zones::{hc_zone_location, hc_zones};

/// The sky, behind the `sky` feature: where the Sun and the Moon are at an
/// instant, the solar terms and the moon phases within a span, and the
/// Sun's decan, from `hc-astro`'s series, in Universal Time. The lines are
/// `hyper_calendar::sky_lines`', shared with the WebAssembly module.
#[cfg(feature = "sky")]
mod sky {
    use core::ffi::c_char;

    use super::{HcStatus, write_answer};
    use hc::sky_lines;

    /// The Sun and the Moon at a POSIX timestamp, as one NUL-terminated
    /// UTF-8 line in a caller-owned buffer.
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
    /// which `hc-astro` states its series valid, is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_sky_at(
        unix_seconds: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(sky_lines::sky_line(unix_seconds), buffer, capacity, written) }
    }

    /// Every solar term whose instant falls in `[from_unix, to_unix)`, as
    /// NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// One line per term, in time order, tab-separated: the Sun's
    /// apparent longitude that defines the term in degrees (0 for 春分
    /// through 345), the instant as whole POSIX seconds, rounded down, in
    /// Universal Time, the term's name in traditional Chinese and its name
    /// in Japanese. The span is half-open and is refused with
    /// `HC_ERROR_OUT_OF_RANGE` when either end lies outside the years
    /// −1000 to 3000 or when it is longer than 400 years; `to_unix` at or
    /// before `from_unix` is an empty answer. Writes the required length,
    /// including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// As `hc_sky_at`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_solar_terms_between(
        from_unix: i64,
        to_unix: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        unsafe {
            write_answer(
                sky_lines::term_lines(from_unix, to_unix),
                buffer,
                capacity,
                written,
            )
        }
    }

    /// Every new moon, first quarter, full moon and last quarter whose
    /// instant falls in `[from_unix, to_unix)`, as NUL-terminated UTF-8
    /// lines in a caller-owned buffer.
    ///
    /// One line per phase, in time order, tab-separated: the Moon's
    /// elongation from the Sun that defines the phase in degrees (0, 90,
    /// 180 or 270), the instant as whole POSIX seconds, rounded down, in
    /// Universal Time, the phase's name (`new`, `first-quarter`, `full` or
    /// `last-quarter`) and an empty fourth column, so the lines have the
    /// shape of `hc_solar_terms_between`'s. The instants are the phase
    /// series of Meeus chapter 49, the same series that dates the new
    /// moons of `hc_sky_at`. The span fails as for
    /// `hc_solar_terms_between`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// As `hc_sky_at`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_moon_phases_between(
        from_unix: i64,
        to_unix: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        unsafe {
            write_answer(
                sky_lines::phase_lines(from_unix, to_unix),
                buffer,
                capacity,
                written,
            )
        }
    }

    /// The decan the Sun is in at a POSIX timestamp, as one NUL-terminated
    /// UTF-8 line in a caller-owned buffer.
    ///
    /// The line is the WebAssembly module's: the tropical sign's number and
    /// English name, the decan within it, 1 to 3, its ruler's identifier and
    /// English name, and the degrees into the decan. An instant outside the
    /// years −1000 to 3000 is `HC_ERROR_OUT_OF_RANGE`. Writes the required
    /// length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_decan_at(
        unix_seconds: i64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = sky_lines::decan_line(unix_seconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }
}

#[cfg(feature = "sky")]
pub use sky::{hc_decan_at, hc_moon_phases_between, hc_sky_at, hc_solar_terms_between};

/// The Earth's rotation and the Sun's hours, behind the `sky` feature: the
/// Earth Rotation Angle, the Greenwich mean sidereal time by two
/// conventions, UT2 − UT1, and the clocks and named times of day of
/// `hc_astro::solar_time`, the named horizons with sunrise and sunset
/// against each, and the Heliocentric Julian Date in TT and in UTC, from
/// `hyper_calendar::astro_lines`, shared with the WebAssembly module, for
/// the years −1000 to 3000.
#[cfg(feature = "sky")]
mod earth_and_sun {
    use core::ffi::{c_char, c_int};

    use hc::astro_lines;

    use super::{HC_ERROR_NULL_POINTER, HC_OK, HcStatus, name, status, write_answer};

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

    /// Every named horizon a rising or a setting can be measured against,
    /// as NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// The lines are the WebAssembly module's, one per horizon: the
    /// identifier, the English name, what it takes the visible horizon to
    /// be, and the source. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_horizons(
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = Ok(astro_lines::horizons_lines());
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// Sunrise on a fixed day at a place against a named horizon, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// `horizon` is an identifier `hc_horizons` lists, in any case; anything
    /// else is `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`. The line
    /// is the WebAssembly module's: the instant as POSIX seconds, the three
    /// cells of `hc_solar_event` naming a missing `sunrise`, and the
    /// altitude of the Sun's centre at the crossing. A place off the globe,
    /// or a day outside the years −1000 to 3000, is `HC_ERROR_OUT_OF_RANGE`.
    /// Writes the required length, including the terminator, into
    /// `written`.
    ///
    /// # Safety
    ///
    /// `horizon` must be null or NUL-terminated; `buffer` must be writable
    /// for `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_sunrise(
        horizon: *const c_char,
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let horizon = match unsafe { name(horizon) } {
            Ok(horizon) => horizon,
            Err(status) => return status,
        };
        let answer = astro_lines::location(latitude, longitude, elevation)
            .and_then(|place| astro_lines::sunrise_line(horizon, fixed, place));
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// Sunset on a fixed day at a place against a named horizon, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// As `hc_sunrise`, for the upper limb's setting, with `sunset` as the
    /// missing event.
    ///
    /// # Safety
    ///
    /// As `hc_sunrise`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_sunset(
        horizon: *const c_char,
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let horizon = match unsafe { name(horizon) } {
            Ok(horizon) => horizon,
            Err(status) => return status,
        };
        let answer = astro_lines::location(latitude, longitude, elevation)
            .and_then(|place| astro_lines::sunset_line(horizon, fixed, place));
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
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
    pub unsafe extern "C" fn hc_gmst_iau2006(
        ut1_unix_seconds: f64,
        out_degrees: *mut f64,
    ) -> HcStatus {
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
    pub unsafe extern "C" fn hc_gmst_iau1982(
        ut1_unix_seconds: f64,
        out_degrees: *mut f64,
    ) -> HcStatus {
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

    /// A local clock's reading at a POSIX timestamp and a place, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// `clock` is `local-mean`, `local-apparent`, `temporal` or `italian`,
    /// in any case; anything else is `HC_ERROR_UNKNOWN`, and null
    /// `HC_ERROR_NULL_POINTER`. The line is the WebAssembly module's: the
    /// local date's fixed day, the hours into it, and three cells naming a
    /// solar event the reading needs that does not happen, empty when it
    /// exists. A place off the globe, or an instant outside the years −1000
    /// to 3000, is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
    /// including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `clock` must be null or NUL-terminated; `buffer` must be writable for
    /// `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_solar_time(
        clock: *const c_char,
        unix_seconds: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let clock = match unsafe { name(clock) } {
            Ok(clock) => clock,
            Err(status) => return status,
        };
        let answer = astro_lines::location(latitude, longitude, elevation)
            .and_then(|place| astro_lines::solar_time_line(clock, unix_seconds, place));
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// A named time of day on a fixed day at a place, as one NUL-terminated
    /// UTF-8 line in a caller-owned buffer.
    ///
    /// `event` is `asr-shafii`, `asr-hanafi`, `jewish-dusk-vilna-gaon`,
    /// `jewish-sabbath-ends-cohn` or `italian-zero-hour`, in any case;
    /// anything else is `HC_ERROR_UNKNOWN`, and null
    /// `HC_ERROR_NULL_POINTER`. The line is the WebAssembly module's: the
    /// instant as whole POSIX seconds of Universal Time, and the three cells
    /// of `hc_solar_time` naming a missing solar event. A place off the
    /// globe, or a day outside the years −1000 to 3000, is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// As `hc_solar_time`, for `event`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_solar_event(
        event: *const c_char,
        fixed: i64,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let event = match unsafe { name(event) } {
            Ok(event) => event,
            Err(status) => return status,
        };
        let answer = astro_lines::location(latitude, longitude, elevation)
            .and_then(|place| astro_lines::solar_event_line(event, fixed, place));
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The Heliocentric Julian Date in TT, HJD_TT, of a Julian Date of TT
    /// for a target, as one NUL-terminated UTF-8 line in a caller-owned
    /// buffer.
    ///
    /// The target is its right ascension, 0 to 360 degrees, and its
    /// declination, −90 to 90 degrees, on the mean equator and equinox of
    /// J2000. The line is the WebAssembly module's: the HJD_TT and the
    /// light-time correction in seconds. A date outside the years −1000 to
    /// 3000 or not finite, or a direction outside those ranges, is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_hjd_tt(
        tt_julian_date: f64,
        right_ascension: f64,
        declination: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = astro_lines::hjd_tt_line(tt_julian_date, right_ascension, declination);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The Heliocentric Julian Date in UTC, HJD_UTC, of a Julian Date of
    /// UTC for a target, as one NUL-terminated UTF-8 line in a caller-owned
    /// buffer.
    ///
    /// As `hc_hjd_tt`, with the Earth taken at the TT instant of the UTC
    /// date, TT − UTC being 32.184 s plus TAI − UTC from the leap-second
    /// table. The line is the WebAssembly module's: the HJD_UTC, the
    /// correction in seconds and the TT − UTC used. `strict` non-zero
    /// refuses a date outside the leap-second table with
    /// `HC_ERROR_NO_DATA`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_hjd_utc(
        utc_julian_date: f64,
        right_ascension: f64,
        declination: f64,
        strict: c_int,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer =
            astro_lines::hjd_utc_line(utc_julian_date, right_ascension, declination, strict != 0);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
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
    use core::ffi::c_char;

    use hc::hc_core::math::floor;
    use hc::hc_orbital::{
        LATITUDE_65N, MID_JUNE_SOLAR_LONGITUDE, SOLAR_CONSTANT_BERGER_LOUTRE_1991, SOURCE,
        VALID_SPAN, daily_insolation, elements_at,
    };

    use super::{HC_ERROR_OUT_OF_RANGE, HcStatus, push_cell, write_text};

    /// The solar constant every insolation cell is computed with: the
    /// 1360 W m⁻² of Berger & Loutre's 1991 tables, `hc-orbital`'s
    /// `SOLAR_CONSTANT_BERGER_LOUTRE_1991`, so a figure here compares with
    /// the NOAA `orbit91` table exactly. The value is written in its own
    /// column and the constant's name in the source; a caller who wants
    /// another value scales the cell, since the insolation is proportional
    /// to the constant.
    pub(super) const SOLAR_CONSTANT: f64 = SOLAR_CONSTANT_BERGER_LOUTRE_1991;

    /// The name the source cell gives the constant.
    const SOLAR_CONSTANT_NAME: &str = "SOLAR_CONSTANT_BERGER_LOUTRE_1991";

    /// The most samples one call of [`hc_orbit_series`] writes.
    pub(super) const MAX_SERIES_SAMPLES: usize = 10_000;

    /// Append the eleven cells of one epoch and the line break; see
    /// [`hc_orbit_at`] for the columns.
    ///
    /// # Errors
    ///
    /// [`HC_ERROR_OUT_OF_RANGE`] for an epoch the crate refuses: not
    /// finite, or beyond a million years either side of 1950.
    fn push_epoch(out: &mut String, years_before_present: f64) -> Result<(), HcStatus> {
        use core::fmt::Write;
        let elements = elements_at(years_before_present).map_err(|_| HC_ERROR_OUT_OF_RANGE)?;
        let insolation = daily_insolation(
            &elements,
            LATITUDE_65N,
            MID_JUNE_SOLAR_LONGITUDE,
            SOLAR_CONSTANT,
        )
        .map_err(|_| HC_ERROR_OUT_OF_RANGE)?;
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
    pub(super) fn orbit_line(years_before_present: f64) -> Result<String, HcStatus> {
        let mut out = String::new();
        push_epoch(&mut out, years_before_present)?;
        Ok(out)
    }

    /// One line per sample from `from` to `to` in steps of `step`, each
    /// with the epoch first; see [`hc_orbit_series`].
    ///
    /// # Errors
    ///
    /// [`HC_ERROR_OUT_OF_RANGE`] for a step that is not finite and
    /// positive, an end outside the span, or more than
    /// [`MAX_SERIES_SAMPLES`] samples.
    pub(super) fn series_lines(from: f64, to: f64, step: f64) -> Result<String, HcStatus> {
        use core::fmt::Write;
        if !(from.is_finite() && to.is_finite() && step.is_finite()) || step <= 0.0 {
            return Err(HC_ERROR_OUT_OF_RANGE);
        }
        if !(VALID_SPAN.contains(&from) && VALID_SPAN.contains(&to)) {
            return Err(HC_ERROR_OUT_OF_RANGE);
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
            return Err(HC_ERROR_OUT_OF_RANGE);
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
    /// epoch, as one NUL-terminated UTF-8 line in a caller-owned buffer.
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
    /// `HC_ERROR_OUT_OF_RANGE`: the series would return numbers there, and
    /// they would be fiction. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_orbit_at(
        years_before_1950: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let text = match orbit_line(years_before_1950) {
            Ok(text) => text,
            Err(status) => return status,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }

    /// The line of `hc_orbit_at` at every epoch from `from_years_before_1950`
    /// to `to_years_before_1950` in steps of `step_years`, each with the
    /// epoch as a first column, as NUL-terminated UTF-8 lines in a
    /// caller-owned buffer.
    ///
    /// One line per sample, tab-separated: the epoch in years before 1950,
    /// then the eleven columns of `hc_orbit_at`. The samples are `from`,
    /// `from + step`, `from + 2 step` and so on, every one at or before
    /// `to`. Both ends have to lie within a million years either side of
    /// 1950 and `step` has to be finite and positive, else
    /// `HC_ERROR_OUT_OF_RANGE`; more than 10 000 samples is
    /// `HC_ERROR_OUT_OF_RANGE` too, and a caller who wants more asks in
    /// pieces. A `to` before `from` is an empty answer, not an error. Writes
    /// the required length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// As `hc_orbit_at`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_orbit_series(
        from_years_before_1950: f64,
        to_years_before_1950: f64,
        step_years: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let text = match series_lines(from_years_before_1950, to_years_before_1950, step_years) {
            Ok(text) => text,
            Err(status) => return status,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }
}

#[cfg(feature = "orbital")]
pub use orbital::{hc_orbit_at, hc_orbit_series};

/// Time on other bodies, behind the `planetary` feature: Mars time, the
/// surface missions' sol counts, and the solar day and local mean solar
/// time of every body in `hc-planetary`'s table.
///
/// The calendars of other bodies' days are [`hc_circad_date`], one entry
/// point beside [`hc_mars_time`] with its own line, not new columns of an
/// existing one.
#[cfg(feature = "planetary")]
mod planetary {
    use core::ffi::c_char;

    use hc::planetary_lines;

    use super::{HC_ERROR_NULL_POINTER, HC_OK, HcStatus, name, status, write_answer};

    /// Mars at a POSIX instant and an east longitude, as one NUL-terminated
    /// UTF-8 line in a caller-owned buffer.
    ///
    /// The line is the WebAssembly module's: the Mars Sol Date;
    /// Coordinated Mars Time, local mean solar time and local true solar
    /// time at the longitude, each as `HH:MM:SS` on the 24-hour Martian
    /// clock (truncated) and in decimal Martian hours; the equation of time
    /// in Martian minutes; the areocentric solar longitude `Ls` in degrees;
    /// the Mars year under the Clancy convention; the Darian year, month,
    /// sol of the month, month name and sol-of-week name at Airy-0; and the
    /// source. `unix_seconds` is POSIX time with a fraction, read through
    /// the leap-second table with the last offset held;
    /// `east_longitude_degrees` is planetocentric, east-positive, and wraps. An
    /// instant more than 100 Julian years from J2000.0, where Allison and
    /// McEwen's series is an extrapolation, or a value that is not finite,
    /// is `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including
    /// the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_mars_time(
        unix_seconds: f64,
        east_longitude_degrees: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = planetary_lines::mars_time_line(unix_seconds, east_longitude_degrees);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// Every surface mission on Mars and the rules of its sol count, as
    /// NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// The lines are the WebAssembly module's, one per mission in landing
    /// order: the identifier, the name, the landing instant as UTC text and
    /// as a POSIX timestamp, the number of the landing sol (0 or 1), the
    /// clock's midnight (`local-mean-solar-time`, or
    /// `local-true-solar-time-at-landing`), the clock meridian's east
    /// longitude, the achieved site's east longitude, `1` where the
    /// operators published the convention and `0` otherwise, the note and
    /// the source. Where no convention was published the landing sol, the
    /// clock and its meridian are empty. Writes the required length,
    /// including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_missions(
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = Ok(planetary_lines::missions_lines());
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The sol number of a Mars surface mission at a POSIX instant, by the
    /// mission's own clock.
    ///
    /// `mission` is a NUL-terminated identifier or name `hc_missions`
    /// lists, in any ASCII case. The sol is counted as the mission counted
    /// it: from the midnight, mean or true, on the mission's clock meridian
    /// that began the landing sol, which is sol 0 or sol 1 as the operators
    /// numbered it. A mission the table does not carry is
    /// `HC_ERROR_UNKNOWN`; one whose operators published no sol numbering
    /// is `HC_ERROR_NO_DATA`; an instant before the landing sol began, or
    /// outside `hc_mars_time`'s span, is `HC_ERROR_OUT_OF_RANGE`.
    ///
    /// # Safety
    ///
    /// `mission` must be null or NUL-terminated, and `out_sol` must be
    /// writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_mission_sol(
        mission: *const c_char,
        unix_seconds: f64,
        out_sol: *mut i64,
    ) -> HcStatus {
        if out_sol.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        // SAFETY: forwarded to the caller's contract above.
        let mission = match unsafe { name(mission) } {
            Ok(mission) => mission,
            Err(status) => return status,
        };
        match planetary_lines::mission_sol(mission, unix_seconds) {
            Ok(sol) => {
                // SAFETY: checked non-null above.
                unsafe { *out_sol = sol };
                HC_OK
            }
            Err(refusal) => status(refusal),
        }
    }

    /// Every body `hc-planetary` carries, with its solar day, as
    /// NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// The lines are the WebAssembly module's, one per body outward from
    /// the Sun with each planet's moons after it: the identifier, the name,
    /// the kind (`star`, `planet`, `dwarf-planet` or `moon`), the
    /// identifier of the body it orbits, the sidereal rotation period in
    /// hours (negative for a retrograde rotator), the solar day in SI
    /// seconds, `measured` or `derived`, the year in local solar days,
    /// whether the clock's zero point is a `standard` or a `convention`
    /// this library declares, what the zero point is, the source, and the
    /// status of a standard still being drawn up (the Moon's Coordinated
    /// Lunar Time). The Sun's three day cells are empty. Writes the
    /// required length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_bodies(
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = Ok(planetary_lines::bodies_lines());
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// Local mean solar time on a body at a POSIX instant and an east
    /// longitude, as one NUL-terminated UTF-8 line in a caller-owned
    /// buffer.
    ///
    /// The line is the WebAssembly module's: the local day number, the
    /// fraction of it elapsed, the reading as `HH:MM:SS` (truncated) and in
    /// decimal local hours on a 24-hour face, the solar day and the local
    /// hour in SI seconds, whether the zero point is a `standard` or a
    /// `convention`, and what it is. `body` is a NUL-terminated identifier
    /// or name `hc_bodies` lists, in any ASCII case; a body it does not
    /// list is `HC_ERROR_UNKNOWN`, and the Sun, which has no solar day,
    /// `HC_ERROR_NO_DATA`. The instant and the longitude fail as for
    /// `hc_mars_time`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `body` must be null or NUL-terminated; `buffer` must be writable for
    /// `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_body_time(
        body: *const c_char,
        unix_seconds: f64,
        east_longitude_degrees: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let body = match unsafe { name(body) } {
            Ok(body) => body,
            Err(status) => return status,
        };
        let answer = planetary_lines::body_time_line(body, unix_seconds, east_longitude_degrees);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// The date at a POSIX instant in a calendar of another body's days, as
    /// one NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// The line is the WebAssembly module's: the calendar, the year, the
    /// month, the day of the month, the month's name, the name of the day's
    /// place in the week, the day count the date is numbered by (a circad
    /// number, or Martiana's Darian sol number), the fraction of that day
    /// elapsed, `1` for a leap year, and the source. `calendar` is
    /// `darian-titan`, `gregorian-io`, `gregorian-europa`,
    /// `gregorian-ganymede`, `gregorian-callisto` or `martiana`, in any
    /// ASCII case; another is `HC_ERROR_UNKNOWN`, and null
    /// `HC_ERROR_NULL_POINTER`. The instant fails as for `hc_mars_time`.
    /// Writes the required length, including the terminator, into
    /// `written`.
    ///
    /// # Safety
    ///
    /// `calendar` must be null or NUL-terminated; `buffer` must be writable
    /// for `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_circad_date(
        calendar: *const c_char,
        unix_seconds: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let calendar = match unsafe { name(calendar) } {
            Ok(calendar) => calendar,
            Err(status) => return status,
        };
        let answer = planetary_lines::circad_date_line(calendar, unix_seconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
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
    use core::ffi::c_char;

    use hc::relativity_lines;

    use super::{HcStatus, name, write_answer};

    /// A clock moving at a constant speed while some coordinate time
    /// passes, as one NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// The line is the WebAssembly module's: β, the Lorentz factor γ, the
    /// proper time the moving clock records in seconds, its rate
    /// dτ/dt = 1/γ, that rate's offset from 1 in microseconds per
    /// 86 400-second day (negative, computed without cancellation), the
    /// `hc-relativity` constant used (`SPEED_OF_LIGHT`), and the source. A
    /// speed at or beyond the speed of light either way, or a value that is
    /// not finite, is `HC_ERROR_OUT_OF_RANGE`. Writes the required length,
    /// including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_proper_time(
        speed_metres_per_second: f64,
        coordinate_seconds: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer =
            relativity_lines::proper_time_line(speed_metres_per_second, coordinate_seconds);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// A clock held still at a radius from a body's centre, against one far
    /// from every mass, as one NUL-terminated UTF-8 line in a caller-owned
    /// buffer.
    ///
    /// The line is the WebAssembly module's: the body's identifier, its GM
    /// in m³ s⁻², the name of the `hc-relativity` constant that holds it,
    /// the Schwarzschild radius in metres, the static dilation factor
    /// dτ/dt = √(1 − r_s/r), that factor's offset from 1 in microseconds
    /// per 86 400-second day (negative, computed without cancellation), the
    /// constants used, separated by `;`, and the body's source. `body` is a NUL-terminated
    /// identifier or name `hc_gravitating_bodies` lists, in any ASCII case;
    /// another is `HC_ERROR_UNKNOWN`. A radius that is not finite, not
    /// positive, or at or inside the Schwarzschild radius is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
    /// terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `body` must be null or NUL-terminated; `buffer` must be writable for
    /// `capacity` bytes and `written` must be null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gravitational_dilation(
        body: *const c_char,
        radius_metres: f64,
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        // SAFETY: forwarded to the caller's contract above.
        let body = match unsafe { name(body) } {
            Ok(body) => body,
            Err(status) => return status,
        };
        let answer = relativity_lines::gravitational_dilation_line(body, radius_metres);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }

    /// Every body `hc-relativity` carries a gravitational parameter for, as
    /// NUL-terminated UTF-8 lines in a caller-owned buffer.
    ///
    /// The lines are the WebAssembly module's, one per body: the
    /// identifier, the English name, GM in m³ s⁻², the name of the
    /// `hc-relativity` constant that holds it, and the source. Writes the
    /// required length, including the terminator, into `written`.
    ///
    /// # Safety
    ///
    /// `buffer` must be writable for `capacity` bytes and `written` must be
    /// null or writable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn hc_gravitating_bodies(
        buffer: *mut c_char,
        capacity: usize,
        written: *mut usize,
    ) -> HcStatus {
        let answer = Ok(relativity_lines::gravitating_bodies_lines());
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_answer(answer, buffer, capacity, written) }
    }
}

#[cfg(feature = "relativity")]
pub use relativity::{hc_gravitating_bodies, hc_gravitational_dilation, hc_proper_time};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn querying_the_version_works_in_both_passes() {
        let mut written = 0usize;
        assert_eq!(
            unsafe { hc_version(core::ptr::null_mut(), 0, &mut written) },
            HC_ERROR_BUFFER_TOO_SMALL
        );
        assert_eq!(written, hc::VERSION.len() + 1);

        let mut buffer = vec![0 as c_char; written];
        assert_eq!(
            unsafe { hc_version(buffer.as_mut_ptr(), buffer.len(), core::ptr::null_mut()) },
            HC_OK
        );
    }

    /// Call a line-writing entry point the way a C caller does: measure,
    /// allocate, read, and check the terminator.
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
    fn read_lines(call: impl Fn(*mut c_char, usize, *mut usize) -> HcStatus) -> String {
        let mut written = 0usize;
        assert_eq!(
            call(core::ptr::null_mut(), 0, &mut written),
            HC_ERROR_BUFFER_TOO_SMALL
        );
        assert!(written > 1);
        let mut small = [7 as c_char; 1];
        assert_eq!(
            call(small.as_mut_ptr(), small.len(), core::ptr::null_mut()),
            HC_ERROR_BUFFER_TOO_SMALL
        );
        assert_eq!(small, [7 as c_char], "a refused buffer is left untouched");
        let mut buffer = vec![0 as c_char; written];
        let mut again = 0usize;
        assert_eq!(call(buffer.as_mut_ptr(), buffer.len(), &mut again), HC_OK);
        assert_eq!(again, written);
        let text = unsafe { core::ffi::CStr::from_ptr(buffer.as_ptr()) }
            .to_str()
            .expect("UTF-8")
            .to_owned();
        assert_eq!(text.len() + 1, written);
        assert!(text.ends_with('\n'), "{text:?}");
        text
    }

    /// The status of a line-writing entry point's measuring call: a null
    /// buffer, which is `HC_ERROR_BUFFER_TOO_SMALL` when the entry point
    /// answers, and the refusal when it does not.
    #[cfg(any(
        feature = "timestamps",
        feature = "calendars",
        feature = "sky",
        feature = "planetary",
        feature = "relativity"
    ))]
    fn measured(call: impl Fn(*mut c_char, usize, *mut usize) -> HcStatus) -> HcStatus {
        let mut written = 0usize;
        call(core::ptr::null_mut(), 0, &mut written)
    }

    #[cfg(feature = "civil")]
    mod civil {
        use super::super::*;

        /// The leap second at the end of 2016: TAI second 1 483 228 836
        /// from 1970 TAI is 23:59:60, named by POSIX 1 483 228 800, and
        /// 00:00:00 on 1 January 2017 is TAI + 37 s.
        #[test]
        fn the_tai_bridge_names_the_leap_second_of_2016() {
            let new_year = 1_483_228_800;
            let (mut seconds, mut attos) = (0i64, 7u64);
            assert_eq!(
                unsafe { hc_tai_from_unix(new_year, 1, &mut seconds, &mut attos) },
                HC_OK
            );
            assert_eq!((seconds, attos), (new_year + 37, 0));
            let (mut unix, mut leap) = (0i64, 0 as core::ffi::c_int);
            assert_eq!(
                unsafe { hc_utc_from_tai(new_year + 36, 1, &mut unix, &mut leap) },
                HC_OK
            );
            assert_eq!((unix, leap), (new_year, 1));
            assert_eq!(
                unsafe { hc_tai_from_unix(-400_000_000, 1, &mut seconds, &mut attos) },
                HC_ERROR_NO_DATA
            );
            assert_eq!(
                unsafe { hc_tai_from_unix(i64::MAX, 0, &mut seconds, &mut attos) },
                HC_ERROR_OVERFLOW
            );
        }

        #[test]
        fn gregorian_conversion_round_trips_through_the_boundary() {
            let mut fixed = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2026, 9, 21, &mut fixed) },
                HC_OK
            );
            assert_eq!(fixed, 739_880);

            let (mut year, mut month, mut day) = (0i64, 0u8, 0u8);
            assert_eq!(
                unsafe { hc_gregorian_from_fixed(fixed, &mut year, &mut month, &mut day) },
                HC_OK
            );
            assert_eq!((year, month, day), (2026, 9, 21));
        }

        #[test]
        fn invalid_dates_return_a_status_rather_than_panicking() {
            let mut fixed = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2026, 2, 30, &mut fixed) },
                HC_ERROR_INVALID_DATE
            );
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2026, 13, 1, &mut fixed) },
                HC_ERROR_INVALID_DATE
            );
        }

        #[test]
        fn null_out_parameters_are_refused() {
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2026, 9, 21, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe { hc_weekday(0, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
        }

        #[test]
        fn weekdays_use_iso_numbering() {
            let mut weekday = 0u8;
            // 1970-01-01 was a Thursday.
            assert_eq!(unsafe { hc_weekday(719_163, &mut weekday) }, HC_OK);
            assert_eq!(weekday, 4);
            for (fixed, iso) in [(i64::MIN, 6), (i64::MAX, 7)] {
                assert_eq!(unsafe { hc_weekday(fixed, &mut weekday) }, HC_OK);
                assert_eq!(weekday, iso, "{fixed}");
            }
        }

        /// The twins of the WebAssembly module's `hc_day_of_year`,
        /// `hc_is_leap_year`, `hc_fixed_from_unix`, `hc_unix_from_fixed` and
        /// `hc_parse_iso_date` answer as they do, through out-parameters.
        #[test]
        fn the_civil_twins_of_the_webassembly_exports() {
            // 2024-12-31 is day 366 of a leap year, 739 251.
            let (mut day_of_year, mut leap) = (0u32, 0 as core::ffi::c_int);
            assert_eq!(unsafe { hc_day_of_year(739_251, &mut day_of_year) }, HC_OK);
            assert_eq!(day_of_year, 366);
            assert_eq!(unsafe { hc_is_leap_year(739_251, &mut leap) }, HC_OK);
            assert_eq!(leap, 1);
            assert_eq!(unsafe { hc_is_leap_year(739_252, &mut leap) }, HC_OK);
            assert_eq!(leap, 0);
            for outside in [-3_652_425_000, 3_652_424_635] {
                assert_eq!(
                    unsafe { hc_day_of_year(outside, &mut day_of_year) },
                    HC_ERROR_NO_DATA
                );
                assert_eq!(
                    unsafe { hc_is_leap_year(outside, &mut leap) },
                    HC_ERROR_NO_DATA
                );
            }

            let mut fixed = 0i64;
            assert_eq!(unsafe { hc_fixed_from_unix(0, &mut fixed) }, HC_OK);
            assert_eq!(fixed, 719_163);
            assert_eq!(unsafe { hc_fixed_from_unix(-1, &mut fixed) }, HC_OK);
            assert_eq!(fixed, 719_162);
            assert_eq!(unsafe { hc_fixed_from_unix(i64::MIN, &mut fixed) }, HC_OK);
            assert_eq!(fixed, -106_751_990_448_138);
            assert_eq!(unsafe { hc_fixed_from_unix(i64::MAX, &mut fixed) }, HC_OK);
            assert_eq!(fixed, 106_751_991_886_463);

            let mut seconds = 0i64;
            assert_eq!(unsafe { hc_unix_from_fixed(739_880, &mut seconds) }, HC_OK);
            assert_eq!(seconds, 1_789_948_800);
            // No floor: the first day whose midnight fits is answered, as
            // `hc_unix_from_fixed_in_zone` answers it by UTC.
            assert_eq!(
                unsafe { hc_unix_from_fixed(-106_751_990_448_137, &mut seconds) },
                HC_OK
            );
            for outside in [
                -106_751_990_448_138,
                106_751_991_886_464,
                i64::MIN,
                i64::MAX,
            ] {
                assert_eq!(
                    unsafe { hc_unix_from_fixed(outside, &mut seconds) },
                    HC_ERROR_OUT_OF_RANGE,
                    "{outside}"
                );
            }
            assert_eq!(
                unsafe { hc_unix_from_fixed(106_751_991_886_463, &mut seconds) },
                HC_OK
            );

            assert_eq!(
                unsafe { hc_parse_iso_date(c"2026-09-21".as_ptr(), &mut fixed) },
                HC_OK
            );
            assert_eq!(fixed, 739_880);
            assert_eq!(
                unsafe { hc_parse_iso_date(c"2026-02-30".as_ptr(), &mut fixed) },
                HC_ERROR_INVALID_DATE
            );
            assert_eq!(
                unsafe { hc_parse_iso_date(c"\xff".as_ptr(), &mut fixed) },
                HC_ERROR_NOT_UTF8
            );
            assert_eq!(
                unsafe { hc_parse_iso_date(core::ptr::null(), &mut fixed) },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe { hc_parse_iso_date(c"2026-09-21".as_ptr(), core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
        }

        #[test]
        fn a_short_buffer_reports_the_required_length_and_writes_nothing() {
            let mut buffer = [0 as c_char; 4];
            let mut written = 0usize;
            let status = unsafe {
                hc_format_iso_date(739_880, buffer.as_mut_ptr(), buffer.len(), &mut written)
            };
            assert_eq!(status, HC_ERROR_BUFFER_TOO_SMALL);
            assert_eq!(written, "2026-09-21".len() + 1);
            assert!(buffer.iter().all(|byte| *byte == 0));
        }

        #[test]
        fn a_sufficient_buffer_receives_a_nul_terminated_iso_date() {
            let mut buffer = [0 as c_char; 32];
            let mut written = 0usize;
            let status = unsafe {
                hc_format_iso_date(739_880, buffer.as_mut_ptr(), buffer.len(), &mut written)
            };
            assert_eq!(status, HC_OK);
            assert_eq!(written, 11);
            let bytes: Vec<u8> = buffer[..10].iter().map(|byte| *byte as u8).collect();
            assert_eq!(core::str::from_utf8(&bytes).unwrap(), "2026-09-21");
            assert_eq!(buffer[10], 0);
        }

        #[test]
        fn the_leap_second_offset_matches_the_published_table() {
            let mut offset = 0i64;
            assert_eq!(
                unsafe { hc_tai_minus_utc(1_700_000_000, 1, &mut offset) },
                HC_OK
            );
            assert_eq!(offset, 37);
            assert_eq!(
                unsafe { hc_tai_minus_utc(63_072_000, 1, &mut offset) },
                HC_OK
            );
            assert_eq!(offset, 10);
        }

        #[test]
        fn the_strict_policy_refuses_to_forecast_across_the_boundary() {
            let mut offset = 0i64;
            assert_eq!(
                unsafe { hc_tai_minus_utc(4_000_000_000, 1, &mut offset) },
                HC_ERROR_NO_DATA
            );
            assert_eq!(
                unsafe { hc_tai_minus_utc(4_000_000_000, 0, &mut offset) },
                HC_OK
            );
            assert_eq!(offset, 37);
        }

        #[test]
        fn the_leap_second_survives_the_boundary() {
            // 2016-12-31 ends with an inserted second.
            let mut has_leap = 0;
            assert_eq!(
                unsafe { hc_day_has_leap_second(1_483_142_400, &mut has_leap) },
                HC_OK
            );
            assert_eq!(has_leap, 1);
            assert_eq!(
                unsafe { hc_day_has_leap_second(1_483_228_800, &mut has_leap) },
                HC_OK
            );
            assert_eq!(has_leap, 0);
        }

        #[test]
        fn the_leap_second_question_refuses_a_day_with_no_int64_bounds() {
            let ask = |unix: i64| {
                let mut has_leap = 7;
                match unsafe { hc_day_has_leap_second(unix, &mut has_leap) } {
                    HC_OK => Ok(has_leap),
                    status => Err(status),
                }
            };
            // The first whole day of the range, and the part-day before it.
            let first_whole = -9_223_372_036_854_720_000;
            assert_eq!(ask(first_whole), Ok(0));
            assert_eq!(ask(first_whole - 1), Err(HC_ERROR_OUT_OF_RANGE));
            assert_eq!(ask(i64::MIN), Err(HC_ERROR_OUT_OF_RANGE));
            // The last day whose end is an int64_t, and the part-day after.
            let last_end = 9_223_372_036_854_720_000;
            assert_eq!(ask(last_end - 1), Ok(0));
            assert_eq!(ask(last_end), Err(HC_ERROR_OUT_OF_RANGE));
            assert_eq!(ask(i64::MAX), Err(HC_ERROR_OUT_OF_RANGE));
        }

        #[test]
        fn unix_and_tai_round_trip_across_the_boundary() {
            let (mut seconds, mut attos) = (0i64, 0u64);
            assert_eq!(
                unsafe { hc_tai_from_unix(1_700_000_000, 1, &mut seconds, &mut attos) },
                HC_OK
            );
            assert_eq!(seconds, 1_700_000_037);

            let (mut unix_seconds, mut is_leap) = (0i64, 0);
            assert_eq!(
                unsafe { hc_utc_from_tai(seconds, 1, &mut unix_seconds, &mut is_leap) },
                HC_OK
            );
            assert_eq!(unix_seconds, 1_700_000_000);
            assert_eq!(is_leap, 0);
        }

        #[test]
        fn the_int64_results_have_the_readmes_ranges() {
            // The Gregorian range, and the first day on either side of it.
            let (mut fixed, mut year, mut month, mut day) = (0i64, 0i64, 0u8, 0u8);
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(-9_999_999, 1, 1, &mut fixed) },
                HC_OK
            );
            assert_eq!(fixed, -3_652_424_999);
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(9_999_999, 12, 31, &mut fixed) },
                HC_OK
            );
            assert_eq!(fixed, 3_652_424_634);
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(i64::MIN, 1, 1, &mut fixed) },
                HC_ERROR_INVALID_DATE
            );
            for outside in [-3_652_425_000, 3_652_424_635, i64::MIN, i64::MAX] {
                assert_eq!(
                    unsafe { hc_gregorian_from_fixed(outside, &mut year, &mut month, &mut day) },
                    HC_ERROR_NO_DATA,
                    "{outside}"
                );
            }
            // TAI runs 37 seconds ahead, so the last 37 POSIX seconds have
            // no TAI reading an int64_t holds.
            let (mut seconds, mut attos) = (0i64, 0u64);
            assert_eq!(
                unsafe { hc_tai_from_unix(i64::MAX - 37, 0, &mut seconds, &mut attos) },
                HC_OK
            );
            assert_eq!(seconds, i64::MAX);
            assert_eq!(
                unsafe { hc_tai_from_unix(i64::MAX - 36, 0, &mut seconds, &mut attos) },
                HC_ERROR_OVERFLOW
            );
            assert_eq!(
                unsafe { hc_tai_from_unix(i64::MIN, 0, &mut seconds, &mut attos) },
                HC_OK
            );
            assert_eq!(seconds, i64::MIN);
            // Every TAI reading has a UTC label.
            let (mut unix_seconds, mut is_leap) = (0i64, 0);
            for tai in [i64::MIN, i64::MAX] {
                assert_eq!(
                    unsafe { hc_utc_from_tai(tai, 0, &mut unix_seconds, &mut is_leap) },
                    HC_OK,
                    "{tai}"
                );
            }
            let mut offset = 0i64;
            for unix in [i64::MIN, i64::MAX] {
                assert_eq!(
                    unsafe { hc_tai_minus_utc(unix, 0, &mut offset) },
                    HC_OK,
                    "{unix}"
                );
            }
        }

        #[test]
        fn the_inserted_second_is_reachable_through_the_boundary() {
            // The TAI reading one second before the 2017 step is 23:59:60 UTC.
            let (mut unix_seconds, mut is_leap) = (0i64, 0);
            assert_eq!(
                unsafe { hc_utc_from_tai(1_483_228_836, 1, &mut unix_seconds, &mut is_leap) },
                HC_OK
            );
            assert_eq!(unix_seconds, 1_483_228_800);
            assert_eq!(is_leap, 1);
        }
    }

    #[cfg(feature = "calendars")]
    mod calendars {
        use super::super::*;
        use super::read_lines;

        /// 21 March 2005 in Turkmen, the module's line: Başgün of Nowruz.
        #[test]
        fn the_naming_period_line_is_the_modules() {
            let mut day = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2005, 3, 21, &mut day) },
                HC_OK
            );
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_naming_period_on(
                    c"gregory".as_ptr(),
                    day,
                    c"tk".as_ptr(),
                    buffer,
                    capacity,
                    written,
                )
            });
            assert_eq!(
                text,
                hc::lines::naming_period_line(&hc::registry(), "tk", "gregory", day)
                    .expect("known")
            );
            assert!(
                text.starts_with("in-force\tturkmen-2002\tNowruz\tBaşgün\t"),
                "{text}"
            );
            let mut written = 0usize;
            assert_eq!(
                unsafe {
                    hc_naming_period_on(
                        core::ptr::null(),
                        day,
                        c"tk".as_ptr(),
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe {
                    hc_naming_period_on(
                        c"no-such-calendar".as_ptr(),
                        day,
                        c"tk".as_ptr(),
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_UNKNOWN
            );
        }

        #[test]
        fn the_first_day_of_the_week_follows_the_locale() {
            let first_day = |locale: *const core::ffi::c_char| {
                let mut day = 0_u8;
                let status = unsafe { hc_first_day_of_week(locale, &raw mut day) };
                assert_eq!(status, HC_OK);
                day
            };
            assert_eq!(first_day(c"en-US".as_ptr()), 7);
            assert_eq!(first_day(c"en-GB".as_ptr()), 1);
            assert_eq!(first_day(c"ja".as_ptr()), 7);
            assert_eq!(first_day(c"zh-Hans".as_ptr()), 1);
            assert_eq!(first_day(c"ar-SA".as_ptr()), 7);
            assert_eq!(first_day(c"fr".as_ptr()), 1);
            assert_eq!(first_day(c"und".as_ptr()), 1);
            assert_eq!(first_day(core::ptr::null()), 1);
            let mut day = 0_u8;
            assert_eq!(
                unsafe { hc_first_day_of_week(c"\xff".as_ptr(), &raw mut day) },
                HC_ERROR_NOT_UTF8
            );
            assert_eq!(
                unsafe { hc_first_day_of_week(c"en".as_ptr(), core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
        }

        #[test]
        fn the_calendar_list_is_the_calendars_names_without_the_day() {
            let rows = |text: &str| -> Vec<Vec<String>> {
                text.lines()
                    .map(|line| line.split('\t').map(str::to_owned).collect())
                    .collect()
            };
            let list = rows(&read_lines(|buffer, capacity, written| unsafe {
                hc_calendar_list(c"ja-JP".as_ptr(), buffer, capacity, written)
            }));
            let calendars = rows(&read_lines(|buffer, capacity, written| unsafe {
                hc_calendars(739_880, c"ja-JP".as_ptr(), buffer, capacity, written)
            }));
            assert_eq!(list.len(), hc::registry().len());
            assert!(list.iter().all(|row| row.len() == 6), "{list:?}");
            for (row, full) in list.iter().zip(&calendars) {
                assert_eq!(row[..3], full[..3]);
                assert_eq!(row[5], full[9], "the native locales, {row:?}");
            }
            let japanese = list
                .iter()
                .find(|row| row[0] == "japanese")
                .expect("japanese");
            assert_eq!(japanese[3..], ["ja", "hc-calendars-regional", "ja"]);
            assert_eq!(
                unsafe {
                    hc_calendar_list(
                        c"\xff".as_ptr(),
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_NOT_UTF8
            );
        }

        #[test]
        fn one_day_in_every_calendar_decodes_column_by_column() {
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_describe_day(739_880, c"ja-JP".as_ptr(), buffer, capacity, written)
            });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert_eq!(rows.len(), hc::registry().len());
            assert!(rows.iter().all(|row| row.len() == 18), "{rows:?}");
            let japanese = rows
                .iter()
                .find(|row| row[0] == "japanese")
                .expect("japanese");
            assert_eq!(
                japanese[2..10],
                ["reiwa", "令和", "8", "9", "0", "9月", "21", "0"]
            );
            assert_eq!(japanese[11..14], ["", "", "in-use"]);
            assert_eq!(japanese[15..18], ["令和8年9月21日", "ja", ""]);
            let rumi = rows.iter().find(|row| row[0] == "rumi").expect("rumi");
            assert_eq!(rumi[11..14], ["7", "after-supported-range", ""]);
            // Neither Japanese nor Turkish names the Rumi calendar, so it
            // is rendered in English, and says so.
            assert_eq!(rumi[15..17], ["", "en"]);
            // A null locale is `und`.
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_describe_day(739_880, core::ptr::null(), buffer, capacity, written)
            });
            assert!(text.contains("\tM09\t"), "{text}");
            assert_eq!(
                unsafe {
                    hc_describe_day(
                        739_880,
                        c"\xff".as_ptr(),
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_NOT_UTF8
            );
        }

        #[test]
        fn the_gregorian_adoption_is_one_line_per_step() {
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_gregorian_adoption(c"CN".as_ptr(), buffer, capacity, written)
            });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert_eq!(rows.len(), 2, "{rows:?}");
            assert!(rows.iter().all(|row| row.len() == 7), "{rows:?}");
            assert_eq!(rows[0][..4], ["697977", "697978", "chinese", "partial"]);
            assert_eq!(rows[1][3], "civil");
            let mut written = 0usize;
            let mut buffer = [0 as c_char; 4];
            assert_eq!(
                unsafe {
                    hc_gregorian_adoption(c"ZZ".as_ptr(), buffer.as_mut_ptr(), 4, &mut written)
                },
                HC_OK
            );
            assert_eq!(written, 1, "an unknown region is the empty string");
            assert_eq!(
                unsafe {
                    hc_gregorian_adoption(core::ptr::null(), buffer.as_mut_ptr(), 4, &mut written)
                },
                HC_ERROR_NULL_POINTER
            );
        }

        #[test]
        fn the_units_the_calendars_and_the_locales_are_lines_too() {
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_calendar_units(
                    c"japanese".as_ptr(),
                    0,
                    // 1989-01-01 to 2019-12-31.
                    726_103,
                    737_424,
                    c"ja".as_ptr(),
                    buffer,
                    capacity,
                    written,
                )
            });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            let labels: Vec<&str> = rows.iter().map(|row| row[2]).collect();
            assert_eq!(labels, ["昭和", "平成", "令和"]);
            assert!(rows.iter().all(|row| row.len() == 8), "{rows:?}");
            assert_eq!(rows[2][7], "ja");
            assert_eq!(
                unsafe {
                    hc_calendar_units(
                        c"no-such-calendar".as_ptr(),
                        0,
                        0,
                        10,
                        core::ptr::null(),
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_calendar_units(
                        c"gregory".as_ptr(),
                        4,
                        0,
                        10,
                        core::ptr::null(),
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_calendar_units(
                        core::ptr::null(),
                        0,
                        0,
                        10,
                        core::ptr::null(),
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_NULL_POINTER
            );

            // At the cap, 100 000 days as days, the text is measured; one
            // day more, or a trillion, is refused without being built, and
            // `written` is left alone.
            let units_of = |unit: u32, from: i64, to: i64, written: &mut usize| unsafe {
                hc_calendar_units(
                    c"gregory".as_ptr(),
                    unit,
                    from,
                    to,
                    c"en".as_ptr(),
                    core::ptr::null_mut(),
                    0,
                    written,
                )
            };
            let cap = hc::lines::MAX_CALENDAR_UNITS as i64;
            assert_eq!(cap, 100_000);
            let mut written = 0usize;
            assert_eq!(
                units_of(3, 700_000, 700_000 + cap, &mut written),
                HC_ERROR_BUFFER_TOO_SMALL
            );
            assert!(written > 100_000, "{written}");
            let mut untouched = 7usize;
            assert_eq!(
                units_of(3, 700_000, 700_000 + cap + 1, &mut untouched),
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                units_of(3, 0, 1_000_000_000_000, &mut untouched),
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(untouched, 7);
            assert_eq!(
                units_of(1, 700_000, 700_000 + 365_243, &mut written),
                HC_ERROR_BUFFER_TOO_SMALL
            );

            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_calendars(739_880, c"native".as_ptr(), buffer, capacity, written)
            });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert_eq!(rows.len(), hc::registry().len());
            assert!(rows.iter().all(|row| row.len() == 11), "{rows:?}");
            let hebrew = rows.iter().find(|row| row[0] == "hebrew").expect("hebrew");
            assert_eq!(hebrew[2], "Hebrew");
            assert_eq!(hebrew[9], "he");
            assert!(!hebrew[1].is_empty(), "named in its own language");

            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_locales(buffer, capacity, written)
            });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert_eq!(rows.len(), hc::hc_i18n::data::LOCALES.len());
            assert!(rows.iter().all(|row| row.len() == 7), "{rows:?}");
            assert_eq!(rows[0][0], "am");
        }
    }

    #[cfg(feature = "holiday")]
    mod holiday {
        use super::super::*;
        use super::read_lines;

        #[test]
        fn holiday_tables_answer_by_identifier() {
            use core::ffi::CStr;
            let jp = c"JP";
            let xnys = c"XNYS";
            let us = c"US";
            let zz = c"ZZ";
            let mut fixed = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2026, 4, 3, &mut fixed) },
                HC_OK
            );
            let mut answer = -1;
            assert_eq!(
                unsafe {
                    hc_holiday_is_day_off(xnys.as_ptr(), core::ptr::null(), fixed, &mut answer)
                },
                HC_OK
            );
            assert_eq!(answer, 1);
            assert_eq!(
                unsafe {
                    hc_holiday_is_day_off(us.as_ptr(), core::ptr::null(), fixed, &mut answer)
                },
                HC_OK
            );
            assert_eq!(answer, 0);
            assert_eq!(
                unsafe {
                    hc_holiday_is_day_off(zz.as_ptr(), core::ptr::null(), fixed, &mut answer)
                },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_holiday_is_day_off(
                        jp.as_ptr(),
                        core::ptr::null(),
                        fixed,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe {
                    hc_holiday_is_day_off(core::ptr::null(), core::ptr::null(), fixed, &mut answer)
                },
                HC_ERROR_NULL_POINTER
            );
            // A string that is not UTF-8 says so, in the code and in the region
            // alike, rather than passing for a null pointer or for no region.
            let not_utf8 = c"\xff";
            assert_eq!(
                unsafe {
                    hc_holiday_is_day_off(not_utf8.as_ptr(), core::ptr::null(), fixed, &mut answer)
                },
                HC_ERROR_NOT_UTF8
            );
            assert_eq!(
                unsafe {
                    hc_holiday_is_day_off(jp.as_ptr(), not_utf8.as_ptr(), fixed, &mut answer)
                },
                HC_ERROR_NOT_UTF8
            );
            let mut written = 0usize;
            assert_eq!(
                unsafe {
                    hc_holidays_in_year(
                        zz.as_ptr(),
                        core::ptr::null(),
                        2026,
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_UNKNOWN
            );
            // Too small reports the need; big enough receives the lines.
            assert_eq!(
                unsafe {
                    hc_holidays_in_year(
                        jp.as_ptr(),
                        core::ptr::null(),
                        2026,
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_BUFFER_TOO_SMALL
            );
            assert!(written > 1);
            let mut buffer = vec![0 as c_char; written];
            assert_eq!(
                unsafe {
                    hc_holidays_in_year(
                        jp.as_ptr(),
                        core::ptr::null(),
                        2026,
                        buffer.as_mut_ptr(),
                        buffer.len(),
                        &mut written,
                    )
                },
                HC_OK
            );
            let text = unsafe { CStr::from_ptr(buffer.as_ptr()) }
                .to_str()
                .expect("UTF-8");
            assert!(
                text.starts_with("2026-01-01\tNew Year's Day\t元日\tpublic\texact\t0\t\n"),
                "{text}"
            );
            assert!(text.contains("\t1\t2026-05-03\n"), "{text}");
            assert_eq!(
                unsafe { hc_holiday_codes(core::ptr::null_mut(), 0, &mut written) },
                HC_ERROR_BUFFER_TOO_SMALL
            );
            let mut buffer = vec![0 as c_char; written];
            assert_eq!(
                unsafe { hc_holiday_codes(buffer.as_mut_ptr(), buffer.len(), &mut written) },
                HC_OK
            );
            let text = unsafe { CStr::from_ptr(buffer.as_ptr()) }
                .to_str()
                .expect("UTF-8");
            assert!(
                text.contains("\nJP\n") && text.contains("\nXNYS\n") && text.contains("un-days\n")
            );
        }

        #[test]
        fn one_day_across_every_table_decodes_column_by_column() {
            let mut day = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2026, 5, 6, &mut day) },
                HC_OK
            );
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_holidays_on(day, buffer, capacity, written)
            });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert!(rows.iter().all(|row| row.len() == 9), "{rows:?}");
            let substitute = rows
                .iter()
                .find(|row| row[0] == "JP" && row[7] == "1")
                .expect("Japan's substitute for Constitution Memorial Day");
            assert_eq!(
                substitute[1..7],
                [
                    "Japan",
                    "Constitution Memorial Day",
                    "憲法記念日",
                    "public",
                    "exact",
                    ""
                ]
            );
            assert_eq!(substitute[8], (day - 3).to_string());
            assert_eq!(
                unsafe {
                    hc_holidays_on(i64::MAX, core::ptr::null_mut(), 0, core::ptr::null_mut())
                },
                HC_ERROR_OUT_OF_RANGE
            );
        }
    }

    #[cfg(feature = "seasons")]
    mod seasons {
        use super::super::*;
        use super::read_lines;

        #[test]
        fn the_term_and_pentad_in_effect_decode_column_by_column() {
            let mut day = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2024, 2, 10, &mut day) },
                HC_OK
            );
            let term = read_lines(|buffer, capacity, written| unsafe {
                hc_term_in_effect(day, c"japan".as_ptr(), buffer, capacity, written)
            });
            let columns: Vec<&str> = term.trim_end().split('\t').collect();
            assert_eq!(columns.len(), 7, "{columns:?}");
            assert_eq!(columns[..3], ["21", "立春", "立春"]);
            assert_eq!(columns[3], (day - 6).to_string());
            assert_eq!(columns[4], (day + 8).to_string());
            let pentad = read_lines(|buffer, capacity, written| unsafe {
                hc_pentad_in_effect(day, core::ptr::null(), buffer, capacity, written)
            });
            let columns: Vec<&str> = pentad.trim_end().split('\t').collect();
            assert_eq!(columns.len(), 7, "{columns:?}");
            assert_eq!(columns[..3], ["64", "蟄蟲始振", "黄鶯睍睆"]);
            assert_eq!(
                unsafe {
                    hc_term_in_effect(
                        day,
                        c"mars".as_ptr(),
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_pentad_in_effect(
                        day,
                        c"\xff".as_ptr(),
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_NOT_UTF8
            );
        }

        /// KASI's 월력요항: 한식 on 6 April 2026.
        #[test]
        fn the_cold_food_day_crosses_through_an_out_parameter() {
            let (mut day, mut expected) = (0i64, 0i64);
            assert_eq!(
                unsafe { hc_cold_food_day(c"hansik".as_ptr(), 2026, &mut day) },
                HC_OK
            );
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2026, 4, 6, &mut expected) },
                HC_OK
            );
            assert_eq!(day, expected);
            assert_eq!(
                unsafe { hc_cold_food_day(c"hanshi".as_ptr(), 2026, &mut day) },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe { hc_cold_food_day(c"hansik".as_ptr(), 3001, &mut day) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_cold_food_day(core::ptr::null(), 2026, &mut day) },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe { hc_cold_food_day(c"hansik".as_ptr(), 2026, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
        }

        /// The days of the years −1000 to 3000 answer, as `hc_sky_at`'s do;
        /// the days either side of them are refused.
        #[test]
        fn the_term_and_pentad_refuse_days_outside_the_era() {
            let (mut first, mut last) = (0i64, 0i64);
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(-1000, 1, 1, &mut first) },
                HC_OK
            );
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(3000, 12, 31, &mut last) },
                HC_OK
            );
            let null = core::ptr::null_mut();
            for day in [first, last] {
                let mut written = 0usize;
                assert_eq!(
                    unsafe { hc_term_in_effect(day, core::ptr::null(), null, 0, &mut written) },
                    HC_ERROR_BUFFER_TOO_SMALL
                );
                assert!(written > 0);
            }
            for day in [first - 1, last + 1, i64::MIN, i64::MAX] {
                for status in [
                    unsafe { hc_term_in_effect(day, core::ptr::null(), null, 0, null.cast()) },
                    unsafe { hc_pentad_in_effect(day, core::ptr::null(), null, 0, null.cast()) },
                ] {
                    assert_eq!(status, HC_ERROR_OUT_OF_RANGE, "{day}");
                }
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

        #[test]
        fn days_follow_the_zones_wall_clock() {
            // 08:00 on 25 September 2026 in Tokyo is 23:00 UTC on the 24th.
            let mut september_24 = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2026, 9, 24, &mut september_24) },
                HC_OK
            );
            let instant = (september_24 - 719_163) * 86_400 + 23 * 3_600;
            let mut day = 0i64;
            assert_eq!(
                unsafe { hc_fixed_from_unix_in_zone(instant, c"Asia/Tokyo".as_ptr(), &mut day) },
                HC_OK
            );
            assert_eq!(day, september_24 + 1);
            let mut start = 0i64;
            assert_eq!(
                unsafe {
                    hc_unix_from_fixed_in_zone(september_24 + 1, c"Asia/Tokyo".as_ptr(), &mut start)
                },
                HC_OK
            );
            // The Tokyo day began at 15:00 UTC on the 24th.
            assert_eq!(start, instant - 8 * 3_600);
            // Cairo's clocks go forward at midnight: 24 April 2026 begins
            // at 01:00 EEST, 22:00 UTC on the 23rd.
            let april_24 = september_24 - 153;
            assert_eq!(
                unsafe {
                    hc_unix_from_fixed_in_zone(april_24, c"Africa/Cairo".as_ptr(), &mut start)
                },
                HC_OK
            );
            assert_eq!(start, (april_24 - 719_163) * 86_400 - 2 * 3_600);
            assert_eq!(
                unsafe { hc_fixed_from_unix_in_zone(0, c"Mars/Olympus".as_ptr(), &mut day) },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe { hc_fixed_from_unix_in_zone(0, core::ptr::null(), &mut day) },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe { hc_fixed_from_unix_in_zone(0, c"UTC".as_ptr(), core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
        }

        /// `hc_unix_from_fixed_in_zone` as a `Result`.
        fn starts(fixed: i64, zone: &core::ffi::CStr) -> Result<i64, HcStatus> {
            let mut start = 0i64;
            match unsafe { hc_unix_from_fixed_in_zone(fixed, zone.as_ptr(), &mut start) } {
                HC_OK => Ok(start),
                status => Err(status),
            }
        }

        #[test]
        fn a_days_start_refuses_rather_than_saturates_downwards() {
            // The last day whose UTC midnight fits an int64_t, and the
            // first that would not.
            let first = -106_751_990_448_137;
            let midnight = -9_223_372_036_854_720_000;
            assert_eq!(starts(first, c"UTC"), Ok(midnight));
            assert_eq!(starts(first - 1, c"UTC"), Err(HC_ERROR_OUT_OF_RANGE));
            assert_eq!(starts(first, c"Asia/Tokyo"), Ok(midnight - 9 * 3_600));
            assert_eq!(starts(first - 1, c"Asia/Tokyo"), Err(HC_ERROR_OUT_OF_RANGE));
            assert_eq!(starts(i64::MIN, c"UTC"), Err(HC_ERROR_OUT_OF_RANGE));
            assert_eq!(
                starts(i64::MIN, c"America/New_York"),
                Err(HC_ERROR_OUT_OF_RANGE)
            );
            // There are no sentinels here, so a day 54 billion years back
            // is an answer, as it cannot be in the WebAssembly module.
            let far = -19_723_095_000_000;
            assert_eq!(starts(far, c"UTC"), Ok((far - 719_163) * 86_400));
        }

        #[test]
        fn a_days_start_refuses_rather_than_saturates_upwards() {
            let last = 106_751_991_886_463;
            let midnight = 9_223_372_036_854_720_000;
            assert_eq!(starts(last, c"UTC"), Ok(midnight));
            assert_eq!(starts(last + 1, c"UTC"), Err(HC_ERROR_OUT_OF_RANGE));
            assert_eq!(starts(last, c"America/Sao_Paulo"), Ok(midnight + 3 * 3_600));
            assert_eq!(
                starts(last + 1, c"America/Sao_Paulo"),
                Err(HC_ERROR_OUT_OF_RANGE)
            );
            // Tokyo's day after UTC's last begins nine hours before it,
            // which still fits.
            assert_eq!(
                starts(last + 1, c"Asia/Tokyo"),
                Ok(midnight - 9 * 3_600 + 86_400)
            );
            assert_eq!(starts(last + 2, c"Asia/Tokyo"), Err(HC_ERROR_OUT_OF_RANGE));
            // 2^53 days, whose start a saturating product would clamp to
            // INT64_MAX.
            assert_eq!(starts(1 << 53, c"Asia/Tokyo"), Err(HC_ERROR_OUT_OF_RANGE));
            assert_eq!(starts(i64::MAX, c"UTC"), Err(HC_ERROR_OUT_OF_RANGE));
        }

        #[test]
        fn a_loaded_zone_answers_by_name() {
            let name = c"Test/Eastern";
            let mut day = 0i64;
            assert_eq!(
                unsafe { hc_fixed_from_unix_in_zone(0, name.as_ptr(), &mut day) },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_zone_load(
                        name.as_ptr(),
                        TZIF_V2_EASTERN.as_ptr(),
                        TZIF_V2_EASTERN.len(),
                    )
                },
                HC_OK
            );
            // 2025-03-09 07:00 UTC is 03:00 EDT, just after the gap.
            let mut march_9 = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2025, 3, 9, &mut march_9) },
                HC_OK
            );
            let midnight_utc = (march_9 - 719_163) * 86_400;
            assert_eq!(
                unsafe {
                    hc_fixed_from_unix_in_zone(midnight_utc + 7 * 3_600, name.as_ptr(), &mut day)
                },
                HC_OK
            );
            assert_eq!(day, march_9);
            let junk = b"not a zone";
            assert_eq!(
                unsafe { hc_zone_load(c"Test/Junk".as_ptr(), junk.as_ptr(), junk.len()) },
                HC_ERROR_MALFORMED
            );
            assert_eq!(
                unsafe { hc_zone_load(c"".as_ptr(), junk.as_ptr(), junk.len()) },
                HC_ERROR_NULL_POINTER
            );
        }
    }

    #[cfg(feature = "tz")]
    mod zones {
        use super::super::*;
        use super::read_lines;

        fn line(zone: &core::ffi::CStr, locale: &core::ffi::CStr) -> String {
            read_lines(|buffer, capacity, written| unsafe {
                hc_zone_location(zone.as_ptr(), locale.as_ptr(), buffer, capacity, written)
            })
        }

        /// `zone1970.tab` 2026c: `JP,AU +353916+1394441 Asia/Tokyo Eyre
        /// Bird Observatory`; `zone.tab`: `NO +5955+01045 Europe/Oslo`;
        /// `backward`: `Link Asia/Kolkata Asia/Calcutta`.
        #[test]
        fn the_lines_are_the_modules_and_links_answer_with_their_rows() {
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_zones(core::ptr::null(), buffer, capacity, written)
            });
            assert_eq!(text.lines().count(), 312);
            assert!(text.lines().any(|line| line
                == "Asia/Tokyo\t35.654444\t139.744722\tJP;AU\tEyre Bird Observatory\tTokyo\ten"));
            assert!(
                line(c"Europe/Oslo", c"en").starts_with("Europe/Oslo\t59.916667\t10.750000\tNO\t")
            );
            assert!(line(c"Asia/Calcutta", c"en").starts_with("Asia/Kolkata\t"));
            let mut written = 0usize;
            assert_eq!(
                unsafe {
                    hc_zone_location(
                        c"UTC".as_ptr(),
                        c"en".as_ptr(),
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_zone_location(
                        core::ptr::null(),
                        c"en".as_ptr(),
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_NULL_POINTER
            );
            let tokyo = line(c"Asia/Tokyo", c"ja");
            let expected = if cfg!(feature = "calendars") {
                "\t東京\tja\n"
            } else {
                "\tTokyo\ten\n"
            };
            assert!(tokyo.ends_with(expected), "{tokyo}");
        }
    }
    #[cfg(feature = "deep-time")]
    mod deep_time {
        use super::super::*;
        use super::read_lines;

        #[test]
        fn deep_time_lines_decode_column_by_column() {
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_place_years_ago(66.0e6, 0.0, c"ja-JP".as_ptr(), buffer, capacity, written)
            });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert!(rows.iter().all(|row| row.len() == 15), "{rows:?}");
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
            assert_eq!(rows[8][..3], ["age", "Maastrichtian", "Upper Cretaceous"]);
            assert_eq!(rows[8][11], "megayears-before-present");
            assert_eq!(rows[8][14], "マーストリヒチアン");
            assert_eq!(rows[2][14], "");
            // A few years ahead the whole present chain is still there.
            let near = read_lines(|buffer, capacity, written| unsafe {
                hc_place_years_ago(-3.0, 0.0, core::ptr::null(), buffer, capacity, written)
            });
            assert!(near.contains("\tMeghalayan\t"), "{near}");
            assert!(near.contains("\tModern period\t"), "{near}");
            let cosmic = read_lines(|buffer, capacity, written| unsafe {
                hc_cosmic_events(core::ptr::null(), buffer, capacity, written)
            });
            assert_eq!(
                cosmic.lines().count(),
                hc::hc_deep_time::universe::EPOCHS.len() + hc::hc_deep_time::universe::EVENTS.len()
            );
            let eons = read_lines(|buffer, capacity, written| unsafe {
                hc_geologic_intervals(0, c"zh-Hans".as_ptr(), buffer, capacity, written)
            });
            assert!(
                eons.starts_with("eon\tPhanerozoic\t\t538.8\t0.6\t4\t0\t0\t0\t1\t0\t"),
                "{eons}"
            );
            assert!(
                eons.lines()
                    .next()
                    .is_some_and(|line| line.ends_with("\t显生宇"))
            );
            assert_eq!(
                unsafe {
                    hc_geologic_intervals(
                        5,
                        core::ptr::null(),
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_place_years_ago(
                        f64::NAN,
                        0.0,
                        core::ptr::null(),
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_OUT_OF_RANGE
            );
        }
    }

    #[cfg(feature = "sky")]
    mod sky {
        use super::super::*;
        use super::{measured, read_lines};

        /// The Sun in the Moon's face of Libra an hour after the NAOJ's 秋分
        /// of 2026, 23 September 00:05 UTC, as the module writes it.
        #[test]
        fn the_decan_line_is_the_modules() {
            let instant = at(2026, 9, 23, 1, 5);
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_decan_at(instant, buffer, capacity, written)
            });
            assert_eq!(
                text,
                hc::sky_lines::decan_line(instant).expect("in the era")
            );
            assert!(text.starts_with("7\tLibra\t1\tmoon\tMoon\t"), "{text}");
            let decan = |unix_seconds: i64| {
                measured(|buffer, capacity, written| unsafe {
                    hc_decan_at(unix_seconds, buffer, capacity, written)
                })
            };
            assert_eq!(decan(-93_724_128_000), HC_ERROR_BUFFER_TOO_SMALL);
            assert_eq!(decan(32_535_215_999), HC_ERROR_BUFFER_TOO_SMALL);
            assert_eq!(decan(-93_724_128_001), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(decan(32_535_216_000), HC_ERROR_OUT_OF_RANGE);
        }

        /// The POSIX timestamp of a UTC date and time.
        fn at(year: i64, month: u8, day: u8, hour: i64, minute: i64) -> i64 {
            let mut fixed = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(year, month, day, &mut fixed) },
                HC_OK
            );
            (fixed - 719_163) * 86_400 + hour * 3_600 + minute * 60
        }

        fn rows(text: &str) -> Vec<Vec<String>> {
            text.lines()
                .map(|line| line.split('\t').map(str::to_owned).collect())
                .collect()
        }

        /// The 暦要項 of the National Astronomical Observatory of Japan for
        /// 2026 puts the new moon of September at 11 September 12:27 JST,
        /// 03:27 UTC, and the equinox at 23 September 09:05 JST, 00:05 UTC.
        #[test]
        fn the_sky_the_terms_and_the_phases_decode_column_by_column() {
            let instant = at(2026, 9, 11, 3, 0);
            let sky = read_lines(|buffer, capacity, written| unsafe {
                hc_sky_at(instant, buffer, capacity, written)
            });
            let columns = rows(&sky);
            assert_eq!(columns.len(), 1, "{sky:?}");
            let columns = &columns[0];
            assert_eq!(columns.len(), 12, "{columns:?}");
            let elongation: f64 = columns[5].parse().expect("a number");
            assert!(elongation > 359.0, "{elongation}");
            let next: i64 = columns[8].parse().expect("a timestamp");
            assert!((next - at(2026, 9, 11, 3, 27)).abs() <= 90, "{next}");
            assert_eq!(columns[10], "predicted");
            assert!(columns[11].contains("VSOP87"), "{}", columns[11]);

            let (from, to) = (at(2026, 9, 1, 0, 0), at(2026, 10, 1, 0, 0));
            let terms = rows(&read_lines(|buffer, capacity, written| unsafe {
                hc_solar_terms_between(from, to, buffer, capacity, written)
            }));
            assert_eq!(terms.len(), 2, "{terms:?}");
            assert_eq!(terms[1][0], "180");
            assert_eq!(terms[1][2..], ["秋分", "秋分"]);
            let equinox: i64 = terms[1][1].parse().expect("a timestamp");
            assert!((equinox - at(2026, 9, 23, 0, 5)).abs() <= 90, "{equinox}");

            let phases = rows(&read_lines(|buffer, capacity, written| unsafe {
                hc_moon_phases_between(from, to, buffer, capacity, written)
            }));
            assert_eq!(phases.len(), 4, "{phases:?}");
            assert_eq!(phases[1][0], "0");
            assert_eq!(phases[1][2..], ["new", ""]);
            assert_eq!(phases[1][1], columns[8]);
        }

        #[test]
        fn instants_outside_the_era_and_spans_too_long_are_refused() {
            let null = core::ptr::null_mut();
            let mut written = 0usize;
            assert_eq!(
                unsafe { hc_sky_at(at(3001, 1, 1, 0, 0), null, 0, &mut written) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_sky_at(i64::MIN, null, 0, &mut written) },
                HC_ERROR_OUT_OF_RANGE
            );
            let from = at(1600, 1, 1, 0, 0);
            assert_eq!(
                unsafe {
                    hc_solar_terms_between(from, at(2001, 1, 1, 0, 0), null, 0, &mut written)
                },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe {
                    hc_moon_phases_between(from, at(3001, 1, 1, 0, 1), null, 0, &mut written)
                },
                HC_ERROR_OUT_OF_RANGE
            );
            // An empty span is an empty, terminated answer.
            let mut buffer = [7 as c_char; 4];
            assert_eq!(
                unsafe { hc_solar_terms_between(from, from, buffer.as_mut_ptr(), 4, &mut written) },
                HC_OK
            );
            assert_eq!(written, 1);
            assert_eq!(buffer[0], 0);
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

        fn number(cell: &str) -> f64 {
            cell.parse()
                .unwrap_or_else(|_| panic!("not a number: {cell}"))
        }

        /// The Last Glacial Maximum, 21 000 years before 1950, as the
        /// author's own table and the PMIP experiments print it, and the
        /// same line at the head of a series.
        #[test]
        fn the_orbit_and_a_series_decode_column_by_column() {
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_orbit_at(21_000.0, buffer, capacity, written)
            });
            let lines = rows(&text);
            assert_eq!(lines.len(), 1, "{text:?}");
            let columns = &lines[0];
            assert_eq!(columns.len(), 11, "{columns:?}");
            assert!(
                (number(&columns[0]) - 0.018_994).abs() < 1e-6,
                "{columns:?}"
            );
            assert_eq!(columns[1], "0.002");
            assert!((number(&columns[2]) - 22.949).abs() < 1e-3, "{columns:?}");
            assert!((number(&columns[4]) - 114.42).abs() < 0.01, "{columns:?}");
            assert!((number(&columns[6]) - 0.017_29).abs() < 5e-6, "{columns:?}");
            assert_eq!(columns[9], "1360");
            assert!(
                columns[10].contains("SOLAR_CONSTANT_BERGER_LOUTRE_1991"),
                "{}",
                columns[10]
            );

            let series = read_lines(|buffer, capacity, written| unsafe {
                hc_orbit_series(21_000.0, 23_000.0, 1_000.0, buffer, capacity, written)
            });
            let series = rows(&series);
            assert_eq!(series.len(), 3, "{series:?}");
            assert!(series.iter().all(|line| line.len() == 12), "{series:?}");
            assert_eq!(series[0][0], "21000");
            assert_eq!(series[0][1..], columns[..]);
            assert_eq!(series[2][0], "23000");
        }

        #[test]
        fn epochs_off_the_span_and_series_too_long_are_refused() {
            let null = core::ptr::null_mut();
            let mut written = 0usize;
            for epoch in [1_000_001.0, -1_000_001.0, f64::NAN] {
                assert_eq!(
                    unsafe { hc_orbit_at(epoch, null, 0, &mut written) },
                    HC_ERROR_OUT_OF_RANGE,
                    "{epoch}"
                );
            }
            for (from, to, step) in [
                (0.0, 1_000_001.0, 100.0),
                (0.0, 1_000.0, 0.0),
                (0.0, 1_000.0, f64::NAN),
                (0.0, 10_000.0, 1.0),
            ] {
                assert_eq!(
                    unsafe { hc_orbit_series(from, to, step, null, 0, &mut written) },
                    HC_ERROR_OUT_OF_RANGE,
                    "{from} to {to} by {step}"
                );
            }
            // A `to` before `from` is an empty, terminated answer.
            let mut buffer = [7 as c_char; 4];
            assert_eq!(
                unsafe {
                    hc_orbit_series(1_000.0, 0.0, 100.0, buffer.as_mut_ptr(), 4, &mut written)
                },
                HC_OK
            );
            assert_eq!(written, 1);
            assert_eq!(buffer[0], 0);
        }
    }

    #[cfg(feature = "timestamps")]
    mod time_scales {
        use super::super::*;
        use super::{measured, read_lines};

        /// `TTBIPM.2025`'s 27.6740 µs on MJD 58 479, as the module reads it.
        #[test]
        fn the_tt_bipm_line_is_the_modules() {
            let series = c"58479\t27.6740\n58489\t27.6745\n";
            let tai = (58_479 - 40_587) * 86_400 + 37;
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_tt_bipm(series.as_ptr(), tai, 0, 1, buffer, capacity, written)
            });
            let expected =
                hc::time_lines::tt_bipm_line(series.to_str().expect("UTF-8"), tai, 0, true);
            assert_eq!(Ok(text), expected);
            let mut written = 0usize;
            assert_eq!(
                unsafe {
                    hc_tt_bipm(
                        series.as_ptr(),
                        tai - 1,
                        0,
                        1,
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_NO_DATA
            );
        }

        /// POSIX 0 is `@400000000000000a` on daemontools' ordinary clock.
        #[test]
        fn the_posix_plus_10_labels_cross_both_ways() {
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_tai64_posix_plus_10_encode(
                    1,
                    500_000_000_000_000_000,
                    c"tai64n".as_ptr(),
                    buffer,
                    capacity,
                    written,
                )
            });
            assert_eq!(text, "400000000000000b1dcd6500\n");
            let (mut seconds, mut attos) = (7i64, 7u64);
            assert_eq!(
                unsafe {
                    hc_tai64_posix_plus_10_decode(
                        c"400000000000000A".as_ptr(),
                        &mut seconds,
                        &mut attos,
                    )
                },
                HC_OK
            );
            assert_eq!((seconds, attos), (0, 0));
            assert_eq!(
                unsafe {
                    hc_tai64_posix_plus_10_encode(
                        0,
                        0,
                        c"tai64na".as_ptr(),
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe {
                    hc_tai64_posix_plus_10_decode(core::ptr::null(), &mut seconds, &mut attos)
                },
                HC_ERROR_NULL_POINTER
            );
        }

        /// RFC 9562's version 6 vector, Appendix A, and its version 4 one.
        #[test]
        fn a_uuid_timestamp_crosses_the_boundary() {
            let (mut version, mut timestamp, mut unix, mut attos) = (0, 0u64, 0i64, 7u64);
            assert_eq!(
                unsafe {
                    hc_uuid_timestamp(
                        c"1EC9414C-232A-6B00-B3C8-9F6BDECED846".as_ptr(),
                        &mut version,
                        &mut timestamp,
                        &mut unix,
                        &mut attos,
                    )
                },
                HC_OK
            );
            assert_eq!(
                (version, timestamp, unix, attos),
                (6, 0x1EC_9414_C232_AB00, 1_645_557_742, 0)
            );
            assert_eq!(
                unsafe {
                    hc_uuid_timestamp(
                        c"919108f7-52d1-4320-9bac-f847db4148a8".as_ptr(),
                        &mut version,
                        &mut timestamp,
                        &mut unix,
                        &mut attos,
                    )
                },
                HC_ERROR_NO_DATA
            );
            assert_eq!(
                unsafe {
                    hc_uuid_timestamp(
                        c"not a uuid".as_ptr(),
                        &mut version,
                        &mut timestamp,
                        &mut unix,
                        &mut attos,
                    )
                },
                HC_ERROR_MALFORMED
            );
        }

        /// RFC 5905's Figure 4: 8 February 2036 is era 1, offset 63 104.
        #[test]
        fn an_ntp_timestamp_crosses_with_its_era() {
            let (mut era, mut offset, mut fraction, mut unix, mut attos) =
                (7, 7u32, 7u64, 7i64, 7u64);
            assert_eq!(
                unsafe {
                    hc_ntp_resolve(
                        63_104,
                        0,
                        1_893_456_000,
                        &mut era,
                        &mut offset,
                        &mut fraction,
                        &mut unix,
                        &mut attos,
                    )
                },
                HC_OK
            );
            assert_eq!(
                (era, offset, fraction, unix, attos),
                (1, 63_104, 0, 2_086_041_600, 0)
            );
            assert_eq!(
                unsafe {
                    hc_ntp_resolve(
                        0,
                        0,
                        0,
                        &mut era,
                        &mut offset,
                        &mut fraction,
                        &mut unix,
                        &mut attos,
                    )
                },
                HC_ERROR_NO_DATA
            );
        }

        /// RFC 9562's example instant, POSIX 1 645 557 742, is the timestamp
        /// of its vectors, and RFC 5905's Figure 4 puts 1 January 1970 at
        /// era 0, offset 2 208 988 800.
        #[test]
        fn a_posix_instant_crosses_as_a_uuid_timestamp_and_an_ntp_date() {
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_uuid_timestamp_encode(1_645_557_742, 0, buffer, capacity, written)
            });
            assert_eq!(
                line,
                "138648505420000000\tc232ab00-9414-11ec\t1ec9414c-232a-6b00\n"
            );
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_ntp_encode(0, 0, buffer, capacity, written)
            });
            assert_eq!(
                line,
                "0\t2208988800\t0\t0000000083aa7e800000000000000000\t83aa7e8000000000\n"
            );
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_uuid_timestamp_encode(-12_219_292_801, 0, null, 0, null.cast()) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_ntp_encode(i64::MAX, 0, null, 0, null.cast()) },
                HC_ERROR_OVERFLOW
            );
            assert_eq!(
                unsafe { hc_ntp_encode(0, 1_000_000_000_000_000_000, null, 0, null.cast()) },
                HC_ERROR_OUT_OF_RANGE
            );
        }

        /// 2026-09-26 23:59:58 is the FAT words 23 866 and 49 021.
        #[test]
        fn the_fat_words_cross_both_ways() {
            let mut day = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2026, 9, 26, &mut day) },
                HC_OK
            );
            let (mut fixed, mut seconds) = (0i64, 0u32);
            assert_eq!(
                unsafe { hc_fat_decode(23_866, 49_021, &mut fixed, &mut seconds) },
                HC_OK
            );
            assert_eq!((fixed, seconds), (day, 86_398));
            let (mut date, mut time) = (0u16, 0u16);
            assert_eq!(
                unsafe { hc_fat_encode(day, 86_399, &mut date, &mut time) },
                HC_OK
            );
            assert_eq!((date, time), (23_866, 49_021));
            assert_eq!(
                unsafe { hc_fat_decode(46 << 9 | 2 << 5 | 30, 0, &mut fixed, &mut seconds) },
                HC_ERROR_INVALID_DATE
            );
            assert_eq!(
                unsafe { hc_fat_encode(day, 86_400, &mut date, &mut time) },
                HC_ERROR_OUT_OF_RANGE
            );
        }

        /// @248 is 04:57:07.2 UTC; SOFA's JD 2457073.05631 TT is
        /// J2015.1349933196, and J2000.0 946 728 000 TT seconds.
        #[test]
        fn a_beat_and_the_epochs_cross_the_boundary() {
            let mut beat = 0u16;
            assert_eq!(
                unsafe {
                    hc_swatch_beat((4 * 60 + 57) * 60 + 7, 200_000_000_000_000_000, &mut beat)
                },
                HC_OK
            );
            assert_eq!(beat, 248);
            let mut year = 0.0f64;
            assert_eq!(
                unsafe {
                    hc_epoch_from_tt(
                        c"J".as_ptr(),
                        1_424_352_065,
                        184_000_000_000_000_000,
                        &mut year,
                    )
                },
                HC_OK
            );
            assert!((year - 2_015.134_993_319_6).abs() < 1e-10, "{year}");
            let (mut seconds, mut attos) = (0i64, 7u64);
            assert_eq!(
                unsafe { hc_tt_from_epoch(core::ptr::null(), 2000.0, &mut seconds, &mut attos) },
                HC_OK
            );
            assert_eq!((seconds, attos), (946_728_000, 0));
            assert_eq!(
                unsafe { hc_epoch_from_tt(core::ptr::null(), 0, 0, &mut year) },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe { hc_tt_from_epoch(c"J".as_ptr(), 1e300, &mut seconds, &mut attos) },
                HC_ERROR_OVERFLOW
            );
        }

        #[test]
        fn a_tai64_label_crosses_both_ways() {
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_tai64_encode(0, 0, c"TAI64".as_ptr(), buffer, capacity, written)
            });
            assert_eq!(text, "4000000000000000\n");
            let (mut seconds, mut attos) = (7i64, 7u64);
            assert_eq!(
                unsafe {
                    hc_tai64_decode(
                        c"3fffffffffffffff3b9ac9ff3b9ac9ff".as_ptr(),
                        &mut seconds,
                        &mut attos,
                    )
                },
                HC_OK
            );
            assert_eq!((seconds, attos), (-1, 999_999_999_999_999_999));
            // The last label below 2⁶³ is the last second the int64_t holds.
            assert_eq!(
                unsafe { hc_tai64_decode(c"7fffffffffffffff".as_ptr(), &mut seconds, &mut attos) },
                HC_OK
            );
            assert_eq!(seconds, 4_611_686_018_427_387_903);
            assert_eq!(
                unsafe { hc_tai64_decode(c"8000000000000000".as_ptr(), &mut seconds, &mut attos) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_tai64_decode(c"xyz".as_ptr(), &mut seconds, &mut attos) },
                HC_ERROR_MALFORMED
            );
            assert_eq!(
                unsafe { hc_tai64_decode(core::ptr::null(), &mut seconds, &mut attos) },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe {
                    hc_tai64_encode(
                        0,
                        0,
                        c"tai32".as_ptr(),
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_UNKNOWN
            );
        }

        /// GPS week 2048 began at 2019-04-06 23:59:42 UTC, TAI second
        /// 1 554 595 219.
        #[test]
        fn the_april_2019_rollover_crosses_the_boundary() {
            let tai = 1_554_595_219;
            let (mut week, mut broadcast, mut tow, mut tow_attos) = (0u32, 0u32, 0u32, 0u64);
            let id = c"gps-lnav-week".as_ptr();
            assert_eq!(
                unsafe {
                    hc_gnss_week(
                        id,
                        tai - 1,
                        5,
                        &mut week,
                        &mut broadcast,
                        &mut tow,
                        &mut tow_attos,
                    )
                },
                HC_OK
            );
            assert_eq!((week, broadcast, tow, tow_attos), (2047, 1023, 604_799, 5));
            let (mut seconds, mut attos) = (0i64, 0u64);
            assert_eq!(
                unsafe { hc_gnss_to_tai(id, 2048, 0, 0, &mut seconds, &mut attos) },
                HC_OK
            );
            assert_eq!((seconds, attos), (tai, 0));
            // The latest instant the table of ranges gives.
            assert_eq!(
                unsafe {
                    hc_gnss_to_tai(
                        c"beidou-week".as_ptr(),
                        u32::MAX,
                        604_799,
                        0,
                        &mut seconds,
                        &mut attos,
                    )
                },
                HC_OK
            );
            assert_eq!(seconds, 2_597_597_356_694_432);
            assert_eq!(
                unsafe { hc_gnss_resolve_week(id, 1023, c"nearest".as_ptr(), tai, &mut week) },
                HC_OK
            );
            assert_eq!(week, 2047);
            assert_eq!(
                unsafe { hc_gnss_resolve_week(id, 1023, core::ptr::null(), tai, &mut week) },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe {
                    hc_gnss_week(
                        id,
                        0,
                        0,
                        &mut week,
                        &mut broadcast,
                        &mut tow,
                        &mut tow_attos,
                    )
                },
                HC_ERROR_NO_DATA
            );
        }

        #[test]
        fn glonass_ole_and_excel_dates_cross_the_boundary() {
            let (mut interval, mut day) = (0u32, 0u32);
            assert_eq!(
                unsafe { hc_glonass_date(1_704_067_200 + 37, 0, 1, &mut interval, &mut day) },
                HC_OK
            );
            assert_eq!((interval, day), (8, 1));
            let mut new_year_1900 = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(1900, 1, 1, &mut new_year_1900) },
                HC_OK
            );
            let (mut fixed, mut seconds) = (0i64, 0f64);
            assert_eq!(
                unsafe { hc_fixed_from_ole_automation(-1.25, &mut fixed, &mut seconds) },
                HC_OK
            );
            assert_eq!((fixed, seconds), (new_year_1900 - 3, 21_600.0));
            let mut value = 0f64;
            assert_eq!(
                unsafe { hc_ole_automation_from_fixed(new_year_1900 - 3, 21_600.0, &mut value) },
                HC_OK
            );
            assert_eq!(value, -1.25);
            let (mut serial_day, mut phantom) = (7i64, 7);
            assert_eq!(
                unsafe { hc_excel_1900_day(60, &mut serial_day, &mut phantom) },
                HC_OK
            );
            assert_eq!((serial_day, phantom), (7, 1), "serial 60 writes no day");
            assert_eq!(
                unsafe { hc_excel_1900_day(1, &mut serial_day, &mut phantom) },
                HC_OK
            );
            assert_eq!((serial_day, phantom), (new_year_1900, 0));
            assert_eq!(
                unsafe { hc_excel_1900_day(2_958_466, &mut serial_day, &mut phantom) },
                HC_ERROR_OUT_OF_RANGE
            );
        }

        /// Every refusal of the time-scale entry points that their
        /// documentation names and the tests above do not reach.
        #[test]
        fn the_time_scale_entry_points_refuse_as_documented() {
            const TOO_MANY_ATTOSECONDS: u64 = 1_000_000_000_000_000_000;
            let series = c"58479\t27.6740\n58489\t27.6745\n";
            let tai = (58_479 - 40_587) * 86_400 + 37;
            assert_eq!(
                measured(|buffer, capacity, written| unsafe {
                    hc_tt_bipm(
                        series.as_ptr(),
                        tai,
                        TOO_MANY_ATTOSECONDS,
                        1,
                        buffer,
                        capacity,
                        written,
                    )
                }),
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                measured(|buffer, capacity, written| unsafe {
                    hc_tt_bipm(c"".as_ptr(), tai, 0, 1, buffer, capacity, written)
                }),
                HC_ERROR_NO_DATA
            );
            assert_eq!(
                measured(|buffer, capacity, written| unsafe {
                    hc_tt_bipm(c"58479 27.6".as_ptr(), tai, 0, 1, buffer, capacity, written)
                }),
                HC_ERROR_MALFORMED
            );
            assert_eq!(
                measured(|buffer, capacity, written| unsafe {
                    hc_tt_bipm(core::ptr::null(), tai, 0, 1, buffer, capacity, written)
                }),
                HC_ERROR_NULL_POINTER
            );

            let (mut seconds, mut attos) = (7i64, 7u64);
            for (label, expected) in [
                (c"xyz", HC_ERROR_MALFORMED),
                (c"8000000000000000", HC_ERROR_OUT_OF_RANGE),
                (c"400000000000000a3b9aca00", HC_ERROR_OUT_OF_RANGE),
            ] {
                assert_eq!(
                    unsafe {
                        hc_tai64_posix_plus_10_decode(label.as_ptr(), &mut seconds, &mut attos)
                    },
                    expected,
                    "{label:?}"
                );
            }

            let (mut era, mut offset, mut fraction, mut unix) = (7, 7u32, 7u64, 7i64);
            assert_eq!(
                unsafe {
                    hc_ntp_resolve(
                        1,
                        0,
                        i64::MAX,
                        &mut era,
                        &mut offset,
                        &mut fraction,
                        &mut unix,
                        &mut attos,
                    )
                },
                HC_ERROR_OVERFLOW
            );
            assert_eq!(
                unsafe {
                    hc_ntp_resolve(
                        1,
                        0,
                        0,
                        &mut era,
                        &mut offset,
                        core::ptr::null_mut(),
                        &mut unix,
                        &mut attos,
                    )
                },
                HC_ERROR_NULL_POINTER
            );

            let (mut fixed, mut seconds_of_day) = (0i64, 0u32);
            assert_eq!(
                unsafe {
                    hc_fat_decode(23_866, 49_021, core::ptr::null_mut(), &mut seconds_of_day)
                },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe { hc_fat_decode(23_866, 49_021, &mut fixed, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
            let (mut date, mut time) = (0u16, 0u16);
            assert_eq!(
                unsafe { hc_fat_encode(739_885, 0, core::ptr::null_mut(), &mut time) },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe { hc_fat_encode(739_885, 0, &mut date, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );

            let mut beat = 0u16;
            assert_eq!(
                unsafe { hc_swatch_beat(0, TOO_MANY_ATTOSECONDS, &mut beat) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_swatch_beat(0, 0, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );

            let mut year = 0.0f64;
            assert_eq!(
                unsafe { hc_epoch_from_tt(c"X".as_ptr(), 0, 0, &mut year) },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe { hc_epoch_from_tt(c"J".as_ptr(), 0, TOO_MANY_ATTOSECONDS, &mut year) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_tt_from_epoch(c"X".as_ptr(), 2000.0, &mut seconds, &mut attos) },
                HC_ERROR_UNKNOWN
            );
            for year in [f64::NAN, f64::INFINITY] {
                assert_eq!(
                    unsafe { hc_tt_from_epoch(c"J".as_ptr(), year, &mut seconds, &mut attos) },
                    HC_ERROR_OUT_OF_RANGE,
                    "{year}"
                );
            }
            assert_eq!(
                unsafe {
                    hc_tt_from_epoch(c"J".as_ptr(), 2000.0, core::ptr::null_mut(), &mut attos)
                },
                HC_ERROR_NULL_POINTER
            );
        }
    }

    #[cfg(feature = "calendars")]
    mod calendar_days {
        use super::super::*;
        use super::{measured, read_lines};

        /// 5782 is a sabbatical year; 23 September AD 4 is Sebaste of Kaisar.
        #[test]
        fn the_sabbatical_place_and_the_asian_day_are_the_modules() {
            let mut place = 0i64;
            assert_eq!(
                unsafe { hc_hebrew_sabbatical_cycle_year(5_782, &mut place) },
                HC_OK
            );
            assert_eq!(place, 7);
            assert_eq!(
                unsafe { hc_hebrew_sabbatical_cycle_year(10_000, &mut place) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_hebrew_sabbatical_cycle_year(5_782, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_asian_day(1_360, buffer, capacity, written)
            });
            assert_eq!(text, "4\t1\tKaisar\tunnumbered\t1\n");
        }

        /// Chaitra śukla 1 of Śaka 1947 on 30 March 2025 at Ujjain on the
        /// Siddhānta's sky, its sky at its sunrise, and a crescent line, each
        /// the module's.
        #[test]
        fn the_hindu_and_crescent_lines_are_the_modules() {
            let mut day = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2025, 3, 30, &mut day) },
                HC_OK
            );
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_hindu_lunar_date(
                    c"surya-siddhanta".as_ptr(),
                    day,
                    23.15,
                    75.768_333,
                    0.0,
                    buffer,
                    capacity,
                    written,
                )
            });
            assert!(line.starts_with("1947\t2082\t1\t0\t1\t0\t"), "{line:?}");
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_surya_siddhanta_sunrise(day, 23.15, 75.768_333, buffer, capacity, written)
            });
            let sunrise: i64 = line.trim_end().parse().expect("an instant");
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_surya_siddhanta_at(sunrise, buffer, capacity, written)
            });
            assert!(line.ends_with("\t1\t12\n"), "{line:?}");
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_crescent_visible(
                    c"saudi-rule".as_ptr(),
                    day,
                    21.4,
                    39.8,
                    0.0,
                    buffer,
                    capacity,
                    written,
                )
            });
            assert_eq!(
                line.trim_end_matches('\n').split('\t').count(),
                7,
                "{line:?}"
            );
            let mut written = 0usize;
            assert_eq!(
                unsafe {
                    hc_crescent_visible(
                        core::ptr::null(),
                        day,
                        0.0,
                        0.0,
                        0.0,
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe {
                    hc_hindu_lunar_date(
                        c"lahiri".as_ptr(),
                        0,
                        0.0,
                        0.0,
                        0.0,
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_OUT_OF_RANGE
            );
        }

        /// Every refusal of the Hindu, crescent, Sūrya Siddhānta and Asian
        /// entry points that their documentation names, at the ends of the
        /// ranges the README gives.
        #[test]
        fn the_calendar_day_entry_points_refuse_as_documented() {
            let (ujjain_lat, ujjain_lon) = (23.15, 75.768_333);
            let sunrise = |fixed: i64, latitude: f64| {
                measured(|buffer, capacity, written| unsafe {
                    hc_surya_siddhanta_sunrise(
                        fixed, latitude, ujjain_lon, buffer, capacity, written,
                    )
                })
            };
            assert_eq!(sunrise(-1_132_604, ujjain_lat), HC_ERROR_BUFFER_TOO_SMALL);
            assert_eq!(sunrise(2_519_974, ujjain_lat), HC_ERROR_BUFFER_TOO_SMALL);
            assert_eq!(sunrise(-1_132_605, ujjain_lat), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(sunrise(2_519_975, ujjain_lat), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(sunrise(739_340, 66.0), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(sunrise(739_340, 91.0), HC_ERROR_OUT_OF_RANGE);

            let at = |unix_seconds: i64| {
                measured(|buffer, capacity, written| unsafe {
                    hc_surya_siddhanta_at(unix_seconds, buffer, capacity, written)
                })
            };
            assert_eq!(at(-159_992_668_800), HC_ERROR_BUFFER_TOO_SMALL);
            assert_eq!(at(155_590_156_799), HC_ERROR_BUFFER_TOO_SMALL);
            assert_eq!(at(-159_992_668_801), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(at(155_590_156_800), HC_ERROR_OUT_OF_RANGE);

            // The true sky at the Central Station with Lahiri's ayanamsa.
            let hindu = |sky: *const c_char, fixed: i64, latitude: f64| {
                measured(|buffer, capacity, written| unsafe {
                    hc_hindu_lunar_date(sky, fixed, latitude, 82.5, 0.0, buffer, capacity, written)
                })
            };
            let lahiri = c"lahiri".as_ptr();
            assert_eq!(
                hindu(lahiri, 620_627, 23.183_333),
                HC_ERROR_BUFFER_TOO_SMALL
            );
            assert_eq!(
                hindu(lahiri, 839_773, 23.183_333),
                HC_ERROR_BUFFER_TOO_SMALL
            );
            assert_eq!(hindu(lahiri, 620_626, 23.183_333), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(hindu(lahiri, 839_774, 23.183_333), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(
                hindu(c"surya-siddhanta".as_ptr(), 739_340, 66.0),
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                hindu(c"sidereal".as_ptr(), 739_340, 23.183_333),
                HC_ERROR_UNKNOWN
            );
            assert_eq!(hindu(c"".as_ptr(), 739_340, 23.183_333), HC_ERROR_UNKNOWN);
            assert_eq!(
                hindu(core::ptr::null(), 739_340, 23.183_333),
                HC_ERROR_NULL_POINTER
            );

            let crescent = |criterion: *const c_char, fixed: i64, latitude: f64| {
                measured(|buffer, capacity, written| unsafe {
                    hc_crescent_visible(
                        criterion, fixed, latitude, 39.8, 0.0, buffer, capacity, written,
                    )
                })
            };
            let shaukat = c"shaukat".as_ptr();
            assert_eq!(crescent(shaukat, -365_607, 21.4), HC_ERROR_BUFFER_TOO_SMALL);
            assert_eq!(
                crescent(shaukat, 1_095_727, 21.4),
                HC_ERROR_BUFFER_TOO_SMALL
            );
            assert_eq!(crescent(shaukat, -365_608, 21.4), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(crescent(shaukat, 1_095_728, 21.4), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(crescent(shaukat, 739_340, 91.0), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(
                crescent(c"danjon".as_ptr(), 739_340, 21.4),
                HC_ERROR_UNKNOWN
            );

            let asian = |fixed: i64| {
                measured(|buffer, capacity, written| unsafe {
                    hc_asian_day(fixed, buffer, capacity, written)
                })
            };
            assert_eq!(asian(3_652_398), HC_ERROR_BUFFER_TOO_SMALL);
            assert_eq!(asian(1_359), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(asian(3_652_399), HC_ERROR_OUT_OF_RANGE);
        }

        fn fixed(year: i64, month: u8, day: u8) -> i64 {
            let mut fixed = 0;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(year, month, day, &mut fixed) },
                HC_OK
            );
            fixed
        }

        /// Drik Panchang for 1 January 2025, read at sunrise.
        #[test]
        fn the_panchanga_lines_are_the_modules() {
            let day = fixed(2025, 1, 1);
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_panchanga_of_day(
                    day,
                    23.183_333,
                    82.5,
                    0.0,
                    c"lahiri".as_ptr(),
                    buffer,
                    capacity,
                    written,
                )
            });
            let rows: Vec<Vec<&str>> = text
                .lines()
                .map(|line| line.split('\t').collect())
                .collect();
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0][2], "Vyaghata");
            assert_eq!(rows[1][2], "Balava");
            let mut written = 0usize;
            assert_eq!(
                unsafe {
                    hc_panchanga_at(0, core::ptr::null(), core::ptr::null_mut(), 0, &mut written)
                },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe {
                    hc_panchanga_of_day(
                        day,
                        89.0,
                        0.0,
                        0.0,
                        c"Lahiri".as_ptr(),
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_NO_DATA
            );
        }

        #[test]
        fn the_olympiads_and_the_hebrew_anniversaries_are_out_parameters() {
            let mut out = 0i64;
            assert_eq!(unsafe { hc_ioc_olympiad(2021, &mut out) }, HC_OK);
            assert_eq!(out, 32);
            assert_eq!(
                unsafe { hc_ioc_olympiad(1895, &mut out) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_ioc_olympiad(2021, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe { hc_hebrew_yahrzeit(fixed(2020, 1, 7), 5781, &mut out) },
                HC_OK
            );
            assert_eq!(out, fixed(2020, 12, 25));
            assert_eq!(
                unsafe { hc_hebrew_birthday(fixed(2020, 1, 7), 5781, &mut out) },
                HC_OK
            );
            assert_eq!(out, fixed(2020, 12, 25));
            assert_eq!(
                unsafe { hc_hebrew_yahrzeit(fixed(2020, 1, 7), 10_000, &mut out) },
                HC_ERROR_OUT_OF_RANGE
            );
        }

        #[test]
        fn the_chinese_reckonings_cross_the_boundary() {
            let mut age = 0u32;
            let birth = fixed(2000, 6, 15);
            assert_eq!(
                unsafe { hc_chinese_reckoned_age(birth, fixed(2012, 1, 23), &mut age) },
                HC_OK
            );
            assert_eq!(age, 13);
            assert_eq!(
                unsafe { hc_chinese_reckoned_age(birth, birth - 1, &mut age) },
                HC_ERROR_NO_DATA
            );
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_chinese_marriage_augury(4_646, buffer, capacity, written)
            });
            assert_eq!(line, "double-bright\t1\t1\n");
        }
    }

    #[cfg(feature = "holiday")]
    mod observances {
        use super::super::*;
        use super::read_lines;

        /// The jubilee of 2025 and St George's Day of 2025, a Festival kept
        /// on Monday 28 April, as the module writes them.
        #[test]
        fn the_holy_year_and_common_worship_lines_are_the_modules() {
            let mut day = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2025, 4, 28, &mut day) },
                HC_OK
            );
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_holy_year_on(day, buffer, capacity, written)
            });
            assert_eq!(Ok(text), hc::holiday_lines::holy_year_line(day));
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_common_worship_on(day, buffer, capacity, written)
            });
            assert_eq!(
                text,
                "George, Martyr, Patron of England\tfestival\tFestival\n"
            );
            let mut written = 0usize;
            assert_eq!(
                unsafe { hc_holy_year_on(0, core::ptr::null_mut(), 0, &mut written) },
                HC_ERROR_NO_DATA
            );
            assert_eq!(
                unsafe {
                    hc_common_worship_on(-3_652_425_000, core::ptr::null_mut(), 0, &mut written)
                },
                HC_ERROR_OUT_OF_RANGE
            );
            // A day that keeps nothing writes the empty string.
            let mut buffer = [7 as c_char; 1];
            assert_eq!(
                unsafe { hc_common_worship_on(day - 5, buffer.as_mut_ptr(), 1, &mut written) },
                HC_OK
            );
            assert_eq!((buffer[0], written), (0, 1));
        }

        #[test]
        fn the_tables_the_lectionary_and_easter_cross_the_boundary() {
            let english = read_lines(|buffer, capacity, written| unsafe {
                hc_holiday_tables(c"en-US".as_ptr(), buffer, capacity, written)
            });
            let none = read_lines(|buffer, capacity, written| unsafe {
                hc_holiday_tables(core::ptr::null(), buffer, capacity, written)
            });
            assert_eq!(english.lines().count(), none.lines().count());
            let japan: Vec<&str> = english
                .lines()
                .find(|line| line.starts_with("JP\t"))
                .expect("Japan")
                .split('\t')
                .collect();
            assert_eq!(japan[..5], ["JP", "country", "Japan", "Japan", "en"]);
            assert_eq!(japan.len(), 8);
            assert_eq!(japan[7], "", "CLDR has no short name for Japan");
            let japanese = read_lines(|buffer, capacity, written| unsafe {
                hc_holiday_tables(c"ja".as_ptr(), buffer, capacity, written)
            });
            let hong_kong: Vec<&str> = japanese
                .lines()
                .find(|line| line.starts_with("HK\t"))
                .expect("Hong Kong")
                .split('\t')
                .collect();
            assert_eq!(hong_kong[2], "中華人民共和国香港特別行政区");
            assert_eq!(hong_kong[7], "香港");
            let mut day = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2025, 11, 30, &mut day) },
                HC_OK
            );
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_lectionary(day, buffer, capacity, written)
            });
            assert_eq!(line, "2026\tA\tII\t\n");
            let mut easter = 0i64;
            assert_eq!(unsafe { hc_astronomical_easter(2001, &mut easter) }, HC_OK);
            let mut expected = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2001, 4, 15, &mut expected) },
                HC_OK
            );
            assert_eq!(easter, expected);
            assert_eq!(
                unsafe { hc_astronomical_easter(1582, &mut easter) },
                HC_ERROR_OUT_OF_RANGE
            );
        }

        /// The Aleppo statement's table: the vernal full moon of 2001 on
        /// Sunday 8 April, a week before its Easter.
        #[test]
        fn the_astronomical_paschal_full_moon_crosses_through_an_out_parameter() {
            let (mut full_moon, mut expected, mut easter) = (0i64, 0i64, 0i64);
            assert_eq!(
                unsafe { hc_astronomical_paschal_full_moon(2001, &mut full_moon) },
                HC_OK
            );
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2001, 4, 8, &mut expected) },
                HC_OK
            );
            assert_eq!(full_moon, expected);
            assert_eq!(unsafe { hc_astronomical_easter(2001, &mut easter) }, HC_OK);
            assert_eq!(easter - full_moon, 7);
            assert_eq!(
                unsafe { hc_astronomical_paschal_full_moon(2151, &mut full_moon) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_astronomical_paschal_full_moon(2001, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
        }
    }

    #[cfg(feature = "sky")]
    mod earth_and_sun {
        use super::super::*;
        use super::{measured, read_lines};

        /// Warren's row for 29 February 1992 in the IDL Astronomy Library's
        /// `helio_jd`, as the module writes both scales.
        #[test]
        fn the_hjd_lines_are_the_modules() {
            let mut day = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(1992, 2, 29, &mut day) },
                HC_OK
            );
            let date = day as f64 + 1_721_424.5 + 11_756.2 / 86_400.0;
            let (alpha, delta) = (194.114_166_7, 42.171_388_9);
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_hjd_tt(date, alpha, delta, buffer, capacity, written)
            });
            assert_eq!(Ok(text), hc::astro_lines::hjd_tt_line(date, alpha, delta));
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_hjd_utc(date, alpha, delta, 1, buffer, capacity, written)
            });
            assert_eq!(
                Ok(text),
                hc::astro_lines::hjd_utc_line(date, alpha, delta, true)
            );
            let mut written = 0usize;
            assert_eq!(
                unsafe { hc_hjd_tt(date, 400.0, delta, core::ptr::null_mut(), 0, &mut written) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe {
                    hc_hjd_tt(
                        f64::NAN,
                        alpha,
                        delta,
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_OUT_OF_RANGE
            );
            // 1858, before the leap-second table: refused only under `strict`.
            let before_1961 = 2_400_000.5;
            assert_eq!(
                measured(|buffer, capacity, written| unsafe {
                    hc_hjd_utc(before_1961, alpha, delta, 1, buffer, capacity, written)
                }),
                HC_ERROR_NO_DATA
            );
            assert_eq!(
                measured(|buffer, capacity, written| unsafe {
                    hc_hjd_utc(before_1961, alpha, delta, 0, buffer, capacity, written)
                }),
                HC_ERROR_BUFFER_TOO_SMALL
            );
        }

        /// The USNO's sunrise for Jerusalem on 2024-01-01, 06:39 at UT+2
        /// (`usno-api-rstt`), under its own horizon, and the module's lines.
        #[test]
        fn the_horizons_are_the_modules_and_the_usnos_rises_on_its_minute() {
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_horizons(buffer, capacity, written)
            });
            assert_eq!(text, hc::astro_lines::horizons_lines());
            let mut day = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2024, 1, 1, &mut day) },
                HC_OK
            );
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_sunrise(
                    c"usno".as_ptr(),
                    day,
                    31.78,
                    35.24,
                    740.0,
                    buffer,
                    capacity,
                    written,
                )
            });
            let instant: i64 = line
                .split('\t')
                .next()
                .expect("a cell")
                .parse()
                .expect("an instant");
            assert!((instant - 1_704_083_940).abs() <= 31, "{line:?}");
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_sunset(
                    c"usno".as_ptr(),
                    day,
                    31.78,
                    35.24,
                    740.0,
                    buffer,
                    capacity,
                    written,
                )
            });
            let instant: i64 = line
                .split('\t')
                .next()
                .expect("a cell")
                .parse()
                .expect("an instant");
            assert!((instant - 1_704_120_360).abs() <= 31, "{line:?}");
            let mut written = 0usize;
            assert_eq!(
                unsafe {
                    hc_sunrise(
                        core::ptr::null(),
                        day,
                        0.0,
                        0.0,
                        0.0,
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe {
                    hc_sunset(
                        c"naoj".as_ptr(),
                        day,
                        0.0,
                        0.0,
                        0.0,
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_UNKNOWN
            );
        }

        #[test]
        fn the_rotation_is_erfas_through_an_out_parameter() {
            let mut degrees = 0f64;
            assert_eq!(
                unsafe { hc_earth_rotation_angle(1_192_406_400.0, &mut degrees) },
                HC_OK
            );
            assert!((degrees - 0.402_283_724_002_815_8_f64.to_degrees()).abs() < 1e-8);
            assert_eq!(
                unsafe { hc_gmst_iau1982(1_136_073_600.0, &mut degrees) },
                HC_OK
            );
            assert!((degrees - 1.754_174_981_860_675_f64.to_degrees()).abs() < 1e-8);
            assert_eq!(
                unsafe { hc_gmst_iau2006(1_136_073_600.0, &mut degrees) },
                HC_OK
            );
            assert!((degrees - 1.754_174_971_870_091_2_f64.to_degrees()).abs() < 1e-7);
            let mut seconds = 0f64;
            assert_eq!(
                unsafe { hc_ut2_minus_ut1(946_684_800.0 + 2_592.0, &mut seconds) },
                HC_OK
            );
            assert!((seconds + 0.005).abs() < 1e-6);
            assert_eq!(
                unsafe { hc_ut2_minus_ut1(f64::NAN, &mut seconds) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe { hc_gmst_iau2006(0.0, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
        }

        #[test]
        fn the_solar_lines_are_the_modules() {
            // Tromsø in the polar night: no noon shadow for ʿaṣr.
            let mut midwinter = 0i64;
            assert_eq!(
                unsafe { hc_gregorian_to_fixed(2024, 12, 21, &mut midwinter) },
                HC_OK
            );
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_solar_event(
                    c"asr-hanafi".as_ptr(),
                    midwinter,
                    69.6496,
                    18.956,
                    0.0,
                    buffer,
                    capacity,
                    written,
                )
            });
            assert_eq!(line, format!("\tno-noon-shadow\t{midwinter}\t\n"));
            let line = read_lines(|buffer, capacity, written| unsafe {
                hc_solar_time(
                    c"local-mean".as_ptr(),
                    1_704_110_400,
                    0.0,
                    15.0,
                    0.0,
                    buffer,
                    capacity,
                    written,
                )
            });
            let cells: Vec<&str> = line.trim_end().split('\t').collect();
            let hours: f64 = cells[1].parse().expect("hours");
            assert!((hours - 13.0).abs() < 1e-6, "{line:?}");
            let mut written = 0usize;
            assert_eq!(
                unsafe {
                    hc_solar_time(
                        core::ptr::null(),
                        0,
                        0.0,
                        0.0,
                        0.0,
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_NULL_POINTER
            );
        }
    }

    #[cfg(feature = "planetary")]
    mod planetary {
        use super::super::*;
        use super::{measured, read_lines};

        fn row(text: &str) -> Vec<String> {
            text.trim_end().split('\t').map(str::to_owned).collect()
        }

        /// Gangale's Titan calibration: 2002-12-18 10:42 UTC was
        /// 209 Aries 13, Julian Circad 144 096.
        #[test]
        fn a_circad_date_crosses_the_boundary() {
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_circad_date(
                    c"darian-titan".as_ptr(),
                    1_040_208_120.0,
                    buffer,
                    capacity,
                    written,
                )
            });
            let cells = row(&text);
            assert_eq!(cells.len(), 10);
            assert_eq!(
                cells[..7],
                ["darian-titan", "209", "9", "13", "Aries", "Solis", "144096"]
            );
            let mut written = 0usize;
            assert_eq!(
                unsafe {
                    hc_circad_date(
                        core::ptr::null(),
                        0.0,
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe {
                    hc_circad_date(
                        c"darian".as_ptr(),
                        0.0,
                        core::ptr::null_mut(),
                        0,
                        &mut written,
                    )
                },
                HC_ERROR_UNKNOWN
            );
            // 2103, past 100 Julian years from J2000.0, and an instant not finite.
            for unix_seconds in [4_200_000_000.0, f64::NAN] {
                assert_eq!(
                    measured(|buffer, capacity, written| unsafe {
                        hc_circad_date(
                            c"darian-titan".as_ptr(),
                            unix_seconds,
                            buffer,
                            capacity,
                            written,
                        )
                    }),
                    HC_ERROR_OUT_OF_RANGE,
                    "{unix_seconds}"
                );
            }
        }

        #[test]
        fn mars_time_and_a_mission_sol_cross_the_boundary() {
            // Mars24's worked example A: 2000-01-06T00:00:00Z.
            let text = read_lines(|buffer, capacity, written| unsafe {
                hc_mars_time(947_116_800.0, 0.0, buffer, capacity, written)
            });
            let cells = row(&text);
            assert_eq!(cells.len(), 16, "{cells:?}");
            assert_eq!(cells[1], "23:59:39");
            assert_eq!(cells[5], "23:38:54");
            let mut sol = -1i64;
            assert_eq!(
                unsafe { hc_mission_sol(c"Curiosity".as_ptr(), 1_344_230_277.0, &mut sol) },
                HC_OK
            );
            assert_eq!(sol, 0);
            assert_eq!(
                unsafe { hc_mission_sol(c"zhurong".as_ptr(), 1_700_000_000.0, &mut sol) },
                HC_ERROR_NO_DATA
            );
            assert_eq!(
                unsafe { hc_mission_sol(c"beagle-2".as_ptr(), 1_700_000_000.0, &mut sol) },
                HC_ERROR_UNKNOWN
            );
            assert_eq!(
                unsafe { hc_mission_sol(core::ptr::null(), 0.0, &mut sol) },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe { hc_mission_sol(c"curiosity".as_ptr(), 0.0, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
            assert_eq!(
                unsafe {
                    hc_mars_time(
                        f64::NAN,
                        0.0,
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_OUT_OF_RANGE
            );
        }

        #[test]
        fn the_tables_and_a_body_clock_are_the_modules_lines() {
            let missions = read_lines(|buffer, capacity, written| unsafe {
                hc_missions(buffer, capacity, written)
            });
            assert_eq!(missions, hc::planetary_lines::missions_lines());
            let bodies = read_lines(|buffer, capacity, written| unsafe {
                hc_bodies(buffer, capacity, written)
            });
            assert_eq!(bodies, hc::planetary_lines::bodies_lines());
            let titan = read_lines(|buffer, capacity, written| unsafe {
                hc_body_time(
                    c"Titan".as_ptr(),
                    947_116_800.0,
                    0.0,
                    buffer,
                    capacity,
                    written,
                )
            });
            assert_eq!(row(&titan).len(), 8, "{titan:?}");
            assert_eq!(
                unsafe {
                    hc_body_time(
                        c"sun".as_ptr(),
                        0.0,
                        0.0,
                        core::ptr::null_mut(),
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_NO_DATA
            );
            let body = |name: *const c_char, unix_seconds: f64, east_longitude: f64| {
                measured(|buffer, capacity, written| unsafe {
                    hc_body_time(
                        name,
                        unix_seconds,
                        east_longitude,
                        buffer,
                        capacity,
                        written,
                    )
                })
            };
            let titan = c"titan".as_ptr();
            assert_eq!(body(c"vulcan".as_ptr(), 0.0, 0.0), HC_ERROR_UNKNOWN);
            assert_eq!(body(core::ptr::null(), 0.0, 0.0), HC_ERROR_NULL_POINTER);
            assert_eq!(body(titan, f64::NAN, 0.0), HC_ERROR_OUT_OF_RANGE);
            assert_eq!(body(titan, 0.0, f64::INFINITY), HC_ERROR_OUT_OF_RANGE);
            // 2103, past 100 Julian years from J2000.0.
            assert_eq!(body(titan, 4_200_000_000.0, 0.0), HC_ERROR_OUT_OF_RANGE);
        }
    }

    #[cfg(feature = "relativity")]
    mod relativity {
        use super::super::*;
        use super::{measured, read_lines};

        fn row(text: &str) -> Vec<String> {
            text.trim_end().split('\t').map(str::to_owned).collect()
        }

        #[test]
        fn dilation_crosses_the_boundary() {
            let moving = read_lines(|buffer, capacity, written| unsafe {
                hc_proper_time(0.6 * 299_792_458.0, 10.0, buffer, capacity, written)
            });
            let cells = row(&moving);
            assert_eq!(cells.len(), 7, "{cells:?}");
            assert_eq!(cells[5], "SPEED_OF_LIGHT");
            let still = read_lines(|buffer, capacity, written| unsafe {
                hc_gravitational_dilation(c"earth".as_ptr(), 6_378_137.0, buffer, capacity, written)
            });
            assert_eq!(row(&still)[2], "GM_EARTH");
            let listed = read_lines(|buffer, capacity, written| unsafe {
                hc_gravitating_bodies(buffer, capacity, written)
            });
            assert_eq!(listed, hc::relativity_lines::gravitating_bodies_lines());
            let null = core::ptr::null_mut();
            assert_eq!(
                unsafe { hc_proper_time(3e8, 1.0, null, 0, core::ptr::null_mut()) },
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                unsafe {
                    hc_gravitational_dilation(
                        c"vulcan".as_ptr(),
                        1e7,
                        null,
                        0,
                        core::ptr::null_mut(),
                    )
                },
                HC_ERROR_UNKNOWN
            );
            // The Earth's Schwarzschild radius is about 8.87 mm.
            assert_eq!(
                measured(|buffer, capacity, written| unsafe {
                    hc_gravitational_dilation(c"earth".as_ptr(), 0.001, buffer, capacity, written)
                }),
                HC_ERROR_OUT_OF_RANGE
            );
            assert_eq!(
                measured(|buffer, capacity, written| unsafe {
                    hc_gravitational_dilation(core::ptr::null(), 1e7, buffer, capacity, written)
                }),
                HC_ERROR_NULL_POINTER
            );
        }
    }
}
