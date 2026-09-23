//! 暦注下段 — the lower register of the almanac page.
//!
//! The 具注暦 printed the date and the sexagenary day at the top, the 十二直
//! in the middle, and everything else below: 大明日, 天赦日, 受死日, 往亡日
//! and a couple of dozen more. That "everything else" is the 下段, and it is
//! the part the Meiji government struck out of the official calendar in 1873
//! as superstition — after which it survived in commercial almanacs and, for
//! a while, in illegally printed おばけ暦.
//!
//! # Every rule here is a rule over a cycle
//!
//! Twenty-one annotations, and between them only seven shapes: a list of
//! sexagenary days, a list of branches, a branch per 節月, a stem per 節月, a
//! sexagenary day per 節月, a day count from a sectional term, and a
//! mansion. So each entry below is a
//! name and an [`AlmanacRule`] value, and [`rule_applies`] evaluates all of
//! them. See [`crate::rules`].
//!
//! # 節切り and 月切り
//!
//! The National Astronomical Observatory of Japan's 暦Wiki gives the
//! vocabulary: a rule keyed to the 節月 is 節切り, one keyed to the calendar
//! month is 月切り, and one keyed to neither — a pure 干支 rule — is 不断.
//! It also says that 「暦注には節月を基準とするものが多く存在します」.
//!
//! In this module: 母倉日, 月徳日, 天赦日, 受死日, 十死日, 帰忌日, 血忌日,
//! 復日, 天火日, 地火日, the three 悪日 and 往亡日 are 節切り; 大明日, 天恩日,
//! 神吉日, 重日 and 五墓日 are 不断; 鬼宿日 is the 28-day mansion cycle; and
//! 凶会日 is the one where the sources genuinely fight — see
//! [`LowerRegister::KUENICHI`].
//!
//! # Provenance, and a warning about apparent corroboration
//!
//! NAOJ publishes **no** per-item 下段 rule. Nor does the 暦要項. The
//! National Diet Library's 「日本の暦」exhibition does, and so do several
//! well-known web references — but 岡田芳朗・阿久根末忠『現代こよみ読み解き
//! 事典』(柏書房, 1993) is the common ancestor of most of the latter, so
//! four agreeing websites are frequently *one* witness. The genuinely
//! independent checks used here are the National Diet Library and
//! 精選版日本国語大辞典 / デジタル大辞泉 via コトバンク, plus published
//! almanac date lists, which the tests carry.
//!
//! # Sources
//!
//! * National Diet Library, 「日本の暦」, 吉凶を表す言葉③下段
//!   (<https://www.ndl.go.jp/koyomi/chapter3/s5.html>) — the three 悪日 table,
//!   往亡日, 天赦日, 帰忌日, 母倉日, 月徳日 and the glosses.
//! * NAOJ 暦Wiki 「暦注」 and 「節月」 — the 上段/中段/下段 division, the
//!   撰日法 vocabulary, and the 節月 numbering.
//! * Japanese Wikipedia 暦注下段, which enumerates the 干支 lists and gives
//!   the 凶会日 tables for both the 宣明暦 and the 貞享暦.
//! * こよみのページ 「暦注の説明（その３）下段」.

use hc_calendar::Rd;
use hc_seasons::Meridian;

use crate::context::DayContext;
use crate::mansions::Mansion;
use crate::rules::{AlmanacRule, rule_applies};

/// 大明日 — 25 sexagenary days.
///
/// 己巳 5, 庚午 6, 辛未 7, 壬申 8, 癸酉 9, 丁丑 13, 己卯 15, 壬午 18, 甲申 20,
/// 丁亥 23, 壬辰 28, 乙未 31, 壬寅 38, 甲辰 40, 乙巳 41, 丙午 42, 丁未 43,
/// 己酉 45, 庚戌 46, 辛亥 47, 丙辰 52, 戊午 54, 己未 55, 庚申 56, 辛酉 57.
///
/// Japanese Wikipedia 暦注下段 records a 21-day variant that drops 己巳, 庚午,
/// 丁未 and 戊午. The 25-day list is the one published almanacs use and is
/// the one implemented; 2024-01-06 is 己巳 and is printed as 大明日, which
/// settles it against the 21-day variant.
static DAIMYONICHI: [u8; 25] = [
    5, 6, 7, 8, 9, 13, 15, 18, 20, 23, 28, 31, 38, 40, 41, 42, 43, 45, 46, 47, 52, 54, 55, 56, 57,
];

/// 天恩日 — three runs of five consecutive sexagenary days.
///
/// 甲子 0 through 戊辰 4, 己卯 15 through 癸未 19, and 己酉 45 through 癸丑 49.
/// Japanese Wikipedia attributes the fifteen to 『暦林問答集』 (early
/// Muromachi). Both it and こよみのページ note 「配当には諸説ある」 without
/// naming a differing set, and none was found.
static TENONNICHI: [u8; 15] = [0, 1, 2, 3, 4, 15, 16, 17, 18, 19, 45, 46, 47, 48, 49];

/// 神吉日 — 33 sexagenary days.
///
/// The Edo almanacs printed noticeably fewer than 33 in 60, because a 神吉日
/// overlapping certain 凶日 was suppressed — but which 凶日, and when, is not
/// known. Japanese Wikipedia says outright 「その規則は完全には判明していな
/// い」 and こよみのページ concludes it was each diviner's own practice. This
/// crate therefore emits all 33 and does not guess at a suppression rule.
static KAMIYOSHINICHI: [u8; 33] = [
    1, 3, 5, 6, 8, 9, 13, 15, 18, 20, 21, 24, 27, 30, 32, 33, 35, 36, 37, 39, 41, 42, 43, 44, 45,
    47, 48, 51, 54, 55, 56, 57, 59,
];

/// 五墓日 — 戊辰 4, 丙戌 22, 壬辰 28, 乙丑 1, 辛未 7.
///
/// Two things about this one are genuinely contested.
///
/// **Which 干支.** 精選版日本国語大辞典 and the National Diet Library give
/// 乙未 and 辛丑 where the 岡田芳朗 lineage and every almanac calculator give
/// 乙丑 and 辛未 — a 丑/未 swap on two of the five. The five above are the
/// calculator reading, because that is what Japanese almanacs print.
///
/// **Whether it applies to everyone.** The older rule is per-person, by the
/// 納音 of one's birth year: Japanese Wikipedia says 「その日取りは人によって
/// 異なり」 and 精選版日本国語大辞典 phrases every entry as 「木性の**人**は
/// 乙未の日」. こよみのページ records that modern practice dropped the
/// distinction — 「近年は区別をしなくなっている」 — and computes it for
/// everyone. This crate does the same. A caller who wants the per-person
/// form must filter by 納音 itself; the crate does not model 納音.
static GOMUNICHI: [u8; 5] = [1, 4, 7, 22, 28];

/// 母倉日, by 節月, as earthly branches.
///
/// 「春は亥子、夏は寅卯、秋は辰戌丑未、冬は申酉、土用は巳午」, where the
/// seasons are the 節月 in threes. Source: NDL 日本の暦; Japanese Wikipedia;
/// こよみのページ, which states 「選日法は、節月と日の十二支による」.
static BOSONICHI: [&[u8]; 12] = [
    &[0, 11],       // 正月 (寅): 子・亥
    &[0, 11],       // 二月 (卯): 子・亥
    &[5, 6],        // 三月 (辰, 土用): 巳・午
    &[2, 3],        // 四月 (巳): 寅・卯
    &[2, 3],        // 五月 (午): 寅・卯
    &[5, 6],        // 六月 (未, 土用): 巳・午
    &[1, 4, 7, 10], // 七月 (申): 丑・辰・未・戌
    &[1, 4, 7, 10], // 八月 (酉): 丑・辰・未・戌
    &[5, 6],        // 九月 (戌, 土用): 巳・午
    &[8, 9],        // 十月 (亥): 申・酉
    &[8, 9],        // 十一月 (子): 申・酉
    &[5, 6],        // 十二月 (丑, 土用): 巳・午
];

/// 月徳日, by 節月, as heavenly stems.
///
/// 寅午戌の月は丙, 亥卯未の月は甲, 申子辰の月は壬, 巳酉丑の月は庚 — the four
/// 三合 groups of the branches, which in 節月 order is a four-month repeat.
static TSUKITOKUNICHI: [&[u8]; 12] = [
    &[2], // 正月 (寅): 丙
    &[0], // 二月 (卯): 甲
    &[8], // 三月 (辰): 壬
    &[6], // 四月 (巳): 庚
    &[2], // 五月 (午): 丙
    &[0], // 六月 (未): 甲
    &[8], // 七月 (申): 壬
    &[6], // 八月 (酉): 庚
    &[2], // 九月 (戌): 丙
    &[0], // 十月 (亥): 甲
    &[8], // 十一月 (子): 壬
    &[6], // 十二月 (丑): 庚
];

/// 天赦日, by 節月, as sexagenary days.
///
/// 春 (節月 1–3, 立春 to the eve of 立夏) 戊寅 14; 夏 (4–6) 甲午 30; 秋 (7–9)
/// 戊申 44; 冬 (10–12) 甲子 0. Source: NDL 日本の暦, which states the
/// boundaries as 節気 rather than as months.
static TENSHANICHI: [&[u8]; 12] = [
    &[14],
    &[14],
    &[14],
    &[30],
    &[30],
    &[30],
    &[44],
    &[44],
    &[44],
    &[0],
    &[0],
    &[0],
];

/// 大禍日, by 節月, as earthly branches. One of the 三箇の悪日.
static TAIKANICHI: [&[u8]; 12] = [
    &[11],
    &[6],
    &[1],
    &[8],
    &[3],
    &[10],
    &[5],
    &[0],
    &[7],
    &[2],
    &[9],
    &[4],
];

/// 狼藉日, by 節月, as earthly branches. One of the 三箇の悪日.
///
/// A four-month repeat 子・卯・午・酉, which is *character for character* the
/// 天火日 table; Japanese Wikipedia notes the coincidence outright
/// (「この日取りは天火日と全く同じ」). Both are kept as separate entries
/// because both are printed separately, and the tests assert the identity
/// rather than hiding it.
static ROJAKUNICHI: [&[u8]; 12] = [
    &[0],
    &[3],
    &[6],
    &[9],
    &[0],
    &[3],
    &[6],
    &[9],
    &[0],
    &[3],
    &[6],
    &[9],
];

/// 滅門日, by 節月, as earthly branches. One of the 三箇の悪日.
///
/// Always the 冲 — the opposing branch, six places on — of 大禍日.
static METSUMONNICHI: [&[u8]; 12] = [
    &[5],
    &[0],
    &[7],
    &[2],
    &[9],
    &[4],
    &[11],
    &[6],
    &[1],
    &[8],
    &[3],
    &[10],
];

/// 帰忌日, by 節月: 丑, 寅, 子 repeating in threes.
static KIKONICHI: [&[u8]; 12] = [
    &[1],
    &[2],
    &[0],
    &[1],
    &[2],
    &[0],
    &[1],
    &[2],
    &[0],
    &[1],
    &[2],
    &[0],
];

/// 血忌日, by 節月, as earthly branches.
static CHIIMINICHI: [&[u8]; 12] = [
    &[1],
    &[7],
    &[2],
    &[8],
    &[3],
    &[9],
    &[4],
    &[10],
    &[5],
    &[11],
    &[6],
    &[0],
];

/// 重日 — 巳 and 亥, the doubled 陽 and doubled 陰 branches, all year.
static JUNICHI: [u8; 2] = [5, 11];

/// 復日, by 節月, as heavenly stems.
///
/// 正月・七月は甲・庚, 二月・八月は乙・辛, 三六九十二月は戊・己, 四月・十月は
/// 丙・壬, 五月・十一月は丁・癸.
///
/// The trap here is that Japanese Wikipedia writes the rule as 「寅節・申節は
/// 甲日・庚日」, which reads like a pairwise mapping — 寅 to 甲, 申 to 庚 —
/// and is in fact a cross product: *either* stem in *either* month.
/// こよみのページ's own implementation settles it, and so does a published
/// almanac: 2024-02-06 is 庚子 in 節月 正月 and is printed as 復日.
static FUKUNICHI: [&[u8]; 12] = [
    &[0, 6], // 正月 (寅): 甲・庚
    &[1, 7], // 二月 (卯): 乙・辛
    &[4, 5], // 三月 (辰): 戊・己
    &[2, 8], // 四月 (巳): 丙・壬
    &[3, 9], // 五月 (午): 丁・癸
    &[4, 5], // 六月 (未): 戊・己
    &[0, 6], // 七月 (申): 甲・庚
    &[1, 7], // 八月 (酉): 乙・辛
    &[4, 5], // 九月 (戌): 戊・己
    &[2, 8], // 十月 (亥): 丙・壬
    &[3, 9], // 十一月 (子): 丁・癸
    &[4, 5], // 十二月 (丑): 戊・己
];

/// 往亡日, by 節月, as days past the opening sectional term.
///
/// The published rule counts inclusively — 「正月節（立春）から7日目」, with
/// the 節入り day itself as day 1 — so the values here are one less. NDL
/// gives the twelve counts as 7, 14, 21, 8, 16, 24, 9, 18, 27, 10, 20, 30.
///
/// No source states the inclusive convention in prose; it is established
/// here from こよみのページ's implementation and from the published date
/// lists, which the tests check for a whole year.
static OMONICHI: [u8; 12] = [6, 13, 20, 7, 15, 23, 8, 17, 26, 9, 19, 29];

/// 十死日, by 節月: 酉, 巳, 丑 repeating in threes.
static JUSHINICHI: [&[u8]; 12] = [
    &[9],
    &[5],
    &[1],
    &[9],
    &[5],
    &[1],
    &[9],
    &[5],
    &[1],
    &[9],
    &[5],
    &[1],
];

/// 受死日 (黒日), by 節月, as earthly branches.
static JUSHINICHI_BLACK: [&[u8]; 12] = [
    &[10],
    &[4],
    &[11],
    &[5],
    &[0],
    &[6],
    &[1],
    &[7],
    &[2],
    &[8],
    &[3],
    &[9],
];

/// 天火日, by 節月: 子, 卯, 午, 酉 repeating in fours.
static TENKANICHI: [&[u8]; 12] = [
    &[0],
    &[3],
    &[6],
    &[9],
    &[0],
    &[3],
    &[6],
    &[9],
    &[0],
    &[3],
    &[6],
    &[9],
];

/// 地火日, by 節月: the branch three places past the month's own.
///
/// That offset is exactly the 十二直 「平」 rule, and Japanese Wikipedia
/// points out the contradiction it creates: 「十二直の『平』と全く同じ配当で
/// あるが、地火日で凶としているものが『平』では吉となっており、矛盾がある」.
/// The crate reproduces both and asserts the identity in a test rather than
/// quietly reconciling them.
static JIKANICHI: [&[u8]; 12] = [
    &[5],
    &[6],
    &[7],
    &[8],
    &[9],
    &[10],
    &[11],
    &[0],
    &[1],
    &[2],
    &[3],
    &[4],
];

/// 凶会日, by 節月, as sexagenary days — the 貞享暦 table.
///
/// Seventy entries. The 宣明暦 had eighty-two; the 貞享 reform struck twelve
/// out. Source: Japanese Wikipedia 暦注下段, whose table marks the struck
/// entries in parentheses, corroborated by the identical 貞享暦 tables
/// published by 歳事暦 and うまずたゆまず.
///
/// See [`LowerRegister::KUENICHI`] for the 節切り / 月切り dispute.
static KUENICHI: [&[u8]; 12] = [
    &[27, 50],                                       // 寅節
    &[15, 51, 57],                                   // 卯節
    &[0, 1, 2, 3, 4, 8, 16, 20, 32, 40, 44, 56],     // 辰節
    &[4, 7, 19, 31, 35, 42, 43, 54, 55, 59],         // 巳節
    &[42, 54],                                       // 午節
    &[5, 42, 43, 53, 55],                            // 未節
    &[21, 40, 56],                                   // 申節
    &[45, 51, 57],                                   // 酉節
    &[10, 27, 28, 29, 30, 31, 32, 33, 34, 46, 50],   // 戌節
    &[1, 5, 13, 24, 25, 34, 35, 37, 48, 49, 53, 59], // 亥節
    &[24, 42, 48],                                   // 子節
    &[24, 43, 48, 59],                               // 丑節
];

/// One of the 暦注下段.
///
/// Ordering groups the auspicious ones first, then the inauspicious, which
/// is roughly how a reference lists them. The set varies by publisher and by
/// era, which is why it is a table and not an `enum` (ADR 0007): an
/// annotation this crate has not met is an entry with its own rule.
#[derive(Debug, Clone, Copy)]
pub struct LowerRegister {
    /// A short identifier, the variant name in kebab case.
    pub id: &'static str,
    rule: AlmanacRule,
    japanese_name: &'static str,
    romaji: &'static str,
    english_name: &'static str,
    auspicious: bool,
    meaning: &'static str,
    suppresses_the_rest: bool,
}

impl PartialEq for LowerRegister {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for LowerRegister {}

impl core::hash::Hash for LowerRegister {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for LowerRegister {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LowerRegister {
    /// Listing order: the position in [`LowerRegister::ALL`]. A value that is not
    /// in it sorts after every entry, and among such values by identifier.
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        use core::cmp::Ordering;
        match (self.index(), other.index()) {
            (Some(left), Some(right)) => left.cmp(&right),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => self.id.cmp(other.id),
        }
    }
}

hc_core::catalogue! {
    type: LowerRegister,
    id: |entry| entry.id,
    tests: lower_register_tests,
    associated;

    /// All twenty-one, in listing order.
    pub const ALL;
    /// The entry with this identifier.
    pub fn by_id;

    entries: {
        /// 大明日 — "the great brightness"; heaven and earth are in accord.
        pub const DAIMYONICHI = Self {
            id: "daimyonichi",
            rule: AlmanacRule::SexagenaryIn(&DAIMYONICHI),
            japanese_name: "大明日",
            romaji: "daimyōnichi",
            english_name: "day of great brightness",
            auspicious: true,
            meaning: "yin and yang are in accord; auspicious in all things, and especially \
                 for building, moving house and travel",
            suppresses_the_rest: false,
        };
        /// 天恩日 — heaven's grace descends. For celebrations only.
        pub const TENONNICHI = Self {
            id: "tenonnichi",
            rule: AlmanacRule::SexagenaryIn(&TENONNICHI),
            japanese_name: "天恩日",
            romaji: "ten'onnichi",
            english_name: "day of heaven's grace",
            auspicious: true,
            meaning: "heaven's grace descends on all below; use it for celebrations, and \
                 never for a funeral or any other sorrowful matter",
            suppresses_the_rest: false,
        };
        /// 母倉日 — heaven cherishes mankind as a mother her child.
        pub const BOSONICHI = Self {
            id: "bosonichi",
            rule: AlmanacRule::BranchBySolarMonth(&BOSONICHI),
            japanese_name: "母倉日",
            romaji: "bosōnichi",
            english_name: "day heaven cherishes as a mother",
            auspicious: true,
            meaning: "heaven cherishes mankind as a mother her child; auspicious in all \
                 things, and greatly so for marriage",
            suppresses_the_rest: false,
        };
        /// 月徳日 — the month's virtue; for building and moving earth.
        pub const TSUKITOKUNICHI = Self {
            id: "tsukitokunichi",
            rule: AlmanacRule::StemBySolarMonth(&TSUKITOKUNICHI),
            japanese_name: "月徳日",
            romaji: "tsukitokunichi",
            english_name: "day of the month's virtue",
            auspicious: true,
            meaning: "the virtue of the month; good for building, repair and breaking ground",
            suppresses_the_rest: false,
        };
        /// 神吉日 — good for anything to do with the gods.
        pub const KAMIYOSHINICHI = Self {
            id: "kamiyoshinichi",
            rule: AlmanacRule::SexagenaryIn(&KAMIYOSHINICHI),
            japanese_name: "神吉日",
            romaji: "kamiyoshinichi",
            english_name: "day good for the gods",
            auspicious: true,
            meaning: "good for everything to do with the gods — shrine visits, festivals, \
                 prayers, ancestral rites — and bad for anything unclean",
            suppresses_the_rest: false,
        };
        /// 鬼宿日 — the day of 鬼宿; the best of the twenty-eight mansions.
        pub const KISHUKUNICHI = Self {
            id: "kishukunichi",
            rule: AlmanacRule::Mansion(Mansion::GHOST),
            japanese_name: "鬼宿日",
            romaji: "kishukunichi",
            english_name: "day of the ghost mansion",
            auspicious: true,
            meaning: "the best of the twenty-eight mansions; the dictionaries add that \
                 marriage is the one exception",
            suppresses_the_rest: false,
        };
        /// 天赦日 — heaven pardons all things. The most auspicious day there is.
        pub const TENSHANICHI = Self {
            id: "tenshanichi",
            rule: AlmanacRule::SexagenaryBySolarMonth(&TENSHANICHI),
            japanese_name: "天赦日",
            romaji: "tenshanichi",
            english_name: "day heaven pardons everything",
            auspicious: true,
            meaning: "the hundred gods assemble in heaven and heaven forgives every offence; \
                 the almanac prints 万よし against it, and it is the most auspicious day \
                 in the calendar",
            suppresses_the_rest: false,
        };
        /// 大禍日 — the great calamity; one of the 三箇の悪日.
        pub const TAIKANICHI = Self {
            id: "taikanichi",
            rule: AlmanacRule::BranchBySolarMonth(&TAIKANICHI),
            japanese_name: "大禍日",
            romaji: "taikanichi",
            english_name: "day of great calamity",
            auspicious: false,
            meaning: "the severest of the three evil days; bad for quarrels, house repair, \
                 gates, sea voyages and funerals",
            suppresses_the_rest: false,
        };
        /// 狼藉日 — violence and disorder; one of the 三箇の悪日.
        pub const ROJAKUNICHI = Self {
            id: "rojakunichi",
            rule: AlmanacRule::BranchBySolarMonth(&ROJAKUNICHI),
            japanese_name: "狼藉日",
            romaji: "rōjakunichi",
            english_name: "day of violence and disorder",
            auspicious: false,
            meaning: "whatever is attempted comes to violence and fails; one of the three \
                 evil days",
            suppresses_the_rest: false,
        };
        /// 滅門日 — the ruin of a whole house; one of the 三箇の悪日.
        pub const METSUMONNICHI = Self {
            id: "metsumonnichi",
            rule: AlmanacRule::BranchBySolarMonth(&METSUMONNICHI),
            japanese_name: "滅門日",
            romaji: "metsumonnichi",
            english_name: "day that destroys a house",
            auspicious: false,
            meaning: "a whole house and line is destroyed; one of the three evil days",
            suppresses_the_rest: false,
        };
        /// 帰忌日 — the taboo on returning home.
        pub const KIKONICHI = Self {
            id: "kikonichi",
            rule: AlmanacRule::BranchBySolarMonth(&KIKONICHI),
            japanese_name: "帰忌日",
            romaji: "kikonichi",
            english_name: "day one must not go home",
            auspicious: false,
            meaning: "the essence of a baleful star blocks the doorway; do not travel, do not \
                 come home, do not move house or take a wife",
            suppresses_the_rest: false,
        };
        /// 血忌日 — the taboo on the sight of blood.
        pub const CHIIMINICHI = Self {
            id: "chiiminichi",
            rule: AlmanacRule::BranchBySolarMonth(&CHIIMINICHI),
            japanese_name: "血忌日",
            romaji: "chiiminichi",
            english_name: "day one must not see blood",
            auspicious: false,
            meaning: "do nothing that draws blood: no acupuncture, no surgery, no execution, \
                 no hunting",
            suppresses_the_rest: false,
        };
        /// 重日 — whatever is done today is doubled, for good or ill.
        pub const JUNICHI = Self {
            id: "junichi",
            rule: AlmanacRule::BranchIn(&JUNICHI),
            japanese_name: "重日",
            romaji: "jūnichi",
            english_name: "day that doubles whatever is done",
            auspicious: false,
            meaning: "good done today is doubled and ill done today is doubled; marriage is \
                 avoided because it would mean a second one",
            suppresses_the_rest: false,
        };
        /// 復日 — the same doubling, keyed to the stem instead of the branch.
        pub const FUKUNICHI = Self {
            id: "fukunichi",
            rule: AlmanacRule::StemBySolarMonth(&FUKUNICHI),
            japanese_name: "復日",
            romaji: "fukunichi",
            english_name: "day that repeats whatever is done",
            auspicious: false,
            meaning: "the same doubling as 重日, and the same avoidance of marriage",
            suppresses_the_rest: false,
        };
        /// 往亡日 — "go and perish"; the taboo on setting out.
        pub const OMONICHI = Self {
            id: "omonichi",
            rule: AlmanacRule::DaysIntoSolarMonth(&OMONICHI),
            japanese_name: "往亡日",
            romaji: "ōmōnichi",
            english_name: "day one goes out and perishes",
            auspicious: false,
            meaning: "to go out is to perish; bad for setting out, marching, travel and posting",
            suppresses_the_rest: false,
        };
        /// 凶会日 — the gathering of ills; the two breaths fail to harmonise.
        ///
        /// # The one rule this crate had to choose
        ///
        /// Sources contradict each other about whether 凶会日 is 節切り or 月切り,
        /// and the contradiction is inside single documents. Japanese
        /// Wikipedia's prose says 「宣明暦時代は節切りで、貞享暦以降は月切り（旧
        /// 暦）による」 but its own table is headed 「注：節切り。」 with rows
        /// labelled 寅節 through 丑節. こよみる computes it by 旧暦月;
        /// うまずたゆまず and 歳事暦 by 節月; 精選版日本国語大辞典 phrases it as
        /// 旧暦正月.
        ///
        /// **This crate uses the 節月**, because the table it ships is the one
        /// Japanese Wikipedia prints and that table is labelled 節切り, and
        /// because every other 節-or-month rule in this module is 節切り. A
        /// caller who needs the 月切り reading must build the rule itself from
        /// [`LowerRegister::rule`]'s table and evaluate it against
        /// [`DayContext::lunisolar`].
        pub const KUENICHI = Self {
            id: "kuenichi",
            rule: AlmanacRule::SexagenaryBySolarMonth(&KUENICHI),
            japanese_name: "凶会日",
            romaji: "kuenichi",
            english_name: "day the ills gather",
            auspicious: false,
            meaning: "the two breaths fail to harmonise and every ill gathers; bad for \
                 marriage, travel and everything else",
            suppresses_the_rest: false,
        };
        /// 十死日 — also 十死一生日 or 天殺日. Second only to 受死日.
        pub const JUSHINICHI = Self {
            id: "jushinichi",
            rule: AlmanacRule::BranchBySolarMonth(&JUSHINICHI),
            japanese_name: "十死日",
            romaji: "jūshinichi",
            english_name: "day of ten deaths",
            auspicious: false,
            meaning: "second only to 受死日, and worse in one respect: unlike 受死日 it \
                 forbids funerals too",
            suppresses_the_rest: true,
        };
        /// 受死日, also called 黒日 — the worst day of all, printed as a black
        /// dot. Named `Kurobi` here only because 十死日 and 受死日 romanise to
        /// almost the same thing and one of them had to take its other name.
        pub const KUROBI = Self {
            id: "kurobi",
            rule: AlmanacRule::BranchBySolarMonth(&JUSHINICHI_BLACK),
            japanese_name: "受死日",
            romaji: "jushinichi",
            english_name: "day of receiving death; the black day",
            auspicious: false,
            meaning: "the worst day of the almanac, marked with a black dot; fall ill today \
                 and you die. Only a funeral is unaffected",
            suppresses_the_rest: true,
        };
        /// 天火日 — a roof raised today burns.
        pub const TENKANICHI = Self {
            id: "tenkanichi",
            rule: AlmanacRule::BranchBySolarMonth(&TENKANICHI),
            japanese_name: "天火日",
            romaji: "tenkanichi",
            english_name: "day of heaven's fire",
            auspicious: false,
            meaning: "raise a roof or thatch one today and it will certainly burn; other \
                 matters are unaffected",
            suppresses_the_rest: false,
        };
        /// 地火日 — the earth is full of fire; do not break it.
        pub const JIKANICHI = Self {
            id: "jikanichi",
            rule: AlmanacRule::BranchBySolarMonth(&JIKANICHI),
            japanese_name: "地火日",
            romaji: "jikanichi",
            english_name: "day of the earth's fire",
            auspicious: false,
            meaning: "fire fills the earth; no foundations, no post-setting, no well-digging, \
                 no sowing, no grave-building",
            suppresses_the_rest: false,
        };
        /// 五墓日 — five graves; five deaths.
        pub const GOMUNICHI = Self {
            id: "gomunichi",
            rule: AlmanacRule::SexagenaryIn(&GOMUNICHI),
            japanese_name: "五墓日",
            romaji: "gomunichi",
            english_name: "day of the five graves",
            auspicious: false,
            meaning: "do these things and you pile up five graves; bad for breaking ground, \
                 funerals, sowing and travel, though not for building a house as such",
            suppresses_the_rest: false,
        };
    }
}

/// Byte-for-byte equality of two identifiers, usable in `const` context.
const fn same_id(left: &str, right: &str) -> bool {
    let (left, right) = (left.as_bytes(), right.as_bytes());
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if left[index] != right[index] {
            return false;
        }
        index += 1;
    }
    true
}

impl LowerRegister {
    /// This entry's position in [`LowerRegister::ALL`], or `None` for a
    /// value that is not one of its entries, such as a copy whose public `id`
    /// was changed.
    #[must_use]
    pub const fn index(self) -> Option<u8> {
        let mut index = 0;
        while index < Self::ALL.len() {
            if same_id(Self::ALL[index].id, self.id) {
                return Some(index as u8);
            }
            index += 1;
        }
        None
    }

    /// The three 悪日, which an almanac prints together.
    pub const THREE_EVIL_DAYS: [Self; 3] =
        [Self::TAIKANICHI, Self::ROJAKUNICHI, Self::METSUMONNICHI];

    /// The rule that fixes this day.
    #[must_use]
    pub const fn rule(self) -> AlmanacRule {
        self.rule
    }

    /// The name in Japanese characters, e.g. `"大明日"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        self.japanese_name
    }

    /// The reading in Hepburn romaji.
    ///
    /// Several have two readings in print: 月徳日 is *tsukitokunichi* in
    /// こよみのページ and *gettokunichi* in the National Diet Library, and
    /// Japanese Wikipedia heads the same rule 節徳日 *settokunichi*. 受死日
    /// is also *kurobi* (黒日) or *marobu hi* (辷日). The commoner form is
    /// given.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        self.romaji
    }

    /// A one-line English gloss.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        self.english_name
    }

    /// Whether the day is auspicious.
    #[must_use]
    pub const fn is_auspicious(self) -> bool {
        self.auspicious
    }

    /// What the day is held to mean, in one sentence.
    #[must_use]
    pub const fn meaning(self) -> &'static str {
        self.meaning
    }

    /// Whether this annotation traditionally suppresses every other entry in
    /// the lower register when it falls.
    ///
    /// 受死日 and 十死日 were printed alone: 「受死日（●）と十死日（十し）は他
    /// のものと重複して記載されず」 (Japanese Wikipedia 暦注下段). Everything
    /// else stacks, and several entries on one day is normal. This crate
    /// computes every annotation regardless; the flag is here so a caller
    /// rendering an almanac page can reproduce the convention.
    #[must_use]
    pub const fn suppresses_the_rest(self) -> bool {
        self.suppresses_the_rest
    }

    /// Whether this 暦注 holds on the day a context describes.
    ///
    /// Never `None` in this module: every entry has an established rule.
    #[must_use]
    pub fn applies_to(self, context: &DayContext) -> Option<bool> {
        rule_applies(self.rule(), context)
    }
}

/// Every 暦注下段 in force on a day, as a bit set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LowerRegisterSet {
    bits: u32,
}

impl LowerRegisterSet {
    /// The empty set.
    pub const EMPTY: Self = Self { bits: 0 };

    /// Whether an annotation is in the set.
    #[must_use]
    pub const fn contains(self, note: LowerRegister) -> bool {
        match note.index() {
            Some(index) => self.bits & (1 << index) != 0,
            None => false,
        }
    }

    /// Add an annotation to the set.
    ///
    /// `None` for a value that is not one of [`LowerRegister::ALL`], which has no
    /// place in a set.
    #[must_use]
    pub const fn with(self, note: LowerRegister) -> Option<Self> {
        match note.index() {
            Some(index) => Some(self.with_position(index as usize)),
            None => None,
        }
    }

    /// Add the entry at a position in [`LowerRegister::ALL`].
    const fn with_position(self, position: usize) -> Self {
        Self {
            bits: self.bits | (1 << position),
        }
    }

    /// How many annotations are in the set.
    #[must_use]
    pub const fn len(self) -> u32 {
        self.bits.count_ones()
    }

    /// Whether the set is empty.
    ///
    /// A real almanac page occasionally has an empty 下段, so this is a
    /// meaningful answer and not an error.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }

    /// The annotations in the set, in [`LowerRegister::ALL`] order.
    pub fn iter(self) -> impl Iterator<Item = LowerRegister> {
        LowerRegister::ALL
            .iter()
            .copied()
            .filter(move |note| self.contains(*note))
    }

    /// The set as an almanac would print it: if 受死日 or 十死日 falls, only
    /// that, because the two suppress everything else.
    #[must_use]
    pub fn as_printed(self) -> Self {
        for (position, note) in LowerRegister::ALL.iter().enumerate() {
            if note.suppresses_the_rest() && self.contains(*note) {
                return Self::EMPTY.with_position(position);
            }
        }
        self
    }
}

/// Every 暦注下段 in force on a day, from a context.
#[must_use]
pub fn lower_register_of_context(context: &DayContext) -> LowerRegisterSet {
    let mut set = LowerRegisterSet::EMPTY;
    for (position, note) in LowerRegister::ALL.iter().enumerate() {
        if note.applies_to(context) == Some(true) {
            set = set.with_position(position);
        }
    }
    set
}

/// Every 暦注下段 in force on a day at a meridian.
#[must_use]
pub fn lower_register(day: Rd, meridian: Meridian) -> LowerRegisterSet {
    lower_register_of_context(&DayContext::new(day, meridian))
}

#[cfg(test)]
mod tests {
    use crate::twelve_directs::{TwelveDirect, direct_of};

    use super::*;

    const JAPAN: Meridian = Meridian::JAPAN;

    /// 2024-01-01 and 2026-01-01.
    const NEW_YEAR_2024: i64 = 738_886;
    const NEW_YEAR_2026: i64 = 739_617;

    const DAYS_IN_MONTH: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const CUMULATIVE_COMMON: [i64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    const CUMULATIVE_LEAP: [i64; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

    fn rd_2024(month: usize, day: i64) -> i64 {
        NEW_YEAR_2024 + CUMULATIVE_LEAP[month - 1] + day - 1
    }

    fn rd_2026(month: usize, day: i64) -> i64 {
        NEW_YEAR_2026 + CUMULATIVE_COMMON[month - 1] + day - 1
    }

    fn holds(note: LowerRegister, rd: i64) -> bool {
        note.applies_to(&DayContext::new(Rd(rd), JAPAN)) == Some(true)
    }

    #[test]
    fn every_annotation_has_a_name_a_reading_a_gloss_and_a_rule() {
        for note in LowerRegister::ALL.iter().copied() {
            assert!(!note.japanese_name().is_empty());
            assert!(!note.romaji().is_empty());
            assert!(!note.english_name().is_empty());
            assert!(!note.meaning().is_empty());
            assert_ne!(note.rule(), AlmanacRule::Undetermined);
        }
    }

    #[test]
    fn a_value_outside_the_table_has_no_index_and_no_place_in_a_set() {
        let mut stranger = LowerRegister::ALL[0];
        stranger.id = "not-a-lower-register-note";
        assert_eq!(stranger.index(), None);
        assert!(!LowerRegisterSet::EMPTY.contains(stranger));
        assert_eq!(LowerRegisterSet::EMPTY.with(stranger), None);
        assert!(stranger > LowerRegister::ALL[0]);
    }

    #[test]
    fn the_indices_are_distinct_and_match_the_listing_order() {
        for (position, note) in LowerRegister::ALL.iter().enumerate() {
            assert_eq!(note.index().map(usize::from), Some(position));
        }
        let mut set = LowerRegisterSet::EMPTY;
        assert!(set.is_empty());
        for note in LowerRegister::ALL.iter().copied() {
            assert!(!set.contains(note));
            set = set.with(note).unwrap();
        }
        assert_eq!(set.len() as usize, LowerRegister::ALL.len());
        assert_eq!(set.iter().count(), LowerRegister::ALL.len());
    }

    /// The published 天赦日 for 2024 are 1 January, 15 March, 30 May, 29
    /// July, 12 August, 11 October and 26 December; for 2025, 10 March, 25
    /// May, 24 July, 7 August, 6 October and 21 December. Sources: 吉日
    /// カレンダー, zired.
    #[test]
    fn the_published_days_of_heavens_pardon_match() {
        let expected_2024 = [
            (1, 1),
            (3, 15),
            (5, 30),
            (7, 29),
            (8, 12),
            (10, 11),
            (12, 26),
        ];
        for (month, day) in expected_2024 {
            assert!(
                holds(LowerRegister::TENSHANICHI, rd_2024(month, day)),
                "2024-{month:02}-{day:02}"
            );
        }
        let count = (0..366)
            .filter(|offset| holds(LowerRegister::TENSHANICHI, NEW_YEAR_2024 + offset))
            .count();
        assert_eq!(count, expected_2024.len());
    }

    /// 立秋 2025 fell on 7 August at 22:52 JST, and 7 August 2025 was 戊申 —
    /// the autumn 天赦日. Published almanacs list it, which they can only do
    /// if the 節入り *day* belongs entirely to the new 節月, however late in
    /// the evening the instant falls. That is the single sharpest test of
    /// the 節月 boundary convention in the crate.
    #[test]
    fn a_solar_term_that_arrives_at_ten_at_night_still_opens_its_month_that_day() {
        let seventh_of_august = Rd(739_470);
        let context = DayContext::new(seventh_of_august, JAPAN);
        assert_eq!(context.sexagenary().index(), 44); // 戊申
        assert_eq!(context.solar_month().number(), 7); // 申月, opened by 立秋
        assert_eq!(context.days_into_solar_month(), 0);
        assert!(holds(LowerRegister::TENSHANICHI, seventh_of_august.0));
        // The day before is still 未月, where the 天赦日 is 甲午 and not 戊申.
        assert!(!holds(LowerRegister::TENSHANICHI, seventh_of_august.0 - 1));
    }

    /// The published 往亡日 for 2026, all twelve, from the 節気 dates in the
    /// National Astronomical Observatory of Japan's 暦要項 for that year.
    /// This is the test that pins the inclusive counting convention: 立春
    /// 2026 was 4 February and the 往亡日 is 10 February, which is day 7
    /// counting the term's own day as the first.
    #[test]
    fn the_published_2026_days_of_going_out_and_perishing_match() {
        let expected = [
            (2usize, 3i64),
            (2, 10),
            (3, 18),
            (4, 25),
            (5, 12),
            (6, 21),
            (7, 30),
            (8, 15),
            (9, 24),
            (11, 3),
            (11, 16),
            (12, 26),
        ];
        for month in 1..=12usize {
            for day in 1..=DAYS_IN_MONTH[month - 1] {
                assert_eq!(
                    holds(LowerRegister::OMONICHI, rd_2026(month, day)),
                    expected.contains(&(month, day)),
                    "2026-{month:02}-{day:02}"
                );
            }
        }
    }

    /// Individually published values from 吉日カレンダー for 2024, chosen so
    /// that between them they exercise eight different rules and both sides
    /// of a 節月 boundary.
    #[test]
    fn the_published_2024_lower_register_entries_match() {
        // 2024-01-06 is 己巳 in 節月 十二月: 大明日 and 神吉日.
        assert!(holds(LowerRegister::DAIMYONICHI, rd_2024(1, 6)));
        assert!(holds(LowerRegister::KAMIYOSHINICHI, rd_2024(1, 6)));
        // 2024-02-21 is 乙卯 in 節月 正月: 神吉日 but *not* 大明日. This is
        // what rules out the circulated 大明日 list that contains 乙卯.
        assert!(holds(LowerRegister::KAMIYOSHINICHI, rd_2024(2, 21)));
        assert!(!holds(LowerRegister::DAIMYONICHI, rd_2024(2, 21)));
        // 2024-01-01 is 甲子: 天恩日. 2024-02-15 is 己酉: 天恩日 and 十死日.
        assert!(holds(LowerRegister::TENONNICHI, rd_2024(1, 1)));
        assert!(holds(LowerRegister::TENONNICHI, rd_2024(2, 15)));
        assert!(holds(LowerRegister::JUSHINICHI, rd_2024(2, 15)));
        // 2024-01-31 is 甲午: *not* 天恩日. This rules out the circulated
        // third run of 甲午 through 戊戌.
        assert!(!holds(LowerRegister::TENONNICHI, rd_2024(1, 31)));
        // 2024-01-31 is 甲午 in 節月 十二月 (土用): 母倉日, since the 土用
        // months take 巳・午.
        assert!(holds(LowerRegister::BOSONICHI, rd_2024(1, 31)));
        // 2024-02-06 is 庚子 in 節月 正月: 母倉日 (子), 復日 (庚) and 天火日.
        assert!(holds(LowerRegister::BOSONICHI, rd_2024(2, 6)));
        assert!(holds(LowerRegister::FUKUNICHI, rd_2024(2, 6)));
        assert!(holds(LowerRegister::TENKANICHI, rd_2024(2, 6)));
        // 2024-02-11 is 乙巳 in 節月 正月: 地火日, and *not* 復日 — the trap
        // in the 復日 rule.
        assert!(holds(LowerRegister::JIKANICHI, rd_2024(2, 11)));
        assert!(!holds(LowerRegister::FUKUNICHI, rd_2024(2, 11)));
        // 2024-02-04 is 戊戌 and 立春: 受死日, since 節月 正月 takes 戌.
        assert!(holds(LowerRegister::KUROBI, rd_2024(2, 4)));
        // 2024-01-29 is 壬辰 in 節月 十二月: 地火日, which takes 辰.
        assert!(holds(LowerRegister::JIKANICHI, rd_2024(1, 29)));
        // 2024-02-10 is 甲辰, six days past 立春: 往亡日.
        assert!(holds(LowerRegister::OMONICHI, rd_2024(2, 10)));
    }

    /// 受死日 proves the 節切り reading on its own: 2024-02-04 is 旧暦十二月
    /// 廿五日 but 節月 正月, and the 正月 branch 戌 is what makes it a 受死日.
    /// A 旧暦月 reading would take the twelfth month's 酉 and miss it.
    #[test]
    fn the_day_of_receiving_death_follows_the_solar_month_not_the_lunar_one() {
        let lichun = Rd(rd_2024(2, 4));
        let context = DayContext::new(lichun, JAPAN);
        assert_eq!(context.solar_month().number(), 1);
        assert_eq!(context.lunisolar().month, 12);
        assert_eq!(context.branch_index(), 10); // 戌
        assert!(holds(LowerRegister::KUROBI, lichun.0));
    }

    /// Japanese Wikipedia states that 狼藉日 and 天火日 share a table exactly.
    /// They are separate annotations and both are printed, so the crate keeps
    /// both — and asserts the identity rather than hiding it.
    #[test]
    fn the_day_of_violence_and_the_day_of_heavens_fire_always_coincide() {
        for offset in 0..800 {
            let rd = NEW_YEAR_2024 + offset;
            assert_eq!(
                holds(LowerRegister::ROJAKUNICHI, rd),
                holds(LowerRegister::TENKANICHI, rd),
                "RD {rd}"
            );
        }
    }

    /// 地火日 is the branch three places past the month's, and so is the
    /// 十二直 「平」. Japanese Wikipedia points out that the two contradict
    /// each other — 平 is auspicious, 地火日 is not — and the crate
    /// reproduces both.
    #[test]
    fn the_day_of_the_earths_fire_is_always_the_twelfth_direct_of_levelling() {
        for offset in 0..800 {
            let rd = Rd(NEW_YEAR_2024 + offset);
            assert_eq!(
                holds(LowerRegister::JIKANICHI, rd.0),
                direct_of(rd, JAPAN) == TwelveDirect::Level,
                "RD {}",
                rd.0
            );
        }
    }

    /// 大禍日 is always the 冲 of 滅門日 — the opposing branch, six places on.
    /// The three 悪日 therefore never all fall together, and 大禍 and 滅門
    /// never do.
    #[test]
    fn the_great_calamity_always_opposes_the_ruin_of_the_house() {
        for month in 0..12 {
            assert_eq!(
                (TAIKANICHI[month][0] + 6) % 12,
                METSUMONNICHI[month][0],
                "at 節月 {}",
                month + 1
            );
        }
        for offset in 0..400 {
            let rd = NEW_YEAR_2024 + offset;
            assert!(
                !(holds(LowerRegister::TAIKANICHI, rd) && holds(LowerRegister::METSUMONNICHI, rd))
            );
        }
    }

    /// The 凶会日 table is the 貞享暦 one: seventy entries across the twelve
    /// 節月, down from the 宣明暦's eighty-two.
    #[test]
    fn the_evil_gathering_table_holds_the_seventy_entries_of_the_jokyo_calendar() {
        let total: usize = KUENICHI.iter().map(|row| row.len()).sum();
        assert_eq!(total, 70);
        for row in KUENICHI {
            for window in row.windows(2) {
                assert!(window[0] < window[1], "rows must be sorted and distinct");
            }
            for entry in row {
                assert!(*entry < 60);
            }
        }
    }

    /// Every sexagenary list must be sorted, distinct and in range — a
    /// transcription check on the data, which is where the bugs in a
    /// data-driven crate live.
    #[test]
    fn the_sexagenary_lists_are_sorted_distinct_and_in_range() {
        for list in [
            &DAIMYONICHI[..],
            &TENONNICHI[..],
            &KAMIYOSHINICHI[..],
            &GOMUNICHI[..],
        ] {
            for window in list.windows(2) {
                assert!(window[0] < window[1]);
            }
            for entry in list {
                assert!(*entry < 60);
            }
        }
        assert_eq!(DAIMYONICHI.len(), 25);
        assert_eq!(TENONNICHI.len(), 15);
        assert_eq!(KAMIYOSHINICHI.len(), 33);
        assert_eq!(GOMUNICHI.len(), 5);
    }

    /// Every by-節月 table must cover all twelve months with in-range
    /// branches or stems.
    #[test]
    fn the_monthly_tables_cover_twelve_months_with_in_range_values() {
        for table in [
            &BOSONICHI,
            &TAIKANICHI,
            &ROJAKUNICHI,
            &METSUMONNICHI,
            &KIKONICHI,
            &CHIIMINICHI,
            &JUSHINICHI,
            &JUSHINICHI_BLACK,
            &TENKANICHI,
            &JIKANICHI,
        ] {
            assert_eq!(table.len(), 12);
            for row in table.iter() {
                assert!(!row.is_empty());
                for branch in row.iter() {
                    assert!(*branch < 12);
                }
            }
        }
        for table in [&TSUKITOKUNICHI, &FUKUNICHI] {
            for row in table.iter() {
                for stem in row.iter() {
                    assert!(*stem < 10);
                }
            }
        }
        for count in OMONICHI {
            assert!(count < 32);
        }
    }

    /// 重日 is 巳 and 亥 all year, so it falls exactly one day in six.
    #[test]
    fn the_doubling_day_falls_twice_in_every_twelve() {
        let count = (0..60)
            .filter(|offset| holds(LowerRegister::JUNICHI, NEW_YEAR_2024 + offset))
            .count();
        assert_eq!(count, 10);
    }

    /// 天赦日 is 節切り, so it can only fall on a day whose 節月 season takes
    /// that sexagenary position: every 天赦日 must be 戊寅, 甲午, 戊申 or 甲子.
    #[test]
    fn every_day_of_heavens_pardon_is_one_of_four_sexagenary_positions() {
        for offset in 0..1_100 {
            let rd = Rd(NEW_YEAR_2024 + offset);
            if holds(LowerRegister::TENSHANICHI, rd.0) {
                let position = DayContext::new(rd, JAPAN).sexagenary().index();
                assert!(matches!(position, 0 | 14 | 30 | 44), "RD {}", rd.0);
            }
        }
    }

    /// 受死日 and 十死日 suppress everything else on the page, and nothing
    /// else does.
    #[test]
    fn only_the_two_death_days_suppress_the_rest_of_the_page() {
        for note in LowerRegister::ALL.iter().copied() {
            assert_eq!(
                note.suppresses_the_rest(),
                note == LowerRegister::JUSHINICHI || note == LowerRegister::KUROBI
            );
        }
        assert!(lower_register(Rd(rd_2024(2, 4)), JAPAN).contains(LowerRegister::KUROBI));
        let mut suppressed = 0;
        let mut untouched = 0;
        for offset in 0..400 {
            let set = lower_register(Rd(NEW_YEAR_2024 + offset), JAPAN);
            let has_death =
                set.contains(LowerRegister::KUROBI) || set.contains(LowerRegister::JUSHINICHI);
            let printed = set.as_printed();
            if has_death {
                assert_eq!(printed.len(), 1, "a death day must print alone");
                if set.len() > 1 {
                    suppressed += 1;
                }
            } else {
                assert_eq!(printed, set, "nothing else suppresses anything");
                untouched += 1;
            }
        }
        assert!(
            suppressed > 0,
            "some death day must have suppressed something"
        );
        assert!(untouched > 0);
    }

    #[test]
    fn the_set_of_a_day_agrees_with_the_individual_rules() {
        for offset in 0..120 {
            let rd = Rd(NEW_YEAR_2024 + offset);
            let set = lower_register(rd, JAPAN);
            for note in LowerRegister::ALL.iter().copied() {
                assert_eq!(set.contains(note), holds(note, rd.0), "RD {}", rd.0);
            }
        }
    }

    /// Seven of the twenty-one are auspicious, and 天赦日 is one of them.
    #[test]
    fn seven_of_the_twenty_one_are_auspicious() {
        let auspicious = LowerRegister::ALL
            .iter()
            .copied()
            .filter(|note| note.is_auspicious())
            .count();
        assert_eq!(auspicious, 7);
        assert!(LowerRegister::TENSHANICHI.is_auspicious());
        assert!(!LowerRegister::KUROBI.is_auspicious());
        assert_eq!(LowerRegister::THREE_EVIL_DAYS.len(), 3);
        for note in LowerRegister::THREE_EVIL_DAYS {
            assert!(!note.is_auspicious());
        }
    }
}
