//! The Coptic calendar.
//!
//! Twelve months of thirty days and a thirteenth month of five days, six in
//! every fourth year. That is the Egyptian civil year with the leap day the
//! decree of Canopus proposed in 238 BC and Augustus finally imposed in
//! 25 BC, which is why the Coptic year keeps step with the Julian one
//! exactly: 1 Thout falls on 29 August Julian, or 30 August in the year
//! before a Coptic leap year.
//!
//! The era is the Era of the Martyrs, *Anno Martyrum*, counted from the
//! accession of Diocletian: 1 Thout 1 A.M. is 29 August AD 284 in the Julian
//! calendar, [`EPOCH`].
//!
//! Because the calendar tracks the Julian year, it has drifted with it: the
//! Coptic new year has fallen on 11 September Gregorian since 1900, and will
//! move to 12 September in 2100. That drift is a fact about the calendar,
//! not an error in this implementation.
//!
//! Month names are not here. They differ by language and script — Coptic,
//! Arabic, transliterated — so they belong with the other locale data in
//! `hc-i18n`, and this module holds only the arithmetic.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::common;

/// The fixed day of 1 Thout 1 A.M., which is 284-08-29 in the Julian
/// calendar and 284-08-29 in no other.
pub const EPOCH: Rd = Rd(103_605);

/// The era code of the Coptic era, *Anno Martyrum*.
pub const ERA: &str = "AM";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 999_999;

/// Whether `year` is a Coptic leap year.
///
/// The sixth epagomenal day falls in the year before a Julian leap year, so
/// the test is on the remainder 3 rather than 0.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    common::coptic_style_is_leap(year)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=13`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    common::coptic_style_days_in_month(year, month)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(common::coptic_style_to_fixed(EPOCH.0, MIN_YEAR, 1, 1));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(common::coptic_style_to_fixed(EPOCH.0, MAX_YEAR + 1, 1, 1) - 1);

/// The fixed day of a Coptic date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    match common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(common::coptic_style_to_fixed(EPOCH.0, year, month, day))),
    }
}

/// The Coptic year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    Ok(common::coptic_style_from_fixed(EPOCH.0, rd.0))
}

/// A Coptic date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CopticDate {
    /// The year of the Era of the Martyrs, counting from 1.
    pub year: i64,
    /// The month, 1 through 13; month 13 is the epagomenal period.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl CopticDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        match to_fixed(year, month, day) {
            Err(error) => Err(error),
            Ok(_) => Ok(Self { year, month, day }),
        }
    }

    /// Whether this date falls in the short thirteenth month.
    #[must_use]
    pub const fn is_epagomenal(self) -> bool {
        self.month == 13
    }
}

/// The Coptic calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CopticCalendar;

impl Calendar for CopticCalendar {
    type Date = CopticDate;

    fn cycles(&self) -> Option<&'static [hc_calendar::shape::CycleShape]> {
        Some(hc_calendar::shape::WANDERING_THIRTEEN)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("coptic"),
            english_name: "Coptic",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(CopticDate { year, month, day })
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
        CopticDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{gregorian, julian};

    #[test]
    fn the_epoch_is_the_accession_of_diocletian() {
        // 1 Thout 1 A.M. is 29 August AD 284 in the Julian calendar.
        assert_eq!(julian::to_fixed(284, 8, 29), Ok(EPOCH));
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
        assert_eq!(from_fixed(EPOCH), Ok((1, 1, 1)));
    }

    #[test]
    fn the_new_year_tracks_the_julian_calendar_exactly() {
        // 1 Thout falls on 29 August Julian, except in the year before a
        // Coptic leap year, when the extra epagomenal day pushes it to the
        // 30th and the Julian leap day pulls it back.
        for year in 1500..1600i64 {
            let new_year = to_fixed(year, 1, 1).unwrap();
            let (_, month, day) = julian::from_fixed(new_year).unwrap();
            assert_eq!(month, 8, "Coptic year {year}");
            assert!(day == 29 || day == 30, "Coptic year {year} on 8-{day}");
        }
    }

    #[test]
    fn the_coptic_new_year_is_the_eleventh_of_september_in_our_century() {
        // Nayrouz has fallen on 11 September Gregorian since 1900 and falls
        // on 12 September in the year before a Coptic leap year.
        let new_year = to_fixed(1742, 1, 1).unwrap();
        let (gregorian_year, month, day) = gregorian::from_fixed(new_year).unwrap();
        assert_eq!((gregorian_year, month), (2025, 9));
        assert!(day == 11 || day == 12);
    }

    #[test]
    fn every_fourth_year_has_a_sixth_epagomenal_day() {
        assert!(is_leap_year(3));
        assert!(!is_leap_year(4));
        assert_eq!(days_in_month(3, 13), Some(6));
        assert_eq!(days_in_month(4, 13), Some(5));
        assert_eq!(days_in_year(3), 366);
        assert_eq!(days_in_year(4), 365);
        assert_eq!(days_in_month(4, 1), Some(30));
        assert_eq!(days_in_month(4, 14), None);
    }

    #[test]
    fn the_thirteenth_month_is_five_or_six_days_long() {
        assert!(to_fixed(3, 13, 6).is_ok());
        assert_eq!(to_fixed(4, 13, 6), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(4, 13, 0), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(4, 14, 1), Err(CalendarError::MonthOutOfRange));
        assert!(CopticDate::new(3, 13, 6).unwrap().is_epagomenal());
        assert!(!CopticDate::new(3, 12, 30).unwrap().is_epagomenal());
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=1_200_000).step_by(29) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = to_fixed(1, 1, 1).unwrap().0;
        let end = to_fixed(21, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn dates_before_the_epoch_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = CopticCalendar;
        for rd in (EARLIEST.0..=900_000).step_by(733) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("coptic"));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(1, 1, 1).with_era("BC")),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn the_dynamic_year_length_matches_the_leap_rule() {
        use hc_calendar::{DynAdapter, DynCalendar};

        let calendar = DynAdapter::new(CopticCalendar);
        assert_eq!(calendar.days_in_year(3), Ok(366));
        assert_eq!(calendar.days_in_year(4), Ok(365));
        assert_eq!(calendar.days_in_month(&DateFields::ymd(3, 13, 1)), Ok(6));
        assert_eq!(calendar.days_in_month(&DateFields::ymd(4, 13, 1)), Ok(5));
    }
}
