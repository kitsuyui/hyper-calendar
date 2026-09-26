//! CLDR cardinal plural rules.
//!
//! The rule text is that of Unicode CLDR 48, `common/supplemental/
//! plurals.xml` (`cldr48-supplemental`, tag `release-48`, retrieved
//! 2026-09-26); the operands and rule syntax are those of UTS #35 version
//! 48, Part 3 (Numbers), §5 "Language Plural Rules" (`uts35-v48`).
//!
//! "3 days" is easy; "3 дня", "5 дней" and "21 день" are why this module
//! exists. A formatter that wants to say "in 2 months" has to ask the
//! language which of up to six forms the surrounding message should use, and
//! the answer depends on more than the value: `1`, `1.0` and `1.00` take
//! different forms in several languages.
//!
//! # Operands
//!
//! UTS #35 Part 3 §5.1 defines six operands on the *source string*, not on
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
//!
//! So `1.0` has `i=1, v=1, w=0, f=0, t=0`, and English calls it `other`
//! while plain `1` is `one`. [`PluralOperands::parse`] preserves that
//! distinction; [`PluralOperands::from_integer`] cannot, because an `i64`
//! has no trailing zeros to preserve.
//!
//! The compact-notation operands `c` and `e` (CLDR 38 and later) are **not**
//! modelled. Where a rule has an `e = 0 and …` branch, the `e = 0` case is
//! implemented and the `e != 0` alternatives are dropped; this only affects
//! compact forms such as "1M", which this workspace never produces.

use crate::error::{I18nError, I18nResult};
use crate::locale::Locale;

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

    /// `n = value`, exactly.
    const fn n_is(&self, value: u64) -> bool {
        self.fraction == 0 && self.integer == value
    }

    /// `n = low..high`, which in CLDR range notation only matches integers.
    const fn n_in(&self, low: u64, high: u64) -> bool {
        self.fraction == 0 && self.integer >= low && self.integer <= high
    }

    /// `n % modulus = low..high`.
    const fn n_mod_in(&self, modulus: u64, low: u64, high: u64) -> bool {
        if self.fraction != 0 {
            return false;
        }
        let remainder = self.integer % modulus;
        remainder >= low && remainder <= high
    }

    /// `i % modulus = low..high`.
    const fn i_mod_in(&self, modulus: u64, low: u64, high: u64) -> bool {
        let remainder = self.integer % modulus;
        remainder >= low && remainder <= high
    }

    /// The `many` branch shared by French, Italian, Spanish and Portuguese,
    /// which marks exact millions: "3 millions de…".
    ///
    /// CLDR writes it `e = 0 and i != 0 and i % 1000000 = 0 and v = 0`; the
    /// `e != 0` alternative belongs to compact notation and is not modelled.
    const fn is_exact_million(&self) -> bool {
        self.integer != 0
            && self.integer.is_multiple_of(1_000_000)
            && self.visible_fraction_digits == 0
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

/// The shape of a plural rule: operands in, category out.
type RuleFn = fn(&PluralOperands) -> PluralCategory;

/// The plural rules of one language.
#[derive(Debug, Clone, Copy)]
pub struct PluralRules {
    language: &'static str,
    rule: RuleFn,
}

impl PluralRules {
    /// The rules for a language subtag, or for one of the regional rows
    /// such as `pt-PT`, if this crate carries them.
    #[must_use]
    pub fn for_language(language: &str) -> Option<Self> {
        RULES
            .iter()
            .find(|(subtag, _)| *subtag == language)
            .map(|(subtag, rule)| Self {
                language: subtag,
                rule: *rule,
            })
    }

    /// The rules for a locale, walking its fallback chain.
    ///
    /// Each step of the chain is matched against the whole row key first,
    /// so `pt-PT` finds its own row before `pt`. A language this crate has
    /// no rules for gets the root behaviour, which is CLDR's: everything is
    /// `other`.
    #[must_use]
    pub fn for_locale(locale: &Locale) -> Self {
        for candidate in locale.fallback() {
            let identity = candidate.without_extensions();
            if let Some((subtag, rule)) = RULES
                .iter()
                .find(|(subtag, _)| subtag.contains('-') && identity.matches_tag(subtag))
            {
                return Self {
                    language: subtag,
                    rule: *rule,
                };
            }
            if let Some(rules) = Self::for_language(candidate.language()) {
                return rules;
            }
        }
        Self {
            language: "und",
            rule: rule_other_only,
        }
    }

    /// The language subtag these rules came from.
    #[must_use]
    pub const fn language(&self) -> &'static str {
        self.language
    }

    /// The category of a set of operands.
    #[must_use]
    pub fn select(&self, operands: &PluralOperands) -> PluralCategory {
        (self.rule)(operands)
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

/// Every language whose cardinal rules this crate carries.
///
/// Languages sharing a rule share its function: CLDR's own rule text for
/// Russian and Ukrainian is identical, and so is the text for the whole
/// `i = 1 and v = 0` group. Adding a language is one row here.
///
/// Every locale with names in [`crate::data::LOCALES`] has a row here when
/// CLDR 48 lists its language in `plurals.xml`. Balinese, Coptic,
/// Mandaic, Sanskrit, Yucatec Maya, Zapotec and Standard Moroccan Tamazight
/// are not listed there, so they take root's rule, which is also `other`
/// for everything. `pt-PT` is the one regional row: CLDR 48 gives Portugal
/// the Italian rule, not Brazil's `i = 0..1`.
pub static RULES: &[(&str, RuleFn)] = &[
    ("am", rule_hindi),
    ("ar", rule_arabic),
    ("bn", rule_hindi),
    ("bo", rule_other_only),
    ("cs", rule_czech),
    ("cy", rule_welsh),
    ("da", rule_danish),
    ("de", rule_one_if_i_is_one_and_v_is_zero),
    ("en", rule_one_if_i_is_one_and_v_is_zero),
    ("es", rule_spanish),
    ("fa", rule_hindi),
    ("fi", rule_one_if_i_is_one_and_v_is_zero),
    ("fr", rule_french),
    ("ga", rule_irish),
    ("he", rule_hebrew),
    ("hi", rule_hindi),
    ("id", rule_other_only),
    ("it", rule_italian),
    ("ja", rule_other_only),
    ("jv", rule_other_only),
    ("kab", rule_one_if_i_is_zero_or_one),
    ("ko", rule_other_only),
    ("lt", rule_lithuanian),
    ("lv", rule_latvian),
    ("ml", rule_one_if_n_is_one),
    ("my", rule_other_only),
    ("nah", rule_one_if_n_is_one),
    ("ne", rule_one_if_n_is_one),
    ("nl", rule_one_if_i_is_one_and_v_is_zero),
    ("pl", rule_polish),
    ("ps", rule_one_if_n_is_one),
    ("pt", rule_portuguese),
    ("pt-PT", rule_italian),
    ("ro", rule_romanian),
    ("ru", rule_east_slavic),
    ("sl", rule_slovenian),
    ("sv", rule_one_if_i_is_one_and_v_is_zero),
    ("syr", rule_one_if_n_is_one),
    ("ta", rule_one_if_n_is_one),
    ("th", rule_other_only),
    ("tr", rule_one_if_n_is_one),
    ("uk", rule_east_slavic),
    ("vi", rule_other_only),
    ("zh", rule_other_only),
];

/// Languages with no grammatical number agreement on the numeral.
fn rule_other_only(_operands: &PluralOperands) -> PluralCategory {
    PluralCategory::Other
}

/// `one: i = 1 and v = 0` — English, German, Dutch, Swedish, Finnish.
///
/// The `v = 0` is what makes "1.0 days" plural in English.
fn rule_one_if_i_is_one_and_v_is_zero(operands: &PluralOperands) -> PluralCategory {
    if operands.i() == 1 && operands.v() == 0 {
        PluralCategory::One
    } else {
        PluralCategory::Other
    }
}

/// Danish: `one: n = 1 or t != 0 and i = 0,1`.
fn rule_danish(operands: &PluralOperands) -> PluralCategory {
    if operands.n_is(1) || (operands.t() != 0 && (operands.i() == 0 || operands.i() == 1)) {
        PluralCategory::One
    } else {
        PluralCategory::Other
    }
}

/// `one: n = 1` — Turkish, Malayalam, Nahuatl, Nepali, Pashto, Syriac,
/// Tamil.
fn rule_one_if_n_is_one(operands: &PluralOperands) -> PluralCategory {
    if operands.n_is(1) {
        PluralCategory::One
    } else {
        PluralCategory::Other
    }
}

/// Spanish: `one: n = 1`, plus the exact-million `many`.
fn rule_spanish(operands: &PluralOperands) -> PluralCategory {
    if operands.n_is(1) {
        PluralCategory::One
    } else if operands.is_exact_million() {
        PluralCategory::Many
    } else {
        PluralCategory::Other
    }
}

/// French: `one: i = 0,1`, plus the exact-million `many`.
fn rule_french(operands: &PluralOperands) -> PluralCategory {
    if operands.i() == 0 || operands.i() == 1 {
        PluralCategory::One
    } else if operands.is_exact_million() {
        PluralCategory::Many
    } else {
        PluralCategory::Other
    }
}

/// Kabyle: `one: i = 0,1`, the French `one` without its `many`.
fn rule_one_if_i_is_zero_or_one(operands: &PluralOperands) -> PluralCategory {
    if operands.i() <= 1 {
        PluralCategory::One
    } else {
        PluralCategory::Other
    }
}

/// Portuguese: `one: i = 0..1`, plus the exact-million `many`.
fn rule_portuguese(operands: &PluralOperands) -> PluralCategory {
    if operands.i() <= 1 {
        PluralCategory::One
    } else if operands.is_exact_million() {
        PluralCategory::Many
    } else {
        PluralCategory::Other
    }
}

/// Italian: `one: i = 1 and v = 0`, plus the exact-million `many`.
fn rule_italian(operands: &PluralOperands) -> PluralCategory {
    if operands.i() == 1 && operands.v() == 0 {
        PluralCategory::One
    } else if operands.is_exact_million() {
        PluralCategory::Many
    } else {
        PluralCategory::Other
    }
}

/// Hebrew: `one: i = 1 and v = 0 or i = 0 and v != 0`; `two: i = 2 and v = 0`.
fn rule_hebrew(operands: &PluralOperands) -> PluralCategory {
    if (operands.i() == 1 && operands.v() == 0) || (operands.i() == 0 && operands.v() != 0) {
        PluralCategory::One
    } else if operands.i() == 2 && operands.v() == 0 {
        PluralCategory::Two
    } else {
        PluralCategory::Other
    }
}

/// `one: i = 0 or n = 1` — Hindi, Amharic, Bengali, Persian.
fn rule_hindi(operands: &PluralOperands) -> PluralCategory {
    if operands.i() == 0 || operands.n_is(1) {
        PluralCategory::One
    } else {
        PluralCategory::Other
    }
}

/// Arabic: the full six-way system.
///
/// `zero: n = 0`, `one: n = 1`, `two: n = 2`,
/// `few: n % 100 = 3..10`, `many: n % 100 = 11..99`.
fn rule_arabic(operands: &PluralOperands) -> PluralCategory {
    if operands.n_is(0) {
        PluralCategory::Zero
    } else if operands.n_is(1) {
        PluralCategory::One
    } else if operands.n_is(2) {
        PluralCategory::Two
    } else if operands.n_mod_in(100, 3, 10) {
        PluralCategory::Few
    } else if operands.n_mod_in(100, 11, 99) {
        PluralCategory::Many
    } else {
        PluralCategory::Other
    }
}

/// Russian and Ukrainian, whose CLDR rule text is identical.
///
/// `one: v = 0 and i % 10 = 1 and i % 100 != 11`;
/// `few: v = 0 and i % 10 = 2..4 and i % 100 != 12..14`;
/// `many: v = 0 and (i % 10 = 0 or i % 10 = 5..9 or i % 100 = 11..14)`.
fn rule_east_slavic(operands: &PluralOperands) -> PluralCategory {
    if operands.v() != 0 {
        return PluralCategory::Other;
    }
    if operands.i_mod_in(10, 1, 1) && !operands.i_mod_in(100, 11, 11) {
        PluralCategory::One
    } else if operands.i_mod_in(10, 2, 4) && !operands.i_mod_in(100, 12, 14) {
        PluralCategory::Few
    } else if operands.i_mod_in(10, 0, 0)
        || operands.i_mod_in(10, 5, 9)
        || operands.i_mod_in(100, 11, 14)
    {
        PluralCategory::Many
    } else {
        PluralCategory::Other
    }
}

/// Polish, which differs from Russian in `one` and in the `many` branch.
///
/// `one: i = 1 and v = 0`;
/// `few: v = 0 and i % 10 = 2..4 and i % 100 != 12..14`;
/// `many: v = 0 and i != 1 and i % 10 = 0..1 or v = 0 and i % 10 = 5..9 or
/// v = 0 and i % 100 = 12..14`.
fn rule_polish(operands: &PluralOperands) -> PluralCategory {
    if operands.i() == 1 && operands.v() == 0 {
        return PluralCategory::One;
    }
    if operands.v() != 0 {
        return PluralCategory::Other;
    }
    if operands.i_mod_in(10, 2, 4) && !operands.i_mod_in(100, 12, 14) {
        PluralCategory::Few
    } else if (operands.i() != 1 && operands.i_mod_in(10, 0, 1))
        || operands.i_mod_in(10, 5, 9)
        || operands.i_mod_in(100, 12, 14)
    {
        PluralCategory::Many
    } else {
        PluralCategory::Other
    }
}

/// Czech: `one: i = 1 and v = 0`; `few: i = 2..4 and v = 0`; `many: v != 0`.
///
/// Czech is the language that makes `many` a *fraction* category rather than
/// a large-number one.
fn rule_czech(operands: &PluralOperands) -> PluralCategory {
    if operands.v() != 0 {
        return PluralCategory::Many;
    }
    if operands.i() == 1 {
        PluralCategory::One
    } else if (2..=4).contains(&operands.i()) {
        PluralCategory::Few
    } else {
        PluralCategory::Other
    }
}

/// Romanian: `one: i = 1 and v = 0`;
/// `few: v != 0 or n = 0 or n != 1 and n % 100 = 1..19`.
///
/// The `other` form is the one that takes "de": *21 de zile*, against the
/// `few` form *19 zile*.
fn rule_romanian(operands: &PluralOperands) -> PluralCategory {
    if operands.i() == 1 && operands.v() == 0 {
        return PluralCategory::One;
    }
    if operands.v() != 0 || operands.n_is(0) || (!operands.n_is(1) && operands.n_mod_in(100, 1, 19))
    {
        PluralCategory::Few
    } else {
        PluralCategory::Other
    }
}

/// Lithuanian: `one: n % 10 = 1 and n % 100 != 11..19`;
/// `few: n % 10 = 2..9 and n % 100 != 11..19`; `many: f != 0`.
fn rule_lithuanian(operands: &PluralOperands) -> PluralCategory {
    if operands.n_mod_in(10, 1, 1) && !operands.n_mod_in(100, 11, 19) {
        PluralCategory::One
    } else if operands.n_mod_in(10, 2, 9) && !operands.n_mod_in(100, 11, 19) {
        PluralCategory::Few
    } else if operands.f() != 0 {
        PluralCategory::Many
    } else {
        PluralCategory::Other
    }
}

/// Latvian, the language with a genuine `zero` for every multiple of ten.
///
/// `zero: n % 10 = 0 or n % 100 = 11..19 or v = 2 and f % 100 = 11..19`;
/// `one: n % 10 = 1 and n % 100 != 11 or v = 2 and f % 10 = 1 and
/// f % 100 != 11 or v != 2 and f % 10 = 1`.
///
/// The `one` branch is kept in CLDR's own redundant form rather than the
/// minimal Boolean equivalent, so that it can be read against the published
/// rule text.
#[allow(clippy::nonminimal_bool)]
fn rule_latvian(operands: &PluralOperands) -> PluralCategory {
    let fraction = operands.f();
    let v = operands.v();
    if operands.n_mod_in(10, 0, 0)
        || operands.n_mod_in(100, 11, 19)
        || (v == 2 && (11..=19).contains(&(fraction % 100)))
    {
        PluralCategory::Zero
    } else if (operands.n_mod_in(10, 1, 1) && !operands.n_mod_in(100, 11, 11))
        || (v == 2 && fraction % 10 == 1 && fraction % 100 != 11)
        || (v != 2 && fraction % 10 == 1)
    {
        PluralCategory::One
    } else {
        PluralCategory::Other
    }
}

/// Slovenian, whose dual survives: `one: v = 0 and i % 100 = 1`;
/// `two: v = 0 and i % 100 = 2`; `few: v = 0 and i % 100 = 3..4 or v != 0`.
fn rule_slovenian(operands: &PluralOperands) -> PluralCategory {
    if operands.v() == 0 && operands.i_mod_in(100, 1, 1) {
        PluralCategory::One
    } else if operands.v() == 0 && operands.i_mod_in(100, 2, 2) {
        PluralCategory::Two
    } else if (operands.v() == 0 && operands.i_mod_in(100, 3, 4)) || operands.v() != 0 {
        PluralCategory::Few
    } else {
        PluralCategory::Other
    }
}

/// Irish: `one: n = 1`; `two: n = 2`; `few: n = 3..6`; `many: n = 7..10`.
fn rule_irish(operands: &PluralOperands) -> PluralCategory {
    if operands.n_is(1) {
        PluralCategory::One
    } else if operands.n_is(2) {
        PluralCategory::Two
    } else if operands.n_in(3, 6) {
        PluralCategory::Few
    } else if operands.n_in(7, 10) {
        PluralCategory::Many
    } else {
        PluralCategory::Other
    }
}

/// Welsh, the language that uses all six categories.
///
/// `zero: n = 0`; `one: n = 1`; `two: n = 2`; `few: n = 3`; `many: n = 6`.
/// The `many` for six alone is not a typo: *chwe blwyddyn* mutates where
/// other numbers do not.
fn rule_welsh(operands: &PluralOperands) -> PluralCategory {
    if operands.n_is(0) {
        PluralCategory::Zero
    } else if operands.n_is(1) {
        PluralCategory::One
    } else if operands.n_is(2) {
        PluralCategory::Two
    } else if operands.n_is(3) {
        PluralCategory::Few
    } else if operands.n_is(6) {
        PluralCategory::Many
    } else {
        PluralCategory::Other
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

    #[test]
    fn the_name_locales_take_their_cldr_48_rules() {
        // `i = 0 or n = 1`, samples from CLDR 48 for `am bn fa`.
        for language in ["am", "bn", "fa"] {
            assert_samples(
                language,
                &[
                    (One, &["0", "1", "0.0", "0.5", "1.0", "0.04"]),
                    (Other, &["2", "17", "100", "1.1", "2.6", "10.0"]),
                ],
            );
        }
        // `n = 1`, samples from CLDR 48 for `ml nah ne ps syr ta`.
        for language in ["ml", "nah", "ne", "ps", "syr", "ta"] {
            assert_samples(
                language,
                &[
                    (One, &["1", "1.0", "1.00", "1.000"]),
                    (Other, &["0", "2", "16", "0.9", "1.1", "10.0"]),
                ],
            );
        }
        // `i = 0,1`, samples from CLDR 48 for `kab`.
        assert_samples(
            "kab",
            &[
                (One, &["0", "1", "0.0", "1.5"]),
                (Other, &["2", "17", "100", "2.0", "3.5"]),
            ],
        );
        for language in ["bo", "jv", "my"] {
            assert_samples(language, &[(Other, &["0", "1", "15", "1.0", "1.5"])]);
        }
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
        assert_eq!(rules.select_integer(1), Other);
        assert!(PluralRules::for_language("xx").is_none());
    }

    #[test]
    fn the_rule_table_is_sorted_and_free_of_duplicates() {
        for pair in RULES.windows(2) {
            assert!(pair[0].0 < pair[1].0, "{} !< {}", pair[0].0, pair[1].0);
        }
        assert_eq!(RULES.len(), 44);
    }

    #[test]
    fn category_keywords_round_trip() {
        for category in PluralCategory::ALL {
            assert_eq!(
                PluralCategory::from_keyword(category.as_str()),
                Some(category)
            );
        }
        assert!(PluralCategory::from_keyword("plural").is_none());
    }
}
