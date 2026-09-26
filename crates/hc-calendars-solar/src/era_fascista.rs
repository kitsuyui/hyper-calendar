//! The Era Fascista, the year count of Fascist Italy.
//!
//! "Day 1 of Anno I of the Era Fascista corresponded to 29 October 1922",
//! the day after the March on Rome, and each Anno begins on 29 October, so
//! a Gregorian date carries the year less 1921 from 29 October and less
//! 1922 before it: 28 October 1936 is Anno XIV, 29 October 1936 Anno XV. The
//! count was "introduced in 1926 (Anno IV) and officialized in 1927
//! (Anno V)", written beside the common year, "abandoned in most of Italy
//! with the fall of the Fascist regime in 1943 (Anno XXI)", and kept in the
//! Republic of Salò "until the death of Mussolini in April 1945
//! (Anno XXIII)". The quotations are Wikipedia, "Era Fascista"
//! (`wikipedia-era-fascista`), retrieved 2026-09-26, with its coin of 1928
//! reading "A.VI" and its sundial of 1939 reading "XVII E F"; the decrees
//! were not read. The system document is `docs/systems/era-counts.md` in
//! the repository.
//!
//! The arithmetic is bounded to the Anni that were written, I to XXIII,
//! 29 October 1922 to 28 October 1945. The months and days are the
//! Gregorian ones, so October falls in two Anni: its 29th to 31st open one
//! and its 1st to 28th close the one before. The source dates the count's
//! use by the year only, so [`EraFascistaCalendar`] records no period of
//! days; [`FIRST_WRITTEN_ANNO`], [`LAST_ANNO_IN_ITALY`] and [`LAST_ANNO`]
//! carry the years it gives.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::gregorian;

/// The era code of the Era Fascista, *E.F.*
pub const ERA: &str = "ef";

/// The month and day each Anno begins on.
pub const NEW_YEAR: (u8, u8) = (10, 29);

/// The Gregorian year Anno I begins in.
pub const FIRST_COMMON_YEAR: i64 = 1_922;

/// Anno IV, in which the count was introduced, 1926.
pub const FIRST_WRITTEN_ANNO: i64 = 4;

/// Anno XXI, in which it was abandoned in most of Italy, 1943.
pub const LAST_ANNO_IN_ITALY: i64 = 21;

/// Anno XXIII, the last the Republic of Salò wrote, and the last this
/// implementation converts.
pub const LAST_ANNO: i64 = 23;

/// The first day of Anno I, 29 October 1922.
pub const EPOCH: Rd = match gregorian::to_fixed(FIRST_COMMON_YEAR, NEW_YEAR.0, NEW_YEAR.1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The last day of Anno XXIII, 28 October 1945.
pub const LATEST: Rd =
    match gregorian::to_fixed(FIRST_COMMON_YEAR + LAST_ANNO, NEW_YEAR.0, NEW_YEAR.1) {
        Ok(rd) => Rd(rd.0 - 1),
        Err(_) => Rd(0),
    };

/// Whether a month and day fall on or after the new year of 29 October.
const fn opens_the_anno(month: u8, day: u8) -> bool {
    month > NEW_YEAR.0 || (month == NEW_YEAR.0 && day >= NEW_YEAR.1)
}

/// The Gregorian year of a month and day of `anno`.
#[must_use]
pub const fn common_year(anno: i64, month: u8, day: u8) -> i64 {
    if opens_the_anno(month, day) {
        anno + FIRST_COMMON_YEAR - 1
    } else {
        anno + FIRST_COMMON_YEAR
    }
}

/// Whether `anno` holds a 29 February: that of the Gregorian year it ends
/// in.
#[must_use]
pub const fn is_leap_year(anno: i64) -> bool {
    gregorian::is_leap_year(anno + FIRST_COMMON_YEAR)
}

/// The fixed day of an Era Fascista date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside Anno I to XXIII, and
/// the Gregorian field errors otherwise.
pub const fn to_fixed(anno: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if anno < 1 || anno > LAST_ANNO {
        return Err(CalendarError::YearOutOfRange);
    }
    gregorian::to_fixed(common_year(anno, month, day), month, day)
}

/// The Anno, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EPOCH`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EPOCH.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    match gregorian::from_fixed(rd) {
        Ok((year, month, day)) => {
            let anno = if opens_the_anno(month, day) {
                year - FIRST_COMMON_YEAR + 1
            } else {
                year - FIRST_COMMON_YEAR
            };
            Ok((anno, month, day))
        }
        Err(error) => Err(error),
    }
}

/// An Era Fascista date: an Anno and a Gregorian month and day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EraFascistaDate {
    /// The Anno, I to XXIII.
    pub anno: i64,
    /// The Gregorian month.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl EraFascistaDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(anno: i64, month: u8, day: u8) -> CalendarResult<Self> {
        match to_fixed(anno, month, day) {
            Err(error) => Err(error),
            Ok(_) => Ok(Self { anno, month, day }),
        }
    }

    /// The Gregorian year the date falls in.
    #[must_use]
    pub const fn common_year(self) -> i64 {
        common_year(self.anno, self.month, self.day)
    }
}

/// The Era Fascista.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EraFascistaCalendar;

impl Calendar for EraFascistaCalendar {
    type Date = EraFascistaDate;

    /// Unrecorded as a period of days: the source gives the years, Anno IV
    /// to XXI in Italy and XXIII in the Republic of Salò, and no day of
    /// either end.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn is_leap_year(&self, anno: i64) -> CalendarResult<bool> {
        if !(1..=LAST_ANNO).contains(&anno) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(anno))
    }

    /// The Gregorian length of the month: October is split between two
    /// Anni and is still a month of thirty-one days.
    fn days_in_month(&self, fields: &DateFields) -> CalendarResult<u16> {
        let date = self.from_fields(fields)?;
        gregorian::days_in_month(date.common_year(), date.month)
            .map(u16::from)
            .ok_or(CalendarError::MonthOutOfRange)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("era-fascista"),
            english_name: "Era Fascista",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EPOCH),
            latest: Some(LATEST),
            native_locales: &["it"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.anno, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (anno, month, day) = from_fixed(rd)?;
        Ok(EraFascistaDate { anno, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.anno, date.month, date.day)
            .with_era(ERA)
            .with_extra("common-era-year", date.common_year())
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        EraFascistaDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn anno_one_begins_on_the_twenty_ninth_of_october_1922() {
        assert_eq!(from_fixed(ymd(1922, 10, 29)), Ok((1, 10, 29)));
        assert_eq!(
            from_fixed(ymd(1922, 10, 28)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(from_fixed(ymd(1923, 10, 28)), Ok((1, 10, 28)));
        assert_eq!(from_fixed(ymd(1923, 10, 29)), Ok((2, 10, 29)));
    }

    #[test]
    fn the_dated_objects_carry_the_anno_the_rule_gives() {
        // The source's coin of 1928, "A.VI", and sundial of 1939, "XVII E F":
        // each year is two Anni, and the one the object names is one of them.
        for (year, anno) in [(1928, 6), (1939, 17)] {
            let spring = from_fixed(ymd(year, 3, 1)).unwrap().0;
            let winter = from_fixed(ymd(year, 12, 1)).unwrap().0;
            assert!(spring == anno || winter == anno, "{year}");
        }
        // Introduced in 1926, Anno IV; abandoned in 1943, Anno XXI; the
        // Republic of Salò to April 1945, Anno XXIII.
        assert_eq!(from_fixed(ymd(1926, 6, 1)).unwrap().0, FIRST_WRITTEN_ANNO);
        assert_eq!(from_fixed(ymd(1943, 6, 1)).unwrap().0, LAST_ANNO_IN_ITALY);
        assert_eq!(from_fixed(ymd(1945, 4, 1)).unwrap().0, LAST_ANNO);
    }

    #[test]
    fn the_last_anno_ends_on_the_twenty_eighth_of_october_1945() {
        assert_eq!(LATEST, ymd(1945, 10, 28));
        assert_eq!(from_fixed(LATEST), Ok((23, 10, 28)));
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(24, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(0, 12, 1), Err(CalendarError::YearOutOfRange));
    }

    #[test]
    fn the_leap_day_of_an_anno_is_the_one_of_the_year_it_ends_in() {
        // Anno XIV runs from 29 October 1935 to 28 October 1936.
        assert!(is_leap_year(14));
        assert_eq!(to_fixed(14, 2, 29), Ok(ymd(1936, 2, 29)));
        assert!(!is_leap_year(15));
        assert_eq!(to_fixed(15, 2, 29), Err(CalendarError::DayOutOfRange));
    }

    #[test]
    fn every_day_round_trips_and_october_keeps_thirty_one_days() {
        let calendar = EraFascistaCalendar;
        for rd in EPOCH.0..=LATEST.0 {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date));
            assert_eq!(
                fields.extra.get("common-era-year"),
                Some(gregorian::from_fixed(Rd(rd)).unwrap().0)
            );
        }
        let october = calendar
            .to_fields(EraFascistaDate::new(15, 10, 5).unwrap())
            .unwrap();
        assert_eq!(calendar.days_in_month(&october), Ok(31));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(15, 1, 1).with_era("ad")),
            Err(CalendarError::UnknownEra)
        );
    }
}
