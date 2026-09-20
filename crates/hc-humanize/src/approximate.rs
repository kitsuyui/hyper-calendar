//! Hedged phrasing: *about 3 hours*, *just over a week*, *nearly a year*.
//!
//! # The round-number preference
//!
//! People do not say "1.0952 years". They pick a round number they can hold
//! in their head and put a word in front of it that says which way the truth
//! lies. Four hundred days is *just over a year*; three hundred and fifty is
//! *nearly a year*; three hours and three minutes is *about 3 hours*. The
//! number gets simpler and the hedge carries what was lost.
//!
//! This module does exactly that and nothing more clever. It chooses a unit
//! with [`crate::unit_choice`], takes the whole part, and reads the leftover
//! fraction off a table of boundaries:
//!
//! | fraction of a unit | hedge | count |
//! |---|---|---|
//! | < 0.02 | none | whole |
//! | < 0.08 | *about* | whole |
//! | < 0.35 | *just over* | whole |
//! | < 0.70 | *over* | whole |
//! | otherwise | *nearly* | whole + 1 |
//!
//! The boundaries are [`ApproximatePolicy`], not constants baked into the
//! code, because where *just over* stops being honest is a judgement about
//! the reader and not about arithmetic.
//!
//! A whole part of zero is the interesting case: 350 days is 0.958 years, so
//! the whole part is zero and the fraction is 0.958, and the table says
//! *nearly 1 year*. That is how a span shorter than its own unit still gets
//! a sensible phrase instead of *0 years*.
//!
//! # The indefinite article
//!
//! Where the locale data states one, a count of exactly one is written as
//! the indefinite singular — *nearly a year*, not *nearly 1 year*. Languages
//! with no indefinite article, and the ones where the hedge would have to
//! elide in front of it, state nothing and get the numeral; see
//! [`crate::data`].

use core::fmt;

use hc_core::Duration;
use hc_core::math::floor;

use crate::error::HumanizeResult;
use crate::lookup;
use crate::pattern::RelativeStyle;
use crate::render;
use crate::unit::UnitAmount;
use crate::unit_choice::{Thresholds, count_of, fraction_of};

/// Which way the truth lies from the round number that is about to be said.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Approximation {
    /// The count is right: no hedge.
    Exactly,
    /// *about {0}*: a little either way.
    About,
    /// *just over {0}*.
    JustOver,
    /// *over {0}*: well past, but not most of the way to the next.
    Over,
    /// *nearly {0}*: almost there, where `{0}` is the *next* count up.
    Nearly,
    /// *less than {0}*. Never produced by [`approximate`]; a caller states
    /// it when they have a bound rather than a measurement.
    LessThan,
    /// *more than {0}*, likewise.
    MoreThan,
}

impl Approximation {
    /// The locale's pattern for this hedge.
    #[must_use]
    pub fn pattern(self, locale: &hc_i18n::Locale) -> &'static str {
        let patterns = lookup::approximate_patterns(locale);
        match self {
            Self::Exactly => patterns.exactly,
            Self::About => patterns.about,
            Self::JustOver => patterns.just_over,
            Self::Over => patterns.over,
            Self::Nearly => patterns.nearly,
            Self::LessThan => patterns.less_than,
            Self::MoreThan => patterns.more_than,
        }
    }
}

/// Where each hedge takes over, as a fraction of one unit.
///
/// The fields must increase; a table that does not is not rejected, it
/// simply makes the later hedges unreachable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ApproximatePolicy {
    /// Below this, the count is said without a hedge.
    pub exact_within: f64,
    /// Below this, *about*.
    pub about_within: f64,
    /// Below this, *just over*.
    pub just_over_within: f64,
    /// Below this, *over*; at or above it, *nearly* the next count up.
    pub over_within: f64,
}

impl ApproximatePolicy {
    /// The boundaries documented at the top of this module.
    pub const DEFAULT: Self = Self {
        exact_within: 0.02,
        about_within: 0.08,
        just_over_within: 0.35,
        over_within: 0.70,
    };

    /// A policy that never says *about*: everything is *over* or *nearly*.
    ///
    /// For a caller who needs the phrase to be a true bound — a deadline, a
    /// quota — rather than a fair description.
    pub const BOUNDED: Self = Self {
        exact_within: 0.0,
        about_within: 0.0,
        just_over_within: 0.35,
        over_within: 0.70,
    };
}

impl Default for ApproximatePolicy {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// A hedge and the count it hedges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApproximateSpan {
    /// The hedge.
    pub qualifier: Approximation,
    /// The count, already adjusted: *nearly* carries the next count up.
    pub amount: UnitAmount,
}

/// Reduce a span to a round count and a hedge.
///
/// The sign is dropped: an approximation describes a length, and
/// [`crate::relative`] is where direction lives.
///
/// # Errors
///
/// Returns [`crate::HumanizeError::Overflow`] if the count does not fit an
/// `i64`.
pub fn approximate(
    span: Duration,
    thresholds: &Thresholds,
    policy: ApproximatePolicy,
) -> HumanizeResult<ApproximateSpan> {
    let unit = thresholds.unit_for(span);
    let magnitude = count_of(span, unit);
    let magnitude = if magnitude < 0.0 {
        -magnitude
    } else {
        magnitude
    };
    let whole = floor(magnitude);
    let fraction = fraction_of(span, unit);
    let (qualifier, count) = if fraction < policy.exact_within {
        (Approximation::Exactly, whole)
    } else if fraction < policy.about_within {
        (Approximation::About, whole)
    } else if fraction < policy.just_over_within {
        (Approximation::JustOver, whole)
    } else if fraction < policy.over_within {
        (Approximation::Over, whole)
    } else {
        (Approximation::Nearly, whole + 1.0)
    };
    // `count` is already whole, so the policy here only converts it to an
    // integer; truncation is named rather than rounding so that a future
    // fractional `count` could not silently change the hedge it was paired
    // with.
    let amount = crate::unit_choice::RoundingPolicy::Truncate.apply(count, unit)?;
    Ok(ApproximateSpan { qualifier, amount })
}

/// Formats an approximation.
#[derive(Debug, Clone, Copy)]
pub struct ApproximateFormatter {
    locale: hc_i18n::Locale,
    style: RelativeStyle,
    thresholds: Thresholds,
    policy: ApproximatePolicy,
}

impl ApproximateFormatter {
    /// A formatter for a locale, with the default table and policy.
    #[must_use]
    pub const fn new(locale: hc_i18n::Locale) -> Self {
        Self {
            locale,
            style: RelativeStyle::Long,
            thresholds: Thresholds::DEFAULT,
            policy: ApproximatePolicy::DEFAULT,
        }
    }

    /// The same formatter in another style.
    #[must_use]
    pub const fn with_style(mut self, style: RelativeStyle) -> Self {
        self.style = style;
        self
    }

    /// The same formatter with another threshold table.
    #[must_use]
    pub const fn with_thresholds(mut self, thresholds: Thresholds) -> Self {
        self.thresholds = thresholds;
        self
    }

    /// The same formatter with another hedge policy.
    #[must_use]
    pub const fn with_policy(mut self, policy: ApproximatePolicy) -> Self {
        self.policy = policy;
        self
    }

    /// The locale.
    #[must_use]
    pub const fn locale(&self) -> &hc_i18n::Locale {
        &self.locale
    }

    /// Write the hedged phrase for a span.
    ///
    /// # Errors
    ///
    /// As [`approximate`], plus [`crate::HumanizeError::NoPattern`] if the
    /// locale states no phrase for the chosen unit.
    pub fn write<W: fmt::Write>(&self, span: Duration, out: &mut W) -> HumanizeResult<()> {
        let approximated = approximate(span, &self.thresholds, self.policy)?;
        self.write_span(approximated, out)
    }

    /// Write an already-computed approximation.
    ///
    /// # Errors
    ///
    /// As [`Self::write`].
    pub fn write_span<W: fmt::Write>(
        &self,
        approximated: ApproximateSpan,
        out: &mut W,
    ) -> HumanizeResult<()> {
        let pattern = approximated.qualifier.pattern(&self.locale);
        let mut inner = |sink: &mut W| -> HumanizeResult<()> {
            if let Some(article) = render::indefinite_of(&self.locale, approximated.amount) {
                sink.write_str(article)?;
                return Ok(());
            }
            render::write_count(&self.locale, self.style, approximated.amount, sink)
        };
        render::write_pattern(pattern, &mut [&mut inner], out)
    }

    /// The hedged phrase for a span.
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::TimeUnit;

    fn locale(tag: &str) -> hc_i18n::Locale {
        tag.parse().expect("well-formed tag")
    }

    fn formatter(tag: &str) -> ApproximateFormatter {
        ApproximateFormatter::new(locale(tag))
    }

    fn say(tag: &str, seconds: i128) -> alloc::string::String {
        formatter(tag)
            .format(Duration::from_secs(seconds))
            .expect("a phrase")
    }

    const HOUR: i128 = 3_600;
    const DAY: i128 = 86_400;

    #[test]
    fn the_hedge_carries_what_the_round_number_lost() {
        assert_eq!(say("en", 3 * HOUR + 3 * 60), "about 3 hours");
        assert_eq!(say("en", 8 * DAY), "just over a week");
        assert_eq!(say("en", 350 * DAY), "nearly a year");
        assert_eq!(say("en", 400 * DAY), "just over a year");
        assert_eq!(say("en", 23 * HOUR), "nearly a day");
    }

    #[test]
    fn an_exact_span_gets_no_hedge_at_all() {
        assert_eq!(say("en", 3 * HOUR), "3 hours");
        assert_eq!(say("en", 2 * DAY), "2 days");
    }

    #[test]
    fn a_whole_part_of_zero_becomes_nearly_one_of_the_next_unit_up() {
        // 350 days is 0.958 years: the whole part is zero, and "0 years" is
        // not what anyone means.
        let span = Duration::from_secs(350 * DAY);
        let approximated =
            approximate(span, &Thresholds::DEFAULT, ApproximatePolicy::DEFAULT).expect("fits");
        assert_eq!(approximated.qualifier, Approximation::Nearly);
        assert_eq!(approximated.amount, UnitAmount::whole(1, TimeUnit::Year));
    }

    #[test]
    fn the_hedge_is_read_off_the_fraction_left_over() {
        let check = |seconds: i128, expected: Approximation| {
            let approximated = approximate(
                Duration::from_secs(seconds),
                &Thresholds::EXACT,
                ApproximatePolicy::DEFAULT,
            )
            .expect("fits");
            assert_eq!(approximated.qualifier, expected, "for {seconds} s");
        };
        check(3 * HOUR, Approximation::Exactly);
        check(3 * HOUR + 3 * 60, Approximation::About);
        check(3 * HOUR + 15 * 60, Approximation::JustOver);
        check(3 * HOUR + 30 * 60, Approximation::Over);
        check(3 * HOUR + 55 * 60, Approximation::Nearly);
    }

    #[test]
    fn the_boundaries_belong_to_the_caller() {
        let span = Duration::from_secs(3 * HOUR + 3 * 60);
        let default =
            approximate(span, &Thresholds::EXACT, ApproximatePolicy::DEFAULT).expect("fits");
        assert_eq!(default.qualifier, Approximation::About);
        // A caller who needs the phrase to be a true bound never says
        // "about".
        let bounded =
            approximate(span, &Thresholds::EXACT, ApproximatePolicy::BOUNDED).expect("fits");
        assert_eq!(bounded.qualifier, Approximation::JustOver);
        let never_exact = ApproximatePolicy {
            exact_within: 0.0,
            about_within: 1.0,
            just_over_within: 1.0,
            over_within: 1.0,
        };
        let always_about = approximate(span, &Thresholds::EXACT, never_exact).expect("fits");
        assert_eq!(always_about.qualifier, Approximation::About);
    }

    #[test]
    fn the_sign_is_dropped_because_an_approximation_is_a_length() {
        let forward = approximate(
            Duration::from_secs(8 * DAY),
            &Thresholds::DEFAULT,
            ApproximatePolicy::DEFAULT,
        )
        .expect("fits");
        let back = approximate(
            Duration::from_secs(-8 * DAY),
            &Thresholds::DEFAULT,
            ApproximatePolicy::DEFAULT,
        )
        .expect("fits");
        assert_eq!(forward, back);
        assert!(!back.amount.is_negative());
    }

    #[test]
    fn the_indefinite_article_replaces_the_numeral_where_a_language_has_one() {
        assert_eq!(say("en", 8 * DAY), "just over a week");
        assert_eq!(say("de", 8 * DAY), "etwas mehr als eine Woche");
        assert_eq!(say("fr", 8 * DAY), "une semaine et quelques");
        assert_eq!(say("es", 8 * DAY), "poco más de una semana");
        assert_eq!(say("it", 350 * DAY), "quasi un anno");
        assert_eq!(say("nl", 350 * DAY), "bijna een jaar");
    }

    #[test]
    fn a_language_with_no_indefinite_article_gets_the_numeral() {
        assert_eq!(say("ru", 350 * DAY), "почти 1 год");
        assert_eq!(say("ja", 350 * DAY), "1 年近く");
        assert_eq!(say("pl", 350 * DAY), "prawie 1 rok");
    }

    #[test]
    fn the_article_is_only_for_exactly_one_whole_unit() {
        assert_eq!(say("en", 2 * DAY), "2 days");
        assert_eq!(say("en", 3 * HOUR + 3 * 60), "about 3 hours");
    }

    #[test]
    fn the_hedge_wording_is_per_locale() {
        let span = 3 * HOUR + 3 * 60;
        assert_eq!(say("de", span), "ungefähr 3 Stunden");
        assert_eq!(say("fr", span), "environ 3 heures");
        assert_eq!(say("ja", span), "約3 時間");
        assert_eq!(say("cy", span), "tua 3 awr");
        assert_eq!(say("ko", span), "약 3시간");
    }

    #[test]
    fn a_hedge_a_caller_states_themselves_can_be_written_too() {
        let bound = ApproximateSpan {
            qualifier: Approximation::LessThan,
            amount: UnitAmount::whole(1, TimeUnit::Hour),
        };
        let mut text = alloc::string::String::new();
        formatter("en")
            .write_span(bound, &mut text)
            .expect("a phrase");
        assert_eq!(text, "less than an hour");
        text.clear();
        let bound = ApproximateSpan {
            qualifier: Approximation::MoreThan,
            amount: UnitAmount::whole(5, TimeUnit::Day),
        };
        formatter("en")
            .write_span(bound, &mut text)
            .expect("a phrase");
        assert_eq!(text, "more than 5 days");
    }

    #[test]
    fn every_shipped_locale_can_hedge_every_span() {
        for data in crate::data::LOCALES {
            let approximator = formatter(data.tag);
            for seconds in [50i128, 3 * HOUR + 3 * 60, 8 * DAY, 350 * DAY, 400 * DAY] {
                let phrase = approximator
                    .format(Duration::from_secs(seconds))
                    .unwrap_or_else(|_| panic!("{} {seconds}", data.tag));
                assert!(!phrase.is_empty(), "{} {seconds}", data.tag);
                assert!(!phrase.contains("{0}"), "{} {seconds}", data.tag);
            }
        }
    }

    #[test]
    fn the_hedge_patterns_come_from_the_locale_chain() {
        assert_eq!(Approximation::About.pattern(&locale("en-AU")), "about {0}");
        assert_eq!(Approximation::Exactly.pattern(&locale("en")), "{0}");
        // An unknown locale gets the root's language-free markers.
        assert_eq!(Approximation::About.pattern(&locale("xx")), "~{0}");
    }
}
