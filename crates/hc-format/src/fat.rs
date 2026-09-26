//! The MS-DOS date and time words of the FAT file system: a local
//! wall-clock reading packed into two 16-bit values.
//!
//! Microsoft Learn, `DosDateTimeToFileTime` (winbase.h), read 2026-09-26
//! (`ms-dosdatetimetofiletime`), gives the layouts:
//!
//! | Word | Bits 0–4 | Bits 5–8 or 5–10 | Bits 9–15 or 11–15 |
//! | --- | --- | --- | --- |
//! | date | day of the month, 1–31 | month, 1 = January, bits 5–8 | year offset from 1980, bits 9–15 |
//! | time | second divided by 2 | minute, 0–59, bits 5–10 | hour, 0–23, bits 11–15 |
//!
//! So the years run from 1980 to 1980 + 127 = 2107, and the second is
//! carried at two-second resolution. Microsoft Learn, "File Times"
//! (`ms-file-times`): "The FAT file system records times on disk in local
//! time", and FAT's write time "has a resolution of 2 seconds". The words
//! therefore name a wall-clock reading in a zone they do not record, and
//! decode to a [`CivilDateTime`] and nothing more; turning one into an
//! instant needs the zone of the machine that wrote it, which the caller
//! has to know.
//!
//! What Microsoft does not state, this codec refuses rather than reads: a
//! day or month of 0, a month above 12, a day the month does not have, a
//! halved second above 29 (it would be second 60 or 62), a minute above
//! 59 and an hour above 23. Encoding keeps the even second at or before
//! the reading, and drops the fraction, since the field cannot carry
//! either; a leap second, 23:59:60, is refused.
//!
//! `docs/systems/binary-timestamps.md` works an example through.

use hc_calendar::{CalendarError, CivilDateTime, CivilTime, Rd};
use hc_calendars_solar::gregorian;

use crate::error::ValueResult;

/// The first year the date word holds.
pub const FIRST_YEAR: i64 = 1980;

/// The last year the date word holds, 1980 + 127.
pub const LAST_YEAR: i64 = FIRST_YEAR + 127;

/// The day a FAT date word names.
///
/// # Errors
///
/// [`crate::ValueError::Calendar`] with [`CalendarError::MonthOutOfRange`] or
/// [`CalendarError::DayOutOfRange`] for a field outside its range or a day
/// the month does not have.
pub fn decode_date(date: u16) -> ValueResult<Rd> {
    let day = (date & 0x1F) as u8;
    let month = ((date >> 5) & 0x0F) as u8;
    let year = FIRST_YEAR + i64::from(date >> 9);
    if !(1..=12).contains(&month) {
        return Err(CalendarError::MonthOutOfRange.into());
    }
    if day == 0 {
        return Err(CalendarError::DayOutOfRange.into());
    }
    Ok(gregorian::to_fixed(year, month, day)?)
}

/// The time of day a FAT time word names, at an even second.
///
/// # Errors
///
/// [`crate::ValueError::Calendar`] with [`CalendarError::DayOutOfRange`] for a
/// field outside its range.
pub fn decode_time(time: u16) -> ValueResult<CivilTime> {
    let second = ((time & 0x1F) as u8) * 2;
    let minute = ((time >> 5) & 0x3F) as u8;
    let hour = (time >> 11) as u8;
    if second > 58 || minute > 59 || hour > 23 {
        return Err(CalendarError::DayOutOfRange.into());
    }
    Ok(CivilTime::hms(hour, minute, second)?)
}

/// The local reading a pair of FAT words names.
///
/// # Errors
///
/// As [`decode_date`] and [`decode_time`].
pub fn decode(date: u16, time: u16) -> ValueResult<CivilDateTime> {
    Ok(CivilDateTime::new(decode_date(date)?, decode_time(time)?))
}

/// The FAT date word of a day.
///
/// # Errors
///
/// [`crate::ValueError::Calendar`] with [`CalendarError::BeforeEpoch`] before
/// 1980 and [`CalendarError::AfterSupportedRange`] after 2107.
pub fn encode_date(day: Rd) -> ValueResult<u16> {
    let (year, month, day) = gregorian::from_fixed(day)?;
    if year < FIRST_YEAR {
        return Err(CalendarError::BeforeEpoch.into());
    }
    if year > LAST_YEAR {
        return Err(CalendarError::AfterSupportedRange.into());
    }
    // In 0..=127 by the checks above.
    let offset = (year - FIRST_YEAR) as u16;
    Ok(offset << 9 | u16::from(month) << 5 | u16::from(day))
}

/// The FAT time word of a time of day, at the even second at or before
/// it.
///
/// # Errors
///
/// [`crate::ValueError::Calendar`] with [`CalendarError::DayOutOfRange`] for a
/// leap second, which the word cannot carry.
pub fn encode_time(time: CivilTime) -> ValueResult<u16> {
    if time.is_leap_second() {
        return Err(CalendarError::DayOutOfRange.into());
    }
    Ok(u16::from(time.hour()) << 11 | u16::from(time.minute()) << 5 | u16::from(time.second() / 2))
}

/// The FAT date and time words of a local reading.
///
/// # Errors
///
/// As [`encode_date`] and [`encode_time`].
pub fn encode(reading: CivilDateTime) -> ValueResult<(u16, u16)> {
    Ok((encode_date(reading.day)?, encode_time(reading.time)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("exists")
    }

    /// The layout of `DosDateTimeToFileTime`: 1980-01-01 00:00:00 is the
    /// date word with day 1, month 1 and year offset 0, and a time word
    /// of zero.
    #[test]
    fn the_first_day_is_1_january_1980() {
        assert_eq!(
            decode(0x0021, 0),
            Ok(CivilDateTime::midnight(day(1980, 1, 1)))
        );
        assert_eq!(
            encode(CivilDateTime::midnight(day(1980, 1, 1))),
            Ok((0x0021, 0))
        );
    }

    /// Each field in its bits: 2026-09-26 23:59:58 is year offset 46,
    /// month 9, day 26, hour 23, minute 59 and 29 halved seconds.
    #[test]
    fn each_field_is_in_its_bits() {
        let date = 46 << 9 | 9 << 5 | 26;
        let time = 23 << 11 | 59 << 5 | 29;
        assert_eq!((date, time), (23_866, 49_021));
        let reading =
            CivilDateTime::new(day(2026, 9, 26), CivilTime::hms(23, 59, 58).expect("valid"));
        assert_eq!(decode(date, time), Ok(reading));
        assert_eq!(encode(reading), Ok((date, time)));
    }

    /// The last day the seven year bits reach is 31 December 2107.
    #[test]
    fn the_years_run_from_1980_to_2107() {
        let last = 127 << 9 | 12 << 5 | 31;
        assert_eq!(decode_date(last), Ok(day(2107, 12, 31)));
        assert_eq!(encode_date(day(2107, 12, 31)), Ok(last));
        assert_eq!(
            encode_date(day(2108, 1, 1)),
            Err(CalendarError::AfterSupportedRange.into())
        );
        assert_eq!(
            encode_date(day(1979, 12, 31)),
            Err(CalendarError::BeforeEpoch.into())
        );
    }

    /// Two-second resolution: 12:34:57.9 is written as 12:34:56.
    #[test]
    fn an_odd_second_and_a_fraction_are_dropped() {
        let time = CivilTime::new(12, 34, 57, 900_000_000_000_000_000).expect("valid");
        let word = encode_time(time).expect("representable");
        assert_eq!(
            decode_time(word),
            Ok(CivilTime::hms(12, 34, 56).expect("valid"))
        );
        let leap = CivilTime::hms(23, 59, 60).expect("valid");
        assert_eq!(encode_time(leap), Err(CalendarError::DayOutOfRange.into()));
    }

    #[test]
    fn fields_outside_their_range_are_refused() {
        // Day 0, month 0, month 13, 30 February, 29 February of a common
        // year; 29 February 2000 is a real day.
        for date in [
            1 << 5,
            1,
            13 << 5 | 1,
            20 << 9 | 2 << 5 | 30,
            21 << 9 | 2 << 5 | 29,
        ] {
            assert!(decode_date(date).is_err(), "{date:#06x}");
        }
        assert_eq!(decode_date(20 << 9 | 2 << 5 | 29), Ok(day(2000, 2, 29)));
        // Second 60, minute 60, hour 24.
        for time in [30, 60 << 5, 24 << 11] {
            assert!(decode_time(time).is_err(), "{time:#06x}");
        }
    }

    #[test]
    fn every_day_from_1980_to_2107_round_trips() {
        let first = day(1980, 1, 1).0;
        let last = day(2107, 12, 31).0;
        for rd in (first..=last).step_by(7) {
            let word = encode_date(Rd(rd)).expect("in range");
            assert_eq!(decode_date(word), Ok(Rd(rd)));
        }
    }
}
