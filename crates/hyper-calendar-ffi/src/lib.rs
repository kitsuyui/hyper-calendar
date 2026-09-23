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

#![allow(unsafe_code)]
#![warn(missing_docs)]

use core::ffi::{c_char, c_int};

use hc::civil::Date;
use hc::hc_calendar::{CalendarError, Weekday};
use hc::hc_core::unix::{self, LeapPolicy, UtcInstant};
use hc::hc_core::{Duration, Instant, Tai, TimeError, UnixTime};

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
    let day_start = unix_seconds.div_euclid(86_400) * 86_400;
    let next_day = day_start + 86_400;
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

/// The holiday tables, behind the `holiday` feature: every country,
/// exchange, tradition and international set of `hc-holiday`, looked up by
/// identifier and rendered as tab-separated lines.
#[cfg(feature = "holiday")]
mod holiday {
    use core::ffi::{CStr, c_char, c_int};

    use hc::hc_calendar::Rd;
    use hc::hc_holiday::engine::HolidayCalendar;
    use hc::hc_holiday::hc_calendars_solar::gregorian;
    use hc::hc_holiday::rule::{Confidence, Kind, RuleSet};
    use hc::hc_holiday::{countries, exchanges, international, traditions};

    use super::{
        HC_ERROR_NOT_UTF8, HC_ERROR_NULL_POINTER, HC_ERROR_OUT_OF_RANGE, HC_ERROR_UNKNOWN, HC_OK,
        HcStatus, write_text,
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

    /// A NUL-terminated string: `Ok(None)` for a null pointer and
    /// [`HC_ERROR_NOT_UTF8`] for bytes that are not UTF-8.
    ///
    /// # Safety
    ///
    /// `pointer` must be null or point to a NUL-terminated string.
    unsafe fn text<'a>(pointer: *const c_char) -> Result<Option<&'a str>, HcStatus> {
        if pointer.is_null() {
            return Ok(None);
        }
        // SAFETY: the caller guarantees a NUL-terminated string.
        unsafe { CStr::from_ptr(pointer) }
            .to_str()
            .map(Some)
            .map_err(|_| HC_ERROR_NOT_UTF8)
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
        let Ok(year) = gregorian::year_from_fixed(day) else {
            return HC_ERROR_OUT_OF_RANGE;
        };
        let answer = HolidayCalendar::for_year(table, region, year).is_holiday(day);
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
}

#[cfg(feature = "holiday")]
pub use holiday::{hc_holiday_codes, hc_holiday_is_day_off, hc_holidays_in_year};

#[cfg(test)]
mod tests {
    use super::*;

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
        let status =
            unsafe { hc_format_iso_date(739_880, buffer.as_mut_ptr(), buffer.len(), &mut written) };
        assert_eq!(status, HC_ERROR_BUFFER_TOO_SMALL);
        assert_eq!(written, "2026-09-21".len() + 1);
        assert!(buffer.iter().all(|byte| *byte == 0));
    }

    #[test]
    fn a_sufficient_buffer_receives_a_nul_terminated_iso_date() {
        let mut buffer = [0 as c_char; 32];
        let mut written = 0usize;
        let status =
            unsafe { hc_format_iso_date(739_880, buffer.as_mut_ptr(), buffer.len(), &mut written) };
        assert_eq!(status, HC_OK);
        assert_eq!(written, 11);
        let bytes: Vec<u8> = buffer[..10].iter().map(|byte| *byte as u8).collect();
        assert_eq!(core::str::from_utf8(&bytes).unwrap(), "2026-09-21");
        assert_eq!(buffer[10], 0);
    }

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

    #[cfg(feature = "holiday")]
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
            unsafe { hc_holiday_is_day_off(xnys.as_ptr(), core::ptr::null(), fixed, &mut answer) },
            HC_OK
        );
        assert_eq!(answer, 1);
        assert_eq!(
            unsafe { hc_holiday_is_day_off(us.as_ptr(), core::ptr::null(), fixed, &mut answer) },
            HC_OK
        );
        assert_eq!(answer, 0);
        assert_eq!(
            unsafe { hc_holiday_is_day_off(zz.as_ptr(), core::ptr::null(), fixed, &mut answer) },
            HC_ERROR_UNKNOWN
        );
        assert_eq!(
            unsafe {
                hc_holiday_is_day_off(jp.as_ptr(), core::ptr::null(), fixed, core::ptr::null_mut())
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
            unsafe { hc_holiday_is_day_off(jp.as_ptr(), not_utf8.as_ptr(), fixed, &mut answer) },
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
        assert!(text.contains("\nJP\n") && text.contains("\nXNYS\n") && text.contains("un-days\n"));
    }
}
