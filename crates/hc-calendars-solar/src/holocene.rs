//! The Holocene calendar, or Human Era.
//!
//! Gregorian structure with ten thousand added to the year: 2026 CE is
//! 12026 HE. Cesare Emiliani proposed it in 1993 so that the start of the
//! calendar would sit near the beginning of the Holocene and of settled
//! agriculture, which has two practical effects and one rhetorical one:
//!
//! * every date in recorded human history has a positive year number, so
//!   comparing them is subtraction rather than case analysis;
//! * there is no year zero problem and no BC/AD switch inside the range
//!   anyone cares about;
//! * the choice of 10 000 is round, not measured. The Holocene actually
//!   begins about 11 700 years before 2000 CE, so the era's name is an
//!   approximation and its epoch is a convention.
//!
//! The offset is exactly 10 000 years, so the leap rule, the month lengths
//! and the weekday of any day are the Gregorian ones; only the year number
//! differs. This implementation runs from HE 1, which is 10000 BCE in
//! astronomical numbering, forward.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, gregorian};

/// How far the Human Era runs ahead of the Common Era.
pub const YEAR_OFFSET: i64 = 10_000;

/// The era code of the Human Era.
pub const ERA: &str = "HE";

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

/// The earliest fixed day this implementation converts, the first day of
/// HE 1.
pub const EARLIEST: Rd = match gregorian::to_fixed(MIN_YEAR - YEAR_OFFSET, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of a Holocene date.
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

/// The Holocene year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] for any day before HE 1.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    common::offset_from_fixed(rd, YEAR_OFFSET)
}

/// A Holocene date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HoloceneDate {
    /// The year of the Human Era, counting from 1.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl HoloceneDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        to_fixed(year, month, day)?;
        Ok(Self { year, month, day })
    }

    /// The same day in astronomical Common Era year numbering, where 1 BC
    /// is year 0.
    #[must_use]
    pub const fn common_era_year(self) -> i64 {
        self.year - YEAR_OFFSET
    }
}

/// The Holocene calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HoloceneCalendar;

impl Calendar for HoloceneCalendar {
    type Date = HoloceneDate;

    /// Unrecorded: Emiliani's proposal of 1993, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("holocene"),
            english_name: "Holocene (Human Era)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(gregorian::LATEST),
            native_locales: &[],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(HoloceneDate { year, month, day })
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
        HoloceneDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_year_offset_is_exactly_ten_thousand() {
        assert_eq!(
            to_fixed(11_970, 1, 1),
            Ok(gregorian::to_fixed(1970, 1, 1).unwrap())
        );
        assert_eq!(from_fixed(Rd(730_120)), Ok((12_000, 1, 1)));
        assert_eq!(
            HoloceneDate::new(12_026, 9, 20).unwrap().common_era_year(),
            2026
        );
    }

    #[test]
    fn the_era_starts_in_ten_thousand_bce() {
        // HE 1 is astronomical year -9999, which historians write 10000 BCE.
        assert_eq!(EARLIEST, gregorian::to_fixed(-9_999, 1, 1).unwrap());
        assert_eq!(from_fixed(EARLIEST), Ok((1, 1, 1)));
        assert_eq!(
            HoloceneDate::new(1, 1, 1).unwrap().common_era_year(),
            -9_999
        );
    }

    #[test]
    fn every_date_in_recorded_history_has_a_positive_year() {
        // The oldest dates anyone converts — the Egyptian epoch of 747 BC,
        // the Julian period epoch of 4713 BC — are positive Holocene years.
        for rd in [crate::egyptian::EPOCH, Rd::from_julian_day_number(0)] {
            let (year, _, _) = from_fixed(rd).unwrap();
            assert!(year > 0, "rd {rd} gave year {year}");
        }
    }

    #[test]
    fn leap_years_follow_the_gregorian_rule() {
        assert!(is_leap_year(12_000)); // 2000 CE
        assert!(!is_leap_year(11_900)); // 1900 CE
        assert!(is_leap_year(12_024)); // 2024 CE
        assert_eq!(days_in_month(12_000, 2), Some(29));
        assert_eq!(days_in_year(11_900), 365);
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=1_000_000).step_by(1_013) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
            assert!(year >= MIN_YEAR);
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = HoloceneCalendar;
        for rd in (-100_000..=900_000).step_by(331) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("holocene"));
    }

    #[test]
    fn dates_before_the_human_era_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(12_025, 2, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            HoloceneCalendar.from_fields(&DateFields::ymd(12_026, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }
}
