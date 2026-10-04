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
//! A year here is the liturgical year, which turns at the First Sunday of
//! Advent and takes the number of the civil year that holds its Easter
//! (`liturgyoffice-moveable`): the year that begins on 30 November 2025 is
//! 2026. Year A is the one whose Advent falls in a
//! civil year divisible by three (CCT, *Revised Common Lectionary:
//! Introduction*, §8, `cct-rcl`), so 2026 is Year A; the weekday cycle is
//! Year I when the liturgical year is odd (Wikipedia, "Lectionary", and
//! the Liturgy Office's table, 2020–2060), so 2026 is Year II.
//!
//! This carries the rules, not the readings. The texts are the
//! Consultation on Common Texts' and the Holy See's, under their own
//! copyright, and a caller who has them can look a Sunday up by the year
//! and Proper these functions give. The Roman Sundays in Ordinary Time are
//! numbered by [`sunday_in_ordinary_time`], and the weeks by
//! [`week_of_ordinary_time`] and, for a calendar that keeps the Epiphany on
//! a Sunday, [`week_of_ordinary_time_epiphany_on_sunday`], as the Liturgy
//! Office of England and Wales tabulates them. The Propers of the Sundays
//! after the Epiphany, which the RCL's calendar prints as Propers 1 to 3
//! for churches that use them there, are not numbered here.
//!
//! The cycles are arithmetic, and answer from the day the reform they
//! belong to went into effect to the last liturgical year whose Easter the
//! Gregorian computus gives, 4099: the Roman cycles from
//! [`ROMAN_REFORM_IN_EFFECT`], 1 January 1970 (`mysterii-paschalis`), the
//! RCL's Propers from Advent 1992 (`cct-rcl`, §8), and `None` before.
//! [`liturgical_year`] and [`first_sunday_of_advent`] name and date a year
//! and are not the reform's.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::computus::{self, COMPUTUS_LAST_YEAR, GREGORIAN_COMPUTUS_FIRST_YEAR};

/// The day the Roman calendar of 1969 and its general norms went into
/// effect, 1 January 1970 (Paul VI, *Mysterii Paschalis*, 14 February
/// 1969): the first day of Ordinary Time and of the Roman cycles.
pub const ROMAN_REFORM_IN_EFFECT: Rd = Rd(719_163);

/// The year whose First Sunday of Advent began the Revised Common
/// Lectionary's use, 1992 (`cct-rcl`, §8).
pub const RCL_FIRST_ADVENT_YEAR: i64 = 1992;

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
    if day < ROMAN_REFORM_IN_EFFECT {
        return None;
    }
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
    if day < ROMAN_REFORM_IN_EFFECT {
        return None;
    }
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
/// Sunday's Proper is 29 less the weeks it falls before Proper 29. That
/// parenthesis pairs 26 Propers with 25 Sundays: with Easter on 22 March
/// Propers 3 to 28 are the Second to the Twenty-Seventh Sunday, and a later
/// Easter drops the lowest Propers. Which
/// Proper the first Sunday after Trinity is depends on Easter: "When
/// Easter is as early as March 22, the numbered Proper for the Sunday
/// following Trinity Sunday is Proper 3". The Ordinary Time number in the
/// RCL's brackets, which the Roman Lectionary uses, is the Proper plus
/// five.
///
/// Returns `None` for a day that is not a Sunday, for a Sunday outside
/// the season after Trinity Sunday, before Advent 1992 and outside the
/// Gregorian computus's years.
#[must_use]
pub fn rcl_proper(day: Rd) -> Option<u8> {
    if Weekday::from_rd(day) != Weekday::Sunday
        || day < first_sunday_of_advent(RCL_FIRST_ADVENT_YEAR)?
    {
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

/// The number of the last Sunday in Ordinary Time, Christ the King, on the
/// Sunday between 20 and 26 November (`liturgyoffice-sundays`).
pub const LAST_SUNDAY_IN_ORDINARY_TIME: u8 = 34;

/// Days from Easter to Ash Wednesday, the day after Ordinary Time's first
/// part ends.
const ASH_WEDNESDAY: i64 = computus::offsets::ASH_WEDNESDAY as i64;

/// Days from Easter to Pentecost, the day before Ordinary Time resumes.
const PENTECOST: i64 = computus::offsets::PENTECOST as i64;

/// The Sundays of a year that bound Ordinary Time: the Second Sunday, the
/// Sunday between 14 and 20 January, and the Thirty-fourth, between 20 and
/// 26 November, with Ash Wednesday and Pentecost between them.
struct OrdinaryTime {
    second_sunday: Rd,
    ash_wednesday: Rd,
    pentecost: Rd,
    last_sunday: Rd,
}

impl OrdinaryTime {
    fn of(year: i64) -> Option<Self> {
        let easter = computus::gregorian_easter(year)?;
        Some(Self {
            second_sunday: Weekday::Sunday.on_or_after(gregorian::to_fixed(year, 1, 14).ok()?),
            ash_wednesday: Rd(easter.0 + ASH_WEDNESDAY),
            pentecost: Rd(easter.0 + PENTECOST),
            last_sunday: Weekday::Sunday.on_or_after(gregorian::to_fixed(year, 11, 20).ok()?),
        })
    }

    /// The number of the week of Ordinary Time after Pentecost that holds
    /// `day`: 34 less the weeks from the Sunday that begins it to the last.
    fn week_after_pentecost(&self, day: Rd) -> Option<u8> {
        let sunday = Weekday::Sunday.on_or_before(day);
        let weeks_before = u8::try_from((self.last_sunday.0 - sunday.0) / 7).ok()?;
        LAST_SUNDAY_IN_ORDINARY_TIME.checked_sub(weeks_before)
    }
}

/// The number the Roman calendar gives a Sunday in Ordinary Time, from the
/// Second, the Sunday between 14 and 20 January, to the Thirty-fourth,
/// Christ the King, on the Sunday between 20 and 26 November.
///
/// Before Lent the Sundays are counted on from the Second; after Pentecost
/// they are counted back from the Thirty-fourth, so the number a Sunday
/// takes after Pentecost is set by the date, not by the Sundays before it,
/// and one number is skipped in a year of 33 weeks. The Liturgy Office of
/// England and Wales tabulates each Sunday's window (`liturgyoffice-
/// sundays`): the 2nd on 14–20 January, the 8th on 22–28 May, the 34th on
/// 20–26 November. A Sunday that a solemnity or feast takes the place of
/// — Trinity Sunday, the Presentation on 2 February — keeps its number
/// here: which celebration is kept is the ordo's question.
///
/// Returns `None` for a day that is not a Sunday, for a Sunday outside
/// Ordinary Time (the Baptism of the Lord among them), before
/// [`ROMAN_REFORM_IN_EFFECT`] and outside the Gregorian computus's years.
///
/// ```
/// use hc_holiday::lectionary::sunday_in_ordinary_time;
/// use hc_calendars_solar::gregorian;
///
/// // 2026: the 2nd Sunday on 18 January, the 10th on 7 June, the Sunday
/// // after Trinity Sunday, and Christ the King on 22 November.
/// assert_eq!(sunday_in_ordinary_time(gregorian::to_fixed(2026, 1, 18)?), Some(2));
/// assert_eq!(sunday_in_ordinary_time(gregorian::to_fixed(2026, 6, 7)?), Some(10));
/// assert_eq!(sunday_in_ordinary_time(gregorian::to_fixed(2026, 11, 22)?), Some(34));
/// # Ok::<(), hc_calendar::CalendarError>(())
/// ```
#[must_use]
pub fn sunday_in_ordinary_time(day: Rd) -> Option<u8> {
    if Weekday::from_rd(day) != Weekday::Sunday || day < ROMAN_REFORM_IN_EFFECT {
        return None;
    }
    let time = OrdinaryTime::of(gregorian::year_from_fixed(day).ok()?)?;
    if day >= time.second_sunday && day < time.ash_wednesday {
        return u8::try_from((day.0 - time.second_sunday.0) / 7 + 2).ok();
    }
    if day > time.pentecost && day <= time.last_sunday {
        return time.week_after_pentecost(day);
    }
    None
}

/// The week of Ordinary Time a day falls in, on the calendar that keeps
/// the Epiphany on 6 January and the Baptism of the Lord on the Sunday
/// after it, as the General Roman Calendar does ([`crate::roman_calendar`]).
///
/// Ordinary Time's first part runs from the Monday after the Baptism to the
/// Tuesday before Ash Wednesday; its first week is the days before the
/// Second Sunday, and a week then runs from Sunday to Saturday. Its second
/// part runs from the Monday after Pentecost to the Saturday before the
/// First Sunday of Advent, the weeks numbered back from the Thirty-fourth
/// (the Liturgy Office of England and Wales, "Dates for Sundays" and the
/// Table of Moveable Feasts 2020–2060, `liturgyoffice-sundays`,
/// `liturgyoffice-moveable`). The week after Pentecost is therefore
/// not always one more than the week before Lent.
///
/// Returns `None` for a day outside Ordinary Time, before
/// [`ROMAN_REFORM_IN_EFFECT`] and outside the Gregorian computus's years. [`week_of_ordinary_time_epiphany_on_sunday`]
/// is the reckoning of the conferences that keep the Epiphany on a Sunday.
#[must_use]
pub fn week_of_ordinary_time(day: Rd) -> Option<u8> {
    week_of_ordinary_time_from(day, |year| {
        // The Baptism of the Lord, the Sunday after 6 January.
        let epiphany = gregorian::to_fixed(year, 1, 6).ok()?;
        Some(Weekday::Sunday.after(epiphany))
    })
}

/// The week of Ordinary Time a day falls in, on the calendar of a
/// conference that keeps the Epiphany on the Sunday between 2 and
/// 8 January, as England and Wales do. The Baptism of the Lord is then the
/// Sunday after it, or the Monday after it "When the Epiphany falls on
/// either 7 or 8 January" (`liturgyoffice-sundays`), and Ordinary Time
/// begins the day after the Baptism. Otherwise as [`week_of_ordinary_time`]:
/// the two differ only on the day after the Baptism when that is a Monday,
/// which is Ordinary Time on one calendar and the Baptism on the other.
///
/// This is the reckoning of the Liturgy Office's Table of Moveable Feasts
/// (`liturgyoffice-moveable`), whose weeks of Ordinary Time it gives for
/// every year from 2020 to 2060.
#[must_use]
pub fn week_of_ordinary_time_epiphany_on_sunday(day: Rd) -> Option<u8> {
    week_of_ordinary_time_from(day, |year| {
        let epiphany = Weekday::Sunday.on_or_after(gregorian::to_fixed(year, 1, 2).ok()?);
        let (_, _, date) = gregorian::from_fixed(epiphany).ok()?;
        Some(if date >= 7 {
            Rd(epiphany.0 + 1)
        } else {
            Weekday::Sunday.after(epiphany)
        })
    })
}

/// The week of Ordinary Time of a day, given the day of the Baptism of the
/// Lord in a year.
fn week_of_ordinary_time_from(day: Rd, baptism: impl Fn(i64) -> Option<Rd>) -> Option<u8> {
    if day < ROMAN_REFORM_IN_EFFECT {
        return None;
    }
    let year = gregorian::year_from_fixed(day).ok()?;
    let time = OrdinaryTime::of(year)?;
    if day > baptism(year)? && day < time.ash_wednesday {
        if day < time.second_sunday {
            return Some(1);
        }
        return u8::try_from((day.0 - time.second_sunday.0) / 7 + 2).ok();
    }
    if day > time.pentecost && day.0 < time.last_sunday.0 + 7 {
        return time.week_after_pentecost(day);
    }
    None
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
        // The RCL begins with Advent 1992: its first Proper 29 is 1993's,
        // and 22 November 1992 has none.
        assert_eq!(rcl_proper(greg(1992, 11, 22)), None);
        for year in 1993..=2040 {
            let last = Weekday::Sunday.on_or_after(greg(year, 11, 20));
            assert_eq!(rcl_proper(last), Some(29), "{year}");
            assert_eq!(rcl_proper(Rd(last.0 + 7)), None, "{year}: Advent");
            assert_eq!(rcl_proper(Rd(last.0 - 7)), Some(28), "{year}");
        }
    }

    #[test]
    fn an_easter_on_22_march_makes_the_sunday_after_trinity_proper_3() {
        // 2285 has the earliest Easter, 22 March, as 1818 had (Meeus), and
        // the first since the RCL began; Trinity Sunday is 17 May and the
        // Sunday after it 24 May. In 1818 the RCL was not yet in use.
        assert_eq!(computus::gregorian_easter(2285), Some(greg(2285, 3, 22)));
        assert_eq!(rcl_proper(greg(2285, 5, 17)), None);
        assert_eq!(rcl_proper(greg(2285, 5, 24)), Some(3));
        assert_eq!(rcl_proper(greg(1818, 5, 24)), None);
        // In no year is a Proper lower than 3.
        for year in 1993..=2500 {
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
        // The Roman cycles begin on 1 January 1970, when Mysterii Paschalis
        // put the calendar of 1969 into effect: the liturgical year 1970,
        // whose Advent year 1969 leaves 1 over three, is Year B.
        assert_eq!(ROMAN_REFORM_IN_EFFECT, greg(1970, 1, 1));
        assert_eq!(sunday_cycle(greg(1583, 6, 1)), None);
        assert_eq!(sunday_cycle(greg(1969, 12, 31)), None);
        assert_eq!(sunday_cycle(greg(1970, 1, 1)), Some(SundayCycle::B));
        assert_eq!(roman_weekday_cycle(greg(1969, 12, 31)), None);
        assert_eq!(
            roman_weekday_cycle(greg(1970, 1, 1)),
            Some(WeekdayCycle::II)
        );
        assert_eq!(week_of_ordinary_time(greg(1969, 6, 10)), None);
        assert!(week_of_ordinary_time(greg(1970, 6, 10)).is_some());
        assert_eq!(sunday_in_ordinary_time(greg(1969, 11, 23)), None);
        assert_eq!(sunday_in_ordinary_time(greg(1970, 11, 22)), Some(34));
        assert_eq!(sunday_cycle(greg(4099, 6, 1)), Some(SundayCycle::A));
        assert_eq!(sunday_cycle(greg(4099, 12, 25)), None);
    }

    /// The Liturgy Office's Table of Moveable Feasts 2020–2060: for each
    /// year, the number of weeks of Ordinary Time before Lent and the day
    /// that part ends, and the day Ordinary Time begins again after Easter
    /// with the number of its week. The table is England and Wales's,
    /// whose Epiphany is on a Sunday.
    #[test]
    fn the_liturgy_office_weeks_of_ordinary_time_2020_to_2060() {
        // (year, weeks before Lent, their last day, the day Ordinary Time
        // resumes, its week)
        type Row = (i64, u8, (u8, u8), (u8, u8), u8);
        let table: [Row; 41] = [
            (2020, 7, (2, 25), (6, 1), 9),
            (2021, 6, (2, 16), (5, 24), 8),
            (2022, 8, (3, 1), (6, 6), 10),
            (2023, 7, (2, 21), (5, 29), 8),
            (2024, 6, (2, 13), (5, 20), 7),
            (2025, 8, (3, 4), (6, 9), 10),
            (2026, 6, (2, 17), (5, 25), 8),
            (2027, 5, (2, 9), (5, 17), 7),
            (2028, 8, (2, 29), (6, 5), 9),
            (2029, 6, (2, 13), (5, 21), 7),
            (2030, 8, (3, 5), (6, 10), 10),
            (2031, 7, (2, 25), (6, 2), 9),
            (2032, 5, (2, 10), (5, 17), 7),
            (2033, 8, (3, 1), (6, 6), 10),
            (2034, 7, (2, 21), (5, 29), 8),
            (2035, 4, (2, 6), (5, 14), 6),
            (2036, 7, (2, 26), (6, 2), 9),
            (2037, 6, (2, 17), (5, 25), 8),
            (2038, 9, (3, 9), (6, 14), 11),
            (2039, 7, (2, 22), (5, 30), 9),
            (2040, 6, (2, 14), (5, 21), 7),
            (2041, 8, (3, 5), (6, 10), 10),
            (2042, 6, (2, 18), (5, 26), 8),
            (2043, 5, (2, 10), (5, 18), 7),
            (2044, 8, (3, 1), (6, 6), 10),
            (2045, 7, (2, 21), (5, 29), 8),
            (2046, 5, (2, 6), (5, 14), 6),
            (2047, 7, (2, 26), (6, 3), 9),
            (2048, 6, (2, 18), (5, 25), 8),
            (2049, 8, (3, 2), (6, 7), 10),
            (2050, 7, (2, 22), (5, 30), 9),
            (2051, 6, (2, 14), (5, 22), 7),
            (2052, 9, (3, 5), (6, 10), 10),
            (2053, 6, (2, 18), (5, 26), 8),
            (2054, 5, (2, 10), (5, 18), 7),
            (2055, 8, (3, 2), (6, 7), 10),
            (2056, 6, (2, 15), (5, 22), 7),
            (2057, 9, (3, 6), (6, 11), 10),
            (2058, 7, (2, 26), (6, 3), 9),
            (2059, 5, (2, 11), (5, 19), 7),
            (2060, 8, (3, 2), (6, 7), 10),
        ];
        for (year, weeks, (em, ed), (bm, bd), week) in table {
            let ending = greg(year, em, ed);
            let beginning = greg(year, bm, bd);
            let before = week_of_ordinary_time_epiphany_on_sunday;
            // The table prints 4 weeks for 2035 and 5 for 2046, two years
            // of the same days: both begin on a Monday, with the Epiphany
            // on Sunday 7 January, the Baptism on Monday 8 January, Easter
            // on 25 March and Ash Wednesday on 7 February. The Sundays of
            // 14 January to 4 February are the 2nd to the 5th, so the week
            // that ends on 6 February is the 5th in both; 2035's 4 is the
            // table's slip, and the only row that disagrees.
            let weeks = if year == 2035 { 5 } else { weeks };
            assert_eq!(
                before(ending),
                Some(weeks),
                "{year}: the last week before Lent"
            );
            assert_eq!(before(Rd(ending.0 + 1)), None, "{year}: Ash Wednesday");
            assert_eq!(before(Rd(beginning.0 - 1)), None, "{year}: Pentecost");
            assert_eq!(
                before(beginning),
                Some(week),
                "{year}: the week after Pentecost"
            );
            // The universal calendar's weeks are the same on these days.
            assert_eq!(week_of_ordinary_time(ending), Some(weeks), "{year}");
            assert_eq!(week_of_ordinary_time(beginning), Some(week), "{year}");
            // The last week, 34, ends the Saturday before Advent.
            let advent = first_sunday_of_advent(year).expect("in range");
            assert_eq!(before(Rd(advent.0 - 1)), Some(34), "{year}");
            assert_eq!(before(advent), None, "{year}: Advent");
        }
    }

    /// The Liturgy Office's Dates for Sundays: the 2nd Sunday in Ordinary
    /// Time on 14–20 January, the 3rd on 21–27 January, and so on to the
    /// 9th, whose window ends on 7 March; after Pentecost the 8th on
    /// 22–28 May to the 34th on 20–26 November.
    #[test]
    fn every_sunday_in_ordinary_time_is_in_the_liturgy_office_window() {
        for year in 1970..=2600 {
            let mut sunday = Weekday::Sunday.on_or_after(greg(year, 1, 1));
            while gregorian::year_from_fixed(sunday).unwrap() == year {
                if let Some(n) = sunday_in_ordinary_time(sunday) {
                    let window = if n <= 9 && sunday < greg(year, 3, 10) {
                        greg(year, 1, 14).0 + 7 * (i64::from(n) - 2)
                    } else {
                        greg(year, 11, 20).0 - 7 * i64::from(34 - n)
                    };
                    assert!(
                        (window..window + 7).contains(&sunday.0),
                        "{year}: Sunday {n} on {:?}",
                        gregorian::from_fixed(sunday)
                    );
                    assert_eq!(week_of_ordinary_time(sunday), Some(n));
                }
                sunday = Rd(sunday.0 + 7);
            }
        }
    }

    #[test]
    fn the_sundays_of_ordinary_time_in_2026() {
        // Easter 5 April 2026: Ash Wednesday 18 February, Pentecost 24 May.
        assert_eq!(
            sunday_in_ordinary_time(greg(2026, 1, 11)),
            None,
            "the Baptism"
        );
        assert_eq!(sunday_in_ordinary_time(greg(2026, 1, 18)), Some(2));
        assert_eq!(sunday_in_ordinary_time(greg(2026, 2, 15)), Some(6));
        assert_eq!(sunday_in_ordinary_time(greg(2026, 2, 22)), None, "Lent");
        assert_eq!(
            sunday_in_ordinary_time(greg(2026, 5, 24)),
            None,
            "Pentecost"
        );
        // Trinity Sunday takes the place of the 9th, the week of 25 May
        // being the 8th: 2026 has 34 weeks, 6 before Lent and 8 to 34.
        assert_eq!(sunday_in_ordinary_time(greg(2026, 5, 31)), Some(9));
        assert_eq!(sunday_in_ordinary_time(greg(2026, 6, 7)), Some(10));
        assert_eq!(sunday_in_ordinary_time(greg(2026, 11, 22)), Some(34));
        assert_eq!(sunday_in_ordinary_time(greg(2026, 11, 29)), None, "Advent");
        assert_eq!(sunday_in_ordinary_time(greg(2026, 6, 8)), None, "a Monday");
        // The RCL's bracket, Ordinary Time = Proper + 5, is the same number.
        assert_eq!(rcl_proper(greg(2026, 6, 7)), Some(5));
        assert_eq!(week_of_ordinary_time(greg(2026, 5, 25)), Some(8));
        assert_eq!(week_of_ordinary_time(greg(2026, 1, 12)), Some(1));
        assert_eq!(week_of_ordinary_time(greg(2026, 1, 11)), None);
    }

    #[test]
    fn the_two_epiphany_reckonings_differ_only_on_a_monday_baptism() {
        // 2023: 6 January a Friday. The universal calendar keeps the
        // Baptism on Sunday 8 January, and 9 January is the first day of
        // Ordinary Time; with the Epiphany on Sunday 8 January, the Baptism
        // is Monday 9 January and Ordinary Time begins on the 10th.
        assert_eq!(week_of_ordinary_time(greg(2023, 1, 9)), Some(1));
        assert_eq!(
            week_of_ordinary_time_epiphany_on_sunday(greg(2023, 1, 9)),
            None
        );
        assert_eq!(
            week_of_ordinary_time_epiphany_on_sunday(greg(2023, 1, 10)),
            Some(1)
        );
        for year in 1583..=2600 {
            for day in greg(year, 1, 1).0..greg(year, 12, 31).0 {
                let (a, b) = (
                    week_of_ordinary_time(Rd(day)),
                    week_of_ordinary_time_epiphany_on_sunday(Rd(day)),
                );
                if a != b {
                    assert_eq!((a, b), (Some(1), None), "{year}");
                    assert_eq!(Weekday::from_rd(Rd(day)), Weekday::Monday);
                }
            }
        }
        assert_eq!(week_of_ordinary_time(greg(4100, 6, 1)), None);
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
