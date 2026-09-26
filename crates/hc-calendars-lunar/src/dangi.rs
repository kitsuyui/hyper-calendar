//! The Korean lunisolar calendar — CLDR `dangi`.
//!
//! The same rules as [`crate::chinese`], read at a different meridian and
//! numbered from a different epoch. Korea used the Chinese calendar of the
//! day throughout, so the arithmetic is not merely similar: it is the same
//! arithmetic, which is why this module is thirty lines of constants and no
//! algorithm. The calendar as KASI publishes it, Joseon's adoption of the
//! Shíxiàn rules in 1653, the days within the years below, Seollal 1988
//! worked by hand and what was checked against which publication are in
//! `docs/systems/east-asian-lunisolar.md`.
//!
//! # The meridian, and why it has four entries
//!
//! | From | Offset | |
//! |---|---|---|
//! | — | UT+7:45:40 | Beijing local mean time, 116°25′E: the Qing calendar |
//! | 1912 | UT+9 | the 135°E zone, under the Governor-General |
//! | 1954 | UT+8:30 | the 127°30′E zone |
//! | 1961 | UT+9 | back to 135°E, where it remains |
//!
//! Before 1912 Korea kept the Qing calendar itself, not the Qing rules at
//! Seoul: KASI's conversion data (`kasi-lunisolar-conversion`) give the
//! Chinese first day for every month of 1900–1911, including the five
//! months — in 1903, 1904, 1905, 1908 and 1911 — whose conjunction fell
//! after midnight at Seoul and before it at Beijing, and the almanac's day
//! for the fourth month of 1906. So the calendar reads Beijing's meridian
//! and [`crate::chinese::ALMANAC_CORRECTIONS`] until 1912, where the
//! published code's `korean-location` reads Seoul mean time to 1908 and
//! the Korean Empire's 127°30′E zone from 1908 (`reingold2018code`). The
//! 1908 zone was the clock's, not the calendar's. From 1912 the calendar is
//! computed on Korean standard time, and the two half-hour periods really
//! did move its day boundary: over 1900–2049 the Korean and Chinese new
//! years fall on different days nine times, 1988 among them.
//!
//! # Year numbering
//!
//! *Dangi* (단기) years count from the traditional foundation of Gojoseon in
//! 2333 BCE, so the year that began on 2024-02-10 is Dangi 4357, 304 less
//! than the Chinese count of the same year, which is what the published
//! code's `korean-year` gives. The count was the Republic of Korea's
//! official year number under the Act on Era Names (연호에 관한 법률, Act
//! No. 4) of 25 September 1948 until the Act of the same name, Act No. 775
//! of 2 December 1961, made the Common Era official from 1 January 1962
//! (`encykorea-dangun-giwon`, read 2026-09-26; the statutes themselves were
//! not read). It is not in official use now; the sexagenary term is the
//! same, and [`LunisolarParameters::sexagenary_year`] corrects for the
//! offset so that it stays so.
//!
//! # Range
//!
//! As for [`crate::chinese`]: 1645-01-01 to 2150-12-31. Joseon adopted the
//! Shíxiàn rules in 1653, so dates in the gap are what these rules give
//! rather than what was proclaimed in Hanseong.

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

/// The last day the lunisolar calendar was Korea's civil calendar,
/// 31 December 1895: the next day was 建陽 元年 1月 1日, Gregorian.
pub const LAST_CIVIL: Rd = civil::to_rd(1895, 12, 31);

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Joseon's adoption of the Shíxiàn rules in 1653 [wikipedia-ko-siheollyeok, \
    wikipedia-en-korean-calendar], the range beginning with the rules themselves in 1645; \
    civil until Korea adopted the Gregorian calendar on 1 January 1896 [kowiki-geonyang]; \
    kept since for Seollal and Chuseok, as \
    docs/systems/east-asian-lunisolar.md states";

/// The earliest fixed day this calendar converts.
pub const EARLIEST: Rd = civil::to_rd(1645, 1, 1);

/// The latest fixed day this calendar converts.
pub const LATEST: Rd = civil::to_rd(2150, 12, 31);

/// Where [`MERIDIANS`] comes from.
pub const MERIDIAN_SOURCES: &str = "Beijing local mean time, 1397/180 hours, before 1912, the meridian of the \
    Qing calendar that KASI's conversion data follow for 1900-1911 [kasi-lunisolar-conversion], \
    as chinese-location in the published code of Calendrical Calculations [reingold2018code]; \
    the zones of 1912, 1954 and 1961 as korean-location there, the years corroborated by the \
    history of Korean standard time [wikipedia-en-time-in-south-korea]";

/// The meridian history of the Korean calendar. Sources:
/// [`MERIDIAN_SOURCES`].
///
/// The eras are keyed by year where the published code changes on
/// 21 March 1954 and 10 August 1961; the test
/// `the_year_keyed_eras_give_the_days_the_day_keyed_changes_give` shows
/// that no new moon and no zhōngqì in the two partial years falls on a
/// different day under either offset.
pub static MERIDIANS: [MeridianEra; 4] = [
    crate::chinese::MERIDIANS[0],
    MeridianEra::from_zone(1912, 9.0, "the 135°E zone"),
    MeridianEra::from_zone(1954, 8.5, "the 127°30′E zone"),
    MeridianEra::from_zone(1961, 9.0, "the 135°E zone"),
];

/// The parameters of the Korean calendar.
pub static PARAMETERS: LunisolarParameters = LunisolarParameters {
    id: ID,
    english_name: "Dangi (Korean lunisolar)",
    native_locales: &["ko"],
    meridians: &MERIDIANS,
    epoch: CHINESE_EPOCH,
    year_offset: YEAR_OFFSET,
    solar_term_mode: SolarTermMode::Apparent,
    // Keyed by the days the rules give at Beijing, which is the meridian
    // this calendar reads in the years they cover.
    month_start_corrections: &crate::chinese::ALMANAC_CORRECTIONS,
    mean_motion: None,
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

    /// In use from the Shíxiàn rules of 1645, which Joseon adopted in 1653 —
    /// a day in that gap is what the rules give, not what Hanseong proclaimed
    /// — civil until the end of 1895, and the calendar of Seollal and Chuseok
    /// since.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(EARLIEST, USAGE_SOURCE).civil_until(LAST_CIVIL)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        PARAMETERS.is_leap_year(year)
    }

    fn meta(&self) -> CalendarMeta {
        ENGINE.meta()
    }

    /// The engine's one-new-moon rule, not the trait's day-by-day walk.
    fn days_in_month(&self, fields: &DateFields) -> CalendarResult<u16> {
        ENGINE.days_in_month(fields)
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
            (1900i64, 1_397.0 / 180.0),
            // The Korean Empire's 127°30′E zone of 1908 was the clock's;
            // the calendar stayed on Beijing's.
            (1908, 1_397.0 / 180.0),
            (1911, 1_397.0 / 180.0),
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

    /// A parameter set reading the whole range at one offset, for the
    /// comparison below.
    const fn at_offset(meridians: &'static [MeridianEra]) -> LunisolarParameters {
        LunisolarParameters {
            id: CalendarId("dangi-test"),
            english_name: "test",
            native_locales: &["ko"],
            meridians,
            epoch: CHINESE_EPOCH,
            year_offset: YEAR_OFFSET,
            solar_term_mode: SolarTermMode::Apparent,
            month_start_corrections: &[],
            mean_motion: None,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        }
    }

    #[test]
    fn the_year_keyed_eras_give_the_days_the_day_keyed_changes_give() {
        // The published code's `korean-location` changes offset on
        // 21 March 1954 and 10 August 1961; this table changes at the start
        // of each of those years. In the two windows where the two
        // disagree, every new moon and every zhōngqì must fall on the same
        // day under either offset, or a date would differ. (The winter
        // solstice is outside both.) Its 1908 change is not this
        // calendar's, and its 1912 one falls on 1 January, as here.
        static HALF: [MeridianEra; 1] = [MeridianEra::from_zone(
            i64::MIN / 4,
            8.5,
            "the 127°30′E zone",
        )];
        static NINE: [MeridianEra; 1] =
            [MeridianEra::from_zone(i64::MIN / 4, 9.0, "the 135°E zone")];
        static AT_HALF: LunisolarParameters = at_offset(&HALF);
        static AT_NINE: LunisolarParameters = at_offset(&NINE);
        let windows: [(&LunisolarParameters, &LunisolarParameters, Rd, Rd); 2] = [
            // 1 January to 20 March 1954: UT+9 against UT+8:30.
            (
                &AT_NINE,
                &AT_HALF,
                civil::to_rd(1954, 1, 1),
                civil::to_rd(1954, 3, 20),
            ),
            // 1 January to 9 August 1961: UT+8:30 against UT+9.
            (
                &AT_HALF,
                &AT_NINE,
                civil::to_rd(1961, 1, 1),
                civil::to_rd(1961, 8, 9),
            ),
        ];
        let mut events = 0;
        for (published, table, first, last) in windows {
            let mut previous_moon = None;
            for rd in first.0..=last.0 {
                let rd = Rd(rd);
                let moon = published.new_moon_on_or_after(rd);
                assert_eq!(moon, table.new_moon_on_or_after(rd), "new moon from {rd}");
                if previous_moon != Some(moon) {
                    events += 1;
                    previous_moon = Some(moon);
                }
                assert_eq!(
                    published.major_solar_term(rd),
                    table.major_solar_term(rd),
                    "zhōngqì index at {rd}"
                );
                assert_eq!(published.from_fixed(rd), table.from_fixed(rd), "{rd}");
            }
        }
        // The windows hold about ten lunations between them.
        assert!(events >= 10, "{events} new moons checked");
        // And the table does read the offsets the windows assume.
        assert!(
            (PARAMETERS
                .meridian_era(civil::to_rd(1954, 2, 1))
                .offset_hours
                - 8.5)
                .abs()
                < 1e-9
        );
        assert!(
            (PARAMETERS
                .meridian_era(civil::to_rd(1961, 2, 1))
                .offset_hours
                - 9.0)
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn before_1912_the_months_begin_where_kasi_has_them_and_not_where_seoul_would() {
        // KASI's conversion data, read 2026-09-27
        // (`kasi-lunisolar-conversion`): each of these days is the first of
        // a lunar month, and the next day the second. They are the five
        // months of 1900–1911 whose conjunction fell before midnight at
        // Beijing and after it at Seoul, and the fourth month of 1906, which
        // the Qing almanac began a day after the rules do at Beijing.
        static SEOUL: [MeridianEra; 1] = [MeridianEra::from_zone(
            i64::MIN / 4,
            3_809.0 / 450.0,
            "Seoul local mean time",
        )];
        static AT_SEOUL: LunisolarParameters = at_offset(&SEOUL);
        for ((year, month, day), dangi_year, ordinal, seoul_is_a_day_late) in [
            ((1904, 1, 17), 4_236, 12, true),
            ((1904, 11, 7), 4_237, 10, true),
            ((1905, 5, 4), 4_238, 4, true),
            ((1906, 4, 24), 4_239, 4, false),
            ((1908, 4, 30), 4_241, 4, true),
            ((1911, 12, 20), 4_244, 11, true),
        ] {
            let rd = civil::to_rd(year, month, day);
            assert_eq!(
                DangiCalendar.from_fixed(rd),
                Ok(LunisolarDate::new(dangi_year, Month::regular(ordinal), 1)),
                "{year}-{month}-{day}"
            );
            assert_eq!(
                chinese::PARAMETERS.new_moon_on_or_after(rd),
                rd,
                "the Chinese calendar agrees on {year}-{month}-{day}"
            );
            if seoul_is_a_day_late {
                assert_eq!(
                    AT_SEOUL.new_moon_on_or_after(rd),
                    Rd(rd.0 + 1),
                    "{year}-{month}-{day}"
                );
            }
        }
        // And the two calendars are one before 1912: every day of 1900–1911
        // has the same month and day in both.
        for rd in
            (civil::to_rd(1900, 1, 1).0..civil::to_rd(1912, 1, 1).0).step_by(crate::sweep_stride(7))
        {
            let korean = DangiCalendar.from_fixed(Rd(rd)).expect("in range");
            let chinese_date = ChineseCalendar.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(
                (korean.month, korean.day),
                (chinese_date.month, chinese_date.day),
                "RD {rd}"
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
            // Every day in a release build, every fifth in a debug one.
            for offset in (0..1_200i64).step_by(crate::sweep_stride(5)) {
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
        // Every day in a release build, every eleventh in a debug one.
        for offset in (0..4_000i64).step_by(crate::sweep_stride(11)) {
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
