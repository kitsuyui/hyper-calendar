//! The names of the zodiac signs in the locales that carry them.
//!
//! A tropical sign is one of the twelve 30-degree arcs of the tropical zodiac,
//! Aries to Pisces. A sidereal sign is a rāśi of the Indian sidereal zodiac,
//! Mesha to Mīna. Each name is the one the locale's own Wikipedia article on
//! the zodiac writes, read on 2026-10-11. Two independent readings of each
//! article agreed on every name carried here. A locale whose article gives no
//! names, or whose two readings disagreed, carries none, and the caller gets
//! the English name from the zodiac code instead.
//!
//! The readings were made from the page text that the fetching tool returns,
//! not from the raw wikitext, so a glyph or diacritic may differ from the raw
//! table; the `source` of each entry is the article to check against.
//!
//! The Chinese-family entries give the 座 forms of the article's 星座名称
//! column; the same article's classical 宮 forms differ in form only.

use crate::Locale;

/// The names one locale gives the signs, with the article they were read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignNames {
    /// The data entry's tag, as the locale names it.
    pub tag: &'static str,
    /// The tropical signs, Aries to Pisces, where the article gives them.
    pub tropical: Option<[&'static str; 12]>,
    /// The sidereal signs, Mesha to Mīna, where the article gives them.
    pub sidereal: Option<[&'static str; 12]>,
    /// The article the names were read from.
    pub source: &'static str,
}

/// The carried names, one entry per locale that has any.
pub static SIGN_NAMES: &[SignNames] = &[
    SignNames {
        tag: "ar",
        tropical: Some([
            "حَمَل",
            "ثور",
            "جوزاء",
            "سرطان",
            "أسد",
            "سنبلة",
            "ميزان",
            "عقرب",
            "قوس",
            "جدي",
            "دلو",
            "حوت",
        ]),
        sidereal: None,
        source: "https://ar.wikipedia.org/wiki/الأبراج_الفلكية",
    },
    SignNames {
        tag: "bn",
        tropical: Some([
            "মেষ",
            "বৃষ",
            "মিথুন",
            "কর্কট",
            "সিংহ",
            "কন্যা",
            "তুলা",
            "বৃশ্চিক",
            "ধনু",
            "মকর",
            "কুম্ভ",
            "মীন",
        ]),
        sidereal: None,
        source: "https://bn.wikipedia.org/wiki/রাশিচক্র",
    },
    SignNames {
        tag: "cs",
        tropical: Some([
            "Beran",
            "Býk",
            "Blíženci",
            "Rak",
            "Lev",
            "Panna",
            "Váhy",
            "Štír",
            "Střelec",
            "Kozoroh",
            "Vodnář",
            "Ryby",
        ]),
        sidereal: None,
        source: "https://cs.wikipedia.org/wiki/Znamení_zvěrokruhu",
    },
    SignNames {
        tag: "de",
        tropical: Some([
            "Widder",
            "Stier",
            "Zwillinge",
            "Krebs",
            "Löwe",
            "Jungfrau",
            "Waage",
            "Skorpion",
            "Schütze",
            "Steinbock",
            "Wassermann",
            "Fische",
        ]),
        sidereal: None,
        source: "https://de.wikipedia.org/wiki/Tierkreis",
    },
    SignNames {
        tag: "es",
        tropical: Some([
            "Aries",
            "Tauro",
            "Géminis",
            "Cáncer",
            "Leo",
            "Virgo",
            "Libra",
            "Escorpio",
            "Sagitario",
            "Capricornio",
            "Acuario",
            "Piscis",
        ]),
        sidereal: None,
        source: "https://es.wikipedia.org/wiki/Zodiaco",
    },
    SignNames {
        tag: "fa",
        tropical: Some([
            "حَمَل",
            "ثَور",
            "جَوزا",
            "سَرَطان",
            "اَسَد",
            "سُنبُله",
            "میزان",
            "عَقرَب",
            "قَوس",
            "جَدْی",
            "دَلو",
            "حوت",
        ]),
        sidereal: None,
        source: "https://fa.wikipedia.org/wiki/برج_(نجوم)",
    },
    SignNames {
        tag: "fr",
        tropical: Some([
            "Bélier",
            "Taureau",
            "Gémeaux",
            "Cancer",
            "Lion",
            "Vierge",
            "Balance",
            "Scorpion",
            "Sagittaire",
            "Capricorne",
            "Verseau",
            "Poissons",
        ]),
        sidereal: None,
        source: "https://fr.wikipedia.org/wiki/Zodiaque",
    },
    SignNames {
        tag: "he",
        tropical: Some([
            "טלה",
            "שור",
            "תאומים",
            "סרטן",
            "אריה",
            "בתולה",
            "מאזניים",
            "עקרב",
            "קשת",
            "גדי",
            "דלי",
            "דגים",
        ]),
        sidereal: None,
        source: "https://he.wikipedia.org/wiki/גלגל_המזלות",
    },
    SignNames {
        tag: "hi",
        tropical: None,
        sidereal: Some([
            "मेष",
            "वृष",
            "मिथुन",
            "कर्क",
            "सिंह",
            "कन्या",
            "तुला",
            "वृश्चिक",
            "धनु",
            "मकर",
            "कुम्भ",
            "मीन",
        ]),
        source: "https://hi.wikipedia.org/wiki/राशि",
    },
    SignNames {
        tag: "id",
        tropical: Some([
            "Aries",
            "Taurus",
            "Gemini",
            "Kanser",
            "Leo",
            "Virgo",
            "Libra",
            "Skorpio",
            "Sagitarius",
            "Kaprikornus",
            "Akuarius",
            "Pises",
        ]),
        sidereal: None,
        source: "https://id.wikipedia.org/wiki/Zodiak",
    },
    SignNames {
        tag: "it",
        tropical: Some([
            "Ariete",
            "Toro",
            "Gemelli",
            "Cancro",
            "Leone",
            "Vergine",
            "Bilancia",
            "Scorpione",
            "Sagittario",
            "Capricorno",
            "Aquario",
            "Pesci",
        ]),
        sidereal: None,
        source: "https://it.wikipedia.org/wiki/Zodiaco",
    },
    SignNames {
        tag: "ja",
        tropical: Some([
            "白羊宮",
            "金牛宮",
            "双児宮",
            "巨蟹宮",
            "獅子宮",
            "処女宮",
            "天秤宮",
            "天蝎宮",
            "人馬宮",
            "磨羯宮",
            "宝瓶宮",
            "双魚宮",
        ]),
        sidereal: None,
        source: "https://ja.wikipedia.org/wiki/黄道十二宮",
    },
    SignNames {
        tag: "ko",
        tropical: Some([
            "백양궁",
            "금우궁",
            "쌍아궁",
            "거해궁",
            "사자궁",
            "처녀궁",
            "천칭궁",
            "천갈궁",
            "인마궁",
            "마갈궁",
            "보병궁",
            "쌍어궁",
        ]),
        sidereal: None,
        source: "https://ko.wikipedia.org/wiki/황도_12궁",
    },
    SignNames {
        tag: "ml",
        tropical: Some([
            "മേടം",
            "ഇടവം",
            "മിഥുനം",
            "കർക്കിടകം",
            "ചിങ്ങം",
            "കന്നി",
            "തുലാം",
            "വൃശ്ചികം",
            "ധനു",
            "മകരം",
            "കുംഭം",
            "മീനം",
        ]),
        sidereal: None,
        source: "https://ml.wikipedia.org/wiki/രാശിചക്രം",
    },
    SignNames {
        tag: "mr",
        tropical: Some([
            "मेष",
            "वृषभ",
            "मिथुन",
            "कर्क",
            "सिंह",
            "कन्या",
            "तूळ",
            "वृश्चिक",
            "धनु",
            "मकर",
            "कुंभ",
            "मीन",
        ]),
        sidereal: None,
        source: "https://mr.wikipedia.org/wiki/राशी",
    },
    SignNames {
        tag: "my",
        tropical: Some([
            "မိဿ",
            "ပြိဿ",
            "မေထုန်",
            "ကရကဋ်",
            "သိဟ်",
            "ကန်",
            "တူ",
            "ဗြိစ္ဆာ",
            "ဓနု",
            "မကာရ",
            "ကုံ",
            "မိန်",
        ]),
        sidereal: None,
        source: "https://my.wikipedia.org/wiki/ရာသီခွင်",
    },
    SignNames {
        tag: "ne",
        tropical: None,
        sidereal: Some([
            "मेष",
            "वृष",
            "मिथुन",
            "कर्क",
            "सिंह",
            "कन्या",
            "तुला",
            "वृश्चिक",
            "धनु",
            "मकर",
            "कुम्भ",
            "मीन",
        ]),
        source: "https://ne.wikipedia.org/wiki/राशि",
    },
    SignNames {
        tag: "nl",
        tropical: Some([
            "Ram",
            "Stier",
            "Tweelingen",
            "Kreeft",
            "Leeuw",
            "Maagd",
            "Weegschaal",
            "Schorpioen",
            "Boogschutter",
            "Steenbok",
            "Waterman",
            "Vissen",
        ]),
        sidereal: None,
        source: "https://nl.wikipedia.org/wiki/Dierenriem",
    },
    SignNames {
        tag: "pa-Guru",
        tropical: Some([
            "ਮੇਖ",
            "ਵ੍ਰਸ਼",
            "ਮਿਥੁਨ",
            "ਕਰਕ",
            "ਸਿੰਘ",
            "ਕੰਨਿਆ",
            "ਤੱਕੜੀ",
            "ਵ੍ਰਸਚਿਕ",
            "ਧਨੁ",
            "ਮਕਰ",
            "ਕੁੰਭ",
            "ਮੀਨ",
        ]),
        sidereal: None,
        source: "https://pa.wikipedia.org/wiki/ਰਾਸ਼ੀ_ਚੱਕਰ",
    },
    SignNames {
        tag: "pl",
        tropical: Some([
            "Baran",
            "Byk",
            "Bliźnięta",
            "Rak",
            "Lew",
            "Panna",
            "Waga",
            "Skorpion",
            "Strzelec",
            "Koziorożec",
            "Wodnik",
            "Ryby",
        ]),
        sidereal: None,
        source: "https://pl.wikipedia.org/wiki/Znak_zodiaku",
    },
    SignNames {
        tag: "pt",
        tropical: None,
        sidereal: Some([
            "Meṣa",
            "Vṛṣabha",
            "Mithuna",
            "Karka",
            "Siṃha",
            "Kanyā",
            "Tulā",
            "Vṛścika",
            "Dhanuṣa",
            "Makara",
            "Kumbha",
            "Mīna",
        ]),
        source: "https://pt.wikipedia.org/wiki/Zod%C3%ADaco",
    },
    SignNames {
        tag: "ru",
        tropical: Some([
            "Овен",
            "Телец",
            "Близнецы",
            "Рак",
            "Лев",
            "Дева",
            "Весы",
            "Скорпион",
            "Стрелец",
            "Козерог",
            "Водолей",
            "Рыбы",
        ]),
        sidereal: None,
        source: "https://ru.wikipedia.org/wiki/Знаки_зодиака",
    },
    SignNames {
        tag: "th",
        tropical: Some([
            "ราศีเมษ",
            "ราศีพฤษภ",
            "ราศีเมถุน",
            "ราศีกรกฎ",
            "ราศีสิงห์",
            "ราศีกันย์",
            "ราศีตุล",
            "ราศีพิจิก",
            "ราศีธนู",
            "ราศีมกร",
            "ราศีกุมภ์",
            "ราศีมีน",
        ]),
        sidereal: None,
        source: "https://th.wikipedia.org/wiki/จักรราศี",
    },
    SignNames {
        tag: "tr",
        tropical: Some([
            "Koç", "Boğa", "İkizler", "Yengeç", "Aslan", "Başak", "Terazi", "Akrep", "Yay",
            "Oğlak", "Kova", "Balık",
        ]),
        sidereal: None,
        source: "https://tr.wikipedia.org/wiki/Burçlar",
    },
    SignNames {
        tag: "vi",
        tropical: Some([
            "Bạch Dương",
            "Kim Ngưu",
            "Song Tử",
            "Cự Giải",
            "Sư Tử",
            "Xử Nữ",
            "Thiên Bình",
            "Thiên Yết",
            "Nhân Mã",
            "Ma Kết",
            "Bảo Bình",
            "Song Ngư",
        ]),
        sidereal: None,
        source: "https://vi.wikipedia.org/wiki/Cung_hoàng_đạo",
    },
    SignNames {
        tag: "yue-Hant",
        tropical: Some([
            "白羊座",
            "金牛座",
            "雙子座",
            "巨蟹座",
            "獅子座",
            "室女座",
            "天秤座",
            "天蠍座",
            "人馬座",
            "摩羯座",
            "寶瓶座",
            "雙魚座",
        ]),
        sidereal: None,
        source: "https://zh-yue.wikipedia.org/wiki/黃道十二宮",
    },
    SignNames {
        tag: "zh-Hans",
        tropical: Some([
            "白羊座",
            "金牛座",
            "双子座",
            "巨蟹座",
            "狮子座",
            "室女座",
            "天秤座",
            "天蝎座",
            "人马座",
            "摩羯座",
            "宝瓶座",
            "双鱼座",
        ]),
        sidereal: None,
        source: "https://zh.wikipedia.org/zh-cn/黄道带",
    },
    SignNames {
        tag: "zh-Hant",
        tropical: Some([
            "白羊座",
            "金牛座",
            "雙子座",
            "巨蟹座",
            "獅子座",
            "室女座",
            "天秤座",
            "天蠍座",
            "人馬座",
            "摩羯座",
            "寶瓶座",
            "雙魚座",
        ]),
        sidereal: None,
        source: "https://zh.wikipedia.org/zh-hant/黃道帶",
    },
];

/// The name of the tropical sign at `index` (0 for Aries, 11 for Pisces) in
/// `locale`, with the tag of the entry the name came from. `None` where no
/// locale in the chain carries the name, which is the caller's cue to write
/// the English name.
#[must_use]
pub fn tropical_sign_name(locale: &Locale, index: usize) -> Option<(&'static str, &'static str)> {
    sign_name(locale, index, |entry| entry.tropical)
}

/// The name of the sidereal sign at `index` (0 for Mesha, 11 for Mīna) in
/// `locale`, with the tag of the entry the name came from. See
/// [`tropical_sign_name`] for `None`.
#[must_use]
pub fn sidereal_sign_name(locale: &Locale, index: usize) -> Option<(&'static str, &'static str)> {
    sign_name(locale, index, |entry| entry.sidereal)
}

/// The name at `index` from the first entry in the locale's fallback chain
/// that carries the list `names` picks.
fn sign_name(
    locale: &Locale,
    index: usize,
    names: impl Fn(&SignNames) -> Option<[&'static str; 12]>,
) -> Option<(&'static str, &'static str)> {
    if index >= 12 {
        return None;
    }
    for candidate in locale.fallback() {
        let Some(rendered) = candidate.rendered() else {
            continue;
        };
        let Some(entry) = SIGN_NAMES
            .iter()
            .find(|entry| entry.tag == rendered.as_str())
        else {
            continue;
        };
        if let Some(list) = names(entry) {
            return Some((list[index], entry.tag));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).expect("a tag")
    }

    #[test]
    fn every_entry_carries_a_list_and_names_its_article() {
        for entry in SIGN_NAMES {
            assert!(
                entry.tropical.is_some() || entry.sidereal.is_some(),
                "{}",
                entry.tag
            );
            assert!(entry.source.starts_with("https://"), "{}", entry.tag);
        }
    }

    #[test]
    fn the_tags_are_distinct() {
        for (index, entry) in SIGN_NAMES.iter().enumerate() {
            assert!(
                SIGN_NAMES[index + 1..]
                    .iter()
                    .all(|other| other.tag != entry.tag),
                "{}",
                entry.tag
            );
        }
    }

    #[test]
    fn a_japanese_request_gets_the_japanese_tropical_sign() {
        assert_eq!(tropical_sign_name(&locale("ja"), 0), Some(("白羊宮", "ja")));
        assert_eq!(
            tropical_sign_name(&locale("ja"), 11),
            Some(("双魚宮", "ja"))
        );
    }

    #[test]
    fn a_region_of_a_carried_language_answers_from_it() {
        assert_eq!(
            tropical_sign_name(&locale("zh-Hant-HK"), 6),
            Some(("天秤座", "zh-Hant"))
        );
    }

    #[test]
    fn a_locale_whose_article_gives_no_tropical_names_answers_none() {
        assert_eq!(tropical_sign_name(&locale("pt-PT"), 0), None);
    }

    #[test]
    fn the_sidereal_names_are_the_locales_own_where_carried() {
        assert_eq!(sidereal_sign_name(&locale("pt"), 0), Some(("Meṣa", "pt")));
        assert_eq!(sidereal_sign_name(&locale("ja"), 0), None);
    }

    #[test]
    fn an_index_past_the_twelfth_sign_is_refused() {
        assert_eq!(tropical_sign_name(&locale("ja"), 12), None);
    }
}
