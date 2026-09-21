//! The proleptic Gregorian calendar.
//!
//! This is the load-bearing module of the workspace. Half the other calendars
//! here are defined in terms of it — the Buddhist, Minguo, Juche and Holocene
//! calendars are literally Gregorian dates with a different year number, the
//! Indian national calendar pins its new year to a Gregorian date, and the
//! ISO week and ordinal calendars are two more ways of naming a Gregorian
//! day — so everything is written as integer arithmetic with no division of
//! a value that can be negative unless it is `div_euclid`.
//!
//! "Proleptic" means the rules of the 1582 reform are projected backwards
//! without limit. No date before 1582-10-15 in this calendar was ever used by
//! anyone; for what a given country actually wrote on a given day, use
//! [`crate::julian_gregorian`].
//!
//! The formulae are those of Reingold and Dershowitz, *Calendrical
//! Calculations* (4th ed., 2018), chapter 2. They are exact — there is no
//! floating point anywhere in this module — for every year in
//! [`MIN_YEAR`]..=[`MAX_YEAR`], which comfortably contains the
//! -9999..=9999 range that callers of this crate can assume.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::common;

/// The earliest year this implementation converts.
///
/// The bound exists so that `365 * year` cannot overflow `i64` and so that
/// `from_fixed` can reject nonsense rather than wrapping.
pub const MIN_YEAR: i64 = -9_999_999;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999_999;

/// Whether `year` is a Gregorian leap year.
///
/// The rule is the one of the 1582 bull *Inter gravissimas*: every fourth
/// year, except centuries, except every fourth century.
///
/// ```
/// use hc_calendars_solar::gregorian::is_leap_year;
/// assert!(is_leap_year(2000));
/// assert!(!is_leap_year(1900));
/// assert!(!is_leap_year(2100));
/// ```
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
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

/// The 1-based day of the year, counting 1 January as day 1.
///
/// # Errors
///
/// Returns [`CalendarError::MonthOutOfRange`] or
/// [`CalendarError::DayOutOfRange`] when the date does not exist.
pub const fn day_of_year(year: i64, month: u8, day: u8) -> CalendarResult<u16> {
    match common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(common::julian_style_day_of_year(
            month,
            day,
            is_leap_year(year),
        )),
    }
}

/// The fixed day of 1 January of `year`, without any range check.
const fn new_year_raw(year: i64) -> i64 {
    let prior = year - 1;
    365 * prior + prior.div_euclid(4) - prior.div_euclid(100) + prior.div_euclid(400) + 1
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(new_year_raw(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1) - 1);

/// The fixed day of 1 January of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(Rd(new_year_raw(year)))
}

/// The fixed day of a proleptic Gregorian date.
///
/// ```
/// use hc_calendar::Rd;
/// use hc_calendars_solar::gregorian::to_fixed;
/// // The POSIX epoch.
/// assert_eq!(to_fixed(1970, 1, 1), Ok(Rd(719_163)));
/// ```
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    match day_of_year(year, month, day) {
        Err(error) => Err(error),
        Ok(within) => Ok(Rd(new_year_raw(year) + within as i64 - 1)),
    }
}

/// The Gregorian year containing a fixed day.
///
/// The three nested cycle lengths are 146 097 days per 400 years, 36 524 per
/// century and 1 461 per four years; the two `== 4` tests catch the last day
/// of a leap-century cycle, which would otherwise be attributed to a year
/// that does not exist.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn year_from_fixed(rd: Rd) -> CalendarResult<i64> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    let elapsed = rd.0 - 1;
    let cycles_400 = elapsed.div_euclid(146_097);
    let within_400 = elapsed.rem_euclid(146_097);
    let cycles_100 = within_400 / 36_524;
    let within_100 = within_400 % 36_524;
    let cycles_4 = within_100 / 1_461;
    let within_4 = within_100 % 1_461;
    let years_1 = within_4 / 365;
    let years = 400 * cycles_400 + 100 * cycles_100 + 4 * cycles_4 + years_1;
    if cycles_100 == 4 || years_1 == 4 {
        Ok(years)
    } else {
        Ok(years + 1)
    }
}

/// The proleptic Gregorian year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    match year_from_fixed(rd) {
        Err(error) => Err(error),
        Ok(year) => {
            let within = (rd.0 - new_year_raw(year) + 1) as u16;
            let (month, day) = common::julian_style_month_day(within, is_leap_year(year));
            Ok((year, month, day))
        }
    }
}

/// A proleptic Gregorian date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GregorianDate {
    /// The astronomical year: 1 BC is year 0, 2 BC is year -1.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl GregorianDate {
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

    /// The weekday of this date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn weekday(self) -> CalendarResult<Weekday> {
        match to_fixed(self.year, self.month, self.day) {
            Err(error) => Err(error),
            Ok(rd) => Ok(Weekday::from_rd(rd)),
        }
    }

    /// The 1-based day of the year.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn day_of_year(self) -> CalendarResult<u16> {
        day_of_year(self.year, self.month, self.day)
    }
}

/// The proleptic Gregorian calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GregorianCalendar;

impl Calendar for GregorianCalendar {
    type Date = GregorianDate;

    /// Promulgated by *Inter gravissimas* and first used on 15 October 1582.
    /// The arithmetic runs to either side of that by millions of years, and
    /// every day before it is proleptic — including, for most of the world,
    /// a good deal of time *after* it, since adoption took until 1923. Use
    /// [`crate::julian_gregorian`] when the country matters.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(match to_fixed(1582, 10, 15) {
            Ok(rd) => rd,
            Err(_) => Rd(0),
        })
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("gregory"),
            english_name: "Gregorian",
            year_kind: YearKind::Astronomical,
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
        Ok(GregorianDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        GregorianDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_posix_epoch_sits_where_every_reference_puts_it() {
        // POSIX.1-2017 and RFC 3339 both anchor on 1970-01-01, which is
        // RD 719163, JDN 2440588, and a Thursday.
        let rd = to_fixed(1970, 1, 1).unwrap();
        assert_eq!(rd, Rd(719_163));
        assert_eq!(rd.to_julian_day_number(), 2_440_588);
        assert_eq!(Weekday::from_rd(rd), Weekday::Thursday);
        assert_eq!(rd.to_modified_julian_day(), 40_587);
    }

    #[test]
    fn the_year_two_thousand_starts_on_the_published_fixed_day() {
        let rd = to_fixed(2000, 1, 1).unwrap();
        assert_eq!(rd, Rd(730_120));
        // Cross-check: 2000-01-01 was a Saturday, and JDN 2451545 is the
        // J2000.0 epoch day 2000-01-01.
        assert_eq!(Weekday::from_rd(rd), Weekday::Saturday);
        assert_eq!(rd.to_julian_day_number(), 2_451_545);
    }

    #[test]
    fn rata_die_day_one_is_the_first_of_january_year_one() {
        assert_eq!(to_fixed(1, 1, 1), Ok(Rd(1)));
        assert_eq!(from_fixed(Rd(1)), Ok((1, 1, 1)));
        assert_eq!(Weekday::from_rd(Rd(1)), Weekday::Monday);
    }

    #[test]
    fn the_century_rule_is_the_one_from_the_bull() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(1600));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        assert_eq!(days_in_month(2000, 2), Some(29));
        assert_eq!(days_in_month(1900, 2), Some(28));
        assert_eq!(days_in_year(2000), 366);
        assert_eq!(days_in_year(1900), 365);
    }

    #[test]
    fn the_leap_rule_extends_backwards_through_year_zero() {
        // Astronomical numbering: year 0 is 1 BC and is divisible by 400.
        assert!(is_leap_year(0));
        assert!(!is_leap_year(-1));
        assert!(is_leap_year(-4));
        assert!(!is_leap_year(-100));
        assert!(is_leap_year(-400));
        assert_eq!(to_fixed(0, 12, 31), Ok(Rd(0)));
        assert_eq!(from_fixed(Rd(0)), Ok((0, 12, 31)));
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (-2_000_000..=2_000_000).step_by(97) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_four_centuries_round_trips() {
        // A whole 400-year cycle, day by day, so no month boundary escapes.
        let start = to_fixed(1600, 1, 1).unwrap().0;
        let end = to_fixed(2000, 1, 1).unwrap().0;
        assert_eq!(end - start, 146_097);
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)));
        }
    }

    #[test]
    fn every_date_of_the_required_year_range_round_trips() {
        for year in (-9_999..=9_999).step_by(7) {
            for month in 1..=12u8 {
                let length = days_in_month(year, month).unwrap();
                for day in [1, length / 2, length] {
                    let rd = to_fixed(year, month, day).unwrap();
                    assert_eq!(from_fixed(rd), Ok((year, month, day)));
                }
            }
        }
    }

    #[test]
    fn the_days_of_a_year_are_numbered_consecutively() {
        for year in [1583, 1900, 2000, 2023, 2024] {
            let start = new_year(year).unwrap();
            let length = days_in_year(year);
            for offset in 0..i64::from(length) {
                let (_, month, day) = from_fixed(Rd(start.0 + offset)).unwrap();
                assert_eq!(day_of_year(year, month, day), Ok(offset as u16 + 1));
            }
            assert_eq!(new_year(year + 1).unwrap().0 - start.0, i64::from(length));
        }
    }

    #[test]
    fn out_of_range_fields_are_rejected_with_the_right_error() {
        assert_eq!(to_fixed(2024, 0, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(2024, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(2024, 1, 0), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2023, 2, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2024, 2, 30), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2024, 4, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(MIN_YEAR - 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn fixed_days_outside_the_supported_range_are_rejected() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert!(from_fixed(EARLIEST).is_ok());
        assert!(from_fixed(LATEST).is_ok());
        assert_eq!(from_fixed(EARLIEST), Ok((MIN_YEAR, 1, 1)));
        assert_eq!(from_fixed(LATEST), Ok((MAX_YEAR, 12, 31)));
    }

    #[test]
    fn the_calendar_impl_agrees_with_the_free_functions() {
        let calendar = GregorianCalendar;
        let date = GregorianDate::new(2026, 9, 20).unwrap();
        let rd = calendar.to_fixed(date).unwrap();
        assert_eq!(rd, to_fixed(2026, 9, 20).unwrap());
        assert_eq!(calendar.from_fixed(rd), Ok(date));
        let fields = calendar.to_fields(date).unwrap();
        assert_eq!(fields.year, 2026);
        assert_eq!(calendar.from_fields(&fields), Ok(date));
        assert_eq!(calendar.meta().id, CalendarId("gregory"));
        assert!(!calendar.meta().is_astronomical);
    }

    #[test]
    fn leap_months_are_not_a_gregorian_concept() {
        let calendar = GregorianCalendar;
        let fields = DateFields::ymd_leap_month(2024, 2, 1);
        assert_eq!(
            calendar.from_fields(&fields),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::new(2024)),
            Err(CalendarError::MissingField("month"))
        );
    }

    #[test]
    fn weekdays_march_forward_one_day_at_a_time() {
        let mut expected = Weekday::Thursday;
        for offset in 0..400 {
            let rd = Rd(719_163 + offset);
            assert_eq!(Weekday::from_rd(rd), expected);
            expected = Weekday::ALL[(expected.iso_number() % 7) as usize];
        }
        assert_eq!(
            GregorianDate::new(2026, 9, 20).unwrap().weekday(),
            Ok(Weekday::Sunday)
        );
    }

    #[test]
    fn day_of_year_matches_the_familiar_landmarks() {
        assert_eq!(day_of_year(2023, 12, 31), Ok(365));
        assert_eq!(day_of_year(2024, 12, 31), Ok(366));
        assert_eq!(day_of_year(2024, 3, 1), Ok(61));
        assert_eq!(day_of_year(2023, 3, 1), Ok(60));
        assert_eq!(
            GregorianDate::new(2024, 2, 29).unwrap().day_of_year(),
            Ok(60)
        );
    }
}
