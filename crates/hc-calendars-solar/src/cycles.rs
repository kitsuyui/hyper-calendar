//! The classical year cycles of the computus.
//!
//! Before Easter was a formula it was a set of numbers you looked up, and
//! those numbers are stated on the page of every almanac, missal and
//! chronicle that gives a date at all: the golden number, the dominical
//! letter, the epact, the solar cycle, the indiction, and the position in
//! Scaliger's Julian Period. A reader working with a medieval source has
//! them and wants the year; a reader working with a year wants them.
//!
//! Easter itself is computed in `hc-holiday`; this module states the
//! numbers it is computed from.
//!
//! # What each one is
//!
//! * **Golden number** — position in the 19-year Metonic cycle, so the
//!   phase of the ecclesiastical moon repeats with it.
//! * **Dominical letter** — which of A–G, counting 1 January as A, falls on
//!   the year's Sundays. A leap year has two, because 29 February shifts
//!   the rest of the year back a letter.
//! * **Epact** — the age of the ecclesiastical moon on a fixed day, which
//!   with the golden number fixes the paschal full moon: 22 March in the
//!   Julian (Alexandrian) computus, 1 January in the Gregorian.
//! * **Solar cycle** — position in the 28 years after which Julian weekdays
//!   repeat.
//! * **Indiction** — position in the 15-year Roman tax cycle, which
//!   Byzantine and papal documents date by.
//! * **Julian Period** — Scaliger's 7980 years, the lowest common multiple
//!   of 19, 28 and 15, so that a year is fixed by the three cycles
//!   together.
//!
//! **Sources:** Bede, *De temporum ratione* (725); Clavius, *Explicatio
//! Romani Calendarii* (1603) for the Gregorian epact; Scaliger, *De
//! emendatione temporum* (1583) for the Period.
//!
//! [`runic`] reads the golden number and the Sunday letter the way the
//! Swedish runestaff carves them, against every day of the Julian year.

pub mod runic;

use hc_calendar::{Rd, Weekday};

use crate::{gregorian, julian};

/// The length of the Metonic cycle in years.
pub const METONIC_YEARS: i64 = 19;

/// The length of the solar cycle in years.
pub const SOLAR_CYCLE_YEARS: i64 = 28;

/// The length of the indiction cycle in years.
pub const INDICTION_YEARS: i64 = 15;

/// The length of the Julian Period, 19 × 28 × 15.
pub const JULIAN_PERIOD_YEARS: i64 = METONIC_YEARS * SOLAR_CYCLE_YEARS * INDICTION_YEARS;

/// The Julian year in which all three cycles were last simultaneously 1,
/// and from which Scaliger numbered his Period: 4713 BC, that is −4712.
pub const JULIAN_PERIOD_EPOCH_YEAR: i64 = -4712;

/// The golden number of `year`: its position in the 19-year Metonic cycle.
///
/// 1 through 19. Named for the practice of writing it in gold in the
/// calendars of the late Middle Ages.
#[must_use]
pub const fn golden_number(year: i64) -> u8 {
    (year.rem_euclid(METONIC_YEARS) + 1) as u8
}

/// The indiction of `year`: its position in the 15-year Roman tax cycle.
///
/// 1 through 15, reckoned from the AD year, so AD 313 is indiction 1.
///
/// This is the same cycle as [`crate::byzantine::indiction`], which counts
/// it from the Byzantine year of the world instead. A test asserts the two
/// agree where they describe the same year; they are kept separate because
/// a caller holding one kind of year should not have to convert to get an
/// answer about it.
#[must_use]
pub const fn indiction(year: i64) -> u8 {
    ((year + 2).rem_euclid(INDICTION_YEARS) + 1) as u8
}

/// The solar cycle of `year`: its position in the 28 years after which the
/// Julian calendar's weekdays repeat.
///
/// 1 through 28. The Gregorian calendar breaks the repetition at three
/// century years in four, so this is a Julian quantity and the cycle it
/// names is only a cycle there.
#[must_use]
pub const fn solar_cycle(year: i64) -> u8 {
    ((year + 8).rem_euclid(SOLAR_CYCLE_YEARS) + 1) as u8
}

/// The year of the Julian Period corresponding to `year` AD.
///
/// Scaliger set the Period so that its year 1 is the last year in which the
/// golden number, the solar cycle and the indiction were all 1 — 4713 BC —
/// and so that every year of the 7980 has a distinct triple. Herschel later
/// turned the same idea into the Julian Day count.
#[must_use]
pub const fn julian_period_year(year: i64) -> i64 {
    year - JULIAN_PERIOD_EPOCH_YEAR + 1
}

/// A dominical letter, A through G.
///
/// The letter of the Sundays of a year, where 1 January is A and 7 January
/// is G.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DominicalLetter {
    /// Sundays fall on 1 January and every seventh day after.
    A = 1,
    /// Sundays fall on 2 January and every seventh day after.
    B = 2,
    /// Sundays fall on 3 January and every seventh day after.
    C = 3,
    /// Sundays fall on 4 January and every seventh day after.
    D = 4,
    /// Sundays fall on 5 January and every seventh day after.
    E = 5,
    /// Sundays fall on 6 January and every seventh day after.
    F = 6,
    /// Sundays fall on 7 January and every seventh day after.
    G = 7,
}

impl DominicalLetter {
    /// All seven, in order.
    pub const ALL: [Self; 7] = [
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
    ];

    /// The letter for a day of January in `1..=7`, or `None` otherwise.
    #[must_use]
    pub const fn from_january_day(day: u8) -> Option<Self> {
        match day {
            1 => Some(Self::A),
            2 => Some(Self::B),
            3 => Some(Self::C),
            4 => Some(Self::D),
            5 => Some(Self::E),
            6 => Some(Self::F),
            7 => Some(Self::G),
            _ => None,
        }
    }

    /// The letter as its character.
    #[must_use]
    pub const fn as_char(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::F => 'F',
            Self::G => 'G',
        }
    }

    /// The letter one step earlier in the alphabet, wrapping G to F and A
    /// to G.
    ///
    /// This is what a leap day does to the rest of the year.
    #[must_use]
    pub const fn previous(self) -> Self {
        match self {
            Self::A => Self::G,
            Self::B => Self::A,
            Self::C => Self::B,
            Self::D => Self::C,
            Self::E => Self::D,
            Self::F => Self::E,
            Self::G => Self::F,
        }
    }
}

/// The dominical letter or letters of a year, derived from its first day.
///
/// Computed rather than tabulated: the letter is defined as the one on the
/// year's Sundays, so finding the first Sunday on or after 1 January and
/// reading off its day of the month *is* the definition. That works for
/// either calendar without a second formula.
const fn letters_from_new_year(
    new_year: Rd,
    leap: bool,
) -> (DominicalLetter, Option<DominicalLetter>) {
    // The first Sunday on or after 1 January; its day of the month is the
    // letter, 1 being A.
    let first_sunday = Weekday::Sunday.on_or_after(new_year);
    let offset = first_sunday.0 - new_year.0;
    let letter = match DominicalLetter::from_january_day((offset + 1) as u8) {
        Some(letter) => letter,
        // Unreachable: offset is 0..=6, so the day is 1..=7.
        None => DominicalLetter::A,
    };
    if leap {
        (letter, Some(letter.previous()))
    } else {
        (letter, None)
    }
}

/// The dominical letter of a Gregorian year.
///
/// In a leap year the second letter is returned too, and governs from
/// 1 March; the first governs January and February.
#[must_use]
pub fn gregorian_dominical_letter(year: i64) -> (DominicalLetter, Option<DominicalLetter>) {
    let Ok(new_year) = gregorian::new_year(year) else {
        // Outside the implementation's year bounds; A is as good an answer
        // as any and the caller has already left the supported range.
        return (DominicalLetter::A, None);
    };
    letters_from_new_year(new_year, gregorian::is_leap_year(year))
}

/// The dominical letter of a Julian year.
///
/// In a leap year the second letter is returned too, and governs from
/// 1 March.
#[must_use]
pub fn julian_dominical_letter(year: i64) -> (DominicalLetter, Option<DominicalLetter>) {
    let Ok(new_year) = julian::to_fixed(year, 1, 1) else {
        // Outside the Julian implementation's range; fall back to the
        // Gregorian reckoning of the same year number, which is the best
        // this can say.
        return gregorian_dominical_letter(year);
    };
    letters_from_new_year(new_year, julian::is_leap_year(year))
}

/// The Julian (Alexandrian) epact: the age of the ecclesiastical moon on
/// 22 March, under the unreformed computus — 0 in a year whose March new
/// moon falls on the 23rd, golden number 1.
///
/// 0 through 29, and a simple function of the golden number — eleven days
/// of lunar drift per solar year, taken modulo the 30-day ecclesiastical
/// lunation. This is Bede's epact in *De temporum ratione* (not read here).
/// The day it is reckoned on is checked against the *Explanatory
/// Supplement to the Astronomical Ephemeris* (HMSO, 1961), table 14.4, the
/// Julian ecclesiastical new moons by golden number: for every golden
/// number, the days from the table's March new moon to 22 March, the new
/// moon counting as the first, are this epact
/// (`runic::tests::the_march_new_moon_gives_the_julian_epact`). It is not
/// the age on 1 January, which is what [`gregorian_epact`] states for the
/// reformed computus.
#[must_use]
pub const fn julian_epact(year: i64) -> u8 {
    let golden = golden_number(year) as i64 - 1;
    (11 * golden).rem_euclid(30) as u8
}

/// The Gregorian epact: the age of the ecclesiastical moon on 1 January
/// under the reformed computus.
///
/// 0 through 29. The reform corrected the Julian epact twice over: the
/// *solar equation* decrements it wherever the Gregorian rule drops a leap
/// day, and the *lunar equation* increments it about eight times in 2500
/// years, because the Metonic cycle itself runs long.
///
/// The century number is ⌊y/100⌋ **+ 1**, which is the one place this is
/// easy to get wrong, and which the cross-check below guards.
///
/// The correctness of all of it is not asserted but *derived*: `hc-holiday`
/// rebuilds Easter out of this epact and compares it against Butcher's
/// arrangement of the computus, which shares no line of code with it, for
/// every year from 1583 to 4099.
#[must_use]
pub const fn gregorian_epact(year: i64) -> u8 {
    let century = year.div_euclid(100) + 1;
    let golden = golden_number(year) as i64;
    let solar_equation = (3 * century).div_euclid(4);
    let lunar_equation = (8 * century + 5).div_euclid(25);
    (11 * golden - solar_equation + lunar_equation + 27).rem_euclid(30) as u8
}

/// The March date of the paschal full moon implied by the Gregorian epact.
///
/// "March 32" and beyond mean April; the caller adds Easter's own rule,
/// which is the Sunday after. Returned as a day of March so the classical
/// statement — *luna XIV* falls on such a day of March — survives.
///
/// Clavius's two exceptions are applied here rather than to the epact
/// itself, because they correct the moon and not the moon's age.
///
/// Both stop two epacts sharing a paschal moon. An epact of 24 takes the
/// day before the arithmetic gives, landing on 18 April; so does an epact
/// of 25 when the golden number is above 11, landing on 17 April. Without
/// them a full lunation would be skipped once a cycle.
#[must_use]
pub const fn gregorian_paschal_moon_march_day(year: i64) -> i64 {
    let epact = gregorian_epact(year) as i64;
    let day = 44 - epact;
    let day = if day < 21 { day + 30 } else { day };
    if epact == 24 || (epact == 25 && golden_number(year) > 11) {
        day - 1
    } else {
        day
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_golden_number_runs_one_to_nineteen_and_repeats() {
        // 2024's golden number is 11, as every almanac for that year says.
        assert_eq!(golden_number(2024), 11);
        assert_eq!(golden_number(2024 + 19), 11);
        for year in 1..=19 {
            assert_eq!(golden_number(year), (year % 19 + 1) as u8);
        }
        for year in -3_000..3_000 {
            let number = golden_number(year);
            assert!((1..=19).contains(&number), "{year} gave {number}");
        }
    }

    #[test]
    fn the_indiction_starts_its_cycle_in_three_hundred_and_thirteen() {
        assert_eq!(indiction(313), 1);
        assert_eq!(indiction(312), 15);
        assert_eq!(indiction(2024), 2);
        assert_eq!(indiction(313 + 15), 1);
        for year in -3_000..3_000 {
            assert!((1..=15).contains(&indiction(year)));
        }
    }

    /// The indiction is the same cycle the Byzantine calendar carries, and
    /// this asserts it rather than assuming it.
    ///
    /// The offset is 5508 and not 5509, because [`indiction`] reckons from
    /// 1 January and the Byzantine year of the world turns on 1 September.
    /// In January the AM year already running is the AD year plus 5508; it
    /// becomes plus 5509 only in the September after.
    #[test]
    fn it_agrees_with_the_byzantine_indiction() {
        for year in 500..2_500 {
            assert_eq!(
                indiction(year),
                crate::byzantine::indiction(year + 5508),
                "AD {year}"
            );
        }
    }

    /// The defining property, asserted against the calendar rather than
    /// against the formula: 28 Julian years later, every weekday is back.
    #[test]
    fn the_solar_cycle_is_the_period_of_the_julian_weekday() {
        for year in 500..2_000 {
            let start = julian::to_fixed(year, 1, 1).expect("in range");
            let later = julian::to_fixed(year + 28, 1, 1).expect("in range");
            assert_eq!(
                Weekday::from_rd(start),
                Weekday::from_rd(later),
                "Julian {year} and {} should share a weekday",
                year + 28
            );
            assert_eq!(solar_cycle(year + 28), solar_cycle(year));
            assert!((1..=28).contains(&solar_cycle(year)));
        }
        // And it is *not* a cycle in the Gregorian calendar, which is why
        // the doc says so.
        let gregorian_1900 = gregorian::new_year(1900).expect("in range");
        let gregorian_1928 = gregorian::new_year(1928).expect("in range");
        assert_ne!(
            Weekday::from_rd(gregorian_1900),
            Weekday::from_rd(gregorian_1928)
        );
    }

    #[test]
    fn the_julian_period_year_is_the_ad_year_plus_four_thousand_seven_hundred_and_thirteen() {
        assert_eq!(julian_period_year(1), 4714);
        assert_eq!(julian_period_year(2024), 6737);
        assert_eq!(julian_period_year(JULIAN_PERIOD_EPOCH_YEAR), 1);
        assert_eq!(JULIAN_PERIOD_YEARS, 7_980);
    }

    /// Scaliger's reason for the Period: inside it, the three cycles
    /// together name a year uniquely.
    #[test]
    fn the_three_cycles_identify_a_year_within_the_period() {
        let first = 1_i64;
        let triple = |year: i64| (golden_number(year), solar_cycle(year), indiction(year));
        // The triple repeats after exactly 7980 years and not before.
        assert_eq!(triple(first), triple(first + JULIAN_PERIOD_YEARS));
        for offset in 1..2_000 {
            assert_ne!(
                triple(first),
                triple(first + offset),
                "the triple repeated after {offset} years"
            );
        }
    }

    /// The letter is defined by where the Sundays fall, so this checks the
    /// definition and not the formula.
    #[test]
    fn the_dominical_letter_marks_the_years_sundays() {
        for year in 1_500..2_600 {
            let (first, second) = gregorian_dominical_letter(year);
            let january_sunday = gregorian::to_fixed(year, 1, first as u8).expect("in range");
            assert_eq!(
                Weekday::from_rd(january_sunday),
                Weekday::Sunday,
                "{year} letter {}",
                first.as_char()
            );
            match second {
                None => assert!(!gregorian::is_leap_year(year), "{year}"),
                Some(second) => {
                    assert!(gregorian::is_leap_year(year), "{year}");
                    assert_eq!(second, first.previous());
                    // The second letter governs from 1 March. The letters
                    // run A–G through the year skipping the leap day, and
                    // 1 March is always D — 59 lettered days after 1
                    // January in both a leap and a common year. So the
                    // Sunday marked by `second` sits (second − D) mod 7
                    // days after 1 March.
                    let march_first = gregorian::to_fixed(year, 3, 1).expect("in range");
                    let offset = (second as i64 - DominicalLetter::D as i64).rem_euclid(7);
                    assert_eq!(
                        Weekday::from_rd(Rd(march_first.0 + offset)),
                        Weekday::Sunday,
                        "{year} second letter {}",
                        second.as_char()
                    );
                }
            }
        }
    }

    /// The classical mnemonic, which is also a check on the whole scheme:
    /// the first of each month carries the letters A D D G B E G C F A D F,
    /// in both a common year and a leap year after February.
    #[test]
    fn the_first_of_each_month_carries_the_classical_letter() {
        use DominicalLetter::{A, B, C, D, E, F, G};
        let expected = [A, D, D, G, B, E, G, C, F, A, D, F];
        for year in [1999, 2000, 2023, 2024] {
            let (first, second) = gregorian_dominical_letter(year);
            for (index, month_letter) in expected.into_iter().enumerate() {
                let month = (index + 1) as u8;
                // Which letter marks Sundays in this month.
                let marker = if month <= 2 {
                    first
                } else {
                    second.unwrap_or(first)
                };
                let offset = (marker as i64 - month_letter as i64).rem_euclid(7);
                let day = gregorian::to_fixed(year, month, 1).expect("in range");
                assert_eq!(
                    Weekday::from_rd(Rd(day.0 + offset)),
                    Weekday::Sunday,
                    "{year}-{month:02}"
                );
            }
        }
    }

    #[test]
    fn the_julian_dominical_letter_marks_julian_sundays() {
        for year in 800..1_600 {
            let (first, _) = julian_dominical_letter(year);
            let sunday = julian::to_fixed(year, 1, first as u8).expect("in range");
            assert_eq!(Weekday::from_rd(sunday), Weekday::Sunday, "Julian {year}");
        }
    }

    #[test]
    fn the_julian_epact_is_eleven_days_of_drift_per_year() {
        assert_eq!(
            julian_epact(1),
            11 * (golden_number(1) as u32 - 1) as u8 % 30
        );
        for year in 1..2_000 {
            let epact = julian_epact(year);
            assert!(epact < 30, "{year} gave {epact}");
            // Consecutive years differ by eleven, modulo thirty, except
            // where the Metonic cycle restarts.
            if golden_number(year + 1) != 1 {
                assert_eq!(
                    julian_epact(year + 1),
                    (epact + 11) % 30,
                    "{year} to {}",
                    year + 1
                );
            }
        }
    }

    #[test]
    fn the_gregorian_epact_stays_in_range_and_avoids_claviuss_exceptions() {
        for year in 1_583..4_100 {
            let epact = gregorian_epact(year);
            assert!(epact < 30, "{year} gave {epact}");
        }
        // The paschal moon never falls before 21 March, which is the
        // constraint the whole reform is built around.
        for year in 1_583..4_100 {
            let day = gregorian_paschal_moon_march_day(year);
            assert!((21..=49).contains(&day), "{year} gave March {day}");
        }
    }
}
