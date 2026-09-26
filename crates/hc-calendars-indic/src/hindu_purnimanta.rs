//! The Hindu lunisolar calendar, *pūrṇimānta* — `hindu-lunar-purnimanta`.
//!
//! The naming is explained, with the almanac's labelling of Śaka 1945 and
//! its sources, in `docs/systems/hindu-calendars.md` in the repository,
//! beside the amānta calendar. This page summarises it and states the
//! code's own facts.
//!
//! Northern India keeps the same tithis, the same fortnights and the same
//! year as the amānta reckoning of [`crate::hindu_lunar`], and names the
//! months differently: a month ends at the full moon, so the dark
//! fortnight comes *first* and carries the name of the bright fortnight
//! that follows it. Janmāṣṭamī is Śrāvaṇa kṛṣṇa 8 in the south and
//! Bhādrapada kṛṣṇa 8 in the north, on the same night.
//!
//! # The intercalary month is the exception
//!
//! An *adhika* month is inserted whole and runs bright fortnight first,
//! as in amānta: the ordinary month's dark half, then the adhika month's
//! bright and dark halves, then the ordinary month's bright half, which is
//! how the *Rashtriya Panchang*'s vadi column labels it and how this
//! module maps it.
//!
//! # What this is
//!
//! A renaming of [`HinduLunarCalendar`]'s months, day for day: every
//! conversion goes through the amānta calendar, so the two can never
//! disagree about a tithi, only about what to call its month. The year is
//! the Śaka year and turns at Chaitra śukla 1 in both, so the dark
//! fortnight the north calls Chaitra kṛṣṇa, in March, is the last of the
//! old year.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};

use crate::hindu_lunar::{ERA, HinduLunarCalendar, HinduLunarDate, MONTHS_IN_YEAR};

/// The identifier of the pūrṇimānta Hindu lunisolar calendar.
pub const ID: CalendarId = CalendarId("hindu-lunar-purnimanta");

/// The last tithi of the bright fortnight.
const FULL_MOON: u8 = 15;

/// The pūrṇimānta Hindu lunisolar calendar: the amānta calendar's tithis
/// under the north's month names.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HinduPurnimantaCalendar {
    /// The amānta calendar whose tithis these are.
    pub amanta: HinduLunarCalendar,
}

impl Default for HinduPurnimantaCalendar {
    fn default() -> Self {
        Self::RASHTRIYA
    }
}

impl HinduPurnimantaCalendar {
    /// The calendar as the *Rashtriya Panchang* labels it: sunrise at the
    /// Central Station, Lahiri ayanamsa. The registered
    /// `hindu-lunar-purnimanta`.
    pub const RASHTRIYA: Self = Self {
        amanta: HinduLunarCalendar::RASHTRIYA,
    };

    /// The pūrṇimānta calendar over any amānta one.
    #[must_use]
    pub const fn new(amanta: HinduLunarCalendar) -> Self {
        Self { amanta }
    }

    /// The pūrṇimānta name of an amānta date.
    ///
    /// A bright-fortnight date keeps its month. A dark-fortnight date of
    /// an intercalary month keeps it too — the intercalary month runs
    /// bright half first. Any other dark-fortnight date takes the name of
    /// the ordinary month that follows, which is the next month unless
    /// that next month is intercalary, in which case it is the ordinary
    /// month after that: the intercalary month is inserted between the
    /// ordinary month's two halves.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::to_fixed`], for a date that does not exist.
    pub fn from_amanta(&self, date: HinduLunarDate) -> CalendarResult<HinduLunarDate> {
        if date.day <= FULL_MOON || date.leap_month {
            return Ok(date);
        }
        // The month after this one, by its saṅkrānti: the next amānta
        // month's label, made ordinary.
        let day = self.amanta.to_fixed(date)?;
        let (month, _) = self.amanta.next_month_label(day);
        Ok(HinduLunarDate {
            month,
            leap_month: false,
            ..date
        })
    }

    /// The amānta date of a pūrṇimānta one.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::to_fixed`], for a date that does not exist.
    pub fn to_amanta(&self, date: HinduLunarDate) -> CalendarResult<HinduLunarDate> {
        if date.day <= FULL_MOON || date.leap_month {
            return Ok(date);
        }
        // The dark half of an ordinary pūrṇimānta month is the dark half of
        // the amānta month before the first amānta month of that name —
        // before the intercalary one, if the year has it.
        let year = if date.month == 1 {
            date.year + 1
        } else {
            date.year
        };
        let (month, _) = self.amanta.month_span(year, date.month, false)?;
        let (first, _) = self
            .amanta
            .month_span(year, date.month, true)
            .unwrap_or((month, month));
        let first_start = if first < month { first } else { month };
        let previous = self.amanta.from_fixed(Rd(first_start.0 - 1))?;
        Ok(HinduLunarDate {
            year: previous.year,
            month: previous.month,
            leap_month: previous.leap_month,
            day: date.day,
            leap_day: date.leap_day,
        })
    }

    /// The fixed day of a pūrṇimānta date.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::to_fixed`].
    pub fn to_fixed(&self, date: HinduLunarDate) -> CalendarResult<Rd> {
        self.amanta.to_fixed(self.to_amanta(date)?)
    }

    /// The pūrṇimānta date of a fixed day.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::from_fixed`].
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<HinduLunarDate> {
        self.from_amanta(self.amanta.from_fixed(rd)?)
    }
}

impl Calendar for HinduPurnimantaCalendar {
    type Date = HinduLunarDate;

    /// As the amānta calendar's: in use today, undated at the start.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::undated(crate::hindu_lunar::USAGE_SOURCE)
    }

    /// Twelve months with a thirteenth in an intercalary year, and the
    /// seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// A year with an adhika māsa, which is the amānta year's.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(self.amanta.leap_month_of(year)?.is_some())
    }

    /// The Hindu day begins at sunrise and is named by the civil day on
    /// whose sunrise it begins: Reingold and Dershowitz read a fixed day's
    /// date at "Sunrise that day" (`reingold2018code`, `hindu-lunar-from-fixed`).
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunrise(hc_calendar::DayNaming::ByStart)
    }

    fn meta(&self) -> CalendarMeta {
        let amanta = self.amanta.meta();
        CalendarMeta {
            id: ID,
            english_name: "Hindu lunisolar (purnimanta)",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: amanta.earliest,
            latest: amanta.latest,
            native_locales: &["sa", "hi"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        HinduPurnimantaCalendar::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        HinduPurnimantaCalendar::from_fixed(self, rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let month = if date.leap_month {
            Month::leap(date.month)
        } else {
            Month::regular(date.month)
        };
        let mut fields = DateFields::ymd(date.year, date.month, date.day).with_era(ERA);
        fields.month = Some(month);
        fields.leap_day = date.leap_day;
        fields.with_extra("vikrama-year", date.vikrama_year())
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.ordinal == 0 || month.ordinal > MONTHS_IN_YEAR {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = HinduLunarDate {
            year: fields.year,
            month: month.ordinal,
            leap_month: month.leap,
            day: fields.require_day()?,
            leap_day: fields.leap_day,
        };
        HinduPurnimantaCalendar::to_fixed(self, date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    const RASHTRIYA: HinduPurnimantaCalendar = HinduPurnimantaCalendar::RASHTRIYA;

    /// The "vadi" column of the *Rashtriya Panchang*'s lunar-month table
    /// for Śaka 1945 and 1946: the day each pūrṇimānta dark fortnight
    /// begins, with the month it is named for. The three that begin with a
    /// tithi holding no sunrise (31 August 2023, 22 June 2024,
    /// 18 September 2024) are read on the day after.
    /// A Gregorian date and the Śaka year, month and intercalary flag of the
    /// pūrṇimānta dark fortnight beginning on it.
    type Vadi = ((i64, u8, u8), (i64, u8, bool));

    const VADI: &[Vadi] = &[
        ((2023, 4, 7), (1945, 2, false)),
        ((2023, 5, 6), (1945, 3, false)),
        ((2023, 6, 5), (1945, 4, false)),
        ((2023, 7, 4), (1945, 5, false)),
        ((2023, 8, 2), (1945, 5, true)),
        ((2023, 9, 1), (1945, 6, false)),
        ((2023, 9, 30), (1945, 7, false)),
        ((2023, 10, 29), (1945, 8, false)),
        ((2023, 11, 28), (1945, 9, false)),
        ((2023, 12, 27), (1945, 10, false)),
        ((2024, 1, 26), (1945, 11, false)),
        ((2024, 2, 25), (1945, 12, false)),
        ((2024, 3, 26), (1945, 1, false)),
        ((2024, 4, 24), (1946, 2, false)),
        ((2024, 5, 24), (1946, 3, false)),
        ((2024, 6, 23), (1946, 4, false)),
        ((2024, 7, 22), (1946, 5, false)),
        ((2024, 8, 20), (1946, 6, false)),
        ((2024, 9, 19), (1946, 7, false)),
        ((2024, 10, 18), (1946, 8, false)),
        ((2024, 11, 16), (1946, 9, false)),
        ((2024, 12, 16), (1946, 10, false)),
        ((2025, 1, 14), (1946, 11, false)),
        ((2025, 2, 13), (1946, 12, false)),
        ((2025, 3, 15), (1946, 1, false)),
    ];

    #[test]
    fn every_dark_fortnight_carries_the_name_the_rashtriya_panchang_gives_it() {
        let mut mismatches = alloc::vec::Vec::new();
        for ((y, m, d), (year, month, leap)) in VADI {
            let date = RASHTRIYA.from_fixed(ymd(*y, *m, *d)).unwrap();
            if (date.year, date.month, date.leap_month) != (*year, *month, *leap) || date.day < 16 {
                mismatches.push(alloc::format!("{y}-{m:02}-{d:02}: {date:?}"));
            }
        }
        assert!(mismatches.is_empty(), "{mismatches:#?}");
    }

    #[test]
    fn the_bright_fortnights_keep_their_amanta_names() {
        for (rd, expected) in [
            (ymd(2023, 3, 22), (1945, 1, false, 1)),
            (ymd(2023, 7, 18), (1945, 5, true, 1)),
            (ymd(2023, 8, 17), (1945, 5, false, 1)),
            (ymd(2024, 4, 9), (1946, 1, false, 1)),
        ] {
            let date = RASHTRIYA.from_fixed(rd).unwrap();
            assert_eq!((date.year, date.month, date.leap_month, date.day), expected);
        }
    }

    #[test]
    fn janmashtami_is_bhadrapada_krishna_8_in_the_north() {
        // 26 August 2024: Śrāvaṇa kṛṣṇa 8 in the south, Bhādrapada kṛṣṇa 8
        // in the north, the same day.
        let south = HinduLunarCalendar::RASHTRIYA
            .from_fixed(ymd(2024, 8, 26))
            .unwrap();
        let north = RASHTRIYA.from_fixed(ymd(2024, 8, 26)).unwrap();
        assert_eq!((south.month, south.day), (5, 23));
        assert_eq!((north.month, north.day), (6, 23));
        assert_eq!(north.year, south.year);
    }

    #[test]
    fn every_day_of_two_years_round_trips_through_both_reckonings() {
        // Every day in a release build, every eleventh in a debug one.
        for rd in (ymd(2023, 3, 22).0..ymd(2025, 3, 30).0).step_by(crate::sweep_stride(11)) {
            let date = RASHTRIYA.from_fixed(Rd(rd)).unwrap();
            assert_eq!(RASHTRIYA.to_fixed(date), Ok(Rd(rd)), "rd {rd}: {date:?}");
            let amanta = HinduLunarCalendar::RASHTRIYA.from_fixed(Rd(rd)).unwrap();
            assert_eq!(RASHTRIYA.to_amanta(date), Ok(amanta), "rd {rd}");
            assert_eq!(RASHTRIYA.from_amanta(amanta), Ok(date), "rd {rd}");
            let fields = Calendar::to_fields(&RASHTRIYA, date).unwrap();
            assert_eq!(Calendar::from_fields(&RASHTRIYA, &fields), Ok(date));
        }
    }

    #[test]
    fn the_intercalary_month_runs_bright_half_first() {
        // Adhika Śrāvaṇa 1945: sudi from 18 July, vadi from 2 August 2023;
        // the ordinary Śrāvaṇa's vadi from 4 July, sudi from 17 August.
        let label = |y, m, d| {
            let date = RASHTRIYA.from_fixed(ymd(y, m, d)).unwrap();
            (date.month, date.leap_month, date.day)
        };
        assert_eq!(label(2023, 7, 4), (5, false, 16));
        assert_eq!(label(2023, 7, 18), (5, true, 1));
        assert_eq!(label(2023, 8, 2), (5, true, 16));
        assert_eq!(label(2023, 8, 17), (5, false, 1));
    }
}
