//! The tabular Hijri calendar on the astronomical (Thursday) epoch — CLDR
//! `islamic-tbla`.
//!
//! Identical arithmetic to [`crate::islamic_civil`], one day earlier. The two
//! epochs come from the same tradition read two ways: the Hijra is placed on
//! 16 July 622 Julian if the day is counted from sunrise as the chancelleries
//! did, and on 15 July 622 if it is counted from the preceding sunset as
//! astronomers did. CLDR's `tbla` stands for "tabular, leap year,
//! astronomical epoch".
//!
//! Every remark in [`crate::islamic_civil`] about what an arithmetic Hijri
//! calendar is not applies here unchanged.

use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd};

use crate::tabular::{
    ASTRONOMICAL_EPOCH, IslamicDate, LeapYearRule, TabularIslamicCalendar, earliest, latest,
};

/// The machine identifier CLDR uses for this calendar.
pub const ID: CalendarId = CalendarId("islamic-tbla");

/// The fixed day of 1 Muḥarram 1 AH in this variant: Thursday 15 July 622
/// Julian, 18 July 622 proleptic Gregorian.
pub const EPOCH: Rd = ASTRONOMICAL_EPOCH;

/// The parameterised calendar this one delegates to.
pub const PARAMETERS: TabularIslamicCalendar = TabularIslamicCalendar::new(
    ID,
    "Hijri (tabular, astronomical epoch)",
    EPOCH,
    LeapYearRule::Civil,
);

/// The earliest fixed day this calendar converts.
pub const EARLIEST: Rd = earliest(EPOCH, LeapYearRule::Civil);

/// The latest fixed day this calendar converts.
pub const LATEST: Rd = latest(EPOCH, LeapYearRule::Civil);

/// The tabular Hijri calendar on the astronomical epoch.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IslamicAstronomicalCalendar;

impl Calendar for IslamicAstronomicalCalendar {
    type Date = IslamicDate;

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    /// The Islamic day begins at sunset, which is also why the month begins
    /// with a crescent seen after one.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset
    }

    fn meta(&self) -> CalendarMeta {
        PARAMETERS.meta()
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        PARAMETERS.to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        PARAMETERS.from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        PARAMETERS.to_fields(date)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        PARAMETERS.from_fields(fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::civil;
    use crate::islamic_civil::IslamicCivilCalendar;
    use hc_calendar::Weekday;

    #[test]
    fn the_epoch_is_thursday_the_fifteenth_of_july_622_julian() {
        assert_eq!(Weekday::from_rd(EPOCH), Weekday::Thursday);
        assert_eq!(civil::from_rd(EPOCH), (622, 7, 18));
    }

    #[test]
    fn the_metadata_names_the_cldr_identifier() {
        assert_eq!(
            IslamicAstronomicalCalendar.meta().id,
            CalendarId("islamic-tbla")
        );
    }

    #[test]
    fn the_calendar_round_trips_over_a_hundred_thousand_days() {
        let calendar = IslamicAstronomicalCalendar;
        for offset in 0..100_000i64 {
            let rd = Rd(EARLIEST.0 + offset);
            let date = calendar.from_fixed(rd).expect("inside the range");
            assert_eq!(calendar.to_fixed(date), Ok(rd));
        }
    }

    #[test]
    fn the_same_date_is_one_day_earlier_than_in_the_civil_variant() {
        let astronomical = IslamicAstronomicalCalendar;
        let civil_calendar = IslamicCivilCalendar;
        for year in (1..=5_000i64).step_by(37) {
            let date = IslamicDate {
                year,
                month: 7,
                day: 12,
            };
            let a = astronomical.to_fixed(date).expect("valid");
            let c = civil_calendar.to_fixed(date).expect("valid");
            assert_eq!(c.0 - a.0, 1, "year {year}");
        }
    }
}
