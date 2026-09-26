//! The ancient Egyptian wandering year.
//!
//! The simplest calendar in this crate and, for that reason, the one
//! astronomers loved: twelve months of thirty days, five epagomenal days at
//! the end, no intercalation ever, so every year is exactly 365 days and any
//! two dates are a fixed number of days apart without a single exception to
//! remember. Ptolemy used it in the *Almagest* for precisely that reason.
//!
//! The price is that it slips against the seasons by about a day every four
//! years and returns to where it started only after 1 460 Julian years — the
//! Sothic cycle. "Wandering year", *annus vagus*, is a description of the
//! calendar working correctly, not of a defect.
//!
//! The epoch used here is the era of Nabonassar, 26 February 747 BC in the
//! proleptic Julian calendar ([`EPOCH`]), which is the era Ptolemy's tables
//! count from and therefore the one that makes his observations directly
//! usable. The Egyptians themselves numbered years by the reigning king, and
//! this crate does not attempt to model regnal years; a caller who needs
//! them should map a regnal year to a Nabonassar year and convert.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::common;

/// The fixed day of 1 Thoth 1 of the era of Nabonassar, 26 February 747 BC
/// in the proleptic Julian calendar and Julian Day Number 1 448 638.
pub const EPOCH: Rd = Rd(-272_787);

/// The era code of the era of Nabonassar.
pub const ERA: &str = "Nabonassar";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 999_999;

/// The length of every Egyptian year, without exception.
pub const DAYS_IN_YEAR: u16 = 365;

/// The number of days in `month`, or `None` when `month` is not in `1..=13`.
///
/// The year number is irrelevant: this calendar has no leap rule.
#[must_use]
pub const fn days_in_month(month: u8) -> Option<u8> {
    common::wandering_days_in_month(month)
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(common::wandering_to_fixed(EPOCH.0, MIN_YEAR, 1, 1));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(common::wandering_to_fixed(EPOCH.0, MAX_YEAR + 1, 1, 1) - 1);

/// The fixed day of an Egyptian date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    match common::check_day(day, days_in_month(month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(common::wandering_to_fixed(EPOCH.0, year, month, day))),
    }
}

/// The Egyptian year, month and day of a fixed day.
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
    Ok(common::wandering_from_fixed(EPOCH.0, rd.0))
}

/// An ancient Egyptian date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EgyptianDate {
    /// The year of the era of Nabonassar, counting from 1.
    pub year: i64,
    /// The month, 1 through 13; month 13 holds the five epagomenal days.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl EgyptianDate {
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

    /// Whether this date is one of the five days "upon the year".
    #[must_use]
    pub const fn is_epagomenal(self) -> bool {
        self.month == 13
    }
}

/// The ancient Egyptian calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EgyptianCalendar;

/// The thirteen months under the Greek names Egyptology uses, Thoth first.
/// The thirteenth is the five epagomenal days, *ḥryw rnpt*, "those upon
/// the year", under their Greek name.
///
/// The Egyptian names themselves (Ḏḥwty, Pꜣ-n-Ipt …) are transliterations
/// of hieroglyphs and are not what any date is written with; the Greek
/// forms are the convention of the field. Source: the months table of
/// Wikipedia, "Egyptian calendar", retrieved 2026-09-22.
pub const MONTHS: [&str; 13] = [
    "Thoth",
    "Phaophi",
    "Athyr",
    "Choiak",
    "Tybi",
    "Mechir",
    "Phamenoth",
    "Pharmuthi",
    "Pachons",
    "Payni",
    "Epiphi",
    "Mesore",
    "Epagomenai",
];

/// Thirteen named months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for EgyptianCalendar {
    type Date = EgyptianDate;

    /// Unrecorded. The wandering year ran for three millennia and the era
    /// of Nabonassar it is counted in is Ptolemy's astronomical convention,
    /// applied to it centuries later and kept by astronomers into the
    /// Renaissance; no source read bounds either, so there is no period to
    /// state.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Thirteen months under their Greek names, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// Never: a wandering year of 365 days intercalates nothing.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(false)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("egyptian"),
            english_name: "Egyptian",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["egy"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(EgyptianDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(ERA))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        EgyptianDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::julian;

    #[test]
    fn the_epoch_is_the_era_of_nabonassar() {
        // 1 Thoth 1 is 26 February 747 BC in the proleptic Julian calendar,
        // astronomical year -746, and Julian Day Number 1448638.
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
        assert_eq!(julian::from_fixed(EPOCH), Ok((-746, 2, 26)));
        assert_eq!(EPOCH.to_julian_day_number(), 1_448_638);
    }

    #[test]
    fn every_year_is_exactly_three_hundred_and_sixty_five_days() {
        for year in (1..10_000).step_by(13) {
            let start = to_fixed(year, 1, 1).unwrap();
            let next = to_fixed(year + 1, 1, 1).unwrap();
            assert_eq!(next.0 - start.0, i64::from(DAYS_IN_YEAR), "year {year}");
        }
    }

    #[test]
    fn the_wandering_year_returns_after_a_sothic_cycle() {
        // 1 461 Egyptian years of 365 days are 1 460 Julian years of
        // 365.25 days — 533 265 days either way — so the new year comes back
        // to the same Julian date.
        let first = to_fixed(1, 1, 1).unwrap();
        let later = to_fixed(1_462, 1, 1).unwrap();
        let (first_year, first_month, first_day) = julian::from_fixed(first).unwrap();
        let (later_year, later_month, later_day) = julian::from_fixed(later).unwrap();
        assert_eq!((first_month, first_day), (later_month, later_day));
        assert_eq!(later_year - first_year, 1_460);
        assert_eq!(later.0 - first.0, 1_461 * 365);
        assert_eq!(later.0 - first.0, 533_265);
    }

    #[test]
    fn the_new_year_slips_one_day_every_four_years() {
        let start = to_fixed(1, 1, 1).unwrap();
        let (_, _, start_day) = julian::from_fixed(start).unwrap();
        let four_years_on = to_fixed(5, 1, 1).unwrap();
        let (_, _, later_day) = julian::from_fixed(four_years_on).unwrap();
        assert_eq!(i64::from(later_day), i64::from(start_day) - 1);
    }

    #[test]
    fn the_thirteenth_month_holds_exactly_five_days() {
        assert_eq!(days_in_month(13), Some(5));
        assert_eq!(days_in_month(12), Some(30));
        assert_eq!(days_in_month(14), None);
        assert_eq!(days_in_month(0), None);
        assert!(to_fixed(1, 13, 5).is_ok());
        assert_eq!(to_fixed(1, 13, 6), Err(CalendarError::DayOutOfRange));
        assert!(EgyptianDate::new(1, 13, 5).unwrap().is_epagomenal());
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=1_000_000).step_by(23) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_five_years_round_trips() {
        let start = to_fixed(1, 1, 1).unwrap().0;
        for rd in start..(start + 5 * 365) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = EgyptianCalendar;
        for rd in (EARLIEST.0..=900_000).step_by(521) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("egyptian"));
        assert_eq!(calendar.meta().year_kind, YearKind::EpochForward);
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
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            EgyptianCalendar.from_fields(&DateFields::ymd(1, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }
}
