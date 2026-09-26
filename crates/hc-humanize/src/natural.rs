//! The functions of Python's `humanize` package, with its thresholds and its
//! phrasing.
//!
//! The rest of this crate phrases time the way CLDR does. This module
//! phrases it the way the Python `humanize` package does — *a moment*,
//! *an hour*, *1 year, 3 months*, *2 days, 1 hour and 33.12 seconds* — for a
//! caller porting code that already depends on those exact strings. The two
//! are different conventions and neither is wrong, so they are separate
//! (policy §5): [`crate::RelativeTimeFormatter`] is the CLDR one.
//!
//! The behaviour is that of `humanize` 4.x as its documentation states it
//! (<https://humanize.readthedocs.io/en/latest/time/> and `/number/`,
//! retrieved 2026-09-26), including the source shown there for
//! `naturaldelta` and `naturaltime`. `humanize` is MIT-licensed; nothing is
//! copied from it but the thresholds and the English strings, which are the
//! behaviour being reproduced.
//!
//! # What differs from Python, and why
//!
//! * **No clock.** `naturaltime`, `naturalday` and `naturaldate` take the
//!   present from the caller, as every entry point of this crate does.
//! * **Typed options.** `minimum_unit` and `suppress` are [`PreciseUnit`]
//!   values rather than strings, and `precisedelta`'s `format="%0.2f"` is a
//!   number of decimals. A value that is not a number is a type error here,
//!   so the "return `str(value)` unchanged" paths do not exist.
//! * **Microseconds.** A [`Duration`] is exact to the attosecond; these
//!   functions read it, as Python's `timedelta` would hold it, truncated to
//!   the microsecond.
//! * **One vocabulary.** [`NaturalPhrases::ENGLISH`] is `humanize`'s source
//!   strings. `humanize` ships gettext catalogues for about forty languages;
//!   none of them is carried here, because they were not read. A language is
//!   added as one more [`NaturalPhrases`] value, and its plurals are chosen
//!   by [`hc_i18n::PluralRules`], never by comparing a count to one.
//! * **`intcomma` separators** are a [`Grouping`] the caller passes rather
//!   than `humanize`'s per-locale table, for the same reason.
//! * **`naturalsize`** is not here: it formats bytes, not time.

use core::fmt;
use core::fmt::Write as _;

use hc_calendar::{CivilDateTime, Rd};
use hc_core::Duration;
use hc_format::patterns::{FormatContext, strftime};
use hc_i18n::{PluralCategory, PluralRules};

use crate::error::{HumanizeError, HumanizeResult};

const MICROS_PER_SECOND: i128 = 1_000_000;
const MICROS_PER_DAY: i128 = 86_400 * MICROS_PER_SECOND;

/// A singular and a plural phrase, each holding `{0}` where the number goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PluralPair {
    /// The form for CLDR's `one`.
    pub one: &'static str,
    /// The form for every other category.
    pub other: &'static str,
}

/// Every string the `humanize` functions produce, in one language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct NaturalPhrases {
    /// The language whose plural rules choose between the forms.
    pub language: &'static str,
    /// Below the smallest unit: `a moment`.
    pub a_moment: &'static str,
    /// `a second`.
    pub a_second: &'static str,
    /// `a minute`.
    pub a_minute: &'static str,
    /// `an hour`.
    pub an_hour: &'static str,
    /// `a day`.
    pub a_day: &'static str,
    /// `a month`.
    pub a_month: &'static str,
    /// `a year`.
    pub a_year: &'static str,
    /// `1 year, 1 month`.
    pub one_year_one_month: &'static str,
    /// `1 year, {0} day(s)`.
    pub one_year_days: PluralPair,
    /// `1 year, {0} month(s)`.
    pub one_year_months: PluralPair,
    /// `{0} microsecond(s)`.
    pub microseconds: PluralPair,
    /// `{0} millisecond(s)`.
    pub milliseconds: PluralPair,
    /// `{0} second(s)`.
    pub seconds: PluralPair,
    /// `{0} minute(s)`.
    pub minutes: PluralPair,
    /// `{0} hour(s)`.
    pub hours: PluralPair,
    /// `{0} day(s)`.
    pub days: PluralPair,
    /// `{0} month(s)`.
    pub months: PluralPair,
    /// `{0} year(s)`.
    pub years: PluralPair,
    /// `now`, which `naturaltime` says instead of `a moment ago`.
    pub now: &'static str,
    /// `{0} ago`.
    pub ago: &'static str,
    /// `{0} from now`.
    pub from_now: &'static str,
    /// `today`.
    pub today: &'static str,
    /// `tomorrow`.
    pub tomorrow: &'static str,
    /// `yesterday`.
    pub yesterday: &'static str,
    /// The separator between list items but the last: `, `.
    pub list_separator: &'static str,
    /// The last two list items: `{0} and {1}`.
    pub list_last: &'static str,
    /// The ordinal suffix of 11, 12 and 13: `th`.
    pub ordinal_teens: &'static str,
    /// The ordinal suffix by last digit, 0 through 9.
    pub ordinal_by_last_digit: [&'static str; 10],
    /// The names of the powers of a thousand, from `thousand` (10³) to
    /// `decillion` (10³³), as singular and plural.
    pub powers: [PluralPair; 11],
}

const fn same(word: &'static str) -> PluralPair {
    PluralPair {
        one: word,
        other: word,
    }
}

impl NaturalPhrases {
    /// `humanize`'s own strings: the English message identifiers its
    /// catalogues translate.
    pub const ENGLISH: Self = Self {
        language: "en",
        a_moment: "a moment",
        a_second: "a second",
        a_minute: "a minute",
        an_hour: "an hour",
        a_day: "a day",
        a_month: "a month",
        a_year: "a year",
        one_year_one_month: "1 year, 1 month",
        one_year_days: PluralPair {
            one: "1 year, {0} day",
            other: "1 year, {0} days",
        },
        one_year_months: PluralPair {
            one: "1 year, {0} month",
            other: "1 year, {0} months",
        },
        microseconds: PluralPair {
            one: "{0} microsecond",
            other: "{0} microseconds",
        },
        milliseconds: PluralPair {
            one: "{0} millisecond",
            other: "{0} milliseconds",
        },
        seconds: PluralPair {
            one: "{0} second",
            other: "{0} seconds",
        },
        minutes: PluralPair {
            one: "{0} minute",
            other: "{0} minutes",
        },
        hours: PluralPair {
            one: "{0} hour",
            other: "{0} hours",
        },
        days: PluralPair {
            one: "{0} day",
            other: "{0} days",
        },
        months: PluralPair {
            one: "{0} month",
            other: "{0} months",
        },
        years: PluralPair {
            one: "{0} year",
            other: "{0} years",
        },
        now: "now",
        ago: "{0} ago",
        from_now: "{0} from now",
        today: "today",
        tomorrow: "tomorrow",
        yesterday: "yesterday",
        list_separator: ", ",
        list_last: "{0} and {1}",
        ordinal_teens: "th",
        ordinal_by_last_digit: ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"],
        powers: [
            same("thousand"),
            same("million"),
            same("billion"),
            same("trillion"),
            same("quadrillion"),
            same("quintillion"),
            same("sextillion"),
            same("septillion"),
            same("octillion"),
            same("nonillion"),
            same("decillion"),
        ],
    };
}

/// A unit `precisedelta` and `naturaldelta` can stop at, smallest first as
/// `humanize`'s `Unit` enumeration orders them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PreciseUnit {
    /// A microsecond.
    Microseconds,
    /// A thousand microseconds.
    Milliseconds,
    /// A second.
    Seconds,
    /// Sixty seconds.
    Minutes,
    /// Sixty minutes.
    Hours,
    /// 86 400 seconds.
    Days,
    /// 30.5 days, `humanize`'s month.
    Months,
    /// 365 days, `humanize`'s year.
    Years,
}

impl PreciseUnit {
    /// Largest first, the order the phrases are written in.
    const DESCENDING: [Self; 8] = [
        Self::Years,
        Self::Months,
        Self::Days,
        Self::Hours,
        Self::Minutes,
        Self::Seconds,
        Self::Milliseconds,
        Self::Microseconds,
    ];

    /// The unit's length in microseconds.
    const fn micros(self) -> i128 {
        match self {
            Self::Microseconds => 1,
            Self::Milliseconds => 1_000,
            Self::Seconds => MICROS_PER_SECOND,
            Self::Minutes => 60 * MICROS_PER_SECOND,
            Self::Hours => 3_600 * MICROS_PER_SECOND,
            Self::Days => MICROS_PER_DAY,
            Self::Months => 61 * MICROS_PER_DAY / 2,
            Self::Years => 365 * MICROS_PER_DAY,
        }
    }

    /// The resolution `humanize` computes this unit's fraction from: whole
    /// days for years and months, whole seconds for days, hours and minutes,
    /// microseconds below that. The numerator and denominator of the unit's
    /// length in that resolution come with it.
    const fn fraction_basis(self) -> (i128, i128, i128) {
        match self {
            Self::Years => (MICROS_PER_DAY, 365, 1),
            Self::Months => (MICROS_PER_DAY, 61, 2),
            Self::Days => (MICROS_PER_SECOND, 86_400, 1),
            Self::Hours => (MICROS_PER_SECOND, 3_600, 1),
            Self::Minutes => (MICROS_PER_SECOND, 60, 1),
            Self::Seconds => (1, MICROS_PER_SECOND, 1),
            Self::Milliseconds => (1, 1_000, 1),
            Self::Microseconds => (1, 1, 1),
        }
    }

    const fn phrases(self, phrases: &NaturalPhrases) -> PluralPair {
        match self {
            Self::Microseconds => phrases.microseconds,
            Self::Milliseconds => phrases.milliseconds,
            Self::Seconds => phrases.seconds,
            Self::Minutes => phrases.minutes,
            Self::Hours => phrases.hours,
            Self::Days => phrases.days,
            Self::Months => phrases.months,
            Self::Years => phrases.years,
        }
    }
}

/// How `naturaldelta` and `naturaltime` phrase a span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeltaOptions {
    /// Whether months (of 30.5 days) are used between days and years:
    /// `humanize`'s `months`, `true` by default.
    pub months: bool,
    /// The smallest unit: seconds (the default), milliseconds or
    /// microseconds.
    pub minimum_unit: PreciseUnit,
}

impl Default for DeltaOptions {
    fn default() -> Self {
        Self {
            months: true,
            minimum_unit: PreciseUnit::Seconds,
        }
    }
}

/// Thousands and decimal separators for [`Natural::write_intcomma`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grouping {
    /// Between groups of three digits.
    pub thousands: &'static str,
    /// Before the fraction.
    pub decimal: &'static str,
}

impl Grouping {
    /// `1,234,567.25`, `humanize`'s default.
    pub const ENGLISH: Self = Self {
        thousands: ",",
        decimal: ".",
    };
}

/// A phrase before its number is written.
#[derive(Debug, Clone, Copy)]
enum Phrase {
    Fixed(&'static str),
    Count(PluralPair, i128),
    Grouped(PluralPair, i128),
}

/// The `humanize` functions, in one vocabulary.
///
/// ```
/// use hc_core::Duration;
/// use hc_humanize::natural::{DeltaOptions, Natural};
///
/// let natural = Natural::english();
/// assert_eq!(natural.naturaldelta(Duration::from_minutes(30), DeltaOptions::default())?, "30 minutes");
/// assert_eq!(natural.ordinal(103)?, "103rd");
/// # Ok::<(), hc_humanize::HumanizeError>(())
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Natural {
    phrases: &'static NaturalPhrases,
    rules: PluralRules,
}

impl Natural {
    /// `humanize`'s English.
    #[must_use]
    pub fn english() -> Self {
        Self::with_phrases(&NaturalPhrases::ENGLISH)
    }

    /// Another vocabulary, with the plural rules of its language.
    #[must_use]
    pub fn with_phrases(phrases: &'static NaturalPhrases) -> Self {
        let rules = PluralRules::for_language(phrases.language)
            .unwrap_or_else(|| PluralRules::for_locale(&hc_i18n::Locale::ROOT));
        Self { phrases, rules }
    }

    /// The vocabulary in use.
    #[must_use]
    pub const fn phrases(&self) -> &'static NaturalPhrases {
        self.phrases
    }

    fn pick(&self, pair: PluralPair, count: i128) -> &'static str {
        let count = i64::try_from(count).unwrap_or(i64::MAX);
        if self.rules.select_integer(count) == PluralCategory::One {
            pair.one
        } else {
            pair.other
        }
    }

    // --- naturaldelta and naturaltime --------------------------------------

    /// `naturaldelta(value, months, minimum_unit)`: the span, without tense.
    ///
    /// The sign is ignored, as Python's `abs(delta)` ignores it.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::Unsupported`] when the minimum unit is not seconds,
    /// milliseconds or microseconds — Python's `ValueError` —
    /// [`HumanizeError::Overflow`] for a span of more than about 10²⁶ years,
    /// and [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_naturaldelta<W: fmt::Write>(
        &self,
        out: &mut W,
        delta: Duration,
        options: DeltaOptions,
    ) -> HumanizeResult<()> {
        let phrase = self.delta_phrase(delta, options)?;
        self.write_phrase(out, phrase)
    }

    /// `naturaltime(value, when=now)` for two readings: *3 hours ago*,
    /// *3 hours from now*, or *now*.
    ///
    /// Both readings are naive, as Python's are when `value` has no
    /// `tzinfo`, and every day counts 86 400 seconds.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_naturaldelta`].
    pub fn write_naturaltime<W: fmt::Write>(
        &self,
        out: &mut W,
        value: CivilDateTime,
        now: CivilDateTime,
        options: DeltaOptions,
    ) -> HumanizeResult<()> {
        let delta = now
            .nominal_duration_since(value)
            .map_err(|_| HumanizeError::Overflow)?;
        self.write_naturaltime_delta(out, delta, options)
    }

    /// `naturaltime(timedelta)`: a span is how long *ago*, so a positive one
    /// is in the past and a negative one in the future, as in Python.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_naturaldelta`].
    pub fn write_naturaltime_delta<W: fmt::Write>(
        &self,
        out: &mut W,
        delta: Duration,
        options: DeltaOptions,
    ) -> HumanizeResult<()> {
        let phrase = self.delta_phrase(delta, options)?;
        if let Phrase::Fixed(text) = phrase
            && text == self.phrases.a_moment
        {
            out.write_str(self.phrases.now)?;
            return Ok(());
        }
        let pattern = if delta.is_negative() {
            self.phrases.from_now
        } else {
            self.phrases.ago
        };
        let (before, after) = pattern.split_once("{0}").unwrap_or((pattern, ""));
        out.write_str(before)?;
        self.write_phrase(out, phrase)?;
        out.write_str(after)?;
        Ok(())
    }

    fn delta_phrase(&self, delta: Duration, options: DeltaOptions) -> HumanizeResult<Phrase> {
        let minimum = options.minimum_unit;
        if minimum > PreciseUnit::Seconds {
            return Err(HumanizeError::Unsupported("a minimum unit above seconds"));
        }
        let phrases = self.phrases;
        let total = abs_micros(delta)?;
        let days = total / MICROS_PER_DAY;
        let seconds = total % MICROS_PER_DAY / MICROS_PER_SECOND;
        let micros = total % MICROS_PER_SECOND;
        let years = days / 365;
        let days_left = days % 365;
        // `round(days / 30.5)`, half to even.
        let months = round_half_even(days_left * 2, 61);

        if years == 0 && days_left < 1 {
            if seconds == 0 {
                if minimum == PreciseUnit::Microseconds && micros < 1_000 {
                    return Ok(Phrase::Count(phrases.microseconds, micros));
                }
                if minimum == PreciseUnit::Milliseconds
                    || (minimum == PreciseUnit::Microseconds && micros >= 1_000)
                {
                    return Ok(Phrase::Count(phrases.milliseconds, micros / 1_000));
                }
                return Ok(Phrase::Fixed(phrases.a_moment));
            }
            if seconds == 1 {
                return Ok(Phrase::Fixed(phrases.a_second));
            }
            if seconds < 60 {
                return Ok(Phrase::Count(phrases.seconds, seconds));
            }
            if seconds < 3_600 {
                return Ok(match round_half_even(seconds, 60) {
                    1 => Phrase::Fixed(phrases.a_minute),
                    60 => Phrase::Fixed(phrases.an_hour),
                    minutes => Phrase::Count(phrases.minutes, minutes),
                });
            }
            return Ok(match round_half_even(seconds, 3_600) {
                1 => Phrase::Fixed(phrases.an_hour),
                24 => Phrase::Fixed(phrases.a_day),
                hours => Phrase::Count(phrases.hours, hours),
            });
        }
        if years == 0 {
            if days_left == 1 {
                return Ok(Phrase::Fixed(phrases.a_day));
            }
            if !options.months || months == 0 {
                return Ok(Phrase::Count(phrases.days, days_left));
            }
            return Ok(match months {
                1 => Phrase::Fixed(phrases.a_month),
                12 => Phrase::Fixed(phrases.a_year),
                months => Phrase::Count(phrases.months, months),
            });
        }
        if years == 1 {
            if months == 0 && days_left == 0 {
                return Ok(Phrase::Fixed(phrases.a_year));
            }
            if months == 0 || !options.months {
                return Ok(Phrase::Count(phrases.one_year_days, days_left));
            }
            return Ok(match months {
                1 => Phrase::Fixed(phrases.one_year_one_month),
                12 => Phrase::Count(phrases.years, 2),
                months => Phrase::Count(phrases.one_year_months, months),
            });
        }
        Ok(Phrase::Grouped(phrases.years, round_half_even(days, 365)))
    }

    fn write_phrase<W: fmt::Write>(&self, out: &mut W, phrase: Phrase) -> HumanizeResult<()> {
        match phrase {
            Phrase::Fixed(text) => out.write_str(text)?,
            Phrase::Count(pair, count) => {
                substitute(out, self.pick(pair, count), |out| write!(out, "{count}"))?;
            }
            Phrase::Grouped(pair, count) => {
                substitute(out, self.pick(pair, count), |out| {
                    write_grouped(out, count, Grouping::ENGLISH.thousands)
                })?;
            }
        }
        Ok(())
    }

    // --- precisedelta ------------------------------------------------------

    /// `precisedelta(value, minimum_unit, suppress, format="%0.{decimals}f")`.
    ///
    /// Units are listed largest first and joined `a, b and c`; units in
    /// `suppress` are folded into the next smaller one; the smallest unit
    /// written takes the fraction of what is left, to `decimals` places.
    /// The sign is ignored, as in Python.
    ///
    /// The fraction is computed as `humanize` computes it — the fraction of
    /// a month or year from whole days, of a day, hour or minute from whole
    /// seconds — and rounded half to even from the nearest double, as
    /// Python's `%` formatting rounds it.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::Unsupported`] when the minimum unit is suppressed and
    /// no larger unit is left — Python's `ValueError` —
    /// [`HumanizeError::Overflow`] for an unrepresentable span, and
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_precisedelta<W: fmt::Write>(
        &self,
        out: &mut W,
        delta: Duration,
        minimum_unit: PreciseUnit,
        suppress: &[PreciseUnit],
        decimals: u8,
    ) -> HumanizeResult<()> {
        let minimum = suitable_minimum(minimum_unit, suppress)?;
        let suppressed = |unit: PreciseUnit| unit < minimum || suppress.contains(&unit);
        let mut remaining = abs_micros(delta)?;
        // Whole counts for the units above the minimum, and the minimum's
        // own value as a double.
        let mut counts = [0i128; 8];
        let mut fraction = 0.0f64;
        for (index, unit) in PreciseUnit::DESCENDING.into_iter().enumerate() {
            if unit == minimum {
                let (resolution, numerator, denominator) = unit.fraction_basis();
                let basis = remaining / resolution;
                fraction = (basis * denominator) as f64 / numerator as f64;
                break;
            }
            if suppressed(unit) {
                continue;
            }
            // `humanize` divides whole days by 30.5 for months, so a month
            // is only taken from whole days.
            let quotient = if unit == PreciseUnit::Months {
                remaining / MICROS_PER_DAY * 2 / 61
            } else {
                remaining / unit.micros()
            };
            remaining -= quotient * unit.micros();
            if let Some(slot) = counts.get_mut(index) {
                *slot = quotient;
            }
        }

        // The non-zero units above the minimum, largest first; at most
        // seven, so they are held on the stack.
        let mut items = [(PreciseUnit::Years, 0i128); 8];
        let mut whole_items = 0usize;
        for (index, unit) in PreciseUnit::DESCENDING.into_iter().enumerate() {
            if unit == minimum {
                break;
            }
            let count = counts.get(index).copied().unwrap_or(0);
            if count > 0
                && let Some(slot) = items.get_mut(whole_items)
            {
                *slot = (unit, count);
                whole_items += 1;
            }
        }
        // The minimum is written when it is non-zero, or when nothing else
        // was: `0 minutes`.
        let with_minimum = fraction > 0.0 || whole_items == 0;
        let total = whole_items + usize::from(with_minimum);
        for position in 0..total {
            if position > 0 {
                self.write_joiner(out, position, total)?;
            }
            match items.get(position) {
                Some(&item) if position < whole_items => {
                    self.write_item(out, item, None, decimals, minimum)?;
                }
                _ => self.write_item(out, (minimum, 0), Some(fraction), decimals, minimum)?,
            }
        }
        Ok(())
    }

    fn write_joiner<W: fmt::Write>(
        &self,
        out: &mut W,
        position: usize,
        total: usize,
    ) -> HumanizeResult<()> {
        if position + 1 == total {
            let joiner = self
                .phrases
                .list_last
                .split_once("{0}")
                .and_then(|(_, rest)| rest.split_once("{1}"))
                .map_or(" and ", |(middle, _)| middle);
            out.write_str(joiner)?;
        } else {
            out.write_str(self.phrases.list_separator)?;
        }
        Ok(())
    }

    fn write_item<W: fmt::Write>(
        &self,
        out: &mut W,
        (unit, count): (PreciseUnit, i128),
        fraction: Option<f64>,
        decimals: u8,
        minimum: PreciseUnit,
    ) -> HumanizeResult<()> {
        let pair = unit.phrases(self.phrases);
        let Some(value) = fraction else {
            let text = self.pick(pair, count);
            return if unit == PreciseUnit::Years {
                substitute(out, text, |out| {
                    write_grouped(out, count, Grouping::ENGLISH.thousands)
                })
            } else {
                substitute(out, text, |out| write!(out, "{count}"))
            };
        };
        // `2 if 1 < value < 2 else int(value)` picks the plural.
        let whole = hc_core::math::trunc(value);
        let chooser = if value > 1.0 && value < 2.0 {
            2
        } else {
            whole as i128
        };
        let text = self.pick(pair, chooser);
        let precision = usize::from(decimals);
        if value - whole > 0.0 {
            substitute(out, text, |out| write!(out, "{value:.precision$}"))
        } else if minimum == PreciseUnit::Years {
            // Python formats a whole year count that is still a float with
            // `intcomma`, which writes it as `2.0`.
            substitute(out, text, |out| {
                write_grouped(out, whole as i128, Grouping::ENGLISH.thousands)?;
                out.write_str(".0")
            })
        } else {
            substitute(out, text, |out| write!(out, "{}", whole as i128))
        }
    }

    // --- naturalday and naturaldate ----------------------------------------

    /// `naturalday(value, format)`, with today supplied: *today*,
    /// *tomorrow*, *yesterday*, or the day written with a `strftime`
    /// pattern in the C locale. Python's default pattern is `%b %d`.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::Unsupported`] for a pattern `hc-format` cannot write,
    /// and [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_naturalday<W: fmt::Write>(
        &self,
        out: &mut W,
        day: Rd,
        today: Rd,
        pattern: &str,
    ) -> HumanizeResult<()> {
        let word = match day.checked_days_since(today) {
            Ok(0) => Some(self.phrases.today),
            Ok(1) => Some(self.phrases.tomorrow),
            Ok(-1) => Some(self.phrases.yesterday),
            _ => None,
        };
        if let Some(word) = word {
            out.write_str(word)?;
            return Ok(());
        }
        strftime::format(
            out,
            pattern,
            &FormatContext::new(CivilDateTime::midnight(day)),
        )
        .map_err(|error| match error {
            hc_format::FormatError::Sink => HumanizeError::WriteFailed,
            _ => HumanizeError::Unsupported("the strftime pattern"),
        })
    }

    /// `naturaldate(value)`: [`Natural::write_naturalday`], with the year
    /// added (`%b %d %Y`) once the day is at least five months — 5 × 365 / 12
    /// days — from today.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_naturalday`].
    pub fn write_naturaldate<W: fmt::Write>(
        &self,
        out: &mut W,
        day: Rd,
        today: Rd,
    ) -> HumanizeResult<()> {
        let distance = day
            .checked_days_since(today)
            .map_err(|_| HumanizeError::Overflow)?
            .unsigned_abs();
        // `delta.days >= 5 * 365 / 12`, in integers.
        let pattern = if distance.saturating_mul(12) >= 5 * 365 {
            "%b %d %Y"
        } else {
            "%b %d"
        };
        self.write_naturalday(out, day, today, pattern)
    }

    // --- numbers -----------------------------------------------------------

    /// `ordinal(value)`: `1st`, `2nd`, `103rd`, `111th`.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_ordinal<W: fmt::Write>(&self, out: &mut W, value: i128) -> HumanizeResult<()> {
        let suffix = if matches!(value.rem_euclid(100), 11..=13) {
            self.phrases.ordinal_teens
        } else {
            let digit = value.rem_euclid(10) as usize;
            self.phrases
                .ordinal_by_last_digit
                .get(digit)
                .copied()
                .unwrap_or("")
        };
        write!(out, "{value}{suffix}")?;
        Ok(())
    }

    /// `intcomma(value)` for an integer: `1,000,000`.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_intcomma<W: fmt::Write>(
        &self,
        out: &mut W,
        value: i128,
        grouping: Grouping,
    ) -> HumanizeResult<()> {
        write_grouped(out, value, grouping.thousands)?;
        Ok(())
    }

    /// `intcomma(value, ndigits)` for a float: `1,234,567.25`, or with
    /// `ndigits = Some(2)`, `1,234.55`.
    ///
    /// Without `ndigits` the number is written as Python's `repr` would
    /// write it — the shortest digits that read back as the same double,
    /// with `.0` on a whole number and an exponent from 10¹⁶ up and below
    /// 10⁻⁴ — and only the digits before the point are grouped. A value that
    /// is not finite is `NaN`, `+Inf` or `-Inf`, as `humanize` writes it.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_intcomma_f64<W: fmt::Write>(
        &self,
        out: &mut W,
        value: f64,
        ndigits: Option<u8>,
        grouping: Grouping,
    ) -> HumanizeResult<()> {
        if value.is_nan() {
            out.write_str("NaN")?;
            return Ok(());
        }
        if value.is_infinite() {
            out.write_str(if value < 0.0 { "-Inf" } else { "+Inf" })?;
            return Ok(());
        }
        let mut buffer = DigitBuffer::default();
        match ndigits {
            Some(digits) => {
                let precision = usize::from(digits);
                write!(buffer, "{value:.precision$}")?;
            }
            None => write_python_repr(&mut buffer, value)?,
        }
        let text = buffer.as_str();
        if text.contains('e') {
            out.write_str(text)?;
            return Ok(());
        }
        let (sign, unsigned) = match text.strip_prefix('-') {
            Some(rest) => ("-", rest),
            None => ("", text),
        };
        let (whole, fraction) = match unsigned.split_once('.') {
            Some((whole, fraction)) => (whole, Some(fraction)),
            None => (unsigned, None),
        };
        out.write_str(sign)?;
        write_grouped_digits(out, whole, grouping.thousands)?;
        if let Some(fraction) = fraction {
            out.write_str(grouping.decimal)?;
            out.write_str(fraction)?;
        }
        Ok(())
    }

    /// `intword(value, format="%.{decimals}f")`: `12.4 thousand`,
    /// `1.2 billion`, `8.1 decillion`. Below a thousand the number is
    /// written as it is.
    ///
    /// A value that rounds up to the next power is written in it:
    /// `999 999` is `1.0 million`, not `1000.0 thousand`. `humanize`'s last
    /// power, the googol, is beyond any `i128`.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_intword<W: fmt::Write>(
        &self,
        out: &mut W,
        value: i128,
        decimals: u8,
    ) -> HumanizeResult<()> {
        let magnitude = value.unsigned_abs();
        let sign = if value < 0 { "-" } else { "" };
        if magnitude < 1_000 {
            write!(out, "{value}")?;
            return Ok(());
        }
        // The largest power of a thousand not above the value, as an index
        // into `powers` (0 is a thousand), capped at the decillion.
        let mut index = 0usize;
        let mut power = 1_000u128;
        while index + 1 < self.phrases.powers.len() && magnitude / 1_000 >= power {
            power *= 1_000;
            index += 1;
        }
        let precision = usize::from(decimals);
        let mut chopped = magnitude as f64 / power as f64;
        // Rounding may reach the next power: 999 999 → "1000.0 thousand".
        let mut buffer = DigitBuffer::default();
        write!(buffer, "{chopped:.precision$}")?;
        let rounded: f64 = buffer.as_str().parse().unwrap_or(chopped);
        if rounded == 1_000.0 && index + 1 < self.phrases.powers.len() {
            index += 1;
            power *= 1_000;
            chopped = magnitude as f64 / power as f64;
        }
        let pair = self
            .phrases
            .powers
            .get(index)
            .copied()
            .unwrap_or(PluralPair { one: "", other: "" });
        let name = self.pick(pair, hc_core::math::ceil(chopped) as i128);
        write!(out, "{sign}{chopped:.precision$} {name}")?;
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl Natural {
    fn collect(
        write: impl FnOnce(&mut alloc::string::String) -> HumanizeResult<()>,
    ) -> HumanizeResult<alloc::string::String> {
        let mut out = alloc::string::String::new();
        write(&mut out)?;
        Ok(out)
    }

    /// [`Natural::write_naturaldelta`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_naturaldelta`].
    pub fn naturaldelta(
        &self,
        delta: Duration,
        options: DeltaOptions,
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_naturaldelta(out, delta, options))
    }

    /// [`Natural::write_naturaltime`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_naturaltime`].
    pub fn naturaltime(
        &self,
        value: CivilDateTime,
        now: CivilDateTime,
        options: DeltaOptions,
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_naturaltime(out, value, now, options))
    }

    /// [`Natural::write_naturaltime_delta`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_naturaltime_delta`].
    pub fn naturaltime_delta(
        &self,
        delta: Duration,
        options: DeltaOptions,
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_naturaltime_delta(out, delta, options))
    }

    /// [`Natural::write_precisedelta`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_precisedelta`].
    pub fn precisedelta(
        &self,
        delta: Duration,
        minimum_unit: PreciseUnit,
        suppress: &[PreciseUnit],
        decimals: u8,
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_precisedelta(out, delta, minimum_unit, suppress, decimals))
    }

    /// [`Natural::write_naturalday`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_naturalday`].
    pub fn naturalday(
        &self,
        day: Rd,
        today: Rd,
        pattern: &str,
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_naturalday(out, day, today, pattern))
    }

    /// [`Natural::write_naturaldate`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_naturaldate`].
    pub fn naturaldate(&self, day: Rd, today: Rd) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_naturaldate(out, day, today))
    }

    /// [`Natural::write_ordinal`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_ordinal`].
    pub fn ordinal(&self, value: i128) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_ordinal(out, value))
    }

    /// [`Natural::write_intcomma`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_intcomma`].
    pub fn intcomma(
        &self,
        value: i128,
        grouping: Grouping,
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_intcomma(out, value, grouping))
    }

    /// [`Natural::write_intcomma_f64`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_intcomma_f64`].
    pub fn intcomma_f64(
        &self,
        value: f64,
        ndigits: Option<u8>,
        grouping: Grouping,
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_intcomma_f64(out, value, ndigits, grouping))
    }

    /// [`Natural::write_intword`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_intword`].
    pub fn intword(&self, value: i128, decimals: u8) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_intword(out, value, decimals))
    }
}

/// The minimum unit, moved up past suppressed units as `humanize`'s
/// `_suitable_minimum_unit` does.
fn suitable_minimum(minimum: PreciseUnit, suppress: &[PreciseUnit]) -> HumanizeResult<PreciseUnit> {
    if !suppress.contains(&minimum) {
        return Ok(minimum);
    }
    PreciseUnit::DESCENDING
        .into_iter()
        .rev()
        .find(|unit| *unit > minimum && !suppress.contains(unit))
        .ok_or(HumanizeError::Unsupported(
            "a suppressed minimum unit with no larger unit left",
        ))
}

/// The absolute span in whole microseconds, truncated.
fn abs_micros(delta: Duration) -> HumanizeResult<i128> {
    let span = delta.checked_abs()?;
    span.whole_seconds()
        .checked_mul(MICROS_PER_SECOND)
        .and_then(|micros| micros.checked_add(i128::from(span.subsec_attos() / 1_000_000_000_000)))
        .ok_or(HumanizeError::Overflow)
}

/// `numerator / denominator` rounded to the nearest integer, ties to even,
/// as Python's `round` rounds; both are non-negative.
const fn round_half_even(numerator: i128, denominator: i128) -> i128 {
    let quotient = numerator / denominator;
    let twice_remainder = 2 * (numerator % denominator);
    if twice_remainder > denominator || (twice_remainder == denominator && quotient % 2 == 1) {
        quotient + 1
    } else {
        quotient
    }
}

/// Write `pattern` with `{0}` replaced by whatever `number` writes.
fn substitute<W: fmt::Write>(
    out: &mut W,
    pattern: &str,
    number: impl FnOnce(&mut W) -> fmt::Result,
) -> HumanizeResult<()> {
    match pattern.split_once("{0}") {
        Some((before, after)) => {
            out.write_str(before)?;
            number(out)?;
            out.write_str(after)?;
        }
        None => out.write_str(pattern)?,
    }
    Ok(())
}

/// An integer with its digits in groups of three.
fn write_grouped<W: fmt::Write>(out: &mut W, value: i128, separator: &str) -> fmt::Result {
    let mut buffer = DigitBuffer::default();
    write!(buffer, "{}", value.unsigned_abs())?;
    if value < 0 {
        out.write_char('-')?;
    }
    write_grouped_digits(out, buffer.as_str(), separator)
}

fn write_grouped_digits<W: fmt::Write>(out: &mut W, digits: &str, separator: &str) -> fmt::Result {
    let length = digits.len();
    for (index, digit) in digits.char_indices() {
        if index > 0 && (length - index).is_multiple_of(3) {
            out.write_str(separator)?;
        }
        out.write_char(digit)?;
    }
    Ok(())
}

/// Python's `repr` of a float: the shortest round-trip digits, `.0` on a
/// whole number, and exponent notation outside `[1e-4, 1e16)`.
fn write_python_repr<W: fmt::Write>(out: &mut W, value: f64) -> fmt::Result {
    let magnitude = value.abs();
    if magnitude != 0.0 && !(1e-4..1e16).contains(&magnitude) {
        let mut buffer = DigitBuffer::default();
        write!(buffer, "{value:e}")?;
        let text = buffer.as_str();
        let (mantissa, exponent) = text.split_once('e').unwrap_or((text, "0"));
        let (sign, digits) = match exponent.strip_prefix('-') {
            Some(rest) => ('-', rest),
            None => ('+', exponent),
        };
        write!(out, "{mantissa}e{sign}")?;
        if digits.len() < 2 {
            out.write_char('0')?;
        }
        return out.write_str(digits);
    }
    if hc_core::math::trunc(value) == value {
        write!(out, "{value:.1}")
    } else {
        write!(out, "{value}")
    }
}

/// A stack buffer long enough for any number these functions write: the
/// largest double with 255 decimals is 565 bytes.
struct DigitBuffer {
    bytes: [u8; 640],
    length: usize,
}

impl Default for DigitBuffer {
    fn default() -> Self {
        Self {
            bytes: [0; 640],
            length: 0,
        }
    }
}

impl DigitBuffer {
    fn as_str(&self) -> &str {
        self.bytes
            .get(..self.length)
            .and_then(|bytes| core::str::from_utf8(bytes).ok())
            .unwrap_or("")
    }
}

impl fmt::Write for DigitBuffer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self.length + text.len();
        let slot = self.bytes.get_mut(self.length..end).ok_or(fmt::Error)?;
        slot.copy_from_slice(text.as_bytes());
        self.length = end;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use hc_calendar::CivilTime;

    fn natural() -> Natural {
        Natural::english()
    }

    fn delta(span: Duration) -> String {
        natural()
            .naturaldelta(span, DeltaOptions::default())
            .unwrap()
    }

    fn with_minimum(span: Duration, minimum_unit: PreciseUnit) -> String {
        natural()
            .naturaldelta(
                span,
                DeltaOptions {
                    months: true,
                    minimum_unit,
                },
            )
            .unwrap()
    }

    /// `humanize`'s documentation:
    ///
    /// ```text
    /// >>> later = now + dt.timedelta(minutes=30)
    /// >>> naturaldelta(later - now)
    /// '30 minutes'
    /// ```
    #[test]
    fn naturaldelta_matches_the_documented_example() {
        assert_eq!(delta(Duration::from_minutes(30)), "30 minutes");
    }

    /// The branches of the `naturaldelta` source the documentation shows,
    /// one span per branch, with `round` rounding half to even.
    #[test]
    fn naturaldelta_follows_every_branch_of_the_documented_source() {
        let seconds = |value: i128| delta(Duration::from_secs(value));
        let days = |value: i64| delta(Duration::from_days(value));
        assert_eq!(seconds(0), "a moment");
        assert_eq!(seconds(1), "a second");
        assert_eq!(seconds(59), "59 seconds");
        assert_eq!(seconds(60), "a minute");
        // round(1.5) is 2 and round(2.5) is 2: ties go to even.
        assert_eq!(seconds(90), "2 minutes");
        assert_eq!(seconds(150), "2 minutes");
        // round(59.5) is 60, which is "an hour".
        assert_eq!(seconds(3_570), "an hour");
        assert_eq!(seconds(3_600), "an hour");
        assert_eq!(seconds(5_400), "2 hours");
        // round(23.5) is 24, which is "a day".
        assert_eq!(seconds(84_600), "a day");
        assert_eq!(days(1), "a day");
        // round(15 / 30.5) is 0 and round(16 / 30.5) is 1.
        assert_eq!(days(15), "15 days");
        assert_eq!(days(16), "a month");
        assert_eq!(days(350), "11 months");
        assert_eq!(days(360), "a year");
        assert_eq!(days(365), "a year");
        assert_eq!(days(366), "1 year, 1 day");
        assert_eq!(days(400), "1 year, 1 month");
        assert_eq!(days(450), "1 year, 3 months");
        assert_eq!(days(725), "2 years");
        assert_eq!(days(1_095), "3 years");
        assert_eq!(days(365_000), "1,000 years");
        // The sign is dropped, as `abs(delta)` drops it.
        assert_eq!(delta(Duration::from_minutes(-30)), "30 minutes");
        let no_months = natural()
            .naturaldelta(
                Duration::from_days(45),
                DeltaOptions {
                    months: false,
                    minimum_unit: PreciseUnit::Seconds,
                },
            )
            .unwrap();
        assert_eq!(no_months, "45 days");
    }

    #[test]
    fn naturaldelta_reaches_below_a_second_only_when_asked() {
        assert_eq!(delta(Duration::from_micros(500)), "a moment");
        assert_eq!(
            with_minimum(Duration::from_micros(500), PreciseUnit::Microseconds),
            "500 microseconds"
        );
        assert_eq!(
            with_minimum(Duration::from_micros(1), PreciseUnit::Microseconds),
            "1 microsecond"
        );
        assert_eq!(
            with_minimum(Duration::from_micros(1_500), PreciseUnit::Microseconds),
            "1 millisecond"
        );
        assert_eq!(
            with_minimum(Duration::from_millis(500), PreciseUnit::Milliseconds),
            "500 milliseconds"
        );
        assert_eq!(
            natural().naturaldelta(
                Duration::SECOND,
                DeltaOptions {
                    months: true,
                    minimum_unit: PreciseUnit::Minutes,
                },
            ),
            Err(HumanizeError::Unsupported("a minimum unit above seconds"))
        );
    }

    /// The `naturaltime` source: `now` for a moment, `%s ago` for the past,
    /// `%s from now` for the future, and a `timedelta` means "that long ago".
    #[test]
    fn naturaltime_adds_the_tense_the_documented_source_adds() {
        let natural = natural();
        let options = DeltaOptions::default();
        let time = |span| natural.naturaltime_delta(span, options).unwrap();
        assert_eq!(time(Duration::ZERO), "now");
        assert_eq!(time(Duration::from_secs(30)), "30 seconds ago");
        assert_eq!(time(Duration::from_hours(-1)), "an hour from now");
        let now = CivilDateTime::new(Rd(739_880), CivilTime::hms(12, 0, 0).unwrap());
        let earlier = CivilDateTime::new(Rd(739_879), CivilTime::hms(12, 0, 0).unwrap());
        assert_eq!(
            natural.naturaltime(earlier, now, options).unwrap(),
            "a day ago"
        );
        assert_eq!(
            natural.naturaltime(now, earlier, options).unwrap(),
            "a day from now"
        );
    }

    /// `humanize`'s documentation, `precisedelta`:
    ///
    /// ```text
    /// >>> delta = dt.timedelta(seconds=3633, days=2, microseconds=123000)
    /// >>> precisedelta(delta)
    /// '2 days, 1 hour and 33.12 seconds'
    /// >>> precisedelta(delta, format="%0.4f")
    /// '2 days, 1 hour and 33.1230 seconds'
    /// >>> precisedelta(delta, minimum_unit="microseconds")
    /// '2 days, 1 hour, 33 seconds and 123 milliseconds'
    /// >>> precisedelta(delta, suppress=['days'])
    /// '49 hours and 33.12 seconds'
    /// >>> delta = dt.timedelta(seconds=90, microseconds=100)
    /// >>> precisedelta(delta, suppress=['seconds', 'milliseconds', 'microseconds'])
    /// '1.50 minutes'
    /// >>> delta = dt.timedelta(seconds=1)
    /// >>> precisedelta(delta, minimum_unit="minutes")
    /// '0.02 minutes'
    /// >>> delta = dt.timedelta(seconds=0.1)
    /// >>> precisedelta(delta, minimum_unit="minutes")
    /// '0 minutes'
    /// ```
    #[test]
    fn precisedelta_matches_the_documented_examples() {
        use PreciseUnit::{Days, Microseconds, Milliseconds, Minutes, Seconds};
        let natural = natural();
        let precise = |span, minimum, suppress: &[PreciseUnit], decimals| {
            natural
                .precisedelta(span, minimum, suppress, decimals)
                .unwrap()
        };
        let span =
            Duration::from_days(2) + Duration::from_secs(3_633) + Duration::from_micros(123_000);
        assert_eq!(
            precise(span, Seconds, &[], 2),
            "2 days, 1 hour and 33.12 seconds"
        );
        assert_eq!(
            precise(span, Seconds, &[], 4),
            "2 days, 1 hour and 33.1230 seconds"
        );
        assert_eq!(
            precise(span, Microseconds, &[], 2),
            "2 days, 1 hour, 33 seconds and 123 milliseconds"
        );
        assert_eq!(
            precise(span, Seconds, &[Days], 2),
            "49 hours and 33.12 seconds"
        );
        let ninety = Duration::from_secs(90) + Duration::from_micros(100);
        assert_eq!(
            precise(ninety, Seconds, &[Seconds, Milliseconds, Microseconds], 2),
            "1.50 minutes"
        );
        assert_eq!(precise(Duration::SECOND, Minutes, &[], 2), "0.02 minutes");
        assert_eq!(
            precise(Duration::from_millis(100), Minutes, &[], 2),
            "0 minutes"
        );
    }

    #[test]
    fn precisedelta_takes_months_from_whole_days_and_refuses_an_impossible_minimum() {
        use PreciseUnit::{Days, Months, Seconds, Years};
        let natural = natural();
        // 31 days is a month and half a day: the month is taken from whole
        // days, and what is left is half of one.
        assert_eq!(
            natural
                .precisedelta(Duration::from_days(31), Days, &[], 2)
                .unwrap(),
            "1 month and 0.50 days"
        );
        // The fraction of a month counts whole days only, as `humanize`
        // divides `days / 30.5`: 45 days and 12 hours is 45 / 30.5 months.
        let span = Duration::from_days(45) + Duration::from_hours(12);
        assert_eq!(
            natural.precisedelta(span, Months, &[], 2).unwrap(),
            "1.48 months"
        );
        assert_eq!(
            natural
                .precisedelta(Duration::from_days(400), Seconds, &[], 2)
                .unwrap(),
            // 400 days: `divmod(400, 365)` is (1, 35), `divmod(35, 30.5)` is
            // (1, 4.5), and 4.5 days is 4 days and 12 hours.
            "1 year, 1 month, 4 days and 12 hours"
        );
        assert_eq!(
            natural
                .precisedelta(Duration::from_days(730), Years, &[], 2)
                .unwrap(),
            "2.0 years"
        );
        assert_eq!(
            natural
                .precisedelta(Duration::ZERO, Seconds, &[], 2)
                .unwrap(),
            "0 seconds"
        );
        assert!(matches!(
            natural.precisedelta(Duration::SECOND, Years, &[Years], 2),
            Err(HumanizeError::Unsupported(_))
        ));
    }

    #[test]
    fn naturalday_and_naturaldate_name_the_near_days_and_write_the_rest() {
        let natural = natural();
        let today = Rd(739_880); // 2026-09-21
        assert_eq!(natural.naturalday(today, today, "%b %d").unwrap(), "today");
        assert_eq!(
            natural.naturalday(today + 1, today, "%b %d").unwrap(),
            "tomorrow"
        );
        assert_eq!(
            natural.naturalday(today - 1, today, "%b %d").unwrap(),
            "yesterday"
        );
        assert_eq!(
            natural.naturalday(today + 9, today, "%b %d").unwrap(),
            "Sep 30"
        );
        assert_eq!(
            natural.naturalday(today + 9, today, "%Y-%m-%d").unwrap(),
            "2026-09-30"
        );
        // 5 × 365 / 12 is 152.08 days: 152 days away keeps the short form and
        // 153 gains the year.
        assert_eq!(natural.naturaldate(today + 152, today).unwrap(), "Feb 20");
        assert_eq!(
            natural.naturaldate(today + 153, today).unwrap(),
            "Feb 21 2027"
        );
        assert_eq!(
            natural.naturaldate(today - 200, today).unwrap(),
            "Mar 05 2026"
        );
        assert_eq!(natural.naturaldate(today, today).unwrap(), "today");
    }

    /// `humanize`'s documentation, `ordinal`:
    ///
    /// ```text
    /// >>> ordinal(1)
    /// '1st'
    /// >>> ordinal(1002)
    /// '1002nd'
    /// >>> ordinal(103)
    /// '103rd'
    /// >>> ordinal(4)
    /// '4th'
    /// >>> ordinal(12)
    /// '12th'
    /// >>> ordinal(101)
    /// '101st'
    /// >>> ordinal(111)
    /// '111th'
    /// ```
    #[test]
    fn ordinal_matches_the_documented_examples() {
        let natural = natural();
        for (value, expected) in [
            (1, "1st"),
            (1_002, "1002nd"),
            (103, "103rd"),
            (4, "4th"),
            (12, "12th"),
            (101, "101st"),
            (111, "111th"),
        ] {
            assert_eq!(natural.ordinal(value).unwrap(), expected);
        }
    }

    /// `humanize`'s documentation, `intcomma`:
    ///
    /// ```text
    /// >>> intcomma(100)
    /// '100'
    /// >>> intcomma("1000")
    /// '1,000'
    /// >>> intcomma(1_000_000)
    /// '1,000,000'
    /// >>> intcomma(1_234_567.25)
    /// '1,234,567.25'
    /// >>> intcomma(1234.5454545, 2)
    /// '1,234.55'
    /// >>> intcomma(14308.40, 1)
    /// '14,308.4'
    /// ```
    #[test]
    fn intcomma_matches_the_documented_examples() {
        let natural = natural();
        let english = Grouping::ENGLISH;
        assert_eq!(natural.intcomma(100, english).unwrap(), "100");
        assert_eq!(natural.intcomma(1_000, english).unwrap(), "1,000");
        assert_eq!(natural.intcomma(1_000_000, english).unwrap(), "1,000,000");
        assert_eq!(natural.intcomma(-1_234_567, english).unwrap(), "-1,234,567");
        assert_eq!(
            natural.intcomma_f64(1_234_567.25, None, english).unwrap(),
            "1,234,567.25"
        );
        assert_eq!(
            natural
                .intcomma_f64(1_234.545_454_5, Some(2), english)
                .unwrap(),
            "1,234.55"
        );
        assert_eq!(
            natural.intcomma_f64(14_308.40, Some(1), english).unwrap(),
            "14,308.4"
        );
        // Python's `repr` writes `.0` on a whole float and an exponent from
        // 10^16 up.
        assert_eq!(
            natural.intcomma_f64(1_000.0, None, english).unwrap(),
            "1,000.0"
        );
        assert_eq!(natural.intcomma_f64(1e16, None, english).unwrap(), "1e+16");
        assert_eq!(
            natural.intcomma_f64(f64::NAN, None, english).unwrap(),
            "NaN"
        );
        let german = Grouping {
            thousands: ".",
            decimal: ",",
        };
        assert_eq!(
            natural.intcomma_f64(1_234_567.25, None, german).unwrap(),
            "1.234.567,25"
        );
    }

    /// `humanize`'s documentation, `intword`:
    ///
    /// ```text
    /// >>> intword("100")
    /// '100'
    /// >>> intword("12400")
    /// '12.4 thousand'
    /// >>> intword("1000000")
    /// '1.0 million'
    /// >>> intword(1_200_000_000)
    /// '1.2 billion'
    /// >>> intword(8100000000000000000000000000000000)
    /// '8.1 decillion'
    /// >>> intword("1234000", "%0.3f")
    /// '1.234 million'
    /// ```
    #[test]
    fn intword_matches_the_documented_examples() {
        let natural = natural();
        assert_eq!(natural.intword(100, 1).unwrap(), "100");
        assert_eq!(natural.intword(12_400, 1).unwrap(), "12.4 thousand");
        assert_eq!(natural.intword(1_000_000, 1).unwrap(), "1.0 million");
        assert_eq!(natural.intword(1_200_000_000, 1).unwrap(), "1.2 billion");
        assert_eq!(
            natural
                .intword(8_100_000_000_000_000_000_000_000_000_000_000, 1)
                .unwrap(),
            "8.1 decillion"
        );
        assert_eq!(natural.intword(1_234_000, 3).unwrap(), "1.234 million");
        // Rounding that reaches the next power moves to it.
        assert_eq!(natural.intword(999_999, 1).unwrap(), "1.0 million");
        assert_eq!(natural.intword(-12_400, 1).unwrap(), "-12.4 thousand");
        assert_eq!(natural.intword(i128::MAX, 1).unwrap(), "170141.2 decillion");
    }
}
