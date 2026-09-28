//! The Olympiads: the Greek count of four-year periods from the Games of
//! 776 BC, and the modern Olympiads the International Olympic Committee
//! numbers from 1896.
//!
//! # The ancient count
//!
//! Timaeus of Tauromenium, in the third century BC, was the first to date
//! consistently by Olympiad, and Christian chroniclers kept the count: the
//! *Chronicon Paschale* runs to the 352nd Olympiad (Wikipedia, "Olympiad",
//! `wikipedia-olympiad`, retrieved 2026-09-26). Year 1 of Olympiad 1 is
//! Coroebus's victory "in the summer of 776 BC", and Olympiad *N* before
//! the 195th began in 780 − 4*N* BC; Jerome's year 3 of Olympiad 194 is
//! 2 BC. The Olympic year began with the Games, near midsummer, so each
//! straddles two Julian years.
//!
//! This module does what Reingold and Dershowitz's published code does
//! (`reingold2018code`, `calendar.l`, read 2026-09-26: `olympiad-start`,
//! "(bce 776)", `olympiad-from-julian-year` and `julian-year-from-olympiad`)
//! and treats each Olympic year as the Julian year it begins in, from
//! 1 January: [`from_julian_year`] and [`julian_year`] are those two
//! functions in astronomical year numbering, and [`OlympiadCalendar`] is
//! the Julian calendar with the Olympiad and its year beside each date.
//! The midsummer boundary is not modelled — no source read fixes its day,
//! and it moved with the full moon — so a date between January and the
//! Games is given the Olympic year that begins the following summer, by
//! nobody's reckoning but this convention's: 1 March 775 BC is year 2 of
//! Olympiad 1, although the Games that opened that year were months away.
//! The count is a year count and is used as one.
//!
//! # The modern count
//!
//! The IOC counts Olympiads from 1896. Since the Olympic Charter of
//! 1 September 2004 an Olympiad "is a period of four consecutive calendar
//! years, beginning on the first of January of the first year and ending
//! on the 31st of December of the fourth year"; before it, an Olympiad ran
//! from the opening of one Games to the opening of the next (Olympedia,
//! "Olympiad", `olympedia-olympiad`, retrieved 2026-09-26). The first "started on 1 January 1896, and an Olympiad starts
//! on 1 January of the years evenly divisible by four", and Games that were
//! not held keep their numbers — "not celebrated" in 1916, 1940 and 1944 —
//! while the 2020 Games of the XXXII Olympiad were "postponed to 2021
//! rather than cancelled" (Wikipedia, "Olympiad"). [`ioc_olympiad`] is the
//! 2004 definition, a function of the Gregorian year rather than a
//! calendar. [`ioc_olympiad_before_2004`] is the earlier definition, a
//! function of the day: the Charter's Rule 10 in force until then, "the
//! Olympiad begins with the opening of one edition of the Games of the
//! Olympiad and ends with the opening of the following edition", and "in
//! the event of non-celebration of the Games of an Olympiad, such Olympiad
//! begins four years after the start of the preceding Olympiad" (*Olympic
//! Charter*, in force from 11 September 2000, Rule 10, read in the
//! Wayback Machine's copy of olympic.org, 2026-09-29), with the opening
//! days in [`IOC_OPENINGS`]. The system document is
//! `docs/systems/olympiads.md` in the repository.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_calendars_solar::julian;

/// The astronomical Julian year of Olympiad 1, year 1: 776 BC.
pub const OLYMPIAD_START: i64 = -775;

/// The Olympiad and the year within it, 1 to 4, of an astronomical Julian
/// year, as `olympiad-from-julian-year` gives them.
///
/// Years before 776 BC give an Olympiad below 1, as the published function
/// does; [`OlympiadCalendar`] refuses them.
#[must_use]
pub const fn from_julian_year(year: i64) -> (i64, u8) {
    let elapsed = year - OLYMPIAD_START;
    (elapsed.div_euclid(4) + 1, (elapsed.rem_euclid(4) + 1) as u8)
}

/// The astronomical Julian year of year `year` of Olympiad `olympiad`, as
/// `julian-year-from-olympiad` gives it.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] for an Olympiad below 1 or a
/// year outside 1 to 4.
pub const fn julian_year(olympiad: i64, year: u8) -> CalendarResult<i64> {
    if olympiad < 1 || year < 1 || year > 4 {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(OLYMPIAD_START + 4 * (olympiad - 1) + year as i64 - 1)
}

/// The first Gregorian year of the I Olympiad of the modern count.
pub const IOC_FIRST_YEAR: i64 = 1_896;

/// The modern Olympiads whose Games of the Olympiad were not celebrated:
/// the VI (1916), the XII (1940) and the XIII (1944), as Wikipedia,
/// "Olympiad", records them to the Games of the XXXIII Olympiad.
pub const IOC_GAMES_NOT_CELEBRATED: [i64; 3] = [6, 12, 13];

/// The number of the modern Olympiad a Gregorian year belongs to, under
/// the Olympic Charter's definition of 2004, or `None` before 1896.
#[must_use]
pub const fn ioc_olympiad(gregorian_year: i64) -> Option<i64> {
    if gregorian_year < IOC_FIRST_YEAR {
        return None;
    }
    Some((gregorian_year - IOC_FIRST_YEAR).div_euclid(4) + 1)
}

/// The first and last Gregorian years of a modern Olympiad, or `None` for a
/// number below 1.
#[must_use]
pub const fn ioc_olympiad_years(olympiad: i64) -> Option<(i64, i64)> {
    if olympiad < 1 {
        return None;
    }
    let first = IOC_FIRST_YEAR + 4 * (olympiad - 1);
    Some((first, first + 3))
}

/// The opening of the Games of one modern Olympiad before the Charter of
/// 2004, or the day the Olympiad began without Games.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IocOpening {
    /// The Olympiad, from 1.
    pub olympiad: i64,
    /// The day it began: the opening ceremony of its Games, as Olympedia
    /// dates it; for 1900, which had "no official opening", the first day
    /// of the Games; for the Olympiads whose Games were not celebrated,
    /// the day four years after the start of the one before.
    pub from: Rd,
    /// Where the Olympiad began on a later day than `from` by another
    /// reading, the last day of the doubt: the XVI, whose equestrian Games
    /// opened at Stockholm on 10 June 1956 and whose Games at Melbourne on
    /// 22 November. `None` elsewhere.
    pub doubtful_until: Option<Rd>,
}

/// The day the Charter of 2004 took effect, 1 September 2004, from which an
/// Olympiad is four calendar years ([`ioc_olympiad`]).
pub const IOC_CHARTER_2004: Rd = hc_calendar::gregorian::to_fixed_saturating(2004, 9, 1);

const fn opening(olympiad: i64, year: i64, month: u8, day: u8) -> IocOpening {
    IocOpening {
        olympiad,
        from: hc_calendar::gregorian::to_fixed_saturating(year, month, day),
        doubtful_until: None,
    }
}

/// The beginnings of the modern Olympiads under the Charter's definition
/// before 1 September 2004, in order: the opening ceremony of each Games of
/// the Olympiad as Olympedia's editions give it (retrieved 2026-09-29), the
/// Gregorian day for 1896 (25 March Julian), checked against olympics.com's
/// Games pages where those give the ceremony; 14 May 1900, the first day of
/// Games that had "no official opening" (olympics.com) and whose "true
/// Opening and Closing Ceremonies were not held" (Olympedia); 14 May 1904,
/// the ceremony Olympedia dates, though competition began on 1 July; and
/// for the VI, XII and XIII, Rule 10.2's "four years after the start of the
/// preceding Olympiad", read as the same day of the month four years on,
/// which the rule does not state.
pub const IOC_OPENINGS: [IocOpening; 28] = [
    opening(1, 1896, 4, 6),
    opening(2, 1900, 5, 14),
    opening(3, 1904, 5, 14),
    opening(4, 1908, 7, 13),
    opening(5, 1912, 7, 6),
    opening(6, 1916, 7, 6),
    opening(7, 1920, 8, 14),
    opening(8, 1924, 7, 5),
    opening(9, 1928, 7, 28),
    opening(10, 1932, 7, 30),
    opening(11, 1936, 8, 1),
    opening(12, 1940, 8, 1),
    opening(13, 1944, 8, 1),
    opening(14, 1948, 7, 29),
    opening(15, 1952, 7, 19),
    IocOpening {
        olympiad: 16,
        from: hc_calendar::gregorian::to_fixed_saturating(1956, 6, 10),
        doubtful_until: Some(hc_calendar::gregorian::to_fixed_saturating(1956, 11, 21)),
    },
    opening(17, 1960, 8, 25),
    opening(18, 1964, 10, 10),
    opening(19, 1968, 10, 12),
    opening(20, 1972, 8, 26),
    opening(21, 1976, 7, 17),
    opening(22, 1980, 7, 19),
    opening(23, 1984, 7, 28),
    opening(24, 1988, 9, 17),
    opening(25, 1992, 7, 25),
    opening(26, 1996, 7, 19),
    opening(27, 2000, 9, 15),
    opening(28, 2004, 8, 13),
];

/// The modern Olympiad a day belongs to under the Charter's definition
/// before 1 September 2004, from the opening of one Games to the opening
/// of the next ([`IOC_OPENINGS`]).
///
/// `None` before the opening of 6 April 1896; from 1 September 2004, where
/// [`ioc_olympiad`] takes over; and from 10 June to 21 November 1956, which
/// is the XV Olympiad if the Melbourne Games opened the XVI and the XVI if
/// the Stockholm equestrian Games did, on which no source read rules.
#[must_use]
pub const fn ioc_olympiad_before_2004(rd: Rd) -> Option<i64> {
    if rd.0 >= IOC_CHARTER_2004.0 {
        return None;
    }
    let mut index = IOC_OPENINGS.len();
    while index > 0 {
        index -= 1;
        let opening = IOC_OPENINGS[index];
        if opening.from.0 <= rd.0 {
            return match opening.doubtful_until {
                Some(last) if rd.0 <= last.0 => None,
                _ => Some(opening.olympiad),
            };
        }
    }
    None
}

/// A Julian date with its Olympiad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OlympiadDate {
    /// The Olympiad, from 1.
    pub olympiad: i64,
    /// The year of the Olympiad, 1 to 4.
    pub year: u8,
    /// The Julian month.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl OlympiadDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub fn new(olympiad: i64, year: u8, month: u8, day: u8) -> CalendarResult<Self> {
        julian::to_fixed(julian_year(olympiad, year)?, month, day)?;
        Ok(Self {
            olympiad,
            year,
            month,
            day,
        })
    }

    /// The astronomical Julian year of the date.
    ///
    /// # Errors
    ///
    /// As [`julian_year`].
    pub const fn julian_year(self) -> CalendarResult<i64> {
        julian_year(self.olympiad, self.year)
    }
}

/// The ancient Olympiads over the Julian year.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OlympiadCalendar;

/// The first day this calendar converts, 1 January 776 BC: the published
/// code's convention, the Olympic year taken as the Julian year. Grumel
/// dates the era from "the beginning of July 776 b.c." (`grumel-eras-historical`),
/// and `docs/systems/olympiads.md` says why this calendar does not.
pub const EARLIEST: Rd = match julian::to_fixed(OLYMPIAD_START, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

impl Calendar for OlympiadCalendar {
    type Date = OlympiadDate;

    /// Unrecorded as a period of days: the source dates the count's use by
    /// the century and the Olympiad, from Timaeus to the *Chronicon
    /// Paschale*.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// The Julian months and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    /// The Julian rule, on the Julian year the fields carry.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if year < OLYMPIAD_START {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(julian::is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("olympiad"),
            english_name: "Olympiads",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(julian::LATEST),
            native_locales: &["grc"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        julian::to_fixed(date.julian_year()?, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        if rd < EARLIEST {
            return Err(CalendarError::BeforeEpoch);
        }
        let (year, month, day) = julian::from_fixed(rd)?;
        let (olympiad, year) = from_julian_year(year);
        Ok(OlympiadDate {
            olympiad,
            year,
            month,
            day,
        })
    }

    /// The Julian year, month and day, in astronomical numbering, and the
    /// Olympiad and its year as extra fields.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.julian_year()?, date.month, date.day)
            .with_extra("olympiad", date.olympiad)?
            .with_extra("year-of-olympiad", i64::from(date.year))
    }

    /// The Julian year of the fields, or the Olympiad and its year when
    /// both are given; when both are given they must agree.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some() {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        let (olympiad, year) = from_julian_year(fields.year);
        if let (Some(given), Some(within)) = (
            fields.extra.get("olympiad"),
            fields.extra.get("year-of-olympiad"),
        ) && (given, within) != (olympiad, i64::from(year))
        {
            return Err(CalendarError::YearOutOfRange);
        }
        OlympiadDate::new(olympiad, year, month.ordinal, day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The Charter's Rule 10 before 2004, with the opening days: the I
    /// from 6 April 1896, the VI from 6 July 1916 by Rule 10.2, the doubt
    /// of 1956, and the XXVIII from 13 August 2004 until the Charter of
    /// 1 September 2004 took over.
    #[test]
    fn the_olympiads_before_2004_run_from_opening_to_opening() {
        let day = hc_calendar::gregorian::to_fixed_saturating;
        assert_eq!(ioc_olympiad_before_2004(day(1896, 4, 5)), None);
        assert_eq!(ioc_olympiad_before_2004(day(1896, 4, 6)), Some(1));
        assert_eq!(ioc_olympiad_before_2004(day(1900, 5, 13)), Some(1));
        assert_eq!(ioc_olympiad_before_2004(day(1900, 5, 14)), Some(2));
        // London's Games began on 27 April 1908 and opened on 13 July.
        assert_eq!(ioc_olympiad_before_2004(day(1908, 7, 12)), Some(3));
        assert_eq!(ioc_olympiad_before_2004(day(1908, 7, 13)), Some(4));
        assert_eq!(ioc_olympiad_before_2004(day(1916, 7, 5)), Some(5));
        assert_eq!(ioc_olympiad_before_2004(day(1916, 7, 6)), Some(6));
        assert_eq!(ioc_olympiad_before_2004(day(1920, 8, 14)), Some(7));
        assert_eq!(ioc_olympiad_before_2004(day(1940, 8, 1)), Some(12));
        assert_eq!(ioc_olympiad_before_2004(day(1944, 8, 1)), Some(13));
        assert_eq!(ioc_olympiad_before_2004(day(1948, 7, 28)), Some(13));
        assert_eq!(ioc_olympiad_before_2004(day(1948, 7, 29)), Some(14));
        assert_eq!(ioc_olympiad_before_2004(day(1956, 6, 9)), Some(15));
        assert_eq!(ioc_olympiad_before_2004(day(1956, 6, 10)), None);
        assert_eq!(ioc_olympiad_before_2004(day(1956, 11, 21)), None);
        assert_eq!(ioc_olympiad_before_2004(day(1956, 11, 22)), Some(16));
        assert_eq!(ioc_olympiad_before_2004(day(2000, 9, 14)), Some(26));
        assert_eq!(ioc_olympiad_before_2004(day(2000, 9, 15)), Some(27));
        assert_eq!(ioc_olympiad_before_2004(day(2004, 8, 12)), Some(27));
        assert_eq!(ioc_olympiad_before_2004(day(2004, 8, 13)), Some(28));
        assert_eq!(ioc_olympiad_before_2004(day(2004, 8, 31)), Some(28));
        assert_eq!(ioc_olympiad_before_2004(day(2004, 9, 1)), None);
        // Every opening is later than the one before, the Olympiads run
        // 1 to 28 without a gap, and each Games opened in the Olympiad's
        // first year by the 2004 count.
        for (index, pair) in IOC_OPENINGS.windows(2).enumerate() {
            assert!(pair[0].from < pair[1].from);
            assert_eq!(pair[0].olympiad, index as i64 + 1);
        }
        for opening in IOC_OPENINGS {
            let (year, _, _) = hc_calendar::gregorian::ymd(opening.from);
            assert_eq!(ioc_olympiad(year), Some(opening.olympiad));
            let not_celebrated = IOC_GAMES_NOT_CELEBRATED.contains(&opening.olympiad);
            if not_celebrated {
                let before = IOC_OPENINGS[opening.olympiad as usize - 2].from;
                let (y0, m0, d0) = hc_calendar::gregorian::ymd(before);
                assert_eq!(opening.from, day(y0 + 4, m0, d0));
            }
        }
    }

    #[test]
    fn the_first_olympiad_is_776_bc_and_jeromes_194_3_is_2_bc() {
        // Reingold and Dershowitz: `olympiad-start` is (bce 776), astronomical
        // -775. Wikipedia, "Olympiad": Jerome's year 3 of Olympiad 194 is
        // 2 BC, astronomical -1; Olympiad N below 195 begins in 780 - 4N BC.
        assert_eq!(from_julian_year(-775), (1, 1));
        assert_eq!(julian_year(194, 3), Ok(-1));
        assert_eq!(from_julian_year(-1), (194, 3));
        for olympiad in 1..195 {
            let bc = 780 - 4 * olympiad;
            assert_eq!(julian_year(olympiad, 1), Ok(1 - bc), "Olympiad {olympiad}");
        }
        // 1 BC is year 4 of Olympiad 194 and AD 1 opens Olympiad 195, as
        // the published function's two branches give them.
        assert_eq!(from_julian_year(0), (194, 4));
        assert_eq!(from_julian_year(1), (195, 1));
    }

    #[test]
    fn the_two_functions_are_inverses() {
        for year in OLYMPIAD_START..3_000 {
            let (olympiad, within) = from_julian_year(year);
            assert!((1..=4).contains(&within));
            assert_eq!(julian_year(olympiad, within), Ok(year));
        }
        assert_eq!(julian_year(0, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(julian_year(1, 5), Err(CalendarError::YearOutOfRange));
        assert_eq!(from_julian_year(-776), (0, 4));
    }

    #[test]
    fn the_modern_olympiads_count_from_1896_with_the_lost_games_numbered() {
        // Wikipedia, "Olympiad": the 1936 Games were the XI Olympiad's and
        // the next Summer Games, 1948, the XIV's; the VI of 1916-1919; the
        // XXXII Olympiad's Games of 2020 held in 2021.
        assert_eq!(ioc_olympiad(1895), None);
        assert_eq!(ioc_olympiad(1896), Some(1));
        assert_eq!(ioc_olympiad(1899), Some(1));
        assert_eq!(ioc_olympiad(1916), Some(6));
        assert_eq!(ioc_olympiad(1919), Some(6));
        assert_eq!(ioc_olympiad(1936), Some(11));
        assert_eq!(ioc_olympiad(1948), Some(14));
        assert_eq!(ioc_olympiad(2020), Some(32));
        assert_eq!(ioc_olympiad(2021), Some(32));
        assert_eq!(ioc_olympiad_years(6), Some((1916, 1919)));
        assert_eq!(ioc_olympiad_years(0), None);
        for olympiad in IOC_GAMES_NOT_CELEBRATED {
            let (first, _) = ioc_olympiad_years(olympiad).unwrap();
            assert!(matches!(first, 1916 | 1940 | 1944));
        }
    }

    #[test]
    fn the_calendar_is_the_julian_calendar_with_the_olympiad_beside_it() {
        let calendar = OlympiadCalendar;
        let day = julian::to_fixed(-1, 7, 1).unwrap();
        let date = calendar.from_fixed(day).unwrap();
        assert_eq!(
            (date.olympiad, date.year, date.month, date.day),
            (194, 3, 7, 1)
        );
        let fields = calendar.to_fields(date).unwrap();
        assert_eq!(fields.year, -1);
        assert_eq!(fields.extra.get("olympiad"), Some(194));
        assert_eq!(fields.extra.get("year-of-olympiad"), Some(3));
        // A day between January and the Games takes the Olympic year that
        // begins the following summer: 1 March 775 BC is year 2 of
        // Olympiad 1, as the module documentation says.
        let march = calendar
            .from_fixed(julian::to_fixed(-774, 3, 1).unwrap())
            .unwrap();
        assert_eq!((march.olympiad, march.year), (1, 2));
        // Every day of the first 2 000 000 in a release build; every 733rd
        // in a debug one, with each 1 January and the day before.
        let new_years = (OLYMPIAD_START..OLYMPIAD_START + 5_500)
            .map(|year| julian::to_fixed(year, 1, 1).unwrap().0);
        for rd in crate::sweep_days(EARLIEST.0, EARLIEST.0 + 1_999_999, 733, new_years) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date));
        }
    }

    #[test]
    fn the_calendar_refuses_before_776_bc_and_disagreeing_fields() {
        let calendar = OlympiadCalendar;
        assert_eq!(
            calendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        let wrong = DateFields::ymd(-1, 7, 1)
            .with_extra("olympiad", 195)
            .unwrap()
            .with_extra("year-of-olympiad", 3)
            .unwrap();
        assert_eq!(
            calendar.from_fields(&wrong),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(-1, 7, 1).with_era("bc")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(-800, 1, 1)),
            Err(CalendarError::YearOutOfRange)
        );
    }
}
