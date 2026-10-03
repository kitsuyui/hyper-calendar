//! The functions of Python's `humanize` package, with its thresholds, its
//! arithmetic and its phrasing.
//!
//! The system is written up in `docs/systems/natural-time.md`.
//!
//! The rest of this crate phrases time the way CLDR does. This module
//! phrases it the way the Python `humanize` package does — *a moment*,
//! *an hour*, *1 year, 3 months*, *2 days, 1 hour and 33.12 seconds*, *1.2
//! billion*, *2.9 KiB* — for a caller porting code that already depends on
//! those exact strings. The two are different conventions and neither is
//! wrong, so they are separate (policy §5): [`crate::RelativeTimeFormatter`]
//! is the CLDR one.
//!
//! The behaviour is that of `humanize` 4.16.0, the release of 2026-06-30,
//! whose sources (`time.py`, `number.py`, `filesize.py`, `lists.py`,
//! `i18n.py`) and catalogues were read at the tag `4.16.0` of
//! <https://github.com/python-humanize/humanize> on 2026-10-03, and whose
//! documentation (<https://humanize.readthedocs.io/en/latest/time/> and
//! `/number/`) was read the same day. `humanize` is MIT-licensed; nothing is
//! copied from it but the thresholds, the arithmetic and the strings, which
//! are the behaviour being reproduced. The system document
//! `docs/systems/python-compatibility.md` explains the arithmetic and gives
//! the measured agreement; `docs/python-parity.md` lists what the project's
//! later, unreleased code does differently.
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
//! * **A language is a value.** [`NaturalPhrases::ENGLISH`] is `humanize`'s
//!   source strings, and [`NaturalPhrases::by_catalogue`] the 35 gettext
//!   catalogues of the release, by the names `humanize.i18n.activate` takes.
//!   The language is the [`Natural`] you hold, not a process-wide setting
//!   (policy §13), and a plural is chosen by the catalogue's own expression
//!   ([`gettext`]), never by comparing a count to one.
//! * **`intcomma` separators** are a [`Grouping`] the caller passes;
//!   [`NaturalPhrases::grouping`] is the language's own, from `i18n.py`.
//! * **Integers of any length.** Python's `intword` reaches the googol
//!   because its integers have no end; [`Natural::write_intword_digits`]
//!   takes the digits.

use core::fmt;
use core::fmt::Write as _;

use hc_calendar::CivilDateTime;
#[cfg(any(test, feature = "format"))]
use hc_calendar::Rd;
use hc_core::Duration;
#[cfg(feature = "format")]
use hc_format::patterns::{FormatContext, strftime};
use hc_i18n::Locale;

mod catalogues;
pub mod gettext;
mod numbers;

pub use numbers::{ClampFormat, SizeStyle};

use crate::error::{HumanizeError, HumanizeResult};

const MICROS_PER_SECOND: i128 = 1_000_000;
const MICROS_PER_DAY: i128 = 86_400 * MICROS_PER_SECOND;

/// A phrase `humanize` chooses by count: the English singular and plural of
/// its source, each holding `{0}` where the number goes, and the forms a
/// catalogue translates them to.
///
/// Python's `ngettext(singular, plural, n)` looks `n`'s form up in the
/// catalogue (see [`gettext`]) and, for a message the catalogue does not
/// translate, answers the singular for 1 and the plural for any other count.
/// An untranslated message has no `forms`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plural {
    /// The English singular, the message's `msgid`.
    pub one: &'static str,
    /// The English plural, its `msgid_plural`.
    pub other: &'static str,
    /// The catalogue's `msgstr[0]`, `msgstr[1]`, ...; empty when it has none.
    pub forms: &'static [&'static str],
}

impl Plural {
    /// A message with no translation: the English pair.
    #[must_use]
    pub const fn english(one: &'static str, other: &'static str) -> Self {
        Self {
            one,
            other,
            forms: &[],
        }
    }

    /// A message a catalogue translates to `forms`.
    #[must_use]
    pub const fn translated(
        one: &'static str,
        other: &'static str,
        forms: &'static [&'static str],
    ) -> Self {
        Self { one, other, forms }
    }
}

/// Every string the `humanize` functions produce, in one language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct NaturalPhrases {
    /// The catalogue's name, as `humanize.i18n.activate` takes it: `en` for
    /// the English of the source, `ru_RU` for the Russian catalogue.
    pub catalogue: &'static str,
    /// The language the catalogue says it translates to (its header's
    /// `Language`), `en`, `ru`.
    pub language: &'static str,
    /// The catalogue's `plural=` expression, which chooses a form by count
    /// ([`gettext::plural_index`]); `n != 1` for the English of the source.
    pub plural_expression: &'static str,
    /// The separators `humanize` writes numbers with in this language, which
    /// its `i18n` module holds apart from the catalogue: `,` and `.`
    /// everywhere but German, French, Italian, Brazilian Portuguese,
    /// Hungarian and Latvian.
    pub grouping: Grouping,
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
    pub one_year_days: Plural,
    /// `1 year, {0} month(s)`.
    pub one_year_months: Plural,
    /// `{0} microsecond(s)`.
    pub microseconds: Plural,
    /// `{0} millisecond(s)`.
    pub milliseconds: Plural,
    /// `{0} second(s)`.
    pub seconds: Plural,
    /// `{0} minute(s)`.
    pub minutes: Plural,
    /// `{0} hour(s)`.
    pub hours: Plural,
    /// `{0} day(s)`.
    pub days: Plural,
    /// `{0} month(s)`.
    pub months: Plural,
    /// `{0} year(s)`.
    pub years: Plural,
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
    /// The masculine ordinal suffix by last digit, 0 through 9: `th`, `st`,
    /// `nd`, `rd`, `th`, ... Index 0, the suffix of 10, is also the one of
    /// 11, 12 and 13.
    pub ordinal_by_last_digit: [&'static str; 10],
    /// The feminine ordinal suffix by last digit, as `ordinal(value,
    /// gender="female")` writes it; English has the same ones.
    pub ordinal_by_last_digit_female: [&'static str; 10],
    /// The names of the powers `intword` knows, from `thousand` (10³) to
    /// `decillion` (10³³) and the `googol` (10¹⁰⁰), as singular and plural.
    pub powers: [Plural; 12],
    /// `zero` to `nine`, which `apnumber` spells out.
    pub apnumber: [&'static str; 10],
    /// `naturalsize` of exactly one byte: `{0} Byte`.
    pub byte: &'static str,
    /// `naturalsize` below the base: `{0} Bytes`.
    pub bytes: &'static str,
    /// The decimal suffixes of `naturalsize`, `kB` to `QB`.
    pub size_decimal: [&'static str; 10],
    /// The binary suffixes of `naturalsize`, `KiB` to `QiB`.
    pub size_binary: [&'static str; 10],
    /// The one-letter suffixes of `naturalsize`'s GNU style, `K` to `Q`.
    pub size_gnu: [&'static str; 10],
    /// Which groups of messages the catalogue really translates, so that a
    /// locale is served only by a catalogue that translates every message
    /// of the function asked ([`NaturalPhrases::for_locale`]).
    pub translated: Translated,
}

/// Which groups of messages of a catalogue are translated: every message
/// of the group has a translation in the `.po` file (not fuzzy, not empty)
/// that holds no raw `gettext` placeholder (`%d`, `%(value)s`), which a
/// few catalogues carry where the crate writes the number itself.
///
/// A message a catalogue does not translate is the English of the source
/// in [`NaturalPhrases`], as it is in Python, so these flags are the only
/// way to tell it from a translation that happens to be the English word.
/// They are generated with the catalogues by `scripts/humanize-gettext.py`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "one flag for each group of messages a function needs"
)]
pub struct Translated {
    /// `a moment`, `a second`, `an hour`, `1 year, 1 month`, `1 year, {0}
    /// days` and the other words `naturaldelta` writes for a count of one
    /// or a year and a part.
    pub articles: bool,
    /// `{0} seconds`, `{0} minutes`, `{0} hours`, `{0} days`, `{0} months`
    /// and `{0} years`.
    pub units: bool,
    /// `{0} milliseconds` and `{0} microseconds`, which a minimum unit
    /// below the second brings in.
    pub fine_units: bool,
    /// `now`, `{0} ago` and `{0} from now`.
    pub moments: bool,
    /// `{0} and {1}`, which joins the last two units of `precisedelta`.
    pub list_last: bool,
    /// The ordinal suffixes of both genders.
    pub ordinals: bool,
    /// `zero` to `nine`.
    pub apnumber: bool,
    /// `thousand` to `googol`.
    pub powers: bool,
    /// `today`, `tomorrow` and `yesterday`.
    pub days: bool,
    /// `Byte`, `Bytes` and the decimal and binary suffixes.
    pub sizes: bool,
}

impl Translated {
    /// Every group: the English of the source, which is no translation of
    /// anything and so needs none.
    pub const ALL: Self = Self {
        articles: true,
        units: true,
        fine_units: true,
        moments: true,
        list_last: true,
        ordinals: true,
        apnumber: true,
        powers: true,
        days: true,
        sizes: true,
    };
}

impl NaturalPhrases {
    /// The decimal exponents of [`NaturalPhrases::powers`]: 3, 6, ..., 33
    /// and 100.
    pub const POWER_EXPONENTS: [u8; 12] = [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 100];

    /// The catalogue `humanize.i18n.activate(name)` loads: one of the
    /// translations `humanize` 4.16.0 ships, by the name of its directory,
    /// `ru_RU`, `pt_BR`, `lv` (a hyphen for the underscore is accepted), or
    /// the English of the source for any name that starts with `en`, as
    /// `activate` takes it.
    ///
    /// Python does not look a language up: `activate("ru")` is a
    /// `FileNotFoundError`, since no directory is called `ru`, and so this
    /// is `None` for it; [`NaturalPhrases::by_language`] is the lookup that
    /// finds `ru_RU` for it.
    ///
    /// ```
    /// use hc_core::Duration;
    /// use hc_humanize::natural::{DeltaOptions, Natural, NaturalPhrases};
    ///
    /// let russian = Natural::with_phrases(NaturalPhrases::by_catalogue("ru_RU").unwrap());
    /// let span = Duration::from_secs(3);
    /// assert_eq!(russian.naturaltime_delta(span, DeltaOptions::default())?, "3 секунды назад");
    /// # Ok::<(), hc_humanize::HumanizeError>(())
    /// ```
    #[must_use]
    pub fn by_catalogue(name: &str) -> Option<&'static Self> {
        if name.starts_with("en") {
            return Some(&Self::ENGLISH);
        }
        catalogues::ALL.into_iter().find(|phrases| {
            phrases.catalogue.eq_ignore_ascii_case(name)
                || phrases.language.eq_ignore_ascii_case(name)
        })
    }

    /// The catalogue of a language, by a BCP 47 tag, when exactly one of the
    /// shipped ones is for it: `ru` and `ru-RU` give `ru_RU`, and `pt-BR`
    /// gives `pt_BR`, but `pt` and `zh`, which have two, give none, and a
    /// language with none gives none.
    #[must_use]
    pub fn by_language(tag: &str) -> Option<&'static Self> {
        if let Some(exact) = Self::by_catalogue(tag) {
            return Some(exact);
        }
        let primary = tag.split(['-', '_']).next().unwrap_or(tag);
        let mut found = catalogues::ALL.into_iter().filter(|phrases| {
            phrases
                .language
                .split('-')
                .next()
                .is_some_and(|language| language.eq_ignore_ascii_case(primary))
        });
        let first = found.next()?;
        found.next().is_none().then_some(first)
    }

    /// The catalogue that serves a locale for the words of one function:
    /// the first step of the locale's fallback chain ([`Locale::fallback`],
    /// `pt-AO` → `pt-PT` → `pt` → root) that a catalogue is for *and* that
    /// translates those words, else the English of the source.
    ///
    /// A catalogue is for a step when it has the step's language and, if the
    /// step names a region, the same region; a step with no region but a
    /// script takes the catalogue whose own likely script is that one (`zh-Hant`
    /// takes `zh_HK`, the only Traditional one, as `zh-Hans` takes `zh_CN`); and a step that is a bare
    /// language takes the catalogue only when exactly one is for it, so `pt`,
    /// which has two, picks neither and no region is guessed. `nb` stands for
    /// `no`, the parent CLDR gives `nb`.
    ///
    /// A catalogue the `humanize` project has only part translated holds the
    /// English of the source in the messages it lacks, and some hold a raw
    /// `%d` or `%(value)s` in a word that takes no number. A catalogue is
    /// served only if it translates *every* message the function can write
    /// ([`NaturalPhrases::translates`]); one that lacks any is passed over,
    /// and where no step has a complete catalogue the answer is the English
    /// of the source whole, so a result is never written half in one
    /// language and half in another. The answer's
    /// [`NaturalPhrases::language`] says which language it is.
    ///
    /// ```
    /// use hc_humanize::natural::{NaturalPhrases, NaturalWords};
    /// use hc_i18n::Locale;
    ///
    /// let chain = |tag: &str| {
    ///     NaturalPhrases::for_locale(&Locale::parse(tag).unwrap(), NaturalWords::Apnumber).language
    /// };
    /// assert_eq!(chain("ru"), "ru-RU");
    /// assert_eq!(chain("pt-AO"), "pt-PT");
    /// assert_eq!(chain("zh-Hant-TW"), "zh-HK");
    /// assert_eq!(chain("pt"), "en");
    /// ```
    #[must_use]
    pub fn for_locale(locale: &Locale, words: NaturalWords) -> &'static Self {
        for step in locale.fallback() {
            if step.language() == "en" {
                return &Self::ENGLISH;
            }
            let mut found = catalogues::ALL
                .into_iter()
                .filter(|phrases| phrases.serves(&step));
            // A bare language is served only when exactly one catalogue is
            // for it, whether or not that one translates the words: `pt`
            // has two, and picking the one that happens to be complete
            // would guess a region.
            if let Some(first) = found.next()
                && (step.region().is_some() || step.script().is_some() || found.next().is_none())
                && first.translates(words)
            {
                return first;
            }
        }
        &Self::ENGLISH
    }

    /// Whether this catalogue is for a step of a fallback chain; see
    /// [`NaturalPhrases::for_locale`].
    fn serves(&self, step: &Locale) -> bool {
        let Ok(own) = Locale::parse(self.language) else {
            return false;
        };
        let language = if step.language() == "no" {
            "nb"
        } else {
            step.language()
        };
        if own.language() != language {
            return false;
        }
        if let Some(region) = step.region() {
            return own.region() == Some(region);
        }
        if let Some(script) = step.script() {
            return own.with_likely_script().script() == Some(script);
        }
        true
    }

    /// Whether this catalogue translates every message the function of
    /// `words` can write: all of them, so that its result is in one
    /// language. A message the catalogue leaves untranslated, or translates
    /// to a raw placeholder, makes it fail, however many of the others it
    /// does translate. The English of the source translates all of its own.
    #[must_use]
    pub const fn translates(&self, words: NaturalWords) -> bool {
        let t = &self.translated;
        match words {
            NaturalWords::Any => true,
            NaturalWords::Apnumber => t.apnumber,
            NaturalWords::Intword => t.powers,
            NaturalWords::Delta => t.articles && t.units,
            NaturalWords::DeltaFine => t.articles && t.units && t.fine_units,
            NaturalWords::Time => t.articles && t.units && t.moments,
            NaturalWords::TimeFine => t.articles && t.units && t.moments && t.fine_units,
            NaturalWords::Precise => t.units && t.list_last,
            NaturalWords::PreciseFine => t.units && t.list_last && t.fine_units,
            NaturalWords::Ordinal => t.ordinals,
            NaturalWords::Day => t.days,
            NaturalWords::Naturalsize => t.sizes,
        }
    }

    /// Every translation `humanize` 4.16.0 ships, in the order of its
    /// directories: 35 of them, each [`NaturalPhrases::catalogue`] named as
    /// `humanize.i18n.activate` takes it. The English of the source is
    /// [`NaturalPhrases::ENGLISH`], not in the list.
    #[must_use]
    pub fn catalogues() -> &'static [&'static Self] {
        &catalogues::ALL
    }
}

const fn same(word: &'static str) -> Plural {
    Plural::english(word, word)
}

impl NaturalPhrases {
    /// `humanize`'s own strings: the English message identifiers its
    /// catalogues translate.
    pub const ENGLISH: Self = Self {
        catalogue: "en",
        language: "en",
        plural_expression: "n != 1",
        grouping: Grouping::ENGLISH,
        a_moment: "a moment",
        a_second: "a second",
        a_minute: "a minute",
        an_hour: "an hour",
        a_day: "a day",
        a_month: "a month",
        a_year: "a year",
        one_year_one_month: "1 year, 1 month",
        one_year_days: Plural::english("1 year, {0} day", "1 year, {0} days"),
        one_year_months: Plural::english("1 year, {0} month", "1 year, {0} months"),
        microseconds: Plural::english("{0} microsecond", "{0} microseconds"),
        milliseconds: Plural::english("{0} millisecond", "{0} milliseconds"),
        seconds: Plural::english("{0} second", "{0} seconds"),
        minutes: Plural::english("{0} minute", "{0} minutes"),
        hours: Plural::english("{0} hour", "{0} hours"),
        days: Plural::english("{0} day", "{0} days"),
        months: Plural::english("{0} month", "{0} months"),
        years: Plural::english("{0} year", "{0} years"),
        now: "now",
        ago: "{0} ago",
        from_now: "{0} from now",
        today: "today",
        tomorrow: "tomorrow",
        yesterday: "yesterday",
        list_separator: ", ",
        list_last: "{0} and {1}",
        ordinal_by_last_digit: ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"],
        ordinal_by_last_digit_female: ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"],
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
            same("googol"),
        ],
        apnumber: [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ],
        byte: "{0} Byte",
        bytes: "{0} Bytes",
        size_decimal: ["kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB", "RB", "QB"],
        size_binary: [
            "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB", "RiB", "QiB",
        ],
        size_gnu: ["K", "M", "G", "T", "P", "E", "Z", "Y", "R", "Q"],
        translated: Translated::ALL,
    };
}

/// The words of one `humanize` function, for choosing the catalogue that
/// serves a locale ([`NaturalPhrases::for_locale`]): the catalogue must
/// translate every one of them ([`NaturalPhrases::translates`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NaturalWords {
    /// A function that writes no word of its own, `metric`: the first
    /// catalogue for the locale serves.
    Any,
    /// `apnumber`'s *zero* to *nine*.
    Apnumber,
    /// `intword`'s *thousand* to *googol*.
    Intword,
    /// `naturaldelta` of a span the unit of seconds or above reaches: *a
    /// moment*, *an hour*, *3 days*, *1 year, 3 months*.
    Delta,
    /// [`NaturalWords::Delta`] with a minimum unit below the second, which
    /// brings *milliseconds* and *microseconds* in.
    DeltaFine,
    /// `naturaltime`: [`NaturalWords::Delta`]'s words with *now*, *ago* and
    /// *from now*.
    Time,
    /// [`NaturalWords::Time`] with a minimum unit below the second.
    TimeFine,
    /// `precisedelta`'s *seconds* to *years* and the *and* before its last
    /// unit.
    Precise,
    /// [`NaturalWords::Precise`] with a minimum unit below the second.
    PreciseFine,
    /// `ordinal`'s suffixes.
    Ordinal,
    /// `naturalday`'s *today*, *tomorrow* and *yesterday*.
    Day,
    /// `naturalsize`'s *Byte* and the suffixes.
    Naturalsize,
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
    /// Every unit, smallest first, with the identifier a caller names it by.
    pub const ALL: [(&'static str, Self); 8] = [
        ("microseconds", Self::Microseconds),
        ("milliseconds", Self::Milliseconds),
        ("seconds", Self::Seconds),
        ("minutes", Self::Minutes),
        ("hours", Self::Hours),
        ("days", Self::Days),
        ("months", Self::Months),
        ("years", Self::Years),
    ];

    /// The unit an identifier names, `microseconds` to `years`, in any
    /// case; `None` for other text.
    #[must_use]
    pub fn by_id(id: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|(name, _)| hc_core::catalogue::matches(id, name))
            .map(|(_, unit)| unit)
    }

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

    const fn phrases(self, phrases: &NaturalPhrases) -> Plural {
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

/// The grammatical gender `ordinal` takes: `humanize`'s `male` and `female`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Gender {
    /// `gender="male"`, the default.
    #[default]
    Male,
    /// `gender="female"`.
    Female,
}

impl Gender {
    /// The gender an identifier names, `male` or `female`, in any case;
    /// `None` for other text.
    #[must_use]
    pub fn by_id(id: &str) -> Option<Self> {
        if hc_core::catalogue::matches(id, "male") {
            Some(Self::Male)
        } else if hc_core::catalogue::matches(id, "female") {
            Some(Self::Female)
        } else {
            None
        }
    }
}

/// A phrase before its number is written.
#[derive(Debug, Clone, Copy)]
enum Phrase {
    Fixed(&'static str),
    Count(Plural, i128),
    Grouped(Plural, i128),
}

/// The text around and between the two halves of `%s and %s` once its
/// placeholders are `{0}` and `{1}`.
fn list_parts(pattern: &str) -> (&str, &str, &str) {
    match pattern.split_once("{0}") {
        Some((before, rest)) => match rest.split_once("{1}") {
            Some((middle, after)) => (before, middle, after),
            None => (before, " and ", ""),
        },
        None => ("", " and ", ""),
    }
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
}

impl Natural {
    /// `humanize`'s English.
    #[must_use]
    pub fn english() -> Self {
        Self::with_phrases(&NaturalPhrases::ENGLISH)
    }

    /// Another vocabulary, which chooses its forms by its own catalogue's
    /// plural expression.
    #[must_use]
    pub const fn with_phrases(phrases: &'static NaturalPhrases) -> Self {
        Self { phrases }
    }

    /// The vocabulary of a catalogue, by [`NaturalPhrases::by_catalogue`].
    #[must_use]
    pub fn for_catalogue(name: &str) -> Option<Self> {
        NaturalPhrases::by_catalogue(name).map(Self::with_phrases)
    }

    /// The vocabulary of a language, by [`NaturalPhrases::by_language`].
    #[must_use]
    pub fn for_language(tag: &str) -> Option<Self> {
        NaturalPhrases::by_language(tag).map(Self::with_phrases)
    }

    /// The vocabulary in use.
    #[must_use]
    pub const fn phrases(&self) -> &'static NaturalPhrases {
        self.phrases
    }

    /// Python's `ngettext` over the catalogue: the translated form its
    /// expression selects for `count`, or, for a message with none or a
    /// form the catalogue lacks, the English singular for 1 and the plural
    /// for any other count.
    fn pick(&self, message: Plural, count: i128) -> &'static str {
        let count = i64::try_from(count).unwrap_or(i64::MAX);
        gettext::plural_index(self.phrases.plural_expression, count)
            .and_then(|index| message.forms.get(index).copied())
            .unwrap_or(if count == 1 {
                message.one
            } else {
                message.other
            })
    }

    /// [`Natural::pick`] for a count written in decimal digits.
    fn pick_digits(&self, message: Plural, digits: &str) -> &'static str {
        gettext::plural_index_digits(self.phrases.plural_expression, digits)
            .and_then(|index| message.forms.get(index).copied())
            .unwrap_or(if digits.trim_start_matches('0') == "1" {
                message.one
            } else {
                message.other
            })
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
        // `humanize` 4.16.0 counts whole years, `delta.days // 365`; its later,
        // unreleased code rounds `delta.days / 365`.
        Ok(Phrase::Grouped(phrases.years, days / 365))
    }

    fn write_phrase<W: fmt::Write>(&self, out: &mut W, phrase: Phrase) -> HumanizeResult<()> {
        match phrase {
            Phrase::Fixed(text) => out.write_str(text)?,
            Phrase::Count(pair, count) => {
                substitute(out, self.pick(pair, count), |out| write!(out, "{count}"))?;
            }
            Phrase::Grouped(pair, count) => {
                substitute(out, self.pick(pair, count), |out| {
                    write_grouped(out, count, self.phrases.grouping.thousands)
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
    /// The arithmetic is `humanize` 4.16's, step for step, so that its
    /// quirks are reproduced rather than corrected:
    ///
    /// * the fraction of a year is `days / 365` and of a month `days / 30.5`,
    ///   taken from whole days, and of a day, hour or minute from whole
    ///   seconds;
    /// * the remainder left after a division is truncated to a whole number
    ///   (`int(r)`), so the half day left by a 30.5-day month is lost, where
    ///   exact arithmetic would write it as 12 hours;
    /// * the minimum unit's value is rounded with the format *before* it is
    ///   compared with zero, so two days and one microsecond read `2 days`;
    /// * a unit that rounding pushes to its next unit's size is carried:
    ///   59.999 999 seconds at `minimum_unit` seconds is `1 minute`. The
    ///   carry from days to months subtracts 31, as `humanize` does.
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
        let split = |value: Num, divisor: Divisor, unit: PreciseUnit| {
            quotient_and_remainder(value, divisor, unit, minimum, &suppressed, decimals)
        };

        let total = abs_micros(delta)?;
        let days = total / MICROS_PER_DAY;
        let seconds = total % MICROS_PER_DAY / MICROS_PER_SECOND;
        let micros = total % MICROS_PER_SECOND;

        let (years, days) = split(Num::Int(days), Divisor::Whole(365), PreciseUnit::Years);
        let (months, days) = split(days, Divisor::Half(61), PreciseUnit::Months);
        let seconds = days.int() * 86_400 + seconds;
        let (days, seconds) = split(Num::Int(seconds), Divisor::Whole(86_400), PreciseUnit::Days);
        let (hours, seconds) = split(seconds, Divisor::Whole(3_600), PreciseUnit::Hours);
        let (minutes, seconds) = split(seconds, Divisor::Whole(60), PreciseUnit::Minutes);
        // From here Python holds the count in a float: `secs * 1e6 + usecs`.
        let micros = Num::Float(seconds.int() as f64 * 1e6 + micros as f64);
        let (seconds, micros) = split(micros, Divisor::Float(1e6), PreciseUnit::Seconds);
        let (millis, micros) = split(micros, Divisor::Whole(1_000), PreciseUnit::Milliseconds);

        // Rounding can leave a unit the size of the next one up: carry it.
        let (mut years, mut months, mut days, mut hours) = (years, months, days, hours);
        let (mut minutes, mut seconds, mut millis) = (minutes, seconds, millis);
        let micros = micros.count();
        let carry = |small: &mut Count, big: &mut Count, size: i128, big_unit: PreciseUnit| {
            if small.at_least(size) && !suppressed(big_unit) {
                *small = small.minus(size);
                *big = big.plus(1);
            }
        };
        carry(&mut millis, &mut seconds, 1_000, PreciseUnit::Seconds);
        carry(&mut seconds, &mut minutes, 60, PreciseUnit::Minutes);
        carry(&mut minutes, &mut hours, 60, PreciseUnit::Hours);
        carry(&mut hours, &mut days, 24, PreciseUnit::Days);
        carry(&mut days, &mut months, 31, PreciseUnit::Months);
        carry(&mut months, &mut years, 12, PreciseUnit::Years);

        let values = [years, months, days, hours, minutes, seconds, millis, micros];
        // The units written: non-zero ones above the minimum, and the
        // minimum itself when it is non-zero or nothing else is written
        // (`0 minutes`). The loop stops at the minimum.
        let mut written = [None::<(PreciseUnit, Count)>; 8];
        let mut total_items = 0usize;
        for (unit, value) in PreciseUnit::DESCENDING.into_iter().zip(values) {
            if (value.is_positive() || (total_items == 0 && unit == minimum))
                && let Some(slot) = written.get_mut(total_items)
            {
                *slot = Some((unit, value));
                total_items += 1;
            }
            if unit == minimum {
                break;
            }
        }
        // `_("%s and %s") % (head, tail)`: the catalogue's text around and
        // between the two halves, which Klingon puts a word after.
        let (before, middle, after) = list_parts(self.phrases.list_last);
        if total_items > 1 {
            out.write_str(before)?;
        }
        for (position, item) in written.into_iter().flatten().enumerate() {
            if position > 0 {
                out.write_str(if position + 1 == total_items {
                    middle
                } else {
                    self.phrases.list_separator
                })?;
            }
            self.write_item(out, item, decimals, minimum)?;
        }
        if total_items > 1 {
            out.write_str(after)?;
        }
        Ok(())
    }

    fn write_item<W: fmt::Write>(
        &self,
        out: &mut W,
        (unit, count): (PreciseUnit, Count),
        decimals: u8,
        minimum: PreciseUnit,
    ) -> HumanizeResult<()> {
        let pair = unit.phrases(self.phrases);
        // `2 if 1 < value < 2 else int(value)` picks the plural.
        let chooser = match count {
            Count::Real(value) if value > 1.0 && value < 2.0 => 2,
            other => other.truncated(),
        };
        let text = self.pick(pair, chooser);
        if unit == minimum
            && let Count::Real(value) = count
            && value - hc_core::math::trunc(value) > 0.0
        {
            let precision = usize::from(decimals);
            return substitute(out, text, |out| write!(out, "{value:.precision$}"));
        }
        if unit == PreciseUnit::Years {
            substitute(out, text, |out| {
                write_grouped(out, count.truncated(), self.phrases.grouping.thousands)
            })
        } else {
            substitute(out, text, |out| write!(out, "{}", count.truncated()))
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
    #[cfg(feature = "format")]
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
    #[cfg(feature = "format")]
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

    /// `ordinal(value)`: `1st`, `2nd`, `103rd`, `111th`, with the masculine
    /// suffixes, as Python's `gender="male"` default.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_ordinal<W: fmt::Write>(&self, out: &mut W, value: i128) -> HumanizeResult<()> {
        self.write_ordinal_of(out, value, Gender::Male)
    }

    /// `ordinal(value, gender)`: the suffix of the last digit in the gender,
    /// and of 11, 12 and 13 the suffix of 0, as `humanize` writes it.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_ordinal_of<W: fmt::Write>(
        &self,
        out: &mut W,
        value: i128,
        gender: Gender,
    ) -> HumanizeResult<()> {
        let table = match gender {
            Gender::Male => &self.phrases.ordinal_by_last_digit,
            Gender::Female => &self.phrases.ordinal_by_last_digit_female,
        };
        let digit = if matches!(value.rem_euclid(100), 11..=13) {
            0
        } else {
            value.rem_euclid(10) as usize
        };
        let suffix = table.get(digit).copied().unwrap_or("");
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
    /// `999 999` is `1.0 million`, not `1000.0 thousand`. An `i128` stops
    /// short of `humanize`'s last power, the googol (10¹⁰⁰);
    /// [`Natural::write_intword_digits`] takes an integer of any length.
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
        let mut buffer = DigitBuffer::default();
        write!(buffer, "{value}")?;
        self.write_intword_digits(out, buffer.as_str(), decimals)
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
    #[cfg(feature = "format")]
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
    #[cfg(feature = "format")]
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

/// What `humanize` divides a count of one unit by to get the next: a whole
/// number, `30.5` days as 61 half days, or `1e6`.
#[derive(Debug, Clone, Copy)]
enum Divisor {
    Whole(i128),
    Half(i128),
    Float(f64),
}

/// A value being divided: a Python `int`, or the `float` that
/// `secs * 1e6 + usecs` makes of the microseconds. The float is exact up to
/// 2⁵³ microseconds (about 285 years) and rounds beyond, as Python's does.
#[derive(Debug, Clone, Copy)]
enum Num {
    Int(i128),
    Float(f64),
}

impl Num {
    fn int(self) -> i128 {
        match self {
            Self::Int(value) => value,
            Self::Float(value) => hc_core::math::trunc(value) as i128,
        }
    }

    fn as_f64(self) -> f64 {
        match self {
            Self::Int(value) => value as f64,
            Self::Float(value) => value,
        }
    }

    fn count(self) -> Count {
        match self {
            Self::Int(value) => Count::Whole(value),
            Self::Float(value) => Count::Real(value),
        }
    }
}

/// A count as Python holds it in `precisedelta`: an `int`, or a `float` once
/// the minimum unit has been rounded with the format.
#[derive(Debug, Clone, Copy)]
enum Count {
    Whole(i128),
    Real(f64),
}

impl Count {
    fn is_positive(self) -> bool {
        match self {
            Self::Whole(value) => value > 0,
            Self::Real(value) => value > 0.0,
        }
    }

    fn at_least(self, size: i128) -> bool {
        match self {
            Self::Whole(value) => value >= size,
            Self::Real(value) => value >= size as f64,
        }
    }

    fn minus(self, size: i128) -> Self {
        match self {
            Self::Whole(value) => Self::Whole(value - size),
            Self::Real(value) => Self::Real(value - size as f64),
        }
    }

    fn plus(self, size: i128) -> Self {
        self.minus(-size)
    }

    /// Python's `int(value)`.
    fn truncated(self) -> i128 {
        match self {
            Self::Whole(value) => value,
            Self::Real(value) => hc_core::math::trunc(value) as i128,
        }
    }
}

/// `humanize`'s `_rounding_by_fmt`: `format % value`, read back as an `int`
/// when it parses as one and as a `float` otherwise.
fn round_by_format(value: f64, decimals: u8) -> Count {
    let mut buffer = DigitBuffer::default();
    let precision = usize::from(decimals);
    if write!(buffer, "{value:.precision$}").is_err() {
        return Count::Real(value);
    }
    let text = buffer.as_str();
    match text.parse::<i128>() {
        Ok(whole) => Count::Whole(whole),
        Err(_) => Count::Real(text.parse().unwrap_or(value)),
    }
}

/// `humanize`'s `_quotient_and_remainder`: the minimum unit takes the whole
/// value as a rounded fraction and leaves nothing; a suppressed unit takes
/// nothing; any other takes the quotient and leaves the remainder truncated
/// to a whole number (`int(r)`).
fn quotient_and_remainder(
    value: Num,
    divisor: Divisor,
    unit: PreciseUnit,
    minimum: PreciseUnit,
    suppressed: &impl Fn(PreciseUnit) -> bool,
    decimals: u8,
) -> (Count, Num) {
    let divisor_f64 = match divisor {
        Divisor::Whole(whole) => whole as f64,
        Divisor::Half(halves) => halves as f64 / 2.0,
        Divisor::Float(float) => float,
    };
    if unit == minimum {
        return (
            round_by_format(value.as_f64() / divisor_f64, decimals),
            Num::Int(0),
        );
    }
    if suppressed(unit) {
        return (Count::Whole(0), value);
    }
    match (value, divisor) {
        (Num::Int(value), Divisor::Whole(whole)) => {
            (Count::Whole(value / whole), Num::Int(value % whole))
        }
        // `divmod(value, 30.5)` of whole days: exact in halves.
        (Num::Int(value), Divisor::Half(halves)) => (
            Count::Whole(value * 2 / halves),
            Num::Int(value * 2 % halves / 2),
        ),
        // Python's float `divmod`, which takes `fmod` and rounds the
        // quotient to the nearest integer.
        (value, _) => {
            let x = value.as_f64();
            let remainder = x % divisor_f64;
            let quotient = (x - remainder) / divisor_f64;
            let floored = hc_core::math::floor(quotient);
            let quotient = if quotient - floored > 0.5 {
                floored + 1.0
            } else {
                floored
            };
            (
                Count::Real(quotient),
                Num::Int(hc_core::math::trunc(remainder) as i128),
            )
        }
    }
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
fn substitute<W: fmt::Write + ?Sized>(
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

/// Python's `repr` of a float: the shortest digits that read back as the
/// same double — the one nearest the value when several are as short, and
/// the even digit on an exact tie, as `repr` chooses — with `.0` on a whole
/// number, and exponent notation outside `[1e-4, 1e16)`.
fn write_python_repr<W: fmt::Write + ?Sized>(out: &mut W, value: f64) -> fmt::Result {
    if value.is_sign_negative() {
        out.write_char('-')?;
    }
    let magnitude = value.abs();
    if magnitude == 0.0 {
        return out.write_str("0.0");
    }
    // `{:.Ne}` rounds the exact value to N+1 digits, ties to even; the first
    // N that reads back is the shortest.
    let mut buffer = DigitBuffer::default();
    for precision in 0..=17usize {
        buffer = DigitBuffer::default();
        write!(buffer, "{magnitude:.precision$e}")?;
        if buffer.as_str().parse::<f64>() == Ok(magnitude) {
            break;
        }
    }
    let text = buffer.as_str();
    let (mantissa, exponent) = text.split_once('e').unwrap_or((text, "0"));
    let exponent: i32 = exponent.parse().unwrap_or(0);
    let mut digits = DigitBuffer::default();
    for digit in mantissa.chars().filter(char::is_ascii_digit) {
        digits.write_char(digit)?;
    }
    let digits = digits.as_str();
    let count = i32::try_from(digits.len()).unwrap_or(0);
    let point = exponent + 1;
    if (-3..=16).contains(&point) {
        if point <= 0 {
            out.write_str("0.")?;
            for _ in point..0 {
                out.write_char('0')?;
            }
            out.write_str(digits)
        } else if point >= count {
            out.write_str(digits)?;
            for _ in count..point {
                out.write_char('0')?;
            }
            out.write_str(".0")
        } else {
            let (whole, fraction) = digits.split_at(point as usize);
            write!(out, "{whole}.{fraction}")
        }
    } else {
        let (first, rest) = digits.split_at(1);
        out.write_str(first)?;
        if !rest.is_empty() {
            write!(out, ".{rest}")?;
        }
        write!(
            out,
            "e{}{:02}",
            if exponent < 0 { '-' } else { '+' },
            exponent.abs()
        )
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
        // Whole years, `delta.days // 365`, in `humanize` 4.16.0: 1000 days
        // are 2 years; its unreleased code rounds and says 3.
        assert_eq!(days(1_000), "2 years");
        assert_eq!(days(2_000), "5 years");
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

    /// `humanize` 4.16's `precisedelta` (`time.py`, read on the project's
    /// `main` 2026-10-03), traced by hand and checked against a transcription
    /// of that source: a month is `divmod(days, 30.5)` of whole days, and the
    /// remainder is truncated (`int(r)`).
    #[test]
    fn precisedelta_takes_months_from_whole_days_and_refuses_an_impossible_minimum() {
        use PreciseUnit::{Days, Months, Seconds, Years};
        let natural = natural();
        // 31 days: `divmod(31, 30.5)` is (1.0, 0.5), `int(0.5)` is 0, so the
        // half day is dropped and no day is left to write.
        assert_eq!(
            natural
                .precisedelta(Duration::from_days(31), Days, &[], 2)
                .unwrap(),
            "1 month"
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
            // (1.0, 4.5) and `int(4.5)` is 4: the half day is lost.
            "1 year, 1 month and 4 days"
        );
        // A whole year count that rounding made a float is written as an
        // integer: `if math.modf(fmt_value)[0] == 0: fmt_value = int(...)`.
        assert_eq!(
            natural
                .precisedelta(Duration::from_days(730), Years, &[], 2)
                .unwrap(),
            "2 years"
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

    /// Three cases in which the result differs from a plain rounding of the
    /// span, each traced through `humanize` 4.16's source: the minimum unit is rounded with the
    /// format before it is compared with zero (`_rounding_by_fmt`), and a
    /// unit rounded up to the next one's size is carried.
    #[test]
    fn precisedelta_rounds_before_testing_the_fraction_and_carries() {
        use PreciseUnit::{Days, Hours, Microseconds, Minutes, Seconds};
        let natural = natural();
        let precise = |span: Duration, minimum, suppress: &[PreciseUnit], decimals| {
            natural
                .precisedelta(span, minimum, suppress, decimals)
                .unwrap()
        };
        // 2 days and 1 microsecond: 1e-6 seconds rounds to `0.00`, which is
        // not greater than zero, so no seconds are written.
        let span = Duration::from_days(2) + Duration::from_micros(1);
        assert_eq!(precise(span, Seconds, &[], 2), "2 days");
        // 59.999999 seconds rounds to `60.00`: carried into one minute.
        assert_eq!(
            precise(Duration::from_micros(59_999_999), Seconds, &[], 2),
            "1 minute"
        );
        // 59 minutes and 59.999999 seconds: the carry runs on through the
        // minutes into one hour.
        let span = Duration::from_minutes(59) + Duration::from_micros(59_999_999);
        assert_eq!(precise(span, Seconds, &[], 2), "1 hour");
        // A day that rounds to 31 is carried into a month by subtracting 31.
        let span = Duration::from_days(30) + Duration::from_hours(24) - Duration::from_micros(1);
        assert_eq!(precise(span, Days, &[], 2), "1 month");
        // With zero decimals the rounded value reads back as an integer.
        assert_eq!(
            precise(Duration::from_millis(90_500), Minutes, &[], 0),
            "2 minutes"
        );
        // Whole units never take a fraction, and a zero minimum is written
        // only when nothing else is.
        assert_eq!(precise(Duration::from_hours(5), Hours, &[], 2), "5 hours");
        assert_eq!(
            precise(Duration::ZERO, Microseconds, &[], 2),
            "0 microseconds"
        );
    }

    /// Python holds the microseconds as a float once seconds are suppressed
    /// (`secs * 1e6 + usecs`), so a count beyond 2⁵³ microseconds rounds to
    /// a multiple of its ulp; the library reproduces it.
    #[test]
    fn precisedelta_keeps_pythons_float_microseconds_beyond_two_to_the_53() {
        use PreciseUnit::{Days, Hours, Microseconds, Minutes, Months, Seconds, Years};
        let span = Duration::from_micros(83_917_295_999_999_000);
        let text = natural()
            .precisedelta(
                span,
                Microseconds,
                &[
                    Years,
                    Months,
                    Days,
                    Hours,
                    Minutes,
                    Seconds,
                    PreciseUnit::Milliseconds,
                ],
                2,
            )
            .unwrap();
        // `float(83917295999999000)` is 83917295999999008.0.
        assert_eq!(text, "83917295999999008 microseconds");
    }

    #[cfg(feature = "format")]
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

    /// The catalogue that serves a locale follows its fallback chain and
    /// passes over one that does not translate the words asked for: German
    /// has *fünf* and *Millionen* but no *Byte* (`de_DE.po` of `humanize`
    /// 4.16.0, which `tests/gettext_catalogues.rs` holds to GNU gettext).
    #[test]
    fn a_locale_is_served_by_the_first_catalogue_that_translates_the_words() {
        use crate::natural::{NaturalPhrases, NaturalWords};
        let language = |tag: &str, words| {
            NaturalPhrases::for_locale(&Locale::parse(tag).unwrap(), words).language
        };
        assert_eq!(language("de", NaturalWords::Apnumber), "de-DE");
        assert_eq!(language("de-AT", NaturalWords::Intword), "de-DE");
        assert_eq!(language("de", NaturalWords::Naturalsize), "en");
        assert_eq!(language("pt-AO", NaturalWords::Apnumber), "pt-PT");
        assert_eq!(language("pt", NaturalWords::Apnumber), "en");
        assert_eq!(language("zh-CN", NaturalWords::Time), "zh-CN");
        assert_eq!(language("zh-Hant-TW", NaturalWords::Time), "zh-HK");
        assert_eq!(language("nb", NaturalWords::Time), "nb");
        assert_eq!(language("no", NaturalWords::Time), "nb");
        assert_eq!(language("en-GB", NaturalWords::Time), "en");
        assert_eq!(language("und", NaturalWords::Any), "en");
        assert_eq!(language("tlh", NaturalWords::Any), "tlh");
        // A catalogue serves exactly the languages it is for, never another,
        // and only for the functions whose every message it translates:
        // Japanese leaves *now*, *a moment* and the *and* of a list
        // untranslated, so it serves the days and not the times.
        assert_eq!(language("ja-JP", NaturalWords::Day), "ja-JP");
        assert_eq!(language("ja-JP", NaturalWords::Time), "en");
        assert_eq!(language("ja-JP", NaturalWords::Precise), "en");
        assert_eq!(language("ja-JP", NaturalWords::Apnumber), "en");
        // The fine units of a minimum unit below the second are a message
        // more: German, Finnish and Hungarian lack them.
        assert_eq!(language("de", NaturalWords::Time), "de-DE");
        assert_eq!(language("de", NaturalWords::TimeFine), "en");
        assert_eq!(language("sv-FI", NaturalWords::Time), "sv-SE");
    }
}
