//! 臘日 — the day of the year-end sacrifice, one rule per definition.
//!
//! 臘日 (*rōnichi*, *rōjitsu*) began as the Chinese 臘祭, the sacrifice to
//! the gods and the ancestors together at the end of the year, and
//! survives in the Japanese almanac as one of the 選日. Its day is
//! reckoned several ways, and many almanacs leave it out
//! (Japanese Wikipedia, 「臘日」, `wikipedia-ja-rounichi`, read
//! 2026-09-29). Each reckoning a source states is a [`RounichiRule`] of
//! its own ([`docs/policy.md`] §5); none is a default.
//! `docs/systems/japanese-almanac-notes.md` describes them.
//!
//! | Identifier | Rule | Stated by |
//! | --- | --- | --- |
//! | `second-dragon-after-minor-cold` | the second 辰 day after 小寒 | Japanese Wikipedia; こよみる; 日本文化研究ブログ |
//! | `dragon-nearest-major-cold` | the 辰 day nearest 大寒 | the same; こよみる calls it the current mainstream and the 神社暦's; 西野神社 |
//! | `first-dog-after-major-cold` | the first 戌 day after 大寒 | Japanese Wikipedia; こよみる; 日本文化研究ブログ |
//! | `lunar-twelfth-ninth` | the ninth of the twelfth lunar month | こよみる; 日本文化研究ブログ |
//! | `third-dog-after-winter-solstice` | the third 戌 day after 冬至, the 臘 of Qin and Han | 『説文解字』, through Chinese Wikipedia 「腊八节」 |
//!
//! Japanese Wikipedia writes the fourth Japanese reckoning 「丑節9日」,
//! the ninth of the 節月 丑月 rather than of the lunar month, without
//! saying whether 小寒's own day is the first; it is not a rule here,
//! because no source read settles that. The festival of the eighth of the
//! twelfth lunar month, 臘八, is not the 選日: こよみる says so outright.
//!
//! # The year
//!
//! Every rule takes the Gregorian year whose January holds 小寒 and 大寒,
//! and answers the 臘日 of the winter that ends in it: the day falls in
//! January or early February of that year, and the 冬至 the Han rule
//! counts from is the one of the December before.
//!
//! # Where a rule does not say
//!
//! "After" a term does not say whether the term's own day counts, and no
//! source read shows a year in which it would matter. So a rule that
//! counts days *after* a term answers `None` in a year whose term day
//! itself bears the sign, and the nearest-辰 rule answers `None` when the
//! 辰 days before and after 大寒 are equally far, six days each side. The
//! lunar rule reads the ordinary twelfth month next to 大寒, which is not
//! always the month holding it (in 1985 and 2053 it is not), and would
//! answer `None` where no month there is one; no winter of 1901–2100 is
//! such.
//!
//! [`docs/policy.md`]: https://github.com/kitsuyui/hyper-calendar/blob/main/docs/policy.md

use hc_calendar::Rd;
use hc_calendar::cycle::{branch, branch_day_on_or_after, sexagenary_day};
use hc_calendar::{Month, gregorian};
use hc_seasons::Meridian;
use hc_seasons::solar_terms::{SolarTerm, term_day};

use crate::lunisolar::lunisolar_date;

/// Days between two days of one branch.
const BRANCH_CYCLE: i64 = 12;

/// 小寒, the sectional term at 285° that opens 丑月.
const MINOR_COLD: SolarTerm = SolarTerm::WINTER_SOLSTICE.next();

/// 大寒, the principal term at 300°.
const MAJOR_COLD: SolarTerm = SolarTerm::WINTER_SOLSTICE.next().next();

/// One reckoning of 臘日.
#[derive(Debug, Clone, Copy)]
pub struct RounichiRule {
    /// The identifier, e.g. `"dragon-nearest-major-cold"`.
    pub id: &'static str,
    /// The rule as its sources write it, e.g. `"大寒に最も近い辰の日"`.
    pub japanese_rule: &'static str,
    /// The sources that state it.
    pub source: &'static str,
    /// The day in the winter ending in a Gregorian year, at a meridian;
    /// `None` where the rule does not say, as the module documentation
    /// sets out, or outside the years the astronomy answers for.
    pub day: fn(i64, Meridian) -> Option<Rd>,
}

impl RounichiRule {
    /// The rule's day in the winter that ends in Gregorian `year`.
    #[must_use]
    pub fn day_of_year(&self, year: i64, meridian: Meridian) -> Option<Rd> {
        if !gregorian::year_in_range(year - 1) || !gregorian::year_in_range(year + 1) {
            return None;
        }
        (self.day)(year, meridian)
    }

    /// Whether a day is 臘日 by this rule; `None` where the rule does not
    /// say for the winter the day falls in.
    #[must_use]
    pub fn is_rounichi(&self, day: Rd, meridian: Meridian) -> Option<bool> {
        let (year, month, _) = gregorian::from_fixed(day).ok()?;
        // The winter a day of the second half of a year belongs to ends in
        // the next year.
        let winter = if month >= 7 { year + 1 } else { year };
        self.day_of_year(winter, meridian).map(|found| found == day)
    }
}

/// Whether a day bears a branch.
const fn is_branch(day: Rd, branch_index: u8) -> bool {
    sexagenary_day(day).branch_index() == branch_index
}

/// The `nth` day of a branch after a term's day, counting from one;
/// `None` when the term's day itself bears the branch.
fn nth_after(term: Rd, branch_index: u8, nth: i64) -> Option<Rd> {
    if is_branch(term, branch_index) {
        return None;
    }
    let first = branch_day_on_or_after(term, branch_index);
    Some(Rd(first.0 + (nth - 1) * BRANCH_CYCLE))
}

/// 小寒後の2度目の辰の日.
fn second_dragon_after_minor_cold(year: i64, meridian: Meridian) -> Option<Rd> {
    nth_after(term_day(year, MINOR_COLD, meridian), branch::CHEN, 2)
}

/// 大寒に最も近い辰の日.
fn dragon_nearest_major_cold(year: i64, meridian: Meridian) -> Option<Rd> {
    let term = term_day(year, MAJOR_COLD, meridian);
    let after = branch_day_on_or_after(term, branch::CHEN);
    let before = Rd(after.0 - BRANCH_CYCLE);
    match (after.0 - term.0).cmp(&(term.0 - before.0)) {
        core::cmp::Ordering::Less => Some(after),
        core::cmp::Ordering::Greater => Some(before),
        core::cmp::Ordering::Equal => None,
    }
}

/// 大寒後の最初の戌の日.
fn first_dog_after_major_cold(year: i64, meridian: Meridian) -> Option<Rd> {
    nth_after(term_day(year, MAJOR_COLD, meridian), branch::XU, 1)
}

/// 旧暦12月9日: the ninth of the ordinary twelfth month of the lunar
/// year that ends near 大寒 — the month holding 大寒, the one before it or
/// the one after, whichever is numbered 12; `None` where none is, or two
/// are.
fn lunar_twelfth_ninth(year: i64, meridian: Meridian) -> Option<Rd> {
    let start_of_month = |day: Rd| Rd(day.0 - i64::from(lunisolar_date(day, meridian).day) + 1);
    let holding = start_of_month(term_day(year, MAJOR_COLD, meridian));
    let before = start_of_month(Rd(holding.0 - 5));
    let after = start_of_month(Rd(holding.0 + 35));
    let mut found = None;
    for start in [before, holding, after] {
        let candidate = Rd(start.0 + 8);
        let date = lunisolar_date(candidate, meridian);
        if date.month == Month::regular(12) && date.day == 9 {
            if found.is_some() {
                return None;
            }
            found = Some(candidate);
        }
    }
    found
}

/// 冬至後三戌: the third 戌 day after the December solstice before.
fn third_dog_after_winter_solstice(year: i64, meridian: Meridian) -> Option<Rd> {
    nth_after(
        term_day(year - 1, SolarTerm::WINTER_SOLSTICE, meridian),
        branch::XU,
        3,
    )
}

hc_core::catalogue! {
    type: RounichiRule,
    id: |rule| rule.id,
    provenance: |rule| rule.source,
    tests: rounichi_rule_tests,
    associated;

    /// Every reckoning, in the order of the table above.
    pub const ALL;
    /// The reckoning with this identifier.
    pub fn by_id;

    entries: {
        /// The second 辰 day after 小寒.
        pub const SECOND_DRAGON_AFTER_MINOR_COLD = Self {
            id: "second-dragon-after-minor-cold",
            japanese_rule: "小寒後の2度目の辰の日",
            source: "Japanese Wikipedia 「臘日」; こよみる 「臘日とは」; 日本文化研究ブログ 「臘日」",
            day: second_dragon_after_minor_cold,
        };
        /// The 辰 day nearest 大寒.
        pub const DRAGON_NEAREST_MAJOR_COLD = Self {
            id: "dragon-nearest-major-cold",
            japanese_rule: "大寒に最も近い辰の日",
            source: "Japanese Wikipedia 「臘日」; こよみる 「臘日とは」, the 神社暦's; 西野神社 「六曜・選日・二十四節気」",
            day: dragon_nearest_major_cold,
        };
        /// The first 戌 day after 大寒.
        pub const FIRST_DOG_AFTER_MAJOR_COLD = Self {
            id: "first-dog-after-major-cold",
            japanese_rule: "大寒後の最初の戌の日",
            source: "Japanese Wikipedia 「臘日」; こよみる 「臘日とは」; 日本文化研究ブログ 「臘日」",
            day: first_dog_after_major_cold,
        };
        /// The ninth of the twelfth lunar month.
        pub const LUNAR_TWELFTH_NINTH = Self {
            id: "lunar-twelfth-ninth",
            japanese_rule: "旧暦12月9日",
            source: "こよみる 「臘日とは」; 日本文化研究ブログ 「臘日」",
            day: lunar_twelfth_ninth,
        };
        /// The third 戌 day after 冬至, the 臘 of Qin and Han.
        pub const THIRD_DOG_AFTER_WINTER_SOLSTICE = Self {
            id: "third-dog-after-winter-solstice",
            japanese_rule: "冬至後三戌",
            source: "『説文解字』 「腊，冬至后三戌腊祭百神」, through Chinese Wikipedia 「腊八节」",
            day: third_dog_after_winter_solstice,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JAPAN: Meridian = Meridian::JAPAN;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    /// こよみる's candidates for 2024 to 2027 (`koyomil-rounichi`, read
    /// 2026-09-29), one day for each of its four reckonings: in 2025 the
    /// first two fall together on 23 January, which it lists once.
    #[test]
    fn the_published_candidates_of_2024_to_2027_match() {
        let rules = [
            RounichiRule::SECOND_DRAGON_AFTER_MINOR_COLD,
            RounichiRule::DRAGON_NEAREST_MAJOR_COLD,
            RounichiRule::FIRST_DOG_AFTER_MAJOR_COLD,
            RounichiRule::LUNAR_TWELFTH_NINTH,
        ];
        let expected = [
            (
                2024,
                [
                    ymd(2024, 1, 29),
                    ymd(2024, 1, 17),
                    ymd(2024, 1, 23),
                    ymd(2024, 1, 19),
                ],
            ),
            (
                2025,
                [
                    ymd(2025, 1, 23),
                    ymd(2025, 1, 23),
                    ymd(2025, 1, 29),
                    ymd(2025, 1, 8),
                ],
            ),
            (
                2026,
                [
                    ymd(2026, 1, 18),
                    ymd(2026, 1, 18),
                    ymd(2026, 1, 24),
                    ymd(2026, 1, 27),
                ],
            ),
            (
                2027,
                [
                    ymd(2027, 1, 25),
                    ymd(2027, 1, 25),
                    ymd(2027, 1, 31),
                    ymd(2027, 1, 16),
                ],
            ),
        ];
        hc_core::memo::scope(|| {
            for (year, days) in expected {
                for (rule, day) in rules.iter().zip(days) {
                    assert_eq!(
                        rule.day_of_year(year, JAPAN),
                        Some(day),
                        "{} {year}",
                        rule.id
                    );
                    assert_eq!(rule.is_rounichi(day, JAPAN), Some(true));
                    assert_eq!(rule.is_rounichi(Rd(day.0 + 1), JAPAN), Some(false));
                }
            }
        });
    }

    /// Each rule's day bears its sign, lies in the right window of its
    /// term, and a rule answers `None` only for the reasons it gives.
    #[test]
    fn every_day_bears_its_sign_and_every_refusal_has_its_reason() {
        hc_core::memo::scope(|| {
            for year in 1901..=2100 {
                let minor = term_day(year, MINOR_COLD, JAPAN);
                let major = term_day(year, MAJOR_COLD, JAPAN);
                match RounichiRule::SECOND_DRAGON_AFTER_MINOR_COLD.day_of_year(year, JAPAN) {
                    Some(day) => {
                        assert!(is_branch(day, branch::CHEN));
                        assert!((13..=24).contains(&(day.0 - minor.0)), "{year}");
                    }
                    None => assert!(is_branch(minor, branch::CHEN), "{year}"),
                }
                match RounichiRule::DRAGON_NEAREST_MAJOR_COLD.day_of_year(year, JAPAN) {
                    Some(day) => {
                        assert!(is_branch(day, branch::CHEN));
                        assert!((day.0 - major.0).abs() < 6, "{year}");
                    }
                    None => assert!(is_branch(Rd(major.0 + 6), branch::CHEN), "{year}"),
                }
                match RounichiRule::FIRST_DOG_AFTER_MAJOR_COLD.day_of_year(year, JAPAN) {
                    Some(day) => {
                        assert!(is_branch(day, branch::XU));
                        assert!((1..=11).contains(&(day.0 - major.0)), "{year}");
                    }
                    None => assert!(is_branch(major, branch::XU), "{year}"),
                }
                let solstice = term_day(year - 1, SolarTerm::WINTER_SOLSTICE, JAPAN);
                match RounichiRule::THIRD_DOG_AFTER_WINTER_SOLSTICE.day_of_year(year, JAPAN) {
                    Some(day) => {
                        assert!(is_branch(day, branch::XU));
                        assert!((25..=35).contains(&(day.0 - solstice.0)), "{year}");
                    }
                    None => assert!(is_branch(solstice, branch::XU), "{year}"),
                }
            }
        });
    }

    /// The lunar rule's day is the ninth of an ordinary twelfth month in
    /// every winter of 1901–2100, 1985's and 2053's included, where the
    /// month holding 大寒 is numbered otherwise.
    #[test]
    fn the_lunar_rule_is_the_ninth_of_the_twelfth_month() {
        hc_core::memo::scope(|| {
            for year in 1901..=2100 {
                let day = RounichiRule::LUNAR_TWELFTH_NINTH
                    .day_of_year(year, JAPAN)
                    .expect("an ordinary twelfth month");
                let date = lunisolar_date(day, JAPAN);
                assert_eq!((date.month, date.day), (Month::regular(12), 9), "{year}");
            }
        });
    }

    #[test]
    fn a_year_the_calendar_does_not_reach_is_refused() {
        let far = gregorian::MAX_YEAR;
        for rule in RounichiRule::ALL {
            assert_eq!(rule.day_of_year(far, JAPAN), None, "{}", rule.id);
        }
    }
}
