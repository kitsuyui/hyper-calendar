//! The Jalālī (Malekī) calendar in the arithmetic form of Ṭūsī's table.
//!
//! The Seljuk reform dated from Friday 9 Ramaḍān 471, 15 March 1079 in the
//! Julian calendar, fixed the new year at the vernal equinox; its twelve
//! months under the Zoroastrian names were "not true solar months but
//! consisted of thirty days each", and a year had five extra days or six.
//! Naṣīr-al-Dīn Ṭūsī's *Zīj-e īl-ḵānī* tabulates "the quadrennia and
//! quinquennia of the first 295 Jalālī years": a sixth day every fourth
//! year, and after several quadrennia a period of five, whose leap years are
//! 31, 64, 97, 130, 163, 192, 225, 258 and 291. Abdollahy's rule, that a
//! year is long when (*y* + 3)·39 mod 161 < 39, reproduces the table for
//! all 295 years, which [`is_leap_year`] computes and a test checks against
//! the quinquennia. Everything so far is Abdollahy, "Calendars ii. In the
//! Islamic period", *Encyclopaedia Iranica* IV (1990) (`abdollahy1990`),
//! read in the Wayback Machine's copy on 2026-09-26; the *Zīj* itself and
//! Iranica's Tables 35 and 36, which are images, were not read.
//!
//! Where the extra days stand is from the same article's fourth part,
//! Panaino, "Calendars iv. Other modern calendars" (`panaino1990iv`):
//! among the Zoroastrian communities of Iran that adopted the Jalālī
//! calendar, "the 5 or 6 epagomenal days follow the month of Esfandārmoḏ",
//! the twelfth, "or, in some villages in the district of Naṭanz, the month
//! of Bahman". This module places them after the twelfth month, as a
//! thirteenth; the Naṭanz placement would be a calendar of its own name
//! under policy §5 and is not carried. The month names are
//! [`crate::zoroastrian::MONTHS`], the same names in that module's forms,
//! since the Jalālī months were the Zoroastrian ones qualified as *jalālī*.
//!
//! **Only the table's years.** Ṭūsī's table ends at year 295, AD 1373/74,
//! and this calendar refuses the years after it rather than extend the
//! rule: the rule is Abdollahy's fit to the table, not a rule anyone kept.
//! The calendar as the astronomers defined it, with the new year on the
//! day the Sun enters Aries before noon, is `jalali`, which is not carried;
//! the days here are the arithmetic scheme's, and a year whose equinox
//! fell near noon may begin a day apart from it. The system document is
//! `docs/systems/jalali.md` in the repository.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, julian, zoroastrian};

/// The fixed day of 1 Farvardīn 1, Friday 15 March 1079 in the proleptic
/// Julian calendar.
pub const EPOCH: Rd = match julian::to_fixed(1_079, 3, 15) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The era code of the Jalālī era, *tārīḵ-e jalālī*.
pub const ERA: &str = "jalali";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The last year of Ṭūsī's table, and of this implementation.
pub const MAX_YEAR: i64 = 295;

/// The leap years that close a period of five years rather than four, as
/// Iranica lists them from Ṭūsī's table.
pub const QUINQUENNIAL_LEAP_YEARS: [i64; 9] = [31, 64, 97, 130, 163, 192, 225, 258, 291];

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Abdollahy, \"Calendars ii. In the Islamic period\", \
    Encyclopaedia Iranica IV (1990), read in the Wayback Machine's copy on 2026-09-26 \
    [abdollahy1990]: the Hejrī date of the calendar's adoption, Friday 9 Ramaḍān 471, \
    15 March 1079. It names no end; the Zoroastrian communities of Iran kept it after 1925 \
    (Panaino, \"Calendars iv\" [panaino1990iv])";

/// Whether `year` has six extra days: (`year` + 3)·39 mod 161 < 39, which
/// is Ṭūsī's table for every year it covers.
///
/// Outside [`MIN_YEAR`]..=[`MAX_YEAR`] the answer is the rule's and not the
/// table's; [`JalaliTusiCalendar::is_leap_year`] refuses those years.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    ((year + 3) * 39).rem_euclid(161) < 39
}

/// The number of days in `month` of `year`, or `None` when `month` is not
/// in `1..=13`. Month 13 is the extra days.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1..=12 => Some(30),
        13 => Some(if is_leap_year(year) { 6 } else { 5 }),
        _ => None,
    }
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The first fixed day of each year from 1 to 296, the day after the
/// table's last.
const NEW_YEARS: [i64; (MAX_YEAR + 1) as usize] = {
    let mut table = [0i64; (MAX_YEAR + 1) as usize];
    table[0] = EPOCH.0;
    let mut index = 1;
    while index < table.len() {
        table[index] = table[index - 1] + days_in_year(index as i64) as i64;
        index += 1;
    }
    table
};

/// The latest fixed day this implementation converts, the last extra day
/// of year 295.
pub const LATEST: Rd = Rd(NEW_YEARS[MAX_YEAR as usize] - 1);

/// The fixed day of a Jalālī date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside the table's years,
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    match common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(NEW_YEARS[(year - 1) as usize]
            + 30 * (month as i64 - 1)
            + day as i64
            - 1)),
    }
}

/// The Jalālī year, month and day of a fixed day.
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
    // The table's mean year is within a day of 365¼ over its 295 years, so
    // the Julian estimate is at most one year out either way.
    let mut year = (4 * (rd.0 - EPOCH.0)).div_euclid(1_461) + 1;
    if year > MAX_YEAR {
        year = MAX_YEAR;
    }
    while year > MIN_YEAR && rd.0 < NEW_YEARS[(year - 1) as usize] {
        year -= 1;
    }
    while year < MAX_YEAR && rd.0 >= NEW_YEARS[year as usize] {
        year += 1;
    }
    let day_of_year = rd.0 - NEW_YEARS[(year - 1) as usize];
    let month = (day_of_year.div_euclid(30) + 1) as u8;
    let day = (day_of_year.rem_euclid(30) + 1) as u8;
    Ok((year, month, day))
}

/// A Jalālī date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JalaliDate {
    /// The year of the Jalālī era, 1 to 295.
    pub year: i64,
    /// The month, 1 (Farvardīn) through 12 (Esfandārmoḏ); 13 holds the
    /// extra days.
    pub month: u8,
    /// The day of the month, from 1.
    pub day: u8,
}

impl JalaliDate {
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
}

/// The Jalālī calendar by Ṭūsī's table.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JalaliTusiCalendar;

/// The Zoroastrian months, the extra days as a thirteenth, and the
/// seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &zoroastrian::MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for JalaliTusiCalendar {
    type Date = JalaliDate;

    /// From the adoption of 15 March 1079, with no end the source names.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(EPOCH, USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// Ṭūsī's table, for its 295 years only.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("jalali-tusi"),
            english_name: "Jalali (Tusi's arithmetic)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EPOCH),
            latest: Some(LATEST),
            native_locales: &["fa"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(JalaliDate { year, month, day })
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
        JalaliDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::Weekday;

    #[test]
    fn the_epoch_is_friday_the_fifteenth_of_march_1079() {
        assert_eq!(julian::from_fixed(EPOCH), Ok((1_079, 3, 15)));
        assert_eq!(Weekday::from_rd(EPOCH), Weekday::Friday);
        assert_eq!(from_fixed(EPOCH), Ok((1, 1, 1)));
        // "= 19 Farvardīn 448 Yazdegerdī": the Qadimi reckoning, which did
        // not intercalate before the Parsi intercalation of the 1120s.
        assert_eq!(
            zoroastrian::Reckoning::Qadimi.to_fixed(448, 1, 19),
            Ok(EPOCH)
        );
    }

    #[test]
    fn the_rule_is_tusis_table_for_all_295_years() {
        // The table as Iranica describes it: a sixth day every fourth year,
        // five years before each quinquennial leap year. Walking back by
        // fours from the first quinquennium, 31, reaches year 2.
        let mut expected = [false; (MAX_YEAR + 1) as usize];
        let mut leap = 2;
        while leap <= MAX_YEAR {
            expected[leap as usize] = true;
            leap += if QUINQUENNIAL_LEAP_YEARS.contains(&(leap + 5)) {
                5
            } else {
                4
            };
        }
        let mut count = 0;
        for year in MIN_YEAR..=MAX_YEAR {
            assert_eq!(is_leap_year(year), expected[year as usize], "year {year}");
            count += usize::from(is_leap_year(year));
        }
        // 295 years with 286 quarter-days intercalated, less a half.
        assert_eq!(count, 72);
        for year in QUINQUENNIAL_LEAP_YEARS {
            assert!(is_leap_year(year) && is_leap_year(year - 5) && !is_leap_year(year - 4));
        }
    }

    #[test]
    fn the_extra_days_follow_the_twelfth_month() {
        assert_eq!(days_in_month(2, 13), Some(6));
        assert_eq!(days_in_month(1, 13), Some(5));
        assert_eq!(days_in_month(1, 14), None);
        let last = to_fixed(1, 13, 5).unwrap();
        assert_eq!(last.0 + 1, to_fixed(2, 1, 1).unwrap().0);
        assert_eq!(to_fixed(1, 13, 6), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            to_fixed(1, 12, 30).unwrap().0 + 1,
            to_fixed(1, 13, 1).unwrap().0
        );
    }

    #[test]
    fn every_day_of_the_table_round_trips() {
        for rd in EPOCH.0..=LATEST.0 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
        assert_eq!(from_fixed(LATEST).map(|(year, _, _)| year), Ok(MAX_YEAR));
    }

    #[test]
    fn the_years_after_the_table_are_refused() {
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(from_fixed(Rd(EPOCH.0 - 1)), Err(CalendarError::BeforeEpoch));
        assert_eq!(to_fixed(296, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            JalaliTusiCalendar.is_leap_year(296),
            Err(CalendarError::YearOutOfRange)
        );
        // The last day is in AD 1374: year 295 began in March 1373.
        assert_eq!(
            julian::from_fixed(to_fixed(295, 1, 1).unwrap()).map(|(year, _, _)| year),
            Ok(1_373)
        );
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = JalaliTusiCalendar;
        for rd in (EPOCH.0..=LATEST.0).step_by(97) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date));
        }
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(1, 1, 1).with_era("yz")),
            Err(CalendarError::UnknownEra)
        );
    }
}
