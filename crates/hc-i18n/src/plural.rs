//! CLDR plural rules, cardinal and ordinal.
//!
//! The system is written up in `docs/systems/plural-rules.md`.
//!
//! The rules are those of Unicode CLDR 48, `common/supplemental/plurals.xml`
//! and `common/supplemental/ordinals.xml` (`cldr48-supplemental`, tag
//! `release-48`, read 2026-10-04), generated into `plural/cldr48.rs` by
//! `scripts/plurals-cldr.py`; the operands and rule syntax are those of
//! UTS #35 version 48, Part 3 (Numbers), §5 "Language Plural Rules"
//! (`uts35-v48`).
//!
//! "3 days" is easy; "3 дня", "5 дней" and "21 день" are why this module
//! exists. A formatter that wants to say "in 2 months" has to ask the
//! language which of up to six forms the surrounding message should use, and
//! the answer depends on more than the value: `1`, `1.0` and `1.00` take
//! different forms in several languages. The ordinal rules answer the other
//! question, the form of a position: *1st*, *2nd*, *3rd*, *4th*.
//!
//! # Data is not code
//!
//! A rule is data: a [`PluralCategory`] with a condition, which is an `or`
//! of `and`s of relations, each relation an operand, an optional modulus,
//! `=` or `!=` and a list of ranges, exactly as UTS #35 writes it. One
//! evaluator, [`PluralRules::select`], reads every language's rules; no
//! language has a function of its own, and a language is added by the
//! generator alone.
//!
//! # Operands
//!
//! UTS #35 Part 3 §5.1 defines the operands on the *source string*, not on
//! the number:
//!
//! | operand | meaning |
//! |---|---|
//! | `n` | absolute value of the source number |
//! | `i` | integer digits |
//! | `v` | count of visible fraction digits, trailing zeros included |
//! | `w` | count of visible fraction digits, trailing zeros excluded |
//! | `f` | visible fraction digits as an integer, trailing zeros included |
//! | `t` | visible fraction digits as an integer, trailing zeros excluded |
//! | `c`, `e` | the compact decimal exponent |
//!
//! So `1.0` has `i=1, v=1, w=0, f=0, t=0`, and English calls it `other`
//! while plain `1` is `one`. [`PluralOperands::parse`] preserves that
//! distinction; [`PluralOperands::from_integer`] cannot, because an `i64`
//! has no trailing zeros to preserve.
//!
//! The compact-notation operands `c` and `e` (CLDR 38 and later) are **not**
//! modelled: [`PluralOperands::parse`] reads no exponent, and the evaluator
//! reads the operand as 0, so that a rule's `e = 0 and …` branch applies and
//! its `e != 0` alternatives never do. This only affects compact forms such
//! as "1M", which this workspace never produces.

use crate::error::{I18nError, I18nResult};
use crate::locale::Locale;

mod cldr48;

/// The six CLDR plural categories.
///
/// A language uses a subset; every language has `Other`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PluralCategory {
    /// `zero` — Arabic ٠, Latvian 0, 10, 20, Welsh 0.
    Zero,
    /// `one` — the singular, wherever the language has one.
    One,
    /// `two` — the dual: Arabic, Hebrew, Slovenian, Welsh, Irish.
    Two,
    /// `few` — the paucal: Russian 2–4, Polish 2–4, Arabic 3–10.
    Few,
    /// `many` — Russian 5–20, Arabic 11–99, Czech fractions.
    Many,
    /// `other` — the catch-all every language has.
    Other,
}

impl PluralCategory {
    /// Every category, in CLDR order.
    pub const ALL: [Self; 6] = [
        Self::Zero,
        Self::One,
        Self::Two,
        Self::Few,
        Self::Many,
        Self::Other,
    ];

    /// The CLDR keyword.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::One => "one",
            Self::Two => "two",
            Self::Few => "few",
            Self::Many => "many",
            Self::Other => "other",
        }
    }

    /// Parse a CLDR keyword.
    #[must_use]
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|category| category.as_str() == keyword)
    }
}

impl core::fmt::Display for PluralCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The two kinds of plural rule UTS #35 defines for a language, which are
/// independent of one another: the cardinal rules choose the form after a
/// count, *1 day*, and the ordinal rules the form of a position, *1st*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluralType {
    /// `plurals.xml`: the form after a count.
    Cardinal,
    /// `ordinals.xml`: the form of a position.
    Ordinal,
}

impl PluralType {
    /// Both kinds, cardinal first.
    pub const ALL: [Self; 2] = [Self::Cardinal, Self::Ordinal];

    /// The CLDR keyword, `cardinal` or `ordinal`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cardinal => "cardinal",
            Self::Ordinal => "ordinal",
        }
    }

    /// Parse a CLDR keyword, in either case.
    #[must_use]
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|kind| kind.as_str().eq_ignore_ascii_case(keyword))
    }

    const fn table(self) -> &'static [(&'static str, &'static [Rule])] {
        match self {
            Self::Cardinal => cldr48::CARDINAL,
            Self::Ordinal => cldr48::ORDINAL,
        }
    }
}

impl core::fmt::Display for PluralType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The CLDR plural operands of one source number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PluralOperands {
    integer: u64,
    visible_fraction_digits: u32,
    significant_fraction_digits: u32,
    fraction: u64,
    significant_fraction: u64,
}

impl PluralOperands {
    /// Operands for an integer, with no visible fraction digits.
    #[must_use]
    pub const fn from_integer(value: i64) -> Self {
        Self {
            integer: value.unsigned_abs(),
            visible_fraction_digits: 0,
            significant_fraction_digits: 0,
            fraction: 0,
            significant_fraction: 0,
        }
    }

    /// Operands for a number written with exactly `v` fraction digits.
    ///
    /// `fraction` is read as those `v` digits: `from_parts(1, 2, 30)` is the
    /// source string `1.30`.
    #[must_use]
    pub fn from_parts(integer: u64, visible_fraction_digits: u32, fraction: u64) -> Self {
        let mut significant = fraction;
        let mut significant_digits = visible_fraction_digits;
        while significant_digits > 0 && significant.is_multiple_of(10) {
            significant /= 10;
            significant_digits -= 1;
        }
        if significant == 0 {
            significant_digits = 0;
        }
        Self {
            integer,
            visible_fraction_digits,
            significant_fraction_digits: significant_digits,
            fraction,
            significant_fraction: significant,
        }
    }

    /// Operands read from a decimal source string such as `"1.30"`.
    ///
    /// The string is the input on purpose: `1`, `1.0` and `1.00` are three
    /// different plural questions and only the written form distinguishes
    /// them.
    ///
    /// # Errors
    ///
    /// Returns [`I18nError::InvalidNumber`] if the text is not a plain
    /// decimal number, and [`I18nError::NumberOutOfRange`] if either part
    /// does not fit in a `u64`.
    pub fn parse(text: &str) -> I18nResult<Self> {
        let body = text.strip_prefix('-').unwrap_or(text);
        let (integer_part, fraction_part) = match body.split_once('.') {
            Some((integer, fraction)) => (integer, fraction),
            None => (body, ""),
        };
        if integer_part.is_empty() || !integer_part.bytes().all(|b| b.is_ascii_digit()) {
            return Err(I18nError::InvalidNumber);
        }
        if !fraction_part.bytes().all(|b| b.is_ascii_digit()) {
            return Err(I18nError::InvalidNumber);
        }
        if body.contains('.') && fraction_part.is_empty() {
            return Err(I18nError::InvalidNumber);
        }
        if fraction_part.len() > 19 {
            return Err(I18nError::NumberOutOfRange);
        }
        let integer = integer_part
            .parse::<u64>()
            .map_err(|_| I18nError::NumberOutOfRange)?;
        let fraction = if fraction_part.is_empty() {
            0
        } else {
            fraction_part
                .parse::<u64>()
                .map_err(|_| I18nError::NumberOutOfRange)?
        };
        Ok(Self::from_parts(
            integer,
            fraction_part.len() as u32,
            fraction,
        ))
    }

    /// Operand `i`: the integer digits.
    #[must_use]
    pub const fn i(&self) -> u64 {
        self.integer
    }

    /// Operand `v`: visible fraction digits, trailing zeros included.
    #[must_use]
    pub const fn v(&self) -> u32 {
        self.visible_fraction_digits
    }

    /// Operand `w`: visible fraction digits, trailing zeros excluded.
    #[must_use]
    pub const fn w(&self) -> u32 {
        self.significant_fraction_digits
    }

    /// Operand `f`: the fraction digits as an integer, zeros included.
    #[must_use]
    pub const fn f(&self) -> u64 {
        self.fraction
    }

    /// Operand `t`: the fraction digits as an integer, zeros excluded.
    #[must_use]
    pub const fn t(&self) -> u64 {
        self.significant_fraction
    }

    /// Operands `c` and `e`: the compact decimal exponent, which this crate
    /// does not read, so always 0.
    #[must_use]
    pub const fn c(&self) -> u64 {
        0
    }

    /// Operand `n`, as the nearest `f64`.
    ///
    /// The rules themselves never use this — they work on the exact integer
    /// operands — so the rounding here cannot change a category.
    #[must_use]
    pub fn n(&self) -> f64 {
        let mut divisor = 1u64;
        for _ in 0..self.visible_fraction_digits {
            divisor = divisor.saturating_mul(10);
        }
        self.integer as f64 + self.fraction as f64 / divisor as f64
    }

    /// Whether `n` is an integer, that is whether the fraction is zero.
    #[must_use]
    pub const fn n_is_integer(&self) -> bool {
        self.fraction == 0
    }
}

impl From<i64> for PluralOperands {
    fn from(value: i64) -> Self {
        Self::from_integer(value)
    }
}

impl From<u64> for PluralOperands {
    fn from(value: u64) -> Self {
        Self {
            integer: value,
            visible_fraction_digits: 0,
            significant_fraction_digits: 0,
            fraction: 0,
            significant_fraction: 0,
        }
    }
}

/// An operand of a relation, as UTS #35 names them; `C` stands for both
/// `c` and `e`, which the specification makes synonyms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Operand {
    N,
    I,
    V,
    /// No rule of CLDR 48 reads `w`; it is here because the syntax has it.
    #[allow(dead_code)]
    W,
    F,
    T,
    C,
}

/// One relation of a rule: `operand [% modulus] (= | !=) ranges`.
///
/// `n % 10 = 1` is true of a number whose fraction is zero and whose integer
/// digits leave 1 modulo 10, because a range in CLDR's syntax matches
/// integers alone: 21.33 modulo 10 is 1.33, which no range holds. `!=`
/// negates the whole relation, so `n != 1` is true of 1.5.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Relation {
    pub(crate) operand: Operand,
    /// 0 for none.
    pub(crate) modulus: u64,
    pub(crate) negated: bool,
    /// Inclusive `(low, high)` pairs; a value is `(v, v)`.
    pub(crate) ranges: &'static [(u64, u64)],
}

impl Relation {
    fn holds(&self, operands: &PluralOperands) -> bool {
        let (value, integral) = match self.operand {
            Operand::N => (operands.i(), operands.n_is_integer()),
            Operand::I => (operands.i(), true),
            Operand::V => (u64::from(operands.v()), true),
            Operand::W => (u64::from(operands.w()), true),
            Operand::F => (operands.f(), true),
            Operand::T => (operands.t(), true),
            Operand::C => (operands.c(), true),
        };
        let remainder = if self.modulus == 0 {
            value
        } else {
            value % self.modulus
        };
        let within = integral
            && self
                .ranges
                .iter()
                .any(|(low, high)| remainder >= *low && remainder <= *high);
        within != self.negated
    }
}

/// A block's samples for the differential test: (category, `@integer`
/// samples, `@decimal` samples), as the file writes them.
#[cfg(test)]
pub(crate) type Samples = &'static [(PluralCategory, &'static str, &'static str)];

/// One category's rule: its condition, an `or` of `and`s of relations.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Rule {
    pub(crate) category: PluralCategory,
    pub(crate) condition: &'static [&'static [Relation]],
}

impl Rule {
    fn holds(&self, operands: &PluralOperands) -> bool {
        self.condition
            .iter()
            .any(|group| group.iter().all(|relation| relation.holds(operands)))
    }
}

/// The plural rules of one language, of one kind.
#[derive(Debug, Clone, Copy)]
pub struct PluralRules {
    language: &'static str,
    kind: PluralType,
    rules: &'static [Rule],
}

impl PluralRules {
    /// The cardinal rules for a language subtag, or for one of the regional
    /// rows such as `pt-PT`, if CLDR 48 carries them.
    #[must_use]
    pub fn for_language(language: &str) -> Option<Self> {
        Self::of_kind_for_language(PluralType::Cardinal, language)
    }

    /// The cardinal rules for a locale, walking its fallback chain.
    ///
    /// Each step of the chain is matched against the whole row key first,
    /// so `pt-PT` finds its own row before `pt`. A language CLDR 48 has no
    /// rules for gets the root behaviour: everything is `other`.
    #[must_use]
    pub fn for_locale(locale: &Locale) -> Self {
        Self::of_kind_for_locale(PluralType::Cardinal, locale)
    }

    /// The rules of a kind for a language subtag or regional row.
    #[must_use]
    pub fn of_kind_for_language(kind: PluralType, language: &str) -> Option<Self> {
        kind.table()
            .binary_search_by(|(subtag, _)| subtag.cmp(&language))
            .ok()
            .map(|index| {
                let (subtag, rules) = kind.table()[index];
                Self {
                    language: subtag,
                    kind,
                    rules,
                }
            })
    }

    /// The rules of a kind for a locale, walking its fallback chain as
    /// [`PluralRules::for_locale`] does.
    #[must_use]
    pub fn of_kind_for_locale(kind: PluralType, locale: &Locale) -> Self {
        for candidate in locale.fallback() {
            let identity = candidate.without_extensions();
            if let Some((subtag, rules)) = kind
                .table()
                .iter()
                .find(|(subtag, _)| subtag.contains('-') && identity.matches_tag(subtag))
            {
                return Self {
                    language: subtag,
                    kind,
                    rules,
                };
            }
            if let Some(rules) = Self::of_kind_for_language(kind, candidate.language()) {
                return rules;
            }
        }
        Self {
            language: "und",
            kind,
            rules: &[],
        }
    }

    /// Every language subtag or regional row that has rules of a kind, in
    /// sorted order.
    pub fn languages(kind: PluralType) -> impl Iterator<Item = &'static str> {
        kind.table().iter().map(|(subtag, _)| *subtag)
    }

    /// The language subtag these rules came from, `und` for root's.
    #[must_use]
    pub const fn language(&self) -> &'static str {
        self.language
    }

    /// Whether these are cardinal or ordinal rules.
    #[must_use]
    pub const fn kind(&self) -> PluralType {
        self.kind
    }

    /// The categories these rules name, in CLDR order, `Other` last.
    pub fn categories(&self) -> impl Iterator<Item = PluralCategory> + '_ {
        PluralCategory::ALL.into_iter().filter(move |category| {
            *category == PluralCategory::Other
                || self.rules.iter().any(|rule| rule.category == *category)
        })
    }

    /// The category of a set of operands: the first rule whose condition
    /// holds, in the order `zero`, `one`, `two`, `few`, `many`, else
    /// `other`.
    #[must_use]
    pub fn select(&self, operands: &PluralOperands) -> PluralCategory {
        PluralCategory::ALL
            .into_iter()
            .find(|category| {
                self.rules
                    .iter()
                    .any(|rule| rule.category == *category && rule.holds(operands))
            })
            .unwrap_or(PluralCategory::Other)
    }

    /// The category of an integer.
    #[must_use]
    pub fn select_integer(&self, value: i64) -> PluralCategory {
        self.select(&PluralOperands::from_integer(value))
    }

    /// The category of a decimal source string.
    ///
    /// # Errors
    ///
    /// As [`PluralOperands::parse`].
    pub fn select_decimal(&self, text: &str) -> I18nResult<PluralCategory> {
        Ok(self.select(&PluralOperands::parse(text)?))
    }
}

#[cfg(test)]
mod tests {
    use super::PluralCategory::{Few, Many, One, Other, Two, Zero};
    use super::*;

    /// Assert the CLDR sample values of one language, integers and decimals
    /// alike. Every expectation here is a sample from the language's own
    /// `plurals.xml` rule, not a value derived from this implementation.
    fn assert_samples(language: &str, samples: &[(PluralCategory, &[&str])]) {
        let rules = PluralRules::for_language(language)
            .unwrap_or_else(|| panic!("no rules for {language}"));
        for (expected, values) in samples {
            for value in *values {
                let actual = rules.select_decimal(value).unwrap();
                assert_eq!(
                    actual, *expected,
                    "{language}: {value} should be {expected}, got {actual}"
                );
            }
        }
    }

    #[test]
    fn operands_come_from_the_written_form_not_the_value() {
        let plain = PluralOperands::parse("1").unwrap();
        assert_eq!(
            (plain.i(), plain.v(), plain.w(), plain.f(), plain.t()),
            (1, 0, 0, 0, 0)
        );
        let one_zero = PluralOperands::parse("1.0").unwrap();
        assert_eq!(
            (
                one_zero.i(),
                one_zero.v(),
                one_zero.w(),
                one_zero.f(),
                one_zero.t()
            ),
            (1, 1, 0, 0, 0)
        );
        let padded = PluralOperands::parse("1.30").unwrap();
        assert_eq!(
            (padded.i(), padded.v(), padded.w(), padded.f(), padded.t()),
            (1, 2, 1, 30, 3)
        );
        let long = PluralOperands::parse("-1234.5060").unwrap();
        assert_eq!(
            (long.i(), long.v(), long.w(), long.f(), long.t()),
            (1234, 4, 3, 5060, 506)
        );
        assert_eq!(long.c(), 0);
    }

    #[test]
    fn the_n_operand_is_the_absolute_value() {
        assert!((PluralOperands::parse("1.30").unwrap().n() - 1.3).abs() < 1e-12);
        assert!((PluralOperands::from_integer(-7).n() - 7.0).abs() < 1e-12);
        assert!(PluralOperands::parse("2.00").unwrap().n_is_integer());
        assert!(!PluralOperands::parse("2.01").unwrap().n_is_integer());
    }

    #[test]
    fn malformed_decimal_strings_are_rejected() {
        assert_eq!(PluralOperands::parse(""), Err(I18nError::InvalidNumber));
        assert_eq!(PluralOperands::parse("1."), Err(I18nError::InvalidNumber));
        assert_eq!(
            PluralOperands::parse("1.2.3"),
            Err(I18nError::InvalidNumber)
        );
        assert_eq!(PluralOperands::parse("one"), Err(I18nError::InvalidNumber));
        assert_eq!(
            PluralOperands::parse("1.00000000000000000000"),
            Err(I18nError::NumberOutOfRange)
        );
    }

    #[test]
    fn english_makes_one_singular_and_one_point_zero_plural() {
        assert_samples(
            "en",
            &[
                (One, &["1"]),
                (Other, &["0", "2", "17", "100", "1.0", "1.00", "0.0", "1.5"]),
            ],
        );
    }

    #[test]
    fn german_dutch_swedish_and_finnish_follow_the_english_shape() {
        for language in ["de", "nl", "sv", "fi"] {
            assert_samples(
                language,
                &[(One, &["1"]), (Other, &["0", "2", "1.0", "10"])],
            );
        }
    }

    #[test]
    fn danish_counts_a_fraction_of_one_as_singular() {
        assert_samples(
            "da",
            &[
                (One, &["1", "0.1", "1.6", "0.8", "1.1", "1.0"]),
                (Other, &["0", "2", "2.0", "10", "0.0"]),
            ],
        );
    }

    #[test]
    fn languages_without_numeral_agreement_answer_other_to_everything() {
        for language in ["ja", "zh", "ko", "th", "vi", "id"] {
            assert_samples(
                language,
                &[(Other, &["0", "1", "2", "5", "11", "100", "1.5"])],
            );
        }
    }

    #[test]
    fn arabic_uses_all_six_categories() {
        assert_samples(
            "ar",
            &[
                (Zero, &["0"]),
                (One, &["1"]),
                (Two, &["2"]),
                (Few, &["3", "4", "10", "103", "110", "1003"]),
                (Many, &["11", "12", "26", "99", "111", "1011"]),
                (Other, &["100", "101", "102", "200", "1000", "0.1", "1.5"]),
            ],
        );
        let arabic = PluralRules::for_language("ar").unwrap();
        assert_eq!(
            arabic.categories().collect::<Vec<_>>(),
            [Zero, One, Two, Few, Many, Other]
        );
        assert_eq!(
            PluralRules::for_language("ja")
                .unwrap()
                .categories()
                .collect::<Vec<_>>(),
            [Other]
        );
    }

    #[test]
    fn russian_and_ukrainian_share_the_east_slavic_three_way_split() {
        for language in ["ru", "uk"] {
            assert_samples(
                language,
                &[
                    (One, &["1", "21", "31", "41", "101", "1001"]),
                    (Few, &["2", "3", "4", "22", "23", "24", "102", "1002"]),
                    (
                        Many,
                        &[
                            "0", "5", "6", "9", "11", "12", "13", "14", "19", "100", "1000",
                        ],
                    ),
                    (Other, &["1.0", "1.5", "0.1", "2.0"]),
                ],
            );
        }
    }

    #[test]
    fn polish_differs_from_russian_at_one_hundred_and_one() {
        assert_samples(
            "pl",
            &[
                (One, &["1"]),
                (Few, &["2", "3", "4", "22", "23", "24", "102", "1002"]),
                (
                    Many,
                    &[
                        "0", "5", "9", "11", "12", "14", "21", "26", "100", "101", "1000",
                    ],
                ),
                (Other, &["0.0", "1.0", "1.5", "2.0"]),
            ],
        );
    }

    #[test]
    fn czech_puts_every_fraction_in_many() {
        assert_samples(
            "cs",
            &[
                (One, &["1"]),
                (Few, &["2", "3", "4"]),
                (Many, &["0.0", "0.1", "1.0", "1.5", "10.0"]),
                (Other, &["0", "5", "9", "11", "100", "1000"]),
            ],
        );
    }

    #[test]
    fn welsh_exercises_every_category_including_six() {
        assert_samples(
            "cy",
            &[
                (Zero, &["0"]),
                (One, &["1"]),
                (Two, &["2"]),
                (Few, &["3"]),
                (Many, &["6"]),
                (Other, &["4", "5", "7", "10", "100", "1.5", "0.5"]),
            ],
        );
    }

    #[test]
    fn irish_has_a_paucal_and_a_many_that_stop_at_ten() {
        assert_samples(
            "ga",
            &[
                (One, &["1"]),
                (Two, &["2"]),
                (Few, &["3", "4", "5", "6"]),
                (Many, &["7", "8", "9", "10"]),
                (Other, &["0", "11", "100", "1.5"]),
            ],
        );
    }

    #[test]
    fn slovenian_keeps_the_dual_on_every_hundred() {
        assert_samples(
            "sl",
            &[
                (One, &["1", "101", "201", "1001"]),
                (Two, &["2", "102", "202", "1002"]),
                (Few, &["3", "4", "103", "104", "0.0", "1.5"]),
                (Other, &["0", "5", "6", "100", "105"]),
            ],
        );
    }

    #[test]
    fn latvian_has_a_zero_for_every_multiple_of_ten() {
        assert_samples(
            "lv",
            &[
                (
                    Zero,
                    &[
                        "0", "10", "11", "12", "19", "20", "30", "100", "0.0", "10.0",
                    ],
                ),
                (
                    One,
                    &["1", "21", "31", "101", "1031", "0.1", "1.1", "0.031"],
                ),
                (Other, &["2", "3", "9", "22", "102", "0.2", "1.2"]),
            ],
        );
    }

    #[test]
    fn lithuanian_sends_every_fraction_to_many() {
        assert_samples(
            "lt",
            &[
                (One, &["1", "21", "31", "101", "1001", "1.0", "21.0"]),
                (Few, &["2", "9", "22", "29", "102", "1002"]),
                (Many, &["0.1", "1.5", "10.1"]),
                (Other, &["0", "10", "11", "19", "20", "100"]),
            ],
        );
    }

    #[test]
    fn romanian_marks_the_de_form_as_few() {
        assert_samples(
            "ro",
            &[
                (One, &["1"]),
                (
                    Few,
                    &[
                        "0", "2", "12", "16", "19", "101", "119", "1001", "1.0", "1.5",
                    ],
                ),
                (Other, &["20", "21", "100", "120", "1000"]),
            ],
        );
    }

    #[test]
    fn hebrew_keeps_a_dual_for_exactly_two() {
        assert_samples(
            "he",
            &[
                (One, &["1", "0.1", "0.5"]),
                (Two, &["2"]),
                (Other, &["0", "3", "20", "1.0", "2.0", "10"]),
            ],
        );
    }

    #[test]
    fn hindi_treats_zero_and_fractions_below_one_as_singular() {
        assert_samples(
            "hi",
            &[
                (One, &["0", "1", "0.0", "0.5", "0.9", "1.0"]),
                (Other, &["2", "1.5", "10", "100"]),
            ],
        );
    }

    #[test]
    fn french_counts_zero_as_singular_and_exact_millions_as_many() {
        assert_samples(
            "fr",
            &[
                (One, &["0", "1", "0.5", "1.5"]),
                (Many, &["1000000", "2000000", "3000000"]),
                (Other, &["2", "17", "100", "1000001", "2.5"]),
            ],
        );
    }

    #[test]
    fn spanish_italian_and_portuguese_differ_only_in_their_singular() {
        assert_samples(
            "es",
            &[
                (One, &["1", "1.0", "1.00"]),
                (Many, &["1000000"]),
                (Other, &["0", "2", "1.5"]),
            ],
        );
        assert_samples(
            "it",
            &[
                (One, &["1"]),
                (Many, &["1000000"]),
                (Other, &["0", "1.0", "2"]),
            ],
        );
        assert_samples(
            "pt",
            &[
                (One, &["0", "1", "0.5", "1.5"]),
                (Many, &["1000000"]),
                (Other, &["2", "10", "2.5"]),
            ],
        );
    }

    #[test]
    fn turkish_is_singular_only_for_exactly_one() {
        assert_samples(
            "tr",
            &[(One, &["1", "1.0"]), (Other, &["0", "2", "1.5", "0.0"])],
        );
    }

    /// Languages the earlier hand-written table did not carry, with CLDR
    /// 48's samples: Greek and Hungarian `n = 1`; Icelandic's `one` for
    /// every number ending in 1 but 11; Maltese's four categories; Breton's
    /// five; Scottish Gaelic's `one` for 1 and 11 and `two` for 2 and 12.
    #[test]
    fn the_languages_beyond_the_old_table_follow_cldrs_samples() {
        for language in ["el", "hu"] {
            assert_samples(
                language,
                &[(One, &["1", "1.0"]), (Other, &["0", "2", "0.5", "1.5"])],
            );
        }
        assert_samples(
            "is",
            &[
                (One, &["1", "21", "31", "101", "0.1", "1.1", "21.1"]),
                (Other, &["0", "2", "11", "100", "0.0", "0.2", "11.0"]),
            ],
        );
        assert_samples(
            "mt",
            &[
                (One, &["1"]),
                (Two, &["2"]),
                (Few, &["0", "3", "4", "10", "103", "1003"]),
                (Many, &["11", "12", "19", "111", "1011"]),
                (Other, &["20", "21", "100", "1000", "0.5", "1.5"]),
            ],
        );
        assert_samples(
            "gd",
            &[
                (One, &["1", "11"]),
                (Two, &["2", "12"]),
                (Few, &["3", "10", "13", "19"]),
                (Other, &["0", "20", "100", "0.5"]),
            ],
        );
        assert_samples(
            "br",
            &[
                (One, &["1", "21", "31", "101"]),
                (Two, &["2", "22", "32", "102"]),
                (Few, &["3", "4", "9", "23", "103"]),
                (Many, &["1000000"]),
                (Other, &["0", "5", "11", "71", "91", "100", "1.5"]),
            ],
        );
    }

    /// CLDR 48's `ordinals.xml`: English's four forms, Welsh's six, and
    /// the languages whose positions take one form.
    #[test]
    fn the_ordinal_rules_choose_the_form_of_a_position() {
        let english = PluralRules::of_kind_for_language(PluralType::Ordinal, "en").unwrap();
        assert_eq!(english.kind(), PluralType::Ordinal);
        assert_eq!(english.language(), "en");
        for (value, expected) in [
            (1, One),
            (2, Two),
            (3, Few),
            (4, Other),
            (11, Other),
            (12, Other),
            (13, Other),
            (21, One),
            (22, Two),
            (23, Few),
            (101, One),
            (111, Other),
        ] {
            assert_eq!(english.select_integer(value), expected, "en {value}");
        }
        assert_eq!(
            english.categories().collect::<Vec<_>>(),
            [One, Two, Few, Other]
        );
        let welsh =
            PluralRules::of_kind_for_locale(PluralType::Ordinal, &Locale::parse("cy-GB").unwrap());
        for (value, expected) in [
            (0, Zero),
            (7, Zero),
            (8, Zero),
            (9, Zero),
            (1, One),
            (2, Two),
            (3, Few),
            (4, Few),
            (5, Many),
            (6, Many),
            (10, Other),
        ] {
            assert_eq!(welsh.select_integer(value), expected, "cy {value}");
        }
        let french = PluralRules::of_kind_for_language(PluralType::Ordinal, "fr").unwrap();
        assert_eq!(french.select_integer(1), One);
        assert_eq!(french.select_integer(2), Other);
        let italian = PluralRules::of_kind_for_language(PluralType::Ordinal, "it").unwrap();
        assert_eq!(italian.select_integer(8), Many);
        assert_eq!(italian.select_integer(11), Many);
        assert_eq!(italian.select_integer(3), Other);
        let german = PluralRules::of_kind_for_language(PluralType::Ordinal, "de").unwrap();
        assert_eq!(german.select_integer(1), Other);
        assert_eq!(
            PluralRules::of_kind_for_locale(PluralType::Ordinal, &Locale::parse("ru").unwrap())
                .select_integer(1),
            Other
        );
        // Portugal's cardinal row has no ordinal counterpart: `pt-PT`
        // takes Portuguese's ordinal rules.
        let portugal =
            PluralRules::of_kind_for_locale(PluralType::Ordinal, &Locale::parse("pt-PT").unwrap());
        assert_eq!(portugal.language(), "pt");
    }

    #[test]
    fn portugal_takes_its_own_row_and_brazil_the_language_one() {
        let portugal = PluralRules::for_locale(&Locale::parse("pt-PT").unwrap());
        assert_eq!(portugal.language(), "pt-PT");
        // CLDR 48 `pt_PT`: `one` is `i = 1 and v = 0` only.
        assert_eq!(portugal.select_integer(0), Other);
        assert_eq!(portugal.select_integer(1), One);
        assert_eq!(portugal.select_decimal("1.5").unwrap(), Other);
        assert_eq!(portugal.select_integer(1_000_000), Many);
        let brazil = PluralRules::for_locale(&Locale::parse("pt-BR-u-ca-gregory").unwrap());
        assert_eq!(brazil.language(), "pt");
        assert_eq!(brazil.select_integer(0), One);
        let extended = PluralRules::for_locale(&Locale::parse("pt-PT-u-ca-gregory").unwrap());
        assert_eq!(extended.language(), "pt-PT");
    }

    #[test]
    fn a_locale_finds_its_rules_through_the_fallback_chain() {
        let locale = Locale::parse("ru-RU-u-ca-gregory").unwrap();
        assert_eq!(PluralRules::for_locale(&locale).language(), "ru");
        assert_eq!(PluralRules::for_locale(&locale).select_integer(5), Many);
    }

    #[test]
    fn an_unknown_language_gets_the_root_rule() {
        let locale = Locale::parse("xx-YY").unwrap();
        let rules = PluralRules::for_locale(&locale);
        assert_eq!(rules.language(), "und");
        assert_eq!(rules.kind(), PluralType::Cardinal);
        assert_eq!(rules.select_integer(1), Other);
        assert_eq!(rules.categories().collect::<Vec<_>>(), [Other]);
        assert!(PluralRules::for_language("xx").is_none());
        assert!(PluralRules::for_language("root").is_none());
    }

    #[test]
    fn the_rule_tables_are_sorted_and_free_of_duplicates() {
        for kind in PluralType::ALL {
            let languages: Vec<_> = PluralRules::languages(kind).collect();
            for pair in languages.windows(2) {
                assert!(pair[0] < pair[1], "{kind}: {} !< {}", pair[0], pair[1]);
            }
        }
        // CLDR 48's `plurals.xml` lists 227 locales and `ordinals.xml` 110,
        // `root` among each.
        assert_eq!(PluralRules::languages(PluralType::Cardinal).count(), 226);
        assert_eq!(PluralRules::languages(PluralType::Ordinal).count(), 109);
        assert!(PluralRules::languages(PluralType::Cardinal).any(|l| l == "pt-PT"));
    }

    #[test]
    fn category_and_kind_keywords_round_trip() {
        for category in PluralCategory::ALL {
            assert_eq!(
                PluralCategory::from_keyword(category.as_str()),
                Some(category)
            );
        }
        assert!(PluralCategory::from_keyword("plural").is_none());
        for kind in PluralType::ALL {
            assert_eq!(PluralType::from_keyword(kind.as_str()), Some(kind));
        }
        assert_eq!(
            PluralType::from_keyword("ORDINAL"),
            Some(PluralType::Ordinal)
        );
        assert!(PluralType::from_keyword("fractions").is_none());
    }

    /// Expand one `@integer` or `@decimal` sample list of `plurals.xml` or
    /// `ordinals.xml`: values and `low~high` ranges, whose step is the
    /// place of the endpoints' last digit, the trailing `…` dropped and a
    /// compact sample such as `1c6` skipped, since the exponent operands
    /// are not carried.
    fn expand_samples(text: &str) -> Vec<String> {
        let mut out = Vec::new();
        for item in text.split(',') {
            let item = item.trim();
            if item.is_empty() || item == "…" || item.contains(['c', 'e']) {
                continue;
            }
            let (low, high) = match item.split_once('~') {
                Some((low, high)) => (low.trim(), high.trim()),
                None => (item, item),
            };
            let places = low.split_once('.').map_or(0, |(_, f)| f.len());
            assert_eq!(
                high.split_once('.').map_or(0, |(_, f)| f.len()),
                places,
                "{item}"
            );
            let scaled = |s: &str| s.replace('.', "").parse::<u64>().unwrap();
            for value in scaled(low)..=scaled(high) {
                if places == 0 {
                    out.push(value.to_string());
                } else {
                    let divisor = 10u64.pow(places as u32);
                    out.push(format!(
                        "{}.{:0places$}",
                        value / divisor,
                        value % divisor,
                        places = places
                    ));
                }
            }
        }
        out
    }

    /// Every block's samples, `@integer` and `@decimal`, against the
    /// evaluator: the differential test of the generated rules against
    /// CLDR 48's own sample ranges.
    #[test]
    fn every_block_answers_cldrs_own_samples() {
        let mut checked = 0;
        for (kind, table) in [
            (PluralType::Cardinal, cldr48::CARDINAL_SAMPLES),
            (PluralType::Ordinal, cldr48::ORDINAL_SAMPLES),
        ] {
            for (language, samples) in table {
                let rules = PluralRules::of_kind_for_language(kind, language)
                    .unwrap_or_else(|| panic!("{kind}: no rules for {language}"));
                for (expected, integer, decimal) in *samples {
                    for sample in expand_samples(integer)
                        .into_iter()
                        .chain(expand_samples(decimal))
                    {
                        let actual = rules.select_decimal(&sample).unwrap();
                        assert_eq!(
                            actual, *expected,
                            "{kind} {language}: {sample} should be {expected}, got {actual}"
                        );
                        checked += 1;
                    }
                }
            }
        }
        // 3 767 samples on 2026-10-04.
        assert!(checked > 3_000, "{checked} samples");
    }

    #[test]
    fn samples_expand_as_the_specification_says() {
        assert_eq!(expand_samples("0, 2~4, 1c6, …"), ["0", "2", "3", "4"]);
        assert_eq!(
            expand_samples("0.0~0.3, 1.5, 10.0, …"),
            ["0.0", "0.1", "0.2", "0.3", "1.5", "10.0"]
        );
        assert_eq!(expand_samples("1.00~1.02"), ["1.00", "1.01", "1.02"]);
    }
}
