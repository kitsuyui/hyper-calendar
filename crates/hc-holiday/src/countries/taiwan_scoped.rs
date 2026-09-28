//! Taiwan — the days its statute leaves to the authority of one service,
//! and the days it lets indigenous people choose.
//!
//! The regime is written up in `docs/systems/taiwan-holidays.md` in the
//! repository, in its section on the scoped days. The table itself,
//! [`TAIWAN`](super::TAIWAN), is in `asia.rs`; this file holds the rules of
//! it that are given to one group alone, and the table takes them in
//! through [`TW_ALL_RULES`].
//!
//! # Article 6: the services' days
//!
//! 紀念日及節日實施條例 Article 5 names the festivals and their dates, and
//! Article 6 paragraph 1 says which are days off: for everyone, the Lunar
//! New Year days, Children's Day, 清明, Labour Day, the Dragon Boat
//! Festival, Teachers' Day and the Mid-Autumn Festival; and, in its items
//! 4 to 6, 消防節及警察節：依主管機關規定放假, 軍人節：依國防部規定放假
//! and 海巡節：依海洋委員會規定放假. Item 3 lets each indigenous person
//! choose three days for the ceremonies of their people
//! (由原住民依其族別歲時祭儀擇定三日放假).
//!
//! The services' days are on fixed dates — 消防節 19 January, 警察節
//! 15 June, 軍人節 3 September, 海巡節 8 November — but whether and for
//! whom each is a day off is the rule of the service's authority, which
//! was not read, and the indigenous days have no date the statute gives.
//! So each is a rule of [`TAIWAN`](super::TAIWAN) given to its group
//! alone, and a gap in every year: a caller who asks for the police is
//! told that Police Day's effect is not known, not that the police have no
//! day. The statute came into force on 28 May 2025; the
//! 紀念日及節日實施辦法 it replaced was not read
//! for them, so the years before are gaps too.

use crate::group::{COAST_GUARD, FIREFIGHTERS, INDIGENOUS_PEOPLES, MILITARY, POLICE};
use crate::rule::{HolidayRule, Rule, joined};

use super::asia::TW_RULES;

/// A day Article 6 leaves to the authority of one group's service, or to
/// the group itself: a gap in every year, the years before the 條例 among
/// them, whose 辦法 was not read for these days.
const fn tw_group_day(
    name: &'static str,
    local_name: &'static str,
    groups: &'static [crate::group::Group],
    item: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::UNREAD)
        .for_groups(groups)
        .cited(item)
}

/// The days of Article 6 paragraph 1, items 3 to 6, in their order.
pub(super) static TW_SCOPED_RULES: &[HolidayRule] = &[
    tw_group_day(
        "Indigenous ceremonies",
        "原住民族歲時祭儀",
        &[INDIGENOUS_PEOPLES],
        "紀念日及節日實施條例, 第六條第一項第三款: 由原住民依其族別歲時祭儀擇定三日放假",
    ),
    tw_group_day(
        "Fire Fighters' Day",
        "消防節",
        &[FIREFIGHTERS],
        "紀念日及節日實施條例, 第六條第一項第四款: 消防節及警察節：依主管機關規定放假",
    ),
    tw_group_day(
        "Police Day",
        "警察節",
        &[POLICE],
        "紀念日及節日實施條例, 第六條第一項第四款: 消防節及警察節：依主管機關規定放假",
    ),
    tw_group_day(
        "Armed Forces Day",
        "軍人節",
        &[MILITARY],
        "紀念日及節日實施條例, 第六條第一項第五款: 軍人節：依國防部規定放假",
    ),
    tw_group_day(
        "Coast Guard Day",
        "海巡節",
        &[COAST_GUARD],
        "紀念日及節日實施條例, 第六條第一項第六款: 海巡節：依海洋委員會規定放假",
    ),
];

/// How many rules [`TAIWAN`](super::TAIWAN) has in all.
const TW_ALL_LEN: usize = TW_RULES.len() + TW_SCOPED_RULES.len();

/// Every rule of [`TAIWAN`](super::TAIWAN): the ones of `asia.rs`, then the
/// scoped ones here.
pub(super) static TW_ALL_RULES: [HolidayRule; TW_ALL_LEN] = joined(&[TW_RULES, TW_SCOPED_RULES]);
