//! Where an era's year opens among the amānta months, and the arithmetic
//! that follows from it: the eras of `docs/systems/indian-eras.md` in the
//! repository, Nepal Sambat ([`crate::nepal_sambat`]) and the Vira Nirvana
//! Samvat ([`crate::vira_nirvana`]).
//!
//! An era over the lunisolar months renames nothing but the year: a date
//! keeps its tithi, its fortnight and its intercalary or repeated day
//! exactly as [`crate::hindu_lunar`] has them. What an era adds is a
//! tithi of a month at which its year opens, and how far the Śaka year in
//! which a year opens is from that year's number — the *offset*, the Śaka
//! year less the era's year, for the part of the Śaka year after the
//! opening; before it, the Śaka year is one more.
//!
//! A year that opens at śukla 1 opens with the first month of the name —
//! the intercalary one, in a year that has it, whose śukla 1 comes first.
//! A year that opens later in a month opens in the ordinary month, as the
//! Odia Anka does, and on the first day whose sunrise tithi is the opening
//! one or later, should that tithi hold no sunrise.

use hc_calendar::{CalendarError, CalendarResult, Rd};

use crate::hindu_lunar::{HinduLunarCalendar, HinduLunarDate};

/// The months in a year.
const MONTHS: u8 = 12;

/// The month and tithi at which an era's year opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YearStart {
    /// The amānta month, 1 for Chaitra through 12 for Phālguna.
    pub month: u8,
    /// The tithi of that month, 1 to 15, that opens the year.
    pub tithi: u8,
}

impl YearStart {
    /// Chaitra śukla 1, where the Śaka year itself opens: the Chaitrādi
    /// year.
    pub const CHAITRADI: Self = Self::new(1, 1);

    /// Āśvina śukla 1: the Āśvinādi year.
    pub const ASVINADI: Self = Self::new(7, 1);

    /// Kārttika śukla 1, the day after Dīpāvalī: the Kārttikādi year.
    pub const KARTTIKADI: Self = Self::new(8, 1);

    /// A year opening at `tithi` of the amānta `month`.
    #[must_use]
    pub const fn new(month: u8, tithi: u8) -> Self {
        Self { month, tithi }
    }

    /// Whether the year opens at the first day of its month, so that its
    /// months can be numbered from the opening one.
    #[must_use]
    pub const fn opens_a_month(self) -> bool {
        self.tithi == 1
    }

    /// Whether a day of a Śaka year, by its amānta month, whether that
    /// month is intercalary, and its tithi, is at or after the opening.
    #[must_use]
    pub const fn is_opened_by(self, month: u8, leap_month: bool, day: u8) -> bool {
        if month != self.month {
            return month > self.month;
        }
        self.opens_a_month() || (!leap_month && day >= self.tithi)
    }

    /// The era's year of a day of Śaka year `saka`, for an era whose year
    /// opens in Śaka year `year + offset`.
    #[must_use]
    pub const fn era_year(
        self,
        saka: i64,
        month: u8,
        leap_month: bool,
        day: u8,
        offset: i64,
    ) -> i64 {
        if self.is_opened_by(month, leap_month, day) {
            saka - offset
        } else {
            saka - offset - 1
        }
    }

    /// The Śaka year of a day of the era's year `year`: the inverse of
    /// [`YearStart::era_year`].
    #[must_use]
    pub const fn saka_year(
        self,
        year: i64,
        month: u8,
        leap_month: bool,
        day: u8,
        offset: i64,
    ) -> i64 {
        if self.is_opened_by(month, leap_month, day) {
            year + offset
        } else {
            year + offset + 1
        }
    }

    /// The era's month, 1 for the opening month, of an amānta month, 1 for
    /// Chaitra.
    #[must_use]
    pub const fn era_month(self, amanta: u8) -> u8 {
        (amanta + MONTHS - self.month) % MONTHS + 1
    }

    /// The amānta month, 1 for Chaitra, of an era's month, 1 for the
    /// opening month: the inverse of [`YearStart::era_month`].
    #[must_use]
    pub const fn amanta_month(self, month: u8) -> u8 {
        (month + self.month + MONTHS - 2) % MONTHS + 1
    }

    /// New Year's Day of an era's year: the first day of the first month of
    /// the opening name for a year that opens at śukla 1, and otherwise the
    /// first day of the ordinary month whose tithi is the opening one or
    /// later.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] outside the amānta engine's
    /// range.
    pub fn new_year(
        self,
        lunar: &HinduLunarCalendar,
        year: i64,
        offset: i64,
    ) -> CalendarResult<Rd> {
        let saka = year + offset;
        if self.opens_a_month() {
            return lunar
                .month_span(saka, self.month, true)
                .or_else(|_| lunar.month_span(saka, self.month, false))
                .map(|(first, _)| first);
        }
        let mut last_error = CalendarError::DayOutOfRange;
        for day in self.tithi..=15 {
            let date = HinduLunarDate {
                year: saka,
                month: self.month,
                leap_month: false,
                day,
                leap_day: false,
            };
            match lunar.to_fixed(date) {
                Ok(rd) => return Ok(rd),
                Err(error @ CalendarError::YearOutOfRange) => return Err(error),
                Err(error) => last_error = error,
            }
        }
        Err(last_error)
    }

    /// Whether an era's year has an adhika māsa. The year runs from the
    /// opening in one Śaka year to the opening in the next, so the month
    /// may fall in either.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] outside the amānta engine's
    /// range.
    pub fn is_leap_year(
        self,
        lunar: &HinduLunarCalendar,
        year: i64,
        offset: i64,
    ) -> CalendarResult<bool> {
        let saka = year + offset;
        // An intercalary month belongs to the year its first day is in:
        // after the opening, or before the next one.
        let after = |month: u8| self.is_opened_by(month, true, 1);
        let autumn = lunar
            .leap_month_of(saka)?
            .is_some_and(|(month, _, _)| after(month));
        let spring = if self.month == 1 && self.opens_a_month() {
            // A Chaitrādi year is the Śaka year itself.
            false
        } else {
            match lunar.leap_month_of(saka + 1) {
                Ok(leap) => leap.is_some_and(|(month, _, _)| !after(month)),
                // The engine's last year: the rest is not converted.
                Err(CalendarError::YearOutOfRange) if saka == crate::hindu_lunar::MAX_YEAR => false,
                Err(error) => return Err(error),
            }
        };
        Ok(autumn || spring)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_months_turn_at_the_opening_and_back() {
        for start in [
            YearStart::CHAITRADI,
            YearStart::ASVINADI,
            YearStart::KARTTIKADI,
        ] {
            assert_eq!(start.era_month(start.month), 1);
            assert_eq!(start.amanta_month(1), start.month);
            for month in 1..=12 {
                assert_eq!(start.era_month(start.amanta_month(month)), month);
            }
        }
        assert_eq!(YearStart::KARTTIKADI.amanta_month(12), 7);
        assert_eq!(YearStart::KARTTIKADI.amanta_month(6), 1);
        assert_eq!(YearStart::CHAITRADI.era_month(5), 5);
    }

    #[test]
    fn a_year_opening_mid_month_opens_in_the_ordinary_month() {
        let jyeshtha_13 = YearStart::new(3, 13);
        assert!(!jyeshtha_13.opens_a_month());
        assert!(!jyeshtha_13.is_opened_by(3, false, 12));
        assert!(jyeshtha_13.is_opened_by(3, false, 13));
        assert!(jyeshtha_13.is_opened_by(3, false, 30));
        assert!(!jyeshtha_13.is_opened_by(3, true, 20));
        assert!(!jyeshtha_13.is_opened_by(2, false, 30));
        assert!(jyeshtha_13.is_opened_by(4, true, 1));
        // A Kārttikādi year opens with the intercalary Kārttika too.
        assert!(YearStart::KARTTIKADI.is_opened_by(8, true, 1));
        assert!(!YearStart::KARTTIKADI.is_opened_by(7, false, 30));
    }

    #[test]
    fn the_year_steps_at_the_opening_against_the_saka_year() {
        let start = YearStart::new(3, 13);
        for (month, day) in [(1, 1), (3, 12), (3, 13), (7, 1), (12, 30)] {
            let saka = start.saka_year(351, month, false, day, 1_595);
            assert_eq!(start.era_year(saka, month, false, day, 1_595), 351);
        }
        assert_eq!(start.saka_year(351, 3, false, 13, 1_595), 1_946);
        assert_eq!(start.saka_year(351, 3, false, 12, 1_595), 1_947);
    }
}
