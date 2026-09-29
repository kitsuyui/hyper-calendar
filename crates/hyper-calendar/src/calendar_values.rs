//! The single numbers the WebAssembly module and the C library answer
//! about the lunar and regional calendars, written once: the modern
//! Olympiad of a year, the Hebrew anniversaries of a day and a Hebrew
//! year's place in the sabbatical cycle, the Chinese reckoned age and
//! marriage augury, and how the calendar of the province of Asia writes a
//! day, unnumbered days included.
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
use hc_calendars_regional::olympiad;
use hc_calendars_solar::asian::{AsianCalendar, WrittenDay};

use crate::boundary::{Answer, Refusal, line};

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
}
