//! Japan — 国民の祝日に関する法律, complete from 1948.
//!
//! The regime is written up in `docs/systems/japan-holidays.md` in the
//! repository: the 祝日法 and each of its amendments with its law number
//! and commencement, the acts beside it for the imperial one-offs, the
//! accession of 2019 and the two Olympic years, the 振替休日 in its two
//! wordings and the 国民の休日, how the equinox days are announced and how
//! the crate computes them, and how the table was checked against the
//! Cabinet Office's lists and the 暦要項, with a worked example. This
//! comment keeps the summary and the code's own facts.
//!
//! This is the crate's worked example, and the claim it makes is a strong
//! one: every Japanese public holiday from the enactment of the 祝日法
//! (昭和23年法律第178号, in force 20 July 1948) to today, with every
//! amendment, expressed entirely as rule values. There is no function in
//! this file. The 振替休日, the 国民の休日, ハッピーマンデー, the imperial
//! one-offs and the two Olympic years are all data: a holiday whose date or
//! name changed is one rule per date, each bounded to its years.
//!
//! # The equinoxes are computed, not tabulated
//!
//! 春分の日 and 秋分の日 are defined by the statute as 「春分日」 and
//! 「秋分日」, the day of the equinox itself, which the National
//! Astronomical Observatory of Japan computes in JST and the Cabinet Office
//! publishes in the 官報 a year ahead; nobody legislates a table. So this
//! file carries [`Rule::SolarTerm`] at [`Meridian::JAPAN`] and lets
//! `hc-seasons` answer. `hc-seasons`'s own tests compare its equinox days
//! with the published days for 1980–2030 and with the formula that
//! reproduces them to 2099, and find no disagreement; the document says
//! what that comparison is worth.
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
