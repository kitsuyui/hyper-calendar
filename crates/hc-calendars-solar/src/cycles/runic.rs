//! The runic calendar — the Swedish runestaff read against the Julian year.
//!
//! A runestaff is a perpetual Julian calendar in three rows: a day-letter
//! rune on every day, the seven runes ᚠ ᚢ ᚦ ᚬ ᚱ ᚴ ᚼ standing for the Sunday
//! letters A–G; a golden-number rune on the days of the new moons, the
//! sixteen runes of the younger futhark and the three extra ones ᛮ ᛯ ᛰ
//! standing for 1–19; and the feast marks. A year is read off it with its
//! Sunday letter, which says which rune its Sundays carry, and its golden
//! number, which says which rune its new moons carry. This module is that
//! reading: [`reading`] for a Julian month and day, [`stave_day`] for a
//! fixed day. It is a reading, beside the golden number and the Sunday
//! letter it is made of, and not a calendar: the staff has no dates of its
//! own to convert.
//!
//! The golden-number row is the old series, the *aureus numerus antiquus*
//! of the medieval Church calendar, which the Swedish staffs before 1690
//! carry; the staffs after 1690 carry a corrected series that no source
//! read tabulates, and it is not here. Nor are the feast marks, which vary
//! from staff to staff, or the Norwegian *primstav*, which seldom has
//! golden numbers at all. The system, its sources and the checks are
//! written up in `docs/systems/runic-calendar.md` in the repository.
//!
//! # Sources
//!
//! * *Explanatory Supplement to the Astronomical Ephemeris and the American
//!   Ephemeris and Nautical Almanac*, HMSO, London, 1961, §14E: table
//!   14.4, "Julian ecclesiastical lunar calendar", p. 422, transcribed
//!   here as [`NEW_MOONS`]; the letters and the intercalary day, p. 421;
//!   example 14.1, Easter 1513, p. 424. Read 2026-09-26 in the scan at
//!   `archive.org/details/astronomicalalmanac1961`.
//! * Carla Cucina, "A Runic Calendar in the Vatican Library", *Futhark* 9–10
//!   (2018–2019), pp. 261–274, read 2026-09-26: the three rows, the seven
//!   day runes as A–G, the nineteen golden-number runes, the old series.
//! * Wikipedia, "Runic calendar", and Swedish Wikipedia, "Runstav",
//!   retrieved 2026-09-26: the table of the golden-number runes, the
//!   medieval order of *l* and *m*, the 235 signs, and the corrected
//!   series after 1690.
//! * The Unicode Standard 17.0, `NamesList.txt`, "Golden number runes":
//!   U+16EE–U+16F0 as golden numbers 17, 18 and 19.
//!
//! # Exactness
//!
//! Exact: a table and the Julian calendar. How well the table's new moons
//! agree with the Moon is the Alexandrian cycle's question, not this
//! module's.

use hc_calendar::{CalendarError, CalendarResult, Rd};

use super::DominicalLetter;
use crate::julian;

/// The day-letter runes, A through G: the first seven runes of the younger
/// futhark, *fé*, *úr*, *þurs*, *óss*, *reið*, *kaun*, *hagall*.
pub const DAY_LETTER_RUNES: [char; 7] = ['ᚠ', 'ᚢ', 'ᚦ', 'ᚬ', 'ᚱ', 'ᚴ', 'ᚼ'];

/// The golden-number runes, 1 through 19: the sixteen runes of the younger
/// futhark in their medieval order, *l* before *m*, and *árlaug*,
/// *tvímaðr* and *belgþórr* for 17, 18 and 19, as Wikipedia's "Runic
/// calendar" tabulates them.
pub const GOLDEN_NUMBER_RUNES: [char; 19] = [
    'ᚠ', 'ᚢ', 'ᚦ', 'ᚬ', 'ᚱ', 'ᚴ', 'ᚼ', 'ᚾ', 'ᛁ', 'ᛅ', 'ᛋ', 'ᛏ', 'ᛒ', 'ᛚ', 'ᛘ', 'ᛦ', 'ᛮ', 'ᛯ', 'ᛰ',
];

/// The Julian ecclesiastical new moons: for each golden number, 1 to 19,
/// and each month, January to December, the days of that month on which a
/// lunation of that year begins, in a common year.
///
/// *Explanatory Supplement* (1961), table 14.4. Read by column, it is the
/// golden-number row of the staff. In a leap year the table moves three
/// February new moons a day later — 6 from the 26th to the 27th, 14 from
/// the 28th to the 29th, 17 from the 25th to the 26th — which [`reading`]
/// does as reading every February day from the 25th as the day before.
pub const NEW_MOONS: [[&[u8]; 12]; 19] = [
    [
        &[23],
        &[21],
        &[23],
        &[21],
        &[21],
        &[19],
        &[19],
        &[17],
        &[16],
        &[15],
        &[14],
        &[13],
    ],
    [
        &[12],
        &[10],
        &[12],
        &[10],
        &[10],
        &[8],
        &[8],
        &[6],
        &[5],
        &[4],
        &[3],
        &[2],
    ],
    [
        &[1, 31],
        &[],
        &[1, 31],
        &[29],
        &[29],
        &[27],
        &[27],
        &[25],
        &[24],
        &[23],
        &[22],
        &[21],
    ],
    [
        &[20],
        &[18],
        &[20],
        &[18],
        &[18],
        &[16],
        &[16],
        &[14],
        &[13],
        &[12],
        &[11],
        &[10],
    ],
    [
        &[9],
        &[7],
        &[9],
        &[7],
        &[7],
        &[5],
        &[5],
        &[3],
        &[2],
        &[2, 31],
        &[30],
        &[29],
    ],
    [
        &[28],
        &[26],
        &[28],
        &[26],
        &[26],
        &[24],
        &[24],
        &[22],
        &[21],
        &[20],
        &[19],
        &[18],
    ],
    [
        &[17],
        &[15],
        &[17],
        &[15],
        &[15],
        &[13],
        &[13],
        &[11],
        &[10],
        &[9],
        &[8],
        &[7],
    ],
    [
        &[6],
        &[4],
        &[6],
        &[5],
        &[4],
        &[3],
        &[2],
        &[1, 30],
        &[29],
        &[28],
        &[27],
        &[26],
    ],
    [
        &[25],
        &[23],
        &[25],
        &[23],
        &[23],
        &[21],
        &[21],
        &[19],
        &[18],
        &[17],
        &[16],
        &[15],
    ],
    [
        &[14],
        &[12],
        &[14],
        &[12],
        &[12],
        &[10],
        &[10],
        &[8],
        &[7],
        &[6],
        &[5],
        &[4],
    ],
    [
        &[3],
        &[2],
        &[3],
        &[2],
        &[1, 31],
        &[29],
        &[29],
        &[27],
        &[26],
        &[25],
        &[24],
        &[23],
    ],
    [
        &[22],
        &[20],
        &[22],
        &[20],
        &[20],
        &[18],
        &[18],
        &[16],
        &[15],
        &[14],
        &[13],
        &[12],
    ],
    [
        &[11],
        &[9],
        &[11],
        &[9],
        &[9],
        &[7],
        &[7],
        &[5],
        &[4],
        &[3],
        &[2],
        &[1, 31],
    ],
    [
        &[30],
        &[28],
        &[30],
        &[28],
        &[28],
        &[26],
        &[26],
        &[24],
        &[23],
        &[22],
        &[21],
        &[20],
    ],
    [
        &[19],
        &[17],
        &[19],
        &[17],
        &[17],
        &[15],
        &[15],
        &[13],
        &[12],
        &[11],
        &[10],
        &[9],
    ],
    [
        &[8],
        &[6],
        &[8],
        &[6],
        &[6],
        &[4],
        &[4],
        &[2],
        &[1],
        &[1, 30],
        &[29],
        &[28],
    ],
    [
        &[27],
        &[25],
        &[27],
        &[25],
        &[25],
        &[23],
        &[23],
        &[21],
        &[20],
        &[19],
        &[18],
        &[17],
    ],
    [
        &[16],
        &[14],
        &[16],
        &[14],
        &[14],
        &[12],
        &[12],
        &[10],
        &[9],
        &[8],
        &[7],
        &[6],
    ],
    [
        &[5],
        &[3],
        &[5],
        &[4],
        &[3],
        &[2],
        &[1, 30],
        &[28],
        &[27],
        &[26],
        &[25],
        &[24],
    ],
];

/// The rune that stands for a Sunday letter.
#[must_use]
pub const fn day_letter_rune(letter: DominicalLetter) -> char {
    DAY_LETTER_RUNES[letter as usize - 1]
}

/// The rune that stands for a golden number, or `None` outside `1..=19`.
#[must_use]
pub const fn golden_number_rune(golden_number: u8) -> Option<char> {
    match golden_number {
        1..=19 => Some(GOLDEN_NUMBER_RUNES[golden_number as usize - 1]),
        _ => None,
    }
}

/// What one day of the staff carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StaveDay {
    /// The day's letter; `None` for 29 February, which has none.
    pub letter: Option<DominicalLetter>,
    /// The golden number of the year whose new moon falls on this day, if
    /// any year's does.
    pub golden_number: Option<u8>,
}

impl StaveDay {
    /// The day-letter rune.
    #[must_use]
    pub const fn letter_rune(self) -> Option<char> {
        match self.letter {
            Some(letter) => Some(day_letter_rune(letter)),
            None => None,
        }
    }

    /// The golden-number rune.
    #[must_use]
    pub const fn golden_number_rune(self) -> Option<char> {
        match self.golden_number {
            Some(number) => golden_number_rune(number),
            None => None,
        }
    }
}

/// Days of a common Julian year before the first of each month.
const DAYS_BEFORE: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

/// The day's letter in a common year: 1 January is A, and the seven run on
/// round the year.
const fn common_letter(month: u8, day: u8) -> DominicalLetter {
    let index = (DAYS_BEFORE[month as usize - 1] + day as u16 - 1) % 7;
    DominicalLetter::ALL[index as usize]
}

/// The golden number whose new moon falls on a day of a common year.
fn common_golden_number(month: u8, day: u8) -> Option<u8> {
    let column = usize::from(month) - 1;
    (1..=19u8).find(|&number| NEW_MOONS[usize::from(number) - 1][column].contains(&day))
}

/// What the staff carries on a Julian month and day, in a common year or,
/// with `leap`, a leap one.
///
/// A leap year's 29 February has no letter, as the Sunday letters are
/// counted; its February new moons from the 25th on fall a day later, as
/// table 14.4 has them.
///
/// # Errors
///
/// Returns [`CalendarError::MonthOutOfRange`] or
/// [`CalendarError::DayOutOfRange`] for a day the Julian year does not
/// have.
pub fn reading(month: u8, day: u8, leap: bool) -> CalendarResult<StaveDay> {
    let year = if leap { 4 } else { 1 };
    julian::to_fixed(year, month, day)?;
    let letter = if month == 2 && day == 29 {
        None
    } else {
        Some(common_letter(month, day))
    };
    let lunar_day = if leap && month == 2 && day >= 25 {
        day - 1
    } else {
        day
    };
    Ok(StaveDay {
        letter,
        golden_number: common_golden_number(month, lunar_day),
    })
}

/// What the staff carries on a fixed day, read on its Julian date.
///
/// # Errors
///
/// Returns what [`julian::from_fixed`] returns outside the Julian
/// implementation's range.
pub fn stave_day(rd: Rd) -> CalendarResult<StaveDay> {
    let (year, month, day) = julian::from_fixed(rd)?;
    reading(month, day, julian::is_leap_year(year))
}

/// Whether a fixed day is a new moon of the Julian computus: whether the
/// golden number on it is its own year's.
///
/// # Errors
///
/// Returns what [`julian::from_fixed`] returns outside the Julian
/// implementation's range.
pub fn is_new_moon(rd: Rd) -> CalendarResult<bool> {
    let (year, _, _) = julian::from_fixed(rd)?;
    Ok(stave_day(rd)?.golden_number == Some(super::golden_number(year)))
}

/// The paschal new moon of a Julian year: the first new moon of its golden
/// number on or after 8 March, whose fourteenth day is the first on or
/// after 21 March.
///
/// # Errors
///
/// Returns what [`julian::to_fixed`] returns outside the Julian
/// implementation's range, and [`CalendarError::DayOutOfRange`] if the
/// table had no such new moon, which it always has.
pub fn paschal_new_moon(year: i64) -> CalendarResult<Rd> {
    let golden = super::golden_number(year);
    let leap = julian::is_leap_year(year);
    let start = julian::to_fixed(year, 3, 8)?;
    for offset in 0..30 {
        let rd = Rd(start.0 + offset);
        let (_, month, day) = julian::from_fixed(rd)?;
        if reading(month, day, leap)?.golden_number == Some(golden) {
            return Ok(rd);
        }
    }
    Err(CalendarError::DayOutOfRange)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cycles::{golden_number, julian_dominical_letter, julian_epact};
    use hc_calendar::Weekday;

    /// Every day of a common year, in order, with its table column.
    fn common_days() -> impl Iterator<Item = (u8, u8)> {
        (1..=12u8).flat_map(|month| {
            let length = julian::days_in_month(1, month).unwrap_or(0);
            (1..=length).map(move |day| (month, day))
        })
    }

    /// The new moons of one golden number through the year, as days of a
    /// common year counted from 1 January = 0.
    fn new_moons_of(number: u8) -> Vec<i64> {
        let mut days = Vec::new();
        for (column, cell) in NEW_MOONS[usize::from(number) - 1].iter().enumerate() {
            for &day in *cell {
                days.push(i64::from(DAYS_BEFORE[column]) + i64::from(day) - 1);
            }
        }
        days
    }

    #[test]
    fn every_lunation_is_twenty_nine_or_thirty_days() {
        for number in 1..=19u8 {
            let moons = new_moons_of(number);
            for pair in moons.windows(2) {
                let length = pair[1] - pair[0];
                assert!(
                    length == 29 || length == 30,
                    "golden number {number}: a lunation of {length} days"
                );
            }
            // And into the next year, whose golden number is one on, 19
            // being followed by 1.
            let next = if number == 19 { 1 } else { number + 1 };
            let last = moons.last().copied().unwrap_or(0);
            let first = new_moons_of(next)[0] + 365;
            assert!(
                (29..=30).contains(&(first - last)),
                "{number} to {next}: {} days",
                first - last
            );
        }
    }

    #[test]
    fn the_table_has_235_new_moons_and_one_to_a_day() {
        let counts: Vec<usize> = (1..=19u8)
            .map(|number| new_moons_of(number).len())
            .collect();
        assert_eq!(counts.iter().sum::<usize>(), 235);
        assert_eq!(counts.iter().filter(|&&count| count == 13).count(), 7);
        assert!(counts.iter().all(|&count| count == 12 || count == 13));
        for (month, day) in common_days() {
            let column = usize::from(month) - 1;
            let carrying = (0..19)
                .filter(|&row| NEW_MOONS[row][column].contains(&day))
                .count();
            assert!(carrying <= 1, "{month}-{day} carries {carrying}");
        }
        let carried = common_days()
            .filter(|&(month, day)| common_golden_number(month, day).is_some())
            .count();
        assert_eq!(carried, 235);
    }

    /// Down a staff, each golden number is eight on from the last one cut:
    /// eleven days of lunar drift a year, eight years on, is two days less
    /// a lunation.
    #[test]
    fn a_day_carries_the_golden_number_eight_on_from_the_last() {
        let row: Vec<u8> = common_days()
            .filter_map(|(month, day)| common_golden_number(month, day))
            .collect();
        let breaks: Vec<(u8, u8)> = row
            .windows(2)
            .map(|pair| (pair[0], pair[1]))
            .filter(|&(a, b)| (a + 8 - 1) % 19 + 1 != b)
            .collect();
        assert!(breaks.is_empty(), "{breaks:?}");
    }

    /// The pre-1690 staffs begin with þ, 3, on 1 January (Swedish Wikipedia,
    /// "Runstav"); and January reads as the 1559 Book of Common Prayer's
    /// kalendar prints its golden numbers (justus.anglican.org's
    /// transcription, whose "10" on the 11th is a slip for 13).
    #[test]
    fn january_is_the_old_series() {
        let first = reading(1, 1, false).unwrap();
        assert_eq!(first.golden_number, Some(3));
        assert_eq!(first.golden_number_rune(), Some('ᚦ'));
        assert_eq!(first.letter_rune(), Some('ᚠ'));
        let january: Vec<(u8, u8)> = (1..=31)
            .filter_map(|day| common_golden_number(1, day).map(|number| (day, number)))
            .collect();
        assert_eq!(
            january,
            [
                (1, 3),
                (3, 11),
                (5, 19),
                (6, 8),
                (8, 16),
                (9, 5),
                (11, 13),
                (12, 2),
                (14, 10),
                (16, 18),
                (17, 7),
                (19, 15),
                (20, 4),
                (22, 12),
                (23, 1),
                (25, 9),
                (27, 17),
                (28, 6),
                (30, 14),
                (31, 3),
            ]
        );
    }

    /// The Julian epact is the age of the moon on 22 March, so it is the
    /// days from the March new moon of the table to 22 March, the new-moon
    /// day counting as the first.
    #[test]
    fn the_march_new_moon_gives_the_julian_epact() {
        for year in 1..=19 {
            let number = golden_number(year);
            let march = NEW_MOONS[usize::from(number) - 1][2][0];
            let age = (23 - i64::from(march)).rem_euclid(30);
            assert_eq!(i64::from(julian_epact(year)), age, "golden number {number}");
        }
    }

    #[test]
    fn the_leap_year_moves_three_february_new_moons_a_day_later() {
        for (day, common, leap) in [
            (24, None, None),
            (25, Some(17), None),
            (26, Some(6), Some(17)),
            (27, None, Some(6)),
            (28, Some(14), None),
        ] {
            assert_eq!(
                reading(2, day, false).unwrap().golden_number,
                common,
                "{day}"
            );
            assert_eq!(reading(2, day, true).unwrap().golden_number, leap, "{day}");
        }
        let leap_day = reading(2, 29, true).unwrap();
        assert_eq!(leap_day.golden_number, Some(14));
        assert_eq!(leap_day.letter, None);
        assert_eq!(leap_day.letter_rune(), None);
        assert_eq!(reading(2, 29, false), Err(CalendarError::DayOutOfRange));
        assert_eq!(reading(13, 1, false), Err(CalendarError::MonthOutOfRange));
        // 1 March is D, ᚬ, in both.
        assert_eq!(
            reading(3, 1, true).unwrap().letter,
            Some(DominicalLetter::D)
        );
        assert_eq!(reading(3, 1, false).unwrap().letter_rune(), Some('ᚬ'));
        assert_eq!(reading(3, 1, true), reading(3, 1, false));
        assert_eq!(reading(12, 31, true), reading(12, 31, false));
    }

    #[test]
    fn the_runes_are_the_ones_the_sources_tabulate() {
        assert_eq!(day_letter_rune(DominicalLetter::A), 'ᚠ');
        assert_eq!(day_letter_rune(DominicalLetter::G), 'ᚼ');
        // The first seven golden-number runes are the day-letter runes.
        assert_eq!(&GOLDEN_NUMBER_RUNES[..7], &DAY_LETTER_RUNES);
        // l before m, the medieval order.
        assert_eq!(golden_number_rune(14), Some('ᛚ'));
        assert_eq!(golden_number_rune(15), Some('ᛘ'));
        // Unicode's golden number runes, U+16EE–U+16F0.
        assert_eq!(golden_number_rune(17), Some('\u{16EE}'));
        assert_eq!(golden_number_rune(18), Some('\u{16EF}'));
        assert_eq!(golden_number_rune(19), Some('\u{16F0}'));
        assert_eq!(golden_number_rune(0), None);
        assert_eq!(golden_number_rune(20), None);
    }

    /// The *Explanatory Supplement*'s example 14.1: 1513, golden number 13,
    /// Sunday letter B; the paschal new moon 11 March, the full moon
    /// 24 March, a Thursday; Easter 27 March.
    #[test]
    fn easter_1513_read_off_the_stave() {
        assert_eq!(golden_number(1513), 13);
        assert_eq!(julian_dominical_letter(1513), (DominicalLetter::B, None));
        let new_moon = paschal_new_moon(1513).unwrap();
        assert_eq!(new_moon, julian::to_fixed(1513, 3, 11).unwrap());
        assert!(is_new_moon(new_moon).unwrap());
        let full_moon = Rd(new_moon.0 + 13);
        let reading = stave_day(full_moon).unwrap();
        assert_eq!(reading.letter, Some(DominicalLetter::F));
        assert_eq!(Weekday::from_rd(full_moon), Weekday::Thursday);
        let easter = (1..=7)
            .map(|offset| Rd(full_moon.0 + offset))
            .find(|&day| stave_day(day).unwrap().letter == Some(DominicalLetter::B))
            .unwrap();
        assert_eq!(julian::from_fixed(easter), Ok((1513, 3, 27)));
        assert_eq!(Weekday::from_rd(easter), Weekday::Sunday);
    }

    /// Cucina: the Vatican booklet's solar cycle starts at the leap year
    /// 1520, "since the Sunday letters G/A (runes ᚼ/ᚠ, i.e. h/f)
    /// correspond to this year".
    #[test]
    fn the_vatican_booklet_starts_its_solar_cycle_at_1520() {
        let (first, second) = julian_dominical_letter(1520);
        assert_eq!(
            (first, second),
            (DominicalLetter::A, Some(DominicalLetter::G))
        );
        assert_eq!(day_letter_rune(first), 'ᚠ');
        assert_eq!(second.map(day_letter_rune), Some('ᚼ'));
    }

    /// The staff's letters are the Sunday letters: in every Julian year the
    /// days carrying the year's letter are its Sundays.
    #[test]
    fn the_sundays_carry_the_years_letter() {
        for year in [1100, 1513, 1520, 1600, 1699, 1700, 1752] {
            let (first, second) = julian_dominical_letter(year);
            let start = julian::to_fixed(year, 1, 1).unwrap().0;
            let end = julian::to_fixed(year, 12, 31).unwrap().0;
            for rd in start..=end {
                let day = stave_day(Rd(rd)).unwrap();
                if day.letter.is_none() {
                    // 29 February, which has no letter.
                    continue;
                }
                let (_, month, _) = julian::from_fixed(Rd(rd)).unwrap();
                let letter = if month <= 2 {
                    first
                } else {
                    second.unwrap_or(first)
                };
                if Weekday::from_rd(Rd(rd)) == Weekday::Sunday {
                    assert_eq!(day.letter, Some(letter), "{year} rd {rd}");
                } else {
                    assert_ne!(day.letter, Some(letter), "{year} rd {rd}");
                }
            }
        }
    }
}
