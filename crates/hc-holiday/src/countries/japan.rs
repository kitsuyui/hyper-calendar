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
//! amendment, expressed entirely as rule values. The only function in this
//! file is a `const fn` that builds a prefectural rule value. The 振替休日, the 国民の休日, ハッピーマンデー, the imperial
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
//!
//! # The prefectures' own days
//!
//! After the 国民の祝日 come the days the prefectures set for themselves by
//! ordinance, each scoped to its prefecture's ISO 3166-2 code and none of
//! them a 国民の祝日: Tokyo's 都民の日, the 県民の日 of the prefectures that
//! have one, and Okinawa's 慰霊の日. The kind is what the prefecture's
//! instruments make the day: [`Kind::School`] where the prefectural
//! schools' rule of the board of education lists it as a 休業日, from the
//! first year a text of the rule read shows it and the observance its
//! ordinance makes it before, [`Kind::Government`] for Okinawa's 慰霊の日, a 県の休日 of its 休日条例,
//! and [`Kind::Observance`] where the ordinance sets a day of events and
//! nothing closes. None is a day off for business-day arithmetic. The
//! document's section on the prefectures gives all forty-seven, with the
//! ones that have no such day, and what was read for each. The English
//! names are this crate's translations.

use hc_calendar::Weekday;
use hc_seasons::{Meridian, SolarTerm};

use crate::rule::{
    BridgePolicy, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
    SubstituteDirection, SubstitutionPolicy,
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
    // 令和: 23 February, by the 天皇の退位等に関する皇室典範特例法
    // (平成29年法律第63号), which amended the Act from 1 May 2019.
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
    // ── The prefectures' own days, by ordinance ─────────────────────────
    // Not 国民の祝日: each is scoped to its prefecture, and its first year
    // is the first date after the ordinance came into force. A day the
    // prefectural schools close on is `Kind::School` only from the first
    // year a school rule read shows it in force; before that it is the
    // observance its ordinance makes it, and whether the schools closed
    // is not known.
    prefectural(
        "Hokkaido Everyone's Day",
        "北海道みんなの日",
        7,
        17,
        2017,
        HOKKAIDO,
    )
    .cited("北海道みんなの日条例 (平成29年北海道条例第39号)"),
    prefectural(
        "Fukushima Citizens' Day",
        "福島県民の日",
        8,
        21,
        1997,
        FUKUSHIMA,
    )
    .cited("福島県民の日条例 (平成9年福島県条例第61号)"),
    prefectural("Ibaraki Citizens' Day", "県民の日", 11, 13, 1968, IBARAKI)
        .years(Some(1968), Some(2010))
        .cited("県民の日を定める条例 (昭和43年茨城県条例第3号)"),
    prefectural("Ibaraki Citizens' Day", "県民の日", 11, 13, 2011, IBARAKI)
        .of_kind(Kind::School)
        .cited(
            "県民の日を定める条例 (昭和43年茨城県条例第3号); a 休業日 of \
             茨城県県立学校管理規則 (昭和35年茨城県教育委員会規則第6号) Article 8, \
             as the article reads since 平成23年茨城県教育委員会規則第8号, in force 1 July 2011",
        ),
    prefectural("Tochigi Citizens' Day", "県民の日", 6, 15, 1986, TOCHIGI)
        .cited("栃木県県民の日に関する条例 (昭和60年栃木県条例第27号)"),
    prefectural("Gunma Citizens' Day", "群馬県民の日", 10, 28, 1985, GUNMA)
        .years(Some(1985), Some(2013))
        .cited("群馬県民の日を定める条例 (昭和60年群馬県条例第5号)"),
    prefectural("Gunma Citizens' Day", "群馬県民の日", 10, 28, 2014, GUNMA)
        .of_kind(Kind::School)
        .cited(
            "群馬県民の日を定める条例 (昭和60年群馬県条例第5号); a 休業日 of \
             群馬県立高等学校管理に関する規則 (昭和41年群馬県教育委員会規則第13号) Article 5, \
             as the rule reads since 平成26年群馬県教育委員会規則第9号, in force 1 April 2014",
        ),
    prefectural("Saitama Citizens' Day", "県民の日", 11, 14, 1971, SAITAMA)
        .years(Some(1971), Some(2016))
        .cited("県民の日を定める条例 (昭和46年埼玉県条例第58号)"),
    prefectural("Saitama Citizens' Day", "県民の日", 11, 14, 2017, SAITAMA)
        .of_kind(Kind::School)
        .cited(
            "県民の日を定める条例 (昭和46年埼玉県条例第58号); a 休業日 of \
             埼玉県立高等学校通則 (昭和30年埼玉県教育委員会規則第5号) Article 7, \
             as the rule read in force on 19 January 2017, the earliest copy read",
        ),
    prefectural("Chiba Citizens' Day", "県民の日", 6, 15, 1984, CHIBA)
        .years(Some(1984), Some(2015))
        .cited("県民の日を定める条例 (昭和59年千葉県条例第3号)"),
    prefectural("Chiba Citizens' Day", "県民の日", 6, 15, 2016, CHIBA)
        .of_kind(Kind::School)
        .cited(
            "県民の日を定める条例 (昭和59年千葉県条例第3号); a 休業日 of \
             県立高等学校管理規則 (昭和54年千葉県教育委員会規則第1号) Article 7, \
             as the rule read in force on 6 June 2016, the earliest copy read",
        ),
    prefectural("Tokyo Citizens' Day", "都民の日", 10, 1, 1952, TOKYO)
        .years(Some(1952), Some(2001))
        .cited("都民の日条例 (昭和27年東京都条例第75号)"),
    prefectural("Tokyo Citizens' Day", "都民の日", 10, 1, 2002, TOKYO)
        .of_kind(Kind::School)
        .cited(
            "都民の日条例 (昭和27年東京都条例第75号); a 休業日 of \
             東京都立学校の管理運営に関する規則 (昭和35年東京都教育委員会規則第8号) Article 5, \
             as the article reads since 平成14年東京都教育委員会規則第19号, in force 1 April 2002",
        ),
    prefectural(
        "Toyama Hometown Day",
        "県民ふるさとの日",
        5,
        9,
        2013,
        TOYAMA,
    )
    .cited("県民ふるさとの日を定める条例 (平成25年富山県条例第9号)"),
    prefectural("Fukui Hometown Day", "ふるさとの日", 2, 7, 1983, FUKUI)
        .cited("ふるさとの日に関する条例 (昭和57年福井県条例第1号)"),
    prefectural(
        "Yamanashi Citizens' Day",
        "県民の日",
        11,
        20,
        1986,
        YAMANASHI,
    )
    .years(Some(1986), Some(2001))
    .cited("県民の日条例 (昭和61年山梨県条例第1号)"),
    prefectural(
        "Yamanashi Citizens' Day",
        "県民の日",
        11,
        20,
        2002,
        YAMANASHI,
    )
    .of_kind(Kind::School)
    .cited(
        "県民の日条例 (昭和61年山梨県条例第1号); a 休業日 of \
             山梨県立学校管理規則 (昭和36年山梨県教育委員会規則第3号) Article 3, \
             as the article reads since 平成14年山梨県教育委員会規則第2号, in force 1 April 2002",
    ),
    prefectural("Shizuoka Citizens' Day", "県民の日", 8, 21, 1996, SHIZUOKA)
        .cited("静岡県県民の日条例 (平成8年静岡県条例第23号)"),
    // The prefectural schools' day off is one day of 21–27 November that
    // the board of education sets each year, not 27 November itself.
    prefectural("Aichi Citizens' Day", "あいち県民の日", 11, 27, 2023, AICHI)
        .cited("あいち県民の日条例 (令和4年愛知県条例第50号)"),
    prefectural("Mie Citizens' Day", "県民の日", 4, 18, 1976, MIE)
        .cited("県民の日条例 (昭和51年三重県条例第2号)"),
    prefectural(
        "Wakayama Hometown Birthday",
        "ふるさと誕生日",
        11,
        22,
        1989,
        WAKAYAMA,
    )
    .cited("ふるさと誕生日条例 (平成元年和歌山県条例第38号)"),
    prefectural(
        "Tottori Citizens' Day",
        "とっとり県民の日",
        9,
        12,
        1998,
        TOTTORI,
    )
    .cited("とっとり県民の日条例 (平成10年鳥取県条例第13号)"),
    prefectural("Kagawa Citizens' Day", "香川県民の日", 12, 3, 2026, KAGAWA)
        .of_kind(Kind::School)
        .cited(
            "香川県民の日条例 (令和8年香川県条例第1号); a 休業日 of 県立学校学則 \
             (昭和36年香川県教育委員会規則第1号) Article 5",
        ),
    prefectural(
        "Kagoshima Citizens' Day",
        "県民の日",
        7,
        14,
        2019,
        KAGOSHIMA,
    )
    .cited("鹿児島県県民の日を定める条例 (平成30年鹿児島県条例第45号)"),
    // 慰霊の日 was set by ordinance from 1975, and has been a 県の休日 since
    // the 休日条例 came into force on 26 May 1991. What it was for the
    // prefecture's offices in 1975–1990 no instrument read says, so those
    // years are a gap rather than a guess.
    HolidayRule::observance("Okinawa Memorial Day", "慰霊の日", Rule::UNREAD)
        .of_kind(Kind::Government)
        .years(Some(1975), Some(1990))
        .in_regions(OKINAWA)
        .cited("沖縄県慰霊の日を定める条例 (昭和49年沖縄県条例第42号)"),
    prefectural("Okinawa Memorial Day", "慰霊の日", 6, 23, 1991, OKINAWA)
        .of_kind(Kind::Government)
        .cited(
            "沖縄県慰霊の日を定める条例 (昭和49年沖縄県条例第42号); a 県の休日 of \
             沖縄県の休日を定める条例 (平成3年沖縄県条例第15号) Article 1",
        ),
];

/// A prefecture's own day on a fixed date, from `first` onwards, in the
/// prefecture `region`: an observance until the entry says otherwise.
const fn prefectural(
    name: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    first: i32,
    region: &'static [&'static str],
) -> HolidayRule {
    HolidayRule::observance(name, local_name, Rule::FixedGregorian { month, day })
        .years(Some(first), None)
        .in_regions(region)
}

// The prefectures with a day of their own, by ISO 3166-2 code.
const HOKKAIDO: &[&str] = &["JP-01"];
const FUKUSHIMA: &[&str] = &["JP-07"];
const IBARAKI: &[&str] = &["JP-08"];
const TOCHIGI: &[&str] = &["JP-09"];
const GUNMA: &[&str] = &["JP-10"];
const SAITAMA: &[&str] = &["JP-11"];
const CHIBA: &[&str] = &["JP-12"];
const TOKYO: &[&str] = &["JP-13"];
const TOYAMA: &[&str] = &["JP-16"];
const FUKUI: &[&str] = &["JP-18"];
const YAMANASHI: &[&str] = &["JP-19"];
const SHIZUOKA: &[&str] = &["JP-22"];
const AICHI: &[&str] = &["JP-23"];
const MIE: &[&str] = &["JP-24"];
const WAKAYAMA: &[&str] = &["JP-30"];
const TOTTORI: &[&str] = &["JP-31"];
const KAGAWA: &[&str] = &["JP-37"];
const KAGOSHIMA: &[&str] = &["JP-46"];
const OKINAWA: &[&str] = &["JP-47"];

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
    sources_checked: SourceDate::new(2026, 9, 28),
    sources: "国民の祝日に関する法律 (昭和23年法律第178号), as last amended by \
              平成30年法律第57号 (in force 1 January 2020), with the amending acts its \
              supplementary provisions list, 平成29年法律第63号 among them, on e-Gov \
              法令検索 (laws.e-gov.go.jp/law/323AC1000000178, read through its API), \
              retrieved 2026-09-26; the one-off acts, 平成30年法律第99号 for 2019, and \
              the Olympic special measures for 2020 and 2021, among them \
              令和2年法律第68号, as docs/systems/japan-holidays.md cites them; \
              内閣府「国民の祝日について」; the equinox days are computed, not \
              taken from the 官報; the prefectures' own days from each prefecture's \
              ordinance, 休日条例 and school rules in its 例規集, read 2026-09-28, as \
              docs/systems/japan-holidays.md lists them",
};
