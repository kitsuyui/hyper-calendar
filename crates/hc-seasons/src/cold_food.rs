//! 寒食, the Cold Food Day, under each of the reckonings that date it.
//!
//! 寒食 is counted from a solar term, not dated in a month, and the count
//! has changed, so it is carried as one named convention per reckoning, as
//! `docs/policy.md` §5 has it:
//!
//! | Convention | Rule | Meridian |
//! | --- | --- | --- |
//! | [`ColdFoodConvention::SolsticePlus105`] | 105 days after the winter solstice, the older Chinese reckoning | [`Meridian::CHINA`] |
//! | [`ColdFoodConvention::EveOfQingming`] | the day before 清明, as kept in China after the 時憲曆 of 1645 | [`Meridian::CHINA`] |
//! | [`ColdFoodConvention::Hansik`] | Korea's 한식, 105 days after 동지 | [`Meridian::KOREA`] |
//!
//! "105 days after" is 105 days *after* the solstice's day, the solstice
//! not counted: that is what the Korea Astronomy and Space Science
//! Institute's 한식 of 5 April 2024, 5 April 2025 and 6 April 2026 require,
//! against the solstices of 22 December 2023, 21 December 2024 and 22
//! December 2025. Counting the solstice as the first of the 105 would give
//! each a day earlier. Vietnam's Tết Hàn thực is a lunar date, the third
//! of the third month, and is in `hc-holiday`'s Vietnamese folk table, not
//! here. The count of 106 days that the older texts also mention is not
//! carried. The reckonings, the worked example and the sources are in
//! `docs/systems/solar-term-counts.md`.

use hc_calendar::Rd;

use crate::solar_terms::term_day;
use crate::{Meridian, SolarTerm};

/// 清明, the fifth solar term, at 15°.
const QINGMING: SolarTerm = match SolarTerm::from_degrees(15) {
    Some(term) => term,
    None => SolarTerm::SPRING_EQUINOX,
};

/// How many days after the winter solstice the solstice reckonings put the
/// day.
pub const DAYS_AFTER_SOLSTICE: i64 = 105;

/// Which reckoning of the Cold Food Day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColdFoodConvention {
    /// 冬至後一百五日: 105 days after the winter solstice, at the Chinese
    /// meridian — the reckoning of the Chinese texts before 1645.
    SolsticePlus105,
    /// 清明前一日: the day before 清明, at the Chinese meridian — the
    /// reckoning kept in China after the 時憲曆 of 1645 shortened the
    /// interval from the solstice.
    EveOfQingming,
    /// 한식: 105 days after 동지, at the Korean meridian, as the Korea
    /// Astronomy and Space Science Institute dates it.
    Hansik,
}

impl ColdFoodConvention {
    /// Every convention, in the order of the module table.
    pub const ALL: [Self; 3] = [Self::SolsticePlus105, Self::EveOfQingming, Self::Hansik];

    /// A short identifier, in kebab case.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::SolsticePlus105 => "hanshi-solstice-105",
            Self::EveOfQingming => "hanshi-eve-of-qingming",
            Self::Hansik => "hansik",
        }
    }

    /// The name in the convention's own language.
    #[must_use]
    pub const fn local_name(self) -> &'static str {
        match self {
            Self::SolsticePlus105 | Self::EveOfQingming => "寒食",
            Self::Hansik => "한식",
        }
    }

    /// The meridian the solar term's day is read at.
    #[must_use]
    pub const fn meridian(self) -> Meridian {
        match self {
            Self::SolsticePlus105 | Self::EveOfQingming => Meridian::CHINA,
            Self::Hansik => Meridian::KOREA,
        }
    }

    /// The Cold Food Day of Gregorian `year` under this convention: in
    /// April, or the first days of it, every year.
    #[must_use]
    pub fn day(self, year: i64) -> Rd {
        let meridian = self.meridian();
        match self {
            Self::SolsticePlus105 | Self::Hansik => {
                let solstice = term_day(year - 1, SolarTerm::WINTER_SOLSTICE, meridian);
                Rd(solstice.0 + DAYS_AFTER_SOLSTICE)
            }
            Self::EveOfQingming => Rd(term_day(year, QINGMING, meridian).0 - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::{from_year_month_day, year_month_day_from_rd};

    #[test]
    fn hansik_falls_where_the_korean_almanac_puts_it() {
        // KASI, 「2024년도 월력요항」, 「2025년도 월력요항」 and 「2026년
        // 월력요항」: "한식은 4월 5일(금)", "4월 5일(토)", "4월 6일(월)".
        for (year, month, day) in [(2024, 4, 5), (2025, 4, 5), (2026, 4, 6)] {
            assert_eq!(
                ColdFoodConvention::Hansik.day(year),
                from_year_month_day(year, month, day),
                "한식 {year}"
            );
        }
    }

    #[test]
    fn the_solstice_itself_is_not_the_first_of_the_105_days() {
        // 동지 of 2023, 2024 and 2025 at the Korean meridian, and 한식 105
        // days after each: counting the solstice as day 1 would put 한식 on
        // 4 April 2024, 4 April 2025 and 5 April 2026, which KASI does not.
        let solstices = [(2023, 12, 22), (2024, 12, 21), (2025, 12, 22)];
        for (year, month, day) in solstices {
            let solstice = term_day(year, SolarTerm::WINTER_SOLSTICE, Meridian::KOREA);
            assert_eq!(year_month_day_from_rd(solstice), (year, month, day));
            assert_eq!(ColdFoodConvention::Hansik.day(year + 1).0 - solstice.0, 105);
        }
    }

    #[test]
    fn hansik_is_on_cheongmyeong_or_the_day_after() {
        // "한식은 어느 해나 청명절 바로 다음날이거나 같은 날에 든다"
        // (Encyclopedia of Korean Culture, "한식").
        for year in 1900..=2100 {
            let cheongmyeong = term_day(year, QINGMING, Meridian::KOREA);
            let gap = ColdFoodConvention::Hansik.day(year).0 - cheongmyeong.0;
            assert!(matches!(gap, 0 | 1), "{year}: {gap}");
        }
    }

    #[test]
    fn the_eve_of_qingming_is_the_day_before_it() {
        // 清明 was 4 April 2025 and 5 April 2026 in China.
        assert_eq!(
            ColdFoodConvention::EveOfQingming.day(2025),
            from_year_month_day(2025, 4, 3)
        );
        assert_eq!(
            ColdFoodConvention::EveOfQingming.day(2026),
            from_year_month_day(2026, 4, 4)
        );
        // The older reckoning is a day or two after the newer one.
        for year in 1900..=2100 {
            let gap = ColdFoodConvention::SolsticePlus105.day(year).0
                - ColdFoodConvention::EveOfQingming.day(year).0;
            assert!(matches!(gap, 1 | 2), "{year}: {gap}");
        }
    }

    #[test]
    fn the_conventions_have_distinct_identifiers() {
        let ids = ColdFoodConvention::ALL.map(ColdFoodConvention::id);
        assert_eq!(
            ids,
            ["hanshi-solstice-105", "hanshi-eve-of-qingming", "hansik"]
        );
        assert_eq!(ColdFoodConvention::Hansik.local_name(), "한식");
    }
}
