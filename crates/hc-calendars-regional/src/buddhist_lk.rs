//! Sri Lanka's Buddhist Era over the Gregorian day: `buddhist-lk`.
//!
//! Sri Lanka numbers the Buddhist year from the Vesak Full Moon Poya Day:
//! the day is the Common Era year plus 544 from Vesak and plus 543 before
//! it, so that the year 2569 ran from Vesak 2025 to the day before Vesak
//! 2026. The State Vesak Festival of 10–16 May 2025 was held "for the
//! Buddhist Year 2569" and the Prime Minister's message of Vesak day,
//! 12 May 2025, is headed "B.C 2569/2025" (`media-gov-lk-vesak-2025`,
//! `media-gov-lk-vesak-message-2569`); the festival of 27 May 2026, around
//! the Vesak Poya of 30 May, is "for the Buddhist Year 2570"
//! (`ziradaily-vesak-2026`). The rule in those words — add 544 after Wesak
//! and 543 before it, against Thailand's 543 from 1 January — is the
//! Buddhist Missionary Society Malaysia's statement for Sri Lanka, Malaysia
//! and Singapore (`wesak-determining-be`); no Sri Lankan text read states the rule
//! itself, and the government desk calendars whose covers carry the two
//! numbers of a year are PDFs, not read.
//!
//! The Vesak Poya Day is whatever the Minister declares under the Holidays
//! Act, No. 29 of 1971, and the orders read fix it for 2023–2027: 5 May
//! 2023, 23 May 2024, 12 May 2025, 30 May 2026 (the Cabinet decision of
//! 30 March 2026, which moved it from 1 May; `adaderana-vesak-2026`) and
//! 19 May 2027, the same days `hc-holiday` carries for Sri Lanka. A year
//! outside them is a gap (ADR 0013), not a computed full moon: the rule
//! the Poya committee follows is the roadmap's `sinhalese-lunar` question.
//!
//! The month and the day are the Gregorian ones, as the Sri Lankan
//! documents write them beside the year. Because the Vesak day moves, a
//! Buddhist year with a Gregorian month and day can name two days — 20 May
//! 2569 is both 20 May 2025, after Vesak, and 20 May 2026, before it — so
//! [`BuddhistLkCalendar::to_fields`] writes the Gregorian year as the extra
//! field `gregorian-year`, and [`BuddhistLkCalendar::from_fields`] reads it
//! back; without the field it answers when the day is unambiguous and
//! refuses with [`CalendarError::MissingField`] when it is not.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_calendars_solar::gregorian;

/// The machine identifier.
pub const ID: CalendarId = CalendarId("buddhist-lk");

/// The Vesak Full Moon Poya Day of each year the Holidays Act orders read
/// fix, as a Gregorian date.
pub static VESAK_POYA: [(i64, u8, u8); 5] = [
    (2023, 5, 5),
    (2024, 5, 23),
    (2025, 5, 12),
    (2026, 5, 30),
    (2027, 5, 19),
];

/// The first Common Era year whose Vesak day is fixed here.
pub const FIRST_YEAR: i64 = 2023;

/// The last.
pub const LAST_YEAR: i64 = 2027;

/// The first day the calendar converts: 1 January 2023.
pub const EARLIEST: Rd = match gregorian::to_fixed(FIRST_YEAR, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The last: 31 December 2027.
pub const LATEST: Rd = match gregorian::to_fixed(LAST_YEAR, 12, 31) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The count's offset from Vesak to the year's end.
pub const OFFSET_FROM_VESAK: i64 = 544;

/// Where the period comes from.
pub const USAGE_SOURCE: &str = "The Buddhist year of Sri Lanka, the Common Era year plus 544 from the Vesak Full Moon \
    Poya Day and plus 543 before it: the State Vesak Festival of May 2025 for the Buddhist Year \
    2569 and the Vesak message of 12 May 2025 headed B.C 2569/2025 [media-gov-lk-vesak-2025, \
    media-gov-lk-vesak-message-2569], the festival of 2026 for the Buddhist Year 2570 \
    [ziradaily-vesak-2026], the rule as stated for Sri Lanka, Malaysia and Singapore \
    [wesak-determining-be]; the Vesak days are the Holidays Act orders of 2023-2027, so the count is \
    carried from 1 January 2023 and a year outside them is a gap, not the count's beginning";

/// The Vesak Full Moon Poya Day of a Common Era year, where an order fixes it.
#[must_use]
pub fn vesak_poya(year: i64) -> Option<Rd> {
    VESAK_POYA
        .iter()
        .find(|(y, _, _)| *y == year)
        .and_then(|&(y, m, d)| gregorian::to_fixed(y, m, d).ok())
}

/// The Buddhist year of a day.
///
/// # Errors
///
/// [`CalendarError::BeforeEpoch`] before 2023 and
/// [`CalendarError::AfterSupportedRange`] after 2027, the years whose
/// Vesak day no order read fixes.
pub fn year_of(rd: Rd) -> CalendarResult<i64> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    let (year, _, _) = gregorian::from_fixed(rd)?;
    let vesak = vesak_poya(year).ok_or(CalendarError::YearOutOfRange)?;
    Ok(if rd >= vesak {
        year + OFFSET_FROM_VESAK
    } else {
        year + OFFSET_FROM_VESAK - 1
    })
}

/// A day in Sri Lanka's Buddhist Era.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuddhistLkDate {
    /// The Buddhist year.
    pub year: i64,
    /// The Gregorian month.
    pub month: u8,
    /// The Gregorian day of the month.
    pub day: u8,
    /// The Gregorian year, which the Buddhist year does not determine on
    /// its own.
    pub gregorian_year: i64,
}

/// The date of a fixed day.
///
/// # Errors
///
/// As [`year_of`].
pub fn from_fixed(rd: Rd) -> CalendarResult<BuddhistLkDate> {
    let year = year_of(rd)?;
    let (gregorian_year, month, day) = gregorian::from_fixed(rd)?;
    Ok(BuddhistLkDate {
        year,
        month,
        day,
        gregorian_year,
    })
}

/// The fixed day of a date.
///
/// # Errors
///
/// The Gregorian errors for a month or day the year did not have, and
/// [`CalendarError::YearOutOfRange`] when the day does not carry the
/// Buddhist year the date claims.
pub fn to_fixed(date: BuddhistLkDate) -> CalendarResult<Rd> {
    let rd = gregorian::to_fixed(date.gregorian_year, date.month, date.day)?;
    if year_of(rd)? != date.year {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(rd)
}

/// Sri Lanka's Buddhist Era over the Gregorian day: `buddhist-lk`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BuddhistLkCalendar;

impl Calendar for BuddhistLkCalendar {
    type Date = BuddhistLkDate;

    /// From 1 January 2023, the first day whose Vesak the orders read fix,
    /// and onwards; the count is centuries older, and the years before are
    /// a gap, not a time before the count.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(EARLIEST, USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    /// For a Buddhist year: whether either Gregorian year it spans is a
    /// leap year is not one question, so this answers for the Gregorian
    /// year the Buddhist year began in, the one Vesak falls in.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        let began = year - OFFSET_FROM_VESAK;
        if !(FIRST_YEAR..=LAST_YEAR).contains(&began) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(gregorian::is_leap_year(began))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Buddhist Era (Sri Lanka, from Vesak)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["si", "ta"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("gregorian-year", date.gregorian_year)
    }

    /// Reads the Gregorian year from the extra field, or finds the one
    /// Gregorian year on which this month and day carry the Buddhist year;
    /// when two do, refuses with [`CalendarError::MissingField`].
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        let date = |gregorian_year| BuddhistLkDate {
            year: fields.year,
            month: month.ordinal,
            day,
            gregorian_year,
        };
        if let Some(gregorian_year) = fields.extra.get("gregorian-year") {
            let date = date(gregorian_year);
            to_fixed(date)?;
            return Ok(date);
        }
        let candidates = [
            fields.year - OFFSET_FROM_VESAK,
            fields.year - OFFSET_FROM_VESAK + 1,
        ];
        let mut found = None;
        for gregorian_year in candidates {
            if to_fixed(date(gregorian_year)).is_ok() {
                if found.is_some() {
                    return Err(CalendarError::MissingField("gregorian-year"));
                }
                found = Some(date(gregorian_year));
            }
        }
        found.ok_or(CalendarError::YearOutOfRange)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    #[test]
    fn the_year_turns_on_vesak_poya_day() {
        // The State Vesak Festival of May 2025 for the Buddhist Year 2569,
        // the message of 12 May 2025 headed B.C 2569/2025, and 2570 for the
        // festival around 30 May 2026.
        assert_eq!(year_of(greg(2025, 5, 11)), Ok(2568));
        assert_eq!(year_of(greg(2025, 5, 12)), Ok(2569));
        assert_eq!(year_of(greg(2026, 1, 1)), Ok(2569));
        assert_eq!(year_of(greg(2026, 5, 29)), Ok(2569));
        assert_eq!(year_of(greg(2026, 5, 30)), Ok(2570));
        // 1 May 2026, the adhi Poya the schedule had first named Vesak, is
        // not the turn.
        assert_eq!(year_of(greg(2026, 5, 1)), Ok(2569));
        assert_eq!(year_of(greg(2023, 5, 4)), Ok(2566));
        assert_eq!(year_of(greg(2023, 5, 5)), Ok(2567));
        assert_eq!(year_of(greg(2027, 5, 19)), Ok(2571));
        assert_eq!(year_of(greg(2027, 12, 31)), Ok(2571));
    }

    #[test]
    fn the_years_outside_the_orders_are_a_gap() {
        assert_eq!(year_of(greg(2022, 12, 31)), Err(CalendarError::BeforeEpoch));
        assert_eq!(
            year_of(greg(2028, 1, 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(vesak_poya(2022), None);
    }

    #[test]
    fn every_day_round_trips_through_fields() {
        let calendar = BuddhistLkCalendar;
        for rd in EARLIEST.0..=LATEST.0 {
            let rd = Rd(rd);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "{rd}");
            let fields = calendar.to_fields(date).expect("fields");
            assert_eq!(
                fields.extra.get("gregorian-year"),
                Some(date.gregorian_year)
            );
            assert_eq!(calendar.from_fields(&fields), Ok(date), "{rd}");
        }
    }

    #[test]
    fn a_date_without_the_gregorian_year_is_read_when_it_is_unambiguous() {
        let calendar = BuddhistLkCalendar;
        // 20 May 2569 is 20 May 2025 (after Vesak) and 20 May 2026 (before
        // Vesak): refused without the field.
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(2569, 5, 20)),
            Err(CalendarError::MissingField("gregorian-year"))
        );
        // 1 January 2569 is 1 January 2026 alone.
        assert_eq!(
            calendar
                .from_fields(&DateFields::ymd(2569, 1, 1))
                .map(|d| d.gregorian_year),
            Ok(2026)
        );
        // 1 June 2569 is 1 June 2025 alone.
        assert_eq!(
            calendar
                .from_fields(&DateFields::ymd(2569, 6, 1))
                .map(|d| d.gregorian_year),
            Ok(2025)
        );
        // A year no day carries.
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(2560, 6, 1)),
            Err(CalendarError::YearOutOfRange)
        );
        // The field, when given, must agree with the year.
        let wrong = DateFields::ymd(2569, 5, 11)
            .with_extra("gregorian-year", 2025)
            .expect("one extra");
        assert_eq!(
            calendar.from_fields(&wrong),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_metadata_and_usage_say_what_the_module_says() {
        let calendar = BuddhistLkCalendar;
        assert_eq!(calendar.meta().id, ID);
        assert_eq!(calendar.usage().from, Some(EARLIEST));
        assert_eq!(calendar.is_leap_year(2568), Ok(true));
        assert_eq!(calendar.is_leap_year(2569), Ok(false));
        assert_eq!(
            calendar.is_leap_year(2566),
            Err(CalendarError::YearOutOfRange)
        );
    }
}
