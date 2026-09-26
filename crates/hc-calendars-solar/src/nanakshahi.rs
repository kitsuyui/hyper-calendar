//! The Nanakshahi calendar of 2003, the Sikh solar calendar.
//!
//! The calendar Pal Singh Purewal designed and the Shiromani Gurdwara
//! Parbandhak Committee adopted in 2003, with the approval of the Akal
//! Takht, to replace the lunisolar Bikrami calendar for Sikh observances.
//! It is a naming of the Gregorian day, as the Indian national calendar is:
//! the year counts from the birth of Guru Nanak, so that year 1 is 1469 and
//! the year turns on 1 Chet, 14 March; five months of 31 days are followed
//! by seven of 30, and the last, Phaggan, gains a day in the Gregorian leap
//! years — 12 February to 13 March holds the 29th — so that every date of
//! the calendar falls on the same Gregorian date every year.
//!
//! | Month | Gurmukhi | Days | Begins |
//! | --- | --- | --- | --- |
//! | Chet | ਚੇਤ | 31 | 14 March |
//! | Vaisakh | ਵੈਸਾਖ | 31 | 14 April |
//! | Jeth | ਜੇਠ | 31 | 15 May |
//! | Harh | ਹਾੜ | 31 | 15 June |
//! | Sawan | ਸਾਵਣ | 31 | 16 July |
//! | Bhadon | ਭਾਦੋਂ | 30 | 16 August |
//! | Assu | ਅੱਸੂ | 30 | 15 September |
//! | Kattak | ਕੱਤਕ | 30 | 15 October |
//! | Maghar | ਮੱਘਰ | 30 | 14 November |
//! | Poh | ਪੋਹ | 30 | 14 December |
//! | Magh | ਮਾਘ | 30 | 13 January |
//! | Phaggan | ਫੱਗਣ | 30 or 31 | 12 February |
//!
//! # Which Nanakshahi
//!
//! This is the 2003 calendar, the one its supporters call the *Mool* — the
//! original — Nanakshahi. In 2010 the SGPC amended it so that the months
//! begin with the Bikrami *saṅkrāntis* and several observances return to
//! their lunar dates, and by 2014 it had reverted to the Bikrami calendar
//! entirely while still publishing it under the Nanakshahi name; that
//! calendar is the Vikrami solar year of `hindu-solar-vikrami` and the
//! lunar dates of `hindu-lunar`, not this module. A caller who wants the
//! SGPC's current dates wants those.
//!
//! Source: Wikipedia, "Nanakshahi calendar", retrieved 2026-09-22, for the
//! months table, the leap rule, the epoch, the 2003 adoption and the later
//! revisions.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::gregorian;

/// How far the Common Era runs ahead of the Nanakshahi era: year *N* opens
/// on 14 March of Gregorian year *N* + 1468.
pub const YEAR_OFFSET: i64 = 1_468;

/// The era code of the Nanakshahi era.
pub const ERA: &str = "NS";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// Whether `year` is a leap year, so that Phaggan has 31 days.
///
/// It is exactly when the Gregorian year the Nanakshahi year *ends* in is
/// one, since Phaggan holds 29 February.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year + YEAR_OFFSET + 1)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1..=5 => Some(31),
        6..=11 => Some(30),
        12 => Some(if is_leap_year(year) { 31 } else { 30 }),
        _ => None,
    }
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The fixed day of 1 Chet of `year`, which is 14 March.
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

/// The fixed day of 1 Chet of `year`, without the range check, so that the
/// range itself can be expressed in terms of it.
const fn new_year_raw(year: i64) -> i64 {
    match gregorian::to_fixed(year + YEAR_OFFSET, 3, 14) {
        Ok(rd) => rd.0,
        // Unreachable for any year inside the Gregorian range, which the
        // bounds on this calendar guarantee.
        Err(_) => 0,
    }
}

/// Days elapsed in the year before the first of `month`.
const fn days_before_month(month: u8) -> i64 {
    if month <= 5 {
        31 * (month as i64 - 1)
    } else {
        5 * 31 + 30 * (month as i64 - 6)
    }
}

/// The day the calendar was launched, 14 April 2003.
pub const LAUNCHED: Rd = match gregorian::to_fixed(2003, 4, 14) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The last day of Nanakshahi 541, 13 March 2010, the year before the
/// SGPC's amendment of 2010; the source gives the year only.
pub const LAST_KEPT: Rd = match gregorian::to_fixed(2010, 3, 13) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Wikipedia, \"Nanakshahi calendar\", retrieved 2026-09-22 and 2026-09-26: approved by the \
    SGPC in January 2003 and launched on 14 April 2003; amended in 2010 to follow the \
    Bikrami month starts, the year only, so the calendar year's end is taken; scrapped \
    entirely by 2014";

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(new_year_raw(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1) - 1);

/// The fixed day of a Nanakshahi date.
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
            Ok(start) => Ok(Rd(start.0 + days_before_month(month) + day as i64 - 1)),
        },
    }
}

/// The Nanakshahi year, month and day of a fixed day.
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
    let day_of_year = rd.0 - start.0 + 1;
    let (month, day) = if day_of_year <= 5 * 31 {
        ((day_of_year - 1) / 31 + 1, (day_of_year - 1) % 31 + 1)
    } else {
        let after_long = day_of_year - 5 * 31;
        if after_long <= 6 * 30 {
            ((after_long - 1) / 30 + 6, (after_long - 1) % 30 + 1)
        } else {
            // Phaggan, which alone can reach a 31st day.
            (12, after_long - 6 * 30)
        }
    };
    Ok((year, month as u8, day as u8))
}

/// A Nanakshahi date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NanakshahiDate {
    /// The year of the Nanakshahi era, counting from 1.
    pub year: i64,
    /// The month, 1 for Chet through 12 for Phaggan.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl NanakshahiDate {
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

    /// The Common Era year this Nanakshahi year opens in.
    #[must_use]
    pub const fn common_era_year(self) -> i64 {
        self.year + YEAR_OFFSET
    }
}

/// The Nanakshahi calendar of 2003.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NanakshahiCalendar;

/// The twelve months in Gurmukhi, Chet first, as the months table of
/// Wikipedia, "Nanakshahi calendar", retrieved 2026-09-22, prints them; the
/// Latin forms (Chet, Vaisakh, Jeth …) are a locale's and live in `hc-i18n`.
pub const MONTHS: [&str; 12] = [
    "ਚੇਤ",
    "ਵੈਸਾਖ",
    "ਜੇਠ",
    "ਹਾੜ",
    "ਸਾਵਣ",
    "ਭਾਦੋਂ",
    "ਅੱਸੂ",
    "ਕੱਤਕ",
    "ਮੱਘਰ",
    "ਪੋਹ",
    "ਮਾਘ",
    "ਫੱਗਣ",
];

/// Twelve named months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for NanakshahiCalendar {
    type Date = NanakshahiDate;

    /// The 2003 calendar was kept from its launch on 14 April 2003 until the
    /// SGPC's amendment of 2010 moved the month starts onto the Bikrami
    /// calendar; the amended and later calendars are `hindu-solar-vikrami`
    /// under this name, so the record ends with Nanakshahi 541.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(LAUNCHED, LAST_KEPT, USAGE_SOURCE)
    }

    /// Twelve months, named in Gurmukhi, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("nanakshahi"),
            english_name: "Nanakshahi",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["pa"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(NanakshahiDate { year, month, day })
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
        NanakshahiDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_year_turns_on_fourteen_march_and_counts_from_guru_nanaks_birth() {
        // The SGPC released the calendar on 1 Chet 535, 14 March 2003, and
        // the article's own example: 20 September 2026 is Nanakshahi 558.
        assert_eq!(to_fixed(535, 1, 1), Ok(gregorian(2003, 3, 14)));
        assert_eq!(from_fixed(gregorian(2026, 9, 20)), Ok((558, 7, 6)));
        assert_eq!(from_fixed(gregorian(2026, 3, 13)), Ok((557, 12, 30)));
        assert_eq!(from_fixed(gregorian(2026, 3, 14)), Ok((558, 1, 1)));
        assert_eq!(to_fixed(1, 1, 1), Ok(gregorian(1469, 3, 14)));
        assert_eq!(
            NanakshahiDate::new(558, 1, 1).unwrap().common_era_year(),
            2026
        );
    }

    #[test]
    fn every_month_begins_on_the_gregorian_date_of_the_table() {
        // 535 NS, 2003–2004, a common year.
        let starts = [
            (1, 2003, 3, 14),
            (2, 2003, 4, 14),
            (3, 2003, 5, 15),
            (4, 2003, 6, 15),
            (5, 2003, 7, 16),
            (6, 2003, 8, 16),
            (7, 2003, 9, 15),
            (8, 2003, 10, 15),
            (9, 2003, 11, 14),
            (10, 2003, 12, 14),
            (11, 2004, 1, 13),
            (12, 2004, 2, 12),
        ];
        for (month, year, gregorian_month, gregorian_day) in starts {
            assert_eq!(
                to_fixed(535, month, 1),
                Ok(gregorian(year, gregorian_month, gregorian_day)),
                "month {month}"
            );
        }
        // The gurpurab dates of the 2003 table: 23 Poh is 5 January and
        // 19 Magh is 31 January.
        assert_eq!(to_fixed(535, 10, 23), Ok(gregorian(2004, 1, 5)));
        assert_eq!(to_fixed(535, 11, 19), Ok(gregorian(2004, 1, 31)));
    }

    #[test]
    fn phaggan_holds_the_gregorian_leap_day() {
        // 555 NS ends on 13 March 2024, so its Phaggan spans 29 February
        // 2024 and has 31 days; 556 NS does not.
        assert!(is_leap_year(555));
        assert_eq!(days_in_month(555, 12), Some(31));
        assert_eq!(to_fixed(555, 12, 31), Ok(gregorian(2024, 3, 13)));
        assert_eq!(from_fixed(gregorian(2024, 2, 29)), Ok((555, 12, 18)));
        assert_eq!(to_fixed(556, 1, 1), Ok(gregorian(2024, 3, 14)));
        assert!(!is_leap_year(556));
        assert_eq!(days_in_month(556, 12), Some(30));
        assert_eq!(to_fixed(556, 12, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(days_in_year(555), 366);
        assert_eq!(days_in_year(556), 365);
        // The Gregorian century rule comes with it: 631 NS ends in 2100.
        assert!(!is_leap_year(631));
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        let calendar = NanakshahiCalendar;
        for rd in (EARLIEST.0..=EARLIEST.0 + 800_000).step_by(29) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
        }
        let start = to_fixed(554, 1, 1).unwrap().0;
        for rd in start..start + 3 * 365 + 1 {
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
        assert_eq!(to_fixed(535, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(
            NanakshahiCalendar.from_fields(&DateFields::ymd(535, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(NanakshahiCalendar.meta().id, CalendarId("nanakshahi"));
    }
}
