//! Julian Day Numbers and Modified Julian Dates, as calendars.
//!
//! A Julian Day Number is a plain count of days from noon UT on 1 January
//! 4713 BC in the proleptic Julian calendar — the epoch Joseph Scaliger chose
//! in 1583 because three independent cycles coincide there. It is not a
//! calendar in the sense of having months, and treating it as one is a little
//! perverse, but it is the interchange format of astronomy, and expressing it
//! through the same [`Calendar`] trait as everything else means a caller can
//! convert a Hebrew date to a JDN without either side knowing about the
//! other.
//!
//! Two conventions ship here:
//!
//! * [`JulianDayCalendar`] counts JDN, so JDN 2 440 588 is 1970-01-01.
//! * [`ModifiedJulianDayCalendar`] counts MJD = JDN − 2 400 000.5, the
//!   convention of the IERS bulletins and of most spacecraft telemetry, so
//!   MJD 40 587 is 1970-01-01.
//!
//! # Days, not instants
//!
//! A Julian Date proper has a fraction: JD 2440587.5 is midnight and
//! JD 2440588.0 is noon, because Scaliger's day starts at noon. A *day
//! number* has no fraction, and the conversion here maps a whole JDN to the
//! whole calendar day that *begins* at the preceding midnight — the
//! convention of `Rd::to_julian_day_number`. For fractional work use
//! [`hc_calendar::fixed::Moment::to_julian_date`], which keeps the half-day
//! offset. MJD needs no such care: it was defined with a half-day shift
//! precisely so that it rolls over at midnight.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

/// The Julian Day Number of the Rata Die epoch, `0001-01-01` Gregorian.
pub const JDN_OF_RD_ONE: i64 = 1_721_426;

/// The difference between a Julian Day Number and a Modified Julian Date.
///
/// MJD is defined as JD − 2 400 000.5; in whole days from midnight that is
/// JDN − 2 400 001.
pub const JDN_MINUS_MJD: i64 = 2_400_001;

/// A Julian Day Number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct JulianDayNumber(pub i64);

/// A Modified Julian Date, as a whole number of days.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ModifiedJulianDay(pub i64);

impl JulianDayNumber {
    /// The equivalent Modified Julian Date.
    #[must_use]
    pub const fn to_modified(self) -> ModifiedJulianDay {
        ModifiedJulianDay(self.0 - JDN_MINUS_MJD)
    }
}

impl ModifiedJulianDay {
    /// The equivalent Julian Day Number.
    #[must_use]
    pub const fn to_julian_day_number(self) -> JulianDayNumber {
        JulianDayNumber(self.0 + JDN_MINUS_MJD)
    }
}

/// How far a day count may stray from the Rata Die epoch before the
/// conversion is refused.
///
/// The bound is generous — about ±5.9 million years — and exists only so
/// that adding the epoch offset cannot overflow `i64`.
const MAX_MAGNITUDE: i64 = 1 << 44;

/// Metadata shared by both day-count calendars.
const fn day_count_meta(id: &'static str, english_name: &'static str) -> CalendarMeta {
    CalendarMeta {
        id: CalendarId(id),
        english_name,
        year_kind: YearKind::Astronomical,
        has_leap_months: false,
        is_astronomical: false,
        earliest: Some(Rd(-MAX_MAGNITUDE)),
        latest: Some(Rd(MAX_MAGNITUDE)),
    }
}

/// The count of days since the Julian Period epoch.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JulianDayCalendar;

impl Calendar for JulianDayCalendar {
    type Date = JulianDayNumber;

    /// A day count names nothing: it has no months and no week, only a number.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        &[]
    }

    /// The Julian Day begins at noon, not midnight. Astronomers count that
    /// way so that one night's observations carry a single date, and the
    /// convention has outlived the reason.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Noon
    }

    fn meta(&self) -> CalendarMeta {
        day_count_meta("julian-day", "Julian Day Number")
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        let rd = Rd::from_julian_day_number(date.0);
        self.meta().check_range(rd)?;
        Ok(rd)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.meta().check_range(rd)?;
        Ok(JulianDayNumber(rd.to_julian_day_number()))
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        // The day count goes in `year` because it is the only unbounded
        // signed field a generic consumer is guaranteed to have; the extra
        // field carries the same day in the other convention.
        DateFields::new(date.0).with_extra("modified-julian-day", date.to_modified().0)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        reject_calendar_fields(fields)?;
        Ok(JulianDayNumber(fields.year))
    }
}

/// The count of days since midnight on 17 November 1858.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ModifiedJulianDayCalendar;

impl Calendar for ModifiedJulianDayCalendar {
    type Date = ModifiedJulianDay;

    /// A day count names nothing: it has no months and no week, only a number.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        &[]
    }

    /// The Modified Julian Date begins at midnight, unlike the Julian Day
    /// it is derived from — the 0.5 in its definition is exactly that
    /// shift. It is recorded here because the neighbouring calendar
    /// differs, and because half the confusion between the two counts is
    /// about which half of the day they mean.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Midnight
    }

    fn meta(&self) -> CalendarMeta {
        day_count_meta("modified-julian-day", "Modified Julian Date")
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        let rd = Rd::from_modified_julian_day(date.0);
        self.meta().check_range(rd)?;
        Ok(rd)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.meta().check_range(rd)?;
        Ok(ModifiedJulianDay(rd.to_modified_julian_day()))
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::new(date.0).with_extra("julian-day-number", date.to_julian_day_number().0)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        reject_calendar_fields(fields)?;
        Ok(ModifiedJulianDay(fields.year))
    }
}

/// A day count has no months and no days-within-months; accepting them
/// silently would let a caller believe a conversion happened that did not.
fn reject_calendar_fields(fields: &DateFields) -> CalendarResult<()> {
    if fields.day.is_some() {
        return Err(CalendarError::UnsupportedField("day"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;

    #[test]
    fn the_posix_epoch_has_the_published_day_numbers() {
        // 1970-01-01 is JDN 2440588 and MJD 40587.
        let rd = gregorian::to_fixed(1970, 1, 1).unwrap();
        assert_eq!(
            JulianDayCalendar.from_fixed(rd),
            Ok(JulianDayNumber(2_440_588))
        );
        assert_eq!(
            ModifiedJulianDayCalendar.from_fixed(rd),
            Ok(ModifiedJulianDay(40_587))
        );
    }

    #[test]
    fn the_j2000_epoch_has_the_published_day_numbers() {
        // JD 2451545.0 is 2000-01-01 at 12:00 TT, so the day 2000-01-01 is
        // JDN 2451545 and MJD 51544.
        let rd = gregorian::to_fixed(2000, 1, 1).unwrap();
        assert_eq!(
            JulianDayCalendar.from_fixed(rd),
            Ok(JulianDayNumber(2_451_545))
        );
        assert_eq!(
            ModifiedJulianDayCalendar.from_fixed(rd),
            Ok(ModifiedJulianDay(51_544))
        );
    }

    #[test]
    fn the_modified_julian_epoch_is_the_seventeenth_of_november_1858() {
        // MJD 0 is 1858-11-17, the definition adopted by the IAU in 1973.
        let rd = ModifiedJulianDayCalendar
            .to_fixed(ModifiedJulianDay(0))
            .unwrap();
        assert_eq!(gregorian::from_fixed(rd), Ok((1858, 11, 17)));
    }

    #[test]
    fn the_julian_period_epoch_is_4713_bc_in_the_julian_calendar() {
        use crate::julian;

        // JDN 0 is 1 January 4713 BC in the proleptic Julian calendar, which
        // is astronomical year -4712.
        let rd = JulianDayCalendar.to_fixed(JulianDayNumber(0)).unwrap();
        assert_eq!(julian::from_fixed(rd), Ok((-4712, 1, 1)));
    }

    #[test]
    fn the_two_conventions_differ_by_a_fixed_offset() {
        for jdn in (0..5_000_000).step_by(9_973) {
            let number = JulianDayNumber(jdn);
            assert_eq!(number.to_modified().to_julian_day_number(), number);
            assert_eq!(number.to_modified().0, jdn - 2_400_001);
        }
    }

    #[test]
    fn rata_die_day_one_has_the_published_julian_day_number() {
        assert_eq!(
            JulianDayCalendar.from_fixed(Rd(1)),
            Ok(JulianDayNumber(JDN_OF_RD_ONE))
        );
        assert_eq!(JDN_OF_RD_ONE, 1_721_426);
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        let julian_day = JulianDayCalendar;
        let modified = ModifiedJulianDayCalendar;
        for rd in (-1_000_000..=2_000_000).step_by(17) {
            let date = julian_day.from_fixed(Rd(rd)).unwrap();
            assert_eq!(julian_day.to_fixed(date), Ok(Rd(rd)));
            let mjd = modified.from_fixed(Rd(rd)).unwrap();
            assert_eq!(modified.to_fixed(mjd), Ok(Rd(rd)));
            assert_eq!(date.to_modified(), mjd);
        }
    }

    #[test]
    fn day_counts_round_trip_through_fields() {
        let julian_day = JulianDayCalendar;
        let modified = ModifiedJulianDayCalendar;
        for rd in (-100_000..=900_000).step_by(2_003) {
            let date = julian_day.from_fixed(Rd(rd)).unwrap();
            let fields = julian_day.to_fields(date).unwrap();
            assert_eq!(fields.year, date.0);
            assert_eq!(
                fields.extra.get("modified-julian-day"),
                Some(date.to_modified().0)
            );
            assert_eq!(julian_day.from_fields(&fields), Ok(date));

            let mjd = modified.from_fixed(Rd(rd)).unwrap();
            let fields = modified.to_fields(mjd).unwrap();
            assert_eq!(modified.from_fields(&fields), Ok(mjd));
        }
    }

    #[test]
    fn a_day_count_has_no_day_of_the_month() {
        assert_eq!(
            JulianDayCalendar.from_fields(&DateFields::ymd(2_440_588, 1, 1)),
            Err(CalendarError::UnsupportedField("day"))
        );
        assert_eq!(
            ModifiedJulianDayCalendar.from_fields(&DateFields::ymd(40_587, 1, 1)),
            Err(CalendarError::UnsupportedField("day"))
        );
        assert_eq!(
            JulianDayCalendar.from_fields(&DateFields::new(2_440_588)),
            Ok(JulianDayNumber(2_440_588))
        );
    }

    #[test]
    fn counts_far_outside_the_supported_range_are_rejected() {
        assert_eq!(
            JulianDayCalendar.to_fixed(JulianDayNumber(i64::MAX / 2)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            JulianDayCalendar.to_fixed(JulianDayNumber(i64::MIN / 2)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            ModifiedJulianDayCalendar.from_fixed(Rd(MAX_MAGNITUDE + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }
}
