//! The Thai solar calendar (Buddhist Era).
//!
//! Gregorian structure with the year number raised by [`YEAR_OFFSET`]: 2026
//! CE is 2569 BE. Thailand adopted the Gregorian month structure in 1889 and
//! moved the start of the year to 1 January in 1941. The calendar here is
//! the modern Thai civil calendar, whose year is the Gregorian one.
//!
//! # The year as it was printed before 1941
//!
//! The history behind this — the solar reckoning from 1 April 1889 in the
//! Rattanakosin era, the Buddhist Era from 1 April 2456 (1913), and the
//! Calendar Years Act of 2483 that began 2484 on 1 January 1941 — is
//! written up with its sources in `docs/systems/thai-lunar.md` in the
//! repository, beside the lunar calendar. This page summarises it and
//! states the code's own facts.
//!
//! Before 1941 the months were the same but the year began on 1 April, so
//! a Thai document of those years dates January to March a year lower
//! than [`BuddhistCalendar`] does — 1 January 1920 was printed 2462, not
//! 2463 — and before 1 April 1913 in another era altogether, RS 108 to
//! 131. [`printed_year`] gives the era and year a document of the time
//! would have printed for a day, and [`printed_to_fixed`] reads such a
//! dateline back; 2483 ran from 1 April to 31 December 1940 and has no
//! January to March. Like [`crate::year_style`], it changes the year
//! number only. The month and day are the Gregorian ones throughout, and
//! the calendar itself keeps the modern year, whose every year begins on
//! 1 January.
//!
//! # What this deliberately does not do
//!
//! * It does not model the lunar reckoning in the Chulasakarat era that
//!   official use left on 1 April 1889; [`printed_year`] refuses an earlier
//!   day.
//! * It does not model the Burmese, Sinhalese, Khmer or Lao Buddhist eras,
//!   which use the same era name with different epochs and, in several
//!   cases, a lunisolar year.
//! * It says nothing about where the Buddhist era's own epoch comes from.
//!   The parinirvana is dated 544 or 543 BCE depending on the tradition, and
//!   the Thai reckoning is the one that makes the offset 543.
//!
//! **Sources:** the proclamation ให้ใช้วันอย่างใหม่ of 1888, in force from
//! 1 April RS 108; the proclamation of 21 February RS 131 adopting the
//! Buddhist Era from 1 April 2456; พระราชบัญญัติปีปฏิทิน พุทธศักราช ๒๔๘๓,
//! Royal Gazette vol. 57, section ก, p. 419, 17 September 1940.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, gregorian};

/// How far the Buddhist Era runs ahead of the Common Era.
pub const YEAR_OFFSET: i64 = 543;

/// The era code of the Buddhist Era.
pub const ERA: &str = "BE";

/// The earliest year this implementation converts, so that the year number
/// is never zero or negative.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = gregorian::MAX_YEAR + YEAR_OFFSET;

/// Whether `year` is a leap year, by the Gregorian rule applied to the
/// corresponding Common Era year.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year - YEAR_OFFSET)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    gregorian::days_in_month(year - YEAR_OFFSET, month)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    gregorian::days_in_year(year - YEAR_OFFSET)
}

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "พระราชบัญญัติปีปฏิทิน พุทธศักราช ๒๔๘๓, Royal Gazette vol. 57, section ก, p. 419, \
    17 September 1940: the year from 1 January from 2484 (1941) on; the solar months \
    since 1 April 1889 and the Buddhist Era since 1 April 1913 are the printed year, \
    `printed_year`";

/// The earliest fixed day this implementation converts, the first day of
/// Buddhist year 1.
pub const EARLIEST: Rd = match gregorian::to_fixed(MIN_YEAR - YEAR_OFFSET, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of a Thai solar date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    common::offset_to_fixed(year, month, day, YEAR_OFFSET)
}

/// The Thai solar year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] when the day precedes Buddhist
/// year 1, or the Gregorian range errors.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    common::offset_from_fixed(rd, YEAR_OFFSET)
}

/// The era a Thai document printed its year in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrintedEra {
    /// The Rattanakosin era, รัตนโกสินทรศก, from 1 April 1889 (RS 108) to
    /// 31 March 1913 (the end of RS 131).
    Rattanakosin,
    /// The Buddhist Era, from 1 April 1913 (2456).
    Buddhist,
}

impl PrintedEra {
    /// The abbreviation: `RS` or `BE`.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Rattanakosin => "RS",
            Self::Buddhist => "BE",
        }
    }
}

/// How far the Rattanakosin era runs behind the Common Era, for the days
/// from April to December: 1889 was RS 108.
pub const RATTANAKOSIN_OFFSET: i64 = -1781;

/// The first day of the solar reckoning, 1 April 1889, RS 108.
pub const PRINTED_EARLIEST: Rd = match gregorian::to_fixed(1889, 4, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The first day of the Buddhist Era in official use, 1 April 1913, 2456.
pub const BUDDHIST_ERA_FROM: Rd = match gregorian::to_fixed(1913, 4, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The first day of a year beginning on 1 January, 1 January 1941, 2484.
pub const JANUARY_YEAR_FROM: Rd = match gregorian::to_fixed(1941, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The era and year a Thai document of the time would print for `rd`.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] before 1 April 1889, when the
/// reckoning was lunar, or the Gregorian range errors.
pub fn printed_year(rd: Rd) -> CalendarResult<(PrintedEra, i64)> {
    if rd < PRINTED_EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    let (year, month, _) = gregorian::from_fixed(rd)?;
    // Before 1941 January to March belong to the year that began the April
    // before.
    let april_year = if rd < JANUARY_YEAR_FROM && month < 4 {
        year - 1
    } else {
        year
    };
    Ok(if rd < BUDDHIST_ERA_FROM {
        (PrintedEra::Rattanakosin, april_year + RATTANAKOSIN_OFFSET)
    } else {
        (PrintedEra::Buddhist, april_year + YEAR_OFFSET)
    })
}

/// The fixed day of a dateline as a Thai document of the time printed it:
/// a year in `era`, a Gregorian month and a day.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] for a year outside the era's
/// use — RS 108 to 131, or BE from 2456 — and
/// [`CalendarError::MonthOutOfRange`] for January to March of 2483, a year
/// that ended in December. Otherwise the Gregorian errors.
pub fn printed_to_fixed(era: PrintedEra, year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    let april_year = match era {
        PrintedEra::Rattanakosin if (108..=131).contains(&year) => year - RATTANAKOSIN_OFFSET,
        PrintedEra::Buddhist if year >= 2456 => year - YEAR_OFFSET,
        _ => return Err(CalendarError::YearOutOfRange),
    };
    if !(1..=12).contains(&month) {
        return Err(CalendarError::MonthOutOfRange);
    }
    if april_year >= 1941 {
        return gregorian::to_fixed(april_year, month, day);
    }
    if month >= 4 {
        return gregorian::to_fixed(april_year, month, day);
    }
    let rd = gregorian::to_fixed(april_year + 1, month, day)?;
    if rd >= JANUARY_YEAR_FROM {
        return Err(CalendarError::MonthOutOfRange);
    }
    Ok(rd)
}

/// A Thai solar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuddhistDate {
    /// The year of the Buddhist Era, counting from 1.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl BuddhistDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        to_fixed(year, month, day)?;
        Ok(Self { year, month, day })
    }

    /// The same day in the Common Era year numbering.
    #[must_use]
    pub const fn common_era_year(self) -> i64 {
        self.year - YEAR_OFFSET
    }
}

/// The Thai solar calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BuddhistCalendar;

impl Calendar for BuddhistCalendar {
    type Date = BuddhistDate;

    /// The year as this calendar counts it, from 1 January, has been kept
    /// since 1 January 1941 under the Calendar Years Act of 2483. The earlier
    /// solar years from 1 April are [`printed_year`]'s, not this calendar's,
    /// so a day before 1941 is proleptic here even where Thailand was already
    /// writing solar dates.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(JANUARY_YEAR_FROM, USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("buddhist"),
            english_name: "Thai Buddhist",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(gregorian::LATEST),
            native_locales: &["th"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(BuddhistDate { year, month, day })
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
        BuddhistDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_year_offset_is_five_hundred_and_forty_three() {
        // The reference conversion everyone knows: 2026 CE is 2569 BE.
        assert_eq!(
            to_fixed(2569, 1, 1),
            Ok(gregorian::to_fixed(2026, 1, 1).unwrap())
        );
        assert_eq!(from_fixed(Rd(719_163)), Ok((2513, 1, 1)));
        assert_eq!(
            BuddhistDate::new(2569, 1, 1).unwrap().common_era_year(),
            2026
        );
    }

    #[test]
    fn the_constitution_of_1932_has_the_year_printed_on_it() {
        // The permanent constitution was promulgated on 10 December 2475 BE,
        // which is 10 December 1932 CE and still a public holiday.
        assert_eq!(
            to_fixed(2475, 12, 10),
            Ok(gregorian::to_fixed(1932, 12, 10).unwrap())
        );
    }

    #[test]
    fn leap_years_follow_the_gregorian_rule_on_the_common_era_year() {
        assert!(is_leap_year(2543)); // 2000 CE
        assert!(!is_leap_year(2443)); // 1900 CE
        assert!(is_leap_year(2567)); // 2024 CE
        assert_eq!(days_in_month(2543, 2), Some(29));
        assert_eq!(days_in_month(2443, 2), Some(28));
        assert_eq!(days_in_year(2543), 366);
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=1_000_000).step_by(41) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
            assert!(year >= MIN_YEAR);
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = BuddhistCalendar;
        for rd in (EARLIEST.0..=900_000).step_by(613) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("buddhist"));
    }

    #[test]
    fn dates_before_buddhist_year_one_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(2569, 2, 30), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            BuddhistCalendar.from_fields(&DateFields::ymd(2569, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn a_document_before_1941_printed_january_to_march_a_year_lower() {
        // 1 January 1920 was printed 2462: the year 2462 began on 1 April
        // 1919. The calendar's own year is the modern one, 2463.
        assert_eq!(
            printed_year(ymd(1920, 1, 1)),
            Ok((PrintedEra::Buddhist, 2462))
        );
        assert_eq!(from_fixed(ymd(1920, 1, 1)), Ok((2463, 1, 1)));
        assert_eq!(
            printed_year(ymd(1920, 4, 1)),
            Ok((PrintedEra::Buddhist, 2463))
        );
        // The constitution of 10 December 2475 is 1932 either way.
        assert_eq!(
            printed_to_fixed(PrintedEra::Buddhist, 2475, 12, 10),
            Ok(ymd(1932, 12, 10))
        );
    }

    #[test]
    fn the_rattanakosin_era_ran_from_1889_to_march_1913() {
        assert_eq!(
            printed_year(ymd(1889, 4, 1)),
            Ok((PrintedEra::Rattanakosin, 108))
        );
        assert_eq!(
            printed_year(ymd(1889, 3, 31)),
            Err(CalendarError::BeforeEpoch)
        );
        // 21 February RS 131, the day of the proclamation, was in 1913.
        assert_eq!(
            printed_to_fixed(PrintedEra::Rattanakosin, 131, 2, 21),
            Ok(ymd(1913, 2, 21))
        );
        assert_eq!(
            printed_year(ymd(1913, 3, 31)),
            Ok((PrintedEra::Rattanakosin, 131))
        );
        assert_eq!(
            printed_year(ymd(1913, 4, 1)),
            Ok((PrintedEra::Buddhist, 2456))
        );
        // RS 108 ended on 31 March 1890.
        assert_eq!(
            printed_to_fixed(PrintedEra::Rattanakosin, 108, 3, 31),
            Ok(ymd(1890, 3, 31))
        );
        assert_eq!(
            printed_to_fixed(PrintedEra::Rattanakosin, 132, 4, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            printed_to_fixed(PrintedEra::Buddhist, 2455, 4, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_year_2483_was_nine_months_long() {
        let first = printed_to_fixed(PrintedEra::Buddhist, 2483, 4, 1).unwrap();
        let last = printed_to_fixed(PrintedEra::Buddhist, 2483, 12, 31).unwrap();
        assert_eq!(first, ymd(1940, 4, 1));
        assert_eq!(last.0 - first.0 + 1, 275);
        assert_eq!(
            printed_to_fixed(PrintedEra::Buddhist, 2483, 1, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            printed_year(ymd(1941, 1, 1)),
            Ok((PrintedEra::Buddhist, 2484))
        );
        assert_eq!(
            printed_to_fixed(PrintedEra::Buddhist, 2484, 1, 1),
            Ok(ymd(1941, 1, 1))
        );
    }

    #[test]
    fn a_printed_dateline_round_trips() {
        for rd in (PRINTED_EARLIEST.0..=ymd(1960, 12, 31).0).step_by(7) {
            let (era, year) = printed_year(Rd(rd)).unwrap();
            let (_, month, day) = gregorian::from_fixed(Rd(rd)).unwrap();
            assert_eq!(printed_to_fixed(era, year, month, day), Ok(Rd(rd)));
        }
        // From 1941 the printed year is the calendar's.
        for rd in (JANUARY_YEAR_FROM.0..=ymd(2100, 1, 1).0).step_by(97) {
            let (_, year) = printed_year(Rd(rd)).unwrap();
            assert_eq!(Ok(year), from_fixed(Rd(rd)).map(|(year, _, _)| year));
        }
    }

    #[test]
    fn the_dynamic_year_length_matches_the_gregorian_one() {
        use hc_calendar::{DynAdapter, DynCalendar};

        let calendar = DynAdapter::new(BuddhistCalendar);
        assert_eq!(calendar.days_in_year(2567), Ok(366));
        assert_eq!(calendar.days_in_year(2566), Ok(365));
        assert_eq!(calendar.is_leap_year(2567), Ok(true));
    }
}
