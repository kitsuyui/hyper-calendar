//! The modern Assyrian calendar.
//!
//! The Gregorian year under the Akkadian-derived month names of the Syriac
//! calendar, beginning on 1 Neesan = 1 April and counted from 4750 BC, so
//! that the Assyrian year is the Gregorian year plus [`YEAR_OFFSET`] from
//! April and one less before it: 1 April 2026 opened 6776, and the January
//! before it, Kanoon Treyana, was still 6775. Every month is the Gregorian
//! month of the same days — Neesan has April's thirty, Eshwat February's
//! twenty-eight or twenty-nine — so the calendar is exact wherever the
//! Gregorian one is, and its leap day falls in the eleventh month.
//!
//! The reckoning is a diaspora construction of the 1950s: a series of
//! articles in the Assyrian nationalist magazine *Gilgamesh* (Tehran) —
//! Nimrod Simono on the Akitu festival in 1952, then Jean Alkhas in the
//! April 1955 issue, number 34, who fixed 4750 BC as the epoch on the word
//! of an unnamed French archaeologist's cuneiform tablet — and it has been
//! the community's calendar since, printed on every Kha b-Neesan greeting.
//! It is **not** the calendar of the ancient Assyrians, who named years for
//! the *limmu* eponym officials and kept lunar months; nor the Seleucid era
//! from 312 BC in which Syriac-speaking Christians long dated their
//! documents. [`usage`](Calendar::usage) begins with the 1955 article that
//! fixed the epoch; earlier years are [`hc_calendar::Standing::Proleptic`].
//!
//! The month names are declared with the shape as [`MONTHS`], in the forms
//! the Assyrian International News Agency prints; the Syriac script and
//! the scholarly transliteration (Nīsān, ʾĪyār, Ḥzīrān, Tammūz, ʾĀb, ʾĪlūl,
//! Tešrīn Qḏīm and ʾḤrāy, Kānōn Qḏīm and ʾḤrāy, Šḇāṭ, ʾĀḏar) are a locale's.
//!
//! # Sources
//!
//! * Peter BetBasoo, "The Assyrian Calendar", Assyrian International News
//!   Agency, posted 2001-01-01, `aina.org/releases/20120221025652.htm`,
//!   retrieved 2026-09-25: the epoch 4750 BC, "the Assyrian year is 4750 +
//!   the Gregorian Year ... if the date is after April 1, before that the
//!   year is one less", 1 Neesan on 1 April, and the twelve month names
//!   against the Gregorian months, which are the forms used here.
//! * Wikipedia, "Assyrian calendar", retrieved 2026-09-25: the *Gilgamesh*
//!   articles of 1952 and 1955 and the French archaeologist, which it
//!   cites from Robert Paulissian, "Assyrian and Babylonian New Year
//!   Celebrations (Part II)", *Journal of Assyrian Academic Studies* 13.2
//!   (1999), p. 35, and Sennacherib Daniel, "Modern Festival, Ancient
//!   Tradition", *Nakosha* 39 (2001), p. 3, neither read here; and the
//!   Seleucid era in earlier Assyrian use, which it cites from J. F.
//!   Coakley, *Robinson's Paradigms and Exercises in Syriac Grammar*, 6th
//!   ed. (2013), p. 148, not read.
//! * SyriacPress and PaleoJudaica, 1 April 2026, retrieved 2026-09-25: the
//!   year 6776 opening on that day.
//!
//! The system document is `docs/systems/assyrian.md`.
//!
//! # Exactness
//!
//! Exact — arithmetic, the Gregorian calendar with its months relabelled.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};

use crate::gregorian;

/// The calendar identifier.
pub const ID: &str = "assyrian";

/// The era code, A.Y., "Assyrian Year".
pub const ERA: &str = "ay";

/// What the era adds to the Gregorian year from 1 April: year 1 is
/// 4750 BC.
pub const YEAR_OFFSET: i64 = 4_750;

/// The earliest year this implementation converts: the era's first.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 999_999;

/// The year whose Kha b-Neesan the epoch was fixed for: April 1955.
pub const FIRST_YEAR_KEPT: i64 = 1955 + YEAR_OFFSET;

/// The twelve months from Neesan, in the forms AINA prints.
pub const MONTHS: [&str; 12] = [
    "Neesan",
    "Yar",
    "Khzeeran",
    "Tammuz",
    "Tabakh",
    "Eelool",
    "Tishrin Qamaya",
    "Tishrin Treyana",
    "Kanoon Qamaya",
    "Kanoon Treyana",
    "Eshwat",
    "Adar",
];

/// The Gregorian month each Assyrian month is: Neesan is April.
const GREGORIAN_MONTH: [u8; 12] = [4, 5, 6, 7, 8, 9, 10, 11, 12, 1, 2, 3];

/// The Gregorian year that `month` of Assyrian `year` falls in: the year
/// less the offset from Neesan to Kanoon Qamaya, one more from Kanoon
/// Treyana to Adar.
#[must_use]
pub const fn gregorian_year(year: i64, month: u8) -> i64 {
    if month >= 10 {
        year - YEAR_OFFSET + 1
    } else {
        year - YEAR_OFFSET
    }
}

/// Whether `year` has a 29 Eshwat: when the Gregorian year its February
/// falls in is a leap year.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(gregorian_year(year, 11))
}

/// The number of days in `month` of `year`, or `None` when `month` is not
/// in `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if month == 0 || month > 12 {
        return None;
    }
    gregorian::days_in_month(
        gregorian_year(year, month),
        GREGORIAN_MONTH[month as usize - 1],
    )
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The fixed day of Kha b-Neesan, 1 Neesan of `year`: 1 April of the
/// Gregorian year less [`YEAR_OFFSET`].
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    to_fixed(year, 1, 1)
}

/// The earliest fixed day this implementation converts: 1 Neesan 1, which
/// is 1 April 4750 BC in the proleptic Gregorian calendar.
pub const EARLIEST: Rd = match gregorian::to_fixed(MIN_YEAR - YEAR_OFFSET, 4, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The latest fixed day this implementation converts: 31 Adar of the last
/// year.
pub const LATEST: Rd = match gregorian::to_fixed(MAX_YEAR - YEAR_OFFSET + 1, 3, 31) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Wikipedia, \"Assyrian calendar\", retrieved 2026-09-25, citing Paulissian 1999 and \
    Daniel 2001, not read here: the epoch fixed by Jean Alkhas in *Gilgamesh* in April 1955, \
    so 1 Neesan 1955 is the first new year kept under it";

/// The first Kha b-Neesan under the epoch: 1 April 1955.
pub const FIRST_KEPT: Rd = match gregorian::to_fixed(FIRST_YEAR_KEPT - YEAR_OFFSET, 4, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of an Assyrian date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    if month == 0 || month > 12 {
        return Err(CalendarError::MonthOutOfRange);
    }
    gregorian::to_fixed(
        gregorian_year(year, month),
        GREGORIAN_MONTH[month as usize - 1],
        day,
    )
}

/// The Assyrian year, month and day of a fixed day.
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
    match gregorian::from_fixed(rd) {
        Err(error) => Err(error),
        Ok((year, month, day)) => {
            if month >= 4 {
                Ok((year + YEAR_OFFSET, month - 3, day))
            } else {
                Ok((year + YEAR_OFFSET - 1, month + 9, day))
            }
        }
    }
}

/// A date in the modern Assyrian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssyrianDate {
    /// The Assyrian year, counting from 1 in 4750 BC.
    pub year: i64,
    /// The month, 1 for Neesan through 12 for Adar.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl AssyrianDate {
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

    /// The Gregorian year this date falls in.
    #[must_use]
    pub const fn gregorian_year(self) -> i64 {
        gregorian_year(self.year, self.month)
    }

    /// The name of the month.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[self.month as usize - 1]
    }

    /// Whether this is Kha b-Neesan, the new year.
    #[must_use]
    pub const fn is_kha_b_neesan(self) -> bool {
        self.month == 1 && self.day == 1
    }
}

/// The modern Assyrian calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AssyrianCalendar;

/// Twelve named months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for AssyrianCalendar {
    type Date = AssyrianDate;

    /// The Gregorian rule, on the Gregorian year whose 29 February the
    /// Assyrian year contains.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    /// The Gregorian months from April under their Syriac names, and the
    /// seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// Kept since the article of April 1955 that fixed the epoch.
    fn usage(&self) -> Usage {
        Usage::since(FIRST_KEPT, USAGE_SOURCE)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Assyrian (modern)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["syr"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(AssyrianDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_era(ERA)
            .with_extra("gregorian-year", date.gregorian_year())
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        AssyrianDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::Standing;

    #[test]
    fn kha_b_neesan_is_the_first_of_april_and_the_year_is_gregorian_plus_4750() {
        // SyriacPress, 1 April 2026: the year 6776; AINA: 2012 was 6762.
        assert_eq!(new_year(6776), gregorian::to_fixed(2026, 4, 1));
        assert_eq!(new_year(6762), gregorian::to_fixed(2012, 4, 1));
        assert_eq!(
            from_fixed(gregorian::to_fixed(2019, 4, 1).unwrap()),
            Ok((6769, 1, 1))
        );
        let date = AssyrianDate::new(6776, 1, 1).unwrap();
        assert!(date.is_kha_b_neesan());
        assert_eq!(date.month_name(), "Neesan");
        assert_eq!(date.gregorian_year(), 2026);
    }

    #[test]
    fn january_to_march_belong_to_the_year_that_began_the_previous_april() {
        // AINA: "if the date is after April 1, before that the year is one
        // less", so 1 January 2026 is Kanoon Treyana 6775 and 31 March
        // 2026 is the last day of 6775.
        assert_eq!(
            from_fixed(gregorian::to_fixed(2026, 1, 1).unwrap()),
            Ok((6775, 10, 1))
        );
        assert_eq!(
            from_fixed(gregorian::to_fixed(2026, 3, 31).unwrap()),
            Ok((6775, 12, 31))
        );
        assert_eq!(
            to_fixed(6775, 12, 31).unwrap().0 + 1,
            new_year(6776).unwrap().0
        );
        assert_eq!(
            AssyrianDate::new(6775, 10, 1).unwrap().gregorian_year(),
            2026
        );
        assert_eq!(
            AssyrianDate::new(6775, 9, 1).unwrap().gregorian_year(),
            2025
        );
        assert_eq!(gregorian_year(6775, 9), 2025);
        assert_eq!(gregorian_year(6775, 10), 2026);
    }

    #[test]
    fn the_months_are_the_gregorian_ones_from_april_and_eshwat_takes_the_leap_day() {
        assert_eq!(MONTHS[0], "Neesan");
        assert_eq!(MONTHS[10], "Eshwat");
        assert_eq!(MONTHS[11], "Adar");
        assert_eq!(days_in_month(6776, 1), Some(30)); // April
        assert_eq!(days_in_month(6776, 2), Some(31)); // May
        assert_eq!(days_in_month(6776, 12), Some(31)); // March 2027
        // 6773 runs from April 2023 to March 2024, and its Eshwat is
        // February 2024, which has 29 days.
        assert!(is_leap_year(6773));
        assert_eq!(days_in_month(6773, 11), Some(29));
        assert_eq!(days_in_year(6773), 366);
        assert!(!is_leap_year(6774));
        assert_eq!(days_in_month(6774, 11), Some(28));
        assert_eq!(days_in_year(6774), 365);
        // 6749 has February 2000 and 6649 has February 1900.
        assert!(is_leap_year(6749));
        assert!(!is_leap_year(6649));
        assert_eq!(days_in_month(6776, 0), None);
        assert_eq!(days_in_month(6776, 13), None);
    }

    #[test]
    fn the_epoch_is_4750_bc_and_the_reckoning_dates_from_1955() {
        assert_eq!(gregorian::from_fixed(EARLIEST), Ok((-4749, 4, 1)));
        assert_eq!(new_year(1), Ok(EARLIEST));
        assert_eq!(FIRST_KEPT, gregorian::to_fixed(1955, 4, 1).unwrap());
        assert_eq!(FIRST_YEAR_KEPT, 6705);
        assert_eq!(
            AssyrianCalendar.standing(Rd(FIRST_KEPT.0 - 1)),
            Standing::Proleptic
        );
        assert_eq!(AssyrianCalendar.standing(FIRST_KEPT), Standing::InUse);
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (-1_500_000..=2_000_000).step_by(97) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
            let (gy, gm, gd) = gregorian::from_fixed(Rd(rd)).unwrap();
            assert_eq!(gd, day);
            assert_eq!(GREGORIAN_MONTH[month as usize - 1], gm);
            assert_eq!(gregorian_year(year, month), gy);
        }
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = new_year(6770).unwrap().0;
        let end = new_year(6778).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = AssyrianCalendar;
        for rd in (-500_000..=1_000_000).step_by(1_009) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
            assert_eq!(
                fields.extra.get("gregorian-year"),
                Some(date.gregorian_year())
            );
        }
        assert_eq!(calendar.meta().id, CalendarId(ID));
        assert_eq!(calendar.meta().year_kind, YearKind::EpochForward);
        assert_eq!(calendar.usage(), Usage::since(FIRST_KEPT, USAGE_SOURCE));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(6776, 1, 1).with_era("BC")),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn dates_outside_the_supported_range_are_refused() {
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(to_fixed(6776, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(6776, 0, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(6776, 1, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(6774, 11, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            gregorian::from_fixed(LATEST),
            Ok((MAX_YEAR - YEAR_OFFSET + 1, 3, 31))
        );
        assert_eq!(
            AssyrianCalendar.from_fields(&DateFields::ymd_leap_month(6776, 4, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
    }
}
