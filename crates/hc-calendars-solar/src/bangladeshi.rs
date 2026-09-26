//! The Bangladeshi national calendar: the Bengali year of Bangladesh, a
//! fixed naming of the Gregorian day.
//!
//! The Bengali months under their Bengali names, Boishakh first, with the
//! year turning on 14 April, and every month opening on the same Gregorian
//! date each year because the one variable month, Falgun, holds the
//! Gregorian leap day. Years count in the Bengali era, *Bangabda*: year *N*
//! opens in Gregorian year *N* + 593, so 1 Boishakh 1433 is 14 April 2026.
//!
//! # Two sets of month lengths
//!
//! A committee under Muhammad Shahidullah proposed the calendar in 1966,
//! and Bangladesh adopted it in 1987: the first five months of 31 days,
//! the rest of 30, Falgun of 31 in a leap year. In 2018 the government
//! revised it, and the revision went into effect on 16 October 2019:
//! Ashvin gained a 31st day — Kartik began on Thursday 17 October — and
//! Falgun gave one up, 29 days, 30 in a leap year. Under it the national
//! days fall on the Bengali dates the source lists for them: 21 February
//! on 8 Falgun, 26 March on 12 Choitro, 14 April on 1 Boishakh, 5 August
//! on 21 Srabon and 16 December on 1 Poush.
//!
//! | Month | Bengali | 1394–1425 | From 1426 | Begins (from 1426) |
//! | --- | --- | --- | --- | --- |
//! | Boishakh | বৈশাখ | 31 | 31 | 14 April |
//! | Joishtho | জ্যৈষ্ঠ | 31 | 31 | 15 May |
//! | Asharh | আষাঢ় | 31 | 31 | 15 June |
//! | Srabon | শ্রাবণ | 31 | 31 | 16 July |
//! | Bhadro | ভাদ্র | 31 | 31 | 16 August |
//! | Ashvin | আশ্বিন | 30 | 31 | 16 September |
//! | Kartik | কার্তিক | 30 | 30 | 17 October |
//! | Ogrohayon | অগ্রহায়ণ | 30 | 30 | 16 November |
//! | Poush | পৌষ | 30 | 30 | 16 December |
//! | Magh | মাঘ | 30 | 30 | 15 January |
//! | Falgun | ফাল্গুন | 30 or 31 | 29 or 30 | 14 February |
//! | Choitro | চৈত্র | 30 | 30 | 15 March |
//!
//! Year 1426, 2019–2020, is the first under the revision: its first five
//! months are the same under both, and its Ashvin is the revision's.
//! Earlier years are the 1966 calendar's, proleptically before 1987.
//!
//! # The leap year
//!
//! The sources say Falgun gains its day "in every leap year" and not
//! which. It is taken here as the Gregorian leap year of the February that
//! Falgun spans — the year the Bengali year ends in — the one reading
//! under which 29 February always falls in Falgun and the months open on
//! the same Gregorian dates every year, as the revised table gives them.
//!
//! The calendar, its decrees and the Nanakshahi calendar built the same way
//! are written up in `docs/systems/fixed-solar-namings.md` in the
//! repository.
//!
//! Sources: Wikipedia, "Bangladeshi national calendar", retrieved
//! 2026-09-23, for the months, both sets of lengths, the start dates, the
//! 1966 committee, the 1987 adoption, the 2018 revision and the day it
//! took effect, and the national days' Bengali dates; Wikipedia, "Bengali
//! calendar", retrieved 2026-09-23 and 2026-09-26
//! (`wikipedia-bengali-calendars`), to which the first title now
//! redirects, for the same table and the era. The decree-level facts rest
//! on these secondary pages: the Bangla Academy's and the Cabinet
//! Division's notices of 1987 and 2019 were not read.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::gregorian;

/// How far the Common Era runs ahead of the Bengali era: year *N* opens on
/// 14 April of Gregorian year *N* + 593.
pub const YEAR_OFFSET: i64 = 593;

/// The first year under the 2019 revision.
pub const REVISED_FROM: i64 = 1_426;

/// The era code.
pub const ERA: &str = "bangabda";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// Whether `year` is a leap year, so that Falgun has its extra day: when
/// the Gregorian year the Bengali year ends in is one, since Falgun holds
/// 29 February.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year + YEAR_OFFSET + 1)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    let leap = if is_leap_year(year) { 1 } else { 0 };
    let revised = year >= REVISED_FROM;
    match month {
        1..=5 => Some(31),
        6 => Some(if revised { 31 } else { 30 }),
        7..=10 | 12 => Some(30),
        11 => Some(if revised { 29 + leap } else { 30 + leap }),
        _ => None,
    }
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The fixed day of 1 Boishakh of `year`, 14 April, without the range
/// check, so that the range itself can be expressed in terms of it.
const fn new_year_raw(year: i64) -> i64 {
    match gregorian::to_fixed(year + YEAR_OFFSET, 4, 14) {
        Ok(rd) => rd.0,
        // Unreachable for any year inside the Gregorian range, which the
        // bounds on this calendar guarantee.
        Err(_) => 0,
    }
}

/// The fixed day of 1 Boishakh of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(Rd(new_year_raw(year)))
}

/// Days elapsed in `year` before the first of `month`.
const fn days_before_month(year: i64, month: u8) -> i64 {
    let mut total = 0;
    let mut earlier = 1;
    while earlier < month {
        total += match days_in_month(year, earlier) {
            Some(days) => days as i64,
            None => 0,
        };
        earlier += 1;
    }
    total
}

/// The first Pohela Boishakh under the adopted calendar, 14 April 1987,
/// opening 1394. The source gives the year of adoption and no day.
pub const ADOPTED: Rd = match gregorian::to_fixed(1987, 4, 14) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Wikipedia, \"Bangladeshi national calendar\" [wikipedia-bengali-calendars], \
    retrieved 2026-09-23 and 2026-09-26: adopted by Bangladesh in 1987, the year only, so \
    the Pohela Boishakh of that year is taken; the 2019 revision in effect from 16 October \
    2019";

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(new_year_raw(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1) - 1);

/// The fixed day of a date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    match crate::common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => match new_year(year) {
            Err(error) => Err(error),
            Ok(start) => Ok(Rd(start.0 + days_before_month(year, month) + day as i64 - 1)),
        },
    }
}

/// The year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    // The year a day belongs to opens either in the Gregorian year of that
    // day or in the one before.
    let (gregorian_year, _, _) = gregorian::from_fixed(rd)?;
    let mut year = (gregorian_year - YEAR_OFFSET).min(MAX_YEAR);
    let mut start = new_year(year)?;
    if rd < start {
        year -= 1;
        start = new_year(year)?;
    }
    let mut remaining = rd.0 - start.0;
    let mut month = 1;
    while let Some(days) = days_in_month(year, month) {
        if remaining < i64::from(days) {
            break;
        }
        remaining -= i64::from(days);
        month += 1;
    }
    Ok((year, month, (remaining + 1) as u8))
}

/// A date in the Bangladeshi national calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BangladeshiDate {
    /// The year of the Bengali era.
    pub year: i64,
    /// The month, 1 for Boishakh through 12 for Choitro.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl BangladeshiDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        match to_fixed(year, month, day) {
            Err(error) => Err(error),
            Ok(_) => Ok(Self { year, month, day }),
        }
    }
}

/// The Bangladeshi national calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BangladeshiCalendar;

/// The twelve months in Bengali, Boishakh first, as the months table of
/// Wikipedia, "Bangladeshi national calendar", prints them; the Latin
/// forms are a locale's and live in `hc-i18n`.
pub const MONTHS: [&str; 12] = [
    "বৈশাখ",
    "জ্যৈষ্ঠ",
    "আষাঢ়",
    "শ্রাবণ",
    "ভাদ্র",
    "আশ্বিন",
    "কার্তিক",
    "অগ্রহায়ণ",
    "পৌষ",
    "মাঘ",
    "ফাল্গুন",
    "চৈত্র",
];

/// Twelve named months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for BangladeshiCalendar {
    type Date = BangladeshiDate;

    /// Kept since 1987, from the Pohela Boishakh of that year, the source
    /// giving no day; the 1966 lengths before it are proleptic.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(ADOPTED, USAGE_SOURCE)
    }

    /// Twelve months, named in Bengali, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("bangladeshi"),
            english_name: "Bangladeshi national calendar",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["bn"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(BangladeshiDate { year, month, day })
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
        BangladeshiDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_national_days_fall_on_their_bengali_dates() {
        // "National calendar dates for the national holidays of
        // Bangladesh": 21 February is 8 Falgun, 26 March 12 Choitro,
        // 14 April 1 Boishakh, 5 August 21 Srabon, 16 December 1 Poush.
        for gregorian_year in 2020..2040 {
            let (_, month, day) = from_fixed(gregorian(gregorian_year, 2, 21)).unwrap();
            assert_eq!((month, day), (11, 8), "{gregorian_year}");
            let (_, month, day) = from_fixed(gregorian(gregorian_year, 3, 26)).unwrap();
            assert_eq!((month, day), (12, 12), "{gregorian_year}");
            let (_, month, day) = from_fixed(gregorian(gregorian_year, 4, 14)).unwrap();
            assert_eq!((month, day), (1, 1), "{gregorian_year}");
            let (_, month, day) = from_fixed(gregorian(gregorian_year, 8, 5)).unwrap();
            assert_eq!((month, day), (4, 21), "{gregorian_year}");
            let (_, month, day) = from_fixed(gregorian(gregorian_year, 12, 16)).unwrap();
            assert_eq!((month, day), (9, 1), "{gregorian_year}");
        }
        assert_eq!(to_fixed(1433, 1, 1), Ok(gregorian(2026, 4, 14)));
    }

    #[test]
    fn every_month_begins_on_the_date_the_revised_table_gives() {
        let starts = [
            (1, 2026, 4, 14),
            (2, 2026, 5, 15),
            (3, 2026, 6, 15),
            (4, 2026, 7, 16),
            (5, 2026, 8, 16),
            (6, 2026, 9, 16),
            (7, 2026, 10, 17),
            (8, 2026, 11, 16),
            (9, 2026, 12, 16),
            (10, 2027, 1, 15),
            (11, 2027, 2, 14),
            (12, 2027, 3, 15),
        ];
        for (month, year, gregorian_month, gregorian_day) in starts {
            assert_eq!(
                to_fixed(1433, month, 1),
                Ok(gregorian(year, gregorian_month, gregorian_day)),
                "month {month}"
            );
        }
    }

    #[test]
    fn the_revision_took_effect_in_1426() {
        // Kartik 1426 began on Thursday 17 October 2019, a day later than
        // under the 1987 lengths, which the year before kept: Kartik 1425
        // began on 16 October 2018.
        assert_eq!(to_fixed(1426, 7, 1), Ok(gregorian(2019, 10, 17)));
        assert_eq!(gregorian(2019, 10, 17).0.rem_euclid(7), 4, "a Thursday");
        assert_eq!(to_fixed(1425, 7, 1), Ok(gregorian(2018, 10, 16)));
        assert_eq!(days_in_month(1425, 6), Some(30));
        assert_eq!(days_in_month(1426, 6), Some(31));
        // Both keep Choitro on 15 March and the year on 14 April.
        assert_eq!(to_fixed(1425, 12, 1), Ok(gregorian(2019, 3, 15)));
        assert_eq!(to_fixed(1426, 1, 1), Ok(gregorian(2019, 4, 14)));
    }

    #[test]
    fn falgun_holds_the_gregorian_leap_day() {
        // 1430 ends in 2024: its Falgun spans 29 February and has 30 days
        // under the revision; 1431 does not.
        assert!(is_leap_year(1430));
        assert_eq!(days_in_month(1430, 11), Some(30));
        assert_eq!(from_fixed(gregorian(2024, 2, 29)), Ok((1430, 11, 16)));
        assert_eq!(days_in_month(1431, 11), Some(29));
        assert_eq!(days_in_year(1430), 366);
        // Under the 1987 lengths, 31: 1402 ends in 1996.
        assert!(is_leap_year(1402));
        assert_eq!(days_in_month(1402, 11), Some(31));
        assert_eq!(to_fixed(1403, 1, 1), Ok(gregorian(1996, 4, 14)));
        // The Gregorian century rule comes with it: 1506 ends in 2100.
        assert!(!is_leap_year(1506));
    }

    #[test]
    fn every_day_round_trips() {
        let calendar = BangladeshiCalendar;
        for rd in (EARLIEST.0..=EARLIEST.0 + 800_000).step_by(29) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
        }
        let start = to_fixed(1423, 1, 1).unwrap().0;
        for rd in start..start + 6 * 366 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
        for rd in LATEST.0 - 400..=LATEST.0 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn dates_outside_the_supported_range_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(1433, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(1431, 11, 30), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            BangladeshiCalendar.from_fields(&DateFields::ymd(1433, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }
}
