//! The tab-separated lines the WebAssembly module and the C library write
//! about `hc-i18n`'s locale data on its own, written once: the day periods
//! of a time of day, a number in a numbering system and back, the
//! numbering systems, and the eras of a calendar.

use alloc::string::String;

use hc_i18n::day_periods::{self, FlexibleDayPeriod};
use hc_i18n::names::{self, DayPeriod, NameContext, NameWidth};
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
}
