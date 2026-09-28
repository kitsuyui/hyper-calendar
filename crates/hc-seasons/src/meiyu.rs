//! 入梅 and 出梅 in the Chinese almanac: the plum rains counted in day
//! signs from 芒种 and 小暑.
//!
//! The 民用通书 put the beginning of the plum rains, 入梅, on a stem day
//! after 芒种 and their end, 出梅, on a branch day after 小暑, and the
//! regions differ on the stem. 中国气象局, 「梅雨与农事」
//! (`cma-meiyu-nongshi`, retrieved 2026-09-28), gives South China's
//! 芒种后逢丙入梅, 小暑后逢未出梅, and Central China's 芒种后逢壬入梅, four
//! or six days away. Each convention is a function of its own, under
//! policy §5:
//!
//! | Function | Identifier | Rule | Region in the source |
//! | --- | --- | --- | --- |
//! | [`ru_mei_bing`] | `ru-mei-bing` | the first 丙 day from 芒种 | 华南 |
//! | [`ru_mei_ren`] | `ru-mei-ren` | the first 壬 day from 芒种 | 华中 |
//! | [`chu_mei_wei`] | `chu-mei-wei` | the first 未 day from 小暑 | 华南 |
//!
//! The identifiers are the table [`PlumRainRule::ALL`], by which a caller at
//! the boundary selects a rule.
//!
//! The source gives no 出梅 for Central China, and none is carried. A
//! reading that puts 入梅 at 立夏's first 庚 day and 出梅 at 芒种's first 壬
//! day, attributed to 闽人 in one news article, is not carried either.
//!
//! # From the term or after it
//!
//! A term day that is itself the sign is counted. That reproduces the
//! published 出梅 of 2024: 小暑 fell on 6 July, a 未 day, and 出梅 was that
//! day (`qq-meiyu-2024`). No year read has 芒种 on a 丙 or a 壬 day, and
//! the same reading is taken for 入梅, as [`crate::san_fu`] does for 末伏.
//!
//! The day a term falls on depends on the meridian; the published days are
//! China's, [`Meridian::CHINA`]. None of this is the Japanese 入梅 of
//! [`crate::zassetsu`], which is at 80° of solar longitude, nor the
//! meteorological onset of the rains, which the weather service declares
//! from observation.
//!
//! The dated examples, from news articles that apply the rule, are in the
//! tests: `qq-meiyu-2024`, `qq-meiyu-2025`, `netease-meiyu-2025` and
//! `qq-meiyu-2026`. `docs/systems/east-asian-folk-days.md` in the
//! repository describes the rules and their tests.

use hc_calendar::Rd;
use hc_calendar::cycle::{branch, branch_day_on_or_after, stem, stem_day_on_or_after};

use crate::solar_terms::term_day;
use crate::{Meridian, SolarTerm};

/// 入梅 by South China's rule: the first 丙 day from 芒种 of `year`, with
/// the term at `meridian`.
#[must_use]
pub fn ru_mei_bing(year: i64, meridian: Meridian) -> Rd {
    stem_day_on_or_after(
        term_day(year, SolarTerm::GRAIN_IN_EAR, meridian),
        stem::BING,
    )
}

/// 入梅 by Central China's rule: the first 壬 day from 芒种 of `year`, with
/// the term at `meridian`.
#[must_use]
pub fn ru_mei_ren(year: i64, meridian: Meridian) -> Rd {
    stem_day_on_or_after(term_day(year, SolarTerm::GRAIN_IN_EAR, meridian), stem::REN)
}

/// 出梅 by South China's rule: the first 未 day from 小暑 of `year`, with
/// the term at `meridian`.
#[must_use]
pub fn chu_mei_wei(year: i64, meridian: Meridian) -> Rd {
    branch_day_on_or_after(term_day(year, SolarTerm::MINOR_HEAT, meridian), branch::WEI)
}

/// A rule of 入梅 or 出梅 under the identifier a caller selects it by
/// (policy §5): the function that gives its day in a Gregorian year, with
/// its solar term at a meridian.
#[derive(Debug, Clone, Copy)]
pub struct PlumRainRule {
    /// A stable identifier, lowercase and hyphenated.
    pub id: &'static str,
    /// The day the rule gives in a Gregorian year.
    pub day: fn(i64, Meridian) -> Rd,
}

hc_core::catalogue! {
    type: PlumRainRule,
    id: |rule| rule.id,
    tests: plum_rain_rule_catalogue_tests,
    associated;

    /// The three rules, in the order of the module's table.
    pub const ALL;
    /// The rule with this identifier.
    pub fn by_id;

    entries: {
        /// [`ru_mei_bing`].
        pub const RU_MEI_BING = Self { id: "ru-mei-bing", day: ru_mei_bing };
        /// [`ru_mei_ren`].
        pub const RU_MEI_REN = Self { id: "ru-mei-ren", day: ru_mei_ren };
        /// [`chu_mei_wei`].
        pub const CHU_MEI_WEI = Self { id: "chu-mei-wei", day: chu_mei_wei };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::cycle::readings::HAN;
    use hc_calendar::cycle::sexagenary_day;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        hc_calendar::gregorian::to_fixed(year, month, day).expect("a date")
    }

    fn name(day: Rd) -> (&'static str, &'static str) {
        let sign = sexagenary_day(day);
        (HAN.stem(sign), HAN.branch(sign))
    }

    #[test]
    fn the_plum_rains_of_2026_are_the_published_days() {
        // 腾讯新闻, 「梅雨时节将至，今年哪天入梅？哪天出梅？」 (`qq-meiyu-2026`):
        // 芒种 on 5 June, 入梅 on 11 June, 丙辰; 小暑 on 7 July, 出梅 on
        // 8 July, 癸未; 27 days.
        let meridian = Meridian::CHINA;
        assert_eq!(
            term_day(2026, SolarTerm::GRAIN_IN_EAR, meridian),
            ymd(2026, 6, 5)
        );
        assert_eq!(ru_mei_bing(2026, meridian), ymd(2026, 6, 11));
        assert_eq!(name(ymd(2026, 6, 11)), ("丙", "辰"));
        assert_eq!(
            term_day(2026, SolarTerm::MINOR_HEAT, meridian),
            ymd(2026, 7, 7)
        );
        assert_eq!(chu_mei_wei(2026, meridian), ymd(2026, 7, 8));
        assert_eq!(name(ymd(2026, 7, 8)), ("癸", "未"));
        assert_eq!(
            chu_mei_wei(2026, meridian).0 - ru_mei_bing(2026, meridian).0,
            27
        );
    }

    #[test]
    fn the_two_rules_of_2025_are_six_days_apart() {
        // 网易, 「2025年入梅时间表来了！」 (`netease-meiyu-2025`): 芒种 on
        // 5 June, 乙巳, 入梅 on 6 June, 丙午; 出梅 on 13 July, 癸未. 腾讯新闻
        // (`qq-meiyu-2025`): "五月芒种后遇壬入梅", 12 June, 壬子.
        let meridian = Meridian::CHINA;
        assert_eq!(name(ymd(2025, 6, 5)), ("乙", "巳"));
        assert_eq!(ru_mei_bing(2025, meridian), ymd(2025, 6, 6));
        assert_eq!(ru_mei_ren(2025, meridian), ymd(2025, 6, 12));
        assert_eq!(name(ymd(2025, 6, 12)), ("壬", "子"));
        assert_eq!(chu_mei_wei(2025, meridian), ymd(2025, 7, 13));
    }

    #[test]
    fn a_term_day_on_the_sign_is_the_day_itself() {
        // 腾讯新闻 (`qq-meiyu-2024`): 芒种 on 5 June, 入梅 on 11 June, 丙午;
        // 小暑 on 6 July, which is itself the first 未 day, so 出梅 is 6 July.
        let meridian = Meridian::CHINA;
        assert_eq!(ru_mei_bing(2024, meridian), ymd(2024, 6, 11));
        assert_eq!(name(ymd(2024, 6, 11)), ("丙", "午"));
        assert_eq!(
            term_day(2024, SolarTerm::MINOR_HEAT, meridian),
            ymd(2024, 7, 6)
        );
        assert_eq!(chu_mei_wei(2024, meridian), ymd(2024, 7, 6));
    }

    #[test]
    fn every_day_is_its_sign_and_within_a_cycle_of_its_term() {
        let meridian = Meridian::CHINA;
        for year in 1900..=2100 {
            let grain = term_day(year, SolarTerm::GRAIN_IN_EAR, meridian);
            let heat = term_day(year, SolarTerm::MINOR_HEAT, meridian);
            for (day, start, width) in [
                (ru_mei_bing(year, meridian), grain, 10),
                (ru_mei_ren(year, meridian), grain, 10),
                (chu_mei_wei(year, meridian), heat, 12),
            ] {
                assert!((0..width).contains(&(day.0 - start.0)), "{year}");
            }
            assert_eq!(sexagenary_day(ru_mei_bing(year, meridian)).stem_index(), 2);
            assert_eq!(sexagenary_day(ru_mei_ren(year, meridian)).stem_index(), 8);
            assert_eq!(
                sexagenary_day(chu_mei_wei(year, meridian)).branch_index(),
                7
            );
            // The source's "相差4或6天".
            let apart = (ru_mei_ren(year, meridian).0 - ru_mei_bing(year, meridian).0).abs();
            assert!(apart == 4 || apart == 6, "{year}: {apart}");
        }
    }
}
