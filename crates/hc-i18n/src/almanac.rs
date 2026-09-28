//! What a locale calls the annotations of the Japanese almanac, as data:
//! 六曜, 二十八宿, 九星, 十二直, 納音, the 暦注下段, the 選日 and the modern
//! combinations of them, for the lines `hc_almanac_day` writes.
//!
//! The annotations are `hc-almanac`'s, which computes them and gives each
//! its own Japanese name and its Hepburn reading. This module is the
//! locale's half, as [`crate::names`] is for a calendar's months: the name
//! each carried locale gives an annotation, looked up by the annotation's
//! kind and its position in a cycle or its identifier in a table. The
//! facade's vocabulary test holds the two crates together: every term
//! here is one `hc-almanac` computes, and every term it computes is named
//! in Japanese.
//!
//! # What is carried
//!
//! * **`ja`**: the names the almanacs print, which are `hc-almanac`'s own.
//!   They were compared on 2026-09-27 with the National Astronomical
//!   Observatory of Japan's 暦Wiki pages 十二直 (建 … 納 開 閉) and 星宿
//!   (角 … 軫) [nao-rekiwiki-junichoku, nao-rekiwiki-28shuku], and with the
//!   National Diet Library's 「日本の暦」, 吉凶を表す言葉① 六曜, ③ 下段 and
//!   ④ その他 [ndl-koyomi-rokuyo, ndl-koyomi-gedan, ndl-koyomi-sonota],
//!   which print the six 六曜, sixteen of the 暦注下段 and the 選日 as here
//!   — except 狼藉日, which that page writes 狼籍日; the almanacs' and
//!   `hc-almanac`'s 狼藉 is kept. The combinations are the names
//!   `hc-almanac` gives the marketing categories it labels as commerce.
//! * **`en`**: English writes these Japanese terms in their Hepburn
//!   romanisation — 納音 included, as `hc-almanac` reads them from
//!   Japanese Wikipedia's kana — as English Wikipedia's "Rokuyō" names the six days
//!   Senshō, Tomobiki, Senbu, Butsumetsu, Taian and Shakkō
//!   [wikipedia-rokuyo] and as CLDR's English names the Japanese eras; the forms
//!   are `hc-almanac`'s readings as it writes them, lower case. The
//!   mansions are the exception: English names the twenty-eight asterisms,
//!   Horn to Chariot, as English Wikipedia's "Twenty-Eight Mansions" table
//!   does [wikipedia-twenty-eight-mansions] and `hc-almanac`'s English
//!   naming carries. The
//!   combinations have no reading in `hc-almanac` and no English name.
//!
//! * **`zh-Hans`**: the 納音 alone, as 『三命通會』 卷一 heads its sections
//!   in Wikisource's simplified transcription (炉中火, 路旁土, 石榴木),
//!   read 2026-09-29 [sanming-tonghui-nayin], which `hc-almanac`'s
//!   Chinese naming carries.
//!
//! No other locale has a table, and Chinese has none for the other
//! cycles. Chinese and Korean almanacs name some of them, but no source
//! for their names was read, and nothing here is translated. The sexagenary day is not here either: [`crate::names`]
//! writes it in each locale's reading.
//!
//! # Locales
//!
//! [`almanac_name`] walks [`Locale::fallback`] and takes the first table in
//! the chain that names the term, as [`crate::territories`] does, and
//! [`name_or_fallback`] adds the rule every line writer follows: a locale
//! with no name for a term has it from English, and a term English does
//! not name from Japanese, the almanac's own language, which names every
//! term; a request for no locale in particular, the facade's `native`,
//! has it from Japanese first.

use crate::locale::Locale;

/// 六曜, a cycle of six from 先勝.
pub const ROKUYO: &str = "rokuyo";
/// 二十八宿, a cycle of twenty-eight from 角. The twenty-seven of 宿曜道 are
/// the same asterisms with 牛 left out, and are named from this cycle.
pub const MANSION: &str = "mansion";
/// 九星, a cycle of nine from 一白水星, for the year's, the month's and the
/// day's star alike.
pub const NINE_STAR: &str = "nine-star";
/// 十二直, a cycle of twelve from 建.
pub const TWELVE_DIRECT: &str = "twelve-direct";
/// 納音, a cycle of thirty from 海中金, one for each pair of the sixty.
pub const NAYIN: &str = "nayin";
/// The 暦注下段, by identifier.
pub const LOWER_REGISTER: &str = "lower-register";
/// The 選日, by identifier.
pub const SELECTED_DAY: &str = "selected-day";
/// The modern combinations of 六曜, 下段 and 選日, by identifier.
pub const COMBINATION: &str = "combination";

/// The cycles, with how many positions each has.
pub const CYCLES: &[(&str, usize)] = &[
    (ROKUYO, 6),
    (MANSION, 28),
    (NINE_STAR, 9),
    (TWELVE_DIRECT, 12),
    (NAYIN, 30),
];

/// The tables whose terms are addressed by identifier.
pub const TABLES: &[&str] = &[LOWER_REGISTER, SELECTED_DAY, COMBINATION];

/// Where the names come from, for a `source` cell.
pub const SOURCE: &str = "hc-almanac's names, compared with NAOJ 暦Wiki (十二直, 星宿) and NDL 「日本の暦」 \
     (六曜, 下段, その他), read 2026-09-27; 納音 from Japanese Wikipedia and 『三命通會』, read \
     2026-09-29; English as English Wikipedia's Rokuyō and Twenty-Eight Mansions write them";

/// One term of the almanac: a position in a cycle, from zero, or an
/// identifier in a table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Term<'a> {
    /// The zero-based position in one of the [`CYCLES`].
    Position(usize),
    /// The identifier of an entry of one of the [`TABLES`], as
    /// `hc-almanac` spells it: `tenshanichi`, `ichiryu-manbai`.
    Id(&'a str),
}

/// One locale's names for the almanac's terms.
#[derive(Debug, Clone, Copy)]
pub struct AlmanacNames {
    /// The BCP 47 tag, spelled as [`crate::data::LOCALES`] spells it.
    pub tag: &'static str,
    /// Each cycle's names, in cycle order, by kind.
    pub cycles: &'static [(&'static str, &'static [&'static str])],
    /// Each named term: its kind, its identifier and its name.
    pub terms: &'static [(&'static str, &'static str, &'static str)],
}

impl AlmanacNames {
    /// The name this table gives a term of a kind, if it has one.
    #[must_use]
    pub fn name_of(&self, kind: &str, term: Term<'_>) -> Option<&'static str> {
        match term {
            Term::Position(position) => self
                .cycles
                .iter()
                .find(|(cycle, _)| *cycle == kind)
                .and_then(|(_, names)| names.get(position).copied()),
            Term::Id(id) => self
                .terms
                .iter()
                .find(|(table, entry, _)| *table == kind && *entry == id)
                .map(|(_, _, name)| *name),
        }
    }
}

/// A term's name and the tag of the table that gave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlmanacName {
    /// What the locale calls the term.
    pub name: &'static str,
    /// The tag of the table that answered: `ja` for a request for `ja-JP`.
    pub tag: &'static str,
}

/// The table for a data tag, spelled as [`AlmanacNames::tag`] is.
#[must_use]
pub fn table(tag: &str) -> Option<&'static AlmanacNames> {
    VOCABULARIES.iter().find(|table| table.tag == tag)
}

/// What `locale` calls a term, from the first table in its fallback chain
/// that names it; `None` when none does, the root locale included.
#[must_use]
pub fn almanac_name(locale: &Locale, kind: &str, term: Term<'_>) -> Option<AlmanacName> {
    locale.fallback().find_map(|candidate| {
        let rendered = candidate.rendered()?;
        let table = table(rendered.as_str())?;
        table.name_of(kind, term).map(|name| AlmanacName {
            name,
            tag: table.tag,
        })
    })
}

/// A term's name as every line writer gives it: the locale's, where its
/// chain has one; else English's; else Japanese's. A `locale` of `None`
/// asks for none in particular, and Japanese then answers first. The
/// answer is `None` only for a term no table names, which the facade's
/// tests show no term `hc-almanac` computes is.
#[must_use]
pub fn name_or_fallback(
    locale: Option<&Locale>,
    kind: &str,
    term: Term<'_>,
) -> Option<AlmanacName> {
    let own = |tag: &str| {
        let table = table(tag)?;
        table.name_of(kind, term).map(|name| AlmanacName {
            name,
            tag: table.tag,
        })
    };
    match locale {
        Some(locale) => almanac_name(locale, kind, term)
            .or_else(|| own(ENGLISH.tag))
            .or_else(|| own(JAPANESE.tag)),
        None => own(JAPANESE.tag).or_else(|| own(ENGLISH.tag)),
    }
}

/// Every table, in tag order.
pub static VOCABULARIES: &[AlmanacNames] = &[ENGLISH, JAPANESE, CHINESE_SIMPLIFIED];

/// English: the Hepburn readings, and the asterisms' English names.
pub const ENGLISH: AlmanacNames = AlmanacNames {
    tag: "en",
    cycles: &[
        (
            ROKUYO,
            &[
                "senshō",
                "tomobiki",
                "senbu",
                "butsumetsu",
                "taian",
                "shakkō",
            ],
        ),
        (
            MANSION,
            &[
                "Horn",
                "Neck",
                "Root",
                "Room",
                "Heart",
                "Tail",
                "Winnowing Basket",
                "Dipper",
                "Ox",
                "Girl",
                "Emptiness",
                "Rooftop",
                "Encampment",
                "Wall",
                "Legs",
                "Bond",
                "Stomach",
                "Hairy Head",
                "Net",
                "Turtle Beak",
                "Three Stars",
                "Well",
                "Ghost",
                "Willow",
                "Star",
                "Extended Net",
                "Wings",
                "Chariot",
            ],
        ),
        (
            NINE_STAR,
            &[
                "ippaku suisei",
                "jikoku dosei",
                "sanpeki mokusei",
                "shiroku mokusei",
                "goō dosei",
                "roppaku kinsei",
                "shichiseki kinsei",
                "happaku dosei",
                "kyūshi kasei",
            ],
        ),
        (
            TWELVE_DIRECT,
            &[
                "tatsu", "nozoku", "mitsu", "taira", "sadan", "toru", "yaburu", "ayabu", "naru",
                "osan", "hiraku", "tozu",
            ],
        ),
        (
            NAYIN,
            &[
                "kaichūkin",
                "rochūka",
                "tairinboku",
                "robōdo",
                "jinbōkin",
                "santōka",
                "kankasui",
                "jōtōdo",
                "hakurōkin",
                "yōryūboku",
                "seisensui",
                "okujōdo",
                "hekirekika",
                "shōhakuboku",
                "chōryūsui",
                "sachūkin",
                "sangeka",
                "heichiboku",
                "hekijōdo",
                "kinpakukin",
                "fukutōka",
                "tengasui",
                "taiekido",
                "saisenkin",
                "sōshakumoku",
                "daikeisui",
                "sachūdo",
                "tenjōka",
                "zakuroboku",
                "taikaisui",
            ],
        ),
    ],
    terms: &[
        (LOWER_REGISTER, "daimyonichi", "daimyōnichi"),
        (LOWER_REGISTER, "tenonnichi", "ten'onnichi"),
        (LOWER_REGISTER, "bosonichi", "bosōnichi"),
        (LOWER_REGISTER, "tsukitokunichi", "tsukitokunichi"),
        (LOWER_REGISTER, "kamiyoshinichi", "kamiyoshinichi"),
        (LOWER_REGISTER, "kishukunichi", "kishukunichi"),
        (LOWER_REGISTER, "tenshanichi", "tenshanichi"),
        (LOWER_REGISTER, "taikanichi", "taikanichi"),
        (LOWER_REGISTER, "rojakunichi", "rōjakunichi"),
        (LOWER_REGISTER, "metsumonnichi", "metsumonnichi"),
        (LOWER_REGISTER, "kikonichi", "kikonichi"),
        (LOWER_REGISTER, "chiiminichi", "chiiminichi"),
        (LOWER_REGISTER, "junichi", "jūnichi"),
        (LOWER_REGISTER, "fukunichi", "fukunichi"),
        (LOWER_REGISTER, "omonichi", "ōmōnichi"),
        (LOWER_REGISTER, "kuenichi", "kuenichi"),
        (LOWER_REGISTER, "jushinichi", "jūshinichi"),
        (LOWER_REGISTER, "kurobi", "jushinichi"),
        (LOWER_REGISTER, "tenkanichi", "tenkanichi"),
        (LOWER_REGISTER, "jikanichi", "jikanichi"),
        (LOWER_REGISTER, "gomunichi", "gomunichi"),
        (SELECTED_DAY, "ichiryu-manbai", "ichiryū manbai bi"),
        (SELECTED_DAY, "sanrinbo", "sanrinbō"),
        (SELECTED_DAY, "fujoju", "fujōju bi"),
        (SELECTED_DAY, "hassen", "hassen"),
        (SELECTED_DAY, "hassen-interval", "hassen no manibi"),
        (SELECTED_DAY, "jippogure", "jippōgure"),
        (SELECTED_DAY, "tenichi-tenjo", "ten'ichi tenjō"),
        (SELECTED_DAY, "koshin", "kōshin"),
        (SELECTED_DAY, "kinoene", "kinoene"),
        (SELECTED_DAY, "tsuchinoto-mi", "tsuchinoto mi"),
        (SELECTED_DAY, "tiger-day", "tora no hi"),
        (SELECTED_DAY, "snake-day", "mi no hi"),
        (SELECTED_DAY, "great-earth-taboo", "ōtsuchi"),
        (SELECTED_DAY, "lesser-earth-taboo", "kotsuchi"),
        (SELECTED_DAY, "earth-taboo-interval", "bondo no manibi"),
    ],
};

/// Japanese: the names the almanacs print.
pub const JAPANESE: AlmanacNames = AlmanacNames {
    tag: "ja",
    cycles: &[
        (ROKUYO, &["先勝", "友引", "先負", "仏滅", "大安", "赤口"]),
        (
            MANSION,
            &[
                "角", "亢", "氐", "房", "心", "尾", "箕", "斗", "牛", "女", "虚", "危", "室", "壁",
                "奎", "婁", "胃", "昴", "畢", "觜", "参", "井", "鬼", "柳", "星", "張", "翼", "軫",
            ],
        ),
        (
            NINE_STAR,
            &[
                "一白水星",
                "二黒土星",
                "三碧木星",
                "四緑木星",
                "五黄土星",
                "六白金星",
                "七赤金星",
                "八白土星",
                "九紫火星",
            ],
        ),
        (
            TWELVE_DIRECT,
            &[
                "建", "除", "満", "平", "定", "執", "破", "危", "成", "納", "開", "閉",
            ],
        ),
        (
            NAYIN,
            &[
                "海中金",
                "爐中火",
                "大林木",
                "路傍土",
                "釼鋒金",
                "山頭火",
                "澗下水",
                "城頭土",
                "白鑞金",
                "楊柳木",
                "井泉水",
                "屋上土",
                "霹靂火",
                "松柏木",
                "長流水",
                "沙中金",
                "山下火",
                "平地木",
                "壁上土",
                "金箔金",
                "覆燈火",
                "天河水",
                "大駅土",
                "釵釧金",
                "桑柘木",
                "大溪水",
                "沙中土",
                "天上火",
                "柘榴木",
                "大海水",
            ],
        ),
    ],
    terms: &[
        (LOWER_REGISTER, "daimyonichi", "大明日"),
        (LOWER_REGISTER, "tenonnichi", "天恩日"),
        (LOWER_REGISTER, "bosonichi", "母倉日"),
        (LOWER_REGISTER, "tsukitokunichi", "月徳日"),
        (LOWER_REGISTER, "kamiyoshinichi", "神吉日"),
        (LOWER_REGISTER, "kishukunichi", "鬼宿日"),
        (LOWER_REGISTER, "tenshanichi", "天赦日"),
        (LOWER_REGISTER, "taikanichi", "大禍日"),
        (LOWER_REGISTER, "rojakunichi", "狼藉日"),
        (LOWER_REGISTER, "metsumonnichi", "滅門日"),
        (LOWER_REGISTER, "kikonichi", "帰忌日"),
        (LOWER_REGISTER, "chiiminichi", "血忌日"),
        (LOWER_REGISTER, "junichi", "重日"),
        (LOWER_REGISTER, "fukunichi", "復日"),
        (LOWER_REGISTER, "omonichi", "往亡日"),
        (LOWER_REGISTER, "kuenichi", "凶会日"),
        (LOWER_REGISTER, "jushinichi", "十死日"),
        (LOWER_REGISTER, "kurobi", "受死日"),
        (LOWER_REGISTER, "tenkanichi", "天火日"),
        (LOWER_REGISTER, "jikanichi", "地火日"),
        (LOWER_REGISTER, "gomunichi", "五墓日"),
        (SELECTED_DAY, "ichiryu-manbai", "一粒万倍日"),
        (SELECTED_DAY, "sanrinbo", "三隣亡"),
        (SELECTED_DAY, "fujoju", "不成就日"),
        (SELECTED_DAY, "hassen", "八専"),
        (SELECTED_DAY, "hassen-interval", "八専の間日"),
        (SELECTED_DAY, "jippogure", "十方暮"),
        (SELECTED_DAY, "tenichi-tenjo", "天一天上"),
        (SELECTED_DAY, "koshin", "庚申"),
        (SELECTED_DAY, "kinoene", "甲子"),
        (SELECTED_DAY, "tsuchinoto-mi", "己巳"),
        (SELECTED_DAY, "tiger-day", "寅の日"),
        (SELECTED_DAY, "snake-day", "巳の日"),
        (SELECTED_DAY, "great-earth-taboo", "大犯土"),
        (SELECTED_DAY, "lesser-earth-taboo", "小犯土"),
        (SELECTED_DAY, "earth-taboo-interval", "犯土の間日"),
        (COMBINATION, "pardon-and-grain", "天赦日＋一粒万倍日"),
        (
            COMBINATION,
            "pardon-and-grain-and-taian",
            "天赦日＋一粒万倍日＋大安",
        ),
        (COMBINATION, "taian-and-grain", "大安＋一粒万倍日"),
        (COMBINATION, "pardon-and-taian", "天赦日＋大安"),
        (COMBINATION, "tiger-and-taian", "寅の日＋大安"),
        (COMBINATION, "earth-serpent-and-taian", "己巳＋大安"),
        (
            COMBINATION,
            "grain-and-no-accomplishment",
            "一粒万倍日＋不成就日",
        ),
        (
            COMBINATION,
            "grain-and-three-neighbours",
            "一粒万倍日＋三隣亡",
        ),
    ],
};

/// Chinese, simplified: the 納音 as 『三命通會』 names them.
pub const CHINESE_SIMPLIFIED: AlmanacNames = AlmanacNames {
    tag: "zh-Hans",
    cycles: &[(
        NAYIN,
        &[
            "海中金",
            "炉中火",
            "大林木",
            "路旁土",
            "剑锋金",
            "山头火",
            "涧下水",
            "城头土",
            "白蜡金",
            "杨柳木",
            "井泉水",
            "屋上土",
            "霹雳火",
            "松柏木",
            "长流水",
            "砂中金",
            "山下火",
            "平地木",
            "壁上土",
            "金泊金",
            "覆灯火",
            "天河水",
            "大驿土",
            "钗钏金",
            "桑柘木",
            "大溪水",
            "砂中土",
            "天上火",
            "石榴木",
            "大海水",
        ],
    )],
    terms: &[],
};

#[cfg(test)]
mod tests {
    use super::*;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).expect("a tag")
    }

    /// Every table names each cycle it lists at the cycle's length, only
    /// kinds that exist, and no term twice; Japanese names every cycle.
    #[test]
    fn every_table_is_well_formed() {
        for table in VOCABULARIES {
            assert!(
                crate::data::LOCALES
                    .iter()
                    .any(|data| data.tag == table.tag),
                "{}",
                table.tag
            );
            for (kind, names) in table.cycles {
                let length = CYCLES
                    .iter()
                    .find(|(cycle, _)| cycle == kind)
                    .map(|(_, length)| *length);
                assert_eq!(Some(names.len()), length, "{} {kind}", table.tag);
                assert!(names.iter().all(|name| !name.is_empty()));
            }
            for (index, (kind, id, name)) in table.terms.iter().enumerate() {
                assert!(TABLES.contains(kind), "{} {kind}", table.tag);
                assert!(!name.is_empty() && !id.is_empty());
                assert!(
                    !table.terms[index + 1..]
                        .iter()
                        .any(|(other, again, _)| other == kind && again == id),
                    "{} names {kind} {id} twice",
                    table.tag
                );
            }
        }
        for (kind, _) in CYCLES {
            assert!(JAPANESE.cycles.iter().any(|(cycle, _)| cycle == kind));
        }
    }

    /// 大安 is the fifth of the six, 畢 the nineteenth mansion and 天赦日 a
    /// term of the lower register; `ja-JP` finds `ja`, a locale with no
    /// table falls to English and then to Japanese, and `None` asks for
    /// Japanese first.
    #[test]
    fn a_name_comes_from_the_locale_then_english_then_japanese() {
        assert_eq!(
            almanac_name(&locale("ja-JP"), ROKUYO, Term::Position(4)),
            Some(AlmanacName {
                name: "大安",
                tag: "ja"
            })
        );
        assert_eq!(
            almanac_name(&locale("de"), MANSION, Term::Position(18)),
            None
        );
        assert_eq!(
            name_or_fallback(Some(&locale("de")), MANSION, Term::Position(18)),
            Some(AlmanacName {
                name: "Net",
                tag: "en"
            })
        );
        assert_eq!(
            name_or_fallback(
                Some(&locale("en-GB")),
                COMBINATION,
                Term::Id("pardon-and-grain")
            ),
            Some(AlmanacName {
                name: "天赦日＋一粒万倍日",
                tag: "ja"
            })
        );
        assert_eq!(
            name_or_fallback(None, LOWER_REGISTER, Term::Id("tenshanichi")),
            Some(AlmanacName {
                name: "天赦日",
                tag: "ja"
            })
        );
        assert_eq!(
            name_or_fallback(Some(&Locale::ROOT), TWELVE_DIRECT, Term::Position(0)),
            Some(AlmanacName {
                name: "tatsu",
                tag: "en"
            })
        );
        assert_eq!(JAPANESE.name_of(ROKUYO, Term::Position(6)), None);
        assert_eq!(JAPANESE.name_of(LOWER_REGISTER, Term::Id("taian")), None);
    }
}
