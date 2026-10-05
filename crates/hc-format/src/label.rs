//! A calendar's eras, years, months, days and dates, written the way a
//! locale writes them.
//!
//! # What this is for
//!
//! A timeline lane needs a word for each box it draws — 令和元年, *Adar I*,
//! 閏二月, 初四 — and a date needs to read as one in the language it is
//! shown in: 令和8年9月21日, 2023癸卯年闰二月初一, *September 21, 2026*. The
//! vocabulary is `hc-i18n`'s; this module is the renderer that assembles
//! it, for any [`DynCalendar`], from the templates the locale states.
//!
//! # How a label is found
//!
//! Every locale states how it writes a year with its era, a day of the
//! month and a whole date ([`hc_i18n::names::DateTemplates`]), and an
//! entry that serves a calendar family states what differs for that
//! family: the Chinese calendar's year by its stem and branch, its days by
//! their Han names. A calendar written in a notation of its own whatever
//! the language, the Long Count's `13.0.13.17.8`, states it in
//! [`hc_i18n::notation`]. A template is tried level by level — the
//! family's, the calendar's notation, the locale's, then
//! [`DateTemplates::DEFAULT`] — and a level whose every placeholder comes
//! out empty is passed over, so that `{sexagenary}年` yields to
//! `{era}{year}年` for a calendar with no sexagenary count.
//!
//! # The extra fields
//!
//! A date's extra fields ([`hc_calendar::fields::ExtraFields`]) are written only
//! where a template names one, `{extra:samvatsara}`, which a template does
//! only where the calendar's sources write the date with it. There is no
//! way to write them all: their identifiers are keys, and a raw
//! `name=value` pair is not something a reader can read. The rest are
//! metadata, which [`extra`] writes one at a time, as the value a
//! template would write, for the lines that list them beside the date;
//! [`date_marking`] says which of them the date itself wrote, a field
//! whose value the date writes by the same name in the calendar's own
//! words included: the Burmese phase *waning* is the half *waning*.
//!
//! The pieces come from the same fallbacks the rest of the workspace uses:
//! an era's name is the locale's, else the calendar's own
//! ([`DynCalendar::era_name`], romanised for a Latin-script locale), else
//! English's, else nothing — never its code, by
//! [`hc_i18n::names::era_label`]; a month's is the locale's, else the calendar's own shape name,
//! else its number; a day's is the locale's day name where it has one and
//! its number otherwise. A number is written in the locale's numbering
//! system, or in the one a template names, `{year:hans}`, where a
//! standard writes a calendar's years in Han numerals and the locale's
//! other dates in Latin digits. Nothing here invents an orthography: a locale that has stated
//! no template gets its fields in [`hc_calendar::DateFields`] order,
//! separated by spaces.
//!
//! # Reading a date back
//!
//! [`parse_date`] is the inverse: it walks the same templates, tries at
//! each placeholder every name the locale's chain has at every width and
//! numbers in the locale's digits, Latin digits and, for a locale written
//! in Han characters, Han numerals, and keeps a reading only where the
//! calendar's own fields for the day agree with the text. A text that is
//! not one day is refused with a [`DateRefusal`] rather than guessed at.
//! `docs/systems/written-dates.md` in the repository explains the
//! matching and each refusal.
//!
//! # Which locale
//!
//! [`locale_for`] answers that, with one rule for every rendered cell: the
//! one asked for when it names the calendar, else English, else the one
//! asked for with the calendar's own names from its shape. A named locale
//! never borrows the calendar's own language. A caller that wants each
//! calendar in its own language passes `None`, which reaches for it first,
//! then English.

use core::fmt::{self, Write};

use hc_calendar::cycle::Sexagenary;
use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::units::Unit;
use hc_calendar::{CalendarId, DateFields, DynCalendar, Month};
use hc_i18n::Locale;
use hc_i18n::fields;
use hc_i18n::names::{self, DateTemplates, NameContext, NameWidth, TemplateChain};
use hc_i18n::numbering::{self, NumberingSystem};

mod read;

pub use read::{DateRefusal, ParsedDate, parse_date};

/// The locale a calendar is rendered in: the one asked for when it names
/// the calendar and every name a date of the calendar writes, else English,
/// else the one asked for with the calendar's own names; only `None` asks
/// for the calendar's own language first.
///
/// A date is in one language. The locale names a month from its own data
/// and an era from its own data or CLDR's root, whose abbreviations (`AH`,
/// `BE`) belong to every language; the calendar's own name for a month or
/// an era is written in the locale's script (romanised for a Latin one, in
/// Han or Hangul for a Chinese, Japanese or Korean one) or not at all. A
/// locale that cannot write each of them leaves the whole date to English,
/// whose tag is then the locale used, and never takes a name from another
/// language into its own date.
///
/// See [`hc_i18n::names::locale_for_calendar_with`], which this wraps.
#[must_use]
pub fn locale_for(calendar: &dyn DynCalendar, requested: Option<&Locale>) -> Locale {
    names::locale_for_calendar_with(requested, &calendar.meta(), |locale| {
        writes_in_one_language_cached(calendar, locale)
    })
}

/// [`writes_in_one_language`], remembered: it asks for every era of the
/// calendar, which is some five hundred names for the Japanese ones, and
/// is a function of the calendar and the locale alone, the data being
/// static. The memory is bounded and is emptied when it fills.
#[cfg(feature = "std")]
fn writes_in_one_language_cached(calendar: &dyn DynCalendar, locale: &Locale) -> bool {
    use std::cell::RefCell;
    use std::collections::HashMap;
    /// The most answers kept: the registry's calendars in a few hundred
    /// locales.
    const MOST: usize = 1 << 16;
    std::thread_local! {
        static ANSWERS: RefCell<HashMap<(&'static str, Locale), bool>> =
            RefCell::new(HashMap::new());
    }
    let key = (calendar.meta().id.as_str(), *locale);
    if let Some(known) = ANSWERS.with(|answers| answers.borrow().get(&key).copied()) {
        return known;
    }
    let answer = writes_in_one_language(calendar, locale);
    ANSWERS.with(|answers| {
        let mut answers = answers.borrow_mut();
        if answers.len() >= MOST {
            answers.clear();
        }
        answers.insert(key, answer);
    });
    answer
}

#[cfg(not(feature = "std"))]
fn writes_in_one_language_cached(calendar: &dyn DynCalendar, locale: &Locale) -> bool {
    writes_in_one_language(calendar, locale)
}

/// Whether every month and every era a date of the calendar writes in
/// `locale` is a name of the locale's own, as [`locale_for`] asks.
fn writes_in_one_language(calendar: &dyn DynCalendar, locale: &Locale) -> bool {
    let id = calendar.meta().id;
    for cycle in calendar.cycles() {
        if !locale_names_cycle(locale, id, cycle) {
            return false;
        }
    }
    let english = names::english();
    let native = names::writes_native_names(locale);
    let writes = |code: &str| {
        // A name at the wide width is one at the abbreviated, which
        // degrades to it, so the wide is asked.
        if names::era_name_by_code(locale, id, code, NameWidth::Wide).is_some() {
            return true;
        }
        match calendar.era_name(code) {
            // The name the renderer writes, romanised unless the locale
            // writes Han characters, kana or Hangul, where the calendar
            // gives a romanisation; an era whose romanisation another era
            // shares is written by its characters or, where English names
            // it, by English's name, root's, the same word in every
            // language, which no check of a script can refuse.
            Some(own) if !native && !own.romanised.is_empty() => true,
            Some(own) => own_name_fits(locale, if native { own.native } else { own.latin() }),
            None => {
                locale.language() == "en"
                    || names::era_name_by_code(&english, id, code, NameWidth::Wide).is_none()
            }
        }
    };
    // The eras a date may write are the calendar's own table and the ones
    // English names, which the facade's vocabulary test holds to being
    // every era code a calendar writes.
    let mut every = true;
    for index in 0.. {
        let Some(code) = calendar.era_code(index) else {
            break;
        };
        if !writes(code) {
            return false;
        }
    }
    names::for_each_era_code(&english, id, |code| {
        every = every && (calendar.era_name(code).is_some() || writes(code));
    });
    every
}

/// Whether a date writes the positions of one of the calendar's cycles in
/// the locale's language: from its data, else the calendar's own names in
/// the locale's script, else by number. The weekdays are the locale's own
/// by their own lookup, and CLDR's root names a Gregorian month `M09`,
/// which no reader takes for a name.
fn locale_names_cycle(locale: &Locale, calendar: CalendarId, cycle: &CycleShape) -> bool {
    if cycle.kind == WEEKDAY || names::locale_names_cycle(locale, calendar, cycle.kind) {
        return true;
    }
    if cycle.names.is_empty() {
        if cycle.kind != MONTH {
            return true;
        }
        let first = Month {
            ordinal: 1,
            leap: false,
        };
        return names::month_label(
            locale,
            calendar,
            first,
            NameWidth::Wide,
            NameContext::Format,
        )
        .is_none();
    }
    cycle.names.iter().all(|name| own_name_fits(locale, name))
}

/// Whether a name of the calendar's own may stand in a date of the locale:
/// in the locale's script, or romanised, a proper name no locale translates,
/// as CLDR's root writes them, except in a date of Han characters, kana or
/// Hangul, which takes no Latin letters.
fn own_name_fits(locale: &Locale, name: &str) -> bool {
    in_script_of(locale, name)
        || (!names::writes_native_names(locale)
            && name
                .chars()
                .filter(|letter| letter.is_alphabetic())
                .all(|letter| script_of(letter) == "Latn"))
}

/// Whether the letters of `text` are written in the script of the locale:
/// the one its data entry names, Han characters, kana and Hangul counting
/// as one for a Chinese, Japanese or Korean locale. Text with no letters is
/// in every script.
fn in_script_of(locale: &Locale, text: &str) -> bool {
    let script = match names::locale_data(locale).script {
        "Jpan" | "Hans" | "Hant" | "Kore" => "Hani",
        other => other,
    };
    text.chars()
        .filter(|letter| letter.is_alphabetic())
        .all(|letter| script_of(letter) == script)
}

/// The script a letter is written in, by the blocks the carried locales'
/// scripts use: ISO 15924 codes, `Hani` for Han, kana and Hangul together,
/// and `Zzzz` for any other.
fn script_of(letter: char) -> &'static str {
    match u32::from(letter) {
        0x41..=0x2FF | 0x1E00..=0x1EFF => "Latn",
        0x370..=0x3FF => "Grek",
        0x400..=0x52F => "Cyrl",
        0x590..=0x5FF | 0xFB1D..=0xFB4F => "Hebr",
        0x600..=0x6FF | 0x750..=0x77F | 0xFB50..=0xFDFF | 0xFE70..=0xFEFF => "Arab",
        0x700..=0x74F => "Syrc",
        0x840..=0x85F => "Mand",
        0x900..=0x97F => "Deva",
        0x980..=0x9FF => "Beng",
        0xA00..=0xA7F => "Guru",
        0xB80..=0xBFF => "Taml",
        0xC00..=0xC7F => "Telu",
        0xD00..=0xD7F => "Mlym",
        0xE00..=0xE7F => "Thai",
        0xF00..=0xFFF => "Tibt",
        0x1000..=0x109F => "Mymr",
        0x1200..=0x139F => "Ethi",
        0x1100..=0x11FF
        | 0x3040..=0x30FF
        | 0x3400..=0x4DBF
        | 0x4E00..=0x9FFF
        | 0xAC00..=0xD7AF
        | 0xF900..=0xFAFF
        | 0x20000..=0x2FA1F => "Hani",
        0x2C80..=0x2CFF => "Copt",
        0x2D30..=0x2D7F => "Tfng",
        _ => "Zzzz",
    }
}

/// An era's name as a date writes it: [`hc_i18n::names::era_label`], the
/// locale's name, else the calendar's own, else English's — except that a
/// romanisation the calendar gives another of its eras too is not written
/// in a Latin-script locale, since the text would name both. The nengō
/// 延慶 and 延享 are both *Enkyo* in the calendar's own table; English's
/// names, CLDR root's, tell them apart, *Enkyō (1308–1311)* and *Enkyō
/// (1744–1748)*, and where English has none, as for the Northern court's
/// 貞和 beside 承和, both *Jowa*, the era is written by its native name.
#[must_use]
pub fn era_label(
    calendar: &dyn DynCalendar,
    locale: &Locale,
    code: &str,
    width: NameWidth,
) -> Option<&'static str> {
    let id = calendar.meta().id;
    let own = calendar.era_name(code).and_then(|own| {
        if own.romanised.is_empty()
            || names::writes_native_names(locale)
            || !shares_romanisation(calendar, code, own.romanised)
        {
            Some(own)
        } else if names::era_name_by_code(&names::english(), id, code, width).is_some() {
            None
        } else {
            Some(hc_calendar::shape::EraName::new(own.native, ""))
        }
    });
    names::era_label(locale, id, code, own, width)
}

/// Whether another of the calendar's eras has the romanisation `name`,
/// in either case.
fn shares_romanisation(calendar: &dyn DynCalendar, code: &str, name: &str) -> bool {
    (0..)
        .map_while(|index| calendar.era_code(index))
        .filter(|other| *other != code)
        .filter_map(|other| calendar.era_name(other))
        .any(|other| other.romanised.eq_ignore_ascii_case(name))
}

/// Write the label of one unit of a date: its era, its year with the era,
/// its month, or its day.
///
/// # Errors
///
/// Only what the sink returns.
pub fn write_label<W: Write>(
    calendar: &dyn DynCalendar,
    fields: &DateFields,
    unit: Unit,
    locale: &Locale,
    out: &mut W,
) -> fmt::Result {
    let renderer = Renderer::new(calendar, fields, locale);
    let mut collapse = Collapse::new(out);
    renderer.write_unit(unit, &mut collapse)
}

/// Write the whole date as the locale writes one.
///
/// # Errors
///
/// Only what the sink returns.
pub fn write_date<W: Write>(
    calendar: &dyn DynCalendar,
    fields: &DateFields,
    locale: &Locale,
    out: &mut W,
) -> fmt::Result {
    write_date_marking(calendar, fields, locale, out).map(|_| ())
}

/// Write the whole date as the locale writes one, and say which of the
/// date's extra fields it wrote.
///
/// # Errors
///
/// Only what the sink returns.
pub fn write_date_marking<W: Write>(
    calendar: &dyn DynCalendar,
    fields: &DateFields,
    locale: &Locale,
    out: &mut W,
) -> Result<ExtrasWritten, fmt::Error> {
    let renderer = Renderer::new(calendar, fields, locale);
    if renderer.merged.omitted_thousands != 0 && !renderer.reads_back() {
        renderer.year_in_digits.set(true);
    }
    let mut collapse = Collapse::new(out);
    renderer.write_levels(|templates| templates.date, Mode::Date, &mut collapse)?;
    Ok(renderer.written_with_shared_names())
}

/// Write one extra field's value as a template's `{extra:FIELD}` writes
/// it: the name of the position it holds in the cycle it counts, where
/// that is named ([`hc_i18n::fields::write_value_name`]), else the number in
/// the locale's numbering system. Nothing for a field the date does not
/// carry.
///
/// # Errors
///
/// Only what the sink returns.
pub fn write_extra<W: Write>(
    calendar: &dyn DynCalendar,
    fields: &DateFields,
    field: &str,
    locale: &Locale,
    out: &mut W,
) -> fmt::Result {
    let renderer = Renderer::new(calendar, fields, locale);
    renderer.write_extra(field, NameWidth::Wide, NameContext::Standalone, out)
}

/// Which of a date's extra fields its formatted text writes, by their
/// place in [`hc_calendar::fields::ExtraFields::iter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExtrasWritten(u32);

impl ExtrasWritten {
    /// Whether the extra field at `index` of [`hc_calendar::fields::ExtraFields::iter`]
    /// is written in the date.
    #[must_use]
    pub const fn contains(self, index: usize) -> bool {
        index < 32 && self.0 & (1 << index) != 0
    }

    /// Whether the date writes no extra field.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// The label of one unit of a date, as a string.
#[cfg(feature = "alloc")]
#[must_use]
pub fn label(
    calendar: &dyn DynCalendar,
    fields: &DateFields,
    unit: Unit,
    locale: &Locale,
) -> alloc::string::String {
    let mut out = alloc::string::String::new();
    // A `String` never refuses a write.
    let _ = write_label(calendar, fields, unit, locale, &mut out);
    out
}

/// The whole date, as a string.
#[cfg(feature = "alloc")]
#[must_use]
pub fn date(
    calendar: &dyn DynCalendar,
    fields: &DateFields,
    locale: &Locale,
) -> alloc::string::String {
    date_marking(calendar, fields, locale).0
}

/// The whole date, as a string, and which of its extra fields it wrote.
#[cfg(feature = "alloc")]
#[must_use]
pub fn date_marking(
    calendar: &dyn DynCalendar,
    fields: &DateFields,
    locale: &Locale,
) -> (alloc::string::String, ExtrasWritten) {
    let mut out = alloc::string::String::new();
    // A `String` never refuses a write.
    let written = write_date_marking(calendar, fields, locale, &mut out).unwrap_or_default();
    (out, written)
}

/// One extra field's value, as a string: see [`write_extra`].
#[cfg(feature = "alloc")]
#[must_use]
pub fn extra(
    calendar: &dyn DynCalendar,
    fields: &DateFields,
    field: &str,
    locale: &Locale,
) -> alloc::string::String {
    let mut out = alloc::string::String::new();
    // A `String` never refuses a write.
    let _ = write_extra(calendar, fields, field, locale, &mut out);
    out
}

/// What the placeholders of a template stand for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// A unit's own template: `{year}` is the number, `{era}` the era's
    /// name unless the locale implies it.
    Unit,
    /// The era template: `{era}` is always written, and wide.
    Era,
    /// The date template: `{year}`, `{month}` and `{day}` are the rendered
    /// units.
    Date,
}

struct Renderer<'a> {
    calendar: &'a dyn DynCalendar,
    id: CalendarId,
    fields: &'a DateFields,
    locale: &'a Locale,
    chain: TemplateChain,
    merged: DateTemplates,
    numbering: &'static NumberingSystem,
    /// Whether the year has the calendar's intercalary month, which some
    /// month names depend on — the Adar of a Hebrew leap year is Adar II —
    /// found the first time a month asks, because asking an astronomical
    /// calendar costs as much as a conversion and most months never need
    /// to know.
    in_leap_year: core::cell::OnceCell<bool>,
    /// The extra fields a placeholder has written, by their place in
    /// [`hc_calendar::fields::ExtraFields::iter`]. A level passed over wrote no
    /// placeholder at all, so whatever is marked is in the text.
    written: core::cell::Cell<u32>,
    /// Whether a year that [`Renderer::write_in`] would write in numerals
    /// is written in digits instead, because the whole date in numerals
    /// does not read back as the day ([`Renderer::reads_back`]).
    year_in_digits: core::cell::Cell<bool>,
}

impl<'a> Renderer<'a> {
    fn new(calendar: &'a dyn DynCalendar, fields: &'a DateFields, locale: &'a Locale) -> Self {
        let id = calendar.meta().id;
        let chain = names::templates(locale, id);
        Self {
            calendar,
            id,
            fields,
            locale,
            chain,
            merged: chain.merged(),
            numbering: NumberingSystem::for_locale(locale),
            in_leap_year: core::cell::OnceCell::new(),
            written: core::cell::Cell::new(0),
            year_in_digits: core::cell::Cell::new(false),
        }
    }

    /// Whether the whole date, written with its year in numerals, reads
    /// back as the day the fields name: the check a date whose templates
    /// let a year's thousands go unwritten needs, since its numerals and
    /// the text around them may read as another day. A text too long to
    /// hold, or fields that are no day, count as not reading back.
    fn reads_back(&self) -> bool {
        let Ok(fixed) = self.calendar.fields_to_fixed(self.fields) else {
            return false;
        };
        let mut text = Buffer::<256>::default();
        let mut collapse = Collapse::new(&mut text);
        let written = self.write_levels(|templates| templates.date, Mode::Date, &mut collapse);
        self.written.set(0);
        written.is_ok()
            && text.as_str().is_some_and(|text| {
                parse_date(self.calendar, self.locale, text).is_ok_and(|read| read.fixed == fixed)
            })
    }

    fn in_leap_year(&self) -> bool {
        *self.in_leap_year.get_or_init(|| {
            self.calendar
                .has_intercalary_month_of(self.fields)
                .unwrap_or(false)
        })
    }

    fn write_unit(&self, unit: Unit, out: &mut dyn Write) -> fmt::Result {
        match unit {
            Unit::Era => self.write_levels(|templates| templates.era, Mode::Era, out),
            Unit::Year => {
                if self.fields.era.is_some()
                    && self.fields.year == 1
                    && self.write_levels(|templates| templates.first_year, Mode::Unit, out)?
                {
                    return Ok(());
                }
                self.write_levels(|templates| templates.year, Mode::Unit, out)
            }
            Unit::Month => self.write_levels(|templates| templates.month, Mode::Unit, out),
            // A day of a calendar that has no day of the month — a day
            // count, the Long Count — is labelled by the whole date, which
            // is the only name the day has.
            Unit::Day if self.fields.day.is_none() => {
                self.write_levels(|templates| templates.date, Mode::Date, out)
            }
            Unit::Day if let Some(name) = self.leap_day_name() => {
                out.write_str(name).map(|()| true)
            }
            Unit::Day => self.write_levels(|templates| templates.day, Mode::Unit, out),
        }
        .map(|_| ())
    }

    /// The name the date's day is written by, where it is a leap day the
    /// locale names ([`names::leap_day_name`]): St. Tib's Day.
    fn leap_day_name(&self) -> Option<&'static str> {
        let named = names::leap_day_name(self.locale, self.id)?;
        (self.fields.leap_day
            && self.fields.month.map(|month| month.ordinal) == Some(named.month)
            && self.fields.day == Some(named.day))
        .then_some(named.name)
    }

    /// Render the first level whose template for a field says something —
    /// has a placeholder that came out non-empty — reporting whether any
    /// did. The dry run into a counting sink is what tells.
    fn write_levels(
        &self,
        field: impl Fn(&DateTemplates) -> &'static str,
        mode: Mode,
        out: &mut dyn Write,
    ) -> Result<bool, fmt::Error> {
        for level in self.chain.levels() {
            let template = field(&level);
            if template.is_empty() {
                continue;
            }
            if self.write_template(template, mode, &mut Counter)? > 0 {
                self.write_template(template, mode, out)?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Render one template, writing its literal text as it is and each
    /// placeholder as the field it names, and count the placeholders that
    /// wrote something.
    fn write_template(
        &self,
        template: &str,
        mode: Mode,
        out: &mut dyn Write,
    ) -> Result<usize, fmt::Error> {
        let mut filled_count = 0;
        let mut rest = template;
        while let Some(start) = rest.find('{') {
            out.write_str(&rest[..start])?;
            let after = &rest[start + 1..];
            let Some(end) = after.find('}') else {
                out.write_str(&rest[start..])?;
                return Ok(filled_count);
            };
            let inside = &after[..end];
            let (name, field, spec) = parse_placeholder(inside);
            let mut fill = Fill { out, filled: false };
            match (name, spec) {
                ("extra", Spec::Digits(digits)) => {
                    if let Some(value) = self.fields.extra.get(field) {
                        self.mark(field);
                        self.write_padded(value, digits, &mut fill)?;
                    }
                }
                ("extra", spec) => {
                    let context = match mode {
                        Mode::Date => NameContext::Format,
                        Mode::Unit | Mode::Era => NameContext::Standalone,
                    };
                    self.write_extra(field, spec.width(), context, &mut fill)?;
                }
                (_, Spec::Digits(digits)) => self.write_digits(name, digits, &mut fill)?,
                (_, Spec::Numbering(system)) => self.write_in(name, system, &mut fill)?,
                ("day", Spec::ZeroBased) => {
                    if let Some(day) = self.fields.day {
                        self.write_number(i64::from(day) - 1, &mut fill)?;
                    }
                }
                (_, spec) => {
                    let width = match spec {
                        Spec::Width(width) => Some(width),
                        _ => None,
                    };
                    self.write_placeholder(name, width, mode, &mut fill)?;
                }
            }
            if fill.filled {
                filled_count += 1;
            }
            rest = &after[end + 1..];
        }
        out.write_str(rest)?;
        Ok(filled_count)
    }

    fn write_placeholder(
        &self,
        name: &str,
        width: Option<NameWidth>,
        mode: Mode,
        out: &mut dyn Write,
    ) -> fmt::Result {
        match (name, mode) {
            // The year's unit may begin or end with the space an empty
            // era leaves, which a date whose own text sits against the
            // year, 2026 after a Tibetan སྤྱི་ལོ་, does not want.
            ("year", Mode::Date) => {
                let mut sink = out;
                self.write_unit(Unit::Year, &mut Collapse::new(&mut sink))
            }
            // A leap day the locale names is written by its name in the
            // day's place, which stands for the month too.
            ("month", Mode::Date) if self.leap_day_name().is_some() => Ok(()),
            // Inside a date the month is in its format context — сентября,
            // not сентябрь — and at the width the template asks for, since
            // `y年M月d日` wants the numbered form.
            ("month", Mode::Date) => match self.fields.month {
                Some(month) => self.write_month(
                    month,
                    width.unwrap_or(NameWidth::Wide),
                    NameContext::Format,
                    out,
                ),
                None => Ok(()),
            },
            ("day", Mode::Date) if let Some(name) = self.leap_day_name() => out.write_str(name),
            // An absent day writes nothing here: the Day unit of a calendar
            // without days is the date, and the date must not ask for it back.
            ("day", Mode::Date) => match self.fields.day {
                Some(_) => self.write_unit(Unit::Day, out),
                None => Ok(()),
            },
            ("era", _) => {
                let Some(code) = self.fields.era else {
                    return Ok(());
                };
                if mode != Mode::Era && self.merged.implies_era(code) {
                    return Ok(());
                }
                let width = width.unwrap_or(match mode {
                    Mode::Era => NameWidth::Wide,
                    Mode::Unit | Mode::Date => NameWidth::Abbreviated,
                });
                match era_label(self.calendar, self.locale, code, width) {
                    Some(name) => out.write_str(name),
                    None => Ok(()),
                }
            }
            ("year", _) => self.write_number(self.fields.year, out),
            ("sexagenary", _) => {
                let Some(index) = self.fields.extra.get("sexagenary_year") else {
                    return Ok(());
                };
                let position = Sexagenary::from_index(index);
                let Some((stem, branch)) = names::sexagenary_names(self.locale, position) else {
                    return Ok(());
                };
                self.mark("sexagenary_year");
                out.write_str(stem)?;
                out.write_str(names::sexagenary_joiner(self.locale))?;
                out.write_str(branch)
            }
            ("month", _) => match self.fields.month {
                Some(month) => self.write_month(
                    month,
                    width.unwrap_or(NameWidth::Wide),
                    NameContext::Standalone,
                    out,
                ),
                None => Ok(()),
            },
            ("day", _) => match self.fields.day {
                Some(day) => {
                    let context = match mode {
                        Mode::Date => NameContext::Format,
                        Mode::Unit | Mode::Era => NameContext::Standalone,
                    };
                    self.write_day(day, width.unwrap_or(NameWidth::Wide), context, out)
                }
                None => Ok(()),
            },
            _ => Ok(()),
        }
    }

    /// A day of the month by the locale's name for it, else by its name
    /// in a named cycle — the Pawukon's seven-day week — else by its
    /// number.
    fn write_day(
        &self,
        day: u8,
        width: NameWidth,
        context: NameContext,
        out: &mut dyn Write,
    ) -> fmt::Result {
        let named = usize::from(day)
            .checked_sub(1)
            .and_then(|index| self.merged.day_names.get(index));
        if let Some(name) = named {
            return out.write_str(name);
        }
        if fields::write_value_name(
            self.locale,
            self.id,
            self.calendar.cycles(),
            "day",
            i64::from(day),
            width,
            context,
            out,
        )? {
            return Ok(());
        }
        self.write_number(i64::from(day), out)
    }

    /// An extra field, where the date carries it: the name of the position
    /// it holds where its cycle is named, else its number; and marked as
    /// written.
    fn write_extra(
        &self,
        field: &str,
        width: NameWidth,
        context: NameContext,
        out: &mut dyn Write,
    ) -> fmt::Result {
        let Some(value) = self.fields.extra.get(field) else {
            return Ok(());
        };
        self.mark(field);
        self.write_extra_value(field, value, width, context, out)
    }

    /// An extra field's value: the name of the position it holds where
    /// its cycle is named, else its number.
    fn write_extra_value(
        &self,
        field: &str,
        value: i64,
        width: NameWidth,
        context: NameContext,
        out: &mut dyn Write,
    ) -> fmt::Result {
        if fields::write_value_name(
            self.locale,
            self.id,
            self.calendar.cycles(),
            field,
            value,
            width,
            context,
            out,
        )? {
            return Ok(());
        }
        self.write_number(value, out)
    }

    /// The extra fields the date wrote, and with them each field whose
    /// value has the same name in the calendar's own words
    /// ([`fields::own_value_name`]) as a field it wrote: the Burmese phase
    /// *waning* is written by the half of the month *waning*, and the full
    /// moon, *waxing* 15, is not.
    fn written_with_shared_names(&self) -> ExtrasWritten {
        let written = self.written.get();
        let name = |index: usize| {
            let extra = self.fields.extra.iter().nth(index)?;
            fields::own_value_name(self.locale, self.id, extra.name, extra.value)
        };
        let mut shared = written;
        for index in 0..self.fields.extra.len().min(32) {
            if written & 1 << index != 0 {
                continue;
            }
            if let Some(own) = name(index)
                && (0..self.fields.extra.len().min(32))
                    .any(|other| written & 1 << other != 0 && name(other) == Some(own))
            {
                shared |= 1 << index;
            }
        }
        ExtrasWritten(shared)
    }

    /// Note that the date's text holds an extra field.
    fn mark(&self, field: &str) {
        if let Some(index) = self
            .fields
            .extra
            .iter()
            .position(|extra| extra.name == field)
            .filter(|index| *index < 32)
        {
            self.written.set(self.written.get() | 1 << index);
        }
    }

    /// A field as a number of at least `digits` digits, never a name: the
    /// year without its era, the month's ordinal, the day of the month.
    fn write_digits(&self, name: &str, digits: u8, out: &mut dyn Write) -> fmt::Result {
        match self.number_of(name) {
            Some(value) => self.write_padded(value, digits, out),
            None => Ok(()),
        }
    }

    /// The year, the month's number or the day, as a placeholder names it.
    fn number_of(&self, name: &str) -> Option<i64> {
        match name {
            "year" => Some(self.fields.year),
            "month" => self.fields.month.map(|month| i64::from(month.ordinal)),
            "day" => self.fields.day.map(i64::from),
            _ => None,
        }
    }

    /// The year, the month's number or the day in the numbering system a
    /// template names, `{year:hans}`, whatever the locale's own: a
    /// regnal year that a standard writes in Han numerals in a locale
    /// that writes its other dates in Latin digits. A value the system
    /// cannot hold is written as [`Renderer::write_number`] writes it.
    ///
    /// A year of a date whose templates let the thousands go unwritten,
    /// [`DateTemplates::omitted_thousands`], is written in the system only
    /// where its numerals read back as it: ה׳תשפ״ז, and not א׳ for the year
    /// 1 or ה׳ for 5000, which a reader takes for 5001 and 5005. A whole
    /// date is written so only where it reads back as its day too
    /// ([`Renderer::reads_back`]).
    fn write_in(&self, name: &str, system: &NumberingSystem, out: &mut dyn Write) -> fmt::Result {
        let Some(value) = self.number_of(name) else {
            return Ok(());
        };
        let reads_back = || {
            let mut written = Buffer::<64>::default();
            system.write_integer(value, &mut written).is_ok()
                && value >= 1_000
                && written
                    .as_str()
                    .and_then(|text| system.parse_integer(text).ok())
                    == Some(value)
        };
        let whole = name != "year"
            || self.merged.omitted_thousands == 0
            || !system.is_algorithmic()
            || (!self.year_in_digits.get() && reads_back());
        if whole && system.write_integer(value, &mut Counter).is_ok() {
            let mut out = Fill { out, filled: false };
            return system
                .write_integer(value, &mut out)
                .map_err(|_| fmt::Error);
        }
        self.write_number(value, out)
    }

    /// A number of at least `digits` digits, padded with the zero of the
    /// locale's numbering system, or of Latin digits where that system
    /// has no digits of its own to pad with.
    fn write_padded(&self, value: i64, digits: u8, out: &mut dyn Write) -> fmt::Result {
        let zeros: &[char; 10] = match self.numbering.digits() {
            Some(zeros) => zeros,
            None => match numbering::LATN.digits() {
                Some(zeros) => zeros,
                None => return self.write_number(value, out),
            },
        };
        // Least significant first; a u64 has at most twenty digits.
        let mut decimal = [0_u8; 20];
        let mut count = 0;
        let mut rest = value.unsigned_abs();
        loop {
            decimal[count] = (rest % 10) as u8;
            count += 1;
            rest /= 10;
            if rest == 0 {
                break;
            }
        }
        if value < 0 {
            out.write_char('-')?;
        }
        for _ in count..usize::from(digits) {
            out.write_char(zeros[0])?;
        }
        for digit in decimal[..count].iter().rev() {
            out.write_char(zeros[usize::from(*digit)])?;
        }
        Ok(())
    }

    /// The month's name in the locale, else the calendar's own, else its
    /// number — with the locale's leap prefix where it has one, and
    /// [`Month`]'s own spelling of a leap month it has to number.
    fn write_month(
        &self,
        month: Month,
        width: NameWidth,
        context: NameContext,
        out: &mut dyn Write,
    ) -> fmt::Result {
        let in_leap_year =
            names::has_leap_year_month_names(self.locale, self.id) && self.in_leap_year();
        self.write_month_in(month, in_leap_year, width, context, out)
    }

    /// [`Renderer::write_month`] in a year that has the intercalary month
    /// or not, as `in_leap_year` says.
    fn write_month_in(
        &self,
        month: Month,
        in_leap_year: bool,
        width: NameWidth,
        context: NameContext,
        out: &mut dyn Write,
    ) -> fmt::Result {
        if let Some(label) =
            names::month_label_in(self.locale, self.id, month, in_leap_year, width, context)
        {
            out.write_str(label.prefix)?;
            out.write_str(label.name)?;
            return out.write_str(label.suffix);
        }
        let own = self
            .calendar
            .cycles()
            .iter()
            .find(|cycle| cycle.kind == MONTH)
            .and_then(|cycle| {
                let index = usize::from(month.ordinal).checked_sub(1)?;
                names::position_name(self.locale, self.id, cycle, index, width, context)
            });
        match own {
            Some(name) => {
                if month.leap {
                    out.write_str(names::leap_month_prefix(self.locale, self.id))?;
                    out.write_str(name)?;
                    return out.write_str(names::leap_month_suffix(self.locale, self.id));
                }
                out.write_str(name)
            }
            None if month.leap => {
                let prefix = names::leap_month_prefix(self.locale, self.id);
                let suffix = names::leap_month_suffix(self.locale, self.id);
                if prefix.is_empty() && suffix.is_empty() {
                    write!(out, "{month}")
                } else {
                    out.write_str(prefix)?;
                    self.write_number(i64::from(month.ordinal), out)?;
                    out.write_str(suffix)
                }
            }
            None => self.write_number(i64::from(month.ordinal), out),
        }
    }

    /// A number in the locale's numbering system, or in Latin digits when
    /// that system cannot hold it.
    fn write_number(&self, value: i64, out: &mut dyn Write) -> fmt::Result {
        let mut out = Fill { out, filled: false };
        if self.numbering.write_integer(value, &mut Counter).is_ok() {
            self.numbering
                .write_integer(value, &mut out)
                .map_err(|_| fmt::Error)
        } else {
            numbering::LATN
                .write_integer(value, &mut out)
                .map_err(|_| fmt::Error)
        }
    }
}

/// What follows a placeholder's colon.
#[derive(Debug, Clone, Copy)]
enum Spec {
    /// Nothing, or nothing this renderer knows.
    None,
    /// A width: `{month:abbreviated}`.
    Width(NameWidth),
    /// A number of at least this many digits: `{month:2}`.
    Digits(u8),
    /// The day counted from zero: `{day:0-based}`.
    ZeroBased,
    /// The number in a numbering system by its CLDR identifier,
    /// `{year:hans}`.
    Numbering(&'static NumberingSystem),
}

impl Spec {
    fn parse(text: &str) -> Self {
        if text == "0-based" {
            return Self::ZeroBased;
        }
        if let Ok(digits) = text.parse::<u8>() {
            return Self::Digits(digits.min(20));
        }
        if let Some(width) = width_named(text) {
            return Self::Width(width);
        }
        NumberingSystem::from_id(text).map_or(Self::None, Self::Numbering)
    }

    fn width(self) -> NameWidth {
        match self {
            Self::Width(width) => width,
            _ => NameWidth::Wide,
        }
    }
}

/// A placeholder's name, the extra field it names, and what follows its
/// colon: `month:abbreviated` is the month at that width, `month:2` its
/// number in two digits, `extra:samvatsara` the extra field
/// `samvatsara`, and `extra:samvatsara:abbreviated` its name at that
/// width.
fn parse_placeholder(inside: &str) -> (&str, &str, Spec) {
    if let Some(rest) = inside.strip_prefix("extra:") {
        return match rest.split_once(':') {
            Some((field, spec)) => ("extra", field, Spec::parse(spec)),
            None => ("extra", rest, Spec::None),
        };
    }
    match inside.split_once(':') {
        Some((name, spec)) => (name, "", Spec::parse(spec)),
        None => (inside, "", Spec::None),
    }
}

/// A width named in a placeholder, `{month:abbreviated}`.
fn width_named(name: &str) -> Option<NameWidth> {
    match name {
        "wide" => Some(NameWidth::Wide),
        "abbreviated" => Some(NameWidth::Abbreviated),
        "short" => Some(NameWidth::Short),
        "narrow" => Some(NameWidth::Narrow),
        _ => None,
    }
}

/// A sink that discards what it is given, for the dry run that decides
/// whether a template says anything and for checking that a numbering
/// system can hold a value.
struct Counter;

impl Write for Counter {
    fn write_str(&mut self, _: &str) -> fmt::Result {
        Ok(())
    }
}

/// A sink that keeps the bytes written, up to `N`, for reading a numeral
/// or a date back, and refuses more.
struct Buffer<const N: usize> {
    bytes: [u8; N],
    length: usize,
}

impl<const N: usize> Default for Buffer<N> {
    fn default() -> Self {
        Self {
            bytes: [0; N],
            length: 0,
        }
    }
}

impl<const N: usize> Buffer<N> {
    fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(&self.bytes[..self.length]).ok()
    }
}

impl<const N: usize> Write for Buffer<N> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self.length + text.len();
        let slot = self.bytes.get_mut(self.length..end).ok_or(fmt::Error)?;
        slot.copy_from_slice(text.as_bytes());
        self.length = end;
        Ok(())
    }
}

/// A sink that remembers whether a placeholder wrote anything through it.
struct Fill<'a> {
    out: &'a mut dyn Write,
    filled: bool,
}

impl Write for Fill<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if !s.is_empty() {
            self.filled = true;
        }
        self.out.write_str(s)
    }
}

/// A sink that tidies what an empty placeholder leaves behind: runs of
/// spaces become one, leading and trailing spaces go, and a separator —
/// a comma, an ideographic comma — that would begin or end the text, or
/// follow a space, is dropped or pulled up to the word before it. So
/// `{month} {day}, {year}` with no year reads *September 21* and with no
/// month *21, 2026*.
struct Collapse<'a, W: Write> {
    out: &'a mut W,
    started: bool,
    pending_space: bool,
    pending_separator: Option<char>,
}

impl<'a, W: Write> Collapse<'a, W> {
    fn new(out: &'a mut W) -> Self {
        Self {
            out,
            started: false,
            pending_space: false,
            pending_separator: None,
        }
    }
}

impl<W: Write> Write for Collapse<'_, W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for character in s.chars() {
            if character.is_whitespace() {
                self.pending_space = self.started;
            } else if matches!(character, ',' | '、' | '،') {
                if self.started {
                    self.pending_separator = Some(character);
                    self.pending_space = false;
                }
            } else {
                if let Some(separator) = self.pending_separator.take() {
                    self.out.write_char(separator)?;
                }
                if self.pending_space {
                    self.out.write_char(' ')?;
                    self.pending_space = false;
                }
                self.out.write_char(character)?;
                self.started = true;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::{String, ToString as _};
    use hc_calendar::shape::{CycleShape, EraName};
    use hc_calendar::{
        Calendar, CalendarError, CalendarMeta, CalendarResult, DynAdapter, Rd, YearKind,
    };

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).unwrap()
    }

    /// The Gregorian calendar, which every locale names.
    fn gregorian() -> DynAdapter<hc_calendars_solar::GregorianCalendar> {
        DynAdapter::new(hc_calendars_solar::GregorianCalendar)
    }

    fn gregorian_fields(rd: i64) -> DateFields {
        gregorian().fixed_to_fields(Rd(rd)).unwrap()
    }

    /// A stand-in for the Chinese calendar: registered under its identifier
    /// so that the locale data answers for it, with the sexagenary year
    /// among its extras and a leap month, but arithmetic of its own.
    #[derive(Debug, Clone, Copy)]
    struct Toy;

    impl Calendar for Toy {
        type Date = DateFields;

        fn cycles(&self) -> &'static [CycleShape] {
            const SHAPE: &[CycleShape] = &[CycleShape::intercalary(MONTH, 12, 13)];
            SHAPE
        }

        fn is_leap_year(&self, _: i64) -> CalendarResult<bool> {
            Ok(false)
        }

        fn era_name(&self, code: &str) -> Option<EraName> {
            (code == "kaei").then_some(EraName::new("嘉永", "Kaei"))
        }

        fn meta(&self) -> CalendarMeta {
            CalendarMeta {
                id: CalendarId("chinese"),
                english_name: "Toy",
                year_kind: YearKind::Astronomical,
                has_leap_months: true,
                is_astronomical: false,
                earliest: None,
                latest: None,
                native_locales: &["zh-Hans"],
            }
        }

        fn to_fixed(&self, _: Self::Date) -> CalendarResult<Rd> {
            Err(CalendarError::UnsupportedField("day"))
        }

        fn from_fixed(&self, _: Rd) -> CalendarResult<Self::Date> {
            Err(CalendarError::UnsupportedField("day"))
        }

        fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
            Ok(date)
        }

        fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
            Ok(*fields)
        }
    }

    /// 2023-03-22: 癸卯年闰二月初一, with the sexagenary index of 癸卯.
    fn guimao() -> DateFields {
        let mut fields = DateFields::ymd_leap_month(4660, 2, 1);
        fields.extra.set("sexagenary_year", 39).unwrap();
        fields.extra.set("related-gregorian-year", 2023).unwrap();
        fields
    }

    fn render(calendar: &dyn DynCalendar, fields: &DateFields, tag: &str) -> [String; 5] {
        let locale = locale(tag);
        [
            label(calendar, fields, Unit::Era, &locale),
            label(calendar, fields, Unit::Year, &locale),
            label(calendar, fields, Unit::Month, &locale),
            label(calendar, fields, Unit::Day, &locale),
            date(calendar, fields, &locale),
        ]
    }

    #[test]
    fn the_gregorian_calendar_reads_as_each_locale_writes_it() {
        let day = gregorian_fields(739_880); // 2026-09-21
        // The Gregorian calendar counts astronomical years and carries no
        // era, so the era lane is empty and 44 BC is the year -43.
        assert_eq!(
            render(&gregorian(), &day, "en"),
            ["", "2026", "September", "21", "September 21, 2026"]
        );
        assert_eq!(
            render(&gregorian(), &day, "ja-JP"),
            ["", "2026年", "9月", "21日", "2026年9月21日"]
        );
        assert_eq!(
            render(&gregorian(), &day, "zh-Hans"),
            ["", "2026年", "九月", "21日", "2026年9月21日"]
        );
        assert_eq!(render(&gregorian(), &day, "de")[4], "21. September 2026");
        assert_eq!(
            render(&gregorian(), &day, "es")[4],
            "21 de septiembre de 2026"
        );
        assert_eq!(render(&gregorian(), &day, "ru")[4], "21 сентября 2026 г.");
        assert_eq!(render(&gregorian(), &day, "ko")[4], "2026년 9월 21일");
        // `ar.xml` writes Latin digits, `ar_EG.xml` Arabic-Indic ones.
        assert_eq!(render(&gregorian(), &day, "ar")[1], "2026");
        assert_eq!(render(&gregorian(), &day, "ar-EG")[1], "٢٠٢٦");
        // `am.xml`'s long date is "d MMMM y", day first as the other
        // locales' Gregorian dates are.
        assert_eq!(render(&gregorian(), &day, "am")[4], "21 ሴፕቴምበር 2026");
        // And one with no data at all, the root's names.
        assert_eq!(render(&gregorian(), &day, "tlh")[4], "2026 M09 21");
    }

    /// The locales of the most-spoken languages, each as its CLDR 48 file's
    /// long date writes 21 September 2026: `sw.xml` "d MMMM y", `ur.xml`
    /// "d MMMM، y", `mr.xml` "d MMMM, y" in Devanagari digits, `fil.xml`
    /// "MMMM d, y", `pa_Arab.xml` "d MMMM y" in Persian digits, `yue.xml`
    /// "y年M月d日", and `pt_PT.xml` `pt.xml`'s "d 'de' MMMM 'de' y".
    #[test]
    fn the_most_spoken_languages_write_a_date_as_their_files_do() {
        let day = gregorian_fields(739_880); // 2026-09-21
        for (tag, expected) in [
            ("sw", "21 Septemba 2026"),
            ("ur", "21 ستمبر، 2026"),
            ("mr", "२१ सप्टेंबर, २०२६"),
            ("te", "21 సెప్టెంబర్, 2026"),
            ("ha", "21 Satumba, 2026"),
            ("pcm", "21 Sẹptẹ́mba 2026"),
            ("fil", "Setyembre 21, 2026"),
            ("pa", "21 ਸਤੰਬਰ 2026"),
            ("pa-PK", "۲۱ ستمبر ۲۰۲۶"),
            ("yue", "2026年9月21日"),
            ("yue-CN", "2026年9月21日"),
            ("pt-PT", "21 de setembro de 2026"),
        ] {
            assert_eq!(render(&gregorian(), &day, tag)[4], expected, "{tag}");
        }
        // `yue.xml`'s Chinese months, 閏 its leap prefix, and the Han days,
        // in its long date, "U (r) 年MMMd".
        assert_eq!(
            render(&DynAdapter::new(Toy), &guimao(), "yue")[4],
            "癸卯 (2023) 年閏二月初一"
        );
    }

    #[test]
    fn an_era_is_written_where_the_locale_writes_one() {
        let ides = gregorian()
            .fields_to_fixed(&DateFields::ymd(-43, 3, 15))
            .unwrap();
        let day = gregorian_fields(ides.0);
        assert_eq!(day.era, None);
        assert_eq!(render(&gregorian(), &day, "en")[1], "-43");
        assert_eq!(render(&gregorian(), &day, "ja")[1], "-43年");
        // The Buddhist calendar counts in BE, which Thai writes before the
        // year and English after it, and nobody implies. Thai's long date,
        // `th.xml`'s "d MMMM y" for the Buddhist calendar, writes no era at
        // all, though the year by itself does.
        let buddhist = DynAdapter::new(hc_calendars_solar::BuddhistCalendar);
        let today = buddhist.fixed_to_fields(Rd(739_880)).unwrap();
        assert_eq!(today.era, Some("be"));
        assert_eq!(
            render(&buddhist, &today, "th"),
            ["พุทธศักราช", "พ.ศ. 2569", "กันยายน", "21", "21 กันยายน 2569"]
        );
        assert_eq!(render(&buddhist, &today, "en")[1], "2569 BE");
        assert_eq!(render(&buddhist, &today, "en")[4], "September 21, 2569 BE");
        assert_eq!(render(&buddhist, &today, "ja")[1], "仏暦2569年");
    }

    #[test]
    fn the_chinese_family_writes_its_year_by_the_cycle_and_its_days_by_name() {
        let toy = DynAdapter::new(Toy);
        assert_eq!(
            render(&toy, &guimao(), "zh-Hans"),
            ["", "2023癸卯年", "闰二月", "初一", "2023癸卯年闰二月初一"]
        );
        assert_eq!(
            render(&toy, &guimao(), "zh-Hant"),
            ["", "2023癸卯年", "閏二月", "初一", "2023癸卯年閏二月初一"]
        );
        assert_eq!(
            render(&toy, &guimao(), "ja"),
            ["", "癸卯年", "閏二月", "1日", "癸卯年閏二月1日"]
        );
        assert_eq!(
            render(&toy, &guimao(), "ko"),
            [
                "",
                "2023년(계묘년)",
                "윤2월",
                "1일",
                "2023년(계묘년) 윤2월 1일"
            ]
        );
        assert_eq!(
            render(&toy, &guimao(), "en"),
            [
                "",
                "2023(gui-mao)",
                "intercalary Second Month",
                "1",
                "intercalary Second Month 1, 2023(gui-mao)"
            ]
        );
        // Without the sexagenary extra the family's year template says
        // nothing and the locale's own takes over.
        let plain = DateFields::ymd_leap_month(4660, 2, 4);
        assert_eq!(render(&toy, &plain, "zh-Hans")[1], "4660年");
        assert_eq!(render(&toy, &plain, "zh-Hans")[3], "初四");
        // A German request does not name the Chinese calendar, so the
        // resolved locale is English; only `None` asks for the calendar's
        // own.
        assert_eq!(locale_for(&toy, Some(&locale("de"))).to_string(), "en");
        assert_eq!(locale_for(&toy, None).to_string(), "zh-Hans");
        assert_eq!(locale_for(&toy, Some(&locale("ja"))).to_string(), "ja");
    }

    #[test]
    fn an_era_year_is_written_from_the_locale_then_the_calendar_and_never_the_code() {
        let toy = DynAdapter::new(Toy);
        let mut kaei = DateFields::ymd(3, 1, 1).with_era("kaei");
        assert_eq!(render(&toy, &kaei, "ja")[1], "嘉永3年");
        assert_eq!(render(&toy, &kaei, "ja")[0], "嘉永");
        assert_eq!(render(&toy, &kaei, "en")[1], "3 Kaei");
        kaei.year = 1;
        assert_eq!(render(&toy, &kaei, "ja")[1], "嘉永元年");
        assert_eq!(render(&toy, &kaei, "en")[1], "1 Kaei");
        // An era no locale and no calendar names is written as nothing,
        // never as its code.
        let unknown = DateFields::ymd(3, 1, 1).with_era("no-such-era");
        assert_eq!(render(&toy, &unknown, "ja")[1], "3年");
        assert!(
            render(&toy, &unknown, "en")
                .iter()
                .all(|cell| !cell.contains("no-such-era"))
        );
    }

    #[test]
    fn a_missing_field_vanishes_with_its_affixes() {
        let toy = DynAdapter::new(Toy);
        let yearless = DateFields {
            month: Some(Month::regular(3)),
            day: Some(4),
            ..DateFields::new(0)
        };
        // The year is there as a number even when it means little; a
        // month-only date drops the day and its 日, and its day unit, having
        // no day to name, is the date.
        let month_only = DateFields {
            month: Some(Month::regular(3)),
            ..DateFields::new(4660)
        };
        assert_eq!(render(&toy, &month_only, "ja")[4], "4660年三月");
        assert_eq!(render(&toy, &month_only, "ja")[3], "4660年三月");
        assert_eq!(render(&toy, &yearless, "en")[4], "Third Month 4, 0");
        let mut collapsed = String::new();
        let mut sink = Collapse::new(&mut collapsed);
        sink.write_str("  September  , 21 ,  ").unwrap();
        assert_eq!(collapsed, "September, 21");
    }
}
