//! The Martiana calendar: the Darian months and sols with Robert G. Aitken's
//! scheme for the week, as Thomas Gangale modified it in 2002.
//!
//! The months, the month lengths and the epoch are the Darian calendar's:
//! four quarters of five 28-sol months and one of 27, Vrishika keeping a
//! 28th sol in long years, and 1 Sagittarius 0 on Mars Sol Date −94 129.
//! Two things differ.
//!
//! * **The week runs on.** A Darian month always begins on Sol Solis and a
//!   27-sol month drops its Sol Saturni; in the Martiana calendar the
//!   seven-sol week is never shortened, so all the months of a quarter begin
//!   on the same sol of the week and each quarter begins a sol earlier than
//!   the last. In even years the quarters begin on Sol Solis, Sol Saturni,
//!   Sol Veneris and Sol Jovis; in odd years on Sol Mercurii, Sol Martis,
//!   Sol Lunae and Sol Solis. The pattern repeats every two years.
//! * **The long years.** The leap sol, Vrishika 28, falls in every odd year
//!   and is a sol of the week; every tenth year has a 28th sol of Vrishika
//!   too, but it is *epagomenal*, outside the week, so the two-year rotation
//!   is not disturbed. The page states no exception for the centuries, and
//!   none is applied: every year divisible by ten is long, where the Darian
//!   calendar makes years divisible by 100 and not by 500 short. The two
//!   calendars therefore agree on every date of years 0 to 99, and from
//!   year 101 onwards the Martiana year begins one sol later for every such
//!   century passed: two sols from year 201.
//!
//! The Martiana [`Rd`] is the **Darian sol number** — the same count from
//! the same epoch as [`super::darian::DarianCalendar`]'s — so the two
//! calendars convert into each other through it, and, as there, it must
//! never be handed to a terrestrial calendar.
//!
//! The page's Table 1-13 is the perpetual calendar: its even-year column
//! puts Vrishika 28 under a heading "Decennial" outside the week, and its
//! odd-year column puts it on Sol Saturni. Its text says "In the
//! even-numbered years" twice, the second time for the quarters that begin
//! on Sol Mercurii, Sol Martis, Sol Lunae and Sol Solis; Table 1-13 and the
//! arithmetic put those in the odd years, and this module follows them.
//! Aitken's own calendar of 1936 — years of 668, 669 and 670 sols, the leap
//! sol and the epagomenal sol both in even years, the latter at mid-year —
//! was not read and is not carried.
//!
//! Source: Gangale, T., "The Darian Calendar for Mars", §1.4.1, "The
//! Martiana Calendar", <https://ops-alaska.com/time/gangale_mst/darian.htm>,
//! and Table 1-13 at <https://ops-alaska.com/time/gangale_mst/t2002martiana.htm>,
//! retrieved 2026-09-26 (`gangale-darian`). The system is described in
//! `docs/systems/mars-timekeeping.md`.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use super::MarsMoment;
use super::darian::{Intercalation, MAX_YEAR, MIN_YEAR, MONTH_NAMES, WEEKDAY_NAMES};

/// The position in the week, `0` being Sol Solis, on which each quarter's
/// months begin, for even and for odd years (§1.4.1; Table 1-13).
pub const QUARTER_START_WEEKDAYS: [[u8; 4]; 2] = [[0, 6, 5, 4], [3, 2, 1, 0]];

/// A date in the Martiana calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MartianaDate {
    /// The year, counted as the Darian year is.
    pub year: i64,
    /// The month, in `1..=24`.
    pub month: u8,
    /// The sol of the month, in `1..=28`.
    pub day: u8,
}

impl MartianaDate {
    /// A Martiana date, unchecked; [`sol_from_date`] is what validates it.
    #[must_use]
    pub const fn new(year: i64, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Whether this is the epagomenal sol, Vrishika 28 of an even year,
    /// which belongs to no week.
    #[must_use]
    pub const fn is_epagomenal(&self) -> bool {
        self.month == 24 && self.day == 28 && self.year.rem_euclid(2) == 0
    }

    /// The name of this date's month.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] when the month is not in
    /// `1..=24`.
    pub fn month_name(&self) -> CalendarResult<&'static str> {
        MONTH_NAMES
            .get(usize::from(self.month).wrapping_sub(1))
            .copied()
            .ok_or(CalendarError::MonthOutOfRange)
    }

    /// The sol of the week, `0..7` from Sol Solis, or `None` for the
    /// epagomenal sol.
    ///
    /// # Errors
    ///
    /// Returns the error [`sol_from_date`] would for a date that does not
    /// exist.
    pub fn weekday_index(&self) -> CalendarResult<Option<u8>> {
        sol_from_date(*self)?;
        if self.is_epagomenal() {
            return Ok(None);
        }
        let parity = self.year.rem_euclid(2) as usize;
        let quarter = usize::from((self.month - 1) / 6);
        Ok(Some(
            (QUARTER_START_WEEKDAYS[parity][quarter] + self.day - 1) % 7,
        ))
    }

    /// The name of the sol of the week, or `None` for the epagomenal sol.
    ///
    /// # Errors
    ///
    /// As [`Self::weekday_index`].
    pub fn weekday_name(&self) -> CalendarResult<Option<&'static str>> {
        Ok(self
            .weekday_index()?
            .map(|index| WEEKDAY_NAMES[usize::from(index)]))
    }
}

/// Whether `year` is long, 669 sols: odd, with the leap sol, or divisible by
/// ten, with the epagomenal sol.
#[must_use]
pub const fn is_long_year(year: i64) -> bool {
    Intercalation::Martiana.is_long(year)
}

/// Whether `year` has the leap sol, the 28th of Vrishika counted in the
/// week: the odd years.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    year.rem_euclid(2) != 0
}

/// Whether `year` has the epagomenal sol outside the week: the years
/// divisible by ten.
#[must_use]
pub const fn has_epagomenal_sol(year: i64) -> bool {
    year.rem_euclid(10) == 0
}

/// The Darian sol number of 1 Sagittarius of a Martiana year.
#[must_use]
pub const fn year_start_sol(year: i64) -> i64 {
    Intercalation::Martiana.year_start_sol(year)
}

/// The sols in `month` of `year`, or `None` for a month not in `1..=24`.
#[must_use]
pub const fn sols_in_month(year: i64, month: u8) -> Option<u8> {
    Intercalation::Martiana.sols_in_month(year, month)
}

/// The Darian sol number of a Martiana date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`]
/// when the date does not exist.
pub const fn sol_from_date(date: MartianaDate) -> CalendarResult<i64> {
    Intercalation::Martiana.sol_from_date(date.year, date.month, date.day)
}

/// The Martiana date of a Darian sol number.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn date_from_sol(sol: i64) -> CalendarResult<MartianaDate> {
    match Intercalation::Martiana.date_from_sol(sol) {
        Ok((year, month, day)) => Ok(MartianaDate::new(year, month, day)),
        Err(error) => Err(error),
    }
}

/// The Martiana calendar, `martiana`.
///
/// Its `Rd` values are Darian sol numbers, not Earth days; see the module
/// documentation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MartianaCalendar;

impl MartianaCalendar {
    /// The Martiana date at a Martian moment, at the prime meridian.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside the supported
    /// years.
    pub fn date_at(&self, moment: MarsMoment) -> CalendarResult<MartianaDate> {
        date_from_sol(super::darian::sol_from_mars_sol_date(
            moment.mars_sol_date(),
        ))
    }
}

impl Calendar for MartianaCalendar {
    type Date = MartianaDate;

    /// The Darian months and the seven-sol week, which here runs on across
    /// months and skips only the epagomenal sol.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        const SHAPE: &[hc_calendar::shape::CycleShape] = &[
            hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTH_NAMES),
            hc_calendar::shape::CycleShape::named("sol-of-week", &WEEKDAY_NAMES),
        ];
        SHAPE
    }

    /// An odd year, which carries the leap sol. The decennial epagomenal
    /// sol is not the calendar's leap sol and does not make a year leap; see
    /// [`has_epagomenal_sol`] and [`is_long_year`].
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("martiana"),
            english_name: "Martiana (Mars)",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(Rd(year_start_sol(MIN_YEAR))),
            latest: Some(Rd(year_start_sol(MAX_YEAR + 1) - 1)),
            native_locales: &[],
        }
    }

    /// The Darian **sol** number of a date, carried in an [`Rd`].
    ///
    /// # Errors
    ///
    /// See [`sol_from_date`].
    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        sol_from_date(date).map(Rd)
    }

    /// The date of a Darian **sol** number carried in an [`Rd`].
    ///
    /// # Errors
    ///
    /// See [`date_from_sol`].
    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        date_from_sol(rd.get())
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        sol_from_date(date)?;
        Ok(DateFields::ymd(date.year, date.month, date.day))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = MartianaDate::new(fields.year, month.ordinal, fields.require_day()?);
        sol_from_date(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::super::darian::{self, DarianCalendar, DarianDate};
    use super::*;

    #[test]
    fn the_long_years_are_the_odd_and_the_decennial_ones() {
        // §1.4.1: odd years 669 sols "as do decennial years", even
        // non-decennial years 668.
        for year in -40..40 {
            let expected = year % 2 != 0 || year % 10 == 0;
            assert_eq!(is_long_year(year), expected, "{year}");
            assert_eq!(
                year_start_sol(year + 1) - year_start_sol(year),
                if expected { 669 } else { 668 }
            );
        }
        // No century exception: 100 and 200 are long, as Darian's are not.
        assert!(is_long_year(100) && is_long_year(200) && is_long_year(300));
        assert!(!darian::is_leap_year(100) && !darian::is_leap_year(200));
        assert!(is_leap_year(3) && !is_leap_year(10) && has_epagomenal_sol(10));
        assert_eq!(MartianaCalendar.is_leap_year(10), Ok(false));
        assert_eq!(MartianaCalendar.is_leap_year(11), Ok(true));
        assert_eq!(
            MartianaCalendar.is_leap_year(MAX_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
        );
        // The mean year is Aitken's, 668.6 sols.
        let mean = (year_start_sol(1_000) - year_start_sol(0)) as f64 / 1_000.0;
        assert!((mean - 668.6).abs() < 1e-12);
    }

    /// Table 1-13: the sol of the week on which the months of each quarter
    /// begin, in even and odd years.
    #[test]
    fn every_month_of_a_quarter_begins_on_the_table_1_13_weekday() {
        let even = ["Sol Solis", "Sol Saturni", "Sol Veneris", "Sol Jovis"];
        let odd = ["Sol Mercurii", "Sol Martis", "Sol Lunae", "Sol Solis"];
        for year in -21..221 {
            let expected = if year % 2 == 0 { even } else { odd };
            for month in 1..=24u8 {
                let date = MartianaDate::new(year, month, 1);
                assert_eq!(
                    date.weekday_name().unwrap(),
                    Some(expected[usize::from((month - 1) / 6)]),
                    "{year} {month}"
                );
            }
        }
    }

    #[test]
    fn the_week_never_breaks_except_for_the_epagomenal_sol() {
        let mut previous: Option<u8> = None;
        let mut epagomenal = 0;
        let mut sol = year_start_sol(-30);
        while sol < year_start_sol(230) {
            let date = date_from_sol(sol).unwrap();
            match date.weekday_index().unwrap() {
                None => {
                    assert!(has_epagomenal_sol(date.year) && date.month == 24 && date.day == 28);
                    epagomenal += 1;
                }
                Some(index) => {
                    if let Some(before) = previous {
                        assert_eq!((before + 1) % 7, index, "{date:?}");
                    }
                    previous = Some(index);
                }
            }
            sol += 1;
        }
        assert_eq!(epagomenal, 26);
    }

    #[test]
    fn the_leap_sol_is_a_sol_saturni_and_the_epagomenal_sol_has_no_weekday() {
        // Table 1-13: odd years' Vrishika 28 is under Sa; even decennial
        // years' Vrishika 28 is under "Decennial", outside the week.
        assert_eq!(
            MartianaDate::new(219, 24, 28).weekday_name(),
            Ok(Some("Sol Saturni"))
        );
        assert_eq!(MartianaDate::new(220, 24, 28).weekday_name(), Ok(None));
        assert!(MartianaDate::new(220, 24, 28).is_epagomenal());
        assert_eq!(
            MartianaDate::new(222, 24, 28).weekday_name(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            MartianaDate::new(222, 25, 1).weekday_name(),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            MartianaDate::new(1, 25, 1).month_name(),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(MartianaDate::new(1, 7, 1).month_name(), Ok("Pisces"));
    }

    #[test]
    fn martiana_and_darian_agree_before_the_first_century() {
        // Same months, same epoch, same long years until year 100.
        let mut sol = darian::year_start_sol(-99);
        while sol < darian::year_start_sol(100) {
            let d = darian::date_from_sol(sol).unwrap();
            let m = date_from_sol(sol).unwrap();
            assert_eq!((d.year, d.month, d.day), (m.year, m.month, m.day));
            sol += 1;
        }
        // Year 100 is long in Martiana and short in Darian, so from year 101
        // the Martiana year begins one sol later, and from 201 two.
        assert_eq!(year_start_sol(101) - darian::year_start_sol(101), 1);
        assert_eq!(year_start_sol(201) - darian::year_start_sol(201), 2);
        assert_eq!(year_start_sol(219) - darian::year_start_sol(219), 2);
    }

    #[test]
    fn the_two_calendars_convert_through_the_shared_sol_count() {
        // Perseverance landed on 13 Sagittarius 219 Darian; two sols of
        // Martiana year starts later, that sol is 11 Sagittarius 219.
        let darian_date = DarianDate::new(219, 1, 13);
        let martiana = DarianCalendar
            .convert_to(darian_date, &MartianaCalendar)
            .unwrap();
        assert_eq!(martiana, MartianaDate::new(219, 1, 11));
        assert_eq!(
            MartianaCalendar.convert_to(martiana, &DarianCalendar),
            Ok(darian_date)
        );
        let landing = super::super::missions::mission("Perseverance")
            .unwrap()
            .landing_moment()
            .unwrap();
        assert_eq!(MartianaCalendar.date_at(landing), Ok(martiana));
    }

    #[test]
    fn the_calendar_trait_round_trips_and_refuses_what_does_not_exist() {
        let calendar = MartianaCalendar;
        let mut sol = -3_000;
        while sol < 3_000 {
            let date = calendar.from_fixed(Rd(sol)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(sol)));
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date));
            sol += 1;
        }
        assert_eq!(
            calendar.to_fixed(MartianaDate::new(2, 24, 28)),
            Err(CalendarError::DayOutOfRange)
        );
        assert!(calendar.to_fixed(MartianaDate::new(200, 24, 28)).is_ok());
        assert_eq!(
            calendar.to_fixed(MartianaDate::new(1, 6, 28)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            calendar.to_fixed(MartianaDate::new(MAX_YEAR + 1, 1, 1)),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::ymd_leap_month(1, 1, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert!(calendar.from_fields(&DateFields::new(1)).is_err());
        assert_eq!(
            calendar.to_fields(MartianaDate::new(1, 0, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        let meta = calendar.meta();
        assert_eq!(meta.id, CalendarId("martiana"));
        assert!(meta.supports(Rd(0)));
        assert_eq!(calendar.cycles()[1].names.len(), 7);
        assert_eq!(sols_in_month(3, 24), Some(28));
        assert_eq!(sols_in_month(3, 25), None);
    }
}
