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

use alloc::string::String;
use core::fmt::Write;

use hc_calendar::{CalendarError, Rd};
use hc_calendars_solar::spreadsheet::{self, Excel1900Day};
use hc_core::gnss::{self, GlonassDate, GlonassTime, WeekNumbering, WeekTime};
use hc_core::tai64;
use hc_core::unix::LeapPolicy;
use hc_core::{Duration, Instant, Tai};

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
/// [`Refusal::OutOfRange`] for a second outside the labels below 2⁶³.
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
    let digits = hex.trim().as_bytes();
    let format = Tai64Format::ALL
        .into_iter()
        .find(|format| format.bytes() * 2 == digits.len())
        .ok_or(Refusal::Malformed)?;
    let mut bytes = [0u8; 16];
    let (pairs, _) = digits.as_chunks::<2>();
    for (byte, [high, low]) in bytes.iter_mut().zip(pairs) {
        let (Some(high), Some(low)) = (hex_digit(*high), hex_digit(*low)) else {
            return Err(Refusal::Malformed);
        };
        *byte = high << 4 | low;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
