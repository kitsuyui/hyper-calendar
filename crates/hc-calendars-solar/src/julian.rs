//! The proleptic Julian calendar.
//!
//! The calendar of the Julian reform of 45 BC: the same twelve months as the
//! Gregorian calendar, but a leap year every fourth year without exception,
//! which is why it drifts from the tropical year by about three days every
//! four centuries.
//!
//! # Two ways to number a year
//!
//! Historians write "44 BC"; astronomers write "-43". There is no year zero
//! in the historical convention, so 1 BC is followed directly by AD 1, while
//! the astronomical convention numbers 1 BC as 0 and 2 BC as -1. Arithmetic
//! is only tractable in the astronomical convention, so [`JulianDate::year`]
//! holds that, and [`JulianDate::era_year`] / [`JulianDate::from_era`]
//! translate to and from the historical one with the era codes
//! [`ERA_BC`] and [`ERA_AD`].
//!
//! Leap years follow the astronomical numbering: year 0 (1 BC) is a leap
//! year, as are -4 (5 BC) and -8 (9 BC). Whether the Roman priesthood
//! actually intercalated on that schedule between 45 BC and AD 4 is a
//! separate and genuinely disputed question; this module implements the
//! regular proleptic rule and does not model the "Augustan correction".

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::common;

/// The fixed day of 1 January of Julian year 1, which is 0000-12-30 in the
/// proleptic Gregorian calendar.
pub const EPOCH: Rd = Rd(-1);

/// The era code for years before the incarnation.
pub const ERA_BC: &str = "BC";

/// The era code for years after it.
pub const ERA_AD: &str = "AD";

/// The earliest astronomical year this implementation converts.
pub const MIN_YEAR: i64 = -9_999_999;

/// The latest astronomical year this implementation converts.
pub const MAX_YEAR: i64 = 9_999_999;

/// Whether an astronomical Julian year is a leap year.
///
/// Every fourth year, with no century exception, and counting through year
/// zero: 1 BC, 5 BC and 9 BC are leap years on this reckoning.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    year.rem_euclid(4) == 0
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    common::julian_style_days_in_month(month, is_leap_year(year))
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The fixed day of 1 January of `year`, without any range check.
const fn new_year_raw(year: i64) -> i64 {
    let prior = year - 1;
    EPOCH.0 + 365 * prior + prior.div_euclid(4)
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(new_year_raw(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1) - 1);

/// The fixed day of a proleptic Julian date, in astronomical year numbering.
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
        Ok(()) => {
            let within = common::julian_style_day_of_year(month, day, is_leap_year(year));
            Ok(Rd(new_year_raw(year) + within as i64 - 1))
        }
    }
}

/// The proleptic Julian year, month and day of a fixed day.
///
/// The year comes from one division: four Julian years are exactly 1 461
/// days, so the cycle has no nested exceptions to unwind the way the
/// Gregorian one does.
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
    let elapsed = rd.0 - EPOCH.0;
    let year = (4 * elapsed + 3).div_euclid(1_461) + 1;
    let within = (rd.0 - new_year_raw(year) + 1) as u16;
    let (month, day) = common::julian_style_month_day(within, is_leap_year(year));
    Ok((year, month, day))
}

/// A proleptic Julian date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JulianDate {
    /// The astronomical year: 1 BC is year 0, 2 BC is year -1.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl JulianDate {
    /// A validated date in astronomical year numbering.
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

    /// A validated date in historical era numbering, where `era` is
    /// [`ERA_BC`] or [`ERA_AD`] and `year` is always positive.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::UnknownEra`] for any other era code,
    /// [`CalendarError::YearOutOfRange`] for a non-positive era year, and
    /// the usual field errors otherwise.
    pub const fn from_era(era: &str, year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        if year < 1 {
            return Err(CalendarError::YearOutOfRange);
        }
        // `match` on `&str` is not available in a const fn, so compare bytes.
        let astronomical = if equals_ignoring_case(era, ERA_AD) || equals_ignoring_case(era, "CE") {
            year
        } else if equals_ignoring_case(era, ERA_BC) || equals_ignoring_case(era, "BCE") {
            1 - year
        } else {
            return Err(CalendarError::UnknownEra);
        };
        Self::new(astronomical, month, day)
    }

    /// The era code and positive year of the historical convention.
    #[must_use]
    pub const fn era_year(self) -> (&'static str, i64) {
        if self.year > 0 {
            (ERA_AD, self.year)
        } else {
            (ERA_BC, 1 - self.year)
        }
    }
}

/// ASCII case-insensitive string comparison, usable in a `const fn`.
const fn equals_ignoring_case(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if !left[index].eq_ignore_ascii_case(&right[index]) {
            return false;
        }
        index += 1;
    }
    true
}

/// The proleptic Julian calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JulianCalendar;

impl Calendar for JulianCalendar {
    type Date = JulianDate;

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("julian"),
            english_name: "Julian",
            year_kind: YearKind::EraRelative,
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
        Ok(JulianDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let (era, year) = date.era_year();
        DateFields::ymd(year, date.month, date.day)
            .with_era(era)
            .with_extra("astronomical-year", date.year)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        match fields.era {
            // Without an era the year is taken to be astronomical, which is
            // what every other calendar in this crate means by `year`.
            None => JulianDate::new(fields.year, month.ordinal, day),
            Some(era) => JulianDate::from_era(era, fields.year, month.ordinal, day),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;
    use hc_calendar::Weekday;

    #[test]
    fn the_julian_epoch_is_two_days_before_the_gregorian_one() {
        // 0001-01-01 Julian is 0000-12-30 proleptic Gregorian.
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
        assert_eq!(gregorian::from_fixed(EPOCH), Ok((0, 12, 30)));
    }

    #[test]
    fn the_gregorian_reform_skipped_ten_days() {
        // Thursday 4 October 1582 Julian was followed by Friday 15 October
        // 1582 Gregorian: the same weekday sequence, ten dates removed.
        let last_julian = to_fixed(1582, 10, 4).unwrap();
        let first_gregorian = gregorian::to_fixed(1582, 10, 15).unwrap();
        assert_eq!(first_gregorian.0, last_julian.0 + 1);
        assert_eq!(Weekday::from_rd(last_julian), Weekday::Thursday);
        assert_eq!(Weekday::from_rd(first_gregorian), Weekday::Friday);
    }

    #[test]
    fn the_two_calendars_agree_between_ad_200_and_ad_300() {
        // The calendars coincide exactly from 1 March AD 200 to 29 February
        // AD 300, the century in which the accumulated drift is zero.
        let julian = to_fixed(200, 3, 1).unwrap();
        let gregorian = gregorian::to_fixed(200, 3, 1).unwrap();
        assert_eq!(julian, gregorian);
    }

    #[test]
    fn the_drift_grows_by_three_days_every_four_centuries() {
        // The difference steps up at every century that the Gregorian rule
        // refuses to intercalate: 300, 500, 600, 700, 900, 1000, 1100, 1300,
        // 1400, 1500, 1700, 1800, 1900, 2100.
        for (year, expected) in [
            (300, 1),
            (500, 2),
            (900, 5),
            (1300, 8),
            (1582, 10),
            (1700, 11),
            (1800, 12),
            (1900, 13),
            (2100, 14),
        ] {
            let julian = to_fixed(year, 3, 1).unwrap();
            let gregorian = gregorian::to_fixed(year, 3, 1).unwrap();
            assert_eq!(julian.0 - gregorian.0, expected, "year {year}");
        }
    }

    #[test]
    fn every_fourth_year_is_a_leap_year_with_no_exceptions() {
        assert!(is_leap_year(1900));
        assert!(is_leap_year(2100));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1901));
        assert_eq!(days_in_month(1900, 2), Some(29));
        assert_eq!(days_in_year(1900), 366);
        assert!(to_fixed(1900, 2, 29).is_ok());
        assert_eq!(
            gregorian::to_fixed(1900, 2, 29),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn leap_years_continue_through_year_zero_into_bc() {
        assert!(is_leap_year(0));
        assert!(is_leap_year(-4));
        assert!(!is_leap_year(-1));
        assert_eq!(days_in_year(0), 366);
        assert!(to_fixed(0, 2, 29).is_ok());
    }

    #[test]
    fn eras_translate_both_ways() {
        let bc = JulianDate::from_era("BC", 44, 3, 15).unwrap();
        // The Ides of March, 44 BC, is astronomical year -43.
        assert_eq!(bc.year, -43);
        assert_eq!(bc.era_year(), ("BC", 44));
        let ad = JulianDate::from_era("ad", 1, 1, 1).unwrap();
        assert_eq!(ad.year, 1);
        assert_eq!(ad.era_year(), ("AD", 1));
        assert_eq!(JulianDate::from_era("CE", 1, 1, 1), Ok(ad));
        assert_eq!(JulianDate::from_era("BCE", 1, 1, 1).unwrap().year, 0);
    }

    #[test]
    fn there_is_no_year_zero_in_the_era_convention() {
        assert_eq!(
            JulianDate::from_era("AD", 0, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            JulianDate::from_era("BC", -1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            JulianDate::from_era("Reiwa", 1, 1, 1),
            Err(CalendarError::UnknownEra)
        );
        // 1 BC is immediately followed by AD 1: one day apart.
        let last_bc = to_fixed(0, 12, 31).unwrap();
        let first_ad = to_fixed(1, 1, 1).unwrap();
        assert_eq!(first_ad.0, last_bc.0 + 1);
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (-2_000_000..=2_000_000).step_by(89) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = to_fixed(-5, 1, 1).unwrap().0;
        let end = to_fixed(15, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = JulianCalendar;
        for rd in (-500_000..=1_000_000).step_by(1_009) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
            assert!(fields.year > 0, "era years are always positive");
            assert_eq!(fields.extra.get("astronomical-year"), Some(date.year));
        }
        assert_eq!(calendar.meta().id, CalendarId("julian"));
        assert_eq!(calendar.meta().year_kind, YearKind::EraRelative);
    }

    #[test]
    fn out_of_range_fields_are_rejected() {
        assert_eq!(to_fixed(2024, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(2023, 2, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2024, 2, 30), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            to_fixed(MIN_YEAR - 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }
}
