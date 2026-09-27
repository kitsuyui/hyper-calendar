//! What a locale calls the terms of the reckonings of a day and a year
//! outside the Japanese almanac, as data, for the lines the boundary
//! crates write about them: the choghadiya and the Panchak kinds, the sites
//! of the Kumbh Mela and the rivers of Pushkaram, the planets that rule
//! the planetary hours, the Chinese night watches, and the folk days of
//! Vietnam, China and Turkey.
//!
//! The terms are other crates' — `hc-calendars-indic`, `hc-seasons`,
//! `hc-format` and `hc-almanac` — and each gives its own name, as its
//! source writes it. This module is the locale's half, as
//! [`crate::almanac`] is for the Japanese almanac: the name each carried
//! locale gives a term, looked up by the term's kind and its identifier.
//! The facade's vocabulary test holds the crates together: every term
//! named here is one they compute, and every term they compute is named
//! in its own language with the crate's own name.
//!
//! # What is carried
//!
//! Each kind is named in the one language its source writes it in, which
//! is the kind's own ([`NATIVE`]), with the crate's own names:
//!
//! * **`en`**: the seven choghadiya, as Drik Panchang prints them in
//!   English [drik-choghadiya-2025]; the five Panchak kinds, as Prokerala
//!   and India TV write them [prokerala-panchak, indiatv-panchak-2025];
//!   the four sites of the Kumbh Mela, as the Mela Adhikari of 2013 names
//!   them [kumbh-allahabad-astrology]; the rivers of Pushkaram, as
//!   Wikipedia's "Pushkaram" names them [wikipedia-pushkaram]; and the
//!   seven classical planets, `hc-seasons`'s English names.
//! * **`zh-Hant`**: the five night watches, 一更 to 五更 [wikipedia-zh-geng].
//! * **`zh-Hans`**: 入梅 and 出梅 [cma-meiyu-nongshi], and the four counts
//!   of the first month, 几龙治水, 几牛耕田, 几日得辛 and 几人分饼
//!   [wikipedia-zh-long-zhi-shui].
//! * **`vi`**: Tam Nương and Nguyệt Kỵ [vtc-ngay-xau-2018].
//! * **`tr`**: the two halves of the folk year, *Hızır günleri* and *Kasım
//!   günleri*, and its named days, Hıdırellez to *üçüncü cemre*
//!   [wikipedia-tr-hidirellez, bilkent-cemre].
//!
//! No kind has a name in a second language: none was read, and nothing
//! here is translated. The English glosses some crates give in their
//! documentation — "the three maidens" for Tam Nương — are the author's
//! renderings, not a source's names, and are not carried.
//!
//! # Locales
//!
//! [`reckoning_name`] walks [`Locale::fallback`] and takes the first table
//! in the chain that names the term, as [`crate::almanac`] does, and
//! [`name_or_fallback`] adds the rule every line writer follows: a locale
//! with no name for a term has it from English, and a term English does
//! not name from the kind's own language, which names every term of the
//! kind; a request for no locale in particular, the facade's `native`,
//! has it from the kind's own language first.

use crate::locale::Locale;

/// The choghadiya, the seven kinds of the eighths of a day and a night,
/// by `hc-calendars-indic`'s identifiers, `udvega` to `roga`.
pub const CHOGHADIYA: &str = "choghadiya";
/// The kinds of a Panchak window, `rog`, `raj`, `agni`, `chor` and
/// `mrityu`: the lower case of `hc-calendars-indic`'s names.
pub const PANCHAK: &str = "panchak";
/// The four sites of the Kumbh Mela, `haridwar`, `prayag`, `nashik` and
/// `ujjain`.
pub const KUMBH_SITE: &str = "kumbh-site";
/// The rivers of Pushkaram, by `hc-calendars-indic`'s identifiers,
/// `pushkaram-ganga` to `pushkaram-pranahita`.
pub const PUSHKARAM_RIVER: &str = "pushkaram-river";
/// The seven classical planets, by `hc-seasons`'s identifiers, `sun` to
/// `saturn`.
pub const PLANET: &str = "planet";
/// The five night watches, `1` to `5`.
pub const NIGHT_WATCH: &str = "night-watch";
/// Tam Nương and Nguyệt Kỵ, `tam-nuong` and `nguyet-ky`.
pub const VIETNAMESE_DAY: &str = "vietnamese-day";
/// 入梅 and 出梅, one identifier for each rule: `ru-mei-bing`,
/// `ru-mei-ren` and `chu-mei-wei`.
pub const PLUM_RAINS: &str = "plum-rains";
/// The four counts of the first month: `dragons`, `oxen`, `xin` and
/// `cakes`.
pub const FIRST_MONTH_COUNT: &str = "first-month-count";
/// The two halves of the Turkish folk year, `hizir` and `kasim`.
pub const FOLK_HALF: &str = "folk-half";
/// The named days of the Turkish folk year: `hidirellez`, `kasim`,
/// `erbain`, `hamsin`, `cemre-air`, `cemre-water` and `cemre-earth`.
pub const FOLK_NAMED_DAY: &str = "folk-named-day";

/// Every kind, with the tag of the table in its own language, which names
/// every term of it.
pub const NATIVE: &[(&str, &str)] = &[
    (CHOGHADIYA, "en"),
    (PANCHAK, "en"),
    (KUMBH_SITE, "en"),
    (PUSHKARAM_RIVER, "en"),
    (PLANET, "en"),
    (NIGHT_WATCH, "zh-Hant"),
    (VIETNAMESE_DAY, "vi"),
    (PLUM_RAINS, "zh-Hans"),
    (FIRST_MONTH_COUNT, "zh-Hans"),
    (FOLK_HALF, "tr"),
    (FOLK_NAMED_DAY, "tr"),
];

/// One locale's names for the terms.
#[derive(Debug, Clone, Copy)]
pub struct ReckoningNames {
    /// The BCP 47 tag, spelled as [`crate::data::LOCALES`] spells it.
    pub tag: &'static str,
    /// Each named term: its kind, its identifier and its name.
    pub terms: &'static [(&'static str, &'static str, &'static str)],
}

impl ReckoningNames {
    /// The name this table gives a term of a kind, if it has one. The
    /// identifier is matched without regard to ASCII case.
    #[must_use]
    pub fn name_of(&self, kind: &str, id: &str) -> Option<&'static str> {
        self.terms
            .iter()
            .find(|(table, entry, _)| *table == kind && entry.eq_ignore_ascii_case(id))
            .map(|(_, _, name)| *name)
    }
}

/// A term's name and the tag of the table that gave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReckoningName {
    /// What the locale calls the term.
    pub name: &'static str,
    /// The tag of the table that answered: `tr` for a request for `tr-CY`.
    pub tag: &'static str,
}

/// The table for a data tag, spelled as [`ReckoningNames::tag`] is.
#[must_use]
pub fn table(tag: &str) -> Option<&'static ReckoningNames> {
    VOCABULARIES.iter().find(|table| table.tag == tag)
}

/// The tag of a kind's own language, from [`NATIVE`].
#[must_use]
pub fn native_tag(kind: &str) -> Option<&'static str> {
    NATIVE
        .iter()
        .find(|(entry, _)| *entry == kind)
        .map(|(_, tag)| *tag)
}

/// The name a table gives a term, with the table's tag.
fn from_table(tag: &str, kind: &str, id: &str) -> Option<ReckoningName> {
    let table = table(tag)?;
    table.name_of(kind, id).map(|name| ReckoningName {
        name,
        tag: table.tag,
    })
}

/// What `locale` calls a term, from the first table in its fallback chain
/// that names it; `None` when none does, the root locale included.
#[must_use]
pub fn reckoning_name(locale: &Locale, kind: &str, id: &str) -> Option<ReckoningName> {
    locale.fallback().find_map(|candidate| {
        let rendered = candidate.rendered()?;
        from_table(rendered.as_str(), kind, id)
    })
}

/// A term's name as every line writer gives it: the locale's, where its
/// chain has one; else English's; else the kind's own language's. A
/// `locale` of `None` asks for none in particular, and the kind's own
/// language then answers first. The answer is `None` only for a term no
/// table names, which the facade's tests show no term the crates compute
/// is.
#[must_use]
pub fn name_or_fallback(locale: Option<&Locale>, kind: &str, id: &str) -> Option<ReckoningName> {
    let native = || native_tag(kind).and_then(|tag| from_table(tag, kind, id));
    match locale {
        Some(locale) => reckoning_name(locale, kind, id)
            .or_else(|| from_table(ENGLISH.tag, kind, id))
            .or_else(native),
        None => native().or_else(|| from_table(ENGLISH.tag, kind, id)),
    }
}

/// Every table, in tag order.
pub static VOCABULARIES: &[ReckoningNames] = &[
    ENGLISH,
    TURKISH,
    VIETNAMESE,
    CHINESE_SIMPLIFIED,
    CHINESE_TRADITIONAL,
];

/// English: the choghadiya, the Panchak kinds, the Kumbh sites, the
/// Pushkaram rivers and the planets, as their sources write them.
pub const ENGLISH: ReckoningNames = ReckoningNames {
    tag: "en",
    terms: &[
        (CHOGHADIYA, "udvega", "Udvega"),
        (CHOGHADIYA, "chara", "Chara"),
        (CHOGHADIYA, "labha", "Labha"),
        (CHOGHADIYA, "amrita", "Amrita"),
        (CHOGHADIYA, "kala", "Kala"),
        (CHOGHADIYA, "shubha", "Shubha"),
        (CHOGHADIYA, "roga", "Roga"),
        (PANCHAK, "rog", "Rog"),
        (PANCHAK, "raj", "Raj"),
        (PANCHAK, "agni", "Agni"),
        (PANCHAK, "chor", "Chor"),
        (PANCHAK, "mrityu", "Mrityu"),
        (KUMBH_SITE, "haridwar", "Haridwar"),
        (KUMBH_SITE, "prayag", "Prayag"),
        (KUMBH_SITE, "nashik", "Nashik"),
        (KUMBH_SITE, "ujjain", "Ujjain"),
        (PUSHKARAM_RIVER, "pushkaram-ganga", "Ganga"),
        (PUSHKARAM_RIVER, "pushkaram-narmada", "Narmada"),
        (PUSHKARAM_RIVER, "pushkaram-sarasvati", "Sarasvati"),
        (PUSHKARAM_RIVER, "pushkaram-yamuna", "Yamuna"),
        (PUSHKARAM_RIVER, "pushkaram-godavari", "Godavari"),
        (PUSHKARAM_RIVER, "pushkaram-krishna", "Krishna"),
        (PUSHKARAM_RIVER, "pushkaram-kaveri", "Kaveri"),
        (PUSHKARAM_RIVER, "pushkaram-bhima", "Bhima"),
        (PUSHKARAM_RIVER, "pushkaram-tamraparni", "Tamraparni"),
        (PUSHKARAM_RIVER, "pushkaram-tapti", "Tapti"),
        (PUSHKARAM_RIVER, "pushkaram-brahmaputra", "Brahmaputra"),
        (PUSHKARAM_RIVER, "pushkaram-tungabhadra", "Tungabhadra"),
        (PUSHKARAM_RIVER, "pushkaram-sindhu", "Sindhu"),
        (PUSHKARAM_RIVER, "pushkaram-pranahita", "Pranahita"),
        (PLANET, "sun", "Sun"),
        (PLANET, "moon", "Moon"),
        (PLANET, "mercury", "Mercury"),
        (PLANET, "venus", "Venus"),
        (PLANET, "mars", "Mars"),
        (PLANET, "jupiter", "Jupiter"),
        (PLANET, "saturn", "Saturn"),
    ],
};

/// Turkish: the halves and the named days of the folk year.
pub const TURKISH: ReckoningNames = ReckoningNames {
    tag: "tr",
    terms: &[
        (FOLK_HALF, "hizir", "Hızır günleri"),
        (FOLK_HALF, "kasim", "Kasım günleri"),
        (FOLK_NAMED_DAY, "hidirellez", "Hıdırellez"),
        (FOLK_NAMED_DAY, "kasim", "Kasım"),
        (FOLK_NAMED_DAY, "erbain", "erbain"),
        (FOLK_NAMED_DAY, "hamsin", "hamsin"),
        (FOLK_NAMED_DAY, "cemre-air", "birinci cemre"),
        (FOLK_NAMED_DAY, "cemre-water", "ikinci cemre"),
        (FOLK_NAMED_DAY, "cemre-earth", "üçüncü cemre"),
    ],
};

/// Vietnamese: Tam Nương and Nguyệt Kỵ.
pub const VIETNAMESE: ReckoningNames = ReckoningNames {
    tag: "vi",
    terms: &[
        (VIETNAMESE_DAY, "tam-nuong", "Tam Nương"),
        (VIETNAMESE_DAY, "nguyet-ky", "Nguyệt Kỵ"),
    ],
};

/// Simplified Chinese: 入梅 and 出梅, and the counts of the first month.
pub const CHINESE_SIMPLIFIED: ReckoningNames = ReckoningNames {
    tag: "zh-Hans",
    terms: &[
        (PLUM_RAINS, "ru-mei-bing", "入梅"),
        (PLUM_RAINS, "ru-mei-ren", "入梅"),
        (PLUM_RAINS, "chu-mei-wei", "出梅"),
        (FIRST_MONTH_COUNT, "dragons", "几龙治水"),
        (FIRST_MONTH_COUNT, "oxen", "几牛耕田"),
        (FIRST_MONTH_COUNT, "xin", "几日得辛"),
        (FIRST_MONTH_COUNT, "cakes", "几人分饼"),
    ],
};

/// Traditional Chinese: the night watches.
pub const CHINESE_TRADITIONAL: ReckoningNames = ReckoningNames {
    tag: "zh-Hant",
    terms: &[
        (NIGHT_WATCH, "1", "一更"),
        (NIGHT_WATCH, "2", "二更"),
        (NIGHT_WATCH, "3", "三更"),
        (NIGHT_WATCH, "4", "四更"),
        (NIGHT_WATCH, "5", "五更"),
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).expect("a tag")
    }

    /// Every table is a carried locale, names only kinds that exist and no
    /// term twice; every kind's own table names it.
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
            for (index, (kind, id, name)) in table.terms.iter().enumerate() {
                assert!(native_tag(kind).is_some(), "{} {kind}", table.tag);
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
        for (kind, tag) in NATIVE {
            let own = table(tag).expect("a table");
            assert!(
                own.terms.iter().any(|(entry, _, _)| entry == kind),
                "{kind}"
            );
        }
        assert!(
            VOCABULARIES
                .windows(2)
                .all(|pair| pair[0].tag < pair[1].tag),
            "tag order"
        );
    }

    /// `tr-CY` finds `tr`; a locale with no table has English, and a kind
    /// English does not name its own language's; `None` asks for the own
    /// language first.
    #[test]
    fn a_name_comes_from_the_locale_then_english_then_the_kinds_own() {
        assert_eq!(
            name_or_fallback(Some(&locale("tr-CY")), FOLK_NAMED_DAY, "cemre-earth"),
            Some(ReckoningName {
                name: "üçüncü cemre",
                tag: "tr"
            })
        );
        assert_eq!(
            name_or_fallback(Some(&locale("de")), CHOGHADIYA, "amrita"),
            Some(ReckoningName {
                name: "Amrita",
                tag: "en"
            })
        );
        assert_eq!(
            name_or_fallback(Some(&locale("ja")), VIETNAMESE_DAY, "NGUYET-KY"),
            Some(ReckoningName {
                name: "Nguyệt Kỵ",
                tag: "vi"
            })
        );
        assert_eq!(
            name_or_fallback(None, PLUM_RAINS, "chu-mei-wei").map(|named| named.tag),
            Some("zh-Hans")
        );
        assert_eq!(
            name_or_fallback(Some(&locale("zh-TW")), NIGHT_WATCH, "3"),
            Some(ReckoningName {
                name: "三更",
                tag: "zh-Hant"
            })
        );
        assert_eq!(reckoning_name(&locale("fr"), PLANET, "sun"), None);
        assert_eq!(name_or_fallback(Some(&Locale::ROOT), PLANET, "pluto"), None);
    }
}
