//! The Minguo calendar (Republic of China era).
//!
//! Gregorian structure with the year counted from the founding of the
//! Republic on 1 January 1912, which is Minguo 1: 2026 CE is Minguo 115.
//! The calendar remains the official one in Taiwan, and was used on the
//! mainland until 1949.
//!
//! Unlike the Buddhist or Holocene eras, this one has a *before*: dates
//! earlier than 1912 are written 民國前 N 年, "N years before the Republic",
//! with 1911 CE being 民國前 1 年. There is no year zero, so this module
//! stores a signed year — 1911 CE is year 0 internally — and translates to
//! and from the two era codes [`ERA_REPUBLIC`] and [`ERA_BEFORE_REPUBLIC`]
//! at the field boundary, exactly as [`crate::julian`] does for BC and AD.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, gregorian};

/// The Common Era year that Minguo year zero corresponds to; Minguo 1 is
/// therefore 1912.
pub const YEAR_OFFSET: i64 = -1_911;

/// The era code for years of the Republic.
pub const ERA_REPUBLIC: &str = "roc";

/// The era code for years before the Republic, 民國前.
pub const ERA_BEFORE_REPUBLIC: &str = "broc";

/// The earliest signed year this implementation converts.
pub const MIN_YEAR: i64 = gregorian::MIN_YEAR + YEAR_OFFSET;

/// The latest signed year this implementation converts.
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

/// The fixed day of a Minguo date, in signed year numbering.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    common::offset_to_fixed(year, month, day, YEAR_OFFSET)
}

/// The Minguo year, month and day of a fixed day, in signed year numbering.
///
/// # Errors
///
/// Propagates the Gregorian range errors.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    common::offset_from_fixed(rd, YEAR_OFFSET)
}

/// A Minguo date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MinguoDate {
    /// The year of the Republic, signed: 1912 CE is 1, 1911 CE is 0, and
    /// 1910 CE is -1. The era form of year 0 is 民國前 1 年.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl MinguoDate {
    /// A validated date in signed year numbering.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        to_fixed(year, month, day)?;
        Ok(Self { year, month, day })
    }

    /// A validated date in era numbering, where the era year is always
    /// positive.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::UnknownEra`] for an era code other than
    /// [`ERA_REPUBLIC`] or [`ERA_BEFORE_REPUBLIC`], and
    /// [`CalendarError::YearOutOfRange`] for a non-positive era year.
    pub fn from_era(era: &str, year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        if year < 1 {
            return Err(CalendarError::YearOutOfRange);
        }
        let signed = match era {
            ERA_REPUBLIC => year,
            ERA_BEFORE_REPUBLIC => 1 - year,
            _ => return Err(CalendarError::UnknownEra),
        };
        Self::new(signed, month, day)
    }

    /// The era code and positive year of the two-era convention.
    #[must_use]
    pub const fn era_year(self) -> (&'static str, i64) {
        if self.year > 0 {
            (ERA_REPUBLIC, self.year)
        } else {
            (ERA_BEFORE_REPUBLIC, 1 - self.year)
        }
    }

    /// The same day in the Common Era year numbering.
    #[must_use]
    pub const fn common_era_year(self) -> i64 {
        self.year - YEAR_OFFSET
    }
}

/// The Minguo calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MinguoCalendar;

impl Calendar for MinguoCalendar {
    type Date = MinguoDate;

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    /// In use from the founding of the Republic on 1 January 1912. The
    /// 民國前 years before it are a back-count, which is what the name says.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(match gregorian::to_fixed(1912, 1, 1) {
            Ok(rd) => rd,
            Err(_) => Rd(0),
        })
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("roc"),
            english_name: "Minguo (Republic of China)",
            year_kind: YearKind::EraRelative,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(gregorian::EARLIEST),
            latest: Some(gregorian::LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(MinguoDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let (era, year) = date.era_year();
        DateFields::ymd(year, date.month, date.day)
            .with_era(era)
            .with_extra("signed-year", date.year)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        match fields.era {
            // Without an era the year is the signed one, which is what the
            // object-safe layer passes when it probes a year's length.
            None => MinguoDate::new(fields.year, month.ordinal, day),
            Some(era) => MinguoDate::from_era(era, fields.year, month.ordinal, day),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_republic_was_founded_on_minguo_one_january_one() {
        assert_eq!(
            to_fixed(1, 1, 1),
            Ok(gregorian::to_fixed(1912, 1, 1).unwrap())
        );
        assert_eq!(from_fixed(Rd(719_163)), Ok((59, 1, 1)));
        assert_eq!(MinguoDate::new(115, 1, 1).unwrap().common_era_year(), 2026);
    }

    #[test]
    fn years_before_the_republic_count_backwards_from_one() {
        // 1911 CE is 民國前 1 年 and 1900 CE is 民國前 12 年.
        let before = MinguoDate::from_era(ERA_BEFORE_REPUBLIC, 1, 1, 1).unwrap();
        assert_eq!(before.year, 0);
        assert_eq!(before.common_era_year(), 1911);
        let earlier = MinguoDate::from_era(ERA_BEFORE_REPUBLIC, 12, 1, 1).unwrap();
        assert_eq!(earlier.common_era_year(), 1900);
        assert_eq!(earlier.era_year(), (ERA_BEFORE_REPUBLIC, 12));
    }

    #[test]
    fn there_is_no_year_zero_in_the_era_convention() {
        assert_eq!(
            MinguoDate::from_era(ERA_REPUBLIC, 0, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            MinguoDate::from_era("showa", 1, 1, 1),
            Err(CalendarError::UnknownEra)
        );
        // The last day before the Republic and its first day are adjacent.
        let last = to_fixed(0, 12, 31).unwrap();
        let first = to_fixed(1, 1, 1).unwrap();
        assert_eq!(first.0, last.0 + 1);
    }

    #[test]
    fn leap_years_follow_the_gregorian_rule() {
        assert!(is_leap_year(89)); // 2000 CE
        assert!(!is_leap_year(-11)); // 1900 CE
        assert!(is_leap_year(113)); // 2024 CE
        assert_eq!(days_in_month(89, 2), Some(29));
        assert_eq!(days_in_year(89), 366);
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (-200_000..=1_000_000).step_by(47) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = MinguoCalendar;
        for rd in (-100_000..=900_000).step_by(419) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert!(fields.year > 0, "era years are always positive");
            assert_eq!(fields.extra.get("signed-year"), Some(date.year));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("roc"));
        assert_eq!(calendar.meta().year_kind, YearKind::EraRelative);
    }

    #[test]
    fn the_dynamic_year_length_matches_the_gregorian_one() {
        use hc_calendar::{DynAdapter, DynCalendar};

        let calendar = DynAdapter::new(MinguoCalendar);
        assert_eq!(calendar.days_in_year(113), Ok(366));
        assert_eq!(calendar.days_in_year(114), Ok(365));
        assert_eq!(calendar.is_leap_year(113), Ok(true));
    }
}
