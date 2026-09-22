//! Quarter days and term days of the British Isles.
//!
//! The four days a year on which rents fell due, servants were hired and
//! leases began, in the traditions that kept them: not a calendar, but a
//! set of named days a calendar prints in its margins, which is what this
//! crate holds. Each tradition is four fixed Gregorian dates, and the
//! traditions differ in which four:
//!
//! | Tradition | Days |
//! | --- | --- |
//! | [`QuarterDayTradition::EnglandAndWales`] | Lady Day 25 March, Midsummer Day 24 June, Michaelmas 29 September, Christmas 25 December |
//! | [`QuarterDayTradition::EnglishCrossQuarter`] | Candlemas 2 February, May Day 1 May, Lammas 1 August, All Hallows 1 November — halfway between the quarter days |
//! | [`QuarterDayTradition::Ireland`] | the same four dates as the Celtic quarter days Imbolc, Bealtaine, Lughnasadh and Samhain, under the names the source gives them |
//! | [`QuarterDayTradition::ScotlandTraditional`] | Candlemas 2 February, Whitsunday 15 May, Lammas 1 August, Martinmas 11 November |
//! | [`QuarterDayTradition::Scotland1990`] | 28 February, 28 May, 28 August and 28 November, the term days of the Term and Quarter Days (Scotland) Act 1990 |
//!
//! Lady Day was the first day of the civil year in England and Wales until
//! 1752, which is the business of `year_style` in `hc-calendars-solar`, not
//! this module's. The Scottish Whitsunday is the fixed 15 May of the term, not
//! the movable feast; the removal and hiring dates were moved to 28 May
//! and 28 November in 1886, and the 1990 Act made the 28th the term day
//! of all four months in official use — the traditional dates are kept
//! here beside them, since the names live on. The Wheel of the Year keeps
//! Imbolc on 1 February; the source's Irish list gives Candlemas, 2
//! February, as its counterpart, and that is the date carried here.
//!
//! Source: Wikipedia, "Quarter days", retrieved 2026-09-22, for every
//! date, the 1886 change and the 1990 Act.

use hc_calendar::Rd;

use crate::gregorian;

/// Whose quarter days these are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuarterDayTradition {
    /// The English and Welsh quarter days.
    EnglandAndWales,
    /// The English cross-quarter days, halfway between the quarter days.
    EnglishCrossQuarter,
    /// The Irish quarter days, the Celtic ones under their later names.
    Ireland,
    /// The traditional Scottish term days.
    ScotlandTraditional,
    /// The Scottish term days of the Term and Quarter Days (Scotland) Act
    /// 1990.
    Scotland1990,
}

impl QuarterDayTradition {
    /// Every tradition, in the order of the module table.
    pub const ALL: [Self; 5] = [
        Self::EnglandAndWales,
        Self::EnglishCrossQuarter,
        Self::Ireland,
        Self::ScotlandTraditional,
        Self::Scotland1990,
    ];

    /// The four days of this tradition, in the order they fall in the year.
    pub fn days(self) -> impl Iterator<Item = QuarterDay> {
        QuarterDay::ALL
            .iter()
            .copied()
            .filter(move |day| day.tradition == self)
    }

    /// The fixed days of this tradition in `year`, in order.
    #[must_use]
    pub fn days_in(self, year: i64) -> [Rd; 4] {
        let mut out = [Rd(0); 4];
        for (slot, day) in out.iter_mut().zip(self.days()) {
            *slot = day.day_in(year);
        }
        out
    }
}

/// A quarter day or term day: a fixed Gregorian date with a name, in a
/// tradition.
#[derive(Debug, Clone, Copy)]
pub struct QuarterDay {
    /// A short identifier, the tradition and the day in kebab case.
    pub id: &'static str,
    /// Whose day it is.
    pub tradition: QuarterDayTradition,
    /// The name in English.
    pub english_name: &'static str,
    /// The name in Irish for the Irish days, or `""`.
    pub local_name: &'static str,
    /// The Gregorian month, 1–12.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl PartialEq for QuarterDay {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for QuarterDay {}

impl core::hash::Hash for QuarterDay {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl QuarterDay {
    /// The fixed day this falls on in `year`.
    #[must_use]
    pub const fn day_in(self, year: i64) -> Rd {
        gregorian::from_year_month_day(year, self.month, self.day)
    }
}

/// A day of a tradition.
const fn day(
    id: &'static str,
    tradition: QuarterDayTradition,
    english_name: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
) -> QuarterDay {
    QuarterDay {
        id,
        tradition,
        english_name,
        local_name,
        month,
        day,
    }
}

hc_core::catalogue! {
    type: QuarterDay,
    id: |entry| entry.id,
    tests: quarter_day_tests,
    associated;

    /// Every quarter day and term day, tradition by tradition, each
    /// tradition's four in the order they fall in the year.
    pub const ALL;
    /// The entry with this identifier.
    pub fn by_id;

    entries: {
        /// Lady Day, the Feast of the Annunciation, 25 March.
        pub const LADY_DAY = day("england-lady-day", QuarterDayTradition::EnglandAndWales, "Lady Day", "", 3, 25);
        /// Midsummer Day, the Nativity of St John the Baptist, 24 June.
        pub const MIDSUMMER_DAY = day("england-midsummer-day", QuarterDayTradition::EnglandAndWales, "Midsummer Day", "", 6, 24);
        /// Michaelmas, the Feast of St Michael and All Angels, 29 September.
        pub const MICHAELMAS = day("england-michaelmas", QuarterDayTradition::EnglandAndWales, "Michaelmas", "", 9, 29);
        /// Christmas Day, 25 December.
        pub const CHRISTMAS = day("england-christmas", QuarterDayTradition::EnglandAndWales, "Christmas Day", "", 12, 25);

        /// Candlemas, 2 February.
        pub const CANDLEMAS = day("england-candlemas", QuarterDayTradition::EnglishCrossQuarter, "Candlemas", "", 2, 2);
        /// May Day, 1 May.
        pub const MAY_DAY = day("england-may-day", QuarterDayTradition::EnglishCrossQuarter, "May Day", "", 5, 1);
        /// Lammas, 1 August.
        pub const LAMMAS = day("england-lammas", QuarterDayTradition::EnglishCrossQuarter, "Lammas", "", 8, 1);
        /// All Hallows, 1 November.
        pub const ALL_HALLOWS = day("england-all-hallows", QuarterDayTradition::EnglishCrossQuarter, "All Hallows", "", 11, 1);

        /// Candlemas, 2 February, the counterpart of Imbolc, St Brigid's Day.
        pub const IRELAND_CANDLEMAS = day("ireland-candlemas", QuarterDayTradition::Ireland, "Candlemas", "Lá Fhéile Bríde", 2, 2);
        /// May Day, 1 May, Bealtaine.
        pub const IRELAND_MAY_DAY = day("ireland-may-day", QuarterDayTradition::Ireland, "May Day", "Lá Bealtaine", 5, 1);
        /// Lammas, 1 August, Lughnasadh.
        pub const IRELAND_LAMMAS = day("ireland-lammas", QuarterDayTradition::Ireland, "Lammas", "Lá Lúnasa", 8, 1);
        /// All Hallows, 1 November, Samhain.
        pub const IRELAND_ALL_HALLOWS = day("ireland-all-hallows", QuarterDayTradition::Ireland, "All Hallows", "Lá Samhna", 11, 1);

        /// Candlemas, 2 February.
        pub const SCOTLAND_CANDLEMAS = day("scotland-candlemas", QuarterDayTradition::ScotlandTraditional, "Candlemas", "", 2, 2);
        /// Whitsunday, fixed for the term at 15 May.
        pub const SCOTLAND_WHITSUNDAY = day("scotland-whitsunday", QuarterDayTradition::ScotlandTraditional, "Whitsunday", "", 5, 15);
        /// Lammas, 1 August.
        pub const SCOTLAND_LAMMAS = day("scotland-lammas", QuarterDayTradition::ScotlandTraditional, "Lammas", "", 8, 1);
        /// Martinmas, 11 November.
        pub const SCOTLAND_MARTINMAS = day("scotland-martinmas", QuarterDayTradition::ScotlandTraditional, "Martinmas", "", 11, 11);

        /// The February term day of the 1990 Act, the 28th.
        pub const SCOTLAND_1990_FEBRUARY = day("scotland-1990-february", QuarterDayTradition::Scotland1990, "Candlemas term day", "", 2, 28);
        /// The May term day of the 1990 Act, the 28th.
        pub const SCOTLAND_1990_MAY = day("scotland-1990-may", QuarterDayTradition::Scotland1990, "Whitsunday term day", "", 5, 28);
        /// The August term day of the 1990 Act, the 28th.
        pub const SCOTLAND_1990_AUGUST = day("scotland-1990-august", QuarterDayTradition::Scotland1990, "Lammas term day", "", 8, 28);
        /// The November term day of the 1990 Act, the 28th.
        pub const SCOTLAND_1990_NOVEMBER = day("scotland-1990-november", QuarterDayTradition::Scotland1990, "Martinmas term day", "", 11, 28);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::from_year_month_day(year, month, day)
    }

    #[test]
    fn every_tradition_has_four_days_in_the_order_of_the_year() {
        for tradition in QuarterDayTradition::ALL {
            let days: Vec<QuarterDay> = tradition.days().collect();
            assert_eq!(days.len(), 4, "{tradition:?}");
            let fixed = tradition.days_in(2026);
            assert!(
                fixed.windows(2).all(|pair| pair[0] < pair[1]),
                "{tradition:?}"
            );
        }
        assert_eq!(QuarterDay::ALL.len(), 20);
    }

    #[test]
    fn the_dates_are_the_sources() {
        assert_eq!(
            QuarterDayTradition::EnglandAndWales.days_in(2026),
            [
                ymd(2026, 3, 25),
                ymd(2026, 6, 24),
                ymd(2026, 9, 29),
                ymd(2026, 12, 25)
            ]
        );
        assert_eq!(
            QuarterDayTradition::EnglishCrossQuarter.days_in(2026),
            [
                ymd(2026, 2, 2),
                ymd(2026, 5, 1),
                ymd(2026, 8, 1),
                ymd(2026, 11, 1)
            ]
        );
        assert_eq!(
            QuarterDayTradition::Ireland.days_in(2026),
            QuarterDayTradition::EnglishCrossQuarter.days_in(2026)
        );
        assert_eq!(
            QuarterDayTradition::ScotlandTraditional.days_in(2026),
            [
                ymd(2026, 2, 2),
                ymd(2026, 5, 15),
                ymd(2026, 8, 1),
                ymd(2026, 11, 11)
            ]
        );
        assert_eq!(
            QuarterDayTradition::Scotland1990.days_in(2026),
            [
                ymd(2026, 2, 28),
                ymd(2026, 5, 28),
                ymd(2026, 8, 28),
                ymd(2026, 11, 28)
            ]
        );
        assert_eq!(QuarterDay::IRELAND_MAY_DAY.local_name, "Lá Bealtaine");
        assert_eq!(
            QuarterDay::by_id("scotland-martinmas"),
            Some(QuarterDay::SCOTLAND_MARTINMAS)
        );
        assert_eq!(QuarterDay::by_id("wales-lady-day"), None);
    }
}
