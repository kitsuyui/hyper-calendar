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
//! months beginning 17 January 1904, 7 November 1904, 4 May 1905,
//! 30 April 1908 and 20 December 1911, whose conjunction fell after
//! midnight at Seoul and before it at Beijing, and the almanac's day for
//! the fourth month of 1906, 24 April. So the calendar reads Beijing's meridian,
//! [`crate::chinese::ALMANAC_CORRECTIONS`] and
//! [`crate::chinese::ALMANAC_TERM_CORRECTIONS`] until 1912, where the
//! published code's `korean-location` reads Seoul mean time to 1908 and
//! the Korean Empire's 127°30′E zone from 1908 (`reingold2018code`). The
//! 1908 zone was the clock's, not the calendar's. From 1912 the calendar is
//! computed on Korean standard time, and the two half-hour periods really
//! did move its day boundary: over 1900–2049 the Korean and Chinese new
//! years fall on different days nine times, 1988 among them.
//!
//! Before 1900 KASI's data follow the Qing almanac too, month starts the
//! modern rules miss included, with three exceptions it does not carry:
//! 1653, Joseon's first year on the Shíxiàn rules, where KASI has 閏七月
//! from 23 August and 八月 from 21 September against the Qing 閏六月 from
//! 24 July and 八月 from 22 September; the twelfth month of the same year,
//! which KASI begins on 19 January 1654, Seoul's day, and the Qing
//! almanac on the 18th; and the twelfth month of 1841, which KASI begins on
//! 12 January 1842, the rules' day, and the Qing almanac on the 11th.
//! Nothing read says what Hanseong printed in those months, so this
//! calendar stays the Qing one; KASI's reading is a calendar of its own
//! name, [`DangiKasiCalendar`], `dangi-kasi`, which follows KASI in all
//! four, and `docs/systems/east-asian-lunisolar.md` has the measurement.
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

use hc_calendar::gregorian;
use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd};

use crate::chinese::{ALMANAC_CORRECTIONS, ALMANAC_TERM_CORRECTIONS};
use crate::lunisolar::{
    CHINESE_EPOCH, LunisolarCalendar, LunisolarDate, LunisolarParameters, MajorTermCorrection,
    MeridianEra, MonthStartCorrection, SolarTermMode,
};

/// The machine identifier CLDR uses for this calendar.
pub const ID: CalendarId = CalendarId("dangi");

/// How far the Dangi year number falls below the continuous Chinese count.
///
/// The Dangi epoch is 2333 BCE and the Chinese one 2637 BCE.
pub const YEAR_OFFSET: i64 = -304;

/// The last day the lunisolar calendar was Korea's civil calendar,
/// 31 December 1895: the next day was 建陽 元年 1月 1日, Gregorian.
pub const LAST_CIVIL: Rd = gregorian::to_fixed_saturating(1895, 12, 31);

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Joseon's adoption of the Shíxiàn rules in 1653 [wikipedia-ko-siheollyeok, \
    wikipedia-en-korean-calendar], the range beginning with the rules themselves in 1645; \
    civil until Korea adopted the Gregorian calendar on 1 January 1896 [kowiki-geonyang]; \
    kept since for Seollal and Chuseok, as \
    docs/systems/east-asian-lunisolar.md states";

/// The earliest fixed day this calendar converts.
pub const EARLIEST: Rd = gregorian::to_fixed_saturating(1645, 1, 1);

/// The latest fixed day this calendar converts.
pub const LATEST: Rd = gregorian::to_fixed_saturating(2150, 12, 31);

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
    // this calendar reads in the years they cover: 1906 is the last.
    month_start_corrections: &ALMANAC_CORRECTIONS,
    major_term_corrections: &ALMANAC_TERM_CORRECTIONS,
    mean_motion: None,
    earliest: Some(EARLIEST),
    latest: Some(LATEST),
};

/// The engine configured as the Korean calendar.
pub const ENGINE: LunisolarCalendar = LunisolarCalendar::new(&PARAMETERS);

/// The identifier of the Korean calendar as KASI publishes it before 1912.
pub const ID_KASI: CalendarId = CalendarId("dangi-kasi");

/// The first days of the Korean calendar as KASI publishes it: the Qing
/// almanac's, [`crate::chinese::ALMANAC_CORRECTIONS`], less the three it
/// does not follow and with one of its own.
///
/// - **Not followed**: 八月 of 1653 and 十二月 of 1841, which KASI begins
///   on the rules' days, 21 September 1653 and 12 January 1842, where the
///   Veritable Records have the day after and the day before
///   (`kasi-lunisolar-conversion`, `qing-shilu`). 九月 of 1652, which KASI
///   also begins on the rules' day, is before [`KASI_EARLIEST`].
/// - **Its own**: 十二月 of 1653, which KASI begins on 19 January 1654,
///   the day of the conjunction at Seoul, where the rules at Beijing and
///   the Records have the 18th.
pub static KASI_CORRECTIONS: [MonthStartCorrection; 27] = [
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1654, 1, 18),
        gregorian::to_fixed_saturating(1654, 1, 19),
        "KASI's conversion data, 12월 1일 on 1654-01-19 and 11월 30일 on 1654-01-18 \
        [kasi-lunisolar-conversion]",
    ),
    ALMANAC_CORRECTIONS[2],
    ALMANAC_CORRECTIONS[3],
    ALMANAC_CORRECTIONS[4],
    ALMANAC_CORRECTIONS[5],
    ALMANAC_CORRECTIONS[6],
    ALMANAC_CORRECTIONS[7],
    ALMANAC_CORRECTIONS[8],
    ALMANAC_CORRECTIONS[9],
    ALMANAC_CORRECTIONS[10],
    ALMANAC_CORRECTIONS[11],
    ALMANAC_CORRECTIONS[12],
    ALMANAC_CORRECTIONS[13],
    ALMANAC_CORRECTIONS[14],
    ALMANAC_CORRECTIONS[15],
    ALMANAC_CORRECTIONS[16],
    ALMANAC_CORRECTIONS[17],
    ALMANAC_CORRECTIONS[18],
    ALMANAC_CORRECTIONS[19],
    ALMANAC_CORRECTIONS[21],
    ALMANAC_CORRECTIONS[22],
    ALMANAC_CORRECTIONS[23],
    ALMANAC_CORRECTIONS[24],
    ALMANAC_CORRECTIONS[25],
    ALMANAC_CORRECTIONS[26],
    ALMANAC_CORRECTIONS[27],
    ALMANAC_CORRECTIONS[28],
];

/// The term days of the Korean calendar as KASI publishes it: the Qing
/// almanac's five, [`crate::chinese::ALMANAC_TERM_CORRECTIONS`], and one
/// inferred from KASI's leap month of 1653.
///
/// KASI has 七月 from 24 July 1653, 閏七月 from 23 August and 八月 from
/// 21 September (`kasi-lunisolar-conversion`), where the rules and the
/// Veritable Records have 閏六月 from 24 July (`qing-shilu`). The rules put
/// 處暑 at 05:43 Beijing mean time on 23 August, the first day of the
/// month after; KASI's months follow only if its 處暑 fell on the 22nd,
/// in the month from 24 July. KASI gives no term days, so the 22nd is
/// inferred from its months, as the entry says.
pub static KASI_TERM_CORRECTIONS: [MajorTermCorrection; 6] = [
    ALMANAC_TERM_CORRECTIONS[0],
    ALMANAC_TERM_CORRECTIONS[1],
    MajorTermCorrection::new(
        7,
        gregorian::to_fixed_saturating(1653, 8, 23),
        gregorian::to_fixed_saturating(1653, 8, 22),
        "處暑 on 22 August 1653 inferred from KASI's 閏七月 from 23 August and 七月 from 24 July \
        [kasi-lunisolar-conversion]; no record of the term day read",
    ),
    ALMANAC_TERM_CORRECTIONS[2],
    ALMANAC_TERM_CORRECTIONS[3],
    ALMANAC_TERM_CORRECTIONS[4],
];

/// The Korean calendar as the Korea Astronomy and Space Science Institute
/// publishes it (`kasi-lunisolar-conversion`), a reading of its own
/// before 1912 and the same as [`PARAMETERS`] from then.
///
/// `dangi` is the Qing almanac before 1912 throughout, because KASI's data
/// follow it in every month but four, and nothing read says what Hanseong
/// printed in those four; this reading follows KASI in them too. The two
/// disagree, so each is registered under its own name (docs/policy.md §5):
/// this one is [`DangiKasiCalendar`], `dangi-kasi`, and its departures are
/// [`KASI_CORRECTIONS`] and [`KASI_TERM_CORRECTIONS`].
///
/// It begins with 1653, the first year Joseon kept the Shíxiàn rules
/// ([`KASI_EARLIEST`]). Before it KASI's data are Korea's older calendar,
/// which no parameter set here computes: KASI has 1651's new year on
/// 20 February and no 閏二月, where the Shíxiàn rules have 閏二月 from
/// 21 March.
pub static KASI_PARAMETERS: LunisolarParameters = LunisolarParameters {
    id: ID_KASI,
    english_name: "Dangi (Korean lunisolar, as KASI publishes it)",
    month_start_corrections: &KASI_CORRECTIONS,
    major_term_corrections: &KASI_TERM_CORRECTIONS,
    earliest: Some(KASI_EARLIEST),
    ..PARAMETERS
};

/// The first day of [`KASI_PARAMETERS`]: 29 January 1653, the first day of
/// the first year Joseon kept the Shíxiàn rules, 효종 4년
/// (`wikipedia-ko-siheollyeok`), as the rules and the Veritable Records
/// (`qing-shilu`, 順治十年正月) give it.
pub const KASI_EARLIEST: Rd = gregorian::to_fixed_saturating(1653, 1, 29);

/// The engine configured with [`KASI_PARAMETERS`].
pub const KASI_ENGINE: LunisolarCalendar = LunisolarCalendar::new(&KASI_PARAMETERS);

/// A Korean lunisolar date. See [`crate::chinese::ChineseDate`] for why the
/// representation is shared.
pub type DangiDate = LunisolarDate;

/// A Korean lunisolar calendar: a unit type delegating to one configured
/// engine, so that `dangi` and `dangi-kasi` are one implementation over two
/// parameter sets.
macro_rules! korean_calendar {
    ($(#[$doc:meta])* $name:ident, $engine:expr, $parameters:expr, $usage:expr) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct $name;

        impl Calendar for $name {
            type Date = DangiDate;

            fn usage(&self) -> hc_calendar::Usage {
                $usage
            }

            fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
                hc_calendar::shape::LUNISOLAR_TWELVE
            }

            fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
                $parameters.is_leap_year(year)
            }

            fn meta(&self) -> CalendarMeta {
                $engine.meta()
            }

            /// The engine's one-new-moon rule, not the trait's day-by-day walk.
            fn days_in_month(&self, fields: &DateFields) -> CalendarResult<u16> {
                $engine.days_in_month(fields)
            }

            fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
                $engine.to_fixed(date)
            }

            fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
                $engine.from_fixed(rd)
            }

            fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
                $engine.to_fields(date)
            }

            fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
                $engine.from_fields(fields)
            }
        }
    };
}

korean_calendar!(
    /// The Korean lunisolar calendar, `dangi`: in use from the Shíxiàn
    /// rules of 1645, which Joseon adopted in 1653 — a day in that gap is
    /// what the rules give, not what Hanseong proclaimed — civil until the
    /// end of 1895, and the calendar of Seollal and Chuseok since.
    DangiCalendar,
    ENGINE,
    PARAMETERS,
    hc_calendar::Usage::since(EARLIEST, USAGE_SOURCE).civil_until(LAST_CIVIL)
);

/// Where the period of use of `dangi-kasi` comes from.
pub const KASI_USAGE_SOURCE: &str = "The Korean calendar as the Korea Astronomy and Space Science \
    Institute's conversion data give it [kasi-lunisolar-conversion], from Joseon's adoption of the \
    Shíxiàn rules in 1653 [wikipedia-ko-siheollyeok]; civil until Korea adopted the Gregorian \
    calendar on 1 January 1896 [kowiki-geonyang] and kept since for Seollal and Chuseok, as \
    `dangi`";

korean_calendar!(
    /// The Korean lunisolar calendar as KASI publishes it, `dangi-kasi`
    /// ([`KASI_PARAMETERS`]): `dangi` but in the four months of 1653 and
    /// 1841 where KASI's data leave the Qing almanac, from 1653.
    DangiKasiCalendar,
    KASI_ENGINE,
    KASI_PARAMETERS,
    hc_calendar::Usage::since(KASI_EARLIEST, KASI_USAGE_SOURCE).civil_until(LAST_CIVIL)
);

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
    use hc_calendar::{CalendarError, Month};

    use super::*;
    use crate::chinese::{self, ChineseCalendar};

    /// KASI's conversion data (`kasi-lunisolar-conversion`) on both sides of
    /// each of its departures from the Qing almanac, as queried on
    /// 2026-09-27: Gregorian day, then Dangi year, month, leap and day.
    /// A Gregorian day and the Dangi date KASI gives it.
    type KasiDay = ((i64, u8, u8), (i64, u8, bool, u8));

    const KASI_DAYS: [KasiDay; 12] = [
        ((1653, 7, 24), (3_986, 7, false, 1)),
        ((1653, 8, 22), (3_986, 7, false, 30)),
        ((1653, 8, 23), (3_986, 7, true, 1)),
        ((1653, 9, 21), (3_986, 8, false, 1)),
        ((1653, 9, 22), (3_986, 8, false, 2)),
        ((1653, 10, 20), (3_986, 8, false, 30)),
        ((1653, 10, 21), (3_986, 9, false, 1)),
        ((1654, 1, 18), (3_986, 11, false, 30)),
        ((1654, 1, 19), (3_986, 12, false, 1)),
        ((1842, 1, 11), (4_174, 11, false, 30)),
        ((1842, 1, 12), (4_174, 12, false, 1)),
        ((1906, 4, 24), (4_239, 4, false, 1)),
    ];

    #[test]
    fn the_kasi_reading_follows_kasi_where_the_qing_almanac_does_not() {
        let mut differs_from_dangi = 0;
        for ((year, month, day), (dangi_year, ordinal, leap, dangi_day)) in KASI_DAYS {
            let rd = gregorian::to_fixed_saturating(year, month, day);
            let month = if leap {
                Month::leap(ordinal)
            } else {
                Month::regular(ordinal)
            };
            let kasi = LunisolarDate::new(dangi_year, month, dangi_day);
            assert_eq!(
                KASI_ENGINE.from_fixed(rd),
                Ok(kasi),
                "{year}-{month:?}-{day}"
            );
            assert_eq!(KASI_ENGINE.to_fixed(kasi), Ok(rd));
            if ENGINE.from_fixed(rd) != Ok(kasi) {
                differs_from_dangi += 1;
            }
        }
        // Every one of those days differs from `dangi`, the Qing almanac,
        // but 九月初一 of 1653, where the two meet again, and 1906's.
        assert_eq!(differs_from_dangi, KASI_DAYS.len() - 2);
        // From 1912 the two readings are one calendar.
        for year in [1912, 1954, 1988, 2024] {
            let chinese_year = year + 2_637 + YEAR_OFFSET;
            assert_eq!(
                KASI_PARAMETERS.new_year(chinese_year),
                PARAMETERS.new_year(chinese_year)
            );
        }
        assert_eq!(
            KASI_ENGINE.from_fixed(Rd(KASI_EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            KASI_ENGINE.from_fixed(KASI_EARLIEST),
            Ok(LunisolarDate::new(3_986, Month::regular(1), 1))
        );
        // The tables stay sorted by the rules' day, as the engine needs.
        assert!(
            KASI_CORRECTIONS
                .windows(2)
                .all(|pair| pair[0].computed < pair[1].computed)
        );
        assert!(
            KASI_TERM_CORRECTIONS
                .windows(2)
                .all(|pair| pair[0].computed < pair[1].computed)
        );
    }

    #[test]
    fn seollal_2024_was_the_tenth_of_february_and_the_year_is_dangi_4357() {
        assert_eq!(
            new_year(4_357),
            Ok(gregorian::to_fixed_saturating(2024, 2, 10))
        );
        assert_eq!(
            DangiCalendar.from_fixed(gregorian::to_fixed_saturating(2024, 2, 10)),
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
        assert_eq!(
            new_year(4_321),
            Ok(gregorian::to_fixed_saturating(1988, 2, 18))
        );
        assert_eq!(
            chinese::new_year(4_625),
            Ok(gregorian::to_fixed_saturating(1988, 2, 17))
        );
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
            let era = PARAMETERS.meridian_era(gregorian::to_fixed_saturating(year, 6, 1));
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
            major_term_corrections: &[],
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
                gregorian::to_fixed_saturating(1954, 1, 1),
                gregorian::to_fixed_saturating(1954, 3, 20),
            ),
            // 1 January to 9 August 1961: UT+8:30 against UT+9.
            (
                &AT_HALF,
                &AT_NINE,
                gregorian::to_fixed_saturating(1961, 1, 1),
                gregorian::to_fixed_saturating(1961, 8, 9),
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
                .meridian_era(gregorian::to_fixed_saturating(1954, 2, 1))
                .offset_hours
                - 8.5)
                .abs()
                < 1e-9
        );
        assert!(
            (PARAMETERS
                .meridian_era(gregorian::to_fixed_saturating(1961, 2, 1))
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
            let rd = gregorian::to_fixed_saturating(year, month, day);
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
        // has the same month and day in both. A debug build takes every
        // seventh day and each new year and the day before it.
        let new_years = (4_233..=4_245).filter_map(|year| new_year(year).ok().map(|rd| rd.0));
        for rd in crate::sweep_days(
            gregorian::to_fixed_saturating(1900, 1, 1).0,
            gregorian::to_fixed_saturating(1911, 12, 31).0,
            7,
            new_years,
        ) {
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
    fn before_1900_the_almanacs_months_are_kasis_but_for_three() {
        // KASI's data, read 2026-09-27 (`kasi-lunisolar-conversion`), on
        // the first days the Qing almanac moved from the rules
        // (`chinese::ALMANAC_CORRECTIONS`): day 1 of the month in all of
        // 1655–1899 but 1842, and the leap months of 1661, 1727 and 1805
        // where the almanac, not the rules, has them.
        for (year, month, day) in [
            (1673, 11, 9),
            (1686, 4, 23),
            (1687, 3, 13),
            (1692, 6, 15),
            (1693, 4, 6),
            (1704, 10, 29),
            (1708, 2, 21),
            (1713, 12, 18),
            (1715, 3, 6),
            (1728, 8, 6),
            (1731, 6, 5),
            (1754, 9, 17),
            (1789, 10, 19),
            (1794, 11, 23),
            (1813, 5, 1),
            (1817, 10, 11),
            (1820, 12, 6),
            (1823, 5, 11),
            (1842, 11, 3),
            (1849, 9, 17),
            (1856, 11, 28),
            (1861, 11, 3),
            (1869, 5, 12),
            (1880, 11, 3),
            (1887, 3, 25),
        ] {
            let rd = gregorian::to_fixed_saturating(year, month, day);
            assert_eq!(
                DangiCalendar.from_fixed(rd).map(|date| date.day),
                Ok(1),
                "{year}-{month}-{day}"
            );
        }
        for (year, leap, (y, m, d)) in [
            (4_298, 7, (1661, 8, 25)),
            (4_364, 3, (1727, 4, 21)),
            (4_442, 6, (1805, 7, 26)),
        ] {
            assert_eq!(
                DangiCalendar.to_fixed(LunisolarDate::new(
                    year + YEAR_OFFSET,
                    Month::leap(leap),
                    1
                )),
                Ok(gregorian::to_fixed_saturating(y, m, d))
            );
        }
        // The three KASI does not follow the almanac in, which this calendar
        // does: 閏六月 of 1653 from 24 July (KASI: 閏七月 from 23 August),
        // and the twelfth months that begin on 18 January 1654 and
        // 11 January 1842 (KASI: the 19th and the 12th).
        assert_eq!(PARAMETERS.leap_month(4_290 + YEAR_OFFSET), Ok(Some(6)));
        for (y, m, d) in [(1654, 1, 18), (1842, 1, 11)] {
            let date = DangiCalendar
                .from_fixed(gregorian::to_fixed_saturating(y, m, d))
                .expect("in range");
            assert_eq!((date.month, date.day), (Month::regular(12), 1));
        }
    }

    /// The Qing years, where the Chinese corrections this calendar shares
    /// live: every day of 1645–1911 round-trips, in a release build; a
    /// debug build takes every sixty-first day, and every new year, every
    /// day a correction moves a first day or a term from or to, and the day
    /// before each.
    #[test]
    fn the_calendar_round_trips_over_the_qing_years() {
        let calendar = DangiCalendar;
        let years =
            (4_281 + YEAR_OFFSET..=4_549 + YEAR_OFFSET).filter_map(|year| new_year(year).ok());
        let corrections = chinese::ALMANAC_CORRECTIONS
            .iter()
            .flat_map(|c| [c.computed, c.promulgated])
            .chain(
                chinese::ALMANAC_TERM_CORRECTIONS
                    .iter()
                    .flat_map(|c| [c.computed, c.promulgated]),
            );
        let boundaries: Vec<i64> = years.chain(corrections).map(|rd| rd.0).collect();
        for rd in crate::sweep_days(
            EARLIEST.0,
            chinese::LAST_CIVIL.0,
            61,
            boundaries.iter().copied(),
        ) {
            let rd = Rd(rd);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    /// `dangi-kasi` is registered under its own name and round-trips every
    /// day of its range in a release build; a debug build takes every
    /// sixty-first day and every new year, every day one of its
    /// corrections moves, and the day before each. From 1912 it is `dangi`
    /// day for day, which a sample of the modern years holds.
    #[test]
    fn the_kasi_reading_is_registered_and_round_trips() {
        let calendar = DangiKasiCalendar;
        assert_eq!(calendar.meta().id, ID_KASI);
        assert_eq!(calendar.meta().native_locales, &["ko"]);
        assert_eq!(calendar.usage().from, Some(KASI_EARLIEST));
        assert!(!calendar.usage().source.is_empty());
        let years = (4_286 + YEAR_OFFSET..=4_789 + YEAR_OFFSET)
            .filter_map(|year| KASI_PARAMETERS.new_year(year).ok());
        let corrections = KASI_CORRECTIONS
            .iter()
            .flat_map(|c| [c.computed, c.promulgated])
            .chain(
                KASI_TERM_CORRECTIONS
                    .iter()
                    .flat_map(|c| [c.computed, c.promulgated]),
            );
        let boundaries: Vec<i64> = years
            .chain(corrections)
            .map(|rd| rd.0)
            .chain([LATEST.0 + 1])
            .collect();
        let mut days: Vec<i64> =
            crate::sweep_days(KASI_EARLIEST.0, LATEST.0, 61, boundaries.iter().copied()).collect();
        days.sort_unstable();
        days.dedup();
        assert!(days.contains(&KASI_EARLIEST.0) && days.contains(&LATEST.0));
        crate::check_days(&days, |rd| {
            let rd = Rd(rd);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        });
        let modern = gregorian::to_fixed_saturating(1912, 1, 1).0;
        for rd in (modern..=LATEST.0).step_by(crate::sweep_stride(1) * 97) {
            assert_eq!(
                calendar.from_fixed(Rd(rd)),
                DangiCalendar.from_fixed(Rd(rd)),
                "RD {rd}"
            );
        }
    }

    #[test]
    fn the_calendar_round_trips_across_every_meridian_change() {
        let calendar = DangiCalendar;
        for start in [
            gregorian::to_fixed_saturating(1906, 1, 1),
            gregorian::to_fixed_saturating(1910, 1, 1),
            gregorian::to_fixed_saturating(1952, 1, 1),
            gregorian::to_fixed_saturating(1959, 1, 1),
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
        let start = gregorian::to_fixed_saturating(2010, 1, 1);
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
        let rd = gregorian::to_fixed_saturating(2024, 6, 1);
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
