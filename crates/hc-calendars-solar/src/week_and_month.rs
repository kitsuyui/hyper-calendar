//! Karl Palmen's Week and Month Calendar (2018).
//!
//! The ISO 8601 week-numbering year, its weeks grouped into twelve months
//! named as the Gregorian months are: four weeks in each month but
//! February, May, August and November, which have five, and December,
//! which has five in a year of 53 ISO weeks. A date names no day of the
//! month; it names the week of the month, Alpha to Epsilon, and the day of
//! the week, so Thursday 10 January 2019 is *Thursday Beta January 2019*,
//! written 2019-01-2-4. The year is the Gregorian year of the week's
//! Thursday, which is the ISO year. It is a naming of [`crate::iso_week`]
//! and computes nothing of its own. The calendar is written up with the
//! other Hermetic Systems reforms in `docs/systems/hermetic-reforms.md`.
//!
//! The fields carry a day of the month as well, `7 · (week − 1) + weekday`,
//! because every calendar with months carries one; the week of the month
//! and the weekday, which are what Palmen writes, are the `week-of-month`
//! and `day-of-week` extra fields.
//!
//! # Sources
//!
//! * Karl Palmen, "Week and Month Calendar", Hermetic Systems,
//!   <https://www.hermetic.ch/cal_stud/palmen/wkmth.htm>, © 2018, retrieved
//!   2026-09-26 (`palmen-week-and-month`): the grouping of ISO weeks into
//!   months, the week names, the forms of the date, and the examples of
//!   10 January 2019, 25 March 1970 and 4 July 1776 and the rule for an
//!   Epsilon December.
//!
//! # Exactness
//!
//! Exact: the ISO week arithmetic, proleptic before 2018.

use hc_calendar::shape::{CycleLength, CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::iso_week::{self, IsoWeekCalendar};

/// The calendar identifier.
pub const ID: &str = "week-and-month";

/// The weeks of a month, as Palmen names them.
pub const WEEKS: [&str; 5] = ["Alpha", "Beta", "Gamma", "Delta", "Epsilon"];

/// The cycle kind of the week of the month.
pub const WEEK_OF_MONTH: &str = "week-of-month";

/// The ISO week that opens each month: 4, 5 and 4 weeks a quarter.
const FIRST_WEEK: [u8; 12] = [1, 5, 10, 14, 18, 23, 27, 31, 36, 40, 44, 49];

/// The number of weeks in `month` of `year`, or `None` outside `1..=12`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside the ISO week range.
pub fn weeks_in_month(year: i64, month: u8) -> CalendarResult<Option<u8>> {
    Ok(match month {
        1..=11 => Some(FIRST_WEEK[usize::from(month)] - FIRST_WEEK[usize::from(month) - 1]),
        12 => Some(iso_week::weeks_in_year(year)? + 1 - FIRST_WEEK[11]),
        _ => None,
    })
}

/// A Week and Month date: the year, month, week of the month and weekday.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WeekAndMonthDate {
    /// The year, the ISO week-numbering year.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The week of the month, 1 for Alpha through 5 for Epsilon.
    pub week: u8,
    /// The weekday, 1 for Monday through 7 for Sunday.
    pub weekday: u8,
}

impl WeekAndMonthDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] for a month outside
    /// `1..=12`, [`CalendarError::DayOutOfRange`] for a week the month does
    /// not have or a weekday outside `1..=7`, and
    /// [`CalendarError::YearOutOfRange`] outside the ISO week range.
    pub fn new(year: i64, month: u8, week: u8, weekday: u8) -> CalendarResult<Self> {
        let weeks = weeks_in_month(year, month)?.ok_or(CalendarError::MonthOutOfRange)?;
        if week == 0 || week > weeks || !(1..=7).contains(&weekday) {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(Self {
            year,
            month,
            week,
            weekday,
        })
    }

    /// The date from a day of the month, `7 · (week − 1) + weekday`.
    ///
    /// # Errors
    ///
    /// As [`WeekAndMonthDate::new`].
    pub fn from_day_of_month(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        if day == 0 {
            return Err(CalendarError::DayOutOfRange);
        }
        Self::new(year, month, (day - 1) / 7 + 1, (day - 1) % 7 + 1)
    }

    /// The ISO week of the year.
    #[must_use]
    pub const fn iso_week(self) -> u8 {
        FIRST_WEEK[self.month as usize - 1] + self.week - 1
    }

    /// The day of the month, 1 to 28 or 35.
    #[must_use]
    pub const fn day_of_month(self) -> u8 {
        7 * (self.week - 1) + self.weekday
    }

    /// The weekday as a [`Weekday`].
    #[must_use]
    pub const fn day_of_week(self) -> Option<Weekday> {
        Weekday::from_iso_number(self.weekday)
    }

    /// The name of the week of the month, Alpha to Epsilon.
    #[must_use]
    pub const fn week_name(self) -> &'static str {
        WEEKS[self.week as usize - 1]
    }
}

/// The fixed day of a Week and Month date.
///
/// # Errors
///
/// Returns a [`CalendarError`] when the date does not exist.
pub fn to_fixed(date: WeekAndMonthDate) -> CalendarResult<Rd> {
    let date = WeekAndMonthDate::new(date.year, date.month, date.week, date.weekday)?;
    iso_week::to_fixed(date.year, date.iso_week(), date.weekday)
}

/// The Week and Month date of a fixed day.
///
/// # Errors
///
/// Returns a [`CalendarError`] outside the ISO week range.
pub fn from_fixed(rd: Rd) -> CalendarResult<WeekAndMonthDate> {
    let (year, week, weekday) = iso_week::from_fixed(rd)?;
    let month = FIRST_WEEK.partition_point(|first| *first <= week) as u8;
    Ok(WeekAndMonthDate {
        year,
        month,
        week: week - FIRST_WEEK[usize::from(month) - 1] + 1,
        weekday,
    })
}

/// The Week and Month Calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WeekAndMonthCalendar;

/// Twelve months, named by a locale's Gregorian month names; four or five
/// weeks in a month, named Alpha to Epsilon; the seven-day week.
const SHAPE: &[CycleShape] = &[
    CycleShape::fixed(MONTH, 12),
    CycleShape {
        kind: WEEK_OF_MONTH,
        length: CycleLength::Intercalary {
            ordinary: 4,
            extended: 5,
        },
        names: &WEEKS,
    },
    CycleShape::fixed(WEEKDAY, 7),
];

impl Calendar for WeekAndMonthCalendar {
    type Date = WeekAndMonthDate;

    /// Unrecorded: a proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve months, the weeks of the month and the week.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A year of 53 ISO weeks, with an Epsilon December.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        iso_week::is_long_year(year)
    }

    fn meta(&self) -> CalendarMeta {
        let iso = IsoWeekCalendar.meta();
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Week and Month (Palmen)",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: iso.earliest,
            latest: iso.latest,
            native_locales: &[],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.meta().check_range(rd)?;
        from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day_of_month())
            .with_extra("week-of-month", i64::from(date.week))?
            .with_extra("day-of-week", i64::from(date.weekday))
    }

    /// The month and the day of the month; the extra fields are derived
    /// from them and not read.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        WeekAndMonthDate::from_day_of_month(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;

    fn date(year: i64, month: u8, day: u8) -> WeekAndMonthDate {
        from_fixed(gregorian::to_fixed(year, month, day).unwrap()).unwrap()
    }

    #[test]
    fn palmens_examples() {
        // "Gregorian Thursday, 10 January 2019 would be Thursday Beta
        // January 2019", 2019-01-2-4.
        let tenth = date(2019, 1, 10);
        assert_eq!(
            (tenth.year, tenth.month, tenth.week, tenth.weekday),
            (2019, 1, 2, 4)
        );
        assert_eq!(tenth.week_name(), "Beta");
        // "Wednesday, March 25, 1970 ... Wednesday Delta March 1970".
        let birthday = date(1970, 3, 25);
        assert_eq!((birthday.month, birthday.week_name()), (3, "Delta"));
        assert_eq!(birthday.day_of_week(), Some(Weekday::Wednesday));
        // "July 4, 1776 would be Thursday Alpha July 1776 ... the fourth day
        // of July".
        let fourth = date(1776, 7, 4);
        assert_eq!((fourth.month, fourth.week, fourth.weekday), (7, 1, 4));
        assert_eq!(fourth.day_of_month(), 4);
    }

    #[test]
    fn the_months_hold_the_weeks_palmen_lists() {
        let weeks: [u8; 12] = [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4];
        for (index, expected) in weeks.iter().enumerate() {
            let month = index as u8 + 1;
            assert_eq!(weeks_in_month(2021, month), Ok(Some(*expected)), "{month}");
        }
        assert_eq!(weeks_in_month(2020, 12), Ok(Some(5)));
        assert_eq!(weeks_in_month(2020, 13), Ok(None));
        // Week 53 is Epsilon December, and week 9 Epsilon February.
        assert_eq!(
            WeekAndMonthDate::new(2020, 12, 5, 1).unwrap().iso_week(),
            53
        );
        assert_eq!(WeekAndMonthDate::new(2021, 2, 5, 1).unwrap().iso_week(), 9);
        assert_eq!(
            WeekAndMonthDate::new(2021, 12, 5, 1),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            WeekAndMonthDate::new(2021, 1, 5, 1),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            WeekAndMonthDate::new(2021, 0, 1, 1),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    /// "December has an epsilon week, if and only if, Christmas occurs on
    /// Friday Delta December or later."
    #[test]
    fn an_epsilon_december_is_a_late_christmas() {
        for year in 1_600..=2_400 {
            let christmas = date(year, 12, 25);
            assert_eq!(christmas.year, year);
            let late = christmas.week == 5 || (christmas.week == 4 && christmas.weekday >= 5);
            assert_eq!(iso_week::is_long_year(year), Ok(late), "{year}");
        }
    }

    #[test]
    fn every_day_of_three_decades_is_its_iso_week_date() {
        let start = gregorian::to_fixed(2000, 1, 1).unwrap().0;
        for rd in start..start + 11_000 {
            let date = WeekAndMonthCalendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(
                iso_week::from_fixed(Rd(rd)),
                Ok((date.year, date.iso_week(), date.weekday))
            );
            assert_eq!(date.day_of_week(), Some(Weekday::from_rd(Rd(rd))));
            assert_eq!(WeekAndMonthCalendar.to_fixed(date), Ok(Rd(rd)));
            let fields = WeekAndMonthCalendar.to_fields(date).unwrap();
            assert_eq!(WeekAndMonthCalendar.from_fields(&fields), Ok(date));
        }
        assert_eq!(
            WeekAndMonthDate::from_day_of_month(2021, 1, 0),
            Err(CalendarError::DayOutOfRange)
        );
    }
}
