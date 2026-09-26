//! The 364-day year of the Qumran scrolls and *Jubilees*, with the
//! *mishmarot*.
//!
//! The calendar of the Dead Sea Scrolls' calendrical texts (4Q319–330) and
//! of the Book of Jubilees: twelve numbered months grouped in four quarters
//! of 30, 30 and 31 days, 91 days or exactly thirteen weeks each, so that
//! the year and every quarter begin on the fourth day of the week — the day
//! the luminaries were created — every Sabbath falls on the same dates and
//! every festival on the same weekday. No intercalation is specified in any
//! text: the year is 364 days, always, and so it **drifts against the
//! seasons by a day and a quarter a year**, a month in 25 years. This
//! module carries it exactly so, as a pure week calendar, and makes no
//! claim about the seasons; whether and how the community corrected it is
//! contested (Glessmer reads a leap week into 4Q319, Beckwith thinks none
//! was wanted), and none of the proposals is carried.
//!
//! Against it the scrolls run the **mishmarot**, the weekly service of the
//! twenty-four priestly courses of 1 Chronicles 24:7–18, which the texts
//! trace to the creation, "on the fourth \[day\] of Ga\[mul\]" (4Q319 i.11),
//! Gamul being "at the head of all years": the week containing the first
//! day of the first year is Gamul's, and the courses follow one another a
//! week at a time, so that after six years of 52 weeks, 312 = 13 × 24, the
//! first year comes round with Gamul again. The rotation is carried as the
//! `mishmar` cycle with Talmon's spellings of the names, each course's
//! week running from the first day to the Sabbath, as the texts date "on
//! the third in the week of the sons of Maaziah". 4Q325 instead names each
//! Sabbath after the course that enters that afternoon and serves from the
//! next morning; [`entering_course`] gives that name.
//!
//! # The epoch is not the scrolls'
//!
//! Nothing in the texts, and nothing read, ties this year to a Julian day:
//! a calendar that drifts through the seasons has no fixed place in them,
//! and the scholars read reconstruct its internal structure, not its
//! correlation. So the fixed day of year 1, day 1 is **this library's
//! convention**, [`EPOCH`], the Wednesday on or after 21 March of AD 1
//! (proleptic Julian), chosen only to be a Wednesday near an equinox at
//! the turn of the era; years before it count 0, −1 and so on. The years
//! of the six-year cycle, which the texts do count, follow from it: year 1
//! is Gamul's. A date here is right about the weekday, the month, the
//! festival and the course, and says nothing about the Julian date the
//! community would have written, for which no source exists. The system
//! document is `docs/systems/qumran.md`.
//!
//! # Sources
//!
//! * Shemaryahu Talmon, "Calendars and Mishmarot", in Lawrence H. Schiffman
//!   and James C. VanderKam (eds.), *Encyclopedia of the Dead Sea Scrolls*,
//!   vol. 1 (Oxford University Press, 2000), pp. 108–117, read in the scan
//!   at web.tusculum.edu: p. 110 for the Sabbaths of the second and third
//!   months (4Q320–321), "The first and fifteenth days of the first month
//!   of each quarter fall invariably on the fourth day of the week", the
//!   festivals and their weekdays, and 4Q319 i.11; p. 111 for the rotation
//!   of the courses from Gamul, the courses at the head of each of the six
//!   years (4Q328–329), the festivals of the first year by course (4Q320
//!   4.ii) and the Sabbaths "named after the watches that enter the Temple
//!   on Saturday afternoon" (4Q325).
//! * Wikipedia, "Qumran calendrical texts", retrieved 2026-09-26, for the
//!   30-30-31 quarters, the year of Jubilees, and the intercalation
//!   debate as reported from VanderKam (1998) and Stern (2000), not read.
//!
//! # Exactness
//!
//! Exact as arithmetic; the correlation to other calendars is a convention.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::julian;

/// The calendar identifier.
pub const ID: &str = "qumran";

/// The days of every year.
pub const DAYS_IN_YEAR: i64 = 364;

/// The twenty-four priestly courses in the order of 1 Chronicles 24:7–18,
/// in Talmon's spellings.
pub const COURSES: [&str; 24] = [
    "Joiarib",
    "Jedaiah",
    "Ḥarim",
    "Seorim",
    "Malkiah",
    "Mijamin",
    "Haqqots",
    "Abiah",
    "Jeshua",
    "Shekaniah",
    "Eliashib",
    "Jaqim",
    "Ḥuppah",
    "Jeshbeab",
    "Bilgah",
    "Immer",
    "Ḥezir",
    "Happittet",
    "Petaḥaiah",
    "Jehezkel",
    "Jakin",
    "Gamul",
    "Delaiah",
    "Maaziah",
];

/// The course serving the week the first year opens in, Gamul, as a
/// position in [`COURSES`] from 1.
pub const GAMUL: u8 = 22;

/// The years of the *mishmarot* cycle.
pub const CYCLE_YEARS: i64 = 6;

/// Year 1, day 1: this library's convention, the Wednesday on or after
/// 21 March AD 1, proleptic Julian. See the module documentation.
pub const EPOCH: Rd = match julian::to_fixed(1, 3, 21) {
    Ok(rd) => Weekday::Wednesday.on_or_after(rd),
    Err(_) => Rd(0),
};

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = -99_999;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 99_999;

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(EPOCH.0 + DAYS_IN_YEAR * (MIN_YEAR - 1));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(EPOCH.0 + DAYS_IN_YEAR * MAX_YEAR - 1);

/// The days in `month`: 31 in the third month of each quarter, else 30;
/// `None` outside `1..=12`.
#[must_use]
pub const fn days_in_month(month: u8) -> Option<u8> {
    if month == 0 || month > 12 {
        None
    } else if month.is_multiple_of(3) {
        Some(31)
    } else {
        Some(30)
    }
}

/// The fixed day of a date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    if let Err(error) = crate::common::check_day(day, days_in_month(month)) {
        return Err(error);
    }
    let quarters = (month as i64 - 1) / 3;
    let within = (month as i64 - 1) % 3;
    Ok(Rd(EPOCH.0
        + DAYS_IN_YEAR * (year - 1)
        + 91 * quarters
        + 30 * within
        + day as i64
        - 1))
}

/// The year, month and day of a fixed day.
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
    let elapsed = rd.0 - EPOCH.0;
    let year = elapsed.div_euclid(DAYS_IN_YEAR) + 1;
    let day_of_year = elapsed.rem_euclid(DAYS_IN_YEAR);
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

/// The year of the six-year *mishmarot* cycle, 1 to 6; 1 is Gamul's.
#[must_use]
pub const fn cycle_year(year: i64) -> u8 {
    ((year - 1).rem_euclid(CYCLE_YEARS) + 1) as u8
}

/// The course serving the week of `rd`, as a position in [`COURSES`] from
/// 1: the week from the first day to the Sabbath.
#[must_use]
pub const fn course(rd: Rd) -> u8 {
    let first_week = Weekday::Sunday.on_or_before(EPOCH).0;
    let weeks = (Weekday::Sunday.on_or_before(rd).0 - first_week).div_euclid(7);
    ((GAMUL as i64 - 1 + weeks).rem_euclid(24) + 1) as u8
}

/// The course that enters on the afternoon of `rd`, if it is a Sabbath, by
/// whose name 4Q325 calls that Sabbath: the course of the next week.
#[must_use]
pub const fn entering_course(rd: Rd) -> Option<u8> {
    if matches!(Weekday::from_rd(rd), Weekday::Saturday) {
        Some(course(Rd(rd.0 + 1)))
    } else {
        None
    }
}

/// The day of the week as the scrolls number it: 1 for the first day,
/// Sunday, to 7 for the Sabbath.
#[must_use]
pub const fn day_of_week(weekday: Weekday) -> u8 {
    weekday.sunday_first_number() + 1
}

/// A date of the 364-day year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QumranDate {
    /// The year from this library's [`EPOCH`].
    pub year: i64,
    /// The month, 1 to 12.
    pub month: u8,
    /// The day, 1 to 30, or 31 in months 3, 6, 9 and 12.
    pub day: u8,
}

impl QumranDate {
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

    /// The weekday, which depends on the month and day alone: day 1 of
    /// every quarter is the fourth day of the week, Wednesday.
    #[must_use]
    pub const fn weekday(self) -> Weekday {
        let day_of_year = 91 * ((self.month as i64 - 1) / 3)
            + 30 * ((self.month as i64 - 1) % 3)
            + self.day as i64
            - 1;
        match day_of_year % 7 {
            0 => Weekday::Wednesday,
            1 => Weekday::Thursday,
            2 => Weekday::Friday,
            3 => Weekday::Saturday,
            4 => Weekday::Sunday,
            5 => Weekday::Monday,
            _ => Weekday::Tuesday,
        }
    }
}

/// The 364-day year.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct QumranCalendar;

/// Twelve numbered months, the seven-day week, and the twenty-four
/// courses by name.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::MONTH, 12),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
    hc_calendar::shape::CycleShape::named("mishmar", &COURSES),
];

impl Calendar for QumranCalendar {
    type Date = QumranDate;

    /// Unrecorded: whether the calendar was kept in practice, and when, is
    /// contested, and no source read dates a period — let alone one on the
    /// Julian days this module's convention puts it on.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve months, the week, and the priestly courses.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// Never: no intercalation is specified.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(false)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Qumran / Jubilees 364-day year",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["he", "arc"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(QumranDate { year, month, day })
    }

    /// The date, the day of the week as the scrolls number it (1 for the
    /// first day, 7 for the Sabbath), the course of the week, and the year
    /// of the six-year cycle.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let rd = self.to_fixed(date)?;
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("day-of-week", i64::from(day_of_week(date.weekday())))?
            .with_extra("mishmar", i64::from(course(rd)))?
            .with_extra("cycle-year", i64::from(cycle_year(date.year)))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        QumranDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i64, month: u8, day: u8) -> Rd {
        to_fixed(year, month, day).unwrap()
    }

    fn named(position: u8) -> &'static str {
        COURSES[position as usize - 1]
    }

    #[test]
    fn every_quarter_begins_on_the_fourth_day_of_the_week() {
        assert_eq!(Weekday::from_rd(EPOCH), Weekday::Wednesday);
        for year in [-5_000, -150, 0, 1, 2, 7, 2_026] {
            for month in [1u8, 4, 7, 10] {
                // "The first and fifteenth days of the first month of each
                // quarter fall invariably on the fourth day of the week."
                assert_eq!(Weekday::from_rd(day(year, month, 1)), Weekday::Wednesday);
                assert_eq!(Weekday::from_rd(day(year, month, 15)), Weekday::Wednesday);
                assert_eq!(
                    QumranDate::new(year, month, 1).unwrap().weekday(),
                    Weekday::Wednesday
                );
            }
        }
    }

    /// The Sabbaths and festivals Talmon quotes and dates, p. 110.
    #[test]
    fn the_sabbaths_and_festivals_fall_where_the_scrolls_put_them() {
        let sabbaths = [
            (2u8, 23u8),
            (2, 30),
            (3, 7),
            (3, 14),
            (3, 21),
            (3, 28),
            (6, 21),
        ];
        for year in [1, 2, 1_000] {
            for (month, date) in sabbaths {
                assert_eq!(
                    Weekday::from_rd(day(year, month, date)),
                    Weekday::Saturday,
                    "{month}/{date}"
                );
            }
            // "After it, the first and second day [of the week] a day is
            // added and the quarter terminates [with] ninety-one days."
            assert_eq!(Weekday::from_rd(day(year, 3, 30)), Weekday::Monday);
            assert_eq!(Weekday::from_rd(day(year, 3, 31)), Weekday::Tuesday);
            assert_eq!(day(year, 4, 1).0 - day(year, 1, 1).0, 91);
            // The Passover lamb on Tuesday the fourteenth, Passover on
            // Wednesday the fifteenth, Shavuot on Sunday 15/III, fifty days
            // after Sunday 26/I; Yom Kippur on Friday 10/VII.
            assert_eq!(Weekday::from_rd(day(year, 1, 14)), Weekday::Tuesday);
            assert_eq!(Weekday::from_rd(day(year, 1, 26)), Weekday::Sunday);
            assert_eq!(Weekday::from_rd(day(year, 3, 15)), Weekday::Sunday);
            assert_eq!(day(year, 3, 15).0 - day(year, 1, 26).0, 49);
            assert_eq!(Weekday::from_rd(day(year, 7, 10)), Weekday::Friday);
            assert_eq!(Weekday::from_rd(day(year, 7, 15)), Weekday::Wednesday);
        }
    }

    /// 4Q320 4.ii as Talmon restores it: the festivals of the first year by
    /// the day of the week and the course.
    #[test]
    fn the_first_years_festivals_fall_in_the_courses_of_4q320() {
        let cases = [
            ((1u8, 14u8), 3u8, "Maaziah"), // Pesah
            ((1, 26), 1, "Jedaiah"),       // the Omer
            ((2, 14), 5, "Seorim"),        // the Second Pesah
            ((3, 15), 1, "Jeshua"),        // the Feast of Weeks
            ((7, 1), 4, "Maaziah"),        // the Day of Remembrance
            ((7, 10), 6, "Joiarib"),       // the Day of Atonement
            ((7, 15), 4, "Jedaiah"),       // the Feast of Booths
        ];
        for ((month, date), weekday, name) in cases {
            let rd = day(1, month, date);
            assert_eq!(day_of_week(Weekday::from_rd(rd)), weekday);
            assert_eq!(named(course(rd)), name, "{month}/{date}");
        }
        // 4Q319, 4Q320: the first day is "the fourth in the week of the sons
        // of Gamul".
        assert_eq!(named(course(EPOCH)), "Gamul");
    }

    /// 4Q328–329 as Talmon restores them: the courses at the head of the six
    /// years, and the cycle closing after 312 weeks.
    #[test]
    fn the_six_years_are_headed_by_the_courses_of_4q329() {
        let heads = [
            "Gamul",
            "Jedaiah",
            "Mijamin",
            "Shekaniah",
            "Jeshbeab",
            "Happittet",
        ];
        for base in [1i64, 7, 601, -5] {
            for (offset, name) in heads.iter().enumerate() {
                let year = base + offset as i64;
                assert_eq!(named(course(day(year, 1, 1))), *name, "{year}");
                assert_eq!(cycle_year(year) as usize, offset + 1);
            }
        }
        // And each course serves 13 weeks in the six years.
        let mut served = [0u32; 24];
        for week in 0..312 {
            served[course(Rd(EPOCH.0 + 7 * week)) as usize - 1] += 1;
        }
        assert!(served.iter().all(|weeks| *weeks == 13), "{served:?}");
    }

    /// 4Q325: "The beginning of the second month is on the sixth day of the
    /// course of Jedaiah. On the second of the month is the Sabbath of the
    /// course of Harim."
    #[test]
    fn the_sabbath_is_named_for_the_course_that_enters() {
        let first = day(1, 2, 1);
        assert_eq!(Weekday::from_rd(first), Weekday::Friday);
        assert_eq!(named(course(first)), "Jedaiah");
        let sabbath = day(1, 2, 2);
        assert_eq!(entering_course(sabbath).map(named), Some("Ḥarim"));
        assert_eq!(named(course(sabbath)), "Jedaiah");
        assert_eq!(entering_course(first), None);
    }

    #[test]
    fn the_months_are_thirty_thirty_thirty_one() {
        let lengths: Vec<u8> = (1..=12)
            .map(|month| days_in_month(month).unwrap())
            .collect();
        assert_eq!(lengths, [30, 30, 31, 30, 30, 31, 30, 30, 31, 30, 30, 31]);
        assert_eq!(
            lengths.iter().map(|days| u32::from(*days)).sum::<u32>(),
            364
        );
        assert_eq!(days_in_month(0), None);
        assert_eq!(days_in_month(13), None);
        assert_eq!(to_fixed(1, 1, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(1, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(QumranCalendar.is_leap_year(2), Ok(false));
    }

    /// The price of no intercalation, stated rather than hidden: the first
    /// day of the year runs a day and a quarter earlier through the Julian
    /// year every year.
    #[test]
    fn the_year_drifts_against_the_seasons() {
        let first = day(1, 1, 1).0;
        let hundredth = day(101, 1, 1).0;
        let julian_years = f64::from(i32::try_from(hundredth - first).unwrap()) / 365.25;
        assert!((julian_years - 99.66).abs() < 0.01, "{julian_years}");
        let (_, month, _) = julian::from_fixed(Rd(hundredth)).unwrap();
        assert_ne!(
            month, 3,
            "a hundred years on, the year no longer opens in March"
        );
    }

    #[test]
    fn every_day_round_trips() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(7_919) {
            let (year, month, date) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, date), Ok(Rd(rd)), "rd {rd}");
            assert_eq!(
                QumranDate {
                    year,
                    month,
                    day: date
                }
                .weekday(),
                Weekday::from_rd(Rd(rd))
            );
        }
        let calendar = QumranCalendar;
        for rd in EPOCH.0 - 800..EPOCH.0 + 3_000 {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(from_fixed(EARLIEST), Ok((MIN_YEAR, 1, 1)));
        assert_eq!(from_fixed(LATEST), Ok((MAX_YEAR, 12, 31)));
    }
}
