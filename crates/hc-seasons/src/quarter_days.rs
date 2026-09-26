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
//! | [`QuarterDayTradition::Scotland1990`] | Candlemas 28 February, Whitsunday 28 May, Lammas 28 August, Martinmas 28 November, the quarter days of the Term and Quarter Days (Scotland) Act 1990 |
//!
//! Lady Day was the first day of the civil year in England and Wales until
//! 1752, which is the business of `year_style` in `hc-calendars-solar`, not
//! this module's. The Scottish Whitsunday is the fixed 15 May of the term, not
//! the movable feast. The Removal Terms (Scotland) Act 1886 made a tenant
//! of a house under a lease entered after it enter or remove at noon on 28
//! May for a Whitsunday term and 28 November for a Martinmas one, while
//! keeping warnings of removal at forty days before 15 May and 11 November
//! (section 4). The Term and Quarter Days (Scotland) Act 1990 defines
//! Whitsunday, Martinmas, Candlemas and Lammas as 28 May, 28 November, 28
//! February and 28 August for any enactment or rule of law and for any
//! lease, agreement or document made after it came into force, twelve
//! months after its passing on 13 July 1990; the term days are 28 May and
//! 28 November and the quarter days all four (section 1(1), (2); section
//! 3(2)). The traditional dates are kept here beside them, since the names
//! live on. The Wheel of the Year keeps Imbolc on 1 February; the
//! source's Irish list gives Candlemas, 2 February, as its counterpart, and
//! that is the date carried here.
//!
//! Sources:
//!
//! * Wikipedia, "Quarter days", retrieved 2026-09-22, for the English,
//!   Welsh, Irish and traditional Scottish dates.
//! * `uk-removal-terms-scotland-1886`: Removal Terms (Scotland) Act 1886
//!   (49 & 50 Vict. c. 50), as revised on legislation.gov.uk,
//!   <https://www.legislation.gov.uk/ukpga/Vict/49-50/50/section/4>,
//!   retrieved 2026-09-26, section 4.
//! * `uk-term-quarter-days-scotland-1990`: Term and Quarter Days (Scotland)
//!   Act 1990 (c. 22), as revised on legislation.gov.uk,
//!   <https://www.legislation.gov.uk/ukpga/1990/22/section/1>, retrieved
//!   2026-09-26, sections 1 and 3.

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
    /// The Scottish quarter days of the Term and Quarter Days (Scotland) Act
    /// 1990, of which the May and November days are also its term days.
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

        /// Candlemas under the 1990 Act, 28 February: a quarter day, section
        /// 1(1)(b) and (d).
        pub const SCOTLAND_1990_FEBRUARY = day("scotland-1990-february", QuarterDayTradition::Scotland1990, "Candlemas", "", 2, 28);
        /// Whitsunday under the 1990 Act, 28 May: a term day and a quarter
        /// day, section 1(1)(a), (c) and (d).
        pub const SCOTLAND_1990_MAY = day("scotland-1990-may", QuarterDayTradition::Scotland1990, "Whitsunday", "", 5, 28);
        /// Lammas under the 1990 Act, 28 August: a quarter day, section
        /// 1(1)(b) and (d).
        pub const SCOTLAND_1990_AUGUST = day("scotland-1990-august", QuarterDayTradition::Scotland1990, "Lammas", "", 8, 28);
        /// Martinmas under the 1990 Act, 28 November: a term day and a
        /// quarter day, section 1(1)(a), (c) and (d).
        pub const SCOTLAND_1990_NOVEMBER = day("scotland-1990-november", QuarterDayTradition::Scotland1990, "Martinmas", "", 11, 28);
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
        // The 1990 Act's names are the traditional four, moved to the 28th.
        let traditional: Vec<&str> = QuarterDayTradition::ScotlandTraditional
            .days()
            .map(|day| day.english_name)
            .collect();
        let statutory: Vec<&str> = QuarterDayTradition::Scotland1990
            .days()
            .map(|day| day.english_name)
            .collect();
        assert_eq!(traditional, statutory);
        assert_eq!(
            QuarterDay::by_id("scotland-martinmas"),
            Some(QuarterDay::SCOTLAND_MARTINMAS)
        );
        assert_eq!(QuarterDay::by_id("wales-lady-day"), None);
    }
}
