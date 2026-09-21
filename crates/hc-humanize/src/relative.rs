//! Relative time: *3 days ago*, *in 2 hours*, *last month*, *yesterday*.
//!
//! # Numeric versus non-numeric
//!
//! CLDR gives a field two parallel vocabularies, and choosing between them
//! is the option `Intl.RelativeTimeFormat` calls `numeric`:
//!
//! | `numeric` | −1 day | −3 days | 0 seconds |
//! |---|---|---|---|
//! | [`Numeric::Always`] | *1 day ago* | *3 days ago* | *in 0 seconds* |
//! | [`Numeric::Auto`] | *yesterday* | *3 days ago* | *now* |
//!
//! `Auto` does **not** mean "be clever": it means "use the special word if
//! the language has one for exactly this offset, otherwise fall back to the
//! numeric pattern". A language with no word for the day before yesterday
//! gets *2 days ago* from `Auto`, and so does any offset outside ±2.
//!
//! # Direction
//!
//! A negative count is the past and everything else — zero included — is the
//! future, which is what `Intl.RelativeTimeFormat` does: `format(0, 'day')`
//! is *in 0 days*. An `i64` cannot carry a negative zero, so a caller who
//! wants *0 days ago* must ask for it through [`Numeric::Auto`] and the
//! *today* word, or build the phrase themselves.
//!
//! # Plurals
//!
//! Every numeric pattern is chosen by [`hc_i18n::PluralRules`], on the
//! *magnitude* of the count. That is the whole reason this crate depends on
//! `hc-i18n`: Russian needs four forms of "day", Polish needs four with
//! different boundaries, Arabic needs six and Welsh needs six that are not
//! Arabic's.

use core::fmt;

use hc_core::Duration;

use crate::error::HumanizeResult;
use crate::lookup;
use crate::pattern::RelativeStyle;
use crate::render;
use crate::unit::{TimeUnit, UnitAmount};
use crate::unit_choice::{RoundingPolicy, Thresholds, choose};

/// Whether to prefer the numeric pattern or the special word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Numeric {
    /// Always the numeric pattern: *1 day ago*, never *yesterday*.
    #[default]
    Always,
    /// The special word where the language has one for that exact offset,
    /// and the numeric pattern otherwise.
    Auto,
}

/// Formats a signed count of a unit as a phrase.
///
/// The formatter is `Copy` and holds no allocation; build one per locale and
/// keep it.
#[derive(Debug, Clone, Copy)]
pub struct RelativeTimeFormatter {
    locale: hc_i18n::Locale,
    style: RelativeStyle,
    numeric: Numeric,
}

impl RelativeTimeFormatter {
    /// A formatter for a locale, long and numeric.
    #[must_use]
    pub const fn new(locale: hc_i18n::Locale) -> Self {
        Self {
            locale,
            style: RelativeStyle::Long,
            numeric: Numeric::Always,
        }
    }

    /// The same formatter in another style.
    #[must_use]
    pub const fn with_style(mut self, style: RelativeStyle) -> Self {
        self.style = style;
        self
    }

    /// The same formatter with another numeric preference.
    #[must_use]
    pub const fn with_numeric(mut self, numeric: Numeric) -> Self {
        self.numeric = numeric;
        self
    }

    /// The locale.
    #[must_use]
    pub const fn locale(&self) -> &hc_i18n::Locale {
        &self.locale
    }

    /// The style.
    #[must_use]
    pub const fn style(&self) -> RelativeStyle {
        self.style
    }

    /// The numeric preference.
    #[must_use]
    pub const fn numeric(&self) -> Numeric {
        self.numeric
    }

    /// Write a signed count of a unit.
    ///
    /// # Errors
    ///
    /// Returns [`crate::HumanizeError::NoPattern`] if no entry in the locale's
    /// fallback chain states that unit, and [`crate::HumanizeError::WriteFailed`]
    /// if the sink refuses a write.
    pub fn write<W: fmt::Write>(
        &self,
        value: i64,
        unit: TimeUnit,
        out: &mut W,
    ) -> HumanizeResult<()> {
        self.write_amount(UnitAmount::whole(value, unit), out)
    }

    /// Write a [`UnitAmount`], which may carry a half.
    ///
    /// A half never matches a special word — no language has one word for
    /// "an hour and a half ago" — so an amount with a half is always
    /// numeric, whatever [`Numeric`] says.
    ///
    /// # Errors
    ///
    /// As [`Self::write`].
    pub fn write_amount<W: fmt::Write>(
        &self,
        amount: UnitAmount,
        out: &mut W,
    ) -> HumanizeResult<()> {
        if self.numeric == Numeric::Auto
            && !amount.has_half()
            && let Some(word) =
                lookup::special_word(&self.locale, amount.unit(), self.style, amount.count())
        {
            out.write_str(word)?;
            return Ok(());
        }
        let past = amount.is_negative();
        render::write_unit_pattern(
            &self.locale,
            self.style,
            amount,
            move |patterns| {
                if past {
                    &patterns.past
                } else {
                    &patterns.future
                }
            },
            out,
        )
    }

    /// Write an elapsed span, choosing its unit first.
    ///
    /// A negative span is in the past. This is the call a "3 minutes ago"
    /// timestamp wants: give it `now.duration_since(then)` negated, or
    /// `then − now`.
    ///
    /// # Errors
    ///
    /// As [`Self::write`], plus [`crate::HumanizeError::Overflow`] if the span does
    /// not fit the unit it was rounded into.
    pub fn write_span<W: fmt::Write>(
        &self,
        span: Duration,
        thresholds: &Thresholds,
        policy: RoundingPolicy,
        out: &mut W,
    ) -> HumanizeResult<()> {
        self.write_amount(choose(span, thresholds, policy)?, out)
    }

    /// Write an elapsed span with the conversational defaults: the
    /// [`Thresholds::DEFAULT`] table and [`RoundingPolicy::Truncate`].
    ///
    /// Truncation is the default because a humaniser that rounds up says
    /// "2 hours ago" about something that happened 61 minutes ago, and a
    /// reader checks the clock and disagrees.
    ///
    /// # Errors
    ///
    /// As [`Self::write_span`].
    pub fn write_elapsed<W: fmt::Write>(&self, span: Duration, out: &mut W) -> HumanizeResult<()> {
        self.write_span(span, &Thresholds::DEFAULT, RoundingPolicy::Truncate, out)
    }

    /// The phrase for a signed count of a unit.
    ///
    /// # Errors
    ///
    /// As [`Self::write`].
    #[cfg(feature = "alloc")]
    pub fn format(&self, value: i64, unit: TimeUnit) -> HumanizeResult<alloc::string::String> {
        let mut text = alloc::string::String::new();
        self.write(value, unit, &mut text)?;
        Ok(text)
    }

    /// The phrase for a [`UnitAmount`].
    ///
    /// # Errors
    ///
    /// As [`Self::write`].
    #[cfg(feature = "alloc")]
    pub fn format_amount(&self, amount: UnitAmount) -> HumanizeResult<alloc::string::String> {
        let mut text = alloc::string::String::new();
        self.write_amount(amount, &mut text)?;
        Ok(text)
    }

    /// The phrase for an elapsed span, with the conversational defaults.
    ///
    /// # Errors
    ///
    /// As [`Self::write_span`].
    #[cfg(feature = "alloc")]
    pub fn format_elapsed(&self, span: Duration) -> HumanizeResult<alloc::string::String> {
        let mut text = alloc::string::String::new();
        self.write_elapsed(span, &mut text)?;
        Ok(text)
    }
}

/// The plural category a locale would use for a count of a unit.
///
/// Exposed because it is the thing worth testing about a translation: a
/// caller writing their own patterns needs the same answer this crate uses.
#[must_use]
pub fn plural_category(locale: &hc_i18n::Locale, value: i64) -> hc_i18n::PluralCategory {
    render::category_of(locale, UnitAmount::whole(value, TimeUnit::Day))
}

/// Whether a locale has a special word for an offset in a unit.
///
/// This is what [`Numeric::Auto`] asks before it decides.
#[must_use]
pub fn has_special_word(
    locale: &hc_i18n::Locale,
    unit: TimeUnit,
    style: RelativeStyle,
    offset: i64,
) -> bool {
    lookup::special_word(locale, unit, style, offset).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use alloc::vec::Vec;
    use hc_i18n::Locale;

    fn locale(tag: &str) -> Locale {
        tag.parse().expect("well-formed tag")
    }

    fn numeric(tag: &str) -> RelativeTimeFormatter {
        RelativeTimeFormatter::new(locale(tag))
    }

    fn auto(tag: &str) -> RelativeTimeFormatter {
        numeric(tag).with_numeric(Numeric::Auto)
    }

    fn say(formatter: &RelativeTimeFormatter, value: i64, unit: TimeUnit) -> String {
        formatter.format(value, unit).expect("a phrase")
    }

    // --- the plural boundaries that matter -------------------------------

    #[test]
    fn russian_has_four_forms_of_day_and_uses_them_all() {
        // 1 день, 2 дня, 5 дней, 21 день, 22 дня, 25 дней. The switch at 21
        // is what a naive `n == 1` check gets wrong.
        let ru = numeric("ru");
        assert_eq!(say(&ru, -1, TimeUnit::Day), "1 день назад");
        assert_eq!(say(&ru, -2, TimeUnit::Day), "2 дня назад");
        assert_eq!(say(&ru, -5, TimeUnit::Day), "5 дней назад");
        assert_eq!(say(&ru, -21, TimeUnit::Day), "21 день назад");
        assert_eq!(say(&ru, -22, TimeUnit::Day), "22 дня назад");
        assert_eq!(say(&ru, -25, TimeUnit::Day), "25 дней назад");
    }

    #[test]
    fn russian_teens_are_many_even_though_their_last_digit_says_otherwise() {
        let ru = numeric("ru");
        assert_eq!(say(&ru, -11, TimeUnit::Day), "11 дней назад");
        assert_eq!(say(&ru, -12, TimeUnit::Day), "12 дней назад");
        assert_eq!(say(&ru, -14, TimeUnit::Day), "14 дней назад");
        assert_eq!(say(&ru, -111, TimeUnit::Day), "111 дней назад");
    }

    #[test]
    fn russian_years_have_a_suppletive_many_form() {
        let ru = numeric("ru");
        assert_eq!(say(&ru, -1, TimeUnit::Year), "1 год назад");
        assert_eq!(say(&ru, -2, TimeUnit::Year), "2 года назад");
        assert_eq!(say(&ru, -5, TimeUnit::Year), "5 лет назад");
    }

    #[test]
    fn polish_differs_from_russian_exactly_where_cldr_says_it_does() {
        // Both call 22 `few`; only Russian calls 21 `one`, and Polish puts
        // 12 in `many` where its own 2 is `few`.
        let pl = numeric("pl");
        assert_eq!(say(&pl, -1, TimeUnit::Day), "1 dzień temu");
        assert_eq!(say(&pl, -2, TimeUnit::Day), "2 dni temu");
        assert_eq!(say(&pl, -5, TimeUnit::Day), "5 dni temu");
        assert_eq!(say(&pl, -22, TimeUnit::Day), "22 dni temu");
        assert_eq!(say(&pl, -3, TimeUnit::Year), "3 lata temu");
        assert_eq!(say(&pl, -5, TimeUnit::Year), "5 lat temu");
        assert_eq!(say(&pl, -21, TimeUnit::Year), "21 lat temu");
    }

    #[test]
    fn arabic_uses_all_six_categories() {
        // zero, one, two, few (3–10), many (11–99), other (100).
        let ar = numeric("ar");
        assert_eq!(say(&ar, 0, TimeUnit::Day), "خلال ٠ يوم");
        assert_eq!(say(&ar, -1, TimeUnit::Day), "قبل يوم واحد");
        assert_eq!(say(&ar, -2, TimeUnit::Day), "قبل يومين");
        assert_eq!(say(&ar, -3, TimeUnit::Day), "قبل ٣ أيام");
        assert_eq!(say(&ar, -11, TimeUnit::Day), "قبل ١١ يومًا");
        assert_eq!(say(&ar, -100, TimeUnit::Day), "قبل ١٠٠ يوم");
    }

    #[test]
    fn the_arabic_dual_names_its_number_in_the_noun() {
        // يومين and ساعتين carry the "two" themselves, so their patterns
        // have no placeholder at all and no digit is printed.
        let ar = numeric("ar");
        assert!(!say(&ar, -2, TimeUnit::Hour).contains('٢'));
        assert_eq!(say(&ar, -2, TimeUnit::Hour), "قبل ساعتين");
    }

    #[test]
    fn welsh_uses_six_categories_that_are_not_arabics() {
        // Welsh singles out 2, 3 and 6 — chwe blynedd mutates where other
        // numerals do not — and 0 takes its own form.
        let cy = numeric("cy");
        assert_eq!(say(&cy, 0, TimeUnit::Year), "ymhen 0 mlynedd");
        assert_eq!(say(&cy, -1, TimeUnit::Year), "1 flwyddyn yn ôl");
        assert_eq!(say(&cy, -2, TimeUnit::Year), "2 flynedd yn ôl");
        assert_eq!(say(&cy, -3, TimeUnit::Year), "3 blynedd yn ôl");
        assert_eq!(say(&cy, -6, TimeUnit::Year), "6 blynedd yn ôl");
        assert_eq!(say(&cy, -8, TimeUnit::Year), "8 o flynyddoedd yn ôl");
    }

    #[test]
    fn welsh_mutates_the_dual_of_day_and_nothing_else() {
        let cy = numeric("cy");
        assert_eq!(say(&cy, -1, TimeUnit::Day), "1 diwrnod yn ôl");
        assert_eq!(say(&cy, -2, TimeUnit::Day), "2 ddiwrnod yn ôl");
        assert_eq!(say(&cy, -3, TimeUnit::Day), "3 diwrnod yn ôl");
        assert_eq!(say(&cy, -8, TimeUnit::Day), "8 o ddiwrnodau yn ôl");
    }

    #[test]
    fn czech_reaches_one_few_and_other_and_never_many_for_an_integer() {
        // Czech's `many` is its fraction category, so no whole number of
        // years can land in it.
        let cs = numeric("cs");
        assert_eq!(say(&cs, -1, TimeUnit::Year), "před 1 rokem");
        assert_eq!(say(&cs, -3, TimeUnit::Year), "před 3 lety");
        assert_eq!(say(&cs, -5, TimeUnit::Year), "před 5 lety");
    }

    #[test]
    fn a_language_with_no_numeral_agreement_uses_one_form_throughout() {
        let ja = numeric("ja");
        assert_eq!(say(&ja, -1, TimeUnit::Day), "1 日前");
        assert_eq!(say(&ja, -5, TimeUnit::Day), "5 日前");
        let ko = numeric("ko");
        assert_eq!(say(&ko, -1, TimeUnit::Year), "1년 전");
        assert_eq!(say(&ko, -5, TimeUnit::Year), "5년 전");
    }

    // --- numeric versus non-numeric --------------------------------------

    #[test]
    fn numeric_auto_says_yesterday_where_numeric_always_says_one_day_ago() {
        assert_eq!(say(&auto("en"), -1, TimeUnit::Day), "yesterday");
        assert_eq!(say(&numeric("en"), -1, TimeUnit::Day), "1 day ago");
        assert_eq!(say(&auto("ru"), -1, TimeUnit::Day), "вчера");
        assert_eq!(say(&numeric("ru"), -1, TimeUnit::Day), "1 день назад");
    }

    #[test]
    fn auto_reaches_the_offsets_a_language_has_words_for_and_no_further() {
        let en = auto("en");
        assert_eq!(say(&en, -2, TimeUnit::Day), "the day before yesterday");
        assert_eq!(say(&en, 2, TimeUnit::Day), "the day after tomorrow");
        assert_eq!(say(&en, -3, TimeUnit::Day), "3 days ago");
        assert_eq!(say(&en, 3, TimeUnit::Day), "in 3 days");
    }

    #[test]
    fn auto_covers_every_unit_that_has_a_word_for_the_offset() {
        let en = auto("en");
        assert_eq!(say(&en, 0, TimeUnit::Second), "now");
        assert_eq!(say(&en, -1, TimeUnit::Week), "last week");
        assert_eq!(say(&en, 0, TimeUnit::Month), "this month");
        assert_eq!(say(&en, 1, TimeUnit::Year), "next year");
        assert_eq!(say(&en, -1, TimeUnit::Quarter), "last quarter");
    }

    #[test]
    fn a_unit_with_no_word_for_an_offset_stays_numeric_under_auto() {
        // No language has a word for "one second ago".
        let en = auto("en");
        assert_eq!(say(&en, -1, TimeUnit::Second), "1 second ago");
        assert_eq!(say(&en, 1, TimeUnit::Second), "in 1 second");
    }

    #[test]
    fn hindi_uses_the_same_word_for_yesterday_and_tomorrow() {
        // कल is both, and परसों is both the day before and the day after;
        // Hindi takes the direction from the verb. Auto reports what the
        // language actually says rather than inventing a distinction.
        let hi = auto("hi");
        assert_eq!(say(&hi, -1, TimeUnit::Day), say(&hi, 1, TimeUnit::Day));
        assert_eq!(say(&hi, -2, TimeUnit::Day), say(&hi, 2, TimeUnit::Day));
        assert_eq!(say(&hi, -1, TimeUnit::Day), "कल");
    }

    #[test]
    fn zero_is_the_future_because_an_integer_has_no_negative_zero() {
        assert_eq!(say(&numeric("en"), 0, TimeUnit::Day), "in 0 days");
        assert_eq!(say(&auto("en"), 0, TimeUnit::Day), "today");
    }

    #[test]
    fn a_half_is_always_numeric_whatever_the_preference_says() {
        let half = UnitAmount::half_past(-1, TimeUnit::Day);
        let phrase = auto("en").format_amount(half).expect("a phrase");
        assert_eq!(phrase, "1.5 days ago");
        assert_ne!(phrase, "yesterday");
    }

    // --- styles -----------------------------------------------------------

    #[test]
    fn the_three_english_styles_are_three_different_phrases() {
        let long = numeric("en");
        let short = long.with_style(RelativeStyle::Short);
        let narrow = long.with_style(RelativeStyle::Narrow);
        assert_eq!(say(&long, -3, TimeUnit::Month), "3 months ago");
        assert_eq!(say(&short, -3, TimeUnit::Month), "3 mo. ago");
        assert_eq!(say(&narrow, -3, TimeUnit::Month), "3mo ago");
        assert_eq!(say(&narrow, -3, TimeUnit::Day), "3d ago");
        assert_eq!(say(&narrow, 3, TimeUnit::Hour), "in 3h");
    }

    #[test]
    fn german_abbreviates_the_month_and_not_the_day() {
        let short = numeric("de").with_style(RelativeStyle::Short);
        let narrow = numeric("de").with_style(RelativeStyle::Narrow);
        assert_eq!(say(&short, -3, TimeUnit::Month), "vor 3 Mon.");
        // German states no narrow forms, so narrow finds short.
        assert_eq!(say(&narrow, -3, TimeUnit::Month), "vor 3 Mon.");
        // and German short states no day forms, so short finds long.
        assert_eq!(say(&short, -3, TimeUnit::Day), "vor 3 Tagen");
        assert_eq!(say(&narrow, -3, TimeUnit::Day), "vor 3 Tagen");
    }

    #[test]
    fn a_locale_that_abbreviates_nothing_still_answers_every_style() {
        for style in RelativeStyle::ALL {
            let formatter = numeric("it").with_style(style);
            assert_eq!(say(&formatter, -3, TimeUnit::Day), "3 giorni fa");
        }
    }

    #[test]
    fn every_shipped_locale_can_phrase_every_unit_in_both_directions() {
        for data in crate::data::LOCALES {
            for style in RelativeStyle::ALL {
                let formatter = numeric(data.tag).with_style(style);
                for unit in TimeUnit::ALL {
                    for value in [-5i64, -1, 0, 1, 5, 11, 22, 101] {
                        let phrase = formatter
                            .format(value, unit)
                            .unwrap_or_else(|_| panic!("{} {style:?} {unit} {value}", data.tag));
                        assert!(
                            !phrase.is_empty(),
                            "{} {style:?} {unit} {value} is empty",
                            data.tag
                        );
                        assert!(
                            !phrase.contains("{0}"),
                            "{} {style:?} {unit} {value} left a placeholder",
                            data.tag
                        );
                    }
                }
            }
        }
    }

    // --- numbers ----------------------------------------------------------

    #[test]
    fn the_digits_come_from_the_locales_numbering_system() {
        // Arabic defaults to arab digits, so "13" is ١٣ and not 13.
        let phrase = say(&numeric("ar"), -13, TimeUnit::Hour);
        assert!(phrase.contains("١٣"), "{phrase}");
        assert!(!phrase.contains("13"));
        // And an explicit -u-nu- overrides it.
        let latin =
            RelativeTimeFormatter::new(locale("ar").with_numbering_system("latn").expect("valid"));
        assert!(say(&latin, -13, TimeUnit::Hour).contains("13"));
    }

    #[test]
    fn the_sign_is_carried_by_the_words_and_never_printed_twice() {
        for data in crate::data::LOCALES {
            let phrase = say(&numeric(data.tag), -3, TimeUnit::Day);
            assert!(!phrase.contains("-3"), "{} printed a sign", data.tag);
        }
    }

    // --- spans ------------------------------------------------------------

    #[test]
    fn an_elapsed_span_chooses_its_own_unit() {
        let en = numeric("en");
        let cases: Vec<(i128, &str)> = alloc::vec![
            (-30, "30 seconds ago"),
            (-50, "1 minute ago"),
            (-3 * 3_600, "3 hours ago"),
            (-2 * 86_400, "2 days ago"),
            (-400 * 86_400, "1 year ago"),
            (3_600, "in 1 hour"),
        ];
        for (seconds, expected) in cases {
            let mut text = String::new();
            en.write_elapsed(Duration::from_secs(seconds), &mut text)
                .expect("a phrase");
            assert_eq!(text, expected, "for {seconds} s");
        }
    }

    #[test]
    fn the_caller_chooses_the_table_and_the_policy() {
        let en = numeric("en");
        let span = Duration::from_secs(-36 * 3_600);
        let mut floored = String::new();
        en.write_span(
            span,
            &Thresholds::EXACT,
            RoundingPolicy::Floor,
            &mut floored,
        )
        .expect("a phrase");
        let mut truncated = String::new();
        en.write_span(
            span,
            &Thresholds::EXACT,
            RoundingPolicy::Truncate,
            &mut truncated,
        )
        .expect("a phrase");
        assert_eq!(floored, "2 days ago");
        assert_eq!(truncated, "1 day ago");
    }

    // --- the fallback floor ----------------------------------------------

    #[test]
    fn an_unknown_locale_gets_the_language_free_root_and_not_english() {
        let unknown = numeric("xx");
        assert_eq!(say(&unknown, -3, TimeUnit::Day), "-3 d");
        assert_eq!(say(&unknown, 3, TimeUnit::Day), "+3 d");
        // and the root has no special words, so Auto changes nothing.
        assert_eq!(
            say(&auto("xx"), -1, TimeUnit::Day),
            say(&numeric("xx"), -1, TimeUnit::Day)
        );
    }

    #[test]
    fn a_region_answers_in_its_languages_words() {
        assert_eq!(say(&numeric("en-GB"), -3, TimeUnit::Day), "3 days ago");
        assert_eq!(say(&numeric("pt-BR"), -3, TimeUnit::Day), "há 3 dias");
        assert_eq!(say(&numeric("zh-Hans"), -3, TimeUnit::Day), "3天前");
        assert_eq!(say(&numeric("zh-Hant"), -3, TimeUnit::Week), "3週前");
    }

    // --- accessors and helpers -------------------------------------------

    #[test]
    fn the_formatter_reports_what_it_was_built_with() {
        let formatter = numeric("fr")
            .with_style(RelativeStyle::Short)
            .with_numeric(Numeric::Auto);
        assert_eq!(formatter.locale().language(), "fr");
        assert_eq!(formatter.style(), RelativeStyle::Short);
        assert_eq!(formatter.numeric(), Numeric::Auto);
    }

    #[test]
    fn the_special_word_query_agrees_with_what_auto_does() {
        for offset in -3..=3 {
            let has = has_special_word(&locale("en"), TimeUnit::Day, RelativeStyle::Long, offset);
            let phrase = say(&auto("en"), offset, TimeUnit::Day);
            assert_eq!(has, !phrase.contains(char::is_numeric), "offset {offset}");
        }
    }

    #[test]
    fn the_plural_category_helper_reports_what_the_rules_say() {
        use hc_i18n::PluralCategory;
        assert_eq!(plural_category(&locale("ru"), 21), PluralCategory::One);
        assert_eq!(plural_category(&locale("ru"), 22), PluralCategory::Few);
        assert_eq!(plural_category(&locale("ru"), 25), PluralCategory::Many);
        assert_eq!(plural_category(&locale("ja"), 25), PluralCategory::Other);
    }
}
