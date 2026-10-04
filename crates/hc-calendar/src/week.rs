//! Week rules: which day a week starts on and which week is a year's or a
//! month's first, and the week numbers they give to the Gregorian year.
//!
//! A week number depends on two facts about the place that writes it, and
//! UTS #35 Part 4, "Week of Year" and "Week Elements", gives both: the
//! `firstDay` of the week and `minDays`, "the minimal days required in the
//! first week of a month or year". "Week 1 for a year is the first week that
//! contains at least the specified minimum number of days from that year.
//! Weeks between week 1 of one year and week 1 of the following year are
//! numbered sequentially from 2 to 52 or 53." The spec's example: "January 1,
//! 1998 was a Thursday. If the first day of the week is MONDAY and the
//! minimum days in a week is 4 (these are the values reflecting ISO 8601 and
//! many national standards), then week 1 of 1998 starts on December 29, 1997,
//! and ends on January 4, 1998. However, if the first day of the week is
//! SUNDAY, then week 1 of 1998 starts on January 4, 1998, and ends on
//! January 10, 1998. The first three days of 1998 are then part of week 53
//! of 1997." "Values are similarly calculated for the Week of Month": a week
//! counts as part of a month where it has `minDays` days in it.
//!
//! The United States' Sunday and 1 and Germany's Monday and 4 are two
//! places on one scale, and ISO 8601's Monday and 4 is one of its points:
//! [`WeekRule::ISO`]. The arithmetic is written here once, for every week
//! number the workspace gives: `hc-calendars-solar`'s ISO week-date calendar
//! counts its weeks by [`WeekRule::ISO`], and `hc-i18n`'s `week::for_locale`
//! reads a locale's rule from CLDR's `weekData` and hands it back as one of
//! these. The rule is Gregorian arithmetic only, so it numbers the weeks of
//! the Gregorian year and month.
//!
//! `docs/systems/week-rules.md` works a date through the rule and says how it
//! was measured.
//!
//! # Days before the first week of a month
//!
//! A day that falls in the partial week before the first week of its month
//! has week of month 0. UTS #35 says such a week "will count as" the
//! previous period's and numbers nothing; ICU4J's `Calendar.weekNumber`,
//! which `WEEK_OF_MONTH` uses (read 2026-10-03, a comparison and not a
//! source of the rule), returns 0 for it (`icu4j-calendar-week-number` in
//! `docs/references.bib`), and so does this. The week of year has no such
//! case: a day before week 1 is in the last week of the year before.
//!
//! # Sources
//!
//! UTS #35 Part 4, version 48.2, "Week of Year" and "Week Elements" (read
//! 2026-10-03; `uts35-v48` in `docs/references.bib`).

use crate::fixed::Rd;
use crate::gregorian;
use crate::weekday::Weekday;

/// A week rule: the first day of the week and the fewest days of a year or
/// month that a week needs to be its first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WeekRule {
    first_day: Weekday,
    min_days: u8,
}

impl WeekRule {
    /// ISO 8601's rule: weeks begin on Monday and week 1 is the one with at
    /// least four days of the year, the one holding 4 January.
    pub const ISO: Self = Self::new(Weekday::Monday, 4);

    /// The world's rule in CLDR 48's `weekData` (region `001`): Monday and
    /// one day.
    pub const WORLD: Self = Self::new(Weekday::Monday, 1);

    /// A rule. `min_days` is kept within 1 to 7, the range `minDays` allows.
    #[must_use]
    pub const fn new(first_day: Weekday, min_days: u8) -> Self {
        let min_days = if min_days < 1 {
            1
        } else if min_days > 7 {
            7
        } else {
            min_days
        };
        Self {
            first_day,
            min_days,
        }
    }

    /// The day the week begins on.
    #[must_use]
    pub const fn first_day(self) -> Weekday {
        self.first_day
    }

    /// The fewest days of a year or month in a week that is its first.
    #[must_use]
    pub const fn min_days(self) -> u8 {
        self.min_days
    }

    /// A weekday's place in the week, counted from the first day as 0: with
    /// Sunday first, Sunday is 0 and Saturday 6.
    #[must_use]
    pub const fn position(self, weekday: Weekday) -> u8 {
        (weekday.iso_number() + 7 - self.first_day.iso_number()) % 7
    }

    /// The first day of week 1 of a Gregorian year.
    ///
    /// The week holding 1 January starts on or before it; it is week 1 where
    /// it has `min_days` days of the year, else week 1 is the next one.
    /// The arithmetic is [`gregorian::new_year`]'s, plain `i64`, which
    /// holds one year either side of the Gregorian range; a caller that
    /// takes a year from outside the library checks
    /// [`gregorian::year_in_range`] first.
    #[must_use]
    pub const fn week_one_start(self, year: i64) -> Rd {
        let first = gregorian::new_year(year);
        let start = self.first_day.on_or_before(first);
        let days_in_year = 7 - (first.0 - start.0);
        if days_in_year >= self.min_days as i64 {
            start
        } else {
            Rd(start.0 + 7)
        }
    }

    /// The week-numbering year and the week of the year of a day: the week
    /// counted from week 1 of the year, and the year whose week 1 it follows.
    ///
    /// A day before week 1 of its Gregorian year is in the last week of the
    /// year before, and one on or after week 1 of the next year is in that
    /// year's week 1: 1 January 2021 is week 1 of 2021 for Sunday and 1 and
    /// week 53 of 2020 for Monday and 4.
    #[must_use]
    pub const fn week_of_year(self, day: Rd) -> (i64, u8) {
        let year = gregorian::year_from_fixed(day);
        let next = self.week_one_start(year + 1).0;
        let this = self.week_one_start(year).0;
        let (week_year, start) = if day.0 >= next {
            (year + 1, next)
        } else if day.0 >= this {
            (year, this)
        } else {
            (year - 1, self.week_one_start(year - 1).0)
        };
        (week_year, ((day.0 - start) / 7 + 1) as u8)
    }

    /// The number of weeks of a week-numbering year, 52 or 53.
    #[must_use]
    pub const fn weeks_in_year(self, year: i64) -> u8 {
        ((self.week_one_start(year + 1).0 - self.week_one_start(year).0) / 7) as u8
    }

    /// The week of the month of a day, 1 for the month's first week: the one
    /// with `min_days` days of the month, counted from the first day of the
    /// week. The days of a shorter partial week before it are week 0.
    #[must_use]
    pub const fn week_of_month(self, day: Rd) -> u8 {
        let (_, _, day_of_month) = gregorian::ymd(day);
        let first = Rd(day.0 - (day_of_month as i64 - 1));
        let offset = self.position(Weekday::from_rd(first)) as i64;
        let mut week = (day_of_month as i64 - 1 + offset) / 7;
        if 7 - offset >= self.min_days as i64 {
            week += 1;
        }
        week as u8
    }

    /// The day with a week-numbering year, a week and a weekday: the first
    /// day of week 1, a week later for each week after it, and the weekday's
    /// place in the week.
    ///
    /// A week past the year's last is the arithmetic's: week 53 of a year
    /// of 52 weeks is the next year's week 1, as UTS #35 asks a parser to
    /// read it.
    #[must_use]
    pub const fn to_fixed(self, year: i64, week: u8, weekday: Weekday) -> Rd {
        Rd(self.week_one_start(year).0 + 7 * (week as i64 - 1) + self.position(weekday) as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    /// UTS #35 Part 4, "Week of Year": 1 January 1998 was a Thursday; with
    /// Monday and four days week 1 of 1998 runs from 29 December 1997 to 4
    /// January 1998, and with Sunday it starts on 4 January 1998 and ends on
    /// the 10th, "the first three days of 1998 are then part of week 53 of
    /// 1997".
    #[test]
    fn the_specs_1998_example_holds() {
        let iso = WeekRule::new(Weekday::Monday, 4);
        assert_eq!(iso.week_one_start(1998), day(1997, 12, 29));
        assert_eq!(iso.week_of_year(day(1998, 1, 4)), (1998, 1));
        assert_eq!(iso.week_of_year(day(1998, 1, 5)), (1998, 2));
        let sunday = WeekRule::new(Weekday::Sunday, 4);
        assert_eq!(sunday.week_one_start(1998), day(1998, 1, 4));
        assert_eq!(sunday.week_of_year(day(1998, 1, 10)), (1998, 1));
        for date in 1..=3 {
            assert_eq!(sunday.week_of_year(day(1998, 1, date)), (1997, 53));
        }
        assert_eq!(sunday.weeks_in_year(1997), 53);
    }

    /// ISO 8601: 2021-01-01 is in week 53 of 2020 and 2019-12-30 in week 1
    /// of 2020 (the standard's own example, and Python's
    /// `date.isocalendar()`).
    #[test]
    fn the_iso_rule_gives_iso_weeks() {
        assert_eq!(WeekRule::ISO.week_of_year(day(2021, 1, 1)), (2020, 53));
        assert_eq!(WeekRule::ISO.week_of_year(day(2019, 12, 30)), (2020, 1));
        assert_eq!(WeekRule::ISO.week_of_year(day(2026, 9, 21)), (2026, 39));
        assert_eq!(WeekRule::ISO.weeks_in_year(2020), 53);
        assert_eq!(WeekRule::ISO.weeks_in_year(2021), 52);
    }

    /// `min_days` is kept to the range `minDays` allows.
    #[test]
    fn the_minimal_days_are_kept_within_one_to_seven() {
        assert_eq!(WeekRule::new(Weekday::Monday, 0).min_days(), 1);
        assert_eq!(WeekRule::new(Weekday::Monday, 9).min_days(), 7);
        assert_eq!(
            WeekRule::new(Weekday::Sunday, 4).first_day(),
            Weekday::Sunday
        );
        assert_eq!(
            WeekRule::new(Weekday::Sunday, 4).position(Weekday::Saturday),
            6
        );
    }

    #[test]
    fn a_week_date_round_trips() {
        for rule in [
            WeekRule::ISO,
            WeekRule::WORLD,
            WeekRule::new(Weekday::Sunday, 1),
            WeekRule::new(Weekday::Saturday, 7),
            WeekRule::new(Weekday::Friday, 3),
        ] {
            for step in 0..800 {
                let date = Rd(day(2019, 6, 1).0 + step);
                let (year, week) = rule.week_of_year(date);
                assert_eq!(
                    rule.to_fixed(year, week, Weekday::from_rd(date)),
                    date,
                    "{rule:?} {date:?}"
                );
            }
        }
    }

    /// Week of month by ICU4J's `Calendar.weekNumber`, which is 0 for the
    /// days before the first week. October 2026 starts on a Thursday and
    /// November 2026 on a Sunday: Monday and four starts October's week 1 on
    /// 1 October (four days) and leaves 1 November alone in week 0, while
    /// Sunday and 1 numbers 1 November week 1.
    #[test]
    fn the_week_of_the_month_counts_from_its_first_week() {
        let de = WeekRule::ISO;
        assert_eq!(de.week_of_month(day(2026, 10, 1)), 1);
        assert_eq!(de.week_of_month(day(2026, 10, 4)), 1);
        assert_eq!(de.week_of_month(day(2026, 10, 5)), 2);
        assert_eq!(de.week_of_month(day(2026, 10, 31)), 5);
        assert_eq!(de.week_of_month(day(2026, 11, 1)), 0);
        assert_eq!(de.week_of_month(day(2026, 11, 2)), 1);
        let us = WeekRule::new(Weekday::Sunday, 1);
        assert_eq!(us.week_of_month(day(2026, 11, 1)), 1);
        assert_eq!(us.week_of_month(day(2026, 11, 7)), 1);
        assert_eq!(us.week_of_month(day(2026, 11, 8)), 2);
        assert_eq!(us.week_of_month(day(2026, 10, 1)), 1);
        assert_eq!(us.week_of_month(day(2026, 10, 4)), 2);
    }
}
