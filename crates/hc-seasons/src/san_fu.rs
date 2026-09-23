//! 三伏 and 數九: the hottest and coldest stretches of the Chinese year,
//! counted from the solstices.
//!
//! "冷在三九，熱在三伏" — cold in the third nine, hot in the three *fu*.
//! Both are counts that start at a solstice, and neither is a solar term:
//!
//! * **三伏**, the three *fu*, are counted in 庚 days — the seventh stem of
//!   the sexagenary day cycle, so one day in ten. 入伏, the first day of
//!   初伏, is the third 庚 day from the summer solstice; 中伏 begins on the
//!   fourth; 末伏 begins on the first 庚 day from 立秋, the beginning of
//!   autumn; and 出伏, the day the *fu* end, is the second. 初伏 and 末伏
//!   are ten days each, and 中伏 is ten or twenty, as the fifth 庚 day falls
//!   before 立秋 or after it.
//! * **數九**, counting the nines, starts at the winter solstice: nine
//!   periods of nine days, 一九 to 九九, eighty-one days in all, the third of
//!   them the coldest.
//!
//! # From the solstice or after it
//!
//! "夏至後第三庚" — the third 庚 day *after* the solstice — is read here as
//! counting the solstice itself when it is a 庚 day. That is the reading
//! that reproduces the published dates: in 2021 and 2023 the solstice fell
//! on a 庚 day, and 入伏 was 11 July, the third 庚 day counting it, not the
//! 21st. No year of the table falls on a 立秋 that is itself a 庚 day, so
//! the same reading is taken for 末伏 without a year to test it. For 數九
//! the solstice is the first day of 一九, as the Hong Kong Observatory counts
//! it; the older count that the source mentions, from the first 壬 day after
//! the solstice, is not carried.
//!
//! The day a solar term falls on depends on the meridian; the published
//! dates are China's, [`Meridian::CHINA`].
//!
//! Sources: 陳浩新, "「冷在三九，熱在三伏」", Hong Kong Observatory, for the
//! three *fu* and the third nine, retrieved 2026-09-23 from the Internet
//! Archive's copy of 15 June 2018; Wikipedia (zh), "三伏", retrieved
//! 2026-09-23, for the same rule from the Tang *陰陽書*, the lengths, and the
//! dates of 2017–2030; Wikipedia (zh), "數九", retrieved 2026-09-23, for the
//! nines and the alternative count.

use hc_calendar::Rd;
use hc_calendar::cycle::sexagenary_day;

use crate::solar_terms::term_day;
use crate::{Meridian, SolarTerm};

/// The index of 庚 among the ten stems, 甲 being 0.
const GENG: u8 = 6;

/// The `n`th 庚 day counting from `start`, `start` included.
fn nth_geng_from(start: Rd, n: i64) -> Rd {
    let offset = (i64::from(GENG) - i64::from(sexagenary_day(start).stem_index())).rem_euclid(10);
    Rd(start.0 + offset + 10 * (n - 1))
}

/// Which of the three *fu* a day is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fu {
    /// 初伏, the first ten days.
    Chu,
    /// 中伏, ten or twenty days.
    Zhong,
    /// 末伏, the last ten days.
    Mo,
}

impl Fu {
    /// The name in Chinese.
    #[must_use]
    pub const fn chinese_name(self) -> &'static str {
        match self {
            Self::Chu => "初伏",
            Self::Zhong => "中伏",
            Self::Mo => "末伏",
        }
    }
}

/// The three *fu* of a year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SanFu {
    /// 入伏, the first day of 初伏: the third 庚 day from the summer solstice.
    pub chu: Rd,
    /// The first day of 中伏: the fourth 庚 day from the summer solstice.
    pub zhong: Rd,
    /// The first day of 末伏: the first 庚 day from 立秋.
    pub mo: Rd,
    /// 出伏, the day after the *fu*: the second 庚 day from 立秋.
    pub end: Rd,
}

impl SanFu {
    /// The three *fu* of `year`, with the solar terms at `meridian`.
    #[must_use]
    pub fn of_year(year: i64, meridian: Meridian) -> Self {
        let solstice = term_day(year, SolarTerm::SUMMER_SOLSTICE, meridian);
        let autumn = term_day(year, SolarTerm::BEGINNING_OF_AUTUMN, meridian);
        Self {
            chu: nth_geng_from(solstice, 3),
            zhong: nth_geng_from(solstice, 4),
            mo: nth_geng_from(autumn, 1),
            end: nth_geng_from(autumn, 2),
        }
    }

    /// How many days 中伏 lasts: ten or twenty.
    #[must_use]
    pub const fn zhong_days(&self) -> i64 {
        self.mo.0 - self.zhong.0
    }

    /// Which *fu* a day is in, if any.
    #[must_use]
    pub const fn fu_of(&self, day: Rd) -> Option<Fu> {
        if day.0 < self.chu.0 || day.0 >= self.end.0 {
            None
        } else if day.0 < self.zhong.0 {
            Some(Fu::Chu)
        } else if day.0 < self.mo.0 {
            Some(Fu::Zhong)
        } else {
            Some(Fu::Mo)
        }
    }
}

/// Which nine of 數九 a day falls in, and which day of it: `(nine, day)`,
/// each counted from 1, from the winter solstice at or before `day` — the
/// solstice of the previous year for a day in January. `None` outside the
/// eighty-one days.
#[must_use]
pub fn shu_jiu(day: Rd, meridian: Meridian) -> Option<(u8, u8)> {
    let (year, _, _) = crate::gregorian::year_month_day_from_rd(day);
    let this_year = term_day(year, SolarTerm::WINTER_SOLSTICE, meridian);
    let start = if day >= this_year {
        this_year
    } else {
        term_day(year - 1, SolarTerm::WINTER_SOLSTICE, meridian)
    };
    let elapsed = day.0 - start.0;
    if !(0..81).contains(&elapsed) {
        return None;
    }
    Some(((elapsed / 9 + 1) as u8, (elapsed % 9 + 1) as u8))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::{from_year_month_day, year_month_day_from_rd};

    #[test]
    fn the_three_fu_begin_on_the_days_published_for_2017_to_2030() {
        // Wikipedia (zh), "三伏", the table of recent years: the first day of
        // each fu. Its 2029 row ends 中伏 on 8 August and begins 末伏 on the
        // same day; 末伏 does begin on the 8th, so 中伏 ends on the 7th.
        type MonthDay = (u8, u8);
        let table: [(i64, MonthDay, MonthDay, MonthDay); 14] = [
            (2017, (7, 12), (7, 22), (8, 11)),
            (2018, (7, 17), (7, 27), (8, 16)),
            (2019, (7, 12), (7, 22), (8, 11)),
            (2020, (7, 16), (7, 26), (8, 15)),
            (2021, (7, 11), (7, 21), (8, 10)),
            (2022, (7, 16), (7, 26), (8, 15)),
            (2023, (7, 11), (7, 21), (8, 10)),
            (2024, (7, 15), (7, 25), (8, 14)),
            (2025, (7, 20), (7, 30), (8, 9)),
            (2026, (7, 15), (7, 25), (8, 14)),
            (2027, (7, 20), (7, 30), (8, 9)),
            (2028, (7, 14), (7, 24), (8, 13)),
            (2029, (7, 19), (7, 29), (8, 8)),
            (2030, (7, 14), (7, 24), (8, 13)),
        ];
        for (year, chu, zhong, mo) in table {
            let fu = SanFu::of_year(year, Meridian::CHINA);
            assert_eq!(fu.chu, from_year_month_day(year, chu.0, chu.1), "{year}");
            assert_eq!(
                fu.zhong,
                from_year_month_day(year, zhong.0, zhong.1),
                "{year}"
            );
            assert_eq!(fu.mo, from_year_month_day(year, mo.0, mo.1), "{year}");
            // Each fu starts on a 庚 day, and 初伏 and 末伏 are ten days.
            for start in [fu.chu, fu.zhong, fu.mo, fu.end] {
                assert_eq!(sexagenary_day(start).stem_index(), GENG);
            }
            assert_eq!(fu.zhong.0 - fu.chu.0, 10);
            assert_eq!(fu.end.0 - fu.mo.0, 10);
            assert!(matches!(fu.zhong_days(), 10 | 20), "{year}");
        }
        // 2025 has the short 中伏, of ten days, and 2026 the long one.
        assert_eq!(SanFu::of_year(2025, Meridian::CHINA).zhong_days(), 10);
        assert_eq!(SanFu::of_year(2026, Meridian::CHINA).zhong_days(), 20);
    }

    #[test]
    fn a_solstice_on_a_geng_day_is_the_first_geng_day() {
        // 21 June 2021 was a 庚 day, and 入伏 is the third 庚 day counting it.
        let solstice = term_day(2021, SolarTerm::SUMMER_SOLSTICE, Meridian::CHINA);
        assert_eq!(year_month_day_from_rd(solstice), (2021, 6, 21));
        assert_eq!(sexagenary_day(solstice).stem_index(), GENG);
        assert_eq!(
            SanFu::of_year(2021, Meridian::CHINA).chu,
            Rd(solstice.0 + 20)
        );
    }

    #[test]
    fn a_day_knows_its_fu() {
        let fu = SanFu::of_year(2026, Meridian::CHINA);
        assert_eq!(fu.fu_of(from_year_month_day(2026, 7, 14)), None);
        assert_eq!(fu.fu_of(from_year_month_day(2026, 7, 15)), Some(Fu::Chu));
        assert_eq!(fu.fu_of(from_year_month_day(2026, 7, 25)), Some(Fu::Zhong));
        assert_eq!(fu.fu_of(from_year_month_day(2026, 8, 13)), Some(Fu::Zhong));
        assert_eq!(fu.fu_of(from_year_month_day(2026, 8, 14)), Some(Fu::Mo));
        assert_eq!(fu.fu_of(from_year_month_day(2026, 8, 24)), None);
        assert_eq!(Fu::Mo.chinese_name(), "末伏");
    }

    #[test]
    fn the_nines_count_from_the_winter_solstice() {
        // The winter solstice of 2025 fell on 21 December in China: 一九 is 21
        // to 29 December, and 三九, the coldest, 8 to 16 January 2026, "in
        // mid-January" as the Observatory has it.
        let solstice = term_day(2025, SolarTerm::WINTER_SOLSTICE, Meridian::CHINA);
        assert_eq!(year_month_day_from_rd(solstice), (2025, 12, 21));
        assert_eq!(
            shu_jiu(from_year_month_day(2025, 12, 20), Meridian::CHINA),
            None
        );
        assert_eq!(shu_jiu(solstice, Meridian::CHINA), Some((1, 1)));
        assert_eq!(
            shu_jiu(from_year_month_day(2026, 1, 8), Meridian::CHINA),
            Some((3, 1))
        );
        assert_eq!(
            shu_jiu(from_year_month_day(2026, 1, 16), Meridian::CHINA),
            Some((3, 9))
        );
        // The eighty-first day, and the day after it.
        assert_eq!(shu_jiu(Rd(solstice.0 + 80), Meridian::CHINA), Some((9, 9)));
        assert_eq!(shu_jiu(Rd(solstice.0 + 81), Meridian::CHINA), None);
    }
}
