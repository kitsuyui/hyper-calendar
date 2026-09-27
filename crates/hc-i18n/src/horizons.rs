//! What a locale calls a horizon a rising is measured against, as data,
//! for the lines `hc_horizons` writes.
//!
//! The horizons are `hc-astro`'s named conventions, `hc_astro::horizon`,
//! each with an English name. A name in another language is carried only
//! where a source in that language names the convention: for Japanese, the
//! National Astronomical Observatory of Japan's own wording for its
//! conventions; for another language, the national observatory's or
//! almanac office's wording, or the title of the convention's own source
//! in that language. Nothing here is translated.
//!
//! # What is carried
//!
//! Only `usno`, the U.S. Naval Observatory's sea-level horizon, has names
//! outside English, and each is the observatory's name as an observatory
//! or almanac office writes it:
//!
//! * `zh-Hant` 美國海軍天文氣象台 and `zh-Hans` 美国海军天文气象台, the
//!   Hong Kong Observatory's, whose astronomy portal says its data are
//!   computed from the Royal Nautical Almanac Office's and this
//!   observatory's, for mean sea level, in its traditional and its
//!   simplified pages [hko-astronomy-portal]. The simplified form is the
//!   Hong Kong Observatory's own page, not a mainland observatory's.
//! * `fr` Observatoire naval de Washington D.C., the heading of the
//!   Institut de mécanique céleste et de calcul des éphémérides' page on
//!   the observatory in its *Promenade dans le système solaire*, in
//!   sentence case [imcce-promenade-usno].
//!
//! Left without a name, and so written in English by the line writers:
//! every locale for `geometric-dip`, a convention of Meeus's and the
//! USNO's sea-level horizon with `calendar-code2`'s dip, which no source
//! names in another language; every locale for
//! `calendrical-calculations`, whose book, Reingold and Dershowitz's
//! *Calendrical Calculations*, has no translated title that was found; and
//! for `usno`, Japanese — the Observatory's page defining its own sunrise
//! [nao-rekiwiki-hinode-teigi] names no convention of these three, and its
//! own horizon, 35′8″ of refraction at a height of 0 m, is not carried —
//! and Korean, German, Spanish, Arabic, Persian and
//! Hebrew, for which no observatory's or almanac office's page naming the
//! USNO was read. Every other carried locale has no table either.
//!
//! # Locales
//!
//! [`horizon_name`] walks [`Locale::fallback`] and takes the first table in
//! the chain that names the horizon, as [`crate::territories`] does:
//! `zh-TW` finds `zh-Hant`, `fr-CA` finds `fr`.

use crate::locale::Locale;

/// One locale's names for the horizons.
#[derive(Debug, Clone, Copy)]
pub struct HorizonNames {
    /// The BCP 47 tag, spelled as [`crate::data::LOCALES`] spells it.
    pub tag: &'static str,
    /// Each named horizon: its `hc_astro::horizon` identifier and its name.
    pub names: &'static [(&'static str, &'static str)],
}

impl HorizonNames {
    /// The name this table gives the horizon with an identifier, matched
    /// without regard to ASCII case.
    #[must_use]
    pub fn name_of(&self, id: &str) -> Option<&'static str> {
        self.names
            .iter()
            .find(|(horizon, _)| horizon.eq_ignore_ascii_case(id))
            .map(|(_, name)| *name)
    }
}

/// A horizon's name and the tag of the table that gave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HorizonName {
    /// What the locale calls the horizon.
    pub name: &'static str,
    /// The tag of the table that answered: `zh-Hant` for `zh-TW`.
    pub tag: &'static str,
}

/// What `locale` calls a horizon, from the first table in its fallback
/// chain that names it; `None` when none does, the root locale included.
#[must_use]
pub fn horizon_name(locale: &Locale, id: &str) -> Option<HorizonName> {
    locale.fallback().find_map(|candidate| {
        let rendered = candidate.rendered()?;
        TABLES
            .iter()
            .filter(|table| rendered.as_str() == table.tag)
            .find_map(|table| {
                table.name_of(id).map(|name| HorizonName {
                    name,
                    tag: table.tag,
                })
            })
    })
}

/// Every table, in tag order.
pub static TABLES: &[HorizonNames] = &[
    HorizonNames {
        tag: "fr",
        names: &[("usno", "Observatoire naval de Washington D.C.")],
    },
    HorizonNames {
        tag: "zh-Hans",
        names: &[("usno", "美国海军天文气象台")],
    },
    HorizonNames {
        tag: "zh-Hant",
        names: &[("usno", "美國海軍天文氣象台")],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).expect("a tag")
    }

    #[test]
    fn every_table_is_a_carried_locale_and_names_each_horizon_once() {
        for table in TABLES {
            assert!(
                crate::data::LOCALES
                    .iter()
                    .any(|data| data.tag == table.tag)
            );
            for (index, (id, name)) in table.names.iter().enumerate() {
                assert!(!name.is_empty());
                assert!(
                    !table.names[index + 1..]
                        .iter()
                        .any(|(other, _)| other == id)
                );
            }
        }
    }

    /// `zh-TW` and `zh-HK` find `zh-Hant`, `fr-CA` finds `fr`; Japanese
    /// and the root locale name nothing, and nothing names the other two
    /// horizons.
    #[test]
    fn a_name_comes_from_the_locales_chain_or_not_at_all() {
        for tag in ["zh-TW", "zh-HK", "zh-Hant"] {
            assert_eq!(
                horizon_name(&locale(tag), "USNO"),
                Some(HorizonName {
                    name: "美國海軍天文氣象台",
                    tag: "zh-Hant"
                }),
                "{tag}"
            );
        }
        assert_eq!(
            horizon_name(&locale("fr-CA"), "usno").map(|named| named.tag),
            Some("fr")
        );
        assert_eq!(horizon_name(&locale("ja"), "usno"), None);
        assert_eq!(horizon_name(&Locale::ROOT, "usno"), None);
        for tag in ["fr", "zh-Hans", "zh-Hant"] {
            assert_eq!(horizon_name(&locale(tag), "geometric-dip"), None);
            assert_eq!(horizon_name(&locale(tag), "calendrical-calculations"), None);
        }
    }
}
