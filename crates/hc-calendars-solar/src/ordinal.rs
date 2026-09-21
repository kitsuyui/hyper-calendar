//! ISO 8601 ordinal dates.
//!
//! An ordinal date names a day as a year and a day number in `1..=366` —
//! `2026-263`. ISO 8601 calls this the "ordinal date" and writes it
//! `YYYY-DDD`; the C library calls the same number `tm_yday` but counts it
//! from zero, and this module counts from one as the standard does.
//!
//! The year is the Gregorian year, not the ISO week-numbering year of
//! [`crate::iso_week`]: every ordinal date lies in the Gregorian year it
//! names, which is exactly the property week dates give up.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::gregorian;

/// An ISO 8601 ordinal date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrdinalDate {
    /// The astronomical Gregorian year: 1 BC is year 0.
    pub year: i64,
    /// The day of the year, 1 through 365 or 366.
    pub day_of_year: u16,
}

impl OrdinalDate {
    /// A validated ordinal date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when the day number does not
    /// exist in that year, and [`CalendarError::YearOutOfRange`] outside the
    /// Gregorian range.
    pub const fn new(year: i64, day_of_year: u16) -> CalendarResult<Self> {
        if year < gregorian::MIN_YEAR || year > gregorian::MAX_YEAR {
            return Err(CalendarError::YearOutOfRange);
        }
        if day_of_year == 0 || day_of_year > gregorian::days_in_year(year) {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(Self { year, day_of_year })
    }

    /// The equivalent Gregorian date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the ordinal date does not exist.
    pub const fn to_gregorian(self) -> CalendarResult<(i64, u8, u8)> {
        match to_fixed(self.year, self.day_of_year) {
            Err(error) => Err(error),
            Ok(rd) => gregorian::from_fixed(rd),
        }
    }
}

/// The fixed day of an ordinal date.
///
/// # Errors
///
/// Returns a [`CalendarError`] when the ordinal date does not exist.
pub const fn to_fixed(year: i64, day_of_year: u16) -> CalendarResult<Rd> {
    match OrdinalDate::new(year, day_of_year) {
        Err(error) => Err(error),
        Ok(_) => match gregorian::new_year(year) {
            Err(error) => Err(error),
            Ok(start) => Ok(Rd(start.0 + day_of_year as i64 - 1)),
        },
    }
}

/// The ordinal date of a fixed day.
///
/// # Errors
///
/// Returns a [`CalendarError`] when the day is outside the Gregorian range.
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u16)> {
    match gregorian::year_from_fixed(rd) {
        Err(error) => Err(error),
        Ok(year) => match gregorian::new_year(year) {
            Err(error) => Err(error),
            Ok(start) => Ok((year, (rd.0 - start.0 + 1) as u16)),
        },
    }
}

/// The ISO 8601 ordinal-date calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OrdinalCalendar;

impl Calendar for OrdinalCalendar {
    type Date = OrdinalDate;

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("iso8601-ordinal"),
            english_name: "ISO 8601 ordinal date",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(gregorian::EARLIEST),
            latest: Some(gregorian::LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.day_of_year)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, day_of_year) = from_fixed(rd)?;
        Ok(OrdinalDate { year, day_of_year })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::new(date.year).with_extra("day-of-year", i64::from(date.day_of_year))
    }

    /// The day number lives in an extra field because it does not fit the
    /// `day`-within-`month` shape. It defaults to 1 when absent, so the
    /// object-safe layer can still measure a year.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let day_of_year = fields.extra.get("day-of-year").unwrap_or(1);
        let day_of_year = u16::try_from(day_of_year).map_err(|_| CalendarError::DayOutOfRange)?;
        OrdinalDate::new(fields.year, day_of_year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_and_last_days_of_a_year_are_one_and_the_year_length() {
        assert_eq!(to_fixed(2026, 1), gregorian::to_fixed(2026, 1, 1));
        assert_eq!(to_fixed(2026, 365), gregorian::to_fixed(2026, 12, 31));
        assert_eq!(to_fixed(2024, 366), gregorian::to_fixed(2024, 12, 31));
        assert_eq!(from_fixed(Rd(719_163)), Ok((1970, 1)));
    }

    #[test]
    fn the_leap_day_shifts_every_later_day_number() {
        // 1 March is day 60 in an ordinary year and day 61 in a leap year.
        assert_eq!(to_fixed(2023, 60), gregorian::to_fixed(2023, 3, 1));
        assert_eq!(to_fixed(2024, 61), gregorian::to_fixed(2024, 3, 1));
        assert_eq!(
            OrdinalDate::new(2024, 60).unwrap().to_gregorian(),
            Ok((2024, 2, 29))
        );
    }

    #[test]
    fn day_three_hundred_and_sixty_six_only_exists_in_leap_years() {
        assert!(OrdinalDate::new(2024, 366).is_ok());
        assert_eq!(
            OrdinalDate::new(2023, 366),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(OrdinalDate::new(2023, 0), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            OrdinalDate::new(gregorian::MAX_YEAR + 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (-500_000..=1_500_000).step_by(43) {
            let (year, day_of_year) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, day_of_year), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = gregorian::to_fixed(1896, 1, 1).unwrap().0;
        let end = gregorian::to_fixed(1912, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, day_of_year) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, day_of_year), Ok(Rd(rd)));
            assert!((1..=366).contains(&day_of_year));
        }
    }

    #[test]
    fn ordinal_dates_agree_with_gregorian_day_of_year() {
        for year in [1900, 2000, 2023, 2024] {
            for month in 1..=12u8 {
                let length = gregorian::days_in_month(year, month).unwrap();
                for day in 1..=length {
                    let rd = gregorian::to_fixed(year, month, day).unwrap();
                    let expected = gregorian::day_of_year(year, month, day).unwrap();
                    assert_eq!(from_fixed(rd), Ok((year, expected)));
                }
            }
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = OrdinalCalendar;
        for rd in (-100_000..=900_000).step_by(397) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(
                fields.extra.get("day-of-year"),
                Some(i64::from(date.day_of_year))
            );
            assert_eq!(calendar.from_fields(&fields), Ok(date));
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("iso8601-ordinal"));
    }

    #[test]
    fn the_dynamic_year_length_is_the_gregorian_one() {
        use hc_calendar::{DynAdapter, DynCalendar};

        let calendar = DynAdapter::new(OrdinalCalendar);
        assert_eq!(calendar.days_in_year(2024), Ok(366));
        assert_eq!(calendar.days_in_year(2023), Ok(365));
        assert_eq!(calendar.is_leap_year(2024), Ok(true));
        assert_eq!(calendar.is_leap_year(1900), Ok(false));
    }
}
