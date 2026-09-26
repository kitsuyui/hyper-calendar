//! The Soviet revolutionary weeks, 1929–1940.
//!
//! From 1929 to 1940 the Soviet Union kept the Gregorian calendar — its
//! months, their lengths and its year — and replaced the week. Pravda
//! printed 31 January and 31 March throughout and never a 30 February; the
//! thirty-day "Soviet revolutionary calendar" proposed in February 1930 was
//! rejected. What changed was the working week, by decree:
//!
//! 1. **The continuous five-day week** (*nepreryvka*), under Sovnarkom's
//!    decree of 26 August 1929 on continuous production, begun with the
//!    economic year on 1 October 1929, with its rules set by the decree of
//!    24 September 1929: four days of work and one of rest, the rest day
//!    staggered across five groups of workers, so that the day of the
//!    week named a *group* at rest and no day was a rest day for all. The
//!    five revolutionary holidays, 22 January, 1–2 May and 7–8 November,
//!    "are not counted in the working weeks": they stand outside the
//!    five-day cycle, so a common year is 72 weeks and five holidays.
//! 2. **The interrupted six-day week** (*shestidnevka*), under the decree of
//!    21 November 1931, from 1 December 1931: fixed common rest days on
//!    the 6th, 12th, 18th, 24th and 30th of every month, 1 March in place
//!    of February's, so that the six-day week is read off the day of the
//!    month and the 31st belongs to none.
//! 3. **The seven-day week** again, under the decree of the Presidium of
//!    the Supreme Soviet of 26 June 1940, from 27 June, Sunday the day of
//!    rest; that is the plain Gregorian calendar, and outside this one's
//!    range.
//!
//! The periods are [`PERIODS`], one row per decree. `soviet-week` covers
//! 1 October 1929 to 26 June 1940 and refuses the days outside it.
//!
//! # What is carried, and what is not
//!
//! The date is the Gregorian one. `to_fields` adds the week of the day's
//! period: `week-length`, 5 or 6; `soviet-week-day`, the day of that week,
//! 1 to 5 (the groups' days, I to V) or 1 to 6, and 0 on a day outside
//! the week — a revolutionary holiday of the five-day week, the 31st of a
//! month in the six-day one; and `rest-day`, 1 on the six-day week's common
//! rest days. The two weeks are declared as cycles of their own,
//! `five-day-week` named I to V as the 1931 pocket calendar numbers its
//! rows and `six-day-week` numbered, beside the seven-day `weekday`, which
//! the rest of life, and every newspaper masthead, went on using.
//!
//! The decrees set dates for enterprises and institutions that converted
//! in their own time — by 1930 some fifty lengths of continuous week were
//! in use, and a quarter of industry was still continuous in 1935 — so
//! this is the week of the decrees, not of every workplace. The five-day
//! week's first day is 1 October 1929 counted as day I, which the 1930 and
//! 1931 calendars bear out; which colour named which day varies between
//! sources and is not carried. The system document is
//! `docs/systems/soviet-week.md`.
//!
//! # Sources
//!
//! * Sovnarkom SSSR, "О переходе на непрерывное производство в
//!   предприятиях и учреждениях Союза ССР", 26 August 1929; "О рабочем
//!   времени и времени отдыха в предприятиях и учреждениях, переходящих на
//!   непрерывную производственную неделю", 24 September 1929 (СЗ СССР
//!   1929, № 63, ст. 586), arts. 1, 2 and 6 with its note; "О непрерывной
//!   производственной неделе в учреждениях", 21 November 1931, arts. 4 and
//!   8 — all read in the Stolypin Museum's collection, museumreforms.ru
//!   `node/13988`, retrieved 2026-09-26.
//! * Presidium of the Supreme Soviet of the USSR, "О переходе на
//!   восьмичасовой рабочий день, на семидневную рабочую неделю и о
//!   запрещении самовольного ухода рабочих и служащих с предприятий и
//!   учреждений", 26 June 1940 (Ведомости Верховного Совета СССР, 1940,
//!   № 20), art. 2, read at 1000dokumente.de, retrieved 2026-09-26.
//! * Wikipedia, "Soviet calendar", retrieved 2026-09-26, for the 1930 and
//!   1931 calendars (after Foss, *History Today* 54/9, 2004, and Malyavin),
//!   the unchanged Gregorian months, and the six-day week's 1 March; and
//!   Russian Wikipedia, "Шестидневка", for a 1935 calendar's "22 октября —
//!   четвёртый день шестидневки" (Russian State Library).
//!
//! # Exactness
//!
//! Exact within the decrees; bounded to them.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::gregorian;

/// The calendar identifier.
pub const ID: &str = "soviet-week";

/// The kind of week a period kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Week {
    /// Four days of work and one of rest, staggered across five groups;
    /// the revolutionary holidays outside the cycle.
    ContinuousFive,
    /// Common rest days on the 6th, 12th, 18th, 24th and 30th and on
    /// 1 March; the 31st outside the week.
    InterruptedSix,
}

impl Week {
    /// The days of the week.
    #[must_use]
    pub const fn length(self) -> u8 {
        match self {
            Self::ContinuousFive => 5,
            Self::InterruptedSix => 6,
        }
    }
}

/// One decree's period.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Period {
    /// The first day, as Gregorian year, month and day.
    pub from: (i64, u8, u8),
    /// The week it kept.
    pub week: Week,
    /// The decree.
    pub decree: &'static str,
}

/// The periods in order; each lasts until the next begins, and the last
/// until [`LAST_DAY`].
pub const PERIODS: [Period; 2] = [
    Period {
        from: (1929, 10, 1),
        week: Week::ContinuousFive,
        decree: "Sovnarkom SSSR, 26 August 1929, on continuous production, from the economic year \
            1929–1930; the five-day week by the decree of 24 September 1929",
    },
    Period {
        from: (1931, 12, 1),
        week: Week::InterruptedSix,
        decree: "Sovnarkom SSSR, 21 November 1931, on the continuous production week in \
            institutions, the six-day interrupted week from 1 December 1931",
    },
];

/// The last day of the six-day week: the seven-day week returned on
/// 27 June 1940.
pub const LAST_DAY: (i64, u8, u8) = (1940, 6, 26);

/// The revolutionary holidays of the decree of 24 September 1929, art. 6,
/// as month and day.
pub const HOLIDAYS: [(u8, u8); 5] = [(1, 22), (5, 1), (5, 2), (11, 7), (11, 8)];

/// The names of the five days of the continuous week, as the 1931 pocket
/// calendar numbers them.
pub const FIVE_DAYS: [&str; 5] = ["I", "II", "III", "IV", "V"];

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Sovnarkom SSSR, 26 August 1929 and 24 September 1929 (the five-day week, \
    from the economic year beginning 1 October 1929) and 21 November 1931 (the six-day week from \
    1 December 1931); Presidium of the Supreme Soviet, 26 June 1940 (the seven-day week from \
    27 June 1940)";

const fn fixed(date: (i64, u8, u8)) -> Rd {
    match gregorian::to_fixed(date.0, date.1, date.2) {
        Ok(rd) => rd,
        Err(_) => Rd(0),
    }
}

/// The earliest fixed day this calendar converts: 1 October 1929.
pub const EARLIEST: Rd = fixed(PERIODS[0].from);

/// The latest fixed day this calendar converts: 26 June 1940.
pub const LATEST: Rd = fixed(LAST_DAY);

/// The period `rd` falls in, if any.
#[must_use]
pub fn period(rd: Rd) -> Option<&'static Period> {
    if rd < EARLIEST || rd > LATEST {
        return None;
    }
    PERIODS.iter().rev().find(|period| fixed(period.from) <= rd)
}

/// Whether a Gregorian month and day is a revolutionary holiday.
#[must_use]
pub fn is_holiday(month: u8, day: u8) -> bool {
    HOLIDAYS.contains(&(month, day))
}

/// Where a day stands in its week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeekDay {
    /// The week of the day's period.
    pub week: Week,
    /// The day of that week from 1, or `None` for a day outside it.
    pub day: Option<u8>,
    /// Whether it is a common rest day of the six-day week.
    pub rest_day: bool,
}

/// The day of the Soviet week of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub fn week_day(rd: Rd) -> CalendarResult<WeekDay> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    let Some(period) = period(rd) else {
        return Err(CalendarError::AfterSupportedRange);
    };
    let (year, month, day) = gregorian::from_fixed(rd)?;
    match period.week {
        Week::ContinuousFive => {
            if is_holiday(month, day) {
                return Ok(WeekDay {
                    week: period.week,
                    day: None,
                    rest_day: false,
                });
            }
            // The holidays are not counted in the working weeks, so the
            // cycle advances only over the other days.
            let start = fixed(period.from);
            let mut counted = rd.0 - start.0;
            for holiday_year in period.from.0..=year {
                for (m, d) in HOLIDAYS {
                    let holiday = gregorian::to_fixed(holiday_year, m, d)?;
                    if start <= holiday && holiday < rd {
                        counted -= 1;
                    }
                }
            }
            Ok(WeekDay {
                week: period.week,
                day: Some((counted.rem_euclid(5) + 1) as u8),
                rest_day: false,
            })
        }
        Week::InterruptedSix => {
            let position = if day == 31 {
                None
            } else {
                Some((day - 1) % 6 + 1)
            };
            Ok(WeekDay {
                week: period.week,
                day: position,
                rest_day: position == Some(6) || (month, day) == (3, 1),
            })
        }
    }
}

/// A date of the Soviet weeks: a Gregorian date inside their period.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SovietWeekDate {
    /// The Gregorian year.
    pub year: i64,
    /// The Gregorian month.
    pub month: u8,
    /// The Gregorian day.
    pub day: u8,
}

/// The Soviet revolutionary weeks.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SovietWeekCalendar;

/// The Gregorian months, the two Soviet weeks, and the seven-day week that
/// went on beside them.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::MONTH, 12),
    hc_calendar::shape::CycleShape::named("five-day-week", &FIVE_DAYS),
    hc_calendar::shape::CycleShape::fixed("six-day-week", 6),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for SovietWeekCalendar {
    type Date = SovietWeekDate;

    /// The decrees' period, 1 October 1929 to 26 June 1940.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(EARLIEST, LATEST, USAGE_SOURCE)
    }

    /// The Gregorian months, the five- and six-day weeks and the seven-day
    /// week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// The Gregorian leap day, for the years the calendar covers.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(PERIODS[0].from.0..=LAST_DAY.0).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(gregorian::is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Soviet revolutionary weeks (1929–1940)",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["ru"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        let rd = gregorian::to_fixed(date.year, date.month, date.day)?;
        self.meta().check_range(rd)?;
        Ok(rd)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.meta().check_range(rd)?;
        let (year, month, day) = gregorian::from_fixed(rd)?;
        Ok(SovietWeekDate { year, month, day })
    }

    /// The Gregorian date with the week of its period.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let at = week_day(self.to_fixed(date)?)?;
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("week-length", i64::from(at.week.length()))?
            .with_extra("soviet-week-day", i64::from(at.day.unwrap_or(0)))?
            .with_extra("rest-day", i64::from(at.rest_day))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = SovietWeekDate {
            year: fields.year,
            month: month.ordinal,
            day: fields.require_day()?,
        };
        self.to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(year: i64, month: u8, day: u8) -> WeekDay {
        week_day(gregorian::to_fixed(year, month, day).unwrap()).unwrap()
    }

    #[test]
    fn the_five_day_week_runs_from_1_october_1929_and_skips_the_holidays() {
        assert_eq!(at(1929, 10, 1).day, Some(1));
        assert_eq!(at(1929, 10, 6).day, Some(1));
        // 7 and 8 November are outside the week, so the 9th follows the 6th.
        assert_eq!(at(1929, 11, 6).day, Some(2));
        assert_eq!(at(1929, 11, 7).day, None);
        assert_eq!(at(1929, 11, 8).day, None);
        assert_eq!(at(1929, 11, 9).day, Some(3));
        // The 1930 calendar begins its order on 1 January, and the 1931
        // pocket calendar's grid of 360 days opens with day I: 92 days of
        // 1929 less two holidays, and 365 of 1930 less five, are whole
        // five-day weeks.
        assert_eq!(at(1930, 1, 1).day, Some(1));
        assert_eq!(at(1931, 1, 1).day, Some(1));
        assert_eq!(at(1930, 1, 22).day, None);
        assert_eq!(at(1930, 5, 1).day, None);
        assert!(!at(1930, 1, 6).rest_day);
    }

    /// Art. 2 of the decree of 24 September 1929: "not less than 72" rest
    /// days a year. Each group rests on exactly 72 days of 1930.
    #[test]
    fn each_group_rests_72_days_in_1930() {
        let mut days = [0u32; 5];
        for day in 0..365 {
            let rd = Rd(gregorian::to_fixed(1930, 1, 1).unwrap().0 + day);
            if let Some(position) = week_day(rd).unwrap().day {
                days[position as usize - 1] += 1;
            }
        }
        assert_eq!(days, [72; 5]);
    }

    #[test]
    fn the_six_day_week_is_read_off_the_day_of_the_month() {
        assert_eq!(at(1931, 11, 30).week, Week::ContinuousFive);
        assert_eq!(at(1931, 12, 1).week, Week::InterruptedSix);
        // A 1935 calendar: "22 октября — четвёртый день шестидневки".
        assert_eq!(at(1935, 10, 22).day, Some(4));
        // Art. 4 of the decree of 21 November 1931.
        for day in [6u8, 12, 18, 24, 30] {
            let here = at(1935, 10, day);
            assert_eq!(here.day, Some(6));
            assert!(here.rest_day);
        }
        // The 31st is in no week and is no rest day.
        let last = at(1935, 10, 31);
        assert_eq!((last.day, last.rest_day), (None, false));
        // "Вместо выходного дня в конце февраля предоставить выходной день
        // 1 марта."
        assert!(at(1936, 3, 1).rest_day);
        assert!(!at(1936, 2, 29).rest_day);
        assert!(!at(1936, 3, 2).rest_day);
        // Aviation Day, 18 August from 1933, was set as the third rest day
        // of August under the six-day week.
        let august: Vec<u8> = (1..=31).filter(|day| at(1933, 8, *day).rest_day).collect();
        assert_eq!(august[2], 18);
    }

    #[test]
    fn the_calendar_is_the_gregorian_one_within_the_decrees() {
        let calendar = SovietWeekCalendar;
        assert_eq!(EARLIEST, gregorian::to_fixed(1929, 10, 1).unwrap());
        assert_eq!(LATEST, gregorian::to_fixed(1940, 6, 26).unwrap());
        for rd in EARLIEST.0..=LATEST.0 {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
            assert_eq!(
                gregorian::from_fixed(Rd(rd)),
                Ok((date.year, date.month, date.day))
            );
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
        }
        assert_eq!(
            calendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            calendar.from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            week_day(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            calendar.to_fixed(SovietWeekDate {
                year: 1940,
                month: 6,
                day: 27
            }),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(calendar.is_leap_year(1932), Ok(true));
        assert_eq!(
            calendar.is_leap_year(1941),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_period_of_use_is_the_decrees() {
        use hc_calendar::Standing;
        let calendar = SovietWeekCalendar;
        assert_eq!(calendar.standing(EARLIEST), Standing::InUse);
        assert_eq!(calendar.standing(Rd(LATEST.0 + 1)), Standing::Extended);
        assert_eq!(period(EARLIEST).map(|p| p.week), Some(Week::ContinuousFive));
        assert_eq!(period(Rd(LATEST.0 + 1)), None);
    }
}
