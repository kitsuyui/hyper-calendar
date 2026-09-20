//! Humanising a span on its own: *2 hours 30 minutes*, *1 day, 3 hours*,
//! *3 weeks*, *2h30m*.
//!
//! # Not relative to anything
//!
//! [`crate::relative`] answers "when"; this module answers "how long". The
//! two need different vocabulary — *3 days* is not *3 days ago* with a word
//! removed, as German's dative shows — and different structure, because a
//! duration is a *list* of components and a relative time is one.
//!
//! # Which units
//!
//! The default component set is [`TimeUnit::CLOCK`]: days, hours, minutes,
//! seconds. Weeks, months and quarters are available but not default,
//! because a bare span has no calendar to anchor them to: "1 month" out of
//! context is 2 629 746 seconds, which is not any actual month. A caller who
//! knows that is fine — a project plan, a subscription length — passes them
//! in with [`DurationFormatter::with_units`].
//!
//! # Truncation, not rounding
//!
//! Limiting the component count drops the remainder rather than rounding it
//! into the last component kept. Two hours, thirty minutes and fifty seconds
//! with two components is *2 hours 30 minutes*, not *2 hours 31 minutes*: a
//! duration that is displayed while it runs must never appear to go
//! backwards, and rounding the tail makes it do exactly that as the seconds
//! tick past the halfway mark.

use core::fmt;

use hc_core::Duration;

use crate::error::{HumanizeError, HumanizeResult};
use crate::lookup;
use crate::pattern::{ListForms, RelativeStyle};
use crate::render;
use crate::unit::{TimeUnit, UnitAmount};

/// The largest number of components a decomposition can have, which is the
/// number of units there are.
pub const MAX_COMPONENTS: usize = 8;

/// How a duration is phrased.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DurationStyle {
    /// *2 hours 30 minutes*, joined with the locale's conjunction list.
    #[default]
    Long,
    /// The abbreviated unit phrases, joined with commas.
    Short,
    /// The narrow unit phrases, joined with spaces.
    Narrow,
    /// *2h30m*: the bare suffixes, with nothing between them.
    ///
    /// Not every locale has a compact convention; those fall back to the
    /// root's Latin suffixes, which is wrong inside right-to-left text. Use
    /// [`DurationStyle::Narrow`] there.
    Compact,
}

impl DurationStyle {
    /// The pattern style this phrasing draws its unit phrases from.
    ///
    /// The compact form draws on no pattern style at all — it uses the bare
    /// suffixes — but it still needs a width for the fallback chain, and
    /// narrow is the nearest.
    #[must_use]
    pub const fn pattern_style(self) -> RelativeStyle {
        match self {
            Self::Long => RelativeStyle::Long,
            Self::Short => RelativeStyle::Short,
            Self::Narrow | Self::Compact => RelativeStyle::Narrow,
        }
    }
}

/// A span broken into whole units.
#[derive(Debug, Clone, Copy)]
pub struct Components {
    entries: [UnitAmount; MAX_COMPONENTS],
    len: usize,
    negative: bool,
}

impl Components {
    /// The components, largest unit first.
    #[must_use]
    pub fn as_slice(&self) -> &[UnitAmount] {
        &self.entries[..self.len]
    }

    /// How many components there are. Never zero: a span shorter than the
    /// smallest unit still produces one zero component, because *0 seconds*
    /// is an answer and an empty string is not.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Always false; present because a length without it reads oddly.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Whether the original span was negative.
    ///
    /// The components themselves carry no sign: a duration is a length, and
    /// a caller who needs the direction asks here.
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.negative
    }
}

/// Break a span into whole units, largest first.
///
/// `units` must be ordered longest-first; a unit whose count is zero is
/// skipped entirely, so 25 hours over the clock units is *1 day 1 hour*, not
/// *1 day 1 hour 0 minutes 0 seconds*. At most `max_components` are kept and
/// the rest of the span is dropped.
///
/// # Errors
///
/// Returns [`HumanizeError::NoComponents`] if `units` is empty or
/// `max_components` is zero, since there is then nothing that could be
/// written.
pub fn decompose(
    span: Duration,
    units: &[TimeUnit],
    max_components: usize,
) -> HumanizeResult<Components> {
    if units.is_empty() || max_components == 0 {
        return Err(HumanizeError::NoComponents);
    }
    let negative = span.is_negative();
    // Whole seconds only: the sub-second part of a Duration is attoseconds,
    // and no locale in CLDR has a relative-time field below the second.
    let mut remaining = span.whole_seconds().unsigned_abs();
    let mut entries = [UnitAmount::whole(0, TimeUnit::Second); MAX_COMPONENTS];
    let mut len = 0usize;
    for unit in units {
        if len == max_components.min(MAX_COMPONENTS) {
            break;
        }
        let size = unit.mean_seconds().unsigned_abs() as u128;
        let count = remaining / size;
        if count == 0 {
            continue;
        }
        let count = i64::try_from(count).map_err(|_| HumanizeError::Overflow)?;
        entries[len] = UnitAmount::whole(count, *unit);
        len += 1;
        remaining -= (count as u128) * size;
    }
    if len == 0 {
        let smallest = units[units.len() - 1];
        entries[0] = UnitAmount::whole(0, smallest);
        len = 1;
    }
    Ok(Components {
        entries,
        len,
        negative,
    })
}

/// Formats a span as a length.
#[derive(Debug, Clone, Copy)]
pub struct DurationFormatter {
    locale: hc_i18n::Locale,
    style: DurationStyle,
    units: &'static [TimeUnit],
    max_components: usize,
}

impl DurationFormatter {
    /// A formatter for a locale: long style, clock units, every component.
    #[must_use]
    pub const fn new(locale: hc_i18n::Locale) -> Self {
        Self {
            locale,
            style: DurationStyle::Long,
            units: &TimeUnit::CLOCK,
            max_components: MAX_COMPONENTS,
        }
    }

    /// The same formatter in another style.
    #[must_use]
    pub const fn with_style(mut self, style: DurationStyle) -> Self {
        self.style = style;
        self
    }

    /// The same formatter over another set of units, longest first.
    #[must_use]
    pub const fn with_units(mut self, units: &'static [TimeUnit]) -> Self {
        self.units = units;
        self
    }

    /// The same formatter limited to `max_components` components.
    #[must_use]
    pub const fn with_max_components(mut self, max_components: usize) -> Self {
        self.max_components = max_components;
        self
    }

    /// The locale.
    #[must_use]
    pub const fn locale(&self) -> &hc_i18n::Locale {
        &self.locale
    }

    /// Write a span.
    ///
    /// # Errors
    ///
    /// Returns [`HumanizeError::NoComponents`] if the unit set is empty,
    /// [`HumanizeError::NoPattern`] if the locale states no phrase for a
    /// unit in the set, and [`HumanizeError::WriteFailed`] if the sink
    /// refuses a write.
    pub fn write<W: fmt::Write>(&self, span: Duration, out: &mut W) -> HumanizeResult<()> {
        let components = decompose(span, self.units, self.max_components)?;
        self.write_components(&components, out)
    }

    /// Write an already-decomposed span.
    ///
    /// # Errors
    ///
    /// As [`Self::write`].
    pub fn write_components<W: fmt::Write>(
        &self,
        components: &Components,
        out: &mut W,
    ) -> HumanizeResult<()> {
        let items = components.as_slice();
        if self.style == DurationStyle::Compact {
            for amount in items {
                render::write_compact(&self.locale, *amount, out)?;
            }
            return Ok(());
        }
        let forms = lookup::list_forms(&self.locale, self.style.pattern_style());
        let style = self.style.pattern_style();
        for (index, amount) in items.iter().enumerate() {
            if index > 0 {
                out.write_str(glue(pattern_for(&forms, index, items.len()))?)?;
            }
            render::write_count(&self.locale, style, *amount, out)?;
        }
        Ok(())
    }

    /// Write one amount on its own: *3 weeks*, *an hour and a half*.
    ///
    /// This is the call that pairs with
    /// [`RoundingPolicy::NearestHalf`](crate::RoundingPolicy::NearestHalf):
    /// round ninety minutes into an hour and a half, then say it. A half is
    /// written through the locale's idiom where the data states one and as
    /// the decimal `1.5` otherwise, which is why the plural category comes
    /// out of `v = 1` operands rather than out of an integer.
    ///
    /// # Errors
    ///
    /// As [`Self::write`].
    pub fn write_amount<W: fmt::Write>(
        &self,
        amount: UnitAmount,
        out: &mut W,
    ) -> HumanizeResult<()> {
        if self.style == DurationStyle::Compact {
            return render::write_compact(&self.locale, amount, out);
        }
        render::write_count(&self.locale, self.style.pattern_style(), amount, out)
    }

    /// The phrase for a span.
    ///
    /// # Errors
    ///
    /// As [`Self::write`].
    #[cfg(feature = "alloc")]
    pub fn format(&self, span: Duration) -> HumanizeResult<alloc::string::String> {
        let mut text = alloc::string::String::new();
        self.write(span, &mut text)?;
        Ok(text)
    }

    /// The phrase for one amount.
    ///
    /// # Errors
    ///
    /// As [`Self::write_amount`].
    #[cfg(feature = "alloc")]
    pub fn format_amount(&self, amount: UnitAmount) -> HumanizeResult<alloc::string::String> {
        let mut text = alloc::string::String::new();
        self.write_amount(amount, &mut text)?;
        Ok(text)
    }
}

/// Which list pattern joins item `index` to the ones before it.
const fn pattern_for(forms: &ListForms, index: usize, total: usize) -> &'static str {
    if total == 2 {
        forms.two
    } else if index == 1 {
        forms.start
    } else if index + 1 == total {
        forms.end
    } else {
        forms.middle
    }
}

/// The text between `{0}` and `{1}` of a two-place list pattern.
///
/// CLDR list patterns are always `{0}<glue>{1}` with nothing outside the
/// placeholders, which is what lets a list be written left to right into a
/// sink that cannot be read back. A pattern that breaks that shape is a data
/// error, not a case to handle.
fn glue(pattern: &str) -> HumanizeResult<&str> {
    let after_first = pattern
        .strip_prefix("{0}")
        .ok_or(HumanizeError::NoPattern)?;
    after_first
        .strip_suffix("{1}")
        .ok_or(HumanizeError::NoPattern)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_choice::{RoundingPolicy, Thresholds, choose};
    use hc_i18n::Locale;

    fn locale(tag: &str) -> Locale {
        tag.parse().expect("well-formed tag")
    }

    fn formatter(tag: &str) -> DurationFormatter {
        DurationFormatter::new(locale(tag))
    }

    fn say(formatter: &DurationFormatter, seconds: i128) -> alloc::string::String {
        formatter
            .format(Duration::from_secs(seconds))
            .expect("a phrase")
    }

    #[test]
    fn two_and_a_half_hours_is_two_hours_and_thirty_minutes() {
        assert_eq!(say(&formatter("en"), 9_000), "2 hours and 30 minutes");
    }

    #[test]
    fn the_conjunction_closes_a_longer_list_and_commas_carry_the_rest() {
        assert_eq!(
            say(&formatter("en"), 3_600 + 2 * 60 + 3),
            "1 hour, 2 minutes and 3 seconds"
        );
    }

    #[test]
    fn the_conjunction_is_per_locale() {
        let span = 3_600 + 2 * 60 + 3;
        assert_eq!(
            say(&formatter("de"), span),
            "1 Stunde, 2 Minuten und 3 Sekunden"
        );
        assert_eq!(
            say(&formatter("fr"), span),
            "1 heure, 2 minutes et 3 secondes"
        );
        assert_eq!(
            say(&formatter("es"), span),
            "1 hora, 2 minutos y 3 segundos"
        );
        assert_eq!(say(&formatter("ru"), span), "1 час, 2 минуты и 3 секунды");
        // Japanese has no list conjunction at all: it uses 、throughout.
        assert_eq!(say(&formatter("ja"), span), "1 時間、2 分、3 秒");
    }

    #[test]
    fn the_component_limit_truncates_rather_than_rounding_into_the_tail() {
        // 2:30:50 with two components must not become "2 hours 31 minutes":
        // a running timer that rounds its tail appears to go backwards.
        let span = 2 * 3_600 + 30 * 60 + 50;
        assert_eq!(
            say(&formatter("en").with_max_components(2), span),
            "2 hours and 30 minutes"
        );
        assert_eq!(
            say(&formatter("en").with_max_components(1), span),
            "2 hours"
        );
        assert_eq!(
            say(&formatter("en").with_max_components(3), span),
            "2 hours, 30 minutes and 50 seconds"
        );
    }

    #[test]
    fn a_zero_component_is_skipped_and_not_printed() {
        // 25 hours is one day and one hour, with nothing in between.
        assert_eq!(say(&formatter("en"), 25 * 3_600), "1 day and 1 hour");
        assert_eq!(say(&formatter("en"), 86_400 + 5), "1 day and 5 seconds");
    }

    #[test]
    fn a_span_shorter_than_every_unit_still_says_something() {
        assert_eq!(say(&formatter("en"), 0), "0 seconds");
        assert_eq!(
            formatter("en")
                .format(Duration::from_millis(400))
                .expect("a phrase"),
            "0 seconds"
        );
    }

    #[test]
    fn the_short_and_narrow_styles_change_both_the_units_and_the_joins() {
        let span = 86_400 + 2 * 3_600 + 3 * 60 + 4;
        assert_eq!(
            say(&formatter("en").with_style(DurationStyle::Short), span),
            "1 day, 2 hr., 3 min., 4 sec."
        );
        assert_eq!(
            say(&formatter("en").with_style(DurationStyle::Narrow), span),
            "1d 2h 3m 4s"
        );
    }

    #[test]
    fn the_compact_form_has_no_separators_at_all() {
        let compact = formatter("en").with_style(DurationStyle::Compact);
        assert_eq!(say(&compact, 9_000), "2h30m");
        assert_eq!(say(&compact, 86_400 + 2 * 3_600 + 3 * 60 + 4), "1d2h3m4s");
        // Japanese has its own counters, so the compact form stays Japanese.
        let ja = formatter("ja").with_style(DurationStyle::Compact);
        assert_eq!(say(&ja, 9_000), "2時間30分");
    }

    #[test]
    fn weeks_are_available_but_never_chosen_by_default() {
        let three_weeks = Duration::from_days(21);
        assert_eq!(
            formatter("en").format(three_weeks).expect("a phrase"),
            "21 days"
        );
        static WEEKS: &[TimeUnit] = &[TimeUnit::Week, TimeUnit::Day, TimeUnit::Hour];
        assert_eq!(
            formatter("en")
                .with_units(WEEKS)
                .format(three_weeks)
                .expect("a phrase"),
            "3 weeks"
        );
        assert_eq!(
            formatter("en")
                .with_units(WEEKS)
                .format(Duration::from_days(23))
                .expect("a phrase"),
            "3 weeks and 2 days"
        );
    }

    #[test]
    fn an_empty_component_request_is_an_error_and_not_an_empty_string() {
        assert_eq!(
            decompose(Duration::from_secs(60), &[], 4).unwrap_err(),
            HumanizeError::NoComponents
        );
        assert_eq!(
            decompose(Duration::from_secs(60), &TimeUnit::CLOCK, 0).unwrap_err(),
            HumanizeError::NoComponents
        );
    }

    #[test]
    fn a_decomposition_reports_the_direction_without_signing_its_parts() {
        let components =
            decompose(Duration::from_secs(-9_000), &TimeUnit::CLOCK, 8).expect("decomposes");
        assert!(components.is_negative());
        assert_eq!(components.len(), 2);
        assert!(!components.is_empty());
        for amount in components.as_slice() {
            assert!(!amount.is_negative());
        }
        assert_eq!(
            components.as_slice()[0],
            UnitAmount::whole(2, TimeUnit::Hour)
        );
        assert_eq!(
            components.as_slice()[1],
            UnitAmount::whole(30, TimeUnit::Minute)
        );
    }

    #[test]
    fn an_hour_and_a_half_uses_the_idiom_where_the_locale_has_one() {
        let ninety = Duration::from_secs(5_400);
        let amount =
            choose(ninety, &Thresholds::DEFAULT, RoundingPolicy::NearestHalf).expect("fits");
        assert_eq!(
            formatter("en").format_amount(amount).expect("a phrase"),
            "an hour and a half"
        );
        assert_eq!(
            formatter("de").format_amount(amount).expect("a phrase"),
            "anderthalb Stunden"
        );
        assert_eq!(
            formatter("ru").format_amount(amount).expect("a phrase"),
            "полтора часа"
        );
        assert_eq!(
            formatter("fr").format_amount(amount).expect("a phrase"),
            "une heure et demie"
        );
    }

    #[test]
    fn half_a_unit_has_its_own_idiom() {
        let half = UnitAmount::half_past(0, TimeUnit::Hour);
        assert_eq!(
            formatter("en").format_amount(half).expect("a phrase"),
            "half an hour"
        );
        assert_eq!(
            formatter("de").format_amount(half).expect("a phrase"),
            "eine halbe Stunde"
        );
        assert_eq!(
            formatter("ru").format_amount(half).expect("a phrase"),
            "полчаса"
        );
    }

    #[test]
    fn a_half_with_no_idiom_becomes_a_decimal_and_picks_its_plural_from_it() {
        // No language has a word for "two and a half hours", so the decimal
        // is written and the plural rules see `v = 1`.
        let amount = UnitAmount::half_past(2, TimeUnit::Hour);
        assert_eq!(
            formatter("en").format_amount(amount).expect("a phrase"),
            "2.5 hours"
        );
        assert_eq!(
            formatter("de").format_amount(amount).expect("a phrase"),
            "2,5 Stunden"
        );
        // Czech's `many` is the fraction category: 2,5 hodiny, not hodiny.
        assert_eq!(
            formatter("cs").format_amount(amount).expect("a phrase"),
            "2,5 hodiny"
        );
    }

    #[test]
    fn the_decimal_separator_is_the_locales_own() {
        let amount = UnitAmount::half_past(2, TimeUnit::Hour);
        assert!(
            formatter("en")
                .format_amount(amount)
                .expect("a phrase")
                .contains('.')
        );
        for tag in ["de", "fr", "ru", "pl", "cs", "it", "es", "pt", "nl", "tr"] {
            assert!(
                formatter(tag)
                    .format_amount(amount)
                    .expect("a phrase")
                    .contains(','),
                "{tag} should use a comma"
            );
        }
        // Arabic writes its own decimal separator, U+066B.
        assert!(
            formatter("ar")
                .format_amount(amount)
                .expect("a phrase")
                .contains('\u{66b}')
        );
    }

    #[test]
    fn the_undirected_phrase_is_not_the_relative_one_with_a_word_removed() {
        // German's "vor" takes the dative, so its relative form has an
        // ending its plain form does not. This is why `count` is stored.
        assert_eq!(say(&formatter("de"), 3 * 86_400), "3 Tage");
        let relative = crate::relative::RelativeTimeFormatter::new(locale("de"))
            .format(-3, TimeUnit::Day)
            .expect("a phrase");
        assert_eq!(relative, "vor 3 Tagen");
        assert_eq!(
            formatter("de")
                .format_amount(UnitAmount::whole(3, TimeUnit::Year))
                .expect("a phrase"),
            "3 Jahre"
        );
    }

    #[test]
    fn every_shipped_locale_can_phrase_a_multi_component_span() {
        let span = Duration::from_secs(86_400 + 2 * 3_600 + 3 * 60 + 4);
        for data in crate::data::LOCALES {
            for style in [
                DurationStyle::Long,
                DurationStyle::Short,
                DurationStyle::Narrow,
                DurationStyle::Compact,
            ] {
                let phrase = formatter(data.tag)
                    .with_style(style)
                    .format(span)
                    .unwrap_or_else(|_| panic!("{} {style:?}", data.tag));
                assert!(!phrase.is_empty());
                assert!(!phrase.contains("{0}"), "{} {style:?}", data.tag);
                assert!(!phrase.contains("{1}"), "{} {style:?}", data.tag);
            }
        }
    }

    #[test]
    fn a_list_pattern_that_cannot_be_written_left_to_right_is_rejected() {
        assert_eq!(glue("{0}, {1}"), Ok(", "));
        assert_eq!(glue("{0} and {1}"), Ok(" and "));
        assert_eq!(glue("{1} and {0}"), Err(HumanizeError::NoPattern));
        assert_eq!(glue("{0} and {1}!"), Err(HumanizeError::NoPattern));
    }
}
