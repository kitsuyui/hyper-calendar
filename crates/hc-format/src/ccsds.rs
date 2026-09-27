//! The CCSDS calendar codes: the Calendar Segmented Code (CCS) and the
//! ASCII codes A and B; and [`decode`], which reads any binary CCSDS code
//! from its preamble field.
//!
//! CCSDS 301.0-B-4, *Time Code Formats*, November 2010, read 2026-09-27
//! (`ccsds-301-0-b-4`), §§3.4–3.5 and annex A. All three codes are UTC and
//! "leap second corrections must be made": a second may be 60 on a day
//! that ends in an inserted leap second, which is checked against the table
//! in `hc_core::leap` when a code is read as an instant.
//!
//! - **CCS** is binary-coded decimal: the year in 16 bits, the month and
//!   day of the month or the day of the year, the hour, minute and second,
//!   and up to six octets of two decimal digits each, 10⁻² s to 10⁻¹² s.
//! - **ASCII A** is `YYYY-MM-DDThh:mm:ss.d→dZ` and **ASCII B**
//!   `YYYY-DDDThh:mm:ss.d→dZ`, with the `Z` optional and the time part
//!   truncated from the right as §3.5.1.3 allows. The calendar and time
//!   subsets used alone name a day or a time of day, not an instant, and
//!   are refused.
//!
//! The binary counts, CUC and CDS, and the preamble field are
//! `hc_core::ccsds`. `docs/systems/ccsds-time-codes.md` works the
//! standard's ASCII example through CCS and CDS, and an example of its own,
//! 2000-01-01T00:00:00 UTC, through CUC.

use core::fmt;

use hc_calendar::{CalendarError, CivilDateTime, CivilTime, Rd, gregorian};
use hc_core::ccsds::{CcsFormat, CcsVariation, CdsTime, CucTime, Octets, Preamble};
use hc_core::unix::UtcInstant;
use hc_core::{TimeError, leap};

use crate::error::{ErrorKind, FormatResult, ParseError, ParseResult, ValueError, ValueResult};

/// The first and last year the codes hold (annex A).
const YEARS: core::ops::RangeInclusive<i64> = 1..=9_999;

/// The UTC instant of a reading, with the leap second checked: 23:59:60
/// only on a day that ends in an inserted second, and 23:59:59 not on one
/// that ends in an omitted second.
///
/// # Errors
///
/// [`ValueError::Time`] with [`TimeError::OutOfRange`] for a second the day
/// does not have, and [`TimeError::AfterModelEnd`] for 23:59:60 past the
/// leap-second table.
pub(crate) fn checked_utc_instant(reading: CivilDateTime) -> ValueResult<UtcInstant> {
    let unix_day = reading.day.to_unix_days();
    let time = reading.time;
    if time.is_leap_second() {
        if leap::end_of_day_step(unix_day)? != 1 {
            return Err(TimeError::OutOfRange.into());
        }
    } else if (time.hour(), time.minute(), time.second()) == (23, 59, 59)
        && leap::end_of_day_step(unix_day) == Ok(-1)
    {
        return Err(TimeError::OutOfRange.into());
    }
    let seconds = unix_day
        .checked_mul(86_400)
        .and_then(|start| start.checked_add(time.since_midnight().whole_seconds() as i64))
        .ok_or(TimeError::Overflow)?;
    Ok(UtcInstant {
        unix_seconds: seconds,
        leap_second: time.is_leap_second(),
        subsec_attos: time.subsec_attos(),
    })
}

/// The UTC reading of an instant: its day and time of day, 23:59:60 for a
/// leap second.
fn reading_of(utc: UtcInstant) -> ValueResult<CivilDateTime> {
    let (seconds, leap_second) = if utc.leap_second {
        (utc.unix_seconds - 1, true)
    } else {
        (utc.unix_seconds, false)
    };
    let day = Rd::from_unix_days(seconds.div_euclid(86_400));
    let second_of_day = seconds.rem_euclid(86_400);
    let time = if leap_second {
        if second_of_day != 86_399 {
            return Err(TimeError::OutOfRange.into());
        }
        CivilTime::new(23, 59, 60, utc.subsec_attos)?
    } else {
        CivilTime::new(
            (second_of_day / 3_600) as u8,
            (second_of_day % 3_600 / 60) as u8,
            (second_of_day % 60) as u8,
            utc.subsec_attos,
        )?
    };
    Ok(CivilDateTime::new(day, time))
}

/// Attoseconds in one unit of the last of `digits` decimal places.
const fn decimal_unit(digits: u32) -> u64 {
    10u64.pow(18 - digits)
}

/// A CCS code: a UTC reading at the format's resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CcsTime {
    format: CcsFormat,
    reading: CivilDateTime,
}

impl CcsTime {
    /// The code of a UTC reading, the fraction of the second floored to the
    /// format's resolution.
    ///
    /// # Errors
    ///
    /// [`ValueError::Calendar`] with [`CalendarError::YearOutOfRange`]
    /// outside the years 1 to 9999, [`ValueError::Time`] with
    /// [`TimeError::OutOfRange`] for more than six subsecond octets or a
    /// leap second the day does not have, and [`TimeError::AfterModelEnd`]
    /// for 23:59:60 past the leap-second table.
    pub fn new(format: CcsFormat, reading: CivilDateTime) -> ValueResult<Self> {
        if format.subsecond_octets > 6 {
            return Err(TimeError::OutOfRange.into());
        }
        let (year, _, _) = gregorian::from_fixed(reading.day)?;
        if !YEARS.contains(&year) {
            return Err(CalendarError::YearOutOfRange.into());
        }
        let unit = decimal_unit(2 * u32::from(format.subsecond_octets));
        let time = reading.time;
        let floored = CivilTime::new(
            time.hour(),
            time.minute(),
            time.second(),
            time.subsec_attos() / unit * unit,
        )?;
        let reading = CivilDateTime::new(reading.day, floored);
        checked_utc_instant(reading)?;
        Ok(Self { format, reading })
    }

    /// The format.
    #[must_use]
    pub const fn format(self) -> CcsFormat {
        self.format
    }

    /// The UTC reading.
    #[must_use]
    pub const fn reading(self) -> CivilDateTime {
        self.reading
    }

    /// The UTC instant.
    ///
    /// # Errors
    ///
    /// None in practice: the leap second was checked when the code was
    /// built.
    pub fn to_utc_instant(self) -> ValueResult<UtcInstant> {
        checked_utc_instant(self.reading)
    }

    /// The code of a UTC instant.
    ///
    /// # Errors
    ///
    /// As [`CcsTime::new`].
    pub fn from_utc_instant(format: CcsFormat, utc: UtcInstant) -> ValueResult<Self> {
        Self::new(format, reading_of(utc)?)
    }

    /// A T-field read in a format the caller knows.
    ///
    /// # Errors
    ///
    /// [`ValueError::Time`] with [`TimeError::OutOfRange`] for a T-field of
    /// the wrong length, a nibble above 9, or a day-of-year segment whose
    /// top four bits are not zero; [`ValueError::Calendar`] for a month, day
    /// or time of day outside annex A's ranges; and as [`CcsTime::new`].
    pub fn decode(format: CcsFormat, t_field: &[u8]) -> ValueResult<Self> {
        if format.subsecond_octets > 6 || t_field.len() != format.t_field_octets() {
            return Err(TimeError::OutOfRange.into());
        }
        let digits = |bytes: &[u8]| -> ValueResult<u64> {
            bytes.iter().try_fold(0, |value, &byte| {
                let (high, low) = (byte >> 4, byte & 0x0F);
                if high > 9 || low > 9 {
                    return Err(ValueError::Time(TimeError::OutOfRange));
                }
                Ok(value * 100 + u64::from(high) * 10 + u64::from(low))
            })
        };
        let year = digits(&t_field[..2])? as i64;
        if !YEARS.contains(&year) {
            return Err(CalendarError::YearOutOfRange.into());
        }
        let day = match format.variation {
            CcsVariation::MonthOfYear => {
                let month = digits(&t_field[2..3])? as u8;
                let day = digits(&t_field[3..4])? as u8;
                gregorian::to_fixed(year, month, day)?
            }
            CcsVariation::DayOfYear => {
                // "The four most significant bits of this segment are not
                // used and are set to zero."
                if t_field[2] >> 4 != 0 {
                    return Err(TimeError::OutOfRange.into());
                }
                let day_of_year = digits(&t_field[2..4])?;
                if !(1..=u64::from(gregorian::days_in_year(year))).contains(&day_of_year) {
                    return Err(CalendarError::DayOutOfRange.into());
                }
                Rd(gregorian::new_year(year).0 + day_of_year as i64 - 1)
            }
        };
        let hour = digits(&t_field[4..5])? as u8;
        let minute = digits(&t_field[5..6])? as u8;
        let second = digits(&t_field[6..7])? as u8;
        let places = 2 * u32::from(format.subsecond_octets);
        let fraction = digits(&t_field[7..])?;
        let time = CivilTime::new(hour, minute, second, fraction * decimal_unit(places))?;
        Self::new(format, CivilDateTime::new(day, time))
    }

    /// A P-field naming a CCS code, and its T-field.
    ///
    /// # Errors
    ///
    /// As [`CcsTime::decode`], and [`ValueError::Time`] for a P-field that
    /// does not decode or names another code.
    pub fn decode_with_preamble(bytes: &[u8]) -> ValueResult<Self> {
        let (preamble, used) = Preamble::decode(bytes)?;
        match preamble {
            Preamble::Ccs(format) => Self::decode(format, &bytes[used..]),
            _ => Err(TimeError::OutOfRange.into()),
        }
    }

    /// The T-field, and the P-field before it when `with_preamble` is set.
    ///
    /// # Errors
    ///
    /// None in practice: the fields were checked when the code was built.
    pub fn encode(self, with_preamble: bool) -> ValueResult<Octets> {
        let mut out = if with_preamble {
            Preamble::Ccs(self.format).encode()?
        } else {
            Octets::new()
        };
        let bcd = |out: &mut Octets, value: u64, octets: u32| -> ValueResult<()> {
            for index in (0..octets).rev() {
                let pair = value / 100u64.pow(index) % 100;
                out.push((((pair / 10) << 4) | (pair % 10)) as u8)?;
            }
            Ok(())
        };
        let (year, month, day) = gregorian::from_fixed(self.reading.day)?;
        bcd(&mut out, year as u64, 2)?;
        match self.format.variation {
            CcsVariation::MonthOfYear => {
                bcd(&mut out, u64::from(month), 1)?;
                bcd(&mut out, u64::from(day), 1)?;
            }
            CcsVariation::DayOfYear => {
                bcd(
                    &mut out,
                    u64::from(gregorian::day_of_year(year, month, day)?),
                    2,
                )?;
            }
        }
        let time = self.reading.time;
        bcd(&mut out, u64::from(time.hour()), 1)?;
        bcd(&mut out, u64::from(time.minute()), 1)?;
        bcd(&mut out, u64::from(time.second()), 1)?;
        let octets = u32::from(self.format.subsecond_octets);
        bcd(
            &mut out,
            time.subsec_attos() / decimal_unit(2 * octets),
            octets,
        )?;
        Ok(out)
    }
}

/// A binary CCSDS code, read from its P-field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CcsdsCode<'a> {
    /// The Unsegmented Code.
    Cuc(CucTime),
    /// The Day Segmented Code.
    Cds(CdsTime),
    /// The Calendar Segmented Code.
    Ccs(CcsTime),
    /// A Level 3 or Level 4 code: its octets, which only its agency can
    /// read.
    AgencyDefined(&'a [u8]),
}

/// A binary code: a P-field and exactly the T-field it announces.
///
/// # Errors
///
/// [`ValueError::Time`] with [`TimeError::OutOfRange`] for a P-field that
/// does not decode or a T-field of another length, and as the decoders of
/// the code it names.
pub fn decode(bytes: &[u8]) -> ValueResult<CcsdsCode<'_>> {
    let (preamble, used) = Preamble::decode(bytes)?;
    let t_field = &bytes[used..];
    if t_field.len() != preamble.t_field_octets() {
        return Err(TimeError::OutOfRange.into());
    }
    Ok(match preamble {
        Preamble::Cuc(format) => CcsdsCode::Cuc(CucTime::decode(format, t_field)?),
        Preamble::Cds(format) => CcsdsCode::Cds(CdsTime::decode(format, t_field)?),
        Preamble::Ccs(format) => CcsdsCode::Ccs(CcsTime::decode(format, t_field)?),
        Preamble::AgencyDefined { .. } => CcsdsCode::AgencyDefined(t_field),
    })
}

/// Which ASCII code: A with month and day, B with the day of the year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsciiVariation {
    /// `YYYY-MM-DDThh:mm:ss.d→dZ` (§3.5.1.1).
    A,
    /// `YYYY-DDDThh:mm:ss.d→dZ` (§3.5.1.2).
    B,
}

/// How far to the right an ASCII code's time part runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsciiPrecision {
    /// `hh`.
    Hour,
    /// `hh:mm`.
    Minute,
    /// `hh:mm:ss`.
    Second,
    /// `hh:mm:ss.d…d`, with 1 to 18 digits.
    Fraction(u8),
}

/// An ASCII code: a UTC reading and how it was written.
///
/// A truncated code reads as the start of the span it names:
/// `1988-018T17:20` is 17:20:00.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AsciiTime {
    /// A or B.
    pub variation: AsciiVariation,
    /// The UTC reading.
    pub reading: CivilDateTime,
    /// How much of the time part is written.
    pub precision: AsciiPrecision,
    /// Whether the optional terminator `Z` follows.
    pub terminator: bool,
}

/// A cursor over the ASCII bytes of a code.
struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Cursor<'_> {
    fn error(&self, kind: ErrorKind) -> ParseError {
        ParseError::new(kind, self.at)
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.at).copied()
    }

    fn eat(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.at += 1;
            true
        } else {
            false
        }
    }

    fn literal(&mut self, byte: u8, text: &'static str) -> ParseResult<()> {
        if self.eat(byte) {
            Ok(())
        } else if self.peek().is_none() {
            Err(self.error(ErrorKind::UnexpectedEnd))
        } else {
            Err(self.error(ErrorKind::Literal(text)))
        }
    }

    /// Exactly `count` digits.
    fn digits(&mut self, count: u8) -> ParseResult<u32> {
        let mut value = 0;
        for _ in 0..count {
            match self.peek() {
                Some(byte @ b'0'..=b'9') => {
                    value = value * 10 + u32::from(byte - b'0');
                    self.at += 1;
                }
                None => return Err(self.error(ErrorKind::UnexpectedEnd)),
                Some(_) => return Err(self.error(ErrorKind::DigitCount(count))),
            }
        }
        Ok(value)
    }

    /// The number of digits that follow, without consuming them.
    fn digit_run(&self) -> usize {
        self.bytes[self.at..]
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count()
    }
}

/// Parse an ASCII code A or B.
///
/// # Errors
///
/// A [`ParseError`] at the byte where the text stops being a code: a
/// missing or wrong separator, a field of the wrong width, a field out of
/// annex A's range, a date that does not exist, a second 60 anywhere but
/// 23:59, a fraction of more than 18 digits, which this library cannot
/// hold, or a calendar or time subset used alone.
pub fn parse(text: &str) -> ParseResult<AsciiTime> {
    let mut cursor = Cursor {
        bytes: text.as_bytes(),
        at: 0,
    };
    if cursor.bytes.is_empty() {
        return Err(cursor.error(ErrorKind::Empty));
    }
    let year_at = cursor.at;
    let year = i64::from(cursor.digits(4)?);
    if year == 0 {
        return Err(ParseError::new(ErrorKind::OutOfRange("year"), year_at));
    }
    cursor.literal(b'-', "-")?;
    let date_at = cursor.at;
    let (variation, day) = if cursor.digit_run() == 3 {
        let day_of_year = cursor.digits(3)?;
        if !(1..=u32::from(gregorian::days_in_year(year))).contains(&day_of_year) {
            return Err(ParseError::new(
                ErrorKind::OutOfRange("day of year"),
                date_at,
            ));
        }
        (
            AsciiVariation::B,
            Rd(gregorian::new_year(year).0 + i64::from(day_of_year) - 1),
        )
    } else {
        let month = cursor.digits(2)? as u8;
        cursor.literal(b'-', "-")?;
        let day = cursor.digits(2)? as u8;
        let rd = gregorian::to_fixed(year, month, day)
            .map_err(|_| ParseError::new(ErrorKind::Invalid("date"), date_at))?;
        (AsciiVariation::A, rd)
    };
    cursor.literal(b'T', "T")?;
    let time_at = cursor.at;
    let hour = cursor.digits(2)? as u8;
    let mut minute = 0;
    let mut second = 0;
    let mut attos = 0;
    let mut precision = AsciiPrecision::Hour;
    if cursor.eat(b':') {
        minute = cursor.digits(2)? as u8;
        precision = AsciiPrecision::Minute;
        if cursor.eat(b':') {
            second = cursor.digits(2)? as u8;
            precision = AsciiPrecision::Second;
            if cursor.eat(b'.') {
                let count = cursor.digit_run();
                if count == 0 {
                    return Err(cursor.error(ErrorKind::Digit));
                }
                if count > 18 {
                    return Err(cursor.error(ErrorKind::Unrepresentable(
                        "a fraction finer than the attosecond",
                    )));
                }
                let digits = &cursor.bytes[cursor.at..cursor.at + count];
                cursor.at += count;
                attos = digits
                    .iter()
                    .fold(0u64, |value, byte| value * 10 + u64::from(byte - b'0'))
                    * decimal_unit(count as u32);
                precision = AsciiPrecision::Fraction(count as u8);
            }
        }
    }
    if hour > 23 {
        return Err(ParseError::new(ErrorKind::OutOfRange("hour"), time_at));
    }
    if minute > 59 {
        return Err(ParseError::new(
            ErrorKind::OutOfRange("minute"),
            time_at + 3,
        ));
    }
    if second > 60 {
        return Err(ParseError::new(
            ErrorKind::OutOfRange("second"),
            time_at + 6,
        ));
    }
    let time = CivilTime::new(hour, minute, second, attos).map_err(|_| {
        ParseError::new(
            ErrorKind::Invalid("a leap second outside 23:59"),
            time_at + 6,
        )
    })?;
    let terminator = cursor.eat(b'Z');
    if cursor.peek().is_some() {
        return Err(cursor.error(ErrorKind::TrailingText));
    }
    Ok(AsciiTime {
        variation,
        reading: CivilDateTime::new(day, time),
        precision,
        terminator,
    })
}

impl AsciiTime {
    /// The code of a UTC instant.
    ///
    /// # Errors
    ///
    /// [`ValueError::Calendar`] with [`CalendarError::YearOutOfRange`]
    /// outside the years 1 to 9999, and [`ValueError::Time`] for a
    /// fraction of 0 or more than 18 digits.
    pub fn from_utc_instant(
        utc: UtcInstant,
        variation: AsciiVariation,
        precision: AsciiPrecision,
        terminator: bool,
    ) -> ValueResult<Self> {
        if let AsciiPrecision::Fraction(digits) = precision
            && !(1..=18).contains(&digits)
        {
            return Err(TimeError::OutOfRange.into());
        }
        let reading = reading_of(utc)?;
        let (year, _, _) = gregorian::from_fixed(reading.day)?;
        if !YEARS.contains(&year) {
            return Err(CalendarError::YearOutOfRange.into());
        }
        Ok(Self {
            variation,
            reading: truncate(reading, precision)?,
            precision,
            terminator,
        })
    }

    /// The UTC instant: the start of the span the code names.
    ///
    /// # Errors
    ///
    /// [`ValueError::Time`] with [`TimeError::OutOfRange`] for a second the
    /// day does not have, and [`TimeError::AfterModelEnd`] for 23:59:60 past
    /// the leap-second table.
    pub fn to_utc_instant(&self) -> ValueResult<UtcInstant> {
        checked_utc_instant(self.reading)
    }

    /// Write the code.
    ///
    /// # Errors
    ///
    /// [`crate::FormatError::Unrepresentable`] for a year outside 1 to 9999,
    /// and [`crate::FormatError::Sink`] when the sink refuses.
    pub fn write<W: fmt::Write>(&self, out: &mut W) -> FormatResult<()> {
        let (year, month, day) = gregorian::from_fixed(self.reading.day)
            .map_err(|_| crate::FormatError::Unrepresentable("the day"))?;
        if !YEARS.contains(&year) {
            return Err(crate::FormatError::Unrepresentable(
                "a year outside 1 to 9999",
            ));
        }
        write!(out, "{year:04}-")?;
        match self.variation {
            AsciiVariation::A => write!(out, "{month:02}-{day:02}")?,
            AsciiVariation::B => {
                let day_of_year = gregorian::day_of_year(year, month, day)
                    .map_err(|_| crate::FormatError::Unrepresentable("the day"))?;
                write!(out, "{day_of_year:03}")?;
            }
        }
        let time = self.reading.time;
        write!(out, "T{:02}", time.hour())?;
        if self.precision != AsciiPrecision::Hour {
            write!(out, ":{:02}", time.minute())?;
        }
        if matches!(
            self.precision,
            AsciiPrecision::Second | AsciiPrecision::Fraction(_)
        ) {
            write!(out, ":{:02}", time.second())?;
        }
        if let AsciiPrecision::Fraction(digits) = self.precision {
            let digits = u32::from(digits.clamp(1, 18));
            let value = time.subsec_attos() / decimal_unit(digits);
            write!(out, ".{value:0width$}", width = digits as usize)?;
        }
        if self.terminator {
            out.write_char('Z')?;
        }
        Ok(())
    }
}

impl fmt::Display for AsciiTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f).map_err(|_| fmt::Error)
    }
}

/// A reading cut to the start of the span a precision names.
fn truncate(reading: CivilDateTime, precision: AsciiPrecision) -> ValueResult<CivilDateTime> {
    let time = reading.time;
    let (minute, second, attos) = match precision {
        AsciiPrecision::Hour => (0, 0, 0),
        AsciiPrecision::Minute => (time.minute(), 0, 0),
        AsciiPrecision::Second => (time.minute(), time.second(), 0),
        AsciiPrecision::Fraction(digits) => {
            let unit = decimal_unit(u32::from(digits.clamp(1, 18)));
            (
                time.minute(),
                time.second(),
                time.subsec_attos() / unit * unit,
            )
        }
    };
    Ok(CivilDateTime::new(
        reading.day,
        CivilTime::new(time.hour(), minute, second, attos)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString as _;
    use hc_core::unix::UnixTime;

    /// 1988-01-18T17:20:43.123456 UTC, the standard's example (§3.5.1).
    fn example() -> CivilDateTime {
        CivilDateTime::new(
            gregorian::to_fixed(1988, 1, 18).expect("exists"),
            CivilTime::new(17, 20, 43, 123_456_000_000_000_000).expect("valid"),
        )
    }

    const MICROSECONDS_A: CcsFormat = CcsFormat {
        variation: CcsVariation::MonthOfYear,
        subsecond_octets: 3,
    };
    const MICROSECONDS_B: CcsFormat = CcsFormat {
        variation: CcsVariation::DayOfYear,
        subsecond_octets: 3,
    };

    /// §3.5.1.1 and §3.5.1.2: `1988-01-18T17:20:43.123456Z` and
    /// `1988-018T17:20:43.123456Z`.
    #[test]
    fn the_standards_ascii_examples() {
        let a = parse("1988-01-18T17:20:43.123456Z").expect("code A");
        assert_eq!(a.variation, AsciiVariation::A);
        assert_eq!(a.reading, example());
        assert_eq!(a.precision, AsciiPrecision::Fraction(6));
        assert!(a.terminator);
        assert_eq!(a.to_string(), "1988-01-18T17:20:43.123456Z");
        let b = parse("1988-018T17:20:43.123456Z").expect("code B");
        assert_eq!(b.variation, AsciiVariation::B);
        assert_eq!(b.reading, example());
        assert_eq!(b.to_string(), "1988-018T17:20:43.123456Z");
        let utc = a.to_utc_instant().expect("an ordinary second");
        assert_eq!(utc.unix_seconds, 6_591 * 86_400 + 62_443);
        assert_eq!(
            AsciiTime::from_utc_instant(utc, AsciiVariation::B, AsciiPrecision::Fraction(6), true),
            Ok(b)
        );
        // The terminator is optional, and the time part may stop early.
        let bare = parse("1988-018T17:20").expect("truncated on the right");
        assert_eq!(bare.precision, AsciiPrecision::Minute);
        assert!(!bare.terminator);
        assert_eq!(bare.reading.time, CivilTime::hms(17, 20, 0).expect("valid"));
        assert_eq!(bare.to_string(), "1988-018T17:20");
        assert_eq!(
            parse("1988-01-18T17Z").expect("the hour").to_string(),
            "1988-01-18T17Z"
        );
    }

    /// The same instant in CCS: `53 19 88 01 18 17 20 43 12 34 56` and
    /// `5B 19 88 00 18 17 20 43 12 34 56`.
    #[test]
    fn the_ascii_example_in_ccs() {
        let a = [
            0x53, 0x19, 0x88, 0x01, 0x18, 0x17, 0x20, 0x43, 0x12, 0x34, 0x56,
        ];
        let b = [
            0x5B, 0x19, 0x88, 0x00, 0x18, 0x17, 0x20, 0x43, 0x12, 0x34, 0x56,
        ];
        for (bytes, format) in [(a, MICROSECONDS_A), (b, MICROSECONDS_B)] {
            let code = CcsTime::decode_with_preamble(&bytes).expect("valid");
            assert_eq!(code.format(), format);
            assert_eq!(code.reading(), example());
            assert_eq!(code.encode(true).expect("fits").as_slice(), bytes);
            assert_eq!(CcsTime::new(format, example()), Ok(code));
            assert_eq!(decode(&bytes), Ok(CcsdsCode::Ccs(code)));
        }
    }

    /// 2016-12-31 ended in an inserted second: 23:59:60.5 is a reading in
    /// every UTC code on that day and in none on the day before.
    #[test]
    fn the_2016_leap_second_in_every_utc_code() {
        let leap = UtcInstant {
            unix_seconds: 1_483_228_800,
            leap_second: true,
            subsec_attos: 500_000_000_000_000_000,
        };
        let text = "2016-366T23:59:60.5Z";
        let ascii = parse(text).expect("a leap second at 23:59");
        assert_eq!(ascii.to_utc_instant(), Ok(leap));
        assert_eq!(
            AsciiTime::from_utc_instant(leap, AsciiVariation::B, AsciiPrecision::Fraction(1), true)
                .expect("representable")
                .to_string(),
            text
        );
        let format = CcsFormat {
            variation: CcsVariation::MonthOfYear,
            subsecond_octets: 1,
        };
        let ccs = CcsTime::from_utc_instant(format, leap).expect("a real leap second");
        let bytes = ccs.encode(true).expect("fits");
        assert_eq!(
            bytes.as_slice(),
            [0x51, 0x20, 0x16, 0x12, 0x31, 0x23, 0x59, 0x60, 0x50]
        );
        assert_eq!(CcsTime::decode_with_preamble(bytes.as_slice()), Ok(ccs));
        assert_eq!(ccs.to_utc_instant(), Ok(leap));
        let cds = CdsTime::from_utc(
            hc_core::ccsds::CdsFormat {
                epoch: hc_core::ccsds::EpochLevel::Recommended,
                day: hc_core::ccsds::DaySegment::Bits16,
                submillisecond: hc_core::ccsds::Submillisecond::None,
            },
            leap,
        )
        .expect("a real leap second");
        assert_eq!((cds.day(), cds.millisecond()), (21_549, 86_400_500));
        // The day before has no second 60.
        let refused = ValueError::Time(TimeError::OutOfRange);
        assert_eq!(
            parse("2016-365T23:59:60Z")
                .expect("syntax")
                .to_utc_instant(),
            Err(refused)
        );
        let bytes = [0x51, 0x20, 0x16, 0x12, 0x30, 0x23, 0x59, 0x60, 0x00];
        assert_eq!(CcsTime::decode_with_preamble(&bytes), Err(refused));
        // Past the table it is not yet known.
        assert_eq!(
            parse("2040-12-31T23:59:60Z")
                .expect("syntax")
                .to_utc_instant(),
            Err(ValueError::Time(TimeError::AfterModelEnd))
        );
        assert!(
            parse("2040-12-31T23:59:59Z")
                .expect("syntax")
                .to_utc_instant()
                .is_ok()
        );
    }

    #[test]
    fn malformed_ascii_codes_are_refused_where_they_break() {
        for (text, offset) in [
            ("", 0),
            ("0000-001T00Z", 0),
            ("1988-01-32T00", 5),
            ("1988-367T00", 5),
            ("1987-366T00", 5),
            ("1988-018", 8),
            ("1988-018 17:20", 8),
            ("1988-018T24", 9),
            ("1988-018T17:60", 12),
            ("1988-018T17:20:61", 15),
            ("1988-018T12:00:60", 15),
            ("1988-018T17:20:43.", 18),
            ("1988-018T17:20:43.1234567890123456789", 18),
            ("1988-018T17:20:43Zx", 18),
            ("88-018T17", 2),
            ("1988-18T17", 7),
        ] {
            let error = parse(text).expect_err(text);
            assert_eq!(error.offset(), offset, "{text}: {error}");
        }
    }

    #[test]
    fn malformed_ccs_codes_are_refused() {
        let refused = Err(ValueError::Time(TimeError::OutOfRange));
        // A nibble above 9, a day-of-year segment with a top nibble, a
        // T-field one octet short, a P-field of another code.
        let mut bytes = [0x50, 0x19, 0x88, 0x01, 0x18, 0x17, 0x20, 0x4A];
        assert_eq!(CcsTime::decode_with_preamble(&bytes), refused);
        bytes = [0x58, 0x19, 0x88, 0x10, 0x18, 0x17, 0x20, 0x43];
        assert_eq!(CcsTime::decode_with_preamble(&bytes), refused);
        assert_eq!(CcsTime::decode_with_preamble(&bytes[..7]), refused);
        assert_eq!(
            CcsTime::decode_with_preamble(&[0x40, 0, 0, 0, 0, 0, 0]),
            refused
        );
        // Month 13, and 29 February 1900, which the Gregorian rule refuses.
        assert!(CcsTime::decode_with_preamble(&[0x50, 0x19, 0x88, 0x13, 0x01, 0, 0, 0]).is_err());
        assert!(CcsTime::decode_with_preamble(&[0x50, 0x19, 0x00, 0x02, 0x29, 0, 0, 0]).is_err());
        assert!(CcsTime::decode_with_preamble(&[0x50, 0x20, 0x00, 0x02, 0x29, 0, 0, 0]).is_ok());
        // Year 0000.
        assert!(CcsTime::decode_with_preamble(&[0x50, 0x00, 0x00, 0x01, 0x01, 0, 0, 0]).is_err());
    }

    /// A fraction of no digits or of more than 18, and a CCS format with
    /// more than six subsecond octets, are refused on every path.
    #[test]
    fn out_of_range_precisions_are_refused() {
        let refused = ValueError::Time(TimeError::OutOfRange);
        let utc = UtcInstant::from_unix(UnixTime::from_seconds(569_524_843));
        for digits in [0, 19, u8::MAX] {
            assert_eq!(
                AsciiTime::from_utc_instant(
                    utc,
                    AsciiVariation::A,
                    AsciiPrecision::Fraction(digits),
                    true
                ),
                Err(refused),
                "{digits} digits"
            );
        }
        for subsecond_octets in [7, u8::MAX] {
            let format = CcsFormat {
                variation: CcsVariation::MonthOfYear,
                subsecond_octets,
            };
            assert_eq!(CcsTime::new(format, example()), Err(refused));
            assert_eq!(CcsTime::from_utc_instant(format, utc), Err(refused));
            assert_eq!(CcsTime::decode(format, &[0; 14]), Err(refused));
        }
    }

    /// The last microsecond of every day from 0001 to 9999 round-trips in
    /// CCS, both variations, and in ASCII A and B: every day in a release
    /// build; in a debug one every 997th, every year's first and last, and
    /// every day that ends in an inserted leap second, whose 23:59:60.5
    /// round-trips too (docs/policy.md §7).
    #[test]
    fn every_day_round_trips() {
        let unix_day = |year: i64, month: u8, day: u8| {
            gregorian::to_fixed(year, month, day).expect("exists").0
                - gregorian::to_fixed(1970, 1, 1).expect("exists").0
        };
        let (first, last) = (unix_day(1, 1, 1), unix_day(9999, 12, 31));
        let leap_days: alloc::vec::Vec<i64> = hc_core::leap::steps()
            .filter(|(_, delta)| *delta > 0)
            .map(|(at, _)| at.div_euclid(86_400) - 1)
            .collect();
        let debug = cfg!(debug_assertions);
        let mut days: alloc::vec::Vec<i64> = (first..=last)
            .step_by(if debug { 997 } else { 1 })
            .collect();
        if debug {
            for year in 1..=9999 {
                days.extend([unix_day(year, 1, 1), unix_day(year, 12, 31)]);
            }
            days.extend(leap_days.iter().copied());
            days.sort_unstable();
            days.dedup();
        }
        let formats =
            [CcsVariation::MonthOfYear, CcsVariation::DayOfYear].map(|variation| CcsFormat {
                variation,
                subsecond_octets: 3,
            });
        let check = |utc: UtcInstant| {
            for format in formats {
                let code = CcsTime::from_utc_instant(format, utc).expect("in range");
                let bytes = code.encode(true).expect("fits");
                assert_eq!(decode(bytes.as_slice()), Ok(CcsdsCode::Ccs(code)));
                assert_eq!(code.to_utc_instant(), Ok(utc), "{utc:?}");
            }
            for variation in [AsciiVariation::A, AsciiVariation::B] {
                let code =
                    AsciiTime::from_utc_instant(utc, variation, AsciiPrecision::Fraction(6), true)
                        .expect("in range");
                assert_eq!(parse(&code.to_string()), Ok(code));
                assert_eq!(code.to_utc_instant(), Ok(utc), "{utc:?}");
            }
        };
        for day in days {
            check(UtcInstant::from_unix(
                UnixTime::new(day * 86_400 + 86_399, 999_999_000_000_000_000).expect("valid"),
            ));
            if leap_days.contains(&day) {
                check(UtcInstant {
                    unix_seconds: (day + 1) * 86_400,
                    leap_second: true,
                    subsec_attos: 500_000_000_000_000_000,
                });
            }
        }
    }

    /// Every resolution and both variations, both ways, over a sample of
    /// instants from 0001 to 9999.
    #[test]
    fn round_trips() {
        let instants = [
            UtcInstant::from_unix(UnixTime::from_seconds(-62_135_596_800)),
            UtcInstant::from_unix(UnixTime::new(0, 1).expect("valid")),
            UtcInstant::from_unix(
                UnixTime::new(1_789_968_605, 987_654_321_987_654_321).expect("valid"),
            ),
            UtcInstant::from_unix(
                UnixTime::new(253_402_300_799, 999_999_999_999_999_999).expect("valid"),
            ),
        ];
        for utc in instants {
            for variation in [CcsVariation::MonthOfYear, CcsVariation::DayOfYear] {
                for subsecond_octets in 0..=6 {
                    let format = CcsFormat {
                        variation,
                        subsecond_octets,
                    };
                    let code = CcsTime::from_utc_instant(format, utc).expect("in range");
                    let bytes = code.encode(true).expect("fits");
                    assert_eq!(bytes.len(), 1 + format.t_field_octets());
                    assert_eq!(decode(bytes.as_slice()), Ok(CcsdsCode::Ccs(code)));
                    let back = code.to_utc_instant().expect("valid");
                    assert_eq!(CcsTime::from_utc_instant(format, back), Ok(code));
                }
            }
            for variation in [AsciiVariation::A, AsciiVariation::B] {
                for precision in [
                    AsciiPrecision::Hour,
                    AsciiPrecision::Minute,
                    AsciiPrecision::Second,
                    AsciiPrecision::Fraction(1),
                    AsciiPrecision::Fraction(9),
                    AsciiPrecision::Fraction(18),
                ] {
                    for terminator in [false, true] {
                        let code =
                            AsciiTime::from_utc_instant(utc, variation, precision, terminator)
                                .expect("in range");
                        assert_eq!(parse(&code.to_string()), Ok(code));
                    }
                }
            }
        }
        let after_9999 = UtcInstant::from_unix(UnixTime::from_seconds(253_402_300_800));
        assert_eq!(
            CcsTime::from_utc_instant(MICROSECONDS_A, after_9999),
            Err(ValueError::Calendar(CalendarError::YearOutOfRange))
        );
    }

    /// An agency-defined code comes back as its octets; a T-field of the
    /// wrong length is refused.
    #[test]
    fn the_front_door() {
        let bytes = [0x65, 1, 2, 3, 4, 5, 6];
        assert_eq!(decode(&bytes), Ok(CcsdsCode::AgencyDefined(&bytes[1..])));
        assert_eq!(
            decode(&bytes[..6]),
            Err(ValueError::Time(TimeError::OutOfRange))
        );
        let cuc = [0x1C, 0x4E, 0xFF, 0xA2, 0x20];
        assert!(matches!(decode(&cuc), Ok(CcsdsCode::Cuc(_))));
    }
}
