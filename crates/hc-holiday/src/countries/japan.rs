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
//! amendment, expressed entirely as rule values. The only functions in
//! this file are two `const fn`s that build a prefectural or a municipal
//! rule value. The 振替休日, the 国民の休日, ハッピーマンデー, the imperial
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
//!
//! # The prefectures' other days and the cities' days
//!
//! After them come the other days a prefecture set by an instrument of its
//! own — 竹島の日, 富士山の日, びわ湖の日, the education days and the rest,
//! all observances — and the days of the twenty 政令指定都市 and 長崎市,
//! each scoped to the city's code under its prefecture's, `JP-14-100` for
//! 横浜市, the prefecture's ISO 3166-2 code and the city's three-digit
//! 市区町村コード of JIS X 0402 (ADR 0014). A city is read where a rule
//! names it or [`Subdivisions::Read`] lists it; any other keeps its
//! prefecture's days and reports its own as a gap.

use hc_calendar::Weekday;
use hc_seasons::{Meridian, SolarTerm};

use crate::rule::{
    BridgePolicy, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, Subdivisions,
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
        .except_in(CHIBA_CITY)
        .cited(
            "県民の日を定める条例 (昭和59年千葉県条例第3号); a 休業日 of the prefectural \
             high schools by 県立高等学校管理規則 (昭和54年千葉県教育委員会規則第1号) \
             Article 7, as the rule read in force on 6 June 2016, the earliest copy read",
        ),
    // The prefectural high schools of 千葉市 close on the day, but its own
    // schools do not: 千葉市立小学校及び中学校管理規則 (昭和39年千葉市教育委員会
    // 規則第1号) Article 19-2, as amended to 令和7年教委規則第5号 (1 September
    // 2025), lists no 県民の日 among their 休業日, and a page on the schools
    // says the city moves it to the autumn break. So in the city the day is
    // the ordinance's observance and not a day the schools close on.
    prefectural("Chiba Citizens' Day", "県民の日", 6, 15, 2016, CHIBA_CITY).cited(
        "県民の日を定める条例 (昭和59年千葉県条例第3号); not a 休業日 of 千葉市立小学校及び\
         中学校管理規則 (昭和39年千葉市教育委員会規則第1号) Article 19-2, as amended to \
         令和7年教委規則第5号, in force 1 September 2025",
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
    // Each school, or each municipal board of education, chooses its day
    // among candidates the prefecture's board announces, and the days
    // chosen are published only as PDF lists: a gap in every year.
    HolidayRule::observance(
        "Aichi Citizens' Day school holiday",
        "あいち県民の日学校ホリデー",
        Rule::UNREAD,
    )
    .of_kind(Kind::School)
    .years(Some(2023), None)
    .in_regions(AICHI)
    .cited(
        "愛知県立高等学校学則 (昭和39年愛知県教育委員会規則第2号) Article 4 item 3: one day of \
         21–27 November the board sets; the days each school keeps are in the board's PDF \
         lists, not read",
    ),
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
    // years are a gap rather than a guess. The prefecture was restored on
    // 15 May 1972; the 琉球政府 kept the day from 1961 (22 June, 23 June from
    // 1965) by acts that were not read, and what the prefecture kept in
    // 1972, 1973 and 1974, before the ordinance came into force on 21 October
    // 1974, is not said by any instrument read either: those years are a gap
    // too. Before 1972 the islands were not a prefecture of Japan.
    HolidayRule::observance("Okinawa Memorial Day", "慰霊の日", Rule::UNREAD)
        .of_kind(Kind::Government)
        .years(Some(1972), Some(1990))
        .in_regions(OKINAWA)
        .cited("沖縄県慰霊の日を定める条例 (昭和49年沖縄県条例第42号)"),
    prefectural("Okinawa Memorial Day", "慰霊の日", 6, 23, 1991, OKINAWA)
        .of_kind(Kind::Government)
        .cited(
            "沖縄県慰霊の日を定める条例 (昭和49年沖縄県条例第42号); a 県の休日 of \
             沖縄県の休日を定める条例 (平成3年沖縄県条例第15号) Article 1",
        ),
    // ── The municipalities' days ───────────────────────────────────────
    // Each scoped to the city's code under its prefecture's (ADR 0014):
    // the prefecture's code and the three-digit 市区町村コード of
    // JIS X 0402. The twenty 政令指定都市 were read, and 長崎市 for its
    // day beside 広島市's.
    municipal("Saitama City Citizens' Day", "さいたま市民の日", 5, 1, 2021, SAITAMA_CITY)
        .of_kind(Kind::School)
        .cited(
            "さいたま市民の日条例 (令和3年さいたま市条例第1号) Article 2; a 休業日 of \
             さいたま市立小・中学校管理規則 (平成13年さいたま市教育委員会規則第14号) Article 3, \
             as amended by 令和3年さいたま市教育委員会規則第3号, in force 1 April 2021",
        ),
    municipal("Chiba City Citizens' Day", "千葉市の市民の日", 10, 18, 1996, CHIBA_CITY)
        .cited("千葉市の市民の日 (平成7年千葉市告示第373号)"),
    // The day rested on the city assembly's resolutions before the
    // ordinance of 2025, which were not read; the school rule has listed
    // it since the article's last amendment, of January 2021.
    HolidayRule::observance(
        "Yokohama Port Opening Day",
        "開港記念日",
        Rule::FixedGregorian { month: 6, day: 2 },
    )
    .of_kind(Kind::School)
    .read_from(2021)
    .in_regions(YOKOHAMA)
    .cited(
        "横浜市開港記念日条例 (令和7年横浜市条例第21号) Article 2; a 休業日 of \
         横浜市立学校の管理運営に関する規則 (昭和59年横浜市教育委員会規則第4号) Article 4, \
         as the article reads since 令和3年横浜市教育委員会規則第1号, in force January 2021",
    ),
    municipal("Kawasaki City Foundation Day", "市制記念日", 7, 1, 1937, KAWASAKI)
        .years(Some(1937), Some(2025))
        .cited("市制記念日 (昭和12年川崎市告示第163号)"),
    municipal("Kawasaki City Foundation Day", "市制記念日", 7, 1, 2026, KAWASAKI)
        .of_kind(Kind::School)
        .cited(
            "市制記念日 (昭和12年川崎市告示第163号); a 休業日 of \
             川崎市立小学校及び中学校の管理運営に関する規則 (昭和35年川崎市教育委員会規則第5号) \
             Article 3, as the rule read in force in 2026, the earliest copy read",
        ),
    // The award ordinance names the day without its date, which only the
    // notice of the city's creation, effective 20 November 1954, implies.
    HolidayRule::observance(
        "Sagamihara City Foundation Day",
        "市制施行記念日",
        Rule::UNREAD,
    )
    .in_regions(SAGAMIHARA)
    .cited(
        "相模原市表彰条例 (昭和35年相模原市条例第18号) Article 10: awards on the 市制施行記念日, \
         whose date no instrument read gives",
    ),
    municipal("Shizuoka Tea Day", "お茶の日", 11, 1, 2010, SHIZUOKA_CITY).cited(
        "静岡市めざせ茶どころ日本一条例 (平成20年静岡市条例第160号) Article 9; the date by \
         平成22年静岡市告示第106号",
    ),
    // The ordinance of 1997 opens the city's facilities on a day it names;
    // what set the day before it was not found.
    HolidayRule::observance(
        "Hamamatsu City Foundation Day",
        "市制記念日",
        Rule::FixedGregorian { month: 7, day: 1 },
    )
    .read_from(1997)
    .in_regions(HAMAMATSU)
    .cited(
        "市制記念日及び県民の日における浜松市公の施設の開放に関する条例 \
         (平成9年浜松市条例第62号) Articles 1 and 2, fees waived",
    ),
    municipal("Nagoya Peace Day", "なごや平和の日", 5, 14, 2024, NAGOYA)
        .cited("なごや平和の日を定める条例 (令和6年名古屋市条例第36号) Article 2"),
    municipal("Kyoto Charter Day", "憲章の日", 2, 5, 2012, KYOTO_CITY).cited(
        "子どもを共に育む京都市民憲章の実践の推進に関する条例 (平成23年京都市条例第72号) Article 16",
    ),
    HolidayRule::observance(
        "Kyoto Traditional Industries Day",
        "伝統産業の日",
        Rule::SolarTerm {
            term: SolarTerm::SPRING_EQUINOX,
            meridian: Meridian::JAPAN,
        },
    )
    .years(Some(2006), None)
    .in_regions(KYOTO_CITY)
    .cited("京都市伝統産業活性化推進条例 (平成17年京都市条例第21号) Article 15: 春分の日"),
    municipal("Kyoto Food Safety Day", "食の安全安心推進の日", 8, 1, 2011, KYOTO_CITY).cited(
        "京都市食品等の安全性及び安心な食生活の確保に関する条例 (平成22年京都市条例第59号) \
         Article 15, in force 1 October 2010",
    ),
    municipal("Kyoto City Self-Government Day", "京都市自治記念日", 10, 15, 1958, KYOTO_CITY)
        .cited("京都市自治記念日について (昭和33年9月3日京都市公告)"),
    // The award ordinance of 1971 names the day; what set it was not found.
    HolidayRule::observance(
        "Sakai City Office Foundation Day",
        "開庁記念日",
        Rule::FixedGregorian { month: 7, day: 26 },
    )
    .read_from(1971)
    .in_regions(SAKAI)
    .cited("堺市有功章条例 (昭和46年堺市条例第7号) Article 4: 開庁記念日(7月26日), the day of its awards"),
    municipal("Kobe Citizens' Disaster Prevention Day", "市民防災の日", 1, 17, 1998, KOBE)
        .cited("神戸市民の安全の推進に関する条例 (平成10年神戸市条例第49号) Article 24"),
    municipal("Okayama City Citizens' Day", "岡山市民の日", 6, 1, 2012, OKAYAMA_CITY).cited(
        "岡山市, the mayor's decision of 22 March 2012, as the city's page gives it; \
         岡山市事務分掌規則 assigns its work to a division; no 告示 found",
    ),
    // A day the city's offices close: by 広島市役所事務休停日条例 from 1947
    // to 2021, and by the 休日条例 from 1991.
    municipal("Hiroshima Peace Memorial Day", "平和記念日", 8, 6, 1947, HIROSHIMA_CITY)
        .of_kind(Kind::Government)
        .cited(
            "広島市役所事務休停日条例 (昭和22年広島市条例第14号, repealed 2021); a 市の休日 of \
             広島市の休日を定める条例 (平成3年広島市条例第49号) Article 1 item 4; \
             広島市平和推進基本条例 (令和3年広島市条例第50号) Article 6",
        ),
    municipal("Nagasaki Peace Day", "ながさき平和の日", 8, 9, 1995, NAGASAKI_CITY)
        .cited("ながさき平和の日条例 (平成7年長崎市条例第2号)"),
    municipal("Kumamoto Earthquake Day", "熊本地震の日", 4, 16, 2023, KUMAMOTO_CITY)
        .cited("熊本市防災基本条例 (令和4年熊本市条例第33号) Article 16"),
    municipal("Kumamoto Citizens' Health Day", "市民健康の日", 10, 1, 1986, KUMAMOTO_CITY)
        .cited("熊本市市民健康の日を定める条例 (昭和61年熊本市条例第12号)"),
    // Two prefectures keep a day whose instrument was not found: a gap in
    // every year, rather than the prefecture's want of a day.
    HolidayRule::observance("Akita Prefecture Day", "県の記念日", Rule::UNREAD)
        .in_regions(&["JP-05"])
        .cited("秋田県, the prefecture's page on 県の記念日 (29 August, set in 1965): instrument not found"),
    HolidayRule::observance(
        "Ehime Prefectural Government Day",
        "県政発足記念日",
        Rule::UNREAD,
    )
    .in_regions(&["JP-38"])
    .cited("愛媛県, 県政発足記念日 (20 February), kept with a governor's award since 1973: instrument not found"),
    // ── The prefectures' other days ─────────────────────────────────────
    // Days a prefecture set by ordinance, or by another instrument of its
    // own, that are not the prefecture's own day: none closes anything.
    // Days set by an ordinance whose purpose is the day, or by an article of
    // a broader one.
    prefectural("Takeshima Day", "竹島の日", 2, 22, 2006, SHIMANE).cited("竹島の日を定める条例 (平成17年島根県条例第36号) Article 2"),
    prefectural("Mount Fuji Day", "富士山の日", 2, 23, 2010, SHIZUOKA).cited("静岡県富士山の日条例 (平成21年静岡県条例第72号) Article 2"),
    prefectural("Mount Fuji Day", "富士山の日", 2, 23, 2012, YAMANASHI).cited("山梨県富士山の日条例 (平成23年山梨県条例第55号) Article 2"),
    prefectural("Lake Biwa Day", "びわ湖の日", 7, 1, 1996, SHIGA).cited("滋賀県環境基本条例 (平成8年滋賀県条例第18号) Article 8"),
    prefectural("Tokyo Peace Day", "東京都平和の日", 3, 10, 1991, TOKYO).cited("東京都平和の日条例 (平成2年東京都条例第90号) Article 1; Article 2, commemorative events"),
    prefectural("Day to Hand Down the Great East Japan Earthquake and Tsunami", "東日本大震災津波を語り継ぐ日", 3, 11, 2021, IWATE).cited("東日本大震災津波を語り継ぐ日条例 (令和3年岩手県条例第1号) Article 1"),
    prefectural("Miyagi Day of Remembrance", "みやぎ鎮魂の日", 3, 11, 2014, MIYAGI).cited("みやぎ鎮魂の日を定める条例 (平成25年宮城県条例第18号) Article 2"),
    prefectural("Miyagi Citizens' Disaster Prevention Day", "みやぎ県民防災の日", 6, 12, 2009, MIYAGI).cited("震災対策推進条例 (平成20年宮城県条例第62号) Article 26"),
    prefectural("Hiraizumi World Heritage Day", "平泉世界遺産の日", 6, 29, 2014, IWATE).cited("平泉世界遺産の日条例 (平成26年岩手県条例第17号) Article 2"),
    prefectural("Hida-Mino Pride Day", "飛騨・美濃じまんの日", 8, 21, 2008, GIFU).cited("みんなでつくろう観光王国飛騨・美濃条例 (平成19年岐阜県条例第39号) Article 15"),
    prefectural("Shimakutuba Day", "しまくとぅばの日", 9, 18, 2006, OKINAWA).cited("しまくとぅばの日に関する条例 (平成18年沖縄県条例第35号) Article 2"),
    prefectural("Ryukyu History and Culture Day", "琉球歴史文化の日", 11, 1, 2021, OKINAWA).cited("琉球歴史文化の日条例 (令和3年沖縄県条例第13号) Article 2; Article 5, fees waived"),
    // The education days, 1 November in every prefecture that fixes one by
    // an instrument, except Tokyo's and Yamagata's Saturdays.
    prefectural("Iwate Education Day", "いわて教育の日", 11, 1, 2005, IWATE)
        .cited("いわて教育の日に関する条例 (平成17年岩手県条例第41号) Article 2; いわて教育週間 1–7 November"),
    prefectural("Miyagi Education Day", "みやぎ教育の日", 11, 1, 2005, MIYAGI)
        .cited("みやぎ教育の日を定める条例 (平成17年宮城県条例第90号) Article 2; みやぎ教育月間 November"),
    prefectural("Fukushima Education Day", "ふくしま教育の日", 11, 1, 2003, FUKUSHIMA)
        .cited("ふくしま教育の日条例 (平成15年福島県条例第50号) Article 2; ふくしま教育週間 1–7 November, with some fees of the prefecture's museums waived"),
    prefectural("Ibaraki Education Day", "いばらき教育の日", 11, 1, 2004, IBARAKI)
        .cited("いばらき教育の日を定める条例 (平成16年茨城県条例第35号) Article 2; いばらき教育月間 November"),
    prefectural("Saitama Education Day", "彩の国教育の日", 11, 1, 2003, SAITAMA)
        .cited("彩の国教育の日を定める要綱 (平成15年県・教育委員会告示第1号) Article 2; 彩の国教育週間 1–7 November"),
    prefectural("Niigata Education Day", "新潟県教育の日", 11, 1, 2023, NIIGATA)
        .cited("新潟県教育の日に関する条例 (令和4年新潟県条例第49号) Article 2; 新潟県教育月間 November"),
    prefectural("Ishikawa Education Day", "いしかわ教育の日", 11, 1, 2005, ISHIKAWA)
        .cited("いしかわ教育の日を定める条例 (平成17年石川県条例第32号) Article 2; いしかわ教育ウィーク 1–7 November"),
    prefectural("Shiga Education Day", "滋賀教育の日", 11, 1, 2006, SHIGA)
        .cited("「滋賀 教育の日」を定める要綱 (1 June 2006), as the prefecture's board of education's page gives it; the 要綱 itself not read"),
    prefectural("Nara Education Day", "奈良県教育の日", 11, 1, 2003, NARA)
        .cited("奈良県教育委員会告示第6号 (1 July 2003), as the prefecture's page gives it; the 告示 itself not read; 奈良県教育週間 1–7 November"),
    prefectural("Shimane Education Day", "しまね教育の日", 11, 1, 2002, SHIMANE)
        .cited("しまね教育の日を定める条例 (平成14年島根県条例第66号) Article 2; しまね教育ウィーク 1–7 November"),
    prefectural("Okayama Education Day", "おかやま教育の日", 11, 1, 2001, OKAYAMA)
        .cited("おかやま教育の日を定める条例 (平成13年岡山県条例第58号) Article 2; おかやま教育週間 1–7 November"),
    prefectural("Hiroshima Education Day", "ひろしま教育の日", 11, 1, 2001, HIROSHIMA)
        .cited("ひろしま教育の日を定める条例 (平成13年広島県条例第40号) Article 2; ひろしま教育ウィーク 1–7 November"),
    prefectural("Tokushima Education Day", "とくしま教育の日", 11, 1, 2004, TOKUSHIMA)
        .cited("とくしま教育の日を定める条例 (平成16年徳島県条例第35号) Article 2; とくしま教育週間 1–7 November"),
    prefectural("Oita Education Day", "おおいた教育の日", 11, 1, 2005, OITA)
        .cited("おおいた教育の日条例 (平成17年大分県条例第30号) Article 2; おおいた教育週間 1–7 November"),
    // Decided by the metropolitan board of education in February 2004 and
    // announced in the 東京都公報, by the board's page.
    HolidayRule::observance(
        "Tokyo Education Day",
        "東京都教育の日",
        Rule::nth(11, 1, Weekday::Saturday),
    )
    .years(Some(2004), None)
    .in_regions(TOKYO)
    .cited(
        "東京都教育委員会, 「東京都教育の日」, decided February 2004 and announced in the \
         東京都公報, as the board's page gives it: 毎年11月の第1土曜日",
    ),
    // The 要綱's year is not on the page read.
    HolidayRule::observance(
        "Yamagata Education Day",
        "やまがた教育の日",
        Rule::nth(11, 2, Weekday::Saturday),
    )
    .read_from(2026)
    .in_regions(YAMAGATA)
    .cited(
        "やまがた教育の日を定める要綱 of 山形県教育委員会, as the board's page gives it: \
         11月第2土曜日; the 要綱 and its year not read",
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

/// A municipality's day on a fixed date, from `first` onwards: an
/// observance, as a prefecture's is.
const fn municipal(
    name: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    first: i32,
    region: &'static [&'static str],
) -> HolidayRule {
    prefectural(name, local_name, month, day, first, region)
}

// The municipalities with a day carried, by the prefecture's ISO 3166-2
// code and the city's JIS X 0402 code within it.
const SAITAMA_CITY: &[&str] = &["JP-11-100"];
const CHIBA_CITY: &[&str] = &["JP-12-100"];
const YOKOHAMA: &[&str] = &["JP-14-100"];
const KAWASAKI: &[&str] = &["JP-14-130"];
const SAGAMIHARA: &[&str] = &["JP-14-150"];
const SHIZUOKA_CITY: &[&str] = &["JP-22-100"];
const HAMAMATSU: &[&str] = &["JP-22-130"];
const NAGOYA: &[&str] = &["JP-23-100"];
const KYOTO_CITY: &[&str] = &["JP-26-100"];
const SAKAI: &[&str] = &["JP-27-140"];
const KOBE: &[&str] = &["JP-28-100"];
const OKAYAMA_CITY: &[&str] = &["JP-33-100"];
const HIROSHIMA_CITY: &[&str] = &["JP-34-100"];
const NAGASAKI_CITY: &[&str] = &["JP-42-201"];
const KUMAMOTO_CITY: &[&str] = &["JP-43-100"];

// The prefectures with a day of their own or another day carried, by
// ISO 3166-2 code.
const HOKKAIDO: &[&str] = &["JP-01"];
const IWATE: &[&str] = &["JP-03"];
const MIYAGI: &[&str] = &["JP-04"];
const YAMAGATA: &[&str] = &["JP-06"];
const FUKUSHIMA: &[&str] = &["JP-07"];
const IBARAKI: &[&str] = &["JP-08"];
const TOCHIGI: &[&str] = &["JP-09"];
const GUNMA: &[&str] = &["JP-10"];
const SAITAMA: &[&str] = &["JP-11"];
const CHIBA: &[&str] = &["JP-12"];
const TOKYO: &[&str] = &["JP-13"];
const NIIGATA: &[&str] = &["JP-15"];
const TOYAMA: &[&str] = &["JP-16"];
const ISHIKAWA: &[&str] = &["JP-17"];
const FUKUI: &[&str] = &["JP-18"];
const YAMANASHI: &[&str] = &["JP-19"];
const GIFU: &[&str] = &["JP-21"];
const SHIZUOKA: &[&str] = &["JP-22"];
const AICHI: &[&str] = &["JP-23"];
const MIE: &[&str] = &["JP-24"];
const SHIGA: &[&str] = &["JP-25"];
const NARA: &[&str] = &["JP-29"];
const WAKAYAMA: &[&str] = &["JP-30"];
const TOTTORI: &[&str] = &["JP-31"];
const SHIMANE: &[&str] = &["JP-32"];
const OKAYAMA: &[&str] = &["JP-33"];
const HIROSHIMA: &[&str] = &["JP-34"];
const TOKUSHIMA: &[&str] = &["JP-36"];
const KAGAWA: &[&str] = &["JP-37"];
const OITA: &[&str] = &["JP-44"];
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
        regions: &[],
        avoid: &[],
        valid_from: Some(1973),
        valid_until: Some(2006),
    },
    SubstitutionPolicy {
        trigger: &[Weekday::Sunday],
        direction: SubstituteDirection::Forward,
        skip_occupied: true,
        on_collision: false,
        regions: &[],
        avoid: &[],
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
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "国民の祝日に関する法律 (昭和23年法律第178号), as last amended by \
              平成30年法律第57号 (in force 1 January 2020), with the amending acts its \
              supplementary provisions list, 平成29年法律第63号 among them, on e-Gov \
              法令検索 (laws.e-gov.go.jp/law/323AC1000000178, read through its API), \
              retrieved 2026-09-26; the one-off acts, 平成30年法律第99号 for 2019, and \
              the Olympic special measures for 2020 and 2021, among them \
              令和2年法律第68号, as docs/systems/japan-holidays.md cites them; \
              内閣府「国民の祝日について」; the equinox days are computed, not \
              taken from the 官報; the prefectures' own days from each prefecture's \
              ordinance, 休日条例 and school rules in its 例規集, read 2026-09-28, and \
              the prefectures' other days and the designated cities' days from their \
              例規集 or the 条例Webアーカイブ's copies of them, read 2026-09-29, as \
              docs/systems/japan-holidays.md lists them",
    // The prefectures whose 休日条例 and 例規集 were read and give no day of
    // their own, and the designated cities read that give none, each from
    // the year of its 休日条例: the ordinances read are those in force now,
    // and the regime before the 休日条例 was not read. Prefectures: the
    // 休日条例 of 1989 (公布 February to July 1989), from the 条例Webアーカイブ
    // (docs/systems/japan-holidays.md); cities: Sapporo 1990, Sendai 1989,
    // Niigata 1989, Osaka 1992 (in force 1 April), Kitakyushu 1991 and
    // Fukuoka 1990.
    subdivisions: Subdivisions::ReadFrom(&[
        ("JP-02", 1989),
        ("JP-03", 1989),
        ("JP-04", 1989),
        ("JP-06", 1989),
        ("JP-14", 1989),
        ("JP-15", 1989),
        ("JP-17", 1989),
        ("JP-20", 1989),
        ("JP-21", 1989),
        ("JP-25", 1989),
        ("JP-26", 1989),
        ("JP-27", 1989),
        ("JP-28", 1989),
        ("JP-29", 1989),
        ("JP-32", 1989),
        ("JP-33", 1989),
        ("JP-34", 1989),
        ("JP-35", 1989),
        ("JP-36", 1989),
        ("JP-39", 1989),
        ("JP-40", 1989),
        ("JP-41", 1989),
        ("JP-42", 1989),
        ("JP-43", 1989),
        ("JP-44", 1989),
        ("JP-45", 1989),
        ("JP-01-100", 1990),
        ("JP-04-100", 1989),
        ("JP-15-100", 1989),
        ("JP-27-100", 1992),
        ("JP-40-100", 1991),
        ("JP-40-130", 1990),
    ]),
};
