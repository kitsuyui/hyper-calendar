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
/// After them the attributes, as Henning's almanacs print them and as
/// Janson's rules give them (Appendix E), each with its number in the
/// second cell:
///
/// * of the lunar day that ends on the calendar day, none on the first of
///   two days with one number: `lunar-day-animal`, the animal's number, 1
///   for the Mouse, and its name; `lunar-day-element`, the element's
///   number, 1 for Wood, its name and, in the last cell, its colour, on
///   the versions whose rule for months is given, as for `month-symbol`;
///   `lunar-day-trigram`, the trigram's number, 1 for *li* to 8 for *zon*
///   in Janson's order, its Tibetan and Chinese names, its direction and
///   the attribute Janson calls its element; and `lunar-day-number`, the
///   number 1 to 9, its colour, its element and its direction in the magic
///   square, the last cell empty;
/// * of the calendar day: `day-trigram` and `day-number-janson`, in those
///   two layouts, on Janson's rules, which no almanac read prints; and
///   `day-number-henning` in the number's layout, the other count Henning's
///   Phugpa and Bhutanese almanacs print after the Chinese mansion, on the
///   versions other than the Tsurphu and the Mongolian, whose almanacs
///   print none (two conventions, one function each);
/// * `chinese-mansion`, 1 for *Jiao* to 28, and its name in Henning's
///   spelling, the other cells empty; and `element-pair`, the number of the
///   weekday the almanac names the day by, which on `tibetan-bhutan` and
///   `tibetan-bhutan-lochen` is the Bhutanese one, a day ahead of the
///   `weekday` line's, then the weekday's element and the mansion's, `Water`
///   and `Earth`, of the four of the Indian system.
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
    push_attributes(&mut out, &calendar, &day, rd);
    Ok(out)
}

/// A trigram's line: the trigram's number, 1 for *li*, its Tibetan and
/// Chinese names, its direction and the attribute Janson calls its element.
fn push_trigram(out: &mut String, kind: &str, trigram: usize) {
    let trigram_row = &almanac::TRIGRAMS[trigram];
    let mut line = Line::new(out);
    line.cell(kind)
        .value(trigram + 1)
        .cell(trigram_row.tibetan)
        .cell(trigram_row.chinese)
        .cell(trigram_row.direction)
        .cell(trigram_row.element);
    line.end();
}

/// A number's line: the number, 1 to 9, its colour, its element and its
/// direction in the magic square, with the last cell empty.
fn push_number(out: &mut String, kind: &str, number: u8) {
    let row = &almanac::NINE_NUMBERS[usize::from(number) - 1];
    let mut line = Line::new(out);
    line.cell(kind)
        .value(number)
        .cell(row.colour)
        .cell(row.element)
        .cell(row.direction)
        .empty();
    line.end();
}

/// The attributes of the day: those of its lunar day, the Chinese-style
/// ones of the calendar day, the Chinese mansion and the pair of elements.
///
/// A lunar day's animal, trigram and number are Janson's rules
/// ([`almanac::lunar_day_attributes`]) and are read as Henning's almanacs
/// print them, on the calendar day the lunar day ends in: the first of two
/// days with one number, in which none ends, has none. The Bhutanese
/// versions are read under the Phugpa's month cycle, which Henning's
/// Bhutanese almanacs of 2000 to 2020 agree with; the lunar day's element
/// rests on the month's, so it is written only where the version's rule
/// for months is given.
fn push_attributes(
    out: &mut String,
    calendar: &TibetanCalendar,
    day: &almanac::AlmanacDay,
    rd: Rd,
) {
    let id = hc_calendar::Calendar::meta(calendar).id.0;
    let bhutan = id.starts_with("tibetan-bhutan");
    let rule = month_cycle(id);
    if !day.date.leap_day
        && let Some(attributes) = almanac::lunar_day_attributes(
            rule.unwrap_or(MonthCycle::Phugpa),
            day.date.year,
            day.date.month.ordinal,
            day.date.day,
        )
    {
        let animal = usize::from(attributes.animal);
        let mut line = Line::new(out);
        line.cell("lunar-day-animal")
            .value(animal + 1)
            .cell(ANIMALS[animal])
            .empties(3);
        line.end();
        if rule.is_some() {
            let element = usize::from(attributes.element);
            let mut line = Line::new(out);
            line.cell("lunar-day-element")
                .value(element + 1)
                .cell(ELEMENTS[element])
                .empties(2)
                .cell(almanac::COLOURS[element]);
            line.end();
        }
        push_trigram(out, "lunar-day-trigram", usize::from(attributes.trigram));
        push_number(out, "lunar-day-number", attributes.number);
    }
    push_trigram(out, "day-trigram", usize::from(almanac::day_trigram(rd)));
    push_number(out, "day-number-janson", almanac::janson_day_number(rd));
    // Henning's computed Phugpa and Bhutanese almanacs print the number
    // after the Chinese mansion; the Tsurphu's print none.
    if rule != Some(MonthCycle::Tsurphu) {
        push_number(
            out,
            "day-number-henning",
            almanac::henning_almanac_day_number(rd),
        );
    }
    let mansion = usize::from(almanac::chinese_mansion(rd));
    let mut line = Line::new(out);
    line.cell("chinese-mansion")
        .value(mansion + 1)
        .cell(almanac::CHINESE_MANSIONS[mansion])
        .empties(3);
    line.end();
    // The pair is read at the weekday the almanac names the day by.
    let weekday = if bhutan {
        almanac::bhutanese_weekday(rd)
    } else {
        day.weekday
    };
    if let Some((weekday_element, mansion_element)) = almanac::element_pair(weekday, day.mansion) {
        let mut line = Line::new(out);
        line.cell("element-pair")
            .value(usize::from(weekday) + 1)
            .cell(almanac::INDIAN_ELEMENTS[usize::from(weekday_element)])
            .cell(almanac::INDIAN_ELEMENTS[usize::from(mansion_element)])
            .empties(2);
        line.end();
    }
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

    fn kind_rows<'a>(text: &'a str, kind: &str) -> Vec<Vec<&'a str>> {
        text.lines()
            .map(|line| line.split('\t').collect::<Vec<_>>())
            .filter(|row| row[0] == kind)
            .collect()
    }

    /// Henning's computed almanacs print, for each day, the two elements,
    /// the Chinese mansion and its number, and the lunar day's animal,
    /// trigram and number (`tdata/pl_2013.txt`, `ts_2013.txt`, the
    /// Bhutanese almanac of 2019; as `hc-calendars-regional`'s
    /// `the_attributes_are_those_hennings_almanacs_print` reads them):
    /// Wednesday 20 February 2013 of the Phugpa is "Water-Earth", *Zhen* 9
    /// and, the 10th of month 1, "Pig, gin 7"; Monday 11 February 2013 of
    /// the Tsurphu is "Water-Earth", *Bi* with no number, and "Tiger, li
    /// 1"; and Wednesday 5 February 2019 of the Bhutanese, a Tuesday in the
    /// world, is "Water-Earth", *Zui* 7 and "Tiger, kham 7".
    #[test]
    fn the_attributes_of_a_day_are_those_the_almanacs_print() {
        let phugpa = almanac_day_lines("tibetan", greg(2013, 2, 20)).expect("in range");
        assert!(
            crate::boundary::cells(phugpa.lines().next().expect("a line")).len()
                == TIBETAN_ALMANAC_COLUMNS
        );
        for row in phugpa.lines() {
            assert_eq!(row.split('\t').count(), TIBETAN_ALMANAC_COLUMNS, "{row}");
        }
        assert_eq!(
            kind_rows(&phugpa, "element-pair")[0][1..4],
            ["5", "Water", "Earth"]
        );
        assert_eq!(kind_rows(&phugpa, "chinese-mansion")[0][2], "Zhen");
        assert_eq!(kind_rows(&phugpa, "day-number-henning")[0][1], "9");
        assert_eq!(
            kind_rows(&phugpa, "lunar-day-animal")[0][1..3],
            ["12", "Pig"]
        );
        assert_eq!(
            kind_rows(&phugpa, "lunar-day-trigram")[0][1..3],
            ["6", "gin"]
        );
        assert_eq!(kind_rows(&phugpa, "lunar-day-number")[0][1], "7");
        // The two numbers of the calendar day are two conventions: 11
        // February 2013 is 9 in the almanac and 8 by Janson's rule.
        let first = almanac_day_lines("tibetan", greg(2013, 2, 11)).expect("in range");
        assert_eq!(kind_rows(&first, "day-number-henning")[0][1], "9");
        assert_eq!(kind_rows(&first, "day-number-janson")[0][1], "8");
        assert_eq!(kind_rows(&first, "day-trigram").len(), 1);

        let tsurphu =
            almanac_day_lines("tibetan-tsurphu-karana", greg(2013, 2, 11)).expect("in range");
        assert_eq!(
            kind_rows(&tsurphu, "element-pair")[0][1..4],
            ["3", "Water", "Earth"]
        );
        assert_eq!(kind_rows(&tsurphu, "chinese-mansion")[0][2], "Bi");
        assert!(kind_rows(&tsurphu, "day-number-henning").is_empty());
        assert_eq!(
            kind_rows(&tsurphu, "lunar-day-animal")[0][1..3],
            ["3", "Tiger"]
        );
        assert_eq!(kind_rows(&tsurphu, "lunar-day-trigram")[0][2], "li");
        assert_eq!(kind_rows(&tsurphu, "lunar-day-number")[0][1], "1");
        // The Tsurphu's month rule is Janson's, so the lunar day has an element.
        assert_eq!(kind_rows(&tsurphu, "lunar-day-element").len(), 1);

        let bhutan = almanac_day_lines("tibetan-bhutan", greg(2019, 2, 5)).expect("in range");
        assert_eq!(
            kind_rows(&bhutan, "element-pair")[0][1..4],
            ["5", "Water", "Earth"]
        );
        assert_eq!(kind_rows(&bhutan, "chinese-mansion")[0][2], "Zui");
        assert_eq!(kind_rows(&bhutan, "day-number-henning")[0][1], "7");
        assert_eq!(kind_rows(&bhutan, "lunar-day-animal")[0][2], "Tiger");
        assert_eq!(kind_rows(&bhutan, "lunar-day-trigram")[0][2], "kham");
        assert_eq!(kind_rows(&bhutan, "lunar-day-number")[0][1], "7");
        // Janson gives no month rule for Bhutan, so no element of the day.
        assert!(kind_rows(&bhutan, "lunar-day-element").is_empty());
        // The first of two days with one number ends no lunar day, and
        // Henning prints none: Saturday 8 February 2019.
        let doubled = almanac_day_lines("tibetan-bhutan", greg(2019, 2, 8)).expect("in range");
        assert!(kind_rows(&doubled, "lunar-day-animal").is_empty());
        assert_eq!(kind_rows(&doubled, "day-trigram").len(), 1);
        assert_eq!(kind_rows(&doubled, "chinese-mansion")[0][2], "Gui");
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
