//! The Faṣlī revenue years of Madras and Bombay and the Maratha Sūr-san —
//! `fasli-madras`, `fasli-bombay`, `sur-san`.
//!
//! The years are written up with the other eras of Sewell and Dikshit's
//! Art. 71 in `docs/systems/indian-eras.md` in the repository: their
//! origin in Akbar's harvest year, the regional openings, why only the
//! year number is carried, the day the Mṛgaśira ingress opens and the
//! checks. This page states the code's own facts.
//!
//! # What this is
//!
//! A year number over the Gregorian months and days, which turns on one
//! day a year. Sewell and Dikshit say the solar Faṣlī year keeps the Hijri
//! months and days and "changes its numerical designation on a stated
//! solar day" (*The Indian Calendar*, 1896, Art. 71, p. 44, `sewell1896`),
//! and give no date in those months; the year and the day it changes are
//! what the sources fix, and the Gregorian day is the one the revenue
//! records beside it were written in. A date is the year, the Gregorian
//! month, 1 for January, and the day, and whether it is the second day of
//! that name in the year: a year from the Mṛgaśira ingress that opens on a
//! later day of June than the year before holds its first day twice.
//!
//! * [`MADRAS`]: the year opens on 1 July, "in A.D. 1855 altered ... to
//!   July 1st" from the 13 July of about 1800, and Faṣlī 1302 "began ...
//!   on July 1st, 1892, in Madras" (Sewell and Dikshit, p. 44); Faṣlī 1410
//!   ran from July 2000 to June 2001 (Wikipedia, "Fasli calendar",
//!   retrieved 2026-09-26, `wikipedia-fasli-calendar`). Whether 1 July
//!   first opened the year in 1855 or 1856 the source does not say, so the
//!   calendar begins on 13 July 1855, from which Faṣlī 1265 holds under
//!   either reading.
//! * [`BOMBAY`]: the year opens "when the sun enters the nakshatra
//!   Mrigasirsha", and 1302 began "on June 5th, 1892, in Bombay" (p. 44);
//!   the ingress is [`crate::nakshatra::solar_nakshatra_ingress_after`]'s
//!   with the Lahiri ayanamsa, and the day it opens is the sunrise-to-sunrise
//!   day at Ujjain in which it falls, [`SankrantiRule::SunriseDay`] — the
//!   rule that gives Sewell and Dikshit's 5 June 1892 for an ingress at
//!   04:41 local mean time on the 6th.
//! * [`SUR_SAN`]: "nine years behind the Fasali of the Dakhan, but in
//!   other respects ... just the same"; "to convert it to an A.D. year, add
//!   599" (p. 45).

use hc_astro::riseset::Location;
use hc_calendar::fixed::Moment;
use hc_calendar::shape::{CycleShape, SOLAR_TWELVE};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_calendars_solar::gregorian;
use hc_seasons::zodiac::Ayanamsa;

use crate::hindu_solar::{MAX_GREGORIAN_YEAR, MIN_GREGORIAN_YEAR, SankrantiRule};
use crate::nakshatra::{MRIGASHIRSHA, solar_nakshatra_ingress_after};
use crate::places::UJJAIN;

/// The day a Faṣlī year opens.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opening {
    /// A fixed Gregorian month and day, from the first day the calendar
    /// converts.
    Fixed {
        /// The month, 1 for January.
        month: u8,
        /// The day of the month.
        day: u8,
        /// The first day converted, as a Gregorian year, month and day.
        first: (i64, u8, u8),
    },
    /// The day the Sun enters the nakṣatra Mṛgaśira, 53°20′ of sidereal
    /// longitude: the sunrise-to-sunrise day at `location` in which the
    /// ingress falls.
    Mrigashira {
        /// The ayanamsa the ingress is measured from.
        ayanamsa: Ayanamsa,
        /// The place whose sunrise divides the days.
        location: Location,
    },
}

/// A Faṣlī year count over the Gregorian days.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FasliCalendar {
    /// The calendar's identifier.
    pub id: CalendarId,
    /// Its English name.
    pub english_name: &'static str,
    /// The era code.
    pub era: &'static str,
    /// The day a year opens.
    pub opening: Opening,
    /// The Gregorian year in which the era's year `y` opens, less `y`.
    pub offset: i64,
    /// Where the period of use comes from.
    pub usage_source: &'static str,
}

/// The era code of the Faṣlī years.
pub const FASLI_ERA: &str = "fasli";

/// The Madras Faṣlī year, from 1 July — `fasli-madras`.
pub const MADRAS: FasliCalendar = FasliCalendar {
    id: CalendarId("fasli-madras"),
    english_name: "Fasli (Madras, from 1 July)",
    era: FASLI_ERA,
    opening: Opening::Fixed {
        month: 7,
        day: 1,
        first: (1855, 7, 13),
    },
    offset: 590,
    usage_source: "Sewell and Dikshit 1896, Art. 71, p. 44 [sewell1896]: the Madras Fasli year fixed at \
        1 July in 1855; counted in the revenue records of Andhra Pradesh, Karnataka and Tamil Nadu \
        today [wikipedia-fasli-calendar], as docs/systems/indian-eras.md states",
};

/// The Bombay Faṣlī year, from the Sun's entry into Mṛgaśira —
/// `fasli-bombay`.
pub const BOMBAY: FasliCalendar = FasliCalendar {
    id: CalendarId("fasli-bombay"),
    english_name: "Fasli (Bombay, from the Mrigashira ingress)",
    era: FASLI_ERA,
    opening: Opening::Mrigashira {
        ayanamsa: Ayanamsa::LAHIRI,
        location: UJJAIN,
    },
    offset: 590,
    usage_source: "Sewell and Dikshit 1896, Art. 71, p. 44 [sewell1896]: in parts of Bombay, from the \
        Sun's entry into Mrigasirsha, in their day; neither its first nor its last year is dated \
        by a source read, as docs/systems/indian-eras.md states",
};

/// The Maratha Sūr-san, the Bombay year nine less — `sur-san`.
pub const SUR_SAN: FasliCalendar = FasliCalendar {
    id: CalendarId("sur-san"),
    english_name: "Sur-san (Maratha)",
    era: "sur-san",
    opening: BOMBAY.opening,
    offset: 599,
    usage_source: "Sewell and Dikshit 1896, Art. 71, p. 45 [sewell1896]: used extensively under the \
        Maratha supremacy and rarely in their day, a solar year since it diverged from the Hijra \
        in 1344, on days no source read dates, as docs/systems/indian-eras.md states",
};

/// Every Faṣlī reckoning this crate registers.
pub const ALL: &[FasliCalendar] = &[MADRAS, BOMBAY, SUR_SAN];

/// A date in a Faṣlī year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FasliDate {
    /// The Faṣlī or Sūr-san year.
    pub year: i64,
    /// The Gregorian month, 1 for January.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
    /// Whether this is the second day of the year with this month and day.
    /// A year from the Mṛgaśira ingress can open on a later day of June
    /// than the one before it: Faṣlī 1302 opened on 5 June 1892 and 1303 on
    /// 6 June 1893, so 1302 holds 5 June twice, and the second is
    /// `repeated`, as [`DateFields::leap_day`] marks a second day of the
    /// same name. Always `false` in Madras.
    pub repeated: bool,
}

impl FasliCalendar {
    /// The day the year opening in Gregorian year `gregorian_year` opens.
    fn opening_in(&self, gregorian_year: i64) -> CalendarResult<Rd> {
        match self.opening {
            Opening::Fixed { month, day, .. } => gregorian::to_fixed(gregorian_year, month, day),
            Opening::Mrigashira { ayanamsa, location } => {
                // The ingress comes in late May or June; mid-May is before
                // it in every year converted.
                let may = gregorian::to_fixed(gregorian_year, 5, 15)?;
                let ingress =
                    solar_nakshatra_ingress_after(MRIGASHIRSHA, ayanamsa, Moment(may.0 as f64));
                Ok(SankrantiRule::SunriseDay.month_begins(ingress, location))
            }
        }
    }

    /// The first Gregorian year in which a year opens that is converted.
    const fn first_gregorian_year(&self) -> i64 {
        match self.opening {
            Opening::Fixed { first, .. } => first.0,
            Opening::Mrigashira { .. } => MIN_GREGORIAN_YEAR,
        }
    }

    /// The earliest year converted.
    #[must_use]
    pub const fn min_year(&self) -> i64 {
        self.first_gregorian_year() - self.offset
    }

    /// The latest year converted: the one opening in 2299.
    #[must_use]
    pub const fn max_year(&self) -> i64 {
        MAX_GREGORIAN_YEAR - self.offset
    }

    /// New Year's Day of a year.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] outside
    /// [`min_year`](Self::min_year)..=[`max_year`](Self::max_year), and for
    /// Madras's first year, Faṣlī 1265, whose first day the source leaves
    /// between 1 and 13 July 1855.
    pub fn new_year(&self, year: i64) -> CalendarResult<Rd> {
        if !(self.min_year()..=self.max_year()).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        let opening = self.opening_in(year + self.offset)?;
        if opening < self.earliest()? {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(opening)
    }

    /// The earliest fixed day converted.
    ///
    /// # Errors
    ///
    /// Only if the first day cannot be placed, which it can.
    pub fn earliest(&self) -> CalendarResult<Rd> {
        match self.opening {
            Opening::Fixed { first, .. } => gregorian::to_fixed(first.0, first.1, first.2),
            Opening::Mrigashira { .. } => self.opening_in(MIN_GREGORIAN_YEAR),
        }
    }

    /// The latest fixed day converted: the day before the year after
    /// [`max_year`](Self::max_year) opens.
    ///
    /// # Errors
    ///
    /// Only if the day cannot be placed, which it can.
    pub fn latest(&self) -> CalendarResult<Rd> {
        self.opening_in(MAX_GREGORIAN_YEAR + 1)
            .map(|next| Rd(next.0 - 1))
    }

    /// The date on a fixed day.
    ///
    /// # Errors
    ///
    /// [`CalendarError::BeforeEpoch`] and
    /// [`CalendarError::AfterSupportedRange`] outside the range.
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<FasliDate> {
        if rd < self.earliest()? {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd > self.latest()? {
            return Err(CalendarError::AfterSupportedRange);
        }
        let (gregorian_year, month, day) = gregorian::from_fixed(rd)?;
        let opened = if rd >= self.opening_in(gregorian_year)? {
            gregorian_year
        } else {
            gregorian_year - 1
        };
        // The same month and day in the Gregorian year the Faṣlī year
        // opened in, if that day is in the year too, came first.
        let repeated = gregorian_year > opened
            && gregorian::to_fixed(opened, month, day)
                .is_ok_and(|first| first >= self.opening_in(opened).unwrap_or(first));
        Ok(FasliDate {
            year: opened - self.offset,
            month,
            day,
            repeated,
        })
    }

    /// The fixed day of a date.
    ///
    /// # Errors
    ///
    /// [`CalendarError::MonthOutOfRange`] or
    /// [`CalendarError::DayOutOfRange`] for a month or day the Gregorian
    /// calendar does not have, [`CalendarError::YearOutOfRange`] outside
    /// the range, and [`CalendarError::DayOutOfRange`] for a day of the
    /// Gregorian year that is not in this Faṣlī year.
    pub fn to_fixed(&self, date: FasliDate) -> CalendarResult<Rd> {
        if !(self.min_year()..=self.max_year()).contains(&date.year) {
            return Err(CalendarError::YearOutOfRange);
        }
        let opened = date.year + self.offset;
        let mut last_error = CalendarError::DayOutOfRange;
        for gregorian_year in [opened, opened + 1] {
            match gregorian::to_fixed(gregorian_year, date.month, date.day) {
                Ok(rd) => {
                    if self.from_fixed(rd) == Ok(date) {
                        return Ok(rd);
                    }
                }
                Err(error) => last_error = error,
            }
        }
        Err(match last_error {
            CalendarError::MonthOutOfRange => CalendarError::MonthOutOfRange,
            _ => CalendarError::DayOutOfRange,
        })
    }
}

impl Calendar for FasliCalendar {
    type Date = FasliDate;

    /// Attested by Sewell and Dikshit and, for Madras, in use today; the
    /// sources date no first or last day, so undated.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::undated(self.usage_source)
    }

    /// The Gregorian months and the seven-day week.
    fn cycles(&self) -> &'static [CycleShape] {
        SOLAR_TWELVE
    }

    /// A year that holds a 29 February: the one opening in the year before
    /// a Gregorian leap year.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(self.min_year()..=self.max_year()).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(gregorian::is_leap_year(year + self.offset + 1))
    }

    /// The civil day, midnight to midnight: the dates are Gregorian.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Midnight
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id,
            english_name: self.english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: matches!(self.opening, Opening::Mrigashira { .. }),
            earliest: self.earliest().ok(),
            latest: self.latest().ok(),
            native_locales: &["en"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        FasliCalendar::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        FasliCalendar::from_fixed(self, rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut fields = DateFields::ymd(date.year, date.month, date.day).with_era(self.era);
        fields.leap_day = date.repeated;
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != self.era) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = FasliDate {
            year: fields.year,
            month: month.ordinal,
            day: fields.require_day()?,
            repeated: fields.leap_day,
        };
        FasliCalendar::to_fixed(self, date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    #[test]
    fn fasli_1302_began_where_sewell_and_dikshit_say() {
        // "In Southern India the Fasali year 1302 began on June 5th, 1892,
        // in Bombay, and on July 1st, 1892, in Madras" (Art. 71, p. 44);
        // the Sūr-san is nine years behind, AD = year + 599 (p. 45).
        assert_eq!(MADRAS.new_year(1_302), Ok(ymd(1892, 7, 1)));
        assert_eq!(BOMBAY.new_year(1_302), Ok(ymd(1892, 6, 5)));
        assert_eq!(SUR_SAN.new_year(1_302 - 9), Ok(ymd(1892, 6, 5)));
        assert_eq!(
            MADRAS.from_fixed(ymd(1892, 6, 30)).map(|d| d.year),
            Ok(1_301)
        );
        assert_eq!(
            BOMBAY.from_fixed(ymd(1892, 6, 4)).map(|d| d.year),
            Ok(1_301)
        );
        let date = BOMBAY.from_fixed(ymd(1892, 6, 5)).expect("in range");
        assert_eq!(
            date,
            FasliDate {
                year: 1_302,
                month: 6,
                day: 5,
                repeated: false,
            }
        );
        // The ingress itself was on the 6th before Ujjain's sunrise: by
        // the civil day it would have opened the year a day later.
        let may = Moment(ymd(1892, 5, 15).0 as f64);
        let ingress = solar_nakshatra_ingress_after(MRIGASHIRSHA, Ayanamsa::LAHIRI, may);
        assert_eq!(
            SankrantiRule::CivilDay.month_begins(ingress, UJJAIN),
            ymd(1892, 6, 6)
        );
    }

    #[test]
    fn a_year_that_opens_a_day_later_holds_its_first_day_twice() {
        // Faṣlī 1302 opened on 5 June 1892 and 1303 on 6 June 1893, the
        // ingress then falling after Ujjain's sunrise: 1302 runs 366 days
        // with no 29 February, and 5 June 1893 is its second 5 June. The
        // release-mode sweep found the two days converting to one.
        assert_eq!(BOMBAY.new_year(1_303), Ok(ymd(1893, 6, 6)));
        for calendar in [BOMBAY, SUR_SAN] {
            let first = calendar.from_fixed(ymd(1892, 6, 5)).expect("in range");
            let second = calendar.from_fixed(ymd(1893, 6, 5)).expect("in range");
            assert_eq!((first.year, second.year), (first.year, first.year));
            assert!(!first.repeated && second.repeated, "{}", calendar.id);
            assert_eq!(calendar.to_fixed(first), Ok(ymd(1892, 6, 5)));
            assert_eq!(calendar.to_fixed(second), Ok(ymd(1893, 6, 5)));
            let fields = Calendar::to_fields(&calendar, second).expect("fields");
            assert!(fields.leap_day);
            assert_eq!(Calendar::from_fields(&calendar, &fields), Ok(second));
            // 4 June 1893 is the only 4 June of 1302, so it is not repeated,
            // and a repeated 4 June does not exist.
            let june_4 = FasliDate { day: 4, ..second };
            assert_eq!(calendar.to_fixed(june_4), Err(CalendarError::DayOutOfRange));
            let june_4 = FasliDate {
                repeated: false,
                ..june_4
            };
            assert_eq!(calendar.to_fixed(june_4), Ok(ymd(1893, 6, 4)));
        }
        // Madras opens on 1 July every year and never repeats a day.
        let madras = MADRAS.from_fixed(ymd(1893, 6, 30)).expect("in range");
        assert!(!madras.repeated);
    }

    #[test]
    fn fasli_1410_is_july_2000_to_june_2001() {
        // Wikipedia, "Fasli calendar": "corresponding Gregorian year for
        // Fasli year 1410 was from July 2000 – June 2001".
        assert_eq!(MADRAS.new_year(1_410), Ok(ymd(2000, 7, 1)));
        assert_eq!(
            MADRAS.from_fixed(ymd(2001, 6, 30)).map(|d| d.year),
            Ok(1_410)
        );
        assert_eq!(
            MADRAS.from_fixed(ymd(2001, 7, 1)).map(|d| d.year),
            Ok(1_411)
        );
        assert_eq!(Calendar::is_leap_year(&MADRAS, 1_409), Ok(true));
        assert_eq!(Calendar::is_leap_year(&MADRAS, 1_410), Ok(false));
    }

    #[test]
    fn the_bombay_year_opens_on_7_or_8_june_today() {
        // Wikipedia, "Fasli calendar": "The first day of the year is 7 or
        // 8 June."
        assert_eq!(BOMBAY.new_year(2024 - 590), Ok(ymd(2024, 6, 7)));
        assert_eq!(BOMBAY.new_year(2025 - 590), Ok(ymd(2025, 6, 8)));
        for year in 2000..2030 {
            let (_, month, day) =
                gregorian::from_fixed(BOMBAY.new_year(year - 590).expect("in range"))
                    .expect("a date");
            assert!(
                month == 6 && (5..=9).contains(&day),
                "{year}: {month}-{day}"
            );
        }
    }

    #[test]
    fn madras_begins_where_either_reading_of_1855_agrees() {
        assert_eq!(MADRAS.earliest(), Ok(ymd(1855, 7, 13)));
        assert_eq!(
            MADRAS.from_fixed(ymd(1855, 7, 12)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            MADRAS.from_fixed(ymd(1855, 7, 13)).map(|d| d.year),
            Ok(1_265)
        );
        assert_eq!(
            MADRAS.to_fixed(FasliDate {
                year: 1_265,
                month: 7,
                day: 1,
                repeated: false,
            }),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(MADRAS.new_year(1_266), Ok(ymd(1856, 7, 1)));
        assert_eq!(MADRAS.new_year(1_265), Err(CalendarError::YearOutOfRange));
        assert_eq!(MADRAS.new_year(1_264), Err(CalendarError::YearOutOfRange));
    }

    #[test]
    fn every_reckoning_round_trips_and_refuses_outside_its_range() {
        for calendar in ALL {
            for rd in (ymd(1890, 1, 1).0..ymd(1895, 1, 1).0).step_by(crate::sweep_stride(7)) {
                let date = calendar.from_fixed(Rd(rd)).expect("in range");
                assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{}", calendar.id);
                let fields = Calendar::to_fields(calendar, date).expect("fields");
                assert_eq!(Calendar::from_fields(calendar, &fields), Ok(date));
            }
            let first = calendar.earliest().expect("placed");
            let last = calendar.latest().expect("placed");
            assert_eq!(
                calendar.from_fixed(Rd(first.0 - 1)),
                Err(CalendarError::BeforeEpoch)
            );
            assert_eq!(
                calendar.from_fixed(Rd(last.0 + 1)),
                Err(CalendarError::AfterSupportedRange)
            );
            assert!(calendar.from_fixed(first).is_ok());
            assert_eq!(
                calendar.from_fixed(last).map(|d| d.year),
                Ok(calendar.max_year())
            );
            assert_eq!(
                calendar.to_fixed(FasliDate {
                    year: calendar.max_year() + 1,
                    month: 1,
                    day: 1,
                    repeated: false,
                }),
                Err(CalendarError::YearOutOfRange)
            );
        }
        assert_eq!(
            MADRAS.to_fixed(FasliDate {
                year: 1_302,
                month: 2,
                day: 30,
                repeated: false,
            }),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            MADRAS.to_fixed(FasliDate {
                year: 1_302,
                month: 13,
                day: 1,
                repeated: false,
            }),
            Err(CalendarError::MonthOutOfRange)
        );
        let fields = Calendar::to_fields(
            &SUR_SAN,
            FasliDate {
                year: 1_293,
                month: 6,
                day: 5,
                repeated: false,
            },
        )
        .expect("fields");
        assert_eq!(fields.era, Some("sur-san"));
        let mut wrong = fields;
        wrong.era = Some(FASLI_ERA);
        assert_eq!(
            Calendar::from_fields(&SUR_SAN, &wrong),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn the_worked_example_of_15_june_2025() {
        let day = ymd(2025, 6, 15);
        assert_eq!(BOMBAY.from_fixed(day).map(|d| d.year), Ok(1_435));
        assert_eq!(SUR_SAN.from_fixed(day).map(|d| d.year), Ok(1_426));
        assert_eq!(MADRAS.from_fixed(day).map(|d| d.year), Ok(1_434));
    }
}
