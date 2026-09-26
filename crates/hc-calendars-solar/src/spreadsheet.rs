//! The Excel 1900 date system, and the fractional day numbers of the OLE
//! Automation date and MATLAB's `datenum`.
//!
//! Excel's 1900 system counts serial 1 as 1 January 1900 and gives serial
//! 60 to 29 February 1900, a day that never was: Lotus 1-2-3 treated 1900
//! as a leap year, and Excel kept the count for compatibility. Every
//! serial from 61 is therefore one more than a true count from the same
//! epoch would give. [`Excel1900Calendar`] refuses serial 60 rather than
//! invent a date for it; [`excel_1900_day`] names it instead, for a caller
//! who has to read one.
//!
//! The Excel 1904 system and the OLE Automation date are linear, and are
//! [`crate::day_counts::EXCEL_1904`] and
//! [`crate::day_counts::OLE_AUTOMATION`]; [`ole_automation`] and
//! [`to_ole_automation`] read and write the OLE date's fractional time,
//! which a negative value carries as a magnitude.
//!
//! [`matlab_datenum`] and [`to_matlab_datenum`] do the same for MATLAB's
//! serial date number, whose whole part is
//! [`crate::day_counts::MATLAB`] and whose fraction is the time of day.
//!
//! The spreadsheet systems, the worked examples and the sources are in
//! `docs/systems/spreadsheet-dates.md`; MATLAB's in
//! `docs/systems/statistical-software-dates.md`.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, DayBoundary, Rd,
    YearKind,
};
use hc_core::Duration;

use crate::day_counts::{DayCountCalendar, DayNumber, MATLAB, OLE_AUTOMATION};
use crate::gregorian;

/// Serial 60, the 29 February 1900 that Excel counts and the calendar
/// does not have.
pub const PHANTOM_SERIAL: i64 = 60;

/// The last serial Excel's 1900 system supports, 31 December 9999.
pub const LAST_SERIAL: i64 = 2_958_465;

/// 31 December 1899, the day before serial 1: serials 1 to 59 count from
/// it.
const BEFORE_SERIAL_1: Rd = match gregorian::to_fixed(1899, 12, 31) {
    Ok(rd) => rd,
    Err(_) => panic!("a real date"),
};

/// 30 December 1899: serials from 61 count from it, a day earlier, because
/// serial 60 took a day.
const BEFORE_SERIAL_61: Rd = match gregorian::to_fixed(1899, 12, 30) {
    Ok(rd) => rd,
    Err(_) => panic!("a real date"),
};

/// What an Excel 1900 serial names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Excel1900Day {
    /// A day that exists.
    Date(Rd),
    /// Serial 60, which Excel displays as 29 February 1900. No day
    /// corresponds to it.
    Phantom29February1900,
}

/// What an Excel 1900 serial names, with serial 60 named rather than
/// refused.
///
/// # Errors
///
/// [`CalendarError::BeforeEpoch`] below serial 1 and
/// [`CalendarError::AfterSupportedRange`] above [`LAST_SERIAL`].
pub const fn excel_1900_day(serial: i64) -> CalendarResult<Excel1900Day> {
    if serial < 1 {
        return Err(CalendarError::BeforeEpoch);
    }
    if serial > LAST_SERIAL {
        return Err(CalendarError::AfterSupportedRange);
    }
    if serial < PHANTOM_SERIAL {
        Ok(Excel1900Day::Date(Rd(BEFORE_SERIAL_1.0 + serial)))
    } else if serial == PHANTOM_SERIAL {
        Ok(Excel1900Day::Phantom29February1900)
    } else {
        Ok(Excel1900Day::Date(Rd(BEFORE_SERIAL_61.0 + serial)))
    }
}

/// The Microsoft Excel 1900 date system, `excel-1900`: serial 1 is
/// 1 January 1900, and serial 60 is refused.
///
/// Microsoft Learn, "Excel incorrectly assumes that the year 1900 is a leap
/// year", read 2026-09-26 (`ms-excel-1900-leap`). The article also says the
/// WEEKDAY function "returns incorrect values for dates before March 1,
/// 1900": the serials from March include the phantom day, so a weekday
/// read off the serial in the cycle that holds from March is a day early
/// before it, Sunday for Monday 1 January 1900. This calendar's day is the
/// true one, so its weekday before March 1900 disagrees with Excel's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Excel1900Calendar;

impl Excel1900Calendar {
    /// The serial of a fixed day.
    ///
    /// # Errors
    ///
    /// [`CalendarError::BeforeEpoch`] before 1900 and
    /// [`CalendarError::AfterSupportedRange`] after 9999.
    pub fn serial_of(rd: Rd) -> CalendarResult<i64> {
        Self.meta().check_range(rd)?;
        let march_1900 = Rd(BEFORE_SERIAL_1.0 + PHANTOM_SERIAL);
        if rd < march_1900 {
            Ok(rd.0 - BEFORE_SERIAL_1.0)
        } else {
            Ok(rd.0 - BEFORE_SERIAL_61.0)
        }
    }
}

impl Calendar for Excel1900Calendar {
    type Date = DayNumber;

    /// A serial names nothing: it has no months and no week, only a number.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        &[]
    }

    /// A serial has no year: the `year` field carries the serial itself.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("excel-1900"),
            english_name: "Microsoft Excel 1900 date system",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(Rd(BEFORE_SERIAL_1.0 + 1)),
            latest: Some(Rd(BEFORE_SERIAL_61.0 + LAST_SERIAL)),
            native_locales: &[],
        }
    }

    fn day_boundary(&self) -> DayBoundary {
        DayBoundary::Midnight
    }

    /// # Errors
    ///
    /// [`CalendarError::DayOutOfRange`] for serial 60, which names no day,
    /// and the range errors outside serials 1 to [`LAST_SERIAL`].
    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        match excel_1900_day(date.0)? {
            Excel1900Day::Date(rd) => Ok(rd),
            Excel1900Day::Phantom29February1900 => Err(CalendarError::DayOutOfRange),
        }
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        Self::serial_of(rd).map(DayNumber)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        // As in `crate::day_counts`: the serial goes in `year`.
        let rd = self.to_fixed(date)?;
        DateFields::new(date.0).with_extra("julian-day-number", rd.to_julian_day_number())
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.day.is_some() {
            return Err(CalendarError::UnsupportedField("day"));
        }
        Ok(DayNumber(fields.year))
    }
}

const SECONDS_PER_DAY: f64 = 86_400.0;

/// The day and the time of day of an OLE Automation date.
///
/// The integer part counts days from 30 December 1899 and the fraction is
/// the time of day, read as a magnitude when the value is negative:
/// Microsoft Learn, `DateTime.ToOADate`, gives −1.25 as 06:00 on
/// 29 December 1899 (`ms-tooadate`). So the values in (−1, 0) repeat the
/// times of 30 December that [0, 1) gives. The fraction is converted as
/// the `f64` carries it, without rounding.
///
/// # Errors
///
/// [`CalendarError::Overflow`] for a value that is not finite, and the
/// range errors outside 1 January 100 to 31 December 9999, the range
/// `ToOADate` states.
pub fn ole_automation(value: f64) -> CalendarResult<(Rd, Duration)> {
    if !value.is_finite() {
        return Err(CalendarError::Overflow);
    }
    let (first, last) = OLE_AUTOMATION.range();
    let days = hc_core::math::trunc(value);
    if days < (first.0 - OLE_AUTOMATION.epoch().0) as f64
        || days > (last.0 - OLE_AUTOMATION.epoch().0) as f64
    {
        return Err(if value < 0.0 {
            CalendarError::BeforeEpoch
        } else {
            CalendarError::AfterSupportedRange
        });
    }
    // In range, so an exact integer well inside `i64`.
    let rd = OLE_AUTOMATION.day_of(DayNumber(days as i64));
    // The fraction is at most 1 − 2⁻⁵³, and that times 86 400 rounds to a
    // value below 86 400, so the time of day is always below a day.
    let seconds = hc_core::math::abs(value - days) * SECONDS_PER_DAY;
    let time = Duration::from_secs_f64(seconds).map_err(|_| CalendarError::Overflow)?;
    Ok((rd, time))
}

/// The OLE Automation date of a day and a time of day, the inverse of
/// [`ole_automation`]: before 30 December 1899 the time is subtracted, so
/// 06:00 on 29 December 1899 is −1.25.
///
/// # Errors
///
/// [`CalendarError::DayOutOfRange`] for a time of day that is negative or
/// not below a day, and the range errors outside 1 January 100 to
/// 31 December 9999.
pub fn to_ole_automation(rd: Rd, time_of_day: Duration) -> CalendarResult<f64> {
    if time_of_day.is_negative() || time_of_day >= Duration::DAY {
        return Err(CalendarError::DayOutOfRange);
    }
    DayCountCalendar(OLE_AUTOMATION).meta().check_range(rd)?;
    let days = OLE_AUTOMATION.number_of(rd).0 as f64;
    let fraction = time_of_day.as_secs_f64() / SECONDS_PER_DAY;
    Ok(if days < 0.0 {
        days - fraction
    } else {
        days + fraction
    })
}

/// The day and the time of day of a MATLAB serial date number.
///
/// MathWorks, `datenum` (`mathworks-datenum`): "The whole part of the
/// serial date number corresponds to the date, and the fractional part
/// corresponds to the time of day", counted from "January 0, 0000", day 0.
/// The page gives no negative value and does not say how one would carry
/// its time of day, so a value below 0 is refused rather than read by the
/// OLE date's rule or by the floor. The fraction is converted as the `f64`
/// carries it, without rounding.
///
/// # Errors
///
/// [`CalendarError::Overflow`] for a value that is not finite or does not
/// fit, and [`CalendarError::BeforeEpoch`] for a negative one.
pub fn matlab_datenum(value: f64) -> CalendarResult<(Rd, Duration)> {
    if !value.is_finite() || value >= 9.0e15 {
        return Err(CalendarError::Overflow);
    }
    if value < 0.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    let days = hc_core::math::floor(value);
    // Non-negative and below 9 × 10¹⁵, so an exact integer in `i64`.
    let rd = MATLAB.day_of(DayNumber(days as i64));
    let seconds = (value - days) * SECONDS_PER_DAY;
    let time = Duration::from_secs_f64(seconds).map_err(|_| CalendarError::Overflow)?;
    Ok((rd, time))
}

/// The MATLAB serial date number of a day and a time of day, the inverse
/// of [`matlab_datenum`].
///
/// # Errors
///
/// [`CalendarError::DayOutOfRange`] for a time of day that is negative or
/// not below a day, and [`CalendarError::BeforeEpoch`] before "January 0,
/// 0000", whose values [`matlab_datenum`] refuses.
pub fn to_matlab_datenum(rd: Rd, time_of_day: Duration) -> CalendarResult<f64> {
    if time_of_day.is_negative() || time_of_day >= Duration::DAY {
        return Err(CalendarError::DayOutOfRange);
    }
    let days = MATLAB.number_of(rd).0;
    if days < 0 {
        return Err(CalendarError::BeforeEpoch);
    }
    Ok(days as f64 + time_of_day.as_secs_f64() / SECONDS_PER_DAY)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_counts::EXCEL_1904;

    fn day(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("exists")
    }

    /// Serial 1 is 1 January 1900, 59 is 28 February, 60 is the day that
    /// never was, and 61 is 1 March.
    #[test]
    fn serial_60_is_refused_and_named() {
        let excel = Excel1900Calendar;
        assert_eq!(excel.to_fixed(DayNumber(1)), Ok(day(1900, 1, 1)));
        assert_eq!(excel.to_fixed(DayNumber(59)), Ok(day(1900, 2, 28)));
        assert_eq!(
            excel.to_fixed(DayNumber(PHANTOM_SERIAL)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            excel_1900_day(PHANTOM_SERIAL),
            Ok(Excel1900Day::Phantom29February1900)
        );
        assert_eq!(excel.to_fixed(DayNumber(61)), Ok(day(1900, 3, 1)));
        assert_eq!(excel.from_fixed(day(1900, 2, 28)), Ok(DayNumber(59)));
        assert_eq!(excel.from_fixed(day(1900, 3, 1)), Ok(DayNumber(61)));
        assert!(
            gregorian::to_fixed(1900, 2, 29).is_err(),
            "the calendar has no 29 February 1900"
        );
        // Nothing maps to serial 60, so it cannot be reached from a day.
        for rd in day(1900, 1, 1).0..day(1900, 12, 31).0 {
            assert_ne!(excel.from_fixed(Rd(rd)), Ok(DayNumber(PHANTOM_SERIAL)));
        }
    }

    /// The ends: serial 1 and 9999-12-31, serial 2 958 465; serial 0,
    /// which Excel shows as 0 January 1900, is outside.
    #[test]
    fn the_range_is_serial_1_to_9999() {
        let excel = Excel1900Calendar;
        assert_eq!(
            excel.to_fixed(DayNumber(0)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            excel.to_fixed(DayNumber(LAST_SERIAL)),
            Ok(day(9999, 12, 31))
        );
        assert_eq!(
            excel.to_fixed(DayNumber(LAST_SERIAL + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            excel.from_fixed(day(1899, 12, 31)),
            Err(CalendarError::BeforeEpoch)
        );
        for serial in (1..=LAST_SERIAL).step_by(997).chain([59, 61]) {
            let rd = excel.to_fixed(DayNumber(serial)).expect("a real day");
            assert_eq!(excel.from_fixed(rd), Ok(DayNumber(serial)));
        }
    }

    /// Microsoft Support's example and the 1 462-day difference, which
    /// holds from 1904, where both systems have serials.
    #[test]
    fn the_1904_system_is_1462_behind() {
        assert_eq!(Excel1900Calendar::serial_of(day(2011, 7, 5)), Ok(40_729));
        let excel_1904 = DayCountCalendar(EXCEL_1904);
        for rd in (day(1904, 1, 1).0..day(9999, 12, 31).0).step_by(10_007) {
            let serial_1900 = Excel1900Calendar::serial_of(Rd(rd)).expect("in range");
            let serial_1904 = excel_1904.from_fixed(Rd(rd)).expect("in range").0;
            assert_eq!(serial_1900 - serial_1904, 1_462);
        }
        assert_eq!(Excel1900Calendar::serial_of(day(1904, 1, 1)), Ok(1_462));
    }

    /// Excel's WEEKDAY is wrong before 1 March 1900: a weekday read off
    /// the serial, continuing the cycle that holds from March, is one day
    /// early for serials 1 to 59. The true 1 January 1900 was a Monday.
    #[test]
    fn a_weekday_read_off_the_serial_is_a_day_early_before_march_1900() {
        let excel = Excel1900Calendar;
        // Rata Die counts weekdays with 0 as Sunday and 1 January 1 a Monday.
        let true_weekday = |serial: i64| {
            excel
                .to_fixed(DayNumber(serial))
                .expect("a real day")
                .0
                .rem_euclid(7)
        };
        assert_eq!(true_weekday(1), 1, "Monday");
        // From March the serial and the weekday keep step: serial ≡ weekday + k.
        let k = (61 - true_weekday(61)).rem_euclid(7);
        for serial in 61..61 + 400 {
            assert_eq!((serial - k).rem_euclid(7), true_weekday(serial));
        }
        for serial in 1..PHANTOM_SERIAL {
            assert_eq!(
                (serial - k).rem_euclid(7),
                (true_weekday(serial) - 1).rem_euclid(7)
            );
        }
    }

    /// Microsoft Learn's examples: 1.0 is midnight on 31 December 1899,
    /// 2.25 is 06:00 on 1 January 1900, −1.0 is midnight on 29 December
    /// 1899 and −1.25 is 06:00 on 29 December 1899.
    #[test]
    fn ole_automation_reads_a_negative_fraction_as_a_time_of_day() {
        let six = Duration::from_hours(6);
        for (value, date, time) in [
            (0.0, day(1899, 12, 30), Duration::ZERO),
            (1.0, day(1899, 12, 31), Duration::ZERO),
            (2.25, day(1900, 1, 1), six),
            (-1.0, day(1899, 12, 29), Duration::ZERO),
            (-1.25, day(1899, 12, 29), six),
        ] {
            assert_eq!(ole_automation(value), Ok((date, time)), "{value}");
            assert_eq!(to_ole_automation(date, time), Ok(value), "{value}");
        }
        // (−1, 0) repeats [0, 1): −0.25 is 06:00 on 30 December, written 0.25.
        assert_eq!(ole_automation(-0.25), Ok((day(1899, 12, 30), six)));
        assert_eq!(to_ole_automation(day(1899, 12, 30), six), Ok(0.25));
    }

    #[test]
    fn ole_automation_refuses_what_to_oadate_does() {
        assert_eq!(
            ole_automation(-657_434.0),
            Ok((day(100, 1, 1), Duration::ZERO))
        );
        assert_eq!(ole_automation(-657_435.0), Err(CalendarError::BeforeEpoch));
        assert_eq!(
            ole_automation(2_958_466.0),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(ole_automation(f64::NAN), Err(CalendarError::Overflow));
        assert_eq!(
            to_ole_automation(day(1899, 12, 30), Duration::DAY),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_ole_automation(day(99, 12, 31), Duration::ZERO),
            Err(CalendarError::BeforeEpoch)
        );
        // The largest fraction below a day stays on its day.
        let (rd, time) = ole_automation(1.0 - f64::EPSILON / 2.0).expect("in range");
        assert_eq!(rd, day(1899, 12, 30));
        assert!(time < Duration::DAY);
    }

    /// MathWorks' `datenum(2001,12,19)` is 731 204; noon of that day is
    /// 731 204.5 and 06:00 731 204.25, both exact in binary.
    #[test]
    fn matlab_datenum_carries_the_time_of_day_as_its_fraction() {
        let rd = day(2001, 12, 19);
        assert_eq!(matlab_datenum(731_204.0), Ok((rd, Duration::ZERO)));
        assert_eq!(
            matlab_datenum(731_204.5),
            Ok((rd, Duration::from_hours(12)))
        );
        assert_eq!(
            to_matlab_datenum(rd, Duration::from_hours(6)),
            Ok(731_204.25)
        );
        assert_eq!(
            to_matlab_datenum(rd, Duration::DAY),
            Err(CalendarError::DayOutOfRange)
        );
        // Day 0 is "January 0, 0000"; before it the page says nothing.
        assert_eq!(
            matlab_datenum(0.5),
            Ok((day(-1, 12, 31), Duration::from_hours(12)))
        );
        assert_eq!(matlab_datenum(-0.25), Err(CalendarError::BeforeEpoch));
        assert_eq!(
            to_matlab_datenum(day(-1, 12, 30), Duration::ZERO),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(matlab_datenum(f64::NAN), Err(CalendarError::Overflow));
    }
}
