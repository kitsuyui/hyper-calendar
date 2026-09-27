//! The tab-separated lines the WebAssembly module and the C library write
//! about time scales and day counts, written once.
//!
//! Every function here takes what crosses the boundary — integers, a
//! double, a name — and answers with the values or the line both
//! boundaries write, so that parsing a name, checking a range and writing
//! a number happen in one place. A TAI instant crosses as whole seconds
//! from 1970-01-01 00:00:00 TAI, the origin of [`Instant<Tai>`], and the
//! attoseconds into that second, from 0 to 10¹⁸ − 1: seconds floored, the
//! remainder never negative, as [`Duration`] carries them.
//!
//! * TAI64, TAI64N and TAI64NA labels as lower-case hexadecimal, from
//!   [`hc_core::tai64`].
//! * The GNSS week numbers, their broadcast fields and the two rollover
//!   rules, and GLONASS's *N*4 and *N*T, from [`hc_core::gnss`].
//! * The OLE Automation date both ways and the Excel 1900 serial, which
//!   names serial 60 as the day that never was, from
//!   [`hc_calendars_solar::spreadsheet`].
//! * The TAI–UTC bridge as the C library's `hc_tai_from_unix` and
//!   `hc_utc_from_tai` answer it, from [`hc_core::unix`].
//! * TAI64 labels in the `tai64-posix-plus-10` convention, the
//!   2⁶² + 10 + POSIX seconds that daemontools' `tai64n` writes on an
//!   ordinary clock, from [`hc_core::tai64`]: a convention of its own, so
//!   functions of their own, not a flag on the true-TAI ones
//!   (`docs/policy.md` §5).
//! * The 60-bit timestamp of a version 1 or version 6 UUID both ways, from
//!   [`hc_core::uuid`]; an NTP timestamp placed in its era by a reference
//!   time, and the NTP date and timestamp of a POSIX instant, from
//!   [`hc_core::ntp`]; and the FAT date and time words both ways, from
//!   [`hc_format::fat`].
//! * Swatch Internet Time, from [`hc_core::internet_time`], and the Julian
//!   and Besselian epochs of a TT instant both ways, from
//!   [`hc_core::epoch_notation`].
//! * TT(BIPM) at a TAI instant, from a realisation the caller supplies as
//!   text, by [`hc_core::tt_bipm`]: the library carries none of the BIPM's
//!   tables, which are revised every year.
//!
//! A POSIX instant crosses as a TAI one does, as whole seconds and the
//! attoseconds into the second, and a TT instant as whole seconds from
//! 1970-01-01 00:00:00 TT, the origin of [`Instant<Tt>`], and its
//! attoseconds: TT runs 32.184 s ahead of TAI.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use hc_calendar::{CalendarError, Rd};
use hc_calendars_solar::spreadsheet::{self, Excel1900Day};
use hc_core::epoch_notation::EpochKind;
use hc_core::gnss::{self, GlonassDate, GlonassTime, WeekNumbering, WeekTime};
use hc_core::internet_time::Beat;
use hc_core::ntp::{NtpDate, NtpTimestamp};
use hc_core::tai64;
use hc_core::tt_bipm::TtBipmSeries;
use hc_core::unix::{self, LeapPolicy, UtcInstant};
use hc_core::uuid::{self, TimeVersion};
use hc_core::{Duration, Instant, Tai, Tt, UnixTime};

use crate::boundary::{Answer, Refusal, names};

/// The refusal a calendar error is: every one of them a value outside
/// what the day count answers for, but for arithmetic that overflowed.
const fn calendar_refusal(error: CalendarError) -> Refusal {
    match error {
        CalendarError::Overflow => Refusal::Overflow,
        _ => Refusal::OutOfRange,
    }
}

/// A TAI instant from whole seconds and attoseconds.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸.
pub fn tai_instant(seconds: i64, attoseconds: u64) -> Answer<Instant<Tai>> {
    Duration::new(i128::from(seconds), attoseconds)
        .map(Instant::from_epoch)
        .map_err(|_| Refusal::OutOfRange)
}

/// A TAI instant as whole seconds and attoseconds.
///
/// # Errors
///
/// [`Refusal::Overflow`] for seconds outside an `i64`.
pub fn tai_parts(instant: Instant<Tai>) -> Answer<(i64, u64)> {
    let reading = instant.since_epoch();
    let seconds = i64::try_from(reading.whole_seconds()).map_err(|_| Refusal::Overflow)?;
    Ok((seconds, reading.subsec_attos()))
}

/// Which of Bernstein's three external formats a label is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tai64Format {
    /// Eight bytes: the second.
    Tai64,
    /// Twelve bytes: the second and the nanosecond.
    Tai64N,
    /// Sixteen bytes: the second, the nanosecond and the attosecond.
    Tai64Na,
}

impl Tai64Format {
    /// The three formats.
    pub const ALL: [Self; 3] = [Self::Tai64, Self::Tai64N, Self::Tai64Na];

    /// The format's name as a line carries it: `tai64`, `tai64n` or
    /// `tai64na`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Tai64 => "tai64",
            Self::Tai64N => "tai64n",
            Self::Tai64Na => "tai64na",
        }
    }

    /// The length of the label in bytes.
    #[must_use]
    pub const fn bytes(self) -> usize {
        match self {
            Self::Tai64 => 8,
            Self::Tai64N => 12,
            Self::Tai64Na => 16,
        }
    }

    /// The format a name names, in any case.
    ///
    /// # Errors
    ///
    /// [`Refusal::Unknown`] for any other name.
    pub fn from_name(name: &str) -> Answer<Self> {
        Self::ALL
            .into_iter()
            .find(|format| names(name, format.name()))
            .ok_or(Refusal::Unknown)
    }
}

/// The label of a TAI instant in a format, as lower-case hexadecimal:
/// sixteen digits for TAI64, twenty-four for TAI64N, thirty-two for
/// TAI64NA.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a second that no TAI64 label can hold (the
/// labels run below 2⁶³).
pub fn tai64_hex(instant: Instant<Tai>, format: Tai64Format) -> Answer<String> {
    let mut bytes = [0u8; 16];
    match format {
        Tai64Format::Tai64 => bytes[..8].copy_from_slice(&tai64::encode_tai64(instant)?),
        Tai64Format::Tai64N => bytes[..12].copy_from_slice(&tai64::encode_tai64n(instant)?),
        Tai64Format::Tai64Na => bytes.copy_from_slice(&tai64::encode_tai64na(instant)?),
    }
    let mut out = String::new();
    for byte in &bytes[..format.bytes()] {
        let _ = write!(out, "{byte:02x}");
    }
    Ok(out)
}

/// The line of `hc_tai64_encode`: the label in lower-case hexadecimal.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a format that is not `tai64`, `tai64n` or
/// `tai64na`, and [`Refusal::OutOfRange`] for attoseconds from 10¹⁸ or a
/// second outside the labels.
pub fn tai64_encode_line(seconds: i64, attoseconds: u64, format: &str) -> Answer<String> {
    let format = Tai64Format::from_name(format)?;
    let mut out = tai64_hex(tai_instant(seconds, attoseconds)?, format)?;
    out.push('\n');
    Ok(out)
}

/// A label in hexadecimal read back: its format, by its length, and the
/// instant it names — the start of the second, the nanosecond or the
/// attosecond it labels.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not 16, 24 or 32 hexadecimal
/// digits, in either case, and [`Refusal::OutOfRange`] for a reserved
/// label, from 2⁶³, or a counter above 999 999 999.
pub fn tai64_decode(hex: &str) -> Answer<(Tai64Format, Instant<Tai>)> {
    let (format, bytes) = tai64_bytes(hex)?;
    let mut eight = [0u8; 8];
    let mut twelve = [0u8; 12];
    eight.copy_from_slice(&bytes[..8]);
    twelve.copy_from_slice(&bytes[..12]);
    let instant = match format {
        Tai64Format::Tai64 => tai64::decode_tai64(eight)?,
        Tai64Format::Tai64N => tai64::decode_tai64n(twelve)?,
        Tai64Format::Tai64Na => tai64::decode_tai64na(bytes)?,
    };
    Ok((format, instant))
}

/// A label's bytes, left-aligned in sixteen, and its format by its length.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not 16, 24 or 32 hexadecimal
/// digits, in either case, with white space around them allowed.
fn tai64_bytes(hex: &str) -> Answer<(Tai64Format, [u8; 16])> {
    let digits = hex.trim().as_bytes();
    let format = Tai64Format::ALL
        .into_iter()
        .find(|format| format.bytes() * 2 == digits.len())
        .ok_or(Refusal::Malformed)?;
    let mut bytes = [0u8; 16];
    hex_into(digits, &mut bytes)?;
    Ok((format, bytes))
}

/// Hexadecimal digits, two to a byte, into the front of `bytes`.
///
/// # Errors
///
/// [`Refusal::Malformed`] for a character that is not a hexadecimal digit.
fn hex_into(digits: &[u8], bytes: &mut [u8]) -> Answer<()> {
    let (pairs, _) = digits.as_chunks::<2>();
    for (byte, [high, low]) in bytes.iter_mut().zip(pairs) {
        let (Some(high), Some(low)) = (hex_digit(*high), hex_digit(*low)) else {
            return Err(Refusal::Malformed);
        };
        *byte = high << 4 | low;
    }
    Ok(())
}

const fn hex_digit(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => Some(digit - b'0'),
        b'a'..=b'f' => Some(digit - b'a' + 10),
        b'A'..=b'F' => Some(digit - b'A' + 10),
        _ => None,
    }
}

/// The line of `hc_tai64_decode`: the format, the TAI seconds and the
/// attoseconds.
///
/// # Errors
///
/// As [`tai64_decode`].
pub fn tai64_decode_line(hex: &str) -> Answer<String> {
    let (format, instant) = tai64_decode(hex)?;
    let (seconds, attoseconds) = tai_parts(instant)?;
    Ok(alloc::format!(
        "{}\t{seconds}\t{attoseconds}\n",
        format.name()
    ))
}

/// The week-number field an identifier names, in any case:
/// `gps-lnav-week`, `gps-cnav-week`, `galileo-week`, `beidou-week` or
/// `navic-week`, as [`gnss::ALL`] has them.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other identifier.
pub fn week_numbering(id: &str) -> Answer<WeekNumbering> {
    gnss::ALL
        .iter()
        .copied()
        .find(|numbering| names(id, numbering.id()))
        .ok_or(Refusal::Unknown)
}

/// The full week and the time of week of a TAI instant under a field.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an identifier [`week_numbering`] does not
/// know, [`Refusal::OutOfRange`] for attoseconds from 10¹⁸,
/// [`Refusal::NoData`] before the field's week zero, and
/// [`Refusal::Overflow`] past week 2³² − 1.
pub fn gnss_week(id: &str, seconds: i64, attoseconds: u64) -> Answer<(WeekNumbering, WeekTime)> {
    let numbering = week_numbering(id)?;
    let time = numbering.week_time(tai_instant(seconds, attoseconds)?)?;
    Ok((numbering, time))
}

/// A time of week as whole seconds and attoseconds.
///
/// # Errors
///
/// [`Refusal::Overflow`] where the seconds do not fit a `u32`, which a
/// time of week below 604 800 s never is.
pub fn time_of_week_parts(time: WeekTime) -> Answer<(u32, u64)> {
    let seconds =
        u32::try_from(time.time_of_week.whole_seconds()).map_err(|_| Refusal::Overflow)?;
    Ok((seconds, time.time_of_week.subsec_attos()))
}

/// The line of `hc_gnss_week`: the full week, the week as the field
/// broadcasts it, and the time of week in whole seconds and attoseconds.
///
/// # Errors
///
/// As [`gnss_week`].
pub fn gnss_week_line(id: &str, seconds: i64, attoseconds: u64) -> Answer<String> {
    let (numbering, time) = gnss_week(id, seconds, attoseconds)?;
    let (tow_seconds, tow_attoseconds) = time_of_week_parts(time)?;
    Ok(alloc::format!(
        "{}\t{}\t{tow_seconds}\t{tow_attoseconds}\n",
        time.week,
        numbering.broadcast(time.week)
    ))
}

/// The TAI instant of a full week and a time of week under a field.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an identifier [`week_numbering`] does not
/// know, and [`Refusal::OutOfRange`] for a time of week from 604 800 s or
/// attoseconds from 10¹⁸.
pub fn gnss_to_tai(
    id: &str,
    week: u32,
    tow_seconds: u32,
    tow_attoseconds: u64,
) -> Answer<Instant<Tai>> {
    let numbering = week_numbering(id)?;
    let time_of_week =
        Duration::new(i128::from(tow_seconds), tow_attoseconds).map_err(|_| Refusal::OutOfRange)?;
    Ok(numbering.to_tai(WeekTime { week, time_of_week })?)
}

/// The line of `hc_gnss_to_tai`: the TAI seconds and attoseconds.
///
/// # Errors
///
/// As [`gnss_to_tai`].
pub fn gnss_to_tai_line(
    id: &str,
    week: u32,
    tow_seconds: u32,
    tow_attoseconds: u64,
) -> Answer<String> {
    let (seconds, attoseconds) = tai_parts(gnss_to_tai(id, week, tow_seconds, tow_attoseconds)?)?;
    Ok(alloc::format!("{seconds}\t{attoseconds}\n"))
}

/// The rule that picks one full week out of the family a broadcast week
/// names: `not-before`, [`WeekNumbering::resolve_not_before`], or
/// `nearest`, [`WeekNumbering::resolve_nearest`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for an identifier or a rule not named here or in
/// [`week_numbering`], and [`Refusal::OutOfRange`] for a broadcast week
/// that does not fit the field or an answer past week 2³² − 1.
pub fn gnss_resolve_week(
    id: &str,
    broadcast: u32,
    rule: &str,
    reference_tai_seconds: i64,
) -> Answer<u32> {
    let numbering = week_numbering(id)?;
    let reference = tai_instant(reference_tai_seconds, 0)?;
    let week = if names(rule, "not-before") {
        numbering.resolve_not_before(broadcast, reference)
    } else if names(rule, "nearest") {
        numbering.resolve_nearest(broadcast, reference)
    } else {
        return Err(Refusal::Unknown);
    };
    week.map_err(|_| Refusal::OutOfRange)
}

/// GLONASS's four-year interval *N*4 and day *N*T at a TAI instant,
/// through the leap-second table: strict refuses outside it, else the
/// last published offset holds.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸, and for an instant
/// before 1996 or from 2100, where the intervals are not counted;
/// [`Refusal::NoData`] outside the table under the strict policy.
pub fn glonass_date(seconds: i64, attoseconds: u64, strict: bool) -> Answer<GlonassDate> {
    let policy = if strict {
        LeapPolicy::Strict
    } else {
        LeapPolicy::Extrapolate
    };
    let time = GlonassTime::from_tai(tai_instant(seconds, attoseconds)?, policy)?;
    Ok(time.date()?)
}

/// The line of `hc_glonass_date`: *N*4 and *N*T.
///
/// # Errors
///
/// As [`glonass_date`].
pub fn glonass_date_line(seconds: i64, attoseconds: u64, strict: bool) -> Answer<String> {
    let date = glonass_date(seconds, attoseconds, strict)?;
    Ok(alloc::format!(
        "{}\t{}\n",
        date.four_year_interval,
        date.day
    ))
}

/// The fixed day and the time of day, in seconds, of an OLE Automation
/// date, as [`spreadsheet::ole_automation`] reads it.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a value that is not finite or lies
/// outside 1 January 100 to 31 December 9999.
pub fn fixed_from_ole_automation(value: f64) -> Answer<(Rd, f64)> {
    let (day, time) = spreadsheet::ole_automation(value).map_err(|_| Refusal::OutOfRange)?;
    Ok((day, time.as_secs_f64()))
}

/// The line of `hc_fixed_from_ole_automation`: the fixed day and the
/// seconds into it.
///
/// # Errors
///
/// As [`fixed_from_ole_automation`].
pub fn fixed_from_ole_automation_line(value: f64) -> Answer<String> {
    let (day, seconds) = fixed_from_ole_automation(value)?;
    Ok(alloc::format!("{}\t{seconds}\n", day.0))
}

/// The OLE Automation date of a fixed day and a time of day in seconds,
/// as [`spreadsheet::to_ole_automation`] writes it.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a time of day that is not finite, negative
/// or not below 86 400 s, and for a day outside 1 January 100 to
/// 31 December 9999.
pub fn ole_automation_from_fixed(fixed: i64, seconds_of_day: f64) -> Answer<f64> {
    let time = Duration::from_secs_f64(seconds_of_day).map_err(|_| Refusal::OutOfRange)?;
    spreadsheet::to_ole_automation(Rd(fixed), time).map_err(calendar_refusal)
}

/// The line of `hc_ole_automation_from_fixed`: the value.
///
/// # Errors
///
/// As [`ole_automation_from_fixed`].
pub fn ole_automation_from_fixed_line(fixed: i64, seconds_of_day: f64) -> Answer<String> {
    Ok(alloc::format!(
        "{}\n",
        ole_automation_from_fixed(fixed, seconds_of_day)?
    ))
}

/// What an Excel 1900 serial names, serial 60 included.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] below serial 1 and above
/// [`spreadsheet::LAST_SERIAL`].
pub fn excel_1900_day(serial: i64) -> Answer<Excel1900Day> {
    spreadsheet::excel_1900_day(serial).map_err(calendar_refusal)
}

/// The line of `hc_excel_1900_day`: the fixed day, empty for serial 60,
/// and `1` for serial 60, the 29 February 1900 that Excel counts and no
/// calendar has, else `0`.
///
/// # Errors
///
/// As [`excel_1900_day`].
pub fn excel_1900_day_line(serial: i64) -> Answer<String> {
    Ok(match excel_1900_day(serial)? {
        Excel1900Day::Date(day) => alloc::format!("{}\t0\n", day.0),
        Excel1900Day::Phantom29February1900 => String::from("\t1\n"),
    })
}

/// The leap-second policy a `strict` flag asks for: strict refuses outside
/// the table, else the last published offset holds.
const fn policy(strict: bool) -> LeapPolicy {
    if strict {
        LeapPolicy::Strict
    } else {
        LeapPolicy::Extrapolate
    }
}

/// The TAI instant of a POSIX timestamp, as whole seconds and attoseconds.
///
/// # Errors
///
/// [`Refusal::NoData`] outside the leap-second table under the strict
/// policy, and [`Refusal::Overflow`] for a timestamp whose TAI seconds do
/// not fit an `i64`.
pub fn tai_from_unix(unix_seconds: i64, strict: bool) -> Answer<(i64, u64)> {
    tai_parts(unix::tai_from_unix(
        UnixTime::from_seconds(unix_seconds),
        policy(strict),
    )?)
}

/// The line of `hc_tai_from_unix`: the TAI seconds and the attoseconds.
///
/// # Errors
///
/// As [`tai_from_unix`].
pub fn tai_from_unix_line(unix_seconds: i64, strict: bool) -> Answer<String> {
    let (seconds, attoseconds) = tai_from_unix(unix_seconds, strict)?;
    Ok(alloc::format!("{seconds}\t{attoseconds}\n"))
}

/// The UTC label of a whole TAI second: its POSIX second, and whether it
/// is an inserted leap second, `23:59:60`, which POSIX time cannot
/// express and names by the second that follows it.
///
/// # Errors
///
/// [`Refusal::NoData`] outside the leap-second table under the strict
/// policy.
pub fn utc_from_tai(tai_seconds: i64, strict: bool) -> Answer<UtcInstant> {
    let instant = Instant::<Tai>::from_epoch(Duration::from_secs(i128::from(tai_seconds)));
    Ok(unix::utc_from_tai(instant, policy(strict))?)
}

/// The line of `hc_utc_from_tai`: the POSIX second and `1` for an inserted
/// leap second, else `0`.
///
/// # Errors
///
/// As [`utc_from_tai`].
pub fn utc_from_tai_line(tai_seconds: i64, strict: bool) -> Answer<String> {
    let utc = utc_from_tai(tai_seconds, strict)?;
    Ok(alloc::format!(
        "{}\t{}\n",
        utc.unix_seconds,
        u8::from(utc.leap_second)
    ))
}

/// A POSIX instant from whole seconds and attoseconds.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸.
pub fn unix_instant(seconds: i64, attoseconds: u64) -> Answer<UnixTime> {
    UnixTime::new(seconds, attoseconds).map_err(|_| Refusal::OutOfRange)
}

/// The `tai64-posix-plus-10` label of a POSIX instant, as lower-case
/// hexadecimal: sixteen digits for TAI64, the second, and twenty-four for
/// TAI64N, the nanosecond, as daemontools' `tai64n` writes them.
///
/// # Errors
///
/// [`Refusal::Unknown`] for TAI64NA, which the convention does not write,
/// and [`Refusal::OutOfRange`] for a second whose label would fall outside
/// 0 to 2⁶³ − 1.
pub fn tai64_posix_plus_10_hex(unix: UnixTime, format: Tai64Format) -> Answer<String> {
    let mut bytes = [0u8; 12];
    match format {
        Tai64Format::Tai64 => {
            bytes[..8].copy_from_slice(&tai64::encode_tai64_posix_plus_10(unix)?);
        }
        Tai64Format::Tai64N => bytes = tai64::encode_tai64n_posix_plus_10(unix)?,
        Tai64Format::Tai64Na => return Err(Refusal::Unknown),
    }
    let mut out = String::new();
    for byte in &bytes[..format.bytes()] {
        let _ = write!(out, "{byte:02x}");
    }
    Ok(out)
}

/// The line of `hc_tai64_posix_plus_10_encode`: the label in lower-case
/// hexadecimal.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a format that is not `tai64` or `tai64n`, and
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸ or a second outside
/// the labels.
pub fn tai64_posix_plus_10_encode_line(
    unix_seconds: i64,
    attoseconds: u64,
    format: &str,
) -> Answer<String> {
    let format = Tai64Format::from_name(format)?;
    let mut out = tai64_posix_plus_10_hex(unix_instant(unix_seconds, attoseconds)?, format)?;
    out.push('\n');
    Ok(out)
}

/// A `tai64-posix-plus-10` label in hexadecimal read back: its format, by
/// its length, and the POSIX instant it names, with no leap-second table
/// consulted.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not 16 or 24 hexadecimal
/// digits, and [`Refusal::OutOfRange`] for a reserved label, from 2⁶³, or
/// a nanosecond count above 999 999 999.
pub fn tai64_posix_plus_10_decode(hex: &str) -> Answer<(Tai64Format, UnixTime)> {
    let (format, bytes) = tai64_bytes(hex)?;
    let mut eight = [0u8; 8];
    let mut twelve = [0u8; 12];
    eight.copy_from_slice(&bytes[..8]);
    twelve.copy_from_slice(&bytes[..12]);
    let unix = match format {
        Tai64Format::Tai64 => tai64::decode_tai64_posix_plus_10(eight)?,
        Tai64Format::Tai64N => tai64::decode_tai64n_posix_plus_10(twelve)?,
        Tai64Format::Tai64Na => return Err(Refusal::Malformed),
    };
    Ok((format, unix))
}

/// The line of `hc_tai64_posix_plus_10_decode`: the format, the POSIX
/// seconds and the attoseconds.
///
/// # Errors
///
/// As [`tai64_posix_plus_10_decode`].
pub fn tai64_posix_plus_10_decode_line(hex: &str) -> Answer<String> {
    let (format, unix) = tai64_posix_plus_10_decode(hex)?;
    Ok(alloc::format!(
        "{}\t{}\t{}\n",
        format.name(),
        unix.seconds(),
        unix.subsec_attos()
    ))
}

/// The sixteen octets of a UUID written in RFC 9562's string form: 32
/// hexadecimal digits in either case, either bare or with hyphens after
/// the 8th, 12th, 16th and 20th, optionally after `urn:uuid:`.
///
/// # Errors
///
/// [`Refusal::Malformed`] for any other text.
pub fn uuid_bytes(text: &str) -> Answer<[u8; 16]> {
    let text = text.trim();
    let text = match text.get(..9) {
        Some(prefix) if prefix.eq_ignore_ascii_case("urn:uuid:") => &text[9..],
        _ => text,
    };
    let mut digits = [0u8; 32];
    let bytes = text.as_bytes();
    match bytes.len() {
        32 => digits.copy_from_slice(bytes),
        36 => {
            let mut at = 0;
            for (index, &byte) in bytes.iter().enumerate() {
                if matches!(index, 8 | 13 | 18 | 23) {
                    if byte != b'-' {
                        return Err(Refusal::Malformed);
                    }
                    continue;
                }
                digits[at] = byte;
                at += 1;
            }
        }
        _ => return Err(Refusal::Malformed),
    }
    let mut out = [0u8; 16];
    hex_into(&digits, &mut out)?;
    Ok(out)
}

/// The version, the 60-bit timestamp and the POSIX instant of a version 1
/// or version 6 UUID: the start of the 100-nanosecond interval its
/// timestamp counts.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text [`uuid_bytes`] does not read, and
/// [`Refusal::NoData`] for a UUID of another version or variant, which
/// carries no such timestamp.
pub fn uuid_timestamp(text: &str) -> Answer<(TimeVersion, u64, UnixTime)> {
    let (version, timestamp) = uuid::decode(uuid_bytes(text)?).map_err(|_| Refusal::NoData)?;
    Ok((version, timestamp, uuid::unix_from_timestamp(timestamp)?))
}

/// The line of `hc_uuid_timestamp`: the version, `1` or `6`, the
/// timestamp in 100-nanosecond intervals from 1582-10-15, and its POSIX
/// seconds and attoseconds.
///
/// # Errors
///
/// As [`uuid_timestamp`].
pub fn uuid_timestamp_line(text: &str) -> Answer<String> {
    let (version, timestamp, unix) = uuid_timestamp(text)?;
    Ok(alloc::format!(
        "{}\t{timestamp}\t{}\t{}\n",
        version.number(),
        unix.seconds(),
        unix.subsec_attos()
    ))
}

/// The UUID time fields of a timestamp in one version's layout: octets 0
/// to 7, `time_low` or `time_high`, `time_mid` and the version with the
/// remaining 12 bits, as RFC 9562's hex-and-dash form writes them, in
/// lower case.
fn uuid_time_fields(version: TimeVersion, timestamp: u64) -> Answer<String> {
    // The clock sequence and node are not time; octets 8 to 15 are dropped.
    let uuid = uuid::encode(version, timestamp, [0; 8])?;
    let mut out = String::new();
    for (index, byte) in uuid[..8].iter().enumerate() {
        if index == 4 || index == 6 {
            out.push('-');
        }
        let _ = write!(out, "{byte:02x}");
    }
    Ok(out)
}

/// The 60-bit UUID timestamp of a POSIX instant, the 100-nanosecond
/// interval that contains it counted from 1582-10-15, and the time fields
/// a version 1 and a version 6 UUID write it in.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸, and for an instant
/// before 1582-10-15 00:00 UTC or past the 60-bit field's last interval,
/// on 5236-03-31.
pub fn uuid_timestamp_encode(unix_seconds: i64, attoseconds: u64) -> Answer<(u64, String, String)> {
    let timestamp = uuid::timestamp_from_unix(unix_instant(unix_seconds, attoseconds)?)?;
    Ok((
        timestamp,
        uuid_time_fields(TimeVersion::V1, timestamp)?,
        uuid_time_fields(TimeVersion::V6, timestamp)?,
    ))
}

/// The line of `hc_uuid_timestamp_encode`: the timestamp, and the first
/// three groups of a version 1 and of a version 6 UUID that carry it, the
/// caller's clock sequence and node to follow.
///
/// # Errors
///
/// As [`uuid_timestamp_encode`].
pub fn uuid_timestamp_encode_line(unix_seconds: i64, attoseconds: u64) -> Answer<String> {
    let (timestamp, v1, v6) = uuid_timestamp_encode(unix_seconds, attoseconds)?;
    Ok(alloc::format!("{timestamp}\t{v1}\t{v6}\n"))
}

/// The 128-bit NTP date of a POSIX instant, the fraction floored to 2⁻⁶⁴ s.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸, and
/// [`Refusal::Overflow`] for a second so late that its count from 1900
/// leaves an `i64`.
pub fn ntp_encode(unix_seconds: i64, attoseconds: u64) -> Answer<NtpDate> {
    Ok(NtpDate::from_unix(unix_instant(
        unix_seconds,
        attoseconds,
    )?)?)
}

/// The line of `hc_ntp_encode`: the era, the era offset, the fraction in
/// units of 2⁻⁶⁴ s, the 128-bit date in RFC 5905's Figure 3 layout and the
/// 64-bit timestamp in its wire layout, each in lower-case hexadecimal.
///
/// # Errors
///
/// As [`ntp_encode`].
pub fn ntp_encode_line(unix_seconds: i64, attoseconds: u64) -> Answer<String> {
    let date = ntp_encode(unix_seconds, attoseconds)?;
    let mut out = alloc::format!("{}\t{}\t{}\t", date.era, date.offset, date.fraction);
    for byte in date.to_bytes() {
        let _ = write!(out, "{byte:02x}");
    }
    out.push('\t');
    for byte in date.timestamp().to_bytes() {
        let _ = write!(out, "{byte:02x}");
    }
    out.push('\n');
    Ok(out)
}

/// A 64-bit NTP timestamp placed in the era that puts it within 2³¹ s of a
/// reference POSIX second, from 2³¹ s before it, included, to 2³¹ s after
/// it, excluded: the 128-bit date and its POSIX instant.
///
/// # Errors
///
/// [`Refusal::NoData`] for the zero timestamp, which RFC 5905 reserves for
/// unknown or unsynchronised time, and [`Refusal::Overflow`] or
/// [`Refusal::OutOfRange`] where the reference is so far off that the era
/// or the second does not fit.
pub fn ntp_resolve(
    seconds: u32,
    fraction: u32,
    reference_unix: i64,
) -> Answer<(NtpDate, UnixTime)> {
    let stamp = NtpTimestamp { seconds, fraction };
    if stamp.is_unknown() {
        return Err(Refusal::NoData);
    }
    let date = stamp.resolve(UnixTime::from_seconds(reference_unix))?;
    Ok((date, date.to_unix()?))
}

/// The line of `hc_ntp_resolve`: the era, the era offset, the fraction in
/// units of 2⁻⁶⁴ s, and the POSIX seconds and attoseconds.
///
/// # Errors
///
/// As [`ntp_resolve`].
pub fn ntp_resolve_line(seconds: u32, fraction: u32, reference_unix: i64) -> Answer<String> {
    let (date, unix) = ntp_resolve(seconds, fraction, reference_unix)?;
    Ok(alloc::format!(
        "{}\t{}\t{}\t{}\t{}\n",
        date.era,
        date.offset,
        date.fraction,
        unix.seconds(),
        unix.subsec_attos()
    ))
}

/// The local reading a pair of FAT words names: its fixed day and the
/// seconds into it, always even.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a word above 65 535, and
/// [`Refusal::InvalidDate`] for a word whose fields name no day or no
/// time: a month of 0 or above 12, a day of 0 or one the month lacks, an
/// hour above 23, a minute above 59, or a halved second above 29.
#[cfg(feature = "format")]
pub fn fat_decode(date: u32, time: u32) -> Answer<(Rd, u32)> {
    let (Ok(date), Ok(time)) = (u16::try_from(date), u16::try_from(time)) else {
        return Err(Refusal::OutOfRange);
    };
    let reading = hc_format::fat::decode(date, time).map_err(|_| Refusal::InvalidDate)?;
    let clock = reading.time;
    let seconds = u32::from(clock.hour()) * 3_600
        + u32::from(clock.minute()) * 60
        + u32::from(clock.second());
    Ok((reading.day, seconds))
}

/// The line of `hc_fat_decode`: the fixed day and the seconds into it.
///
/// # Errors
///
/// As [`fat_decode`].
#[cfg(feature = "format")]
pub fn fat_decode_line(date: u32, time: u32) -> Answer<String> {
    let (day, seconds) = fat_decode(date, time)?;
    Ok(alloc::format!("{}\t{seconds}\n", day.0))
}

/// The FAT date and time words of a fixed day and a time of day in whole
/// seconds, the second rounded down to an even one as the word carries it.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a time of day from 86 400 s, or a day
/// outside 1980 to 2107, the years the date word holds.
#[cfg(feature = "format")]
pub fn fat_encode(fixed: i64, seconds_of_day: u32) -> Answer<(u16, u16)> {
    if seconds_of_day >= 86_400 {
        return Err(Refusal::OutOfRange);
    }
    // Each below 24, 60 and 60 by the check above.
    let (hour, minute, second) = (
        (seconds_of_day / 3_600) as u8,
        (seconds_of_day / 60 % 60) as u8,
        (seconds_of_day % 60) as u8,
    );
    let time = hc_calendar::CivilTime::hms(hour, minute, second).map_err(calendar_refusal)?;
    hc_format::fat::encode(hc_calendar::CivilDateTime::new(Rd(fixed), time))
        .map_err(|_| Refusal::OutOfRange)
}

/// The line of `hc_fat_encode`: the date word and the time word.
///
/// # Errors
///
/// As [`fat_encode`].
#[cfg(feature = "format")]
pub fn fat_encode_line(fixed: i64, seconds_of_day: u32) -> Answer<String> {
    let (date, time) = fat_encode(fixed, seconds_of_day)?;
    Ok(alloc::format!("{date}\t{time}\n"))
}

/// The Swatch Internet Time at a POSIX instant, @000 to @999: the
/// thousandth of the day of UTC+1, Biel Mean Time, that it falls in.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸.
pub fn swatch_beat(unix_seconds: i64, attoseconds: u64) -> Answer<u16> {
    Ok(Beat::at(unix_instant(unix_seconds, attoseconds)?).value())
}

/// The epoch notation a name selects: `J` or `julian-epoch` for Julian
/// years of 365.25 days of TT, `B` or `besselian-epoch` for Besselian
/// years, in any case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other name.
pub fn epoch_kind(notation: &str) -> Answer<EpochKind> {
    if names(notation, "J") || names(notation, "julian-epoch") {
        Ok(EpochKind::Julian)
    } else if names(notation, "B") || names(notation, "besselian-epoch") {
        Ok(EpochKind::Besselian)
    } else {
        Err(Refusal::Unknown)
    }
}

/// A TT instant from whole seconds from 1970-01-01 00:00:00 TT and
/// attoseconds.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸.
pub fn tt_instant(seconds: i64, attoseconds: u64) -> Answer<Instant<Tt>> {
    Duration::new(i128::from(seconds), attoseconds)
        .map(Instant::from_epoch)
        .map_err(|_| Refusal::OutOfRange)
}

/// The Julian or Besselian epoch of a TT instant, as a year with a
/// fraction: `J2000.0` is 2000-01-01T12:00:00 TT.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a notation [`epoch_kind`] does not read, and
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸.
pub fn epoch_from_tt(
    notation: &str,
    tt_seconds: i64,
    attoseconds: u64,
) -> Answer<(EpochKind, f64)> {
    let kind = epoch_kind(notation)?;
    let year = kind.epoch(tt_instant(tt_seconds, attoseconds)?);
    if year.is_finite() {
        Ok((kind, year))
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// The line of `hc_epoch_from_tt`: the notation's letter, `J` or `B`, and
/// the epoch.
///
/// # Errors
///
/// As [`epoch_from_tt`].
pub fn epoch_from_tt_line(notation: &str, tt_seconds: i64, attoseconds: u64) -> Answer<String> {
    let (kind, year) = epoch_from_tt(notation, tt_seconds, attoseconds)?;
    Ok(alloc::format!("{}\t{year}\n", kind.letter()))
}

/// The TT instant of a Julian or Besselian epoch, as whole seconds from
/// 1970-01-01 00:00:00 TT and attoseconds, with the notation it was read
/// in. An empty notation reads the year as SOFA reads an epoch written
/// without a letter: Besselian before 1984.0, Julian from it.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a notation [`epoch_kind`] does not read, not
/// empty; [`Refusal::OutOfRange`] for a year that is not finite; and
/// [`Refusal::Overflow`] for one whose instant does not fit.
pub fn tt_from_epoch(notation: &str, year: f64) -> Answer<(EpochKind, i64, u64)> {
    let kind = if notation.trim().is_empty() {
        EpochKind::unprefixed(year)
    } else {
        epoch_kind(notation)?
    };
    let reading = kind.instant(year)?.since_epoch();
    let seconds = i64::try_from(reading.whole_seconds()).map_err(|_| Refusal::Overflow)?;
    Ok((kind, seconds, reading.subsec_attos()))
}

/// The line of `hc_tt_from_epoch`: the notation's letter, the TT seconds
/// and the attoseconds.
///
/// # Errors
///
/// As [`tt_from_epoch`].
pub fn tt_from_epoch_line(notation: &str, year: f64) -> Answer<String> {
    let (kind, seconds, attoseconds) = tt_from_epoch(notation, year)?;
    Ok(alloc::format!(
        "{}\t{seconds}\t{attoseconds}\n",
        kind.letter()
    ))
}

/// The Modified Julian Date of 1970-01-01, the POSIX epoch.
const MJD_OF_UNIX_EPOCH: i64 = 40_587;

/// How many columns [`tt_bipm_line`] writes.
pub const TT_BIPM_COLUMNS: usize = 5;

/// The samples of a TT(BIPM) realisation written as text: one line per
/// sample, the Modified Julian Date at 0 h UTC and TT(BIPMxx) − TAI −
/// 32.184 s there in microseconds, separated by a tab, as the first and
/// third columns of the BIPM's `TTBIPM` files give them. Blank lines are
/// skipped; the dates must ascend.
///
/// # Errors
///
/// [`Refusal::Malformed`] for a line that is not two such cells, a value
/// that is not finite, or a date that does not follow the one before, and
/// [`Refusal::OutOfRange`] for a date whose midnight is not a POSIX second
/// an `i64` holds.
pub fn tt_bipm_samples(text: &str) -> Answer<Vec<(i64, f64)>> {
    let mut samples: Vec<(i64, f64)> = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut cells = line.split('\t');
        let (Some(mjd), Some(microseconds), None) = (cells.next(), cells.next(), cells.next())
        else {
            return Err(Refusal::Malformed);
        };
        let mjd: i64 = mjd.trim().parse().map_err(|_| Refusal::Malformed)?;
        let microseconds = plain_decimal(microseconds.trim()).ok_or(Refusal::Malformed)?;
        if samples.last().is_some_and(|&(last, _)| last >= mjd) {
            return Err(Refusal::Malformed);
        }
        mjd.checked_sub(MJD_OF_UNIX_EPOCH)
            .and_then(|days| days.checked_mul(86_400))
            .ok_or(Refusal::OutOfRange)?;
        samples.push((mjd, microseconds));
    }
    Ok(samples)
}

/// The powers of ten a [`plain_decimal`] divides by, each exact in an
/// `f64`.
const POWERS_OF_TEN: [f64; 19] = [
    1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16,
    1e17, 1e18,
];

/// A number written in plain decimal notation — an optional sign, digits,
/// and an optional point with more digits — of at most 18 digits, as the
/// BIPM's tables write it. The value is the digits as an integer divided
/// by an exact power of ten, so it is the nearest `f64` wherever the
/// digits fit 53 bits. It reads what the tables hold without the general
/// float parser, whose size the `timestamps` layer would carry.
fn plain_decimal(text: &str) -> Option<f64> {
    let (negative, unsigned) = match text.as_bytes().first() {
        Some(b'-') => (true, &text[1..]),
        Some(b'+') => (false, &text[1..]),
        _ => (false, text),
    };
    let (whole, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    let digits = whole.len() + fraction.len();
    if digits == 0 || digits > 18 {
        return None;
    }
    let mut mantissa: u64 = 0;
    for byte in whole.bytes().chain(fraction.bytes()) {
        if !byte.is_ascii_digit() {
            return None;
        }
        mantissa = mantissa * 10 + u64::from(byte - b'0');
    }
    let value = mantissa as f64 / POWERS_OF_TEN[fraction.len()];
    Some(if negative { -value } else { value })
}

/// A duration as whole seconds and attoseconds, the seconds floored.
///
/// # Errors
///
/// [`Refusal::Overflow`] for seconds outside an `i64`.
fn duration_parts(duration: Duration) -> Answer<(i64, u64)> {
    let seconds = i64::try_from(duration.whole_seconds()).map_err(|_| Refusal::Overflow)?;
    Ok((seconds, duration.subsec_attos()))
}

/// The line of `hc_tt_bipm`: at a TAI instant, from a realisation of
/// TT(BIPM) given as [`tt_bipm_samples`] reads it, TT(BIPMxx) − TT(TAI) in
/// seconds, interpolated linearly in TAI between the samples; TT(BIPMxx) −
/// TAI, 32.184 s more, as whole seconds and attoseconds; and the
/// TT(BIPMxx) reading of the instant from that scale's 1970 epoch, as
/// whole seconds and attoseconds. `strict` places the samples by the
/// leap-second table and refuses one outside it; otherwise the table's
/// ends are held.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for attoseconds from 10¹⁸, the errors of
/// [`tt_bipm_samples`], [`Refusal::NoData`] for an instant before the first
/// sample or after the last, an empty series, or under `strict` a sample
/// outside the leap-second table, and [`Refusal::Overflow`] for a reading
/// outside an `i64` of seconds.
pub fn tt_bipm_line(
    series: &str,
    tai_seconds: i64,
    attoseconds: u64,
    strict: bool,
) -> Answer<String> {
    let instant = tai_instant(tai_seconds, attoseconds)?;
    let samples = tt_bipm_samples(series)?;
    // The name of the realisation stays with the caller, who chose it.
    let series = TtBipmSeries {
        realisation: "",
        samples: &samples,
    };
    let policy = policy(strict);
    let offset = series.offset_from_tt_tai(instant, policy)?;
    let (minus_seconds, minus_attoseconds) = duration_parts(series.minus_tai(instant, policy)?)?;
    let (seconds, attos) = duration_parts(series.reading(instant, policy)?)?;
    Ok(alloc::format!(
        "{offset}\t{minus_seconds}\t{minus_attoseconds}\t{seconds}\t{attos}\n"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rows of the BIPM's `TTBIPM.2025`, the first and third columns, as
    /// `hc-core`'s test reads them: 27.6740 µs on MJD 58 479 and
    /// 27.6745 µs on MJD 58 489.
    const TT_BIPM25: &str = "42589\t46.258\n42599\t45.439\n58479\t27.6740\n58489\t27.6745\n";

    /// 0 h UTC on MJD 58 479, 2018-12-22, when TAI − UTC was 37 s.
    const MJD_58479_TAI: i64 = (58_479 - 40_587) * 86_400 + 37;

    #[test]
    fn a_sample_of_ttbipm25_reads_as_the_table_gives_it() {
        let line = tt_bipm_line(TT_BIPM25, MJD_58479_TAI, 0, true).expect("in the series");
        let cells: Vec<&str> = line.trim_end_matches('\n').split('\t').collect();
        assert_eq!(cells.len(), TT_BIPM_COLUMNS);
        let offset: f64 = cells[0].parse().expect("seconds");
        assert!((offset - 27.674e-6).abs() < 1e-15, "{offset}");
        assert_eq!(cells[1], "32");
        let attoseconds: u64 = cells[2].parse().expect("attoseconds");
        assert!(
            attoseconds.abs_diff(184_027_674_000_000_000) < 1_000,
            "{attoseconds}"
        );
        assert_eq!(cells[3], (MJD_58479_TAI + 32).to_string());
        assert_eq!(cells[4], cells[2]);
        // Five days on, halfway to 27.6745 µs.
        let midway = tt_bipm_line(TT_BIPM25, MJD_58479_TAI + 5 * 86_400, 0, true).expect("inside");
        let offset: f64 = midway
            .split('\t')
            .next()
            .expect("a cell")
            .parse()
            .expect("seconds");
        assert!((offset - 27.674_25e-6).abs() < 1e-15, "{offset}");
    }

    #[test]
    fn a_series_is_read_strictly_and_never_extrapolated() {
        let last = MJD_58479_TAI + 10 * 86_400;
        assert_eq!(
            tt_bipm_line(TT_BIPM25, last + 1, 0, true),
            Err(Refusal::NoData)
        );
        assert_eq!(
            tt_bipm_line("", MJD_58479_TAI, 0, true),
            Err(Refusal::NoData)
        );
        for malformed in [
            "58479",
            "58479\t1e3",
            "58479\t.",
            "58479\t-",
            "58479\t1234567890.123456789",
            "58479\t27.6\t1",
            "x\t1",
            "58479\tNaN",
            "2\t1\n1\t1",
        ] {
            assert_eq!(
                tt_bipm_samples(malformed),
                Err(Refusal::Malformed),
                "{malformed:?}"
            );
        }
        assert_eq!(
            tt_bipm_samples(&alloc::format!("{}\t1", i64::MAX)),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            tt_bipm_samples("\r\n58479\t27.6740\r\n"),
            Ok(alloc::vec![(58_479, 27.674)])
        );
        assert_eq!(plain_decimal("-.5"), Some(-0.5));
        assert_eq!(plain_decimal("+46.258"), Some(46.258));
        assert_eq!(plain_decimal("27."), Some(27.0));
        assert_eq!(
            tt_bipm_line(TT_BIPM25, 0, 1_000_000_000_000_000_000, true),
            Err(Refusal::OutOfRange)
        );
    }

    /// Bernstein's page: 2⁶² is the label of the second that began 1970
    /// TAI, so the origin is `4000000000000000`.
    #[test]
    fn the_tai64_origin_is_two_to_the_sixty_second() {
        assert_eq!(
            tai64_encode_line(0, 0, "TAI64").as_deref(),
            Ok("4000000000000000\n")
        );
        assert_eq!(
            tai64_decode_line("4000000000000000").as_deref(),
            Ok("tai64\t0\t0\n")
        );
        assert_eq!(
            tai64_encode_line(-1, 999_999_999_999_999_999, "tai64na").as_deref(),
            Ok("3fffffffffffffff3b9ac9ff3b9ac9ff\n")
        );
        assert_eq!(
            tai64_decode_line("3FFFFFFFFFFFFFFF3B9AC9FF3B9AC9FF").as_deref(),
            Ok("tai64na\t-1\t999999999999999999\n")
        );
        assert_eq!(tai64_decode("400000000000000"), Err(Refusal::Malformed));
        assert_eq!(tai64_decode("400000000000000g"), Err(Refusal::Malformed));
        assert_eq!(tai64_decode("8000000000000000"), Err(Refusal::OutOfRange));
        assert_eq!(tai64_encode_line(0, 0, "tai32"), Err(Refusal::Unknown));
        assert_eq!(
            tai64_encode_line(0, 1_000_000_000_000_000_000, "tai64"),
            Err(Refusal::OutOfRange)
        );
    }

    /// GPS week 2048 began at 2019-04-06 23:59:42 UTC, when `GPS − UTC`
    /// was 18 s and `TAI − UTC` 37 s: POSIX 1 554 595 182, which is
    /// 1 554 595 219 on the count of TAI seconds from 1970 TAI.
    #[test]
    fn the_april_2019_rollover_crosses_the_boundary() {
        let tai = 1_554_595_200 - 18 + 37;
        let (_, time) = gnss_week("gps-lnav-week", tai, 0).expect("after week zero");
        assert_eq!(time.week, 2048);
        assert_eq!(
            gnss_week_line("GPS-LNAV-WEEK", tai, 0).as_deref(),
            Ok("2048\t0\t0\t0\n")
        );
        assert_eq!(
            gnss_to_tai_line("gps-lnav-week", 2048, 0, 0),
            Ok(alloc::format!("{tai}\t0\n"))
        );
        assert_eq!(
            gnss_resolve_week("gps-lnav-week", 0, "not-before", tai),
            Ok(2048)
        );
        assert_eq!(
            gnss_resolve_week("gps-lnav-week", 1023, "nearest", tai),
            Ok(2047)
        );
        assert_eq!(
            gnss_resolve_week("gps-lnav-week", 1024, "nearest", tai),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            gnss_resolve_week("gps-lnav-week", 0, "latest", tai),
            Err(Refusal::Unknown)
        );
        assert_eq!(gnss_week("gps-week", tai, 0), Err(Refusal::Unknown));
        assert_eq!(gnss_week("gps-lnav-week", 0, 0), Err(Refusal::NoData));
        assert_eq!(
            gnss_to_tai("gps-lnav-week", 0, 604_800, 0),
            Err(Refusal::OutOfRange)
        );
    }

    /// Microsoft Learn's examples, `ms-tooadate`: 2.25 is 06:00 on
    /// 1 January 1900 and −1.25 06:00 on 29 December 1899.
    #[test]
    fn ole_automation_dates_cross_both_ways() {
        let new_year_1900 = hc_calendars_solar::gregorian::to_fixed(1900, 1, 1)
            .expect("a date")
            .0;
        assert_eq!(
            fixed_from_ole_automation_line(2.25),
            Ok(alloc::format!("{new_year_1900}\t21600\n"))
        );
        assert_eq!(
            fixed_from_ole_automation_line(-1.25),
            Ok(alloc::format!("{}\t21600\n", new_year_1900 - 3))
        );
        assert_eq!(
            ole_automation_from_fixed_line(new_year_1900 - 3, 21_600.0).as_deref(),
            Ok("-1.25\n")
        );
        assert_eq!(
            ole_automation_from_fixed(new_year_1900, 86_400.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            fixed_from_ole_automation(f64::NAN),
            Err(Refusal::OutOfRange)
        );
    }

    #[test]
    fn serial_60_is_named_and_never_dated() {
        assert_eq!(excel_1900_day_line(60).as_deref(), Ok("\t1\n"));
        let march = hc_calendars_solar::gregorian::to_fixed(1900, 3, 1)
            .expect("a date")
            .0;
        assert_eq!(excel_1900_day_line(61), Ok(alloc::format!("{march}\t0\n")));
        assert_eq!(excel_1900_day(0), Err(Refusal::OutOfRange));
    }

    /// The leap second at the end of 2016: 23:59:59 UTC was TAI + 36 s
    /// and 00:00:00 on 1 January 2017 TAI + 37 s, so TAI second
    /// 1 483 228 836 from 1970 TAI is the inserted 23:59:60, named by the
    /// POSIX second after it.
    #[test]
    fn the_tai_bridge_names_the_leap_second_of_2016() {
        let new_year = 1_483_228_800;
        assert_eq!(
            tai_from_unix_line(new_year, true),
            Ok(alloc::format!("{}\t0\n", new_year + 37))
        );
        assert_eq!(
            tai_from_unix_line(new_year - 1, false),
            Ok(alloc::format!("{}\t0\n", new_year - 1 + 36))
        );
        assert_eq!(
            utc_from_tai_line(new_year + 36, true),
            Ok(alloc::format!("{new_year}\t1\n"))
        );
        assert_eq!(
            utc_from_tai_line(new_year + 37, true),
            Ok(alloc::format!("{new_year}\t0\n"))
        );
        // Before 1961 the strict policy has no table to read.
        assert_eq!(tai_from_unix(-400_000_000, true), Err(Refusal::NoData));
        assert_eq!(tai_from_unix(i64::MAX, false), Err(Refusal::Overflow));
    }

    /// Bernstein's `tai64n` page: POSIX 0 on an ordinary clock is
    /// `@400000000000000a`, ten more than the true-TAI label of 1970 TAI.
    #[test]
    fn the_posix_plus_10_convention_is_its_own_pair() {
        assert_eq!(
            tai64_posix_plus_10_encode_line(0, 0, "tai64").as_deref(),
            Ok("400000000000000a\n")
        );
        assert_eq!(
            tai64_posix_plus_10_encode_line(1, 500_000_000_000_000_000, "TAI64N").as_deref(),
            Ok("400000000000000b1dcd6500\n")
        );
        assert_eq!(
            tai64_posix_plus_10_decode_line("400000000000000A").as_deref(),
            Ok("tai64\t0\t0\n")
        );
        assert_eq!(
            tai64_posix_plus_10_decode_line("400000000000000b1dcd6500").as_deref(),
            Ok("tai64n\t1\t500000000000000000\n")
        );
        // The same label read as true TAI is another instant.
        assert_eq!(
            tai64_decode_line("400000000000000a").as_deref(),
            Ok("tai64\t10\t0\n")
        );
        assert_eq!(
            tai64_posix_plus_10_encode_line(0, 0, "tai64na"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            tai64_posix_plus_10_decode("400000000000000a00000000000000000000"),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            tai64_posix_plus_10_decode("3fffffffffffffff3b9ac9ff3b9ac9ff"),
            Err(Refusal::Malformed),
            "TAI64NA is not written in this convention"
        );
        assert_eq!(
            tai64_posix_plus_10_decode("8000000000000000"),
            Err(Refusal::OutOfRange)
        );
    }

    /// RFC 9562, Appendix A: the version 1 and version 6 test vectors
    /// carry 138 648 505 420 000 000 intervals, POSIX 1 645 557 742.
    #[test]
    fn the_rfc_9562_vectors_give_their_timestamp() {
        let expected = "\t138648505420000000\t1645557742\t0\n";
        assert_eq!(
            uuid_timestamp_line("C232AB00-9414-11EC-B3C8-9F6BDECED846"),
            Ok(alloc::format!("1{expected}"))
        );
        assert_eq!(
            uuid_timestamp_line("urn:uuid:1ec9414c-232a-6b00-b3c8-9f6bdeced846"),
            Ok(alloc::format!("6{expected}"))
        );
        assert_eq!(
            uuid_timestamp_line("1EC9414C232A6B00B3C89F6BDECED846"),
            Ok(alloc::format!("6{expected}"))
        );
        // Appendix A.3's version 4 vector carries no time.
        assert_eq!(
            uuid_timestamp("919108f7-52d1-4320-9bac-f847db4148a8"),
            Err(Refusal::NoData)
        );
        assert_eq!(
            uuid_timestamp("C232AB00-9414-11EC-B3C8-9F6BDECED84"),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            uuid_timestamp("C232AB0-09414-11EC-B3C8-9F6BDECED846"),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            uuid_timestamp("G232AB00-9414-11EC-B3C8-9F6BDECED846"),
            Err(Refusal::Malformed)
        );
    }

    /// RFC 5905's Figure 4 puts 8 February 2036 at era 1, offset 63 104:
    /// read against 2030 the timestamp is that day, against 1920 the same
    /// offset into era 0, 1 January 1900.
    /// RFC 9562 Appendix A.1 and A.5 (`rfc9562`): Tuesday 22 February 2022
    /// 2:22:22 PM GMT-05:00 is the timestamp 0x1EC9414C232AB00, written
    /// `C232AB00-9414-11EC-…` in version 1 and `1EC9414C-232A-6B00-…` in
    /// version 6.
    #[test]
    fn the_rfc_9562_instant_encodes_to_its_vectors_time_fields() {
        assert_eq!(
            uuid_timestamp_encode_line(1_645_557_742, 0).as_deref(),
            Ok("138648505420000000\tc232ab00-9414-11ec\t1ec9414c-232a-6b00\n")
        );
        // A sub-tick fraction is floored into its 100 ns interval.
        assert_eq!(
            uuid_timestamp_encode(1_645_557_742, 99_999_999_999).map(|parts| parts.0),
            Ok(138_648_505_420_000_000)
        );
        // The count begins on 1582-10-15 and ends on 5236-03-31.
        assert_eq!(
            uuid_timestamp_encode(-12_219_292_800, 0).map(|parts| parts.0),
            Ok(0)
        );
        assert_eq!(
            uuid_timestamp_encode(-12_219_292_801, 0),
            Err(Refusal::OutOfRange)
        );
        let (last, _, _) = uuid_timestamp_encode(103_072_857_660, 684_697_500_000_000_000)
            .expect("the last interval");
        assert_eq!(last, uuid::MAX_TIMESTAMP);
        assert_eq!(
            uuid_timestamp_encode(103_072_857_660, 684_697_600_000_000_000),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            uuid_timestamp_encode(0, 1_000_000_000_000_000_000),
            Err(Refusal::OutOfRange)
        );
        // The fields read back through the decoder.
        let (_, v1, v6) = uuid_timestamp_encode(1_645_557_742, 0).expect("in range");
        for fields in [v1, v6] {
            let uuid = alloc::format!("{fields}-8000-000000000000");
            assert_eq!(
                uuid_timestamp(&uuid).map(|parts| parts.1),
                Ok(138_648_505_420_000_000)
            );
        }
    }

    /// RFC 5905 Figure 4 (`rfc5905`): 1 January 1970 is era 0, offset
    /// 2 208 988 800; 15 October 1582 is era −3, offset 2 874 597 888;
    /// 31 December 1899 is era −1, offset 4 294 880 896; and 8 February
    /// 2036 is era 1, offset 63 104.
    #[test]
    fn a_posix_instant_encodes_to_the_ntp_dates_of_figure_4() {
        assert_eq!(
            ntp_encode_line(0, 0).as_deref(),
            Ok("0\t2208988800\t0\t0000000083aa7e800000000000000000\t83aa7e8000000000\n")
        );
        for (unix, era, offset) in [
            (-12_219_292_800, -3, 2_874_597_888),
            (-2_209_075_200, -1, 4_294_880_896),
            (2_086_041_600, 1, 63_104),
        ] {
            let date = ntp_encode(unix, 0).expect("in range");
            assert_eq!((date.era, date.offset), (era, offset), "{unix}");
        }
        // Half a second is 2⁶³ in the date and 2³¹ in the timestamp.
        assert_eq!(
            ntp_encode_line(0, 500_000_000_000_000_000).as_deref(),
            Ok(
                "0\t2208988800\t9223372036854775808\t0000000083aa7e808000000000000000\t83aa7e8080000000\n"
            )
        );
        // Era 1's timestamp is the same 64 bits as era 0's, the era dropped.
        let line = ntp_encode_line(2_086_041_600, 0).expect("in range");
        assert!(line.ends_with("\t0000f68000000000\n"), "{line}");
        assert!(ntp_encode(0, 1_000_000_000_000_000_000).is_err());
        assert_eq!(ntp_encode(i64::MAX, 0), Err(Refusal::Overflow));
        assert!(ntp_encode(i64::MAX - 2_208_988_800, 0).is_ok());
        assert!(ntp_encode(i64::MIN, 0).is_ok());
    }

    #[test]
    fn an_ntp_timestamp_is_placed_in_its_era_by_the_reference() {
        let in_2030 = 1_893_456_000;
        let in_1920 = -1_577_923_200;
        assert_eq!(
            ntp_resolve_line(63_104, 0, in_2030).as_deref(),
            Ok("1\t63104\t0\t2086041600\t0\n")
        );
        assert_eq!(
            ntp_resolve_line(63_104, 1 << 31, in_1920).as_deref(),
            Ok("0\t63104\t9223372036854775808\t-2208925696\t500000000000000000\n")
        );
        assert_eq!(ntp_resolve(0, 0, in_2030), Err(Refusal::NoData));
    }

    /// `docs/systems/binary-timestamps.md`'s worked example: 26 September
    /// 2026 at 23:59:58 is the date word 23 866 and the time word 49 021.
    #[cfg(feature = "format")]
    #[test]
    fn the_fat_words_of_the_last_even_second_of_26_september_2026() {
        let day = hc_calendars_solar::gregorian::to_fixed(2026, 9, 26)
            .expect("a date")
            .0;
        assert_eq!(
            fat_decode_line(23_866, 49_021),
            Ok(alloc::format!("{day}\t86398\n"))
        );
        assert_eq!(
            fat_encode_line(day, 86_399).as_deref(),
            Ok("23866\t49021\n")
        );
        assert_eq!(fat_encode(day, 86_400), Err(Refusal::OutOfRange));
        let first = hc_calendars_solar::gregorian::to_fixed(1980, 1, 1)
            .expect("a date")
            .0;
        assert_eq!(fat_encode(first, 0), Ok((33, 0)));
        assert_eq!(fat_encode(first - 1, 0), Err(Refusal::OutOfRange));
        // Month 0, and 30 February 2026.
        assert_eq!(fat_decode(46 << 9 | 1, 0), Err(Refusal::InvalidDate));
        assert_eq!(
            fat_decode(46 << 9 | 2 << 5 | 30, 0),
            Err(Refusal::InvalidDate)
        );
        assert_eq!(fat_decode(23_866, 24 << 11), Err(Refusal::InvalidDate));
        assert_eq!(fat_decode(65_536, 0), Err(Refusal::OutOfRange));
    }

    /// Wikipedia's example, @248 is 04:57:07.2 UTC; midnight BMT is 23:00
    /// UTC, and the POSIX epoch is @041.
    #[test]
    fn swatch_beats_are_read_on_biel_mean_time() {
        let at_248 = (4 * 60 + 57) * 60 + 7;
        assert_eq!(swatch_beat(at_248, 200_000_000_000_000_000), Ok(248));
        assert_eq!(swatch_beat(at_248, 199_999_999_999_999_999), Ok(247));
        assert_eq!(swatch_beat(-3_600, 0), Ok(0));
        assert_eq!(swatch_beat(0, 0), Ok(41));
        assert_eq!(
            swatch_beat(0, 1_000_000_000_000_000_000),
            Err(Refusal::OutOfRange)
        );
    }

    /// SOFA's Time Scale and Calendar Tools, §2.4: JD 2457073.05631 TT is
    /// J2015.1349933196 and B2015.1365941021. From JD 2440587.5, 1970 TT,
    /// that is 1 424 352 065.184 s.
    #[test]
    fn sofas_example_epochs() {
        let (seconds, attos) = (1_424_352_065, 184_000_000_000_000_000);
        let (kind, julian) = epoch_from_tt("J", seconds, attos).expect("an epoch");
        assert_eq!(kind, EpochKind::Julian);
        assert!((julian - 2_015.134_993_319_6).abs() < 1e-10, "{julian}");
        let (_, besselian) = epoch_from_tt("besselian-epoch", seconds, attos).expect("an epoch");
        assert!(
            (besselian - 2_015.136_594_102_1).abs() < 1e-10,
            "{besselian}"
        );
        assert_eq!(
            epoch_from_tt_line("j", 946_728_000, 0).as_deref(),
            Ok("J\t2000\n")
        );
        assert_eq!(
            tt_from_epoch_line("J", 2000.0).as_deref(),
            Ok("J\t946728000\t0\n")
        );
        let (kind, back, _) = tt_from_epoch("B", besselian).expect("an instant");
        assert_eq!(kind, EpochKind::Besselian);
        assert!((back - seconds).abs() <= 1, "{back}");
        // Without a letter, 1950.0 is Besselian and 2000.0 Julian.
        assert_eq!(
            tt_from_epoch("", 1950.0).map(|(kind, ..)| kind),
            Ok(EpochKind::Besselian)
        );
        assert_eq!(
            tt_from_epoch(" ", 2000.0).map(|(kind, ..)| kind),
            Ok(EpochKind::Julian)
        );
        assert_eq!(epoch_from_tt("", 0, 0), Err(Refusal::Unknown));
        assert_eq!(tt_from_epoch("Q", 2000.0), Err(Refusal::Unknown));
        assert_eq!(tt_from_epoch("J", f64::NAN), Err(Refusal::OutOfRange));
    }

    /// The ranges the READMEs state: every timestamp resolves against a
    /// reference 2³¹ s inside the `i64` range, and the posix-plus-10
    /// labels run from −(2⁶² + 10) to 2⁶² − 11 POSIX seconds.
    #[test]
    fn the_stated_ranges_hold_at_their_ends() {
        let (low, high) = (-9_223_372_034_707_292_160, 9_223_372_032_498_303_360);
        for seconds in [1, 1 << 31, u32::MAX] {
            for fraction in [0, u32::MAX] {
                assert!(ntp_resolve(seconds, fraction, low).is_ok(), "{seconds}");
                assert!(ntp_resolve(seconds, fraction, high).is_ok(), "{seconds}");
            }
        }
        // Beyond them some timestamps still resolve and some do not: at the
        // bottom of the range, one that would lie 2³¹ s before the
        // reference falls off it.
        assert!(ntp_resolve(1, 0, i64::MIN).is_ok());
        assert!(ntp_resolve(61_505_152, 0, i64::MIN).is_err());
        assert!(ntp_resolve(1, 0, i64::MAX).is_err());
        let (first, last) = (-4_611_686_018_427_387_914, 4_611_686_018_427_387_893);
        assert_eq!(
            tai64_posix_plus_10_encode_line(first, 0, "tai64").as_deref(),
            Ok("0000000000000000\n")
        );
        assert_eq!(
            tai64_posix_plus_10_encode_line(last, 999_999_999_999_999_999, "tai64").as_deref(),
            Ok("7fffffffffffffff\n")
        );
        assert_eq!(
            tai64_posix_plus_10_encode_line(first - 1, 0, "tai64"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            tai64_posix_plus_10_encode_line(last + 1, 0, "tai64"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            tai64_posix_plus_10_decode_line("7fffffffffffffff"),
            Ok(alloc::format!("tai64\t{last}\t0\n"))
        );
        assert_eq!(
            tai_from_unix(i64::MAX - 37, false).map(|(seconds, _)| seconds),
            Ok(i64::MAX)
        );
        assert_eq!(tai_from_unix(i64::MAX - 36, false), Err(Refusal::Overflow));
    }
}
