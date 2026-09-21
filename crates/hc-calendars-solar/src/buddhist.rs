//! The Thai solar calendar (Buddhist Era).
//!
//! Gregorian structure with the year number raised by [`YEAR_OFFSET`]: 2026
//! CE is 2569 BE. Thailand adopted the Gregorian month structure in 1889 and
//! moved the start of the year to 1 January in 1941, so this calendar is the
//! modern Thai civil calendar and nothing older.
//!
//! # What this deliberately does not do
//!
//! * It does not model the years before 1941, when the Thai year began on
//!   1 April. A date between 1 January and 31 March in the years 1889 to
//!   1940 therefore carries a year number one higher here than the one
//!   printed on a Thai document of the time — the reason the Thai year 2483
//!   was only nine months long.
//! * It does not model the Burmese, Sinhalese, Khmer or Lao Buddhist eras,
//!   which use the same era name with different epochs and, in several
//!   cases, a lunisolar year.
//! * It says nothing about where the Buddhist era's own epoch comes from.
//!   The parinirvana is dated 544 or 543 BCE depending on the tradition, and
//!   the Thai reckoning is the one that makes the offset 543.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, gregorian};

/// How far the Buddhist Era runs ahead of the Common Era.
pub const YEAR_OFFSET: i64 = 543;

/// The era code of the Buddhist Era.
pub const ERA: &str = "BE";

/// The earliest year this implementation converts, so that the year number
/// is never zero or negative.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = gregorian::MAX_YEAR + YEAR_OFFSET;

/// Whether `year` is a leap year, by the Gregorian rule applied to the
/// corresponding Common Era year.
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

/// The earliest fixed day this implementation converts, the first day of
/// Buddhist year 1.
pub const EARLIEST: Rd = match gregorian::to_fixed(MIN_YEAR - YEAR_OFFSET, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of a Thai solar date.
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

/// The Thai solar year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] when the day precedes Buddhist
/// year 1, or the Gregorian range errors.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    common::offset_from_fixed(rd, YEAR_OFFSET)
}

/// A Thai solar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuddhistDate {
    /// The year of the Buddhist Era, counting from 1.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl BuddhistDate {
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

/// The Thai solar calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BuddhistCalendar;

impl Calendar for BuddhistCalendar {
    type Date = BuddhistDate;

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("buddhist"),
            english_name: "Thai Buddhist",
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
        Ok(BuddhistDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(ERA))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        BuddhistDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_year_offset_is_five_hundred_and_forty_three() {
        // The reference conversion everyone knows: 2026 CE is 2569 BE.
        assert_eq!(
            to_fixed(2569, 1, 1),
            Ok(gregorian::to_fixed(2026, 1, 1).unwrap())
        );
        assert_eq!(from_fixed(Rd(719_163)), Ok((2513, 1, 1)));
        assert_eq!(
            BuddhistDate::new(2569, 1, 1).unwrap().common_era_year(),
            2026
        );
    }

    #[test]
    fn the_constitution_of_1932_has_the_year_printed_on_it() {
        // The permanent constitution was promulgated on 10 December 2475 BE,
        // which is 10 December 1932 CE and still a public holiday.
        assert_eq!(
            to_fixed(2475, 12, 10),
            Ok(gregorian::to_fixed(1932, 12, 10).unwrap())
        );
    }

    #[test]
    fn leap_years_follow_the_gregorian_rule_on_the_common_era_year() {
        assert!(is_leap_year(2543)); // 2000 CE
        assert!(!is_leap_year(2443)); // 1900 CE
        assert!(is_leap_year(2567)); // 2024 CE
        assert_eq!(days_in_month(2543, 2), Some(29));
        assert_eq!(days_in_month(2443, 2), Some(28));
        assert_eq!(days_in_year(2543), 366);
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=1_000_000).step_by(41) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
            assert!(year >= MIN_YEAR);
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = BuddhistCalendar;
        for rd in (EARLIEST.0..=900_000).step_by(613) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("buddhist"));
    }

    #[test]
    fn dates_before_buddhist_year_one_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(2569, 2, 30), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            BuddhistCalendar.from_fields(&DateFields::ymd(2569, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn the_dynamic_year_length_matches_the_gregorian_one() {
        use hc_calendar::{DynAdapter, DynCalendar};

        let calendar = DynAdapter::new(BuddhistCalendar);
        assert_eq!(calendar.days_in_year(2567), Ok(366));
        assert_eq!(calendar.days_in_year(2566), Ok(365));
        assert_eq!(calendar.is_leap_year(2567), Ok(true));
    }
}
