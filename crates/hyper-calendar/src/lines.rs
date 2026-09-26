//! The tab-separated lines the WebAssembly module and the C library write
//! about calendars, written once.
//!
//! Both boundary crates answer with UTF-8 lines ending in `\n`, cells
//! separated by `\t`, one fixed column order per export that only ever
//! grows at the end. The text is the same at both boundaries and differs
//! only in how it crosses — a length and a sentinel there, a NUL and a
//! status code here — so the lines are made in one place, this module,
//! and each boundary does its own marshalling. A cell never contains a tab
//! or a line break; a cell with nothing to say is empty.
//!
//! # Locales
//!
//! Every function here that takes a locale takes a BCP 47 tag, and every
//! rendered cell follows one rule, the one [`hc_format::label::locale_for`]
//! resolves: the locale asked for when it names the calendar, else
//! English, else the locale asked for with the calendar's own names from
//! its shape. A named locale never borrows the calendar's own language —
//! under `ja` the Umm al-Qura calendar is written in English, not Arabic.
//! Only the tag [`NATIVE`] asks for each calendar's own language first,
//! then English. A tag that does not parse is the root locale `und`. The last cell of
//! every line that was rendered in a locale names the data entry that
//! answered — `ja`, `zh-Hans`, `he`, `und` — so that a page knows what
//! language it is showing.

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use hc_calendar::shape::MONTH;
use hc_calendar::units::{Unit, units};
use hc_calendar::{
    CalendarError, CalendarId, CalendarRegistry, DateFields, DayBoundary, DayNaming, DynCalendar,
    Rd, Standing,
};
use hc_format::label;
use hc_i18n::Locale;
use hc_i18n::names::{self, NameContext, NameWidth};

/// The tag that asks for each calendar's own language.
pub const NATIVE: &str = "native";

/// How many columns [`describe_day`] writes.
pub const DESCRIBE_DAY_COLUMNS: usize = 18;
/// How many columns [`calendar_units`] writes.
pub const CALENDAR_UNITS_COLUMNS: usize = 8;
/// How many columns [`calendars`] writes.
pub const CALENDARS_COLUMNS: usize = 11;
/// How many columns [`locales`] writes.
pub const LOCALES_COLUMNS: usize = 7;
/// How many columns [`gregorian_adoption`] writes.
pub const GREGORIAN_ADOPTION_COLUMNS: usize = 7;

/// The locale a tag asks for: `None` for [`NATIVE`], the root locale for
/// a tag that does not parse.
#[must_use]
pub fn requested_locale(tag: &str) -> Option<Locale> {
    if tag == NATIVE {
        None
    } else {
        Some(Locale::parse(tag).unwrap_or(Locale::ROOT))
    }
}

/// The locale a calendar is rendered in for a tag.
#[must_use]
pub fn locale_for(calendar: &dyn DynCalendar, tag: &str) -> Locale {
    label::locale_for(calendar, requested_locale(tag).as_ref())
}

/// The tag of the data entry a locale resolves to: what the `locale used`
/// cell carries.
#[must_use]
pub fn locale_used(locale: &Locale) -> &'static str {
    names::locale_data(locale).tag
}

/// Append one cell of a line: `text` with any tab or line break replaced
/// by a space, so the line format survives whatever a source string holds.
pub fn push_cell(out: &mut String, text: &str) {
    for character in text.chars() {
        out.push(match character {
            '\t' | '\n' | '\r' => ' ',
            other => other,
        });
    }
}

/// [`Standing`] as the word a line carries.
#[must_use]
pub const fn standing_name(standing: Standing) -> &'static str {
    match standing {
        Standing::InUse => "in-use",
        Standing::Proleptic => "proleptic",
        Standing::Extended => "extended",
        Standing::Unrecorded => "unrecorded",
    }
}

/// [`DayBoundary`] as the word a line carries.
fn push_day_boundary(out: &mut String, boundary: DayBoundary) {
    match boundary {
        DayBoundary::Midnight => out.push_str("midnight"),
        DayBoundary::Noon(_) => out.push_str("noon"),
        DayBoundary::Sunset(_) => out.push_str("sunset"),
        DayBoundary::Sunrise(_) => out.push_str("sunrise"),
        DayBoundary::LocalTime(time, _) => {
            let _ = write!(out, "local-time {time}");
        }
    }
}

/// Which civil day names a day that begins at `boundary`, as the word a
/// line carries: `start`, `end`, or nothing for a midnight start, which
/// lies inside one civil day.
const fn day_naming_name(boundary: DayBoundary) -> &'static str {
    match boundary {
        DayBoundary::Midnight => "",
        _ => match boundary.naming() {
            DayNaming::ByStart => "start",
            DayNaming::ByEnd => "end",
        },
    }
}

/// `1` or `0`.
fn push_flag(out: &mut String, flag: bool) {
    out.push(if flag { '1' } else { '0' });
}

/// The era's name for a column that is empty when nobody has one: the
/// locale's, else the calendar's own; never the bare code, which has a
/// column of its own.
fn era_label_or_empty(locale: &Locale, calendar: &dyn DynCalendar, code: &str) -> &'static str {
    let id = calendar.meta().id;
    names::era_name_by_code(locale, id, code, NameWidth::Wide).unwrap_or_else(|| {
        calendar.era_name(code).map_or("", |own| {
            if names::is_latin_script(locale) {
                own.latin()
            } else {
                own.native
            }
        })
    })
}

/// The month's name in the locale, else the calendar's own, else nothing:
/// a month is never numbered here as if that were its name.
fn month_label_or_empty(
    locale: &Locale,
    calendar: &dyn DynCalendar,
    fields: &DateFields,
) -> String {
    let Some(month) = fields.month else {
        return String::new();
    };
    let id = calendar.meta().id;
    let in_leap_year = names::has_leap_year_month_names(locale, id)
        && calendar.is_leap_year_of(fields).unwrap_or(false);
    if let Some(label) = names::month_label_in(
        locale,
        id,
        month,
        in_leap_year,
        NameWidth::Wide,
        NameContext::Standalone,
    ) {
        return alloc::format!("{label}");
    }
    calendar
        .cycles()
        .iter()
        .find(|cycle| cycle.kind == MONTH)
        .and_then(|cycle| {
            let index = usize::from(month.ordinal).checked_sub(1)?;
            names::position_name(
                locale,
                id,
                cycle,
                index,
                NameWidth::Wide,
                NameContext::Standalone,
            )
        })
        .map(str::to_owned)
        .unwrap_or_default()
}

/// The date columns of a converted day: era code, era label, year, month
/// ordinal, leap-month flag, month label, day, leap-day flag and the extra
/// fields.
fn push_fields(out: &mut String, locale: &Locale, calendar: &dyn DynCalendar, fields: &DateFields) {
    let era = fields.era.unwrap_or("");
    push_cell(out, era);
    out.push('\t');
    if let Some(code) = fields.era {
        push_cell(out, era_label_or_empty(locale, calendar, code));
    }
    let _ = write!(out, "\t{}\t", fields.year);
    match fields.month {
        Some(month) => {
            let _ = write!(out, "{}\t{}\t", month.ordinal, u8::from(month.leap));
        }
        None => out.push_str("\t0\t"),
    }
    push_cell(out, &month_label_or_empty(locale, calendar, fields));
    out.push('\t');
    if let Some(day) = fields.day {
        let _ = write!(out, "{day}");
    }
    let _ = write!(out, "\t{}\t", u8::from(fields.leap_day));
    push_extras(out, fields);
}

/// The extra fields as `name=value` pairs joined by `;`.
fn push_extras(out: &mut String, fields: &DateFields) {
    let mut first = true;
    for extra in fields.extra.iter() {
        if !first {
            out.push(';');
        }
        first = false;
        push_cell(out, extra.name);
        let _ = write!(out, "={}", extra.value);
    }
}

/// One day in every registered calendar, one line each, in registry
/// order.
///
/// The columns: the calendar identifier, its English name, the era code,
/// the era's name in the locale (or the calendar's own, or empty), the
/// year, the month ordinal, `1` for a leap month, the month's name in the
/// locale (or the calendar's own, or empty), the day, `1` for a leap day,
/// the extra fields as `name=value` pairs joined by `;`, the error code,
/// the error name, the standing, where the calendar's day begins, the date
/// as the locale writes it, the locale used, and which civil day names a
/// day that does not begin at midnight — `start` for the one it begins on,
/// `end` for the one it ends on, empty for midnight. A calendar that
/// refuses the day is still a line: its date columns, standing and
/// formatted date are empty and the error code and name say why.
#[must_use]
pub fn describe_day(registry: &CalendarRegistry, day: Rd, locale: &str) -> String {
    let mut out = String::new();
    for (id, described) in registry.describe_day(day) {
        let Some(calendar) = registry.get(id) else {
            continue;
        };
        let meta = calendar.meta();
        let locale = locale_for(calendar, locale);
        push_cell(&mut out, id.as_str());
        out.push('\t');
        push_cell(&mut out, meta.english_name);
        out.push('\t');
        match described {
            Ok(fields) => {
                push_fields(&mut out, &locale, calendar, &fields);
                // No error code, no error name; then the standing.
                out.push_str("\t\t\t");
                out.push_str(standing_name(calendar.standing(day)));
                out.push('\t');
                push_day_boundary(&mut out, calendar.day_boundary());
                out.push('\t');
                push_cell(&mut out, &label::date(calendar, &fields, &locale));
            }
            Err(refusal) => {
                // The nine date columns stay empty; the refusal is the
                // answer, and there is no standing for a day the calendar
                // cannot name.
                out.push_str("\t\t\t\t\t\t\t\t\t");
                let _ = write!(out, "{}\t{}\t\t", refusal.code(), refusal.name());
                push_day_boundary(&mut out, calendar.day_boundary());
                out.push('\t');
            }
        }
        out.push('\t');
        out.push_str(locale_used(&locale));
        out.push('\t');
        out.push_str(day_naming_name(calendar.day_boundary()));
        out.push('\n');
    }
    out
}

/// The days from `from` up to but not including `to` as one calendar's
/// eras, years, months or days, one line per span, in order.
///
/// The columns: the first day of the span, the day after its last, the
/// span's label in the locale, `1` for an intercalary unit, the standing
/// of its first day, the error code, the error name, and the locale used.
/// A span the calendar refuses — days before its epoch, a unit it does not
/// have — has an empty label, leap flag and standing and carries the
/// refusal's code and name. See [`hc_calendar::units::units`] for what the
/// spans are.
#[must_use]
pub fn calendar_units(
    calendar: &dyn DynCalendar,
    unit: Unit,
    from: Rd,
    to: Rd,
    locale: &str,
) -> String {
    let locale = locale_for(calendar, locale);
    let used = locale_used(&locale);
    let mut out = String::new();
    for span in units(calendar, unit, from, to) {
        let _ = write!(out, "{}\t{}\t", span.start.0, span.end.0);
        match span.dated {
            Ok(dated) => {
                push_cell(
                    &mut out,
                    &label::label(calendar, &dated.fields, unit, &locale),
                );
                out.push('\t');
                push_flag(&mut out, dated.leap);
                out.push('\t');
                out.push_str(standing_name(dated.standing));
                out.push_str("\t\t\t");
            }
            Err(refusal) => {
                let _ = write!(out, "\t\t\t{}\t{}\t", refusal.code(), refusal.name());
            }
        }
        out.push_str(used);
        out.push('\n');
    }
    out
}

/// Whether a calendar has each unit, judged from one converted day inside
/// its range: `(era, year, month, day)`.
fn has_units(calendar: &dyn DynCalendar, probe: Rd) -> (bool, bool, bool, bool) {
    let month = calendar.cycles().iter().any(|cycle| cycle.kind == MONTH);
    let Ok(fields) = calendar.fixed_to_fields(calendar.meta().sample_day(probe)) else {
        return (false, false, month, false);
    };
    let year = !matches!(
        calendar.is_leap_year_of(&fields),
        Err(CalendarError::UnsupportedField(_))
    );
    (fields.era.is_some(), year, month, fields.day.is_some())
}

/// Every registered calendar, one line each, in registry order.
///
/// The columns: the identifier, what the locale calls the calendar (or
/// empty), its English name, the earliest and latest fixed days it converts
/// (empty where unbounded), whether it has eras, years, months and days
/// (`1` or `0` each), the languages its sources are written in as BCP 47
/// tags joined by `;`, and its standing on `today`.
///
/// The name follows the rule of the module's `# Locales`, with its English
/// step left to the page: it is the requested locale's own, or empty, so
/// that a page knows the locale has no word and falls back to the English
/// name of the next column itself. As with the dates of [`describe_day`],
/// it is never borrowed from the calendar's own language; only [`NATIVE`]
/// asks for each calendar's name in its own language.
#[must_use]
pub fn calendars(registry: &CalendarRegistry, today: Rd, locale: &str) -> String {
    let requested = requested_locale(locale);
    let mut out = String::new();
    for meta in registry.metas() {
        let Some(calendar) = registry.get(meta.id) else {
            continue;
        };
        let named_in = requested.unwrap_or_else(|| label::locale_for(calendar, None));
        push_cell(&mut out, meta.id.as_str());
        out.push('\t');
        push_cell(
            &mut out,
            names::calendar_display_name(&named_in, meta.id).unwrap_or(""),
        );
        out.push('\t');
        push_cell(&mut out, meta.english_name);
        out.push('\t');
        if let Some(earliest) = meta.earliest {
            let _ = write!(out, "{}", earliest.0);
        }
        out.push('\t');
        if let Some(latest) = meta.latest {
            let _ = write!(out, "{}", latest.0);
        }
        out.push('\t');
        let (era, year, month, day) = has_units(calendar, today);
        for flag in [era, year, month, day] {
            push_flag(&mut out, flag);
            out.push('\t');
        }
        push_cell(&mut out, &meta.native_locales.join(";"));
        out.push('\t');
        out.push_str(standing_name(calendar.standing(today)));
        out.push('\n');
    }
    out
}

/// Every locale `hc-i18n` carries, one line each, in tag order.
///
/// The columns: the tag, the language's name in English and in itself,
/// whether the locale's own data names the Gregorian months, the weekdays
/// and the Gregorian eras (`1` or `0` each), and the identifiers of the
/// calendars it has vocabulary of its own for beyond the shared Gregorian
/// months, joined by `;`.
#[must_use]
pub fn locales() -> String {
    let gregory = CalendarId("gregory");
    let mut out = String::new();
    for data in hc_i18n::data::LOCALES {
        push_cell(&mut out, data.tag);
        out.push('\t');
        push_cell(&mut out, data.english_name);
        out.push('\t');
        push_cell(&mut out, data.native_name);
        out.push('\t');
        let months = data
            .calendar(gregory)
            .is_some_and(|entry| !entry.months().is_empty());
        push_flag(&mut out, months);
        out.push('\t');
        push_flag(&mut out, !data.weekdays.is_empty());
        out.push('\t');
        push_flag(&mut out, data.eras_for(gregory).is_some());
        out.push('\t');
        let mut named: Vec<&str> = Vec::new();
        for entry in data.calendars {
            // The entry that serves the Gregorian calendar is the shared
            // vocabulary of the Gregorian family, months or not; every
            // other entry is the locale's own word for a calendar.
            if entry.serves(gregory) {
                continue;
            }
            for id in entry.calendars {
                if !named.contains(&id.0) {
                    named.push(id.0);
                }
            }
        }
        push_cell(&mut out, &named.join(";"));
        out.push('\n');
    }
    out
}

/// The steps by which a country adopted the Gregorian calendar, one line
/// each, oldest first; nothing for an ISO 3166-1 alpha-2 code the table
/// does not know.
///
/// The columns: the last day of the old reckoning and the first day of the
/// new, as fixed days, the registry identifier of the old calendar
/// (`julian`, `japanese-tenpo`, `dangi`, `chinese`, `islamic-umalqura`,
/// `rumi`, `swedish-1700`), the scope (`civil`, `ecclesiastical` or
/// `partial`), the instrument behind the step with its date and whether it
/// was read, the registry identifier of the new calendar (`gregory`, but
/// `swedish-1700` and then `julian` for Sweden's steps of 1700 and 1712),
/// and who took the step, in English. See
/// [`hc_calendars_solar::adoption`].
#[must_use]
pub fn gregorian_adoption(region: &str) -> String {
    let mut out = String::new();
    for row in hc_calendars_solar::adoption::gregorian_adoption(region) {
        if let (Ok(last), Ok(first)) = (row.last_old_day(), row.first_day()) {
            let _ = write!(out, "{}\t{}\t", last.0, first.0);
        } else {
            out.push_str("\t\t");
        }
        push_cell(&mut out, row.old_calendar);
        out.push('\t');
        out.push_str(row.scope.as_str());
        out.push('\t');
        push_cell(&mut out, row.source());
        out.push('\t');
        push_cell(&mut out, row.new_calendar);
        out.push('\t');
        push_cell(&mut out, row.polity);
        out.push('\n');
    }
    out
}
