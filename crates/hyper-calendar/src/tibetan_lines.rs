//! The tab-separated lines the WebAssembly module and the C library write
//! about the Tibetan almanac, written once.
//!
//! Each is [`hc_calendars_regional::tibetan_almanac`]'s arithmetic on a
//! version of the calendar [`hc_calendars_lunar::tibetan::by_id`] finds:
//!
//! * [`almanac_day_lines`]: the five components and the columns after
//!   them for a calendar day, one a line, the kind first, with the rab
//!   byung year, the royal year and the elements and animals;
//! * [`planet_lines`]: the Phugpa planets at the end of a day;
//! * [`bhutanese_winter_solstice_line`]: the Bhutanese calendar's winter
//!   solstice of a Gregorian year;
//! * [`festival_day`]: the day a festival on a Tibetan date is kept, by a
//!   rule for a skipped or repeated number.
//!
//! A longitude or a weekday is written as the almanacs print it, the whole
//! part and two sexagesimal places, `2;11,24`, and beside it as a decimal.
//!
//! `docs/systems/tibetan-almanac.md` describes the components, the planets
//! and the Bhutanese solstice, and `docs/systems/tibetan-calendar-holidays.md`
//! the festival rule for a skipped or repeated day.

use alloc::string::String;

use hc_calendar::{Month, Rd};
use hc_calendars_lunar::tibetan::{
    self, ANIMALS, ELEMENTS, Ratio, TIBETAN_BHUTAN, TibetanCalendar,
};
use hc_calendars_regional::tibetan_almanac::{
    self as almanac, FestivalRule, KARANAS, MANSIONS, MonthCycle, Planet, Symbol, WEEKDAYS, YOGAS,
};

use crate::boundary::{Answer, Line, Refusal};

/// How many columns each line of [`almanac_day_lines`] writes.
pub const TIBETAN_ALMANAC_COLUMNS: usize = 6;

/// The version a registry identifier names.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text.
fn calendar(id: &str) -> Answer<TibetanCalendar> {
    tibetan::by_id(id).ok_or(Refusal::Unknown)
}

/// One line: the kind, the identifier, the Sanskrit or English name and
/// the Tibetan one, the almanac's reading and the value as a decimal.
fn push(
    out: &mut String,
    kind: &str,
    id: Option<usize>,
    names: (&str, &str),
    reading: Option<Ratio>,
) {
    let mut line = Line::new(out);
    line.cell(kind)
        .value_or_empty(id)
        .cell(names.0)
        .cell(names.1)
        .value_or_empty(reading)
        .value_or_empty(reading.map(Ratio::to_f64));
    line.end();
}

/// A symbol's line: its element and animal, `Iron-Horse`; its gender; and
/// the element's colour, as the almanacs name the year.
fn push_symbol(out: &mut String, kind: &str, symbol: Symbol) {
    let mut line = Line::new(out);
    line.cell(kind)
        .value(usize::from(symbol.animal) + 1)
        .value(format_args!(
            "{}-{}",
            ELEMENTS[usize::from(symbol.element)],
            ANIMALS[usize::from(symbol.animal)]
        ))
        .empty()
        .cell(if symbol.male { "male" } else { "female" })
        .cell(almanac::COLOURS[usize::from(symbol.element)]);
    line.end();
}

/// Whose rule names a version's months, where its source says: the
/// Phugpa's for `tibetan` and `tibetan-lochen`, the Tsurphu's, which is
/// the Chinese and the Mongolian one, for `tibetan-tsurphu`,
/// `tibetan-tsurphu-karana` and `mongolian`; none for `tibetan-bhutan` and
/// `tibetan-bhutan-lochen`,
/// whose rule Janson does not give.
fn month_cycle(id: &str) -> Option<MonthCycle> {
    match id {
        "tibetan" | "tibetan-lochen" => Some(MonthCycle::Phugpa),
        "tibetan-tsurphu" | "tibetan-tsurphu-karana" | "mongolian" => Some(MonthCycle::Tsurphu),
        _ => None,
    }
}

/// The lines of `hc_tibetan_almanac_day`: what the almanac of a version
/// prints for a calendar day, one column a line, the kind first.
///
/// The kinds, in order: `weekday`, the day of the week, 1 for Saturday to
/// 7 for Friday, with the true weekday, the end of the lunar day in days
/// after Saturday's dawn, as the reading, empty on the first of two days
/// with one number; `mansion`, the lunar mansion, 1 to 27, with the Moon at
/// daybreak in mansions; `yoga`, 1 to 27, with the yoga longitude;
/// `karana`, 1 to 11 in [`KARANAS`]' order, with no reading; `half-day`,
/// the half of the lunar month in effect at daybreak, 1 to 60, as its
/// identifier; `sun`, the true Sun in mansions; `mean-sun`, the mean Sun in
/// signs; `rahu`, the Head of Rāhu in mansions, on the versions from the
/// epoch of 806 alone; `rab-byung`, the year's name in the sixty-year
/// cycle, 1 for Prabhava; `royal-year`, the year counted from 127 BCE, as
/// the identifier; and `year-symbol`, `month-symbol`, where the version's
/// rule is given, and `day-symbol`, each with its animal's number, 1 for
/// the Mouse, the element and animal, the gender and the colour in place
/// of the reading and the decimal.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a version [`tibetan::by_id`] does not find,
/// and [`Refusal::OutOfRange`] for a day outside its range.
pub fn almanac_day_lines(id: &str, fixed: i64) -> Answer<String> {
    let calendar = calendar(id)?;
    let rd = Rd(fixed);
    let day = almanac::almanac_day(&calendar, rd).map_err(|_| Refusal::OutOfRange)?;
    let mut out = String::new();
    let weekday = usize::from(day.weekday);
    push(
        &mut out,
        "weekday",
        Some(weekday + 1),
        WEEKDAYS[weekday],
        day.lunar_day_end,
    );
    let mansion = usize::from(day.mansion);
    push(
        &mut out,
        "mansion",
        Some(mansion + 1),
        MANSIONS[mansion],
        Some(day.moon),
    );
    let yoga = usize::from(day.yoga);
    push(
        &mut out,
        "yoga",
        Some(yoga + 1),
        YOGAS[yoga],
        Some(day.yoga_longitude),
    );
    let karana = usize::from(day.karana);
    push(&mut out, "karana", Some(karana + 1), KARANAS[karana], None);
    push(
        &mut out,
        "half-day",
        Some(usize::from(day.half_day)),
        ("", ""),
        None,
    );
    push(&mut out, "sun", None, ("", ""), Some(day.sun));
    push(&mut out, "mean-sun", None, ("", ""), Some(day.mean_sun));
    if let Ok(rahu) = almanac::rahu_head(&calendar, day.date.day, day.month_count) {
        push(&mut out, "rahu", None, ("", ""), Some(rahu));
    }
    let year = day.date.year;
    let (tibetan_name, sanskrit) = almanac::rab_byung_name(year);
    let position = tibetan::prabhava_index(year) + 1;
    push(
        &mut out,
        "rab-byung",
        Some(position),
        (sanskrit, tibetan_name),
        None,
    );
    let mut line = Line::new(&mut out);
    line.cell("royal-year")
        .value(almanac::royal_year(year))
        .empties(4);
    line.end();
    push_symbol(&mut out, "year-symbol", almanac::year_symbol(year));
    if let Some(cycle) = month_cycle(hc_calendar::Calendar::meta(&calendar).id.0) {
        push_symbol(
            &mut out,
            "month-symbol",
            almanac::month_symbol(cycle, year, day.date.month.ordinal),
        );
    }
    push_symbol(&mut out, "day-symbol", almanac::day_symbol(rd));
    Ok(out)
}

/// How many columns each line of [`planet_lines`] writes.
pub const TIBETAN_PLANET_COLUMNS: usize = 8;

/// The lines of `hc_tibetan_planets`: where the Phugpa almanac places each
/// of the five planets at the end of a calendar day, from Henning's epoch
/// of 1927 — the planet's identifier, its particular day, and its mean
/// heliocentric, true slow and fast longitudes in mansions, each as the
/// almanac reads it and as a decimal.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside `tibetan`'s range.
pub fn planet_lines(fixed: i64) -> Answer<String> {
    let rd = Rd(fixed);
    tibetan::TIBETAN
        .locate(rd)
        .map_err(|_| Refusal::OutOfRange)?;
    let mut out = String::new();
    for planet in Planet::ALL {
        let place = almanac::planet_place(planet, rd);
        let mut line = Line::new(&mut out);
        line.cell(planet.id()).value(place.particular_day);
        for value in [place.mean_heliocentric, place.true_slow] {
            line.value(value);
        }
        line.value(place.fast)
            .value(place.mean_heliocentric.to_f64())
            .value(place.true_slow.to_f64())
            .value(place.fast.to_f64());
        line.end();
    }
    Ok(out)
}

/// The line of `hc_bhutanese_winter_solstice`: the instant the mean Sun of
/// the Bhutanese calendar reaches 250° in Gregorian `year`,
/// [`almanac::bhutanese_winter_solstice`] — the fixed day it falls on; the
/// weekday and time the almanac prints, days after Saturday's dawn in
/// whole days, nāḍī and pala, `2;51,38`; and the local Julian Date as a
/// decimal.
///
/// The mean Sun's year is 365.270 645 days, so the day drifts a day later
/// every 35½ years against the Gregorian calendar, from 24 December in
/// 1700 to 30 January in 3000, and a year of 365 days can hold none.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a year outside 1000 to 3000, and
/// [`Refusal::NoData`] for one the mean Sun reaches 250° on no day of,
/// 1923, 1927 … 1957.
pub fn bhutanese_winter_solstice_line(year: i64) -> Answer<String> {
    let instant =
        almanac::bhutanese_winter_solstice(&TIBETAN_BHUTAN, year).map_err(|error| match error {
            hc_calendar::CalendarError::DayOutOfRange => Refusal::NoData,
            _ => Refusal::OutOfRange,
        })?;
    let day = Rd::from_julian_day_number(instant.floor() as i64);
    let weekday = tibetan::weekday_of_instant(instant);
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(day.0).value(weekday).value(instant.to_f64());
    line.end();
    Ok(out)
}

/// The fixed day a festival on `day` of `month` (with `leap` for the leap
/// month) of Tibetan `year` is kept on, by a rule of [`FestivalRule::ALL`]
/// selected by its identifier, `berzin` or `henning-almanac`, on a version.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a rule or a version not named;
/// [`Refusal::OutOfRange`] for a year outside 1000 to 3000;
/// [`Refusal::InvalidDate`] for a month the year does not have or a day
/// outside 1 to 30; and [`Refusal::NoData`] where the rule keeps no day,
/// a skipped number under `henning-almanac`.
pub fn festival_day(
    rule: &str,
    id: &str,
    year: i64,
    month: u32,
    leap: bool,
    day: u32,
) -> Answer<i64> {
    let rule = FestivalRule::by_id(rule).ok_or(Refusal::Unknown)?;
    let calendar = calendar(id)?;
    let month = u8::try_from(month).map_err(|_| Refusal::InvalidDate)?;
    let day = u8::try_from(day).map_err(|_| Refusal::InvalidDate)?;
    let month = if leap {
        Month::leap(month)
    } else {
        Month::regular(month)
    };
    (rule.day)(&calendar, year, month, day)
        .map_err(Refusal::from)?
        .map(|kept| kept.0)
        .ok_or(Refusal::NoData)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    fn greg(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    /// Henning's Tsurphu almanac's first day of 2013, as "Open source
    /// Tsurphu calendar software" prints it: Monday, Shatabhishaj/mon gru,
    /// Parigha/yongs 'joms, Vava/gdab pa, 2;11,24 and the yoga's 18;26,38
    /// (`kalacakra-org`); 2013 is the Water-Snake year.
    #[test]
    fn the_tsurphu_programs_worked_day_is_written() {
        let text =
            almanac_day_lines("tibetan-tsurphu-karana", greg(2013, 2, 11)).expect("in range");
        let rows: Vec<Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert!(rows.iter().all(|row| row.len() == TIBETAN_ALMANAC_COLUMNS));
        assert_eq!(
            rows[0][..5],
            ["weekday", "3", "Monday", "zla ba", "2;11,24"]
        );
        assert_eq!(rows[1][2..4], ["Shatabhishaj", "mon gru"]);
        assert_eq!(rows[2][2..5], ["Parigha", "yongs 'joms", "18;26,38"]);
        assert_eq!(rows[3][2..4], ["Vava", "gdab pa"]);
        let year = rows
            .iter()
            .find(|row| row[0] == "year-symbol")
            .expect("a year");
        assert_eq!(year[2..], ["Water-Snake", "", "female", "black"]);
        assert!(!rows.iter().any(|row| row[0] == "rahu"));
        let royal = rows
            .iter()
            .find(|row| row[0] == "royal-year")
            .expect("a royal year");
        assert_eq!(royal[1], "2140");
        assert_eq!(almanac_day_lines("tibetan-x", 0), Err(Refusal::Unknown));
        assert_eq!(almanac_day_lines("tibetan", 0), Err(Refusal::OutOfRange));
        let phugpa = almanac_day_lines("Tibetan", greg(2013, 2, 11)).expect("in range");
        assert!(
            phugpa
                .lines()
                .any(|line| line.starts_with("month-symbol\t"))
        );
        let bhutan =
            almanac_day_lines("tibetan-bhutan-lochen", greg(2013, 2, 11)).expect("in range");
        assert!(
            !bhutan
                .lines()
                .any(|line| line.starts_with("month-symbol\t"))
        );
        assert!(phugpa.lines().any(|line| line.starts_with("rahu\t")));
    }

    /// Henning's Bhutanese almanac of 2001 puts the winter solstice on 1
    /// January, 2;51,38, as `hc-calendars-regional`'s test reads it.
    #[test]
    fn the_bhutanese_winter_solstice_is_the_almanacs() {
        let line = bhutanese_winter_solstice_line(2001).expect("in range");
        let cells = crate::boundary::cells(&line);
        assert_eq!(
            cells[..2],
            [alloc::format!("{}", greg(2001, 1, 1)).as_str(), "2;51,38"]
        );
        assert_eq!(
            bhutanese_winter_solstice_line(3_001),
            Err(Refusal::OutOfRange)
        );
        // The drift of the mean Sun's year: 24 December in 1700, none in
        // 1923, the year between the December solstices and the January
        // ones.
        let cells = |year| {
            bhutanese_winter_solstice_line(year)
                .map(|line| String::from(crate::boundary::cells(&line)[0]))
        };
        assert_eq!(cells(1_700), Ok(alloc::format!("{}", greg(1700, 12, 24))));
        assert_eq!(cells(1_923), Err(Refusal::NoData));
    }

    /// Henning's almanacs do not mark the Birth of the Buddha in 1990,
    /// whose 7th of month 4 is skipped; Berzin's rule keeps it the day
    /// before.
    #[test]
    fn a_skipped_festival_is_kept_by_the_rule() {
        assert_eq!(
            festival_day("henning-almanac", "tibetan-lochen", 1990, 4, false, 7),
            Err(Refusal::NoData)
        );
        let berzin = festival_day("berzin", "tibetan-lochen", 1990, 4, false, 7).expect("a day");
        let almanac = almanac_day_lines("tibetan-lochen", berzin).expect("in range");
        assert!(almanac.lines().next().is_some());
        assert_eq!(
            festival_day("x", "tibetan", 1990, 4, false, 7),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            festival_day("berzin", "tibetan", 1990, 4, false, 31),
            Err(Refusal::InvalidDate)
        );
        let planets = planet_lines(greg(2011, 1, 6)).expect("in range");
        assert_eq!(planets.lines().count(), 5);
        assert!(
            planets
                .lines()
                .nth(2)
                .is_some_and(|line| line.starts_with("mars\t525\t"))
        );
    }
}
