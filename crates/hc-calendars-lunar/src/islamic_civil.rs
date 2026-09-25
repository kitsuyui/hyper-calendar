//! The tabular Hijri calendar on the civil (Friday) epoch — CLDR
//! `islamic-civil`.
//!
//! The Hijri family, this scheme's place in it, a month worked by hand and
//! the closed-form check are in `docs/systems/hijri.md`. This is the
//! arithmetic Hijri calendar in its most common form: the intercalation
//! scheme of al-Fazārī, al-Khwārizmī and al-Battānī
//! ([`LeapYearRule::CIVIL`]), counted from Friday 16 July 622 in the Julian
//! calendar. It is the calendar behind `islamic-civil` in CLDR, behind
//! ICU's civil calculation type, and behind most published conversion
//! tables.
//!
//! # What it is for, and what it is not
//!
//! It is an *arithmetic* calendar: every date it produces is computable
//! centuries in advance, which is exactly why administrations use it and
//! exactly why it is not the calendar of religious practice. Months in
//! practice begin on sighting or on a national table; see
//! [`crate::islamic_umalqura`] for Saudi Arabia's official table and
//! [`crate::islamic_observational`] for a sighting *prediction*. Against
//! the Umm al-Qura table the month starts differ for 39% of months, never
//! by more than three days; the figure is measured in that module's tests.

use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd};

use crate::tabular::{
    CIVIL_EPOCH, IslamicDate, LeapYearRule, TabularIslamicCalendar, earliest, latest,
};

/// The machine identifier CLDR uses for this calendar.
pub const ID: CalendarId = CalendarId("islamic-civil");

/// The fixed day of 1 Muḥarram 1 AH in this variant: Friday 16 July 622
/// Julian, 19 July 622 proleptic Gregorian.
pub const EPOCH: Rd = CIVIL_EPOCH;

/// The parameterised calendar this one delegates to.
pub const PARAMETERS: TabularIslamicCalendar = TabularIslamicCalendar::new(
    ID,
    "Hijri (tabular, civil epoch)",
    EPOCH,
    LeapYearRule::CIVIL,
);

/// The earliest fixed day this calendar converts.
pub const EARLIEST: Rd = earliest(EPOCH, LeapYearRule::CIVIL);

/// The latest fixed day this calendar converts.
pub const LATEST: Rd = latest(EPOCH, LeapYearRule::CIVIL);

/// The tabular Hijri calendar on the civil epoch.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IslamicCivilCalendar;

impl Calendar for IslamicCivilCalendar {
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
    use hc_calendar::{CalendarError, Weekday};

    #[test]
    fn the_epoch_is_friday_the_sixteenth_of_july_622_julian() {
        assert_eq!(Weekday::from_rd(EPOCH), Weekday::Friday);
        assert_eq!(civil::from_rd(EPOCH), (622, 7, 19));
        let calendar = IslamicCivilCalendar;
        assert_eq!(
            calendar.to_fixed(IslamicDate {
                year: 1,
                month: 1,
                day: 1
            }),
            Ok(EPOCH)
        );
    }

    #[test]
    fn the_metadata_names_the_cldr_identifier() {
        let meta = IslamicCivilCalendar.meta();
        assert_eq!(meta.id, CalendarId("islamic-civil"));
        assert!(!meta.has_leap_months);
        assert!(!meta.is_astronomical);
        assert_eq!(meta.earliest, Some(EARLIEST));
        assert_eq!(meta.latest, Some(LATEST));
    }

    #[test]
    fn the_calendar_round_trips_over_a_hundred_thousand_days() {
        let calendar = IslamicCivilCalendar;
        for offset in 0..100_000i64 {
            let rd = Rd(EARLIEST.0 + offset);
            let date = calendar.from_fixed(rd).expect("inside the range");
            assert_eq!(calendar.to_fixed(date), Ok(rd));
        }
    }

    #[test]
    fn fields_round_trip_through_the_generic_interface() {
        let calendar = IslamicCivilCalendar;
        for offset in (0..200_000i64).step_by(97) {
            let rd = Rd(EARLIEST.0 + offset);
            let date = calendar.from_fixed(rd).expect("inside the range");
            let fields = calendar.to_fields(date).expect("describable");
            assert_eq!(fields.era, Some("AH"));
            assert_eq!(calendar.from_fields(&fields), Ok(date));
        }
    }

    #[test]
    fn every_year_has_twelve_months_of_twenty_nine_or_thirty_days() {
        let calendar = IslamicCivilCalendar;
        for year in 1_300..1_500i64 {
            let mut total = 0i64;
            for month in 1..=12u8 {
                let first = calendar
                    .to_fixed(IslamicDate {
                        year,
                        month,
                        day: 1,
                    })
                    .expect("valid");
                let next = if month == 12 {
                    calendar
                        .to_fixed(IslamicDate {
                            year: year + 1,
                            month: 1,
                            day: 1,
                        })
                        .expect("valid")
                } else {
                    calendar
                        .to_fixed(IslamicDate {
                            year,
                            month: month + 1,
                            day: 1,
                        })
                        .expect("valid")
                };
                let length = next.0 - first.0;
                assert!((29..=30).contains(&length), "{year}-{month} was {length}");
                total += length;
            }
            assert!(total == 354 || total == 355, "year {year} was {total}");
        }
    }

    #[test]
    fn dates_outside_the_range_are_refused() {
        let calendar = IslamicCivilCalendar;
        assert_eq!(
            calendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            calendar.from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }
}
