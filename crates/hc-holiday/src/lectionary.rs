//! The lectionary cycles: which year of readings a day falls in, and which
//! numbered Proper a Sunday is.
//!
//! The Roman Lectionary for Mass of 1969 reads the Sundays in three years,
//! A, B and C, and the weekdays of Ordinary Time in two, I and II; the
//! Revised Common Lectionary of 1992 keeps the Roman Sunday cycle and
//! numbers the Sundays after Trinity Sunday as Propers, Proper 29 being
//! Christ the King. The system is written up in
//! `docs/systems/lectionary-cycles.md` in the repository, with 2025–26
//! worked by hand; this page summarises it and states the code's own
//! facts.
//!
//! Every cycle turns at the First Sunday of Advent, the Sunday between
//! 27 November and 3 December. A liturgical year is named here by the
//! civil year that holds its Easter, as the Liturgy Office of England and
//! Wales heads its table (`liturgyoffice-moveable`): the year that begins
//! on 30 November 2025 is 2026. Year A is the one whose Advent falls in a
//! civil year divisible by three (CCT, *Revised Common Lectionary:
//! Introduction*, §8, `cct-rcl`), so 2026 is Year A; the weekday cycle is
//! Year I when the liturgical year is odd (Wikipedia, "Lectionary", and
//! the Liturgy Office's table, 2020–2060), so 2026 is Year II.
//!
//! This carries the rules, not the readings. The texts are the
//! Consultation on Common Texts' and the Holy See's, under their own
//! copyright, and a caller who has them can look a Sunday up by the year
//! and Proper these functions give. The Roman Sundays of Ordinary Time and
//! the Propers of the Sundays after the Epiphany, which the RCL's calendar
//! prints as Propers 1 to 3 for churches that use them there, are not
//! numbered here.
//!
//! The cycles are arithmetic and answer for the liturgical years whose
//! Easter the Gregorian computus gives, 1583 to 4099. The Roman Lectionary dates from 1969 and
//! the RCL from Advent 1992 (§8), so a letter for an earlier year is the
//! rule applied backwards, not a lectionary anyone read from.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::computus::{self, COMPUTUS_LAST_YEAR, GREGORIAN_COMPUTUS_FIRST_YEAR};

/// The year of the three-year Sunday cycle, which the Roman Lectionary and
/// the Revised Common Lectionary share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SundayCycle {
    /// Year A, the year of Matthew.
    A,
    /// Year B, the year of Mark.
    B,
    /// Year C, the year of Luke.
    C,
}

impl SundayCycle {
    /// The letter, as the lectionaries print it.
    #[must_use]
    pub const fn letter(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
        }
    }
}

/// The year of the Roman Lectionary's two-year weekday cycle for Ordinary
/// Time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeekdayCycle {
    /// Year I, in odd-numbered liturgical years.
    I,
    /// Year II, in even-numbered liturgical years.
    II,
}

impl WeekdayCycle {
    /// The numeral, as the lectionary prints it.
    #[must_use]
    pub const fn numeral(self) -> &'static str {
        match self {
            Self::I => "I",
            Self::II => "II",
        }
    }
}

/// The First Sunday of Advent of a Gregorian year: the Sunday between
/// 27 November and 3 December (`cct-rcl`, the table of the Christian
/// year), which begins the liturgical year named for the year after.
///
/// Returns `None` outside 1582 to 4099: the Advents that begin the
/// liturgical years whose Easter the Gregorian computus gives.
#[must_use]
pub fn first_sunday_of_advent(year: i64) -> Option<Rd> {
    if !(GREGORIAN_COMPUTUS_FIRST_YEAR - 1..=COMPUTUS_LAST_YEAR).contains(&year) {
        return None;
    }
    let earliest = gregorian::to_fixed(year, 11, 27).ok()?;
    Some(Weekday::Sunday.on_or_after(earliest))
}

/// The liturgical year a day falls in, named by the civil year of its
/// Easter: from the First Sunday of Advent of the year before to the
/// Saturday before the next.
///
/// Returns `None` outside the liturgical years 1583 to 4099, whose Easter
/// the Gregorian computus gives.
#[must_use]
pub fn liturgical_year(day: Rd) -> Option<i64> {
    let civil = gregorian::year_from_fixed(day).ok()?;
    let year = if day >= first_sunday_of_advent(civil)? {
        civil + 1
    } else {
        civil
    };
    (GREGORIAN_COMPUTUS_FIRST_YEAR..=COMPUTUS_LAST_YEAR)
        .contains(&year)
        .then_some(year)
}

/// The year of the Sunday cycle a day falls in.
///
/// ```
/// use hc_holiday::lectionary::{SundayCycle, sunday_cycle};
/// use hc_calendars_solar::gregorian;
///
/// // Advent 2025 began Year A: 2025 is divisible by three.
/// let advent = gregorian::to_fixed(2025, 11, 30)?;
/// assert_eq!(sunday_cycle(advent), Some(SundayCycle::A));
/// let eve = gregorian::to_fixed(2025, 11, 29)?;
/// assert_eq!(sunday_cycle(eve), Some(SundayCycle::C));
/// # Ok::<(), hc_calendar::CalendarError>(())
/// ```
#[must_use]
pub fn sunday_cycle(day: Rd) -> Option<SundayCycle> {
    // The Advent year is the liturgical year less one; Year A begins in an
    // Advent year divisible by three.
    Some(match (liturgical_year(day)? - 1).rem_euclid(3) {
        0 => SundayCycle::A,
        1 => SundayCycle::B,
        _ => SundayCycle::C,
    })
}

/// The year of the Roman weekday cycle a day falls in: Year I in an
/// odd-numbered liturgical year, Year II in an even one.
#[must_use]
pub fn roman_weekday_cycle(day: Rd) -> Option<WeekdayCycle> {
    Some(if liturgical_year(day)?.rem_euclid(2) == 1 {
        WeekdayCycle::I
    } else {
        WeekdayCycle::II
    })
}

/// The last Proper of the Revised Common Lectionary, Christ the King.
pub const LAST_PROPER: u8 = 29;

/// Days from Easter to Trinity Sunday, the first Sunday after Pentecost.
const TRINITY_SUNDAY: i64 = computus::offsets::TRINITY_SUNDAY as i64;

/// The Revised Common Lectionary's numbered Proper of a Sunday after
/// Trinity Sunday, from 3 to 29.
///
/// Proper 29 is "Sunday between November 20 and November 26", and
/// Propers 3 to 28 the "Second through Twenty-Sixth Sunday after
/// Pentecost" before it (`cct-rcl`, the table of the Christian year), so a
/// Sunday's Proper is 29 less the weeks it falls before Proper 29. Which
/// Proper the first Sunday after Trinity is depends on Easter: "When
/// Easter is as early as March 22, the numbered Proper for the Sunday
/// following Trinity Sunday is Proper 3". The Ordinary Time number in the
/// RCL's brackets, which the Roman Lectionary uses, is the Proper plus
/// five.
///
/// Returns `None` for a day that is not a Sunday, for a Sunday outside
/// the season after Trinity Sunday, and outside the Gregorian computus's
/// years.
#[must_use]
pub fn rcl_proper(day: Rd) -> Option<u8> {
    if Weekday::from_rd(day) != Weekday::Sunday {
        return None;
    }
    let year = gregorian::year_from_fixed(day).ok()?;
    let trinity = computus::gregorian_easter(year)?.0 + TRINITY_SUNDAY;
    let last = Weekday::Sunday.on_or_after(gregorian::to_fixed(year, 11, 20).ok()?);
    if day.0 <= trinity || day > last {
        return None;
    }
    let weeks_before = u8::try_from((last.0 - day.0) / 7).ok()?;
    LAST_PROPER.checked_sub(weeks_before)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    #[test]
    fn the_rcl_table_of_advents_and_years_1992_to_2021() {
        // CCT, Revised Common Lectionary: Introduction, the table after §8:
        // the First Sunday of Advent of each year and the year it begins.
        let table: [(i64, u8, u8, SundayCycle); 30] = [
            (1992, 11, 29, SundayCycle::A),
            (1993, 11, 28, SundayCycle::B),
            (1994, 11, 27, SundayCycle::C),
            (1995, 12, 3, SundayCycle::A),
            (1996, 12, 1, SundayCycle::B),
            (1997, 11, 30, SundayCycle::C),
            (1998, 11, 29, SundayCycle::A),
            (1999, 11, 28, SundayCycle::B),
            (2000, 12, 3, SundayCycle::C),
            (2001, 12, 2, SundayCycle::A),
            (2002, 12, 1, SundayCycle::B),
            (2003, 11, 30, SundayCycle::C),
            (2004, 11, 28, SundayCycle::A),
            (2005, 11, 27, SundayCycle::B),
            (2006, 12, 3, SundayCycle::C),
            (2007, 12, 2, SundayCycle::A),
            (2008, 11, 30, SundayCycle::B),
            (2009, 11, 29, SundayCycle::C),
            (2010, 11, 28, SundayCycle::A),
            (2011, 11, 27, SundayCycle::B),
            (2012, 12, 2, SundayCycle::C),
            (2013, 12, 1, SundayCycle::A),
            (2014, 11, 30, SundayCycle::B),
            (2015, 11, 29, SundayCycle::C),
            (2016, 11, 27, SundayCycle::A),
            (2017, 12, 3, SundayCycle::B),
            (2018, 12, 2, SundayCycle::C),
            (2019, 12, 1, SundayCycle::A),
            (2020, 11, 29, SundayCycle::B),
            (2021, 11, 28, SundayCycle::C),
        ];
        for (year, month, day, cycle) in table {
            let advent = greg(year, month, day);
            assert_eq!(first_sunday_of_advent(year), Some(advent), "{year}");
            assert_eq!(sunday_cycle(advent), Some(cycle), "Advent {year}");
            // The day before is still the previous year of the cycle.
            let previous = match cycle {
                SundayCycle::A => SundayCycle::C,
                SundayCycle::B => SundayCycle::A,
                SundayCycle::C => SundayCycle::B,
            };
            assert_eq!(sunday_cycle(Rd(advent.0 - 1)), Some(previous), "{year}");
        }
    }

    #[test]
    fn the_liturgy_office_cycles_of_2020_to_2030() {
        // The Liturgy Office of England and Wales, Table of Moveable Feasts
        // 2020–2060: the Sunday and weekday cycles of each liturgical year,
        // headed by the civil year of its Easter; checked here at Easter.
        let table: [(i64, SundayCycle, WeekdayCycle); 11] = [
            (2020, SundayCycle::A, WeekdayCycle::II),
            (2021, SundayCycle::B, WeekdayCycle::I),
            (2022, SundayCycle::C, WeekdayCycle::II),
            (2023, SundayCycle::A, WeekdayCycle::I),
            (2024, SundayCycle::B, WeekdayCycle::II),
            (2025, SundayCycle::C, WeekdayCycle::I),
            (2026, SundayCycle::A, WeekdayCycle::II),
            (2027, SundayCycle::B, WeekdayCycle::I),
            (2028, SundayCycle::C, WeekdayCycle::II),
            (2029, SundayCycle::A, WeekdayCycle::I),
            (2030, SundayCycle::B, WeekdayCycle::II),
        ];
        for (year, sunday, weekday) in table {
            let easter = computus::gregorian_easter(year).expect("in range");
            assert_eq!(liturgical_year(easter), Some(year));
            assert_eq!(sunday_cycle(easter), Some(sunday), "{year}");
            assert_eq!(roman_weekday_cycle(easter), Some(weekday), "{year}");
        }
        // The table's Advent Sunday of 2025, 30 November, opens 2026.
        assert_eq!(first_sunday_of_advent(2025), Some(greg(2025, 11, 30)));
        assert_eq!(liturgical_year(greg(2025, 11, 30)), Some(2026));
        assert_eq!(
            roman_weekday_cycle(greg(2025, 11, 29)),
            Some(WeekdayCycle::I)
        );
        assert_eq!(
            roman_weekday_cycle(greg(2025, 11, 30)),
            Some(WeekdayCycle::II)
        );
    }

    #[test]
    fn proper_29_is_the_sunday_between_20_and_26_november() {
        for year in 1990..=2040 {
            let last = Weekday::Sunday.on_or_after(greg(year, 11, 20));
            assert_eq!(rcl_proper(last), Some(29), "{year}");
            assert_eq!(rcl_proper(Rd(last.0 + 7)), None, "{year}: Advent");
            assert_eq!(rcl_proper(Rd(last.0 - 7)), Some(28), "{year}");
        }
    }

    #[test]
    fn an_easter_on_22_march_makes_the_sunday_after_trinity_proper_3() {
        // 1818 had the earliest Easter, 22 March (Meeus); Trinity Sunday was
        // 17 May and the Sunday after it 24 May.
        assert_eq!(computus::gregorian_easter(1818), Some(greg(1818, 3, 22)));
        assert_eq!(rcl_proper(greg(1818, 5, 17)), None);
        assert_eq!(rcl_proper(greg(1818, 5, 24)), Some(3));
        // In no year is a Proper lower than 3.
        for year in 1583..=2500 {
            let trinity = computus::gregorian_easter(year).expect("in range").0 + TRINITY_SUNDAY;
            let first = rcl_proper(Rd(trinity + 7)).expect("a Proper");
            assert!((3..=8).contains(&first), "{year}: Proper {first}");
        }
    }

    #[test]
    fn the_propers_match_the_roman_sundays_in_ordinary_time_windows() {
        // The Liturgy Office's Dates for Sundays: the 8th Sunday in
        // Ordinary Time falls 22–28 May, the 9th 29 May–4 June, …, the
        // 33rd 13–19 November. The RCL's bracket number is the Proper plus
        // five, so a Sunday in the window of Ordinary n is Proper n − 5
        // whenever it follows Trinity Sunday.
        for year in 2000..=2060 {
            let trinity = computus::gregorian_easter(year).expect("in range").0 + TRINITY_SUNDAY;
            for ordinary in 8u8..=34 {
                let weeks_back = i64::from(34 - ordinary) * 7;
                let from = greg(year, 11, 20).0 - weeks_back;
                let sunday = Weekday::Sunday.on_or_after(Rd(from));
                if sunday.0 <= trinity {
                    continue;
                }
                assert_eq!(
                    rcl_proper(sunday),
                    Some(ordinary - 5),
                    "{year}, Ordinary {ordinary}"
                );
            }
        }
    }

    #[test]
    fn a_weekday_has_no_proper_and_the_cycles_refuse_outside_the_computus() {
        assert_eq!(rcl_proper(greg(2026, 7, 1)), None);
        assert_eq!(first_sunday_of_advent(1581), None);
        assert_eq!(sunday_cycle(greg(1582, 11, 27)), None);
        assert_eq!(sunday_cycle(greg(1583, 6, 1)), Some(SundayCycle::B));
        assert_eq!(sunday_cycle(greg(4099, 6, 1)), Some(SundayCycle::A));
        assert_eq!(sunday_cycle(greg(4099, 12, 25)), None);
    }

    #[test]
    fn the_worked_example_of_2025_to_2026() {
        // docs/systems/lectionary-cycles.md: Advent 2025 opens 2026, Year A
        // and Year II; the Sunday after Trinity is Proper 5, Christ the King
        // Proper 29.
        let advent = first_sunday_of_advent(2025).expect("in range");
        assert_eq!(advent, greg(2025, 11, 30));
        assert_eq!(liturgical_year(advent), Some(2026));
        assert_eq!(sunday_cycle(greg(2026, 6, 7)), Some(SundayCycle::A));
        assert_eq!(
            roman_weekday_cycle(greg(2026, 6, 8)),
            Some(WeekdayCycle::II)
        );
        assert_eq!(rcl_proper(greg(2026, 5, 31)), None, "Trinity Sunday");
        assert_eq!(rcl_proper(greg(2026, 6, 7)), Some(5));
        assert_eq!(rcl_proper(greg(2026, 11, 22)), Some(29));
        assert_eq!(first_sunday_of_advent(2026), Some(greg(2026, 11, 29)));
        assert_eq!(SundayCycle::A.letter(), 'A');
        assert_eq!(WeekdayCycle::II.numeral(), "II");
    }
}
