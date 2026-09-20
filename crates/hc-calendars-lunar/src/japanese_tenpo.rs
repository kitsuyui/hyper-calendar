//! The Tenpō calendar (天保暦), Japan's last lunisolar calendar.
//!
//! In force from the first month of Tenpō 15 (1844) until it was abolished by
//! decree in 1872. It was the first Japanese calendar to use *teiki* — the
//! true solar terms — which is the rule [`crate::lunisolar`] implements, and
//! that is why the Tenpō calendar gets a module here and its predecessors do
//! not. CLDR has no identifier for it; the crate uses `japanese-tenpo`, which
//! is deliberately not `japanese` (that name belongs to the Gregorian
//! calendar with nengō years, in `hc-calendars-regional`).
//!
//! # Why the range ends where it does
//!
//! The Dajōkan decree of 9 November 1872 declared that **Meiji 5, twelfth
//! month, third day would be 1 January 1873 in the solar calendar**. So the
//! last day the Tenpō calendar ever named was Meiji 5, twelfth month, second
//! day — Gregorian **1872-12-31** — and [`LATEST`] is that day. The next day
//! has no Tenpō date, and this module returns
//! [`CalendarError::AfterSupportedRange`](hc_calendar::CalendarError::AfterSupportedRange)
//! for it rather than inventing one.
//!
//! The abolition was abrupt for a reason the calendar itself explains: the
//! year that would have been Meiji 6 was due a leap sixth month, so the
//! government faced thirteen months of salary. This module computes that leap
//! month, and the crate tests it — see
//! [`the_year_after_the_abolition_would_have_had_thirteen_months`](self).
//!
//! # The meridian
//!
//! | From | Offset | |
//! |---|---|---|
//! | — | UT+9:03:04 | Kyoto local mean time, 135°46′E |
//! | 1888 | UT+9 | Japan Standard Time, 135°E |
//!
//! The Japanese lunisolar calendars were computed for Kyoto, and Japan
//! Standard Time was not established until 1888, sixteen years after the
//! Tenpō calendar ended — so in practice only the first row is ever used
//! here. The second is kept so that the table states the whole history and
//! so that a caller extending the range past 1888 gets the right offset.
//! Three minutes is small but not nothing: a conjunction in that window
//! lands on different days under the two.
//!
//! # Year numbering
//!
//! Tenpō years were named by nengō — Kōka 2, Ansei 3, Meiji 5 — and the
//! nengō table is not this crate's business. Years here are numbered by the
//! Gregorian year in which they begin, so the year that began on 1872-02-09
//! is 1872 and its twelfth month runs into the Gregorian year after. This is
//! this crate's convention, stated so that nobody mistakes it for a
//! historical one.

use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd};

use crate::civil;
use crate::lunisolar::{
    CHINESE_EPOCH, LunisolarCalendar, LunisolarDate, LunisolarParameters, MeridianEra,
    SolarTermMode,
};

/// The machine identifier this crate uses for the calendar. CLDR has none.
pub const ID: CalendarId = CalendarId("japanese-tenpo");

/// How far the year number falls below the continuous Chinese count.
pub const YEAR_OFFSET: i64 = -2_637;

/// The earliest fixed day this calendar converts: the first day of the first
/// month of Tenpō 15, when the Tenpō calendar took effect.
///
/// This module computes that day as 1844-02-18, and the crate tests it.
pub const EARLIEST: Rd = civil::to_rd(1844, 2, 18);

/// The latest fixed day this calendar converts: Meiji 5, twelfth month,
/// second day, which is Gregorian 1872-12-31.
///
/// The next day was decreed to be 1 January 1873 in the solar calendar, so
/// there is no Tenpō date after this one.
pub const LATEST: Rd = civil::to_rd(1872, 12, 31);

/// The meridian history of the Japanese calendar.
pub static MERIDIANS: [MeridianEra; 2] = [
    MeridianEra::from_longitude(
        i64::MIN / 4,
        135.0 + 46.0 / 60.0,
        "Kyoto local mean time, 135°46′E",
    ),
    MeridianEra::from_zone(1888, 9.0, "Japan Standard Time, 135°E"),
];

/// The parameters of the Tenpō calendar.
pub static PARAMETERS: LunisolarParameters = LunisolarParameters {
    id: ID,
    english_name: "Japanese Tenpō (lunisolar)",
    meridians: &MERIDIANS,
    epoch: CHINESE_EPOCH,
    year_offset: YEAR_OFFSET,
    solar_term_mode: SolarTermMode::Apparent,
    earliest: Some(EARLIEST),
    latest: Some(LATEST),
};

/// The engine configured as the Tenpō calendar.
pub const ENGINE: LunisolarCalendar = LunisolarCalendar::new(&PARAMETERS);

/// A Tenpō date. See [`crate::chinese::ChineseDate`] for why the
/// representation is shared.
pub type JapaneseTenpoDate = LunisolarDate;

/// The Tenpō calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JapaneseTenpoCalendar;

impl Calendar for JapaneseTenpoCalendar {
    type Date = JapaneseTenpoDate;

    fn meta(&self) -> CalendarMeta {
        ENGINE.meta()
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        ENGINE.to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        ENGINE.from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        ENGINE.to_fields(date)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        ENGINE.from_fields(fields)
    }
}

/// The fixed day of the Japanese lunisolar new year of `year`.
///
/// # Errors
///
/// Returns [`hc_calendar::CalendarError::YearOutOfRange`] outside the
/// supported range.
pub fn new_year(year: i64) -> CalendarResult<Rd> {
    PARAMETERS.new_year(year)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lunisolar::SolarTermMode;
    use hc_calendar::{CalendarError, Month};

    /// The same parameters without the range bound, used only to look past
    /// the abolition at what the calendar *would* have said.
    static UNBOUNDED: LunisolarParameters = LunisolarParameters {
        id: CalendarId("japanese-tenpo-unbounded"),
        english_name: "Japanese Tenpō, unbounded",
        meridians: &MERIDIANS,
        epoch: CHINESE_EPOCH,
        year_offset: YEAR_OFFSET,
        solar_term_mode: SolarTermMode::Apparent,
        earliest: None,
        latest: None,
    };

    #[test]
    fn the_calendar_took_effect_on_the_eighteenth_of_february_1844() {
        assert_eq!(new_year(1_844), Ok(EARLIEST));
        assert_eq!(civil::from_rd(EARLIEST), (1844, 2, 18));
        assert_eq!(
            JapaneseTenpoCalendar.from_fixed(EARLIEST),
            Ok(LunisolarDate::new(1_844, Month::regular(1), 1))
        );
    }

    #[test]
    fn the_last_day_is_the_second_of_the_twelfth_month_of_meiji_five() {
        // The decree: Meiji 5, twelfth month, third day became 1873-01-01.
        assert_eq!(civil::from_rd(LATEST), (1872, 12, 31));
        assert_eq!(
            JapaneseTenpoCalendar.from_fixed(LATEST),
            Ok(LunisolarDate::new(1_872, Month::regular(12), 2))
        );
        assert_eq!(
            JapaneseTenpoCalendar.from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        // The day that would have been the third of the twelfth month is
        // 1873-01-01, and the calendar refuses to name it.
        assert_eq!(civil::from_rd(Rd(LATEST.0 + 1)), (1873, 1, 1));
        assert_eq!(
            JapaneseTenpoCalendar.to_fixed(LunisolarDate::new(1_872, Month::regular(12), 3)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn the_twelfth_month_of_meiji_five_began_on_the_thirtieth_of_december() {
        assert_eq!(
            JapaneseTenpoCalendar.to_fixed(LunisolarDate::new(1_872, Month::regular(12), 1)),
            Ok(civil::to_rd(1872, 12, 30))
        );
        assert_eq!(new_year(1_872), Ok(civil::to_rd(1872, 2, 9)));
    }

    #[test]
    fn the_year_after_the_abolition_would_have_had_thirteen_months() {
        // The reason usually given for the abruptness of the reform: Meiji 6
        // was due a leap sixth month, and the new government would have owed
        // its officials thirteen months of salary.
        assert_eq!(UNBOUNDED.months_in_year(1_873), Ok(13));
        assert_eq!(UNBOUNDED.leap_month(1_873), Ok(Some(6)));
        assert_eq!(UNBOUNDED.months_in_year(1_872), Ok(12));
        assert_eq!(UNBOUNDED.leap_month(1_872), Ok(None));
    }

    #[test]
    fn the_calendar_round_trips_over_every_day_it_covers() {
        let calendar = JapaneseTenpoCalendar;
        for rd in EARLIEST.0..=LATEST.0 {
            let date = calendar.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "RD {rd}");
        }
    }

    #[test]
    fn every_month_it_covers_is_twenty_nine_or_thirty_days() {
        let calendar = JapaneseTenpoCalendar;
        let mut cursor = EARLIEST;
        let mut months = 0;
        while cursor.0 + 30 <= LATEST.0 {
            let next = PARAMETERS.new_moon_on_or_after(Rd(cursor.0 + 1));
            let length = next.0 - cursor.0;
            assert!(
                (29..=30).contains(&length),
                "month at {cursor} was {length}"
            );
            assert_eq!(calendar.from_fixed(cursor).expect("in range").day, 1);
            months += 1;
            cursor = next;
        }
        // Twenty-eight years of twelve or thirteen months each.
        assert!((340..=360).contains(&months), "{months} months");
    }

    #[test]
    fn the_years_it_covers_hold_twelve_or_thirteen_months() {
        for year in 1_844..1_872i64 {
            let months = PARAMETERS.months_in_year(year).expect("in range");
            assert!(months == 12 || months == 13, "year {year} had {months}");
        }
    }

    #[test]
    fn dates_before_the_reform_are_refused() {
        assert_eq!(
            JapaneseTenpoCalendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(new_year(1_843), Err(CalendarError::YearOutOfRange));
    }

    #[test]
    fn the_kyoto_meridian_is_three_minutes_ahead_of_japan_standard_time() {
        let kyoto = PARAMETERS.meridian_era(civil::to_rd(1860, 1, 1));
        let standard = PARAMETERS.meridian_era(civil::to_rd(1900, 1, 1));
        assert!((standard.offset_hours - 9.0).abs() < 1e-12);
        let minutes = (kyoto.offset_hours - standard.offset_hours) * 60.0;
        assert!((minutes - 3.066_666).abs() < 1e-3, "{minutes} minutes");
    }

    #[test]
    fn the_metadata_bounds_the_calendar_at_the_abolition() {
        let meta = JapaneseTenpoCalendar.meta();
        assert_eq!(meta.id, CalendarId("japanese-tenpo"));
        assert_eq!(meta.earliest, Some(EARLIEST));
        assert_eq!(meta.latest, Some(LATEST));
        assert!(meta.has_leap_months);
        assert!(meta.is_astronomical);
        assert!(!meta.supports(Rd(LATEST.0 + 1)));
    }
}
