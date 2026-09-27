//! 三元九運 — the three eras and nine periods of twenty years.
//!
//! 玄空 feng shui divides time into periods of twenty years, each ruled by
//! one of the nine stars in turn: three periods make an era, 元, of sixty
//! years, and the three eras 上元, 中元 and 下元 a cycle of 180. The present
//! cycle began in 1864, a 甲子 year, with 上元一運, and 九運, ruled by 九紫,
//! runs from 立春 2024 (阐微堂, 「无常派玄空地理—— 三元九运」, 2023-11-22,
//! `chanweitang-sanyuan-jiuyun`). `docs/systems/east-asian-folk-days.md`
//! describes the system and its tests.
//!
//! | 元 | 運 | Years | 九星 | Ruler, as the source writes it |
//! |---|---|---|---|---|
//! | 上元 | 一運 | 1864–1883 | 一白水星 | 贪狼 |
//! | 上元 | 二運 | 1884–1903 | 二黒土星 | 巨门 |
//! | 上元 | 三運 | 1904–1923 | 三碧木星 | 禄存 |
//! | 中元 | 四運 | 1924–1943 | 四緑木星 | 文曲 |
//! | 中元 | 五運 | 1944–1963 | 五黄土星 | 廉贞 |
//! | 中元 | 六運 | 1964–1983 | 六白金星 | 武曲 |
//! | 下元 | 七運 | 1984–2003 | 七赤金星 | 破军 |
//! | 下元 | 八運 | 2004–2023 | 八白土星 | 左辅 |
//! | 下元 | 九運 | 2024–2043 | 九紫火星 | 右弼 |
//!
//! A period begins at 立春 of its first year: the source has 八運 "从2004年
//! 立春起至2024年立春止", and its table gives each period's first and last
//! years, so a period ends at 立春 of the year after its last. The years
//! before 1864 and after 2043 are the same cycle of 180 years continued.
//! The source's 大三元 of 540 years is not carried: it gives no epoch for
//! it.

use hc_calendar::Rd;
use hc_seasons::Meridian;

use crate::nine_stars::{NineStar, nine_star_year};

/// The first year of the cycle the source tabulates, 1864, 上元一運.
pub const EPOCH_YEAR: i64 = 1864;

/// The length of a period, in years.
pub const PERIOD_YEARS: i64 = 20;

/// The length of the cycle of three eras, in years.
pub const CYCLE_YEARS: i64 = 180;

/// One of the three eras of sixty years.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Era {
    /// 上元, periods one to three.
    Upper,
    /// 中元, periods four to six.
    Middle,
    /// 下元, periods seven to nine.
    Lower,
}

impl Era {
    /// The name, e.g. `"上元"`.
    #[must_use]
    pub const fn chinese_name(self) -> &'static str {
        match self {
            Self::Upper => "上元",
            Self::Middle => "中元",
            Self::Lower => "下元",
        }
    }
}

/// One of the nine periods of twenty years, with the years it covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Period {
    /// The period's number, 1 to 9.
    pub number: u8,
    /// The year whose 立春 begins it.
    pub first_year: i64,
}

impl Period {
    /// The era the period belongs to.
    #[must_use]
    pub const fn era(self) -> Era {
        match self.number {
            1..=3 => Era::Upper,
            4..=6 => Era::Middle,
            _ => Era::Lower,
        }
    }

    /// The star that rules the period: 一白 for 一運 through 九紫 for 九運.
    #[must_use]
    pub const fn star(self) -> NineStar {
        NineStar::from_number(self.number as i64)
    }

    /// The last year the period covers; it ends at the next year's 立春.
    #[must_use]
    pub const fn last_year(self) -> i64 {
        self.first_year + PERIOD_YEARS - 1
    }

    /// The name of the period, e.g. `"九運"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        [
            "一運", "二運", "三運", "四運", "五運", "六運", "七運", "八運", "九運",
        ][(self.number - 1) as usize]
    }

    /// The name of the period in simplified Chinese, as the source writes
    /// it, e.g. `"九运"`.
    #[must_use]
    pub const fn chinese_name(self) -> &'static str {
        [
            "一运", "二运", "三运", "四运", "五运", "六运", "七运", "八运", "九运",
        ][(self.number - 1) as usize]
    }

    /// The star of the Northern Dipper and its two attendants that the
    /// source names as the period's ruler, in simplified Chinese: 贪狼,
    /// 巨门, 禄存, 文曲, 廉贞, 武曲, 破军, 左辅, 右弼.
    #[must_use]
    pub const fn ruling_star_name(self) -> &'static str {
        [
            "贪狼", "巨门", "禄存", "文曲", "廉贞", "武曲", "破军", "左辅", "右弼",
        ][(self.number - 1) as usize]
    }
}

/// The period of a year counted as the 九星 year is, from 立春.
#[must_use]
pub const fn period_of_year(year: i64) -> Period {
    let index = (year - EPOCH_YEAR).div_euclid(PERIOD_YEARS);
    let number = index.rem_euclid(9) as u8 + 1;
    Period {
        number,
        first_year: EPOCH_YEAR + index * PERIOD_YEARS,
    }
}

/// The period in force on a day, the year turning at 立春 at a meridian.
#[must_use]
pub fn period(day: Rd, meridian: Meridian) -> Period {
    period_of_year(nine_star_year(day, meridian))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::gregorian;

    /// The source's table of the nine periods of 1864–2043.
    #[test]
    fn the_nine_periods_of_1864_to_2043_are_the_sources() {
        let table = [
            (1, 1864, 1883, "上元", "贪狼"),
            (2, 1884, 1903, "上元", "巨门"),
            (3, 1904, 1923, "上元", "禄存"),
            (4, 1924, 1943, "中元", "文曲"),
            (5, 1944, 1963, "中元", "廉贞"),
            (6, 1964, 1983, "中元", "武曲"),
            (7, 1984, 2003, "下元", "破军"),
            (8, 2004, 2023, "下元", "左辅"),
            (9, 2024, 2043, "下元", "右弼"),
        ];
        for (number, first, last, era, ruler) in table {
            for year in [first, first + 7, last] {
                let period = period_of_year(year);
                assert_eq!(period.number, number, "{year}");
                assert_eq!((period.first_year, period.last_year()), (first, last));
                assert_eq!(period.era().chinese_name(), era);
                assert_eq!(period.ruling_star_name(), ruler);
                assert_eq!(period.star().number(), number);
            }
        }
        assert_eq!(period_of_year(2024).star(), NineStar::NinePurple);
        assert_eq!(period_of_year(2024).japanese_name(), "九運");
        assert_eq!(period_of_year(2024).chinese_name(), "九运");
    }

    /// "从2024年立春起": 九運 begins at 立春, 4 February 2024 at the Chinese
    /// meridian, and the day before is still 八運.
    #[test]
    fn a_period_turns_at_the_beginning_of_spring() {
        let before = gregorian::to_fixed(2024, 2, 3).expect("a date");
        let after = gregorian::to_fixed(2024, 2, 4).expect("a date");
        assert_eq!(period(before, Meridian::CHINA).number, 8);
        assert_eq!(period(after, Meridian::CHINA).number, 9);
    }

    /// The cycle of 180 years continued both ways: 2044 opens 上元一運
    /// again, and 1863 closes the 九運 of 1844–1863.
    #[test]
    fn the_cycle_repeats_every_one_hundred_and_eighty_years() {
        assert_eq!(
            period_of_year(2044),
            Period {
                number: 1,
                first_year: 2044
            }
        );
        assert_eq!(
            period_of_year(1863),
            Period {
                number: 9,
                first_year: 1844
            }
        );
        for year in [1500, 1864, 1999, 2300] {
            assert_eq!(
                period_of_year(year).number,
                period_of_year(year + CYCLE_YEARS).number
            );
        }
    }
}
