//! Time scales and day counts, behind the `timestamps` feature: TAI64 labels
//! in both conventions, GNSS weeks, GLONASS dates, OLE Automation dates,
//! Excel 1900 serials, UUID timestamps, NTP eras, FAT date and time words,
//! Swatch Internet Time, Julian and Besselian epochs, and TT(BIPM) from a
//! series the caller supplies. The lines are `hyper_calendar::time_lines`',
//! shared with the C library. A TAI instant
//! is whole seconds from 1970-01-01 00:00:00 TAI and the attoseconds into
//! that second, from 0 to 10¹⁸ − 1; a POSIX instant and a TT instant, from
//! 1970-01-01 00:00:00 TT, cross the same way.
//!
//! .NET's ticks and the six-hour clocks, behind the `timestamps` feature:
//! `DateTime.Ticks` to and from POSIX time, and the Ethiopian and Swahili
//! six-hour readings of the civil clock. The lines are
//! `hyper_calendar::time_code_lines`', shared with the C library.

use hc::time_lines;

use crate::marshal::{emit_answer, text};

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
pub unsafe extern "C" fn hc_excel_1900_day(serial: i64, buffer: *mut u8, capacity: usize) -> i64 {
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

hc::exports!("timestamps", w_exports);
