//! The Islamic calendar of the Fiqh Council of North America (FCNA),
//! `islamic-fcna`, as the Council publishes it.
//!
//! The Council computes its calendar and publishes the first day of every
//! month from Muḥarram 1440 (11 September 2018) to Dhū al-Ḥijja 1467
//! (12 October 2045) (`fcna-calendar`). Its stated criterion is the
//! European Council for Fatwa and Research's: after the conjunction,
//! somewhere on the globe at local sunset, the Moon at least 8° from the
//! Sun and at least 5° above the horizon, the month beginning the next day
//! if so and the day after otherwise. The Council does not say whether the
//! angles are geocentric or topocentric, or where "somewhere" may be, and
//! no reading this library tried reproduces the published months: the
//! closest, the Unified Hijri Calendar's rule with geocentric angles, gives
//! 305 of the 335 published first days (`docs/systems/unified-hijri.md`).
//! So this calendar is the **table**, like [`crate::islamic_umalqura`], and
//! no computation stands in for it. The table dates months only: the
//! Council's Eid al-Aḍḥā is "the day after Yawm 'Arafah as determined by
//! the Supreme Court of Saudi Arabia" (`fcna-calendar`), which can differ
//! from the 10th of the table's Dhū al-Ḥijja, and is not carried.
//!
//! The Council's earlier rule, adopted on 10 June 2006, began a month at
//! the sunset of the day on whose Greenwich noon the conjunction had
//! already fallen (`fcna-2006`). No month the Council dated by it was
//! read, and it is not carried.
//!
//! # Range
//!
//! 1 Muḥarram 1440 to the last day of Jumādā II 1465, 11 September 2018 to
//! 7 June 2043. The page skips the first of Shaʿbān 1465, so the length of
//! Rajab 1465 is not known, and it and every later month are refused,
//! though the page lists them. Outside the range the calendar returns
//! [`CalendarError::BeforeEpoch`] or
//! [`CalendarError::AfterSupportedRange`].

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::tabular::{ERA, IslamicDate};

/// The identifier of this calendar.
pub const ID: CalendarId = CalendarId("islamic-fcna");

/// The first Hijri year the table covers.
pub const FIRST_YEAR: i64 = 1_440;

/// The fixed day of 1 Muḥarram 1440, 11 September 2018.
pub const EPOCH: Rd = Rd(736_948);

/// The first day of every month from Muḥarram 1440 to Rajab 1465, in days
/// after [`EPOCH`], as the Council's page lists them (`fcna-calendar`).
/// The last entry, Rajab 1465, closes Jumādā II 1465 and opens no month
/// this table converts.
const MONTH_STARTS: [u16; 307] = [
    0, 29, 58, 88, 118, 148, 178, 207, 237, 266, 296, 325, // 1440 AH
    354, 384, 413, 443, 472, 502, 532, 561, 591, 621, 650, 680, // 1441 AH
    709, 738, 768, 797, 827, 856, 886, 915, 945, 975, 1_004, 1_034, // 1442 AH
    1_063, 1_093, 1_122, 1_152, 1_181, 1_211, 1_240, 1_270, 1_299, 1_329, 1_358,
    1_388, // 1443 AH
    1_417, 1_447, 1_477, 1_506, 1_536, 1_565, 1_595, 1_624, 1_654, 1_683, 1_713,
    1_742, // 1444 AH
    1_771, 1_801, 1_831, 1_861, 1_891, 1_920, 1_950, 1_979, 2_008, 2_038, 2_067,
    2_096, // 1445 AH
    2_126, 2_155, 2_185, 2_215, 2_245, 2_274, 2_304, 2_333, 2_363, 2_392, 2_422,
    2_451, // 1446 AH
    2_480, 2_510, 2_539, 2_569, 2_599, 2_629, 2_658, 2_688, 2_717, 2_747, 2_776,
    2_806, // 1447 AH
    2_835, 2_864, 2_894, 2_923, 2_953, 2_983, 3_012, 3_042, 3_072, 3_101, 3_131,
    3_160, // 1448 AH
    3_190, 3_219, 3_248, 3_278, 3_307, 3_337, 3_366, 3_396, 3_426, 3_455, 3_485,
    3_515, // 1449 AH
    3_544, 3_574, 3_603, 3_633, 3_662, 3_691, 3_721, 3_750, 3_780, 3_809, 3_839,
    3_869, // 1450 AH
    3_898, 3_928, 3_958, 3_987, 4_017, 4_046, 4_075, 4_105, 4_134, 4_164, 4_193,
    4_223, // 1451 AH
    4_252, 4_282, 4_312, 4_342, 4_371, 4_401, 4_430, 4_459, 4_489, 4_518, 4_548,
    4_577, // 1452 AH
    4_607, 4_636, 4_666, 4_696, 4_726, 4_755, 4_784, 4_814, 4_843, 4_873, 4_902,
    4_932, // 1453 AH
    4_961, 4_990, 5_020, 5_050, 5_080, 5_109, 5_139, 5_168, 5_198, 5_227, 5_257,
    5_286, // 1454 AH
    5_316, 5_345, 5_374, 5_404, 5_434, 5_463, 5_493, 5_522, 5_552, 5_582, 5_611,
    5_641, // 1455 AH
    5_670, 5_700, 5_729, 5_758, 5_788, 5_817, 5_847, 5_876, 5_906, 5_936, 5_966,
    5_995, // 1456 AH
    6_025, 6_054, 6_084, 6_113, 6_142, 6_172, 6_201, 6_230, 6_260, 6_290, 6_319,
    6_349, // 1457 AH
    6_379, 6_409, 6_438, 6_468, 6_497, 6_526, 6_556, 6_585, 6_614, 6_644, 6_674,
    6_703, // 1458 AH
    6_733, 6_763, 6_793, 6_822, 6_852, 6_881, 6_910, 6_940, 6_969, 6_998, 7_028,
    7_058, // 1459 AH
    7_087, 7_117, 7_147, 7_176, 7_206, 7_235, 7_265, 7_294, 7_324, 7_353, 7_382,
    7_412, // 1460 AH
    7_442, 7_471, 7_501, 7_530, 7_560, 7_590, 7_619, 7_649, 7_678, 7_708, 7_737,
    7_767, // 1461 AH
    7_796, 7_826, 7_855, 7_885, 7_914, 7_944, 7_973, 8_003, 8_032, 8_062, 8_092,
    8_121, // 1462 AH
    8_151, 8_180, 8_210, 8_239, 8_268, 8_298, 8_327, 8_357, 8_387, 8_416, 8_446,
    8_476, // 1463 AH
    8_505, 8_535, 8_564, 8_594, 8_623, 8_652, 8_682, 8_711, 8_741, 8_770, 8_800,
    8_830, // 1464 AH
    8_860, 8_889, 8_919, 8_948, 8_978, 9_007, 9_036, // 1465 AH
];

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The Fiqh Council of North America's published calendar, \
    Muharram 1440 (2018-09-11) to Jumada II 1465 (2043-06-07) as carried [fcna-calendar]";

/// The earliest fixed day this calendar converts: 1 Muḥarram 1440.
pub const EARLIEST: Rd = EPOCH;

/// The latest fixed day this calendar converts: the last day of Jumādā II
/// 1465, 7 June 2043, the day before the last month start carried.
pub const LATEST: Rd = Rd(EPOCH.0 + MONTH_STARTS[MONTH_STARTS.len() - 1] as i64 - 1);

/// The index into [`MONTH_STARTS`] of a year and month, if the table
/// converts it.
const fn month_index(year: i64, month: u8) -> Option<usize> {
    if month == 0 || month > 12 || year < FIRST_YEAR {
        return None;
    }
    let index = (year - FIRST_YEAR) * 12 + month as i64 - 1;
    if index + 1 < MONTH_STARTS.len() as i64 {
        Some(index as usize)
    } else {
        None
    }
}

/// The number of days in a month the table converts.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month_index(year, month) {
        Some(index) => Some((MONTH_STARTS[index + 1] - MONTH_STARTS[index]) as u8),
        None => None,
    }
}

/// The fixed day of an FCNA date.
///
/// # Errors
///
/// [`CalendarError::MonthOutOfRange`] for a month outside 1 to 12,
/// [`CalendarError::BeforeEpoch`] or [`CalendarError::AfterSupportedRange`]
/// outside the table, and [`CalendarError::DayOutOfRange`] for a day the
/// month does not have.
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if month == 0 || month > 12 {
        return Err(CalendarError::MonthOutOfRange);
    }
    if year < FIRST_YEAR {
        return Err(CalendarError::BeforeEpoch);
    }
    let Some(index) = month_index(year, month) else {
        return Err(CalendarError::AfterSupportedRange);
    };
    let length = MONTH_STARTS[index + 1] - MONTH_STARTS[index];
    if day == 0 || day as u16 > length {
        return Err(CalendarError::DayOutOfRange);
    }
    Ok(Rd(EPOCH.0 + MONTH_STARTS[index] as i64 + day as i64 - 1))
}

/// The FCNA year, month and day of a fixed day.
///
/// # Errors
///
/// [`CalendarError::BeforeEpoch`] or [`CalendarError::AfterSupportedRange`]
/// outside the table.
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    let offset = (rd.0 - EPOCH.0) as u16;
    let mut low = 0usize;
    let mut high = MONTH_STARTS.len() - 2;
    while low < high {
        let middle = low + (high - low).div_ceil(2);
        if MONTH_STARTS[middle] <= offset {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    Ok((
        FIRST_YEAR + (low / 12) as i64,
        (low % 12) as u8 + 1,
        (offset - MONTH_STARTS[low]) as u8 + 1,
    ))
}

/// The FCNA's published Islamic calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IslamicFcnaCalendar;

impl Calendar for IslamicFcnaCalendar {
    type Date = IslamicDate;

    /// The span of the published table carried, which is the whole of the
    /// range it converts.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(EARLIEST, LATEST, USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    /// A year of 355 days; a year the table does not hold entirely is
    /// refused.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        let start = to_fixed(year, 1, 1)?;
        let next = to_fixed(year + 1, 1, 1)?;
        Ok(next.0 - start.0 == 355)
    }

    /// The Islamic day begins at sunset and is named by the civil day it
    /// ends on: the Council's month "will start the next day" after the
    /// evening its criterion is met on.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset(hc_calendar::DayNaming::ByEnd)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Hijri (Fiqh Council of North America)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            // The Council computes the table; this implementation reads it.
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["en"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(IslamicDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(ERA))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = IslamicDate {
            year: fields.year,
            month: month.ordinal,
            day: fields.require_day()?,
        };
        to_fixed(date.year, date.month, date.day)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::gregorian;

    /// The Council's own announcement for 1447: Ramaḍān on 18 February
    /// 2026 and Shawwāl on 20 March (`fcna-ramadan-1447`), and the page's
    /// first and last rows carried.
    #[test]
    fn the_published_dates_are_reproduced() {
        assert_eq!(
            to_fixed(1_447, 9, 1),
            Ok(gregorian::to_fixed_saturating(2026, 2, 18))
        );
        assert_eq!(
            to_fixed(1_447, 10, 1),
            Ok(gregorian::to_fixed_saturating(2026, 3, 20))
        );
        assert_eq!(
            to_fixed(1_440, 1, 1),
            Ok(gregorian::to_fixed_saturating(2018, 9, 11))
        );
        assert_eq!(
            from_fixed(gregorian::to_fixed_saturating(2043, 6, 7)),
            Ok((1_465, 6, 29))
        );
    }

    #[test]
    fn the_range_is_refused_rather_than_extrapolated() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(1_439, 12, 29), Err(CalendarError::BeforeEpoch));
        assert_eq!(
            to_fixed(1_465, 7, 1),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(1_441, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(1_441, 1, 31), Err(CalendarError::DayOutOfRange));
    }

    #[test]
    fn every_day_round_trips_and_every_month_has_twenty_nine_or_thirty_days() {
        for rd in EARLIEST.0..=LATEST.0 {
            let (year, month, day) = from_fixed(Rd(rd)).expect("in range");
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)));
        }
        for pair in MONTH_STARTS.windows(2) {
            assert!(matches!(pair[1] - pair[0], 29 | 30));
        }
    }
}
