//! Japan — 国民の祝日に関する法律, complete from 1948.
//!
//! This is the crate's worked example, and the claim it makes is a strong
//! one: every Japanese public holiday from the enactment of the 祝日法
//! (昭和23年法律第178号, in force 20 July 1948) to today, with every
//! amendment, expressed entirely as rule values. There is no function in
//! this file. The 振替休日, the 国民の休日, ハッピーマンデー, the imperial
//! one-offs and the two Olympic years are all data.
//!
//! # The amendments, in order
//!
//! | In force | Law | Change |
//! | --- | --- | --- |
//! | 1948-07-20 | 昭和23年法律第178号 | the nine original holidays |
//! | 1966-06-25 | 昭和41年法律第86号 | 敬老の日 9/15, 体育の日 10/10, and 建国記念の日, whose date was left to a 政令 that came on 1966-12-09 — so it was first kept in 1967 |
//! | 1973-04-12 | 昭和48年法律第10号 | 振替休日: a holiday falling on a Sunday is kept the following day |
//! | 1985-12-27 | 昭和60年法律第103号 | 国民の休日: a working day trapped between two 祝日 becomes a holiday, from 1986 |
//! | 1989-02-17 | 平成元年法律第5号 | 昭和天皇's death: 天皇誕生日 moves 4/29 → 12/23, and 4/29 becomes みどりの日 |
//! | 1996-01-01 | 平成7年法律第22号 | 海の日, 20 July |
//! | 2000-01-01 | 平成10年法律第141号 | ハッピーマンデー I: 成人の日 → 2nd Monday of January, 体育の日 → 2nd Monday of October |
//! | 2003-01-01 | 平成13年法律第59号 | ハッピーマンデー II: 海の日 → 3rd Monday of July, 敬老の日 → 3rd Monday of September |
//! | 2007-01-01 | 平成17年法律第43号 | 4/29 becomes 昭和の日, みどりの日 moves to 5/4, and 振替休日 becomes "the nearest following day that is not a 祝日" |
//! | 2016-01-01 | 平成26年法律第43号 | 山の日, 11 August |
//! | 2019/2020 | 平成30年法律第99号 | the accession: 2019-05-01 and 2019-10-22 as one-off 祝日, and 天皇誕生日 moves to 2/23 from 2020 |
//! | 2020 | 平成30年法律第76号 | Tokyo 2020: 海の日 → 7/23, 体育の日 renamed スポーツの日 and moved to 7/24, 山の日 → 8/10 |
//! | 2021 | 令和2年法律第68号 | the postponed games: 海の日 → 7/22, スポーツの日 → 7/23, 山の日 → 8/8 |
//!
//! # The equinoxes are computed, not tabulated
//!
//! 春分の日 and 秋分の日 are defined by the statute as 「春分日」 and
//! 「秋分日」 — the day of the equinox itself. The National Astronomical
//! Observatory of Japan computes the instant in JST and the Cabinet Office
//! publishes the resulting date in the 官報 a year ahead; nobody legislates
//! a table. So this file does not carry one either: it carries
//! [`Rule::SolarTerm`] at [`Meridian::JAPAN`] and lets `hc-seasons` answer.
//!
//! That is right in principle and, measured, right in practice:
//! `hc-seasons`'s own test suite compares its equinox days against the 240
//! the Observatory has published for 1980–2099 and finds no disagreement.
//! The underlying solar longitude is VSOP87, good to about 1″, so an equinox
//! lands within the minute the almanacs round to; only an equinox within
//! about a minute of JST midnight could still be given the wrong day, and
//! the closest case in the modern record, the autumn equinox of 2012 at
//! 23:49 JST, is eleven minutes clear.
//!
//! # 1948 is a half year
//!
//! The law came into force on 20 July 1948, so the 1948 holidays that fall
//! before that date were not holidays. 秋分の日, 文化の日 and 勤労感謝の日
//! carry `valid_from = 1948`; everything else in the original nine carries
//! 1949.

use hc_calendar::Weekday;
use hc_seasons::{Meridian, SolarTerm};

use crate::rule::{
    BridgePolicy, HolidayRule, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, SubstituteDirection,
    SubstitutionPolicy,
};

/// Every Japanese public holiday rule, 1948 to today.
static RULES: &[HolidayRule] = &[
    // ── The original nine, 昭和23年法律第178号 ──────────────────────────
    HolidayRule::public(
        "New Year's Day",
        "元日",
        Rule::FixedGregorian { month: 1, day: 1 },
    )
    .years(Some(1949), None),
    HolidayRule::public(
        "Coming of Age Day",
        "成人の日",
        Rule::FixedGregorian { month: 1, day: 15 },
    )
    .years(Some(1949), Some(1999)),
    HolidayRule::public(
        "Coming of Age Day",
        "成人の日",
        Rule::NthWeekday {
            month: 1,
            n: 2,
            weekday: Weekday::Monday,
        },
    )
    .years(Some(2000), None),
    HolidayRule::public(
        "Vernal Equinox Day",
        "春分の日",
        Rule::SolarTerm {
            term: SolarTerm::SPRING_EQUINOX,
            meridian: Meridian::JAPAN,
        },
    )
    .years(Some(1949), None),
    HolidayRule::public(
        "Constitution Memorial Day",
        "憲法記念日",
        Rule::FixedGregorian { month: 5, day: 3 },
    )
    .years(Some(1949), None),
    HolidayRule::public(
        "Children's Day",
        "こどもの日",
        Rule::FixedGregorian { month: 5, day: 5 },
    )
    .years(Some(1949), None),
    HolidayRule::public(
        "Autumnal Equinox Day",
        "秋分の日",
        Rule::SolarTerm {
            term: SolarTerm::AUTUMN_EQUINOX,
            meridian: Meridian::JAPAN,
        },
    )
    .years(Some(1948), None),
    HolidayRule::public(
        "Culture Day",
        "文化の日",
        Rule::FixedGregorian { month: 11, day: 3 },
    )
    .years(Some(1948), None),
    HolidayRule::public(
        "Labour Thanksgiving Day",
        "勤労感謝の日",
        Rule::FixedGregorian { month: 11, day: 23 },
    )
    .years(Some(1948), None),
    // ── 天皇誕生日, three reigns and one gap ─────────────────────────────
    // 昭和: 29 April, until the Emperor's death on 1989-01-07.
    HolidayRule::public(
        "The Emperor's Birthday",
        "天皇誕生日",
        Rule::FixedGregorian { month: 4, day: 29 },
    )
    .years(Some(1949), Some(1988)),
    // 平成: 23 December. 2019 has no 天皇誕生日 at all — the abdication was
    // on 2019-04-30 and the new Emperor's birthday first fell in 2020.
    HolidayRule::public(
        "The Emperor's Birthday",
        "天皇誕生日",
        Rule::FixedGregorian { month: 12, day: 23 },
    )
    .years(Some(1989), Some(2018)),
    // 令和: 23 February, 平成30年法律第99号.
    HolidayRule::public(
        "The Emperor's Birthday",
        "天皇誕生日",
        Rule::FixedGregorian { month: 2, day: 23 },
    )
    .years(Some(2020), None),
    // ── 昭和41年法律第86号 ───────────────────────────────────────────────
    // 建国記念の日's date was fixed by 政令 on 1966-12-09, so 1967 is the
    // first year it was kept.
    // 振替休日 came into force on 1973-04-12, after that year's 11 February,
    // which fell on a Sunday: 1973-02-12 was an ordinary working Monday.
    // 建国記念の日 is the only holiday the gap can affect, so it carries the
    // later substitution year.
    HolidayRule::public(
        "National Foundation Day",
        "建国記念の日",
        Rule::FixedGregorian { month: 2, day: 11 },
    )
    .years(Some(1967), None)
    .substituted_from(1974),
    HolidayRule::public(
        "Respect for the Aged Day",
        "敬老の日",
        Rule::FixedGregorian { month: 9, day: 15 },
    )
    .years(Some(1966), Some(2002)),
    HolidayRule::public(
        "Respect for the Aged Day",
        "敬老の日",
        Rule::NthWeekday {
            month: 9,
            n: 3,
            weekday: Weekday::Monday,
        },
    )
    .years(Some(2003), None),
    HolidayRule::public(
        "Health and Sports Day",
        "体育の日",
        Rule::FixedGregorian { month: 10, day: 10 },
    )
    .years(Some(1966), Some(1999)),
    HolidayRule::public(
        "Health and Sports Day",
        "体育の日",
        Rule::NthWeekday {
            month: 10,
            n: 2,
            weekday: Weekday::Monday,
        },
    )
    .years(Some(2000), Some(2019)),
    // Renamed スポーツの日 in 2020, and displaced by the Games in 2020–21.
    HolidayRule::public(
        "Sports Day",
        "スポーツの日",
        Rule::FixedGregorian { month: 7, day: 24 },
    )
    .years(Some(2020), Some(2020)),
    HolidayRule::public(
        "Sports Day",
        "スポーツの日",
        Rule::FixedGregorian { month: 7, day: 23 },
    )
    .years(Some(2021), Some(2021)),
    HolidayRule::public(
        "Sports Day",
        "スポーツの日",
        Rule::NthWeekday {
            month: 10,
            n: 2,
            weekday: Weekday::Monday,
        },
    )
    .years(Some(2022), None),
    // ── みどりの日 and 昭和の日 ──────────────────────────────────────────
    HolidayRule::public(
        "Greenery Day",
        "みどりの日",
        Rule::FixedGregorian { month: 4, day: 29 },
    )
    .years(Some(1989), Some(2006)),
    HolidayRule::public(
        "Greenery Day",
        "みどりの日",
        Rule::FixedGregorian { month: 5, day: 4 },
    )
    .years(Some(2007), None),
    HolidayRule::public(
        "Shōwa Day",
        "昭和の日",
        Rule::FixedGregorian { month: 4, day: 29 },
    )
    .years(Some(2007), None),
    // ── 海の日 ───────────────────────────────────────────────────────────
    HolidayRule::public(
        "Marine Day",
        "海の日",
        Rule::FixedGregorian { month: 7, day: 20 },
    )
    .years(Some(1996), Some(2002)),
    HolidayRule::public(
        "Marine Day",
        "海の日",
        Rule::NthWeekday {
            month: 7,
            n: 3,
            weekday: Weekday::Monday,
        },
    )
    .years(Some(2003), Some(2019)),
    HolidayRule::public(
        "Marine Day",
        "海の日",
        Rule::FixedGregorian { month: 7, day: 23 },
    )
    .years(Some(2020), Some(2020)),
    HolidayRule::public(
        "Marine Day",
        "海の日",
        Rule::FixedGregorian { month: 7, day: 22 },
    )
    .years(Some(2021), Some(2021)),
    HolidayRule::public(
        "Marine Day",
        "海の日",
        Rule::NthWeekday {
            month: 7,
            n: 3,
            weekday: Weekday::Monday,
        },
    )
    .years(Some(2022), None),
    // ── 山の日 ───────────────────────────────────────────────────────────
    HolidayRule::public(
        "Mountain Day",
        "山の日",
        Rule::FixedGregorian { month: 8, day: 11 },
    )
    .years(Some(2016), Some(2019)),
    HolidayRule::public(
        "Mountain Day",
        "山の日",
        Rule::FixedGregorian { month: 8, day: 10 },
    )
    .years(Some(2020), Some(2020)),
    HolidayRule::public(
        "Mountain Day",
        "山の日",
        Rule::FixedGregorian { month: 8, day: 8 },
    )
    .years(Some(2021), Some(2021)),
    HolidayRule::public(
        "Mountain Day",
        "山の日",
        Rule::FixedGregorian { month: 8, day: 11 },
    )
    .years(Some(2022), None),
    // ── The imperial one-offs, each its own special law ─────────────────
    // 昭和34年法律第16号.
    HolidayRule::public(
        "Wedding of Crown Prince Akihito",
        "皇太子明仁親王の結婚の儀",
        Rule::FixedGregorian { month: 4, day: 10 },
    )
    .years(Some(1959), Some(1959)),
    // 平成元年法律第4号.
    HolidayRule::public(
        "State Funeral of Emperor Shōwa",
        "昭和天皇の大喪の礼",
        Rule::FixedGregorian { month: 2, day: 24 },
    )
    .years(Some(1989), Some(1989)),
    // 平成2年法律第24号.
    HolidayRule::public(
        "Enthronement Ceremony of Emperor Akihito",
        "即位礼正殿の儀",
        Rule::FixedGregorian { month: 11, day: 12 },
    )
    .years(Some(1990), Some(1990)),
    // 平成5年法律第32号.
    HolidayRule::public(
        "Wedding of Crown Prince Naruhito",
        "皇太子徳仁親王の結婚の儀",
        Rule::FixedGregorian { month: 6, day: 9 },
    )
    .years(Some(1993), Some(1993)),
    // 平成30年法律第99号: both declared 国民の祝日, which is why 30 April
    // and 2 May 2019 became 国民の休日 by the bridge rule.
    HolidayRule::public(
        "Accession Day",
        "天皇の即位の日",
        Rule::FixedGregorian { month: 5, day: 1 },
    )
    .years(Some(2019), Some(2019)),
    HolidayRule::public(
        "Enthronement Ceremony of Emperor Naruhito",
        "即位礼正殿の儀の行われる日",
        Rule::FixedGregorian { month: 10, day: 22 },
    )
    .years(Some(2019), Some(2019)),
];

/// 振替休日, in its two successive forms.
///
/// 1973–2006 the statute said simply 「その翌日を休日とする」 — the following
/// day, full stop, so a Sunday holiday whose Monday was itself a holiday
/// produced nothing. From 2007 it reads 「その日後においてその日に最も近い
/// 『国民の祝日』でない日」, which is the same rule with `skip_occupied`.
static SUBSTITUTION: &[SubstitutionPolicy] = &[
    SubstitutionPolicy {
        trigger: &[Weekday::Sunday],
        direction: SubstituteDirection::Forward,
        skip_occupied: false,
        on_collision: false,
        valid_from: Some(1973),
        valid_until: Some(2006),
    },
    SubstitutionPolicy {
        trigger: &[Weekday::Sunday],
        direction: SubstituteDirection::Forward,
        skip_occupied: true,
        on_collision: false,
        valid_from: Some(2007),
        valid_until: None,
    },
];

/// 国民の休日, 昭和60年法律第103号, in force from 1986.
///
/// A day whose previous and following days are both 国民の祝日 becomes a
/// holiday, unless it is itself a 祝日, a Sunday, or a 振替休日. The Sunday
/// exclusion is the `exclude_weekdays` entry; the 祝日 and 振替休日
/// exclusions are the engine's "already taken" check.
static BRIDGES: &[BridgePolicy] = &[BridgePolicy {
    name: "Citizens' Holiday",
    local_name: "国民の休日",
    max_gap: 1,
    exclude_weekdays: &[Weekday::Sunday],
    valid_from: Some(1986),
    valid_until: None,
}];

/// Japan.
pub static JAPAN: RuleSet = RuleSet {
    code: "JP",
    english_name: "Japan",
    rules: RULES,
    substitution: SUBSTITUTION,
    bridges: BRIDGES,
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "国民の祝日に関する法律 (昭和23年法律第178号) and every amending \
              act through 令和2年法律第68号; 内閣府「国民の祝日について」; the \
              equinox days are computed, not taken from the 官報",
};
