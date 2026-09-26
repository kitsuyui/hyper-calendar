//! A minimal lunisolar month-and-day derivation, here only because 六曜 and
//! 十五夜 cannot be computed without one.
//!
//! The derivation, how it differs from the full calculation and where the
//! differences fall are written up with 六曜 in
//! `docs/systems/zassetsu-and-rokuyo.md` in the repository.
//!
//! # This is not the lunisolar calendar
//!
//! `hc-calendars-lunar` owns the Chinese, Dangi and Japanese lunisolar
//! calendars. This derivation is kept beside them rather than replaced by
//! them, for two reasons.
//!
//! **Cost.** Routed through `hc-calendars-lunar`, every call here becomes a
//! full lunisolar conversion with new moon and solar term searches behind it.
//! 六曜 asks for a month and day on every single date, which takes the crate's
//! own test suite from seconds to more than ten minutes.
//!
//! **Coherence.** 六曜, 不成就日 and 二十七宿 must agree with one another.
//! Routing some of them through one derivation and some through another makes
//! the module contradict itself, and routing all of them pays the cost above
//! on every annotation.
//!
//! So this stays, and the difference is measured rather than assumed. With
//! the `lunar` feature on, `exact_lunisolar_day` reads the same day from the
//! Japanese 旧暦 and the test at the bottom counts how often the two differ.
//! It is a comparison, not a substitution: nothing routes through it.
//!
//! What is implemented is the 中気 rule in its plainest form, as the
//! 暦Wiki's 「太陰太陽暦/置閏法」 states it (`nao-rekiwiki-chijun`,
//! <https://eco.mtk.nao.ac.jp/koyomi/wiki/C2C0B1A2C2C0CDDBCEF12FC3D6B1BCCBA1.html>,
//! retrieved 2026-09-26), with the 定気 terms:
//!
//! * A month begins on the day containing a new moon, read at the caller's
//!   meridian.
//! * A month is numbered after the 中気 it contains, 雨水 (330°) naming the
//!   first month, 春分 (0°) the second, and so on round to 大寒 (300°) naming
//!   the twelfth.
//! * A month containing no 中気 is a leap month and repeats the number of the
//!   month before it.
//! * New moons and 中気 are compared by day, not by time of day.
//!
//! # What that rule gets wrong
//!
//! Under 定気 a month can contain two 中気, and the plain rule does not say
//! which names it; this code takes the later, month by month. The 天保暦 adds
//! that the months containing 冬至, 春分, 夏至 and 秋分 must be the eleventh,
//! second, fifth and eighth, which settles most such months; the Chinese
//! rule instead takes the first 中気-less month between two winter
//! solstices thirteen months apart as the leap month (国立天文台 暦計算室,
//! 「旧暦2033年問題について」, `nao-topics-2014-2033`,
//! <https://eco.mtk.nao.ac.jp/koyomi/topics/html/topics2014.html>, retrieved
//! 2026-09-26). In 2033–34 no numbering satisfies the 天保暦 rule: the months
//! holding 秋分 and 冬至 are only one month apart. This is the 旧暦2033年問題,
//! the first such case since the 天保暦 took effect in 1844, and since the
//! calendar is abolished no public body will choose among the three
//! resolutions the Observatory tabulates (暦Wiki 「太陰太陽暦/2033年問題」,
//! `nao-rekiwiki-2033`,
//! <https://eco.mtk.nao.ac.jp/koyomi/wiki/C2C0B1A2C2C0CDDBCEF12F2033C7AFCCE4C2EA.html>,
//! retrieved 2026-09-26). No official lunisolar calculation is made in Japan
//! today (国立天文台, 「「旧暦」ってなに？」, `nao-faq-kyureki`,
//! <https://www.nao.ac.jp/faq/a0304.html>, retrieved 2026-09-26). None of
//! these rules is implemented here.
//!
//! When the two-中気 case bites, a whole month is numbered differently, and
//! every 六曜 in it moves: the test at the bottom counts 89 of the 3,653 days
//! of 2024–2033, all in one run from 25 August to 21 November 2033. The
//! caller who needs them right wants `hc-calendars-lunar`.

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
/// When a month contains two — which the true solar terms make possible
/// near perihelion, where the 中気 are closest together — this returns the
/// later, which is a simplification; see the module documentation.
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

    /// One month start from a table of the Observatory's: the Gregorian
    /// day, the month number and whether it is a leap month.
    type TabulatedMonth = ((i64, u8, u8), u8, bool);

    /// The months of 2014, as the 暦Wiki's 「太陰太陽暦/置閏法」
    /// (`nao-rekiwiki-chijun`) and the Observatory's 「旧暦2033年問題について」
    /// (`nao-topics-2014-2033`) both tabulate them, with the leap ninth month.
    const MONTHS_2014: [TabulatedMonth; 14] = [
        ((2014, 1, 1), 12, false),
        ((2014, 1, 31), 1, false),
        ((2014, 3, 1), 2, false),
        ((2014, 3, 31), 3, false),
        ((2014, 4, 29), 4, false),
        ((2014, 5, 29), 5, false),
        ((2014, 6, 27), 6, false),
        ((2014, 7, 27), 7, false),
        ((2014, 8, 25), 8, false),
        ((2014, 9, 24), 9, false),
        ((2014, 10, 24), 9, true),
        ((2014, 11, 22), 10, false),
        ((2014, 12, 22), 11, false),
        ((2015, 1, 20), 12, false),
    ];

    /// The months of 1984–85 in the Observatory's table
    /// (`nao-topics-2014-2033`): the month from 22 December 1984 holds both
    /// 冬至 and 大寒, and the 天保暦 rule makes it the eleventh.
    const MONTHS_1984: [TabulatedMonth; 8] = [
        ((1984, 8, 27), 8, false),
        ((1984, 9, 25), 9, false),
        ((1984, 10, 24), 10, false),
        ((1984, 11, 23), 10, true),
        ((1984, 12, 22), 11, false),
        ((1985, 1, 21), 12, false),
        ((1985, 2, 20), 1, false),
        ((1985, 3, 21), 2, false),
    ];

    #[test]
    fn the_months_of_2014_are_the_ones_the_observatory_tabulates() {
        for ((year, month, day), number, leap) in MONTHS_2014 {
            let start = from_year_month_day(year, month, day);
            assert_eq!(month_start_containing(start, JAPAN), start, "{start}");
            assert_eq!(month_number(start, JAPAN), (number, leap), "{start}");
        }
    }

    /// The known disagreement, pinned: in the month that holds two 中気
    /// this derivation takes the later, 大寒, and numbers it the twelfth
    /// where the Observatory's table has the eleventh, and the error runs
    /// on until the next month with a single 中気. The months before it,
    /// the leap tenth month included, agree.
    #[test]
    fn a_month_with_two_principal_terms_is_numbered_after_the_later() {
        // What this derivation gives for the same eight months.
        let here = [
            (8, false),
            (9, false),
            (10, false),
            (10, true),
            (12, false),
            (1, false),
            (1, true),
            (2, false),
        ];
        let mut disagreements = Vec::new();
        for (((year, month, day), number, leap), expected) in MONTHS_1984.into_iter().zip(here) {
            let start = from_year_month_day(year, month, day);
            assert_eq!(month_start_containing(start, JAPAN), start, "{start}");
            assert_eq!(month_number(start, JAPAN), expected, "{start}");
            if (number, leap) != expected {
                disagreements.push(start);
            }
        }
        assert_eq!(
            disagreements,
            [
                from_year_month_day(1984, 12, 22),
                from_year_month_day(1985, 1, 21),
                from_year_month_day(1985, 2, 20),
            ]
        );
    }

    /// 2033–34, the 旧暦2033年問題: the Observatory's three resolutions
    /// number the months from 25 August 2033 as 8, 9, 10, 11, 閏11, 12, 1
    /// (案1), 閏7, 8, 9, 10, 11, 12, 閏1 (案2) or 8, 9, 10, 11, 12, 1, 閏1
    /// (案3) (`nao-rekiwiki-2033`). Month by month, taking the later 中気,
    /// this derivation gives none of them: it has no tenth month and two
    /// leap months. Its eighth month is 案2's, so its 中秋の名月 is 案2's 7
    /// October and not 案1's 8 September (`nao-topics-2014-2033`).
    #[test]
    fn the_months_of_2033_follow_none_of_the_observatorys_resolutions() {
        let expected = [
            ((2033, 8, 25), 7, true),
            ((2033, 9, 23), 8, false),
            ((2033, 10, 23), 9, false),
            ((2033, 11, 22), 11, false),
            ((2033, 12, 22), 11, true),
            ((2034, 1, 20), 1, false),
            ((2034, 2, 19), 1, true),
            ((2034, 3, 20), 2, false),
        ];
        for ((year, month, day), number, leap) in expected {
            let start = from_year_month_day(year, month, day);
            assert_eq!(month_start_containing(start, JAPAN), start, "{start}");
            assert_eq!(month_number(start, JAPAN), (number, leap), "{start}");
        }
        assert_eq!(
            ordinary_date_in_gregorian_year(2033, 8, 15, JAPAN),
            Some(from_year_month_day(2033, 10, 7))
        );
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

/// The same day read from `hc-calendars-lunar`'s unbounded Japanese Tenpō
/// engine, for comparison.
///
/// That engine numbers the months from the winter solstice and takes the
/// first 中気-less month of a thirteen-month year as the leap month, at the
/// Japanese meridian. In 2033–34 it gives 閏11月, the first of the three
/// resolutions the Observatory tabulates, and it agrees with the
/// Observatory's tables of 2014 and 1984–85 (`nao-topics-2014-2033`); it is
/// the better answer where the two derivations disagree, but not an
/// official one, since there is none.
///
/// **Nothing in this crate routes through it.** See the module documentation
/// for why. It exists so the difference can be counted instead of guessed,
/// and so a caller who wants the exact article knows where to get it.
///
/// It takes no meridian: the Tenpō calendar carries its own, Kyoto's
/// 135°46′E to 1872, the almanacs' Tokyo time for 1873–1887 and Japan
/// Standard Time's 135°E from 1888 on.
#[cfg(feature = "lunar")]
#[must_use]
pub fn exact_lunisolar_day(day: Rd) -> Option<LunisolarDay> {
    use hc_calendar::Calendar as _;

    let date = hc_calendars_lunar::japanese_tenpo::UNBOUNDED
        .from_fixed(day)
        .ok()?;
    Some(LunisolarDay {
        month: date.month.ordinal,
        leap_month: date.month.leap,
        day: date.day,
        month_start: Rd(day.0 - i64::from(date.day) + 1),
    })
}

#[cfg(all(test, feature = "lunar"))]
mod divergence_tests {
    use super::*;

    /// How far this derivation is from the 旧暦 it approximates.
    ///
    /// Computed here, beside both implementations, rather than in a
    /// downstream crate. If the figure moves, one of the two changed.
    #[test]
    fn the_difference_from_the_real_calendar_is_counted_not_assumed() {
        let start = hc_calendar::gregorian::to_fixed(2024, 1, 1).unwrap();
        let end = hc_calendar::gregorian::to_fixed(2033, 12, 31).unwrap();

        let mut compared = 0usize;
        let mut differing = 0usize;
        for rd in start.0..=end.0 {
            let day = Rd(rd);
            let Some(exact) = exact_lunisolar_day(day) else {
                continue;
            };
            let here = lunisolar_day(day, Meridian::JAPAN);
            compared += 1;
            if (exact.month, exact.leap_month, exact.day) != (here.month, here.leap_month, here.day)
            {
                differing += 1;
            }
        }

        println!("simplified vs 旧暦: {differing} of {compared} days differ");
        assert!(compared > 3_000, "compared only {compared} days");
        // Pinned, so a change in either implementation shows up rather than
        // being absorbed. The differences come in whole-month runs.
        assert!(
            (60..=120).contains(&differing),
            "expected roughly ninety differing days, got {differing} of {compared}"
        );
    }

    /// The full calculation agrees with the Observatory's tables: 2014, with
    /// its leap ninth month, as this derivation does too; 1984–85, with its
    /// two-中気 month, where this derivation does not; and 2033–34, where it
    /// gives the first resolution, 閏11月 (`nao-topics-2014-2033`,
    /// `nao-rekiwiki-2033`).
    #[test]
    fn the_full_calculation_follows_the_observatorys_tables() {
        let months = [
            ((2014, 9, 24), 9, false),
            ((2014, 10, 24), 9, true),
            ((2014, 11, 22), 10, false),
            ((1984, 11, 23), 10, true),
            ((1984, 12, 22), 11, false),
            ((1985, 1, 21), 12, false),
            ((1985, 2, 20), 1, false),
            ((2033, 8, 25), 8, false),
            ((2033, 9, 23), 9, false),
            ((2033, 10, 23), 10, false),
            ((2033, 11, 22), 11, false),
            ((2033, 12, 22), 11, true),
            ((2034, 1, 20), 12, false),
            ((2034, 2, 19), 1, false),
        ];
        for ((year, month, day), number, leap) in months {
            let start = hc_calendar::gregorian::to_fixed(year, month, day).unwrap();
            let exact = exact_lunisolar_day(start).unwrap();
            assert_eq!(
                (exact.month, exact.leap_month, exact.day),
                (number, leap, 1),
                "{start}"
            );
        }
    }

    #[test]
    fn the_comparison_is_against_the_japanese_reckoning_not_the_chinese_one() {
        // Japan computes the 旧暦 at 135°E and China at 120°E. That hour
        // moves month boundaries on its own, so comparing against the
        // Chinese calendar would conflate a meridian difference with a
        // method difference — it roughly triples the apparent error.
        let new_year_2024 = hc_calendar::gregorian::to_fixed(2024, 2, 10).unwrap();
        let exact = exact_lunisolar_day(new_year_2024).unwrap();
        assert_eq!((exact.month, exact.day, exact.leap_month), (1, 1, false));
    }
}
