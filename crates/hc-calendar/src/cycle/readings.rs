//! The ways the sixty names are written.
//!
//! Which of sixty names a year, month, day or hour has is arithmetic, and
//! [`Sexagenary`] answers it. *How the name is spelled* is not: the same
//! 甲子 is *jiǎzǐ* in Beijing, *kinoe-ne* in a Japanese almanac, 갑자 in
//! Seoul and *Giáp Tý* in Hanoi, and no list of spellings is ever finished.
//! A Manchu, Mongolian or Tibetan reading is an entry nobody has written
//! yet, not a case the type failed to foresee. So a reading is a value of
//! [`Reading`], the table of them is [`ALL`], and adding one is adding an
//! entry. Where each reading's spellings come from, and which were checked,
//! is in `docs/systems/sexagenary-cycle.md` in the repository.
//!
//! What *is* fixed is the shape: ten stems and twelve branches, 甲 and 子
//! first. That is the structure of the cycle rather than a fact about any
//! language, so it is in the type. A reading holds exactly ten and exactly
//! twelve, and a reading with nine cannot be written.
//!
//! # Which reading is the default
//!
//! [`Sexagenary`]'s own `stem_name` and `branch_name`, and its `Display`,
//! use [`PINYIN`], toneless Hanyu Pinyin, because it is the form
//! English-language scholarship uses and the only one that is plain ASCII.
//! That is this library's convention, not a claim about the cycle. Which
//! reading a *locale* writes in is `hc-i18n`'s to say, and it says so by
//! naming one entry here.
//!
//! # Ambiguity
//!
//! A reading need not tell the sixty pairs apart. The Japanese 音読み read
//! 甲 and 庚 both as *kō* and 子 and 巳 both as *shi*, so 甲子 and 庚子 are
//! both *kōshi*. [`Reading::is_unambiguous`] says whether a reading has this
//! problem, and it is the reason almanacs print the 訓読み instead.

use super::Sexagenary;

/// One way of spelling the ten stems and twelve branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reading {
    /// The identifier: the language, then the script or romanisation when
    /// the language has more than one, as in `ja-kana`, `ja-kun`, `ja-on`.
    pub id: &'static str,
    /// What to call the reading in English.
    pub english_name: &'static str,
    /// The ten Heavenly Stems, 甲 first.
    pub stems: &'static [&'static str; 10],
    /// The twelve Earthly Branches, 子 first.
    pub branches: &'static [&'static str; 12],
    /// Where the spellings come from.
    pub authority: &'static str,
}

impl Reading {
    /// The stem at a zero-based stem index.
    ///
    /// # Panics
    ///
    /// If `stem_index` is ten or more. [`Sexagenary::stem_index`] never is.
    #[must_use]
    pub const fn stem_at(&self, stem_index: u8) -> &'static str {
        self.stems[stem_index as usize]
    }

    /// The branch at a zero-based branch index.
    ///
    /// # Panics
    ///
    /// If `branch_index` is twelve or more. [`Sexagenary::branch_index`] and
    /// [`DoubleHour::branch_index`](super::DoubleHour::branch_index) never
    /// are.
    #[must_use]
    pub const fn branch_at(&self, branch_index: u8) -> &'static str {
        self.branches[branch_index as usize]
    }

    /// The stem of a cycle position.
    #[must_use]
    pub const fn stem(&self, position: Sexagenary) -> &'static str {
        self.stem_at(position.stem_index())
    }

    /// The branch of a cycle position.
    #[must_use]
    pub const fn branch(&self, position: Sexagenary) -> &'static str {
        self.branch_at(position.branch_index())
    }

    /// The stem and the branch of a cycle position, in that order.
    ///
    /// Returned separately rather than joined, because whether the two are
    /// written adjacent (甲子, 갑자), spaced (Giáp Tý) or hyphenated is a
    /// convention of the text they land in, not of the reading.
    #[must_use]
    pub const fn pair(&self, position: Sexagenary) -> (&'static str, &'static str) {
        (self.stem(position), self.branch(position))
    }

    /// Whether the sixty pairs all read differently from one another.
    ///
    /// False for [`JAPANESE_ON`], where 甲子 and 庚子 are both *kōshi*.
    #[must_use]
    pub fn is_unambiguous(&self) -> bool {
        (0..60i64).all(|first| {
            let pair = self.pair(Sexagenary::from_index(first));
            (first + 1..60).all(|second| pair != self.pair(Sexagenary::from_index(second)))
        })
    }
}

hc_core::catalogue! {
    type: Reading,
    id: |reading| reading.id,
    provenance: |reading| reading.authority,
    tests: reading_catalogue_tests,

    /// Every reading: the characters first, then each language's own script
    /// before its romanisation.
    pub const ALL;
    /// The reading with this identifier.
    pub fn by_id;

    entries: {
        /// The characters, which every language that uses the cycle shares.
        pub const HAN = Reading {
            id: "han",
            english_name: "Han characters",
            stems: &["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"],
            branches: &[
                "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
            ],
            authority: "None of the twenty-two characters was altered by the 1946 Japanese or \
                        1956 Chinese simplifications, so Chinese, Japanese, Korean and \
                        Vietnamese sources print one column",
        };
        /// Hanyu Pinyin without tone marks: the form English-language
        /// scholarship uses, and this library's default.
        pub const PINYIN = Reading {
            id: "zh-pinyin",
            english_name: "Hanyu Pinyin, toneless",
            stems: &["jia", "yi", "bing", "ding", "wu", "ji", "geng", "xin", "ren", "gui"],
            branches: &[
                "zi", "chou", "yin", "mao", "chen", "si", "wu", "wei", "shen", "you", "xu", "hai",
            ],
            authority: "汉语拼音方案 (1958), tone marks omitted",
        };
        /// Hanyu Pinyin with tone marks.
        pub const PINYIN_TONED = Reading {
            id: "zh-pinyin-toned",
            english_name: "Hanyu Pinyin, with tones",
            stems: &["jiǎ", "yǐ", "bǐng", "dīng", "wù", "jǐ", "gēng", "xīn", "rén", "guǐ"],
            branches: &[
                "zǐ", "chǒu", "yín", "mǎo", "chén", "sì", "wǔ", "wèi", "shēn", "yǒu", "xū", "hài",
            ],
            authority: "汉语拼音方案 (1958)",
        };
        /// The Japanese 訓読み in kana, which is what an almanac prints.
        ///
        /// Each stem is its phase plus 兄 (*e*, elder brother, yang) or 弟
        /// (*to*, younger brother, yin): 甲 is 木の兄, *ki-no-e*, yang wood,
        /// and 癸 is 水の弟, *mizu-no-to*, yin water. The cycle's own
        /// Japanese name, *eto*, is that 兄弟 pair. The branches read as the
        /// animals, 子 read *ne* being the rat, though the branch characters
        /// are not the animal characters.
        pub const JAPANESE_KANA = Reading {
            id: "ja-kana",
            english_name: "Japanese kun readings, in kana",
            stems: &[
                "きのえ", "きのと", "ひのえ", "ひのと", "つちのえ", "つちのと", "かのえ", "かのと",
                "みずのえ", "みずのと",
            ],
            branches: &[
                "ね", "うし", "とら", "う", "たつ", "み", "うま", "ひつじ", "さる", "とり", "いぬ",
                "い",
            ],
            authority: "The 訓読み as Japanese almanacs (暦注) print them; each stem is a \
                        phase plus 兄 or 弟, which the tests check",
        };
        /// The Japanese 訓読み, Hepburn-romanised.
        pub const JAPANESE_KUN = Reading {
            id: "ja-kun",
            english_name: "Japanese kun readings, romanised",
            stems: &[
                "kinoe", "kinoto", "hinoe", "hinoto", "tsuchinoe", "tsuchinoto", "kanoe", "kanoto",
                "mizunoe", "mizunoto",
            ],
            branches: &[
                "ne", "ushi", "tora", "u", "tatsu", "mi", "uma", "hitsuji", "saru", "tori", "inu",
                "i",
            ],
            authority: "Hepburn romanisation of the kana reading, which the tests check",
        };
        /// The Japanese 音読み, Hepburn-romanised.
        ///
        /// These collide where the Mandarin readings do not: 甲 and 庚 are
        /// both *kō*, 己 and 癸 both *ki*, 子 and 巳 both *shi*, 辰 and 申
        /// both *shin*. That is why almanacs print the 訓読み, and why this
        /// is the one reading for which [`Reading::is_unambiguous`] is false.
        pub const JAPANESE_ON = Reading {
            id: "ja-on",
            english_name: "Japanese on readings, romanised",
            stems: &["kō", "otsu", "hei", "tei", "bo", "ki", "kō", "shin", "jin", "ki"],
            branches: &[
                "shi", "chū", "in", "bō", "shin", "shi", "go", "bi", "shin", "yū", "jutsu", "gai",
            ],
            authority: "Hepburn romanisation of the Sino-Japanese readings",
        };
        /// Korean, in Hangul.
        pub const HANGUL = Reading {
            id: "ko-hangul",
            english_name: "Korean, in Hangul",
            stems: &["갑", "을", "병", "정", "무", "기", "경", "신", "임", "계"],
            branches: &[
                "자", "축", "인", "묘", "진", "사", "오", "미", "신", "유", "술", "해",
            ],
            authority: "표준국어대사전 (National Institute of Korean Language), entries 십간 and 십이지",
        };
        /// Korean, in the Revised Romanization.
        pub const KOREAN_REVISED = Reading {
            id: "ko-latn",
            english_name: "Korean, Revised Romanization",
            stems: &["gap", "eul", "byeong", "jeong", "mu", "gi", "gyeong", "sin", "im", "gye"],
            branches: &[
                "ja", "chuk", "in", "myo", "jin", "sa", "o", "mi", "sin", "yu", "sul", "hae",
            ],
            authority: "국어의 로마자 표기법 (Revised Romanization of Korean, 2000) applied to \
                        the Hangul",
        };
        /// Vietnamese *can chi*, in quốc ngữ.
        ///
        /// The Vietnamese cycle is the same sixty pairs. What differs is the
        /// animals, which are `hc-i18n`'s to say: 丑 is the buffalo and 卯
        /// the cat.
        pub const VIETNAMESE = Reading {
            id: "vi",
            english_name: "Vietnamese can chi",
            stems: &["Giáp", "Ất", "Bính", "Đinh", "Mậu", "Kỷ", "Canh", "Tân", "Nhâm", "Quý"],
            branches: &[
                "Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ", "Ngọ", "Mùi", "Thân", "Dậu", "Tuất", "Hợi",
            ],
            authority: "The can chi as Vietnamese almanacs (lịch vạn niên) print them; Mão is \
                        the standard form of 卯 and Mẹo the southern one",
        };
    }
}

#[cfg(test)]
mod tests {
    use super::super::{FIVE_PHASES_JAPANESE_KUN, Polarity};
    use super::*;

    /// A reading with its 甲子 and its 癸亥.
    type Anchor = (
        &'static Reading,
        (&'static str, &'static str),
        (&'static str, &'static str),
    );

    /// 甲子 and 癸亥, the first and the last pair, in every reading.
    ///
    /// One anchor at each end catches a table typed in the wrong order as
    /// well as one typed wrong. That the fifty-eight between them are in
    /// place is the shape of the table, which the type fixes.
    const ANCHORS: &[Anchor] = &[
        (&HAN, ("甲", "子"), ("癸", "亥")),
        (&PINYIN, ("jia", "zi"), ("gui", "hai")),
        (&PINYIN_TONED, ("jiǎ", "zǐ"), ("guǐ", "hài")),
        (&JAPANESE_KANA, ("きのえ", "ね"), ("みずのと", "い")),
        (&JAPANESE_KUN, ("kinoe", "ne"), ("mizunoto", "i")),
        (&JAPANESE_ON, ("kō", "shi"), ("ki", "gai")),
        (&HANGUL, ("갑", "자"), ("계", "해")),
        (&KOREAN_REVISED, ("gap", "ja"), ("gye", "hae")),
        (&VIETNAMESE, ("Giáp", "Tý"), ("Quý", "Hợi")),
    ];

    #[test]
    fn every_reading_is_anchored_at_both_ends_of_the_cycle() {
        let jia_zi = Sexagenary::from_index(0);
        let gui_hai = Sexagenary::from_index(59);
        for (reading, first, last) in ANCHORS {
            assert_eq!(reading.pair(jia_zi), *first, "{}", reading.id);
            assert_eq!(reading.pair(gui_hai), *last, "{}", reading.id);
        }
        // A reading added without an anchor is a reading nobody has checked.
        assert_eq!(ANCHORS.len(), ALL.len());
        for reading in ALL {
            assert!(
                ANCHORS
                    .iter()
                    .any(|(anchored, _, _)| anchored.id == reading.id),
                "{} has no anchor",
                reading.id
            );
        }
    }

    #[test]
    fn no_reading_has_an_empty_name() {
        for reading in ALL {
            for name in reading.stems.iter().chain(reading.branches.iter()) {
                assert!(!name.is_empty(), "{} has an empty name", reading.id);
            }
        }
    }

    #[test]
    fn only_the_japanese_on_readings_fail_to_tell_the_sixty_pairs_apart() {
        for reading in ALL {
            assert_eq!(
                reading.is_unambiguous(),
                reading.id != JAPANESE_ON.id,
                "{}",
                reading.id
            );
        }
        // The collision, spelled out: 甲子 is index 0 and 庚子 is index 36.
        let jia_zi = Sexagenary::from_index(0);
        let geng_zi = Sexagenary::from_index(36);
        assert_eq!(HAN.pair(geng_zi), ("庚", "子"));
        assert_eq!(JAPANESE_ON.pair(jia_zi), JAPANESE_ON.pair(geng_zi));
        assert_ne!(JAPANESE_KUN.pair(jia_zi), JAPANESE_KUN.pair(geng_zi));
    }

    #[test]
    fn the_japanese_kun_readings_spell_out_the_phase_and_the_polarity() {
        for (index, reading) in JAPANESE_KUN.stems.iter().enumerate() {
            let phase = FIVE_PHASES_JAPANESE_KUN[index / 2];
            let polarity = Polarity::of_index(index as u8).japanese_kun();
            assert_eq!(*reading, format!("{phase}no{polarity}"), "stem {index}");
        }
    }

    #[test]
    fn the_kana_and_the_romanised_kun_readings_say_the_same_thing() {
        // The two Japanese kun tables are typed independently; a syllable
        // table turns one into the other, so a slip in either fails here.
        fn hepburn(kana: &str) -> String {
            kana.chars()
                .map(|kana| match kana {
                    'き' => "ki",
                    'の' => "no",
                    'え' => "e",
                    'と' => "to",
                    'ひ' => "hi",
                    'つ' => "tsu",
                    'ち' => "chi",
                    'か' => "ka",
                    'み' => "mi",
                    'ず' => "zu",
                    'ね' => "ne",
                    'う' => "u",
                    'し' => "shi",
                    'ら' => "ra",
                    'た' => "ta",
                    'い' => "i",
                    'ぬ' => "nu",
                    'さ' => "sa",
                    'る' => "ru",
                    'り' => "ri",
                    'ま' => "ma",
                    'じ' => "ji",
                    other => panic!("no romanisation for {other}"),
                })
                .collect()
        }
        for (kana, kun) in JAPANESE_KANA.stems.iter().zip(JAPANESE_KUN.stems.iter()) {
            assert_eq!(hepburn(kana), *kun);
        }
        for (kana, kun) in JAPANESE_KANA
            .branches
            .iter()
            .zip(JAPANESE_KUN.branches.iter())
        {
            assert_eq!(hepburn(kana), *kun);
        }
    }

    #[test]
    fn the_toneless_pinyin_is_the_toned_pinyin_with_its_marks_removed() {
        fn strip(toned: &str) -> String {
            toned
                .chars()
                .map(|letter| match letter {
                    'ā' | 'á' | 'ǎ' | 'à' => 'a',
                    'ē' | 'é' | 'ě' | 'è' => 'e',
                    'ī' | 'í' | 'ǐ' | 'ì' => 'i',
                    'ō' | 'ó' | 'ǒ' | 'ò' => 'o',
                    'ū' | 'ú' | 'ǔ' | 'ù' => 'u',
                    other => other,
                })
                .collect()
        }
        for (toned, plain) in PINYIN_TONED.stems.iter().zip(PINYIN.stems.iter()) {
            assert_eq!(strip(toned), *plain);
        }
        for (toned, plain) in PINYIN_TONED.branches.iter().zip(PINYIN.branches.iter()) {
            assert_eq!(strip(toned), *plain);
        }
    }

    #[test]
    fn the_index_and_the_position_accessors_agree() {
        for index in 0..60i64 {
            let position = Sexagenary::from_index(index);
            for reading in ALL {
                assert_eq!(
                    reading.stem(position),
                    reading.stem_at(position.stem_index())
                );
                assert_eq!(
                    reading.branch(position),
                    reading.branch_at(position.branch_index())
                );
            }
        }
    }
}
