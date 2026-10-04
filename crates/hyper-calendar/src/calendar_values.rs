//! The single numbers the WebAssembly module and the C library answer
//! about the lunar and regional calendars, written once: the modern
//! Olympiad of a year, the Hebrew anniversaries of a day and a Hebrew
//! year's place in the sabbatical cycle, the Chinese reckoned age and
//! marriage augury, how the calendar of the province of Asia writes a
//! day, unnumbered days included, and the day and the moment the year
//! changes at the solar New Year of the Burmese, Khmer and Lao calendars.
//!
//! A Hebrew date crosses the boundary as the fixed day it names — the day
//! whose daylight carries it; an event after sunset belongs to the next
//! fixed day, since the Hebrew day begins at sunset — so that no month
//! numbering or leap flag has to be agreed on. The rules are
//! [`hc_calendars_lunar::hebrew::yahrzeit`]'s and
//! [`hc_calendars_lunar::hebrew::birthday`]'s, Reingold and Dershowitz's,
//! which their documentation says are the book's and not a ruling.

use alloc::string::String;

use hc_calendar::{Calendar, Rd};
use hc_calendars_lunar::babylonian;
use hc_calendars_lunar::chinese::{self, ChineseCalendar, MarriageAugury};
use hc_calendars_lunar::hebrew::{self, HebrewDate};
use hc_calendars_regional::chinese_regnal::{self, Dynasty};
use hc_calendars_regional::korean_regnal;
use hc_calendars_regional::nengo::{self, Certainty, Court};
use hc_calendars_regional::olympiad::{self, GamesStatus};
use hc_calendars_regional::{burmese, khmer, lao};
use hc_calendars_solar::asian::{AsianCalendar, WrittenDay};

use crate::boundary::{Answer, Line, Refusal, line};

/// The number of the modern Olympiad a Gregorian year belongs to, from 1
/// for 1896–1899, by [`olympiad::ioc_olympiad`].
///
/// # Errors
///
/// [`Refusal::OutOfRange`] before 1896.
pub fn ioc_olympiad(gregorian_year: i64) -> Answer<i64> {
    olympiad::ioc_olympiad(gregorian_year).ok_or(Refusal::OutOfRange)
}

/// The Hebrew date a fixed day names.
fn hebrew_date(fixed: i64) -> Answer<HebrewDate> {
    let (year, month, day) = hebrew::from_fixed(Rd(fixed)).map_err(|_| Refusal::OutOfRange)?;
    Ok(HebrewDate { year, month, day })
}

/// The fixed day of the anniversary in Hebrew year `year` of a death on
/// the Hebrew date of `death_fixed`: the *yahrzeit*.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day or a year outside the Hebrew years
/// [`hebrew::MIN_YEAR`] to [`hebrew::MAX_YEAR`].
pub fn hebrew_yahrzeit(death_fixed: i64, year: i64) -> Answer<i64> {
    hebrew::yahrzeit(hebrew_date(death_fixed)?, year)
        .map(|day| day.0)
        .map_err(|_| Refusal::OutOfRange)
}

/// The fixed day of the anniversary in Hebrew year `year` of a birth on
/// the Hebrew date of `birth_fixed`.
///
/// # Errors
///
/// As [`hebrew_yahrzeit`].
pub fn hebrew_birthday(birth_fixed: i64, year: i64) -> Answer<i64> {
    hebrew::birthday(hebrew_date(birth_fixed)?, year)
        .map(|day| day.0)
        .map_err(|_| Refusal::OutOfRange)
}

/// The place of a Hebrew year in the seven-year sabbatical cycle, 1 to 7,
/// the seventh being the sabbatical year, *shemittah*, by
/// [`hebrew::sabbatical_cycle_year`]: 5782 and 5789 are sabbatical years.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside the Hebrew years [`hebrew::MIN_YEAR`]
/// to [`hebrew::MAX_YEAR`].
pub fn hebrew_sabbatical_cycle_year(year: i64) -> Answer<i64> {
    hebrew::sabbatical_cycle_year(year)
        .map(i64::from)
        .map_err(|_| Refusal::OutOfRange)
}

/// How many columns [`asian_day_line`] writes.
pub const ASIAN_DAY_COLUMNS: usize = 5;

/// The line of `hc_asian_day`: a fixed day in the calendar of the Roman
/// province of Asia as the calendar writes it — the Julian year, AD, in
/// which the Asian year began; the month, 1 for Kaisar through 12 for
/// Hyperberetaios; the month's name; `unnumbered` for a day before day 1,
/// Sebaste in a 31-day month and in a leap Xandikos Sebaste and the
/// intercalary day, or `numbered`; and the day's number, 1 to 30, or for
/// an unnumbered day its place among them, 1 or 2.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside the days the calendar carries, from
/// 23 September AD 4 to the end of the Asian year 9999.
pub fn asian_day_line(fixed: i64) -> Answer<String> {
    let date = AsianCalendar
        .from_fixed(Rd(fixed))
        .map_err(|_| Refusal::OutOfRange)?;
    let (written, number) = match date.written_day() {
        WrittenDay::Unnumbered(place) => ("unnumbered", place),
        WrittenDay::Numbered(number) => ("numbered", number),
    };
    Ok(line(|line| {
        line.value(date.year)
            .value(date.month)
            .value(date.month_name())
            .cell(written)
            .value(number);
    }))
}

/// A person's age as the Chinese count reckons it on a fixed day, one at
/// birth and one more at each Chinese New Year after, by
/// [`chinese::reckoned_age`]. The birth crosses as its fixed day.
///
/// # Errors
///
/// [`Refusal::NoData`] for a day before the birth, which has no age, and
/// [`Refusal::OutOfRange`] for a day outside the Chinese calendar's range.
pub fn chinese_reckoned_age(birth_fixed: i64, on_fixed: i64) -> Answer<u32> {
    let birth = ChineseCalendar
        .from_fixed(Rd(birth_fixed))
        .map_err(|_| Refusal::OutOfRange)?;
    chinese::reckoned_age(birth, Rd(on_fixed))
        .map_err(|_| Refusal::OutOfRange)?
        .ok_or(Refusal::NoData)
}

/// A person's age on `on_fixed`, born on `birth_fixed`, by a count of
/// [`chinese::AgeConvention::ALL`] selected by its identifier:
/// `chinese-age`, `lichun-age`, `new-year-day-age` or `year-age`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a count not named, [`Refusal::NoData`] for a
/// day before the birth, and [`Refusal::OutOfRange`] for a day the count's
/// calendar does not reach: the Chinese calendar's for `chinese-age` and
/// `lichun-age`, the Gregorian range for the other two.
pub fn chinese_age(convention: &str, birth_fixed: i64, on_fixed: i64) -> Answer<u32> {
    let convention = chinese::AgeConvention::by_id(convention).ok_or(Refusal::Unknown)?;
    for fixed in [birth_fixed, on_fixed] {
        crate::civil::Date::from_ordinal(fixed).map_err(|_| Refusal::OutOfRange)?;
    }
    (convention.age)(Rd(birth_fixed), Rd(on_fixed))
        .map_err(|_| Refusal::OutOfRange)?
        .ok_or(Refusal::NoData)
}

/// How many columns each line of [`almanac_solar_terms_lines`] writes.
#[cfg(feature = "seasons")]
pub const ALMANAC_SOLAR_TERM_COLUMNS: usize = 3;

/// The lines of `hc_chinese_almanac_solar_terms`: the days of the
/// twenty-four solar terms the Qing almanac printed in Gregorian `year`,
/// [`chinese::almanac_solar_term_days`], 小寒 first and 冬至 last — the
/// term's position, 1 for 小寒 to 24 for 冬至, its name in traditional
/// Chinese, and the fixed day.
///
/// # Errors
///
/// [`Refusal::NoData`] outside 1645–1733 and in the Dàtǒng years
/// 1667–1669, whose almanac terms were not carried.
#[cfg(feature = "seasons")]
pub fn almanac_solar_terms_lines(year: i64) -> Answer<String> {
    let days = chinese::almanac_solar_term_days(year).ok_or(Refusal::NoData)?;
    let mut term = hc_seasons::solar_terms::SolarTerm::WINTER_SOLSTICE.next();
    let mut out = String::new();
    for (position, day) in days.iter().enumerate() {
        let mut row = crate::boundary::Line::new(&mut out);
        row.value(position + 1)
            .cell(term.chinese_name())
            .value(day.0);
        row.end();
        term = term.next();
    }
    Ok(out)
}

/// The line of `hc_shmuel_tekufah`: a *tekufah* of Shmuel's reckoning in
/// Hebrew year `year`, [`hebrew::shmuel_tekufah_day`] — the fixed day whose
/// Hebrew day it falls in, the next civil day from the reckoning's
/// nightfall ([`hebrew::SHMUEL_NIGHTFALL_MINUTES`], Maimonides' twelve
/// hours of night before midnight), and the minutes of Jerusalem mean time
/// since the midnight of the moment's civil day, as the reckoning's clock
/// reads them; the *tekufah*, `tishrei`, `tevet`, `nisan` or `tammuz`; `1`
/// when the moment is after that nightfall, so that its Hebrew day is the
/// next civil day's, else `0`; and the moment's civil day, the fixed day of
/// Jerusalem mean time it falls on.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a *tekufah* not named, and
/// [`Refusal::OutOfRange`] for a year outside 1 to 9999.
pub fn shmuel_tekufah_line(year: i64, tekufah: &str) -> Answer<String> {
    let tekufah = hebrew::Tekufah::by_id(tekufah).ok_or(Refusal::Unknown)?;
    if !(hebrew::MIN_YEAR..=hebrew::MAX_YEAR).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    let (day, minutes) = hebrew::shmuel_tekufah_day(year, tekufah);
    let after_nightfall = hebrew::shmuel_tekufah_after_nightfall(year, tekufah);
    Ok(line(|line| {
        line.value(day.0)
            .value(minutes)
            .cell(tekufah.id())
            .flag(after_nightfall)
            .value(day.0 - i64::from(after_nightfall));
    }))
}

/// The modern Olympiad a fixed day belongs to: before 1 September 2004
/// from the opening of one Games to the opening of the next, as the
/// Charter defined it then, [`olympiad::ioc_olympiad_before_2004`], and
/// from then by the Gregorian year, as [`olympiad::ioc_olympiad`].
///
/// # Errors
///
/// [`Refusal::OutOfRange`] before the opening of 6 April 1896, and
/// [`Refusal::NoData`] from 10 June to 21 November 1956, which the sources
/// read do not settle.
pub fn ioc_olympiad_on(fixed: i64) -> Answer<i64> {
    let day = Rd(fixed);
    if day >= olympiad::IOC_CHARTER_2004 {
        let year = crate::civil::Date::from_ordinal(fixed)
            .map_err(Refusal::from)?
            .year();
        return olympiad::ioc_olympiad(year).ok_or(Refusal::OutOfRange);
    }
    if let Some(number) = olympiad::ioc_olympiad_before_2004(day) {
        return Ok(number);
    }
    if day < olympiad::IOC_OPENINGS[0].from {
        Err(Refusal::OutOfRange)
    } else {
        Err(Refusal::NoData)
    }
}

/// How many columns each line of [`era_table_lines`] writes.
pub const ERA_TABLE_COLUMNS: usize = 12;

/// The fixed day before the next era of the stream an era is kept in, or
/// before the era lapsed.
///
/// A [`Court::Unified`] era is read in the Northern stream, the one that
/// carried on to the reunion of 1392 and after it: 建武, which the Southern
/// court replaced with 延元 in 1336, runs to the day before 暦応. The last
/// Southern era, 元中, ends the day before the reunion, as the table's
/// documentation says it was abolished then.
fn japanese_last_day(era: &nengo::Nengo) -> Option<i64> {
    era.start?;
    if let Some(lapsed) = era.lapsed {
        return Some(lapsed.0 - 1);
    }
    let court = if era.court == Court::Unified {
        Court::Northern
    } else {
        era.court
    };
    let next = nengo::next_in_stream(era, court)?;
    let start = next.start?;
    Some(
        if era.court == Court::Southern && start >= nengo::NANBOKUCHO_END {
            nengo::NANBOKUCHO_END.0 - 1
        } else {
            start.0 - 1
        },
    )
}

/// The Gregorian year a fixed day falls in.
fn gregorian_year_of(fixed: i64) -> i64 {
    hc_calendar::gregorian::year_from_fixed(Rd(fixed))
}

/// The lines of `hc_era_table`: every era of a table, in order, one a
/// line, with the columns the table has — the era's code, its name in the
/// characters of its source, its reading, its romanisation, the court or
/// dynasty, the Gregorian year its first year (元年) falls in and the year
/// its last falls in where the source gives one, the fixed day it began
/// and the last fixed day it was in force where the source gives them, its
/// status, the first day by another reading, and a note.
///
/// * `japanese`: [`nengo::ALL`], the 248 eras from 大化 to 令和. The code is
///   the era's identifier (`reiwa`, `showa-1312`); the reading is the first
///   the source gives, in hiragana; the court is `unified`, `northern` or
///   `southern`; the first year is the Gregorian year the lunisolar year of
///   元年 begins in, the last year is empty; the start is the day the era
///   was proclaimed, empty where the source knows the month alone, and the
///   last day the day before the next era of its court's stream, before the
///   lapse of 白雉 and 朱鳥, and empty for 令和 and where the start is
///   unknown; the status is `attested`, `disputed` or `month-only`.
/// * `chinese-regnal`: [`chinese_regnal::ALL`], the 37 eras of the Ming, the
///   Southern Ming, the Shun, the Later Jin and the Qing, with the dynasty
///   as `ming`, `southern-ming`, `shun` or `qing`, the Common Era years of
///   its first and last lunisolar years, no start or last day, which the
///   table does not carry and the Datong calendar of the Ming is not in this
///   library to date, the status `kept` or `not-kept`, and the table's note,
///   with the month of a mid-year proclamation.
/// * `korean-regnal`: the three eras of the Korean Empire, with `korean-empire`
///   for the dynasty column, the days they were chosen on, the last day the
///   day before the next era or the annexation, `attested`, and the first
///   day under the reading that backdates 光武 to 1 January 1897 in the
///   column after the status when it differs.
///
/// The names and the dates are the tables' and their sources', as the
/// system documents say; this lists them and decides nothing.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a table not named, by
/// [`hc_core::catalogue::matches`].
pub fn era_table_lines(table: &str) -> Answer<String> {
    let mut out = String::new();
    if hc_core::catalogue::matches(table, "japanese") {
        for era in nengo::ALL.iter() {
            let mut line = crate::boundary::Line::new(&mut out);
            line.cell(era.id)
                .cell(era.kanji)
                .cell(era.reading)
                .cell(era.romaji)
                .cell(match era.court {
                    Court::Unified => "unified",
                    Court::Northern => "northern",
                    Court::Southern => "southern",
                })
                .value(era.start_year)
                .empty()
                .value_or_empty(era.start.map(|day| day.0))
                .value_or_empty(japanese_last_day(era))
                .cell(match era.certainty {
                    Certainty::Attested => "attested",
                    Certainty::Disputed => "disputed",
                    Certainty::MonthOnly => "month-only",
                })
                .empties(2);
            line.end();
        }
    } else if hc_core::catalogue::matches(table, "chinese-regnal") {
        for era in chinese_regnal::ALL.iter() {
            let mut line = crate::boundary::Line::new(&mut out);
            line.cell(era.id)
                .cell(era.hanzi)
                .cell(era.pinyin)
                .cell(era.pinyin)
                .cell(match era.dynasty {
                    Dynasty::Ming => "ming",
                    Dynasty::SouthernMing => "southern-ming",
                    Dynasty::Shun => "shun",
                    Dynasty::Qing => "qing",
                })
                .value(era.start_year)
                .value(era.end_year)
                .empties(2)
                .cell(if era.in_use { "kept" } else { "not-kept" })
                .empty();
            match era.proclaimed_month {
                Some(month) if !era.note.is_empty() => {
                    line.cell_with(|out| {
                        let _ = out
                            .write_fmt(format_args!("proclaimed in month {month}; {}", era.note));
                    });
                }
                Some(month) => {
                    line.cell_with(|out| {
                        let _ = out.write_fmt(format_args!("proclaimed in month {month}"));
                    });
                }
                None => {
                    line.cell(era.note);
                }
            }
            line.end();
        }
    } else if hc_core::catalogue::matches(table, "korean-regnal") {
        for (position, era) in korean_regnal::ALL.iter().enumerate() {
            let last = korean_regnal::ALL
                .get(position + 1)
                .map_or(korean_regnal::LATEST.0, |next| next.start.0 - 1);
            let mut line = crate::boundary::Line::new(&mut out);
            line.cell(era.id)
                .cell(era.hanja)
                .cell(era.hangul)
                .cell(era.romanised)
                .cell("korean-empire")
                .value(era.start_year)
                .value(gregorian_year_of(last))
                .value(era.start.0)
                .value(last)
                .cell("attested");
            if era.backdated_start == era.start {
                line.empty();
            } else {
                line.value(era.backdated_start.0);
            }
            line.empty();
            line.end();
        }
    } else {
        return Err(Refusal::Unknown);
    }
    Ok(out)
}

/// How many columns each line of [`olympic_games_lines`] writes.
pub const OLYMPIC_GAMES_COLUMNS: usize = 6;

/// The lines of `hc_olympic_games`: the modern Games of a season, one a
/// line in order, as [`olympiad::SUMMER_GAMES`] and
/// [`olympiad::WINTER_GAMES`] list them from Olympedia's editions — the
/// number, which a Summer Games not held keeps and a Winter Games not held
/// has none of, the year they were awarded to (2020 for the Tokyo Games
/// held in 2021), the host city as Olympedia spells it, whether they were
/// `celebrated`, `not-held` or `scheduled`, and the fixed days of the
/// opening and closing ceremonies, each empty where Olympedia dates none.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a season other than `summer` or `winter`, in any
/// case.
pub fn olympic_games_lines(season: &str) -> Answer<String> {
    let table: &[olympiad::Games] = if hc_core::catalogue::matches(season, "summer") {
        &olympiad::SUMMER_GAMES
    } else if hc_core::catalogue::matches(season, "winter") {
        &olympiad::WINTER_GAMES
    } else {
        return Err(Refusal::Unknown);
    };
    let mut out = String::new();
    for games in table {
        let mut line = crate::boundary::Line::new(&mut out);
        line.value_or_empty(games.number)
            .value(games.year)
            .cell(games.host)
            .cell(match games.status {
                GamesStatus::Celebrated => "celebrated",
                GamesStatus::NotHeld => "not-held",
                GamesStatus::Scheduled => "scheduled",
            })
            .value_or_empty(games.opening.map(|day| day.0))
            .value_or_empty(games.closing.map(|day| day.0));
        line.end();
    }
    Ok(out)
}

/// The line of `hc_babylonian_regnal_year`: the king and the regnal year
/// labelling a Seleucid year, as van Gent's converter of Parker and
/// Dubberstein's table labels it, [`babylonian::regnal_year`].
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside SE −314 to
/// [`babylonian::LAST_REGNAL_YEAR`].
pub fn babylonian_regnal_year_line(seleucid_year: i64) -> Answer<String> {
    let (king, year) = babylonian::regnal_year(seleucid_year).ok_or(Refusal::OutOfRange)?;
    Ok(line(|line| {
        line.cell(king).value(year);
    }))
}

/// The name, the naming's identifier and English name, and its authority,
/// of a day of a calendar by one of its namings, or `None` for a day the
/// naming gives no name.
type DayName = (&'static str, &'static str, &'static str, &'static str);

/// A day's name in a naming, from the calendar's month and day.
fn named<const N: usize>(
    naming: &hc_calendar::shape::Naming<N>,
    index: Option<usize>,
) -> Option<DayName> {
    let name = naming.names.get(index?)?;
    Some((name, naming.id, naming.english_name, naming.authority))
}

/// The day-namings each calendar family carries, by the naming's
/// identifier: the French Republican calendars' `fr-fabre-1793`, `fr` and
/// `en`; the Armenian calendar's `hy` and `hy-Latn`.
fn day_name_of(calendar: &str, naming: &str, month: u8, day: u8) -> Answer<Option<DayName>> {
    use hc_calendars_solar::{armenian, french_republican_days as days};
    use hc_core::catalogue::matches;
    let index = days::day_index(month, day);
    if calendar.starts_with("french-republican") {
        for list in [&days::FABRE_1793, &days::IN_USE, &days::ENGLISH] {
            if matches(naming, list.id) {
                return Ok(named(list, index));
            }
        }
    } else if calendar == "armenian" {
        if matches(naming, armenian::DAY_NAMES.id) {
            return Ok(if month == 13 {
                named(
                    &armenian::EPAGOMENAL_DAY_NAMES,
                    usize::from(day).checked_sub(1),
                )
            } else {
                named(&armenian::DAY_NAMES, usize::from(day).checked_sub(1))
            });
        }
        if matches(naming, armenian::DAY_NAMES_ROMANISED.id) {
            return Ok(if month == 13 {
                None
            } else {
                named(
                    &armenian::DAY_NAMES_ROMANISED,
                    usize::from(day).checked_sub(1),
                )
            });
        }
    }
    Err(Refusal::Unknown)
}

/// The line of `hc_day_name`: the name of a fixed day in a calendar whose
/// days are named, by one of its namings — the French Republican
/// calendars' Fabre d'Églantine's table of 1793, `fr-fabre-1793`, the list
/// in use, `fr`, or English Wikipedia's gloss of it, `en`; the Armenian
/// calendar's Armenian names, `hy`, or their romanisation, `hy-Latn`. The
/// cells: the name, the naming's identifier and English name, and its
/// authority.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a calendar the registry does not carry, one
/// whose days are not named, or a naming it does not have;
/// [`Refusal::OutOfRange`] for a day the calendar refuses; and
/// [`Refusal::NoData`] for a day the naming gives no name, an Armenian
/// epagomenal day in `hy-Latn`.
pub fn day_name_line(calendar: &str, naming: &str, fixed: i64) -> Answer<String> {
    let registry = crate::registry();
    let found = registry.get_by_name(calendar).ok_or(Refusal::Unknown)?;
    let id = found.meta().id.0;
    let fields = found.fixed_to_fields(Rd(fixed)).map_err(Refusal::from)?;
    let (month, day) = (
        fields.month.map_or(0, |month| month.ordinal),
        fields.day.unwrap_or(0),
    );
    let (name, naming_id, naming_name, authority) =
        day_name_of(id, naming, month, day)?.ok_or(Refusal::NoData)?;
    Ok(line(|line| {
        line.cell(name)
            .cell(naming_id)
            .cell(naming_name)
            .cell(authority);
    }))
}

/// A calendar's `new_year_margin`: how close an equinox came to the moment
/// of the day that decides the calendar's new year.
#[cfg(feature = "equinox")]
type Margin = fn(i64) -> hc_calendar::CalendarResult<f64>;

/// The calendars whose new year an equinox decides, each with its
/// `new_year_margin`.
#[cfg(feature = "equinox")]
const EQUINOX_MARGINS: [(&str, Margin); 5] = {
    use hc_calendars_equinox::{bahai, french_republican, jalali, persian, persian_apparent_noon};
    [
        (persian::ID.0, persian::new_year_margin),
        (
            persian_apparent_noon::ID.0,
            persian_apparent_noon::new_year_margin,
        ),
        ("jalali", jalali::new_year_margin),
        (bahai::ID.0, bahai::new_year_margin),
        (french_republican::ID.0, french_republican::new_year_margin),
    ]
};

/// The line of `hc_equinox_new_year_margin`: how far the equinox that
/// begins a year of a calendar that reckons its new year by one fell from
/// the moment of the day that decides it, in minutes — for `persian` the
/// nearer noon of Iran Standard Time, for `persian-apparent-noon` and
/// `jalali` the nearer apparent noon at Tehran and at Isfahan, positive
/// before it and negative after; for `bahai-astronomical` the Tehran
/// sunset, positive before it; for `french-republican-equinox` the nearer
/// Paris apparent midnight, always positive — then the calendar's
/// identifier. A margin within the few minutes the astronomy is good to
/// marks a year the calendar decides by a model.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other calendar, and
/// [`Refusal::OutOfRange`] for a year outside its range.
#[cfg(feature = "equinox")]
pub fn equinox_new_year_margin_line(calendar: &str, year: i64) -> Answer<String> {
    let (id, margin) = EQUINOX_MARGINS
        .iter()
        .find(|(id, _)| hc_core::catalogue::matches(calendar, id))
        .ok_or(Refusal::Unknown)?;
    let minutes = margin(year).map_err(|_| Refusal::OutOfRange)?;
    Ok(line(|line| {
        line.value(minutes).cell(id);
    }))
}

/// The word a [`MarriageAugury`] is written as: the published code's
/// names, `widow`, `blind`, `bright` and `double-bright`.
#[must_use]
pub const fn marriage_augury_name(augury: MarriageAugury) -> &'static str {
    match augury {
        MarriageAugury::Widow => "widow",
        MarriageAugury::Blind => "blind",
        MarriageAugury::Bright => "bright",
        MarriageAugury::DoubleBright => "double-bright",
    }
}

/// The line of `hc_chinese_marriage_augury`: the augury of a Chinese year,
/// then `1` or `0` for whether 立春 falls after its New Year and whether
/// another falls before the next.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] when the year or the next begins outside the
/// Chinese calendar's range.
pub fn chinese_marriage_augury_line(chinese_year: i64) -> Answer<String> {
    // The years the calendar's range holds, checked here because
    // `chinese::new_year` does its arithmetic on the year before it checks
    // the range, and a year near the ends of an `i64` overflows there.
    let year_of = |day: Rd| {
        ChineseCalendar
            .from_fixed(day)
            .map(|date| date.year)
            .map_err(|_| Refusal::OutOfRange)
    };
    if !(year_of(chinese::EARLIEST)?..=year_of(chinese::LATEST)?).contains(&chinese_year) {
        return Err(Refusal::OutOfRange);
    }
    let augury = chinese::marriage_augury(chinese_year).map_err(|_| Refusal::OutOfRange)?;
    let names = augury.chinese_names();
    let joined = |part: fn(&chinese::AuguryName) -> &'static str| {
        names
            .iter()
            .map(part)
            .collect::<alloc::vec::Vec<_>>()
            .join(";")
    };
    Ok(line(|line| {
        line.cell(marriage_augury_name(augury))
            .flag(augury.lichun_at_start())
            .flag(augury.lichun_at_end())
            .cell(&joined(|name| name.name))
            .cell(&joined(|name| name.locale))
            .cell(&joined(|name| name.region.unwrap_or("")));
    }))
}

/// How many columns [`solar_new_year_line`] writes.
pub const SOLAR_NEW_YEAR_COLUMNS: usize = 9;

/// The line of `hc_solar_new_year`: the day and the moment at which the
/// year changes at the solar New Year of a Southeast Asian calendar, in the
/// calendar's own arithmetic — `burmese`, the *atat* moment of Thingyan
/// that opens the Myanmar year ([`burmese::thingyan`]); `khmer`, the
/// *Laeung Sak* at which the Jolak Sakaraj, the animal year and the *sak*
/// change ([`khmer::laeung_sak`]); and `lao`, the New Year's day of
/// Dupertuis's table ([`lao::new_year_day`]). `year` is the year as the
/// calendar numbers it: the Myanmar Era for `burmese`, the Buddhist Era
/// for `khmer` and the Chulasakarat for `lao`, as each calendar's own
/// functions take it.
///
/// Tab-separated: the calendar; the year as given; the fixed day on which
/// the new year begins by the country's clock (the day after *atat* for
/// `burmese`, the *Laeung Sak* day for `khmer`, the table's day for `lao`);
/// the seconds after midnight at which the year changes, which the Khmer
/// announcements give to the second for the *Laeung Sak* and the Lao
/// arithmetic gives as the same quantity, and which is empty for `burmese`,
/// whose source states the *atat* moment as a Julian Date and no
/// announcement of the clock time was read to hold it to; the Chulasakarat
/// year that begins, empty for `burmese`, whose year is the one given; and,
/// for `burmese`, the festival's days — *akyo*, the
/// eve; *akya*, its first day; the last *akyat*; and *atat*, the day the
/// old year ends — empty for the other two. The *Maha Songkran* moment at
/// which the Khmer and Lao festivals begin, two to three days before the
/// year changes, is not carried: the Cambodian *hora*'s rule for the true
/// Sun (Roath Kim Soeun, *Pratitin Soryakkatik-Chankatik*) was not read,
/// as `docs/systems/khmer-chhankitek.md` states. Thailand's Songkran is
/// the three Gregorian days its holiday table lists, and Sri Lanka's
/// *Aluth Avurudu* moment is not carried, the Ministry's *Avurudu Nekath
/// Seettuwa* not having been read, as `docs/calendars.md` states.
///
/// # Errors
///
/// [`Refusal::Unknown`] for another calendar, and [`Refusal::OutOfRange`]
/// for a year outside the calendar's: 1 to 3000 ME, 2444 to 2744 BE, and
/// 1301 to 1401 CS.
pub fn solar_new_year_line(calendar: &str, year: i64) -> Answer<String> {
    let mut out = String::new();
    let mut cells = Line::new(&mut out);
    if hc_core::catalogue::matches(calendar, "burmese") {
        if !(burmese::MIN_YEAR..=burmese::MAX_YEAR).contains(&year) {
            return Err(Refusal::OutOfRange);
        }
        let festival = burmese::thingyan(year);
        cells
            .cell("burmese")
            .value(year)
            .value(festival.new_year.0)
            .empties(2)
            .value(festival.akyo.0)
            .value(festival.akya.0)
            .value(festival.last_akyat.0)
            .value(festival.atat.0);
    } else {
        let change = if hc_core::catalogue::matches(calendar, "khmer") {
            khmer::laeung_sak(year)
        } else if hc_core::catalogue::matches(calendar, "lao") {
            lao::new_year_day(year)
        } else {
            return Err(Refusal::Unknown);
        }
        .ok_or(Refusal::OutOfRange)?;
        cells
            .cell(if hc_core::catalogue::matches(calendar, "khmer") {
                "khmer"
            } else {
                "lao"
            })
            .value(year)
            .value(change.day.0)
            .value(change.seconds)
            .value(change.chulasakarat_year)
            .empties(4);
    }
    cells.end();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::Month;

    #[test]
    fn the_games_of_2020_and_2021_are_the_xxxii_olympiad() {
        assert_eq!(ioc_olympiad(2020), Ok(32));
        assert_eq!(ioc_olympiad(2021), Ok(32));
        assert_eq!(ioc_olympiad(1896), Ok(1));
        assert_eq!(ioc_olympiad(1895), Err(Refusal::OutOfRange));
    }

    fn gregorian(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    /// Wikipedia's child born in June 2000, 13 *suì* from the lunar new
    /// year of 2012, 23 January, as `chinese`'s test reads it.
    #[test]
    fn a_child_born_in_june_2000_is_thirteen_from_the_new_year_of_2012() {
        let birth = gregorian(2000, 6, 15);
        assert_eq!(chinese_reckoned_age(birth, gregorian(2012, 1, 23)), Ok(13));
        assert_eq!(chinese_reckoned_age(birth, gregorian(2012, 1, 22)), Ok(12));
        assert_eq!(chinese_reckoned_age(birth, birth), Ok(1));
        assert_eq!(chinese_reckoned_age(birth, birth - 1), Err(Refusal::NoData));
        assert_eq!(
            chinese_reckoned_age(i64::MIN, birth),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            chinese_reckoned_age(birth, i64::MAX),
            Err(Refusal::OutOfRange)
        );
    }

    /// The South China Morning Post's widow year of 2024 (Chinese year
    /// 4661) and double-spring year of 2009 (4646), with the Chinese names
    /// Wikipedia's 「立春」 and the Hong Kong Observatory give them.
    #[test]
    fn the_published_widow_and_double_spring_years_cross() {
        assert_eq!(
            chinese_marriage_augury_line(4_661).as_deref(),
            Ok("widow\t0\t0\t無春年;寡婦年;盲年;无春年;寡妇年;盲年\t\
                zh-Hant;zh-Hant;zh-Hant;zh-Hans;zh-Hans;zh-Hans\t;north;south;;north;south\n")
        );
        assert_eq!(
            chinese_marriage_augury_line(4_646).as_deref(),
            Ok("double-bright\t1\t1\t雙春兼閏月;双春年\tzh-Hant;zh-Hans\t;\n")
        );

        assert_eq!(
            chinese_marriage_augury_line(i64::MAX),
            Err(Refusal::OutOfRange)
        );
    }

    /// Wikipedia's and Chabad.org's sabbatical years, 5782 (2021–22) and
    /// 5789 (2028–29).
    #[test]
    fn the_published_sabbatical_years_are_the_seventh() {
        assert_eq!(hebrew_sabbatical_cycle_year(5_782), Ok(7));
        assert_eq!(hebrew_sabbatical_cycle_year(5_789), Ok(7));
        assert_eq!(hebrew_sabbatical_cycle_year(5_787), Ok(5));
        assert_eq!(hebrew_sabbatical_cycle_year(1), Ok(1));
        assert_eq!(hebrew_sabbatical_cycle_year(0), Err(Refusal::OutOfRange));
        assert_eq!(
            hebrew_sabbatical_cycle_year(10_000),
            Err(Refusal::OutOfRange)
        );
    }

    fn julian(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::julian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    /// The decree's New Year, 23 September, is Sebaste of Kaisar; the
    /// Metropolis *hemerologion* puts 7 October on day 14 of Kaisar; and a
    /// leap Xandikos, year 99's, opens on 21 and 22 February AD 100 with
    /// two unnumbered days.
    #[test]
    fn sebaste_and_the_numbered_days_are_written_apart() {
        assert_eq!(
            asian_day_line(julian(50, 9, 23)).as_deref(),
            Ok("50\t1\tKaisar\tunnumbered\t1\n")
        );
        assert_eq!(
            asian_day_line(julian(50, 10, 7)).as_deref(),
            Ok("50\t1\tKaisar\tnumbered\t14\n")
        );
        assert_eq!(
            asian_day_line(julian(100, 2, 22)).as_deref(),
            Ok("99\t6\tXandikos\tunnumbered\t2\n")
        );
        assert_eq!(
            asian_day_line(julian(100, 2, 23)).as_deref(),
            Ok("99\t6\tXandikos\tnumbered\t1\n")
        );
        assert_eq!(asian_day_line(julian(4, 9, 22)), Err(Refusal::OutOfRange));
        assert_eq!(asian_day_line(i64::MAX), Err(Refusal::OutOfRange));
        // The range the boundary crates' READMEs state.
        assert_eq!(julian(4, 9, 23), 1_360);
        assert_eq!(julian(10_000, 9, 22), 3_652_398);
        assert!(asian_day_line(3_652_398).is_ok());
        assert_eq!(asian_day_line(3_652_399), Err(Refusal::OutOfRange));
    }

    #[test]
    fn an_anniversary_is_the_crates_from_the_day_it_names() {
        let death = hebrew::to_fixed(5780, Month::regular(4), 10).expect("10 Tevet 5780");
        let kept = hebrew::to_fixed(5781, Month::regular(4), 10).expect("10 Tevet 5781");
        assert_eq!(hebrew_yahrzeit(death.0, 5781), Ok(kept.0));
        assert_eq!(hebrew_birthday(death.0, 5781), Ok(kept.0));
        assert_eq!(hebrew_yahrzeit(death.0, 10_000), Err(Refusal::OutOfRange));
        assert_eq!(hebrew_birthday(i64::MIN, 5781), Err(Refusal::OutOfRange));
    }

    /// Wikipedia's child born on 1 June 2000 is 13 *suì* from the lunar
    /// new year of 2012 (`wikipedia-en-east-asian-age-reckoning`); the
    /// *South China Morning Post*'s child born on 1 June 2009 turns two at
    /// 立春, 4 February 2010 (`scmp-double-spring-2009`); a child born on
    /// 31 December is two the next day by the count from 1 January.
    #[test]
    fn each_age_count_is_its_sources() {
        let birth = gregorian(2000, 6, 1);
        assert_eq!(
            chinese_age("chinese-age", birth, gregorian(2012, 1, 23)),
            Ok(13)
        );
        let spring = gregorian(2009, 6, 1);
        assert_eq!(
            chinese_age("lichun-age", spring, gregorian(2010, 2, 3)),
            Ok(1)
        );
        assert_eq!(
            chinese_age("Lichun-Age", spring, gregorian(2010, 2, 4)),
            Ok(2)
        );
        let eve = gregorian(2023, 12, 31);
        assert_eq!(
            chinese_age("new-year-day-age", eve, gregorian(2024, 1, 1)),
            Ok(2)
        );
        assert_eq!(chinese_age("year-age", eve, gregorian(2024, 1, 1)), Ok(1));
        assert_eq!(chinese_age("year-age", eve, eve - 1), Err(Refusal::NoData));
        assert_eq!(chinese_age("korean-age", eve, eve), Err(Refusal::Unknown));
        assert_eq!(
            chinese_age("year-age", i64::MIN, eve),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            chinese_age("lichun-age", i64::MIN, eve),
            Err(Refusal::OutOfRange)
        );
    }

    /// Liu's almanac terms of 1700 start with 小寒 and end with 冬至, and
    /// the Dàtǒng years carry none.
    #[cfg(feature = "seasons")]
    #[test]
    fn the_almanac_terms_run_from_minor_cold() {
        let text = almanac_solar_terms_lines(1700).expect("carried");
        let rows: alloc::vec::Vec<&str> = text.lines().collect();
        assert_eq!(rows.len(), 24);
        assert!(rows[0].starts_with("1\t小寒\t"));
        assert!(rows[23].starts_with("24\t冬至\t"));
        assert_eq!(almanac_solar_terms_lines(1668), Err(Refusal::NoData));
        assert_eq!(almanac_solar_terms_lines(1800), Err(Refusal::NoData));
    }

    /// Maimonides' *tekufat Nisan* of 4930 "on the eighth of Nisan", "on
    /// the night of the fifth day at midnight" (*Hilkhot Kiddush HaChodesh*
    /// 9:5–9:7), which `hc-calendars-lunar` holds; Tishrei 5786 is 7
    /// October 2025.
    #[test]
    fn shmuels_tekufah_is_on_its_day() {
        let line = shmuel_tekufah_line(5_786, "Tishrei").expect("in range");
        assert!(line.starts_with(&alloc::format!("{}\t", gregorian(2025, 10, 7))));
        assert!(line.contains("\ttishrei\t"));
        // 4930's Nisan at midnight of the night of Thursday: after the
        // reckoning's nightfall, on Thursday's Hebrew day, the civil day
        // before.
        let nisan = shmuel_tekufah_line(4_930, "nisan").expect("in range");
        let cells: alloc::vec::Vec<&str> = nisan.trim_end().split('\t').collect();
        assert_eq!(cells[1..4], ["0", "nisan", "0"]);
        assert_eq!(cells[0], cells[4]);
        // The anchor, 5769's Nisan at the reckoning's nightfall on Tuesday
        // 7 April 2009, "the beginning of the night of the fourth day"
        // (Hilkhot Berakhot 10:18): Wednesday's Hebrew day, though the Sun
        // set at Jerusalem after it.
        let anchor = shmuel_tekufah_line(5_769, "nisan").expect("in range");
        let (tuesday, wednesday) = (gregorian(2009, 4, 7), gregorian(2009, 4, 8));
        assert_eq!(
            anchor,
            alloc::format!("{wednesday}\t1080\tnisan\t1\t{tuesday}\n")
        );
        #[cfg(feature = "lunar")]
        {
            let jerusalem = hc_astro::riseset::Location::new(31.78, 35.24, 0.0);
            let sunset =
                hc_astro::riseset::sunset(hc_calendar::Rd(tuesday), jerusalem).expect("a sunset");
            let mean_time = (sunset.0 - tuesday as f64 + 35.24 / 360.0) * 1_440.0;
            assert!(mean_time > 1_080.0, "{mean_time}");
        }
        assert_eq!(shmuel_tekufah_line(5_786, "adar"), Err(Refusal::Unknown));
        assert_eq!(shmuel_tekufah_line(0, "nisan"), Err(Refusal::OutOfRange));
    }

    /// The Charter before 2004 ran an Olympiad from one opening to the
    /// next: 1 January 1900 is still in the I Olympiad, the Paris Games
    /// having no official opening until May; 1956's doubt is refused.
    #[test]
    fn the_olympiad_of_a_day_follows_the_charter_of_its_time() {
        assert_eq!(ioc_olympiad_on(gregorian(1900, 1, 1)), Ok(1));
        assert_eq!(ioc_olympiad_on(gregorian(1956, 8, 1)), Err(Refusal::NoData));
        assert_eq!(
            ioc_olympiad_on(gregorian(1896, 1, 1)),
            Err(Refusal::OutOfRange)
        );
        // The Games of the XXXIII Olympiad were Paris 2024.
        assert_eq!(ioc_olympiad_on(gregorian(2026, 9, 29)), Ok(33));
        assert_eq!(
            babylonian_regnal_year_line(1).as_deref(),
            Ok("Seleucus I Nicator\t1\n")
        );
        assert_eq!(babylonian_regnal_year_line(161), Err(Refusal::OutOfRange));
    }

    /// Fabre d'Églantine's first day is Raisin in the list in use; the
    /// Armenian first day is Արեգ (Areg).
    #[test]
    fn a_named_day_is_named_by_its_list() {
        let first = gregorian(1793, 9, 22);
        let line = day_name_line("french-republican-arithmetic", "fr", first).expect("named");
        assert!(line.starts_with("Raisin\tfr\t"));
        assert_eq!(day_name_line("gregory", "fr", first), Err(Refusal::Unknown));
        assert_eq!(
            day_name_line("french-republican-arithmetic", "hy", first),
            Err(Refusal::Unknown)
        );
    }

    /// `hc-calendars-equinox`'s 183 BE (2026): the equinox under a fifth of
    /// a minute after the Tehran sunset, a year the model does not claim.
    #[cfg(feature = "equinox")]
    #[test]
    fn the_equinox_margin_is_the_calendars() {
        let line = equinox_new_year_margin_line("Bahai-Astronomical", 183).expect("in range");
        let cells = crate::boundary::cells(&line);
        let minutes: f64 = cells[0].parse().expect("minutes");
        assert!((-0.2..0.0).contains(&minutes), "{minutes}");
        assert_eq!(cells[1], "bahai-astronomical");
        assert_eq!(
            equinox_new_year_margin_line("gregory", 2026),
            Err(Refusal::Unknown)
        );
        assert!(equinox_new_year_margin_line("persian", 1404).is_ok());
    }

    fn julian_day(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::julian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    fn table_row<'a>(table: &'a str, code: &str) -> Vec<&'a str> {
        table
            .lines()
            .map(|line| line.split('\t').collect::<Vec<_>>())
            .find(|row| row[0] == code)
            .expect("an era")
    }

    /// Japanese Wikipedia's 元号一覧 (日本), retrieved 2026-10-03: 248 eras
    /// to 令和, which begins on 1 May 2019 and is current; 平成 on 8 January
    /// 1989; 建武 from 5 March 1334 to 1336-04-11 in the south, where 延元
    /// begins, and to 1338-10-11 in the north, where 暦応 begins (Julian
    /// dates, the page's end being the next era's first day); 白雉 to
    /// 655-02-11 and 朱鳥 to 687-02-17, the last days before the gaps with no
    /// era; 元中 to the reunion of 1392-11-19, 明徳 from 1390-04-12 to
    /// 応永's 1394-08-02, and 明治 to 大正's first day, 30 July 1912, so
    /// that the last day of 明治 is the 29th.
    #[test]
    fn the_japanese_table_has_every_era_with_its_first_and_last_day() {
        let table = era_table_lines("Japanese").expect("a table");
        assert_eq!(table.lines().count(), 248);
        assert!(
            table
                .lines()
                .all(|line| line.split('\t').count() == ERA_TABLE_COLUMNS)
        );
        let day = |year, month, date| gregorian(year, month, date).to_string();
        let reiwa = table_row(&table, "reiwa");
        assert_eq!(reiwa[1..5], ["令和", "れいわ", "Reiwa", "unified"]);
        assert_eq!(
            (reiwa[7], reiwa[8], reiwa[9]),
            (day(2019, 5, 1).as_str(), "", "attested")
        );
        let heisei = table_row(&table, "heisei");
        assert_eq!(
            (heisei[7], heisei[8]),
            (day(1989, 1, 8).as_str(), day(2019, 4, 30).as_str())
        );
        let meiji = table_row(&table, "meiji");
        assert_eq!(meiji[8], day(1912, 7, 29));
        let kenmu = table_row(&table, "kenmu");
        assert_eq!(kenmu[4], "unified");
        assert_eq!(kenmu[7], julian_day(1334, 3, 5).to_string());
        assert_eq!(kenmu[8], (julian_day(1338, 10, 11) - 1).to_string());
        assert_eq!(table_row(&table, "engen")[4], "southern");
        assert_eq!(
            table_row(&table, "ryakuo")[7],
            julian_day(1338, 10, 11).to_string()
        );
        assert_eq!(
            table_row(&table, "genchu")[8],
            (julian_day(1392, 11, 19) - 1).to_string()
        );
        let meitoku = table_row(&table, "meitoku");
        assert_eq!(
            (meitoku[4], meitoku[7], meitoku[8]),
            (
                "northern",
                julian_day(1390, 4, 12).to_string().as_str(),
                (julian_day(1394, 8, 2) - 1).to_string().as_str()
            )
        );
        assert_eq!(
            table_row(&table, "hakuchi")[8],
            julian_day(655, 2, 11).to_string()
        );
        assert_eq!(
            table_row(&table, "shucho")[8],
            julian_day(687, 2, 17).to_string()
        );
        assert_eq!(table_row(&table, "showa-1312")[1], "正和");
        assert_eq!(era_table_lines("no-such"), Err(Refusal::Unknown));
        // An era with only its month known has no day to begin or end on.
        for row in table
            .lines()
            .map(|line| line.split('\t').collect::<Vec<_>>())
        {
            if row[9] == "month-only" {
                assert_eq!((row[7], row[8]), ("", ""), "{}", row[0]);
            }
        }
    }

    /// The Chinese eras are year data (`wikipedia-ja-chinese-era-list`):
    /// 康熙 from 1662 to 1722, 祺祥 proclaimed in 1861 and never kept, and
    /// 崇德 from the fourth month of 1636. The Korean Empire's three eras
    /// are the days they were chosen (`sillok-gojong`, `sillok-sunjong`),
    /// 光武 beginning on 14 August 1897, or 1 January by the decree's
    /// backdating, and 隆熙 ending with the annexation of 29 August 1910.
    #[test]
    fn the_chinese_and_korean_eras_are_listed_with_what_their_tables_know() {
        let chinese = era_table_lines("chinese-regnal").expect("a table");
        assert_eq!(chinese.lines().count(), 37);
        assert!(
            chinese
                .lines()
                .all(|line| line.split('\t').count() == ERA_TABLE_COLUMNS)
        );
        let kangxi = table_row(&chinese, "kangxi");
        assert_eq!(
            kangxi[1..7],
            ["康熙", "Kangxi", "Kangxi", "qing", "1662", "1722"]
        );
        assert_eq!((kangxi[7], kangxi[8], kangxi[9]), ("", "", "kept"));
        assert_eq!(table_row(&chinese, "qixiang")[9], "not-kept");
        assert!(table_row(&chinese, "chongde")[11].starts_with("proclaimed in month 4; "));
        let korean = era_table_lines("KOREAN-REGNAL").expect("a table");
        assert_eq!(korean.lines().count(), 3);
        let gwangmu = table_row(&korean, "gwangmu");
        assert_eq!(gwangmu[1..3], ["光武", "광무"]);
        assert_eq!(gwangmu[5..7], ["1897", "1907"]);
        assert_eq!(gwangmu[7], gregorian(1897, 8, 14).to_string());
        assert_eq!(gwangmu[8], (gregorian(1907, 8, 2) - 1).to_string());
        assert_eq!(gwangmu[10], gregorian(1897, 1, 1).to_string());
        let yunghui = table_row(&korean, "yunghui");
        assert_eq!(yunghui[8], gregorian(1910, 8, 29).to_string());
        assert_eq!(yunghui[10], "");
    }

    /// Olympedia's editions (`olympedia-editions`, retrieved 2026-10-03):
    /// the I Games at Athina, 6 to 15 April 1896; the VI not held, Berlin
    /// 1916; the XXXII, awarded to 2020, held at Tokyo from 23 July to
    /// 8 August 2021; Paris 2024, 26 July to 11 August; the Winter Games
    /// of Chamonix 1924, Oslo 1952 as the VI, and Beijing 2022 as the XXIV.
    #[test]
    fn the_games_are_listed_as_olympedia_lists_them() {
        let summer = olympic_games_lines("Summer").expect("a list");
        assert!(
            summer
                .lines()
                .all(|line| line.split('\t').count() == OLYMPIC_GAMES_COLUMNS)
        );
        let row = |text: &str, year: &str| {
            text.lines()
                .map(|line| line.split('\t').map(str::to_owned).collect::<Vec<_>>())
                .find(|row| row[1] == year)
                .expect("a year")
        };
        let day = |year, month, date| gregorian(year, month, date).to_string();
        assert_eq!(
            row(&summer, "1896"),
            [
                "1",
                "1896",
                "Athina",
                "celebrated",
                day(1896, 4, 6).as_str(),
                day(1896, 4, 15).as_str()
            ]
        );
        assert_eq!(
            row(&summer, "1916"),
            ["6", "1916", "Berlin", "not-held", "", ""]
        );
        let tokyo = row(&summer, "2020");
        assert_eq!(
            (
                tokyo[0].as_str(),
                tokyo[3].as_str(),
                tokyo[4].as_str(),
                tokyo[5].as_str()
            ),
            (
                "32",
                "celebrated",
                day(2021, 7, 23).as_str(),
                day(2021, 8, 8).as_str()
            )
        );
        let paris = row(&summer, "2024");
        assert_eq!(
            (paris[4].as_str(), paris[5].as_str()),
            (day(2024, 7, 26).as_str(), day(2024, 8, 11).as_str())
        );
        let winter = olympic_games_lines("winter").expect("a list");
        assert_eq!(row(&winter, "1924")[0..3], ["1", "1924", "Chamonix"]);
        let oslo = row(&winter, "1952");
        assert_eq!(
            (oslo[0].as_str(), oslo[4].as_str()),
            ("6", day(1952, 2, 15).as_str())
        );
        let beijing = row(&winter, "2022");
        assert_eq!(
            (beijing[0].as_str(), beijing[5].as_str()),
            ("24", day(2022, 2, 20).as_str())
        );
        // A Games not held keeps no number among the Winter Games' own.
        assert!(
            winter
                .lines()
                .any(|line| line.contains("not-held") && line.starts_with('\t'))
        );
        assert_eq!(olympic_games_lines("spring"), Err(Refusal::Unknown));
    }
}
