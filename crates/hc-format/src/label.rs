//! A calendar's eras, years, months, days and dates, written the way a
//! locale writes them.
//!
//! # What this is for
//!
//! A timeline lane needs a word for each box it draws — 令和元年, *Adar I*,
//! 閏二月, 初四 — and a date needs to read as one in the language it is
//! shown in: 令和8年9月21日, 癸卯年闰二月初一, *September 21, 2026*. The
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
//! [`date_marking`] says which of them the date itself wrote.
//!
//! The pieces come from the same fallbacks the rest of the workspace uses:
//! an era's name is the locale's, else the calendar's own
//! ([`DynCalendar::era_name`], romanised for a Latin-script locale), else
//! English's, else nothing — never its code, by
//! [`hc_i18n::names::era_label`]; a month's is the locale's, else the calendar's own shape name,
//! else its number; a day's is the locale's day name where it has one and
//! its number otherwise. A number is written in the locale's numbering
//! system. Nothing here invents an orthography: a locale that has stated
//! no template gets its fields in [`hc_calendar::DateFields`] order,
//! separated by spaces.
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
use hc_calendar::shape::MONTH;
use hc_calendar::units::Unit;
use hc_calendar::{CalendarId, DateFields, DynCalendar, Month};
use hc_i18n::Locale;
use hc_i18n::fields;
use hc_i18n::names::{self, DateTemplates, NameContext, NameWidth, TemplateChain};
use hc_i18n::numbering::{self, NumberingSystem};

/// The locale a calendar is rendered in: the one asked for when it names
/// the calendar, else English, else the one asked for with the calendar's
/// own names; only `None` asks for the calendar's own language first.
///
/// See [`hc_i18n::names::locale_for_calendar`], which this wraps.
#[must_use]
pub fn locale_for(calendar: &dyn DynCalendar, requested: Option<&Locale>) -> Locale {
    names::locale_for_calendar(requested, &calendar.meta())
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
    let mut collapse = Collapse::new(out);
    renderer.write_levels(|templates| templates.date, Mode::Date, &mut collapse)?;
    Ok(ExtrasWritten(renderer.written.get()))
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
        }
    }

    fn in_leap_year(&self) -> bool {
        *self
            .in_leap_year
            .get_or_init(|| self.calendar.is_leap_year_of(self.fields).unwrap_or(false))
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
            Unit::Day => self.write_levels(|templates| templates.day, Mode::Unit, out),
        }
        .map(|_| ())
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
            ("year", Mode::Date) => self.write_unit(Unit::Year, out),
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
                match names::era_label(
                    self.locale,
                    self.id,
                    code,
                    self.calendar.era_name(code),
                    width,
                ) {
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
                    let named = usize::from(day)
                        .checked_sub(1)
                        .and_then(|index| self.merged.day_names.get(index));
                    if let Some(name) = named {
                        return out.write_str(name);
                    }
                    // A day that is a position in a named cycle, the
                    // Pawukon's seven-day week, is written by its name.
                    let context = match mode {
                        Mode::Date => NameContext::Format,
                        Mode::Unit | Mode::Era => NameContext::Standalone,
                    };
                    if fields::write_value_name(
                        self.locale,
                        self.id,
                        self.calendar.cycles(),
                        "day",
                        i64::from(day),
                        width.unwrap_or(NameWidth::Wide),
                        context,
                        out,
                    )? {
                        return Ok(());
                    }
                    self.write_number(i64::from(day), out)
                }
                None => Ok(()),
            },
            _ => Ok(()),
        }
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
        let value = match name {
            "year" => Some(self.fields.year),
            "month" => self.fields.month.map(|month| i64::from(month.ordinal)),
            "day" => self.fields.day.map(i64::from),
            _ => None,
        };
        match value {
            Some(value) => self.write_padded(value, digits, out),
            None => Ok(()),
        }
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
        if let Some(label) = names::month_label_in(
            self.locale,
            self.id,
            month,
            names::has_leap_year_month_names(self.locale, self.id) && self.in_leap_year(),
            width,
            context,
        ) {
            out.write_str(label.prefix)?;
            return out.write_str(label.name);
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
                }
                out.write_str(name)
            }
            None if month.leap => {
                let prefix = names::leap_month_prefix(self.locale, self.id);
                if prefix.is_empty() {
                    write!(out, "{month}")
                } else {
                    out.write_str(prefix)?;
                    self.write_number(i64::from(month.ordinal), out)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spec {
    /// Nothing, or nothing this renderer knows.
    None,
    /// A width: `{month:abbreviated}`.
    Width(NameWidth),
    /// A number of at least this many digits: `{month:2}`.
    Digits(u8),
    /// The day counted from zero: `{day:0-based}`.
    ZeroBased,
}

impl Spec {
    fn parse(text: &str) -> Self {
        if text == "0-based" {
            return Self::ZeroBased;
        }
        if let Ok(digits) = text.parse::<u8>() {
            return Self::Digits(digits.min(20));
        }
        width_named(text).map_or(Self::None, Self::Width)
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
        assert_eq!(render(&gregorian(), &day, "ar")[1], "٢٠٢٦");
        // A locale without templates gets the fields in order.
        assert_eq!(render(&gregorian(), &day, "am")[4], "2026 ሴፕቴምበር 21");
        // And one with no data at all, the root's names.
        assert_eq!(render(&gregorian(), &day, "tlh")[4], "2026 M09 21");
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
        // year and English after it, and nobody implies.
        let buddhist = DynAdapter::new(hc_calendars_solar::BuddhistCalendar);
        let today = buddhist.fixed_to_fields(Rd(739_880)).unwrap();
        assert_eq!(today.era, Some("be"));
        assert_eq!(
            render(&buddhist, &today, "th"),
            [
                "พุทธศักราช",
                "พ.ศ. 2569",
                "กันยายน",
                "21",
                "21 กันยายน พ.ศ. 2569"
            ]
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
            ["", "癸卯年", "闰二月", "初一", "癸卯年闰二月初一"]
        );
        assert_eq!(
            render(&toy, &guimao(), "zh-Hant"),
            ["", "癸卯年", "閏二月", "初一", "癸卯年閏二月初一"]
        );
        assert_eq!(
            render(&toy, &guimao(), "ja"),
            ["", "癸卯年", "閏二月", "1日", "癸卯年閏二月1日"]
        );
        assert_eq!(
            render(&toy, &guimao(), "ko"),
            ["", "계묘년", "윤2월", "1일", "계묘년 윤2월 1일"]
        );
        assert_eq!(
            render(&toy, &guimao(), "en"),
            [
                "",
                "4660",
                "leap Second Month",
                "1",
                "leap Second Month 1, 4660"
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
