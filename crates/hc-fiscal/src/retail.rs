//! 52/53-week reporting years, and the 4-4-5 family built on them.
//!
//! A retailer compares this week's takings with the same week last year, and
//! a calendar month is a bad unit for that: it starts on a different weekday
//! every year, and February has four weekends about three years in seven. So
//! retailers, broadcasters and a great many listed companies report on a year
//! that is a whole number of weeks — 52 of them, or 53 when the drift catches
//! up — divided into twelve "periods" of four or five weeks that are called
//! months but are not.
//!
//! # Why this lives here and not in `hc-holiday`
//!
//! `docs/observances.md` lists "4-4-5, 13-period retail" under what the
//! holiday engine will not do. That judgement is right and it is about
//! holidays: a retail period is not an observance, it names no day, and no
//! country legislates one. It is a *year that does not begin on 1 January*,
//! which is exactly this crate's subject, and it shares this crate's
//! machinery — a start rule, a labelling convention, a validity range, and
//! sub-periods counted from the year's own start rather than from January.
//!
//! # The two rules, both named
//!
//! A 52/53-week year is pinned by saying which weekday it ends on and how
//! that weekday is chosen relative to the end of an anchor month. The two
//! rules are not interchangeable, and they are not this crate's invention:
//! they are the two alternatives set out in Treasury Regulation § 1.441-2,
//! which implements the 52-53 week taxable year of IRC § 441(f). A taxpayer
//! may elect a year that ends always on
//!
//! * "whatever date this same day of the week last occurs in a calendar
//!   month" — [`AnchorRule::LastWeekdayOfMonth`], "the last Saturday in
//!   January". The year end never leaves the anchor month, and can be up to
//!   six days before its last day.
//! * "whatever date this same day of the week falls that is the nearest to
//!   the last day of the calendar month" —
//!   [`AnchorRule::WeekdayNearestMonthEnd`], "the Saturday nearest to
//!   31 January". The year end can fall up to three days into the following
//!   month, which is how a retail year comes to end on 3 February.
//!
//! The regulation's own worked example is the neatest demonstration that
//! these are two rules and not one: for a Saturday year end in November
//! 2001, the "last" rule gives 24 November and the "nearest" rule gives
//! 1 December — different weeks, and different months. This module's tests
//! reproduce it.
//!
//! Per `docs/policy.md` §5 they are two names rather than one parameter with
//! a default, and the same goes for [`PeriodShape`]: 4-4-5, 4-5-4 and 5-4-4
//! are three conventions, not a setting.
//!
//! # The 53rd week
//!
//! Because 52 weeks is 364 days, a week-based year loses one to two days a
//! year against the Gregorian one, and roughly every fifth or sixth year the
//! anchor rule produces a 371-day year. The National Retail Federation
//! states the intercalation as a count rather than a rule about weeks — "if,
//! after laying out the entire 52-week calendar for any given year, there
//! are four or more days left in January during the 53rd week, then a 53rd
//! week is added" — which is the same shape as ISO 8601's, and falls out of
//! the "nearest" anchor rule automatically.
//!
//! The extra week is added to the final period. A year-on-year comparison
//! across a 53-week year is therefore not like-for-like, and
//! [`WeekYearSystem::weeks_in_year`] exists so that a caller can find out
//! before drawing the chart.
//!
//! # A retailer is not necessarily on the retail calendar
//!
//! Walmart reports to a fixed 31 January year end; Apple's 10-K defines its
//! year as ending "on the last Saturday of September", which is the other
//! anchor rule and a different month. The named values here are published
//! conventions, not a lookup table of companies, and a filer's own year
//! should be written out as its own [`WeekYearSystem`].

use hc_calendar::{CalendarError, Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::error::{FiscalError, FiscalResult};
use crate::year_system::{LabelConvention, SourceDate};

/// How the year's final weekday is chosen relative to the anchor month's end.
///
/// The two alternatives of Treasury Regulation § 1.441-2(a)(1)(iii). No
/// `Default`: they put the year end up to a week apart and sometimes in
/// different calendar months, and a caller who has not chosen has not
/// described their calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnchorRule {
    /// "The last Saturday in January" — § 1.441-2(a)(1)(iii)(A). The year
    /// always ends inside the anchor month, up to six days before its last
    /// day. Apple's fiscal year, ending "the last Saturday of September", is
    /// of this kind.
    LastWeekdayOfMonth,
    /// "The Saturday nearest to 31 January" — § 1.441-2(a)(1)(iii)(B). The
    /// year may end up to three days into the following month.
    ///
    /// There is never a tie: the distance to the previous occurrence of the
    /// weekday and the distance to the next always sum to seven, which is
    /// odd, so one of them is strictly smaller.
    WeekdayNearestMonthEnd,
}

impl AnchorRule {
    /// A short English description.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::LastWeekdayOfMonth => "last weekday of the anchor month",
            Self::WeekdayNearestMonthEnd => "weekday nearest the anchor month's end",
        }
    }
}

/// How the thirteen weeks of a quarter are split into three periods.
///
/// No `Default`, per `docs/policy.md` §5. All three sum to thirteen; they
/// differ in which period of the quarter carries the extra week, and a
/// company's choice is usually driven by where its month-end cut-offs and
/// promotional weeks fall.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PeriodShape {
    /// Four weeks, four weeks, five weeks. The oldest and plainest form.
    FourFourFive,
    /// Four, five, four — the National Retail Federation's standard, which
    /// puts the extra week in the middle period so that a quarter's opening
    /// and closing periods are comparable.
    FourFiveFour,
    /// Five, four, four — used where a quarter's selling season falls at its
    /// start.
    FiveFourFour,
}

impl PeriodShape {
    /// The weeks in each of a quarter's three periods.
    #[must_use]
    pub const fn weeks_per_period(self) -> [u8; 3] {
        match self {
            Self::FourFourFive => [4, 4, 5],
            Self::FourFiveFour => [4, 5, 4],
            Self::FiveFourFour => [5, 4, 4],
        }
    }

    /// A short English name, as the convention is written.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::FourFourFive => "4-4-5",
            Self::FourFiveFour => "4-5-4",
            Self::FiveFourFour => "5-4-4",
        }
    }
}

/// A year that is a whole number of weeks.
///
/// The year **ends** on `anchor_weekday`, chosen by `anchor_rule` relative to
/// the last day of `anchor_month`; `label` then says whether the year is
/// named after the Gregorian year that end falls in or the one before it.
/// Anchoring on the end rather than the start is not a stylistic choice: it
/// is how every published definition of these calendars is phrased, and
/// deriving the start from it keeps consecutive years abutting exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeekYearSystem {
    /// The English name.
    pub name: &'static str,
    /// The weekday the year ends on.
    pub anchor_weekday: Weekday,
    /// The Gregorian month whose last day the anchor rule is applied to.
    pub anchor_month: u8,
    /// How the anchor weekday is chosen.
    pub anchor_rule: AnchorRule,
    /// Whether the year is named after the Gregorian year it starts or ends
    /// in.
    pub label: LabelConvention,
    /// How a quarter's thirteen weeks split into three periods, or `None`
    /// for a convention that numbers weeks and defines no periods at all —
    /// which is ISO 8601's position.
    pub shape: Option<PeriodShape>,
    /// What the entry does not claim.
    pub note: &'static str,
    /// The published definition this entry came from.
    pub source: &'static str,
    /// When that source was last checked.
    pub sources_checked: SourceDate,
}

impl WeekYearSystem {
    /// The last day of the year labelled `label`.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::Calendar`] when the anchor month is not 1–12
    /// or the Gregorian year is out of range.
    pub fn year_end(&self, label: i64) -> FiscalResult<Rd> {
        let gregorian_year = match self.label {
            LabelConvention::LabelledByEndYear => label,
            LabelConvention::LabelledByStartYear => {
                label.checked_add(1).ok_or(FiscalError::Overflow)?
            }
        };
        let length = gregorian::days_in_month(gregorian_year, self.anchor_month)
            .ok_or(FiscalError::Calendar(CalendarError::MonthOutOfRange))?;
        let month_end = gregorian::to_fixed(gregorian_year, self.anchor_month, length)?;
        Ok(match self.anchor_rule {
            AnchorRule::LastWeekdayOfMonth => self.anchor_weekday.on_or_before(month_end),
            AnchorRule::WeekdayNearestMonthEnd => {
                let before = self.anchor_weekday.on_or_before(month_end);
                let after = self.anchor_weekday.on_or_after(month_end);
                if month_end.0 - before.0 <= after.0 - month_end.0 {
                    before
                } else {
                    after
                }
            }
        })
    }

    /// The first day of the year labelled `label`, which is the day after
    /// the previous year's end.
    ///
    /// # Errors
    ///
    /// As [`WeekYearSystem::year_end`].
    pub fn year_start(&self, label: i64) -> FiscalResult<Rd> {
        let previous = self.year_end(label.checked_sub(1).ok_or(FiscalError::Overflow)?)?;
        Ok(Rd(previous.0 + 1))
    }

    /// How many weeks the year labelled `label` contains — 52 or 53.
    ///
    /// # Errors
    ///
    /// As [`WeekYearSystem::year_end`]. Returns
    /// [`FiscalError::Calendar`] if the anchor rule somehow produced a span
    /// that is not a whole number of weeks, which it cannot by construction.
    pub fn weeks_in_year(&self, label: i64) -> FiscalResult<u8> {
        let days = self.year_end(label)?.0 - self.year_start(label)?.0 + 1;
        if days % 7 != 0 {
            return Err(FiscalError::Calendar(CalendarError::YearOutOfRange));
        }
        u8::try_from(days / 7).map_err(|_| FiscalError::Calendar(CalendarError::YearOutOfRange))
    }

    /// Whether the year labelled `label` is a 53-week year.
    ///
    /// # Errors
    ///
    /// As [`WeekYearSystem::weeks_in_year`].
    pub fn is_long_year(&self, label: i64) -> FiscalResult<bool> {
        Ok(self.weeks_in_year(label)? == 53)
    }

    /// The label of the year containing `rd`.
    ///
    /// # Errors
    ///
    /// As [`WeekYearSystem::year_end`].
    pub fn label_at(&self, rd: Rd) -> FiscalResult<i64> {
        // The Gregorian year of `rd` is within one of the answer, because a
        // week year never strays more than a few days from its anchor month.
        let gregorian_year = gregorian::year_from_fixed(rd)?;
        let seed = match self.label {
            LabelConvention::LabelledByEndYear => gregorian_year,
            LabelConvention::LabelledByStartYear => gregorian_year - 1,
        };
        for candidate in [seed - 1, seed, seed + 1, seed + 2] {
            if rd <= self.year_end(candidate)? && rd >= self.year_start(candidate)? {
                return Ok(candidate);
            }
        }
        Err(FiscalError::Calendar(CalendarError::YearOutOfRange))
    }

    /// Which week of its year `rd` falls in, counting from 1.
    ///
    /// # Errors
    ///
    /// As [`WeekYearSystem::label_at`].
    pub fn week_of_year(&self, rd: Rd) -> FiscalResult<u8> {
        let label = self.label_at(rd)?;
        let start = self.year_start(label)?;
        u8::try_from((rd.0 - start.0) / 7 + 1)
            .map_err(|_| FiscalError::Calendar(CalendarError::YearOutOfRange))
    }

    /// The span of week `week` (1-based) of the year labelled `label`.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::Calendar`] when the week does not exist in
    /// that year, which is how week 53 of a 52-week year is reported.
    pub fn week_span(&self, label: i64, week: u8) -> FiscalResult<(Rd, Rd)> {
        if week == 0 || week > self.weeks_in_year(label)? {
            return Err(FiscalError::Calendar(CalendarError::DayOutOfRange));
        }
        let start = self.year_start(label)?;
        let first = Rd(start.0 + i64::from(week - 1) * 7);
        Ok((first, Rd(first.0 + 6)))
    }

    /// The weeks in each of the twelve periods of the year labelled `label`.
    ///
    /// In a 53-week year the extra week is added to period 12.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::PeriodsNotDefined`] for a week-only convention
    /// such as ISO 8601.
    pub fn period_lengths(&self, label: i64) -> FiscalResult<[u8; 12]> {
        let shape = self.shape.ok_or(FiscalError::PeriodsNotDefined)?;
        let quarter = shape.weeks_per_period();
        let mut lengths = [0u8; 12];
        for (index, slot) in lengths.iter_mut().enumerate() {
            *slot = quarter[index % 3];
        }
        if self.is_long_year(label)? {
            lengths[11] += 1;
        }
        Ok(lengths)
    }

    /// The span of period `period` (1–12) of the year labelled `label`.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::PeriodsNotDefined`] for a week-only
    /// convention, and [`FiscalError::Calendar`] for a period outside 1–12.
    pub fn period_span(&self, label: i64, period: u8) -> FiscalResult<(Rd, Rd)> {
        if period == 0 || period > 12 {
            return Err(FiscalError::Calendar(CalendarError::MonthOutOfRange));
        }
        let lengths = self.period_lengths(label)?;
        let before: u8 = lengths[..usize::from(period) - 1].iter().sum();
        let (first, _) = self.week_span(label, before + 1)?;
        let (_, last) = self.week_span(label, before + lengths[usize::from(period) - 1])?;
        Ok((first, last))
    }

    /// Which period of its year `rd` falls in, counting from 1.
    ///
    /// # Errors
    ///
    /// As [`WeekYearSystem::period_span`].
    pub fn period_of_year(&self, rd: Rd) -> FiscalResult<u8> {
        let label = self.label_at(rd)?;
        let week = self.week_of_year(rd)?;
        let lengths = self.period_lengths(label)?;
        let mut elapsed = 0u8;
        for (index, length) in lengths.iter().enumerate() {
            elapsed += length;
            if week <= elapsed {
                return u8::try_from(index + 1)
                    .map_err(|_| FiscalError::Calendar(CalendarError::MonthOutOfRange));
            }
        }
        Err(FiscalError::Calendar(CalendarError::MonthOutOfRange))
    }

    /// Which quarter of its year `rd` falls in, counting from 1.
    ///
    /// # Errors
    ///
    /// As [`WeekYearSystem::period_of_year`].
    pub fn quarter_of_year(&self, rd: Rd) -> FiscalResult<u8> {
        Ok((self.period_of_year(rd)? - 1) / 3 + 1)
    }
}

/// The National Retail Federation's 4-5-4 retail calendar.
///
/// The NRF publishes the United States retail year as a 4-5-4 calendar of
/// Sunday-to-Saturday weeks whose year ends on the Saturday nearest to
/// 31 January. Retail 2023 ran from Sunday 29 January 2023 to Saturday
/// 3 February 2024 — 53 weeks — and retail 2024 from Sunday 4 February 2024
/// to Saturday 1 February 2025.
///
/// The calendar is equally often described as starting on the Sunday nearest
/// 1 February. That is the same boundary seen from the other side and not a
/// competing convention: 31 January is the day before 1 February, Saturday
/// is the day before Sunday, and neither "nearest" lookup can tie. The
/// module's tests check both formulations against the published years.
///
/// The retail months, in order from February, are 4-5-4 repeated: February
/// four weeks, March five, April four, and so on to January.
pub static NRF_4_5_4: WeekYearSystem = WeekYearSystem {
    name: "NRF 4-5-4 retail calendar",
    anchor_weekday: Weekday::Saturday,
    anchor_month: 1,
    anchor_rule: AnchorRule::WeekdayNearestMonthEnd,
    label: LabelConvention::LabelledByStartYear,
    shape: Some(PeriodShape::FourFiveFour),
    note: "The standing rule, not a promise about a given future year: the NRF publishes a \
           rolling window and has restated it. Individual retailers deviate — Walmart uses a \
           fixed 31 January year end and Apple the last Saturday of September — so do not \
           assume this calendar for a company merely because it sells things.",
    source: "National Retail Federation, 4-5-4 Calendar; corroborated against Target, Burlington \
             and Shoe Carnival Form 10-K filings",
    sources_checked: SourceDate::new(2026, 9, 21),
};

/// The ISO 8601 week-numbering year, expressed in the same terms.
///
/// ISO 8601 defines week 1 as the week containing the year's first Thursday,
/// with weeks running Monday to Sunday. That is equivalent to saying the year
/// ends on the Sunday nearest to 31 December, which is how it is written
/// here — and the equivalence is checked in this module's tests against
/// `hc_calendars_solar::iso_week`, which implements the standard's own
/// definition directly.
///
/// ISO defines no periods, so [`WeekYearSystem::period_span`] refuses rather
/// than inventing a shape for it.
pub static ISO_8601_WEEK_YEAR: WeekYearSystem = WeekYearSystem {
    name: "ISO 8601 week-numbering year",
    anchor_weekday: Weekday::Sunday,
    anchor_month: 12,
    anchor_rule: AnchorRule::WeekdayNearestMonthEnd,
    label: LabelConvention::LabelledByEndYear,
    shape: None,
    note: "ISO 8601 numbers weeks and defines no months, periods or quarters.",
    source: "ISO 8601-1:2019, 4.2.2.6 (week-based year)",
    sources_checked: SourceDate::new(2026, 9, 21),
};

/// A 4-4-5 year ending on the last Saturday of December.
///
/// Included because it is the other rule — [`AnchorRule::LastWeekdayOfMonth`]
/// rather than [`AnchorRule::WeekdayNearestMonthEnd`] — and because a
/// calendar-aligned 4-4-5 is the commonest shape among filers who want a
/// week-based year without moving away from a December year end. It is a
/// shape, not a particular company's calendar; no source names it as a
/// national or industry standard, and the entry says so.
pub static LAST_SATURDAY_OF_DECEMBER_4_4_5: WeekYearSystem = WeekYearSystem {
    name: "4-4-5 year ending the last Saturday of December",
    anchor_weekday: Weekday::Saturday,
    anchor_month: 12,
    anchor_rule: AnchorRule::LastWeekdayOfMonth,
    label: LabelConvention::LabelledByEndYear,
    shape: Some(PeriodShape::FourFourFive),
    note: "A widely used shape rather than a standard. No authority publishes it; \
           a filer's own 52-53 week year should be written out as its own value.",
    source: "Treas. Reg. § 1.441-2(a)(1)(iii)(A) for the anchor rule; SEC Financial Reporting \
             Manual § 1365.7 for how a 52-53 week year is treated on a change of year end",
    sources_checked: SourceDate::new(2026, 9, 21),
};

/// Every named week-year convention in this crate.
pub static ALL: &[&WeekYearSystem] = &[
    &NRF_4_5_4,
    &ISO_8601_WEEK_YEAR,
    &LAST_SATURDAY_OF_DECEMBER_4_4_5,
];

#[cfg(test)]
mod tests {
    use hc_calendars_solar::iso_week;

    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_nrf_retail_year_2023_ran_from_january_29_to_february_3() {
        // Published by the National Retail Federation: retail 2023 begins
        // Sunday 29 January 2023 and ends Saturday 3 February 2024.
        assert_eq!(NRF_4_5_4.year_start(2023).unwrap(), greg(2023, 1, 29));
        assert_eq!(NRF_4_5_4.year_end(2023).unwrap(), greg(2024, 2, 3));
    }

    #[test]
    fn nrf_retail_2023_is_a_fifty_three_week_year() {
        assert_eq!(NRF_4_5_4.weeks_in_year(2023).unwrap(), 53);
        assert!(NRF_4_5_4.is_long_year(2023).unwrap());
        let span = NRF_4_5_4.year_end(2023).unwrap().0 - NRF_4_5_4.year_start(2023).unwrap().0 + 1;
        assert_eq!(span, 371);
    }

    #[test]
    fn the_neighbouring_nrf_years_are_ordinary_fifty_two_week_ones() {
        assert_eq!(NRF_4_5_4.year_start(2022).unwrap(), greg(2022, 1, 30));
        assert_eq!(NRF_4_5_4.year_end(2022).unwrap(), greg(2023, 1, 28));
        assert_eq!(NRF_4_5_4.weeks_in_year(2022).unwrap(), 52);
        assert_eq!(NRF_4_5_4.year_start(2024).unwrap(), greg(2024, 2, 4));
        assert_eq!(NRF_4_5_4.year_end(2024).unwrap(), greg(2025, 2, 1));
        assert_eq!(NRF_4_5_4.weeks_in_year(2024).unwrap(), 52);
    }

    #[test]
    fn a_fifty_three_week_year_puts_the_extra_week_in_the_last_period() {
        let long = NRF_4_5_4.period_lengths(2023).unwrap();
        assert_eq!(long, [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 5]);
        assert_eq!(long.iter().map(|&n| u32::from(n)).sum::<u32>(), 53);
        let short = NRF_4_5_4.period_lengths(2024).unwrap();
        assert_eq!(short, [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4]);
        assert_eq!(short.iter().map(|&n| u32::from(n)).sum::<u32>(), 52);
    }

    #[test]
    fn the_twelve_periods_tile_the_week_year_exactly() {
        for label in 1990..2060 {
            let start = NRF_4_5_4.year_start(label).unwrap();
            let end = NRF_4_5_4.year_end(label).unwrap();
            let mut cursor = start;
            for period in 1..=12 {
                let (first, last) = NRF_4_5_4.period_span(label, period).unwrap();
                assert_eq!(first, cursor, "period {period} of {label}");
                assert_eq!((last.0 - first.0 + 1) % 7, 0);
                cursor = Rd(last.0 + 1);
            }
            assert_eq!(cursor, Rd(end.0 + 1));
        }
    }

    #[test]
    fn every_week_year_is_fifty_two_or_fifty_three_weeks_long() {
        for system in ALL {
            for label in 1900..2100 {
                let weeks = system.weeks_in_year(label).unwrap();
                assert!(weeks == 52 || weeks == 53, "{} gave {weeks}", system.name);
            }
        }
    }

    #[test]
    fn consecutive_week_years_abut_without_a_gap() {
        for system in ALL {
            for label in 1950..2050 {
                assert_eq!(
                    system.year_start(label + 1).unwrap().0,
                    system.year_end(label).unwrap().0 + 1,
                    "{}",
                    system.name
                );
            }
        }
    }

    #[test]
    fn every_week_year_starts_the_day_after_the_anchor_weekday() {
        for system in ALL {
            for label in 1980..2040 {
                assert_eq!(
                    Weekday::from_rd(system.year_end(label).unwrap()),
                    system.anchor_weekday,
                    "{}",
                    system.name
                );
            }
        }
    }

    #[test]
    fn the_iso_week_year_written_as_an_anchor_rule_matches_the_standard() {
        // `hc_calendars_solar::iso_week` implements ISO 8601's own
        // definition — week 1 is the week containing the first Thursday.
        // Expressing the same year as "ends on the Sunday nearest
        // 31 December" must give day-for-day the same answer.
        for year in 1800..2200 {
            assert_eq!(
                ISO_8601_WEEK_YEAR.year_start(year).unwrap(),
                iso_week::week_one_start(year).unwrap(),
                "ISO year {year}"
            );
            assert_eq!(
                ISO_8601_WEEK_YEAR.weeks_in_year(year).unwrap(),
                iso_week::weeks_in_year(year).unwrap(),
                "ISO year {year}"
            );
        }
    }

    #[test]
    fn the_iso_week_number_of_a_day_matches_the_standard() {
        for day in greg(1995, 1, 1).0..=greg(2035, 12, 31).0 {
            let rd = Rd(day);
            let (year, week, _) = iso_week::from_fixed(rd).unwrap();
            assert_eq!(ISO_8601_WEEK_YEAR.label_at(rd).unwrap(), year);
            assert_eq!(ISO_8601_WEEK_YEAR.week_of_year(rd).unwrap(), week);
        }
    }

    #[test]
    fn the_surprising_iso_boundary_days_come_out_right() {
        // The two cases `hc_calendars_solar::iso_week` calls out:
        // 2021-01-01 is 2021-W53-5 of ISO year 2020, and 2019-12-30 is
        // 2020-W01-1.
        assert_eq!(ISO_8601_WEEK_YEAR.label_at(greg(2021, 1, 1)).unwrap(), 2020);
        assert_eq!(
            ISO_8601_WEEK_YEAR.week_of_year(greg(2021, 1, 1)).unwrap(),
            53
        );
        assert_eq!(
            ISO_8601_WEEK_YEAR.label_at(greg(2019, 12, 30)).unwrap(),
            2020
        );
        assert_eq!(
            ISO_8601_WEEK_YEAR.week_of_year(greg(2019, 12, 30)).unwrap(),
            1
        );
    }

    #[test]
    fn iso_8601_defines_no_periods_and_the_type_says_so() {
        assert_eq!(
            ISO_8601_WEEK_YEAR.period_lengths(2024),
            Err(FiscalError::PeriodsNotDefined)
        );
        assert_eq!(
            ISO_8601_WEEK_YEAR.period_span(2024, 1),
            Err(FiscalError::PeriodsNotDefined)
        );
        assert_eq!(
            ISO_8601_WEEK_YEAR.period_of_year(greg(2024, 6, 1)),
            Err(FiscalError::PeriodsNotDefined)
        );
    }

    #[test]
    fn the_two_anchor_rules_give_different_years_for_the_same_weekday() {
        // 31 January 2024 was a Wednesday, so the nearest Saturday is
        // 3 February, three days later, while the last Saturday in January
        // is 27 January — a week apart.
        let nearest = NRF_4_5_4;
        let last = WeekYearSystem {
            anchor_rule: AnchorRule::LastWeekdayOfMonth,
            ..NRF_4_5_4
        };
        assert_eq!(nearest.year_end(2023).unwrap(), greg(2024, 2, 3));
        assert_eq!(last.year_end(2023).unwrap(), greg(2024, 1, 27));
        assert_eq!(nearest.weeks_in_year(2023).unwrap(), 53);
        assert_eq!(last.weeks_in_year(2023).unwrap(), 52);
    }

    #[test]
    fn the_last_weekday_rule_never_leaves_the_anchor_month() {
        for label in 1950..2050 {
            let end = LAST_SATURDAY_OF_DECEMBER_4_4_5.year_end(label).unwrap();
            let (year, month, _) = gregorian::from_fixed(end).unwrap();
            assert_eq!(month, 12);
            assert_eq!(year, label);
        }
    }

    #[test]
    fn the_nearest_rule_does_leave_the_anchor_month_sometimes() {
        let escapes = (1990..2060)
            .filter(|&label| {
                let end = NRF_4_5_4.year_end(label).unwrap();
                gregorian::from_fixed(end).unwrap().1 == 2
            })
            .count();
        assert!(
            escapes > 0,
            "the nearest rule should sometimes reach February"
        );
    }

    #[test]
    fn the_three_period_shapes_all_make_thirteen_week_quarters() {
        for shape in [
            PeriodShape::FourFourFive,
            PeriodShape::FourFiveFour,
            PeriodShape::FiveFourFour,
        ] {
            let weeks = shape.weeks_per_period();
            assert_eq!(
                u32::from(weeks[0]) + u32::from(weeks[1]) + u32::from(weeks[2]),
                13
            );
        }
        assert_eq!(PeriodShape::FourFiveFour.english_name(), "4-5-4");
        assert_eq!(
            AnchorRule::LastWeekdayOfMonth.english_name(),
            "last weekday of the anchor month"
        );
    }

    #[test]
    fn a_day_agrees_with_the_week_period_and_quarter_it_is_said_to_be_in() {
        for day in greg(2015, 1, 1).0..=greg(2035, 12, 31).0 {
            let rd = Rd(day);
            let label = NRF_4_5_4.label_at(rd).unwrap();
            let week = NRF_4_5_4.week_of_year(rd).unwrap();
            let (first, last) = NRF_4_5_4.week_span(label, week).unwrap();
            assert!(rd >= first && rd <= last);
            let period = NRF_4_5_4.period_of_year(rd).unwrap();
            let (pfirst, plast) = NRF_4_5_4.period_span(label, period).unwrap();
            assert!(rd >= pfirst && rd <= plast);
            let quarter = NRF_4_5_4.quarter_of_year(rd).unwrap();
            assert_eq!((period - 1) / 3 + 1, quarter);
            assert!((1..=4).contains(&quarter));
        }
    }

    #[test]
    fn a_week_that_does_not_exist_is_refused() {
        assert_eq!(
            NRF_4_5_4.week_span(2024, 53),
            Err(FiscalError::Calendar(CalendarError::DayOutOfRange))
        );
        assert!(NRF_4_5_4.week_span(2023, 53).is_ok());
        assert_eq!(
            NRF_4_5_4.week_span(2024, 0),
            Err(FiscalError::Calendar(CalendarError::DayOutOfRange))
        );
        assert_eq!(
            NRF_4_5_4.period_span(2024, 13),
            Err(FiscalError::Calendar(CalendarError::MonthOutOfRange))
        );
    }

    #[test]
    fn the_published_nrf_retail_years_all_come_out_right() {
        // Published by the National Retail Federation and corroborated
        // against Form 10-K filings by Target, Burlington and Shoe Carnival.
        let published = [
            (2022, (2022, 1, 30), (2023, 1, 28), 52),
            (2023, (2023, 1, 29), (2024, 2, 3), 53),
            (2024, (2024, 2, 4), (2025, 2, 1), 52),
            (2025, (2025, 2, 2), (2026, 1, 31), 52),
            (2026, (2026, 2, 1), (2027, 1, 30), 52),
            (2027, (2027, 1, 31), (2028, 1, 29), 52),
        ];
        for (label, start, end, weeks) in published {
            assert_eq!(
                NRF_4_5_4.year_start(label).unwrap(),
                greg(start.0, start.1, start.2),
                "start of retail {label}"
            );
            assert_eq!(
                NRF_4_5_4.year_end(label).unwrap(),
                greg(end.0, end.1, end.2),
                "end of retail {label}"
            );
            assert_eq!(NRF_4_5_4.weeks_in_year(label).unwrap(), weeks);
        }
    }

    #[test]
    fn the_nrf_53_week_years_are_the_ones_the_federation_names() {
        // The NRF names 2006, 2012, 2017, 2023 and 2028 as the 53-week
        // years of this era.
        // The window checked is the one the NRF publishes; the rule
        // itself extends further back and produces 2000 as well, which the
        // federation's current calendar does not reach.
        let named = [2006, 2012, 2017, 2023, 2028];
        let mut found = 0;
        for label in 2006..=2030 {
            let long = NRF_4_5_4.is_long_year(label).unwrap();
            assert_eq!(long, named.contains(&label), "retail {label}");
            found += u32::from(long);
        }
        assert_eq!(found, named.len() as u32);
    }

    #[test]
    fn the_sunday_nearest_february_first_is_the_same_boundary() {
        // The NRF calendar is described both as ending on the Saturday
        // nearest 31 January and as starting on the Sunday nearest
        // 1 February. Those are one boundary seen from two sides, and
        // neither lookup can tie, so they must agree on every year.
        let from_the_other_side = WeekYearSystem {
            anchor_weekday: Weekday::Saturday,
            anchor_month: 1,
            ..NRF_4_5_4
        };
        for label in 1950..2100 {
            let start = NRF_4_5_4.year_start(label).unwrap();
            assert_eq!(Weekday::from_rd(start), Weekday::Sunday);
            let first_february = greg(label + 1, 2, 1);
            let end = from_the_other_side.year_end(label).unwrap();
            // The Sunday nearest 1 February of the following year is the
            // day after that year's end.
            let candidate_before = Weekday::Sunday.on_or_before(first_february);
            let candidate_after = Weekday::Sunday.on_or_after(first_february);
            let nearest =
                if first_february.0 - candidate_before.0 <= candidate_after.0 - first_february.0 {
                    candidate_before
                } else {
                    candidate_after
                };
            assert_eq!(nearest.0, end.0 + 1, "retail {label}");
        }
    }

    #[test]
    fn the_regulations_own_november_2001_example_distinguishes_the_rules() {
        // Treas. Reg. § 1.441-2(a)(4): for a Saturday year end in November
        // 2001, "last Saturday of November" gives 24 November 2001 and
        // "Saturday nearest the last day of November" gives 1 December 2001.
        let last = WeekYearSystem {
            anchor_weekday: Weekday::Saturday,
            anchor_month: 11,
            anchor_rule: AnchorRule::LastWeekdayOfMonth,
            label: LabelConvention::LabelledByEndYear,
            ..NRF_4_5_4
        };
        let nearest = WeekYearSystem {
            anchor_rule: AnchorRule::WeekdayNearestMonthEnd,
            ..last
        };
        assert_eq!(last.year_end(2001).unwrap(), greg(2001, 11, 24));
        assert_eq!(nearest.year_end(2001).unwrap(), greg(2001, 12, 1));
    }

    #[test]
    fn a_fifty_three_week_year_is_never_two_in_a_row() {
        for system in ALL {
            for label in 1900..2100 {
                if system.is_long_year(label).unwrap() {
                    assert!(
                        !system.is_long_year(label + 1).unwrap(),
                        "{} claimed two long years running at {label}",
                        system.name
                    );
                }
            }
        }
    }

    #[test]
    fn every_named_week_year_carries_a_source_and_a_check_date() {
        for system in ALL {
            assert!(!system.name.is_empty());
            assert!(!system.source.is_empty(), "{}", system.name);
            assert!(system.sources_checked.year >= 2026, "{}", system.name);
        }
    }
}

/// The system with this name.
#[must_use]
pub fn by_name(name: &str) -> Option<&'static WeekYearSystem> {
    ALL.iter().copied().find(|system| system.name == name)
}

hc_core::catalogue_tests! {
    type: &'static WeekYearSystem,
    id: |system| system.name,
    provenance: |system| system.source,
    tests: system_table_tests,
    all: ALL,
    lookup: by_name,
}
