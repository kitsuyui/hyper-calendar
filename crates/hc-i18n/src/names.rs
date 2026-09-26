//! Localised calendar vocabulary, and the lookup that finds it.
//!
//! # The key
//!
//! A name is identified by four things, and dropping any one of them breaks
//! a real language:
//!
//! * **locale** — obviously.
//! * **calendar** — month 1 is *January* in `gregory`, *Muharram* in
//!   `islamic`, *Tishri* in `hebrew` and 睦月 in the Japanese reading of the
//!   old lunisolar calendar.
//! * **width** — `Wide`, `Abbreviated`, `Narrow`, and for weekdays `Short`.
//!   `Narrow` is not an abbreviation: it may repeat (English narrow weekdays
//!   are M T W T F S S) and is only usable in a column header.
//! * **context** — `Format` for a name inside a full date, `Standalone` for
//!   a name on its own. Russian says *12 сентября* but *сентябрь* as a
//!   heading; Czech, Greek and Catalan do the same. A library that carries
//!   only one form prints one of them wrongly.
//!
//! # The lookup
//!
//! [`locale_data`] and every function beside it walk
//! [`crate::locale::Locale::fallback`] and take the **first entry that
//! actually carries the field asked for**, not merely the first entry that
//! matches the locale. So a locale can define its weekday names and inherit
//! everything else, and a narrow width that nobody has filled in falls back
//! to abbreviated and then to wide rather than returning nothing.
//!
//! All of it is data: see [`crate::data`].

use core::fmt;

use hc_calendar::cycle::Sexagenary;
use hc_calendar::cycle::readings::Reading;
use hc_calendar::shape::{CycleShape, EraName};
use hc_calendar::{CalendarId, CalendarMeta, Month, Weekday};

use crate::casing::CasingStyle;
use crate::data::{LOCALES, ROOT};
use crate::direction::Direction;
use crate::locale::{Locale, region_first_day_of_week};

/// How long a name is allowed to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameWidth {
    /// The full name: *September*, *сентября*.
    Wide,
    /// The usual abbreviation: *Sep*, *сент.*
    Abbreviated,
    /// Weekdays only: the two-letter form used in narrow calendar grids
    /// where the narrow form would be ambiguous.
    Short,
    /// One or two characters, unique only within its own column.
    Narrow,
}

impl NameWidth {
    /// Every width, widest first.
    pub const ALL: [Self; 4] = [Self::Wide, Self::Abbreviated, Self::Short, Self::Narrow];

    /// The width to try when this one has no data.
    ///
    /// Narrow and short both degrade to abbreviated, abbreviated to wide,
    /// and wide to nothing.
    #[must_use]
    pub const fn wider(self) -> Option<Self> {
        match self {
            Self::Narrow | Self::Short => Some(Self::Abbreviated),
            Self::Abbreviated => Some(Self::Wide),
            Self::Wide => None,
        }
    }
}

/// Whether a name stands inside a date or on its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameContext {
    /// Inside a formatted date: the genitive in Russian, Czech and Polish.
    Format,
    /// On its own, as a heading or a list entry: the nominative.
    Standalone,
}

/// The two day periods a 12-hour clock needs.
///
/// CLDR defines finer periods (morning1, afternoon2, …) that vary by locale
/// and by hour; those are out of scope here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DayPeriod {
    /// Before noon.
    Am,
    /// Noon and after.
    Pm,
}

impl DayPeriod {
    /// The period an hour of the day falls in.
    #[must_use]
    pub const fn from_hour(hour: u8) -> Self {
        if hour < 12 { Self::Am } else { Self::Pm }
    }

    /// The index of this period in a data slice.
    const fn index(self) -> usize {
        match self {
            Self::Am => 0,
            Self::Pm => 1,
        }
    }
}

/// One set of names in each width.
///
/// An empty slice means "not stated here"; lookup then tries a wider width
/// and, failing that, the next locale up the fallback chain. Leaving a width
/// empty is how a data entry says *the same as the wider one*, which is the
/// truth for Japanese and Chinese month names.
#[derive(Debug, Clone, Copy)]
pub struct WidthSet {
    /// Full names.
    pub wide: &'static [&'static str],
    /// Abbreviated names.
    pub abbreviated: &'static [&'static str],
    /// Short names; weekdays only.
    pub short: &'static [&'static str],
    /// Narrow names.
    pub narrow: &'static [&'static str],
}

impl WidthSet {
    /// A set with nothing in it.
    pub const EMPTY: Self = Self {
        wide: &[],
        abbreviated: &[],
        short: &[],
        narrow: &[],
    };

    /// A set where every width is the same list.
    #[must_use]
    pub const fn uniform(names: &'static [&'static str]) -> Self {
        Self {
            wide: names,
            abbreviated: &[],
            short: &[],
            narrow: &[],
        }
    }

    /// The names at exactly this width, without any fallback.
    #[must_use]
    pub const fn exact(&self, width: NameWidth) -> &'static [&'static str] {
        match width {
            NameWidth::Wide => self.wide,
            NameWidth::Abbreviated => self.abbreviated,
            NameWidth::Short => self.short,
            NameWidth::Narrow => self.narrow,
        }
    }

    /// The names at this width, degrading to wider ones when it is empty.
    #[must_use]
    pub fn get(&self, width: NameWidth) -> &'static [&'static str] {
        let mut current = Some(width);
        while let Some(candidate) = current {
            let names = self.exact(candidate);
            if !names.is_empty() {
                return names;
            }
            current = candidate.wider();
        }
        &[]
    }

    /// Whether every width is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.wide.is_empty()
            && self.abbreviated.is_empty()
            && self.short.is_empty()
            && self.narrow.is_empty()
    }
}

/// Names in both contexts.
///
/// An empty `standalone` means the language does not distinguish the two,
/// which is the common case.
#[derive(Debug, Clone, Copy)]
pub struct ContextualNames {
    /// Names used inside a formatted date.
    pub format: WidthSet,
    /// Names used on their own, where they differ.
    pub standalone: WidthSet,
}

impl ContextualNames {
    /// Nothing in either context.
    pub const EMPTY: Self = Self {
        format: WidthSet::EMPTY,
        standalone: WidthSet::EMPTY,
    };

    /// The same names in both contexts.
    #[must_use]
    pub const fn same(format: WidthSet) -> Self {
        Self {
            format,
            standalone: WidthSet::EMPTY,
        }
    }

    /// The names for a width and context, applying both fallbacks.
    #[must_use]
    pub fn get(&self, width: NameWidth, context: NameContext) -> &'static [&'static str] {
        if context == NameContext::Standalone {
            let names = self.standalone.get(width);
            if !names.is_empty() {
                return names;
            }
        }
        self.format.get(width)
    }

    /// Whether both contexts are empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.format.is_empty() && self.standalone.is_empty()
    }
}

/// Era names, with the codes they are addressed by.
///
/// `codes` and each non-empty width are parallel: era `n` is `codes[n]` and
/// `names.wide[n]`. Gregorian data has two eras; the Japanese calendar has
/// as many as the data entry lists.
#[derive(Debug, Clone, Copy)]
pub struct EraNames {
    /// Stable machine codes, for example `["bc", "ad"]`.
    pub codes: &'static [&'static str],
    /// The names themselves.
    pub names: WidthSet,
    /// Which calendars these eras belong to.
    ///
    /// Empty means "the calendars this entry serves", which is the usual
    /// case. It is set where the entry's cycles are shared more widely than
    /// its eras: the Gregorian months serve a dozen calendars and BCE/CE
    /// serve five of them, because the Buddhist calendar counts in BE and
    /// the Minguo calendar in 民國 while both write the months the same way.
    ///
    /// Sharing at the level of the *entry* rather than the cycle is what
    /// made that indistinguishable, and it is the same mistake in miniature
    /// that the twelve-or-thirteen assumption was: one shape assumed for
    /// things that do not share one.
    pub calendars: &'static [CalendarId],
}

impl EraNames {
    /// No eras.
    pub const EMPTY: Self = Self {
        codes: &[],
        names: WidthSet::EMPTY,
        calendars: &[],
    };

    /// Whether these eras belong to `calendar`, given the entry that holds
    /// them serves `served`.
    #[must_use]
    pub fn belongs_to(&self, calendar: CalendarId, served: &[CalendarId]) -> bool {
        let scope = if self.calendars.is_empty() {
            served
        } else {
            self.calendars
        };
        scope.contains(&calendar)
    }

    /// The index of an era code.
    ///
    /// Codes are ASCII identifiers and are matched without regard to case,
    /// because the calendars spell theirs as their sources do — `AH`,
    /// `AM`, `BE` — and the data spells them as CLDR does, in lower case.
    /// A lookup that failed on the difference named nothing.
    #[must_use]
    pub fn index_of(&self, code: &str) -> Option<usize> {
        self.codes
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(code))
    }
}

/// How a locale writes a calendar's units and dates: patterns over the
/// placeholders a renderer fills in.
///
/// # The placeholders
///
/// | Placeholder | Filled with |
/// | --- | --- |
/// | `{era}` | the era's name: the locale's, else the calendar's own, else its code; nothing for the [`DateTemplates::implied_era`] |
/// | `{year}` | the year as a number in the locale's numbering system |
/// | `{month}` | the month's label: name and leap prefix, or its number |
/// | `{day}` | the day's name from [`DateTemplates::day_names`], or its number |
/// | `{sexagenary}` | the year's stem and branch in the locale's reading, where the date carries a `sexagenary_year` |
/// | `{extras}` | the calendar's extra fields as `name=value` pairs joined by `;` |
///
/// In [`DateTemplates::date`], `{year}`, `{month}` and `{day}` stand for
/// the *rendered* year, month and day — the year with its era, the day
/// with its 日 — so that a field the date does not carry vanishes with
/// the affixes its own template gave it.
///
/// A placeholder whose field the date does not carry is filled with
/// nothing, and the renderer then trims the pattern and collapses the
/// spaces that are left, so that `{year} {era}` writes `2026` for a
/// calendar without eras. A template that is empty says *no
/// preference*, and so does one whose every placeholder came out empty:
/// the renderer takes the next level of the [`TemplateChain`] — the
/// entries', the locale's, then [`DateTemplates::DEFAULT`].
///
/// # Where they live
///
/// A locale states its general way of writing a date in
/// [`LocaleData::templates`], and an entry that serves a family of
/// calendars states what differs for that family in
/// [`CalendarNames::templates`]: the sexagenary year of the Chinese
/// calendar, the named days of its months. Both are data, and every
/// convention in them is sourced in a comment beside it.
#[derive(Debug, Clone, Copy)]
pub struct DateTemplates {
    /// The era on its own.
    pub era: &'static str,
    /// The year with its era: `{era}{year}年`, `{year} {era}`.
    pub year: &'static str,
    /// The first year of an era, where the language has a word for it
    /// that replaces the number — `{era}元年`, `{era} 원년` — and empty
    /// where it does not. Applied only when the date carries an era.
    pub first_year: &'static str,
    /// The month on its own.
    pub month: &'static str,
    /// The day on its own.
    pub day: &'static str,
    /// The whole date.
    pub date: &'static str,
    /// The locale's names for the days of the month, day 1 at index 0,
    /// where it has them — 初一 … 三十 — and empty where days are numbered.
    pub day_names: &'static [&'static str],
    /// An era code so usual that the locale leaves it unwritten in a year:
    /// `ad` for the Gregorian family, whose 2026 is never *2026 AD* in
    /// running text, `am` for the Hebrew calendar's 5784. Matched without
    /// regard to case, as era codes are; empty where every era is written.
    pub implied_era: &'static str,
}

impl DateTemplates {
    /// No preference in any field.
    pub const NONE: Self = Self {
        era: "",
        year: "",
        first_year: "",
        month: "",
        day: "",
        date: "",
        day_names: &[],
        implied_era: "",
    };

    /// What a renderer writes when no locale has said otherwise: each
    /// field by itself, and a date as its fields in the order
    /// [`hc_calendar::DateFields`] lists them — era, year, month, day, then
    /// the extra fields — separated by spaces. It states the numbers and
    /// names the library already has and invents no orthography.
    pub const DEFAULT: Self = Self {
        era: "{era}",
        year: "{year} {era}",
        first_year: "",
        month: "{month}",
        day: "{day}",
        date: "{year} {month} {day} {extras}",
        day_names: &[],
        implied_era: "",
    };

    /// Whether every field is empty.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.era.is_empty()
            && self.year.is_empty()
            && self.first_year.is_empty()
            && self.month.is_empty()
            && self.day.is_empty()
            && self.date.is_empty()
            && self.day_names.is_empty()
            && self.implied_era.is_empty()
    }

    /// These templates with every empty field taken from `other`.
    #[must_use]
    pub const fn or(self, other: Self) -> Self {
        const fn pick(first: &'static str, second: &'static str) -> &'static str {
            if first.is_empty() { second } else { first }
        }
        Self {
            era: pick(self.era, other.era),
            year: pick(self.year, other.year),
            first_year: pick(self.first_year, other.first_year),
            month: pick(self.month, other.month),
            day: pick(self.day, other.day),
            date: pick(self.date, other.date),
            day_names: if self.day_names.is_empty() {
                other.day_names
            } else {
                self.day_names
            },
            implied_era: pick(self.implied_era, other.implied_era),
        }
    }

    /// Whether `code` is the era this locale leaves unwritten.
    #[must_use]
    pub fn implies_era(&self, code: &str) -> bool {
        !self.implied_era.is_empty() && self.implied_era.eq_ignore_ascii_case(code)
    }
}

/// The templates that apply to one calendar in one locale, level by level:
/// what the entries serving the calendar state, what the locale states in
/// general, and [`DateTemplates::DEFAULT`].
///
/// A renderer tries each level's template for a field in turn and moves
/// to the next when the template says nothing — its field is empty, or
/// every placeholder in it came out empty, as `{sexagenary}年` does for a
/// date that carries no sexagenary year. The levels are kept apart rather
/// than merged so that a renderer can tell the two cases from one another.
#[derive(Debug, Clone, Copy)]
pub struct TemplateChain {
    /// The serving entries' own templates, merged field by field in entry
    /// order.
    pub entry: DateTemplates,
    /// The locale's general templates.
    pub locale: DateTemplates,
}

impl TemplateChain {
    /// Nothing but the default.
    pub const NONE: Self = Self {
        entry: DateTemplates::NONE,
        locale: DateTemplates::NONE,
    };

    /// The levels, most specific first, ending in the default.
    #[must_use]
    pub const fn levels(&self) -> [DateTemplates; 3] {
        [self.entry, self.locale, DateTemplates::DEFAULT]
    }

    /// Every level merged into one, for the fields that do not need the
    /// levels told apart: the day names and the implied era.
    #[must_use]
    pub const fn merged(&self) -> DateTemplates {
        self.entry.or(self.locale).or(DateTemplates::DEFAULT)
    }
}

/// Names for an intercalary month, and for the months of a year that has
/// one, where the locale's word is not the ordinary name with a prefix.
///
/// The Hebrew calendar's *Adar I* is the intercalary repetition of its
/// fifth month, Shevaṭ, and is not called *leap Shevaṭ*; and the Adar of a
/// leap year is *Adar II*. A prefix can say neither, so a calendar entry
/// may list them by the month's ordinal. Most calendars need neither list,
/// and [`LeapMonthNames::NONE`] says so.
#[derive(Debug, Clone, Copy)]
pub struct LeapMonthNames {
    /// The name of the intercalary repetition of a month, by the month's
    /// ordinal: `(5, "Adar I")`.
    pub intercalary: &'static [(u8, &'static str)],
    /// The name a month takes in a year that has the intercalary month, by
    /// ordinal: `(6, "Adar II")`.
    pub in_leap_years: &'static [(u8, &'static str)],
}

impl LeapMonthNames {
    /// The prefix and the ordinary names suffice.
    pub const NONE: Self = Self {
        intercalary: &[],
        in_leap_years: &[],
    };

    fn find(list: &'static [(u8, &'static str)], ordinal: u8) -> Option<&'static str> {
        list.iter()
            .find(|(month, _)| *month == ordinal)
            .map(|(_, name)| *name)
    }
}

/// The vocabulary of one calendar in one locale.
#[derive(Debug, Clone, Copy)]
pub struct CalendarNames {
    /// Every calendar this vocabulary serves, by registry identifier.
    ///
    /// A list rather than one key, because the registry splits calendars
    /// that share a vocabulary. All five tabular and observational Hijri
    /// calendars use the same twelve Arabic month names; the Gregorian
    /// months serve the Julian, Buddhist, Minguo and Japanese calendars as
    /// well. One entry, several identifiers.
    ///
    /// These are [`CalendarId`]s and not CLDR strings, which is what makes
    /// a wrong one a test failure: `hyper-calendar`'s vocabulary test
    /// asserts every identifier here is a calendar the registry answers to.
    /// Three were not — `persian`, `islamic` and `iso8601` were written,
    /// tested and unreachable, because the registry calls them
    /// `persian-arithmetic`, `islamic-civil` and `iso8601-week`.
    pub calendars: &'static [CalendarId],
    /// The names of each positional cycle, keyed by the cycle kind the
    /// calendar declares in [`hc_calendar::Calendar::cycles`].
    ///
    /// `month` is the usual one. A calendar with nineteen months has
    /// nineteen names here and a calendar with a ten-day week has ten;
    /// nothing in this type knows what a month is or how many there should
    /// be, which is the point.
    pub cycles: &'static [CycleNames],
    /// What an intercalary month is prefixed with: 闰 in Chinese, 閏 in
    /// Japanese, `leap ` in English. Empty where the calendar has none.
    pub leap_month_prefix: &'static str,
    /// The intercalary month's own name, and the leap-year names of the
    /// months, where a prefix cannot say them.
    pub leap_names: LeapMonthNames,
    /// Era names.
    pub eras: EraNames,
    /// Quarter names, indexed from zero for quarter 1.
    pub quarters: ContextualNames,
    /// How the locale writes these calendars' units where that differs
    /// from how it writes a date in general; [`DateTemplates::NONE`] where
    /// it does not.
    pub templates: DateTemplates,
}

/// The names of one positional cycle.
#[derive(Debug, Clone, Copy)]
pub struct CycleNames {
    /// Which cycle, matching [`hc_calendar::shape::CycleShape::kind`]:
    /// `month`, `weekday`, `decade-day`, and so on.
    pub kind: &'static str,
    /// The names, position 1 at index 0.
    pub names: ContextualNames,
}

impl CycleNames {
    /// A cycle's names.
    #[must_use]
    pub const fn new(kind: &'static str, names: ContextualNames) -> Self {
        Self { kind, names }
    }
}

/// Returned when a calendar has no names for a cycle, so that callers can
/// take a reference without an `Option` at every step.
static NO_NAMES: ContextualNames = ContextualNames::EMPTY;

impl CalendarNames {
    /// An entry with nothing but its identifiers.
    #[must_use]
    pub const fn empty(calendars: &'static [CalendarId]) -> Self {
        Self {
            calendars,
            cycles: &[],
            leap_month_prefix: "",
            leap_names: LeapMonthNames::NONE,
            eras: EraNames::EMPTY,
            quarters: ContextualNames::EMPTY,
            templates: DateTemplates::NONE,
        }
    }

    /// This entry with names for its intercalary month and its leap years.
    #[must_use]
    pub const fn with_leap_names(mut self, names: LeapMonthNames) -> Self {
        self.leap_names = names;
        self
    }

    /// This entry with templates of its own.
    #[must_use]
    pub const fn with_templates(mut self, templates: DateTemplates) -> Self {
        self.templates = templates;
        self
    }

    /// This entry with a leap-month prefix.
    #[must_use]
    pub const fn with_leap_month_prefix(mut self, prefix: &'static str) -> Self {
        self.leap_month_prefix = prefix;
        self
    }

    /// Whether this entry serves `calendar`.
    #[must_use]
    pub fn serves(&self, calendar: CalendarId) -> bool {
        self.calendars.contains(&calendar)
    }

    /// The names of one cycle, empty when this entry has none.
    #[must_use]
    pub fn cycle(&self, kind: &str) -> &ContextualNames {
        self.cycles
            .iter()
            .find(|entry| entry.kind == kind)
            .map_or(&NO_NAMES, |entry| &entry.names)
    }

    /// The month names, which is the cycle nearly every caller wants.
    #[must_use]
    pub fn months(&self) -> &ContextualNames {
        self.cycle(hc_calendar::shape::MONTH)
    }
}

/// How a locale writes the sexagenary cycle.
///
/// The stems and branches are one of the readings `hc_calendar` catalogues:
/// a locale chooses a reading, it does not spell one. The only names this
/// crate holds itself are the zodiac animals, which are words of the
/// language rather than readings of the cycle — 卯 is the rabbit in Chinese
/// and the cat in Vietnamese.
#[derive(Debug, Clone, Copy)]
pub struct SexagenaryNames {
    /// The reading the locale writes the stems and branches in.
    pub reading: Option<&'static Reading>,
    /// The twelve zodiac animals, in branch order.
    pub zodiac: Option<&'static [&'static str; 12]>,
    /// What the locale writes between a stem and its branch: nothing in
    /// the Han and Hangul readings (甲子, 갑자), a space in the Vietnamese
    /// and pinyin ones (Giáp Tý, jia zi).
    pub joiner: &'static str,
}

impl SexagenaryNames {
    /// No cycle names.
    pub const EMPTY: Self = Self {
        reading: None,
        zodiac: None,
        joiner: "",
    };
}

/// A calendar's name in a locale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarDisplayName {
    /// The calendar.
    pub calendar: CalendarId,
    /// What the locale calls it.
    pub name: &'static str,
}

impl CalendarDisplayName {
    /// A name for a calendar.
    #[must_use]
    pub const fn new(calendar: &'static str, name: &'static str) -> Self {
        Self {
            calendar: CalendarId(calendar),
            name,
        }
    }
}

/// Everything this crate knows about one locale.
///
/// This struct is the unit of data. Adding a language means adding one value
/// of it to [`crate::data::LOCALES`]; no code in this crate learns a new
/// branch.
#[derive(Debug, Clone, Copy)]
pub struct LocaleData {
    /// The canonical tag this entry answers for, such as `zh-Hans`.
    pub tag: &'static str,
    /// The language's name in English: *Japanese*.
    pub english_name: &'static str,
    /// The language's name in itself: 日本語.
    pub native_name: &'static str,
    /// The ISO 15924 code of the script the locale is written in: `Latn`,
    /// `Jpan`, `Hans`, `Arab`. A renderer with a name in a calendar's own
    /// script and a romanisation of it gives a `Latn` locale the
    /// romanisation and every other locale the name itself.
    pub script: &'static str,
    /// The direction of the locale's usual script.
    pub direction: Direction,
    /// The default numbering system identifier.
    pub numbering: &'static str,
    /// The customary first day of the week, where the locale implies one.
    pub first_day_of_week: Weekday,
    /// Which case mappings apply.
    pub casing: CasingStyle,
    /// Whether month and weekday names are written with a capital.
    pub capitalises_month_names: bool,
    /// Weekday names, Monday first.
    pub weekdays: ContextualNames,
    /// Day-period names: `[am, pm]`.
    pub day_periods: ContextualNames,
    /// Sexagenary cycle names.
    pub cycle: SexagenaryNames,
    /// Per-calendar vocabulary.
    pub calendars: &'static [CalendarNames],
    /// How the locale writes a date in general; what an entry in
    /// `calendars` does not override.
    pub templates: DateTemplates,
    /// What the locale calls the calendars it has a name for.
    pub calendar_names: &'static [CalendarDisplayName],
}

impl LocaleData {
    /// Every entry that serves `id`, in table order.
    ///
    /// More than one may: a calendar can take its months from the shared
    /// Gregorian entry and its eras from an entry of its own. Thai is the
    /// case — the Buddhist calendar counts years differently and names the
    /// months identically — and the old code expressed it with a fallback
    /// from `buddhist` to `gregory`. This is that fallback generalised to
    /// every cycle instead of only months.
    pub fn entries_for(&self, id: CalendarId) -> impl Iterator<Item = &CalendarNames> {
        self.calendars.iter().filter(move |entry| entry.serves(id))
    }

    /// The first entry serving `id`, if any.
    #[must_use]
    pub fn calendar(&self, id: CalendarId) -> Option<&CalendarNames> {
        self.entries_for(id).next()
    }

    /// The names of one cycle for `id`, from whichever serving entry has
    /// them.
    #[must_use]
    pub fn cycle_for(&self, id: CalendarId, kind: &str) -> Option<&ContextualNames> {
        self.entries_for(id)
            .map(|entry| entry.cycle(kind))
            .find(|names| !names.is_empty())
    }

    /// The era names for `id`, from whichever serving entry has them.
    #[must_use]
    pub fn eras_for(&self, id: CalendarId) -> Option<&EraNames> {
        self.entries_for(id)
            .find(|entry| {
                !entry.eras.codes.is_empty() && entry.eras.belongs_to(id, entry.calendars)
            })
            .map(|entry| &entry.eras)
    }

    /// The quarter names for `id`, from whichever serving entry has them.
    #[must_use]
    pub fn quarters_for(&self, id: CalendarId) -> Option<&ContextualNames> {
        self.entries_for(id)
            .map(|entry| &entry.quarters)
            .find(|names| !names.is_empty())
    }

    /// The templates for `id`: every serving entry's own, merged field by
    /// field in entry order, and the locale's general ones behind them;
    /// `None` when no entry serves `id` at all, because a locale's way of
    /// writing a date is stated for the calendars it names and not assumed
    /// for the rest.
    #[must_use]
    pub fn templates_for(&self, id: CalendarId) -> Option<TemplateChain> {
        let mut entries = self.entries_for(id).peekable();
        entries.peek()?;
        let entry = entries.fold(DateTemplates::NONE, |merged, entry| {
            merged.or(entry.templates)
        });
        Some(TemplateChain {
            entry,
            locale: self.templates,
        })
    }

    /// Whether this entry names `id` in any way.
    #[must_use]
    pub fn names_calendar(&self, id: CalendarId) -> bool {
        self.entries_for(id).next().is_some()
    }

    /// What this locale calls `id`, if it has a name for it.
    #[must_use]
    pub fn calendar_name(&self, id: CalendarId) -> Option<&'static str> {
        self.calendar_names
            .iter()
            .find(|entry| entry.calendar == id)
            .map(|entry| entry.name)
    }
}

/// Calendars that borrow the Gregorian month and quarter names.
///
/// They differ from `gregory` in how they count years, not in what they call
/// the months, so a data entry only has to state the parts that differ —
/// Japanese era names, say — and month lookup finds the rest.
/// Walk the fallback chain and return the first entry that yields a value.
///
/// The root entry is the floor, so a caller only sees `None` when nothing in
/// the whole chain — root included — carries the field.
fn resolve<T, F>(locale: &Locale, pick: F) -> Option<T>
where
    F: Fn(&'static LocaleData) -> Option<T>,
{
    for candidate in locale.fallback() {
        for data in LOCALES {
            if candidate.matches_tag(data.tag)
                && let Some(value) = pick(data)
            {
                return Some(value);
            }
        }
    }
    pick(&ROOT)
}

/// The data entry a locale resolves to.
///
/// Never fails: the root entry answers for everything else.
#[must_use]
pub fn locale_data(locale: &Locale) -> &'static LocaleData {
    resolve(locale, Some).unwrap_or(&ROOT)
}

/// A month name plus the prefix an intercalary month carries.
///
/// The two are kept apart so that a `no_std` caller can write them straight
/// to a sink without joining them into an allocation first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthLabel {
    /// The leap-month prefix, or `""` for an ordinary month.
    pub prefix: &'static str,
    /// The month name.
    pub name: &'static str,
}

impl fmt::Display for MonthLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.prefix)?;
        f.write_str(self.name)
    }
}

/// The name of a month.
///
/// Returns `None` if no entry in the fallback chain names that month of that
/// calendar — an ordinal past the end of the list, or a calendar this crate
/// has no vocabulary for. This is [`month_label_in`] for a common year; a
/// caller that knows the year has an intercalary month passes that on, so
/// that the Hebrew Adar of a leap year is *Adar II*.
#[must_use]
pub fn month_label(
    locale: &Locale,
    calendar: CalendarId,
    month: Month,
    width: NameWidth,
    context: NameContext,
) -> Option<MonthLabel> {
    month_label_in(locale, calendar, month, false, width, context)
}

/// Whether any locale in the chain names a month of this calendar
/// differently in a leap year — so that a caller can spare itself the
/// question of whether the year is one, which an astronomical calendar
/// answers slowly.
#[must_use]
pub fn has_leap_year_month_names(locale: &Locale, calendar: CalendarId) -> bool {
    resolve(locale, |data| {
        data.entries_for(calendar)
            .find(|entry| !entry.months().is_empty())
            .map(|entry| !entry.leap_names.in_leap_years.is_empty())
    })
    .unwrap_or(false)
}

/// The name of a month, in a year that has the calendar's intercalary
/// month when `in_leap_year` says so.
///
/// An intercalary month is the locale's own name for it where it has one
/// (*Adar I*), else the ordinary name with the locale's prefix (闰二月); a
/// month of a leap year takes its leap-year name where the locale lists
/// one (*Adar II*). Everything comes from whichever entry carries the
/// months, since that is the entry that knows how the calendar writes
/// them.
#[must_use]
pub fn month_label_in(
    locale: &Locale,
    calendar: CalendarId,
    month: Month,
    in_leap_year: bool,
    width: NameWidth,
    context: NameContext,
) -> Option<MonthLabel> {
    let index = usize::from(month.ordinal).checked_sub(1)?;
    resolve(locale, |data| {
        let entry = data
            .entries_for(calendar)
            .find(|entry| !entry.months().is_empty())?;
        let name = entry.months().get(width, context).get(index).copied()?;
        if month.leap {
            return Some(
                LeapMonthNames::find(entry.leap_names.intercalary, month.ordinal).map_or(
                    MonthLabel {
                        prefix: entry.leap_month_prefix,
                        name,
                    },
                    |own| MonthLabel {
                        prefix: "",
                        name: own,
                    },
                ),
            );
        }
        let name = if in_leap_year {
            LeapMonthNames::find(entry.leap_names.in_leap_years, month.ordinal).unwrap_or(name)
        } else {
            name
        };
        Some(MonthLabel { prefix: "", name })
    })
}

/// The name of a position in one of a calendar's cycles: the locale's
/// name when it has one, and otherwise the calendar's own.
///
/// This is the lookup that lets a haabʼ month or a Pawukon week be named
/// without any locale having transcribed it: the calendar declares its
/// names with its shape, and a locale overrides them only where the
/// language has its own word. `index` is zero-based.
#[must_use]
pub fn position_name(
    locale: &Locale,
    calendar: CalendarId,
    cycle: &CycleShape,
    index: usize,
    width: NameWidth,
    context: NameContext,
) -> Option<&'static str> {
    resolve(locale, |data| {
        data.cycle_for(calendar, cycle.kind)?
            .get(width, context)
            .get(index)
            .copied()
    })
    .or_else(|| cycle.name(index))
}

/// The name of a month, without any leap-month prefix.
#[must_use]
pub fn month_name(
    locale: &Locale,
    calendar: CalendarId,
    month: Month,
    width: NameWidth,
    context: NameContext,
) -> Option<&'static str> {
    month_label(locale, calendar, month, width, context).map(|label| label.name)
}

/// What a locale writes before an intercalary month of a calendar: 闰, 閏,
/// `leap `, `Adhika ` — from the first entry in the chain that names the
/// calendar's months, or empty when none does.
#[must_use]
pub fn leap_month_prefix(locale: &Locale, calendar: CalendarId) -> &'static str {
    resolve(locale, |data| {
        data.entries_for(calendar)
            .find(|entry| !entry.months().is_empty())
            .map(|entry| entry.leap_month_prefix)
    })
    .unwrap_or("")
}

/// How many months a calendar is named for in a locale.
#[must_use]
pub fn month_count(locale: &Locale, calendar: CalendarId, context: NameContext) -> Option<usize> {
    resolve(locale, |data| {
        let names = data
            .cycle_for(calendar, hc_calendar::shape::MONTH)?
            .get(NameWidth::Wide, context);
        if names.is_empty() {
            None
        } else {
            Some(names.len())
        }
    })
}

/// The name of a weekday.
#[must_use]
pub fn weekday_name(
    locale: &Locale,
    weekday: Weekday,
    width: NameWidth,
    context: NameContext,
) -> Option<&'static str> {
    let index = usize::from(weekday.iso_number()) - 1;
    resolve(locale, |data| {
        data.weekdays.get(width, context).get(index).copied()
    })
}

/// The name of a day period.
#[must_use]
pub fn day_period_name(
    locale: &Locale,
    period: DayPeriod,
    width: NameWidth,
    context: NameContext,
) -> Option<&'static str> {
    let index = period.index();
    resolve(locale, |data| {
        data.day_periods.get(width, context).get(index).copied()
    })
}

/// The name of an era, by its position in the calendar's era list.
#[must_use]
pub fn era_name(
    locale: &Locale,
    calendar: CalendarId,
    index: usize,
    width: NameWidth,
) -> Option<&'static str> {
    resolve(locale, |data| {
        data.calendar(calendar)?
            .eras
            .names
            .get(width)
            .get(index)
            .copied()
    })
}

/// The name of an era, by the stable code the data lists it under.
#[must_use]
pub fn era_name_by_code(
    locale: &Locale,
    calendar: CalendarId,
    code: &str,
    width: NameWidth,
) -> Option<&'static str> {
    resolve(locale, |data| {
        let eras = data.eras_for(calendar)?;
        let index = eras.index_of(code)?;
        eras.names.get(width).get(index).copied()
    })
}

/// The era codes a calendar is described with, in order.
#[must_use]
pub fn era_codes(locale: &Locale, calendar: CalendarId) -> Option<&'static [&'static str]> {
    resolve(locale, |data| {
        let codes = data.eras_for(calendar)?.codes;
        if codes.is_empty() { None } else { Some(codes) }
    })
}

/// How a locale writes a calendar's units and dates.
///
/// The first entry in the fallback chain that names the calendar decides:
/// its serving entries' templates, its general ones, and
/// [`DateTemplates::DEFAULT`] behind both. A calendar no locale in the
/// chain names gets the default outright.
#[must_use]
pub fn templates(locale: &Locale, calendar: CalendarId) -> TemplateChain {
    resolve(locale, |data| data.templates_for(calendar)).unwrap_or(TemplateChain::NONE)
}

/// Whether a locale is written in Latin letters, judged from the data
/// entry it resolves to.
///
/// A renderer with a calendar's own name for something and a romanisation
/// of it gives a Latin-script locale the romanisation and every other
/// locale the name itself: *Kaei* for `en`, 嘉永 for `ja`, and 嘉永 for
/// `ko` too, because a Korean reader is no better served by Hepburn.
#[must_use]
pub fn is_latin_script(locale: &Locale) -> bool {
    locale_data(locale).script == "Latn"
}

/// The name an era is written under: the locale's own, else the
/// calendar's own ([`EraName`], romanised for a Latin-script locale), else
/// the code itself.
///
/// The width applies to the locale's names and degrades as
/// [`WidthSet::get`] does. The calendar's own name answers for the eras a
/// locale's data does not list — the two hundred and forty-three nengō
/// before 明治 — so that 嘉永三年 is written as itself and never as
/// `kaei 3`.
#[must_use]
pub fn era_label<'a>(
    locale: &Locale,
    calendar: CalendarId,
    code: &'a str,
    own: Option<EraName>,
    width: NameWidth,
) -> &'a str {
    if let Some(name) = era_name_by_code(locale, calendar, code, width) {
        return name;
    }
    match own {
        Some(name) if is_latin_script(locale) => name.latin(),
        Some(name) => name.native,
        None => code,
    }
}

/// What a locale calls a calendar, if any locale in the chain has a name
/// for it.
#[must_use]
pub fn calendar_display_name(locale: &Locale, calendar: CalendarId) -> Option<&'static str> {
    resolve(locale, |data| data.calendar_name(calendar))
}

/// Whether some locale in the chain names `calendar` at all: has months,
/// eras or templates for it.
#[must_use]
pub fn names_calendar(locale: &Locale, calendar: CalendarId) -> bool {
    resolve(locale, |data| data.names_calendar(calendar).then_some(())).is_some()
}

/// Whether a tag names a locale this crate carries data for, rather than
/// one that falls through to the root.
#[must_use]
pub fn is_carried(locale: &Locale) -> bool {
    LOCALES.iter().any(|data| {
        locale
            .fallback()
            .any(|candidate| candidate.matches_tag(data.tag))
    })
}

/// The first of a calendar's native locales that this crate carries.
///
/// A calendar states the languages its sources are written in
/// ([`CalendarMeta::native_locales`]); this is the one it can be rendered
/// in here, or `None` when the crate carries none of them.
#[must_use]
pub fn native_locale(meta: &CalendarMeta) -> Option<Locale> {
    meta.native_locales
        .iter()
        .filter_map(|tag| Locale::parse(tag).ok())
        .find(is_carried)
}

/// English, the locale every fallback ends in before the calendar's own
/// names.
#[must_use]
pub fn english() -> Locale {
    Locale::parse("en").unwrap_or(Locale::ROOT)
}

/// The locale a calendar is rendered in, given the one that was asked for
/// or, with `None`, a request for the calendar's own.
///
/// One rule for every rendered cell. A named locale answers when it names
/// the calendar; when it does not — Japanese has no words for the Hebrew
/// months — English answers, so that a month is named in a language the
/// reader asked for or the common fallback, never in a script the request
/// did not choose; and when English does not name the calendar either,
/// the requested locale is kept for its numbering and its general
/// templates, with the names coming from the calendar's own shape. Only a
/// request for the calendar's own (`None`) reaches for its native locale
/// first, where the crate carries it and it names the calendar, then
/// English. The tag of the data that answered is [`locale_data`] of the
/// result.
#[must_use]
pub fn locale_for_calendar(requested: Option<&Locale>, meta: &CalendarMeta) -> Locale {
    let names = |locale: &Locale| names_calendar(locale, meta.id);
    let english = english();
    match requested {
        Some(requested) if names(requested) => *requested,
        Some(requested) if !names(&english) => *requested,
        Some(_) => english,
        None => native_locale(meta)
            .filter(|native| names(native))
            .unwrap_or(english),
    }
}

/// The name of a quarter, numbered 1 to 4.
#[must_use]
pub fn quarter_name(
    locale: &Locale,
    calendar: CalendarId,
    quarter: u8,
    width: NameWidth,
    context: NameContext,
) -> Option<&'static str> {
    let index = usize::from(quarter).checked_sub(1)?;
    resolve(locale, |data| {
        data.quarters_for(calendar)?
            .get(width, context)
            .get(index)
            .copied()
    })
}

/// The reading a locale writes the sexagenary cycle in.
#[must_use]
pub fn sexagenary_reading(locale: &Locale) -> Option<&'static Reading> {
    resolve(locale, |data| data.cycle.reading)
}

/// The name of a Heavenly Stem, indexed from zero.
#[must_use]
pub fn stem_name(locale: &Locale, index: usize) -> Option<&'static str> {
    sexagenary_reading(locale).and_then(|reading| reading.stems.get(index).copied())
}

/// The name of an Earthly Branch, indexed from zero.
#[must_use]
pub fn branch_name(locale: &Locale, index: usize) -> Option<&'static str> {
    sexagenary_reading(locale).and_then(|reading| reading.branches.get(index).copied())
}

/// The name of a zodiac animal, indexed from zero in branch order.
#[must_use]
pub fn zodiac_animal_name(locale: &Locale, index: usize) -> Option<&'static str> {
    resolve(locale, |data| {
        data.cycle
            .zodiac
            .and_then(|zodiac| zodiac.get(index).copied())
    })
}

/// The stem and branch of a sexagenary position, in the locale's script.
///
/// The two are returned separately because Chinese and Japanese write them
/// adjacent (甲子) while a romanising locale wants a separator; what the
/// locale puts between them is [`sexagenary_joiner`].
#[must_use]
pub fn sexagenary_names(
    locale: &Locale,
    position: Sexagenary,
) -> Option<(&'static str, &'static str)> {
    sexagenary_reading(locale).map(|reading| reading.pair(position))
}

/// What a locale writes between a stem and its branch, from the same
/// entry that supplies its reading.
#[must_use]
pub fn sexagenary_joiner(locale: &Locale) -> &'static str {
    resolve(locale, |data| data.cycle.reading.map(|_| data.cycle.joiner)).unwrap_or("")
}

/// The first day of the week for a locale.
///
/// `-u-fw-` wins, then the region's own convention, then the language's data
/// entry, which defaults to Monday as ISO 8601 says.
#[must_use]
pub fn first_day_of_week(locale: &Locale) -> Weekday {
    if let Some(explicit) = locale.first_day_of_week() {
        return explicit;
    }
    if let Some(region) = locale.region()
        && let Some(day) = region_first_day_of_week(region)
    {
        return day;
    }
    locale_data(locale).first_day_of_week
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString as _;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).unwrap()
    }

    fn gregorian_month(tag: &str, ordinal: u8, width: NameWidth, context: NameContext) -> &str {
        month_name(
            &locale(tag),
            CalendarId("gregory"),
            Month::regular(ordinal),
            width,
            context,
        )
        .unwrap()
    }

    #[test]
    fn month_names_come_back_in_the_language_asked_for() {
        assert_eq!(
            gregorian_month("en", 1, NameWidth::Wide, NameContext::Format),
            "January"
        );
        assert_eq!(
            gregorian_month("ja", 1, NameWidth::Wide, NameContext::Format),
            "1月"
        );
        assert_eq!(
            gregorian_month("de", 3, NameWidth::Wide, NameContext::Format),
            "März"
        );
        assert_eq!(
            gregorian_month("ar", 1, NameWidth::Wide, NameContext::Format),
            "يناير"
        );
        assert_eq!(
            gregorian_month("th", 4, NameWidth::Wide, NameContext::Format),
            "เมษายน"
        );
    }

    #[test]
    fn russian_and_czech_distinguish_the_two_contexts() {
        assert_eq!(
            gregorian_month("ru", 9, NameWidth::Wide, NameContext::Format),
            "сентября"
        );
        assert_eq!(
            gregorian_month("ru", 9, NameWidth::Wide, NameContext::Standalone),
            "сентябрь"
        );
        assert_eq!(
            gregorian_month("cs", 1, NameWidth::Wide, NameContext::Format),
            "ledna"
        );
        assert_eq!(
            gregorian_month("cs", 1, NameWidth::Wide, NameContext::Standalone),
            "leden"
        );
        assert_eq!(
            gregorian_month("pl", 5, NameWidth::Wide, NameContext::Format),
            "maja"
        );
        assert_eq!(
            gregorian_month("pl", 5, NameWidth::Wide, NameContext::Standalone),
            "maj"
        );
    }

    #[test]
    fn a_language_without_the_distinction_answers_the_same_in_both_contexts() {
        for ordinal in 1..=12u8 {
            assert_eq!(
                gregorian_month("en", ordinal, NameWidth::Wide, NameContext::Format),
                gregorian_month("en", ordinal, NameWidth::Wide, NameContext::Standalone)
            );
        }
    }

    #[test]
    fn a_missing_width_degrades_to_a_wider_one() {
        // Japanese does not abbreviate 1月, so every width is the wide form.
        assert_eq!(
            gregorian_month("ja", 1, NameWidth::Abbreviated, NameContext::Format),
            "1月"
        );
        // English narrow months are single letters and do exist.
        assert_eq!(
            gregorian_month("en", 1, NameWidth::Narrow, NameContext::Format),
            "J"
        );
        assert_eq!(
            gregorian_month("en", 1, NameWidth::Abbreviated, NameContext::Format),
            "Jan"
        );
    }

    fn lunisolar_month(tag: &str, calendar: &'static str, month: Month) -> Option<String> {
        month_label(
            &locale(tag),
            CalendarId(calendar),
            month,
            NameWidth::Wide,
            NameContext::Standalone,
        )
        .map(|label| label.to_string())
    }

    #[test]
    fn the_japanese_traditional_months_serve_only_the_japanese_lunisolar_calendars() {
        // 弥生 is Japan's word for the third month of Japan's old calendar,
        // and was answering for the Chinese calendar too.
        for id in [
            "japanese-tenpo",
            "japanese-kansei",
            "japanese-horyaku",
            "japanese-jokyo",
            "japanese-senmyo",
        ] {
            assert_eq!(
                lunisolar_month("ja", id, Month::regular(3)).as_deref(),
                Some("弥生"),
                "{id}"
            );
            assert_eq!(
                lunisolar_month("ja", id, Month::regular(12)).as_deref(),
                Some("師走"),
                "{id}"
            );
        }
        for id in ["chinese", "dangi", "vietnamese"] {
            assert_eq!(
                lunisolar_month("ja", id, Month::regular(1)).as_deref(),
                Some("正月"),
                "{id}"
            );
            assert_eq!(
                lunisolar_month("ja", id, Month::regular(3)).as_deref(),
                Some("三月"),
                "{id}"
            );
            assert_eq!(
                lunisolar_month("ja", id, Month::leap(2)).as_deref(),
                Some("閏二月"),
                "{id}"
            );
            assert_eq!(
                lunisolar_month("ja", id, Month::regular(12)).as_deref(),
                Some("十二月"),
                "{id}"
            );
            assert_eq!(
                lunisolar_month("zh-Hans", id, Month::regular(12)).as_deref(),
                Some("腊月"),
                "{id}"
            );
            assert_eq!(
                lunisolar_month("zh-Hant", id, Month::leap(2)).as_deref(),
                Some("閏二月"),
                "{id}"
            );
        }
        assert_eq!(
            lunisolar_month("ja", "chinese-regnal", Month::regular(12)).as_deref(),
            Some("十二月")
        );
        // Japanese has no word for the Tibetan months, and Chinese none for
        // the Tenpō calendar's: neither leaks the other family's names.
        assert_eq!(lunisolar_month("ja", "tibetan", Month::regular(3)), None);
        assert_eq!(
            lunisolar_month("zh-Hans", "japanese-tenpo", Month::regular(12)),
            None
        );
        // English ordinals serve every numbered lunisolar calendar.
        assert_eq!(
            lunisolar_month("en", "japanese-tenpo", Month::regular(12)).as_deref(),
            Some("Twelfth Month")
        );
        assert_eq!(
            lunisolar_month("en", "chinese", Month::leap(2)).as_deref(),
            Some("leap Second Month")
        );
        assert_eq!(
            leap_month_prefix(&locale("ja"), CalendarId("chinese")),
            "閏"
        );
        assert_eq!(leap_month_prefix(&locale("ko"), CalendarId("dangi")), "윤");
        assert_eq!(leap_month_prefix(&locale("de"), CalendarId("gregory")), "");
    }

    #[test]
    fn the_hebrew_intercalary_month_and_leap_year_adar_have_their_own_names() {
        let hebrew = CalendarId("hebrew");
        let month = |tag: &str, month: Month, leap_year: bool| {
            month_label_in(
                &locale(tag),
                hebrew,
                month,
                leap_year,
                NameWidth::Wide,
                NameContext::Standalone,
            )
            .map(|label| label.to_string())
        };
        // Adar I is the intercalary repetition of Shevaṭ, month 5, and is
        // not "leap Shevat"; the Adar of a leap year is Adar II; Nisan is
        // month 7 in every year.
        assert_eq!(month("en", Month::leap(5), true).as_deref(), Some("Adar I"));
        assert_eq!(
            month("en", Month::regular(5), true).as_deref(),
            Some("Shevat")
        );
        assert_eq!(
            month("en", Month::regular(6), true).as_deref(),
            Some("Adar II")
        );
        assert_eq!(
            month("en", Month::regular(6), false).as_deref(),
            Some("Adar")
        );
        assert_eq!(
            month("en", Month::regular(7), false).as_deref(),
            Some("Nisan")
        );
        assert_eq!(
            month("en", Month::regular(12), true).as_deref(),
            Some("Elul")
        );
        assert_eq!(month("en", Month::regular(13), true), None);
        assert_eq!(month("he", Month::leap(5), true).as_deref(), Some("אדר א׳"));
        assert_eq!(
            month("he", Month::regular(6), true).as_deref(),
            Some("אדר ב׳")
        );
        assert_eq!(
            month("he", Month::regular(6), false).as_deref(),
            Some("אדר")
        );
        // A calendar with a prefix and no such names is unchanged by the
        // year.
        assert_eq!(
            lunisolar_month("zh-Hans", "chinese", Month::leap(2)).as_deref(),
            Some("闰二月")
        );
        assert_eq!(
            month_label_in(
                &locale("zh-Hans"),
                CalendarId("chinese"),
                Month::regular(2),
                true,
                NameWidth::Wide,
                NameContext::Standalone
            )
            .map(|label| label.to_string())
            .as_deref(),
            Some("二月")
        );
    }

    #[test]
    fn an_era_the_locale_does_not_list_is_written_from_the_calendars_own_name() {
        use hc_calendar::shape::EraName;
        let japanese = CalendarId("japanese");
        // The data lists the modern five; 嘉永 is not among them.
        assert_eq!(
            era_name_by_code(&locale("ja"), japanese, "kaei", NameWidth::Wide),
            None
        );
        let kaei = Some(EraName::new("嘉永", "Kaei"));
        assert_eq!(
            era_label(&locale("ja"), japanese, "kaei", kaei, NameWidth::Wide),
            "嘉永"
        );
        assert_eq!(
            era_label(&locale("en"), japanese, "kaei", kaei, NameWidth::Wide),
            "Kaei"
        );
        // A non-Latin locale gets the calendar's own orthography, not
        // Hepburn.
        assert_eq!(
            era_label(&locale("ko"), japanese, "kaei", kaei, NameWidth::Wide),
            "嘉永"
        );
        // The locale's own name still comes first, and the code is the last
        // resort.
        assert_eq!(
            era_label(&locale("ja"), japanese, "reiwa", kaei, NameWidth::Wide),
            "令和"
        );
        assert_eq!(
            era_label(&locale("ja"), japanese, "kaei", None, NameWidth::Wide),
            "kaei"
        );
        // Era codes match without regard to case, as the calendars spell
        // theirs in capitals.
        assert_eq!(
            era_name_by_code(
                &locale("en"),
                CalendarId("gregory"),
                "AD",
                NameWidth::Abbreviated
            ),
            Some("AD")
        );
    }

    #[test]
    fn a_calendar_the_locale_does_not_name_is_rendered_in_english_and_only_native_asks_for_its_own()
    {
        use hc_calendar::{CalendarMeta, Rd, YearKind};
        let meta = |id: &'static str, native: &'static [&'static str]| CalendarMeta {
            id: CalendarId(id),
            english_name: "",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
            native_locales: native,
        };
        let ja = locale("ja");
        // Japanese has no words for the Hebrew months, so the Hebrew
        // calendar answers in English, not in Hebrew.
        let hebrew = meta("hebrew", &["he"]);
        assert_eq!(locale_for_calendar(Some(&ja), &hebrew).to_string(), "en");
        assert_eq!(
            lunisolar_month("ja", "hebrew", Month::regular(1)),
            None,
            "ja itself still has no name"
        );
        assert_eq!(
            month_label(
                &locale_for_calendar(Some(&ja), &hebrew),
                CalendarId("hebrew"),
                Month::regular(1),
                NameWidth::Wide,
                NameContext::Standalone
            )
            .map(|label| label.name),
            Some("Tishri")
        );
        // The same for the Umm al-Qura calendar: English, not Arabic.
        let umalqura = meta("islamic-umalqura", &["ar"]);
        assert_eq!(locale_for_calendar(Some(&ja), &umalqura).to_string(), "en");
        // The crate carries the haabʼ's own language, but only a request
        // for the calendar's own reaches for it; English has no names for
        // it either, so the request stands, with the calendar's own names.
        let maya = meta("maya-haab", &["yua"]);
        assert_eq!(locale_for_calendar(Some(&ja), &maya).to_string(), "ja");
        assert_eq!(locale_for_calendar(None, &maya).to_string(), "yua");
        // When no locale names the calendar at all, the request stands for
        // its numbering.
        let akan = meta("akan", &["ak"]);
        assert_eq!(
            locale_for_calendar(Some(&ja), &akan).to_string(),
            "ja",
            "no locale names the Akan calendar, so the request stands for numbering"
        );
        let babylonian = meta("babylonian", &["akk"]);
        assert_eq!(
            locale_for_calendar(Some(&ja), &babylonian).to_string(),
            "en"
        );
        // Asked for the native locale outright.
        assert_eq!(locale_for_calendar(None, &hebrew).to_string(), "he");
        assert_eq!(locale_for_calendar(None, &umalqura).to_string(), "ar");
        assert_eq!(locale_for_calendar(None, &babylonian).to_string(), "en");
        let gregorian = meta("gregory", &[]);
        assert_eq!(locale_for_calendar(None, &gregorian).to_string(), "en");
        // A request that names the calendar is kept as asked, region and
        // all.
        assert_eq!(
            locale_for_calendar(Some(&locale("ja-JP")), &gregorian).to_string(),
            "ja-JP"
        );
        assert!(names_calendar(&locale("tlh"), CalendarId("gregory")));
        assert!(!names_calendar(&locale("tlh"), CalendarId("hebrew")));
        assert!(is_carried(&locale("zh-Hans-CN")));
        assert!(!is_carried(&locale("tlh")));
        assert_eq!(
            Rd(1).0,
            1,
            "the meta above is a value, not a calendar; nothing converts"
        );
    }

    #[test]
    fn templates_come_level_by_level_and_the_default_fills_the_rest() {
        let chain = templates(&locale("ja"), CalendarId("japanese"));
        assert_eq!(chain.entry.implied_era, "ad");
        assert_eq!(chain.locale.year, "{era}{year}年");
        assert_eq!(chain.locale.first_year, "{era}元年");
        assert_eq!(chain.merged().date, "{year}{month}{day}");
        assert!(chain.merged().implies_era("AD"));
        assert!(!chain.merged().implies_era("reiwa"));
        // The Chinese calendar's entry states the sexagenary year and the
        // day names; the locale's day suffix stands behind it.
        let chinese = templates(&locale("zh-Hans"), CalendarId("chinese"));
        assert_eq!(chinese.entry.year, "{sexagenary}年");
        assert_eq!(chinese.entry.day_names.len(), 30);
        assert_eq!(chinese.levels()[1].day, "{day}日");
        // A calendar nobody names gets the default outright.
        let haab = templates(&locale("ja"), CalendarId("maya-haab"));
        assert!(haab.entry.is_none() && haab.locale.is_none());
        assert_eq!(haab.levels()[2].year, DateTemplates::DEFAULT.year);
        assert_eq!(
            calendar_display_name(&locale("ja-JP"), CalendarId("japanese")),
            Some("和暦")
        );
        assert_eq!(
            calendar_display_name(&locale("de"), CalendarId("maya-haab")),
            None
        );
        assert_eq!(sexagenary_joiner(&locale("en")), " ");
        assert_eq!(sexagenary_joiner(&locale("ja")), "");
        assert_eq!(sexagenary_joiner(&locale("de")), "");
    }

    #[test]
    fn a_region_inherits_its_language_and_an_unknown_language_falls_back_to_root() {
        assert_eq!(
            gregorian_month("de-AT", 1, NameWidth::Wide, NameContext::Format),
            "Januar"
        );
        assert_eq!(
            gregorian_month(
                "de-CH-u-ca-gregory",
                1,
                NameWidth::Wide,
                NameContext::Format
            ),
            "Januar"
        );
        // Root month names are the CLDR root ones: M01 … M12.
        assert_eq!(
            gregorian_month("xx-YY", 1, NameWidth::Wide, NameContext::Format),
            "M01"
        );
        assert_eq!(locale_data(&locale("xx")).tag, "und");
    }

    #[test]
    fn weekday_names_exist_in_four_widths() {
        let english = locale("en");
        assert_eq!(
            weekday_name(
                &english,
                Weekday::Monday,
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("Monday")
        );
        assert_eq!(
            weekday_name(
                &english,
                Weekday::Monday,
                NameWidth::Abbreviated,
                NameContext::Format
            ),
            Some("Mon")
        );
        assert_eq!(
            weekday_name(
                &english,
                Weekday::Monday,
                NameWidth::Short,
                NameContext::Format
            ),
            Some("Mo")
        );
        assert_eq!(
            weekday_name(
                &english,
                Weekday::Monday,
                NameWidth::Narrow,
                NameContext::Format
            ),
            Some("M")
        );
        assert_eq!(
            weekday_name(
                &locale("ja"),
                Weekday::Sunday,
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("日曜日")
        );
        assert_eq!(
            weekday_name(
                &locale("ar"),
                Weekday::Friday,
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("الجمعة")
        );
    }

    #[test]
    fn day_periods_and_quarters_are_localised_too() {
        assert_eq!(
            day_period_name(
                &locale("en"),
                DayPeriod::Am,
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("AM")
        );
        assert_eq!(
            day_period_name(
                &locale("ja"),
                DayPeriod::Pm,
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("午後")
        );
        assert_eq!(DayPeriod::from_hour(0), DayPeriod::Am);
        assert_eq!(DayPeriod::from_hour(12), DayPeriod::Pm);
        assert_eq!(
            quarter_name(
                &locale("en"),
                CalendarId("gregory"),
                1,
                NameWidth::Abbreviated,
                NameContext::Format
            ),
            Some("Q1")
        );
        assert_eq!(
            quarter_name(
                &locale("ja"),
                CalendarId("gregory"),
                4,
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("第4四半期")
        );
        assert_eq!(
            quarter_name(
                &locale("en"),
                CalendarId("gregory"),
                0,
                NameWidth::Wide,
                NameContext::Format
            ),
            None
        );
    }

    #[test]
    fn eras_can_be_addressed_by_index_or_by_code() {
        let english = locale("en");
        assert_eq!(
            era_name(&english, CalendarId("gregory"), 1, NameWidth::Wide),
            Some("Anno Domini")
        );
        assert_eq!(
            era_name_by_code(
                &english,
                CalendarId("gregory"),
                "bc",
                NameWidth::Abbreviated
            ),
            Some("BC")
        );
        assert_eq!(
            era_name_by_code(&locale("ja"), CalendarId("gregory"), "ad", NameWidth::Wide),
            Some("西暦")
        );
        assert_eq!(
            era_codes(&english, CalendarId("gregory")),
            Some(["bc", "ad"].as_slice())
        );
        assert_eq!(
            era_name(&english, CalendarId("gregory"), 9, NameWidth::Wide),
            None
        );
    }

    #[test]
    fn the_japanese_calendar_has_its_own_eras_but_borrows_gregorian_months() {
        let japanese = locale("ja");
        let calendar = CalendarId("japanese");
        assert_eq!(
            era_name_by_code(&japanese, calendar, "reiwa", NameWidth::Wide),
            Some("令和")
        );
        assert_eq!(
            era_name_by_code(&japanese, calendar, "showa", NameWidth::Narrow),
            Some("S")
        );
        assert_eq!(
            era_name_by_code(&locale("en"), calendar, "reiwa", NameWidth::Wide),
            Some("Reiwa")
        );
        assert_eq!(
            month_name(
                &japanese,
                calendar,
                Month::regular(4),
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("4月")
        );
    }

    #[test]
    fn the_hijri_and_hebrew_calendars_have_their_own_months() {
        assert_eq!(
            month_name(
                &locale("ar"),
                CalendarId("islamic-civil"),
                Month::regular(9),
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("رمضان")
        );
        assert_eq!(
            month_name(
                &locale("en"),
                CalendarId("islamic-civil"),
                Month::regular(9),
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("Ramadan")
        );
        assert_eq!(
            month_name(
                &locale("he"),
                CalendarId("hebrew"),
                Month::regular(1),
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("תשרי")
        );
        // Month 6 is Adar, as the calendar numbers it; Adar I is the
        // intercalary repetition of month 5 and has a name of its own.
        assert_eq!(
            month_name(
                &locale("en"),
                CalendarId("hebrew"),
                Month::regular(6),
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("Adar")
        );
        assert_eq!(
            month_name(
                &locale("en"),
                CalendarId("hebrew"),
                Month::leap(5),
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("Adar I")
        );
        assert_eq!(
            month_count(&locale("en"), CalendarId("hebrew"), NameContext::Format),
            Some(12)
        );
    }

    #[test]
    fn the_old_japanese_month_names_are_kept_under_the_japanese_lunisolar_calendars() {
        assert_eq!(
            month_name(
                &locale("ja"),
                CalendarId("japanese-tenpo"),
                Month::regular(12),
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("師走")
        );
        assert_eq!(
            month_name(
                &locale("ja"),
                CalendarId("japanese-senmyo"),
                Month::regular(1),
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("睦月")
        );
    }

    #[test]
    fn an_intercalary_month_carries_the_locales_own_prefix() {
        let label = month_label(
            &locale("zh-Hans"),
            CalendarId("chinese"),
            Month::leap(4),
            NameWidth::Wide,
            NameContext::Format,
        )
        .unwrap();
        assert_eq!(label.prefix, "闰");
        assert_eq!(label.to_string(), "闰四月");

        let plain = month_label(
            &locale("zh-Hans"),
            CalendarId("chinese"),
            Month::regular(4),
            NameWidth::Wide,
            NameContext::Format,
        )
        .unwrap();
        assert_eq!(plain.prefix, "");
        assert_eq!(plain.to_string(), "四月");

        let english = month_label(
            &locale("en"),
            CalendarId("chinese"),
            Month::leap(4),
            NameWidth::Wide,
            NameContext::Format,
        )
        .unwrap();
        assert_eq!(english.to_string(), "leap Fourth Month");
    }

    #[test]
    fn the_sexagenary_cycle_is_written_in_each_locales_own_script() {
        let position = Sexagenary::from_index(0);
        assert_eq!(
            sexagenary_names(&locale("zh-Hans"), position),
            Some(("甲", "子"))
        );
        assert_eq!(
            sexagenary_names(&locale("ja"), position),
            Some(("甲", "子"))
        );
        assert_eq!(
            sexagenary_names(&locale("ko"), position),
            Some(("갑", "자"))
        );
        assert_eq!(
            sexagenary_names(&locale("vi"), position),
            Some(("Giáp", "Tý"))
        );
        assert_eq!(
            sexagenary_names(&locale("en"), position),
            Some(("jia", "zi"))
        );
        assert_eq!(sexagenary_names(&locale("ar"), position), None);
        assert_eq!(
            sexagenary_reading(&locale("ja-JP")).map(|reading| reading.id),
            Some("han")
        );
        assert_eq!(zodiac_animal_name(&locale("zh-Hans"), 0), Some("鼠"));
        assert_eq!(zodiac_animal_name(&locale("zh-Hant"), 4), Some("龍"));
        assert_eq!(zodiac_animal_name(&locale("ko"), 0), Some("쥐"));
        assert_eq!(zodiac_animal_name(&locale("en"), 0), Some("Rat"));
        assert_eq!(zodiac_animal_name(&locale("en"), 12), None);
        // Vietnam's fourth animal is the cat, not the rabbit, and its second
        // the buffalo, not the ox.
        assert_eq!(zodiac_animal_name(&locale("vi"), 3), Some("Mèo"));
        assert_eq!(zodiac_animal_name(&locale("vi"), 1), Some("Trâu"));
    }

    #[test]
    fn the_first_day_of_the_week_prefers_the_extension_then_the_region() {
        assert_eq!(first_day_of_week(&locale("en-US")), Weekday::Sunday);
        assert_eq!(first_day_of_week(&locale("en-GB")), Weekday::Monday);
        assert_eq!(first_day_of_week(&locale("ar-EG")), Weekday::Saturday);
        assert_eq!(first_day_of_week(&locale("ja-JP")), Weekday::Sunday);
        assert_eq!(
            first_day_of_week(&locale("en-US-u-fw-mon")),
            Weekday::Monday
        );
        // With no region, the language's data entry decides.
        assert_eq!(first_day_of_week(&locale("de")), Weekday::Monday);
    }

    #[test]
    fn widths_degrade_in_the_documented_order() {
        assert_eq!(NameWidth::Narrow.wider(), Some(NameWidth::Abbreviated));
        assert_eq!(NameWidth::Short.wider(), Some(NameWidth::Abbreviated));
        assert_eq!(NameWidth::Abbreviated.wider(), Some(NameWidth::Wide));
        assert_eq!(NameWidth::Wide.wider(), None);

        let set = WidthSet {
            wide: &["wide"],
            abbreviated: &[],
            short: &[],
            narrow: &[],
        };
        assert_eq!(set.get(NameWidth::Narrow), &["wide"]);
        assert_eq!(set.exact(NameWidth::Narrow), &[] as &[&str]);
        assert!(WidthSet::EMPTY.is_empty());
        assert!(WidthSet::EMPTY.get(NameWidth::Wide).is_empty());
        assert!(ContextualNames::EMPTY.is_empty());
        assert!(!ContextualNames::same(set).is_empty());
        assert!(
            CalendarNames::empty(&[CalendarId("gregory")])
                .months()
                .is_empty()
        );
        assert_eq!(EraNames::EMPTY.index_of("ad"), None);
        assert!(SexagenaryNames::EMPTY.reading.is_none());
    }

    #[test]
    fn an_unknown_calendar_has_no_names_at_all() {
        assert_eq!(
            month_name(
                &locale("en"),
                CalendarId("mayan"),
                Month::regular(1),
                NameWidth::Wide,
                NameContext::Format
            ),
            None
        );
        assert_eq!(
            month_count(&locale("en"), CalendarId("mayan"), NameContext::Format),
            None
        );
        assert_eq!(era_codes(&locale("en"), CalendarId("mayan")), None);
        assert!(
            locale_data(&locale("en"))
                .calendar(CalendarId("mayan"))
                .is_none()
        );
    }
}
