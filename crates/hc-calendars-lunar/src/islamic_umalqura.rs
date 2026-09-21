//! The Umm al-Qurā calendar of Saudi Arabia — CLDR `islamic-umalqura`.
//!
//! This calendar is not computed. It is **published**: the Umm al-Qurā
//! committee fixes the length of every month in advance and the result is a
//! table, not a formula. Between 1300 AH and 1420 AH the tabulated months
//! follow the conjunction rule then in force; from 1420 AH the criterion is
//! that the geocentric conjunction occurs before sunset at Mecca and the Moon
//! sets after the Sun. Either way, the only correct implementation of a
//! published table is the table.
//!
//! # The table, and where it came from
//!
//! One `u16` per Hijri year carries twelve bits, bit `n` set when month
//! `n + 1` has 30 days rather than 29. That is the whole calendar: 301 years
//! in 602 bytes.
//!
//! The data was extracted on this machine from the platform's own
//! `islamic-umalqura` implementation (Foundation's
//! `Calendar(identifier: .islamicUmmAlQura)`, which is ICU's `UMALQURA`
//! table), by asking it for the first day of every month from 1300 AH to
//! 1600 AH and differencing. The extraction was then spot-checked against
//! dates published by the Saudi authorities and reported in the press:
//!
//! | Hijri | Gregorian | |
//! |---|---|---|
//! | 1 Muḥarram 1300 | 1882-11-12 | the first year of the table |
//! | 1 Muḥarram 1445 | 2023-07-19 | |
//! | 1 Ramaḍān 1445 | 2024-03-11 | start of Ramadan 2024 |
//! | 1 Shawwāl 1445 | 2024-04-10 | Eid al-Fitr 2024 |
//! | 1 Ramaḍān 1446 | 2025-03-01 | start of Ramadan 2025 |
//! | 1 Muḥarram 1447 | 2025-06-26 | |
//!
//! # The range, and why it stops
//!
//! **1300 AH to 1600 AH**, that is 1882-11-12 to 2174-11-25 Gregorian, and
//! not one day more. The official tables are published for exactly this
//! span. Outside it this calendar returns [`CalendarError::BeforeEpoch`] or
//! [`CalendarError::AfterSupportedRange`]; it does **not** fall back to an
//! arithmetic rule, because a computed month presented as an Umm al-Qurā
//! month would be a fabrication. Callers who want a computed answer outside
//! the range should ask [`crate::islamic_civil`] for one and say so.
//!
//! Note that ICU itself *does* fall back to the civil calculation outside
//! 1300–1600, so a caller comparing this crate against ICU outside the range
//! is comparing a refusal against a guess.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::tabular::{ERA, IslamicDate};

/// The machine identifier CLDR uses for this calendar.
pub const ID: CalendarId = CalendarId("islamic-umalqura");

/// The first Hijri year the official table covers.
pub const FIRST_YEAR: i64 = 1_300;

/// The last Hijri year the official table covers.
pub const LAST_YEAR: i64 = 1_600;

/// The fixed day of 1 Muḥarram 1300 AH, which is 1882-11-12 Gregorian.
pub const EPOCH: Rd = Rd(687_337);

/// Month lengths, one year per entry, `FIRST_YEAR` first.
///
/// Bit `n` is set when month `n + 1` of that year has 30 days. Bit 0 is
/// Muḥarram; bit 11 is Dhū al-Ḥijja.
pub const MONTH_LENGTH_MASKS: [u16; 301] = [
    0x555, 0x2AB, 0x937, 0x2B6, 0x576, 0x36C, 0xB55, 0xAAA, // 1300-1307 AH
    0x956, 0x49E, 0x95D, 0x2BA, 0x5B5, 0x3AA, 0xB4B, 0xA96, // 1308-1315 AH
    0x52E, 0x2AD, 0x56D, 0xB5A, 0x752, 0xF25, 0xE8A, 0xD16, // 1316-1323 AH
    0xA56, 0xAB5, 0x6B4, 0xDA9, 0xB92, 0xB25, 0x64B, 0xA9B, // 1324-1331 AH
    0x35A, 0x6D9, 0x5D4, 0xDA5, 0xD4A, 0xA95, 0x536, 0x975, // 1332-1339 AH
    0x2F4, 0x6E9, 0x6D4, 0x6A9, 0x535, 0x25D, 0x4BD, 0x9BA, // 1340-1347 AH
    0x3B4, 0xB69, 0xB2A, 0xA55, 0x4AD, 0xA5D, 0x2DA, 0x6D9, // 1348-1355 AH
    0xEAA, 0xE94, 0xD2A, 0xC56, 0x4AE, 0xA6D, 0x56A, 0xD55, // 1356-1363 AH
    0xD4A, 0xA93, 0x52B, 0xA5B, 0x53A, 0x6B5, 0xEA9, 0xD52, // 1364-1371 AH
    0xD29, 0xA55, 0x4AD, 0x56D, 0xAEA, 0x6E4, 0xED1, 0xDA2, // 1372-1379 AH
    0xAAA, 0x95A, 0x2DA, 0x5B9, 0xBB2, 0x764, 0x6C9, 0x555, // 1380-1387 AH
    0x2AB, 0x4DB, 0xABA, 0x5B4, 0xDA9, 0xD52, 0xAA5, 0x92D, // 1388-1395 AH
    0x26D, 0x8ED, 0x2DA, 0xAD5, 0xAA5, 0xA4B, 0x497, 0x937, // 1396-1403 AH
    0x2B6, 0x975, 0xD69, 0xD52, 0xC95, 0x92B, 0x25B, 0x4DB, // 1404-1411 AH
    0x9D5, 0x5D2, 0xDA5, 0xD4A, 0xA95, 0x54D, 0xAAD, 0x3AA, // 1412-1419 AH
    0xBD2, 0xBC4, 0xB89, 0xA95, 0x52D, 0x5AD, 0xB6A, 0x6D4, // 1420-1427 AH
    0xDC9, 0xD92, 0xAA6, 0x956, 0x2AE, 0x56D, 0x36A, 0xB55, // 1428-1435 AH
    0xAAA, 0x94D, 0x49D, 0x95D, 0x2BA, 0x5B5, 0x5AA, 0xD55, // 1436-1443 AH
    0xA9A, 0x92E, 0x26E, 0x55D, 0xADA, 0x6D4, 0x6A5, 0xB27, // 1444-1451 AH
    0xA4D, 0x4AD, 0x56D, 0xB5A, 0x754, 0xF49, 0xE92, 0xD26, // 1452-1459 AH
    0xA56, 0x356, 0x6B5, 0xBAA, 0xB92, 0xB25, 0x68B, 0xA9B, // 1460-1467 AH
    0x55A, 0xADA, 0x5B4, 0xDA9, 0xB52, 0xA9A, 0x536, 0x276, // 1468-1475 AH
    0x575, 0xAF2, 0x6D4, 0x6A9, 0x555, 0x2AD, 0x4BD, 0x9BA, // 1476-1483 AH
    0x574, 0xB69, 0xB52, 0xA95, 0x52D, 0xA5D, 0x4DA, 0xAD9, // 1484-1491 AH
    0x6B2, 0xE95, 0xE2A, 0xC96, 0x92E, 0xAAD, 0x56A, 0xD65, // 1492-1499 AH
    0xD4A, 0xD15, 0x62B, 0xC5B, 0x53A, 0x6B5, 0xDB2, 0xD64, // 1500-1507 AH
    0xD29, 0xA55, 0x4AD, 0x96D, 0xAEA, 0x6E8, 0xED1, 0xDA4, // 1508-1515 AH
    0xD4A, 0xA6A, 0x2DA, 0x5B9, 0xB72, 0xB68, 0x6D1, 0x655, // 1516-1523 AH
    0x4AB, 0x95B, 0x2BA, 0x5B5, 0xDA9, 0xD52, 0xCA6, 0x94E, // 1524-1531 AH
    0x46E, 0x95D, 0x4DA, 0xAD5, 0xAAA, 0xA4D, 0x49B, 0x937, // 1532-1539 AH
    0x4B6, 0x975, 0xD6A, 0xD52, 0xAA5, 0x94B, 0x2AB, 0x55B, // 1540-1547 AH
    0xAD9, 0x5D2, 0xDC5, 0xD92, 0xB25, 0x555, 0xAB5, 0x5B4, // 1548-1555 AH
    0xBA9, 0x7A2, 0x745, 0x593, 0xAAB, 0x4D6, 0x9D6, 0x5D2, // 1556-1563 AH
    0xBA5, 0xB4A, 0xA95, 0x4AD, 0x15D, 0x2DD, 0x9DA, 0x5B4, // 1564-1571 AH
    0x5A9, 0x52D, 0x25B, 0x8B7, 0x176, 0x56D, 0xB6A, 0xACA, // 1572-1579 AH
    0xA96, 0x52B, 0x15B, 0x2BB, 0x5B6, 0xDAA, 0xB94, 0xD46, // 1580-1587 AH
    0xA8D, 0x52D, 0xA9D, 0x55A, 0x755, 0x749, 0xF13, 0xE4A, // 1588-1595 AH
    0xA96, 0x556, 0x6B5, 0xBAA, 0xB94, // 1596-1600 AH
];

/// How many years the table covers.
pub const YEARS: usize = MONTH_LENGTH_MASKS.len();

/// The fixed day of 1 Muḥarram of each tabulated year, with one extra entry
/// for the day after the table ends.
///
/// Accumulating once at compile time turns `from_fixed` into a binary search
/// and `to_fixed` into two lookups.
const YEAR_STARTS: [i32; YEARS + 1] = build_year_starts();

/// Accumulate the year starts from the month-length masks.
const fn build_year_starts() -> [i32; YEARS + 1] {
    let mut starts = [0i32; YEARS + 1];
    starts[0] = EPOCH.0 as i32;
    let mut index = 0;
    while index < YEARS {
        starts[index + 1] = starts[index] + year_length_from_mask(MONTH_LENGTH_MASKS[index]) as i32;
        index += 1;
    }
    starts
}

/// The number of days a mask describes: 354 plus one for each long month
/// beyond the six a 29/30 alternation would give.
const fn year_length_from_mask(mask: u16) -> u16 {
    let mut total = 0u16;
    let mut month = 0;
    while month < 12 {
        total += if mask & (1 << month) == 0 { 29 } else { 30 };
        month += 1;
    }
    total
}

/// The earliest fixed day this calendar converts: 1 Muḥarram 1300 AH.
pub const EARLIEST: Rd = EPOCH;

/// The latest fixed day this calendar converts: the last day of 1600 AH,
/// which is 2174-11-25 Gregorian.
pub const LATEST: Rd = Rd(YEAR_STARTS[YEARS] as i64 - 1);

/// Whether the table covers `year`.
#[must_use]
pub const fn covers_year(year: i64) -> bool {
    year >= FIRST_YEAR && year <= LAST_YEAR
}

/// The month-length mask of `year`, or `None` outside the tabulated range.
#[must_use]
pub const fn month_length_mask(year: i64) -> Option<u16> {
    if covers_year(year) {
        Some(MONTH_LENGTH_MASKS[(year - FIRST_YEAR) as usize])
    } else {
        None
    }
}

/// The number of days in `month` of `year`.
///
/// `None` when the year is outside the table or the month outside `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if month == 0 || month > 12 {
        return None;
    }
    match month_length_mask(year) {
        None => None,
        Some(mask) => Some(if mask & (1 << (month - 1)) == 0 {
            29
        } else {
            30
        }),
    }
}

/// The number of days in `year`, or `None` outside the tabulated range.
#[must_use]
pub const fn days_in_year(year: i64) -> Option<u16> {
    match month_length_mask(year) {
        None => None,
        Some(mask) => Some(year_length_from_mask(mask)),
    }
}

/// Days elapsed in `year` before the first of `month`.
const fn days_before_month(mask: u16, month: u8) -> i64 {
    let mut total = 0i64;
    let mut index = 0;
    while index < month as usize - 1 {
        total += if mask & (1 << index) == 0 { 29 } else { 30 };
        index += 1;
    }
    total
}

/// The fixed day of an Umm al-Qurā date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside 1300–1600 AH, or
/// [`CalendarError::MonthOutOfRange`] / [`CalendarError::DayOutOfRange`] for
/// a month or day the table does not have.
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    let mask = match month_length_mask(year) {
        None => return Err(CalendarError::YearOutOfRange),
        Some(mask) => mask,
    };
    let length = match days_in_month(year, month) {
        None => return Err(CalendarError::MonthOutOfRange),
        Some(length) => length,
    };
    if day == 0 || day > length {
        return Err(CalendarError::DayOutOfRange);
    }
    let start = YEAR_STARTS[(year - FIRST_YEAR) as usize] as i64;
    Ok(Rd(start + days_before_month(mask, month) + day as i64 - 1))
}

/// The Umm al-Qurā year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] before 1300 AH or
/// [`CalendarError::AfterSupportedRange`] after 1600 AH. The table is the
/// calendar, so there is nothing to return outside it.
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    let target = rd.0 as i32;
    let mut low = 0usize;
    let mut high = YEARS - 1;
    while low < high {
        let middle = low + (high - low).div_ceil(2);
        if YEAR_STARTS[middle] <= target {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    let year = FIRST_YEAR + low as i64;
    let mask = MONTH_LENGTH_MASKS[low];
    let mut within = rd.0 - YEAR_STARTS[low] as i64;
    let mut month = 1u8;
    while month < 12 {
        let length = if mask & (1 << (month - 1)) == 0 {
            29
        } else {
            30
        };
        if within < length {
            break;
        }
        within -= length;
        month += 1;
    }
    Ok((year, month, (within + 1) as u8))
}

/// The Umm al-Qurā calendar of Saudi Arabia.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IslamicUmmAlQuraCalendar;

impl Calendar for IslamicUmmAlQuraCalendar {
    type Date = IslamicDate;

    /// The Islamic day begins at sunset, which is also why the month begins
    /// with a crescent seen after one.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Hijri (Umm al-Qura)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            // The table was derived from astronomical criteria, but this
            // implementation performs no astronomy: it reads a published
            // table, which is what makes it exact.
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
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
    use crate::civil;
    use crate::islamic_civil::IslamicCivilCalendar;

    /// A Hijri year, month and day paired with the Gregorian year, month and
    /// day it is published against.
    type PublishedPair = (i64, u8, u8, (i64, u8, u8));

    /// Dates published by the Saudi authorities and widely reported, used to
    /// check the extracted table rather than to derive it.
    const PUBLISHED: [PublishedPair; 6] = [
        (1_300, 1, 1, (1882, 11, 12)),
        (1_445, 1, 1, (2023, 7, 19)),
        (1_445, 9, 1, (2024, 3, 11)),
        (1_445, 10, 1, (2024, 4, 10)),
        (1_446, 9, 1, (2025, 3, 1)),
        (1_447, 1, 1, (2025, 6, 26)),
    ];

    #[test]
    fn the_published_dates_are_reproduced() {
        for (year, month, day, gregorian) in PUBLISHED {
            let rd = to_fixed(year, month, day).expect("inside the table");
            assert_eq!(civil::from_rd(rd), gregorian, "{year}-{month}-{day} AH");
            assert_eq!(from_fixed(rd), Ok((year, month, day)));
        }
    }

    #[test]
    fn the_table_covers_exactly_thirteen_hundred_to_sixteen_hundred() {
        assert_eq!(YEARS, 301);
        assert_eq!(FIRST_YEAR, 1_300);
        assert_eq!(LAST_YEAR, 1_600);
        assert!(covers_year(1_300));
        assert!(covers_year(1_600));
        assert!(!covers_year(1_299));
        assert!(!covers_year(1_601));
        assert_eq!(EARLIEST, EPOCH);
        assert_eq!(civil::from_rd(EARLIEST), (1882, 11, 12));
        assert_eq!(civil::from_rd(LATEST), (2174, 11, 25));
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
        assert_eq!(to_fixed(1_299, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(1_601, 1, 1), Err(CalendarError::YearOutOfRange));
    }

    #[test]
    fn the_calendar_round_trips_over_every_day_it_covers() {
        let calendar = IslamicUmmAlQuraCalendar;
        for rd in EARLIEST.0..=LATEST.0 {
            let date = calendar.from_fixed(Rd(rd)).expect("inside the table");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "RD {rd}");
        }
    }

    #[test]
    fn every_month_has_twenty_nine_or_thirty_days() {
        for year in FIRST_YEAR..=LAST_YEAR {
            for month in 1..=12u8 {
                let length = days_in_month(year, month).expect("in the table");
                assert!((29..=30).contains(&length), "{year}-{month} was {length}");
            }
        }
    }

    #[test]
    fn every_year_has_between_three_hundred_and_fifty_three_and_three_hundred_and_fifty_five_days()
    {
        for year in FIRST_YEAR..=LAST_YEAR {
            let length = days_in_year(year).expect("in the table");
            assert!((353..=355).contains(&length), "year {year} was {length}");
        }
        assert_eq!(days_in_year(1_299), None);
        assert_eq!(days_in_month(1_299, 1), None);
        assert_eq!(days_in_month(1_445, 13), None);
    }

    #[test]
    fn the_year_starts_accumulate_to_the_last_tabulated_day() {
        let mut running = EPOCH.0;
        for year in FIRST_YEAR..=LAST_YEAR {
            assert_eq!(
                to_fixed(year, 1, 1),
                Ok(Rd(running)),
                "1 Muharram {year} AH"
            );
            running += days_in_year(year).expect("in the table") as i64;
        }
        assert_eq!(LATEST.0, running - 1);
    }

    #[test]
    fn the_table_stays_close_to_the_arithmetic_calendar_without_matching_it() {
        // The two calendars answer different questions, so they disagree
        // often; what matters is that the disagreement stays small, which is
        // what tells us the table is a real lunar calendar and not noise.
        let arithmetic = IslamicCivilCalendar;
        let mut disagreements = 0u32;
        let mut months = 0u32;
        for year in FIRST_YEAR..=LAST_YEAR {
            for month in 1..=12u8 {
                let table = to_fixed(year, month, 1).expect("in the table");
                let computed = arithmetic
                    .to_fixed(IslamicDate {
                        year,
                        month,
                        day: 1,
                    })
                    .expect("in range");
                let difference = (table.0 - computed.0).abs();
                assert!(difference <= 3, "{year}-{month} differs by {difference}");
                months += 1;
                if difference != 0 {
                    disagreements += 1;
                }
            }
        }
        assert_eq!(months, 301 * 12);
        // Measured, not asserted away. Over the 3 612 months of the table
        // the tabular civil calendar names a different first day for 1 421
        // of them, 39.3%, and never by more than three days.
        assert_eq!(disagreements, 1_421, "of {months} months");
    }

    #[test]
    fn fields_round_trip_through_the_generic_interface() {
        let calendar = IslamicUmmAlQuraCalendar;
        for rd in (EARLIEST.0..=LATEST.0).step_by(101) {
            let date = calendar.from_fixed(Rd(rd)).expect("in the table");
            let fields = calendar.to_fields(date).expect("describable");
            assert_eq!(calendar.from_fields(&fields), Ok(date));
        }
        assert_eq!(
            calendar.from_fields(&DateFields::ymd_leap_month(1_445, 1, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(1_445, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }
}
