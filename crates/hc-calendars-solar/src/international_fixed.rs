//! The International Fixed Calendar.
//!
//! Moses B. Cotsworth's thirteen-month year, proposed in *The Rational
//! Almanac* and campaigned for by the International Fixed Calendar League
//! in the 1920s: thirteen months of exactly four weeks — the twelve Gregorian
//! months in their order, with *Sol* inserted between June and July — and
//! the day or two the 364 do not cover set apart from the week, **Year
//! Day** after 28 December, ending every year, and **Leap Day** after
//! 28 June in the years the Gregorian calendar has a 29 February. The year
//! is the Gregorian year: it begins on the same day, carries the same
//! number and follows the same leap rule, so only the division into months
//! and the naming of weekdays differ. George Eastman ran the Eastman Kodak
//! Company on it from 1928 to 1989.
//!
//! This module numbers Leap Day as June 29 and Year Day as December 29,
//! the only way either fits a year-month-day shape, and the month names,
//! Sol included, are declared with the shape as [`MONTHS`], because the
//! calendar was defined in English and Sol is nobody's word but its own.
//!
//! Two things follow from the two blank days, and both are stated rather
//! than hidden:
//!
//! * Up to 28 June the calendar's day of the year is the Gregorian one, so
//!   its dates shift against the Gregorian date by a day in leap years, as
//!   the Gregorian ones do against each other; from Leap Day on the
//!   Gregorian date is fixed for ever — 1 Sol is 18 June, 1 December is
//!   3 December and Year Day is 31 December in every year.
//! * Every month begins on a Sunday and ends on a Saturday, and Year Day
//!   and Leap Day belong to no week. That is what makes the calendar
//!   perennial, and it is also what the seven-day week never consented
//!   to. So there are **two weekdays** for the same day, both correct:
//!   `hc_calendar::Weekday::from_rd` gives the unbroken cycle, and
//!   [`InternationalFixedDate::weekday`] gives the calendar's own naming,
//!   `None` on the blank days. `to_fields` flags those days
//!   `outside-the-week`; the week cycle is declared as the seven positions
//!   it has, as [`crate::world_calendar`] does for the same reason.
//!
//! # Sources
//!
//! * Cotsworth, *The Rational Almanac: Tracing the Evolution of Modern
//!   Almanacs from Ancient Ideas of Time, and Suggesting Improvements*
//!   (Acomb, York: the author), the archive.org copy
//!   `rationalalmanact00cotsuoft` from the University of Toronto, whose
//!   introduction is dated Christmas 1902 and whose tables are "fixed from
//!   Christmas, 1916" — the issue archive.org dates 1916. Wikipedia,
//!   "International Fixed Calendar", retrieved 2026-09-26, cites the book
//!   as published by the author in 1905, from a Google Books record, and
//!   dates the first presentation of the plan to 1902
//!   (`wikipedia-international-fixed-calendar`); no 1905 issue was read.
//!   The 1916 issue's introduction gives the proposal in three steps:
//!   Christmas Day "set apart as the extra yearly day ... without any
//!   week-day name", "Leap Day" "as a Public Holiday without any week-day
//!   name", and "13 months of 4 weeks each ... by inserting a Mid-Summer
//!   month (Sol)". The "Proposed Calendar" chapter calls the year-end day
//!   "The Special Day" or "Year Day", and puts Leap Day at the end of June,
//!   where "it would, by my plan, always fall as an extra Saturday" — which
//!   fixes the 28th of every month as a Saturday and the 1st as a Sunday.
//! * Wikipedia, "International Fixed Calendar", retrieved 2026-09-25 and
//!   2026-09-26 (`wikipedia-international-fixed-calendar`), for
//!   the League's settled form — Year Day after 28 December, Leap Day
//!   between Saturday 28 June and Sunday 1 Sol, neither in any week, the
//!   Gregorian leap rule — and for Kodak's use of it from 1928 to 1989.
//!
//! # Exactness
//!
//! Exact — arithmetic, proleptic and unbounded like the Gregorian calendar
//! it is laid over: the whole Gregorian range converts both ways.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::gregorian;

/// The calendar identifier.
pub const ID: &str = "international-fixed";

/// The thirteen months: the Gregorian twelve with Sol between June and
/// July.
pub const MONTHS: [&str; 13] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "Sol",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// The length of every month before the blank days are added.
pub const DAYS_IN_MONTH: u8 = 28;

/// The day number given to Year Day and Leap Day within their month.
pub const INTERCALARY_DAY: u8 = 29;

/// The month Leap Day is attached to, June.
pub const LEAP_DAY_MONTH: u8 = 6;

/// The month Year Day is attached to, December.
pub const YEAR_DAY_MONTH: u8 = 13;

/// The name of the year-end day outside the week.
pub const YEAR_DAY: &str = "Year Day";

/// The name of the leap-year day outside the week.
pub const LEAP_DAY: &str = "Leap Day";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = gregorian::MIN_YEAR;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = gregorian::MAX_YEAR;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Wikipedia, \"International Fixed Calendar\", retrieved 2026-09-25: the Eastman Kodak \
    Company ran on it from 1928 to 1989, the years only, so the calendar years are taken; \
    adopted by no state";

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = gregorian::EARLIEST;

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = gregorian::LATEST;

/// Whether `year` has a Leap Day, by the Gregorian rule.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=13`.
///
/// December is 29 days because Year Day is counted in it, and June is 29 in
/// a leap year for the same reason.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if month == 0 || month > 13 {
        return None;
    }
    let extra = if month == YEAR_DAY_MONTH || (month == LEAP_DAY_MONTH && is_leap_year(year)) {
        1
    } else {
        0
    };
    Some(DAYS_IN_MONTH + extra)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    gregorian::days_in_year(year)
}

/// Days elapsed in the year before the first of `month`.
const fn days_before_month(year: i64, month: u8) -> i64 {
    let elapsed = DAYS_IN_MONTH as i64 * (month as i64 - 1);
    if month > LEAP_DAY_MONTH && is_leap_year(year) {
        elapsed + 1
    } else {
        elapsed
    }
}

/// The fixed day of an International Fixed date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`] —
/// the last also for Leap Day in a year that has none.
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    match crate::common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => match gregorian::new_year(year) {
            Err(error) => Err(error),
            Ok(start) => Ok(Rd(start.0 + days_before_month(year, month) + day as i64 - 1)),
        },
    }
}

/// The International Fixed year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    let year = match gregorian::year_from_fixed(rd) {
        Err(error) => return Err(error),
        Ok(year) => year,
    };
    let start = match gregorian::new_year(year) {
        Err(error) => return Err(error),
        Ok(start) => start,
    };
    // The day's place in the year, counting from 0.
    let mut elapsed = rd.0 - start.0;
    let leap_day_at = DAYS_IN_MONTH as i64 * LEAP_DAY_MONTH as i64;
    if is_leap_year(year) {
        if elapsed == leap_day_at {
            return Ok((year, LEAP_DAY_MONTH, INTERCALARY_DAY));
        }
        if elapsed > leap_day_at {
            elapsed -= 1;
        }
    }
    if elapsed == DAYS_IN_MONTH as i64 * YEAR_DAY_MONTH as i64 {
        return Ok((year, YEAR_DAY_MONTH, INTERCALARY_DAY));
    }
    let month = (elapsed / DAYS_IN_MONTH as i64 + 1) as u8;
    let day = (elapsed % DAYS_IN_MONTH as i64 + 1) as u8;
    Ok((year, month, day))
}

/// An International Fixed date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternationalFixedDate {
    /// The year, which is the Gregorian year.
    pub year: i64,
    /// The month, 1 for January through 13 for December; 7 is Sol.
    pub month: u8,
    /// The day of the month, 1 through 28. Day 29 of month 13 is Year Day
    /// and day 29 of month 6 is Leap Day; neither belongs to a week.
    pub day: u8,
}

impl InternationalFixedDate {
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

    /// The name of the month.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[self.month as usize - 1]
    }

    /// Whether this date is Year Day, the year-end day outside the week.
    #[must_use]
    pub const fn is_year_day(self) -> bool {
        self.month == YEAR_DAY_MONTH && self.day == INTERCALARY_DAY
    }

    /// Whether this date is Leap Day, the mid-year day outside the week.
    #[must_use]
    pub const fn is_leap_day(self) -> bool {
        self.month == LEAP_DAY_MONTH && self.day == INTERCALARY_DAY
    }

    /// Whether this date belongs to the seven-day week at all.
    #[must_use]
    pub const fn is_in_the_week(self) -> bool {
        !self.is_year_day() && !self.is_leap_day()
    }

    /// The name of the blank day this is, if it is one.
    #[must_use]
    pub const fn blank_day(self) -> Option<&'static str> {
        if self.is_year_day() {
            Some(YEAR_DAY)
        } else if self.is_leap_day() {
            Some(LEAP_DAY)
        } else {
            None
        }
    }

    /// The calendar's own weekday, or `None` for a blank day.
    ///
    /// Every month opens on a Sunday, so this depends only on the day of
    /// the month — never on the year or the month. It is deliberately
    /// **not** the same function as [`Weekday::from_rd`]: see the module
    /// documentation for why the two disagree.
    #[must_use]
    pub const fn weekday(self) -> Option<Weekday> {
        if !self.is_in_the_week() {
            return None;
        }
        Some(match (self.day - 1) % 7 {
            0 => Weekday::Sunday,
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            _ => Weekday::Saturday,
        })
    }
}

/// The International Fixed Calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InternationalFixedCalendar;

/// Thirteen named months and the seven-day week.
///
/// Year Day and Leap Day sit outside the week, and `to_fields` flags them
/// `outside-the-week`, but the week's positions are still the seven.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for InternationalFixedCalendar {
    type Date = InternationalFixedDate;

    /// Kept by one company, Eastman Kodak, from 1928 to 1989, and by no
    /// state; the source gives years, so the whole of each is taken.
    fn usage(&self) -> hc_calendar::Usage {
        match (
            gregorian::to_fixed(1928, 1, 1),
            gregorian::to_fixed(1989, 12, 31),
        ) {
            (Ok(from), Ok(until)) => hc_calendar::Usage::between(from, until, USAGE_SOURCE),
            _ => hc_calendar::Usage::UNRECORDED,
        }
    }

    /// A year with a Leap Day, by the Gregorian rule.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    /// Thirteen months, Sol among them, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "International Fixed",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &[],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(InternationalFixedDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("outside-the-week", i64::from(!date.is_in_the_week()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        InternationalFixedDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_year_is_the_gregorian_year_and_begins_with_it() {
        assert_eq!(to_fixed(2026, 1, 1), Ok(gregorian(2026, 1, 1)));
        assert_eq!(to_fixed(1, 1, 1), Ok(Rd(1)));
        assert_eq!(from_fixed(Rd(719_163)), Ok((1970, 1, 1)));
        assert_eq!(days_in_year(2024), 366);
        assert_eq!(days_in_year(2026), 365);
        assert_eq!(MONTHS[6], "Sol");
        assert_eq!(
            InternationalFixedDate::new(2026, 7, 1)
                .unwrap()
                .month_name(),
            "Sol"
        );
    }

    #[test]
    fn from_leap_day_on_every_date_falls_on_a_fixed_gregorian_date() {
        // The equivalences Wikipedia tabulates: 1 Sol is 18 June, Year Day
        // is 31 December and Leap Day is 17 June, in every year.
        for year in [1900, 1928, 2000, 2024, 2025, 2026] {
            assert_eq!(to_fixed(year, 7, 1), Ok(gregorian(year, 6, 18)), "{year}");
            assert_eq!(to_fixed(year, 8, 1), Ok(gregorian(year, 7, 16)), "{year}");
            assert_eq!(to_fixed(year, 13, 1), Ok(gregorian(year, 12, 3)), "{year}");
            assert_eq!(
                to_fixed(year, 13, 28),
                Ok(gregorian(year, 12, 30)),
                "{year}"
            );
            assert_eq!(
                to_fixed(year, 13, 29),
                Ok(gregorian(year, 12, 31)),
                "{year}"
            );
            assert_eq!(
                to_fixed(year + 1, 1, 1).unwrap().0,
                to_fixed(year, 13, 29).unwrap().0 + 1
            );
        }
        assert_eq!(to_fixed(2024, 6, 29), Ok(gregorian(2024, 6, 17)));
        assert_eq!(to_fixed(2000, 6, 29), Ok(gregorian(2000, 6, 17)));
        // Before it, the day of the year is fixed instead: 28 June is day
        // 168, which is 17 June in a common year and 16 June in a leap one.
        assert_eq!(to_fixed(2025, 6, 28), Ok(gregorian(2025, 6, 17)));
        assert_eq!(to_fixed(2024, 6, 28), Ok(gregorian(2024, 6, 16)));
        assert_eq!(to_fixed(2025, 2, 1), Ok(gregorian(2025, 1, 29)));
        assert_eq!(to_fixed(2024, 2, 1), Ok(gregorian(2024, 1, 29)));
    }

    #[test]
    fn leap_day_only_exists_in_gregorian_leap_years() {
        assert!(to_fixed(2024, 6, 29).is_ok());
        assert_eq!(to_fixed(2026, 6, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(1900, 6, 29), Err(CalendarError::DayOutOfRange));
        assert!(to_fixed(2000, 6, 29).is_ok());
        assert_eq!(days_in_month(2024, 6), Some(29));
        assert_eq!(days_in_month(2026, 6), Some(28));
        assert_eq!(days_in_month(2026, 13), Some(29));
        assert_eq!(days_in_month(2026, 7), Some(28));
        assert_eq!(days_in_month(2026, 14), None);
        assert_eq!(days_in_month(2026, 0), None);
        let leap_day = InternationalFixedDate::new(2024, 6, 29).unwrap();
        assert!(leap_day.is_leap_day());
        assert!(!leap_day.is_in_the_week());
        assert_eq!(leap_day.blank_day(), Some(LEAP_DAY));
        let year_day = InternationalFixedDate::new(2024, 13, 29).unwrap();
        assert!(year_day.is_year_day());
        assert_eq!(year_day.blank_day(), Some(YEAR_DAY));
        assert_eq!(
            InternationalFixedDate::new(2024, 6, 28)
                .unwrap()
                .blank_day(),
            None
        );
    }

    #[test]
    fn every_month_begins_on_sunday_and_ends_on_saturday() {
        for year in [1789, 1928, 2024, 2026] {
            for month in 1..=13u8 {
                let first = InternationalFixedDate::new(year, month, 1).unwrap();
                assert_eq!(first.weekday(), Some(Weekday::Sunday), "{year}-{month}");
                let last = InternationalFixedDate::new(year, month, 28).unwrap();
                assert_eq!(last.weekday(), Some(Weekday::Saturday), "{year}-{month}");
            }
        }
        assert_eq!(
            InternationalFixedDate::new(2026, 3, 17).unwrap().weekday(),
            Some(Weekday::Tuesday)
        );
        assert_eq!(
            InternationalFixedDate::new(2026, 13, 29).unwrap().weekday(),
            None
        );
        assert_eq!(
            InternationalFixedDate::new(2024, 6, 29).unwrap().weekday(),
            None
        );
    }

    #[test]
    fn the_blank_days_break_the_real_week() {
        // The calendar's naming and the unbroken seven-day cycle come
        // apart by a day or two a year, so over thirty years they agree
        // only by coincidence.
        let mut disagreements = 0;
        for year in 2000..2030 {
            let date = InternationalFixedDate::new(year, 5, 17).unwrap();
            let real = Weekday::from_rd(to_fixed(year, 5, 17).unwrap());
            if date.weekday() != Some(real) {
                disagreements += 1;
            }
        }
        assert!(
            disagreements > 20,
            "only {disagreements} of 30 years disagreed"
        );
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (-500_000..=1_500_000).step_by(67) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
        for rd in [EARLIEST.0, EARLIEST.0 + 1, LATEST.0 - 1, LATEST.0] {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = gregorian(1896, 1, 1).0;
        let end = gregorian(1912, 1, 1).0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
            assert!(day <= 29 && month <= 13);
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = InternationalFixedCalendar;
        for rd in (-100_000..=900_000).step_by(257) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(
                fields.extra.get("outside-the-week"),
                Some(i64::from(!date.is_in_the_week()))
            );
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId(ID));
        assert_eq!(calendar.cycles()[0].names.len(), 13);
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
        assert_eq!(to_fixed(2026, 1, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2026, 14, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            InternationalFixedCalendar.from_fields(&DateFields::ymd_leap_month(2026, 1, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
    }
}
