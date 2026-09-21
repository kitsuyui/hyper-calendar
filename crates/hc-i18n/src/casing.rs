//! The parts of casing that depend on the locale rather than the character.
//!
//! Two problems, both real in calendar text:
//!
//! * **Turkish dotted and dotless i.** Turkish and Azerbaijani have four
//!   letters where English has two: `i`/`İ` carry a dot in both cases and
//!   `ı`/`I` carry none. Uppercasing the Turkish month `iyi` with the
//!   default mapping gives `IYI`, which is a different word. The Unicode
//!   default case conversion (UAX 21) is explicitly overridden for `tr` and
//!   `az`, and [`to_uppercase`] and [`to_lowercase`] here honour that.
//! * **Whether a month name is a capitalised word at all.** English,
//!   German, Dutch, Turkish and Indonesian capitalise month and weekday
//!   names; French, Spanish, Italian, Portuguese, Russian, Polish and Czech
//!   do not. German is the strict case: the names are nouns, so they are
//!   capitalised wherever they stand. CLDR carries this as
//!   `contextTransforms`; here it is one boolean per locale.
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

/// The casing style a locale uses.
#[must_use]
pub fn casing_style(locale: &Locale) -> CasingStyle {
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
/// Identical to `str::to_lowercase` except in Turkic locales.
#[cfg(feature = "alloc")]
#[must_use]
pub fn to_lowercase(locale: &Locale, text: &str) -> alloc::string::String {
    let style = casing_style(locale);
    let mut out = alloc::string::String::with_capacity(text.len());
    for character in text.chars() {
        push_lowercase(character, style, &mut out);
    }
    out
}

/// Uppercase `text` for a locale.
///
/// Identical to `str::to_uppercase` except in Turkic locales. Note that the
/// German ß still uppercases to `SS`, which is the Unicode default and what
/// German typography expects in all-caps headings.
#[cfg(feature = "alloc")]
#[must_use]
pub fn to_uppercase(locale: &Locale, text: &str) -> alloc::string::String {
    let style = casing_style(locale);
    let mut out = alloc::string::String::with_capacity(text.len());
    for character in text.chars() {
        push_uppercase(character, style, &mut out);
    }
    out
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
