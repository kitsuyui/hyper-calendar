//! The fixed Armenian calendar of Yovhannēs Sarkawag, 1084.
//!
//! [`crate::armenian`] implements the original 365-day wandering year, which
//! drifts a day against the seasons every four years and had drifted by
//! more than four months by the eleventh century. Yovhannēs Sarkawag — also
//! called Imastaser, "the philosopher" — fixed it in 1084 by adding a sixth
//! epagomenal day every fourth year, so the mean year became 365.25 days
//! and the calendar stopped moving against the Julian one.
//!
//! He anchored it by working backwards: 1 Nawasardi had fallen on 11 August
//! in the time of Maštocʿ, in 428, so 11 August is where he put it. The
//! reform takes effect from Armenian year 533, which begins on 11 August
//! 1084 in the Julian calendar.
//!
//! # How the year is defined here
//!
//! Directly: year *Y* begins on 11 August of Julian year *Y* + 551, and the
//! length of a year is the distance to the next one. Nothing computes a
//! leap rule, so nothing can get one wrong — a long year is simply a year
//! whose 11 August is 366 days after the last, which happens exactly when
//! the Julian 29 February falls inside it.
//!
//! # The reform re-anchored the year; it did not merely stop it
//!
//! This is the same era as [`crate::armenian`], so Armenian year 533 is 533
//! in both — but they do not meet. By 1084 the wandering 1 Nawasardi had
//! drifted back to 29 February, and Sarkawag moved it forward 164 days to
//! 11 August rather than fixing it where it lay.
//!
//! The arithmetic recovers his reason exactly. 11 August is 31 days later
//! in the Julian year than the era's own epoch position of 11 July, and 31
//! days is 124 years of drift at a day per four years — which is 552 − 428,
//! the interval back to Maštocʿ. A test asserts all of it.
//!
//! **Sources:** the reform is treated in the standard Armenian
//! chronological literature; the anchor is Sarkawag's own retrojection of
//! 1 Nawasardi to 11 August.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};

use crate::{common, julian};

/// The Armenian year the reform takes effect in.
pub const REFORM_YEAR: i64 = 533;

/// How far the Julian year runs ahead of the Armenian one at the new year.
///
/// Armenian 533 begins in Julian 1084, and 1084 − 533 is 551.
pub const JULIAN_OFFSET: i64 = 551;

/// The Julian month the year begins in.
pub const NEW_YEAR_MONTH: u8 = 8;

/// The Julian day of the month the year begins on.
pub const NEW_YEAR_DAY: u8 = 11;

/// The era code, the same as the wandering calendar's.
pub const ERA: &str = crate::armenian::ERA;

/// The earliest year this implementation converts.
///
/// The calendar is defined proleptically before the reform — the arithmetic
/// works for any year — but nobody used it there, which is what
/// [`ArmenianFixedCalendar::usage`] records.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// The fixed day on which Armenian year `year` begins.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    julian::to_fixed(year + JULIAN_OFFSET, NEW_YEAR_MONTH, NEW_YEAR_DAY)
}

/// The fixed day the year begins on, without validation.
const fn new_year_raw(year: i64) -> i64 {
    match julian::to_fixed(year + JULIAN_OFFSET, NEW_YEAR_MONTH, NEW_YEAR_DAY) {
        Ok(rd) => rd.0,
        // Unreachable for any year in range: 11 August exists every year.
        Err(_) => 0,
    }
}

/// The number of days in `year`: 365, or 366 when the Julian leap day falls
/// inside it.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    (new_year_raw(year + 1) - new_year_raw(year)) as u16
}

/// Whether `year` carries the sixth epagomenal day.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    days_in_year(year) == 366
}

/// The number of days in `month` of `year`, or `None` outside `1..=13`.
///
/// The twelve months are thirty days each and never vary; the thirteenth is
/// the epagomenal *aweleacʿ*, five days long, or six in a long year.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match common::wandering_days_in_month(month) {
        Some(length) if month == 13 && is_leap_year(year) => Some(length + 1),
        other => other,
    }
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(new_year_raw(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1) - 1);

/// The first day the reform was in force.
pub const REFORM: Rd = Rd(new_year_raw(REFORM_YEAR));

/// Days elapsed in the year before the first of `month`.
const fn days_before_month(month: u8) -> i64 {
    (month as i64 - 1) * 30
}

/// The fixed day of a fixed-Armenian date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    match common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(new_year_raw(year)
            + days_before_month(month)
            + day as i64
            - 1)),
    }
}

/// The fixed-Armenian year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside the supported range.
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    // The mean year is 365.25 days, so this lands within one year and the
    // corrections close the gap.
    let mut year = (rd.0 - EARLIEST.0) * 4 / 1_461 + MIN_YEAR;
    while new_year_raw(year) > rd.0 {
        year -= 1;
    }
    while new_year_raw(year + 1) <= rd.0 {
        year += 1;
    }
    let day_of_year = rd.0 - new_year_raw(year);
    let month = day_of_year / 30 + 1;
    let day = day_of_year - (month - 1) * 30 + 1;
    // The epagomenal days run past twelve thirty-day months.
    if month > 13 {
        return Ok((year, 13, (day_of_year - 360 + 1) as u8));
    }
    Ok((year, month as u8, day as u8))
}

/// A date in the fixed Armenian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArmenianFixedDate {
    /// The Armenian year, numbered as in the wandering calendar.
    pub year: i64,
    /// The month, 1 through 13.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl ArmenianFixedDate {
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

    /// Whether this is one of the epagomenal days.
    #[must_use]
    pub const fn is_epagomenal(self) -> bool {
        self.month == 13
    }
}

/// The fixed Armenian calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ArmenianFixedCalendar;

impl Calendar for ArmenianFixedCalendar {
    type Date = ArmenianFixedDate;

    fn cycles(&self) -> Option<&'static [hc_calendar::shape::CycleShape]> {
        Some(hc_calendar::shape::WANDERING_THIRTEEN)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("armenian-fixed"),
            english_name: "Armenian (fixed, Sarkawag 1084)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        }
    }

    /// In use from the reform of 1084, and proleptic before it. The
    /// wandering calendar is the one that was in use then, and the two
    /// disagree by months.
    fn usage(&self) -> Usage {
        Usage::since(REFORM)
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(ArmenianFixedDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(ERA))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        ArmenianFixedDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::armenian;

    /// The anchor of the whole reform: 1 Nawasardi 533 is 11 August 1084.
    #[test]
    fn the_reform_puts_the_new_year_on_the_eleventh_of_august() {
        let reform = to_fixed(REFORM_YEAR, 1, 1).expect("in range");
        assert_eq!(julian::from_fixed(reform), Ok((1084, 8, 11)));
        assert_eq!(reform, REFORM);
    }

    /// And it stays there, which is the point of a fixed calendar.
    #[test]
    fn the_new_year_never_moves_against_the_julian_calendar() {
        for year in REFORM_YEAR..REFORM_YEAR + 800 {
            let start = new_year(year).expect("in range");
            assert_eq!(
                julian::from_fixed(start).map(|(_, month, day)| (month, day)),
                Ok((NEW_YEAR_MONTH, NEW_YEAR_DAY)),
                "Armenian {year}"
            );
        }
    }

    /// The reform was a *re-anchoring*, not a continuation, and this
    /// recovers Sarkawag's own reasoning from the arithmetic.
    ///
    /// By 1084 the wandering 1 Nawasardi had drifted back to 29 February.
    /// Sarkawag did not fix it there; he moved it forward to 11 August,
    /// where it had stood in the time of Maštocʿ. That is 31 days later
    /// than the era's own epoch position of 11 July — and 31 days is
    /// exactly 124 years of drift at a day per four years, 552 − 428. The
    /// calendar's arithmetic and the historical account agree to the day.
    #[test]
    fn the_reform_restored_the_new_year_to_where_it_stood_in_four_twenty_eight() {
        let wandering = armenian::to_fixed(REFORM_YEAR, 1, 1).expect("in range");
        let fixed = to_fixed(REFORM_YEAR, 1, 1).expect("in range");

        // Where the drift had taken it, and where he put it.
        assert_eq!(julian::from_fixed(wandering), Ok((1084, 2, 29)));
        assert_eq!(julian::from_fixed(fixed), Ok((1084, 8, 11)));
        assert_eq!(fixed.0 - wandering.0, 164, "the jump the reform made");

        // The era's epoch position is 11 July; the reform puts the new year
        // 31 days later in the Julian year.
        assert_eq!(julian::from_fixed(armenian::EPOCH), Ok((552, 7, 11)));
        let epoch_position = 31; // 11 July to 11 August
        assert_eq!(
            epoch_position,
            (552 - 428) / 4,
            "31 days is 124 years of wandering, and 552 - 428 is 124"
        );
    }

    /// After the reform the two calendars diverge without bound, a day
    /// every four years, which is what the reform was for.
    #[test]
    fn the_wandering_calendar_keeps_drifting_and_this_one_does_not() {
        let gap = |year: i64| {
            to_fixed(year, 1, 1).expect("in range").0
                - armenian::to_fixed(year, 1, 1).expect("in range").0
        };
        assert_eq!(gap(REFORM_YEAR), 164);
        assert_eq!(gap(REFORM_YEAR + 400), 164 + 100);
        assert_eq!(gap(REFORM_YEAR + 800), 164 + 200);
    }

    /// The sixth epagomenal day exists, and only in a long year.
    #[test]
    fn a_long_year_has_six_epagomenal_days() {
        let long = (REFORM_YEAR..REFORM_YEAR + 8)
            .find(|year| is_leap_year(*year))
            .expect("one in four");
        assert_eq!(days_in_month(long, 13), Some(6));
        assert_eq!(days_in_year(long), 366);
        assert!(to_fixed(long, 13, 6).is_ok());

        let short = (REFORM_YEAR..REFORM_YEAR + 8)
            .find(|year| !is_leap_year(*year))
            .expect("three in four");
        assert_eq!(days_in_month(short, 13), Some(5));
        assert_eq!(days_in_year(short), 365);
        assert_eq!(to_fixed(short, 13, 6), Err(CalendarError::DayOutOfRange));
    }

    /// One long year in four, and the mean year is therefore the Julian
    /// 365.25 — which is the whole content of the reform.
    #[test]
    fn one_year_in_four_is_long() {
        let span = 400;
        let long = (REFORM_YEAR..REFORM_YEAR + span)
            .filter(|year| is_leap_year(*year))
            .count();
        assert_eq!(long as i64, span / 4);
        let days: i64 = (REFORM_YEAR..REFORM_YEAR + span)
            .map(|year| i64::from(days_in_year(year)))
            .sum();
        assert_eq!(days, 365 * span + span / 4);
    }

    #[test]
    fn it_round_trips_every_day_of_two_centuries() {
        let start = new_year(REFORM_YEAR).expect("in range");
        let end = new_year(REFORM_YEAR + 200).expect("in range");
        for rd in start.0..end.0 {
            let rd = Rd(rd);
            let (year, month, day) = from_fixed(rd).expect("in range");
            assert_eq!(to_fixed(year, month, day), Ok(rd), "{rd}");
            assert!((1..=13).contains(&month));
            assert!(day >= 1);
        }
    }

    #[test]
    fn a_date_before_the_reform_is_proleptic_and_says_so() {
        use hc_calendar::Standing;
        let calendar = ArmenianFixedCalendar;
        assert_eq!(calendar.standing(REFORM), Standing::InUse);
        assert_eq!(
            calendar.standing(Rd(REFORM.0 - 1)),
            Standing::Proleptic,
            "Sarkawag reformed it in 1084, and the wandering year ran before"
        );
    }

    #[test]
    fn the_trait_round_trips_through_fields() {
        let calendar = ArmenianFixedCalendar;
        let date = ArmenianFixedDate::new(1473, 5, 17).expect("valid");
        let fields = calendar.to_fields(date).expect("convertible");
        assert_eq!(calendar.from_fields(&fields), Ok(date));
        assert_eq!(calendar.meta().id, CalendarId("armenian-fixed"));
    }
}
