//! The Chinese lunisolar calendar — CLDR `chinese`.
//!
//! The rules are in [`crate::lunisolar`]; this module is the parameters.
//!
//! # The meridian
//!
//! | From | Offset | |
//! |---|---|---|
//! | — | UT+7:45:40 | Beijing local mean time, 116°25′E |
//! | 1929 | UT+8 | the 120°E standard zone |
//!
//! The change is real and visible. A conjunction or a solstice falling in the
//! fourteen minutes between 116°25′E and 120°E local midnight lands on
//! different days under the two conventions, and that moves a month boundary
//! or, through the zhōngqì test, a leap month.
//!
//! # Year numbering
//!
//! Years are counted continuously from the traditional epoch of 2637 BCE, so
//! the year that began on 2024-02-10 is 4661. That is the numbering
//! *Calendrical Calculations* uses and the one for which
//! [`hc_calendar::cycle::sexagenary_year`] gives the right answer directly:
//! 4661 is *jiǎ-chén*, the Wood Dragon. Other conventions in circulation
//! number the same year 4721 or 4722; none of them is official, because the
//! calendar has no official continuous era. Traditional dates are written
//! with the sexagenary cycle and a reign, and both are available here — the
//! cycle and position through [`Calendar::to_fields`], the reign eras
//! through `hc-calendars-regional`.
//!
//! # Range
//!
//! 1645-01-01 to 2150-12-31 Gregorian. The lower bound is the Shíxiàn
//! calendar of 1645, which introduced the true-solar-term rule implemented
//! here; before it the terms were mean, the month numbering could differ, and
//! no modern computation reproduces what the Bureau of Astronomy actually
//! published. The upper bound is where `hc-astro`'s ΔT fit ends.

use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd};

use crate::civil;
use crate::lunisolar::{
    CHINESE_EPOCH, LunisolarCalendar, LunisolarDate, LunisolarParameters, MeridianEra,
    SolarTermMode,
};

/// The machine identifier CLDR uses for this calendar.
pub const ID: CalendarId = CalendarId("chinese");

/// The earliest fixed day this calendar converts.
pub const EARLIEST: Rd = civil::to_rd(1645, 1, 1);

/// The latest fixed day this calendar converts.
pub const LATEST: Rd = civil::to_rd(2150, 12, 31);

/// The meridian history of the Chinese calendar.
pub static MERIDIANS: [MeridianEra; 2] = [
    MeridianEra::from_longitude(
        i64::MIN / 4,
        116.416_666_666_666_67,
        "Beijing local mean time",
    ),
    MeridianEra::from_zone(1929, 8.0, "the 120°E standard zone"),
];

/// The parameters of the Chinese calendar.
pub static PARAMETERS: LunisolarParameters = LunisolarParameters {
    id: ID,
    english_name: "Chinese",
    meridians: &MERIDIANS,
    epoch: CHINESE_EPOCH,
    year_offset: 0,
    solar_term_mode: SolarTermMode::Apparent,
    mean_motion: None,
    earliest: Some(EARLIEST),
    latest: Some(LATEST),
};

/// The engine configured as the Chinese calendar.
pub const ENGINE: LunisolarCalendar = LunisolarCalendar::new(&PARAMETERS);

/// A Chinese date.
///
/// All four lunisolar calendars in this crate share one representation on
/// purpose: a year, a possibly-intercalary month and a day is the whole of
/// what any of them records, and the calendar the date came from is what says
/// which days those are. Four identical structs would be the copying this
/// crate exists to avoid.
pub type ChineseDate = LunisolarDate;

/// The Chinese lunisolar calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ChineseCalendar;

impl Calendar for ChineseCalendar {
    type Date = ChineseDate;

    fn cycles(&self) -> Option<&'static [hc_calendar::shape::CycleShape]> {
        Some(hc_calendar::shape::LUNISOLAR_TWELVE)
    }

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

/// The fixed day of Chinese New Year — 1 Zhēngyuè — of `year`.
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
    use crate::lunisolar::LunisolarDate;
    use hc_calendar::{CalendarError, Month};

    #[test]
    fn chinese_new_year_2024_was_the_tenth_of_february() {
        // A published anchor: the Year of the Wood Dragon began on
        // 2024-02-10.
        let rd = civil::to_rd(2024, 2, 10);
        assert_eq!(new_year(4_661), Ok(rd));
        assert_eq!(
            ChineseCalendar.from_fixed(rd),
            Ok(LunisolarDate::new(4_661, Month::regular(1), 1))
        );
    }

    #[test]
    fn the_year_that_began_in_2024_is_jia_chen_the_wood_dragon() {
        let cycle = PARAMETERS.sexagenary_year(4_661);
        assert_eq!(cycle.stem_name(), "jia");
        assert_eq!(cycle.branch_name(), "chen");
        assert_eq!(cycle.zodiac_animal(), "dragon");
        assert_eq!(cycle.five_phase(), "wood");
        // 1984 was the last jiǎ-zǐ year, the start of a sexagenary cycle.
        assert_eq!(PARAMETERS.sexagenary_year(4_621).index(), 0);
    }

    #[test]
    fn twenty_twenty_three_had_a_leap_second_month() {
        // A published anchor: 閏二月 of 2023 began on 2023-03-22.
        assert_eq!(PARAMETERS.leap_month(4_660), Ok(Some(2)));
        assert_eq!(
            ChineseCalendar.to_fixed(LunisolarDate::new(4_660, Month::leap(2), 1)),
            Ok(civil::to_rd(2023, 3, 22))
        );
        assert_eq!(PARAMETERS.months_in_year(4_660), Ok(13));
        assert_eq!(PARAMETERS.is_leap_year(4_660), Ok(true));
    }

    #[test]
    fn other_published_new_years_are_reproduced() {
        // Dates in general circulation for the start of the Chinese year.
        for (year, gregorian) in [
            (4_537, (1900, 1, 31)),
            (4_637, (2000, 2, 5)),
            (4_657, (2020, 1, 25)),
            (4_658, (2021, 2, 12)),
            (4_659, (2022, 2, 1)),
            (4_660, (2023, 1, 22)),
            (4_661, (2024, 2, 10)),
            (4_662, (2025, 1, 29)),
            (4_663, (2026, 2, 17)),
        ] {
            assert_eq!(
                new_year(year),
                Ok(civil::to_rd(gregorian.0, gregorian.1, gregorian.2)),
                "year {year}"
            );
        }
    }

    #[test]
    fn chinese_new_year_always_falls_between_january_twenty_first_and_february_twenty_first() {
        for year in 4_570..4_780i64 {
            let rd = new_year(year).expect("in range");
            let (_, month, day) = civil::from_rd(rd);
            let within = (month == 1 && day >= 21) || (month == 2 && day <= 21);
            assert!(within, "year {year} began on {month}-{day}");
        }
    }

    #[test]
    fn the_calendar_round_trips_over_six_thousand_modern_days() {
        let calendar = ChineseCalendar;
        let start = civil::to_rd(2000, 1, 1);
        for offset in 0..6_000i64 {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn the_calendar_round_trips_across_the_1929_meridian_change() {
        let calendar = ChineseCalendar;
        let start = civil::to_rd(1925, 1, 1);
        for offset in 0..3_000i64 {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn the_calendar_round_trips_at_both_ends_of_its_range() {
        let calendar = ChineseCalendar;
        for start in [EARLIEST.0, LATEST.0 - 2_000] {
            for offset in 0..2_000i64 {
                let rd = Rd(start + offset);
                let date = calendar.from_fixed(rd).expect("in range");
                assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
            }
        }
    }

    #[test]
    fn the_range_is_refused_rather_than_extrapolated() {
        let calendar = ChineseCalendar;
        assert_eq!(
            calendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            calendar.from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(new_year(4_000), Err(CalendarError::YearOutOfRange));
    }

    #[test]
    fn months_are_twenty_nine_or_thirty_days_and_years_twelve_or_thirteen_months() {
        for year in 4_630..4_700i64 {
            let months = PARAMETERS.months_in_year(year).expect("in range");
            assert!(months == 12 || months == 13, "year {year}");
            let start = new_year(year).expect("in range");
            let end = new_year(year + 1).expect("in range");
            let mut cursor = start;
            let mut counted = 0u8;
            let mut ordinals = [0u8; 13];
            while cursor < end {
                let date = ChineseCalendar.from_fixed(cursor).expect("in range");
                assert_eq!(date.day, 1);
                let length = PARAMETERS
                    .days_in_month(date.year, date.month)
                    .expect("the month exists") as i64;
                assert!((29..=30).contains(&length), "year {year} month {length}");
                ordinals[counted as usize] = date.month.ordinal;
                counted += 1;
                cursor = Rd(cursor.0 + length);
            }
            assert_eq!(cursor, end);
            assert_eq!(counted, months, "year {year}");
            // The months run 1..=12 in order, with the leap month repeating
            // the ordinal before it.
            assert_eq!(ordinals[0], 1);
        }
    }

    #[test]
    fn every_leap_month_immediately_follows_the_month_it_repeats() {
        for year in 4_600..4_700i64 {
            let Some(ordinal) = PARAMETERS.leap_month(year).expect("in range") else {
                continue;
            };
            let regular = ChineseCalendar
                .to_fixed(LunisolarDate::new(year, Month::regular(ordinal), 1))
                .expect("exists");
            let leap = ChineseCalendar
                .to_fixed(LunisolarDate::new(year, Month::leap(ordinal), 1))
                .expect("exists");
            let length = PARAMETERS
                .days_in_month(year, Month::regular(ordinal))
                .expect("exists") as i64;
            assert_eq!(leap.0 - regular.0, length, "year {year}");
        }
    }

    #[test]
    fn leap_months_are_rare_and_never_the_first_month() {
        let mut leaps = 0;
        for year in 4_600..4_700i64 {
            if let Some(ordinal) = PARAMETERS.leap_month(year).expect("in range") {
                leaps += 1;
                assert_ne!(ordinal, 1, "year {year} had a leap first month");
            }
        }
        // Seven leap years in nineteen, so about 37 in a hundred.
        assert!(
            (30..=45).contains(&leaps),
            "{leaps} leap years in a century"
        );
    }

    #[test]
    fn the_generic_interface_carries_the_cycle_and_its_position() {
        let calendar = ChineseCalendar;
        let date = calendar
            .from_fixed(civil::to_rd(2024, 2, 10))
            .expect("in range");
        let fields = calendar.to_fields(date).expect("describable");
        assert_eq!(fields.year, 4_661);
        // 4661 = 60 * 77 + 41, so it is position 41 of cycle 78.
        assert_eq!(fields.extra.get("cycle"), Some(78));
        assert_eq!(fields.extra.get("year_of_cycle"), Some(41));
        assert_eq!(calendar.from_fields(&fields), Ok(date));
    }

    #[test]
    fn the_metadata_admits_the_calendar_is_astronomical() {
        let meta = ChineseCalendar.meta();
        assert_eq!(meta.id, CalendarId("chinese"));
        assert!(meta.has_leap_months);
        assert!(meta.is_astronomical);
        assert_eq!(meta.earliest, Some(EARLIEST));
        assert_eq!(meta.latest, Some(LATEST));
    }
}
