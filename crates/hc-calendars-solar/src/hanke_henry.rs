//! The Hanke–Henry Permanent Calendar.
//!
//! Steve H. Hanke and Richard Conn Henry's proposal: a 364-day year of four
//! quarters of 30, 30 and 31 days, so that March, June, September and
//! December have 31 days and every date falls on the same weekday every
//! year, the year always beginning on **Monday 1 January**; and a
//! seven-day "mini-month" *Xtr* at the end of December in the years whose
//! Gregorian year "begins or ends on a Thursday" — the years with 53 ISO
//! weeks. The year therefore begins on the Monday that opens ISO week 1,
//! and its number is the ISO week-numbering year: this is ISO 8601's year
//! with the weeks cut into months.
//!
//! *Xtr* is numbered month 13, as the authors' own converter numbers it.
//! Its seven days are in the week like every other day: the proposal's
//! point is that the seven-day cycle is never broken, so the calendar's
//! weekday and [`hc_calendar::Weekday::from_rd`] always agree, unlike the
//! blank days of [`crate::world_calendar`] and [`crate::international_fixed`].
//!
//! The proposal has had two earlier forms, both superseded by its authors
//! and neither carried: Henry's of 2004, whose leap week *Newton* sat
//! between June and July, and the 2011 revision whose year began on a
//! Sunday; the current form moved the start to Monday for 2024. Those two
//! are known here only from Wikipedia. The system document is
//! `docs/systems/hanke-henry.md`.
//!
//! # Sources
//!
//! * Hanke and Henry, "The Hanke-Henry Permanent Calendar",
//!   hankehenryontime.com, `html/calendar.html`, retrieved 2026-09-26:
//!   "The first two months of each quarter are made up of 30 days, and the
//!   third is made up of 31 days", the 364-day year of 52 weeks, and the
//!   list of the years "in which there is a one-week long 'MiniMonth'
//!   called Xtr, at the end of December", 2026 to 3001; `index.html`:
//!   "every year begins on Monday, January 1"; `html/qanda.html`: the test
//!   "if the corresponding Gregorian year begins or ends on a Thursday, that
//!   year contains an Xtr month", credited to Irv Bromberg, 1 January 2024
//!   as a Monday in both calendars, and a 7 March "always" on a Thursday;
//!   `scripts/newconverter.js`: Xtr as month 13, 1 January 2018 as the base
//!   day, and the Xtr years from 1970.
//! * Wikipedia, "Hanke–Henry Permanent Calendar", retrieved 2026-09-26, for
//!   the 2004 *Newton* week between June and July and the Sunday start
//!   before 2016. Henry's page at `henry.pha.jhu.edu` refused to be read.
//!
//! # Exactness
//!
//! Exact — the ISO week arithmetic, proleptic before 2024.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::gregorian;

/// The calendar identifier.
pub const ID: &str = "hanke-henry";

/// The months: the Gregorian twelve and *Xtr*, the leap week.
pub const MONTHS: [&str; 13] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
    "Xtr",
];

/// The month number of *Xtr*, as the authors' converter numbers it.
pub const XTR: u8 = 13;

/// The lengths of the twelve months: 30, 30, 31 in every quarter.
const MONTH_DAYS: [u8; 12] = [30, 30, 31, 30, 30, 31, 30, 30, 31, 30, 30, 31];

/// The days of *Xtr*.
pub const XTR_DAYS: u8 = 7;

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 99_999;

/// The first day of `year`, unchecked: the Monday on or before 4 January,
/// which opens ISO week 1.
const fn new_year_raw(year: i64) -> Rd {
    match gregorian::to_fixed(year, 1, 4) {
        Ok(fourth) => Weekday::Monday.on_or_before(fourth),
        Err(_) => Rd(0),
    }
}

/// The fixed day of 1 January of `year`, always a Monday.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(new_year_raw(year))
}

/// Whether `year` has *Xtr*: whether the Gregorian year begins or ends on
/// a Thursday, which is whether it has 53 ISO weeks.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    new_year_raw(year + 1).0 - new_year_raw(year).0 == 371
}

/// The number of days in `year`: 364, or 371 with *Xtr*.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 371 } else { 364 }
}

/// The number of days in `month` of `year`, or `None` when the month does
/// not exist — month 13 exists only in a year with *Xtr*.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if month >= 1 && month <= 12 {
        Some(MONTH_DAYS[month as usize - 1])
    } else if month == XTR && is_leap_year(year) {
        Some(XTR_DAYS)
    } else {
        None
    }
}

/// Days elapsed in the year before the first of `month`.
const fn days_before_month(month: u8) -> i64 {
    let quarters = (month as i64 - 1) / 3;
    let within = (month as i64 - 1) % 3;
    91 * quarters + 30 * within
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = new_year_raw(MIN_YEAR);

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1).0 - 1);

/// The fixed day of a Hanke–Henry date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] — also for *Xtr* in a year without
/// one — or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    let start = match new_year(year) {
        Ok(start) => start,
        Err(error) => return Err(error),
    };
    match crate::common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(start.0 + days_before_month(month) + day as i64 - 1)),
    }
}

/// The Hanke–Henry year, month and day of a fixed day.
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
    // Four days on is always inside the Gregorian year whose number the
    // day's year carries: the year opens at most three days before
    // 1 January and closes at most three days after 31 December.
    let mut year = match gregorian::year_from_fixed(Rd(rd.0 + 3)) {
        Ok(year) => year,
        Err(error) => return Err(error),
    };
    if new_year_raw(year).0 > rd.0 {
        year -= 1;
    }
    let day_of_year = rd.0 - new_year_raw(year).0;
    if day_of_year >= 364 {
        return Ok((year, XTR, (day_of_year - 364 + 1) as u8));
    }
    let quarter = day_of_year / 91;
    let within = day_of_year % 91;
    let (month_in_quarter, day) = if within < 30 {
        (0, within)
    } else if within < 60 {
        (1, within - 30)
    } else {
        (2, within - 60)
    };
    Ok((
        year,
        (3 * quarter + month_in_quarter + 1) as u8,
        (day + 1) as u8,
    ))
}

/// A Hanke–Henry date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HankeHenryDate {
    /// The year, which is the ISO week-numbering year.
    pub year: i64,
    /// The month, 1 for January through 12 for December, 13 for *Xtr*.
    pub month: u8,
    /// The day of the month, 1 through 30 or 31, or 7 in *Xtr*.
    pub day: u8,
}

impl HankeHenryDate {
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

    /// Whether this date is in *Xtr*.
    #[must_use]
    pub const fn is_xtr(self) -> bool {
        self.month == XTR
    }

    /// The weekday, which depends on the month and day alone: every year
    /// begins on a Monday and is a whole number of weeks.
    #[must_use]
    pub const fn weekday(self) -> Weekday {
        let day_of_year = days_before_month(self.month) + self.day as i64 - 1;
        match day_of_year % 7 {
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            _ => Weekday::Sunday,
        }
    }
}

/// The Hanke–Henry Permanent Calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HankeHenryCalendar;

/// Twelve months and *Xtr* in the years that have it, named here because
/// the calendar was defined in English and Xtr is its own word; a locale
/// supplies its own words for the twelve. And the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape {
        kind: hc_calendar::shape::MONTH,
        length: hc_calendar::shape::CycleLength::Intercalary {
            ordinary: 12,
            extended: 13,
        },
        names: &MONTHS,
    },
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for HankeHenryCalendar {
    type Date = HankeHenryDate;

    /// Unrecorded: a proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve months and *Xtr*, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// A year with *Xtr*.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Hanke–Henry Permanent",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &[],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(HankeHenryDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("day-of-week", i64::from(date.weekday().iso_number()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        HankeHenryDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iso_week;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    /// The years the authors' calendar page lists as having Xtr, 2026 to
    /// 3001, and the earlier ones their converter carries.
    const XTR_YEARS: &[i64] = &[
        1970, 1976, 1981, 1987, 1992, 1998, 2004, 2009, 2015, 2020, 2026, 2032, 2037, 2043, 2048,
        2054, 2060, 2065, 2071, 2076, 2082, 2088, 2093, 2099, 2105, 2111, 2116, 2122, 2128, 2133,
        2139, 2144, 2150, 2156, 2161, 2167, 2172, 2178, 2184, 2189, 2195, 2201, 2207, 2212, 2218,
        2224, 2229, 2235, 2240, 2246, 2252, 2257, 2263, 2268, 2274, 2280, 2285, 2291, 2296, 2303,
        2308, 2314, 2320, 2325, 2331, 2336, 2342, 2348, 2353, 2359, 2364, 2370, 2376, 2381, 2387,
        2392, 2398, 2404, 2409, 2415, 2420, 2426, 2432, 2437, 2443, 2448, 2454, 2460, 2465, 2471,
        2476, 2482, 2488, 2493, 2499, 2505, 2511, 2516, 2522, 2528, 2533, 2539, 2544, 2550, 2556,
        2561, 2567, 2572, 2578, 2584, 2589, 2595, 2601, 2607, 2612, 2618, 2624, 2629, 2635, 2640,
        2646, 2652, 2657, 2663, 2668, 2674, 2680, 2685, 2691, 2696, 2703, 2708, 2714, 2720, 2725,
        2731, 2736, 2742, 2748, 2753, 2759, 2764, 2770, 2776, 2781, 2787, 2792, 2798, 2804, 2809,
        2815, 2820, 2826, 2832, 2837, 2843, 2848, 2854, 2860, 2865, 2871, 2876, 2882, 2888, 2893,
        2899, 2905, 2911, 2916, 2922, 2928, 2933, 2939, 2944, 2950, 2956, 2961, 2967, 2972, 2978,
        2984, 2989, 2995, 3001,
    ];

    #[test]
    fn the_xtr_years_are_the_ones_the_authors_list() {
        for year in 1970..=3001 {
            assert_eq!(
                is_leap_year(year),
                XTR_YEARS.contains(&year),
                "{year}: the authors' list and the rule disagree"
            );
        }
    }

    /// The rule the authors credit to Bromberg is the ISO long year.
    #[test]
    fn xtr_is_where_the_gregorian_year_begins_or_ends_on_a_thursday() {
        for year in (1..8_000).step_by(3) {
            let begins = Weekday::from_rd(gregorian(year, 1, 1)) == Weekday::Thursday;
            let ends = Weekday::from_rd(gregorian(year, 12, 31)) == Weekday::Thursday;
            assert_eq!(is_leap_year(year), begins || ends, "{year}");
            assert_eq!(Ok(is_leap_year(year)), iso_week::is_long_year(year));
            assert_eq!(new_year(year), iso_week::week_one_start(year));
        }
        // 71 in every 400 years, as in ISO 8601.
        assert_eq!((2000..2400).filter(|year| is_leap_year(*year)).count(), 71);
    }

    #[test]
    fn every_year_begins_on_monday_1_january() {
        // The converter's base day and the transition day of the Q&A are
        // the Gregorian 1 January.
        assert_eq!(to_fixed(2018, 1, 1), Ok(gregorian(2018, 1, 1)));
        assert_eq!(to_fixed(2024, 1, 1), Ok(gregorian(2024, 1, 1)));
        // Otherwise it is the Monday nearest it: 2027 opens on 4 January,
        // after 2026's Xtr.
        assert_eq!(to_fixed(2027, 1, 1), Ok(gregorian(2027, 1, 4)));
        assert_eq!(to_fixed(2026, 13, 7), Ok(gregorian(2027, 1, 3)));
        for year in (1..5_000).step_by(7) {
            let start = new_year(year).unwrap();
            assert_eq!(Weekday::from_rd(start), Weekday::Monday, "{year}");
        }
    }

    #[test]
    fn a_date_falls_on_the_same_weekday_every_year() {
        // "If, for example, your birthday is March 7, it will always fall
        // on a Thursday."
        for year in [1970, 2024, 2025, 2026, 2100, 4000] {
            let date = HankeHenryDate::new(year, 3, 7).unwrap();
            assert_eq!(date.weekday(), Weekday::Thursday);
            assert_eq!(
                Weekday::from_rd(to_fixed(year, 3, 7).unwrap()),
                Weekday::Thursday
            );
        }
        // The calendar's weekday is the unbroken week's, Xtr included.
        for rd in (EARLIEST.0..EARLIEST.0 + 800_000).step_by(89) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            let date = HankeHenryDate { year, month, day };
            assert_eq!(date.weekday(), Weekday::from_rd(Rd(rd)), "rd {rd}");
        }
        // Every quarter opens on a Monday and ends on a Sunday.
        for month in [1u8, 4, 7, 10] {
            assert_eq!(
                HankeHenryDate::new(2026, month, 1).unwrap().weekday(),
                Weekday::Monday
            );
            assert_eq!(
                HankeHenryDate::new(2026, month + 2, 31).unwrap().weekday(),
                Weekday::Sunday
            );
        }
    }

    #[test]
    fn the_months_are_thirty_thirty_thirty_one_and_xtr_seven() {
        let lengths: Vec<u8> = (1..=12)
            .map(|month| days_in_month(2025, month).unwrap())
            .collect();
        assert_eq!(lengths, MONTH_DAYS);
        assert_eq!(days_in_month(2025, 13), None);
        assert_eq!(days_in_month(2026, 13), Some(7));
        assert_eq!(days_in_month(2026, 0), None);
        assert_eq!(days_in_month(2026, 14), None);
        assert_eq!(days_in_year(2025), 364);
        assert_eq!(days_in_year(2026), 371);
        assert_eq!(to_fixed(2025, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(2025, 2, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2026, 13, 8), Err(CalendarError::DayOutOfRange));
        assert!(HankeHenryDate::new(2026, 13, 7).unwrap().is_xtr());
    }

    /// "The new calendar is never more than five days off from the
    /// seasons": a date is never more than five days from the Gregorian
    /// date of the same name.
    #[test]
    fn a_date_is_never_more_than_five_days_from_its_gregorian_namesake() {
        for year in 2024..2424 {
            for month in 1..=12u8 {
                let last = gregorian::days_in_month(year, month).unwrap().min(30);
                for day in 1..=last {
                    let here = to_fixed(year, month, day).unwrap().0;
                    let there = gregorian(year, month, day).0;
                    assert!((here - there).abs() <= 5, "{year}-{month}-{day}");
                }
            }
        }
    }

    #[test]
    fn every_day_round_trips() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(4_999) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
        let start = gregorian(2019, 12, 1).0;
        for rd in start..start + 400 * 7 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
        for rd in [EARLIEST.0, EARLIEST.0 + 1, LATEST.0 - 1, LATEST.0] {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn days_outside_the_range_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            HankeHenryCalendar.is_leap_year(MAX_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = HankeHenryCalendar;
        for rd in (700_000..=760_000).step_by(37) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
        }
        assert_eq!(calendar.cycles()[0].names[12], "Xtr");
        assert_eq!(
            calendar.from_fields(&DateFields::ymd_leap_month(2026, 1, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
    }
}
