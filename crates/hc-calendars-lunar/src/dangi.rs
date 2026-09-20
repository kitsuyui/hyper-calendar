//! The Korean lunisolar calendar — CLDR `dangi`.
//!
//! The same rules as [`crate::chinese`], read at a different meridian and
//! numbered from a different epoch. Korea used the Chinese calendar of the
//! day throughout, so the arithmetic is not merely similar: it is the same
//! arithmetic, which is why this module is thirty lines of constants and no
//! algorithm.
//!
//! # The meridian, and why it has five entries
//!
//! | From | Offset | |
//! |---|---|---|
//! | — | UT+8:27:52 | Seoul local mean time, 126°58′E |
//! | 1908 | UT+8:30 | the 127°30′E zone, adopted by the Korean Empire |
//! | 1912 | UT+9 | the 135°E zone, imposed under Japanese rule |
//! | 1954 | UT+8:30 | back to 127°30′E |
//! | 1961 | UT+9 | back to 135°E, where it remains |
//!
//! The table is the one in Reingold and Dershowitz, *Calendrical
//! Calculations*. A calendar that simply used 135°E from 1908 onward would
//! get the twentieth century wrong in places, because the two half-hour
//! periods really did move the day boundary.
//!
//! The meridian is the whole of the difference from the Chinese calendar, and
//! it is enough: over 1900–2049 the Korean and Chinese new years fall on
//! different days nine times, including 1988, when Seollal was 18 February in
//! Korea and Chinese New Year was 17 February in China.
//!
//! # Year numbering
//!
//! *Dangi* (단기) years count from the traditional foundation of Gojoseon in
//! 2333 BCE, so the year that began on 2024-02-10 is Dangi 4357. That is
//! 304 less than the Chinese count of the same year; the sexagenary term is
//! of course identical, and [`LunisolarParameters::sexagenary_year`] corrects
//! for the offset so that it stays so.
//!
//! # Range
//!
//! As for [`crate::chinese`]: 1645-01-01 to 2150-12-31. The Korean court
//! adopted the Shíxiàn rules some years after 1645, so dates in the gap are
//! what these rules give rather than what was proclaimed in Hanseong.

use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd};

use crate::civil;
use crate::lunisolar::{
    CHINESE_EPOCH, LunisolarCalendar, LunisolarDate, LunisolarParameters, MeridianEra,
    SolarTermMode,
};

/// The machine identifier CLDR uses for this calendar.
pub const ID: CalendarId = CalendarId("dangi");

/// How far the Dangi year number falls below the continuous Chinese count.
///
/// The Dangi epoch is 2333 BCE and the Chinese one 2637 BCE.
pub const YEAR_OFFSET: i64 = -304;

/// The earliest fixed day this calendar converts.
pub const EARLIEST: Rd = civil::to_rd(1645, 1, 1);

/// The latest fixed day this calendar converts.
pub const LATEST: Rd = civil::to_rd(2150, 12, 31);

/// The meridian history of the Korean calendar.
pub static MERIDIANS: [MeridianEra; 5] = [
    MeridianEra::from_zone(i64::MIN / 4, 3_809.0 / 450.0, "Seoul local mean time"),
    MeridianEra::from_zone(1908, 8.5, "the 127°30′E zone"),
    MeridianEra::from_zone(1912, 9.0, "the 135°E zone"),
    MeridianEra::from_zone(1954, 8.5, "the 127°30′E zone"),
    MeridianEra::from_zone(1961, 9.0, "the 135°E zone"),
];

/// The parameters of the Korean calendar.
pub static PARAMETERS: LunisolarParameters = LunisolarParameters {
    id: ID,
    english_name: "Dangi (Korean lunisolar)",
    meridians: &MERIDIANS,
    epoch: CHINESE_EPOCH,
    year_offset: YEAR_OFFSET,
    solar_term_mode: SolarTermMode::Apparent,
    earliest: Some(EARLIEST),
    latest: Some(LATEST),
};

/// The engine configured as the Korean calendar.
pub const ENGINE: LunisolarCalendar = LunisolarCalendar::new(&PARAMETERS);

/// A Korean lunisolar date. See [`crate::chinese::ChineseDate`] for why the
/// representation is shared.
pub type DangiDate = LunisolarDate;

/// The Korean lunisolar calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DangiCalendar;

impl Calendar for DangiCalendar {
    type Date = DangiDate;

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

/// The fixed day of Seollal — the Korean new year — of `year`.
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
    use crate::chinese::{self, ChineseCalendar};
    use hc_calendar::{CalendarError, Month};

    #[test]
    fn seollal_2024_was_the_tenth_of_february_and_the_year_is_dangi_4357() {
        assert_eq!(new_year(4_357), Ok(civil::to_rd(2024, 2, 10)));
        assert_eq!(
            DangiCalendar.from_fixed(civil::to_rd(2024, 2, 10)),
            Ok(LunisolarDate::new(4_357, Month::regular(1), 1))
        );
        // 2024 + 2333 = 4357.
        assert_eq!(4_357 - YEAR_OFFSET, 4_661);
    }

    #[test]
    fn the_sexagenary_year_matches_the_chinese_one_despite_the_offset() {
        for gregorian in 1900..2100i64 {
            let dangi = gregorian + 2_333;
            let chinese_year = gregorian + 2_637;
            assert_eq!(
                PARAMETERS.sexagenary_year(dangi),
                chinese::PARAMETERS.sexagenary_year(chinese_year),
                "Gregorian {gregorian}"
            );
        }
    }

    #[test]
    fn seollal_1988_fell_a_day_after_chinese_new_year() {
        // Widely reported: Korea kept Seollal on 18 February 1988 while
        // China's new year was 17 February. The meridian is the only reason.
        assert_eq!(new_year(4_321), Ok(civil::to_rd(1988, 2, 18)));
        assert_eq!(chinese::new_year(4_625), Ok(civil::to_rd(1988, 2, 17)));
    }

    #[test]
    fn the_two_calendars_disagree_only_occasionally() {
        let mut differences = 0;
        for gregorian in 1900..2050i64 {
            let korean = new_year(gregorian + 2_333).expect("in range");
            let chinese_day = chinese::new_year(gregorian + 2_637).expect("in range");
            if korean != chinese_day {
                differences += 1;
                assert_eq!(
                    (korean.0 - chinese_day.0).abs(),
                    1,
                    "Gregorian {gregorian} differed by more than a day"
                );
            }
        }
        // Measured, not asserted away: nine disagreements in 150 years.
        assert_eq!(differences, 9);
    }

    #[test]
    fn the_half_hour_zones_are_read_from_the_table() {
        for (year, hours) in [
            (1900i64, 3_809.0 / 450.0),
            (1908, 8.5),
            (1912, 9.0),
            (1930, 9.0),
            (1954, 8.5),
            (1960, 8.5),
            (1961, 9.0),
            (2024, 9.0),
        ] {
            let era = PARAMETERS.meridian_era(civil::to_rd(year, 6, 1));
            assert!(
                (era.offset_hours - hours).abs() < 1e-9,
                "{year} gave {}",
                era.offset_hours
            );
        }
    }

    #[test]
    fn the_calendar_round_trips_across_every_meridian_change() {
        let calendar = DangiCalendar;
        for start in [
            civil::to_rd(1906, 1, 1),
            civil::to_rd(1910, 1, 1),
            civil::to_rd(1952, 1, 1),
            civil::to_rd(1959, 1, 1),
        ] {
            for offset in 0..1_200i64 {
                let rd = Rd(start.0 + offset);
                let date = calendar.from_fixed(rd).expect("in range");
                assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
            }
        }
    }

    #[test]
    fn the_calendar_round_trips_over_four_thousand_modern_days() {
        let calendar = DangiCalendar;
        let start = civil::to_rd(2010, 1, 1);
        for offset in 0..4_000i64 {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn years_hold_twelve_or_thirteen_months_of_twenty_nine_or_thirty_days() {
        for year in 4_300..4_360i64 {
            let months = PARAMETERS.months_in_year(year).expect("in range");
            assert!(months == 12 || months == 13, "Dangi {year} had {months}");
            let mut cursor = new_year(year).expect("in range");
            let end = new_year(year + 1).expect("in range");
            let mut counted = 0u8;
            while cursor < end {
                let date = DangiCalendar.from_fixed(cursor).expect("in range");
                let length = PARAMETERS
                    .days_in_month(date.year, date.month)
                    .expect("exists") as i64;
                assert!((29..=30).contains(&length));
                counted += 1;
                cursor = Rd(cursor.0 + length);
            }
            assert_eq!(counted, months);
        }
    }

    #[test]
    fn the_calendar_is_not_the_chinese_one_even_where_they_agree() {
        // The same fixed day has different year numbers in the two
        // calendars, so a date is not portable between them by accident.
        let rd = civil::to_rd(2024, 6, 1);
        let korean = DangiCalendar.from_fixed(rd).expect("in range");
        let chinese_date = ChineseCalendar.from_fixed(rd).expect("in range");
        assert_eq!(korean.month, chinese_date.month);
        assert_eq!(korean.day, chinese_date.day);
        assert_eq!(chinese_date.year - korean.year, 304);
    }

    #[test]
    fn the_range_is_refused_rather_than_extrapolated() {
        assert_eq!(
            DangiCalendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            DangiCalendar.from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn the_metadata_names_the_cldr_identifier() {
        assert_eq!(DangiCalendar.meta().id, CalendarId("dangi"));
        assert!(DangiCalendar.meta().is_astronomical);
    }
}
