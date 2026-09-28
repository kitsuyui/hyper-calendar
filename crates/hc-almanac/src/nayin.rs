//! 納音 — the thirty sounds of the sexagenary cycle.
//!
//! 納音 (Japanese *nacchin*, Chinese *nàyīn*) pairs the sixty 干支 in
//! order, 甲子 and 乙丑 first, and gives each pair one of the five phases
//! and a name that qualifies it: 甲子・乙丑 are 海中金, "gold in the sea",
//! 丙寅・丁卯 are 爐中火, "fire in the furnace". Its phase is not the
//! phase of the stem: 甲 is wood, and 甲子 is metal. A person's 納音 is
//! that of their birth year, and the per-person forms of 五墓日 read its
//! phase ([`crate::lower_register::GraveDays`]).
//! `docs/systems/japanese-almanac-notes.md` describes it with the rest of
//! the almanac's notes.
//!
//! # The table
//!
//! Japanese Wikipedia's 「納音」 tabulates the thirty with their readings
//! (`wikipedia-ja-nacchin`, read 2026-09-29), and 『三命通會』 卷一 gives
//! each pair a section headed by its 干支 and name (Wikisource's
//! transcription, `sanming-tonghui-nayin`, read 2026-09-29). The two agree
//! on every pair and on every phase; they write some names differently —
//! 路傍土 and 路旁土, 沙中金 and 砂中金, 金箔金 and 金泊金, 柘榴木 and
//! 石榴木 — and each is carried as it writes them, in [`namings`].
//! 精選版日本国語大辞典 (「納音」, `kotobank-nacchin`, read 2026-09-29)
//! confirms the first two pairs.
//!
//! # Which year
//!
//! [`Nayin::of`] takes a sexagenary position. Which year is someone's
//! birth year is the caller's: the four-pillar year turns at 立春, the
//! lunisolar year at 正月朔日, and the greeting-card year on 1 January, as
//! `hc_calendar::cycle` sets out. こよみる's table lists the Gregorian
//! years with their 納音 phase and does not say (`koyomil-gomunichi`).

use hc_calendar::Rd;
use hc_calendar::cycle::{FivePhase, Sexagenary, sexagenary_day};
use hc_calendar::shape::Naming;

/// How many 納音 there are: one for each pair of the sixty.
pub const NAYIN_COUNT: u8 = 30;

/// The phase of each 納音, by position: the last character of its name
/// in both [`namings::JAPANESE`] and [`namings::CHINESE`].
const PHASES: [FivePhase; 30] = {
    use FivePhase::{Earth, Fire, Metal, Water, Wood};
    [
        Metal, Fire, Wood, Earth, Metal, Fire, Water, Earth, Metal, Wood, Water, Earth, Fire, Wood,
        Water, Metal, Fire, Wood, Earth, Metal, Fire, Water, Earth, Metal, Wood, Water, Earth,
        Fire, Wood, Water,
    ]
};

/// The ways the thirty are named. See [`hc_calendar::shape::Naming`].
pub mod namings {
    use hc_calendar::shape::Naming;

    hc_core::catalogue! {
        type: Naming<30>,
        id: |naming| naming.id,
        provenance: |naming| naming.authority,
        tests: nayin_naming_tests,

        /// Every naming this crate ships.
        pub const ALL;
        /// The naming with this identifier.
        pub fn by_id;

        entries: {
            /// The names as Japanese Wikipedia writes them, e.g. `"海中金"`.
            pub const JAPANESE = Naming {
                id: "ja",
                english_name: "Japanese",
                names: &[
                    "海中金", "爐中火", "大林木", "路傍土", "釼鋒金", "山頭火", "澗下水",
                    "城頭土", "白鑞金", "楊柳木", "井泉水", "屋上土", "霹靂火", "松柏木",
                    "長流水", "沙中金", "山下火", "平地木", "壁上土", "金箔金", "覆燈火",
                    "天河水", "大駅土", "釵釧金", "桑柘木", "大溪水", "沙中土", "天上火",
                    "柘榴木", "大海水",
                ],
                authority: "Japanese Wikipedia, 「納音」, read 2026-09-29",
            };
            /// Their readings in kana, e.g. `"かいちゅうきん"`.
            pub const KANA = Naming {
                id: "ja-kana",
                english_name: "Japanese readings, in kana",
                names: &[
                    "かいちゅうきん", "ろちゅうか", "たいりんぼく", "ろぼうど",
                    "じんぼうきん", "さんとうか", "かんかすい", "じょうとうど",
                    "はくろうきん", "ようりゅうぼく", "せいせんすい", "おくじょうど",
                    "へきれきか", "しょうはくぼく", "ちょうりゅうすい", "さちゅうきん",
                    "さんげか", "へいちぼく", "へきじょうど", "きんぱくきん", "ふくとうか",
                    "てんがすい", "たいえきど", "さいせんきん", "そうしゃくもく",
                    "だいけいすい", "さちゅうど", "てんじょうか", "ざくろぼく",
                    "たいかいすい",
                ],
                authority: "Japanese Wikipedia, 「納音」, read 2026-09-29",
            };
            /// Those readings in Hepburn romaji, e.g. `"kaichūkin"`.
            pub const ROMAJI = Naming {
                id: "ja-latn",
                english_name: "Japanese readings, romanised",
                names: &[
                    "kaichūkin", "rochūka", "tairinboku", "robōdo", "jinbōkin", "santōka",
                    "kankasui", "jōtōdo", "hakurōkin", "yōryūboku", "seisensui", "okujōdo",
                    "hekirekika", "shōhakuboku", "chōryūsui", "sachūkin", "sangeka",
                    "heichiboku", "hekijōdo", "kinpakukin", "fukutōka", "tengasui",
                    "taiekido", "saisenkin", "sōshakumoku", "daikeisui", "sachūdo",
                    "tenjōka", "zakuroboku", "taikaisui",
                ],
                authority: "Hepburn romanisation of the kana",
            };
            /// The names as 『三命通會』 heads its sections, in Wikisource's
            /// simplified transcription, e.g. `"炉中火"`.
            pub const CHINESE = Naming {
                id: "zh-Hans",
                english_name: "Chinese, simplified",
                names: &[
                    "海中金", "炉中火", "大林木", "路旁土", "剑锋金", "山头火", "涧下水",
                    "城头土", "白蜡金", "杨柳木", "井泉水", "屋上土", "霹雳火", "松柏木",
                    "长流水", "砂中金", "山下火", "平地木", "壁上土", "金泊金", "覆灯火",
                    "天河水", "大驿土", "钗钏金", "桑柘木", "大溪水", "砂中土", "天上火",
                    "石榴木", "大海水",
                ],
                authority: "『三命通會』 卷一, 「释六十甲子性质吉凶」, as Wikisource transcribes it, read 2026-09-29",
            };
        }
    }
}

/// One of the thirty 納音.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nayin(u8);

impl Nayin {
    /// The 納音 of a sexagenary position: the pair it belongs to.
    #[must_use]
    pub const fn of(sexagenary: Sexagenary) -> Self {
        Self(sexagenary.index() / 2)
    }

    /// The 納音 of a day's sexagenary position.
    #[must_use]
    pub const fn of_day(day: Rd) -> Self {
        Self::of(sexagenary_day(day))
    }

    /// The 納音 at a zero-based position, 0 for 海中金 through 29 for
    /// 大海水; `None` from 30 on.
    #[must_use]
    pub const fn from_index(index: u8) -> Option<Self> {
        if index < NAYIN_COUNT {
            Some(Self(index))
        } else {
            None
        }
    }

    /// All thirty, in cycle order.
    #[must_use]
    pub const fn all() -> [Self; 30] {
        let mut all = [Self(0); 30];
        let mut index = 0;
        while index < 30 {
            all[index] = Self(index as u8);
            index += 1;
        }
        all
    }

    /// The zero-based position, 0 for 海中金.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// The two sexagenary positions that carry it, the yang one first.
    #[must_use]
    pub const fn pair(self) -> [Sexagenary; 2] {
        let first = self.0 as i64 * 2;
        [
            Sexagenary::from_index(first),
            Sexagenary::from_index(first + 1),
        ]
    }

    /// Its phase, which is not its stems' phase: 海中金 is metal though
    /// 甲 is wood.
    #[must_use]
    pub const fn phase(self) -> FivePhase {
        PHASES[self.0 as usize]
    }

    /// Its name in one naming.
    #[must_use]
    pub const fn name(self, naming: &Naming<30>) -> &'static str {
        naming.names[self.0 as usize]
    }

    /// The Japanese name, e.g. `"海中金"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        self.name(&namings::JAPANESE)
    }

    /// The reading in Hepburn romaji, e.g. `"kaichūkin"`.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        self.name(&namings::ROMAJI)
    }
}

#[cfg(test)]
mod tests {
    use hc_calendar::cycle::{readings::HAN, sexagenary_year_from_gregorian_year};

    use super::*;

    fn pair_name(nayin: Nayin) -> String {
        nayin
            .pair()
            .iter()
            .map(|sign| sign_name(*sign))
            .collect::<Vec<_>>()
            .join("・")
    }

    fn sign_name(sign: Sexagenary) -> String {
        format!("{}{}", HAN.stem(sign), HAN.branch(sign))
    }

    /// Japanese Wikipedia's table, row by row: 海中金 甲子・乙丑 to 大海水
    /// 壬戌・癸亥; and 『三命通會』's section headings, which name the same
    /// pairs (戊寅己卯 城头土 is headed 戊寅已卯 there, with 已 for 己).
    #[test]
    fn the_pairs_are_the_tables() {
        let rows = [
            ("海中金", "甲子・乙丑"),
            ("爐中火", "丙寅・丁卯"),
            ("大林木", "戊辰・己巳"),
            ("路傍土", "庚午・辛未"),
            ("釼鋒金", "壬申・癸酉"),
            ("山頭火", "甲戌・乙亥"),
            ("澗下水", "丙子・丁丑"),
            ("城頭土", "戊寅・己卯"),
            ("白鑞金", "庚辰・辛巳"),
            ("楊柳木", "壬午・癸未"),
            ("井泉水", "甲申・乙酉"),
            ("屋上土", "丙戌・丁亥"),
            ("霹靂火", "戊子・己丑"),
            ("松柏木", "庚寅・辛卯"),
            ("長流水", "壬辰・癸巳"),
            ("沙中金", "甲午・乙未"),
            ("山下火", "丙申・丁酉"),
            ("平地木", "戊戌・己亥"),
            ("壁上土", "庚子・辛丑"),
            ("金箔金", "壬寅・癸卯"),
            ("覆燈火", "甲辰・乙巳"),
            ("天河水", "丙午・丁未"),
            ("大駅土", "戊申・己酉"),
            ("釵釧金", "庚戌・辛亥"),
            ("桑柘木", "壬子・癸丑"),
            ("大溪水", "甲寅・乙卯"),
            ("沙中土", "丙辰・丁巳"),
            ("天上火", "戊午・己未"),
            ("柘榴木", "庚申・辛酉"),
            ("大海水", "壬戌・癸亥"),
        ];
        for (nayin, (name, pair)) in Nayin::all().into_iter().zip(rows) {
            assert_eq!(nayin.japanese_name(), name);
            assert_eq!(pair_name(nayin), pair);
            for sign in nayin.pair() {
                assert_eq!(Nayin::of(sign), nayin);
            }
        }
    }

    /// The phase is the last character of the name, in both namings.
    #[test]
    fn the_phase_is_the_last_character_in_both_namings() {
        for nayin in Nayin::all() {
            for naming in [&namings::JAPANESE, &namings::CHINESE] {
                let last = nayin.name(naming).chars().last().map(String::from);
                assert_eq!(last.as_deref(), Some(nayin.phase().cjk()), "{}", nayin.0);
            }
        }
        assert_eq!(
            Nayin::of(Sexagenary::from_index(0)).phase(),
            FivePhase::Metal
        );
        assert_eq!(Nayin::from_index(30), None);
    }

    /// こよみる's table of birth years and their 納音 phases, 1921–1998
    /// (`koyomil-gomunichi`), by the Gregorian year's 干支: 1921 辛酉 wood,
    /// 1922 壬戌 water, 1924 甲子 metal, 1926 丙寅 fire, 1928 戊辰 wood,
    /// 1956 丙申 fire, 1958 戊戌 wood, 1960 庚子 earth, 1962 壬寅 metal,
    /// 1991 辛未 earth, 1992 壬申 metal, 1994 甲戌 fire, 1996 丙子 water,
    /// 1998 戊寅 earth.
    #[test]
    fn the_published_birth_years_have_their_phases() {
        use FivePhase::{Earth, Fire, Metal, Water, Wood};
        for (year, phase) in [
            (1921, Wood),
            (1922, Water),
            (1924, Metal),
            (1926, Fire),
            (1928, Wood),
            (1956, Fire),
            (1958, Wood),
            (1960, Earth),
            (1962, Metal),
            (1991, Earth),
            (1992, Metal),
            (1994, Fire),
            (1996, Water),
            (1998, Earth),
        ] {
            let sign = sexagenary_year_from_gregorian_year(year);
            assert_eq!(Nayin::of(sign).phase(), phase, "{year}");
        }
    }

    /// 21 December 2025 is 甲子, so 海中金.
    #[test]
    fn a_day_has_the_nayin_of_its_sign() {
        let day = Rd(739_606);
        assert_eq!(sexagenary_day(day).index(), 0);
        assert_eq!(Nayin::of_day(day).japanese_name(), "海中金");
        assert_eq!(Nayin::of_day(day).romaji(), "kaichūkin");
    }
}
