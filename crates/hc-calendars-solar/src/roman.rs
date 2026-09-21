//! Years *ab urbe condita*.
//!
//! The Julian calendar with the year counted from the founding of Rome as
//! Varro dated it, 753 BC: AD 1 is AUC 754, and AUC 1 is the year beginning
//! 1 January 753 BC in the proleptic Julian calendar.
//!
//! # What this deliberately does not do
//!
//! Two large things, and it is worth being explicit about both.
//!
//! * **It does not model the Roman republican calendar.** Before Caesar's
//!   reform of 45 BC the year had 355 days and an intercalary month,
//!   *Mercedonius*, inserted at the discretion of the pontifices — who
//!   inserted it late, early or not at all for political reasons, so that by
//!   46 BC the calendar was about three months out of step with the seasons.
//!   No arithmetic reconstructs that; the surviving dates have to be
//!   tabulated from inscriptions and the results are still disputed. What
//!   this module gives for a year before AUC 709 is the *proleptic Julian*
//!   date with an AUC year number, which is what modern editors mean when
//!   they write one, not what a Roman would have written.
//! * **It does not count days by kalends, nones and ides.** A Roman wrote
//!   *a.d. III Kal. Apr.*, counting backwards inclusively from the next
//!   named day, not "30 March". That is a formatting question rather than a
//!   calendar one, and it lives with the other presentation logic, in
//!   `hc_format::roman` — including the doubled sixth day before the
//!   Kalends of March that gives bissextile years their name.
//!
//! The AUC era was in any case rarely used for dating in antiquity — Romans
//! named years after the consuls. It is a convenience of later historians,
//! and it is implemented here for the same reason.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::julian;

/// How far *ab urbe condita* runs ahead of the astronomical Julian year:
/// AUC 1 is astronomical year -752, which historians write 753 BC.
pub const YEAR_OFFSET: i64 = 753;

/// The era code.
pub const ERA: &str = "AUC";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 99_999;

/// The year of Caesar's reform, after which the Julian structure this module
/// uses is the one Rome actually kept.
pub const JULIAN_REFORM_YEAR: i64 = 709;

/// Whether `year` is a leap year under the Julian rule.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    julian::is_leap_year(year - YEAR_OFFSET)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    julian::days_in_month(year - YEAR_OFFSET, month)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The fixed day of an *ab urbe condita* date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    julian::to_fixed(year - YEAR_OFFSET, month, day)
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = match to_fixed(MIN_YEAR, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = match to_fixed(MAX_YEAR, 12, 31) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The *ab urbe condita* year, month and day of a fixed day.
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
    match julian::from_fixed(rd) {
        Err(error) => Err(error),
        Ok((year, month, day)) => Ok((year + YEAR_OFFSET, month, day)),
    }
}

/// A date numbered *ab urbe condita*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RomanDate {
    /// The year from the founding of the city, counting from 1.
    pub year: i64,
    /// The month, 1 for *Ianuarius* through 12 for *December*.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl RomanDate {
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

    /// The astronomical Julian year: 1 BC is year 0.
    #[must_use]
    pub const fn julian_year(self) -> i64 {
        self.year - YEAR_OFFSET
    }

    /// Whether this date falls after Caesar's reform, and so is a date the
    /// Roman calendar would actually have produced.
    #[must_use]
    pub const fn is_after_the_julian_reform(self) -> bool {
        self.year >= JULIAN_REFORM_YEAR
    }
}

/// The *ab urbe condita* calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RomanCalendar;

impl Calendar for RomanCalendar {
    type Date = RomanDate;

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("roman-auc"),
            english_name: "Roman (ab urbe condita)",
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
        Ok(RomanDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_era(ERA)
            .with_extra("julian-year", date.julian_year())
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        RomanDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_year_begins_in_753_bc() {
        // AUC 1 is astronomical Julian year -752, which is 753 BC.
        assert_eq!(to_fixed(1, 1, 1), julian::to_fixed(-752, 1, 1));
        assert_eq!(RomanDate::new(1, 1, 1).unwrap().julian_year(), -752);
    }

    #[test]
    fn ad_one_is_auc_seven_hundred_and_fifty_four() {
        assert_eq!(to_fixed(754, 1, 1), julian::to_fixed(1, 1, 1));
        assert_eq!(
            from_fixed(julian::to_fixed(1, 1, 1).unwrap()),
            Ok((754, 1, 1))
        );
        // And 1 BC, astronomical year 0, is AUC 753: there is no gap,
        // because the AUC count never had a year zero problem.
        assert_eq!(
            from_fixed(julian::to_fixed(0, 1, 1).unwrap()),
            Ok((753, 1, 1))
        );
    }

    #[test]
    fn caesars_reform_falls_in_auc_709() {
        // 45 BC is astronomical year -44, which is AUC 709.
        assert_eq!(
            from_fixed(julian::to_fixed(-44, 1, 1).unwrap()),
            Ok((709, 1, 1))
        );
        assert!(
            RomanDate::new(709, 1, 1)
                .unwrap()
                .is_after_the_julian_reform()
        );
        assert!(
            !RomanDate::new(708, 1, 1)
                .unwrap()
                .is_after_the_julian_reform()
        );
    }

    #[test]
    fn the_ides_of_march_44_bc_is_auc_710() {
        // The assassination of Caesar, 15 March 44 BC, astronomical -43.
        let ides = to_fixed(710, 3, 15).unwrap();
        assert_eq!(julian::from_fixed(ides), Ok((-43, 3, 15)));
    }

    #[test]
    fn leap_years_follow_the_julian_rule() {
        for year in 700..900i64 {
            assert_eq!(
                is_leap_year(year),
                julian::is_leap_year(year - YEAR_OFFSET),
                "AUC {year}"
            );
            let length = to_fixed(year + 1, 1, 1).unwrap().0 - to_fixed(year, 1, 1).unwrap().0;
            assert_eq!(length, i64::from(days_in_year(year)), "AUC {year}");
        }
        assert_eq!(days_in_month(2_653, 2), Some(29)); // AD 1900, Julian leap
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=1_000_000).step_by(71) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_decade_round_trips() {
        let start = to_fixed(700, 1, 1).unwrap().0;
        let end = to_fixed(720, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = RomanCalendar;
        for rd in (EARLIEST.0..=900_000).step_by(563) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(fields.extra.get("julian-year"), Some(date.julian_year()));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("roman-auc"));
    }

    #[test]
    fn dates_outside_the_supported_range_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(754, 2, 30), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            RomanCalendar.from_fields(&DateFields::ymd(754, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }
}
