//! The Vietnamese lunisolar calendar — *âm lịch*.
//!
//! The same rules as [`crate::chinese`] at the Vietnamese meridian. CLDR has
//! no identifier for this calendar, so the crate uses `vietnamese`. The
//! decision of 1967 that fixed the meridian, the calendar's publisher, Tết
//! 1985 worked by hand — where the hour between the two midnights moved the
//! winter solstice, the leap month and the new year by a whole lunation —
//! and what was checked against which publication are in
//! `docs/systems/east-asian-lunisolar.md`.
//!
//! # The meridian, and Tết 1968
//!
//! | From | Offset | |
//! |---|---|---|
//! | — | UT+8 | the 120°E meridian, on which the published code computes the calendar before 1968 (`vietnamese-location`) |
//! | 1968 | UT+7 | the 105°E meridian, by Decision 121-CP of 8 August 1967, from 1 January 1968 |
//!
//! These are the meridians of the *calendar*, not the civil clock: the
//! Democratic Republic had kept UT+7 as civil time since 1945, and the
//! 1967 decision is what moved the calendar's computation onto it. The
//! conjunction that began the Year of the Monkey fell in the hour between
//! the two midnights, so the north, computing on UT+7 from 1 January 1968,
//! kept Tết on **29 January 1968** and the south, whose calendar stayed on
//! UT+8 — its civil time since 1960 — on **30 January**. The calendar here
//! is the northern one, which is the calendar of unified Vietnam; the
//! southern reckoning of that year is [`SOUTHERN_PARAMETERS`], and the
//! crate tests both.
//!
//! The southern reckoning is data, not a registered calendar. It differs
//! from the northern one only between 1968 and the end of the Republic in
//! 1975, and what is known of it here is two new years, Tết 1968 and 1969,
//! and the zone the Republic's clocks kept; no almanac or decree of the
//! Republic was read that fixes its calendar's meridian, its span or its
//! end. A registered `vietnamese-south` would be a claim about seven years
//! of months that nothing read here could check, so the crate keeps the
//! parameters that reproduce the two attested new years and says so. The same hour catches the conjunction of February
//! 1969: Tết Kỷ Dậu fell on 16 February in the north, the day Hồ Chí Minh
//! planted the tree at Vật Lại "sáng 16-2-1969 (mồng 1 Tết)"
//! (`nhandan-tet-trong-cay-2019`), and on 17 February, with Chinese New
//! Year, in the south.
//!
//! # Year numbering
//!
//! There is no continuous Vietnamese era, so years are numbered by the
//! Gregorian year in which they begin: the year that began on 2024-02-10 is
//! 2024. That is this crate's choice rather than anyone's official one, and
//! no claim is made about what other tools do. The
//! sexagenary term — 2024 is *Giáp Thìn* — is the traditional name and is
//! available through [`LunisolarParameters::sexagenary_year`].
//!
//! # Range
//!
//! As for [`crate::chinese`]: 1645-01-01 to 2150-12-31.

use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd};

use crate::civil;
use crate::lunisolar::{
    CHINESE_EPOCH, LunisolarCalendar, LunisolarDate, LunisolarParameters, MeridianEra,
    SolarTermMode,
};

/// The machine identifier this crate uses for the calendar. CLDR has none.
pub const ID: CalendarId = CalendarId("vietnamese");

/// How far the year number falls below the continuous Chinese count.
pub const YEAR_OFFSET: i64 = -2_637;

/// The first day the calendar was computed at UT+7 by decree, 1 January 1968.
pub const DECREED_FROM: Rd = civil::to_rd(1968, 1, 1);

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Decision 121-CP of 8 August 1967, in effect from 1 January 1968 [vn-decision-121-cp]: \
    the calendar at UT+7, the calendar of unified Vietnam and of Tết today; when the \
    Vietnamese court adopted the Shíxiàn rules no source read states, so the earlier \
    years are proleptic";

/// The earliest fixed day this calendar converts.
pub const EARLIEST: Rd = civil::to_rd(1645, 1, 1);

/// The latest fixed day this calendar converts.
pub const LATEST: Rd = civil::to_rd(2150, 12, 31);

/// Where [`MERIDIANS`] comes from.
pub const MERIDIAN_SOURCES: &str = "UT+8 before 1968 and UT+7 from 1 January 1968, as vietnamese-location in \
    the published code of Calendrical Calculations [reingold2018code]; the change by \
    Decision 121-CP of 8 August 1967 [vn-decision-121-cp]";

/// The meridian history of the Vietnamese calendar, northern reckoning.
/// Sources: [`MERIDIAN_SOURCES`].
pub static MERIDIANS: [MeridianEra; 2] = [
    MeridianEra::from_zone(i64::MIN / 4, 8.0, "the 120°E meridian"),
    MeridianEra::from_zone(1968, 7.0, "the 105°E meridian"),
];

/// The meridian of the Republic of Vietnam's calendar, which stayed on
/// UT+8 through Tết 1968.
///
/// Kept as data rather than prose so that the crate can test the
/// disagreement rather than only describe it. It keeps UT+8 for all time,
/// as the published code does for the whole country before 1968, and
/// carries no era for the south's civil time of UT+7 before 1 January
/// 1960: the calendar's meridian is not the civil clock — the north kept
/// UT+7 civil time from 1945 and computed its calendar on UT+8 until 1968
/// — and no southern almanac was read. Had the set been read at UT+7
/// before 1960, three month boundaries of the Republic's years would move
/// by a day (2 to 3 November 1956, 1 to 2 March 1957, 21 to 22 November
/// 1957) and no new year or leap month; the system document has the
/// measurement.
pub static SOUTHERN_MERIDIANS: [MeridianEra; 1] = [MeridianEra::from_zone(
    i64::MIN / 4,
    8.0,
    "the 120°E meridian",
)];

/// The parameters of the Vietnamese calendar.
pub static PARAMETERS: LunisolarParameters = LunisolarParameters {
    id: ID,
    english_name: "Vietnamese lunisolar",
    native_locales: &["vi"],
    meridians: &MERIDIANS,
    epoch: CHINESE_EPOCH,
    year_offset: YEAR_OFFSET,
    solar_term_mode: SolarTermMode::Apparent,
    mean_motion: None,
    earliest: Some(EARLIEST),
    latest: Some(LATEST),
};

/// The parameters of the southern reckoning, kept on UT+8 throughout.
pub static SOUTHERN_PARAMETERS: LunisolarParameters = LunisolarParameters {
    id: CalendarId("vietnamese-south-1968"),
    english_name: "Vietnamese lunisolar (Republic of Vietnam reckoning)",
    native_locales: &["vi"],
    meridians: &SOUTHERN_MERIDIANS,
    epoch: CHINESE_EPOCH,
    year_offset: YEAR_OFFSET,
    solar_term_mode: SolarTermMode::Apparent,
    mean_motion: None,
    earliest: Some(EARLIEST),
    latest: Some(LATEST),
};

/// The engine configured as the Vietnamese calendar.
pub const ENGINE: LunisolarCalendar = LunisolarCalendar::new(&PARAMETERS);

/// A Vietnamese date. See [`crate::chinese::ChineseDate`] for why the
/// representation is shared.
pub type VietnameseDate = LunisolarDate;

/// The Vietnamese lunisolar calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VietnameseCalendar;

impl Calendar for VietnameseCalendar {
    type Date = VietnameseDate;

    /// In use since 1 January 1968, when Decision 121-CP fixed the meridian
    /// this calendar computes at, and the calendar of Tết since. The years
    /// from 1645 convert under the same rules and are proleptic: no source
    /// read dates the Vietnamese court's adoption of them.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(DECREED_FROM, USAGE_SOURCE)
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

/// The fixed day of Tết Nguyên Đán, the Vietnamese new year, of `year`.
///
/// # Errors
///
/// Returns [`hc_calendar::CalendarError::YearOutOfRange`] outside the
/// supported range.
pub fn tet(year: i64) -> CalendarResult<Rd> {
    PARAMETERS.new_year(year)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chinese;
    use hc_calendar::{CalendarError, Month};

    #[test]
    fn tet_1968_fell_on_different_days_in_the_north_and_the_south() {
        // The documented case: the north, computing on UT+7 from 1 January
        // 1968, kept Tết on 29 January; the south, whose calendar stayed on
        // UT+8, on 30 January.
        assert_eq!(tet(1_968), Ok(civil::to_rd(1968, 1, 29)));
        assert_eq!(
            SOUTHERN_PARAMETERS.new_year(1_968),
            Ok(civil::to_rd(1968, 1, 30))
        );
    }

    #[test]
    fn the_two_reckonings_agree_everywhere_else_in_that_decade() {
        let mut differences = 0;
        for year in 1_960..1_980i64 {
            if tet(year) != SOUTHERN_PARAMETERS.new_year(year) {
                differences += 1;
            }
        }
        // Only 1968, 1969 and 1985-style near misses; in this window the
        // change of zone shows up in a handful of years.
        assert!(differences >= 1, "the zone change had no effect at all");
        assert!(differences <= 5, "{differences} years differed");
    }

    #[test]
    fn tet_1969_fell_a_day_before_chinese_new_year() {
        // The conjunction of 16 February 1969 at 16:25 UT is 23:25 at UT+7,
        // still the 16th, and 00:25 at UT+8, the 17th. Nhân Dân dates Hồ
        // Chí Minh's tree-planting at
        // Vật Lại "sáng 16-2-1969 (mồng 1 Tết)" (`nhandan-tet-trong-cay-2019`);
        // the south, on UT+8, kept Tết with China on the 17th.
        assert_eq!(tet(1_969), Ok(civil::to_rd(1969, 2, 16)));
        assert_eq!(
            SOUTHERN_PARAMETERS.new_year(1_969),
            Ok(civil::to_rd(1969, 2, 17))
        );
        assert_eq!(
            chinese::new_year(1_969 + 2_637),
            Ok(civil::to_rd(1969, 2, 17))
        );
    }

    #[test]
    fn tet_2024_was_the_tenth_of_february_and_the_year_is_numbered_2024() {
        assert_eq!(tet(2_024), Ok(civil::to_rd(2024, 2, 10)));
        assert_eq!(
            VietnameseCalendar.from_fixed(civil::to_rd(2024, 2, 10)),
            Ok(LunisolarDate::new(2_024, Month::regular(1), 1))
        );
    }

    #[test]
    fn the_year_2024_is_giap_thin_the_wood_dragon() {
        let cycle = PARAMETERS.sexagenary_year(2_024);
        assert_eq!(cycle.stem_name(), "jia");
        assert_eq!(cycle.branch_name(), "chen");
        assert_eq!(cycle.zodiac_animal(), "dragon");
    }

    #[test]
    fn tet_1985_fell_a_whole_month_before_chinese_new_year() {
        // The other well-known divergence, and a larger one than Tết 1968:
        // the hour between the two midnights moved a zhōngqì, which moved
        // the leap month, which moved the new year by a whole lunation.
        // Tết 1985 was 21 January; Chinese New Year was 20 February.
        assert_eq!(tet(1_985), Ok(civil::to_rd(1985, 1, 21)));
        assert_eq!(
            chinese::new_year(1_985 + 2_637),
            Ok(civil::to_rd(1985, 2, 20))
        );
    }

    #[test]
    fn the_calendar_sometimes_differs_from_the_chinese_one_since_1968() {
        let mut differences = 0;
        for year in 1_968..2_050i64 {
            let vietnamese = tet(year).expect("in range");
            let chinese_day = chinese::new_year(year + 2_637).expect("in range");
            if vietnamese != chinese_day {
                differences += 1;
                // Either a day, from a conjunction between the midnights, or
                // a whole lunation, when the leap month moves with it.
                let gap = (vietnamese.0 - chinese_day.0).abs();
                assert!(
                    gap == 1 || (29..=30).contains(&gap),
                    "year {year} gap {gap}"
                );
            }
        }
        // The hour between UT+7 and UT+8 midnight catches a new moon every
        // few years.
        assert!(differences > 0, "the meridian made no difference at all");
    }

    #[test]
    fn the_calendar_round_trips_across_the_1968_change() {
        let calendar = VietnameseCalendar;
        let start = civil::to_rd(1964, 1, 1);
        for offset in 0..3_000i64 {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn the_calendar_round_trips_over_four_thousand_modern_days() {
        let calendar = VietnameseCalendar;
        let start = civil::to_rd(2005, 1, 1);
        // Every day in a release build, every eleventh in a debug one.
        for offset in (0..4_000i64).step_by(crate::sweep_stride(11)) {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn years_hold_twelve_or_thirteen_months() {
        for year in 1_990..2_050i64 {
            let months = PARAMETERS.months_in_year(year).expect("in range");
            assert!(months == 12 || months == 13, "year {year} had {months}");
            assert_eq!(
                PARAMETERS.leap_month(year).expect("in range").is_some(),
                months == 13
            );
        }
    }

    #[test]
    fn the_range_is_refused_rather_than_extrapolated() {
        assert_eq!(
            VietnameseCalendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            VietnameseCalendar.from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }
}
