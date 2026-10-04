//! The tab-separated lines the WebAssembly module and the C library write
//! about `hc-i18n`'s locale data on its own, written once: the day periods
//! of a time of day, a number in a numbering system and back, the
//! numbering systems, the eras of a calendar; and how a locale resolves —
//! its fallback chain, what it is, which plural category a number has in
//! it — and the names, the casing and the bidi isolates it gives text.

use alloc::string::String;

use hc_calendar::Month;
use hc_calendar::Weekday;
use hc_calendar::shape::CycleLength;
use hc_core::catalogue::matches;
use hc_i18n::casing::{self, CasingStyle};
use hc_i18n::day_periods::{self, FlexibleDayPeriod};
use hc_i18n::direction::{self, Direction};
use hc_i18n::names::{self, DayPeriod, NameContext, NameWidth};
use hc_i18n::plural::{PluralOperands, PluralRules};
use hc_i18n::week;
use hc_i18n::{Locale, NumberingSystem};

use crate::boundary::{Answer, Line, Refusal, line};
use crate::lines::locale_used;

/// The locale a tag names, the root locale for one that does not parse.
fn locale(tag: &str) -> Locale {
    Locale::parse(tag).unwrap_or(Locale::ROOT)
}

/// How many columns [`day_period_line`] writes.
pub const DAY_PERIOD_COLUMNS: usize = 7;

/// The line of `hc_day_period`: the day periods of a time of day in a
/// locale, from CLDR 48's `dayPeriods` rules and names — `am` or `pm`, and
/// the locale's abbreviated name for it in a date's format context; the
/// period the locale's rules give the minute, `midnight` or `noon` at 00:00
/// or 12:00 where the language has a word for it, else a flexible period,
/// `morning1` to `night2`, empty where the language has no rules; that
/// period's abbreviated, wide and narrow names; and the tag of the locale
/// data that answered.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for `seconds_of_day` from 86 400.
pub fn day_period_line(seconds_of_day: u32, tag: &str) -> Answer<String> {
    if seconds_of_day >= 86_400 {
        return Err(Refusal::OutOfRange);
    }
    let locale = locale(tag);
    let minute = (seconds_of_day / 60) as u16;
    let half = DayPeriod::from_hour((seconds_of_day / 3_600) as u8);
    let exact = seconds_of_day.is_multiple_of(60);
    let fixed = match minute {
        0 if exact => Some(FlexibleDayPeriod::Midnight),
        720 if exact => Some(FlexibleDayPeriod::Noon),
        _ => None,
    }
    .filter(|period| day_periods::has_fixed_period(&locale, *period));
    let period = fixed.or_else(|| day_periods::flexible_period(&locale, minute));
    let name = |width| period.and_then(|period| day_periods::period_name(&locale, period, width));
    Ok(line(|line| {
        line.cell(match half {
            DayPeriod::Am => "am",
            DayPeriod::Pm => "pm",
        })
        .cell_or_empty(names::day_period_name(
            &locale,
            half,
            NameWidth::Abbreviated,
            NameContext::Format,
        ))
        .cell_or_empty(period.map(FlexibleDayPeriod::id))
        .cell_or_empty(name(NameWidth::Abbreviated))
        .cell_or_empty(name(NameWidth::Wide))
        .cell_or_empty(name(NameWidth::Narrow))
        .cell(locale_used(&locale));
    }))
}

/// The numbering system an identifier names, by CLDR's identifier.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text.
fn system(id: &str) -> Answer<&'static NumberingSystem> {
    NumberingSystem::from_id(id.trim()).ok_or(Refusal::Unknown)
}

/// The line of `hc_format_number`: an integer written in a numbering
/// system of [`hc_i18n::numbering::ALL`] — the digits of a positional
/// system, `latn`, `arab`, `deva`; or the letters of an algorithmic one,
/// `hebr`'s Hebrew numerals, `grek` and `greklow`'s Greek, `hanidec` and
/// the Han styles — then the system's identifier.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a system not named, and
/// [`Refusal::OutOfRange`] for a value the system cannot write, such as a
/// Hebrew numeral of nothing.
pub fn format_number_line(id: &str, value: i64) -> Answer<String> {
    let system = system(id)?;
    let mut text = String::new();
    system
        .write_integer(value, &mut text)
        .map_err(|_| Refusal::OutOfRange)?;
    Ok(line(|line| {
        line.cell(&text).cell(system.id());
    }))
}

/// An integer read back out of a numbering system's notation, as
/// [`format_number_line`] writes it.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a system not named, [`Refusal::Malformed`] for
/// text that is not a number in the system, and [`Refusal::OutOfRange`]
/// for one it cannot hold.
pub fn parse_number(id: &str, text: &str) -> Answer<i64> {
    let system = system(id)?;
    system.parse_integer(text).map_err(|error| match error {
        hc_i18n::I18nError::NumberOutOfRange => Refusal::OutOfRange,
        _ => Refusal::Malformed,
    })
}

/// How many columns each line of [`numbering_systems_lines`] writes.
pub const NUMBERING_SYSTEM_COLUMNS: usize = 3;

/// The lines of `hc_numbering_systems`: every numbering system `hc-i18n`
/// writes, in its table's order — the identifier, `1` for an algorithmic
/// system that spells numbers out and `0` for a positional one, and a
/// positional system's ten digits, zero first, empty for an algorithmic
/// one.
#[must_use]
pub fn numbering_systems_lines() -> String {
    let mut out = String::new();
    for system in hc_i18n::numbering::ALL {
        let mut line = Line::new(&mut out);
        line.cell(system.id()).flag(system.is_algorithmic());
        match system.digits() {
            Some(digits) => line.cell_with(|out| {
                for digit in digits {
                    let _ = out.write_char(*digit);
                }
            }),
            None => line.empty(),
        };
        line.end();
    }
    out
}

/// How many columns each line of [`calendar_eras_lines`] writes.
pub const CALENDAR_ERA_COLUMNS: usize = 6;

/// The lines of `hc_calendar_eras`: the eras a calendar is described
/// with, in order, [`names::era_codes`] — the era's code, as a date's era
/// field writes it; its wide, abbreviated and narrow names in the locale,
/// by [`names::era_name_by_code`], empty where the locale's chain has none;
/// the calendar's identifier; and the tag of the locale data that answered.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a calendar the registry does not carry, and
/// [`Refusal::NoData`] for one whose eras no locale data lists.
pub fn calendar_eras_lines(calendar: &str, tag: &str) -> Answer<String> {
    let registry = crate::registry();
    let found = registry.get_by_name(calendar).ok_or(Refusal::Unknown)?;
    let id = found.meta().id;
    let locale = crate::lines::locale_for(found, tag);
    let codes = names::era_codes(&locale, id)
        .or_else(|| names::era_codes(&names::english(), id))
        .ok_or(Refusal::NoData)?;
    let mut out = String::new();
    for code in codes {
        let name = |width| names::era_name_by_code(&locale, id, code, width);
        let mut line = Line::new(&mut out);
        line.cell(code)
            .cell_or_empty(name(NameWidth::Wide))
            .cell_or_empty(name(NameWidth::Abbreviated))
            .cell_or_empty(name(NameWidth::Narrow))
            .cell(id.0)
            .cell(locale_used(&locale));
        line.end();
    }
    Ok(out)
}

/// The locale a tag names, strictly: the root locale for the empty tag,
/// and a refusal for one that does not parse, where the lines that name a
/// locale only to read from it take the root locale for such a tag.
///
/// # Errors
///
/// [`Refusal::Malformed`] for a tag that is not BCP 47.
fn strict_locale(tag: &str) -> Answer<Locale> {
    if tag.trim().is_empty() {
        return Ok(Locale::ROOT);
    }
    Locale::parse(tag.trim()).map_err(|_| Refusal::Malformed)
}

/// How many columns each line of [`locale_chain_lines`] writes.
pub const LOCALE_CHAIN_COLUMNS: usize = 4;

/// The lines of `hc_locale_chain`: the fallback chain of a locale, the
/// order `hc-i18n` takes a name from — one line per step, the requested
/// locale first and `und` last: the step's number from 0; its tag; the rule
/// that led to it from the step before, `requested` for the first line,
/// `likely-script` where a language carried only per script takes the script
/// CLDR's likely subtags give it, `extensions` for dropping the `-u-` keys,
/// `variant`, `parent-locales` for a parent CLDR 48's `parentLocales` name
/// (`en-GB` to `en-001`, `zh-Hant` to root), `region` and `script` for
/// truncation, and `root`; and `1` when `hc-i18n` carries an entry of data
/// for exactly that tag, else `0`: `en-001` and `en` are carried, `en-AU` and
/// the root locale `und`, whose vocabulary is the fallback of every lookup
/// and not an entry, are not.
///
/// # Errors
///
/// [`Refusal::Malformed`] for a tag that does not parse.
pub fn locale_chain_lines(tag: &str) -> Answer<String> {
    let requested = strict_locale(tag)?;
    let mut step = requested.with_likely_script();
    let mut rule = if step == requested {
        "requested"
    } else {
        "likely-script"
    };
    let mut out = String::new();
    let mut number = 0u32;
    loop {
        let carried = hc_i18n::data::LOCALES
            .iter()
            .any(|data| step.matches_tag(data.tag));
        let mut line = Line::new(&mut out);
        line.value(number)
            .cell(&step.to_tag())
            .cell(rule)
            .flag(carried);
        line.end();
        let Some((parent, why)) = step.parent_step() else {
            return Ok(out);
        };
        step = parent;
        rule = why.id();
        number += 1;
    }
}

/// How many columns the line of [`locale_info_line`] writes.
pub const LOCALE_INFO_COLUMNS: usize = 19;

/// The line of `hc_locale_info`: what `hc-i18n` knows of a locale — the tag
/// as written canonically; its language, script, region and variant subtags
/// as given, empty where absent; the `-u-` keys the tag carries,
/// `ca`, `nu`, `fw` as an ISO weekday number, and `hc`; the tag of the entry
/// of data that answers for it; its parent (the next step of the chain) and
/// the rule that gave it; the numbering system a number is written in by
/// default (`arab` for `ar`, `latn` for `en`); the ISO weekday number the
/// week begins on and CLDR's `minDays`, the fewest days of a year a week
/// needs to be its first (week data: `fw` and the region's); `ltr` or `rtl`;
/// `standard` or `turkic` casing; `1` when month and weekday names are
/// written with a capital; and the language of the plural rules that apply
/// (`pt-PT` for Portugal, `und` for none carried, where everything is
/// `other`).
///
/// CLDR's weekend days are not carried by `hc-i18n`: it reads `weekData`'s
/// `firstDay` and `minDays` only. The weekend laws `hc-holiday` carries, with
/// their sources, are in `hc_holiday_tables`' fourteenth column.
///
/// # Errors
///
/// [`Refusal::Malformed`] for a tag that does not parse.
pub fn locale_info_line(tag: &str) -> Answer<String> {
    let locale = strict_locale(tag)?;
    let rules = PluralRules::for_locale(&locale);
    let rule = week::for_locale(&locale);
    let parent = locale.parent_step();
    Ok(line(|line| {
        line.cell(&locale.to_tag())
            .cell(locale.language())
            .cell_or_empty(locale.script())
            .cell_or_empty(locale.region())
            .cell_or_empty(locale.variant())
            .cell_or_empty(locale.calendar())
            .cell_or_empty(locale.numbering_system())
            .cell_or_empty(
                locale.first_day_of_week().map(|day| {
                    ["1", "2", "3", "4", "5", "6", "7"][usize::from(day.iso_number()) - 1]
                }),
            )
            .cell_or_empty(locale.hour_cycle().map(hc_i18n::HourCycle::as_str))
            .cell(locale_used(&locale))
            .cell(
                &parent
                    .map(|(parent, _)| parent.to_tag())
                    .unwrap_or_default(),
            )
            .cell_or_empty(parent.map(|(_, why)| why.id()))
            .cell(NumberingSystem::for_locale(&locale).id())
            .value(names::first_day_of_week(&locale).iso_number())
            .value(rule.min_days())
            .cell(direction::locale_direction(&locale).as_str())
            .cell(match casing::casing_style(&locale) {
                CasingStyle::Standard => "standard",
                CasingStyle::Turkic => "turkic",
            })
            .flag(casing::capitalises_month_names(&locale))
            .cell(rules.language());
    }))
}

/// How many columns the line of [`plural_category_line`] writes.
pub const PLURAL_COLUMNS: usize = 7;

/// The line of `hc_plural_category`: the plural category a number has in a
/// locale, by CLDR 48's rules — `zero`, `one`, `two`, `few`, `many` or
/// `other`; the language of the rules that decided it (`ru`, `pt-PT`, `und`
/// where none is carried and everything is `other`); and the operands
/// UTS #35 reads from the number as written, `i`, `v`, `w`, `f` and `t`:
/// `1.0` and `1` are different questions.
///
/// `number` is a plain decimal, an optional minus sign, digits and an
/// optional point with digits, so that a trailing zero can be given. `kind`
/// is `cardinal`, the form after a count (`plurals.xml`), or `ordinal`,
/// the form of a position (`ordinals.xml`: English 1 `one`, 2 `two`, 3
/// `few`, 4 `other`), in either case. The compact-notation operands `c`
/// and `e` are not carried.
///
/// # Errors
///
/// [`Refusal::Malformed`] for a tag that does not parse or a number that is
/// not a plain decimal, [`Refusal::OutOfRange`] for digits beyond a `u64`
/// and [`Refusal::Unknown`] for a kind that is neither.
pub fn plural_category_line(tag: &str, number: &str, kind: &str) -> Answer<String> {
    let kind = hc_i18n::PluralType::from_keyword(kind.trim()).ok_or(Refusal::Unknown)?;
    let locale = strict_locale(tag)?;
    let operands = PluralOperands::parse(number.trim()).map_err(|error| match error {
        hc_i18n::I18nError::NumberOutOfRange => Refusal::OutOfRange,
        _ => Refusal::Malformed,
    })?;
    let rules = PluralRules::of_kind_for_locale(kind, &locale);
    Ok(line(|line| {
        line.cell(rules.select(&operands).as_str())
            .cell(rules.language())
            .value(operands.i())
            .value(operands.v())
            .value(operands.w())
            .value(operands.f())
            .value(operands.t());
    }))
}

/// The line of `hc_parse_pattern_in`: [`crate::datetime_lines::parse_pattern_line`]
/// with the month and weekday names, the day periods and the eras of a locale
/// read besides the C locale's.
///
/// # Errors
///
/// As [`crate::datetime_lines::parse_pattern_in_line`], and
/// [`Refusal::Malformed`] for a tag that does not parse.
pub fn parse_pattern_in_line(syntax: &str, pattern: &str, text: &str, tag: &str) -> Answer<String> {
    let locale = strict_locale(tag)?;
    crate::datetime_lines::parse_pattern_in_line(syntax, pattern, text, &locale)
}

/// How many columns each line of [`names_lines`] writes.
pub const NAMES_COLUMNS: usize = 4;

fn name_width(id: &str) -> Answer<NameWidth> {
    [
        ("wide", NameWidth::Wide),
        ("abbreviated", NameWidth::Abbreviated),
        ("short", NameWidth::Short),
        ("narrow", NameWidth::Narrow),
    ]
    .into_iter()
    .find(|(name, _)| matches(id, name))
    .map(|(_, width)| width)
    .ok_or(Refusal::Unknown)
}

fn name_context(id: &str) -> Answer<NameContext> {
    if matches(id, "format") {
        Ok(NameContext::Format)
    } else if matches(id, "standalone") || matches(id, "stand-alone") {
        Ok(NameContext::Standalone)
    } else {
        Err(Refusal::Unknown)
    }
}

/// The lines of `hc_names`: every name a locale has for a calendar in a
/// width and a context, one line per name: the kind of name — `month`, or
/// `month-in-leap-year` where the locale names a month differently in a year
/// that has the calendar's intercalary month (*Adar II*), every other cycle
/// of the calendar by its kind (`weekday`, `stem`, `branch`, `trecena`),
/// `quarter` where the locale has quarter names for the calendar, and
/// `day-period` for `am` and `pm`; the position, from 1, the ISO number for
/// a weekday; the name; and the tag of the entry of data that answered. A
/// position the locale and the calendar do not name has no line: the months
/// of a calendar that numbers them are not listed.
///
/// `width` is `wide`, `abbreviated`, `short` or `narrow`, and `context`
/// `format` (inside a date, the genitive in Russian) or `standalone`, in any
/// case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a calendar not in the registry, a width or a
/// context not named, and [`Refusal::Malformed`] for a tag that does not
/// parse.
pub fn names_lines(tag: &str, calendar: &str, width: &str, context: &str) -> Answer<String> {
    let locale = strict_locale(tag)?;
    let width = name_width(width)?;
    let context = name_context(context)?;
    let registry = crate::registry();
    let found = registry.get_by_name(calendar).ok_or(Refusal::Unknown)?;
    let id = found.meta().id;
    let used = locale_used(&locale);
    let mut out = String::new();
    let mut emit = |kind: &str, position: usize, name: &str| {
        let mut line = Line::new(&mut out);
        line.cell(kind).value(position).cell(name).cell(used);
        line.end();
    };
    for cycle in found.cycles() {
        let count = if cycle.kind == hc_calendar::shape::MONTH {
            names::month_count(&locale, id, context)
                .unwrap_or_else(|| usize::from(cycle.length.maximum()))
        } else {
            match cycle.length {
                CycleLength::Fixed(length) => usize::from(length),
                CycleLength::Intercalary { ordinary, .. } => usize::from(ordinary),
            }
        };
        for position in 1..=count {
            if cycle.kind == hc_calendar::shape::MONTH {
                let ordinal = u8::try_from(position).map_err(|_| Refusal::OutOfRange)?;
                let month = Month::regular(ordinal);
                let common = names::month_label_in(&locale, id, month, false, width, context)
                    .map(|label| alloc::format!("{label}"));
                if let Some(name) = &common {
                    emit("month", position, name);
                }
                if names::has_leap_year_month_names(&locale, id)
                    && let Some(label) =
                        names::month_label_in(&locale, id, month, true, width, context)
                {
                    let name = alloc::format!("{label}");
                    if common.as_deref() != Some(name.as_str()) {
                        emit("month-in-leap-year", position, &name);
                    }
                }
                if common.is_none()
                    && let Some(name) =
                        names::position_name(&locale, id, cycle, position - 1, width, context)
                {
                    emit("month", position, name);
                }
            } else if cycle.kind == "weekday" && count == 7 {
                let weekday = Weekday::from_iso_number(position as u8);
                if let Some(name) = weekday
                    .and_then(|weekday| names::weekday_name(&locale, weekday, width, context))
                {
                    emit("weekday", position, name);
                }
            } else if let Some(name) =
                names::position_name(&locale, id, cycle, position - 1, width, context)
            {
                emit(cycle.kind, position, name);
            }
        }
    }
    for quarter in 1..=4u8 {
        if let Some(name) = names::quarter_name(&locale, id, quarter, width, context) {
            emit("quarter", usize::from(quarter), name);
        }
    }
    for (position, period) in [(1, DayPeriod::Am), (2, DayPeriod::Pm)] {
        if let Some(name) = names::day_period_name(&locale, period, width, context) {
            emit("day-period", position, name);
        }
    }
    Ok(out)
}

/// A function that recases a text for a locale.
type Recase = fn(&Locale, &str) -> String;

/// How many columns the line of [`case_line`] writes.
pub const CASE_COLUMNS: usize = 4;

/// The line of `hc_case`: a text recased as a locale casing it — the text;
/// the mode; `standard` or `turkic`; and the tag of the entry of data that
/// answered. `mode` is `lower` and `upper`, which differ from Unicode's
/// defaults only for Turkish and Azerbaijani (`i` to `İ`); `capitalise-first`
/// and `lowercase-first`, which recase the first character only, since a
/// title case of every word needs a word-break rule `hc-i18n` does not have;
/// `sentence-start` and `in-sentence`, which set a month or weekday name as
/// the locale writes it at the start of a sentence (always with a capital)
/// and inside one (with a capital in German, English and a few others, and
/// none in French).
///
/// # Errors
///
/// [`Refusal::Unknown`] for a mode not named and [`Refusal::Malformed`] for
/// a tag that does not parse.
pub fn case_line(tag: &str, mode: &str, text: &str) -> Answer<String> {
    let locale = strict_locale(tag)?;
    let modes: [(&str, Recase); 6] = [
        ("lower", casing::to_lowercase),
        ("upper", casing::to_uppercase),
        ("capitalise-first", casing::capitalise_first),
        ("lowercase-first", casing::lowercase_first),
        ("sentence-start", casing::name_at_sentence_start),
        ("in-sentence", casing::name_in_sentence),
    ];
    let (id, recase) = modes
        .into_iter()
        .find(|(name, _)| matches(mode, name))
        .ok_or(Refusal::Unknown)?;
    let style = match casing::casing_style(&locale) {
        CasingStyle::Standard => "standard",
        CasingStyle::Turkic => "turkic",
    };
    Ok(line(|line| {
        line.cell(&recase(&locale, text))
            .cell(id)
            .cell(style)
            .cell(locale_used(&locale));
    }))
}

/// How many columns the line of [`isolate_line`] writes.
pub const ISOLATE_COLUMNS: usize = 5;

/// The line of `hc_isolate`: a text made safe to embed in text running the
/// locale's direction — the text, with U+2066 to U+2069 isolates around it
/// where `mode` says; the locale's direction, `ltr` or `rtl`; the text's own,
/// by the first-strong rule of UAX 9, empty where it has no strong
/// character; `1` when isolates were added; and the mode. `mode` is `field`,
/// which isolates only where the two directions disagree, so a date inside
/// right-to-left prose is not scrambled and one inside left-to-right prose
/// carries nothing extra; `first-strong`, which always wraps the text in
/// U+2068 and U+2069 and leaves the renderer to work the direction out; and
/// `strip`, which removes every isolate and directional mark, for comparing
/// or hashing.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a mode not named and [`Refusal::Malformed`] for
/// a tag that does not parse.
pub fn isolate_line(tag: &str, mode: &str, text: &str) -> Answer<String> {
    let locale = strict_locale(tag)?;
    let outer = direction::locale_direction(&locale);
    let inner = direction::first_strong_direction(text);
    let (id, written, isolated) = if matches(mode, "field") {
        let mut written = String::new();
        let inner_direction = inner.unwrap_or(outer);
        direction::write_field(text, outer, inner_direction, &mut written)
            .map_err(|_| Refusal::OutOfRange)?;
        let isolated = direction::needs_isolation(outer, inner_direction);
        ("field", written, isolated)
    } else if matches(mode, "first-strong") {
        ("first-strong", direction::isolate(text), true)
    } else if matches(mode, "strip") {
        ("strip", direction::strip_isolates(text), false)
    } else {
        return Err(Refusal::Unknown);
    };
    Ok(line(|line| {
        line.cell(&written)
            .cell(outer.as_str())
            .cell_or_empty(inner.map(Direction::as_str))
            .flag(isolated)
            .cell(id);
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::cells;

    /// CLDR 48's English day periods: 15:00 is *in the afternoon*, noon is
    /// *noon*, and 03:00 *at night*.
    #[test]
    fn a_time_of_day_has_its_periods() {
        let line = day_period_line(15 * 3_600, "en").expect("a time");
        assert_eq!(
            cells(&line)[..5],
            [
                "pm",
                "PM",
                "afternoon1",
                "in the afternoon",
                "in the afternoon"
            ]
        );
        let noon = day_period_line(12 * 3_600, "en").expect("a time");
        assert_eq!(cells(&noon)[2], "noon");
        assert_eq!(day_period_line(86_400, "en"), Err(Refusal::OutOfRange));
    }

    /// The Hebrew numeral of 5786 is ה׳תשפ״ו, and Greek 2026 is ͵ΒΚϚʹ
    /// (`docs/systems/hebrew-numerals.md`; `hc-i18n`'s tests).
    #[test]
    fn a_number_goes_into_a_system_and_back() {
        let line = format_number_line("hebr", 5_786).expect("written");
        let text = cells(&line)[0].to_owned();
        assert_eq!(parse_number("hebr", &text), Ok(5_786));
        let line = format_number_line("grek", 2_026).expect("written");
        let text = cells(&line)[0].to_owned();
        assert_eq!(parse_number("grek", &text), Ok(2_026));
        assert_eq!(format_number_line("klingon", 1), Err(Refusal::Unknown));
        assert_eq!(parse_number("latn", "x"), Err(Refusal::Malformed));
        assert!(
            numbering_systems_lines()
                .lines()
                .any(|line| line == "latn\t0\t0123456789")
        );
        assert!(
            numbering_systems_lines()
                .lines()
                .any(|line| line == "hebr\t1\t")
        );
    }

    /// The Japanese eras begin with the ones the locale data lists, and
    /// 令和 is among them under `ja`.
    #[cfg(feature = "regional")]
    #[test]
    fn a_calendars_eras_are_listed() {
        let text = calendar_eras_lines("japanese", "ja").expect("listed");
        assert!(
            text.lines()
                .all(|line| line.split('\t').count() == CALENDAR_ERA_COLUMNS)
        );
        assert!(text.lines().any(|line| line.starts_with("reiwa\t令和\t")));
        assert_eq!(calendar_eras_lines("no-such", "ja"), Err(Refusal::Unknown));
    }

    fn all_cells(text: &str) -> alloc::vec::Vec<alloc::vec::Vec<String>> {
        text.lines()
            .map(|line| line.split('\t').map(str::to_owned).collect())
            .collect()
    }

    /// `docs/systems/locale-fallback.md`'s worked chains, from UTS #35 and
    /// CLDR 48's `parentLocales`: `en-AU` is `en-001`, `en`, `und`; `zh-TW`
    /// takes its likely script; `ht` is `fr-HT`.
    #[test]
    fn a_locale_resolves_along_its_chain() {
        let chain = |tag: &str| {
            all_cells(&locale_chain_lines(tag).expect("a chain"))
                .into_iter()
                .map(|cells| (cells[1].clone(), cells[2].clone()))
                .collect::<alloc::vec::Vec<_>>()
        };
        let pairs = |list: &[(&str, &str)]| {
            list.iter()
                .map(|(tag, rule)| ((*tag).to_owned(), (*rule).to_owned()))
                .collect::<alloc::vec::Vec<_>>()
        };
        assert_eq!(
            chain("en-AU"),
            pairs(&[
                ("en-AU", "requested"),
                ("en-001", "parent-locales"),
                ("en", "region"),
                ("und", "root")
            ])
        );
        assert_eq!(
            chain("zh-TW"),
            pairs(&[
                ("zh-Hant-TW", "likely-script"),
                ("zh-Hant", "region"),
                ("und", "parent-locales")
            ])
        );
        assert_eq!(
            chain("ht"),
            pairs(&[
                ("ht", "requested"),
                ("fr-HT", "parent-locales"),
                ("fr", "region"),
                ("und", "root")
            ])
        );
        assert_eq!(
            chain("ja-JP-u-ca-japanese"),
            pairs(&[
                ("ja-JP-u-ca-japanese", "requested"),
                ("ja-JP", "extensions"),
                ("ja", "region"),
                ("und", "root")
            ])
        );
        let carried: alloc::vec::Vec<String> =
            all_cells(&locale_chain_lines("en-AU").expect("a chain"))
                .into_iter()
                .map(|cells| cells[3].clone())
                .collect();
        assert_eq!(carried, ["0", "1", "1", "0"]);
        assert_eq!(locale_chain_lines("not a tag"), Err(Refusal::Malformed));
        assert_eq!(
            all_cells(&locale_chain_lines("").expect("root")),
            [["0", "und", "requested", "0"]]
        );
    }

    /// CLDR 48's week data (`en-US` Sunday and 1 day, `de-DE` Monday and 4),
    /// directions, numbering systems (`ar-SA` writes `arab`, `ar` `latn`:
    /// `docs/systems/locale-fallback.md`) and the cardinal plural rules.
    #[test]
    fn a_locale_describes_itself() {
        let info = |tag: &str| all_cells(&locale_info_line(tag).expect("a locale")).remove(0);
        assert_eq!(info("en-US").len(), LOCALE_INFO_COLUMNS);
        let us = info("en-US");
        assert_eq!(us[0], "en-US");
        assert_eq!(us[9], "en");
        assert_eq!(us[10], "en");
        assert_eq!(us[11], "region");
        assert_eq!(us[12], "latn");
        assert_eq!(us[13..16], ["7", "1", "ltr"]);
        assert_eq!(us[18], "en");
        let de = info("de-DE");
        assert_eq!(de[13..16], ["1", "4", "ltr"]);
        let sa = info("ar-SA");
        assert_eq!(sa[12], "arab");
        assert_eq!(sa[15], "rtl");
        assert_eq!(info("ar")[12], "latn");
        let tr = info("tr");
        assert_eq!(tr[16], "turkic");
        let keys = info("ja-JP-u-ca-japanese-fw-sun-hc-h11-nu-jpan");
        assert_eq!(keys[5..9], ["japanese", "jpan", "7", "h11"]);
        assert_eq!(keys[10], "ja-JP");
        assert_eq!(keys[11], "extensions");
        assert_eq!(info("pt-PT")[18], "pt-PT");
        assert_eq!(locale_info_line("e n"), Err(Refusal::Malformed));
    }

    /// CLDR 48's `plurals.xml`: Russian 1 `one`, 2 `few`, 5 `many`, 21 `one`,
    /// 1.5 `other`; Arabic 0 `zero`, 2 `two`, 11 `many`; English 1.0 is
    /// `other` because `v` is 1; French 1.5 is `one`.
    #[test]
    fn a_number_has_a_plural_category() {
        let category = |tag: &str, number: &str| {
            all_cells(&plural_category_line(tag, number, "cardinal").expect("a category")).remove(0)
                [0]
            .clone()
        };
        assert_eq!(category("ru", "1"), "one");
        assert_eq!(category("ru", "2"), "few");
        assert_eq!(category("ru", "5"), "many");
        assert_eq!(category("ru", "21"), "one");
        assert_eq!(category("ru", "1.5"), "other");
        assert_eq!(category("ar", "0"), "zero");
        assert_eq!(category("ar", "2"), "two");
        assert_eq!(category("ar", "11"), "many");
        assert_eq!(category("en", "1"), "one");
        assert_eq!(category("en", "1.0"), "other");
        assert_eq!(category("fr", "1.5"), "one");
        assert_eq!(category("ja", "1"), "other");
        let line = plural_category_line("en", "1.30", "CARDINAL").expect("a category");
        assert_eq!(cells(&line), ["other", "en", "1", "2", "1", "30", "3"]);
        // CLDR 48's `ordinals.xml`: English 1st, 2nd, 3rd, 4th, 11th, 21st;
        // Welsh 7 `zero`; German has one form, so `other`.
        let ordinal = |tag: &str, number: &str| {
            all_cells(&plural_category_line(tag, number, "ordinal").expect("a category")).remove(0)
        };
        assert_eq!(ordinal("en", "1")[..2], ["one", "en"]);
        assert_eq!(ordinal("en", "2")[0], "two");
        assert_eq!(ordinal("en", "3")[0], "few");
        assert_eq!(ordinal("en", "4")[0], "other");
        assert_eq!(ordinal("en", "11")[0], "other");
        assert_eq!(ordinal("en-GB", "21")[0], "one");
        assert_eq!(ordinal("cy", "7")[0], "zero");
        assert_eq!(ordinal("de", "1")[..2], ["other", "de"]);
        assert_eq!(ordinal("xx", "1")[..2], ["other", "und"]);
        assert_eq!(
            plural_category_line("en", "1", "fractions"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            plural_category_line("en", "one", "cardinal"),
            Err(Refusal::Malformed)
        );
        assert_eq!(
            plural_category_line("en", "99999999999999999999", "cardinal"),
            Err(Refusal::OutOfRange)
        );
    }

    /// CLDR 48's names: German Monday is *Montag*, Russian September is
    /// *сентября* inside a date and *сентябрь* on its own (`hc-i18n`'s
    /// `names.rs`), the Hebrew calendar names the Adar of a leap year *Adar
    /// II*.
    #[test]
    fn a_locale_names_a_calendar() {
        let find = |text: &str, kind: &str, position: &str| {
            all_cells(text)
                .into_iter()
                .find(|cells| cells[0] == kind && cells[1] == position)
                .map(|cells| cells[2].clone())
        };
        let de = names_lines("de", "gregory", "wide", "format").expect("names");
        assert_eq!(find(&de, "weekday", "1").as_deref(), Some("Montag"));
        assert_eq!(find(&de, "month", "3").as_deref(), Some("März"));
        assert_eq!(find(&de, "day-period", "1").as_deref(), Some("AM"));
        let ru = names_lines("ru", "gregory", "wide", "format").expect("names");
        assert_eq!(find(&ru, "month", "9").as_deref(), Some("сентября"));
        let ru = names_lines("ru", "gregory", "wide", "standalone").expect("names");
        assert_eq!(find(&ru, "month", "9").as_deref(), Some("сентябрь"));
        assert!(
            all_cells(&ru)
                .iter()
                .all(|cells| cells.len() == NAMES_COLUMNS)
        );
        #[cfg(feature = "lunar")]
        {
            let hebrew = names_lines("en", "hebrew", "wide", "format").expect("names");
            assert!(
                all_cells(&hebrew)
                    .iter()
                    .any(|cells| cells[0] == "month-in-leap-year" && cells[2] == "Adar II"),
                "{hebrew}"
            );
        }
        assert_eq!(
            names_lines("de", "gregory", "huge", "format"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            names_lines("de", "gregory", "wide", "middle"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            names_lines("de", "no-such", "wide", "format"),
            Err(Refusal::Unknown)
        );
    }

    /// Unicode's `SpecialCasing` for Turkish (`iyi` is `İYİ`), the default
    /// mapping elsewhere (German `ß` is `SS`), and CLDR's habit of writing
    /// French month names with no capital inside a sentence.
    #[test]
    fn a_locale_cases_a_text() {
        let case = |tag: &str, mode: &str, text: &str| {
            cells(&case_line(tag, mode, text).expect("cased"))[0].to_owned()
        };
        assert_eq!(case("tr", "upper", "iyi"), "İYİ");
        assert_eq!(case("en", "upper", "iyi"), "IYI");
        assert_eq!(case("de", "upper", "straße"), "STRASSE");
        assert_eq!(case("tr", "lower", "IĞDIR"), "ığdır");
        assert_eq!(case("fr", "sentence-start", "janvier"), "Janvier");
        assert_eq!(case("fr", "in-sentence", "Janvier"), "janvier");
        assert_eq!(case("de", "in-sentence", "januar"), "Januar");
        assert_eq!(case("en", "capitalise-first", "2 de enero"), "2 de enero");
        assert_eq!(case_line("en", "title", "x"), Err(Refusal::Unknown));
        let line = case_line("tr-TR", "upper", "i").expect("cased");
        assert_eq!(cells(&line)[1..3], ["upper", "turkic"]);
    }

    /// UAX 9's isolates: an Arabic locale embeds a Latin date in an LTR
    /// isolate (U+2066 to U+2069), an English one embeds Arabic in an RTL
    /// one (U+2067), and a field running the locale's own direction is left
    /// alone.
    #[test]
    fn a_text_is_isolated_for_its_locale() {
        let isolate = |tag: &str, mode: &str, text: &str| {
            all_cells(&isolate_line(tag, mode, text).expect("isolated")).remove(0)
        };
        assert_eq!(
            isolate("ar", "field", "Sep 21")[..],
            ["\u{2066}Sep 21\u{2069}", "rtl", "ltr", "1", "field"]
        );
        assert_eq!(
            isolate("en", "field", "Sep 21")[..],
            ["Sep 21", "ltr", "ltr", "0", "field"]
        );
        assert_eq!(
            isolate("en", "field", "سبتمبر")[..],
            ["\u{2067}سبتمبر\u{2069}", "ltr", "rtl", "1", "field"]
        );
        assert_eq!(
            isolate("en", "field", "2026")[..],
            ["2026", "ltr", "", "0", "field"]
        );
        assert_eq!(isolate("en", "first-strong", "x")[0], "\u{2068}x\u{2069}");
        assert_eq!(isolate("en", "strip", "\u{2068}x\u{2069}")[0], "x");
        assert_eq!(isolate_line("en", "wrap", "x"), Err(Refusal::Unknown));
    }
}
