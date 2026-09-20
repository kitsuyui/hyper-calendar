//! One rule value, one evaluator.
//!
//! There are roughly forty annotations in [`crate::lower_register`] and
//! [`crate::selected_days`], and a naive implementation would be forty
//! functions. It would also be forty places to get the 節月 boundary wrong.
//!
//! In fact they have only a handful of *shapes*. 大明日 is a list of
//! sexagenary days. 受死日 is a branch per 節月. 月徳日 is a stem per 節月.
//! 往亡日 is a day count from a sectional term. 不成就日 is a set of
//! lunisolar days per lunisolar month. 八専 is a run of the sexagenary
//! cycle. That is very nearly all of them.
//!
//! So an annotation here is a name plus an [`AlmanacRule`] value, and
//! [`rule_applies`] is the only function that knows how to evaluate one.
//! This follows [`hc_seasons::ZassetsuRule`], which does the same thing for
//! the 雑節.
//!
//! # Undetermined rules are a value, not a guess
//!
//! Several 暦注 have no rule this crate could establish from a citable
//! source — 凶会日's full table is the clearest case. Those carry
//! [`AlmanacRule::Undetermined`], and [`rule_applies`] answers `None` for
//! them, never `false`. A caller can tell "the almanac says no" from "this
//! crate does not know", which is the whole reason the return type is an
//! `Option`.

use crate::context::DayContext;
use crate::mansions::{Mansion, mansion_of};

/// How many days the sexagenary cycle runs for.
const CYCLE_LENGTH: u8 = 60;

/// The shape of a 暦注 or 選日 rule.
///
/// Tables indexed "by 節月" are twelve entries long, index 0 being 節月 正月
/// — 寅月, which opens at 立春. Tables indexed "by lunisolar month" are also
/// twelve, index 0 being the first month, and a leap month uses the row of
/// the month it follows.
///
/// Stems are zero-based, 0 for 甲 through 9 for 癸. Branches are zero-based,
/// 0 for 子 through 11 for 亥. Sexagenary positions are zero-based, 0 for
/// 甲子 through 59 for 癸亥.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AlmanacRule {
    /// The day's sexagenary position is one of a listed set.
    ///
    /// 大明日, 天恩日 and 神吉日 are lists of this kind, and so are the
    /// single-day 選日 such as 庚申 and 己巳.
    SexagenaryIn(&'static [u8]),
    /// The day falls in a run of consecutive sexagenary positions, wrapping
    /// at 癸亥.
    ///
    /// 八専 (壬子 for twelve days), 十方暮 (甲申 for ten) and 天一天上 (癸巳
    /// for sixteen) are all runs.
    SexagenaryRun {
        /// The sexagenary position the run opens on.
        first: u8,
        /// How many consecutive days it covers.
        length: u8,
    },
    /// The day's earthly branch is one of a listed set, in any month.
    ///
    /// 寅の日, 巳の日 and 重日 are of this kind.
    BranchIn(&'static [u8]),
    /// The day's earthly branch is in the set this 節月 lists.
    ///
    /// The commonest shape in the whole almanac: 受死日, 十死日, 帰忌日,
    /// 血忌日, 天火日, 地火日, 母倉日, 一粒万倍日 and 三隣亡 all look like
    /// this.
    BranchBySolarMonth(&'static [&'static [u8]; 12]),
    /// The day's heavenly stem is in the set this 節月 lists.
    ///
    /// 月徳日 and 復日 are of this kind.
    StemBySolarMonth(&'static [&'static [u8]; 12]),
    /// The day's full sexagenary position is in the set this 節月 lists.
    ///
    /// 天赦日 and 五墓日 are of this kind.
    SexagenaryBySolarMonth(&'static [&'static [u8]; 12]),
    /// The day's full sexagenary position is in the set this *lunisolar*
    /// month lists.
    SexagenaryByLunarMonth(&'static [&'static [u8]; 12]),
    /// The day is a fixed number of days past the sectional term that opened
    /// its 節月, counting that term's own day as zero.
    ///
    /// Only 往亡日 has this shape.
    DaysIntoSolarMonth(&'static [u8; 12]),
    /// The day of the lunisolar month is in the set this lunisolar month
    /// lists.
    ///
    /// Only 不成就日 has this shape. A leap month uses the row of the month
    /// it follows, and a 29-day month simply never reaches a rule day of 30.
    LunarDayByLunarMonth(&'static [&'static [u8]; 12]),
    /// The day's 二十八宿 is a given mansion.
    ///
    /// Only 鬼宿日 has this shape.
    Mansion(Mansion),
    /// No rule could be established from a citable source.
    ///
    /// [`rule_applies`] answers `None`, never `false`. See the crate README
    /// for the list of gaps and why each is one.
    Undetermined,
}

/// Whether a rule holds on the day a context describes.
///
/// Answers `None` only for [`AlmanacRule::Undetermined`] — "this crate does
/// not know", as distinct from `Some(false)`, "the almanac says no".
#[must_use]
pub fn rule_applies(rule: AlmanacRule, context: &DayContext) -> Option<bool> {
    let sexagenary = context.sexagenary().index();
    let solar_month = context.solar_month().table_index();
    let lunar_month = usize::from(context.lunisolar().month - 1);
    Some(match rule {
        AlmanacRule::SexagenaryIn(days) => days.contains(&sexagenary),
        AlmanacRule::SexagenaryRun { first, length } => {
            (sexagenary + CYCLE_LENGTH - first) % CYCLE_LENGTH < length
        }
        AlmanacRule::BranchIn(branches) => branches.contains(&context.branch_index()),
        AlmanacRule::BranchBySolarMonth(table) => table[solar_month].contains(&context.branch_index()),
        AlmanacRule::StemBySolarMonth(table) => table[solar_month].contains(&context.stem_index()),
        AlmanacRule::SexagenaryBySolarMonth(table) => table[solar_month].contains(&sexagenary),
        AlmanacRule::SexagenaryByLunarMonth(table) => table[lunar_month].contains(&sexagenary),
        AlmanacRule::DaysIntoSolarMonth(table) => {
            i64::from(table[solar_month]) == context.days_into_solar_month()
        }
        AlmanacRule::LunarDayByLunarMonth(table) => {
            table[lunar_month].contains(&context.lunisolar().day)
        }
        AlmanacRule::Mansion(mansion) => mansion_of(context.day()) == mansion,
        AlmanacRule::Undetermined => return None,
    })
}

#[cfg(test)]
mod tests {
    use hc_calendar::Rd;
    use hc_seasons::Meridian;

    use super::*;

    /// RD 738886 is 2024-01-01: a 甲子 day, branch 子, stem 甲, inside 節月
    /// 十一月 (子月).
    const NEW_YEAR_2024: Rd = Rd(738_886);

    fn context() -> DayContext {
        DayContext::new(NEW_YEAR_2024, Meridian::JAPAN)
    }

    #[test]
    fn a_sexagenary_list_matches_only_the_days_it_lists() {
        let ctx = context();
        assert_eq!(rule_applies(AlmanacRule::SexagenaryIn(&[0]), &ctx), Some(true));
        assert_eq!(rule_applies(AlmanacRule::SexagenaryIn(&[1, 2]), &ctx), Some(false));
        assert_eq!(rule_applies(AlmanacRule::SexagenaryIn(&[]), &ctx), Some(false));
    }

    /// 八専 opens on 壬子, position 48, and runs twelve days — so it wraps
    /// past 癸亥 at 59 and back to 甲子 at 0. A run must handle that wrap,
    /// which is the only reason the variant exists.
    #[test]
    fn a_sexagenary_run_wraps_past_the_end_of_the_cycle() {
        let wrapping = AlmanacRule::SexagenaryRun {
            first: 55,
            length: 10,
        };
        for offset in 0..60i64 {
            let day = Rd(NEW_YEAR_2024.0 + offset);
            let ctx = DayContext::new(day, Meridian::JAPAN);
            let position = ctx.sexagenary().index();
            let expected = (55..60).contains(&position) || (0..5).contains(&position);
            assert_eq!(rule_applies(wrapping, &ctx), Some(expected), "at {position}");
        }
    }

    #[test]
    fn a_branch_list_ignores_the_stem() {
        let ctx = context();
        assert_eq!(rule_applies(AlmanacRule::BranchIn(&[0]), &ctx), Some(true));
        assert_eq!(rule_applies(AlmanacRule::BranchIn(&[2, 5]), &ctx), Some(false));
    }

    /// Only the eleventh row holds the rat branch, so only a day in 節月
    /// 十一月 can match this table.
    static ONLY_THE_ELEVENTH_MONTH: [&[u8]; 12] =
        [&[], &[], &[], &[], &[], &[], &[], &[], &[], &[], &[0], &[]];

    /// The complement: every row but the eleventh.
    static EVERY_MONTH_BUT_THE_ELEVENTH: [&[u8]; 12] = [
        &[0],
        &[0],
        &[0],
        &[0],
        &[0],
        &[0],
        &[0],
        &[0],
        &[0],
        &[0],
        &[],
        &[0],
    ];

    /// 2024-01-01 is in 節月 十一月, so only the eleventh row of a by-月 table
    /// can match it.
    #[test]
    fn a_table_by_solar_month_reads_the_row_the_day_falls_in() {
        let ctx = context();
        assert_eq!(ctx.solar_month().number(), 11);
        assert_eq!(
            rule_applies(
                AlmanacRule::BranchBySolarMonth(&ONLY_THE_ELEVENTH_MONTH),
                &ctx
            ),
            Some(true)
        );
        assert_eq!(
            rule_applies(
                AlmanacRule::BranchBySolarMonth(&EVERY_MONTH_BUT_THE_ELEVENTH),
                &ctx
            ),
            Some(false)
        );
    }

    /// Twenty-five days into the eleventh 節月 and nowhere else.
    static TWENTY_FIFTH_DAY_OF_THE_ELEVENTH_MONTH: [u8; 12] =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0];

    /// 大雪 2023 fell on 7 December, RD 738861, so 2024-01-01 is 25 days into
    /// its 節月.
    #[test]
    fn a_day_count_from_the_sectional_term_counts_the_terms_own_day_as_zero() {
        let ctx = context();
        assert_eq!(ctx.days_into_solar_month(), 25);
        assert_eq!(
            rule_applies(
                AlmanacRule::DaysIntoSolarMonth(&TWENTY_FIFTH_DAY_OF_THE_ELEVENTH_MONTH),
                &ctx
            ),
            Some(true)
        );
        let earlier = DayContext::new(Rd(NEW_YEAR_2024.0 - 1), Meridian::JAPAN);
        assert_eq!(
            rule_applies(
                AlmanacRule::DaysIntoSolarMonth(&TWENTY_FIFTH_DAY_OF_THE_ELEVENTH_MONTH),
                &earlier
            ),
            Some(false)
        );
    }

    #[test]
    fn an_undetermined_rule_answers_neither_yes_nor_no() {
        assert_eq!(rule_applies(AlmanacRule::Undetermined, &context()), None);
    }

    #[test]
    fn a_mansion_rule_holds_on_exactly_one_day_in_twenty_eight() {
        let days = (0..28)
            .filter(|offset| {
                let ctx = DayContext::new(Rd(NEW_YEAR_2024.0 + offset), Meridian::JAPAN);
                rule_applies(AlmanacRule::Mansion(Mansion::GHOST), &ctx) == Some(true)
            })
            .count();
        assert_eq!(days, 1);
    }
}
