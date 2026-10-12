//! The names of the ISO 8601 week-numbering year in the locales that carry them.
//!
//! The ISO 8601 week-numbering year is a year of 52 or 53 whole weeks, Monday
//! to Sunday, named after the Gregorian year its Thursday falls in. Each term
//! is the one the locale's own Wikipedia article on the ISO week date writes
//! for that year, read on 2026-10-13 by two independent paths: the rendered
//! article page, and a second fetch of the same page. Both readings agreed on
//! every term carried here. A locale whose article gives no term, or whose
//! readings disagreed, carries none, and the caller gets the English name
//! from the week-year system instead.
//!
//! No locale term is carried for the NRF 4-5-4 retail calendar or for the
//! 4-4-5 year ending the last Saturday of December. The sources read for this
//! table name neither of them in any locale, so both stay in English. Only the
//! ISO 8601 week-numbering year has an entry here.
//!
//! The zh-Hans and zh-Hant terms are the forms the two articles write in
//! their defining sentence, in simplified and traditional script. The
//! `source` of each entry is the article to check against.

use crate::Locale;

/// The term one locale gives the ISO 8601 week-numbering year, with the
/// article it was read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeekYearTerm {
    /// The data entry's tag, as the locale names it.
    pub tag: &'static str,
    /// The term for the ISO 8601 week-numbering year.
    pub term: &'static str,
    /// The article the term was read from.
    pub source: &'static str,
}

/// The carried terms, one entry per locale that has one.
pub static ISO_WEEK_YEAR_TERMS: &[WeekYearTerm] = &[
    WeekYearTerm {
        tag: "fr",
        term: "année ISO",
        source: "https://fr.wikipedia.org/wiki/Numérotation_ISO_des_semaines",
    },
    WeekYearTerm {
        tag: "nl",
        term: "weekjaar",
        source: "https://nl.wikipedia.org/wiki/Weeknummer",
    },
    WeekYearTerm {
        tag: "zh-Hans",
        term: "ISO周数年",
        source: "https://zh.wikipedia.org/zh-cn/ISO週日曆",
    },
    WeekYearTerm {
        tag: "zh-Hant",
        term: "ISO週數年",
        source: "https://zh.wikipedia.org/zh-tw/ISO週日曆",
    },
];

/// The term for the ISO 8601 week-numbering year in `locale`, with the tag of
/// the entry the term came from. `None` where no locale in the chain carries
/// a term, which is the caller's cue to write the English name.
#[must_use]
pub fn iso_week_year_term(locale: &Locale) -> Option<(&'static str, &'static str)> {
    for candidate in locale.fallback() {
        let Some(rendered) = candidate.rendered() else {
            continue;
        };
        let Some(entry) = ISO_WEEK_YEAR_TERMS
            .iter()
            .find(|entry| entry.tag == rendered.as_str())
        else {
            continue;
        };
        return Some((entry.term, entry.tag));
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
    fn every_entry_carries_a_term_and_names_its_article() {
        for entry in ISO_WEEK_YEAR_TERMS {
            assert!(!entry.term.is_empty(), "{}", entry.tag);
            assert!(entry.source.starts_with("https://"), "{}", entry.tag);
        }
    }

    #[test]
    fn the_tags_are_distinct() {
        for (index, entry) in ISO_WEEK_YEAR_TERMS.iter().enumerate() {
            assert!(
                ISO_WEEK_YEAR_TERMS[index + 1..]
                    .iter()
                    .all(|other| other.tag != entry.tag),
                "{}",
                entry.tag
            );
        }
    }

    #[test]
    fn a_traditional_chinese_region_answers_from_zh_hant() {
        assert_eq!(
            iso_week_year_term(&locale("zh-Hant-HK")),
            Some(("ISO週數年", "zh-Hant"))
        );
    }

    #[test]
    fn a_simplified_chinese_request_gets_the_simplified_term() {
        assert_eq!(
            iso_week_year_term(&locale("zh-Hans")),
            Some(("ISO周数年", "zh-Hans"))
        );
    }

    #[test]
    fn a_region_of_french_answers_from_fr() {
        assert_eq!(
            iso_week_year_term(&locale("fr-CA")),
            Some(("année ISO", "fr"))
        );
    }

    #[test]
    fn a_dutch_request_gets_the_dutch_term() {
        assert_eq!(iso_week_year_term(&locale("nl")), Some(("weekjaar", "nl")));
    }

    #[test]
    fn a_locale_without_a_carried_term_answers_none() {
        assert_eq!(iso_week_year_term(&locale("de")), None);
    }

    #[test]
    fn the_root_locale_answers_none() {
        assert_eq!(iso_week_year_term(&locale("und")), None);
    }

    #[test]
    fn the_four_terms_are_distinct_from_the_english_name() {
        for entry in ISO_WEEK_YEAR_TERMS {
            assert_ne!(entry.term, "ISO 8601 week-numbering year", "{}", entry.tag);
        }
    }
}
