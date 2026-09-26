//! The Chinese lunisolar calendar — CLDR `chinese`.
//!
//! The rules are in [`crate::lunisolar`]; this module is the parameters.
//! The calendar as the Purple Mountain Observatory promulgates it under
//! GB/T 33661-2017, the Shíxiàn reform of 1645 that bounds it, the 1929
//! change of meridian, the year counts in circulation and the published new
//! years it was checked against are in
//! `docs/systems/east-asian-lunisolar.md`.
//!
//! # The meridian
//!
//! | From | Offset | |
//! |---|---|---|
//! | — | UT+7:45:40 | Beijing local mean time, 116°25′E |
//! | 1929 | UT+8 | the 120°E standard zone |
//!
//! Both rows are Reingold and Dershowitz's (`reingold2018code`,
//! `chinese-location`). The Chinese Wikipedia's 农历 (`wikipedia-zh-nongli`)
//! dates the 120°E standard to 民國十七年, 1928, and quotes 116°23′E as the
//! Beijing local time of the Hong Kong Space Museum's almanac; the first
//! moves no date, the second one month start, in 1687, and the tests below
//! measure both.
//!
//! A conjunction or a solstice falling in the fourteen minutes between the
//! two local midnights lands on different days under the two conventions,
//! and that moves a month boundary or, through the zhōngqì test, a leap
//! month.
//!
//! # The almanac, where it was read
//!
//! Before 1912 the calendar was the Qing 時憲書, computed by the Bureau of
//! Astronomy with the methods of the *Lìxiàng kǎochéng hòubiān* of 1742
//! in Beijing apparent time (`liu-chinese-calendar-computation`), not by
//! modern astronomy; where one of its conjunctions lies minutes from
//! midnight the rules here can put the month on the other day.
//! [`ALMANAC_CORRECTIONS`] carries the months where a published table of
//! the promulgated calendar says so. From 1900 the table is the Purple
//! Mountain Observatory's 1900–2025 calendar (`pmo-calendar-1900-2025`),
//! which follows the 時憲書 to 1911, and it differs from the rules in one
//! month: the fourth of 1906, which the rules begin on 23 April and the
//! almanac on 24 April. Before 1900 no table was read and nothing is
//! corrected, so a date there is the rule's; the document measures how
//! often that is not the almanac's.
//!
//! # Year numbering
//!
//! Years are counted continuously from the 2637 BCE epoch of Reingold and
//! Dershowitz's published code ([`crate::lunisolar::CHINESE_EPOCH`]), so the
//! year that began on 2024-02-10 is 4661: the count for which
//! [`hc_calendar::cycle::sexagenary_year`] is directly right, 4661 being
//! *jiǎ-chén*, the Wood Dragon. The number is this library's choice, not a
//! count anyone prints: the book writes a year as cycle and position, and
//! the almanacs' 黃帝紀元 is 4722. The document says how the counts 4721 and
//! 4722 relate to it; none is official, because the calendar has no
//! official continuous era. The cycle and position are available through
//! [`Calendar::to_fields`], the reign eras through `hc-calendars-regional`.
//!
//! # Range
//!
//! 1645-01-01 to 2150-12-31 Gregorian: from the Shíxiàn calendar, which
//! introduced the true-solar-term rule implemented here, to the end of
//! `hc-astro`'s ΔT fit. Earlier years are refused rather than answered with
//! a rule that was not in force.

use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd};

use crate::civil;
use crate::lunisolar::{
    CHINESE_EPOCH, LunisolarCalendar, LunisolarDate, LunisolarParameters, MeridianEra,
    MonthStartCorrection, SolarTermMode,
};

/// The machine identifier CLDR uses for this calendar.
pub const ID: CalendarId = CalendarId("chinese");

/// The last day the calendar was China's civil calendar, 31 December 1911:
/// the Republic adopted the Gregorian calendar at its founding the next day.
pub const LAST_CIVIL: Rd = civil::to_rd(1911, 12, 31);

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The Shíxiàn calendar promulgated by the Shunzhi Emperor for 1645 [wikipedia-en-chongzhen-calendar]; \
    civil until the Republic adopted the Gregorian calendar at its founding on 1 January 1912 \
    [wikipedia-adoption-gregorian]; kept since for \
    the festivals, under GB/T 33661-2017 today, as docs/systems/east-asian-lunisolar.md states";

/// The earliest fixed day this calendar converts.
pub const EARLIEST: Rd = civil::to_rd(1645, 1, 1);

/// The latest fixed day this calendar converts.
pub const LATEST: Rd = civil::to_rd(2150, 12, 31);

/// Where [`MERIDIANS`] comes from.
pub const MERIDIAN_SOURCES: &str = "Beijing at 116°25′E, 1397/180 hours, before 1929 and the 120°E zone from \
    1929, as chinese-location in the published code of Calendrical Calculations \
    [reingold2018code]; the reference recorded as moving to UT+8 in 1928-1929 \
    [wikipedia-en-time-in-china], and 1928 with 116°23′E in [wikipedia-zh-nongli]";

/// Where [`ALMANAC_CORRECTIONS`] comes from.
pub const ALMANAC_CORRECTION_SOURCES: &str = "The 《时宪书》 of 光绪三十二年 as the Purple Mountain Observatory's \
    1900-2025 calendar gives it, 四月初一 on 戊戌, 24 April 1906 [pmo-calendar-1900-2025]; the same day \
    in the Hong Kong Observatory's table for 1906 [hko-conversion-tables] and in KASI's conversion \
    service [kasi-lunisolar-conversion]";

/// The months of 1900–2025 that the promulgated calendar began on another
/// day than the rules do. Sources: [`ALMANAC_CORRECTION_SOURCES`].
///
/// One entry: 光緒三十二年四月, whose conjunction the rules place at 23:52
/// Beijing mean time on 23 April 1906 and whose first day the 時憲書 gives
/// as 戊戌, 24 April. The third month runs thirty days with it and the
/// fourth twenty-nine.
pub static ALMANAC_CORRECTIONS: [MonthStartCorrection; 1] = [MonthStartCorrection::new(
    civil::to_rd(1906, 4, 23),
    civil::to_rd(1906, 4, 24),
)];

/// The meridian history of the Chinese calendar. Sources:
/// [`MERIDIAN_SOURCES`].
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
    native_locales: &["zh-Hans", "zh-Hant"],
    meridians: &MERIDIANS,
    epoch: CHINESE_EPOCH,
    year_offset: 0,
    solar_term_mode: SolarTermMode::Apparent,
    month_start_corrections: &ALMANAC_CORRECTIONS,
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

    /// In use since the Shíxiàn calendar of 1645, which is also where the
    /// range begins; civil until the end of 1911, and the calendar of the
    /// Spring Festival and every other traditional date since, so it has an
    /// end of civil use and no end.
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

/// The fixed day of Chinese New Year — 1 Zhēngyuè — of `year`.
///
/// # Errors
///
/// Returns [`hc_calendar::CalendarError::YearOutOfRange`] outside the
/// supported range.
pub fn new_year(year: i64) -> CalendarResult<Rd> {
    PARAMETERS.new_year(year)
}

/// A person's age as the Chinese count reckons it, on a fixed day: one at
/// birth, and one more at each Chinese New Year after, whatever the day
/// of birth (`chinese-age` in the published code of *Calendrical
/// Calculations*, `reingold2018code`). `Ok(None)` for a day before the
/// birth, which has no age.
///
/// This is the count Wikipedia's "East Asian age reckoning" gives as the
/// pre-modern reckoning of *suì* in China: one at birth and one more at
/// each lunar new year (`wikipedia-en-east-asian-age-reckoning`). Nothing
/// here says how any other country counts.
///
/// # Errors
///
/// Returns a [`hc_calendar::CalendarError`] when the birth date does not
/// exist or either day is outside the supported range.
pub fn reckoned_age(birth: ChineseDate, on: Rd) -> CalendarResult<Option<u32>> {
    let born = ChineseCalendar.to_fixed(birth)?;
    if on < born {
        return Ok(None);
    }
    let today = ChineseCalendar.from_fixed(on)?;
    u32::try_from(today.year - birth.year + 1)
        .map(Some)
        .map_err(|_| hc_calendar::CalendarError::YearOutOfRange)
}

/// Where 立春 (*lìchūn*, the Beginning of Spring) falls in a Chinese year,
/// the ground of the marriage auguries of the almanacs: none in the year,
/// once near its end, once near its start, or at both
/// (`chinese-year-marriage-augury` and the constants `widow`, `blind`,
/// `bright` and `double-bright` in the published code of *Calendrical
/// Calculations*, `reingold2018code`, whose names these are).
///
/// Wikipedia's "Lichun" calls a year without the term 無春年, 寡婦年
/// ("widow year") in the north and 盲年 ("blind year") in the south, and
/// says marriage in it is thought unlucky (`wikipedia-en-lichun`). So "blind"
/// names a year without 立春 there and a year with it only at the end in
/// the published code; the names here are the code's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarriageAugury {
    /// No 立春 in the year (the code's "double-blind year").
    Widow,
    /// 立春 once, near the end of the year.
    Blind,
    /// 立春 once, near the start of the year.
    Bright,
    /// 立春 twice, at the start and at the end ("double happiness").
    DoubleBright,
}

impl MarriageAugury {
    /// Whether the year's first 立春 comes after its New Year.
    #[must_use]
    pub const fn lichun_at_start(self) -> bool {
        matches!(self, Self::Bright | Self::DoubleBright)
    }

    /// Whether a 立春 comes before the next New Year after the first.
    #[must_use]
    pub const fn lichun_at_end(self) -> bool {
        matches!(self, Self::Blind | Self::DoubleBright)
    }
}

/// The last minor solar term (節氣) to begin before the local midnight
/// that starts `rd`, numbered 1 for 立春 at 315° to 12 for 小寒 at 285°
/// (`current-minor-solar-term` in the published code of *Calendrical
/// Calculations*).
fn minor_solar_term(rd: Rd) -> i64 {
    let longitude = hc_astro::solar_longitude(PARAMETERS.midnight(rd));
    (2 + hc_core::math::floor((longitude - 15.0) / 30.0) as i64).rem_euclid(12) + 1
}

/// The marriage augury of Chinese year `year`
/// (`chinese-year-marriage-augury`): whether 立春 falls after its New Year
/// and whether another falls before the next.
///
/// # Errors
///
/// Returns [`hc_calendar::CalendarError::YearOutOfRange`] when the year or
/// the next one begins outside the supported range.
pub fn marriage_augury(year: i64) -> CalendarResult<MarriageAugury> {
    // At New Year the last minor term is 小寒 (12) when 立春 is still to
    // come, and 立春 (1) itself when it has passed.
    let at_start = minor_solar_term(new_year(year)?) != 1;
    let at_end = minor_solar_term(new_year(year + 1)?) != 12;
    Ok(match (at_start, at_end) {
        (false, false) => MarriageAugury::Widow,
        (false, true) => MarriageAugury::Blind,
        (true, false) => MarriageAugury::Bright,
        (true, true) => MarriageAugury::DoubleBright,
    })
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
    fn new_year_refuses_the_ends_of_i64_instead_of_overflowing() {
        for year in [i64::MIN, i64::MAX] {
            assert_eq!(new_year(year), Err(CalendarError::YearOutOfRange));
            assert_eq!(
                ChineseCalendar.is_leap_year(year),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                ChineseCalendar.to_fixed(LunisolarDate::new(year, Month::regular(1), 1)),
                Err(CalendarError::YearOutOfRange)
            );
        }
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
    fn the_fourth_month_of_1906_began_on_the_day_the_almanac_gave() {
        // 光緒三十二年: 三月 of thirty days ending on 丁酉, 23 April, and
        // 四月初一 on 戊戌, 24 April, in the Purple Mountain Observatory's
        // table from the 時憲書 (`pmo-calendar-1900-2025`); 閏四月 from
        // 23 May, 五月 from 22 June.
        let year = 4_543;
        assert_eq!(new_year(year), Ok(civil::to_rd(1906, 1, 25)));
        assert_eq!(
            ChineseCalendar.to_fixed(LunisolarDate::new(year, Month::regular(4), 1)),
            Ok(civil::to_rd(1906, 4, 24))
        );
        assert_eq!(
            ChineseCalendar.from_fixed(civil::to_rd(1906, 4, 23)),
            Ok(LunisolarDate::new(year, Month::regular(3), 30))
        );
        assert_eq!(PARAMETERS.days_in_month(year, Month::regular(3)), Some(30));
        assert_eq!(PARAMETERS.days_in_month(year, Month::regular(4)), Some(29));
        assert_eq!(PARAMETERS.leap_month(year), Ok(Some(4)));
        assert_eq!(
            ChineseCalendar.to_fixed(LunisolarDate::new(year, Month::leap(4), 1)),
            Ok(civil::to_rd(1906, 5, 23))
        );
        assert_eq!(
            PARAMETERS.sexagenary_day(civil::to_rd(1906, 4, 24)).index(),
            34,
            "戊戌 is the 35th day of the cycle"
        );
    }

    #[test]
    fn the_almanac_corrections_are_live_and_move_a_day_at_most() {
        // Each correction names a first day the rules really give, so an
        // entry cannot outlive a change to the astronomy unnoticed, and none
        // moves further than the engine's search allows.
        static RULES: LunisolarParameters = LunisolarParameters {
            month_start_corrections: &[],
            ..PARAMETERS
        };
        for correction in &ALMANAC_CORRECTIONS {
            assert_eq!(
                RULES.new_moon_on_or_after(correction.computed),
                correction.computed
            );
            assert_ne!(correction.computed, correction.promulgated);
            assert!(
                (correction.promulgated.0 - correction.computed.0).abs()
                    <= MonthStartCorrection::MAX_SHIFT
            );
            // The corrected calendar has a month on the almanac's day and
            // none on the rules'.
            assert_eq!(
                PARAMETERS.new_moon_on_or_after(correction.computed.min(correction.promulgated)),
                correction.promulgated
            );
            assert_eq!(
                PARAMETERS.new_moon_before(Rd(correction.promulgated.0 + 1)),
                correction.promulgated
            );
        }
        // Without the table the rules give 23 April 1906.
        assert_eq!(
            RULES.new_moon_on_or_after(civil::to_rd(1906, 4, 20)),
            civil::to_rd(1906, 4, 23)
        );
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
        // Every day in a release build, every eleventh in a debug one.
        for offset in (0..6_000i64).step_by(crate::sweep_stride(11)) {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    /// The Chinese Wikipedia dates the change to 120°E to 1928 (民國十七年;
    /// `wikipedia-zh-nongli`) and Reingold and Dershowitz to 1929; switching in either year gives
    /// the same day for every date of 1926–1930, so the disagreement moves
    /// nothing. See `docs/systems/solar-terms-and-pentads.md`.
    #[test]
    fn the_1928_and_1929_readings_of_the_meridian_change_agree() {
        static FROM_1928: [MeridianEra; 2] = [
            MERIDIANS[0],
            MeridianEra::from_zone(1928, 8.0, "the 120°E standard zone"),
        ];
        static PARAMETERS_1928: LunisolarParameters = LunisolarParameters {
            meridians: &FROM_1928,
            ..PARAMETERS
        };
        let from_1928 = LunisolarCalendar::new(&PARAMETERS_1928);
        // Every day in a release build, every fifth in a debug one: a month
        // that began a day apart would differ on all of its days.
        for rd in
            (civil::to_rd(1926, 1, 1).0..civil::to_rd(1931, 1, 1).0).step_by(crate::sweep_stride(5))
        {
            assert_eq!(
                ENGINE.from_fixed(Rd(rd)),
                from_1928.from_fixed(Rd(rd)),
                "RD {rd}"
            );
        }
    }

    /// The old Beijing meridian: 116°25′E, Reingold and Dershowitz's 1397⁄180
    /// hours, which this calendar uses, or 116°23′E, the Hong Kong Space
    /// Museum's figure as the Chinese Wikipedia quotes it
    /// (`wikipedia-zh-nongli`). Eight seconds of
    /// time apart, they begin one month of 1645–1929 on different days: the
    /// second month of 4324, on 14 March 1687 at 116°25′ and 13 March at
    /// 116°23′. A release build checks that it is the only one.
    #[test]
    fn the_two_readings_of_the_beijing_meridian_differ_once() {
        static AT_116_23: [MeridianEra; 2] = [
            MeridianEra::from_longitude(i64::MIN / 4, 116.383_333_333_333_33, "116°23′E"),
            MERIDIANS[1],
        ];
        static PARAMETERS_116_23: LunisolarParameters = LunisolarParameters {
            meridians: &AT_116_23,
            ..PARAMETERS
        };
        let other = LunisolarCalendar::new(&PARAMETERS_116_23);
        let march_14 = civil::to_rd(1687, 3, 14);
        let second_month = ENGINE.from_fixed(march_14).expect("in range");
        assert_eq!(
            (
                second_month.year,
                second_month.month.ordinal,
                second_month.day
            ),
            (4324, 2, 1)
        );
        let earlier = other.from_fixed(Rd(march_14.0 - 1)).expect("in range");
        assert_eq!(
            (earlier.year, earlier.month.ordinal, earlier.day),
            (4324, 2, 1)
        );
        if cfg!(debug_assertions) {
            return;
        }
        let differing = (EARLIEST.0..civil::to_rd(1930, 1, 1).0)
            .filter(|rd| ENGINE.from_fixed(Rd(*rd)) != other.from_fixed(Rd(*rd)))
            .count();
        // The thirty days of that one month, 13 March to 11 April 1687.
        assert_eq!(differing, 30);
    }

    #[test]
    fn the_calendar_round_trips_across_the_1929_meridian_change() {
        let calendar = ChineseCalendar;
        let start = civil::to_rd(1925, 1, 1);
        // Every day in a release build, every fifth in a debug one.
        for offset in (0..3_000i64).step_by(crate::sweep_stride(5)) {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn the_calendar_round_trips_at_both_ends_of_its_range() {
        let calendar = ChineseCalendar;
        for start in [EARLIEST.0, LATEST.0 - 2_000] {
            // Every day in a release build, every eleventh in a debug one.
            for offset in (0..2_000i64).step_by(crate::sweep_stride(11)) {
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

    #[test]
    fn a_child_born_in_june_2000_turns_thirteen_at_the_new_year_of_2012() {
        // Wikipedia, "East Asian age reckoning", § People's Republic of
        // China: one suì at birth, one more at each lunar new year, and a
        // child born in June 2000, a dragon year, 13 suì from the lunar new
        // year of 2012.
        let birth = ChineseCalendar
            .from_fixed(civil::to_rd(2000, 6, 15))
            .expect("in range");
        assert_eq!(birth.year, 4_637);
        assert_eq!(PARAMETERS.sexagenary_year(4_637).zodiac_animal(), "dragon");
        let new_year_2012 = new_year(4_649).expect("in range");
        assert_eq!(new_year_2012, civil::to_rd(2012, 1, 23));
        assert_eq!(reckoned_age(birth, new_year_2012), Ok(Some(13)));
        assert_eq!(reckoned_age(birth, Rd(new_year_2012.0 - 1)), Ok(Some(12)));
        assert_eq!(reckoned_age(birth, civil::to_rd(2000, 6, 15)), Ok(Some(1)));
        assert_eq!(reckoned_age(birth, civil::to_rd(2000, 6, 14)), Ok(None));
    }

    #[test]
    fn a_child_born_on_new_years_eve_is_two_the_next_day() {
        let eve = Rd(new_year(4_661).expect("in range").0 - 1);
        let birth = ChineseCalendar.from_fixed(eve).expect("in range");
        assert_eq!(reckoned_age(birth, eve), Ok(Some(1)));
        assert_eq!(reckoned_age(birth, Rd(eve.0 + 1)), Ok(Some(2)));
        assert!(reckoned_age(LunisolarDate::new(4_661, Month::regular(1), 31), eve).is_err());
    }

    #[test]
    fn the_published_widow_and_double_spring_years_are_reproduced() {
        // The Year of the Dragon that began on 10 February 2024 is a Widow
        // Year, "lacking Spring Commences" (South China Morning Post,
        // 3 February 2024).
        assert_eq!(marriage_augury(4_661), Ok(MarriageAugury::Widow));
        // The lunar year that began on 26 January 2009 holds two 立春, on
        // 4 February 2009 and 4 February 2010 (South China Morning Post,
        // 25 January 2009).
        assert_eq!(new_year(4_646), Ok(civil::to_rd(2009, 1, 26)));
        assert_eq!(marriage_augury(4_646), Ok(MarriageAugury::DoubleBright));
    }

    #[test]
    fn a_year_with_two_lichun_has_thirteen_months_and_one_with_none_twelve() {
        // Two 立春 are a tropical year apart and none leaves a gap of one,
        // so the first can only happen in a year longer than 365 days and
        // the second only in a shorter one. The augury and the year length
        // are computed independently, from the Sun and from the Moon.
        let mut seen = [0u32; 4];
        for year in 4_290..4_780i64 {
            let augury = marriage_augury(year).expect("in range");
            let length =
                new_year(year + 1).expect("in range").0 - new_year(year).expect("in range").0;
            match augury {
                MarriageAugury::DoubleBright => assert!(length > 366, "{year}"),
                MarriageAugury::Widow => assert!(length < 365, "{year}"),
                _ => {}
            }
            seen[augury as usize] += 1;
            // What ends one year is what the next does not start with.
            let next = marriage_augury(year + 1).expect("in range");
            assert_eq!(augury.lichun_at_end(), !next.lichun_at_start(), "{year}");
        }
        assert!(seen.iter().all(|count| *count > 0), "{seen:?}");
    }
}
