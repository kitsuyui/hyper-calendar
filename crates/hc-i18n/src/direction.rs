//! Script direction, and the bidirectional isolation a formatter needs.
//!
//! A date is a run of strongly-directional text dropped into a sentence that
//! may run the other way. Without isolation, "١٢/٣/٢٠٢٤ ,الأحد" reorders in
//! ways neither the author nor the reader intended: the Unicode
//! bidirectional algorithm (UAX 9) resolves the whole paragraph at once, so
//! a Latin month name inside an Arabic sentence can drag neighbouring
//! punctuation across with it.
//!
//! The fix defined by UAX 9 §2.4 is an *isolate*: U+2066‥U+2068 open a run
//! whose direction is resolved independently, and U+2069 closes it. This
//! module supplies the constants, the decision of when a field needs one,
//! and a writer that emits the pair.
//!
//! This is not an implementation of the bidirectional algorithm. It does not
//! reorder text, does not resolve neutral runs and does not implement the
//! paragraph-level heuristics beyond the first-strong rule of UAX 9 §P2–P3.

use core::fmt;

use crate::locale::Locale;

/// U+2066 LEFT-TO-RIGHT ISOLATE.
pub const LEFT_TO_RIGHT_ISOLATE: char = '\u{2066}';
/// U+2067 RIGHT-TO-LEFT ISOLATE.
pub const RIGHT_TO_LEFT_ISOLATE: char = '\u{2067}';
/// U+2068 FIRST STRONG ISOLATE: direction taken from the first strong
/// character inside the run.
pub const FIRST_STRONG_ISOLATE: char = '\u{2068}';
/// U+2069 POP DIRECTIONAL ISOLATE, which closes any of the three.
pub const POP_DIRECTIONAL_ISOLATE: char = '\u{2069}';
/// U+200E LEFT-TO-RIGHT MARK.
pub const LEFT_TO_RIGHT_MARK: char = '\u{200E}';
/// U+200F RIGHT-TO-LEFT MARK.
pub const RIGHT_TO_LEFT_MARK: char = '\u{200F}';

/// The direction a script is written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Latin, Cyrillic, Greek, Han, Devanagari, Thai and most others.
    LeftToRight,
    /// Arabic, Hebrew, Syriac, Thaana, NKo, Adlam.
    RightToLeft,
}

impl Direction {
    /// Whether text in this direction runs right to left.
    #[must_use]
    pub const fn is_rtl(self) -> bool {
        matches!(self, Self::RightToLeft)
    }

    /// The CSS/HTML keyword, `ltr` or `rtl`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeftToRight => "ltr",
            Self::RightToLeft => "rtl",
        }
    }

    /// The isolate that opens a run in this direction.
    #[must_use]
    pub const fn isolate_opener(self) -> char {
        match self {
            Self::LeftToRight => LEFT_TO_RIGHT_ISOLATE,
            Self::RightToLeft => RIGHT_TO_LEFT_ISOLATE,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Script subtags written right to left.
///
/// Taken from the `rtl` scripts of CLDR `scriptMetadata.xml`, restricted to
/// the ones a calendar is ever localised into.
const RIGHT_TO_LEFT_SCRIPTS: &[&str] = &[
    "Adlm", "Arab", "Aran", "Hebr", "Nkoo", "Rohg", "Samr", "Syrc", "Thaa", "Yezi",
];

/// The direction of a script subtag such as `Arab` or `Latn`.
#[must_use]
pub fn script_direction(script: &str) -> Direction {
    if RIGHT_TO_LEFT_SCRIPTS.contains(&script) {
        Direction::RightToLeft
    } else {
        Direction::LeftToRight
    }
}

/// The direction of a locale.
///
/// An explicit script subtag decides it; otherwise the locale's own data
/// entry does, walking the fallback chain.
#[must_use]
pub fn locale_direction(locale: &Locale) -> Direction {
    if let Some(script) = locale.script() {
        return script_direction(script);
    }
    crate::names::locale_data(locale).direction
}

/// Ranges of code points whose strong direction is right to left.
///
/// These are the RTL blocks of the Unicode character database: Hebrew,
/// Arabic, Syriac, Thaana, NKo, Samaritan, Mandaic, Arabic Extended-A, the
/// Arabic presentation forms, and the RTL historic and Adlam planes.
const RTL_RANGES: &[(u32, u32)] = &[
    (0x0590, 0x05FF),
    (0x0600, 0x07BF),
    (0x0800, 0x085F),
    (0x08A0, 0x08FF),
    (0xFB1D, 0xFDFF),
    (0xFE70, 0xFEFF),
    (0x0001_0800, 0x0001_0FFF),
    (0x0001_E800, 0x0001_EFFF),
];

/// The direction of the first strongly-directional character, if any.
///
/// This is the first-strong rule of UAX 9 §P2–P3, simplified: the character
/// classes are approximated by block membership plus
/// [`char::is_alphabetic`], which is exact for every script this crate ships
/// data for. Digits and punctuation are neutral and are skipped.
#[must_use]
pub fn first_strong_direction(text: &str) -> Option<Direction> {
    for character in text.chars() {
        let code = character as u32;
        if RTL_RANGES
            .iter()
            .any(|(low, high)| code >= *low && code <= *high)
        {
            return Some(Direction::RightToLeft);
        }
        if character.is_alphabetic() {
            return Some(Direction::LeftToRight);
        }
    }
    None
}

/// Whether a field running `inner` needs isolating inside text running
/// `outer`.
///
/// Same-direction text needs nothing: the isolate would only add two code
/// points to every date a formatter emits.
#[must_use]
pub const fn needs_isolation(outer: Direction, inner: Direction) -> bool {
    !matches!(
        (outer, inner),
        (Direction::LeftToRight, Direction::LeftToRight)
            | (Direction::RightToLeft, Direction::RightToLeft)
    )
}

/// Write `text` wrapped in an explicit isolate for `inner`.
///
/// # Errors
///
/// Returns the sink's error if a write fails.
pub fn write_isolated<W: fmt::Write>(text: &str, inner: Direction, out: &mut W) -> fmt::Result {
    out.write_char(inner.isolate_opener())?;
    out.write_str(text)?;
    out.write_char(POP_DIRECTIONAL_ISOLATE)
}

/// Write `text` wrapped in a first-strong isolate, letting the renderer work
/// the direction out from the content.
///
/// This is the right choice when the field's direction is not known ahead of
/// time — a month name looked up at run time, for instance.
///
/// # Errors
///
/// Returns the sink's error if a write fails.
pub fn write_first_strong_isolated<W: fmt::Write>(text: &str, out: &mut W) -> fmt::Result {
    out.write_char(FIRST_STRONG_ISOLATE)?;
    out.write_str(text)?;
    out.write_char(POP_DIRECTIONAL_ISOLATE)
}

/// Write a date field into surrounding text, isolating it only if the two
/// directions disagree.
///
/// # Errors
///
/// Returns the sink's error if a write fails.
pub fn write_field<W: fmt::Write>(
    text: &str,
    outer: Direction,
    inner: Direction,
    out: &mut W,
) -> fmt::Result {
    if needs_isolation(outer, inner) {
        write_isolated(text, inner, out)
    } else {
        out.write_str(text)
    }
}

/// `text` wrapped in a first-strong isolate.
#[cfg(feature = "alloc")]
#[must_use]
pub fn isolate(text: &str) -> alloc::string::String {
    let mut out = alloc::string::String::with_capacity(text.len() + 6);
    out.push(FIRST_STRONG_ISOLATE);
    out.push_str(text);
    out.push(POP_DIRECTIONAL_ISOLATE);
    out
}

/// `text` with every isolate and directional mark removed.
///
/// Useful when a localised string has to be compared or hashed: the isolates
/// are presentation, not content.
#[cfg(feature = "alloc")]
#[must_use]
pub fn strip_isolates(text: &str) -> alloc::string::String {
    text.chars()
        .filter(|character| {
            !matches!(
                *character,
                LEFT_TO_RIGHT_ISOLATE
                    | RIGHT_TO_LEFT_ISOLATE
                    | FIRST_STRONG_ISOLATE
                    | POP_DIRECTIONAL_ISOLATE
                    | LEFT_TO_RIGHT_MARK
                    | RIGHT_TO_LEFT_MARK
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;

    #[test]
    fn scripts_carry_their_direction() {
        assert_eq!(script_direction("Arab"), Direction::RightToLeft);
        assert_eq!(script_direction("Hebr"), Direction::RightToLeft);
        assert_eq!(script_direction("Latn"), Direction::LeftToRight);
        assert_eq!(script_direction("Hans"), Direction::LeftToRight);
        assert!(Direction::RightToLeft.is_rtl());
        assert_eq!(Direction::LeftToRight.as_str(), "ltr");
    }

    #[test]
    fn locales_inherit_direction_from_data_or_from_an_explicit_script() {
        let arabic = Locale::parse("ar-EG").unwrap();
        assert_eq!(locale_direction(&arabic), Direction::RightToLeft);
        let hebrew = Locale::parse("he").unwrap();
        assert_eq!(locale_direction(&hebrew), Direction::RightToLeft);
        let persian = Locale::parse("fa-IR").unwrap();
        assert_eq!(locale_direction(&persian), Direction::RightToLeft);
        let japanese = Locale::parse("ja-JP").unwrap();
        assert_eq!(locale_direction(&japanese), Direction::LeftToRight);
        // An explicit script overrides the language's usual direction.
        let romanised = Locale::parse("ar-Latn-EG").unwrap();
        assert_eq!(locale_direction(&romanised), Direction::LeftToRight);
    }

    #[test]
    fn the_first_strong_character_decides_a_runs_direction() {
        assert_eq!(first_strong_direction("الأحد"), Some(Direction::RightToLeft));
        assert_eq!(
            first_strong_direction("Sunday"),
            Some(Direction::LeftToRight)
        );
        // Digits and punctuation are neutral, so the letters decide.
        assert_eq!(
            first_strong_direction("12/3 ראשון"),
            Some(Direction::RightToLeft)
        );
        assert_eq!(first_strong_direction("2024-03-12"), None);
        assert_eq!(first_strong_direction(""), None);
    }

    #[test]
    fn isolation_is_added_only_when_the_directions_disagree() {
        assert!(!needs_isolation(
            Direction::LeftToRight,
            Direction::LeftToRight
        ));
        assert!(!needs_isolation(
            Direction::RightToLeft,
            Direction::RightToLeft
        ));
        assert!(needs_isolation(
            Direction::RightToLeft,
            Direction::LeftToRight
        ));
        assert!(needs_isolation(
            Direction::LeftToRight,
            Direction::RightToLeft
        ));
    }

    #[test]
    fn an_embedded_field_is_wrapped_in_the_isolate_of_its_own_direction() {
        let mut out = String::new();
        write_field(
            "2024",
            Direction::RightToLeft,
            Direction::LeftToRight,
            &mut out,
        )
        .unwrap();
        assert_eq!(out, "\u{2066}2024\u{2069}");

        let mut same = String::new();
        write_field(
            "٢٠٢٤",
            Direction::RightToLeft,
            Direction::RightToLeft,
            &mut same,
        )
        .unwrap();
        assert_eq!(same, "٢٠٢٤");

        let mut rtl = String::new();
        write_isolated("الأحد", Direction::RightToLeft, &mut rtl).unwrap();
        assert_eq!(rtl, "\u{2067}الأحد\u{2069}");
    }

    #[test]
    fn a_first_strong_isolate_lets_the_renderer_decide() {
        let mut out = String::new();
        write_first_strong_isolated("Sunday", &mut out).unwrap();
        assert_eq!(out, "\u{2068}Sunday\u{2069}");
        assert_eq!(isolate("Sunday"), out);
    }

    #[test]
    fn isolates_can_be_stripped_back_out() {
        let wrapped = isolate("12 يناير");
        assert_eq!(strip_isolates(&wrapped), "12 يناير");
        assert_eq!(strip_isolates("\u{200F}12\u{200E}"), "12");
    }
}
