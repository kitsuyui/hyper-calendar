//! The Taiping Heavenly Calendar, 天曆 — `taiping-tianli`.
//!
//! The solar calendar of the Taiping Heavenly Kingdom: a year of 366 days,
//! the odd months of 31 days and the even of 30, with no leap day and no
//! leap month (Wikipedia (zh), 「太平天曆」, `wikipedia-zh-taiping-tianli`,
//! citing 張德堅's 《賊情彙纂》, not read). It was promulgated for the second
//! year of the kingdom, 壬子, and its first day, 壬子二年正月初一, was
//! 3 February 1852; the first year, 辛開, still used the Qing calendar.
//! The years are counted from the kingdom's first year, 1851, and named by
//! their stem and branch, with 好, 榮 and 開 written for 丑, 卯 and 亥
//! (`wikipedia-zh-taiping-tianli`, after 吳善中).
//!
//! The rule and the dates are from 羅爾綱, 《太平天國史》, 卷三十二
//! 志第十一 天曆, section 四 「天曆與夏曆陽曆對照及簡表」
//! (`luo-ergang-taiping-tianli`), read in the Internet Archive's copy of
//! 22 July 2015: the first day, 壬子二年正月初一 丙申 星期三, set against
//! 咸豐元年十二月十四日 乙未 and 3 February 1852, a Tuesday; the last day of
//! the same year, 十二月三十日, which the calendar called 辛好 and a
//! Thursday, on the day the Qing calendar called 庚子 and the Western
//! calendar a Wednesday, 2 February 1853; and the last dated use, 己巳十九年
//! 四月十一日, 28 May 1869, at 老岩窑 in Shaanxi. Eighteen years of 366
//! days reach that day exactly, so [`MAX_YEAR`] is the nineteenth year.
//!
//! # The calendar's own day names
//!
//! The calendar named each day by its stem and branch and marked its
//! Sabbath, taking both from the Qing almanac, yet from its first day its
//! names ran one day ahead of the Qing almanac's and the Western week
//! (`luo-ergang-taiping-tianli`). The conversion here is of days, which
//! Luo's pairs fix; [`labelled_sexagenary_day`] and [`labelled_weekday`]
//! give the names the calendar itself printed. Luo records a second view,
//! that the slip began on 癸好三年二月十三日, and says the evidence refutes
//! it; it is not carried. Why the slip happened is disputed, and nothing
//! here depends on it.
//!
//! # Not carried
//!
//! Hong Rengan's reform of the ninth year, a 斡年 of 28-day months every
//! forty years, and the earlier plan of a year of 33-day months every
//! forty: neither year fell within the calendar's use, and the sources do
//! not say which year of the forty would have been the first. The twelve
//! 節 on the first of each month and the twelve 氣 in the middle, which
//! the Wikipedia article gives without a source.
//!
//! `docs/systems/taiping-tianli.md` in the repository works the first and
//! last days through.

use hc_calendar::cycle::{Sexagenary, readings, sexagenary_day};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    Weekday, YearKind,
};

use crate::{common, gregorian};

/// The machine identifier of this calendar.
pub const ID: CalendarId = CalendarId("taiping-tianli");

/// The era code of the Taiping calendar.
pub const ERA: &str = "taiping";

/// The first year the calendar was kept, 壬子二年.
pub const MIN_YEAR: i64 = 2;

/// The last year it was kept, 己巳十九年.
pub const MAX_YEAR: i64 = 19;

/// The number of days in every year.
pub const DAYS_IN_YEAR: i64 = 366;

/// 壬子二年正月初一: 3 February 1852.
pub const EPOCH: Rd = match gregorian::to_fixed(1852, 2, 3) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The last day converted, 十二月三十日 of the nineteenth year.
pub const LATEST: Rd = Rd(EPOCH.0 + (MAX_YEAR - MIN_YEAR + 1) * DAYS_IN_YEAR - 1);

/// The last dated use, 己巳十九年四月十一日: 28 May 1869.
pub const LAST_USE: Rd = match gregorian::to_fixed(1869, 5, 28) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "羅爾綱, 《太平天國史》, 卷三十二 天曆, 四 「天曆與夏曆陽曆對照及簡表」 \
    [luo-ergang-taiping-tianli]: in force from 壬子二年正月初一, 3 February 1852, and kept by \
    the kingdom's armies after the fall of 天京 to 己巳十九年四月十一日, 28 May 1869";

/// The twelve months as the calendar numbers them.
pub const MONTHS: [&str; 12] = [
    "正月",
    "二月",
    "三月",
    "四月",
    "五月",
    "六月",
    "七月",
    "八月",
    "九月",
    "十月",
    "十一月",
    "十二月",
];

/// The twelve branches as the calendar writes them: 好 for 丑, 榮 for 卯
/// and 開 for 亥.
pub const BRANCHES: [&str; 12] = [
    "子", "好", "寅", "榮", "辰", "巳", "午", "未", "申", "酉", "戌", "開",
];

/// The number of days in `month`, or `None` when `month` is not in
/// `1..=12`: 31 for an odd month, 30 for an even one.
#[must_use]
pub const fn days_in_month(month: u8) -> Option<u8> {
    match month {
        1..=12 if month % 2 == 1 => Some(31),
        1..=12 => Some(30),
        _ => None,
    }
}

/// The days of the year before `month`.
const fn days_before_month(month: u8) -> i64 {
    30 * (month as i64 - 1) + (month as i64) / 2
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
    match common::check_day(day, days_in_month(month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(EPOCH.0
            + DAYS_IN_YEAR * (year - MIN_YEAR)
            + days_before_month(month)
            + day as i64
            - 1)),
    }
}

/// The year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EPOCH`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EPOCH.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    let elapsed = rd.0 - EPOCH.0;
    let year = MIN_YEAR + elapsed / DAYS_IN_YEAR;
    let day_of_year = elapsed % DAYS_IN_YEAR;
    let mut month = 12;
    while days_before_month(month) > day_of_year {
        month -= 1;
    }
    Ok((
        year,
        month,
        (day_of_year - days_before_month(month) + 1) as u8,
    ))
}

/// The stem and branch of a year of the kingdom: 壬子 for the second.
#[must_use]
pub const fn year_sexagenary(year: i64) -> Sexagenary {
    // 1851, the first year, was 辛亥, index 47.
    Sexagenary::from_index(46 + year)
}

/// The name of a year as the kingdom wrote it, stem and branch with 好, 榮
/// and 開: `("癸", "好")` for the third.
#[must_use]
pub const fn year_name(year: i64) -> (&'static str, &'static str) {
    let pillar = year_sexagenary(year);
    (
        readings::HAN.stem(pillar),
        BRANCHES[pillar.branch_index() as usize],
    )
}

/// The stem and branch the calendar printed for a day: one ahead of the
/// Qing almanac's (`luo-ergang-taiping-tianli`). `None` outside the
/// calendar's range.
#[must_use]
pub const fn labelled_sexagenary_day(rd: Rd) -> Option<Sexagenary> {
    if rd.0 < EPOCH.0 || rd.0 > LATEST.0 {
        return None;
    }
    Some(sexagenary_day(rd).next())
}

/// The weekday the calendar printed for a day: one ahead of the Western
/// week, so that its Sabbath fell on a Saturday. `None` outside the
/// calendar's range.
#[must_use]
pub const fn labelled_weekday(rd: Rd) -> Option<Weekday> {
    if rd.0 < EPOCH.0 || rd.0 > LATEST.0 {
        return None;
    }
    Some(Weekday::from_rd(Rd(rd.0 + 1)))
}

/// A date of the Taiping calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaipingDate {
    /// The year of the kingdom, 2 to 19.
    pub year: i64,
    /// The month, 1 to 12.
    pub month: u8,
    /// The day of the month, from 1.
    pub day: u8,
}

/// The Taiping Heavenly Calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TaipingCalendar;

/// Twelve numbered months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for TaipingCalendar {
    type Date = TaipingDate;

    /// From its first day to the last dated use Luo gives.
    fn usage(&self) -> Usage {
        Usage::between(EPOCH, LAST_USE, USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// No year is leap: every year has 366 days.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(false)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Taiping Heavenly Calendar",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EPOCH),
            latest: Some(LATEST),
            native_locales: &["zh-Hant"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(TaipingDate { year, month, day })
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
        let day = fields.require_day()?;
        to_fixed(fields.year, month.ordinal, day)?;
        Ok(TaipingDate {
            year: fields.year,
            month: month.ordinal,
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn luo_sets_the_first_day_against_the_third_of_february_1852() {
        // 壬子二年正月初一 丙申 星期三 = 咸豐元年十二月十四日 乙未 = 3 February
        // 1852, a Tuesday.
        let first = day(1852, 2, 3);
        assert_eq!(to_fixed(2, 1, 1), Ok(first));
        assert_eq!(Weekday::from_rd(first), Weekday::Tuesday);
        let han = |day: Sexagenary| (readings::HAN.stem(day), readings::HAN.branch(day));
        assert_eq!(han(sexagenary_day(first)), ("乙", "未"));
        assert_eq!(han(labelled_sexagenary_day(first).unwrap()), ("丙", "申"));
        assert_eq!(labelled_weekday(first), Some(Weekday::Wednesday));
        assert_eq!(year_name(2), ("壬", "子"));
    }

    #[test]
    fn luo_sets_the_last_day_of_the_second_year_a_day_ahead() {
        // 壬子二年十二月三十日, which the calendar called 辛好 and a
        // Thursday, fell on the Qing calendar's 庚子, a Wednesday: 2 February
        // 1853, the day before 3 February.
        let last = to_fixed(2, 12, 30).unwrap();
        assert_eq!(last, day(1853, 2, 2));
        assert_eq!(Weekday::from_rd(last), Weekday::Wednesday);
        let today = sexagenary_day(last);
        assert_eq!(
            (readings::HAN.stem(today), readings::HAN.branch(today)),
            ("庚", "子")
        );
        let labelled = labelled_sexagenary_day(last).unwrap();
        assert_eq!(
            (
                readings::HAN.stem(labelled),
                BRANCHES[labelled.branch_index() as usize]
            ),
            ("辛", "好")
        );
        assert_eq!(labelled_weekday(last), Some(Weekday::Thursday));
        assert_eq!(to_fixed(3, 1, 1), Ok(day(1853, 2, 3)));
        assert_eq!(year_name(3), ("癸", "好"));
    }

    #[test]
    fn luo_dates_the_last_use_to_the_twenty_eighth_of_may_1869() {
        // 己巳十九年四月十一日 = 清同治八年四月十七日 = 28 May 1869.
        assert_eq!(to_fixed(19, 4, 11), Ok(LAST_USE));
        assert_eq!(LAST_USE, day(1869, 5, 28));
        assert_eq!(year_name(19), ("己", "巳"));
        // The other years Luo names: 癸開十三年, 甲子十四年, 乙好十五年,
        // 丙寅十六年, 戊辰十八年.
        assert_eq!(year_name(13), ("癸", "開"));
        assert_eq!(year_name(14), ("甲", "子"));
        assert_eq!(year_name(15), ("乙", "好"));
        assert_eq!(year_name(16), ("丙", "寅"));
        assert_eq!(year_name(18), ("戊", "辰"));
    }

    #[test]
    fn the_months_alternate_thirty_one_and_thirty_days() {
        let total: u32 = (1..=12)
            .map(|month| u32::from(days_in_month(month).unwrap()))
            .sum();
        assert_eq!(i64::from(total), DAYS_IN_YEAR);
        assert_eq!(days_in_month(1), Some(31));
        assert_eq!(days_in_month(12), Some(30));
        assert_eq!(days_in_month(13), None);
        assert_eq!(to_fixed(2, 2, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2, 13, 1), Err(CalendarError::MonthOutOfRange));
    }

    #[test]
    fn every_day_round_trips_and_the_range_is_refused_outside() {
        let calendar = TaipingCalendar;
        for rd in EPOCH.0..=LATEST.0 {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date));
            assert_eq!(labelled_weekday(Rd(rd)), Some(Weekday::from_rd(Rd(rd + 1))));
        }
        assert_eq!(from_fixed(Rd(EPOCH.0 - 1)), Err(CalendarError::BeforeEpoch));
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(from_fixed(LATEST), Ok((19, 12, 30)));
        assert_eq!(to_fixed(1, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(20, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(labelled_weekday(Rd(EPOCH.0 - 1)), None);
        assert_eq!(labelled_sexagenary_day(Rd(LATEST.0 + 1)), None);
        assert_eq!(calendar.is_leap_year(5), Ok(false));
        assert_eq!(calendar.is_leap_year(1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(2, 1, 1).with_era("ad")),
            Err(CalendarError::UnknownEra)
        );
    }
}
