//! 几龙治水, 几牛耕田, 几日得辛 and 几人分饼 — the year's four counts from
//! the first month.
//!
//! The Chinese almanac numbers a year by the day of 正月 on which a day
//! sign first falls, counting 初一 as the first: the first 辰 day gives
//! the dragons that govern the water, 几龙治水; the first 丑 day the oxen
//! that plough, 几牛耕田; the first 辛 day the day 辛 is got, 几日得辛; and
//! the first 丙 day the people who share the cake, 几人分饼 (Wikipedia
//! (zh), 「龍治水」 and its section 「相近概念」, `wikipedia-zh-long-zhi-shui`,
//! retrieved 2026-09-28). A branch recurs every twelve days and a stem
//! every ten, so the dragons and the oxen run from one to twelve and the
//! other two from one to ten. What each number portends is folklore the
//! sources disagree about, and is not carried.
//!
//! The first month is the Chinese calendar's, `chinese`.
//! `docs/systems/east-asian-folk-days.md` describes the counts and their
//! tests.

use hc_calendar::Rd;
use hc_calendar::cycle::{branch, branch_day_on_or_after, stem, stem_day_on_or_after};
use hc_calendars_lunar::chinese;

/// The four counts of a Chinese year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FirstMonthCounts {
    /// 几龙治水: the day of 正月 of the first 辰 day, 1 to 12.
    pub dragons: u8,
    /// 几牛耕田: the day of 正月 of the first 丑 day, 1 to 12.
    pub oxen: u8,
    /// 几日得辛: the day of 正月 of the first 辛 day, 1 to 10.
    pub xin: u8,
    /// 几人分饼: the day of 正月 of the first 丙 day, 1 to 10.
    pub cakes: u8,
}

impl FirstMonthCounts {
    /// The counts' identifiers, in the order [`FirstMonthCounts::by_id`]
    /// gives them: 几龙治水, 几牛耕田, 几日得辛 and 几人分饼, as
    /// `hc_i18n::reckonings` names them.
    pub const IDS: [&'static str; 4] = ["dragons", "oxen", "xin", "cakes"];

    /// Each count with its identifier, in the order of [`Self::IDS`].
    #[must_use]
    pub const fn by_id(&self) -> [(&'static str, u8); 4] {
        [
            (Self::IDS[0], self.dragons),
            (Self::IDS[1], self.oxen),
            (Self::IDS[2], self.xin),
            (Self::IDS[3], self.cakes),
        ]
    }

    /// The counts of the year whose 正月初一 is `new_year`.
    #[must_use]
    pub const fn from_new_year(new_year: Rd) -> Self {
        Self {
            dragons: day_of_first_month(new_year, branch_day_on_or_after(new_year, branch::CHEN)),
            oxen: day_of_first_month(new_year, branch_day_on_or_after(new_year, branch::CHOU)),
            xin: day_of_first_month(new_year, stem_day_on_or_after(new_year, stem::XIN)),
            cakes: day_of_first_month(new_year, stem_day_on_or_after(new_year, stem::BING)),
        }
    }
}

/// The day of 正月 of `day`, 初一 being 1, for a day within twelve days of
/// `new_year`.
const fn day_of_first_month(new_year: Rd, day: Rd) -> u8 {
    (day.0 - new_year.0 + 1) as u8
}

/// The counts of the Chinese year that begins in `gregorian_year`, or
/// `None` outside the years `chinese` converts, 1645 to 2150.
#[must_use]
pub fn first_month_counts(gregorian_year: i64) -> Option<FirstMonthCounts> {
    // The Chinese year beginning in 2024 is 4661.
    let new_year = chinese::new_year(gregorian_year.checked_add(2_637)?).ok()?;
    Some(FirstMonthCounts::from_new_year(new_year))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::cycle::sexagenary_day;
    use hc_calendar::gregorian;

    #[test]
    fn the_counts_of_2026_are_the_published_ones() {
        // 网易, 「老人说:"2026年七龙治水，四牛耕田，五人分饼，十日得辛"，是啥
        // 意思？」 (`netease-2026-longzhishui`), retrieved 2026-09-28: the
        // first 辰 day is 正月初七, the first 丑 day 初四, 初五 is 丙寅 and the
        // tenth day is a 辛 day.
        let counts = first_month_counts(2026).unwrap();
        assert_eq!(
            counts,
            FirstMonthCounts {
                dragons: 7,
                oxen: 4,
                xin: 10,
                cakes: 5,
            }
        );
        // 正月初一 of 2026 is 17 February, 壬戌.
        let new_year = chinese::new_year(4_663).unwrap();
        assert_eq!(new_year, gregorian::to_fixed(2026, 2, 17).unwrap());
    }

    #[test]
    fn every_count_is_the_first_day_of_its_sign() {
        for year in 1645..=2150 {
            let Some(counts) = first_month_counts(year) else {
                continue;
            };
            let new_year = chinese::new_year(year + 2_637).unwrap();
            let day = |n: u8| sexagenary_day(Rd(new_year.0 + i64::from(n) - 1));
            let first_with =
                |test: &dyn Fn(u8) -> bool, limit: u8| (1..=limit).find(|n| test(*n)).unwrap();
            assert_eq!(
                counts.dragons,
                first_with(&|n| day(n).branch_index() == branch::CHEN, 12),
                "{year}"
            );
            assert_eq!(
                counts.oxen,
                first_with(&|n| day(n).branch_index() == branch::CHOU, 12),
                "{year}"
            );
            assert_eq!(
                counts.xin,
                first_with(&|n| day(n).stem_index() == stem::XIN, 10),
                "{year}"
            );
            assert_eq!(
                counts.cakes,
                first_with(&|n| day(n).stem_index() == stem::BING, 10),
                "{year}"
            );
        }
        assert_eq!(first_month_counts(1644), None);
        assert_eq!(first_month_counts(2151), None);
    }
}
