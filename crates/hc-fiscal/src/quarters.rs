//! Quarters, halves and months *of the fiscal year*.
//!
//! Q1 of a Japanese fiscal year is April, May and June. Q1 of a United
//! States federal fiscal year is October, November and December. Neither is
//! Q1 of the calendar year, and a chart that mixes them is wrong in a way
//! that looks right. So nothing here counts from January: every period is
//! counted from the system's own start, and
//! [`YearSystem::calendar_months_of_quarter`] exists precisely so that the
//! calendar months a quarter covers can be printed rather than assumed.
//!
//! # Fiscal months are not always calendar months
//!
//! The United Kingdom's personal tax year begins on 6 April, and HMRC's tax
//! months run from the 6th to the 5th: tax month 1 is 6 April to 5 May. This
//! module generalises that rather than special-casing it. Fiscal month *n*
//! runs from the start day-of-month in the *n*-th month after the start to
//! the day before the same day-of-month in the month after. Where the start
//! is the first of a month — Japan, the United States, India — that reduces
//! to the calendar months, which is why nobody notices the general rule is
//! there.
//!
//! # What this module refuses
//!
//! A year that is not twelve months of equal standing does not divide into
//! four quarters, and this module will not pretend otherwise. The Ethiopic
//! calendar is twelve thirty-day months plus Pagumen; no source this crate
//! could find says which quarter of the Ethiopian budget year Pagumen falls
//! in, so every method here returns [`FiscalError::PeriodsNotDefined`] for
//! an Ethiopic-anchored system. The year's span and its day numbering are
//! still available.

use hc_calendar::{CalendarError, Rd};

use crate::error::{FiscalError, FiscalResult};
use crate::year_system::YearSystem;

/// A quarter of a fiscal year, counted from the fiscal year's own start.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Quarter {
    /// Months 1–3 of the fiscal year.
    First,
    /// Months 4–6.
    Second,
    /// Months 7–9.
    Third,
    /// Months 10–12.
    Fourth,
}

impl Quarter {
    /// Every quarter, in order.
    pub const ALL: [Self; 4] = [Self::First, Self::Second, Self::Third, Self::Fourth];

    /// The quarter's number, 1 to 4.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        match self {
            Self::First => 1,
            Self::Second => 2,
            Self::Third => 3,
            Self::Fourth => 4,
        }
    }

    /// The quarter numbered `ordinal`, or `None` outside 1–4.
    #[must_use]
    pub const fn from_ordinal(ordinal: u8) -> Option<Self> {
        match ordinal {
            1 => Some(Self::First),
            2 => Some(Self::Second),
            3 => Some(Self::Third),
            4 => Some(Self::Fourth),
            _ => None,
        }
    }

    /// The half of the year this quarter falls in.
    #[must_use]
    pub const fn half(self) -> Half {
        match self {
            Self::First | Self::Second => Half::First,
            Self::Third | Self::Fourth => Half::Second,
        }
    }

    /// The fiscal months this quarter covers, counting from 1.
    #[must_use]
    pub const fn fiscal_months(self) -> [u8; 3] {
        let base = (self.ordinal() - 1) * 3;
        [base + 1, base + 2, base + 3]
    }
}

/// A half of a fiscal year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Half {
    /// Months 1–6 of the fiscal year. Japan calls this 上期.
    First,
    /// Months 7–12. Japan calls this 下期.
    Second,
}

impl Half {
    /// Both halves, in order.
    pub const ALL: [Self; 2] = [Self::First, Self::Second];

    /// The half's number, 1 or 2.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        match self {
            Self::First => 1,
            Self::Second => 2,
        }
    }

    /// The half numbered `ordinal`, or `None` outside 1–2.
    #[must_use]
    pub const fn from_ordinal(ordinal: u8) -> Option<Self> {
        match ordinal {
            1 => Some(Self::First),
            2 => Some(Self::Second),
            _ => None,
        }
    }

    /// The two quarters this half covers.
    #[must_use]
    pub const fn quarters(self) -> [Quarter; 2] {
        match self {
            Self::First => [Quarter::First, Quarter::Second],
            Self::Second => [Quarter::Third, Quarter::Fourth],
        }
    }
}

/// A stretch of days shorter than a year — a month, a quarter, a half.
///
/// Both ends are inclusive, as in
/// [`FiscalSpan`](crate::year_system::FiscalSpan).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeriodSpan {
    /// The first day of the period.
    pub first: Rd,
    /// The last day of the period.
    pub last: Rd,
}

impl PeriodSpan {
    /// How many days the period contains.
    #[must_use]
    pub const fn days(&self) -> i64 {
        self.last.0 - self.first.0 + 1
    }

    /// Whether `rd` falls inside the period.
    #[must_use]
    pub const fn contains(&self, rd: Rd) -> bool {
        rd.0 >= self.first.0 && rd.0 <= self.last.0
    }
}

impl YearSystem {
    /// The number of months the start calendar divides a year into, or an
    /// error when it divides a year into no such thing.
    fn month_count(&self) -> FiscalResult<i64> {
        self.start
            .calendar
            .months_of_equal_standing()
            .map(i64::from)
            .ok_or(FiscalError::PeriodsNotDefined)
    }

    /// The first day of fiscal month `month` (1–12) of the year labelled
    /// `label`, ignoring the validity range.
    fn projected_month_start(&self, label: i64, month: u8) -> FiscalResult<Rd> {
        let count = self.month_count()?;
        if month < 1 || i64::from(month) > count {
            return Err(FiscalError::Calendar(CalendarError::MonthOutOfRange));
        }
        let start_year = self.label.start_calendar_year(label);
        let elapsed = i64::from(self.start.month) - 1 + i64::from(month) - 1;
        let calendar_year = start_year + elapsed.div_euclid(count);
        let calendar_month = u8::try_from(elapsed.rem_euclid(count) + 1)
            .map_err(|_| FiscalError::Calendar(CalendarError::MonthOutOfRange))?;
        self.start
            .calendar
            .to_fixed(calendar_year, calendar_month, self.start.day)
    }

    /// The calendar months of the start calendar that `quarter` covers.
    ///
    /// Japan's Q1 is `[4, 5, 6]`; the United States federal Q1 is
    /// `[10, 11, 12]`. This is the method to reach for when a report has to
    /// name the months rather than number the quarter.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::PeriodsNotDefined`] when the start calendar is
    /// not twelve months of equal standing.
    pub fn calendar_months_of_quarter(&self, quarter: Quarter) -> FiscalResult<[u8; 3]> {
        let count = self.month_count()?;
        let mut months = [0u8; 3];
        for (slot, fiscal_month) in months.iter_mut().zip(quarter.fiscal_months()) {
            let elapsed = i64::from(self.start.month) - 1 + i64::from(fiscal_month) - 1;
            *slot = u8::try_from(elapsed.rem_euclid(count) + 1)
                .map_err(|_| FiscalError::Calendar(CalendarError::MonthOutOfRange))?;
        }
        Ok(months)
    }

    /// The span of fiscal month `month` (1–12) of the year labelled `label`.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::OutsideValidity`] outside the system's range,
    /// [`FiscalError::PeriodsNotDefined`] for a calendar without twelve
    /// equal months, and [`FiscalError::Calendar`] for a month outside 1–12
    /// or a start day that does not exist in the month reached.
    pub fn fiscal_month_span(&self, label: i64, month: u8) -> FiscalResult<PeriodSpan> {
        if !self.covers(label) {
            return Err(FiscalError::OutsideValidity);
        }
        let count = self.month_count()?;
        let first = self.projected_month_start(label, month)?;
        let next = if i64::from(month) == count {
            self.projected_start(label.checked_add(1).ok_or(FiscalError::Overflow)?)?
        } else {
            self.projected_month_start(label, month + 1)?
        };
        Ok(PeriodSpan {
            first,
            last: Rd(next.0 - 1),
        })
    }

    /// The span of `quarter` in the year labelled `label`.
    ///
    /// # Errors
    ///
    /// As [`YearSystem::fiscal_month_span`].
    pub fn quarter_span(&self, label: i64, quarter: Quarter) -> FiscalResult<PeriodSpan> {
        let months = quarter.fiscal_months();
        let first = self.fiscal_month_span(label, months[0])?.first;
        let last = self.fiscal_month_span(label, months[2])?.last;
        Ok(PeriodSpan { first, last })
    }

    /// The span of `half` in the year labelled `label`.
    ///
    /// # Errors
    ///
    /// As [`YearSystem::fiscal_month_span`].
    pub fn half_span(&self, label: i64, half: Half) -> FiscalResult<PeriodSpan> {
        let [first_quarter, last_quarter] = half.quarters();
        let first = self.quarter_span(label, first_quarter)?.first;
        let last = self.quarter_span(label, last_quarter)?.last;
        Ok(PeriodSpan { first, last })
    }

    /// Which month of the fiscal year `rd` falls in, counting from 1.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::OutsideValidity`] when `rd` falls outside the
    /// system's range and [`FiscalError::PeriodsNotDefined`] for a calendar
    /// without twelve equal months.
    pub fn month_of_year(&self, rd: Rd) -> FiscalResult<u8> {
        let count = self.month_count()?;
        let label = self.label_at(rd)?;
        let start_year = self.label.start_calendar_year(label);
        let (year, month, day) = self.start.calendar.from_fixed(rd)?;
        let mut elapsed =
            (year - start_year) * count + i64::from(month) - i64::from(self.start.month);
        if day < self.start.day {
            elapsed -= 1;
        }
        u8::try_from(elapsed + 1).map_err(|_| FiscalError::Calendar(CalendarError::MonthOutOfRange))
    }

    /// Which quarter of the fiscal year `rd` falls in.
    ///
    /// # Errors
    ///
    /// As [`YearSystem::month_of_year`].
    pub fn quarter(&self, rd: Rd) -> FiscalResult<Quarter> {
        let month = self.month_of_year(rd)?;
        Quarter::from_ordinal((month - 1) / 3 + 1)
            .ok_or(FiscalError::Calendar(CalendarError::MonthOutOfRange))
    }

    /// Which half of the fiscal year `rd` falls in.
    ///
    /// # Errors
    ///
    /// As [`YearSystem::month_of_year`].
    pub fn half(&self, rd: Rd) -> FiscalResult<Half> {
        Ok(self.quarter(rd)?.half())
    }
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::gregorian;

    use super::*;
    use crate::year_system::{Authority, LabelConvention, StartCalendar, SystemKind, YearStart};

    const fn system(start: YearStart, label: LabelConvention) -> YearSystem {
        YearSystem {
            name: "test",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start,
            label,
            valid_from: None,
            valid_until: None,
            note: "",
        }
    }

    const JAPAN: YearSystem = system(
        YearStart::gregorian(4, 1),
        LabelConvention::LabelledByStartYear,
    );
    const US: YearSystem = system(
        YearStart::gregorian(10, 1),
        LabelConvention::LabelledByEndYear,
    );
    const UK_TAX: YearSystem = system(
        YearStart::gregorian(4, 6),
        LabelConvention::LabelledByStartYear,
    );

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn q1_in_tokyo_is_april_and_q1_in_washington_is_october() {
        assert_eq!(
            JAPAN.calendar_months_of_quarter(Quarter::First).unwrap(),
            [4, 5, 6]
        );
        assert_eq!(
            US.calendar_months_of_quarter(Quarter::First).unwrap(),
            [10, 11, 12]
        );
    }

    #[test]
    fn the_quarters_of_a_year_cover_every_calendar_month_exactly_once() {
        for system in [JAPAN, US, UK_TAX] {
            let mut seen = [false; 12];
            for quarter in Quarter::ALL {
                for month in system.calendar_months_of_quarter(quarter).unwrap() {
                    assert!(!seen[usize::from(month) - 1], "month {month} repeated");
                    seen[usize::from(month) - 1] = true;
                }
            }
            assert!(seen.iter().all(|&flag| flag));
        }
    }

    #[test]
    fn the_us_fourth_quarter_wraps_into_the_labelled_year() {
        assert_eq!(
            US.calendar_months_of_quarter(Quarter::Fourth).unwrap(),
            [7, 8, 9]
        );
        let q4 = US.quarter_span(2024, Quarter::Fourth).unwrap();
        assert_eq!(q4.first, greg(2024, 7, 1));
        assert_eq!(q4.last, greg(2024, 9, 30));
    }

    #[test]
    fn japans_first_quarter_is_april_to_june() {
        let q1 = JAPAN.quarter_span(2024, Quarter::First).unwrap();
        assert_eq!(q1.first, greg(2024, 4, 1));
        assert_eq!(q1.last, greg(2024, 6, 30));
        assert_eq!(q1.days(), 91);
    }

    #[test]
    fn the_four_quarters_tile_the_year_without_a_gap() {
        for label in 1970..2070 {
            let year = JAPAN.span(label).unwrap();
            let mut cursor = year.first;
            for quarter in Quarter::ALL {
                let span = JAPAN.quarter_span(label, quarter).unwrap();
                assert_eq!(span.first, cursor);
                cursor = Rd(span.last.0 + 1);
            }
            assert_eq!(cursor, Rd(year.last.0 + 1));
        }
    }

    #[test]
    fn the_twelve_fiscal_months_tile_the_year_without_a_gap() {
        for label in 2000..2050 {
            for system in [JAPAN, US, UK_TAX] {
                let year = system.span(label).unwrap();
                let mut cursor = year.first;
                let mut total = 0;
                for month in 1..=12 {
                    let span = system.fiscal_month_span(label, month).unwrap();
                    assert_eq!(span.first, cursor, "month {month} of {label}");
                    cursor = Rd(span.last.0 + 1);
                    total += span.days();
                }
                assert_eq!(cursor, Rd(year.last.0 + 1));
                assert_eq!(total, year.days());
            }
        }
    }

    #[test]
    fn the_two_halves_tile_the_year() {
        let first = JAPAN.half_span(2024, Half::First).unwrap();
        let second = JAPAN.half_span(2024, Half::Second).unwrap();
        assert_eq!(first.first, greg(2024, 4, 1));
        // 上期 ends on 30 September; 下期 begins on 1 October.
        assert_eq!(first.last, greg(2024, 9, 30));
        assert_eq!(second.first, greg(2024, 10, 1));
        assert_eq!(second.last, greg(2025, 3, 31));
    }

    #[test]
    fn a_days_month_quarter_and_half_agree_with_one_another() {
        for day in greg(2000, 1, 1).0..=greg(2030, 12, 31).0 {
            let rd = Rd(day);
            for system in [JAPAN, US, UK_TAX] {
                let label = system.label_at(rd).unwrap();
                let month = system.month_of_year(rd).unwrap();
                assert!((1..=12).contains(&month));
                assert!(system.fiscal_month_span(label, month).unwrap().contains(rd));
                let quarter = system.quarter(rd).unwrap();
                assert!(quarter.fiscal_months().contains(&month));
                assert!(system.quarter_span(label, quarter).unwrap().contains(rd));
                let half = system.half(rd).unwrap();
                assert!(half.quarters().contains(&quarter));
                assert!(system.half_span(label, half).unwrap().contains(rd));
            }
        }
    }

    #[test]
    fn a_uk_tax_month_runs_from_the_sixth_to_the_fifth() {
        // HMRC numbers its tax months from the tax-year start: tax month 1
        // is 6 April to 5 May, which is what makes the PAYE deadlines fall
        // on the 19th and 22nd of the following month.
        let month_one = UK_TAX.fiscal_month_span(2024, 1).unwrap();
        assert_eq!(month_one.first, greg(2024, 4, 6));
        assert_eq!(month_one.last, greg(2024, 5, 5));
        assert_eq!(UK_TAX.month_of_year(greg(2024, 5, 5)).unwrap(), 1);
        assert_eq!(UK_TAX.month_of_year(greg(2024, 5, 6)).unwrap(), 2);
        let month_twelve = UK_TAX.fiscal_month_span(2024, 12).unwrap();
        assert_eq!(month_twelve.first, greg(2025, 3, 6));
        assert_eq!(month_twelve.last, greg(2025, 4, 5));
    }

    #[test]
    fn the_month_of_the_year_counts_from_the_fiscal_start() {
        assert_eq!(JAPAN.month_of_year(greg(2024, 4, 1)).unwrap(), 1);
        assert_eq!(JAPAN.month_of_year(greg(2024, 12, 31)).unwrap(), 9);
        assert_eq!(JAPAN.month_of_year(greg(2025, 3, 31)).unwrap(), 12);
        assert_eq!(US.month_of_year(greg(2023, 10, 1)).unwrap(), 1);
        assert_eq!(US.month_of_year(greg(2024, 9, 30)).unwrap(), 12);
    }

    #[test]
    fn a_month_index_outside_one_to_twelve_is_refused() {
        assert_eq!(
            JAPAN.fiscal_month_span(2024, 0),
            Err(FiscalError::Calendar(CalendarError::MonthOutOfRange))
        );
        assert_eq!(
            JAPAN.fiscal_month_span(2024, 13),
            Err(FiscalError::Calendar(CalendarError::MonthOutOfRange))
        );
    }

    #[test]
    fn an_ethiopic_anchored_system_refuses_quarters_but_still_has_a_span() {
        let ethiopia = system(
            YearStart::new(StartCalendar::ETHIOPIC, 11, 8),
            LabelConvention::LabelledByEndYear,
        );
        let rd = greg(2024, 1, 1);
        assert_eq!(ethiopia.quarter(rd), Err(FiscalError::PeriodsNotDefined));
        assert_eq!(ethiopia.half(rd), Err(FiscalError::PeriodsNotDefined));
        assert_eq!(
            ethiopia.month_of_year(rd),
            Err(FiscalError::PeriodsNotDefined)
        );
        assert_eq!(
            ethiopia.calendar_months_of_quarter(Quarter::First),
            Err(FiscalError::PeriodsNotDefined)
        );
        // The year itself is still perfectly well defined.
        assert!(ethiopia.span(ethiopia.label_at(rd).unwrap()).is_ok());
    }

    #[test]
    fn quarters_and_halves_convert_to_and_from_their_ordinals() {
        for quarter in Quarter::ALL {
            assert_eq!(Quarter::from_ordinal(quarter.ordinal()), Some(quarter));
        }
        assert_eq!(Quarter::from_ordinal(0), None);
        assert_eq!(Quarter::from_ordinal(5), None);
        for half in Half::ALL {
            assert_eq!(Half::from_ordinal(half.ordinal()), Some(half));
        }
        assert_eq!(Half::from_ordinal(0), None);
        assert_eq!(Half::from_ordinal(3), None);
        assert_eq!(Quarter::Third.half(), Half::Second);
        assert_eq!(Half::First.quarters(), [Quarter::First, Quarter::Second]);
        assert_eq!(Quarter::Fourth.fiscal_months(), [10, 11, 12]);
    }

    #[test]
    fn a_period_outside_the_validity_range_is_refused() {
        let bounded = YearSystem {
            valid_from: Some(1977),
            ..US
        };
        assert_eq!(
            bounded.fiscal_month_span(1976, 1),
            Err(FiscalError::OutsideValidity)
        );
        assert!(bounded.fiscal_month_span(1977, 1).is_ok());
    }
}
