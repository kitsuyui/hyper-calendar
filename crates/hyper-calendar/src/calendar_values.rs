//! The single numbers the WebAssembly module and the C library answer
//! about the lunar and regional calendars, written once: the modern
//! Olympiad of a year, the Hebrew anniversaries of a day, and the Chinese
//! reckoned age and marriage augury.
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
use hc_calendars_lunar::chinese::{self, ChineseCalendar, MarriageAugury};
use hc_calendars_lunar::hebrew::{self, HebrewDate};
use hc_calendars_regional::olympiad;

use crate::boundary::{Answer, Refusal};

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
    Ok(alloc::format!(
        "{}\t{}\t{}\n",
        marriage_augury_name(augury),
        u8::from(augury.lichun_at_start()),
        u8::from(augury.lichun_at_end())
    ))
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
    /// 4661) and double-spring year of 2009 (4646).
    #[test]
    fn the_published_widow_and_double_spring_years_cross() {
        assert_eq!(
            chinese_marriage_augury_line(4_661).as_deref(),
            Ok("widow\t0\t0\n")
        );
        assert_eq!(
            chinese_marriage_augury_line(4_646).as_deref(),
            Ok("double-bright\t1\t1\n")
        );
        assert_eq!(
            chinese_marriage_augury_line(i64::MAX),
            Err(Refusal::OutOfRange)
        );
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
}
