//! Time scales and day counts, behind the `timestamps` feature: TAI64 labels
//! in both conventions, GNSS weeks, GLONASS dates, OLE Automation dates,
//! Excel 1900 serials, UUID timestamps, NTP eras, FAT date and time words,
//! Swatch Internet Time, Julian and Besselian epochs, and TT(BIPM) from a
//! series the caller supplies, from `hyper_calendar::time_lines`, shared
//! with the WebAssembly module. A TAI
//! instant is whole seconds from 1970-01-01 00:00:00 TAI and the
//! attoseconds into that second, from 0 to 10¹⁸ − 1; a POSIX instant and a
//! TT instant, from 1970-01-01 00:00:00 TT, cross the same way.
//!
//! .NET's ticks and the six-hour clocks, behind the `timestamps` feature.
//! The lines are `hyper_calendar::time_code_lines`', shared with the
//! WebAssembly module.

use core::ffi::{c_char, c_int};

use hc::hc_calendars_solar::spreadsheet::Excel1900Day;
use hc::time_lines;

use crate::marshal::{name, status, text};
use crate::{HC_ERROR_NULL_POINTER, HC_OK, HcStatus};

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
    match time_lines::tai64_decode(hex).and_then(|(_, instant)| time_lines::tai_parts(instant)) {
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

hc::exports!("timestamps", c_exports);
