//! The parts of casing that depend on the locale rather than the character.
//!
//! Two problems, both real in calendar text:
//!
//! * **Turkish dotted and dotless i.** Turkish and Azerbaijani have four
//!   letters where English has two: `i`/`İ` carry a dot in both cases and
//!   `ı`/`I` carry none. Uppercasing the Turkish month `iyi` with the
//!   default mapping gives `IYI`, which is a different word. The Unicode
//!   default case conversion (The Unicode Standard, version 17.0, §3.13
//!   "Default Case Algorithms") is overridden for `tr` and `az` by the
//!   language-sensitive rows of `SpecialCasing-18.0.0.txt`, and
//!   [`to_uppercase`] and [`to_lowercase`] here honour them.
//! * **Whether a month name is a capitalised word at all.** English,
//!   German, Turkish and Indonesian capitalise month and weekday names;
//!   French, Spanish, Italian, Portuguese, Russian, Polish, Czech and Dutch
//!   do not. German is the strict case: the names are nouns, so they are
//!   capitalised wherever they stand. CLDR carries this as
//!   `contextTransforms`; here it is one boolean per locale.
//!
//! `docs/systems/casing-and-bidi.md` gives the rows of `SpecialCasing.txt`
//! this module reads and the ones it does not.
//!
//! Only the first character is ever recased: a "title case" that recased
//! every word would break `2 de enero` and `tháng 1`. Word-level title
//! casing needs a word-break implementation, which this crate does not have
//! and does not pretend to.

use crate::locale::Locale;
use crate::names::locale_data;

/// Which case mappings a locale uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CasingStyle {
    /// The Unicode default case mappings.
    Standard,
    /// The Turkic override: `i` ↔ `İ` and `ı` ↔ `I`.
    Turkic,
}

/// The casing style a locale uses: the Turkic one for Turkish and
/// Azerbaijani, whichever entry the locale resolves to (Azerbaijani has no
/// entry of its own), else its entry's.
#[must_use]
pub fn casing_style(locale: &Locale) -> CasingStyle {
    if matches!(locale.language(), "tr" | "az") {
        return CasingStyle::Turkic;
    }
    locale_data(locale).casing
}

/// Whether the locale writes month and weekday names with a capital.
#[must_use]
pub fn capitalises_month_names(locale: &Locale) -> bool {
    locale_data(locale).capitalises_month_names
}

/// Lowercase one character under a casing style, appending to `out`.
#[cfg(feature = "alloc")]
fn push_lowercase(character: char, style: CasingStyle, out: &mut alloc::string::String) {
    if style == CasingStyle::Turkic {
        match character {
            'I' => {
                out.push('ı');
                return;
            }
            'İ' => {
                out.push('i');
                return;
            }
            _ => {}
        }
    }
    out.extend(character.to_lowercase());
}

/// Uppercase one character under a casing style, appending to `out`.
#[cfg(feature = "alloc")]
fn push_uppercase(character: char, style: CasingStyle, out: &mut alloc::string::String) {
    if style == CasingStyle::Turkic {
        match character {
            'i' => {
                out.push('İ');
                return;
            }
            'ı' => {
                out.push('I');
                return;
            }
            _ => {}
        }
    }
    out.extend(character.to_uppercase());
}

/// Lowercase `text` for a locale.
///
/// `str::to_lowercase` (the Unicode default conversion, with its final
/// sigma rule: `ΟΔΥΣΣΕΥΣ` is `οδυσσευς`) except in Turkic locales, which map
/// `I` to `ı`, `İ` to `i` and `I` followed by a dot above to `i` first
/// (the `tr` and `az` rows of `SpecialCasing.txt`, read in its 18.0.0
/// version on 2026-10-05) and then lowercase the rest by default.
#[cfg(feature = "alloc")]
#[must_use]
pub fn to_lowercase(locale: &Locale, text: &str) -> alloc::string::String {
    if casing_style(locale) != CasingStyle::Turkic {
        return text.to_lowercase();
    }
    let mut mapped = alloc::string::String::with_capacity(text.len());
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            'I' if characters.peek() == Some(&'\u{307}') => {
                characters.next();
                mapped.push('i');
            }
            'I' => mapped.push('ı'),
            'İ' => mapped.push('i'),
            other => mapped.push(other),
        }
    }
    mapped.to_lowercase()
}

/// Uppercase `text` for a locale.
///
/// `str::to_uppercase` except in Turkic locales, which map `i` to `İ` and
/// `ı` to `I` first. Note that the German ß still uppercases to `SS`, which
/// is the Unicode default and what German typography expects in all-caps
/// headings. The Greek rule that drops the accents of capitals (ICU's `el`
/// mapping) is not carried: no source for it has been read.
#[cfg(feature = "alloc")]
#[must_use]
pub fn to_uppercase(locale: &Locale, text: &str) -> alloc::string::String {
    if casing_style(locale) != CasingStyle::Turkic {
        return text.to_uppercase();
    }
    let mapped: alloc::string::String = text
        .chars()
        .map(|character| match character {
            'i' => 'İ',
            'ı' => 'I',
            other => other,
        })
        .collect();
    mapped.to_uppercase()
}

/// `text` with its first character uppercased for the locale.
#[cfg(feature = "alloc")]
#[must_use]
pub fn capitalise_first(locale: &Locale, text: &str) -> alloc::string::String {
    let style = casing_style(locale);
    let mut out = alloc::string::String::with_capacity(text.len());
    let mut characters = text.chars();
    if let Some(first) = characters.next() {
        push_uppercase(first, style, &mut out);
    }
    out.extend(characters);
    out
}

/// `text` with its first character lowercased for the locale.
#[cfg(feature = "alloc")]
#[must_use]
pub fn lowercase_first(locale: &Locale, text: &str) -> alloc::string::String {
    let style = casing_style(locale);
    let mut out = alloc::string::String::with_capacity(text.len());
    let mut characters = text.chars();
    if let Some(first) = characters.next() {
        push_lowercase(first, style, &mut out);
    }
    out.extend(characters);
    out
}

/// A month or weekday name as it should appear at the start of a sentence.
///
/// CLDR's `contextTransforms` type `stand-alone`/`beginning-of-sentence`:
/// even a language that writes *janvier* lowercase writes *Janvier* when it
/// opens the sentence.
#[cfg(feature = "alloc")]
#[must_use]
pub fn name_at_sentence_start(locale: &Locale, name: &str) -> alloc::string::String {
    capitalise_first(locale, name)
}

/// A month or weekday name as it should appear inside a sentence.
///
/// Capitalising languages get the name unchanged; the others get a
/// lowercased first character, which matters when the data itself is stored
/// capitalised.
#[cfg(feature = "alloc")]
#[must_use]
pub fn name_in_sentence(locale: &Locale, name: &str) -> alloc::string::String {
    if capitalises_month_names(locale) {
        capitalise_first(locale, name)
    } else {
        lowercase_first(locale, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::names::{NameContext, NameWidth, month_label};
    use alloc::string::ToString as _;
    use hc_calendar::{CalendarId, Month};

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).unwrap()
    }

    #[test]
    fn turkish_keeps_the_dot_on_its_capital_i() {
        let turkish = locale("tr-TR");
        assert_eq!(casing_style(&turkish), CasingStyle::Turkic);
        assert_eq!(to_uppercase(&turkish, "iyi"), "İYİ");
        assert_eq!(to_lowercase(&turkish, "IYI"), "ıyı");
        assert_eq!(to_lowercase(&turkish, "İYİ"), "iyi");
    }

    /// Azerbaijani has no entry of its own and is Turkic all the same, and
    /// the rest is `str::to_lowercase`, which writes a capital sigma at the
    /// end of a word as `ς` (the `Final_Sigma` row of `SpecialCasing.txt`,
    /// read in its 18.0.0 version on 2026-10-04), as a letter-by-letter
    /// lowercasing does not.
    #[test]
    fn azerbaijani_is_turkic_and_the_rest_is_the_default_conversion() {
        let azerbaijani = locale("az");
        assert_eq!(casing_style(&azerbaijani), CasingStyle::Turkic);
        assert_eq!(to_uppercase(&azerbaijani, "i"), "İ");
        assert_eq!(to_lowercase(&azerbaijani, "I"), "ı");
        assert_eq!(to_lowercase(&locale("az-Latn-AZ"), "İ"), "i");
        // `I` followed by a dot above is `i` in a Turkic locale.
        assert_eq!(to_lowercase(&azerbaijani, "I\u{307}"), "i");
        assert_eq!(to_lowercase(&locale("en"), "I\u{307}"), "i\u{307}");
        let english = locale("en");
        assert_eq!(to_lowercase(&english, "ΟΔΥΣΣΕΥΣ"), "οδυσσευς");
        assert_eq!(
            to_lowercase(&english, "ΟΔΥΣΣΕΥΣ"),
            "ΟΔΥΣΣΕΥΣ".to_lowercase()
        );
        assert_eq!(to_lowercase(&locale("tr"), "ΟΔΥΣΣΕΥΣ"), "οδυσσευς");
        assert_eq!(to_uppercase(&english, "straße"), "STRASSE");
    }

    /// A legacy language subtag reaches the entry of its replacement, so
    /// `iw` is Hebrew in names, direction and plural rules alike.
    #[test]
    fn a_legacy_language_tag_reaches_the_entry_of_its_replacement() {
        for (legacy, current) in [("iw", "he"), ("in", "id"), ("tl", "fil"), ("jw", "jv")] {
            assert_eq!(
                locale_data(&locale(legacy)).tag,
                locale_data(&locale(current)).tag,
                "{legacy}"
            );
        }
        assert_eq!(locale_data(&locale("iw")).tag, "he");
        assert_eq!(
            crate::direction::locale_direction(&locale("iw")),
            crate::direction::Direction::RightToLeft
        );
        assert_eq!(locale_data(&locale("no")).tag, "und");
    }

    #[test]
    fn english_uses_the_default_mappings_for_the_same_letters() {
        let english = locale("en-US");
        assert_eq!(casing_style(&english), CasingStyle::Standard);
        assert_eq!(to_uppercase(&english, "iyi"), "IYI");
        assert_eq!(to_lowercase(&english, "IYI"), "iyi");
    }

    #[test]
    fn the_turkish_month_names_survive_a_case_round_trip() {
        let turkish = locale("tr");
        for ordinal in 1..=12u8 {
            let label = month_label(
                &turkish,
                CalendarId("gregory"),
                Month::regular(ordinal),
                NameWidth::Wide,
                NameContext::Format,
            )
            .unwrap()
            .to_string();
            let shouted = to_uppercase(&turkish, &label);
            let restored = capitalise_first(&turkish, &to_lowercase(&turkish, &shouted));
            assert_eq!(restored, label, "month {ordinal}");
        }
    }

    #[test]
    fn german_month_names_are_nouns_and_stay_capitalised() {
        let german = locale("de-DE");
        assert!(capitalises_month_names(&german));
        assert_eq!(name_in_sentence(&german, "Januar"), "Januar");
        assert_eq!(name_at_sentence_start(&german, "Januar"), "Januar");
    }

    #[test]
    fn french_month_names_are_lowercase_except_at_a_sentence_start() {
        let french = locale("fr-FR");
        assert!(!capitalises_month_names(&french));
        assert_eq!(name_in_sentence(&french, "Janvier"), "janvier");
        assert_eq!(name_at_sentence_start(&french, "janvier"), "Janvier");
    }

    #[test]
    fn recasing_an_empty_or_non_cased_string_is_a_no_op() {
        let japanese = locale("ja");
        assert_eq!(capitalise_first(&japanese, ""), "");
        assert_eq!(capitalise_first(&japanese, "1月"), "1月");
        assert_eq!(to_uppercase(&japanese, "午前"), "午前");
    }

    #[test]
    fn russian_month_names_stay_lowercase_inside_a_sentence() {
        let russian = locale("ru");
        assert!(!capitalises_month_names(&russian));
        assert_eq!(name_in_sentence(&russian, "Январь"), "январь");
        assert_eq!(name_at_sentence_start(&russian, "январь"), "Январь");
    }
}
