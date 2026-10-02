//! What a locale calls a municipality, which CLDR does not name.
//!
//! A holiday table scopes a day to a municipality by a code under its
//! subdivision's, `JP-14-130` for 川崎市 (ADR 0014), and CLDR's
//! `subdivisions/<locale>.xml` names ISO 3166-2 subdivisions only: not one
//! municipality, so [`crate::place_names`] has none of these codes. This
//! module carries the names of the municipalities the holiday tables list,
//! from the sources the tables cite for the cities themselves, and a
//! facade test holds every municipality any table lists to a name here.
//!
//! # What is carried
//!
//! The twenty designated cities (政令指定都市) of Japan and 長崎市, whose
//! days `docs/systems/japan-holidays.md` carries — twenty-one codes, in
//! the form JIS X 0402's code takes under its prefecture's ISO 3166-2 code
//! (`JP-14-130` for 川崎市, 14130):
//!
//! * **`ja`**, the name as the city's own instruments write it —
//!   `川崎市`, `さいたま市` — which is also the name JIS X 0402 gives it
//!   [jis-x0402-cities, jp-city-designated]. JIS X 0402 itself was not
//!   read: the names are the instruments' and Wikipedia's list of its
//!   codes.
//! * **`ja-Latn`**, the Hepburn romanisation of that name with its city
//!   suffix, `Kawasaki-shi`, as English Wikipedia's leads print it for
//!   nineteen of the twenty-one [wikipedia-en-city-leads],
//!   macrons and all: `Kyōto-shi`, `Ōsaka-shi`, `Kōbe-shi` and
//!   `Kitakyūshū-shi`. Japanese Wikipedia prints no romanisation for 札幌市
//!   and 横浜市, whose leads give the readings さっぽろし and よこはまし
//!   [wikipedia-ja-city-readings]; their `ja-Latn` names are those readings
//!   in Hepburn, `Sapporo-shi` and `Yokohama-shi`, which needs no macron.
//! * **`en`**, the English name without the suffix, as English
//!   Wikipedia's list of the designated cities writes it, `Kawasaki`,
//!   `Kyoto`, `Osaka`, and as the leads of 長崎市 and the others do
//!   [wikipedia-en-designated-cities].
//!
//! Not carried: Chinese and Korean names, which no source read prints for
//! these cities beyond a language link's title; the kana readings of the
//! other nineteen, which JIS X 0402's code list prints and was not read;
//! the names of any municipality no table lists. A place CLDR names is
//! [`crate::place_names`]'s.
//!
//! # Locales
//!
//! [`municipal_name`] walks [`Locale::fallback`] and takes the first table
//! in the chain that names the code, then English, as
//! [`crate::holiday_names`] does for a day: `ja-JP` finds `ja`, `ja-Latn`
//! finds its own table, and a locale of no table here gets `en`.

use crate::locale::Locale;

/// One locale's names for municipalities.
#[derive(Debug, Clone, Copy)]
pub struct MunicipalNames {
    /// The BCP 47 tag: `ja`, `ja-Latn` or `en`.
    pub tag: &'static str,
    /// Each named municipality: its code under its subdivision's, `JP-14-130`, in code
    /// order, and its name.
    pub names: &'static [(&'static str, &'static str)],
}

/// A municipality's name and the tag of the table that gave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MunicipalName {
    /// What the locale calls the municipality.
    pub name: &'static str,
    /// The tag of the table that answered.
    pub tag: &'static str,
}

/// The tag of the table a lookup ends at.
pub const ENGLISH: &str = "en";

/// The code a table names a municipality by, matched as every identifier
/// is: white space around it ignored, ASCII letters in either case.
fn same_code(code: &str, given: &str) -> bool {
    code.eq_ignore_ascii_case(given.trim())
}

/// What `locale` calls the municipality `code`, from the first table in its
/// fallback chain that names it, then English; `None` for a `locale` that
/// names no locale, as `native` does, which asks for English, in which case
/// English answers, and for a code no table names. The code matches
/// without regard to ASCII case.
#[must_use]
pub fn municipal_name(locale: Option<&Locale>, code: &str) -> Option<MunicipalName> {
    let in_table = |tag: &str| {
        TABLES
            .iter()
            .filter(|table| table.tag == tag)
            .find_map(|table| {
                table
                    .names
                    .iter()
                    .find(|(listed, _)| same_code(listed, code))
                    .map(|(_, name)| MunicipalName {
                        name,
                        tag: table.tag,
                    })
            })
    };
    locale
        .and_then(|locale| {
            locale.fallback().find_map(|candidate| {
                let rendered = candidate.rendered()?;
                in_table(rendered.as_str())
            })
        })
        .or_else(|| in_table(ENGLISH))
}

/// What English calls the municipality `code`, `Kawasaki`; `None` for a
/// code no table names.
#[must_use]
pub fn english_name(code: &str) -> Option<&'static str> {
    municipal_name(None, code).map(|found| found.name)
}

/// Every municipality named, in code order: the code the English table
/// lists, which every other table is a subset of.
pub fn codes() -> impl Iterator<Item = &'static str> {
    TABLES
        .iter()
        .find(|table| table.tag == ENGLISH)
        .into_iter()
        .flat_map(|table| table.names.iter().map(|(code, _)| *code))
}

/// The code as the tables write it, `JP-14-130`, for `code` in any case
/// and with white space around it; `None` for a code no table names.
#[must_use]
pub fn code_of(code: &str) -> Option<&'static str> {
    codes().find(|listed| same_code(listed, code))
}

/// Whether some table names the municipality `code`.
#[must_use]
pub fn is_named(code: &str) -> bool {
    code_of(code).is_some()
}

/// Every table, in tag order.
pub static TABLES: &[MunicipalNames] = &[
    MunicipalNames {
        tag: "en",
        names: &[
            ("JP-01-100", "Sapporo"),
            ("JP-04-100", "Sendai"),
            ("JP-11-100", "Saitama"),
            ("JP-12-100", "Chiba"),
            ("JP-14-100", "Yokohama"),
            ("JP-14-130", "Kawasaki"),
            ("JP-14-150", "Sagamihara"),
            ("JP-15-100", "Niigata"),
            ("JP-22-100", "Shizuoka"),
            ("JP-22-130", "Hamamatsu"),
            ("JP-23-100", "Nagoya"),
            ("JP-26-100", "Kyoto"),
            ("JP-27-100", "Osaka"),
            ("JP-27-140", "Sakai"),
            ("JP-28-100", "Kobe"),
            ("JP-33-100", "Okayama"),
            ("JP-34-100", "Hiroshima"),
            ("JP-40-100", "Kitakyushu"),
            ("JP-40-130", "Fukuoka"),
            ("JP-42-201", "Nagasaki"),
            ("JP-43-100", "Kumamoto"),
        ],
    },
    MunicipalNames {
        tag: "ja",
        names: &[
            ("JP-01-100", "札幌市"),
            ("JP-04-100", "仙台市"),
            ("JP-11-100", "さいたま市"),
            ("JP-12-100", "千葉市"),
            ("JP-14-100", "横浜市"),
            ("JP-14-130", "川崎市"),
            ("JP-14-150", "相模原市"),
            ("JP-15-100", "新潟市"),
            ("JP-22-100", "静岡市"),
            ("JP-22-130", "浜松市"),
            ("JP-23-100", "名古屋市"),
            ("JP-26-100", "京都市"),
            ("JP-27-100", "大阪市"),
            ("JP-27-140", "堺市"),
            ("JP-28-100", "神戸市"),
            ("JP-33-100", "岡山市"),
            ("JP-34-100", "広島市"),
            ("JP-40-100", "北九州市"),
            ("JP-40-130", "福岡市"),
            ("JP-42-201", "長崎市"),
            ("JP-43-100", "熊本市"),
        ],
    },
    MunicipalNames {
        tag: "ja-Latn",
        names: &[
            ("JP-01-100", "Sapporo-shi"),
            ("JP-04-100", "Sendai-shi"),
            ("JP-11-100", "Saitama-shi"),
            ("JP-12-100", "Chiba-shi"),
            ("JP-14-100", "Yokohama-shi"),
            ("JP-14-130", "Kawasaki-shi"),
            ("JP-14-150", "Sagamihara-shi"),
            ("JP-15-100", "Niigata-shi"),
            ("JP-22-100", "Shizuoka-shi"),
            ("JP-22-130", "Hamamatsu-shi"),
            ("JP-23-100", "Nagoya-shi"),
            ("JP-26-100", "Kyōto-shi"),
            ("JP-27-100", "Ōsaka-shi"),
            ("JP-27-140", "Sakai-shi"),
            ("JP-28-100", "Kōbe-shi"),
            ("JP-33-100", "Okayama-shi"),
            ("JP-34-100", "Hiroshima-shi"),
            ("JP-40-100", "Kitakyūshū-shi"),
            ("JP-40-130", "Fukuoka-shi"),
            ("JP-42-201", "Nagasaki-shi"),
            ("JP-43-100", "Kumamoto-shi"),
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).expect("a tag")
    }

    /// Every table names the same municipalities, once each, in code
    /// order, and every name is written.
    #[test]
    fn every_table_names_the_same_codes_in_order() {
        assert_eq!(codes().count(), 21);
        let mut previous = "";
        for code in codes() {
            assert!(previous < code, "{code}");
            previous = code;
        }
        for table in TABLES {
            assert!(
                table.names.iter().map(|(code, _)| *code).eq(codes()),
                "{}",
                table.tag
            );
            assert!(table.names.iter().all(|(_, name)| !name.is_empty()));
        }
    }

    /// 川崎市 is JIS X 0402's 14130 under Kanagawa's `JP-14`.
    #[test]
    fn kawasaki_is_named_in_japanese_romanised_and_english() {
        assert_eq!(
            municipal_name(Some(&locale("ja")), "JP-14-130"),
            Some(MunicipalName {
                name: "川崎市",
                tag: "ja"
            })
        );
        assert_eq!(
            municipal_name(Some(&locale("ja-JP")), " jp-14-130 "),
            Some(MunicipalName {
                name: "川崎市",
                tag: "ja"
            })
        );
        assert_eq!(
            municipal_name(Some(&locale("ja-Latn")), "JP-14-130"),
            Some(MunicipalName {
                name: "Kawasaki-shi",
                tag: "ja-Latn"
            })
        );
        assert_eq!(english_name("JP-14-130"), Some("Kawasaki"));
        // A locale no table here names, and no locale, get English.
        for requested in [Some(locale("de")), Some(locale("und")), None] {
            assert_eq!(
                municipal_name(requested.as_ref(), "JP-14-130"),
                Some(MunicipalName {
                    name: "Kawasaki",
                    tag: "en"
                })
            );
        }
        assert_eq!(municipal_name(Some(&locale("ja")), "JP-14"), None);
        assert_eq!(municipal_name(Some(&locale("ja")), "JP-14-999"), None);
        assert!(is_named("jp-14-130") && !is_named("JP-14"));
    }

    /// Every `ja` name is a city, 市, and every `ja-Latn` name ends in
    /// `-shi`.
    #[test]
    fn the_names_are_cities() {
        for table in TABLES {
            for (code, name) in table.names {
                match table.tag {
                    "ja" => assert!(name.ends_with('市'), "{code}"),
                    "ja-Latn" => assert!(name.ends_with("-shi"), "{code}"),
                    _ => assert!(!name.contains('-'), "{code}"),
                }
            }
        }
    }
}
