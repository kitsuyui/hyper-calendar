//! The Discordian calendar.
//!
//! The calendar of page 00034 of the *Principia Discordia*: five seasons of
//! seventy-three days — Chaos, Discord, Confusion, Bureaucracy and The
//! Aftermath — a week of five days that starts afresh with Sweetmorn every
//! 1 January, and one intercalary day, St. Tib's Day, between Chaos 59 and
//! Chaos 60 in the years the Gregorian calendar has a 29 February, which
//! belongs to no season's count and no week. The year is the Year of Our
//! Lady of Discord, YOLD, and 1 YOLD is 1166 BC, so that the number is the
//! Gregorian year plus 1166.
//!
//! It is, in other words, a naming of the Gregorian day, as
//! [`crate::indian`] and [`crate::nanakshahi`] are: the same ordinal day of
//! the year has the same Discordian date every year, St. Tib's Day
//! absorbing the leap day, which is what lets the *Principia*'s eleven
//! holydays fall on fixed Gregorian dates — Mungday, Chaos 5, is 5 January
//! in every year, and Afflux, The Aftermath 50, is 8 December.
//!
//! Only those eleven days are named in the *Principia*; the holydays
//! Discordians have added since are not carried. Some users hold the
//! calendar to the Julian leap rule, on which it would part from the
//! Gregorian in 3266 YOLD (AD 2100); this module follows the Gregorian,
//! which is what `ddate` did.
//!
//! Source: Wikipedia, "Discordian calendar", retrieved 2026-09-22, for the
//! seasons, the week, St. Tib's Day, the era and the table of holydays.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{gregorian, ordinal};

/// How far the Year of Our Lady of Discord runs ahead of the Common Era:
/// 1 YOLD is 1166 BC.
pub const YEAR_OFFSET: i64 = 1_166;

/// The era code, the Year of Our Lady of Discord.
pub const ERA: &str = "YOLD";

/// The five seasons.
pub const SEASONS: [&str; 5] = [
    "Chaos",
    "Discord",
    "Confusion",
    "Bureaucracy",
    "The Aftermath",
];

/// The length of every season.
pub const DAYS_IN_SEASON: u8 = 73;

/// The five days of the Erisian week, named for the five elements.
pub const WEEKDAYS: [&str; 5] = [
    "Sweetmorn",
    "Boomtime",
    "Pungenday",
    "Prickle-Prickle",
    "Setting Orange",
];

/// The intercalary day, between Chaos 59 and Chaos 60.
pub const ST_TIBS_DAY: &str = "St. Tib's Day";

/// The Apostle Holydays, the fifth day of each season.
pub const APOSTLE_HOLYDAYS: [&str; 5] = ["Mungday", "Mojoday", "Syaday", "Zaraday", "Maladay"];

/// The Season Holydays, the fiftieth day of each season.
pub const SEASON_HOLYDAYS: [&str; 5] = ["Chaoflux", "Discoflux", "Confuflux", "Bureflux", "Afflux"];

/// The earliest year this implementation converts, 1 YOLD.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999 + YEAR_OFFSET;

/// Whether `year` has St. Tib's Day, which is exactly when the Gregorian
/// year it names has 29 February.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year - YEAR_OFFSET)
}

/// The number of days in `year`, St. Tib's Day included.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The number of days in `season`, or `None` when `season` is not in
/// `1..=5`. St. Tib's Day is not a day of Chaos and is not counted.
#[must_use]
pub const fn days_in_season(season: u8) -> Option<u8> {
    if season >= 1 && season <= 5 {
        Some(DAYS_IN_SEASON)
    } else {
        None
    }
}

/// The fixed day of 1 Chaos of `year`, which is 1 January.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    gregorian::new_year(year - YEAR_OFFSET)
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = match gregorian::new_year(MIN_YEAR - YEAR_OFFSET) {
    Ok(rd) => rd,
    Err(_) => Rd(i64::MIN),
};

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = match gregorian::new_year(MAX_YEAR + 1 - YEAR_OFFSET) {
    Ok(Rd(next)) => Rd(next - 1),
    Err(_) => Rd(i64::MAX),
};

/// The fixed day of a Discordian date. `st_tibs` names St. Tib's Day, for
/// which `season` and `day` must be Chaos 59, the day it follows.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`] —
/// the last also for St. Tib's Day in a year that has none, or placed
/// anywhere but after Chaos 59.
pub const fn to_fixed(year: i64, season: u8, day: u8, st_tibs: bool) -> CalendarResult<Rd> {
    if let Err(error) = crate::common::check_day(day, days_in_season(season)) {
        return Err(error);
    }
    let start = match new_year(year) {
        Ok(rd) => rd,
        Err(error) => return Err(error),
    };
    let leap = is_leap_year(year);
    // The day's place in the season count, 1 through 365.
    let counted = (season as i64 - 1) * DAYS_IN_SEASON as i64 + day as i64;
    if st_tibs {
        if !leap || season != 1 || day != 59 {
            return Err(CalendarError::DayOutOfRange);
        }
        return Ok(Rd(start.0 + 59));
    }
    // From Chaos 60 on, a leap year's days sit one later.
    let after_tibs = if leap && counted >= 60 { 1 } else { 0 };
    Ok(Rd(start.0 + counted - 1 + after_tibs))
}

/// The Discordian year, season, day and St. Tib's flag of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8, bool)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    let (gregorian_year, day_of_year) = match ordinal::from_fixed(rd) {
        Ok(parts) => parts,
        Err(error) => return Err(error),
    };
    let year = gregorian_year + YEAR_OFFSET;
    let leap = gregorian::is_leap_year(gregorian_year);
    if leap && day_of_year == 60 {
        return Ok((year, 1, 59, true));
    }
    let counted = if leap && day_of_year > 60 {
        day_of_year - 1
    } else {
        day_of_year
    };
    let season = ((counted - 1) / DAYS_IN_SEASON as u16 + 1) as u8;
    let day = ((counted - 1) % DAYS_IN_SEASON as u16 + 1) as u8;
    Ok((year, season, day, false))
}

/// A Discordian date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiscordianDate {
    /// The Year of Our Lady of Discord, counting from 1.
    pub year: i64,
    /// The season, 1 for Chaos through 5 for The Aftermath.
    pub season: u8,
    /// The day of the season, 1 through 73.
    pub day: u8,
    /// Whether this is St. Tib's Day, the intercalary day after Chaos 59,
    /// which the season and day then name as the day it follows.
    pub st_tibs: bool,
}

impl DiscordianDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(year: i64, season: u8, day: u8, st_tibs: bool) -> CalendarResult<Self> {
        match to_fixed(year, season, day, st_tibs) {
            Err(error) => Err(error),
            Ok(_) => Ok(Self {
                year,
                season,
                day,
                st_tibs,
            }),
        }
    }

    /// St. Tib's Day of `year`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when `year` has none.
    pub const fn st_tibs_day(year: i64) -> CalendarResult<Self> {
        Self::new(year, 1, 59, true)
    }

    /// The Common Era year this date falls in.
    #[must_use]
    pub const fn common_era_year(self) -> i64 {
        self.year - YEAR_OFFSET
    }

    /// The name of the season.
    #[must_use]
    pub const fn season_name(self) -> &'static str {
        SEASONS[self.season as usize - 1]
    }

    /// The day of the Erisian week, or `None` on St. Tib's Day, which is
    /// outside it. Every year begins with Sweetmorn.
    #[must_use]
    pub const fn weekday(self) -> Option<&'static str> {
        if self.st_tibs {
            return None;
        }
        let counted = (self.season as usize - 1) * DAYS_IN_SEASON as usize + self.day as usize;
        Some(WEEKDAYS[(counted - 1) % 5])
    }

    /// The holyday this date is, if it is one of the eleven the *Principia*
    /// names: an Apostle Holyday on the fifth of a season, a Season Holyday
    /// on the fiftieth, or St. Tib's Day.
    #[must_use]
    pub const fn holyday(self) -> Option<&'static str> {
        if self.st_tibs {
            Some(ST_TIBS_DAY)
        } else if self.day == 5 {
            Some(APOSTLE_HOLYDAYS[self.season as usize - 1])
        } else if self.day == 50 {
            Some(SEASON_HOLYDAYS[self.season as usize - 1])
        } else {
            None
        }
    }
}

/// The Discordian calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DiscordianCalendar;

/// Five named seasons in the place of months, and the five-day Erisian
/// week under its own cycle kind, since it is not the seven-day one.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &SEASONS),
    hc_calendar::shape::CycleShape::named("erisian-day", &WEEKDAYS),
];

impl Calendar for DiscordianCalendar {
    type Date = DiscordianDate;

    /// Five seasons as the month cycle, and the five-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("discordian"),
            english_name: "Discordian",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.season, date.day, date.st_tibs)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, season, day, st_tibs) = from_fixed(rd)?;
        Ok(DiscordianDate {
            year,
            season,
            day,
            st_tibs,
        })
    }

    /// St. Tib's Day is the intercalary repetition of Chaos 59, so it
    /// travels as [`DateFields::leap_day`].
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut fields = DateFields::ymd(date.year, date.season, date.day).with_era(ERA);
        fields.leap_day = date.st_tibs;
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        DiscordianDate::new(
            fields.year,
            month.ordinal,
            fields.require_day()?,
            fields.leap_day,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_year_is_the_gregorian_one_plus_1166_and_begins_with_sweetmorn() {
        let new_year = DiscordianCalendar
            .from_fixed(gregorian(2026, 1, 1))
            .unwrap();
        assert_eq!((new_year.year, new_year.season, new_year.day), (3192, 1, 1));
        assert_eq!(new_year.weekday(), Some("Sweetmorn"));
        assert_eq!(new_year.season_name(), "Chaos");
        assert_eq!(new_year.common_era_year(), 2026);
        // 1 YOLD is 1166 BC, astronomical year -1165.
        assert_eq!(to_fixed(1, 1, 1, false), gregorian::to_fixed(-1165, 1, 1));
    }

    #[test]
    fn the_eleven_holydays_fall_on_the_gregorian_dates_of_the_table() {
        // In a common year and in a leap year alike, since St. Tib's Day
        // takes the leap day.
        for year in [2025, 2024] {
            let holydays = [
                ("Mungday", 1, 5),
                ("Chaoflux", 2, 19),
                ("Mojoday", 3, 19),
                ("Discoflux", 5, 3),
                ("Syaday", 5, 31),
                ("Confuflux", 7, 15),
                ("Zaraday", 8, 12),
                ("Bureflux", 9, 26),
                ("Maladay", 10, 24),
                ("Afflux", 12, 8),
            ];
            for (name, month, day) in holydays {
                let date = DiscordianCalendar
                    .from_fixed(gregorian(year, month, day))
                    .unwrap();
                assert_eq!(date.holyday(), Some(name), "{year}-{month}-{day}");
                assert!(date.day == 5 || date.day == 50);
            }
        }
        let st_tibs = DiscordianCalendar
            .from_fixed(gregorian(2024, 2, 29))
            .unwrap();
        assert!(st_tibs.st_tibs);
        assert_eq!(st_tibs.holyday(), Some(ST_TIBS_DAY));
        assert_eq!(st_tibs.weekday(), None);
        assert_eq!((st_tibs.season, st_tibs.day), (1, 59));
        assert_eq!(DiscordianDate::st_tibs_day(2024 + YEAR_OFFSET), Ok(st_tibs));
        assert_eq!(
            DiscordianDate::st_tibs_day(2025 + YEAR_OFFSET),
            Err(CalendarError::DayOutOfRange)
        );
        // Chaos 59 is 28 February, Chaos 60 is 1 March, in both kinds of
        // year, and Chaos 73 is 14 March.
        for year in [2024, 2025] {
            assert_eq!(
                to_fixed(year + YEAR_OFFSET, 1, 59, false),
                Ok(gregorian(year, 2, 28))
            );
            assert_eq!(
                to_fixed(year + YEAR_OFFSET, 1, 60, false),
                Ok(gregorian(year, 3, 1))
            );
            assert_eq!(
                to_fixed(year + YEAR_OFFSET, 1, 73, false),
                Ok(gregorian(year, 3, 14))
            );
            assert_eq!(
                to_fixed(year + YEAR_OFFSET, 2, 1, false),
                Ok(gregorian(year, 3, 15))
            );
            assert_eq!(
                to_fixed(year + YEAR_OFFSET, 5, 73, false),
                Ok(gregorian(year, 12, 31))
            );
        }
    }

    #[test]
    fn the_week_runs_seventy_three_times_and_ends_on_setting_orange() {
        for year in [2024, 2025] {
            let last = DiscordianCalendar
                .from_fixed(gregorian(year, 12, 31))
                .unwrap();
            assert_eq!(last.weekday(), Some("Setting Orange"), "{year}");
            let fifth = DiscordianCalendar
                .from_fixed(gregorian(year, 1, 5))
                .unwrap();
            assert_eq!(fifth.weekday(), Some("Setting Orange"));
            let sixth = DiscordianCalendar
                .from_fixed(gregorian(year, 1, 6))
                .unwrap();
            assert_eq!(sixth.weekday(), Some("Sweetmorn"));
        }
        // St. Tib's Day does not advance the week: 1 March is Pungenday
        // either side of it.
        for year in [2024, 2025] {
            let march = DiscordianCalendar
                .from_fixed(gregorian(year, 3, 1))
                .unwrap();
            assert_eq!(march.weekday(), Some("Setting Orange"), "{year}");
        }
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        let calendar = DiscordianCalendar;
        for rd in (EARLIEST.0..=EARLIEST.0 + 4_000_000).step_by(41) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.leap_day, date.st_tibs);
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
        }
        let start = gregorian(2023, 1, 1).0;
        for rd in start..gregorian(2026, 1, 1).0 {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
        }
        for rd in LATEST.0 - 400..=LATEST.0 {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
        }
        assert_eq!(days_in_year(2024 + YEAR_OFFSET), 366);
        assert_eq!(days_in_year(2025 + YEAR_OFFSET), 365);
    }

    #[test]
    fn impossible_dates_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(0, 1, 1, false), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            to_fixed(3192, 6, 1, false),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(3192, 1, 74, false),
            Err(CalendarError::DayOutOfRange)
        );
        // St. Tib's Day anywhere but after Chaos 59 is not a date.
        assert_eq!(
            to_fixed(3190, 1, 60, true),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(3190, 2, 59, true),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            DiscordianCalendar.from_fields(&DateFields::ymd(3192, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(DiscordianCalendar.meta().id, CalendarId("discordian"));
    }
}
