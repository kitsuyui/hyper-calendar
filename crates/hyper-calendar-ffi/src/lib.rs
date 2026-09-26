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
//! default), `timestamps`, `calendars`, `holiday`, `seasons`, `deep-time`, `tz`, `sky` and
//! `orbital`, with `full` for all of them. Which feature each needs is in
//! the README's table.

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
/// The supplied buffer was too small; the required length was written out.
pub const HC_ERROR_BUFFER_TOO_SMALL: HcStatus = -5;
/// The value lies outside the range where the requested model has data.
pub const HC_ERROR_NO_DATA: HcStatus = -6;
/// The requested calendar, table or identifier is not known.
pub const HC_ERROR_UNKNOWN: HcStatus = -7;
/// A string argument was not valid UTF-8.
pub const HC_ERROR_NOT_UTF8: HcStatus = -8;
/// Data was not in the format the call expects.
pub const HC_ERROR_MALFORMED: HcStatus = -9;

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
    use hc::hc_calendar::{CalendarError, Weekday};
    use hc::hc_core::unix::{self, LeapPolicy, UtcInstant};
    use hc::hc_core::{Duration, Instant, Tai, UnixTime};

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
    pub unsafe extern "C" fn hc_weekday_from_fixed(fixed: i64, out_weekday: *mut u8) -> HcStatus {
        if out_weekday.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        let weekday = Weekday::from_rd(hc::hc_calendar::Rd(fixed));
        // SAFETY: checked non-null immediately above.
        unsafe { *out_weekday = weekday.iso_number() };
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
        out_seconds: *mut i64,
        out_attos: *mut u64,
    ) -> HcStatus {
        if out_seconds.is_null() || out_attos.is_null() {
            return HC_ERROR_NULL_POINTER;
        }
        let policy = if strict != 0 {
            LeapPolicy::Strict
        } else {
            LeapPolicy::Extrapolate
        };
        match unix::tai_from_unix(UnixTime::from_seconds(unix_seconds), policy) {
            Ok(instant) => {
                let reading = instant.since_epoch();
                let seconds = match i64::try_from(reading.whole_seconds()) {
                    Ok(value) => value,
                    Err(_) => return HC_ERROR_OVERFLOW,
                };
                // SAFETY: both were checked non-null immediately above.
                unsafe {
                    *out_seconds = seconds;
                    *out_attos = reading.subsec_attos();
                }
                HC_OK
            }
            Err(error) => status_from_time(error),
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
        let policy = if strict != 0 {
            LeapPolicy::Strict
        } else {
            LeapPolicy::Extrapolate
        };
        let instant = Instant::<Tai>::from_epoch(Duration::from_secs(tai_seconds as i128));
        match unix::utc_from_tai(instant, policy) {
            Ok(UtcInstant {
                unix_seconds,
                leap_second,
                ..
            }) => {
                // SAFETY: both were checked non-null immediately above.
                unsafe {
                    *out_unix_seconds = unix_seconds;
                    *out_is_leap_second = c_int::from(leap_second);
                }
                HC_OK
            }
            Err(error) => status_from_time(error),
        }
    }
}

#[cfg(feature = "civil")]
pub use civil::{
    hc_day_has_leap_second, hc_format_iso_date, hc_gregorian_from_fixed, hc_gregorian_to_fixed,
    hc_tai_from_unix, hc_tai_minus_utc, hc_utc_from_tai, hc_weekday_from_fixed,
};

/// Time scales and day counts, behind the `timestamps` feature: TAI64 labels,
/// GNSS weeks, GLONASS dates, OLE Automation dates and Excel 1900 serials,
/// from `hyper_calendar::time_lines`, shared with the WebAssembly module.
/// A TAI instant is whole seconds from 1970-01-01 00:00:00 TAI and the
/// attoseconds into that second, from 0 to 10¹⁸ − 1.
#[cfg(feature = "timestamps")]
mod time_scales {
    use core::ffi::{c_char, c_int};

    use hc::hc_calendars_solar::spreadsheet::Excel1900Day;
    use hc::time_lines;

    use super::{HC_ERROR_NULL_POINTER, HC_OK, HcStatus, name, status, write_answer};

    /// A TAI instant as a TAI64, TAI64N or TAI64NA label in lower-case
    /// hexadecimal, as one NUL-terminated UTF-8 line in a caller-owned
    /// buffer.
    ///
    /// `format` is `tai64`, `tai64n` or `tai64na`, in any case; anything
    /// else is `HC_ERROR_UNKNOWN`, and null `HC_ERROR_NULL_POINTER`.
    /// Attoseconds from 10¹⁸, or a second outside the labels below 2⁶³, is
    /// `HC_ERROR_OUT_OF_RANGE`. Writes the required length, including the
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
}

#[cfg(feature = "timestamps")]
pub use time_scales::{
    hc_excel_1900_day, hc_fixed_from_ole_automation, hc_glonass_date, hc_gnss_resolve_week,
    hc_gnss_to_tai, hc_gnss_week, hc_ole_automation_from_fixed, hc_tai64_decode, hc_tai64_encode,
};

/// Every calendar, behind the `calendars` feature: the registry the facade
/// populates, described for one day, walked as eras, years, months and
/// days, and listed, in the vocabulary of a locale; the locales
/// themselves; and when each country adopted the Gregorian calendar. The
/// lines are `hyper_calendar::lines`', shared with the WebAssembly module.
#[cfg(feature = "calendars")]
mod calendars {
    use core::ffi::c_char;

    use hc::hc_calendar::Rd;
    use hc::hc_calendar::units::Unit;
    use hc::lines;

    use super::{HC_ERROR_NULL_POINTER, HC_ERROR_UNKNOWN, HC_OK, HcStatus, text, write_text};

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
    /// An empty range writes an empty string. `locale` is as for
    /// `hc_describe_day`, `native` included. Writes the required length,
    /// including the terminator, into `written`.
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
        let text = lines::calendar_units(calendar, unit, Rd(from_fixed), Rd(to_fixed), tag);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
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
}

#[cfg(feature = "calendars")]
pub use calendars::{
    hc_calendar_units, hc_calendars, hc_describe_day, hc_first_day_of_week, hc_gregorian_adoption,
    hc_locales,
};

/// Days in the calendars, behind the `calendars` feature: the pañcāṅga's
/// yoga and karaṇa, the modern Olympiad of a year, and the Hebrew
/// anniversaries, from `hyper_calendar`'s `panchanga_lines` and
/// `calendar_values`, shared with the WebAssembly module.
#[cfg(feature = "calendars")]
mod calendar_days {
    use core::ffi::c_char;

    use hc::{astro_lines, calendar_values, panchanga_lines};

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
}

#[cfg(feature = "calendars")]
pub use calendar_days::{
    hc_chinese_marriage_augury, hc_chinese_reckoned_age, hc_hebrew_birthday, hc_hebrew_yahrzeit,
    hc_ioc_olympiad, hc_panchanga_at, hc_panchanga_of_day,
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
/// holiday table described, the lectionary cycles of a day and the
/// astronomical Easter, from `hyper_calendar::holiday_lines`, shared with
/// the WebAssembly module.
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
    /// and the ISO 3166-1 country of a subdivision or an exchange where its
    /// table records one. The tables carry English names only, so a `locale`
    /// whose language is `en` has the English name in column 3, and every
    /// other, or null, has it empty. Writes the required length, including
    /// the terminator, into `written`.
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
}

#[cfg(feature = "holiday")]
pub use observances::{hc_astronomical_easter, hc_holiday_tables, hc_lectionary};

/// The almanac, behind the `seasons` feature: the 24 solar terms and the 72
/// pentads of `hc-seasons`, judged at a named meridian.
#[cfg(feature = "seasons")]
mod seasons {
    use core::ffi::c_char;

    use hc::hc_calendar::Rd;
    use hc::hc_seasons::hc_astro::solar::solar_longitude_after;
    use hc::hc_seasons::solar_terms::{TermOrder, namings, term_in_effect};
    use hc::hc_seasons::{Meridian, pentads};

    use super::{HC_ERROR_UNKNOWN, HcStatus, push_cell, text, write_text};

    /// The meridian a string names.
    ///
    /// A name — `universal`, `japan`, `china`, `korea`, `india` or
    /// `china-before-1929`, in any case, with the empty string meaning
    /// `universal` — or a longitude in decimal degrees east of Greenwich,
    /// read as local mean solar time.
    ///
    /// # Errors
    ///
    /// [`HC_ERROR_UNKNOWN`] for anything else.
    pub(super) fn meridian(name: &str) -> Result<Meridian, HcStatus> {
        let lowered = name.trim().to_ascii_lowercase();
        Ok(match lowered.as_str() {
            "" | "universal" => Meridian::UNIVERSAL,
            "japan" => Meridian::JAPAN,
            "china" => Meridian::CHINA,
            "korea" => Meridian::KOREA,
            "india" => Meridian::INDIA,
            "china-before-1929" => Meridian::CHINA_BEFORE_1929,
            degrees => degrees
                .parse::<f64>()
                .ok()
                .filter(|degrees| degrees.is_finite() && (-180.0..=180.0).contains(degrees))
                .map(Meridian::from_longitude_degrees)
                .ok_or(HC_ERROR_UNKNOWN)?,
        })
    }

    /// The solar term in effect on a day as one line; see
    /// [`hc_term_in_effect`] for the columns.
    pub(super) fn term_line(fixed: i64, meridian: Meridian) -> String {
        use core::fmt::Write;
        let event = term_in_effect(Rd(fixed), meridian);
        let next = solar_longitude_after(event.term.next().solar_longitude_degrees(), event.moment);
        let end = Rd(meridian.day_of(next).0 - 1);
        let mut out = String::new();
        let _ = write!(
            out,
            "{}\t{}\t{}\t{}\t{}\t",
            event.term.index(TermOrder::SpringEquinoxFirst),
            event.term.chinese_name(),
            event.term.japanese_name(),
            event.day.0,
            end.0
        );
        push_cell(&mut out, namings::TRADITIONAL_CHINESE.authority);
        out.push('\t');
        push_cell(&mut out, namings::JAPANESE.authority);
        out.push('\n');
        out
    }

    /// The pentad in effect on a day as one line; see
    /// [`hc_pentad_in_effect`] for the columns.
    pub(super) fn pentad_line(fixed: i64, meridian: Meridian) -> String {
        use core::fmt::Write;
        let event = pentads::pentad_in_effect(Rd(fixed), meridian);
        let next =
            solar_longitude_after(event.pentad.next().solar_longitude_degrees(), event.moment);
        let end = Rd(meridian.day_of(next).0 - 1);
        let mut out = String::new();
        let _ = write!(
            out,
            "{}\t{}\t{}\t{}\t{}\t",
            event.pentad.index(TermOrder::SpringEquinoxFirst),
            event.pentad.name(pentads::CHINESE),
            event.pentad.name(pentads::JAPANESE),
            event.day.0,
            end.0
        );
        push_cell(&mut out, pentads::CHINESE.authority);
        out.push('\t');
        push_cell(&mut out, pentads::JAPANESE.authority);
        out.push('\n');
        out
    }

    /// The meridian a NUL-terminated string argument names, null meaning
    /// `universal`.
    ///
    /// # Safety
    ///
    /// `name` must be null or point to a NUL-terminated string.
    unsafe fn meridian_argument(name: *const c_char) -> Result<Meridian, HcStatus> {
        // SAFETY: forwarded to the caller's contract above.
        let name = unsafe { text(name) }?.unwrap_or("");
        meridian(name)
    }

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
    /// text that is not UTF-8 `HC_ERROR_NOT_UTF8`. Writes the required
    /// length, including the terminator, into `written`.
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
        // SAFETY: forwarded to the caller's contract above.
        let meridian = match unsafe { meridian_argument(meridian) } {
            Ok(meridian) => meridian,
            Err(status) => return status,
        };
        let text = term_line(fixed, meridian);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }

    /// The pentad (候) in effect on a fixed day at a meridian, as one
    /// NUL-terminated UTF-8 line in a caller-owned buffer.
    ///
    /// Tab-separated: the pentad's index from the first pentad of 春分 at 0
    /// through 71 (the longitude divided by 5°), its name in the Chinese
    /// tradition, its name in the Japanese tradition, the fixed day the
    /// pentad began at that meridian, the last fixed day before the next
    /// pentad begins, the text the Chinese names come from and the text the
    /// Japanese names come from. `meridian` is as for `hc_term_in_effect`.
    /// Writes the required length, including the terminator, into `written`.
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
        // SAFETY: forwarded to the caller's contract above.
        let meridian = match unsafe { meridian_argument(meridian) } {
            Ok(meridian) => meridian,
            Err(status) => return status,
        };
        let text = pentad_line(fixed, meridian);
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }
}

#[cfg(feature = "seasons")]
pub use seasons::{hc_pentad_in_effect, hc_term_in_effect};

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

/// The sky, behind the `sky` feature: where the Sun and the Moon are at an
/// instant, and the solar terms and the moon phases within a span, from
/// `hc-astro`'s series, in Universal Time.
#[cfg(feature = "sky")]
mod sky {
    use core::ffi::c_char;

    use hc::hc_astro::lunar::{
        MEAN_SYNODIC_MONTH, MoonPhase, lunar_distance, lunar_illuminated_fraction, lunar_latitude,
        lunar_longitude, lunar_phase, new_moon_at_or_after, new_moon_before, nth_moon_phase,
        nth_new_moon,
    };
    use hc::hc_astro::solar::{solar_longitude, solar_longitude_after, solar_radius_vector};
    use hc::hc_astro::time::{DeltaTRegime, decimal_year, delta_t, delta_t_regime};
    use hc::hc_calendar::fixed::{Moment, RD_OF_UNIX_EPOCH};
    use hc::hc_calendar::gregorian::new_year;
    use hc::hc_core::math::floor;
    use hc::hc_seasons::solar_terms::{DEGREES_PER_TERM, SolarTerm, TermOrder};

    use super::{HC_ERROR_OUT_OF_RANGE, HcStatus, push_cell, write_text};

    /// The first proleptic Gregorian year the entry points answer for: the
    /// start of the era, roughly 1000 BCE to 3000 CE, over which
    /// `hc-astro`'s README states its series hold. Outside it the lunar
    /// series and ΔT are not stated valid, so the entry points refuse
    /// rather than extrapolate.
    pub(super) const EARLIEST_YEAR: i64 = -1000;

    /// The last proleptic Gregorian year the entry points answer for.
    pub(super) const LATEST_YEAR: i64 = 3000;

    /// The longest span the `_between` entry points accept, in seconds:
    /// 400 Julian years, about 4 950 lunations and 9 600 solar terms.
    pub(super) const MAX_SPAN_SECONDS: i64 = 400 * 31_557_600;

    /// The first fixed day of the era.
    const FIRST_DAY: i64 = new_year(EARLIEST_YEAR).0;

    /// The first fixed day after the era.
    const END_DAY: i64 = new_year(LATEST_YEAR + 1).0;

    /// Seconds in a day, as the astronomical series count them: no leap
    /// second, because ΔT carries the Earth's rotational irregularity.
    const SECONDS_PER_DAY: i64 = 86_400;

    /// The word a [`DeltaTRegime`] is written as.
    const fn regime_name(regime: DeltaTRegime) -> &'static str {
        match regime {
            DeltaTRegime::Observed => "observed",
            DeltaTRegime::Predicted => "predicted",
            DeltaTRegime::Fitted => "fitted",
            DeltaTRegime::Extrapolated => "extrapolated",
        }
    }

    /// The source ΔT was answered from in a regime, as `hc-astro`'s
    /// README names it.
    const fn delta_t_source(regime: DeltaTRegime) -> &'static str {
        match regime {
            DeltaTRegime::Observed => "USNO deltat.data, observed",
            DeltaTRegime::Predicted => "USNO deltat.preds, predicted",
            DeltaTRegime::Fitted => "Espenak-Meeus polynomials, fitted",
            DeltaTRegime::Extrapolated => "Espenak-Meeus parabola, extrapolated",
        }
    }

    /// The series behind the Sun's columns, as `hc-astro` names them.
    const SUN_SOURCE: &str =
        "Sun: VSOP87D Earth series truncated at 1e-7 (213 terms), Meeus ch. 25";

    /// The series behind the Moon's columns.
    const MOON_SOURCE: &str = "Moon: ELP-2000/82 abridged to Meeus tables 47.A and 47.B (60 terms), illumination Meeus ch. 48";

    /// The series behind the new moons and the phase list.
    const PHASES_SOURCE: &str = "phases: Meeus ch. 49 phase series";

    /// The word a [`MoonPhase`] is written as.
    const fn phase_name(phase: MoonPhase) -> &'static str {
        match phase {
            MoonPhase::New => "new",
            MoonPhase::FirstQuarter => "first-quarter",
            MoonPhase::Full => "full",
            MoonPhase::LastQuarter => "last-quarter",
        }
    }

    /// The Universal Time moment of a POSIX timestamp, if it lies in the
    /// era the entry points answer for.
    ///
    /// # Errors
    ///
    /// [`HC_ERROR_OUT_OF_RANGE`] outside [`EARLIEST_YEAR`]..=[`LATEST_YEAR`].
    pub(super) fn moment_in_era(unix: i64) -> Result<Moment, HcStatus> {
        let day = unix
            .div_euclid(SECONDS_PER_DAY)
            .checked_add(RD_OF_UNIX_EPOCH)
            .ok_or(HC_ERROR_OUT_OF_RANGE)?;
        if !(FIRST_DAY..END_DAY).contains(&day) {
            return Err(HC_ERROR_OUT_OF_RANGE);
        }
        let seconds = unix.rem_euclid(SECONDS_PER_DAY);
        Ok(Moment(day as f64 + seconds as f64 / SECONDS_PER_DAY as f64))
    }

    /// The POSIX second a Universal Time moment falls in.
    ///
    /// Whole seconds, rounded down: the series are not good to better than
    /// a few seconds, and ΔT past the predictions' end is off by nine, so
    /// a fraction would claim what is not known.
    fn unix_from_moment(moment: Moment) -> i64 {
        floor((moment.0 - RD_OF_UNIX_EPOCH as f64) * SECONDS_PER_DAY as f64) as i64
    }

    /// The Sun and the Moon at an instant as one line; see [`hc_sky_at`]
    /// for the columns.
    ///
    /// # Errors
    ///
    /// As [`moment_in_era`].
    pub(super) fn sky_line(unix: i64) -> Result<String, HcStatus> {
        use core::fmt::Write;
        let moment = moment_in_era(unix)?;
        let regime = delta_t_regime(decimal_year(moment));
        let mut out = String::new();
        let _ = write!(
            out,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t",
            solar_longitude(moment),
            solar_radius_vector(moment),
            lunar_longitude(moment),
            lunar_latitude(moment),
            lunar_distance(moment),
            lunar_phase(moment),
            lunar_illuminated_fraction(moment),
            unix_from_moment(new_moon_before(moment)),
            unix_from_moment(new_moon_at_or_after(moment)),
            delta_t(moment),
            regime_name(regime),
        );
        let source = format!(
            "{SUN_SOURCE}; {MOON_SOURCE}; {PHASES_SOURCE}; delta T: {}",
            delta_t_source(regime)
        );
        push_cell(&mut out, &source);
        out.push('\n');
        Ok(out)
    }

    /// The moment a half-open span `[from, to)` of POSIX seconds begins
    /// at, or `None` for an empty span.
    ///
    /// # Errors
    ///
    /// [`HC_ERROR_OUT_OF_RANGE`] for an end outside the era or a span
    /// longer than [`MAX_SPAN_SECONDS`].
    fn span(from: i64, to: i64) -> Result<Option<Moment>, HcStatus> {
        let start = moment_in_era(from)?;
        if to <= from {
            return Ok(None);
        }
        moment_in_era(to - 1)?;
        if to - from > MAX_SPAN_SECONDS {
            return Err(HC_ERROR_OUT_OF_RANGE);
        }
        Ok(Some(start))
    }

    /// Every solar term in `[from, to)`, one line each, in time order; see
    /// [`hc_solar_terms_between`] for the columns.
    ///
    /// # Errors
    ///
    /// As [`span`].
    pub(super) fn term_lines(from: i64, to: i64) -> Result<String, HcStatus> {
        use core::fmt::Write;
        let mut out = String::new();
        let Some(start) = span(from, to)? else {
            return Ok(out);
        };
        // The term in effect at the start; the first line is the one after
        // it. A longitude that rounds to 360° names 春分 again.
        let index = floor(solar_longitude(start) / DEGREES_PER_TERM) as u8;
        let mut term = SolarTerm::from_index(TermOrder::SpringEquinoxFirst, index)
            .unwrap_or(SolarTerm::SPRING_EQUINOX);
        let mut moment = start;
        // The span is bounded, so the loop is; the cap is against a search
        // that fails to advance, which would otherwise spin.
        for _ in 0..(MAX_SPAN_SECONDS / (13 * SECONDS_PER_DAY)) {
            term = term.next();
            moment = solar_longitude_after(term.solar_longitude_degrees(), moment);
            let unix = unix_from_moment(moment);
            if unix >= to {
                break;
            }
            if unix < from {
                continue;
            }
            let _ = writeln!(
                out,
                "{}\t{unix}\t{}\t{}",
                term.solar_longitude_degrees(),
                term.chinese_name(),
                term.japanese_name()
            );
        }
        Ok(out)
    }

    /// Every principal moon phase in `[from, to)`, one line each, in time
    /// order; see [`hc_moon_phases_between`] for the columns.
    ///
    /// # Errors
    ///
    /// As [`span`].
    pub(super) fn phase_lines(from: i64, to: i64) -> Result<String, HcStatus> {
        use core::fmt::Write;
        let mut out = String::new();
        let Some(start) = span(from, to)? else {
            return Ok(out);
        };
        // The lunation containing the start: seeded from the mean synodic
        // month, which is never more than a lunation out, then walked.
        let mut lunation = floor((start.0 - nth_new_moon(0).0) / MEAN_SYNODIC_MONTH) as i64;
        for _ in 0..8 {
            if nth_new_moon(lunation).0 <= start.0 {
                break;
            }
            lunation -= 1;
        }
        for _ in 0..8 {
            if nth_new_moon(lunation + 1).0 > start.0 {
                break;
            }
            lunation += 1;
        }
        'lunations: for _ in 0..(MAX_SPAN_SECONDS / (29 * SECONDS_PER_DAY)) {
            for phase in [
                MoonPhase::New,
                MoonPhase::FirstQuarter,
                MoonPhase::Full,
                MoonPhase::LastQuarter,
            ] {
                let unix = unix_from_moment(nth_moon_phase(lunation, phase));
                if unix >= to {
                    break 'lunations;
                }
                if unix < from {
                    continue;
                }
                let _ = writeln!(
                    out,
                    "{}\t{unix}\t{}\t",
                    phase.elongation_degrees(),
                    phase_name(phase)
                );
            }
            lunation += 1;
        }
        Ok(out)
    }

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
        let text = match sky_line(unix_seconds) {
            Ok(text) => text,
            Err(status) => return status,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
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
        let text = match term_lines(from_unix, to_unix) {
            Ok(text) => text,
            Err(status) => return status,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
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
        let text = match phase_lines(from_unix, to_unix) {
            Ok(text) => text,
            Err(status) => return status,
        };
        // SAFETY: forwarded to the caller's contract above.
        unsafe { write_text(&text, buffer, capacity, written) }
    }
}

#[cfg(feature = "sky")]
pub use sky::{hc_moon_phases_between, hc_sky_at, hc_solar_terms_between};

/// The Earth's rotation and the Sun's hours, behind the `sky` feature: the
/// Earth Rotation Angle, the Greenwich mean sidereal time by two
/// conventions, UT2 − UT1, and the clocks and named times of day of
/// `hc_astro::solar_time`, from `hyper_calendar::astro_lines`, shared with
/// the WebAssembly module, for the years −1000 to 3000.
#[cfg(feature = "sky")]
mod earth_and_sun {
    use core::ffi::c_char;

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
}

#[cfg(feature = "sky")]
pub use earth_and_sun::{
    hc_earth_rotation_angle, hc_gmst_iau1982, hc_gmst_iau2006, hc_solar_event, hc_solar_time,
    hc_ut2_minus_ut1,
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
        feature = "sky",
        feature = "orbital"
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

    #[cfg(feature = "civil")]
    mod civil {
        use super::super::*;

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
                unsafe { hc_weekday_from_fixed(0, core::ptr::null_mut()) },
                HC_ERROR_NULL_POINTER
            );
        }

        #[test]
        fn weekdays_use_iso_numbering() {
            let mut weekday = 0u8;
            // 1970-01-01 was a Thursday.
            assert_eq!(
                unsafe { hc_weekday_from_fixed(719_163, &mut weekday) },
                HC_OK
            );
            assert_eq!(weekday, 4);
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
            // 2^53 days, which used to saturate to INT64_MAX.
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
        use super::read_lines;

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
        use super::read_lines;

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
    }

    #[cfg(feature = "calendars")]
    mod calendar_days {
        use super::super::*;
        use super::read_lines;

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
    }

    #[cfg(feature = "sky")]
    mod earth_and_sun {
        use super::super::*;
        use super::read_lines;

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
}
