//! The Juche calendar of North Korea.
//!
//! Gregorian structure with the year counted from 1912, the birth year of
//! Kim Il-sung, which is Juche 1. The offset is the same as
//! [`crate::minguo`]'s by coincidence, not by kinship: two states picked the
//! same year to start counting from for unrelated reasons.
//!
//! The era was introduced by decree in 1997 and, in practice, is written
//! alongside the Gregorian year — "주체113(2024)". There is no era for years
//! before 1912: the decree does not define one, so this module refuses dates
//! before [`EARLIEST`] rather than inventing a "before Juche" convention.
//!
//! Reporting since 2024 suggests the era is being used less in official
//! media. That is a fact about usage, not about arithmetic, and this module
//! keeps converting either way.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, gregorian};

/// The Common Era year that Juche year zero would correspond to; Juche 1 is
/// therefore 1912.
pub const YEAR_OFFSET: i64 = -1_911;

/// The era code of the Juche era.
pub const ERA: &str = "juche";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = gregorian::MAX_YEAR + YEAR_OFFSET;

/// Whether `year` is a leap year, by the Gregorian rule on the corresponding
/// Common Era year.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year - YEAR_OFFSET)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    gregorian::days_in_month(year - YEAR_OFFSET, month)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    gregorian::days_in_year(year - YEAR_OFFSET)
}

/// The earliest fixed day this implementation converts, 1 January 1912.
pub const EARLIEST: Rd = match gregorian::to_fixed(MIN_YEAR - YEAR_OFFSET, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of a Juche date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    common::offset_to_fixed(year, month, day, YEAR_OFFSET)
}

/// The Juche year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] for any day before 1912.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    common::offset_from_fixed(rd, YEAR_OFFSET)
}

/// A Juche date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JucheDate {
    /// The year of the Juche era, counting from 1.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl JucheDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        to_fixed(year, month, day)?;
        Ok(Self { year, month, day })
    }

    /// The same day in the Common Era year numbering.
    #[must_use]
    pub const fn common_era_year(self) -> i64 {
        self.year - YEAR_OFFSET
    }
}

/// The Juche calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JucheCalendar;

impl Calendar for JucheCalendar {
    type Date = JucheDate;

    fn cycles(&self) -> Option<&'static [hc_calendar::shape::CycleShape]> {
        Some(hc_calendar::shape::SOLAR_TWELVE)
    }

    /// Introduced by decree in 1997. Years between 1912 and 1997 are
    /// computed backwards onto an era that did not yet exist.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(match gregorian::to_fixed(1997, 9, 9) {
            Ok(rd) => rd,
            Err(_) => EARLIEST,
        })
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("juche"),
            english_name: "Juche",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(gregorian::LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(JucheDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_era(ERA)
            .with_extra("common-era-year", date.common_era_year())
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        JucheDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn juche_one_is_nineteen_twelve() {
        assert_eq!(
            to_fixed(1, 1, 1),
            Ok(gregorian::to_fixed(1912, 1, 1).unwrap())
        );
        assert_eq!(EARLIEST, gregorian::to_fixed(1912, 1, 1).unwrap());
        assert_eq!(from_fixed(Rd(719_163)), Ok((59, 1, 1)));
        assert_eq!(JucheDate::new(113, 1, 1).unwrap().common_era_year(), 2024);
    }

    #[test]
    fn the_day_of_the_sun_is_the_fifteenth_of_april() {
        // Kim Il-sung's birthday, 15 April 1912, is Juche 1, 15 April: the
        // era starts with the year of his birth, not the day.
        assert_eq!(
            to_fixed(1, 4, 15),
            Ok(gregorian::to_fixed(1912, 4, 15).unwrap())
        );
    }

    #[test]
    fn there_is_no_era_before_juche_one() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(-1, 1, 1), Err(CalendarError::YearOutOfRange));
    }

    #[test]
    fn leap_years_follow_the_gregorian_rule() {
        assert!(is_leap_year(89)); // 2000 CE
        assert!(is_leap_year(113)); // 2024 CE
        assert!(!is_leap_year(114)); // 2025 CE
        assert_eq!(days_in_month(89, 2), Some(29));
        assert_eq!(days_in_year(113), 366);
    }

    #[test]
    fn every_day_since_the_epoch_round_trips() {
        for rd in (EARLIEST.0..=1_000_000).step_by(53) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
            assert!(year >= MIN_YEAR);
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = JucheCalendar;
        for rd in (EARLIEST.0..=900_000).step_by(367) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(
                fields.extra.get("common-era-year"),
                Some(date.common_era_year())
            );
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("juche"));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(113, 1, 1).with_era("roc")),
            Err(CalendarError::UnknownEra)
        );
    }
}
