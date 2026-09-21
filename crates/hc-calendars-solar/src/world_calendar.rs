//! The World Calendar.
//!
//! Elisabeth Achelis proposed this in 1930 and the World Calendar
//! Association pushed it at the League of Nations and then at the United
//! Nations, where India formally moved its adoption in 1954 and the United
//! States opposed it. It remains the best-known perennial calendar proposal.
//!
//! Four identical quarters of 91 days — a 31-day month followed by two
//! 30-day months — which is 364 days, so every quarter begins on a Sunday
//! and every date falls on the same weekday every year. The two remaining
//! days are taken out of the week entirely:
//!
//! * **Worldsday**, after 30 December, ending every year;
//! * **Leapyear Day**, after 30 June, in Gregorian leap years.
//!
//! This module numbers those two as day 31 of months 12 and 6 respectively,
//! which is how the proposal's own tables write them ("W" and "L").
//!
//! # The blank days, and why [`WorldCalendarDate::weekday`] is not
//! [`Weekday::from_rd`]
//!
//! Worldsday and Leapyear Day belong to no week. That is what makes the
//! calendar perennial — 364 is 52 weeks, so if the two extra days are taken
//! out of the count, every date keeps its weekday forever — and it is also
//! what sank the proposal: Jewish, Seventh-day Adventist and Muslim
//! representatives objected at the United Nations that an uncounted day
//! shifts a sabbath observed without interruption since antiquity.
//!
//! So this calendar has **two different weekdays** for the same day, and
//! both are correct:
//!
//! * [`Weekday::from_rd`] gives the real, unbroken seven-day cycle, which
//!   no calendar reform in history has ever managed to interrupt;
//! * [`WorldCalendarDate::weekday`] gives the World Calendar's own naming,
//!   in which every quarter starts on a Sunday and the blank days have no
//!   weekday at all.
//!
//! They agree only in years where the two happen to coincide, and this
//! module does not pretend otherwise.
//!
//! The calendar is Gregorian-aligned: year 1 begins on the same day as
//! Gregorian year 1, the leap rule is the Gregorian rule, and the year
//! number is the Gregorian one. Only the division of the year into months
//! and the naming of weekdays differ.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::gregorian;

/// The number of days in a quarter, every quarter.
pub const DAYS_IN_QUARTER: i64 = 91;

/// Month lengths before the intercalary days are added.
const MONTH_LENGTHS: [i64; 12] = [31, 30, 30, 31, 30, 30, 31, 30, 30, 31, 30, 30];

/// The day number given to Worldsday and Leapyear Day within their month.
pub const INTERCALARY_DAY: u8 = 31;

/// The month Worldsday is attached to.
pub const WORLDSDAY_MONTH: u8 = 12;

/// The month Leapyear Day is attached to.
pub const LEAPYEAR_DAY_MONTH: u8 = 6;

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = gregorian::MIN_YEAR;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = gregorian::MAX_YEAR;

/// Whether `year` is a leap year, by the Gregorian rule.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
///
/// December is 31 days because Worldsday is counted in it, and June is 31 in
/// a leap year for the same reason.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if month == 0 || month > 12 {
        return None;
    }
    let base = MONTH_LENGTHS[month as usize - 1];
    // Worldsday is counted in December and Leapyear Day in June, which is
    // the only way either fits a year-month-day shape at all.
    let extra = if month == WORLDSDAY_MONTH || (month == LEAPYEAR_DAY_MONTH && is_leap_year(year)) {
        1
    } else {
        0
    };
    Some((base + extra) as u8)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    gregorian::days_in_year(year)
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = gregorian::EARLIEST;

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = gregorian::LATEST;

/// Days elapsed in the year before the first of `month`.
const fn days_before_month(year: i64, month: u8) -> i64 {
    let mut elapsed = 0;
    let mut index = 0;
    while index < month as usize - 1 {
        elapsed += MONTH_LENGTHS[index];
        index += 1;
    }
    if month > LEAPYEAR_DAY_MONTH && is_leap_year(year) {
        elapsed += 1;
    }
    elapsed
}

/// The fixed day of a World Calendar date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    match crate::common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => match gregorian::new_year(year) {
            Err(error) => Err(error),
            Ok(start) => Ok(Rd(start.0 + days_before_month(year, month) + day as i64 - 1)),
        },
    }
}

/// The World Calendar year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    match gregorian::year_from_fixed(rd) {
        Err(error) => Err(error),
        Ok(year) => match gregorian::new_year(year) {
            Err(error) => Err(error),
            Ok(start) => {
                let mut day_of_year = rd.0 - start.0;
                let leap = is_leap_year(year);
                let mut month = 1u8;
                while month < 12 {
                    let mut length = MONTH_LENGTHS[month as usize - 1];
                    if month == LEAPYEAR_DAY_MONTH && leap {
                        length += 1;
                    }
                    if day_of_year < length {
                        break;
                    }
                    day_of_year -= length;
                    month += 1;
                }
                Ok((year, month, (day_of_year + 1) as u8))
            }
        },
    }
}

/// A World Calendar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldCalendarDate {
    /// The year, which is the Gregorian year.
    pub year: i64,
    /// The month, 1 through 12.
    pub month: u8,
    /// The day of the month. Day 31 of month 12 is Worldsday and day 31 of
    /// month 6 is Leapyear Day; neither belongs to a week.
    pub day: u8,
}

impl WorldCalendarDate {
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

    /// Whether this date is Worldsday, the year-end day outside the week.
    #[must_use]
    pub const fn is_worldsday(self) -> bool {
        self.month == WORLDSDAY_MONTH && self.day == INTERCALARY_DAY
    }

    /// Whether this date is Leapyear Day, the mid-year day outside the week.
    #[must_use]
    pub const fn is_leapyear_day(self) -> bool {
        self.month == LEAPYEAR_DAY_MONTH && self.day == INTERCALARY_DAY
    }

    /// Whether this date belongs to the seven-day week at all.
    #[must_use]
    pub const fn is_in_the_week(self) -> bool {
        !self.is_worldsday() && !self.is_leapyear_day()
    }

    /// The quarter, 1 through 4.
    #[must_use]
    pub const fn quarter(self) -> u8 {
        (self.month - 1) / 3 + 1
    }

    /// The day's position within its quarter, counting from 0, or `None`
    /// for a blank day.
    const fn position_in_quarter(self) -> Option<i64> {
        let within_quarter = (self.month - 1) % 3;
        let offset = match within_quarter {
            0 => 0,
            1 => 31,
            _ => 61,
        };
        let position = offset + self.day as i64 - 1;
        if position >= DAYS_IN_QUARTER {
            None
        } else {
            Some(position)
        }
    }

    /// The World Calendar's own weekday, or `None` for a blank day.
    ///
    /// Every quarter opens on a Sunday, so this depends only on the position
    /// within the quarter — never on the year. It is deliberately **not**
    /// the same function as [`Weekday::from_rd`]: see the module
    /// documentation for why the two disagree.
    #[must_use]
    pub const fn weekday(self) -> Option<Weekday> {
        match self.position_in_quarter() {
            None => None,
            Some(position) => Some(match position.rem_euclid(7) {
                0 => Weekday::Sunday,
                1 => Weekday::Monday,
                2 => Weekday::Tuesday,
                3 => Weekday::Wednesday,
                4 => Weekday::Thursday,
                5 => Weekday::Friday,
                _ => Weekday::Saturday,
            }),
        }
    }
}

/// The World Calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WorldCalendar;

impl Calendar for WorldCalendar {
    type Date = WorldCalendarDate;

    /// Twelve months and the seven-day week.
    ///
    /// Worldsday and Leapyear Day sit outside the week, and `to_fields`
    /// flags them `outside-the-week`, but the week's positions are still
    /// the seven.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("world-calendar"),
            english_name: "The World Calendar",
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
        Ok(WorldCalendarDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("quarter", i64::from(date.quarter()))?
            .with_extra("outside-the-week", i64::from(!date.is_in_the_week()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        WorldCalendarDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::Weekday;

    #[test]
    fn the_year_starts_where_the_gregorian_year_starts() {
        assert_eq!(to_fixed(2026, 1, 1), gregorian::to_fixed(2026, 1, 1));
        assert_eq!(to_fixed(1, 1, 1), Ok(Rd(1)));
        assert_eq!(from_fixed(Rd(719_163)), Ok((1970, 1, 1)));
    }

    #[test]
    fn every_quarter_is_ninety_one_days_and_begins_on_a_sunday() {
        // 91 = 13 weeks, so each quarter is a whole number of weeks in the
        // calendar's own naming — which is the entire point of the design.
        for year in [1970, 2000, 2024, 2026] {
            for quarter in 0..4u8 {
                let first_month = 1 + quarter * 3;
                let start = WorldCalendarDate::new(year, first_month, 1).unwrap();
                assert_eq!(
                    start.weekday(),
                    Some(Weekday::Sunday),
                    "{year} quarter {quarter}"
                );
                let total: i64 = (0..3)
                    .map(|offset| i64::from(days_in_month(year, first_month + offset).unwrap()))
                    .sum();
                let intercalary =
                    i64::from(quarter == 1 && is_leap_year(year)) + i64::from(quarter == 3);
                assert_eq!(total, DAYS_IN_QUARTER + intercalary);
            }
        }
    }

    #[test]
    fn the_months_of_a_quarter_are_thirty_one_thirty_thirty() {
        assert_eq!(days_in_month(2026, 1), Some(31));
        assert_eq!(days_in_month(2026, 2), Some(30));
        assert_eq!(days_in_month(2026, 3), Some(30));
        assert_eq!(days_in_month(2026, 4), Some(31));
        assert_eq!(days_in_month(2026, 13), None);
        assert_eq!(days_in_month(2026, 0), None);
    }

    #[test]
    fn worldsday_ends_every_year() {
        for year in [1900, 2000, 2025, 2026] {
            let worldsday = to_fixed(year, 12, 31).unwrap();
            assert_eq!(worldsday, gregorian::to_fixed(year, 12, 31).unwrap());
            assert_eq!(to_fixed(year + 1, 1, 1).unwrap().0, worldsday.0 + 1);
            assert!(WorldCalendarDate::new(year, 12, 31).unwrap().is_worldsday());
            assert!(
                !WorldCalendarDate::new(year, 12, 31)
                    .unwrap()
                    .is_in_the_week()
            );
        }
    }

    #[test]
    fn leapyear_day_only_exists_in_leap_years() {
        assert!(to_fixed(2024, 6, 31).is_ok());
        assert_eq!(to_fixed(2026, 6, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(days_in_month(2024, 6), Some(31));
        assert_eq!(days_in_month(2026, 6), Some(30));
        let day = WorldCalendarDate::new(2024, 6, 31).unwrap();
        assert!(day.is_leapyear_day());
        assert!(!day.is_in_the_week());
        // It sits exactly halfway: day 183 of a 366-day year.
        let rd = to_fixed(2024, 6, 31).unwrap();
        assert_eq!(rd.0 - gregorian::to_fixed(2024, 1, 1).unwrap().0 + 1, 183);
    }

    #[test]
    fn a_date_keeps_its_weekday_for_ever_in_the_calendars_own_naming() {
        // The perennial property: every quarter opens on a Sunday, April
        // has 31 days so May opens on a Wednesday, and 17 May is therefore
        // a Friday in every year for ever.
        for year in 1900..2100 {
            let date = WorldCalendarDate::new(year, 5, 17).unwrap();
            assert_eq!(date.weekday(), Some(Weekday::Friday), "year {year}");
        }
        assert_eq!(
            WorldCalendarDate::new(2026, 5, 1).unwrap().weekday(),
            Some(Weekday::Wednesday)
        );
        // 1 January is always a Sunday, 30 December always a Saturday.
        assert_eq!(
            WorldCalendarDate::new(2026, 1, 1).unwrap().weekday(),
            Some(Weekday::Sunday)
        );
        assert_eq!(
            WorldCalendarDate::new(2026, 12, 30).unwrap().weekday(),
            Some(Weekday::Saturday)
        );
    }

    #[test]
    fn the_blank_days_have_no_weekday_and_break_the_real_one() {
        // The objection that stopped the reform, stated as a test: the
        // calendar's naming and the unbroken seven-day cycle come apart,
        // and by more than a day over a few years.
        assert_eq!(
            WorldCalendarDate::new(2026, 12, 31).unwrap().weekday(),
            None
        );
        assert_eq!(WorldCalendarDate::new(2024, 6, 31).unwrap().weekday(), None);

        let mut disagreements = 0;
        for year in 2000..2030 {
            let date = WorldCalendarDate::new(year, 5, 17).unwrap();
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
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = gregorian::to_fixed(1896, 1, 1).unwrap().0;
        let end = gregorian::to_fixed(1912, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = WorldCalendar;
        for rd in (-100_000..=900_000).step_by(257) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.extra.get("quarter"), Some(i64::from(date.quarter())));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("world-calendar"));
    }

    #[test]
    fn quarters_are_numbered_one_to_four() {
        assert_eq!(WorldCalendarDate::new(2026, 1, 1).unwrap().quarter(), 1);
        assert_eq!(WorldCalendarDate::new(2026, 3, 30).unwrap().quarter(), 1);
        assert_eq!(WorldCalendarDate::new(2026, 4, 1).unwrap().quarter(), 2);
        assert_eq!(WorldCalendarDate::new(2026, 12, 31).unwrap().quarter(), 4);
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
        assert_eq!(to_fixed(2026, 1, 32), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2026, 13, 1), Err(CalendarError::MonthOutOfRange));
    }
}
