//! The arithmetic of a leap-week calendar with one remainder rule.
//!
//! Bromberg's Symmetry calendars and Meyer's Hermetic Leap Week Calendar
//! are the same machine: a year of 52 weeks, 364 days, or 53 in a leap
//! year, the extra week appended to the twelfth month, and a leap year
//! wherever `(a · year + c) mod m < a` — `a` leap years in every `m`. They
//! differ in the three constants, in the day their year 1 begins and in
//! how the 364 days are cut into months. So the constants are a
//! [`LeapWeekRule`], the month layout an array, and everything else is
//! written once, here (policy §2).
//!
//! The rule's form makes the leap years before any year a single division:
//! `(a · year + c) mod m < a` exactly when `⌊(a · year + c) / m⌋` steps up
//! from the year before, so the steps are the count.

use hc_calendar::{CalendarError, CalendarResult, Rd};

use crate::common;

/// Days in an ordinary year: 52 weeks.
pub(crate) const ORDINARY_YEAR_DAYS: i64 = 364;

/// Days in a leap year: 53 weeks.
pub(crate) const LEAP_YEAR_DAYS: i64 = 371;

/// A leap-week rule `(leaps · year + offset) mod cycle < leaps`, with the
/// day year 1 begins on and the years an implementation converts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LeapWeekRule {
    /// Leap years in a cycle, `a`.
    pub leaps: i64,
    /// Years in the cycle, `m`.
    pub cycle: i64,
    /// The offset `c`, which places the leap years in the cycle.
    pub offset: i64,
    /// The fixed day on which year 1 begins.
    pub epoch: i64,
    /// The earliest year converted.
    pub min_year: i64,
    /// The latest year converted.
    pub max_year: i64,
}

impl LeapWeekRule {
    /// Whether `year` has the fifty-third week.
    pub(crate) const fn is_leap_year(self, year: i64) -> bool {
        (self.leaps * year + self.offset).rem_euclid(self.cycle) < self.leaps
    }

    /// Leap years from year 1 up to but not including `year`, negative
    /// before year 1.
    const fn leap_years_before(self, year: i64) -> i64 {
        (self.leaps * (year - 1) + self.offset).div_euclid(self.cycle)
            - self.offset.div_euclid(self.cycle)
    }

    /// Days in a whole cycle.
    pub(crate) const fn cycle_days(self) -> i64 {
        self.cycle * ORDINARY_YEAR_DAYS + self.leaps * 7
    }

    /// Days in `year`.
    pub(crate) const fn days_in_year(self, year: i64) -> u16 {
        if self.is_leap_year(year) {
            LEAP_YEAR_DAYS as u16
        } else {
            ORDINARY_YEAR_DAYS as u16
        }
    }

    /// The fixed day on which `year` begins, without validation.
    pub(crate) const fn new_year_raw(self, year: i64) -> i64 {
        self.epoch + ORDINARY_YEAR_DAYS * (year - 1) + 7 * self.leap_years_before(year)
    }

    /// The fixed day on which `year` begins.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside the rule's years.
    pub(crate) const fn new_year(self, year: i64) -> CalendarResult<Rd> {
        if year < self.min_year || year > self.max_year {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(Rd(self.new_year_raw(year)))
    }

    /// The first day converted.
    pub(crate) const fn earliest(self) -> Rd {
        Rd(self.new_year_raw(self.min_year))
    }

    /// The last day converted.
    pub(crate) const fn latest(self) -> Rd {
        Rd(self.new_year_raw(self.max_year + 1) - 1)
    }

    /// Days in `month` of `year` under `layout`, the week added to month 12
    /// in a leap year.
    pub(crate) const fn days_in_month(self, layout: &[u8; 12], year: i64, month: u8) -> Option<u8> {
        if month == 0 || month > 12 {
            return None;
        }
        let base = layout[month as usize - 1];
        if month == 12 && self.is_leap_year(year) {
            Some(base + 7)
        } else {
            Some(base)
        }
    }

    /// The fixed day of a date under `layout`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`],
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
    pub(crate) const fn fixed_day(
        self,
        layout: &[u8; 12],
        year: i64,
        month: u8,
        day: u8,
    ) -> CalendarResult<Rd> {
        if year < self.min_year || year > self.max_year {
            return Err(CalendarError::YearOutOfRange);
        }
        match common::check_day(day, self.days_in_month(layout, year, month)) {
            Err(error) => Err(error),
            Ok(()) => Ok(Rd(self.new_year_raw(year)
                + days_before_month(layout, month)
                + day as i64
                - 1)),
        }
    }

    /// The year, month and day of a fixed day under `layout`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside the rule's days.
    pub(crate) const fn date_of(self, layout: &[u8; 12], rd: Rd) -> CalendarResult<(i64, u8, u8)> {
        if rd.0 < self.earliest().0 {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd.0 > self.latest().0 {
            return Err(CalendarError::AfterSupportedRange);
        }
        // Dividing by the exact mean year — as the cycle's own ratio, not a
        // float — lands within one year; the corrections below close the gap.
        let mut year = ((rd.0 - self.epoch) * self.cycle).div_euclid(self.cycle_days()) + 1;
        while self.new_year_raw(year) > rd.0 {
            year -= 1;
        }
        while self.new_year_raw(year + 1) <= rd.0 {
            year += 1;
        }
        let day_of_year = rd.0 - self.new_year_raw(year);
        let mut month = 1u8;
        let mut elapsed = 0;
        while month < 12 {
            let length = layout[month as usize - 1] as i64;
            if day_of_year < elapsed + length {
                break;
            }
            elapsed += length;
            month += 1;
        }
        Ok((year, month, (day_of_year - elapsed + 1) as u8))
    }
}

/// Days elapsed in the year before the first of `month`.
const fn days_before_month(layout: &[u8; 12], month: u8) -> i64 {
    let mut total = 0;
    let mut index = 0;
    while index < month as usize - 1 {
        total += layout[index] as i64;
        index += 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const RULE: LeapWeekRule = LeapWeekRule {
        leaps: 71,
        cycle: 400,
        offset: 203,
        epoch: 0,
        min_year: -1_000,
        max_year: 1_000,
    };

    #[test]
    fn the_count_of_leap_years_is_the_count_of_the_rule() {
        let mut counted = 0;
        for year in 1..=900 {
            assert_eq!(RULE.leap_years_before(year), counted, "before {year}");
            if RULE.is_leap_year(year) {
                counted += 1;
            }
        }
        let mut counted = 0;
        for year in (-900..=0).rev() {
            if RULE.is_leap_year(year) {
                counted -= 1;
            }
            assert_eq!(RULE.leap_years_before(year), counted, "before {year}");
        }
    }

    #[test]
    fn a_year_is_as_long_as_the_distance_to_the_next() {
        for year in -999..=999 {
            let length = RULE.new_year_raw(year + 1) - RULE.new_year_raw(year);
            assert_eq!(length, i64::from(RULE.days_in_year(year)), "{year}");
        }
        assert_eq!(
            RULE.new_year_raw(401) - RULE.new_year_raw(1),
            RULE.cycle_days()
        );
    }
}
