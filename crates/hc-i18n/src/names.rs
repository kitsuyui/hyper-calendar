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
use hc_calendar::{CalendarId, Month, Weekday};

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
    #[must_use]
    pub fn index_of(&self, code: &str) -> Option<usize> {
        self.codes.iter().position(|candidate| *candidate == code)
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
    /// Era names.
    pub eras: EraNames,
    /// Quarter names, indexed from zero for quarter 1.
    pub quarters: ContextualNames,
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
            eras: EraNames::EMPTY,
            quarters: ContextualNames::EMPTY,
        }
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
}

impl SexagenaryNames {
    /// No cycle names.
    pub const EMPTY: Self = Self {
        reading: None,
        zodiac: None,
    };
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
/// has no vocabulary for.
#[must_use]
pub fn month_label(
    locale: &Locale,
    calendar: CalendarId,
    month: Month,
    width: NameWidth,
    context: NameContext,
) -> Option<MonthLabel> {
    let index = usize::from(month.ordinal).checked_sub(1)?;
    resolve(locale, |data| {
        let names = data.cycle_for(calendar, hc_calendar::shape::MONTH)?;
        let name = names.get(width, context).get(index).copied()?;
        // The prefix belongs to whichever entry carries the months, since
        // that is the entry that knows how the calendar writes a leap one.
        let prefix = if month.leap {
            data.entries_for(calendar)
                .find(|entry| !entry.months().is_empty())
                .map_or("", |entry| entry.leap_month_prefix)
        } else {
            ""
        };
        Some(MonthLabel { prefix, name })
    })
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
/// adjacent (甲子) while a romanising locale wants a separator.
#[must_use]
pub fn sexagenary_names(
    locale: &Locale,
    position: Sexagenary,
) -> Option<(&'static str, &'static str)> {
    sexagenary_reading(locale).map(|reading| reading.pair(position))
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
        assert_eq!(
            month_name(
                &locale("en"),
                CalendarId("hebrew"),
                Month::regular(6),
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("Adar I")
        );
        assert_eq!(
            month_count(&locale("en"), CalendarId("hebrew"), NameContext::Format),
            Some(13)
        );
    }

    #[test]
    fn the_old_japanese_month_names_are_kept_under_the_lunisolar_calendar() {
        assert_eq!(
            month_name(
                &locale("ja"),
                CalendarId("chinese"),
                Month::regular(12),
                NameWidth::Wide,
                NameContext::Format
            ),
            Some("師走")
        );
        assert_eq!(
            month_name(
                &locale("ja"),
                CalendarId("chinese"),
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
