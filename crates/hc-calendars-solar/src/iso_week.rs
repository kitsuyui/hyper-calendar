//! ISO 8601 week dates.
//!
//! A week date names a day as a year, a week and a weekday — `2021-W53-5`.
//! The rule that makes it well defined is that week 1 of a year is the week
//! containing its first Thursday, equivalently the week containing 4 January,
//! equivalently the first week with the majority of its days in the year.
//! Every week therefore belongs entirely to one ISO year, which is the point
//! of the scheme and also why the ISO year can differ from the Gregorian one
//! by a day or three at either end.
//!
//! The consequence that surprises people: 2021-01-01 is `2021-W53-5` of ISO
//! year **2020**, and 2019-12-30 is `2020-W01-1`.
//!
//! An ISO year has 52 weeks (364 days) or 53 (371). It has 53 when the
//! Gregorian year starts on a Thursday, or starts on a Wednesday and is a
//! leap year — which this module derives rather than tabulates, by measuring
//! the distance between two consecutive week-one Mondays.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::gregorian;

/// The earliest ISO year this implementation converts.
pub const MIN_YEAR: i64 = gregorian::MIN_YEAR + 1;

/// The latest ISO year this implementation converts.
pub const MAX_YEAR: i64 = gregorian::MAX_YEAR - 1;

/// The fixed day of the Monday that opens week 1 of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn week_one_start(year: i64) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    // 4 January is in week 1 by definition, so the Monday on or before it
    // opens week 1.
    Ok(Weekday::Monday.on_or_before(gregorian::to_fixed(year, 1, 4)?))
}

/// The number of weeks in `year`, either 52 or 53.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn weeks_in_year(year: i64) -> CalendarResult<u8> {
    let start = week_one_start(year)?;
    let next = week_one_start(year + 1)?;
    Ok(((next.0 - start.0) / 7) as u8)
}

/// Whether `year` is a 53-week ISO year.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn is_long_year(year: i64) -> CalendarResult<bool> {
    Ok(weeks_in_year(year)? == 53)
}

/// An ISO 8601 week date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IsoWeekDate {
    /// The ISO week-numbering year, which is not always the Gregorian year.
    pub year: i64,
    /// The week, 1 through 52 or 53.
    pub week: u8,
    /// The weekday, 1 for Monday through 7 for Sunday.
    pub weekday: u8,
}

impl IsoWeekDate {
    /// A validated week date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] when the week does not
    /// exist in that year — the ISO week is the closest thing this calendar
    /// has to a month — and [`CalendarError::DayOutOfRange`] when the
    /// weekday is not in `1..=7`.
    pub fn new(year: i64, week: u8, weekday: u8) -> CalendarResult<Self> {
        if !(1..=7).contains(&weekday) {
            return Err(CalendarError::DayOutOfRange);
        }
        if week == 0 || week > weeks_in_year(year)? {
            return Err(CalendarError::MonthOutOfRange);
        }
        Ok(Self {
            year,
            week,
            weekday,
        })
    }

    /// The weekday as a [`Weekday`].
    #[must_use]
    pub const fn day_of_week(self) -> Option<Weekday> {
        Weekday::from_iso_number(self.weekday)
    }
}

/// The fixed day of an ISO week date.
///
/// # Errors
///
/// Returns a [`CalendarError`] when the week date does not exist.
pub fn to_fixed(year: i64, week: u8, weekday: u8) -> CalendarResult<Rd> {
    let date = IsoWeekDate::new(year, week, weekday)?;
    let start = week_one_start(date.year)?;
    Ok(Rd(start.0
        + 7 * (i64::from(date.week) - 1)
        + i64::from(date.weekday)
        - 1))
}

/// The ISO week date of a fixed day.
///
/// # Errors
///
/// Returns a [`CalendarError`] when the day is outside the supported range.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    // Three days back lands inside the ISO year the day belongs to: a day
    // can be at most three days into a week that started in the previous
    // Gregorian year.
    let candidate = gregorian::year_from_fixed(Rd(rd.0 - 3))?;
    let year = if rd >= week_one_start(candidate + 1)? {
        candidate + 1
    } else {
        candidate
    };
    let start = week_one_start(year)?;
    let week = ((rd.0 - start.0) / 7 + 1) as u8;
    let weekday = Weekday::from_rd(rd).iso_number();
    Ok((year, week, weekday))
}

/// The ISO 8601 week-date calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IsoWeekCalendar;

impl Calendar for IsoWeekCalendar {
    type Date = IsoWeekDate;

    fn cycles(&self) -> Option<&'static [hc_calendar::shape::CycleShape]> {
        Some(hc_calendar::shape::SOLAR_TWELVE)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("iso8601-week"),
            english_name: "ISO 8601 week date",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(Rd(gregorian::EARLIEST.0 + 400)),
            latest: Some(Rd(gregorian::LATEST.0 - 400)),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.week, date.weekday)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.meta().check_range(rd)?;
        let (year, week, weekday) = from_fixed(rd)?;
        Ok(IsoWeekDate {
            year,
            week,
            weekday,
        })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::new(date.year)
            .with_extra("week", i64::from(date.week))?
            .with_extra("day-of-week", i64::from(date.weekday))
    }

    /// Weeks and weekdays live in the extra fields because a week date has
    /// no month. A missing week or weekday defaults to 1, so that asking for
    /// "the start of ISO year 2026" does not need a full field set — which
    /// is also what lets [`hc_calendar::DynCalendar::days_in_year`] work,
    /// since it probes a calendar with a bare year-month-day. A month, if
    /// one is supplied, is ignored for the same reason.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let week = fields.extra.get("week").unwrap_or(1);
        let weekday = fields.extra.get("day-of-week").unwrap_or(1);
        let week = u8::try_from(week).map_err(|_| CalendarError::MonthOutOfRange)?;
        let weekday = u8::try_from(weekday).map_err(|_| CalendarError::DayOutOfRange)?;
        IsoWeekDate::new(fields.year, week, weekday)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_year_days_can_belong_to_the_previous_iso_year() {
        // The standard's own example: 2021-01-01 is 2020-W53-5.
        let rd = gregorian::to_fixed(2021, 1, 1).unwrap();
        assert_eq!(from_fixed(rd), Ok((2020, 53, 5)));
        assert_eq!(Weekday::from_rd(rd), Weekday::Friday);
        assert_eq!(to_fixed(2020, 53, 5), Ok(rd));
    }

    #[test]
    fn year_end_days_can_belong_to_the_next_iso_year() {
        // 2019-12-30 was a Monday, and it opens ISO week 1 of 2020.
        let rd = gregorian::to_fixed(2019, 12, 30).unwrap();
        assert_eq!(from_fixed(rd), Ok((2020, 1, 1)));
        assert_eq!(Weekday::from_rd(rd), Weekday::Monday);
    }

    #[test]
    fn week_one_always_contains_the_fourth_of_january() {
        for year in 1583..2400 {
            let start = week_one_start(year).unwrap();
            let fourth = gregorian::to_fixed(year, 1, 4).unwrap();
            assert_eq!(Weekday::from_rd(start), Weekday::Monday, "year {year}");
            assert!(start <= fourth && fourth.0 < start.0 + 7, "year {year}");
        }
    }

    #[test]
    fn week_one_always_contains_the_first_thursday() {
        for year in 1900..2200 {
            let start = week_one_start(year).unwrap();
            let thursday = Rd(start.0 + 3);
            assert_eq!(Weekday::from_rd(thursday), Weekday::Thursday);
            let (gregorian_year, _, _) = gregorian::from_fixed(thursday).unwrap();
            assert_eq!(gregorian_year, year, "first Thursday of {year}");
        }
    }

    #[test]
    fn long_years_are_the_ones_the_rule_predicts() {
        // 53 weeks when 1 January is a Thursday, or a Wednesday in a leap
        // year. Both halves of that rule are checked against the calendar.
        for year in 1600..2400 {
            let jan1 = Weekday::from_rd(gregorian::to_fixed(year, 1, 1).unwrap());
            let predicted = jan1 == Weekday::Thursday
                || (jan1 == Weekday::Wednesday && gregorian::is_leap_year(year));
            assert_eq!(is_long_year(year), Ok(predicted), "year {year}");
        }
    }

    #[test]
    fn known_long_years_have_fifty_three_weeks() {
        for year in [2004, 2009, 2015, 2020, 2026] {
            assert_eq!(weeks_in_year(year), Ok(53), "year {year}");
        }
        for year in [2005, 2021, 2022, 2023, 2024, 2025] {
            assert_eq!(weeks_in_year(year), Ok(52), "year {year}");
        }
    }

    #[test]
    fn an_iso_year_is_a_whole_number_of_weeks() {
        for year in 1800..2300 {
            let length = week_one_start(year + 1).unwrap().0 - week_one_start(year).unwrap().0;
            assert!(length == 364 || length == 371, "year {year} was {length}");
        }
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (-200_000..=1_200_000).step_by(37) {
            let (year, week, weekday) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, week, weekday), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_three_decades_round_trips() {
        let start = gregorian::to_fixed(2000, 1, 1).unwrap().0;
        let end = gregorian::to_fixed(2030, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, week, weekday) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, week, weekday), Ok(Rd(rd)), "rd {rd}");
            assert!((1..=53).contains(&week));
            assert!((1..=7).contains(&weekday));
        }
    }

    #[test]
    fn impossible_week_dates_are_rejected() {
        // 2021 is a 52-week year, so there is no 2021-W53.
        assert_eq!(
            IsoWeekDate::new(2021, 53, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert!(IsoWeekDate::new(2020, 53, 1).is_ok());
        assert_eq!(
            IsoWeekDate::new(2021, 0, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            IsoWeekDate::new(2021, 1, 0),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            IsoWeekDate::new(2021, 1, 8),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = IsoWeekCalendar;
        for rd in (-100_000..=900_000).step_by(431) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.extra.get("week"), Some(i64::from(date.week)));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("iso8601-week"));
    }

    #[test]
    fn the_dynamic_year_length_is_a_whole_number_of_weeks() {
        use hc_calendar::{DynAdapter, DynCalendar};

        let calendar = DynAdapter::new(IsoWeekCalendar);
        assert_eq!(calendar.days_in_year(2020), Ok(371));
        assert_eq!(calendar.days_in_year(2021), Ok(364));
        assert_eq!(calendar.is_leap_year(2020), Ok(true));
        // The probe the dynamic layer uses carries a month this calendar has
        // no use for; it is ignored rather than rejected.
        assert_eq!(
            IsoWeekCalendar.from_fields(&DateFields::ymd(2021, 1, 1)),
            IsoWeekDate::new(2021, 1, 1)
        );
    }

    #[test]
    fn the_day_of_week_maps_onto_the_weekday_type() {
        let date = IsoWeekDate::new(2020, 53, 5).unwrap();
        assert_eq!(date.day_of_week(), Some(Weekday::Friday));
        assert_eq!(
            IsoWeekDate {
                year: 2020,
                week: 1,
                weekday: 9
            }
            .day_of_week(),
            None
        );
    }
}
