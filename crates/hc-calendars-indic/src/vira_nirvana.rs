//! The Vira Nirvana Samvat, the Jain era counted from Mahāvīra's
//! nirvāṇa, on the amānta months — `vira-nirvana-samvat`.
//!
//! The system is written up in `docs/systems/vira-nirvana-samvat.md` in the
//! repository: the era and its two readings of the nirvāṇa, the year that
//! opens the day after Dīpāvalī, the months, what is carried and what is
//! not, the checks against a published Jain calendar, and the sources,
//! keyed in `docs/references.bib`. This page summarises it and states the
//! code's own facts.
//!
//! # What this is
//!
//! The amānta months of [`crate::hindu_lunar`], read at the Central
//! Station's sunrise as the *Rashtriya Panchang* reads them, under the
//! Jain era. The nirvāṇa is placed 605 years and 5 months before the Śaka
//! era by the *Tiloya-paṇṇatti* and Jinasena's *Harivaṃśa* of 783 CE, in
//! 527 BCE, and scholars of both the Śvetāmbara and the Digambara
//! traditions have upheld that date (`wikipedia-vira-nirvana-samvat`,
//! `jain-sagarmal-nirvana`). So the Kārtika of Śaka year *s* opens Vira
//! Nirvana Samvat *s* + 605, and Chaitra to Āśvina belong to Śaka *s* + 1:
//! the Kārtika of 2024 opened 2551, and New Year's Day of 2552 was
//! 22 October 2025, Kārtika śukla 1, as the Oshwal Association's calendar
//! prints it (`oshwal-2025`). The year opens on Kārtika śukla pratipadā,
//! the day after Dīpāvalī, the night of the nirvāṇa, and its months run
//! Kārtika to Āśvina, numbered 1 to 12 from Kārtika as [`MONTHS`] lists
//! them; a date keeps its tithi, its fortnight and its intercalary or
//! repeated days exactly as the amānta calendar has them.
//!
//! # One identifier, not two
//!
//! No source read gives a separate Digambara epoch for the era in use,
//! 662 BCE or any other: the ones read put the Digambara scholars behind
//! 527 BCE too. A second identifier would name no disagreement, so there
//! is one. The
//! scholarly redatings of the nirvāṇa — 467 BCE, from Mahāvīra's relation
//! to Candragupta Maurya — are arguments about history, not eras anyone
//! dates in, and are not carried.

use hc_calendar::shape::{CycleShape, LUNISOLAR_TWELVE};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};

use crate::hindu_lunar::{HinduLunarCalendar, HinduLunarDate};
use crate::kartikadi;

/// The calendar's identifier.
pub const ID: CalendarId = CalendarId("vira-nirvana-samvat");

/// The era it counts in.
pub const ERA: &str = "vira-nirvana";

/// The Śaka year less the Vira Nirvana year, for Kārtika to Phālguna: the
/// nirvāṇa is 605 years and 5 months before the Śaka era.
pub const SAKA_OFFSET: i64 = -605;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The Jain era of Mahāvīra's nirvāṇa, which the Tiloya-paṇṇatti and Jinasena's Harivaṃśa \
    of 783 CE place 605 years and 5 months before the Śaka era [wikipedia-vira-nirvana-samvat]; \
    printed in Jain calendars today [oshwal-2025]; older than any source read dates its use, \
    as docs/systems/vira-nirvana-samvat.md states";

/// The twelve months in Devanagari, Kārtika first: the amānta months of
/// [`crate::hindu_lunar::MONTHS`] in the order the Jain year runs them.
pub const MONTHS: [&str; 12] = [
    "कार्तिक",
    "मार्गशीर्ष",
    "पौष",
    "माघ",
    "फाल्गुन",
    "चैत्र",
    "वैशाख",
    "ज्येष्ठ",
    "आषाढ",
    "श्रावण",
    "भाद्रपद",
    "आश्विन",
];

/// A date in the Vira Nirvana Samvat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViraNirvanaDate {
    /// The Vira Nirvana Samvat year.
    pub year: i64,
    /// The month, 1 for Kārtika through 12 for Āśvina.
    pub month: u8,
    /// Whether this is the intercalary (*adhika*) month of that name, which
    /// precedes the ordinary one.
    pub leap_month: bool,
    /// The tithi, 1 through 30: 1–15 the bright fortnight, 16–30 the dark.
    pub day: u8,
    /// Whether this is the second day to carry that tithi at sunrise.
    pub leap_day: bool,
}

impl ViraNirvanaDate {
    /// The amānta date of the same day.
    const fn to_amanta(self) -> HinduLunarDate {
        let month = kartikadi::amanta_month(self.month);
        HinduLunarDate {
            year: kartikadi::saka_year(self.year, month, SAKA_OFFSET),
            month,
            leap_month: self.leap_month,
            day: self.day,
            leap_day: self.leap_day,
        }
    }

    /// The Vira Nirvana date of an amānta date.
    const fn from_amanta(date: HinduLunarDate) -> Self {
        Self {
            year: kartikadi::era_year(date.year, date.month, SAKA_OFFSET),
            month: kartikadi::kartikadi_month(date.month),
            leap_month: date.leap_month,
            day: date.day,
            leap_day: date.leap_day,
        }
    }
}

/// The Vira Nirvana Samvat over an amānta calendar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViraNirvanaCalendar {
    lunar: HinduLunarCalendar,
}

impl Default for ViraNirvanaCalendar {
    fn default() -> Self {
        Self::RASHTRIYA
    }
}

impl ViraNirvanaCalendar {
    /// The Central Station's sunrise and the Lahiri ayanamsa, the
    /// *Rashtriya Panchang*'s reckoning: the registered
    /// `vira-nirvana-samvat`.
    pub const RASHTRIYA: Self = Self::new(HinduLunarCalendar::RASHTRIYA);

    /// The era over any amānta calendar — another place's sunrise, another
    /// ayanamsa, as a local Jain almanac would read it.
    #[must_use]
    pub const fn new(lunar: HinduLunarCalendar) -> Self {
        Self { lunar }
    }

    /// The fixed day of a date.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::to_fixed`]: a year outside the engine's
    /// range, a month number outside 1–12, an intercalary month the year
    /// does not have, or a tithi the month skips.
    pub fn to_fixed(&self, date: ViraNirvanaDate) -> CalendarResult<Rd> {
        if !(1..=12).contains(&date.month) {
            return Err(CalendarError::MonthOutOfRange);
        }
        self.lunar.to_fixed(date.to_amanta())
    }

    /// The date on a fixed day.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::from_fixed`], outside the engine's range.
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<ViraNirvanaDate> {
        self.lunar.from_fixed(rd).map(ViraNirvanaDate::from_amanta)
    }

    /// New Year's Day of a year: Kārtika śukla 1, the day after Dīpāvalī —
    /// the first day of the intercalary Kārtika, in a year that has one.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] outside the engine's range.
    pub fn new_year(&self, year: i64) -> CalendarResult<Rd> {
        kartikadi::new_year(&self.lunar, year, SAKA_OFFSET)
    }
}

impl Calendar for ViraNirvanaCalendar {
    type Date = ViraNirvanaDate;

    /// In use today and older than any source read dates, so undated at the
    /// start.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::undated(USAGE_SOURCE)
    }

    /// Twelve months with a thirteenth in an intercalary year, and the
    /// seven-day week.
    fn cycles(&self) -> &'static [CycleShape] {
        LUNISOLAR_TWELVE
    }

    /// A year with an adhika māsa, which may fall in either of the Śaka
    /// years the Jain year spans.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        kartikadi::is_leap_year(&self.lunar, year, SAKA_OFFSET)
    }

    /// The day begins at sunrise and is named by the civil day on whose
    /// sunrise it begins, the amānta calendar's reading: Reingold and
    /// Dershowitz read a fixed day's date at "Sunrise that day"
    /// (`reingold2018code`, `hindu-lunar-from-fixed`).
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunrise(hc_calendar::DayNaming::ByStart)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Vira Nirvana Samvat (Jain)",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: self.lunar.earliest().ok(),
            latest: self.lunar.latest().ok(),
            native_locales: &["hi", "gu"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        ViraNirvanaCalendar::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        ViraNirvanaCalendar::from_fixed(self, rd)
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
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        let date = ViraNirvanaDate {
            year: fields.year,
            month: month.ordinal,
            leap_month: month.leap,
            day: fields.require_day()?,
            leap_day: fields.leap_day,
        };
        ViraNirvanaCalendar::to_fixed(self, date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hindu_lunar::MONTHS as AMANTA_MONTHS;
    use hc_calendars_solar::gregorian;

    const VNS: ViraNirvanaCalendar = ViraNirvanaCalendar::RASHTRIYA;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    #[test]
    fn new_years_day_2552_was_kartika_shukla_1_on_22_october_2025() {
        // The Oshwal Association's "Jain Festivals-Events 2025": "Vir
        // Samvat 2552 | Vikram Samvat 2082 / New Year's Day / 22 Wednesday
        // Kartik Sud Ekam".
        let rd = ymd(2025, 10, 22);
        assert_eq!(VNS.new_year(2_552), Ok(rd));
        let date = VNS.from_fixed(rd).expect("in range");
        assert_eq!((date.year, date.month, date.day), (2_552, 1, 1));
        let eve = VNS.from_fixed(Rd(rd.0 - 1)).expect("in range");
        assert_eq!((eve.year, eve.month), (2_551, 12));
    }

    #[test]
    fn the_festivals_of_2025_fall_in_2551_on_the_printed_tithis() {
        // The same calendar, headed "Vir Samvat 2551" until Diwali: Posh
        // Sud Poonam on 13 January, Chaitra Sud Teras (Mahāvīra's birth) on
        // 10 April, Bhadarvo Sud Choth (Samvatsari) on 27 August; and after
        // it, in 2552, Kartik Sud Poonam on 5 November.
        for ((y, m, d), (year, month, day)) in [
            ((2025, 1, 13), (2_551, 3, 15)),
            ((2025, 4, 10), (2_551, 6, 13)),
            ((2025, 8, 27), (2_551, 11, 4)),
            ((2025, 11, 5), (2_552, 1, 15)),
        ] {
            let date = VNS.from_fixed(ymd(y, m, d)).expect("in range");
            assert_eq!(
                (date.year, date.month, date.leap_month, date.day),
                (year, month, false, day),
                "{y}-{m}-{d}"
            );
        }
    }

    #[test]
    fn year_2544_began_at_the_diwali_of_2017() {
        // Wikipedia, "Vira Nirvana Samvat": 2544 "started right after
        // Diwali of 20 October 2017".
        assert_eq!(VNS.from_fixed(ymd(2017, 10, 21)).map(|d| d.year), Ok(2_544));
        assert_eq!(VNS.from_fixed(ymd(2017, 10, 1)).map(|d| d.year), Ok(2_543));
    }

    #[test]
    fn the_year_is_the_saka_year_plus_605_from_kartika_and_604_before() {
        let amanta = HinduLunarCalendar::RASHTRIYA;
        for day in (ymd(2023, 1, 1).0..ymd(2027, 1, 1).0).step_by(5) {
            let rd = Rd(day);
            let lunar = amanta.from_fixed(rd).expect("in range");
            let jain = VNS.from_fixed(rd).expect("in range");
            let offset = if lunar.month >= 8 { 605 } else { 604 };
            assert_eq!(jain.year, lunar.year + offset, "{lunar:?}");
            assert_eq!(
                (jain.day, jain.leap_day, jain.leap_month),
                (lunar.day, lunar.leap_day, lunar.leap_month)
            );
            assert_eq!(
                MONTHS[usize::from(jain.month - 1)],
                AMANTA_MONTHS[usize::from(lunar.month - 1)]
            );
        }
    }

    #[test]
    fn every_day_of_three_years_converts_and_converts_back() {
        let mut intercalary = alloc::vec::Vec::new();
        for day in
            VNS.new_year(2_549).expect("in range").0..VNS.new_year(2_552).expect("in range").0
        {
            let rd = Rd(day);
            let date = VNS.from_fixed(rd).expect("in range");
            assert_eq!(VNS.to_fixed(date), Ok(rd), "{date:?}");
            let fields = Calendar::to_fields(&VNS, date).expect("fields");
            assert_eq!(Calendar::from_fields(&VNS, &fields), Ok(date));
            if date.leap_month && !intercalary.contains(&(date.year, date.month)) {
                intercalary.push((date.year, date.month));
            }
        }
        // The adhika Śrāvaṇa of Śaka 1945, July–August 2023, is in 2549.
        assert_eq!(intercalary, [(2_549, 10)]);
        assert_eq!(Calendar::is_leap_year(&VNS, 2_549), Ok(true));
        assert_eq!(Calendar::is_leap_year(&VNS, 2_550), Ok(false));
    }

    #[test]
    fn fields_carry_the_era_and_refuse_another() {
        let date = VNS.from_fixed(ymd(2025, 10, 22)).expect("in range");
        let fields = Calendar::to_fields(&VNS, date).expect("fields");
        assert_eq!(fields.era, Some(ERA));
        assert_eq!(Calendar::from_fields(&VNS, &fields), Ok(date));
        let mut wrong = fields;
        wrong.era = Some("saka");
        assert_eq!(
            Calendar::from_fields(&VNS, &wrong),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(
            VNS.to_fixed(ViraNirvanaDate { month: 13, ..date }),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(VNS.new_year(1_000), Err(CalendarError::YearOutOfRange));
    }
}
