//! 選日 — the "selected days", the annotations that are neither 中段 nor
//! 下段.
//!
//! 選日 (also 雑注) is the residue: the day-choosing rules that sit outside
//! the 十二直 of the middle register and the fixed repertoire of the lower
//! one. Most are 干支 rules pure and simple; a few are keyed to the 節月 and
//! one, 不成就日, is keyed to the lunisolar date.
//!
//! They are also the ones with money attached today. 一粒万倍日 sells
//! wallets, 三隣亡 empties builders' diaries, and 天赦日 fills bridal
//! calendars. That commercial afterlife is worth knowing about when reading
//! any web source on them.
//!
//! # Provenance, honestly
//!
//! **None of this is official.** The National Astronomical Observatory of
//! Japan publishes no 選日; the 暦要項 contains the solar terms, the 雑節 and
//! the holidays and nothing else. The 中段 and 下段 were struck from the
//! official calendar at the Meiji reform as superstition, and the 選日
//! survived through commercial almanacs and, for a while, through illegal
//! おばけ暦.
//!
//! So every table below is sourced to a published reference and then
//! *checked against printed date lists*, which is the only way to catch a
//! transcription error in a rule nobody standardises. The tests carry those
//! lists.
//!
//! # Sources
//!
//! * National Diet Library, 「日本の暦」, 暦注の項 (<https://www.ndl.go.jp/koyomi/chapter3/s6.html>)
//!   for 八専, 十方暮, 天一天上, 一粒万倍日 and their meanings.
//! * National Astronomical Observatory of Japan, 暦Wiki 「節月」, for the
//!   節切り / 月切り distinction and the numbering of the 節月.
//! * 岡田芳朗『現代こよみ読み解き事典』(柏書房, 1993), relayed through
//!   こよみのページ (<https://koyomi8.com/sub/rekicyuu_doc03.html>), for the
//!   rule tables.
//! * Printed date lists used as checks are cited in the tests.

use hc_calendar::Rd;
use hc_seasons::Meridian;

use crate::context::DayContext;
use crate::rules::{AlmanacRule, rule_applies};

/// 一粒万倍日, by 節月, as earthly branches.
///
/// Source: 岡田芳朗『現代こよみ読み解き事典』, as tabulated by the Japanese
/// Wikipedia article 一粒万倍日 and by the almanac publisher 交通図書協会.
/// Checked against the published 2024, 2025 and 2026 date lists in the
/// tests; note in particular that the 亥月 row is 酉・戌 and not 酉・午, which
/// is a common error in circulated tables.
static ICHIRYU_MANBAI: [&[u8]; 12] = [
    &[1, 6],  // 寅月 (立春〜): 丑・午
    &[2, 9],  // 卯月 (啓蟄〜): 寅・酉
    &[0, 3],  // 辰月 (清明〜): 子・卯
    &[3, 4],  // 巳月 (立夏〜): 卯・辰
    &[5, 6],  // 午月 (芒種〜): 巳・午
    &[6, 9],  // 未月 (小暑〜): 午・酉
    &[0, 7],  // 申月 (立秋〜): 子・未
    &[3, 8],  // 酉月 (白露〜): 卯・申
    &[6, 9],  // 戌月 (寒露〜): 午・酉
    &[9, 10], // 亥月 (立冬〜): 酉・戌
    &[0, 11], // 子月 (大雪〜): 子・亥
    &[0, 3],  // 丑月 (小寒〜): 子・卯
];

/// 三隣亡, by 節月, as earthly branches.
///
/// Source: Japanese Wikipedia 三隣亡, こよみのページ 「暦注の話・三隣亡」
/// (2007-08-04). The pattern is a three-month repeat: 亥 for 節月 1, 4, 7 and
/// 10; 寅 for 2, 5, 8 and 11; 午 for 3, 6, 9 and 12.
static SANRINBO: [&[u8]; 12] = [
    &[11],
    &[2],
    &[6],
    &[11],
    &[2],
    &[6],
    &[11],
    &[2],
    &[6],
    &[11],
    &[2],
    &[6],
];

/// 不成就日, by lunisolar month, as days of that month.
///
/// Source: 岡田芳朗『現代こよみ読み解き事典』 via Japanese Wikipedia 不成就日.
/// The one 選日 keyed to the lunisolar date rather than the 節月; a leap
/// month uses the row of the month it follows, and a 29-day month simply
/// never reaches a rule day of 30.
static FUJOJU: [&[u8]; 12] = [
    &[3, 11, 19, 27], // 一月
    &[2, 10, 18, 26], // 二月
    &[1, 9, 17, 25],  // 三月
    &[4, 12, 20, 28], // 四月
    &[5, 13, 21, 29], // 五月
    &[6, 14, 22, 30], // 六月
    &[3, 11, 19, 27], // 七月
    &[2, 10, 18, 26], // 八月
    &[1, 9, 17, 25],  // 九月
    &[4, 12, 20, 28], // 十月
    &[5, 13, 21, 29], // 十一月
    &[6, 14, 22, 30], // 十二月
];

/// The eight 八専 days: the twelve-day 壬子–癸亥 window less its four 間日.
///
/// 壬子 48, 甲寅 50, 乙卯 51, 丁巳 53, 己未 55, 庚申 56, 辛酉 57, 癸亥 59.
static HASSEN_DAYS: [u8; 8] = [48, 50, 51, 53, 55, 56, 57, 59];

/// The four 間日 inside the 八専 window: 癸丑 49, 丙辰 52, 戊午 54, 壬戌 58.
static HASSEN_INTERVAL_DAYS: [u8; 4] = [49, 52, 54, 58];

/// The 犯土 間日: 丁丑, sexagenary 13, between 大犯土 and 小犯土.
static EARTH_TABOO_INTERVAL: [u8; 1] = [13];

/// 庚申, sexagenary 56.
static KOSHIN: [u8; 1] = [56];

/// 甲子, sexagenary 0 — the head of the cycle.
static KINOENE: [u8; 1] = [0];

/// 己巳, sexagenary 5.
static TSUCHINOTO_MI: [u8; 1] = [5];

/// The tiger branch, 寅, index 2.
static TIGER: [u8; 1] = [2];

/// The snake branch, 巳, index 5.
static SNAKE: [u8; 1] = [5];

/// One of the 選日.
///
/// Ordering is the order a 暦注 reference lists them, roughly by how often a
/// modern almanac prints them. The set is a publisher's list, not a fixed
/// vocabulary, which is why it is a table and not an `enum` (ADR 0007): a
/// 選日 this crate has not met is an entry with its own rule.
#[derive(Debug, Clone, Copy)]
pub struct SelectedDay {
    /// A short identifier, the variant name in kebab case.
    pub id: &'static str,
    rule: AlmanacRule,
    japanese_name: &'static str,
    romaji: &'static str,
    english_name: &'static str,
    auspicious: Option<bool>,
    meaning: &'static str,
}

impl PartialEq for SelectedDay {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SelectedDay {}

impl core::hash::Hash for SelectedDay {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for SelectedDay {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SelectedDay {
    /// Listing order: the position in [`SelectedDay::ALL`]. A value that is not
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
    type: SelectedDay,
    id: |entry| entry.id,
    tests: selected_day_tests,
    associated;

    /// All fifteen, in listing order.
    pub const ALL;
    /// The entry with this identifier.
    pub fn by_id;

    entries: {
        /// 一粒万倍日 — "one grain, ten thousand fold". The best-known of the
        /// modern 選日.
        pub const ICHIRYU_MANBAI = Self {
            id: "ichiryu-manbai",
            rule: AlmanacRule::BranchBySolarMonth(&ICHIRYU_MANBAI),
            japanese_name: "一粒万倍日",
            romaji: "ichiryū manbai bi",
            english_name: "one grain yields ten thousand",
            auspicious: Some(true),
            meaning: "a single grain of rice becomes ten thousand: good for beginnings, \
                 opening a shop and sowing — and bad for borrowing, since a debt \
                 multiplies too",
        };
        /// 三隣亡 — the day a building raised will burn three neighbours down.
        pub const SANRINBO = Self {
            id: "sanrinbo",
            rule: AlmanacRule::BranchBySolarMonth(&SANRINBO),
            japanese_name: "三隣亡",
            romaji: "sanrinbō",
            english_name: "ruin of three neighbours",
            auspicious: Some(false),
            meaning: "a house raised today burns, and takes the three neighbouring houses \
                 with it: builders keep the day free",
        };
        /// 不成就日 — the day nothing comes to fruition.
        pub const FUJOJU = Self {
            id: "fujoju",
            rule: AlmanacRule::LunarDayByLunarMonth(&FUJOJU),
            japanese_name: "不成就日",
            romaji: "fujōju bi",
            english_name: "day of no accomplishment",
            auspicious: Some(false),
            meaning: "nothing undertaken today comes to anything",
        };
        /// 八専 — eight of the twelve days from 壬子 to 癸亥.
        pub const HASSEN = Self {
            id: "hassen",
            rule: AlmanacRule::SexagenaryIn(&HASSEN_DAYS),
            japanese_name: "八専",
            romaji: "hassen",
            english_name: "the eight days of doubled influence",
            auspicious: None,
            meaning: "the stem and branch share one of the five phases, so whatever the \
                 day already was is doubled; modern almanacs read it as simply \
                 unlucky",
        };
        /// 八専の間日 — the four days inside that window the taboo skips.
        pub const HASSEN_INTERVAL = Self {
            id: "hassen-interval",
            rule: AlmanacRule::SexagenaryIn(&HASSEN_INTERVAL_DAYS),
            japanese_name: "八専の間日",
            romaji: "hassen no manibi",
            english_name: "the days the eight-day taboo skips",
            auspicious: None,
            meaning: "inside the 八専 window, but exempt from it",
        };
        /// 十方暮 — ten days when the ten directions are shut.
        pub const JIPPOGURE = Self {
            id: "jippogure",
            rule: AlmanacRule::SexagenaryRun {
                first: 20,
                length: 10,
            },
            japanese_name: "十方暮",
            romaji: "jippōgure",
            english_name: "the ten directions are shut",
            auspicious: Some(false),
            meaning: "the phases of stem and branch are in mutual conquest: much labour \
                 and little result",
        };
        /// 天一天上 — the sixteen days 天一神 spends in heaven, when no
        /// direction is blocked.
        pub const TENICHI_TENJO = Self {
            id: "tenichi-tenjo",
            rule: AlmanacRule::SexagenaryRun {
                first: 29,
                length: 16,
            },
            japanese_name: "天一天上",
            romaji: "ten'ichi tenjō",
            english_name: "the wandering god is in heaven",
            auspicious: Some(true),
            meaning: "天一神 has gone up to heaven, so no direction is blocked and travel \
                 is free — but marriage is still avoided",
        };
        /// 庚申 — the night the 三尸 report your sins to heaven; stay awake.
        pub const KOSHIN = Self {
            id: "koshin",
            rule: AlmanacRule::SexagenaryIn(&KOSHIN),
            japanese_name: "庚申",
            romaji: "kōshin",
            english_name: "the metal-monkey vigil",
            auspicious: None,
            meaning: "the three corpse-worms leave a sleeper's body to report his sins to \
                 heaven, so the night is spent awake",
        };
        /// 甲子 — the head of the sexagenary cycle; 大黒天's day.
        pub const KINOENE = Self {
            id: "kinoene",
            rule: AlmanacRule::SexagenaryIn(&KINOENE),
            japanese_name: "甲子",
            romaji: "kinoene",
            english_name: "the head of the sexagenary cycle",
            auspicious: Some(true),
            meaning: "the first day of the sixty; a vigil kept for 大黒天",
        };
        /// 己巳 — 弁財天's day, the strongest money day of the sixty.
        pub const TSUCHINOTO_MI = Self {
            id: "tsuchinoto-mi",
            rule: AlmanacRule::SexagenaryIn(&TSUCHINOTO_MI),
            japanese_name: "己巳",
            romaji: "tsuchinoto mi",
            english_name: "the earth-serpent day of Benzaiten",
            auspicious: Some(true),
            meaning: "the serpent is 弁財天's messenger and 己 is the earth that nurtures \
                 metal, so the day is doubly one for money",
        };
        /// 寅の日 — the tiger goes a thousand leagues and returns; money spent
        /// comes back.
        pub const TIGER_DAY = Self {
            id: "tiger-day",
            rule: AlmanacRule::BranchIn(&TIGER),
            japanese_name: "寅の日",
            romaji: "tora no hi",
            english_name: "day of the tiger",
            auspicious: Some(true),
            meaning: "the tiger goes a thousand leagues and returns a thousand, so money \
                 spent today comes back — but a bride would come back too",
        };
        /// 巳の日 — the serpent is 弁財天's messenger.
        pub const SNAKE_DAY = Self {
            id: "snake-day",
            rule: AlmanacRule::BranchIn(&SNAKE),
            japanese_name: "巳の日",
            romaji: "mi no hi",
            english_name: "day of the serpent",
            auspicious: Some(true),
            meaning: "弁財天's day; good for money and for the arts",
        };
        /// 大犯土 — seven days when 土公神 is in the earth and it must not be
        /// broken.
        pub const GREAT_EARTH_TABOO = Self {
            id: "great-earth-taboo",
            rule: AlmanacRule::SexagenaryRun {
                first: 6,
                length: 7,
            },
            japanese_name: "大犯土",
            romaji: "ōtsuchi",
            english_name: "greater taboo on breaking ground",
            auspicious: Some(false),
            meaning: "土公神 is in the earth: no digging, no well-sinking, no sowing, no \
                 groundbreaking",
        };
        /// 小犯土 — the lesser seven days of the same taboo.
        pub const LESSER_EARTH_TABOO = Self {
            id: "lesser-earth-taboo",
            rule: AlmanacRule::SexagenaryRun {
                first: 14,
                length: 7,
            },
            japanese_name: "小犯土",
            romaji: "kotsuchi",
            english_name: "lesser taboo on breaking ground",
            auspicious: Some(false),
            meaning: "the same taboo in its lesser seven-day form",
        };
        /// 犯土の間日 — the single day, 丁丑, between the two.
        pub const EARTH_TABOO_INTERVAL = Self {
            id: "earth-taboo-interval",
            rule: AlmanacRule::SexagenaryIn(&EARTH_TABOO_INTERVAL),
            japanese_name: "犯土の間日",
            romaji: "bondo no manibi",
            english_name: "the day between the two earth taboos",
            auspicious: None,
            meaning: "the single day of exemption between the two",
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

impl SelectedDay {
    /// This entry's position in [`SelectedDay::ALL`], or `None` for a value
    /// that is not one of its entries, such as a copy whose public `id` was
    /// changed.
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

    /// The rule that fixes this day.
    ///
    /// 八専, 十方暮, 天一天上 and the two 犯土 are *runs* of the sexagenary
    /// cycle rather than lists, which is why [`AlmanacRule::SexagenaryRun`]
    /// exists. The two 犯土 runs abut: 大犯土 closes on 丙子 at 12, the 間日
    /// 丁丑 is 13, and 小犯土 opens on 戊寅 at 14 and closes on 甲申 at 20 —
    /// which is also the first day of 十方暮.
    #[must_use]
    pub const fn rule(self) -> AlmanacRule {
        self.rule
    }

    /// The name in Japanese characters, e.g. `"一粒万倍日"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        self.japanese_name
    }

    /// The reading in Hepburn romaji.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        self.romaji
    }

    /// A one-line English gloss.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        self.english_name
    }

    /// Whether an almanac counts the day auspicious.
    ///
    /// `None` for 八専 and the two 間日, which are neither: 八専 originally
    /// meant that whatever the day already was — lucky or unlucky — was
    /// doubled, and only later degraded into a plain 凶日 in popular
    /// almanacs. This crate keeps the older reading and refuses to flatten
    /// it.
    #[must_use]
    pub const fn is_auspicious(self) -> Option<bool> {
        self.auspicious
    }

    /// What the day is held to mean, in one sentence.
    #[must_use]
    pub const fn meaning(self) -> &'static str {
        self.meaning
    }

    /// Whether this 選日 holds on the day a context describes.
    ///
    /// Never `None`: every 選日 in this module has an established rule. The
    /// return type matches [`rule_applies`] so that the two catalogues can
    /// be handled alike.
    #[must_use]
    pub fn applies_to(self, context: &DayContext) -> Option<bool> {
        rule_applies(self.rule(), context)
    }
}

/// Every 選日 in force on a day, as a bit set.
///
/// A set rather than a list so that the type is `Copy` and needs no
/// allocator, which keeps the crate usable without `alloc`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectedDaySet {
    bits: u16,
}

impl SelectedDaySet {
    /// The empty set.
    pub const EMPTY: Self = Self { bits: 0 };

    /// Whether a 選日 is in the set.
    #[must_use]
    pub const fn contains(self, day: SelectedDay) -> bool {
        match day.index() {
            Some(index) => self.bits & (1 << index) != 0,
            None => false,
        }
    }

    /// Add a 選日 to the set.
    ///
    /// `None` for a value that is not one of [`SelectedDay::ALL`], which has no
    /// place in a set.
    #[must_use]
    pub const fn with(self, day: SelectedDay) -> Option<Self> {
        match day.index() {
            Some(index) => Some(self.with_position(index as usize)),
            None => None,
        }
    }

    /// Add the entry at a position in [`SelectedDay::ALL`].
    const fn with_position(self, position: usize) -> Self {
        Self {
            bits: self.bits | (1 << position),
        }
    }

    /// How many 選日 are in the set.
    #[must_use]
    pub const fn len(self) -> u32 {
        self.bits.count_ones()
    }

    /// Whether the set is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }

    /// The 選日 in the set, in [`SelectedDay::ALL`] order.
    pub fn iter(self) -> impl Iterator<Item = SelectedDay> {
        SelectedDay::ALL
            .iter()
            .copied()
            .filter(move |day| self.contains(*day))
    }
}

impl SelectedDay {}

/// Every 選日 in force on a day, from a context.
#[must_use]
pub fn selected_days_of_context(context: &DayContext) -> SelectedDaySet {
    let mut set = SelectedDaySet::EMPTY;
    for (position, day) in SelectedDay::ALL.iter().enumerate() {
        if day.applies_to(context) == Some(true) {
            set = set.with_position(position);
        }
    }
    set
}

/// Every 選日 in force on a day at a meridian.
#[must_use]
pub fn selected_days(day: Rd, meridian: Meridian) -> SelectedDaySet {
    selected_days_of_context(&DayContext::new(day, meridian))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2025-01-01.
    const NEW_YEAR_2025: i64 = 739_252;

    fn on(rd: i64) -> DayContext {
        DayContext::new(Rd(rd), Meridian::JAPAN)
    }

    fn holds(day: SelectedDay, rd: i64) -> bool {
        day.applies_to(&on(rd)) == Some(true)
    }

    /// RD for a 2025 Gregorian date.
    fn rd_2025(month: u32, day: i64) -> i64 {
        const CUMULATIVE: [i64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        NEW_YEAR_2025 + CUMULATIVE[(month - 1) as usize] + day - 1
    }

    #[test]
    fn every_selected_day_has_a_name_a_reading_and_a_meaning() {
        for day in SelectedDay::ALL.iter().copied() {
            assert!(!day.japanese_name().is_empty());
            assert!(!day.romaji().is_empty());
            assert!(!day.english_name().is_empty());
            assert!(!day.meaning().is_empty());
            assert_ne!(day.rule(), AlmanacRule::Undetermined);
        }
    }

    #[test]
    fn a_value_outside_the_table_has_no_index_and_no_place_in_a_set() {
        let mut stranger = SelectedDay::ALL[0];
        stranger.id = "not-a-selected-day";
        assert_eq!(stranger.index(), None);
        assert!(!SelectedDaySet::EMPTY.contains(stranger));
        assert_eq!(SelectedDaySet::EMPTY.with(stranger), None);
        assert!(stranger > SelectedDay::ALL[0]);
    }

    #[test]
    fn the_indices_are_distinct_and_match_the_listing_order() {
        for (position, day) in SelectedDay::ALL.iter().enumerate() {
            assert_eq!(day.index().map(usize::from), Some(position));
        }
        let mut set = SelectedDaySet::EMPTY;
        assert!(set.is_empty());
        for day in SelectedDay::ALL.iter().copied() {
            assert!(!set.contains(day));
            set = set.with(day).unwrap();
            assert!(set.contains(day));
        }
        assert_eq!(set.len() as usize, SelectedDay::ALL.len());
        assert_eq!(set.iter().count(), SelectedDay::ALL.len());
    }

    /// The published 一粒万倍日 for December 2025 are the 6th, 8th, 9th, 20th
    /// and 21st (arachne.jp 大安カレンダー 2025年12月; JAL SKYWARD+). The 7th
    /// is absent and the 6th present, which is the 節月 boundary: 大雪 fell on
    /// 7 December 2025, so the 6th is still 亥月 (酉・戌 — and the 6th is 己酉)
    /// while the 7th is already 子月 (子・亥 — and the 7th is 庚戌).
    #[test]
    fn the_published_december_2025_grain_days_include_the_solar_term_boundary() {
        let expected = [6, 8, 9, 20, 21];
        for day in 1..=31 {
            let rd = rd_2025(12, day);
            assert_eq!(
                holds(SelectedDay::ICHIRYU_MANBAI, rd),
                expected.contains(&day),
                "2025-12-{day:02}"
            );
        }
    }

    /// Whole published years, as a check on the 節月 table: 2025 holds 63
    /// 一粒万倍日 by this table and 2024 holds 62.
    #[test]
    fn the_published_2025_grain_days_match_month_by_month() {
        let expected: [&[i64]; 12] = [
            &[7, 10, 19, 22, 31],
            &[6, 13, 18, 25],
            &[2, 5, 10, 17, 22, 29],
            &[3, 4, 13, 16, 25, 28],
            &[10, 11, 22, 23],
            &[3, 4, 5, 6, 17, 18, 29, 30],
            &[12, 15, 24, 27],
            &[5, 11, 18, 23, 30],
            &[4, 7, 12, 19, 24],
            &[1, 6, 16, 19, 28, 31],
            &[12, 13, 24, 25],
            &[6, 8, 9, 20, 21],
        ];
        const LENGTHS: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for month in 1..=12u32 {
            for day in 1..=LENGTHS[(month - 1) as usize] {
                assert_eq!(
                    holds(SelectedDay::ICHIRYU_MANBAI, rd_2025(month, day)),
                    expected[(month - 1) as usize].contains(&day),
                    "2025-{month:02}-{day:02}"
                );
            }
        }
    }

    /// The 2025 三隣亡, as published by こよみる: twenty-nine days.
    #[test]
    fn the_published_2025_sanrinbo_days_match_month_by_month() {
        let expected: [&[i64]; 12] = [
            &[13, 25],
            &[11, 23],
            &[10, 22],
            &[3, 7, 19],
            &[1, 6, 18, 30],
            &[14, 26],
            &[12, 24],
            &[5, 10, 22],
            &[3, 18, 30],
            &[16, 28],
            &[14, 26],
            &[11, 23],
        ];
        const LENGTHS: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut total = 0;
        for month in 1..=12u32 {
            for day in 1..=LENGTHS[(month - 1) as usize] {
                let holds_today = holds(SelectedDay::SANRINBO, rd_2025(month, day));
                assert_eq!(
                    holds_today,
                    expected[(month - 1) as usize].contains(&day),
                    "2025-{month:02}-{day:02}"
                );
                total += i32::from(holds_today);
            }
        }
        assert_eq!(total, 29);
    }

    /// 2025 had a leap sixth month, and its 不成就日 fall on the *sixth
    /// month's* rule days — 30 July, 7 and 15 August were 閏6月6日, 14日 and
    /// 22日. That is the whole reason the rule table is indexed by the base
    /// month number.
    #[test]
    fn the_leap_month_uses_the_row_of_the_month_it_follows() {
        assert!(holds(SelectedDay::FUJOJU, rd_2025(7, 30)), "2025-07-30");
        for day in [7, 15] {
            assert!(holds(SelectedDay::FUJOJU, rd_2025(8, day)), "2025-08-{day}");
        }
    }

    /// The published 2025 不成就日 for December are the 1st, 9th, 17th and
    /// 24th (arachne.jp; こよみる).
    #[test]
    fn the_published_december_2025_days_of_no_accomplishment_match() {
        let expected = [1, 9, 17, 24];
        for day in 1..=31 {
            assert_eq!(
                holds(SelectedDay::FUJOJU, rd_2025(12, day)),
                expected.contains(&day),
                "2025-12-{day:02}"
            );
        }
    }

    /// 八専 runs 壬子 to 癸亥. こよみる gives the 2025 windows as 12–23
    /// February, 13–24 April, 12–23 June, 11–22 August, 10–21 October and
    /// 9–20 December.
    #[test]
    fn the_published_2025_hassen_windows_open_and_close_where_they_should() {
        for (month, first, last) in [
            (2u32, 12i64, 23i64),
            (4, 13, 24),
            (6, 12, 23),
            (8, 11, 22),
            (10, 10, 21),
            (12, 9, 20),
        ] {
            let opening = on(rd_2025(month, first));
            let closing = on(rd_2025(month, last));
            assert_eq!(opening.sexagenary().index(), 48, "2025-{month:02}-{first}");
            assert_eq!(closing.sexagenary().index(), 59, "2025-{month:02}-{last}");
            // The window's first and last days are 八専 proper; the four
            // 間日 inside it are not.
            assert!(holds(SelectedDay::HASSEN, rd_2025(month, first)));
            assert!(holds(SelectedDay::HASSEN, rd_2025(month, last)));
        }
    }

    /// Eight 八専 days and four 間日 make twelve, and the two sets never
    /// overlap.
    #[test]
    fn the_eight_and_the_four_partition_the_twelve_day_window() {
        let mut both = 0;
        let mut window = 0;
        for offset in 0..60 {
            let rd = NEW_YEAR_2025 + offset;
            let eight = holds(SelectedDay::HASSEN, rd);
            let four = holds(SelectedDay::HASSEN_INTERVAL, rd);
            assert!(!(eight && four));
            both += i32::from(eight || four);
            window += i32::from(eight);
        }
        assert_eq!(both, 12);
        assert_eq!(window, 8);
    }

    /// 十方暮 runs 甲申 to 癸巳, ten days. こよみる gives the 2025 windows as
    /// 15–24 January, 16–25 March, 15–24 May, 14–23 July, 12–21 September
    /// and 11–20 November.
    #[test]
    fn the_published_2025_ten_directions_windows_are_ten_days_long() {
        for (month, first, last) in [
            (1u32, 15i64, 24i64),
            (3, 16, 25),
            (5, 15, 24),
            (7, 14, 23),
            (9, 12, 21),
            (11, 11, 20),
        ] {
            assert_eq!(on(rd_2025(month, first)).sexagenary().index(), 20);
            assert_eq!(on(rd_2025(month, last)).sexagenary().index(), 29);
            for day in first..=last {
                assert!(
                    holds(SelectedDay::JIPPOGURE, rd_2025(month, day)),
                    "2025-{month:02}-{day}"
                );
            }
            assert!(!holds(SelectedDay::JIPPOGURE, rd_2025(month, first) - 1));
            assert!(!holds(SelectedDay::JIPPOGURE, rd_2025(month, last) + 1));
        }
    }

    /// 天一天上 runs 癸巳 to 戊申, sixteen days, and 十方暮's last day is its
    /// first: both are 癸巳.
    #[test]
    fn the_heavenly_unity_window_opens_where_the_ten_directions_close() {
        let mut overlap = 0;
        let mut length = 0;
        for offset in 0..60 {
            let rd = NEW_YEAR_2025 + offset;
            let heaven = holds(SelectedDay::TENICHI_TENJO, rd);
            length += i32::from(heaven);
            overlap += i32::from(heaven && holds(SelectedDay::JIPPOGURE, rd));
        }
        assert_eq!(length, 16);
        assert_eq!(overlap, 1);
    }

    /// 2025-12-21 is 甲子 — the head of the cycle — and the almanacs print it
    /// as such (arachne.jp 2025年12月).
    #[test]
    fn the_twenty_first_of_december_2025_opened_the_sexagenary_cycle() {
        let rd = rd_2025(12, 21);
        assert_eq!(on(rd).sexagenary().index(), 0);
        assert!(holds(SelectedDay::KINOENE, rd));
        assert!(!holds(SelectedDay::TSUCHINOTO_MI, rd));
    }

    /// 2025-12-26 is 己巳, and therefore also a 巳の日; arachne.jp prints
    /// 「巳の日、己巳の日」 against it.
    #[test]
    fn the_twenty_sixth_of_december_2025_was_the_earth_serpent_day() {
        let rd = rd_2025(12, 26);
        assert_eq!(on(rd).sexagenary().index(), 5);
        assert!(holds(SelectedDay::TSUCHINOTO_MI, rd));
        assert!(holds(SelectedDay::SNAKE_DAY, rd));
        assert!(!holds(SelectedDay::TIGER_DAY, rd));
    }

    /// The published 寅の日 and 巳の日 for December 2025: tigers on the 11th
    /// and 23rd, serpents on the 2nd, 14th and 26th.
    #[test]
    fn the_branch_days_recur_every_twelve_days() {
        for (kind, days) in [
            (SelectedDay::TIGER_DAY, vec![11, 23]),
            (SelectedDay::SNAKE_DAY, vec![2, 14, 26]),
        ] {
            for day in 1..=31 {
                assert_eq!(
                    holds(kind, rd_2025(12, day)),
                    days.contains(&day),
                    "{} on 2025-12-{day:02}",
                    kind.japanese_name()
                );
            }
        }
    }

    /// 庚申 and 甲子 are single positions in the sixty, so each falls six or
    /// seven times a Gregorian year and exactly once in any sixty days.
    #[test]
    fn the_single_sexagenary_days_fall_once_in_sixty() {
        for kind in [
            SelectedDay::KOSHIN,
            SelectedDay::KINOENE,
            SelectedDay::TSUCHINOTO_MI,
            SelectedDay::EARTH_TABOO_INTERVAL,
        ] {
            let count = (0..60)
                .filter(|offset| holds(kind, NEW_YEAR_2025 + offset))
                .count();
            assert_eq!(count, 1, "{}", kind.japanese_name());
        }
    }

    /// The two 犯土 runs and their 間日 make fifteen consecutive positions,
    /// 庚午 through 甲申, with no gap and no overlap.
    #[test]
    fn the_two_earth_taboos_and_their_interval_day_are_contiguous() {
        let mut run = 0;
        for offset in 0..60 {
            let rd = NEW_YEAR_2025 + offset;
            let great = holds(SelectedDay::GREAT_EARTH_TABOO, rd);
            let lesser = holds(SelectedDay::LESSER_EARTH_TABOO, rd);
            let between = holds(SelectedDay::EARTH_TABOO_INTERVAL, rd);
            assert!(u8::from(great) + u8::from(lesser) + u8::from(between) <= 1);
            let position = on(rd).sexagenary().index();
            assert_eq!(
                great || lesser || between,
                (6..=20).contains(&position),
                "at sexagenary {position}"
            );
            run += i32::from(great || lesser || between);
        }
        assert_eq!(run, 15);
    }

    /// The lesser earth taboo closes on 甲申, which is also the day 十方暮
    /// opens — the two abut exactly.
    #[test]
    fn the_lesser_earth_taboo_closes_on_the_day_the_ten_directions_open() {
        let closing = (0..60)
            .map(|offset| NEW_YEAR_2025 + offset)
            .find(|rd| on(*rd).sexagenary().index() == 20)
            .expect("甲申 occurs within sixty days");
        assert!(holds(SelectedDay::LESSER_EARTH_TABOO, closing));
        assert!(holds(SelectedDay::JIPPOGURE, closing));
        assert!(!holds(SelectedDay::LESSER_EARTH_TABOO, closing + 1));
    }

    #[test]
    fn the_set_of_a_day_agrees_with_the_individual_rules() {
        for offset in 0..90 {
            let rd = Rd(NEW_YEAR_2025 + offset);
            let set = selected_days(rd, Meridian::JAPAN);
            for kind in SelectedDay::ALL.iter().copied() {
                assert_eq!(
                    set.contains(kind),
                    holds(kind, rd.0),
                    "{} at RD {}",
                    kind.japanese_name(),
                    rd.0
                );
            }
        }
    }
}
