//! The tab-separated lines the WebAssembly module and the C library write
//! about calendars, written once.
//!
//! Both boundary crates answer with UTF-8 lines ending in `\n`, cells
//! separated by `\t`, one column order per export, stated in the
//! WebAssembly README; before 1.0 an order may change, each change listed
//! in the pull request that makes it. The text is the same at both boundaries and differs
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
//! language it is showing; in [`calendar_list`]'s lines, whose last cells
//! are the crate and the calendar's own languages, the fourth does.

use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use hc_calendar::Weekday;
use hc_calendar::shape::MONTH;
use hc_calendar::units::{Unit, units_at_most};
use hc_calendar::{
    CalendarError, CalendarId, CalendarRegistry, DateFields, DayBoundary, DayNaming, DynCalendar,
    Rd, Standing,
};
use hc_format::label;
use hc_i18n::Locale;
use hc_i18n::dated::{self, NamingPeriod, PeriodOn};
use hc_i18n::names::{self, NameContext, NameWidth};

use crate::boundary::{Answer, Line, Refusal};

/// The tag that asks for each calendar's own language.
pub const NATIVE: &str = "native";

/// How many columns [`describe_day`] writes.
pub const DESCRIBE_DAY_COLUMNS: usize = 18;
/// How many columns [`day_extras`] writes.
pub const DAY_EXTRAS_COLUMNS: usize = 7;
/// How many columns [`calendar_units`] writes.
pub const CALENDAR_UNITS_COLUMNS: usize = 8;
/// How many columns [`calendars`] writes.
pub const CALENDARS_COLUMNS: usize = 11;
/// How many columns [`calendar_list`] writes.
pub const CALENDAR_LIST_COLUMNS: usize = 6;
/// How many columns [`locales`] writes.
pub const LOCALES_COLUMNS: usize = 7;
/// How many columns [`gregorian_adoption`] writes.
pub const GREGORIAN_ADOPTION_COLUMNS: usize = 7;
/// How many columns [`naming_period_line`] writes.
pub const NAMING_PERIOD_COLUMNS: usize = 9;

/// The most lines one call of [`calendar_units`] writes.
///
/// The range is the caller's, so without a bound a span of days as days
/// is one line a day with no end: a billion days would be tens of
/// gigabytes, and a WebAssembly instance traps on the allocation rather
/// than answering. A hundred thousand lines is some 270 years of days, or
/// every month of eight thousand years, about four megabytes of text; a
/// timeline draws far fewer boxes than that, and a caller that wants more
/// asks in pieces, each starting where the last line ended.
pub const MAX_CALENDAR_UNITS: usize = 100_000;

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

/// The ISO weekday number of the first day of the week in the locale a tag
/// names, Monday = 1 through Sunday = 7, as [`names::first_day_of_week`]
/// reads CLDR 48's week data: `-u-fw-` first, then the tag's region, then,
/// for a tag without one, the region the language's likely subtags give.
/// A tag that does not parse, and [`NATIVE`], which names no one locale,
/// are the root locale `und`, whose week begins on the world's Monday.
#[must_use]
pub fn first_day_of_week(tag: &str) -> u8 {
    let locale = requested_locale(tag).unwrap_or(Locale::ROOT);
    names::first_day_of_week(&locale).iso_number()
}

/// The tag of the data entry a locale resolves to: what the `locale used`
/// cell carries.
#[must_use]
pub fn locale_used(locale: &Locale) -> &'static str {
    names::locale_data(locale).tag
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

/// [`DayBoundary`] as the word a line carries, as a cell.
fn day_boundary_cell(line: &mut Line<'_>, boundary: DayBoundary) {
    match boundary {
        DayBoundary::Midnight => line.cell("midnight"),
        DayBoundary::Noon(_) => line.cell("noon"),
        DayBoundary::Sunset(_) => line.cell("sunset"),
        DayBoundary::Sunrise(_) => line.cell("sunrise"),
        DayBoundary::LocalTime(time, _) => line.value(format_args!("local-time {time}")),
    };
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

/// The era's name for a column that is empty when nobody has one, by
/// [`names::era_label`]'s rule, the one the formatted date follows: the
/// locale's, else the calendar's own, else English's; never the bare code,
/// which has a column of its own.
pub(crate) fn era_label_or_empty(
    locale: &Locale,
    calendar: &dyn DynCalendar,
    code: &str,
) -> &'static str {
    names::era_label(
        locale,
        calendar.meta().id,
        code,
        calendar.era_name(code),
        NameWidth::Wide,
    )
    .unwrap_or("")
}

/// The month's name in the locale, else the calendar's own, else nothing:
/// a month is never numbered here as if that were its name.
pub(crate) fn month_label_or_empty(
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
fn fields_cells(
    line: &mut Line<'_>,
    locale: &Locale,
    calendar: &dyn DynCalendar,
    fields: &DateFields,
) {
    line.cell(fields.era.unwrap_or(""))
        .cell(
            fields
                .era
                .map_or("", |code| era_label_or_empty(locale, calendar, code)),
        )
        .value(fields.year);
    match fields.month {
        Some(month) => line.value(month.ordinal).flag(month.leap),
        None => line.empty().flag(false),
    };
    line.cell(&month_label_or_empty(locale, calendar, fields))
        .value_or_empty(fields.day)
        .flag(fields.leap_day)
        .cell_with(|cell| extras(cell, fields));
}

/// The extra fields as `name=value` pairs joined by `;`: identifiers and
/// integers for a program, which [`day_extras`] labels for a reader.
fn extras(cell: &mut dyn Write, fields: &DateFields) {
    for (index, extra) in fields.extra.iter().enumerate() {
        let separator = if index == 0 { "" } else { ";" };
        let _ = write!(cell, "{separator}{}={}", extra.name, extra.value);
    }
}

/// One day in every registered calendar, one line each, in registry
/// order.
///
/// The columns: the calendar identifier, its English name, the era code,
/// the era's name in the locale (or the calendar's own, or empty), the
/// year, the month ordinal, `1` for a leap month, the month's name in the
/// locale (or the calendar's own, or empty), the day, `1` for a leap day,
/// the extra fields as `name=value` pairs joined by `;` — identifiers and
/// integers for a program, labelled for a reader by [`day_extras`] — the
/// error code, the error name, the standing, where the calendar's day
/// begins, the date as the locale writes it, which holds an extra field
/// only where the calendar's sources write the date with it and never a
/// `name=value` pair, the locale used, and which civil day names a
/// day that does not begin at midnight — `start` for the one it begins on,
/// `end` for the one it ends on, empty for midnight. A calendar that
/// refuses the day is still a line: its date columns, standing and
/// formatted date are empty and the error code and name say why.
#[must_use]
pub fn describe_day(registry: &CalendarRegistry, day: Rd, locale: &str) -> String {
    hc_core::memo::scope(|| describe_day_in_scope(registry, day, locale))
}

/// [`describe_day`], with the memo open.
fn describe_day_in_scope(registry: &CalendarRegistry, day: Rd, locale: &str) -> String {
    let mut out = String::new();
    for (id, described) in registry.describe_day(day) {
        let Some(calendar) = registry.get(id) else {
            continue;
        };
        let meta = calendar.meta();
        let locale = locale_for(calendar, locale);
        let mut line = Line::new(&mut out);
        line.cell(id.as_str()).cell(meta.english_name);
        match described {
            Ok(fields) => {
                fields_cells(&mut line, &locale, calendar, &fields);
                // No error code, no error name; then the standing.
                line.empties(2).cell(standing_name(calendar.standing(day)));
                day_boundary_cell(&mut line, calendar.day_boundary());
                line.cell(&label::date(calendar, &fields, &locale));
            }
            Err(refusal) => {
                // The nine date columns stay empty; the refusal is the
                // answer, and there is no standing for a day the calendar
                // cannot name.
                line.empties(9)
                    .value(refusal.code())
                    .cell(refusal.name())
                    .empty();
                day_boundary_cell(&mut line, calendar.day_boundary());
                line.empty();
            }
        }
        line.cell(locale_used(&locale))
            .cell(day_naming_name(calendar.day_boundary()));
        line.end();
    }
    out
}

/// The extra fields of one day, one line per field, in registry order and,
/// within a calendar, in the order the calendar sets them: every
/// registered calendar's, or with `calendar` only that one's.
///
/// The columns: the calendar identifier; the field's identifier, a key
/// such as `samvatsara` or `julian-day-number`, never reader-facing text;
/// its value, an integer; what the locale calls the field
/// ([`hc_i18n::fields::label`]), else its English label; the value as a
/// reader reads it — the name of the position it holds in the cycle it
/// counts where that is named, *Parābhava*, a flag's *no* or *yes* or its
/// calendar's own words for the two, else the number in the locale's
/// numbering system — as a template's `{extra:FIELD}` writes it; `1` when
/// the date as the locale writes it, [`describe_day`]'s formatted cell,
/// already holds the field, itself or by the same name in the calendar's
/// own words ([`label::date_marking`]), else `0`, so that a page can show
/// the others beside the date; and the locale used, as [`describe_day`] names
/// it. The locale follows the module's rule. A calendar that refuses the
/// day, or whose date has no extra fields, writes no line.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a `calendar` the registry does not carry.
pub fn day_extras(
    registry: &CalendarRegistry,
    day: Rd,
    calendar: Option<&str>,
    locale: &str,
) -> Answer<String> {
    let only = match calendar {
        Some(id) => Some(registry.get_by_name(id).ok_or(Refusal::Unknown)?),
        None => None,
    };
    Ok(hc_core::memo::scope(|| {
        let mut out = String::new();
        match only {
            Some(calendar) => push_day_extras(&mut out, calendar, day, locale),
            None => {
                for meta in registry.metas() {
                    if let Some(calendar) = registry.get(meta.id) {
                        push_day_extras(&mut out, calendar, day, locale);
                    }
                }
            }
        }
        out
    }))
}

/// [`day_extras`]' lines for one calendar.
fn push_day_extras(out: &mut String, calendar: &dyn DynCalendar, day: Rd, tag: &str) {
    let Ok(fields) = calendar.fixed_to_fields(day) else {
        return;
    };
    if fields.extra.is_empty() {
        return;
    }
    let locale = locale_for(calendar, tag);
    let used = locale_used(&locale);
    let (_, written) = label::date_marking(calendar, &fields, &locale);
    let id = calendar.meta().id;
    for (index, extra) in fields.extra.iter().enumerate() {
        let mut line = Line::new(out);
        line.cell(id.as_str())
            .cell(extra.name)
            .value(extra.value)
            .cell_or_empty(hc_i18n::fields::label(&locale, extra.name).map(|name| name.name))
            .cell(&label::extra(calendar, &fields, extra.name, &locale))
            .flag(written.contains(index))
            .cell(used);
        line.end();
    }
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
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a range of more than
/// [`MAX_CALENDAR_UNITS`] spans, found by walking one span past the bound
/// and no further.
pub fn calendar_units(
    calendar: &dyn DynCalendar,
    unit: Unit,
    from: Rd,
    to: Rd,
    locale: &str,
) -> Answer<String> {
    let spans =
        units_at_most(calendar, unit, from, to, MAX_CALENDAR_UNITS).ok_or(Refusal::OutOfRange)?;
    let locale = locale_for(calendar, locale);
    let used = locale_used(&locale);
    let mut out = String::new();
    for span in spans {
        let mut line = Line::new(&mut out);
        line.value(span.start.0).value(span.end.0);
        match span.dated {
            Ok(dated) => line
                .cell(&label::label(calendar, &dated.fields, unit, &locale))
                .flag(dated.leap)
                .cell(standing_name(dated.standing))
                .empties(2),
            Err(refusal) => line.empties(3).value(refusal.code()).cell(refusal.name()),
        };
        line.cell(used);
        line.end();
    }
    Ok(out)
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
///
/// No two rows share a name, in the locale's column or in the English one:
/// calendars that differ only in a convention are named for it, and
/// `tests/distinct_names.rs` holds every locale to that.
#[must_use]
pub fn calendars(registry: &CalendarRegistry, today: Rd, locale: &str) -> String {
    hc_core::memo::scope(|| calendars_in_scope(registry, today, locale))
}

/// [`calendars`], with the memo open.
fn calendars_in_scope(registry: &CalendarRegistry, today: Rd, locale: &str) -> String {
    let requested = requested_locale(locale);
    let mut out = String::new();
    for meta in registry.metas() {
        let Some(calendar) = registry.get(meta.id) else {
            continue;
        };
        let named_in = requested.unwrap_or_else(|| label::locale_for(calendar, None));
        let (era, year, month, day) = has_units(calendar, today);
        let mut line = Line::new(&mut out);
        line.cell(meta.id.as_str())
            .cell(names::calendar_display_name(&named_in, meta.id).unwrap_or(""))
            .cell(meta.english_name)
            .value_or_empty(meta.earliest.map(|earliest| earliest.0))
            .value_or_empty(meta.latest.map(|latest| latest.0))
            .flag(era)
            .flag(year)
            .flag(month)
            .flag(day)
            .cell(&meta.native_locales.join(";"))
            .cell(standing_name(calendar.standing(today)));
        line.end();
    }
    out
}

/// Every registered calendar, one line each, in registry order, with
/// nothing that depends on a day.
///
/// The columns: the identifier, what the locale calls the calendar (or
/// empty), its English name, the locale used — the tag of the data entry
/// the name came from, empty where the name is — and the crate that
/// registers it (`hc-calendars-solar`, `hc-calendars-lunar`,
/// `hc-calendars-equinox`, `hc-calendars-indic`, `hc-calendars-regional`),
/// as [`crate::CALENDAR_CRATES`] names them, or empty for a calendar none of
/// them registers; and the languages its sources are written in, as BCP 47
/// tags joined by `;`, the cell [`calendars`] writes, empty where there are
/// none — what a page needs to list a reader's own calendars first.
///
/// The name is the one [`calendars`] writes in its second column, by the
/// same rule: the requested locale's own or empty, never borrowed from the
/// calendar's own language, and under [`NATIVE`] the calendar's own. What
/// [`calendars`] adds is the range, the units and the standing on a day,
/// which cost a conversion of that day in every calendar; a page that only
/// lists the calendars, which it does far more often than it describes a
/// day, asks for this instead, which converts nothing.
#[must_use]
pub fn calendar_list(registry: &CalendarRegistry, locale: &str) -> String {
    let requested = requested_locale(locale);
    let crates = calendar_crates();
    let mut out = String::new();
    for meta in registry.metas() {
        let Some(calendar) = registry.get(meta.id) else {
            continue;
        };
        let named_in = requested.unwrap_or_else(|| label::locale_for(calendar, None));
        let (name, tag) =
            names::calendar_display_name_with_tag(&named_in, meta.id).unwrap_or(("", ""));
        let mut line = Line::new(&mut out);
        line.cell(meta.id.as_str())
            .cell(name)
            .cell(meta.english_name)
            .cell(tag)
            .cell_or_empty(crates.get(meta.id.as_str()).copied())
            .cell(&meta.native_locales.join(";"));
        line.end();
    }
    out
}

/// Which of [`crate::CALENDAR_CRATES`] registers each identifier: the last
/// to register it, since a later registration replaces an earlier one.
fn calendar_crates() -> BTreeMap<&'static str, &'static str> {
    let mut crates = BTreeMap::new();
    for (krate, register) in crate::CALENDAR_CRATES {
        let mut registry = CalendarRegistry::new();
        register(&mut registry);
        for meta in registry.metas() {
            crates.insert(meta.id.as_str(), *krate);
        }
    }
    crates
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
        let months = data
            .calendar(gregory)
            .is_some_and(|entry| !entry.months().is_empty());
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
        let mut line = Line::new(&mut out);
        line.cell(data.tag)
            .cell(data.english_name)
            .cell(data.native_name)
            .flag(months)
            .flag(!data.weekdays.is_empty())
            .flag(data.eras_for(gregory).is_some())
            .cell(&named.join(";"));
        line.end();
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
        let mut line = Line::new(&mut out);
        if let (Ok(last), Ok(first)) = (row.last_old_day(), row.first_day()) {
            line.value(last.0).value(first.0);
        } else {
            line.empties(2);
        }
        line.cell(row.old_calendar)
            .cell(row.scope.as_str())
            .cell(row.source())
            .cell(row.new_calendar)
            .cell(row.polity);
        line.end();
    }
    out
}

/// The line of `hc_naming_period_on`: which month and weekday names a
/// locale writes for a calendar on a day, where a government renamed them
/// for a period, by [`dated::period_on`].
///
/// The columns: `in-force` when a period's names were in force on the
/// day, `undecided` when a period applies and no source read says whether
/// it was yet in force, or `ordinary` when none applies and the locale's
/// own names hold; then, for a period, its identifier, its name for the
/// day's month and for the day's weekday, the English meaning of that
/// weekday name as the source glosses it, the first day the names can have
/// been in force, the first day by which every source read has them in
/// force, the first day the old names were back, all three as fixed days,
/// and the sources. For `ordinary` the eight cells after the first are
/// empty. The tag is read as [`requested_locale`] reads it; [`NATIVE`],
/// which names no one language, takes no period.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a calendar the registry does not carry.
pub fn naming_period_line(
    registry: &CalendarRegistry,
    tag: &str,
    calendar_id: &str,
    fixed: i64,
) -> Answer<String> {
    let calendar = registry.get_by_name(calendar_id).ok_or(Refusal::Unknown)?;
    let day = Rd(fixed);
    let locale = requested_locale(tag).unwrap_or(Locale::ROOT);
    let (state, period) = match dated::period_on(&locale, calendar.meta().id, day) {
        PeriodOn::InForce(period) => ("in-force", period),
        PeriodOn::Undecided(period) => ("undecided", period),
        PeriodOn::Ordinary => {
            let mut out = String::new();
            let mut line = Line::new(&mut out);
            line.cell("ordinary").empties(NAMING_PERIOD_COLUMNS - 1);
            line.end();
            return Ok(out);
        }
    };
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(state);
    period_cells(&mut line, period, calendar, day);
    line.end();
    Ok(out)
}

/// A period's cells of [`naming_period_line`] for a day, after the state.
fn period_cells(line: &mut Line<'_>, period: &NamingPeriod, calendar: &dyn DynCalendar, day: Rd) {
    let month = calendar
        .fixed_to_fields(day)
        .ok()
        .and_then(|fields| fields.month)
        .and_then(|month| period.month_name(month.ordinal));
    let weekday = Weekday::from_rd(day);
    let meaning = period
        .weekday_meanings
        .get(usize::from(weekday.monday_first_number()))
        .copied();
    line.cell(period.id)
        .cell_or_empty(month)
        .cell_or_empty(period.weekday_name(weekday))
        .cell_or_empty(meaning)
        .value(period.earliest.0)
        .value(period.in_force_by.0)
        .value(period.ended.0)
        .cell(period.source);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells(line: &str) -> Vec<&str> {
        line.strip_suffix('\n')
            .expect("a line")
            .split('\t')
            .collect()
    }

    fn day(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    /// Wikipedia's names of 2002 to 2008: 21 March 2005, a Monday in March,
    /// is Başgün of Nowruz; the People's Council's vote of 8 August 2002
    /// begins the days no source read decides; and from 1 July 2008 the old
    /// names are back.
    #[test]
    fn a_turkmen_day_of_2005_takes_the_names_of_2002() {
        let registry = crate::registry();
        let line = naming_period_line(&registry, "tk", "gregory", day(2005, 3, 21)).expect("known");
        let row = cells(&line);
        assert_eq!(row.len(), NAMING_PERIOD_COLUMNS);
        assert_eq!(
            row[..5],
            ["in-force", "turkmen-2002", "Nowruz", "Başgün", "First day"]
        );
        assert_eq!(
            row[5..8],
            [
                day(2002, 8, 8).to_string(),
                day(2003, 1, 1).to_string(),
                day(2008, 7, 1).to_string()
            ]
        );
        let undecided =
            naming_period_line(&registry, "tk-TM", "gregory", day(2002, 9, 1)).expect("known");
        assert!(
            undecided.starts_with("undecided\tturkmen-2002\tRuhnama\t"),
            "{undecided}"
        );
        for (tag, calendar, on) in [
            ("tk", "gregory", day(2008, 7, 1)),
            ("ru", "gregory", day(2005, 3, 21)),
            ("tk", "julian", day(2005, 3, 21)),
            (NATIVE, "gregory", day(2005, 3, 21)),
        ] {
            let line = naming_period_line(&registry, tag, calendar, on).expect("known");
            assert_eq!(
                cells(&line),
                ["ordinary", "", "", "", "", "", "", "", ""],
                "{tag} {calendar}"
            );
        }
        assert_eq!(
            naming_period_line(&registry, "tk", "no-such-calendar", 0),
            Err(Refusal::Unknown)
        );
    }
}
