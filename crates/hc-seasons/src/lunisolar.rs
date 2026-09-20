//! A minimal lunisolar month-and-day derivation, here only because 六曜 and
//! 十五夜 cannot be computed without one.
//!
//! # This is not the lunisolar calendar
//!
//! `hc-calendars-lunar` is the crate that owns the Chinese, Dangi and
//! Japanese lunisolar calendars, and when it lands everything below should be
//! deleted and the two callers re-pointed at it. It is duplicated here
//! because this crate must not depend on it, and because 六曜 is a function
//! of the lunisolar month and day and of nothing else.
//!
//! What is implemented is the modern 定気 rule in its textbook form:
//!
//! * A month begins on the day containing a new moon, read at the caller's
//!   meridian.
//! * A month is numbered after the 中気 it contains, 雨水 (330°) naming the
//!   first month, 春分 (0°) the second, and so on round to 大寒 (300°) naming
//!   the twelfth.
//! * A month containing no 中気 is a leap month and repeats the number of the
//!   month before it.
//!
//! # What that rule gets wrong
//!
//! Japan's 天保暦 and the modern Chinese rule both add conditions this does
//! not implement. The leap month is properly the *first* 中気-less month
//! after the eleventh, determined by looking at the whole year between two
//! winter solstices; a month can occasionally contain two 中気, which the
//! 天保暦 handles by a rule that has been known since 1844 to be ambiguous
//! and which is why Japan's own official calendar has no legal lunisolar
//! definition today. Neither case is handled here: this code decides month by
//! month, takes the later 中気 when a month holds two, and says so.
//!
//! For 六曜 — a six-day cycle that resets on the first of each month — those
//! edge cases move at most a handful of days a century, and the caller who
//! needs them right wants `hc-calendars-lunar`.

use hc_astro::solar::solar_longitude;
use hc_astro::{new_moon_at_or_after, new_moon_before};
use hc_calendar::Rd;
use hc_core::math::floor;

use crate::meridian::Meridian;

/// How many degrees of solar longitude separate two 中気.
const DEGREES_PER_PRINCIPAL_TERM: f64 = 30.0;

/// The index, in units of 30°, of 雨水 at 330° — the 中気 that names the
/// first lunisolar month.
const FIRST_MONTH_PRINCIPAL_INDEX: i64 = 11;

/// A day's position in the lunisolar calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LunisolarDay {
    /// The month number, 1 to 12.
    ///
    /// A leap month repeats the number of the month it follows, so this is
    /// not unique within a year; `leap_month` distinguishes them.
    pub month: u8,
    /// Whether this is a leap month (閏月).
    pub leap_month: bool,
    /// The day of the month, 1 to 30.
    pub day: u8,
    /// The day the month began, i.e. the day of its new moon.
    pub month_start: Rd,
}

/// The day of the last new moon at or before a given day, at a meridian.
///
/// "At or before" is by *day*: if the conjunction happened at eleven at night
/// on the day asked about, that day is the answer.
#[must_use]
pub fn new_moon_day_on_or_before(day: Rd, meridian: Meridian) -> Rd {
    // Searching back from the instant local midnight ends the day finds the
    // last conjunction whose own local day is at most `day`.
    meridian.day_of(new_moon_before(meridian.midnight(Rd(day.0 + 1))))
}

/// The day the lunisolar month containing a given day began.
#[must_use]
pub fn month_start_containing(day: Rd, meridian: Meridian) -> Rd {
    new_moon_day_on_or_before(day, meridian)
}

/// The day the lunisolar month after the one starting at `start` begins.
#[must_use]
pub fn next_month_start(start: Rd, meridian: Meridian) -> Rd {
    meridian.day_of(new_moon_at_or_after(meridian.midnight(Rd(start.0 + 1))))
}

/// The 中気 contained in a month, as its index in units of 30° of solar
/// longitude, or `None` when the month contains none and is therefore a leap
/// month.
///
/// When a month contains two — which the true solar terms make possible by a
/// hair near perihelion — this returns the later, which is a simplification;
/// see the module documentation.
#[must_use]
pub fn principal_term_index(start: Rd, next_start: Rd, meridian: Meridian) -> Option<u8> {
    let opening = solar_longitude(meridian.midnight(start));
    let closing = solar_longitude(meridian.midnight(next_start));
    let first = floor(opening / DEGREES_PER_PRINCIPAL_TERM) as i64;
    let last = floor(closing / DEGREES_PER_PRINCIPAL_TERM) as i64;
    if first == last {
        None
    } else {
        Some(last.rem_euclid(12) as u8)
    }
}

/// The lunisolar month number named by a 中気 at a given 30° index.
///
/// 雨水 at 330° (index 11) names month 1, so the mapping is a rotation.
#[must_use]
pub const fn month_number_from_principal_index(index: u8) -> u8 {
    let rotated = (index as i64 - FIRST_MONTH_PRINCIPAL_INDEX + 1).rem_euclid(12);
    if rotated == 0 { 12 } else { rotated as u8 }
}

/// The number of the lunisolar month beginning on a day, and whether it is a
/// leap month.
///
/// A leap month takes the number of the month before it, so this may have to
/// step backwards; leap months are never adjacent, so it steps at most once
/// in practice and the loop is bounded at three for safety.
#[must_use]
pub fn month_number(start: Rd, meridian: Meridian) -> (u8, bool) {
    let mut cursor = start;
    let mut leap = false;
    for _ in 0..3 {
        let next = next_month_start(cursor, meridian);
        if let Some(index) = principal_term_index(cursor, next, meridian) {
            return (month_number_from_principal_index(index), leap);
        }
        leap = true;
        cursor = new_moon_day_on_or_before(Rd(cursor.0 - 1), meridian);
    }
    // Unreachable for any real date: three 中気-less months in a row would
    // mean the Sun had stopped. Reporting month 1 beats panicking.
    (1, leap)
}

/// The lunisolar month, leap flag and day of a fixed day.
///
/// ```
/// use hc_seasons::{Meridian, lunisolar::lunisolar_day};
///
/// // 10 February 2024 was the Chinese and Japanese lunar new year:
/// // the first day of the first month.
/// let date = lunisolar_day(hc_calendar::Rd(738_926), Meridian::JAPAN);
/// assert_eq!((date.month, date.day, date.leap_month), (1, 1, false));
/// ```
#[must_use]
pub fn lunisolar_day(day: Rd, meridian: Meridian) -> LunisolarDay {
    let month_start = month_start_containing(day, meridian);
    let (month, leap_month) = month_number(month_start, meridian);
    LunisolarDay {
        month,
        leap_month,
        day: (day.0 - month_start.0 + 1) as u8,
        month_start,
    }
}

/// The fixed day of a lunisolar month and day in a Gregorian year, if that
/// date occurs in it.
///
/// The search walks the new moons of the Gregorian year, so a lunisolar date
/// that falls in January and belongs to the *previous* lunisolar year is
/// found under the Gregorian year it actually lands in. 十五夜 and 十三夜 are
/// the two callers.
#[must_use]
pub fn ordinary_date_in_gregorian_year(
    year: i64,
    month: u8,
    day: u8,
    meridian: Meridian,
) -> Option<Rd> {
    let year_start = crate::gregorian::new_year(year);
    let year_end = crate::gregorian::new_year(year + 1);
    let mut start = month_start_containing(year_start, meridian);
    // Thirteen new moons can begin inside a Gregorian year and a fourteenth
    // can be the one the year opened in, so fourteen steps covers it.
    for _ in 0..14 {
        let (number, leap) = month_number(start, meridian);
        let candidate = Rd(start.0 + i64::from(day) - 1);
        if number == month
            && !leap
            && candidate < next_month_start(start, meridian)
            && candidate >= year_start
            && candidate < year_end
        {
            return Some(candidate);
        }
        start = next_month_start(start, meridian);
        if start >= year_end {
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::from_year_month_day;

    const JAPAN: Meridian = Meridian::JAPAN;

    /// Published lunar new years: the first day of the first lunisolar month.
    /// These are the dates the Chinese and Japanese new year fell on, which
    /// every almanac and every news report agrees about.
    const LUNAR_NEW_YEARS: [(i64, u8, u8); 12] = [
        (2015, 2, 19),
        (2016, 2, 8),
        (2017, 1, 28),
        (2018, 2, 16),
        (2019, 2, 5),
        (2020, 1, 25),
        (2021, 2, 12),
        (2022, 2, 1),
        (2023, 1, 22),
        (2024, 2, 10),
        (2025, 1, 29),
        (2026, 2, 17),
    ];

    #[test]
    fn the_lunar_new_year_is_the_first_day_of_the_first_month() {
        for (year, month, day) in LUNAR_NEW_YEARS {
            let rd = from_year_month_day(year, month, day);
            let date = lunisolar_day(rd, JAPAN);
            assert_eq!(
                (date.month, date.day, date.leap_month),
                (1, 1, false),
                "{year}-{month:02}-{day:02} was not the first of the first month"
            );
            // And the day before is the last day of the twelfth month.
            let eve = lunisolar_day(Rd(rd.0 - 1), JAPAN);
            assert_eq!(eve.month, 12, "the eve of the new year was not in month 12");
            assert!((29..=30).contains(&eve.day));
        }
    }

    #[test]
    fn a_month_starts_on_the_day_of_its_new_moon() {
        for offset in 0..800 {
            let day = Rd(739_000 + offset);
            let date = lunisolar_day(day, JAPAN);
            assert_eq!(date.month_start, month_start_containing(day, JAPAN));
            assert_eq!(
                lunisolar_day(date.month_start, JAPAN).day,
                1,
                "the month start was not day 1"
            );
            assert!((1..=30).contains(&date.day), "day {} of a month", date.day);
            assert!((1..=12).contains(&date.month));
        }
    }

    #[test]
    fn lunisolar_months_are_twenty_nine_or_thirty_days_long() {
        let mut start = month_start_containing(Rd(730_000), JAPAN);
        for _ in 0..400 {
            let next = next_month_start(start, JAPAN);
            let length = next.0 - start.0;
            assert!(
                (29..=30).contains(&length),
                "a month of {length} days beginning at {start}"
            );
            start = next;
        }
    }

    #[test]
    fn the_day_of_the_month_counts_up_and_resets_at_each_new_moon() {
        let mut previous = lunisolar_day(Rd(739_000), JAPAN);
        for offset in 1..1_200 {
            let date = lunisolar_day(Rd(739_000 + offset), JAPAN);
            if date.month_start == previous.month_start {
                assert_eq!(date.day, previous.day + 1);
            } else {
                assert_eq!(date.day, 1, "a new month did not start at day 1");
                assert_eq!(
                    date.month_start.0,
                    previous.month_start.0 + i64::from(previous.day)
                );
            }
            previous = date;
        }
    }

    /// Month 11 is the one containing the winter solstice; that is the
    /// definition the whole numbering hangs off, so it is worth pinning
    /// separately from the new-year dates.
    #[test]
    fn the_month_containing_the_winter_solstice_is_the_eleventh() {
        for year in 1990..2040 {
            let solstice =
                crate::solar_terms::term_day(year, crate::SolarTerm::WINTER_SOLSTICE, JAPAN);
            let date = lunisolar_day(solstice, JAPAN);
            assert_eq!(
                date.month, 11,
                "the solstice of {year} fell in month {}",
                date.month
            );
            assert!(!date.leap_month);
        }
    }

    #[test]
    fn the_month_containing_the_spring_equinox_is_the_second() {
        for year in 1990..2040 {
            let equinox =
                crate::solar_terms::term_day(year, crate::SolarTerm::SPRING_EQUINOX, JAPAN);
            let date = lunisolar_day(equinox, JAPAN);
            assert_eq!(
                date.month, 2,
                "the equinox of {year} fell in month {}",
                date.month
            );
        }
    }

    #[test]
    fn the_principal_term_index_names_the_month_it_should() {
        // 雨水 at 330 degrees is index 11 and names month 1; 冬至 at 270 is
        // index 9 and names month 11.
        assert_eq!(month_number_from_principal_index(11), 1);
        assert_eq!(month_number_from_principal_index(0), 2);
        assert_eq!(month_number_from_principal_index(9), 11);
        assert_eq!(month_number_from_principal_index(10), 12);
        for index in 0..12u8 {
            let number = month_number_from_principal_index(index);
            assert!((1..=12).contains(&number));
        }
        // The mapping is a bijection.
        let mut seen = [false; 13];
        for index in 0..12u8 {
            let number = month_number_from_principal_index(index) as usize;
            assert!(!seen[number]);
            seen[number] = true;
        }
    }

    /// Leap months happen seven times in nineteen years — the Metonic cycle —
    /// so a century should hold about 37 of them, and none should be adjacent
    /// to another.
    #[test]
    fn leap_months_occur_at_the_metonic_rate() {
        let mut start = month_start_containing(from_year_month_day(1950, 1, 1), JAPAN);
        let end = from_year_month_day(2050, 1, 1);
        let mut leaps = 0;
        let mut months = 0;
        let mut previous_was_leap = false;
        while start < end {
            let (_, leap) = month_number(start, JAPAN);
            if leap {
                assert!(!previous_was_leap, "two leap months in a row at {start}");
                leaps += 1;
            }
            previous_was_leap = leap;
            months += 1;
            start = next_month_start(start, JAPAN);
        }
        assert!(
            (1_230..1_250).contains(&months),
            "{months} months in a century"
        );
        assert!(
            (33..=40).contains(&leaps),
            "{leaps} leap months in a century is not the Metonic rate"
        );
    }

    /// 2023 had a leap second month (閏二月) in the Chinese and Japanese
    /// calendars: it began on 22 March 2023 and contained no 中気.
    #[test]
    fn the_leap_second_month_of_2023_is_found() {
        let start = from_year_month_day(2023, 3, 22);
        let (number, leap) = month_number(start, JAPAN);
        assert_eq!(number, 2);
        assert!(leap, "2023-03-22 did not start a leap month");
        let date = lunisolar_day(start, JAPAN);
        assert_eq!((date.month, date.leap_month, date.day), (2, true, 1));
        // The ordinary second month ran before it.
        let ordinary = lunisolar_day(Rd(start.0 - 1), JAPAN);
        assert_eq!((ordinary.month, ordinary.leap_month), (2, false));
    }

    #[test]
    fn a_named_lunisolar_date_is_found_in_the_year_it_falls_in() {
        // The eighth month's fifteenth day, 中秋の名月, fell on 17 September
        // 2024.
        assert_eq!(
            ordinary_date_in_gregorian_year(2024, 8, 15, JAPAN),
            Some(from_year_month_day(2024, 9, 17))
        );
        let found = ordinary_date_in_gregorian_year(2024, 8, 15, JAPAN).unwrap();
        let date = lunisolar_day(found, JAPAN);
        assert_eq!((date.month, date.day, date.leap_month), (8, 15, false));
    }

    #[test]
    fn a_thirtieth_day_is_reported_only_when_the_month_has_one() {
        for year in 2000..2030 {
            if let Some(day) = ordinary_date_in_gregorian_year(year, 8, 30, JAPAN) {
                let date = lunisolar_day(day, JAPAN);
                assert_eq!((date.month, date.day), (8, 30));
            }
        }
    }

    /// The meridian matters for the lunisolar calendar in exactly the way it
    /// matters for the solar terms: a conjunction near local midnight lands
    /// on different days in Tokyo and Beijing, and the whole month then
    /// shifts. This is why Chinese and Japanese new year occasionally differ.
    #[test]
    fn tokyo_and_beijing_sometimes_begin_a_month_on_different_days() {
        let mut disagreements = 0;
        let mut start = month_start_containing(from_year_month_day(1950, 1, 1), Meridian::JAPAN);
        let end = from_year_month_day(2050, 1, 1);
        while start < end {
            if new_moon_day_on_or_before(start, Meridian::CHINA) != start {
                disagreements += 1;
            }
            start = next_month_start(start, Meridian::JAPAN);
        }
        assert!(
            disagreements > 0,
            "the two meridians never disagreed about a new moon day"
        );
    }
}
