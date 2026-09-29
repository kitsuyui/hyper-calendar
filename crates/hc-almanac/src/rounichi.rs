//! 臘日 — the day of the year-end sacrifice, one rule per definition.
//!
//! 臘日 (*rōnichi*, *rōjitsu*) began as the Chinese 臘祭, the sacrifice to
//! the gods and the ancestors together at the end of the year, and
//! survives in the Japanese almanac as one of the 選日. Its day is
//! reckoned several ways, and many almanacs leave it out
//! (Japanese Wikipedia, 「臘日」, `wikipedia-ja-rounichi`, read
//! 2026-09-29). Each reckoning a source states, and each reading of its
//! wording where it admits two, is a [`RounichiRule`] of its own
//! ([`docs/policy.md`] §5); none is a default.
//! `docs/systems/japanese-almanac-notes.md` describes them.
//!
//! | Identifier | Rule | Stated by |
//! | --- | --- | --- |
//! | `second-dragon-after-minor-cold`, `second-dragon-from-minor-cold` | the second 辰 day after 小寒 | Japanese Wikipedia; こよみる; 日本文化研究ブログ |
//! | `dragon-nearest-major-cold-earlier`, `dragon-nearest-major-cold-later` | the 辰 day nearest 大寒 | the same; こよみる calls it the current mainstream and the 神社暦's; 西野神社 |
//! | `first-dog-after-major-cold`, `first-dog-from-major-cold` | the first 戌 day after 大寒 | Japanese Wikipedia; こよみる; 日本文化研究ブログ |
//! | `lunar-twelfth-ninth` | the ninth of the twelfth lunar month | こよみる; 日本文化研究ブログ |
//! | `ox-month-ninth-from-minor-cold`, `ox-month-ninth-after-minor-cold` | 「丑節9日」, the ninth day of the 節月 丑月 | Japanese Wikipedia |
//! | `third-dog-after-winter-solstice`, `third-dog-from-winter-solstice` | the third 戌 day after 冬至, the 臘 of Qin and Han | 『説文解字』, through Chinese Wikipedia 「腊八节」 |
//!
//! The festival of the eighth of the twelfth lunar month, 臘八, is not the
//! 選日: こよみる says so outright.
//!
//! # Two readings of one wording
//!
//! Where a rule's wording admits two readings, each is a rule of its own:
//!
//! * **"After" a term** does not say whether the term's own day counts,
//!   and no source read shows a year in which it would matter. An `-after-`
//!   rule does not count it: the first day of the sign is the first one
//!   strictly after the term's day. A `-from-` rule counts it: in a winter
//!   whose term day bears the sign, that day is the first.
//! * **The 辰 day nearest 大寒** does not say which of two 辰 days six days
//!   either side of it is meant. `-earlier` takes the one before 大寒 and
//!   `-later` the one after.
//! * **「丑節9日」** does not say whether 小寒's own day is the first of
//!   丑月. `ox-month-ninth-from-minor-cold` counts it, as the 節月 of
//!   [`crate::context`] does, which puts the whole of 小寒's day in 丑月,
//!   and answers 小寒 + 8; `ox-month-ninth-after-minor-cold` counts from
//!   the next day and answers 小寒 + 9.
//!
//! The readings of the first three part only in the winters where the
//! question arises: at the Japanese meridian 15, 18, 18 and 15 of
//! 1901–2100 for the second-辰, nearest-辰, first-戌 and third-戌 rules.
//! The two readings of 丑節9日 are always a day apart.
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
//! The lunar rule reads the ordinary twelfth month next to 大寒, which is
//! not always the month holding it (in 1985 and 2053 it is not), and would
//! answer `None` where no month there is one; no winter of 1901–2100 is
//! such.
//!
//! [`docs/policy.md`]: https://github.com/kitsuyui/hyper-calendar/blob/main/docs/policy.md

use hc_calendar::Rd;
use hc_calendar::cycle::{branch, branch_day_on_or_after};
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
    /// sets out for the lunar rule, or outside the years the astronomy
    /// answers for.
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

/// Whether a count of days of a sign after a term counts the term's own
/// day when it bears the sign.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TermDay {
    /// "After": the first day of the sign is strictly after the term's.
    NotCounted,
    /// "From": the term's day is the first when it bears the sign.
    Counted,
}

/// The `nth` day of a branch after a term's day, counting from one.
fn nth_after(term: Rd, branch_index: u8, nth: i64, term_day: TermDay) -> Rd {
    let first = match term_day {
        TermDay::Counted => branch_day_on_or_after(term, branch_index),
        TermDay::NotCounted => branch_day_on_or_after(Rd(term.0 + 1), branch_index),
    };
    Rd(first.0 + (nth - 1) * BRANCH_CYCLE)
}

/// 小寒後の2度目の辰の日, 小寒's own day not counted.
fn second_dragon_after_minor_cold(year: i64, meridian: Meridian) -> Option<Rd> {
    let term = term_day(year, MINOR_COLD, meridian);
    Some(nth_after(term, branch::CHEN, 2, TermDay::NotCounted))
}

/// 小寒後の2度目の辰の日, 小寒's own day counted.
fn second_dragon_from_minor_cold(year: i64, meridian: Meridian) -> Option<Rd> {
    let term = term_day(year, MINOR_COLD, meridian);
    Some(nth_after(term, branch::CHEN, 2, TermDay::Counted))
}

/// Which of two 辰 days equally far from 大寒 a nearest-辰 rule takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tie {
    /// The one before 大寒.
    Earlier,
    /// The one after 大寒.
    Later,
}

/// 大寒に最も近い辰の日, a tie going to `tie`.
fn dragon_nearest_major_cold(year: i64, meridian: Meridian, tie: Tie) -> Rd {
    let term = term_day(year, MAJOR_COLD, meridian);
    let after = branch_day_on_or_after(term, branch::CHEN);
    let before = Rd(after.0 - BRANCH_CYCLE);
    match (after.0 - term.0).cmp(&(term.0 - before.0)) {
        core::cmp::Ordering::Less => after,
        core::cmp::Ordering::Greater => before,
        core::cmp::Ordering::Equal => match tie {
            Tie::Earlier => before,
            Tie::Later => after,
        },
    }
}

/// 大寒に最も近い辰の日, a tie going to the earlier.
fn dragon_nearest_major_cold_earlier(year: i64, meridian: Meridian) -> Option<Rd> {
    Some(dragon_nearest_major_cold(year, meridian, Tie::Earlier))
}

/// 大寒に最も近い辰の日, a tie going to the later.
fn dragon_nearest_major_cold_later(year: i64, meridian: Meridian) -> Option<Rd> {
    Some(dragon_nearest_major_cold(year, meridian, Tie::Later))
}

/// 大寒後の最初の戌の日, 大寒's own day not counted.
fn first_dog_after_major_cold(year: i64, meridian: Meridian) -> Option<Rd> {
    let term = term_day(year, MAJOR_COLD, meridian);
    Some(nth_after(term, branch::XU, 1, TermDay::NotCounted))
}

/// 大寒後の最初の戌の日, 大寒's own day counted.
fn first_dog_from_major_cold(year: i64, meridian: Meridian) -> Option<Rd> {
    let term = term_day(year, MAJOR_COLD, meridian);
    Some(nth_after(term, branch::XU, 1, TermDay::Counted))
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

/// 丑節9日, 小寒's own day the first of 丑月: 小寒 + 8.
fn ox_month_ninth_from_minor_cold(year: i64, meridian: Meridian) -> Option<Rd> {
    Some(Rd(term_day(year, MINOR_COLD, meridian).0 + 8))
}

/// 丑節9日, the day after 小寒 the first: 小寒 + 9.
fn ox_month_ninth_after_minor_cold(year: i64, meridian: Meridian) -> Option<Rd> {
    Some(Rd(term_day(year, MINOR_COLD, meridian).0 + 9))
}

/// 冬至後三戌, the December solstice before; 冬至's own day not counted.
fn third_dog_after_winter_solstice(year: i64, meridian: Meridian) -> Option<Rd> {
    let term = term_day(year - 1, SolarTerm::WINTER_SOLSTICE, meridian);
    Some(nth_after(term, branch::XU, 3, TermDay::NotCounted))
}

/// 冬至後三戌, the December solstice before; 冬至's own day counted.
fn third_dog_from_winter_solstice(year: i64, meridian: Meridian) -> Option<Rd> {
    let term = term_day(year - 1, SolarTerm::WINTER_SOLSTICE, meridian);
    Some(nth_after(term, branch::XU, 3, TermDay::Counted))
}

/// The sources of the second-辰 and first-戌 rules.
const AFTER_TERM_SOURCES: &str =
    "Japanese Wikipedia 「臘日」; こよみる 「臘日とは」; 日本文化研究ブログ 「臘日」";

/// The sources of the nearest-辰 rule.
const NEAREST_SOURCES: &str = "Japanese Wikipedia 「臘日」; こよみる 「臘日とは」, the 神社暦's; 西野神社 「六曜・選日・二十四節気」";

/// The source of the Han rule.
const HAN_SOURCE: &str =
    "『説文解字』 「腊，冬至后三戌腊祭百神」, through Chinese Wikipedia 「腊八节」";

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
        /// The second 辰 day after 小寒, 小寒's own day not counted.
        pub const SECOND_DRAGON_AFTER_MINOR_COLD = Self {
            id: "second-dragon-after-minor-cold",
            japanese_rule: "小寒後の2度目の辰の日",
            source: AFTER_TERM_SOURCES,
            day: second_dragon_after_minor_cold,
        };
        /// The second 辰 day after 小寒, 小寒's own day counted.
        pub const SECOND_DRAGON_FROM_MINOR_COLD = Self {
            id: "second-dragon-from-minor-cold",
            japanese_rule: "小寒後の2度目の辰の日",
            source: AFTER_TERM_SOURCES,
            day: second_dragon_from_minor_cold,
        };
        /// The 辰 day nearest 大寒, a tie going to the one before it.
        pub const DRAGON_NEAREST_MAJOR_COLD_EARLIER = Self {
            id: "dragon-nearest-major-cold-earlier",
            japanese_rule: "大寒に最も近い辰の日",
            source: NEAREST_SOURCES,
            day: dragon_nearest_major_cold_earlier,
        };
        /// The 辰 day nearest 大寒, a tie going to the one after it.
        pub const DRAGON_NEAREST_MAJOR_COLD_LATER = Self {
            id: "dragon-nearest-major-cold-later",
            japanese_rule: "大寒に最も近い辰の日",
            source: NEAREST_SOURCES,
            day: dragon_nearest_major_cold_later,
        };
        /// The first 戌 day after 大寒, 大寒's own day not counted.
        pub const FIRST_DOG_AFTER_MAJOR_COLD = Self {
            id: "first-dog-after-major-cold",
            japanese_rule: "大寒後の最初の戌の日",
            source: AFTER_TERM_SOURCES,
            day: first_dog_after_major_cold,
        };
        /// The first 戌 day after 大寒, 大寒's own day counted.
        pub const FIRST_DOG_FROM_MAJOR_COLD = Self {
            id: "first-dog-from-major-cold",
            japanese_rule: "大寒後の最初の戌の日",
            source: AFTER_TERM_SOURCES,
            day: first_dog_from_major_cold,
        };
        /// The ninth of the twelfth lunar month.
        pub const LUNAR_TWELFTH_NINTH = Self {
            id: "lunar-twelfth-ninth",
            japanese_rule: "旧暦12月9日",
            source: "こよみる 「臘日とは」; 日本文化研究ブログ 「臘日」",
            day: lunar_twelfth_ninth,
        };
        /// The ninth day of 丑月, 小寒's own day the first.
        pub const OX_MONTH_NINTH_FROM_MINOR_COLD = Self {
            id: "ox-month-ninth-from-minor-cold",
            japanese_rule: "丑節9日",
            source: "Japanese Wikipedia 「臘日」",
            day: ox_month_ninth_from_minor_cold,
        };
        /// The ninth day of 丑月, the day after 小寒 the first.
        pub const OX_MONTH_NINTH_AFTER_MINOR_COLD = Self {
            id: "ox-month-ninth-after-minor-cold",
            japanese_rule: "丑節9日",
            source: "Japanese Wikipedia 「臘日」",
            day: ox_month_ninth_after_minor_cold,
        };
        /// The third 戌 day after 冬至, 冬至's own day not counted.
        pub const THIRD_DOG_AFTER_WINTER_SOLSTICE = Self {
            id: "third-dog-after-winter-solstice",
            japanese_rule: "冬至後三戌",
            source: HAN_SOURCE,
            day: third_dog_after_winter_solstice,
        };
        /// The third 戌 day after 冬至, 冬至's own day counted.
        pub const THIRD_DOG_FROM_WINTER_SOLSTICE = Self {
            id: "third-dog-from-winter-solstice",
            japanese_rule: "冬至後三戌",
            source: HAN_SOURCE,
            day: third_dog_from_winter_solstice,
        };
    }
}

#[cfg(test)]
mod tests {
    use hc_calendar::cycle::sexagenary_day;

    use super::*;

    /// Whether a day bears a branch.
    const fn is_branch(day: Rd, branch_index: u8) -> bool {
        sexagenary_day(day).branch_index() == branch_index
    }

    const JAPAN: Meridian = Meridian::JAPAN;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    /// こよみる's candidates for 2024 to 2027 (`koyomil-rounichi`, read
    /// 2026-09-29), one day for each of its four reckonings: in 2025 the
    /// first two fall together on 23 January, which it lists once. No
    /// term day of those winters bears its sign and no 大寒 lies midway
    /// between two 辰 days, so both readings of each rule give the day.
    #[test]
    fn the_published_candidates_of_2024_to_2027_match() {
        let rules = [
            [
                RounichiRule::SECOND_DRAGON_AFTER_MINOR_COLD,
                RounichiRule::SECOND_DRAGON_FROM_MINOR_COLD,
            ],
            [
                RounichiRule::DRAGON_NEAREST_MAJOR_COLD_EARLIER,
                RounichiRule::DRAGON_NEAREST_MAJOR_COLD_LATER,
            ],
            [
                RounichiRule::FIRST_DOG_AFTER_MAJOR_COLD,
                RounichiRule::FIRST_DOG_FROM_MAJOR_COLD,
            ],
            [
                RounichiRule::LUNAR_TWELFTH_NINTH,
                RounichiRule::LUNAR_TWELFTH_NINTH,
            ],
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
                for (readings, day) in rules.iter().zip(days) {
                    for rule in readings {
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
            }
        });
    }

    /// The day of a rule in a winter of 1901–2100 at the Japanese
    /// meridian; every rule but the lunar one answers every winter.
    fn day(rule: RounichiRule, year: i64) -> Rd {
        rule.day_of_year(year, JAPAN).expect("a day")
    }

    /// Each reading's day bears its sign and lies in the right window of
    /// its term; the two readings of an "after" rule differ exactly in the
    /// winters whose term day bears the sign, 15, 18 and 15 of 1901–2100
    /// for the second-辰, first-戌 and third-戌 rules, and the counted
    /// reading then answers the term day itself.
    #[test]
    fn the_two_readings_of_after_differ_only_on_a_term_day_of_the_sign() {
        let pairs = [
            (
                RounichiRule::SECOND_DRAGON_AFTER_MINOR_COLD,
                RounichiRule::SECOND_DRAGON_FROM_MINOR_COLD,
                MINOR_COLD,
                0,
                branch::CHEN,
                2,
                15,
            ),
            (
                RounichiRule::FIRST_DOG_AFTER_MAJOR_COLD,
                RounichiRule::FIRST_DOG_FROM_MAJOR_COLD,
                MAJOR_COLD,
                0,
                branch::XU,
                1,
                18,
            ),
            (
                RounichiRule::THIRD_DOG_AFTER_WINTER_SOLSTICE,
                RounichiRule::THIRD_DOG_FROM_WINTER_SOLSTICE,
                SolarTerm::WINTER_SOLSTICE,
                1,
                branch::XU,
                3,
                15,
            ),
        ];
        hc_core::memo::scope(|| {
            for (after, from, term, back, sign, nth, differing) in pairs {
                let mut count = 0;
                for year in 1901..=2100 {
                    let term = term_day(year - back, term, JAPAN);
                    let (after_day, from_day) = (day(after, year), day(from, year));
                    for found in [after_day, from_day] {
                        assert!(is_branch(found, sign), "{} {year}", after.id);
                    }
                    let window = (nth - 1) * BRANCH_CYCLE;
                    assert!(
                        (window + 1..=window + 12).contains(&(after_day.0 - term.0)),
                        "{} {year}",
                        after.id
                    );
                    if is_branch(term, sign) {
                        count += 1;
                        assert_eq!(from_day, Rd(term.0 + window), "{} {year}", from.id);
                        assert_eq!(after_day, Rd(from_day.0 + BRANCH_CYCLE));
                    } else {
                        assert_eq!(after_day, from_day, "{} {year}", from.id);
                    }
                }
                assert_eq!(count, differing, "{}", after.id);
            }
        });
    }

    /// The two readings of the nearest 辰 are the nearest 辰 day, within
    /// five days of 大寒, in every winter of 1901–2100 but the 18 in which
    /// 大寒 lies six days from a 辰 day each side; there the earlier
    /// reading answers the one before and the later the one after.
    #[test]
    fn the_nearest_dragon_readings_differ_only_on_a_tie() {
        hc_core::memo::scope(|| {
            let mut ties = 0;
            for year in 1901..=2100 {
                let major = term_day(year, MAJOR_COLD, JAPAN);
                let earlier = day(RounichiRule::DRAGON_NEAREST_MAJOR_COLD_EARLIER, year);
                let later = day(RounichiRule::DRAGON_NEAREST_MAJOR_COLD_LATER, year);
                assert!(is_branch(earlier, branch::CHEN) && is_branch(later, branch::CHEN));
                if is_branch(Rd(major.0 + 6), branch::CHEN) {
                    ties += 1;
                    assert_eq!((earlier.0 - major.0, later.0 - major.0), (-6, 6), "{year}");
                } else {
                    assert_eq!(earlier, later, "{year}");
                    assert!((earlier.0 - major.0).abs() < 6, "{year}");
                }
            }
            assert_eq!(ties, 18);
        });
    }

    /// 「丑節9日」 in the winter of 2026: 小寒 fell on 5 January, as the
    /// worked example of `docs/systems/japanese-almanac-notes.md` has it,
    /// so counting its day the ninth is 13 January, and counting from the
    /// next day 14 January. No source read gives a dated 丑節9日. Both
    /// readings lie in 丑月, before 立春, in every winter of 1901–2100.
    #[test]
    fn the_ninth_of_the_ox_month_is_counted_both_ways() {
        let from = RounichiRule::OX_MONTH_NINTH_FROM_MINOR_COLD;
        let after = RounichiRule::OX_MONTH_NINTH_AFTER_MINOR_COLD;
        assert_eq!(from.day_of_year(2026, JAPAN), Some(ymd(2026, 1, 13)));
        assert_eq!(after.day_of_year(2026, JAPAN), Some(ymd(2026, 1, 14)));
        hc_core::memo::scope(|| {
            for year in 1901..=2100 {
                let minor = term_day(year, MINOR_COLD, JAPAN);
                assert_eq!(day(from, year), Rd(minor.0 + 8), "{year}");
                assert_eq!(day(after, year), Rd(minor.0 + 9), "{year}");
                assert!(
                    day(after, year).0 < term_day(year, SolarTerm::BEGINNING_OF_SPRING, JAPAN).0
                );
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
