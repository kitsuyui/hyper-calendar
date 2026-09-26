//! The Byzantine calendar, *Anno Mundi*.
//!
//! The Julian calendar with two changes: the year is numbered from the
//! creation of the world as the Byzantine chancery reckoned it, 1 September
//! 5509 BC, and the year begins on 1 September rather than 1 January. It was
//! the civil calendar of the Eastern Roman Empire from the seventh century,
//! of the Orthodox church afterwards, and of Russia until Peter I replaced
//! it on 1 January 1700 — the reason a Russian document can be dated 7208.
//!
//! The months keep their Julian identity, so 1 January of *Anno Mundi* 7000
//! is 1 January 1492 in the Julian calendar and lies four months into that
//! Byzantine year. That is why the conversion from the Julian year depends
//! on the month: September to December take 5509, January to August take
//! 5508.
//!
//! The indiction, the fifteen-year tax cycle that Byzantine documents cite
//! more reliably than the year, is carried as an extra field.
//!
//! # What this deliberately does not do
//!
//! The Alexandrian and Antiochene world eras put creation in different
//! years, and several Byzantine sources start the year on 1 March or at
//! Easter. Only the 1 September / 5509 BC combination — the "Byzantine era"
//! proper — is implemented.
//!
//! Sources: Wikipedia, "Byzantine calendar", retrieved 2026-09-26
//! (`wikipedia-byzantine-calendar`): the era from 1 September 5509 BC,
//! fixed there since at least the mid-seventh century and found in the
//! Acts of the Quinisext Council of 691; its replacement in the Orthodox
//! church by the Christian era, formally in 1728; its use in Russia until
//! Peter I changed the calendar in 1700. It lists V. Grumel, *La
//! chronologie* (Paris, 1958; `grumel1958`), the standard treatment, which
//! was not read, and neither was Peter I's decree of 20 December 7208.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::julian;

/// How far *Anno Mundi* runs ahead of the Julian year for months September
/// to December.
pub const AUTUMN_OFFSET: i64 = 5_509;

/// How far it runs ahead for January to August.
pub const WINTER_OFFSET: i64 = 5_508;

/// The month the Byzantine year begins in.
pub const NEW_YEAR_MONTH: u8 = 9;

/// The era code of the world era.
pub const ERA: &str = "am";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 99_999;

/// The astronomical Julian year that a Byzantine year and month fall in.
const fn julian_year(year: i64, month: u8) -> i64 {
    if month >= NEW_YEAR_MONTH {
        year - AUTUMN_OFFSET
    } else {
        year - WINTER_OFFSET
    }
}

/// Whether `year` contains a 29 February.
///
/// The February of a Byzantine year belongs to the Julian year `year -
/// 5508`, so the leap test is on that.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    julian::is_leap_year(year - WINTER_OFFSET)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    julian::days_in_month(julian_year(year, month), month)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The indiction of `year`, the position in the fifteen-year cycle,
/// counting from 1.
#[must_use]
pub const fn indiction(year: i64) -> u8 {
    ((year - 1).rem_euclid(15) + 1) as u8
}

/// The fixed day of 1 September of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    to_fixed(year, NEW_YEAR_MONTH, 1)
}

/// The last day of the world era in Russian civil use, 31 December 7208,
/// which is 31 December 1699 Julian: Peter I's decree made the next day
/// 1 January 1700.
pub const LAST_CIVIL: Rd = match julian::to_fixed(1699, 12, 31) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Wikipedia, \"Byzantine calendar\" [wikipedia-byzantine-calendar]: the era fixed at \
    1 September 5509 BC since at least the mid-seventh century, in the Acts of the Quinisext \
    Council of 691, and kept in Russia until Peter I replaced it on 1 January 1700; the \
    beginning is not dated closer by any source read";

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = match new_year(MIN_YEAR) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The latest fixed day this implementation converts, 31 August of the last
/// supported year.
pub const LATEST: Rd = match to_fixed(MAX_YEAR, 8, 31) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of a Byzantine date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    if month == 0 || month > 12 {
        return Err(CalendarError::MonthOutOfRange);
    }
    julian::to_fixed(julian_year(year, month), month, day)
}

/// The Byzantine year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    match julian::from_fixed(rd) {
        Err(error) => Err(error),
        Ok((julian_year, month, day)) => {
            let offset = if month >= NEW_YEAR_MONTH {
                AUTUMN_OFFSET
            } else {
                WINTER_OFFSET
            };
            Ok((julian_year + offset, month, day))
        }
    }
}

/// A Byzantine date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByzantineDate {
    /// The year of the world, counting from 1.
    pub year: i64,
    /// The month, 1 for January through 12 for December. The year begins in
    /// month 9, so months 1 to 8 fall in the *second* half of the year.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl ByzantineDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        match to_fixed(year, month, day) {
            Err(error) => Err(error),
            Ok(_) => Ok(Self { year, month, day }),
        }
    }

    /// The astronomical Julian year this date falls in.
    #[must_use]
    pub const fn julian_year(self) -> i64 {
        julian_year(self.year, self.month)
    }

    /// The indiction of this date's year.
    #[must_use]
    pub const fn indiction(self) -> u8 {
        indiction(self.year)
    }
}

/// The Byzantine calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ByzantineCalendar;

impl Calendar for ByzantineCalendar {
    type Date = ByzantineDate;

    /// In use until 31 December 1699 Julian, the day before Peter I's reform,
    /// from a beginning the sources put in the seventh century and no closer.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::until(LAST_CIVIL, USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("byzantine"),
            english_name: "Byzantine (Anno Mundi)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["el", "cu"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(ByzantineDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_era(ERA)
            .with_extra("indiction", i64::from(date.indiction()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        ByzantineDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;

    #[test]
    fn the_world_begins_on_the_first_of_september_5509_bc() {
        // Astronomical year -5508 is 5509 BC.
        assert_eq!(new_year(1), julian::to_fixed(-5_508, 9, 1));
        assert_eq!(from_fixed(EARLIEST), Ok((1, 9, 1)));
    }

    #[test]
    fn the_year_of_the_world_seven_thousand_is_1491_and_1492() {
        // AM 7000 ran from 1 September 1491 to 31 August 1492 Julian — the
        // year Muscovy expected the world to end, and did not print an
        // Easter table beyond.
        assert_eq!(to_fixed(7_000, 9, 1), julian::to_fixed(1_491, 9, 1));
        assert_eq!(to_fixed(7_000, 1, 1), julian::to_fixed(1_492, 1, 1));
        assert_eq!(to_fixed(7_000, 8, 31), julian::to_fixed(1_492, 8, 31));
        assert_eq!(
            to_fixed(7_001, 9, 1).unwrap().0,
            to_fixed(7_000, 8, 31).unwrap().0 + 1
        );
    }

    #[test]
    fn peter_the_great_ended_the_era_in_the_year_7208() {
        // Russia kept the September year until Peter I decreed that
        // 1 January 7208 A.M. would be 1 January 1700, cutting that year to
        // four months.
        let decree = to_fixed(7_208, 1, 1).unwrap();
        assert_eq!(julian::from_fixed(decree), Ok((1_700, 1, 1)));
        assert_eq!(gregorian::from_fixed(decree), Ok((1_700, 1, 11)));
    }

    #[test]
    fn the_year_number_changes_in_september() {
        let august = from_fixed(julian::to_fixed(2_025, 8, 31).unwrap()).unwrap();
        let september = from_fixed(julian::to_fixed(2_025, 9, 1).unwrap()).unwrap();
        assert_eq!(august.0, 7_533);
        assert_eq!(september.0, 7_534);
    }

    #[test]
    fn the_indiction_cycles_every_fifteen_years() {
        assert_eq!(indiction(1), 1);
        assert_eq!(indiction(15), 15);
        assert_eq!(indiction(16), 1);
        assert_eq!(indiction(7_000), indiction(7_015));
        for year in 1..200i64 {
            assert!((1..=15).contains(&indiction(year)));
        }
    }

    #[test]
    fn leap_days_follow_the_julian_rule_on_the_february_that_falls_inside() {
        // AM 7208 contains February 1700 Julian, which is a leap February
        // even though 1700 is not a Gregorian leap year.
        assert!(is_leap_year(7_208));
        assert_eq!(days_in_month(7_208, 2), Some(29));
        assert_eq!(days_in_year(7_208), 366);
        assert!(to_fixed(7_208, 2, 29).is_ok());
        assert_eq!(days_in_month(7_209, 2), Some(28));
        assert_eq!(to_fixed(7_209, 2, 29), Err(CalendarError::DayOutOfRange));
    }

    #[test]
    fn every_year_runs_from_september_to_august() {
        for year in 6_000..6_500i64 {
            let start = new_year(year).unwrap();
            let end = to_fixed(year, 8, 31).unwrap();
            assert_eq!(end.0 - start.0 + 1, i64::from(days_in_year(year)));
            assert_eq!(new_year(year + 1).unwrap().0, end.0 + 1);
        }
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=1_000_000).step_by(107) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = new_year(7_200).unwrap().0;
        let end = new_year(7_220).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = ByzantineCalendar;
        for rd in (EARLIEST.0..=900_000).step_by(1_231) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(
                fields.extra.get("indiction"),
                Some(i64::from(date.indiction()))
            );
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("byzantine"));
    }

    #[test]
    fn dates_outside_the_supported_range_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(0, 9, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(7_000, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(
            ByzantineDate::new(7_000, 1, 1).unwrap().julian_year(),
            1_492
        );
    }
}
