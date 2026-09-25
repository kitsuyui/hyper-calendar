//! The Rumi calendar of the Ottoman Empire and the early Republic of Turkey.
//!
//! The "Roman" calendar: the Julian months and days, with a year that
//! began on 1 Mart and was numbered by the Hijri era as it stood in 1840,
//! kept from the Tanzimat to the end of 1925. It is the Julian calendar
//! with two changes and one event:
//!
//! * The year turned on 1 Mart, so Kânûn-ı Sânî and Şubat — January and
//!   February — belong to the year that began the March before.
//! * The year number is the Julian one less 584: 1 Mart 1256 was 1 March
//!   1840 (Julian), 13 March 1840 (Gregorian), the day the calendar was
//!   adopted for civil use. The difference was the lunar Hijri count's at
//!   the time, and the solar calendar kept it constant thereafter.
//! * In 1917 the calendar was moved to the Gregorian days without changing
//!   the year number: after 15 Şubat 1332, 28 February 1917, the next day
//!   was 1 Mart 1333, 1 March 1917. The year 1333 ran ten months, to
//!   31 December, and from 1 Kânûn-ı Sânî 1334 — 1 January 1918 — the year
//!   turned with the Gregorian one, as the Gregorian year less 584, until
//!   the era was abandoned for 1926.
//!
//! The months are numbered here as the Julian and Gregorian ones are,
//! Kânûn-ı Sânî first, so that a Rumi date reads as a Julian or Gregorian
//! date with another year number; [`RumiDate::fiscal_month`] gives the
//! position in the fiscal year that began with Mart. The month names are
//! the Ottoman ones in Arabic script, as the source tabulates them; the
//! Latin forms are a locale's and live in `hc-i18n`. The 1945 renaming of
//! four months (Ekim, Kasım, Aralık, Ocak) postdates the calendar.
//!
//! The calendar converts only the days it was in use, 13 March 1840 to
//! 31 December 1925: a Rumi date outside them was never written, and the
//! fiscal-only use before 1840 kept a different, lunar-corrected count
//! that this module does not model.
//!
//! Source: Wikipedia, "Rumi calendar", retrieved 2026-09-22: the 1840
//! adoption, the 584-year difference, the months table, and the 1917
//! conversion table adapted from Richard B. Rose, which the tests
//! reproduce.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{gregorian, julian};

/// How far the Julian or Gregorian year runs ahead of the Rumi year, for
/// the months from Mart on.
pub const YEAR_OFFSET: i64 = 584;

/// The era code.
pub const ERA: &str = "Rumi";

/// The first year the calendar was kept: 1256, from 1 Mart.
pub const MIN_YEAR: i64 = 1_256;

/// The last year the calendar was kept: 1341, to 31 Kânûn-ı Evvel.
pub const MAX_YEAR: i64 = 1_341;

/// The year of ten months, 1 Mart to 31 Kânûn-ı Evvel 1333, the first on
/// the Gregorian days.
pub const REFORM_YEAR: i64 = 1_333;

/// The twelve months in Ottoman Turkish, Arabic script, Kânûn-ı Sânî
/// first. Source: the months table of Wikipedia, "Rumi calendar",
/// retrieved 2026-09-22.
pub const MONTHS: [&str; 12] = [
    "كانون ثانی",
    "شباط",
    "مارت",
    "نیسان",
    "مایس",
    "حزیران",
    "تموز",
    "اغستوس",
    "ایلول",
    "تشرین اول",
    "تشرین ثانی",
    "كانون اول",
];

/// The first day on the Gregorian months, 1 Mart 1333: 1 March 1917.
pub const CUTOVER: Rd = match gregorian::to_fixed(REFORM_YEAR + YEAR_OFFSET, 3, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Wikipedia, \"Rumi calendar\", retrieved 2026-09-22: adopted for civil use on 1 Mart 1256, \
    13 March 1840 Gregorian, and abandoned for 1926, so kept to 31 December 1925";

/// The earliest day this implementation converts, 1 Mart 1256: 1 March
/// 1840 in the Julian calendar.
pub const EARLIEST: Rd = match julian::to_fixed(MIN_YEAR + YEAR_OFFSET, 3, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The latest day this implementation converts, 31 Kânûn-ı Evvel 1341:
/// 31 December 1925.
pub const LATEST: Rd = match gregorian::to_fixed(MAX_YEAR + YEAR_OFFSET, 12, 31) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Whether the days of `year` are Gregorian: from the reform year on.
#[must_use]
pub const fn is_on_gregorian_days(year: i64) -> bool {
    year >= REFORM_YEAR
}

/// The Julian or Gregorian year the days of `month` of Rumi `year` are
/// numbered in: the year plus 584, or plus 585 for the January and
/// February that end a fiscal year before the reform.
#[must_use]
pub const fn western_year(year: i64, month: u8) -> i64 {
    if is_on_gregorian_days(year) || month >= 3 {
        year + YEAR_OFFSET
    } else {
        year + YEAR_OFFSET + 1
    }
}

/// Whether Şubat of `year` has 29 days. The reform year has no Şubat.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    if year == REFORM_YEAR {
        false
    } else if is_on_gregorian_days(year) {
        gregorian::is_leap_year(western_year(year, 2))
    } else {
        julian::is_leap_year(western_year(year, 2))
    }
}

/// The number of days in `month` of `year`, or `None` when the month is
/// not in `1..=12` or did not exist in that year: Kânûn-ı Sânî and Şubat
/// of 1333, the ten-month year.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if month < 1 || month > 12 {
        return None;
    }
    if month <= 2 && year == REFORM_YEAR {
        return None;
    }
    if year == REFORM_YEAR - 1 && month == 2 {
        // 15 Şubat 1332 was the last Julian day.
        return Some(15);
    }
    if is_on_gregorian_days(year) {
        gregorian::days_in_month(western_year(year, month), month)
    } else {
        julian::days_in_month(western_year(year, month), month)
    }
}

/// The fixed day of a Rumi date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`], [`CalendarError::MonthOutOfRange`] for a
/// month that did not exist in the year, or
/// [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    if let Err(error) = crate::common::check_day(day, days_in_month(year, month)) {
        return Err(error);
    }
    if is_on_gregorian_days(year) {
        gregorian::to_fixed(western_year(year, month), month, day)
    } else {
        julian::to_fixed(western_year(year, month), month, day)
    }
}

/// The Rumi year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    if rd.0 >= CUTOVER.0 {
        match gregorian::from_fixed(rd) {
            Ok((year, month, day)) => Ok((year - YEAR_OFFSET, month, day)),
            Err(error) => Err(error),
        }
    } else {
        match julian::from_fixed(rd) {
            Ok((year, month, day)) => {
                let rumi_year = if month >= 3 {
                    year - YEAR_OFFSET
                } else {
                    year - YEAR_OFFSET - 1
                };
                Ok((rumi_year, month, day))
            }
            Err(error) => Err(error),
        }
    }
}

/// A Rumi date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RumiDate {
    /// The Rumi year.
    pub year: i64,
    /// The month, 1 for Kânûn-ı Sânî through 12 for Kânûn-ı Evvel, as the
    /// Julian and Gregorian months are numbered.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl RumiDate {
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

    /// The month's place in the fiscal year that began with Mart: 1 for
    /// Mart through 10 for Kânûn-ı Evvel, 11 for Kânûn-ı Sânî and 12 for
    /// Şubat.
    #[must_use]
    pub const fn fiscal_month(self) -> u8 {
        (self.month + 9) % 12 + 1
    }

    /// Whether this date is on the Gregorian days, from 1 Mart 1333.
    #[must_use]
    pub const fn is_on_gregorian_days(self) -> bool {
        is_on_gregorian_days(self.year)
    }

    /// The Julian or Gregorian year this date's days are numbered in.
    #[must_use]
    pub const fn western_year(self) -> i64 {
        western_year(self.year, self.month)
    }
}

/// The Rumi calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RumiCalendar;

/// Twelve named months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for RumiCalendar {
    type Date = RumiDate;

    /// Kept from 13 March 1840 to 31 December 1925, which is also the whole of
    /// the range it converts.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(EARLIEST, LATEST, USAGE_SOURCE)
    }

    /// Twelve months, named in Ottoman Turkish, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("rumi"),
            english_name: "Rumi",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
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
        Ok(RumiDate { year, month, day })
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
        RumiDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_calendar_began_on_the_first_of_mart_1256() {
        // 13 March 1840 in the Gregorian calendar, 1 March in the Julian.
        assert_eq!(to_fixed(1256, 3, 1), Ok(EARLIEST));
        assert_eq!(EARLIEST, gregorian(1840, 3, 13));
        assert_eq!(julian::from_fixed(EARLIEST), Ok((1840, 3, 1)));
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        // Kânûn-ı Sânî and Şubat of 1256 end the year: they are January and
        // February 1841 (Julian).
        assert_eq!(to_fixed(1256, 2, 28), julian::to_fixed(1841, 2, 28));
        assert_eq!(from_fixed(gregorian(1841, 3, 12)), Ok((1256, 2, 28)));
        assert_eq!(from_fixed(gregorian(1841, 3, 13)), Ok((1257, 3, 1)));
        assert_eq!(to_fixed(1255, 12, 31), Err(CalendarError::YearOutOfRange));
    }

    #[test]
    fn the_1917_conversion_table_is_reproduced() {
        // Rose's table as the article adapts it: Rumi 1332 on the Julian
        // days to 15 Şubat, then 1 Mart 1333 on 1 March 1917.
        let table = [
            ((1332, 11, 1), (1916, 11, 14)),
            ((1332, 12, 1), (1916, 12, 14)),
            ((1332, 1, 1), (1917, 1, 14)),
            ((1332, 2, 1), (1917, 2, 14)),
            ((1332, 2, 14), (1917, 2, 27)),
            ((1332, 2, 15), (1917, 2, 28)),
            ((1333, 3, 1), (1917, 3, 1)),
            ((1333, 3, 3), (1917, 3, 3)),
            ((1333, 3, 13), (1917, 3, 13)),
            ((1333, 3, 14), (1917, 3, 14)),
            ((1333, 4, 1), (1917, 4, 1)),
            ((1333, 10, 1), (1917, 10, 1)),
            ((1333, 12, 31), (1917, 12, 31)),
            ((1334, 1, 1), (1918, 1, 1)),
            ((1334, 2, 28), (1918, 2, 28)),
        ];
        for ((year, month, day), (gy, gm, gd)) in table {
            assert_eq!(
                to_fixed(year, month, day),
                Ok(gregorian(gy, gm, gd)),
                "{year}-{month}-{day}"
            );
            assert_eq!(
                from_fixed(gregorian(gy, gm, gd)),
                Ok((year, month, day)),
                "{gy}-{gm}-{gd}"
            );
        }
        // 16 Şubat 1332 never came, and 1333 had no Kânûn-ı Sânî or Şubat.
        assert_eq!(to_fixed(1332, 2, 16), Err(CalendarError::DayOutOfRange));
        assert_eq!(days_in_month(1332, 2), Some(15));
        assert_eq!(to_fixed(1333, 1, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(1333, 2, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(days_in_month(1333, 2), None);
        assert_eq!(CUTOVER, gregorian(1917, 3, 1));
        // 1332 ran 352 days and 1333 ran 306.
        assert_eq!(
            to_fixed(1332, 2, 15).unwrap().0 - to_fixed(1332, 3, 1).unwrap().0 + 1,
            352
        );
        assert_eq!(
            to_fixed(1333, 12, 31).unwrap().0 - to_fixed(1333, 3, 1).unwrap().0 + 1,
            306
        );
    }

    #[test]
    fn the_year_turned_with_mart_before_the_reform_and_with_january_after() {
        // 1 January 1900 (Gregorian) was 20 Kânûn-ı Evvel 1315 (Julian
        // 20 December 1899); 1 March 1900 Julian, 14 March Gregorian,
        // opened 1316.
        assert_eq!(from_fixed(gregorian(1900, 1, 1)), Ok((1315, 12, 20)));
        assert_eq!(from_fixed(gregorian(1900, 3, 13)), Ok((1315, 2, 29)));
        assert_eq!(from_fixed(gregorian(1900, 3, 14)), Ok((1316, 3, 1)));
        assert!(is_leap_year(1315));
        assert_eq!(days_in_month(1315, 2), Some(29));
        // After the reform Şubat follows the Gregorian rule: 1340 is 1924.
        assert!(is_leap_year(1340));
        assert!(!is_leap_year(1339));
        assert_eq!(days_in_month(1340, 2), Some(29));
        assert_eq!(RumiDate::new(1340, 2, 29).unwrap().western_year(), 1924);
        assert!(!is_leap_year(1333));
        // Fiscal positions: Mart first, Şubat last.
        assert_eq!(
            RumiDate {
                year: 1300,
                month: 3,
                day: 1
            }
            .fiscal_month(),
            1
        );
        assert_eq!(
            RumiDate {
                year: 1300,
                month: 12,
                day: 1
            }
            .fiscal_month(),
            10
        );
        assert_eq!(
            RumiDate {
                year: 1300,
                month: 1,
                day: 1
            }
            .fiscal_month(),
            11
        );
        assert_eq!(
            RumiDate {
                year: 1300,
                month: 2,
                day: 1
            }
            .fiscal_month(),
            12
        );
        assert_eq!(
            RumiDate {
                year: 1300,
                month: 1,
                day: 1
            }
            .western_year(),
            1885
        );
    }

    #[test]
    fn the_calendar_ended_with_1341() {
        assert_eq!(to_fixed(1341, 12, 31), Ok(LATEST));
        assert_eq!(LATEST, gregorian(1925, 12, 31));
        assert_eq!(to_fixed(1342, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn every_day_the_calendar_was_kept_round_trips() {
        let calendar = RumiCalendar;
        let mut previous: Option<RumiDate> = None;
        for rd in EARLIEST.0..=LATEST.0 {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            if let Some(before) = previous {
                assert!(
                    before < date || (before.year == date.year && before.month > date.month),
                    "rd {rd}"
                );
            }
            previous = Some(date);
        }
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(1300, 3, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }
}
